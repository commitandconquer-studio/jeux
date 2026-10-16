// Jeu modèle — le squelette commun d'un jeu Commit & Conquer (voir docs/SQUELETTE.md).
// Jeu libre sous licence GPL-3.0-or-later.
//
// Pour commencer un nouveau jeu : `tools/nouveau-jeu.sh mon-jeu "Mon jeu"`.
// Le crapaud ramasse les pièces de la prairie ; tout le reste est commun.
mod audio;
mod capture;
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
    Playing,
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin { primary_window: Some(game_common::fenetre("Modèle — Commit & Conquer")), ..default() })
                .set(game_common::shared_assets()),
        )
        .init_state::<GameState>()
        // Tout le commun : style, démarrage, univers, son, langues, Options, ambiance.
        .add_plugins(StudioPlugin::new(Universe::Croa, AmbiancePlugin::exterieur()))
        .add_plugins((play::PlayPlugin, ui::UiPlugin, audio::AudioPlugin, capture::CapturePlugin))
        // Langue changée dans les Options : le menu est rebâti dans la nouvelle langue.
        .add_systems(Update, game_common::langue::rebatir_menu(GameState::Menu))
        .run();
}
