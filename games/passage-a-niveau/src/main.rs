// Passage à niveau — un jeu de cartes solo de l'univers Croâ & Cie
// (voir docs/jeux/passage-a-niveau.md). Jeu libre sous licence GPL-3.0-or-later.
//
// Des cartes de rails et de routes arrivent une à une, toujours les mêmes
// pour un numéro donné ; on les pose sur la grille pour faire le meilleur score.
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
    Playing,
    Fini,
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin { primary_window: Some(game_common::fenetre("Passage à niveau — Commit & Conquer")), ..default() })
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
