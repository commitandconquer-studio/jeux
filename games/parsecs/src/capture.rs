//! Mode capture : `PARSECS_CAPTURE=dossier cargo run -p parsecs` enregistre
//! l'accueil, le hangar, une partie (pilote automatique), l'arrivée d'un boss
//! et l'écran de fin, puis quitte.
use bevy::prelude::*;
use bevy::render::view::screenshot::{Screenshot, save_to_disk};

use crate::GameState;
use crate::partie::{DUREE_PARSEC, Joueur, Partie, Rocher};
use crate::reglages::Reglages;

#[derive(Clone, Copy, PartialEq)]
enum Etape {
    Menu,
    Hangar,
    Partie,
    Boss,
    Fin,
}

const CLICHES: [(&str, Etape, f32); 5] = [
    ("1-menu", Etape::Menu, 4.0),
    ("2-hangar", Etape::Hangar, 2.0),
    ("3-partie", Etape::Partie, 9.0),
    ("4-boss", Etape::Boss, 4.5),
    ("5-fin", Etape::Fin, 2.0),
];

#[derive(Resource)]
struct Capture {
    dossier: String,
    i: usize,
    depuis: f32,
    pret: bool,
    pris: bool,
}

pub struct CapturePlugin;

impl Plugin for CapturePlugin {
    fn build(&self, app: &mut App) {
        if let Ok(dossier) = std::env::var("PARSECS_CAPTURE") {
            app.insert_resource(Capture { dossier, i: 0, depuis: 0.0, pret: false, pris: false })
                .add_systems(Update, (capturer, pilote_automatique.run_if(in_state(GameState::Playing))));
        }
    }
}

fn voulu(e: Etape) -> GameState {
    match e {
        Etape::Menu => GameState::Menu,
        Etape::Hangar => GameState::Hangar,
        Etape::Partie | Etape::Boss => GameState::Playing,
        Etape::Fin => GameState::Over,
    }
}

#[allow(clippy::too_many_arguments)]
fn capturer(
    time: Res<Time>,
    mut commands: Commands,
    mut cap: ResMut<Capture>,
    state: Res<State<GameState>>,
    mut next: ResMut<NextState<GameState>>,
    mut partie: ResMut<Partie>,
    mut reglages: ResMut<Reglages>,
    mut exit: MessageWriter<AppExit>,
) {
    let t = time.elapsed_secs();
    let Some(&(nom, etape, delai)) = CLICHES.get(cap.i) else {
        exit.write(AppExit::Success);
        return;
    };
    if !cap.pret {
        cap.pret = true;
        cap.depuis = t;
        reglages.tir_auto = true;
        match etape {
            Etape::Menu => {}
            Etape::Hangar => next.set(GameState::Hangar),
            Etape::Partie => next.set(GameState::Playing),
            // Juste avant le parsec 3 : le boss arrive.
            Etape::Boss => partie.temps = DUREE_PARSEC * 2.0 - 0.5,
            Etape::Fin => {
                partie.score = partie.score.max(4_200);
                next.set(GameState::Over);
            }
        }
        return;
    }
    if *state.get() != voulu(etape) {
        cap.depuis = t;
        return;
    }
    if !cap.pris && t - cap.depuis >= delai {
        cap.pris = true;
        commands.spawn(Screenshot::primary_window()).observe(save_to_disk(format!("{}/{nom}.png", cap.dossier)));
    } else if cap.pris && t - cap.depuis >= delai + 0.5 {
        cap.i += 1;
        cap.pret = false;
        cap.pris = false;
    }
}

/// Le pilote automatique : se place sous le rocher le plus proche et tire ;
/// le bouclier reste plein pour que la capture aille jusqu'au bout.
fn pilote_automatique(
    time: Res<Time>,
    mut partie: ResMut<Partie>,
    mut joueur: Query<&mut Transform, With<Joueur>>,
    rochers: Query<&Transform, (With<Rocher>, Without<Joueur>)>,
) {
    partie.bouclier = partie.bouclier_max;
    let Ok(mut j) = joueur.single_mut() else { return };
    let p = j.translation;
    if let Some(cible) = rochers.iter().map(|r| r.translation).filter(|r| r.y > p.y).min_by(|a, b| (a.x - p.x).abs().total_cmp(&(b.x - p.x).abs())) {
        let dx = (cible.x - p.x).clamp(-1.0, 1.0);
        j.translation.x += dx * 9.0 * time.delta_secs();
    }
}
