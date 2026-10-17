//! Mode capture : `CA_FUIT_CAPTURE=dossier cargo run -p ca-fuit` enregistre
//! le menu, une partie en cours et le bilan, puis quitte. Pendant ce temps
//! le jeu ignore le clavier et la souris (la fenêtre prend le focus : ce
//! qu'on tape à côté arriverait dans le jeu).
use bevy::prelude::*;
use bevy::render::view::screenshot::{Screenshot, save_to_disk};

use crate::GameState;
use crate::jeu::{Jeu, jouer_au_hasard};

#[derive(Clone, Copy, PartialEq)]
enum Etape {
    Menu,
    Partie,
    Fin,
}

const CLICHES: [(&str, Etape, f32); 3] = [("1-menu", Etape::Menu, 4.0), ("2-partie", Etape::Partie, 3.5), ("3-bilan", Etape::Fin, 3.0)];

/// Présente seulement en mode capture : les commandes du joueur sont ignorées.
#[derive(Resource)]
pub struct Capture {
    dossier: String,
    i: usize,
    depuis: f32,
    prete: bool,
    pris: bool,
}

pub struct CapturePlugin;

impl Plugin for CapturePlugin {
    fn build(&self, app: &mut App) {
        if let Ok(dossier) = std::env::var("CA_FUIT_CAPTURE") {
            app.insert_resource(Capture { dossier, i: 0, depuis: 0.0, prete: false, pris: false }).add_systems(Update, capturer);
        }
    }
}

fn capturer(
    time: Res<Time>,
    mut commands: Commands,
    mut cap: ResMut<Capture>,
    state: Res<State<GameState>>,
    mut next: ResMut<NextState<GameState>>,
    mut jeu: ResMut<Jeu>,
    mut exit: MessageWriter<AppExit>,
) {
    let t = time.elapsed_secs();
    let Some(&(nom, etape, delai)) = CLICHES.get(cap.i) else {
        exit.write(AppExit::Success);
        return;
    };
    if !cap.prete {
        match etape {
            Etape::Menu => {
                if *state.get() != GameState::Menu {
                    next.set(GameState::Menu);
                    return;
                }
            }
            Etape::Partie => {
                if *state.get() != GameState::Playing {
                    next.set(GameState::Playing);
                    return;
                }
                if jeu.partie.posees == 0 {
                    // Une partie à graine fixe, 14 cartes posées, l'eau déjà arrivée.
                    *jeu = Jeu::nouveau(2026);
                    let mut rng = fuite_core::Rng::new(7);
                    jouer_au_hasard(&mut jeu, 14, &mut rng, t, true);
                    jeu.curseur = Some((0, 2));
                    return;
                }
            }
            Etape::Fin => {
                if *state.get() == GameState::Playing && jeu.finie_depuis.is_none() {
                    let mut rng = fuite_core::Rng::new(7);
                    jouer_au_hasard(&mut jeu, 80, &mut rng, t, false);
                    return;
                }
                if *state.get() != GameState::Fin {
                    return;
                }
            }
        }
        cap.prete = true;
        cap.depuis = t;
        return;
    }
    if !cap.pris && t - cap.depuis >= delai {
        cap.pris = true;
        commands.spawn(Screenshot::primary_window()).observe(save_to_disk(format!("{}/{nom}.png", cap.dossier)));
    } else if cap.pris && t - cap.depuis >= delai + 0.5 {
        cap.i += 1;
        cap.pris = false;
        cap.prete = false;
        cap.depuis = t;
    }
}
