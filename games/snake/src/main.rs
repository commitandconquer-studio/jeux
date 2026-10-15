// Snake — Commit & Conquer. Jeu libre sous licence GPL-3.0-or-later.
mod audio;
mod capture;
mod game;
mod settings;
mod ui;
mod world;

use std::f32::consts::{PI, TAU};

use bevy::prelude::*;
use game_common::{ISO_OFFSET, LowPolyStylePlugin, Rng, Score, iso_camera, iso_projection};

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
                    primary_window: Some(Window {
                        title: "Snake — Commit & Conquer".into(),
                        ..default()
                    }),
                    ..default()
                })
                .set(game_common::shared_assets()),
        )
        .add_plugins((LowPolyStylePlugin, game_common::splash::SplashPlugin, game::GamePlugin, ui::UiPlugin, audio::AudioPlugin, capture::CapturePlugin))
        .init_state::<GameState>()
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
