// 12 Parsecs — univers Nova. Commit & Conquer, jeu libre (GPL-3.0-or-later).
//
// Traverser les 12 parsecs d'une ceinture d'astéroïdes, aux commandes d'un
// petit vaisseau, avec le pilote et Boulon qui commentent. On commence avec
// un vaisseau ; les autres se gagnent avec le total des points cumulés.
// Cahier des charges : docs/jeux/12-parsecs.md.
mod audio;
mod capture;
mod duo;
mod hangar;
mod partie;
mod reglages;
mod ui;
mod vaisseaux;

use bevy::prelude::*;
use game_common::StudioPlugin;
use game_common::ambiance::AmbiancePlugin;
use game_common::space::{SpaceBackdropPlugin, SpaceView};
use game_common::universe::Universe;

#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    Menu,
    Hangar,
    Reglages,
    Playing,
    Over,
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin { primary_window: Some(game_common::fenetre("12 Parsecs — Commit & Conquer")), ..default() })
                .set(game_common::shared_assets()),
        )
        .init_state::<GameState>()
        // Tout le commun ; dans l'espace, l'ambiance ne garde que les fêtes.
        .add_plugins(StudioPlugin::new(Universe::Nova, AmbiancePlugin::espace()))
        // Le fond étoilé de l'univers Nova, qui défile vers le bas.
        .add_plugins(SpaceBackdropPlugin)
        .insert_resource(SpaceView { center: Vec3::ZERO, size: Vec2::new(64.0, 44.0) })
        .insert_resource(reglages::Reglages::charger())
        .insert_resource(vaisseaux::Progression::charger())
        .add_plugins((partie::PartiePlugin, duo::DuoPlugin, hangar::HangarPlugin, ui::UiPlugin, audio::AudioPlugin, capture::CapturePlugin))
        .add_systems(Update, game_common::langue::rebatir_menu(GameState::Menu))
        .run();
}
