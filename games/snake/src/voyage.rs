//! Le mode Voyage : des niveaux avec un objectif (recruter des touristes, puis passer la sortie),
//! des étoiles, une progression sauvegardée. Finir un lieu en débloque le paysage dans le mode infini.
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use game_common::Score;

use crate::GameState;
use crate::layout::Layout;
use crate::levels::{LEVELS, Level, Place};
use crate::progress::Progress;
use crate::settings::Settings;
use crate::world::Universe;

const SAVE: &str = "snake";

/// Ce qu'on joue.
#[derive(Resource, Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum Mode {
    #[default]
    Infinite,
    /// Indice dans `LEVELS`.
    Voyage(usize),
}

impl Mode {
    pub fn level(self) -> Option<&'static Level> {
        match self {
            Mode::Voyage(i) => LEVELS.get(i),
            Mode::Infinite => None,
        }
    }
}

/// Déroulement du niveau en cours.
#[derive(Resource, Default)]
pub struct Run {
    pub recruited: u32,
    pub elapsed: f32,
    /// Case de la sortie une fois ouverte.
    pub exit: Option<IVec2>,
    /// La file est en train de passer la sortie.
    pub exiting: bool,
    /// La logique a avancé d'une case à ce tour (pour enchaîner le défilé de sortie).
    pub stepped: bool,
}

/// Résultat du dernier niveau.
#[derive(Resource, Default)]
pub struct Outcome {
    pub won: bool,
    pub stars: u8,
    pub time: f32,
    /// Étoiles du meilleur résultat avant cette partie.
    pub best_before: u8,
}

/// Le paysage affiché (celui du mode infini, ou celui du lieu du niveau).
#[derive(Resource, Clone, Copy, Default)]
pub struct Scenery(pub Universe);

#[derive(Resource, Default)]
pub struct VoyageProgress {
    stars: Vec<u8>,
    /// Tous les niveaux sont ouverts (essais).
    open: bool,
}

impl VoyageProgress {
    pub fn stars(&self, level: usize) -> u8 {
        self.stars.get(level).copied().unwrap_or(0)
    }

    /// Un niveau s'ouvre quand le précédent est réussi.
    pub fn unlocked(&self, level: usize) -> bool {
        self.open || level == 0 || self.stars(level - 1) >= 1
    }

    fn record(&mut self, level: usize, stars: u8) {
        if self.stars.len() <= level {
            self.stars.resize(level + 1, 0);
        }
        self.stars[level] = self.stars[level].max(stars);
        let text: Vec<String> = self.stars.iter().map(|s| s.to_string()).collect();
        // Les essais automatiques ne touchent pas à la vraie sauvegarde.
        if std::env::var("SNAKE_AUTO").is_err() {
            game_common::save::store(SAVE, "voyage", &text.join(","));
        }
    }

    /// Tous les niveaux du lieu sont réussis.
    pub fn place_done(&self, place: Place) -> bool {
        let mut any = false;
        for (i, l) in LEVELS.iter().enumerate() {
            if l.place == place {
                any = true;
                if self.stars(i) == 0 {
                    return false;
                }
            }
        }
        any
    }

    pub fn total_stars(&self) -> u32 {
        self.stars.iter().map(|s| *s as u32).sum()
    }
}

/// Étoiles gagnées : 1 pour l'objectif, 2 dans le temps de référence, 3 avec 25 % de moins.
pub fn stars_for(level: &Level, time: f32) -> u8 {
    if time <= level.par * 0.75 {
        3
    } else if time <= level.par {
        2
    } else {
        1
    }
}

/// Ce que les systèmes de jeu ont besoin de savoir du Voyage.
#[derive(SystemParam)]
pub struct Voyage<'w> {
    pub mode: Res<'w, Mode>,
    pub run: ResMut<'w, Run>,
    pub outcome: ResMut<'w, Outcome>,
    pub next: ResMut<'w, NextState<GameState>>,
    pub scenery: Res<'w, Scenery>,
}

pub struct VoyagePlugin;

impl Plugin for VoyagePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Mode>()
            .init_resource::<Run>()
            .init_resource::<Outcome>()
            .init_resource::<Scenery>()
            .init_resource::<VoyageProgress>()
            .add_systems(Startup, load)
            .add_systems(PreUpdate, sync_layout)
            .add_systems(OnEnter(GameState::Menu), |mut m: ResMut<Mode>| *m = Mode::Infinite)
            .add_systems(OnEnter(GameState::GameOver), conclude);
    }
}

fn load(mut p: ResMut<VoyageProgress>, mut progress: ResMut<Progress>, mut mode: ResMut<Mode>, mut next: ResMut<NextState<GameState>>) {
    // Essai : `SNAKE_NIVEAU=n` ouvre directement le niveau n.
    if let Some(n) = std::env::var("SNAKE_NIVEAU").ok().and_then(|v| v.parse::<usize>().ok()) {
        *mode = Mode::Voyage(n.saturating_sub(1));
        next.set(GameState::Playing);
    }
    if let Some(text) = game_common::save::load(SAVE, "voyage") {
        p.stars = text.split(',').filter_map(|s| s.trim().parse().ok()).collect();
    }
    p.open = std::env::var("SNAKE_TOUT").is_ok() || std::env::var("SNAKE_PRESSE").is_ok();
    progress.places = Place::ALL.iter().filter(|pl| p.place_done(**pl)).map(|pl| pl.scenery()).collect();
}

/// La forme et le paysage suivent le mode (infini : les réglages ; voyage : le niveau).
fn sync_layout(
    settings: Res<Settings>,
    mode: Res<Mode>,
    mut layout: ResMut<Layout>,
    mut scenery: ResMut<Scenery>,
) {
    if !(settings.is_changed() || mode.is_changed()) {
        return;
    }
    match mode.level() {
        Some(level) => {
            let Mode::Voyage(i) = *mode else { return };
            *layout = level.layout(i as u32);
            scenery.0 = level.place.scenery();
        }
        None => {
            *layout = settings.layout();
            scenery.0 = settings.universe;
        }
    }
}

/// Fin de partie : en Voyage, enregistre les étoiles d'un niveau réussi.
fn conclude(
    mode: Res<Mode>,
    outcome: Res<Outcome>,
    mut progress_v: ResMut<VoyageProgress>,
    mut progress: ResMut<Progress>,
    _score: Res<Score>,
) {
    let Mode::Voyage(i) = *mode else { return };
    if !outcome.won {
        return;
    }
    info!("voyage : niveau {} gagné, {} étoile(s), {:.1} s", i + 1, outcome.stars, outcome.time);
    progress_v.record(i, outcome.stars);
    progress.places = Place::ALL.iter().filter(|pl| progress_v.place_done(**pl)).map(|pl| pl.scenery()).collect();
}
