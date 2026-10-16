//! Mode capture : `MODELE_CAPTURE=dossier cargo run -p modele` enregistre le
//! menu puis une partie, et quitte (pour vérifier le rendu sans y jouer).
use bevy::prelude::*;
use bevy::render::view::screenshot::{Screenshot, save_to_disk};

use crate::GameState;

const CLICHES: [(&str, GameState, f32); 2] = [("1-menu", GameState::Menu, 4.0), ("2-partie", GameState::Playing, 2.0)];

#[derive(Resource)]
struct Capture {
    dossier: String,
    i: usize,
    depuis: f32,
    pris: bool,
}

pub struct CapturePlugin;

impl Plugin for CapturePlugin {
    fn build(&self, app: &mut App) {
        if let Ok(dossier) = std::env::var("MODELE_CAPTURE") {
            app.insert_resource(Capture { dossier, i: 0, depuis: 0.0, pris: false }).add_systems(Update, capturer);
        }
    }
}

fn capturer(
    time: Res<Time>,
    mut commands: Commands,
    mut cap: ResMut<Capture>,
    state: Res<State<GameState>>,
    mut next: ResMut<NextState<GameState>>,
    mut exit: MessageWriter<AppExit>,
) {
    let t = time.elapsed_secs();
    let Some(&(nom, etat, delai)) = CLICHES.get(cap.i) else {
        exit.write(AppExit::Success);
        return;
    };
    if *state.get() != etat {
        next.set(etat);
        cap.depuis = t;
        return;
    }
    if !cap.pris && t - cap.depuis >= delai {
        cap.pris = true;
        commands.spawn(Screenshot::primary_window()).observe(save_to_disk(format!("{}/{nom}.png", cap.dossier)));
    } else if cap.pris && t - cap.depuis >= delai + 0.5 {
        cap.i += 1;
        cap.pris = false;
        cap.depuis = t;
    }
}
