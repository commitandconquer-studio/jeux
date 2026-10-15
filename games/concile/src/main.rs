// Le Concile des Évêques — univers Eurêka. Commit & Conquer.
// Jeu libre sous licence GPL-3.0-or-later.
//
// Placez un évêque par diocèse, par rangée et par colonne, sans que deux
// évêques se touchent (même en diagonale). Les grilles sont dans
// `puzzles.txt` (générées par `cargo run --release -p concile-gen`).
mod audio;
mod board;
mod capture;
mod feasts;
mod play;
mod ui;

use bevy::prelude::*;
use game_common::LowPolyStylePlugin;

#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    Menu,
    Book,
    Playing,
    Won,
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Le Concile des Évêques — Eurêka".into(),
                        ..default()
                    }),
                    ..default()
                })
                .set(game_common::shared_assets()),
        )
        .insert_resource(ClearColor(Color::srgb(0.20, 0.13, 0.08)))
        .init_state::<GameState>()
        .add_plugins((
            LowPolyStylePlugin,
            game_common::splash::SplashPlugin,
            play::PlayPlugin,
            ui::UiPlugin,
            feasts::FeastPlugin,
            audio::AudioPlugin,
            capture::CapturePlugin,
        ))
        .run();
}
