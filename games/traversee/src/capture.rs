//! Mode « captures d'écran » : `TRAVERSEE_CAPTURE=dossier cargo run -p traversee`
//! enregistre l'accueil, le départ, puis une partie jouée en pilote automatique
//! (le crapaud saute tout droit), et quitte.
use bevy::prelude::*;
use bevy::render::view::screenshot::{Screenshot, save_to_disk};

use crate::GameState;
use crate::extra::{Extra, spawn_fly};
use crate::lanes::Lane;
use crate::models::Kit;
use crate::frog::{Cause, Death, Frog, die};

/// (nom, état, secondes avant la capture, pilote automatique)
const SHOTS: [(&str, GameState, f32, bool); 13] = [
    ("1-menu", GameState::Menu, 4.0, false),
    ("2-depart", GameState::Playing, 1.0, false),
    ("3-en-route", GameState::Playing, 2.5, true),
    ("4-plus-loin", GameState::Playing, 2.5, true),
    // Dying : on joue tout droit jusqu'à la mort, puis on photographie l'effet.
    ("5-ecrase-impact", GameState::Dying, 0.08, true),
    ("6-ecrase-vol", GameState::Dying, 0.45, true),
    ("7-noye", GameState::Dying, 0.25, true),
    // Une mouche posée sur la 3e case : le crapaud la gobe en avançant.
    ("8-mouche", GameState::Playing, 1.2, true),
    ("9-garage", GameState::Garage, 1.0, false),
    ("10-personnages", GameState::Garage, 1.2, false),
    ("11-accessoires", GameState::Garage, 1.2, false),
    ("12-mondes", GameState::Garage, 1.2, false),
    ("13-en-costume", GameState::Playing, 1.0, true),
];

#[derive(Resource)]
struct Capture {
    dir: String,
    index: usize,
    since: f32,
    started: bool,
    shot: bool,
    last_hop: f32,
    fly_done: bool,
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
            app.insert_resource(Capture { dir, index: 0, since: 0.0, started: false, shot: false, last_hop: 0.0, fly_done: false })
                .add_systems(Update, run_capture);
        }
    }
}

fn run_capture(
    time: Res<Time<Real>>,
    mut commands: Commands,
    mut cap: ResMut<Capture>,
    mut frog: ResMut<Frog>,
    mut death: ResMut<Death>,
    lanes: Query<(Entity, &Lane)>,
    mut view: ResMut<crate::garage::View>,
    mut selection: ResMut<crate::characters::Selection>,
    kit: Res<Kit>,
    extra: Res<Extra>,
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
    if *state.get() == GameState::Dying && wanted != GameState::Dying {
        cap.since = now;
        return; // on laisse la mort se terminer
    }
    if wanted == GameState::Dying && *state.get() != GameState::Dying && !cap.shot {
        // En attendant la mort : on (re)lance la partie et on saute.
        cap.since = now;
        if matches!(state.get(), GameState::Menu | GameState::GameOver) {
            next.set(GameState::Playing);
        }
        if *state.get() == GameState::Playing && now - cap.last_hop > 0.3 {
            cap.last_hop = now;
            frog.queued = Some(IVec2::Y);
        }
        // Mort forcée, pour photographier chaque effet : écrasé, puis noyé.
        if *state.get() == GameState::Playing && frog.row >= 2 {
            let cause = if cap.index < 6 { Cause::Squashed } else { Cause::Drowned };
            die(cause, &mut death, &mut next);
            death.fling = Vec3::new(3.6, 0.0, -0.8);
            death.fling_height = 3.2;
        }
        cap.started = true; // on a vu la partie vivante : la prochaine mort est la bonne
        return;
    }
    if wanted == GameState::Dying && !cap.started && !cap.shot {
        cap.since = now;
        return; // encore la mort précédente
    }
    if cap.index == 7 && !cap.fly_done && *state.get() == GameState::Playing && frog.row <= 1 && now - cap.since > 0.25 {
        if let Some((lane, _)) = lanes.iter().find(|(_, l)| l.row == 2) {
            spawn_fly(&mut commands, &kit, &extra, lane, 2, 0, false, 0.0);
            cap.fly_done = true;
        }
    }
    let elapsed = now - cap.since;
    if !cap.started {
        cap.started = true;
        if cap.index == 9 {
            view.tab = crate::garage::Tab::Characters;
            view.dirty = true;
        }
        if cap.index == 10 {
            view.tab = crate::garage::Tab::Costumes;
            view.dirty = true;
            selection.character = crate::characters::Character::Rabbit;
        }
        if cap.index == 11 {
            view.tab = crate::garage::Tab::Worlds;
            view.dirty = true;
        }
        if cap.index == 12 {
            selection.character = crate::characters::Character::Raccoon;
            selection.costume = crate::characters::Costume::Crown;
        }
        // Ne relance pas la partie si on y est déjà (pour continuer à avancer).
        if *state.get() != wanted && wanted != GameState::Dying {
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
    mut windows: Query<&mut Window>,
) {
    let now = time.elapsed_secs();
    for mut w in &mut windows {
        // Sans synchronisation verticale : on voit la vraie vitesse, pas le plafond de 60.
        w.present_mode = bevy::window::PresentMode::AutoNoVsync;
    }
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
