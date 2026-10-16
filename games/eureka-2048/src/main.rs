// 2048 — univers Eurêka. Commit & Conquer. Jeu libre sous licence GPL-3.0-or-later.
//
// Faites glisser les tuiles : deux nombres égaux fusionnent et doublent.
mod audio;
mod capture;
mod desk;
mod grid;
mod play;
mod ui;

use bevy::prelude::*;
use game_common::universe::Universe;
use game_common::{Rng, StudioPlugin};

#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    Menu,
    Playing,
    Won,
    Over,
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(game_common::fenetre("2048 — Eurêka")),
                    ..default()
                })
                .set(game_common::shared_assets()),
        )
        .insert_resource(Rng::from_time())
        .init_state::<GameState>()
        .add_plugins(StudioPlugin::new(Universe::Eureka, game_common::ambiance::AmbiancePlugin::interieur()))
        .add_plugins((desk::DeskPlugin, play::PlayPlugin, ui::UiPlugin, audio::AudioPlugin, capture::CapturePlugin))
        .add_systems(Update, game_common::langue::rebatir_menu(GameState::Menu))
        .run();
}
