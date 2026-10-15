//! Mode « captures d'écran » : `NOVA_CAPTURE=dossier cargo run -p nova`
//! enregistre l'accueil (démo en fond), un dialogue, puis le chapitre 1 joué
//! par le pilote automatique jusqu'à la victoire, et quitte.
use bevy::prelude::*;
use bevy::render::view::screenshot::{Screenshot, save_to_disk};

use crate::GameState;
use crate::game::Game;

/// (nom, état attendu, secondes passées dans cet état avant la capture)
const SHOTS: [(&str, GameState, f32); 5] = [
    ("1-menu", GameState::Menu, 8.0),
    ("2-dialogue", GameState::Story, 1.0),
    ("3-combat", GameState::Playing, 3.0),
    ("4-combat-avance", GameState::Playing, 3.0),
    ("5-victoire", GameState::Victory, 1.5),
];

#[derive(Resource)]
struct Capture {
    dir: String,
    index: usize,
    since: f32,
    shot: bool,
}

pub struct CapturePlugin;

impl Plugin for CapturePlugin {
    fn build(&self, app: &mut App) {
        if let Ok(dir) = std::env::var("NOVA_CAPTURE") {
            app.insert_resource(Capture { dir, index: 0, since: 0.0, shot: false }).add_systems(Update, run_capture);
        }
    }
}

fn run_capture(
    time: Res<Time>,
    mut commands: Commands,
    mut cap: ResMut<Capture>,
    mut game: ResMut<Game>,
    state: Res<State<GameState>>,
    mut input: ResMut<ButtonInput<KeyCode>>,
    mut exit: MessageWriter<AppExit>,
) {
    let now = time.elapsed_secs();
    // En partie, c'est le pilote automatique qui joue.
    if *state.get() == GameState::Playing {
        game.autopilot = true;
    }
    let Some(&(name, wanted, delay)) = SHOTS.get(cap.index) else {
        if now - cap.since > 0.5 {
            exit.write(AppExit::Success);
        }
        return;
    };
    // Si l'étape est dépassée (la partie a déjà été gagnée), on passe.
    if wanted == GameState::Playing && *state.get() == GameState::Victory {
        cap.index += 1;
        return;
    }
    if *state.get() != wanted {
        cap.since = now;
        // Avance dans les menus et dialogues en « appuyant » sur Entrée.
        input.release(KeyCode::Enter);
        if matches!(state.get(), GameState::Menu | GameState::Story) && cap.index > 0 {
            input.press(KeyCode::Enter);
        }
        return;
    }
    input.release(KeyCode::Enter);
    let elapsed = now - cap.since;
    if !cap.shot && elapsed >= delay {
        cap.shot = true;
        commands.spawn(Screenshot::primary_window()).observe(save_to_disk(format!("{}/{name}.png", cap.dir)));
    } else if cap.shot && elapsed >= delay + 0.4 {
        cap.index += 1;
        cap.since = now;
        cap.shot = false;
    }
}
