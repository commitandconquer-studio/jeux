//! Mode capture : `OBSERVATOIRE_CAPTURE=dossier cargo run -p observatoire`
//! enregistre le menu, le livre, une partie (avec erreur et indice) et la
//! victoire, puis quitte (pour vérifier le rendu sans y jouer).
use bevy::prelude::*;
use bevy::render::view::screenshot::{Screenshot, save_to_disk};
use observatoire_core::{VIDE, oppose};

use crate::GameState;
use crate::jeu::Partie;

#[derive(Clone, Copy, PartialEq)]
enum Etape {
    Menu,
    Livre,
    Partie,
    Victoire,
}

const CLICHES: [(&str, Etape, f32); 4] = [("1-menu", Etape::Menu, 6.0), ("2-livre", Etape::Livre, 2.0), ("3-partie", Etape::Partie, 2.5), ("4-victoire", Etape::Victoire, 2.5)];

#[derive(Resource)]
struct Capture {
    dossier: String,
    i: usize,
    depuis: f32,
    prete: bool,
    pris: bool,
}

pub struct CapturePlugin;

impl Plugin for CapturePlugin {
    fn build(&self, app: &mut App) {
        if let Ok(dossier) = std::env::var("OBSERVATOIRE_CAPTURE") {
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
    mut partie: Option<ResMut<Partie>>,
    mut exit: MessageWriter<AppExit>,
) {
    let t = time.elapsed_secs();
    let Some(&(nom, etape, delai)) = CLICHES.get(cap.i) else {
        exit.write(AppExit::Success);
        return;
    };
    if !cap.prete {
        let voulu = match etape {
            Etape::Menu => GameState::Menu,
            Etape::Livre => GameState::Livre,
            Etape::Partie | Etape::Victoire => GameState::Playing,
        };
        if *state.get() != voulu {
            next.set(voulu);
            return;
        }
        let Some(p) = partie.as_deref_mut() else { return };
        let n = p.n();
        match etape {
            Etape::Partie => {
                // La moitié des cases justes, une fausse (mode Novice : signalée), un indice.
                for i in (0..n * n).filter(|i| i % 2 == 0) {
                    p.cases[i] = p.grille.solution[i];
                }
                if let Some(i) = (0..n * n).find(|&i| p.grille.donnees[i] == VIDE && p.cases[i] != VIDE && i > n) {
                    p.cases[i] = oppose(p.grille.solution[i]);
                }
                p.redessiner();
                let mut essai = p.cases.clone();
                for i in 0..n * n {
                    if essai[i] != p.grille.solution[i] {
                        essai[i] = VIDE;
                    }
                }
                let garde = std::mem::replace(&mut p.cases, essai);
                p.calculer_indice();
                p.cases = garde;
                p.temps = 83.0;
            }
            Etape::Victoire => {
                p.cases = p.grille.solution.clone();
                p.indice = None;
                p.redessiner();
            }
            _ => {}
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
    }
}
