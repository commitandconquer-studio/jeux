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
use game_common::StudioPlugin;
use game_common::ambiance::AmbiancePlugin;
use game_common::universe::Universe;

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
                    primary_window: Some(game_common::fenetre("Pousse-caisses — Commit & Conquer")),
                    ..default()
                })
                .set(game_common::shared_assets()),
        )
        .init_state::<GameState>()
        .add_plugins(StudioPlugin::new(Universe::Croa, AmbiancePlugin::exterieur()))
        .add_plugins((play::PlayPlugin, ui::UiPlugin, audio::AudioPlugin, capture::CapturePlugin))
        .add_systems(Update, game_common::langue::rebatir_menu(GameState::Menu))
        .run();
}
