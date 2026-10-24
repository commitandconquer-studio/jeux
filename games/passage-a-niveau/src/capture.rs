//! Mode capture : `PASSAGE_A_NIVEAU_CAPTURE=dossier cargo run -p passage-a-niveau` enregistre le
//! menu puis une partie, et quitte (pour vérifier le rendu sans y jouer).
use bevy::prelude::*;
use bevy::render::view::screenshot::{Screenshot, save_to_disk};

use crate::GameState;
use crate::jeu::Partie;
use passage_core::N;

const CLICHES: [(&str, GameState, f32); 3] = [("1-menu", GameState::Menu, 4.0), ("2-partie", GameState::Playing, 2.0), ("3-fin", GameState::Fini, 3.0)];

#[derive(Resource)]
struct Capture {
    dossier: String,
    i: usize,
    depuis: f32,
    pris: bool,
}

pub struct CapturePlugin;

impl Plugin for CapturePlugin {
    fn build(&self, app: &mut App) {
        if let Ok(dossier) = std::env::var("PASSAGE_A_NIVEAU_CAPTURE") {
            app.insert_resource(Capture { dossier, i: 0, depuis: 0.0, pris: false }).add_systems(Update, capturer);
        }
    }
}

fn capturer(
    time: Res<Time>,
    mut commands: Commands,
    mut cap: ResMut<Capture>,
    mut partie: ResMut<Partie>,
    state: Res<State<GameState>>,
    mut next: ResMut<NextState<GameState>>,
    mut exit: MessageWriter<AppExit>,
) {
    let t = time.elapsed_secs();
    let Some(&(nom, etat, delai)) = CLICHES.get(cap.i) else {
        exit.write(AppExit::Success);
        return;
    };
    if nom == "3-fin" && *state.get() != etat {
        // Pour voir rouler le convoi : une grille bien raccordée, posée d'office.
        partie.jeu.board = grille_d_exemple();
        partie.jeu.finish();
        partie.dessinee = false;
    }
    if *state.get() != etat {
        next.set(etat);
        cap.depuis = t;
        return;
    }
    // Pour que la capture montre une table bien remplie : des cartes posées sans réfléchir.
    let voulues = match nom {
        "2-partie" => 28,
        _ => 0,
    };
    while partie.jeu.played() < voulues {
        let k = partie.jeu.played();
        let rot = (k % 4) as u8;
        // La première case libre en sautant de 11 en 11 (les départs sont déjà pris).
        let Some(cell) = (0..N * N).map(|j| (k * 11 + j) % (N * N)).find(|&i| partie.jeu.board.cells[i].is_none()) else { break };
        if !partie.jeu.play(cell, rot) {
            break;
        }
        partie.dessinee = false;
    }
    if !cap.pris && t - cap.depuis >= delai {
        cap.pris = true;
        commands.spawn(Screenshot::primary_window()).observe(save_to_disk(format!("{}/{nom}.png", cap.dossier)));
    } else if cap.pris && t - cap.depuis >= delai + 0.5 {
        cap.i += 1;
        cap.pris = false;
        cap.depuis = t;
    }
}

/// Une grille où le rail relie les deux gares de départ, la route les deux
/// parkings, et les deux se croisent dans une gare routière à 4 entrées.
fn grille_d_exemple() -> passage_core::Board {
    use passage_core::{Board, Card, Piece};
    let mut board = Board::default();
    for (r, c, piece, rot) in [
        (1, 1, Piece::RailDroit, 1),
        (1, 2, Piece::RailDroit, 1),
        (1, 3, Piece::RailCourbe, 2),
        (2, 3, Piece::RailDroit, 0),
        (4, 3, Piece::RailDroit, 0),
        (5, 3, Piece::RailCourbe, 0),
        (5, 4, Piece::RailDroit, 1),
        (5, 5, Piece::RailDroit, 1),
        (1, 5, Piece::RouteDroite, 0),
        (2, 5, Piece::RouteDroite, 0),
        (3, 5, Piece::RouteCourbe, 3),
        (3, 4, Piece::RouteDroite, 1),
        (3, 2, Piece::RouteDroite, 1),
        (3, 1, Piece::RouteCourbe, 1),
        (4, 1, Piece::RouteDroite, 0),
        (5, 1, Piece::RouteDroite, 0),
        (3, 3, Piece::Gare4, 0),
    ] {
        board.place(r * N + c, Card { piece, rot });
    }
    board
}
