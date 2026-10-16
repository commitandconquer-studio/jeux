// Lanceur — installe, met à jour et lance les jeux Commit & Conquer.
// Jeu libre sous licence GPL-3.0-or-later. Voir docs/SQUELETTE.md.
//
// Les jeux viennent des « Releases » GitHub du studio (archives fabriquées par
// .github/workflows/versions.yml) et s'installent sans droits administrateur
// dans ~/.local/share/commit-and-conquer, avec une entrée dans le menu
// d'applications.
mod capture;
mod catalogue;
mod installation;
mod ui;

use bevy::prelude::*;
use game_common::StudioPlugin;
use game_common::ambiance::AmbiancePlugin;
use game_common::universe::Universe;

#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    Menu,
}

fn main() {
    // Lancé depuis l'archive téléchargée : il s'installe d'abord dans le menu.
    if std::env::var_os("LANCEUR_CAPTURE").is_none() {
        installation::installer_le_lanceur();
    }
    let partage = installation::Partage::default();
    installation::relire_installes(&partage);
    installation::chercher_catalogue(&partage);

    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin { primary_window: Some(game_common::fenetre("Commit & Conquer")), ..default() })
                .set(game_common::shared_assets()),
        )
        .init_state::<GameState>()
        // Le style de Nova (police, panneaux sombres), recoloré aux couleurs du studio par `ui`.
        .add_plugins(StudioPlugin::new(Universe::Nova, AmbiancePlugin::espace()))
        .insert_resource(partage)
        .add_plugins((ui::UiPlugin, capture::CapturePlugin))
        .add_systems(Update, game_common::langue::rebatir_menu(GameState::Menu))
        .run();
}
