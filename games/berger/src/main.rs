// Le Chien de berger — voir docs/jeux/chien-de-berger.md.
// Jeu libre sous licence GPL-3.0-or-later.
//
// Un chien de berger rassemble un troupeau (des boids qui fuient le chien et
// les loups) et le rentre dans l'enclos, sur des prés qui changent.
mod audio;
mod capture;
mod jeu;
mod ui;
mod vue;

use bevy::prelude::*;
use game_common::StudioPlugin;
use game_common::ambiance::AmbiancePlugin;
use game_common::universe::Universe;

#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    Menu,
    Niveaux,
    Playing,
    Fin,
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin { primary_window: Some(game_common::fenetre("Le Chien de berger — Commit & Conquer")), ..default() })
                .set(game_common::shared_assets()),
        )
        .init_state::<GameState>()
        // Tout le commun : style, démarrage, univers, son, langues, Options, ambiance.
        .add_plugins(StudioPlugin::new(Universe::Croa, AmbiancePlugin::exterieur()))
        .add_plugins((jeu::JeuPlugin, ui::UiPlugin, audio::AudioPlugin, capture::CapturePlugin))
        // Langue changée dans les Options : le menu est rebâti dans la nouvelle langue.
        .add_systems(Update, game_common::langue::rebatir_menu(GameState::Menu))
        .run();
}
