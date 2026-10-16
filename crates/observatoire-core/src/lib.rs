//! L'Observatoire — règles, solveurs et générateur, sans affichage.
//!
//! Une grille carrée de côté pair, à remplir de soleils et de lunes :
//! 1. autant de soleils que de lunes dans chaque rangée et chaque colonne ;
//! 2. jamais plus de deux identiques côte à côte (rangée ou colonne) ;
//! 3. un « = » entre deux cases : pareilles ; un « × » : différentes.
//!
//! Chaque grille a une seule solution, trouvable par la logique. Partagé par
//! le jeu (`games/observatoire`) et le générateur (`tools/observatoire-gen`).

mod logic;
mod rng;

pub use logic::{Deduction, Technique, logic_solve, next_deduction};
pub use rng::Rng;

pub const VIDE: u8 = 0;
pub const SOLEIL: u8 = 1;
pub const LUNE: u8 = 2;

/// Le contraire d'un soleil est une lune, et inversement.
pub fn oppose(v: u8) -> u8 {
    match v {
        SOLEIL => LUNE,
        LUNE => SOLEIL,
        _ => VIDE,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Signe {
    Aucun,
    /// Les deux cases sont pareilles.
    Egal,
    /// Les deux cases sont différentes.
    Different,
}

impl Signe {
    fn car(self) -> char {
        match self {
            Signe::Aucun => '.',
            Signe::Egal => '=',
            Signe::Different => 'x',
        }
    }
    fn de(c: char) -> Option<Signe> {
        Some(match c {
            '.' => Signe::Aucun,
            '=' => Signe::Egal,
            'x' => Signe::Different,
            _ => return None,
        })
    }
}

/// Les signes d'une grille : entre chaque case et sa voisine de droite, et
/// entre chaque case et sa voisine du dessous.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Signes {
    pub n: usize,
    pub droite: Vec<Signe>,
    pub dessous: Vec<Signe>,
}

impl Signes {
    pub fn vides(n: usize) -> Signes {
        Signes { n, droite: vec![Signe::Aucun; n * n], dessous: vec![Signe::Aucun; n * n] }
    }

    /// Tous les signes posés : (case, voisine, signe).
    pub fn tous(&self) -> Vec<(usize, usize, Signe)> {
        let n = self.n;
        let mut v = Vec::new();
        for i in 0..n * n {
            if i % n + 1 < n && self.droite[i] != Signe::Aucun {
                v.push((i, i + 1, self.droite[i]));
            }
            if i / n + 1 < n && self.dessous[i] != Signe::Aucun {
                v.push((i, i + n, self.dessous[i]));
            }
        }
        v
    }

    pub fn nombre(&self) -> usize {
        self.tous().len()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Grille {
    pub n: usize,
    /// Cases données au départ (VIDE, SOLEIL ou LUNE).
    pub donnees: Vec<u8>,
    pub signes: Signes,
    /// La solution complète.
    pub solution: Vec<u8>,
    /// Difficulté estimée (plus c'est grand, plus c'est dur).
    pub difficulte: u32,
}

fn car(v: u8) -> char {
    match v {
        SOLEIL => 'S',
        LUNE => 'L',
        _ => '.',
    }
}

fn valeur(c: char) -> Option<u8> {
    Some(match c {
        'S' => SOLEIL,
        'L' => LUNE,
        '.' => VIDE,
        _ => return None,
    })
}

impl Grille {
    /// Une ligne de texte : `n données droite dessous solution difficulté`.
    pub fn to_line(&self) -> String {
        let d: String = self.donnees.iter().map(|&v| car(v)).collect();
        let dr: String = self.signes.droite.iter().map(|s| s.car()).collect();
        let de: String = self.signes.dessous.iter().map(|s| s.car()).collect();
        let s: String = self.solution.iter().map(|&v| car(v)).collect();
        format!("{} {d} {dr} {de} {s} {}", self.n, self.difficulte)
    }

    pub fn from_line(ligne: &str) -> Option<Grille> {
        let mut p = ligne.split_whitespace();
        let n: usize = p.next()?.parse().ok()?;
        let donnees: Vec<u8> = p.next()?.chars().map(valeur).collect::<Option<_>>()?;
        let droite: Vec<Signe> = p.next()?.chars().map(Signe::de).collect::<Option<_>>()?;
        let dessous: Vec<Signe> = p.next()?.chars().map(Signe::de).collect::<Option<_>>()?;
        let solution: Vec<u8> = p.next()?.chars().map(valeur).collect::<Option<_>>()?;
        let difficulte = p.next()?.parse().ok()?;
        let ok = [donnees.len(), droite.len(), dessous.len(), solution.len()].iter().all(|l| *l == n * n);
        ok.then_some(Grille { n, donnees, signes: Signes { n, droite, dessous }, solution, difficulte })
    }

    /// Les cases posées forment-elles la solution ?
    pub fn est_resolue_par(&self, cases: &[u8]) -> bool {
        cases == self.solution.as_slice()
    }
}

/// Les cases qui enfreignent une règle (pour les signaler au joueur) :
/// trois identiques à la suite, rangée ou colonne trop pleine, signe non
/// respecté.
pub fn erreurs(n: usize, signes: &Signes, cases: &[u8]) -> Vec<usize> {
    let mut mauvaises = vec![false; n * n];
    let lignes = lignes(n);
    for ligne in &lignes {
        for v in [SOLEIL, LUNE] {
            if ligne.iter().filter(|&&i| cases[i] == v).count() > n / 2 {
                for &i in ligne {
                    if cases[i] == v {
                        mauvaises[i] = true;
                    }
                }
            }
        }
        for k in 0..n - 2 {
            let (a, b, c) = (ligne[k], ligne[k + 1], ligne[k + 2]);
            if cases[a] != VIDE && cases[a] == cases[b] && cases[b] == cases[c] {
                mauvaises[a] = true;
                mauvaises[b] = true;
                mauvaises[c] = true;
            }
        }
    }
    for (a, b, s) in signes.tous() {
        if cases[a] != VIDE && cases[b] != VIDE {
            let pareil = cases[a] == cases[b];
            if (s == Signe::Egal && !pareil) || (s == Signe::Different && pareil) {
                mauvaises[a] = true;
                mauvaises[b] = true;
            }
        }
    }
    (0..n * n).filter(|&i| mauvaises[i]).collect()
}

/// Les rangées puis les colonnes, chacune comme une liste de cases.
pub fn lignes(n: usize) -> Vec<Vec<usize>> {
    let mut v: Vec<Vec<usize>> = (0..n).map(|r| (0..n).map(|c| r * n + c).collect()).collect();
    v.extend((0..n).map(|c| (0..n).map(|r| r * n + c).collect()));
    v
}

/// Poser `v` en `i` reste-t-il possible (sans regarder plus loin) ?
fn possible(n: usize, signes: &Signes, cases: &[u8], i: usize, v: u8) -> bool {
    let (r, c) = (i / n, i % n);
    let rang: Vec<usize> = (0..n).map(|k| r * n + k).collect();
    let col: Vec<usize> = (0..n).map(|k| k * n + c).collect();
    let val = |j: usize| if j == i { v } else { cases[j] };
    for (ligne, pos) in [(&rang, c), (&col, r)] {
        if ligne.iter().filter(|&&j| val(j) == v).count() > n / 2 {
            return false;
        }
        let lo = pos.saturating_sub(2);
        let hi = (pos + 2).min(n - 1);
        for k in lo..=hi.saturating_sub(2) {
            if k + 2 <= hi && val(ligne[k]) == v && val(ligne[k + 1]) == v && val(ligne[k + 2]) == v {
                return false;
            }
        }
    }
    // Les signes avec les voisines déjà posées.
    let voisins = [
        (c + 1 < n).then(|| (i + 1, signes.droite[i])),
        (c > 0).then(|| (i - 1, signes.droite[i - 1])),
        (r + 1 < n).then(|| (i + n, signes.dessous[i])),
        (r > 0).then(|| (i - n, signes.dessous[i - n])),
    ];
    for (j, s) in voisins.into_iter().flatten() {
        if cases[j] != VIDE {
            match s {
                Signe::Egal if cases[j] != v => return false,
                Signe::Different if cases[j] == v => return false,
                _ => {}
            }
        }
    }
    true
}

/// Nombre de solutions (on s'arrête à `limite`).
pub fn count_solutions(n: usize, signes: &Signes, donnees: &[u8], limite: usize) -> usize {
    fn chercher(n: usize, signes: &Signes, cases: &mut Vec<u8>, i: usize, limite: usize, trouve: &mut usize) {
        if *trouve >= limite {
            return;
        }
        if i == n * n {
            *trouve += 1;
            return;
        }
        if cases[i] != VIDE {
            chercher(n, signes, cases, i + 1, limite, trouve);
            return;
        }
        for v in [SOLEIL, LUNE] {
            if possible(n, signes, cases, i, v) {
                cases[i] = v;
                chercher(n, signes, cases, i + 1, limite, trouve);
                cases[i] = VIDE;
            }
        }
    }
    // Les cases données doivent déjà être cohérentes entre elles.
    if !erreurs(n, signes, donnees).is_empty() {
        return 0;
    }
    let mut cases = donnees.to_vec();
    let mut trouve = 0;
    chercher(n, signes, &mut cases, 0, limite, &mut trouve);
    trouve
}

/// Une grille pleine et valide, tirée au hasard.
fn pleine(n: usize, rng: &mut Rng) -> Vec<u8> {
    fn remplir(n: usize, signes: &Signes, cases: &mut Vec<u8>, i: usize, rng: &mut Rng) -> bool {
        if i == n * n {
            return true;
        }
        let mut ordre = [SOLEIL, LUNE];
        rng.shuffle(&mut ordre);
        for v in ordre {
            if possible(n, signes, cases, i, v) {
                cases[i] = v;
                if remplir(n, signes, cases, i + 1, rng) {
                    return true;
                }
                cases[i] = VIDE;
            }
        }
        false
    }
    let signes = Signes::vides(n);
    let mut cases = vec![VIDE; n * n];
    remplir(n, &signes, &mut cases, 0, rng);
    cases
}

/// Une indication de la grille : une case donnée ou un signe.
#[derive(Clone, Copy)]
enum Indice {
    Case(usize),
    Droite(usize),
    Dessous(usize),
}

/// Fabrique une grille de côté `n` (pair) : solution unique, résoluble par la
/// logique. On part de toutes les indications (cases et signes) et on en
/// retire au hasard tant que la grille reste unique et logique.
pub fn generate(n: usize, rng: &mut Rng) -> Grille {
    assert!(n % 2 == 0 && n >= 4, "côté pair, au moins 4");
    let solution = pleine(n, rng);
    let mut donnees = solution.clone();
    let mut signes = Signes::vides(n);
    for i in 0..n * n {
        if i % n + 1 < n {
            signes.droite[i] = if solution[i] == solution[i + 1] { Signe::Egal } else { Signe::Different };
        }
        if i / n + 1 < n {
            signes.dessous[i] = if solution[i] == solution[i + n] { Signe::Egal } else { Signe::Different };
        }
    }
    // D'abord les cases (pour garder des signes, qui font le charme du jeu),
    // puis les signes ; chaque groupe dans un ordre au hasard.
    let mut cases: Vec<Indice> = (0..n * n).map(Indice::Case).collect();
    let mut liens: Vec<Indice> = (0..n * n)
        .filter(|i| i % n + 1 < n)
        .map(Indice::Droite)
        .chain((0..n * n).filter(|i| i / n + 1 < n).map(Indice::Dessous))
        .collect();
    rng.shuffle(&mut cases);
    rng.shuffle(&mut liens);
    // On garde au moins quelques signes et quelques cases.
    let min_signes = n + n / 2;
    let min_cases = n / 2;
    for ind in cases.into_iter().chain(liens) {
        let (ancien_d, ancien_s) = (donnees.clone(), signes.clone());
        match ind {
            Indice::Case(i) => {
                if donnees.iter().filter(|&&v| v != VIDE).count() <= min_cases {
                    continue;
                }
                donnees[i] = VIDE;
            }
            Indice::Droite(i) | Indice::Dessous(i) if signes.nombre() <= min_signes => {
                let _ = i;
                continue;
            }
            Indice::Droite(i) => signes.droite[i] = Signe::Aucun,
            Indice::Dessous(i) => signes.dessous[i] = Signe::Aucun,
        }
        let unique = count_solutions(n, &signes, &donnees, 2) == 1;
        if !unique || logic_solve(n, &signes, &donnees).is_none() {
            donnees = ancien_d;
            signes = ancien_s;
        }
    }
    let difficulte = logic_solve(n, &signes, &donnees).unwrap_or(0);
    Grille { n, donnees, signes, solution, difficulte }
}

/// La grille du jour : la même pour tout le monde, à partir de la date.
pub fn du_jour(annee: i32, mois: u32, jour: u32) -> Grille {
    let graine = (annee as u64) * 10_000 + mois as u64 * 100 + jour as u64;
    let mut rng = Rng::new(graine.wrapping_mul(0x9E37_79B9_7F4A_7C15) ^ 0x0B5E_57A7);
    generate(6, &mut rng)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn les_grilles_generees_sont_uniques_et_logiques() {
        let mut rng = Rng::new(3);
        for n in [4, 6] {
            for _ in 0..6 {
                let g = generate(n, &mut rng);
                assert_eq!(count_solutions(n, &g.signes, &g.donnees, 2), 1);
                assert!(logic_solve(n, &g.signes, &g.donnees).is_some());
                assert!(erreurs(n, &g.signes, &g.solution).is_empty());
                assert_eq!(Grille::from_line(&g.to_line()), Some(g.clone()));
            }
        }
    }

    #[test]
    fn les_erreurs_sont_reperees() {
        let n = 4;
        let s = Signes::vides(n);
        let mut c = vec![VIDE; 16];
        c[0] = SOLEIL;
        c[1] = SOLEIL;
        c[2] = SOLEIL;
        assert_eq!(erreurs(n, &s, &c), vec![0, 1, 2]);
    }

    #[test]
    fn la_grille_du_jour_est_la_meme_pour_tous() {
        assert_eq!(du_jour(2026, 10, 16), du_jour(2026, 10, 16));
        assert_ne!(du_jour(2026, 10, 16).solution, du_jour(2026, 10, 17).solution);
    }
}
