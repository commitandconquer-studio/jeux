//! « Ça fuit ! » — les règles, sans affichage.
//!
//! Une fuite au milieu d'une pâture : une pièce de 2 × 1 ouverte sur ses six
//! côtés. Le joueur y raccorde des tuyaux, des cartes de 2 × 1 (dominos)
//! tirées d'un paquet de 70, jusqu'à ce que tout soit bouché… ou qu'il n'ait
//! plus rien à poser. L'eau avance toute seule dans tout ce qui est relié ;
//! ce qui sort sur du vide déborde, sans défaite.
//!
//! - [`carte`] : les formes (les 34 possibles), les rotations, la géométrie
//!   d'une carte posée ;
//! - [`grille`] : la grille infinie, les règles de pose, l'eau ;
//! - [`partie`] : le paquet, la main, la partie et le score.
//!
//! Cahier des charges : `docs/jeux/ca-fuit.md`.

pub mod carte;
pub mod grille;
pub mod partie;
mod rng;

pub use carte::{Carte, Case, Direction, Forme, Rotation};
pub use grille::{Grille, Refus};
pub use partie::{Partie, Score, TAILLE_PAQUET, paquet};
pub use rng::Rng;
