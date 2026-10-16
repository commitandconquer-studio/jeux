// Tour de blocs — Commit & Conquer. Jeu libre sous licence GPL-3.0-or-later.
//
// Empilez les blocs le plus haut possible : ce qui dépasse est coupé.
mod audio;
mod capture;
mod tower;
mod ui;

use bevy::prelude::*;
use game_common::universe::Universe;
use game_common::{Rng, Score, StudioPlugin};

#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    Menu,
    Playing,
    GameOver,
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(game_common::fenetre("Tour de blocs — Commit & Conquer")),
                    ..default()
                })
                .set(game_common::shared_assets()),
        )
        .insert_resource(Rng::from_time())
        .init_resource::<Score>()
        .init_state::<GameState>()
        .add_plugins(StudioPlugin::new(Universe::Croa, game_common::ambiance::AmbiancePlugin::exterieur()))
        .add_plugins((tower::TowerPlugin, ui::UiPlugin, audio::AudioPlugin, capture::CapturePlugin))
        .add_systems(Update, game_common::langue::rebatir_menu(GameState::Menu))
        .run();
}
