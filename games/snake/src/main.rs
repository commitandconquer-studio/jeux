// Snake — Commit & Conquer. Jeu libre sous licence GPL-3.0-or-later.
mod audio;
mod board;
mod capture;
mod dress;
mod grid;
mod game;
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
    // Outil : `SNAKE_EXPORT=dossier` écrit tous les niveaux en cartes retouchables (voir l'éditeur), sans lancer le jeu.
    if let Ok(dir) = std::env::var("SNAKE_EXPORT") {
        levels::export(&dir).expect("export des niveaux");
        println!("niveaux écrits dans {dir}");
        return;
    }
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(game_common::fenetre("Suivez le guide ! — Commit & Conquer")),
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
        .add_systems(Update, (world::rebuild_world, world::animate_traffic, move_camera))
        .run();
}

/// Dans les menus la caméra tourne lentement autour du plateau ;
/// en jeu elle revient à sa position fixe.
fn move_camera(
    time: Res<Time>,
    state: Res<State<GameState>>,
    layout: Res<layout::Layout>,
    mut angle: Local<f32>,
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
    for (mut tf, mut proj) in &mut cam {
        *tf = Transform::from_translation(Quat::from_rotation_y(*angle) * ISO_OFFSET).looking_at(Vec3::ZERO, Vec3::Y);
        if layout.is_changed() {
            *proj = iso_projection(w * 1.2 + h * 0.5, h * 0.8 + w * 0.5);
        }
    }
}
