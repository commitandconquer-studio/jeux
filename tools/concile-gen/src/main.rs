// concile-gen — Commit & Conquer. Licence GPL-3.0-or-later.
//
// Fabrique le « Livre des Conciles » : les grilles fixes de la campagne du
// Concile des Évêques, toutes à solution unique et résolubles par la logique,
// classées par taille puis par difficulté.
//
//     cargo run --release -p concile-gen     # écrit games/concile/puzzles.txt
use std::fmt::Write as _;

use concile_core::{Rng, generate};

/// Grilles gardées par taille.
const PER_SIZE: usize = 20;
/// Grilles générées par taille avant de choisir.
const CANDIDATES: usize = 120;

fn main() -> std::io::Result<()> {
    let root = std::env::args().nth(1).unwrap_or_else(|| ".".into());
    let mut out = String::from(
        "# Le Livre des Conciles — grilles de la campagne du Concile des Évêques.\n\
         # Fichier généré par `cargo run --release -p concile-gen` : ne pas modifier à la main.\n\
         # Une grille par ligne : taille, diocèse de chaque case (a, b, c…),\n\
         # colonne de l'évêque de chaque rangée (la solution), difficulté.\n",
    );
    for n in 5..=10 {
        let mut rng = Rng::new(1_000 + n as u64);
        let mut pool: Vec<_> = (0..CANDIDATES).map(|_| generate(n, &mut rng)).collect();
        pool.sort_by_key(|p| p.difficulty);
        pool.dedup_by(|a, b| a.regions == b.regions);
        // On garde des grilles réparties sur toute l'échelle de difficulté.
        let picked: Vec<_> = (0..PER_SIZE).map(|k| pool[k * (pool.len() - 1) / (PER_SIZE - 1)].clone()).collect();
        writeln!(out, "\n# {n} × {n}").unwrap();
        for p in &picked {
            writeln!(out, "{}", p.to_line()).unwrap();
        }
        println!(
            "{n} × {n} : {} grilles, difficulté de {} à {}",
            picked.len(),
            picked.first().unwrap().difficulty,
            picked.last().unwrap().difficulty
        );
    }
    std::fs::write(format!("{root}/games/concile/puzzles.txt"), out)
}
