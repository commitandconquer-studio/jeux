// Ça fuit ! — un jeu de pose de tuyaux de l'univers Croâ & Cie (voir docs/jeux/ca-fuit.md).
// Jeu libre sous licence GPL-3.0-or-later.
//
// Une fuite au milieu d'une pâture ; on y raccorde des tuyaux de zinc tirés
// d'un paquet de 70, jusqu'à ce que plus rien ne fuie… ou qu'il n'y ait plus
// rien à poser. Les règles sont dans `crates/fuite-core`. Le jeu est en 2D :
// ses sprites sont photographiés depuis des modèles 3D (`sprites.rs`).
mod audio;
mod capture;
mod fps;
mod jeu;
mod modeles;
mod sprites;
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
    // Le générateur de sprites : les modèles 3D photographiés du dessus.
    if let Ok(dossier) = std::env::var("CA_FUIT_SPRITES") {
        sprites::generer(dossier);
        return;
    }
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
        // `CA_FUIT_FPS=1` : affiche les images par seconde dans le terminal.
        .add_plugins(fps::FpsPlugin)
        // Langue changée dans les Options : le menu est rebâti dans la nouvelle langue.
        .add_systems(Update, game_common::langue::rebatir_menu(GameState::Menu))
        .run();
}
