// Ça fuit ! — un jeu de pose de tuyaux de l'univers Croâ & Cie (voir docs/jeux/ca-fuit.md).
// Jeu libre sous licence GPL-3.0-or-later.
//
// Une fuite au milieu d'une pâture ; le crapaud plombier y raccorde des
// tuyaux de zinc tirés d'un paquet de 70, jusqu'à ce que plus rien ne fuie…
// ou qu'il n'ait plus rien à poser. Les règles sont dans `crates/fuite-core`.
mod audio;
mod capture;
mod jeu;
mod modeles;
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
    /// Plus rien à poser : le bilan.
    Fin,
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin { primary_window: Some(game_common::fenetre("Ça fuit ! — Commit & Conquer")), ..default() })
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
