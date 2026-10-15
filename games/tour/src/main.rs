// Tour de blocs — Commit & Conquer. Jeu libre sous licence GPL-3.0-or-later.
//
// Empilez les blocs le plus haut possible : ce qui dépasse est coupé.
mod audio;
mod capture;
mod tower;
mod ui;

use bevy::prelude::*;
use game_common::{LowPolyStylePlugin, Rng, Score};

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
                    primary_window: Some(Window {
                        title: "Tour de blocs — Commit & Conquer".into(),
                        ..default()
                    }),
                    ..default()
                })
                .set(game_common::shared_assets()),
        )
        .insert_resource(Rng::from_time())
        .init_resource::<Score>()
        .init_state::<GameState>()
        .add_plugins((LowPolyStylePlugin, game_common::splash::SplashPlugin, tower::TowerPlugin, ui::UiPlugin, audio::AudioPlugin, capture::CapturePlugin))
        .run();
}
