//! Mode « captures d'écran » : `TOUR_CAPTURE=dossier cargo run -p tour`
//! enregistre l'accueil puis une partie jouée en pilote automatique : d'abord
//! des « Parfait », puis des blocs décalés (coupés) jusqu'à la chute.
use bevy::prelude::*;
use bevy::render::view::screenshot::{Screenshot, save_to_disk};

use crate::GameState;
use crate::tower::Autopilot;

/// (nom, état voulu, secondes avant la capture, écart visé par le pilote)
const SHOTS: [(&str, GameState, f32, Option<f32>); 4] = [
    ("1-menu", GameState::Menu, 4.0, None),
    ("2-parfaits", GameState::Playing, 8.0, Some(0.0)),
    ("3-coupes", GameState::Playing, 6.0, Some(0.35)),
    ("4-fin", GameState::GameOver, 3.0, Some(0.35)),
];

#[derive(Resource)]
struct Capture {
    dir: String,
    index: usize,
    since: f32,
    started: bool,
    shot: bool,
}

pub struct CapturePlugin;

impl Plugin for CapturePlugin {
    fn build(&self, app: &mut App) {
        if let Ok(dir) = std::env::var("TOUR_CAPTURE") {
            app.insert_resource(Capture { dir, index: 0, since: 0.0, started: false, shot: false })
                .add_systems(Update, run_capture);
        }
    }
}

fn run_capture(
    time: Res<Time>,
    mut commands: Commands,
    mut cap: ResMut<Capture>,
    mut autopilot: ResMut<Autopilot>,
    state: Res<State<GameState>>,
    mut next: ResMut<NextState<GameState>>,
    mut exit: MessageWriter<AppExit>,
) {
    let now = time.elapsed_secs();
    let Some(&(name, wanted, delay, target)) = SHOTS.get(cap.index) else {
        if now - cap.since > 0.5 {
            exit.write(AppExit::Success);
        }
        return;
    };
    autopilot.0 = target;
    let elapsed = now - cap.since;
    if !cap.started {
        cap.started = true;
        // La fin de partie arrive toute seule (le pilote finit par rater).
        if *state.get() != wanted && wanted != GameState::GameOver {
            next.set(wanted);
        }
    } else if *state.get() != wanted {
        // Pas encore dans l'état voulu (la fin de partie) : le délai part de là.
        cap.since = now;
    } else if !cap.shot && elapsed >= delay {
        cap.shot = true;
        let path = format!("{}/{name}.png", cap.dir);
        commands.spawn(Screenshot::primary_window()).observe(save_to_disk(path));
    } else if cap.shot && elapsed >= delay + 0.4 {
        cap.index += 1;
        cap.since = now;
        cap.started = false;
        cap.shot = false;
    }
}
