//! Mode « captures d'écran » : `TRAVERSEE_CAPTURE=dossier cargo run -p traversee`
//! enregistre l'accueil, le départ, puis une partie jouée en pilote automatique
//! (le crapaud saute tout droit), et quitte.
use bevy::prelude::*;
use bevy::render::view::screenshot::{Screenshot, save_to_disk};

use crate::GameState;
use crate::frog::Frog;

/// (nom, état, secondes avant la capture, pilote automatique)
const SHOTS: [(&str, GameState, f32, bool); 4] = [
    ("1-menu", GameState::Menu, 4.0, false),
    ("2-depart", GameState::Playing, 1.0, false),
    ("3-en-route", GameState::Playing, 2.5, true),
    ("4-plus-loin", GameState::Playing, 2.5, true),
];

#[derive(Resource)]
struct Capture {
    dir: String,
    index: usize,
    since: f32,
    started: bool,
    shot: bool,
    last_hop: f32,
}

pub struct CapturePlugin;

impl Plugin for CapturePlugin {
    fn build(&self, app: &mut App) {
        if std::env::var("TRAVERSEE_PERF").is_ok() {
            // Mesure : fps et nombre d'entités dans la console, en jouant tout seul.
            use bevy::diagnostic::{EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
            app.add_plugins((
                FrameTimeDiagnosticsPlugin::default(),
                EntityCountDiagnosticsPlugin::default(),
                LogDiagnosticsPlugin::default(),
            ))
            .add_systems(Update, run_perf);
        }
        if let Ok(dir) = std::env::var("TRAVERSEE_CAPTURE") {
            app.insert_resource(Capture { dir, index: 0, since: 0.0, started: false, shot: false, last_hop: 0.0 })
                .add_systems(Update, run_capture);
        }
    }
}

fn run_capture(
    time: Res<Time>,
    mut commands: Commands,
    mut cap: ResMut<Capture>,
    mut frog: ResMut<Frog>,
    state: Res<State<GameState>>,
    mut next: ResMut<NextState<GameState>>,
    mut exit: MessageWriter<AppExit>,
) {
    let now = time.elapsed_secs();
    let Some(&(name, wanted, delay, autopilot)) = SHOTS.get(cap.index) else {
        if now - cap.since > 0.5 {
            exit.write(AppExit::Success);
        }
        return;
    };
    let elapsed = now - cap.since;
    if !cap.started {
        cap.started = true;
        // Ne relance pas la partie si on y est déjà (pour continuer à avancer).
        if *state.get() != wanted {
            next.set(wanted);
        }
    } else if !cap.shot && elapsed >= delay {
        cap.shot = true;
        let path = format!("{}/{name}.png", cap.dir);
        commands.spawn(Screenshot::primary_window()).observe(save_to_disk(path));
    } else if elapsed >= delay + 0.4 {
        cap.index += 1;
        cap.since = now;
        cap.started = false;
        cap.shot = false;
    }
    if autopilot && !cap.shot && *state.get() == GameState::Playing && now - cap.last_hop > 0.45 {
        cap.last_hop = now;
        frog.queued = Some(IVec2::Y);
    }
}

/// Pilote automatique pour `TRAVERSEE_PERF` : lance la partie, saute tout droit,
/// rejoue après chaque mort, et quitte au bout de 25 secondes.
fn run_perf(
    time: Res<Time>,
    mut frog: ResMut<Frog>,
    state: Res<State<GameState>>,
    mut next: ResMut<NextState<GameState>>,
    mut exit: MessageWriter<AppExit>,
    mut last: Local<f32>,
) {
    let now = time.elapsed_secs();
    if now > 25.0 {
        exit.write(AppExit::Success);
    }
    match state.get() {
        GameState::Menu | GameState::GameOver if now > 2.0 => next.set(GameState::Playing),
        GameState::Playing if now - *last > 0.3 => {
            *last = now;
            frog.queued = Some(IVec2::Y);
        }
        _ => {}
    }
}
