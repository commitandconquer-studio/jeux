//! Modes « captures d'écran » (puis le jeu quitte) :
//! - `SNAKE_CAPTURE=dossier cargo run -p snake` : les écrans et paysages, tout débloqué.
//! - `SNAKE_PRESSE=dossier cargo run -p snake` : dossier de presse en 1920×1080, sans interface
//!   (sauf le menu), partie figée.
use bevy::prelude::*;
use bevy::render::view::screenshot::{Screenshot, save_to_disk};
use bevy::window::PrimaryWindow;

use crate::GameState;
use crate::game::Demo;
use crate::layout::Shape;
use crate::settings::Settings;
use crate::world::Universe;

struct Shot {
    name: &'static str,
    state: GameState,
    universe: Universe,
    /// File de test de cette longueur (sinon la file de départ).
    long: Option<usize>,
    /// Cases spéciales déjà posées.
    features: bool,
    shape: Shape,
}

const fn shot(name: &'static str, state: GameState, universe: Universe, long: Option<usize>, features: bool) -> Shot {
    Shot { name, state, universe, long, features, shape: Shape::Medium }
}

fn shots(press: bool) -> Vec<Shot> {
    if std::env::var("SNAKE_FORMES").is_ok() {
        return Shape::ALL.iter().map(|s| Shot { name: Box::leak(format!("forme-{s:?}").into_boxed_str()), state: GameState::Playing, universe: Universe::Prairie, long: Some(12), features: true, shape: *s }).collect();
    }
    use GameState::{Menu, Options, Playing};
    use Universe::{Cimetiere, Desert, Ocean, Prairie};
    if press {
        vec![
            shot("1-menu", Menu, Prairie, None, false),
            shot("2-prairie", Playing, Prairie, Some(22), true),
            shot("3-desert", Playing, Desert, Some(22), true),
            shot("4-cimetiere", Playing, Cimetiere, Some(22), true),
            shot("5-ile-des-pirates", Playing, Ocean, Some(22), true),
            shot("6-longue-file", Playing, Prairie, Some(100), false),
            shot("7-cases-speciales", Playing, Prairie, Some(6), true),
        ]
    } else {
        vec![
            shot("1-menu-prairie", Menu, Prairie, None, true),
            shot("2-jeu-prairie", Playing, Prairie, None, true),
            shot("3-options-desert", Options, Desert, None, true),
            shot("4-jeu-desert", Playing, Desert, None, true),
            shot("5-jeu-cimetiere", Playing, Cimetiere, None, true),
            shot("6-jeu-ocean", Playing, Ocean, None, true),
        ]
    }
}

#[derive(Resource)]
struct Capture {
    dir: String,
    shots: Vec<Shot>,
    index: usize,
    since: f32,
    started: bool,
    shot: bool,
}

pub struct CapturePlugin;

impl Plugin for CapturePlugin {
    fn build(&self, app: &mut App) {
        let press = std::env::var("SNAKE_PRESSE").ok();
        if let Some(dir) = press.clone().or_else(|| std::env::var("SNAKE_CAPTURE").ok()).or_else(|| std::env::var("SNAKE_FORMES").ok()) {
            app.insert_resource(Capture { dir, shots: shots(press.is_some()), index: 0, since: 0.0, started: false, shot: false })
                .add_systems(Update, run_capture);
            if press.is_some() {
                app.add_systems(Startup, full_hd);
            }
        }
    }
}

/// Fenêtre en 1920×1080 exactement (échelle 1), pour des images de presse.
fn full_hd(mut windows: Query<&mut Window, With<PrimaryWindow>>) {
    for mut w in &mut windows {
        w.resolution.set_scale_factor_override(Some(1.0));
        w.resolution.set(1920.0, 1080.0);
    }
}

#[allow(clippy::too_many_arguments)]
fn run_capture(
    time: Res<Time>,
    mut commands: Commands,
    mut cap: ResMut<Capture>,
    mut settings: ResMut<Settings>,
    mut demo: ResMut<Demo>,
    mut next: ResMut<NextState<GameState>>,
    mut exit: MessageWriter<AppExit>,
) {
    let now = time.elapsed_secs();
    let Some(s) = cap.shots.get(cap.index) else {
        if now - cap.since > 2.0 {
            exit.write(AppExit::Success);
        }
        return;
    };
    let (name, state, universe, long, features, shape) = (s.name, s.state, s.universe, s.long, s.features, s.shape);
    // Laisse le temps au décor, à la caméra et au serpent de se mettre en place
    // (plus longtemps au démarrage, le temps de compiler les shaders).
    let shoot = if cap.index == 0 { 4.0 } else { 1.5 };
    let elapsed = now - cap.since;
    if !cap.started {
        cap.started = true;
        settings.universe = universe;
        settings.shape = shape;
        demo.long = long;
        demo.features = features;
        next.set(state);
    } else if !cap.shot && elapsed >= shoot {
        cap.shot = true;
        let path = format!("{}/{name}.png", cap.dir);
        commands.spawn(Screenshot::primary_window()).observe(save_to_disk(path));
    } else if elapsed >= shoot + 0.6 {
        cap.index += 1;
        cap.since = now;
        cap.started = false;
        cap.shot = false;
    }
}
