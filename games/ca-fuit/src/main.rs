// Ça fuit ! — un jeu de pose de tuyaux de l'univers Croâ & Cie (voir docs/jeux/ca-fuit.md).
// Jeu libre sous licence GPL-3.0-or-later.
//
// Une fuite au milieu d'une pâture ; on y raccorde des tuyaux de cuivre tirés
// d'un paquet de 70, jusqu'à ce que plus rien ne fuie… ou qu'il n'y ait plus
// rien à poser. Les règles sont dans `crates/fuite-core`. Le jeu est en 2D,
// direction artistique « La plomberie de poche » (docs/jeux/da/ca-fuit/) :
// ses sprites sont dessinés par `tools/sprites-ca-fuit.py`.
mod audio;
mod capture;
mod fps;
mod jeu;
mod ui;

use bevy::prelude::*;
use bevy::render::RenderPlugin;
use bevy::render::settings::{PowerPreference, RenderCreation, WgpuSettings};
use bevy::window::{MonitorSelection, WindowMode};
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
    // La carte graphique : `CA_FUIT_GPU=intel` (ou `integree`) pour la puce
    // intégrée, `nvidia` (ou `dediee`) pour la carte dédiée ; sans rien, la
    // plus puissante. `WGPU_ADAPTER_NAME` et `WGPU_POWER_PREF` marchent aussi.
    let mut wgpu = WgpuSettings::default();
    match std::env::var("CA_FUIT_GPU").unwrap_or_default().to_lowercase().as_str() {
        "intel" | "integree" | "intégrée" => {
            wgpu.power_preference = PowerPreference::LowPower;
            wgpu.adapter_name = Some("intel".into());
        }
        "nvidia" | "dediee" | "dédiée" | "rtx" => {
            wgpu.power_preference = PowerPreference::HighPerformance;
            wgpu.adapter_name = Some("nvidia".into());
        }
        _ => {}
    }
    // Le plein écran se mémorise (F11 ou le bouton du menu).
    let mut fenetre = game_common::fenetre("Ça fuit ! — Commit & Conquer");
    if game_common::save::load("ca-fuit", "plein-ecran").as_deref() == Some("1") {
        fenetre.mode = WindowMode::BorderlessFullscreen(MonitorSelection::Current);
    }
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin { primary_window: Some(fenetre), ..default() })
                .set(RenderPlugin { render_creation: RenderCreation::Automatic(Box::new(wgpu)), ..default() })
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
