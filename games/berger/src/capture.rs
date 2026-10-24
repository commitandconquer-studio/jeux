//! Mode capture : `BERGER_CAPTURE=dossier cargo run -p berger` enregistre le
//! menu, les niveaux puis des parties jouées par le chien automatique, et quitte
//! (pour vérifier le rendu sans y jouer).
use bevy::prelude::*;
use bevy::render::view::screenshot::{Screenshot, save_to_disk};

use crate::GameState;
use crate::jeu::{Commencer, Genre};

/// (nom, état, délai, partie à lancer avant la prise de vue)
const CLICHES: [(&str, GameState, f32, Option<Genre>); 6] = [
    ("1-menu", GameState::Menu, 3.0, None),
    ("2-niveaux", GameState::Niveaux, 1.5, None),
    ("3-premier-pre", GameState::Playing, 14.0, Some(Genre::Campagne(0))),
    ("4-obstacles", GameState::Playing, 25.0, Some(Genre::Campagne(3))),
    ("5-loups", GameState::Playing, 45.0, Some(Genre::Campagne(7))),
    ("6-grand-pre", GameState::Playing, 30.0, Some(Genre::Campagne(11))),
];

#[derive(Resource)]
struct Capture {
    dossier: String,
    i: usize,
    depuis: f32,
    lance: bool,
    pris: bool,
}

pub struct CapturePlugin;

impl Plugin for CapturePlugin {
    fn build(&self, app: &mut App) {
        if let Ok(dossier) = std::env::var("BERGER_CAPTURE") {
            app.insert_resource(Capture { dossier, i: 0, depuis: 0.0, lance: false, pris: false }).add_systems(Update, capturer);
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn capturer(
    time: Res<Time>,
    mut commands: Commands,
    mut cap: ResMut<Capture>,
    state: Res<State<GameState>>,
    mut next: ResMut<NextState<GameState>>,
    mut commencer: MessageWriter<Commencer>,
    mut exit: MessageWriter<AppExit>,
) {
    let t = time.elapsed_secs();
    let Some(&(nom, etat, delai, genre)) = CLICHES.get(cap.i) else {
        exit.write(AppExit::Success);
        return;
    };
    if !cap.lance {
        cap.lance = true;
        cap.depuis = t;
        next.set(etat);
        if let Some(genre) = genre {
            // Le chien automatique joue : on voit le troupeau bouger.
            commencer.write(Commencer { genre, demo: false, robot: true });
        }
        return;
    }
    if *state.get() != etat {
        return;
    }
    if !cap.pris && t - cap.depuis >= delai {
        cap.pris = true;
        commands.spawn(Screenshot::primary_window()).observe(save_to_disk(format!("{}/{nom}.png", cap.dossier)));
    } else if cap.pris && t - cap.depuis >= delai + 0.5 {
        cap.i += 1;
        cap.pris = false;
        cap.lance = false;
    }
}
