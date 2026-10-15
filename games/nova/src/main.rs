// NOVA — Le Dernier Rempart. Commit & Conquer. Jeu libre sous licence GPL-3.0-or-later.
//
// La colonie de Nova résiste à l'Hégémonie : empilez les blocs d'énergie du
// bouclier ; chaque rangée complète tire le canon à ions sur le croiseur ennemi.
mod audio;
mod board;
mod capture;
mod game;
mod scene;
mod story;
mod ui;

use bevy::prelude::*;
use game_common::{LowPolyStylePlugin, Rng};

#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    Menu,
    Chapters,
    Story,
    Playing,
    Victory,
    Defeat,
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "NOVA — Le Dernier Rempart".into(),
                        ..default()
                    }),
                    ..default()
                })
                .set(game_common::shared_assets()),
        )
        .insert_resource(Rng::from_time())
        .init_state::<GameState>()
        .add_plugins((
            LowPolyStylePlugin,
            game::GamePlugin,
            scene::ScenePlugin,
            ui::UiPlugin,
            audio::AudioPlugin,
            capture::CapturePlugin,
        ))
        .run();
}
