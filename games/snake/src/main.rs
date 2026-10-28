// Snake — Commit & Conquer. Jeu libre sous licence GPL-3.0-or-later.
mod audio;
mod board;
mod capture;
mod dress;
pub mod game;
mod layout;
mod levels;
mod miniature;
mod people;
mod progress;
mod settings;
mod ui;
mod voyage;
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
    Voyage,
    Playing,
    GameOver,
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(game_common::fenetre("À la queue leu leu — Commit & Conquer")),
                    ..default()
                })
                .set(game_common::shared_assets()),
        )
        .init_state::<GameState>()
        .add_plugins(StudioPlugin::new(Univers::Croa, game_common::ambiance::AmbiancePlugin::exterieur()))
        .add_plugins(miniature::MiniaturePlugin)
        .add_plugins((progress::ProgressPlugin, voyage::VoyagePlugin, game::GamePlugin, ui::UiPlugin, audio::AudioPlugin, capture::CapturePlugin))
        .add_systems(Update, game_common::langue::rebatir_menu(GameState::Menu))
        .insert_resource(Rng::from_time())
        .insert_resource(bevy::light::DirectionalLightShadowMap { size: 2048 })
        .init_resource::<Settings>()
        .init_resource::<layout::Layout>()
        .init_resource::<Score>()
        .add_systems(Startup, |mut commands: Commands| {
            let mut cam = commands.spawn((iso_camera(Vec3::ZERO, 24.0, 18.0), bevy::core_pipeline::tonemapping::Tonemapping::AcesFitted));
            if miniature::actif() {
                cam.insert(miniature::Miniature::default());
            }
        })
        .add_systems(Update, (world::rebuild_world, move_camera))
        .run();
}

/// Dans les menus la caméra tourne lentement autour du plateau ;
/// en jeu elle revient à sa position fixe.
fn move_camera(
    time: Res<Time>,
    state: Res<State<GameState>>,
    layout: Res<layout::Layout>,
    focus: Res<game::Focus>,
    mut angle: Local<f32>,
    mut at: Local<Vec3>,
    mut zoomed: Local<bool>,
    mut cam: Query<(&mut Transform, &mut Projection), With<Camera3d>>,
) {
    let dt = time.delta_secs();
    if matches!(state.get(), GameState::Menu | GameState::Options | GameState::Voyage) {
        *angle = (*angle + dt * 0.12) % TAU;
    } else {
        let a = (*angle + PI).rem_euclid(TAU) - PI;
        *angle = a * (1.0 - (dt * 4.0).min(1.0));
    }
    let (w, h) = (layout.w as f32, layout.h as f32);
    // Grande carte : en jeu, la caméra zoome et suit la tête ; dans les menus, elle survole la ville.
    let following = layout.follow && matches!(state.get(), GameState::Playing | GameState::GameOver);
    let goal = if following { Vec3::new(focus.0.x, 0.0, focus.0.y) } else { Vec3::ZERO };
    *at = if layout.is_changed() { goal } else { at.lerp(goal, (dt * 10.0).min(1.0)) };
    for (mut tf, mut proj) in &mut cam {
        *tf = Transform::from_translation(*at + Quat::from_rotation_y(*angle) * ISO_OFFSET).looking_at(*at, Vec3::Y);
        if layout.is_changed() || following != (*zoomed) {
            *zoomed = following;
            *proj = if following { iso_projection(20.0, 15.0) } else if layout.follow { iso_projection(w * 0.75, h * 0.75) } else { iso_projection(w * 1.2 + h * 0.5, h * 0.8 + w * 0.5) };
        }
    }
}
