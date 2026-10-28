//! Le mode scénario : des aventures faites de scènes (décor, personnages, sous-titres) et de niveaux, enchaînées.
//! Une aventure est un fichier de `games/snake/scenarios/` (le même format que les niveaux, avec des sections `type = scene`).
use std::collections::HashMap;
use std::sync::LazyLock;

use bevy::prelude::*;

use crate::GameState;
use crate::levels::{Level, Meta, parse_file, preview_text};
use crate::voyage::{Mode, Outcome};

pub struct Scenario {
    pub title: String,
    pub description: String,
    pub steps: Vec<Level>,
}

fn build(text: &str, fallback_title: &str) -> Result<Scenario, String> {
    let (Meta { title, description }, steps) = parse_file(text)?;
    Ok(Scenario { title: if title.is_empty() { fallback_title.to_owned() } else { title }, description, steps })
}

/// Toutes les aventures. En aperçu (éditeur), seul le fichier d'aperçu.
pub static SCENARIOS: LazyLock<Vec<Scenario>> = LazyLock::new(|| {
    if let Some(text) = preview_text() {
        match build(&text, "Aperçu") {
            Ok(s) if !s.steps.is_empty() => return vec![s],
            Ok(_) => warn!("aperçu : aucune étape dans le fichier"),
            Err(e) => warn!("aperçu : fichier illisible : {e}"),
        }
        // Repli : une aventure minimale, pour que le jeu ne plante pas.
        return vec![build("[Aperçu vide]\ntype = scene\nreplique = Guide | Le fichier d'aperçu est illisible ou vide.\n", "Aperçu").expect("aventure de repli")];
    }
    let mut all = Vec::new();
    for (name, text) in [("Découverte", include_str!("../scenarios/decouverte.txt"))] {
        all.push(build(text, name).unwrap_or_else(|e| panic!("aventure « {name} » illisible : {e}")));
    }
    all
});

/// Où en est le joueur dans chaque aventure : la prochaine étape, par titre.
#[derive(Resource, Default)]
pub struct StoryProgress {
    next: HashMap<String, usize>,
}

impl StoryProgress {
    pub fn next_step(&self, title: &str) -> usize {
        self.next.get(title).copied().unwrap_or(0)
    }

    fn set(&mut self, title: &str, step: usize) {
        if crate::levels::preview_active() {
            return;
        }
        self.next.insert(title.to_owned(), step);
        let text: Vec<String> = self.next.iter().map(|(t, s)| format!("{t}\t{s}")).collect();
        game_common::save::store("snake", "scenarios", &text.join("\n"));
    }
}

fn load_progress(mut p: ResMut<StoryProgress>) {
    if let Some(text) = game_common::save::load("snake", "scenarios") {
        p.next = text.lines().filter_map(|l| l.split_once('\t')).filter_map(|(t, s)| Some((t.to_owned(), s.parse().ok()?))).collect();
    }
}

/// Passe à l'étape suivante de l'aventure en cours.
pub fn advance(mode: &mut Mode, progress: &mut StoryProgress, next: &mut NextState<GameState>) {
    let Mode::Scenario { scenario, step } = *mode else { return };
    // Aperçu d'une seule étape : on la rejoue au lieu de passer à la suivante.
    let step = if crate::levels::single_step() { step } else { step + 1 };
    if let Some(sc) = SCENARIOS.get(scenario) {
        progress.set(&sc.title, step);
    }
    *mode = Mode::Scenario { scenario, step };
    next.set(GameState::Step);
}

/// Étape de transition : on choisit l'écran de l'étape en cours (scène, niveau ou fin de l'aventure).
fn enter_step(mode: Res<Mode>, mut next: ResMut<NextState<GameState>>) {
    let Mode::Scenario { scenario, step } = *mode else {
        next.set(GameState::Menu);
        return;
    };
    match SCENARIOS.get(scenario).and_then(|s| s.steps.get(step)) {
        Some(l) if l.scene.is_some() => next.set(GameState::Scene),
        Some(_) => next.set(GameState::Playing),
        // Fin de l'aventure.
        None => next.set(if crate::levels::preview_active() { GameState::Menu } else { GameState::Scenarios }),
    }
}

/// Un niveau réussi dans une aventure : on enchaîne sans écran de fin.
fn after_win(mut mode: ResMut<Mode>, outcome: Res<Outcome>, mut progress: ResMut<StoryProgress>, mut next: ResMut<NextState<GameState>>) {
    if matches!(*mode, Mode::Scenario { .. }) && outcome.won {
        advance(&mut mode, &mut progress, &mut next);
    }
}

pub struct StoryPlugin;

impl Plugin for StoryPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StoryProgress>()
            .add_systems(Startup, load_progress)
            .add_systems(OnEnter(GameState::Step), enter_step)
            .add_systems(OnEnter(GameState::GameOver), after_win.after(crate::voyage::conclude));
    }
}
