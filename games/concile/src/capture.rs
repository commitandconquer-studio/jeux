//! Mode « captures d'écran » : `CONCILE_CAPTURE=dossier cargo run -p concile`
//! enregistre l'accueil, le Livre, une grille (vide, à moitié résolue en mode
//! Novice, avec un indice), puis la victoire. Avec `CC_DATE=2026-12-25`, on
//! voit les décorations de la fête correspondante.
use bevy::prelude::*;
use bevy::render::view::screenshot::{Screenshot, save_to_disk};

use crate::GameState;
use crate::play::{Book, Kind, Mark, Play, start};

#[derive(Clone, Copy, PartialEq)]
enum Step {
    Menu,
    Book,
    Start,
    Half,
    Hint,
    Won,
}

const SHOTS: [(&str, Step, f32); 6] = [
    ("1-accueil", Step::Menu, 4.0),
    ("2-livre", Step::Book, 1.2),
    ("3-grille", Step::Start, 1.5),
    ("4-moitie", Step::Half, 1.2),
    ("5-indice", Step::Hint, 1.2),
    ("6-victoire", Step::Won, 2.5),
];

#[derive(Resource)]
struct Capture {
    dir: String,
    index: usize,
    since: f32,
    prepared: bool,
    shot: bool,
}

pub struct CapturePlugin;

impl Plugin for CapturePlugin {
    fn build(&self, app: &mut App) {
        if let Ok(dir) = std::env::var("CONCILE_CAPTURE") {
            app.insert_resource(Capture { dir, index: 0, since: 0.0, prepared: false, shot: false }).add_systems(Update, run_capture);
        }
    }
}

fn run_capture(
    time: Res<Time>,
    mut commands: Commands,
    mut cap: ResMut<Capture>,
    book: Res<Book>,
    mut play: ResMut<Play>,
    state: Res<State<GameState>>,
    mut next: ResMut<NextState<GameState>>,
    mut exit: MessageWriter<AppExit>,
) {
    let now = time.elapsed_secs();
    let Some(&(name, step, delay)) = SHOTS.get(cap.index) else {
        if now - cap.since > 0.5 {
            exit.write(AppExit::Success);
        }
        return;
    };
    if !cap.prepared {
        cap.prepared = true;
        cap.since = now;
        let n = play.n();
        match step {
            Step::Menu => {}
            Step::Book => next.set(GameState::Book),
            Step::Start => {
                start(&mut commands, book.0[45].clone(), Kind::Book(45));
                next.set(GameState::Playing);
            }
            Step::Half => {
                // La moitié des évêques de la solution, et quelques croix.
                for r in 0..n / 2 {
                    let c = play.puzzle.solution[r] as usize;
                    play.marks[r * n + c] = Mark::Bishop;
                }
                for i in [(n - 1) * n, (n - 1) * n + 1] {
                    if play.marks[i] == Mark::Empty && !play.puzzle.is_bishop(i / n, i % n) {
                        play.marks[i] = Mark::Cross;
                    }
                }
            }
            Step::Hint => play.compute_hint(),
            Step::Won => {
                for r in 0..n {
                    let c = play.puzzle.solution[r] as usize;
                    play.marks[r * n + c] = Mark::Bishop;
                }
            }
        }
        return;
    }
    let wanted = match step {
        Step::Menu => GameState::Menu,
        Step::Book => GameState::Book,
        Step::Won => GameState::Won,
        _ => GameState::Playing,
    };
    if *state.get() != wanted {
        cap.since = now;
        return;
    }
    let elapsed = now - cap.since;
    if !cap.shot && elapsed >= delay {
        cap.shot = true;
        commands.spawn(Screenshot::primary_window()).observe(save_to_disk(format!("{}/{name}.png", cap.dir)));
    } else if cap.shot && elapsed >= delay + 0.4 {
        cap.index += 1;
        cap.prepared = false;
        cap.shot = false;
    }
}
