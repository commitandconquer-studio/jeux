// L'Observatoire — un jeu de logique de l'univers Eurêka (voir docs/jeux/observatoire.md).
// Jeu libre sous licence GPL-3.0-or-later.
//
// Sur la carte du ciel d'un astronome, placez soleils et lunes : autant de
// chaque par rangée et par colonne, jamais trois pareils à la suite, et les
// signes « = » et « × » à respecter.
mod audio;
mod capture;
mod jeu;
mod plateau;
mod ui;

use bevy::prelude::*;
use game_common::StudioPlugin;
use game_common::ambiance::AmbiancePlugin;
use game_common::universe::Universe;

#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    Menu,
    Livre,
    Playing,
    Gagne,
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin { primary_window: Some(game_common::fenetre("L'Observatoire — Commit & Conquer")), ..default() })
                .set(game_common::shared_assets()),
        )
        .init_state::<GameState>()
        // Tout le commun : style, démarrage, univers, son, langues, Options, ambiance.
        .add_plugins(StudioPlugin::new(Universe::Eureka, AmbiancePlugin::interieur()))
        .add_plugins((jeu::JeuPlugin, ui::UiPlugin, audio::AudioPlugin, capture::CapturePlugin))
        // Langue changée dans les Options : le menu est rebâti dans la nouvelle langue.
        .add_systems(Update, game_common::langue::rebatir_menu(GameState::Menu))
        .run();
}
