//! Mode « captures d'écran » : `EUREKA_CAPTURE=dossier cargo run -p eureka-2048`
//! enregistre l'accueil, puis une partie jouée par le pilote automatique
//! jusqu'à la fin, et quitte.
use bevy::prelude::*;
use bevy::render::view::screenshot::{Screenshot, save_to_disk};

use crate::GameState;
use crate::play::Game;

const SHOTS: [(&str, GameState, f32); 4] = [
    ("1-menu", GameState::Menu, 4.0),
    ("2-debut", GameState::Playing, 4.0),
    ("3-milieu", GameState::Playing, 14.0),
    ("4-fin", GameState::Over, 1.5),
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
        if let Ok(dir) = std::env::var("EUREKA_CAPTURE") {
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
    mut next: ResMut<NextState<GameState>>,
    mut exit: MessageWriter<AppExit>,
) {
    let now = time.elapsed_secs();
    game.autopilot = true;
    let Some(&(name, wanted, delay)) = SHOTS.get(cap.index) else {
        if now - cap.since > 0.5 {
            exit.write(AppExit::Success);
        }
        return;
    };
    if *state.get() != wanted {
        cap.since = now;
        if *state.get() == GameState::Menu && cap.index > 0 {
            next.set(GameState::Playing);
        }
        if *state.get() == GameState::Won {
            next.set(GameState::Playing); // on continue après 2048
        }
        return;
    }
    let elapsed = now - cap.since;
    if !cap.shot && elapsed >= delay {
        cap.shot = true;
        commands.spawn(Screenshot::primary_window()).observe(save_to_disk(format!("{}/{name}.png", cap.dir)));
    } else if cap.shot && elapsed >= delay + 0.4 {
        cap.index += 1;
        cap.since = if cap.index < SHOTS.len() && SHOTS[cap.index].1 == wanted { cap.since } else { now };
        cap.shot = false;
    }
}
