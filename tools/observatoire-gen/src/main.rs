//! Génère le Livre des Constellations de L'Observatoire :
//! `cargo run --release -p observatoire-gen` écrit `games/observatoire/grilles.txt`.
//!
//! Pour chaque taille, on fabrique beaucoup de grilles (toutes uniques et
//! résolubles par la logique), on les classe par difficulté et on en garde
//! un nombre fixe, réparties de la plus facile à la plus dure.
use observatoire_core::{Grille, Rng, generate};
use std::collections::HashSet;

/// (côté, grilles candidates, grilles gardées)
const TAILLES: [(usize, usize, usize); 3] = [(4, 60, 10), (6, 420, 70), (8, 90, 20)];

fn main() -> std::io::Result<()> {
    let mut rng = Rng::new(2026);
    let mut livre: Vec<Grille> = Vec::new();
    for (n, candidats, gardes) in TAILLES {
        let mut vues = HashSet::new();
        let mut toutes: Vec<Grille> = Vec::new();
        for k in 0..candidats {
            let g = generate(n, &mut rng);
            if vues.insert(g.solution.clone()) {
                toutes.push(g);
            }
            if k % 50 == 49 {
                eprintln!("  {n}×{n} : {} grilles", toutes.len());
            }
        }
        toutes.sort_by_key(|g| g.difficulte);
        // Réparties sur toute l'échelle de difficulté.
        let pas = toutes.len() as f32 / gardes as f32;
        let choisies: Vec<Grille> = (0..gardes).map(|i| toutes[((i as f32 + 0.5) * pas) as usize].clone()).collect();
        eprintln!("{n}×{n} : {} gardées (difficulté {} à {})", choisies.len(), choisies[0].difficulte, choisies[gardes - 1].difficulte);
        livre.extend(choisies);
    }
    let mut texte = String::from("# Livre des Constellations — L'Observatoire (généré par tools/observatoire-gen)\n# côté données signes-droite signes-dessous solution difficulté\n");
    for g in &livre {
        texte.push_str(&g.to_line());
        texte.push('\n');
    }
    let chemin = concat!(env!("CARGO_MANIFEST_DIR"), "/../../games/observatoire/grilles.txt");
    std::fs::create_dir_all(concat!(env!("CARGO_MANIFEST_DIR"), "/../../games/observatoire"))?;
    std::fs::write(chemin, texte)?;
    eprintln!("{} grilles écrites dans games/observatoire/grilles.txt", livre.len());
    Ok(())
}
