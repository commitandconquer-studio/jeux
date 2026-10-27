// Snake — Commit & Conquer. Jeu libre sous licence GPL-3.0-or-later.
mod audio;
mod board;
mod capture;
mod game;
mod people;
mod progress;
mod settings;
mod ui;
mod world;

use std::f32::consts::{PI, TAU};

use bevy::prelude::*;
use game_common::universe::Universe as Univers;
use game_common::{ISO_OFFSET, Rng, Score, StudioPlugin, iso_camera, iso_projection};

use settings::Settings;

#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    Menu,
    Options,
    Playing,
    GameOver,
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(game_common::fenetre("Snake — Commit & Conquer")),
                    ..default()
                })
                .set(game_common::shared_assets()),
        )
        .init_state::<GameState>()
        .add_plugins(StudioPlugin::new(Univers::Croa, game_common::ambiance::AmbiancePlugin::exterieur()))
        .add_plugins((game::GamePlugin, ui::UiPlugin, audio::AudioPlugin, capture::CapturePlugin))
        .add_systems(Update, game_common::langue::rebatir_menu(GameState::Menu))
        .insert_resource(Rng::from_time())
        .init_resource::<Settings>()
        .init_resource::<Score>()
        .add_systems(Startup, |mut commands: Commands| {
            commands.spawn(iso_camera(Vec3::ZERO, 24.0, 18.0));
        })
        .add_systems(Update, (world::rebuild_world, world::animate_bobs, move_camera))
        .run();
}

/// Dans les menus la caméra tourne lentement autour du plateau ;
/// en jeu elle revient à sa position fixe.
fn move_camera(
    time: Res<Time>,
    state: Res<State<GameState>>,
    settings: Res<Settings>,
    mut angle: Local<f32>,
    mut cam: Query<(&mut Transform, &mut Projection), With<Camera3d>>,
) {
    let dt = time.delta_secs();
    if matches!(state.get(), GameState::Menu | GameState::Options) {
        *angle = (*angle + dt * 0.12) % TAU;
    } else {
        let a = (*angle + PI).rem_euclid(TAU) - PI;
        *angle = a * (1.0 - (dt * 4.0).min(1.0));
    }
    let g = settings.grid() as f32;
    for (mut tf, mut proj) in &mut cam {
        *tf = Transform::from_translation(Quat::from_rotation_y(*angle) * ISO_OFFSET).looking_at(Vec3::ZERO, Vec3::Y);
        if settings.is_changed() {
            *proj = iso_projection(g * 1.7, g * 1.3);
        }
    }
}
