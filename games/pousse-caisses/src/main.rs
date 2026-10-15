// Pousse-caisses — Commit & Conquer. Jeu libre sous licence GPL-3.0-or-later.
//
// Le crapaud range les caisses sur les cibles, niveau après niveau.
mod audio;
mod board;
mod capture;
mod level;
mod play;
mod ui;

use bevy::prelude::*;
use game_common::LowPolyStylePlugin;

#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    Menu,
    Levels,
    Playing,
    Won,
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Pousse-caisses — Commit & Conquer".into(),
                        ..default()
                    }),
                    ..default()
                })
                .set(game_common::shared_assets()),
        )
        .init_state::<GameState>()
        .add_plugins((LowPolyStylePlugin, play::PlayPlugin, ui::UiPlugin, audio::AudioPlugin, capture::CapturePlugin))
        .run();
}
