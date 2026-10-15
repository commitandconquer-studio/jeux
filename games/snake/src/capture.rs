//! Mode « captures d'écran » : `SNAKE_CAPTURE=dossier cargo run -p snake`
//! parcourt les écrans et univers, enregistre des PNG puis quitte.
//! Pratique pour les pages itch.io / magasins d'applications.
use bevy::prelude::*;
use bevy::render::view::screenshot::{Screenshot, save_to_disk};

use crate::GameState;
use crate::settings::Settings;
use crate::world::Universe;

const SHOTS: [(&str, GameState, Universe); 7] = [
    ("1-menu-prairie", GameState::Menu, Universe::Prairie),
    ("2-jeu-prairie", GameState::Playing, Universe::Prairie),
    ("3-options-desert", GameState::Options, Universe::Desert),
    ("4-jeu-desert", GameState::Playing, Universe::Desert),
    ("5-jeu-banquise", GameState::Playing, Universe::Banquise),
    ("6-jeu-ocean", GameState::Playing, Universe::Ocean),
    ("7-jeu-volcan", GameState::Playing, Universe::Volcan),
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
        if let Ok(dir) = std::env::var("SNAKE_CAPTURE") {
            app.insert_resource(Capture { dir, index: 0, since: 0.0, started: false, shot: false })
                .add_systems(Update, run_capture);
        }
    }
}

fn run_capture(
    time: Res<Time>,
    mut commands: Commands,
    mut cap: ResMut<Capture>,
    mut settings: ResMut<Settings>,
    mut next: ResMut<NextState<GameState>>,
    mut exit: MessageWriter<AppExit>,
) {
    let now = time.elapsed_secs();
    let Some(&(name, state, universe)) = SHOTS.get(cap.index) else {
        if now - cap.since > 0.5 {
            exit.write(AppExit::Success);
        }
        return;
    };
    // Laisse le temps au décor, à la caméra et au serpent de se mettre en place
    // (plus longtemps au démarrage, le temps de compiler les shaders).
    let shoot = if cap.index == 0 { 4.0 } else { 0.8 };
    let elapsed = now - cap.since;
    if !cap.started {
        cap.started = true;
        settings.universe = universe;
        next.set(state);
    } else if !cap.shot && elapsed >= shoot {
        cap.shot = true;
        let path = format!("{}/{name}.png", cap.dir);
        commands.spawn(Screenshot::primary_window()).observe(save_to_disk(path));
    } else if elapsed >= shoot + 0.4 {
        cap.index += 1;
        cap.since = now;
        cap.started = false;
        cap.shot = false;
    }
}
