//! Mode « captures d'écran » : `CAISSES_CAPTURE=dossier cargo run -p pousse-caisses`
//! enregistre l'accueil, le choix du niveau, un niveau de chaque univers et
//! une victoire (le niveau 1 est joué automatiquement), puis quitte.
use bevy::prelude::*;
use bevy::render::view::screenshot::{Screenshot, save_to_disk};

use crate::GameState;
use crate::play::{Play, Progress, load_level};

/// (nom, état, niveau à charger, coups à jouer, secondes avant la capture)
const SHOTS: [(&str, GameState, Option<usize>, &str, f32); 8] = [
    ("1-menu", GameState::Menu, None, "", 4.0),
    ("2-niveaux", GameState::Levels, None, "", 1.5),
    ("3-prairie", GameState::Playing, Some(0), "", 1.5),
    ("4-reussi", GameState::Won, None, "DDD", 2.0),
    ("5-desert", GameState::Playing, Some(6), "", 1.5),
    ("6-banquise", GameState::Playing, Some(7), "", 1.5),
    ("7-ocean", GameState::Playing, Some(8), "", 1.5),
    ("8-volcan", GameState::Playing, Some(9), "", 1.5),
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
        if let Ok(dir) = std::env::var("CAISSES_CAPTURE") {
            app.insert_resource(Capture { dir, index: 0, since: 0.0, started: false, shot: false })
                .add_systems(Update, run_capture);
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn run_capture(
    time: Res<Time>,
    mut commands: Commands,
    mut cap: ResMut<Capture>,
    mut play: ResMut<Play>,
    progress: Res<Progress>,
    state: Res<State<GameState>>,
    mut next: ResMut<NextState<GameState>>,
    mut exit: MessageWriter<AppExit>,
) {
    let now = time.elapsed_secs();
    let Some(&(name, wanted, level, moves, delay)) = SHOTS.get(cap.index) else {
        if now - cap.since > 0.5 {
            exit.write(AppExit::Success);
        }
        return;
    };
    let elapsed = now - cap.since;
    if !cap.started {
        cap.started = true;
        if let Some(i) = level {
            load_level(&mut play, &progress, i);
        }
        // D = droite, G = gauche, H = haut, B = bas.
        play.script = moves
            .chars()
            .map(|c| match c {
                'D' => IVec2::X,
                'G' => IVec2::NEG_X,
                'H' => IVec2::NEG_Y,
                _ => IVec2::Y,
            })
            .collect();
        if moves.is_empty() && *state.get() != wanted {
            next.set(wanted);
        }
    } else if *state.get() != wanted {
        cap.since = now; // on attend d'y être (la victoire arrive après les coups)
    } else if !cap.shot && elapsed >= delay {
        cap.shot = true;
        commands.spawn(Screenshot::primary_window()).observe(save_to_disk(format!("{}/{name}.png", cap.dir)));
    } else if cap.shot && elapsed >= delay + 0.4 {
        cap.index += 1;
        cap.since = now;
        cap.started = false;
        cap.shot = false;
    }
}
