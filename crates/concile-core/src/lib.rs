//! Le Concile des Évêques — règles, solveurs et générateur, sans affichage.
//!
//! Une grille N × N est découpée en N diocèses. Il faut placer N évêques :
//! un par rangée, un par colonne, un par diocèse, et aucun voisin (même en
//! diagonale). Chaque grille a une seule solution, trouvable par la logique.
//!
//! Partagé par le jeu (`games/concile`) et le générateur de grilles
//! (`tools/concile-gen`).

mod logic;
mod rng;

pub use logic::{Deduction, Technique, logic_solve, next_deduction};
pub use rng::Rng;

/// Taille maximale d'une grille.
pub const MAX: usize = 10;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Puzzle {
    pub n: usize,
    /// Diocèse de chaque case, rangée par rangée (0..n).
    pub regions: Vec<u8>,
    /// Colonne de l'évêque de chaque rangée (la solution).
    pub solution: Vec<u8>,
    /// Difficulté estimée (plus c'est grand, plus c'est dur).
    pub difficulty: u32,
}

impl Puzzle {
    pub fn region(&self, row: usize, col: usize) -> u8 {
        self.regions[row * self.n + col]
    }

    /// La case (rangée, colonne) contient-elle un évêque dans la solution ?
    pub fn is_bishop(&self, row: usize, col: usize) -> bool {
        self.solution[row] as usize == col
    }

    /// Une ligne de texte : `n diocèses solution difficulté`,
    /// par exemple `5 aaabb… 02413 12`.
    pub fn to_line(&self) -> String {
        let regions: String = self.regions.iter().map(|&r| (b'a' + r) as char).collect();
        let solution: String = self.solution.iter().map(|&c| (b'0' + c) as char).collect();
        format!("{} {} {} {}", self.n, regions, solution, self.difficulty)
    }

    pub fn from_line(line: &str) -> Option<Puzzle> {
        let mut parts = line.split_whitespace();
        let n: usize = parts.next()?.parse().ok()?;
        let regions: Vec<u8> = parts.next()?.bytes().map(|b| b - b'a').collect();
        let solution: Vec<u8> = parts.next()?.bytes().map(|b| b - b'0').collect();
        let difficulty = parts.next()?.parse().ok()?;
        (regions.len() == n * n && solution.len() == n).then_some(Puzzle { n, regions, solution, difficulty })
    }

    /// Les évêques posés (une colonne par rangée, ou rien) respectent-ils toutes les règles ?
    pub fn is_solved_by(&self, placed: &[(usize, usize)]) -> bool {
        placed.len() == self.n && conflicts(self.n, &self.regions, placed).is_empty()
    }
}

/// Évêques en conflit (même rangée, colonne, diocèse, ou voisins).
pub fn conflicts(n: usize, regions: &[u8], placed: &[(usize, usize)]) -> Vec<(usize, usize)> {
    let mut bad = Vec::new();
    for (i, &(r1, c1)) in placed.iter().enumerate() {
        for &(r2, c2) in &placed[i + 1..] {
            let clash = r1 == r2
                || c1 == c2
                || regions[r1 * n + c1] == regions[r2 * n + c2]
                || (r1.abs_diff(r2) <= 1 && c1.abs_diff(c2) <= 1);
            if clash {
                for p in [(r1, c1), (r2, c2)] {
                    if !bad.contains(&p) {
                        bad.push(p);
                    }
                }
            }
        }
    }
    bad
}

/// Compte les solutions (jusqu'à `limit`). Comme il y a un évêque par
/// rangée, il suffit de vérifier le voisinage avec la rangée précédente.
pub fn count_solutions(n: usize, regions: &[u8], limit: usize) -> usize {
    fn rec(row: usize, n: usize, regions: &[u8], cols: u16, regs: u16, prev: i32, count: &mut usize, limit: usize) {
        if row == n {
            *count += 1;
            return;
        }
        for c in 0..n {
            let r = regions[row * n + c];
            if cols & (1 << c) != 0 || regs & (1 << r) != 0 || (prev >= 0 && (c as i32 - prev).abs() <= 1) {
                continue;
            }
            rec(row + 1, n, regions, cols | 1 << c, regs | 1 << r, c as i32, count, limit);
            if *count >= limit {
                return;
            }
        }
    }
    let mut count = 0;
    rec(0, n, regions, 0, 0, -1, &mut count, limit);
    count
}

/// Une solution au hasard : une colonne par rangée, jamais voisines.
fn random_solution(n: usize, rng: &mut Rng) -> Vec<u8> {
    fn rec(row: usize, n: usize, cols: u16, sol: &mut Vec<u8>, rng: &mut Rng) -> bool {
        if row == n {
            return true;
        }
        let mut order: Vec<usize> = (0..n).collect();
        rng.shuffle(&mut order);
        for c in order {
            let near = sol.last().is_some_and(|&p| (p as i32 - c as i32).abs() <= 1);
            if cols & (1 << c) != 0 || near {
                continue;
            }
            sol.push(c as u8);
            if rec(row + 1, n, cols | 1 << c, sol, rng) {
                return true;
            }
            sol.pop();
        }
        false
    }
    let mut sol = Vec::with_capacity(n);
    rec(0, n, 0, &mut sol, rng);
    sol
}

fn neighbours4(n: usize, i: usize) -> impl Iterator<Item = usize> {
    let (r, c) = (i / n, i % n);
    [(r.wrapping_sub(1), c), (r + 1, c), (r, c.wrapping_sub(1)), (r, c + 1)]
        .into_iter()
        .filter(move |&(r, c)| r < n && c < n)
        .map(move |(r, c)| r * n + c)
}

/// Fait pousser les diocèses autour des évêques de la solution.
fn grow_regions(n: usize, sol: &[u8], rng: &mut Rng) -> Vec<u8> {
    const NONE: u8 = u8::MAX;
    let mut regions = vec![NONE; n * n];
    for (r, &c) in sol.iter().enumerate() {
        regions[r * n + c as usize] = r as u8;
    }
    let mut left = n * n - n;
    while left > 0 {
        // Une case libre au hasard, collée à un diocèse : elle le rejoint.
        let free: Vec<usize> = (0..n * n)
            .filter(|&i| regions[i] == NONE && neighbours4(n, i).any(|j| regions[j] != NONE))
            .collect();
        let i = free[rng.below(free.len())];
        let around: Vec<u8> = neighbours4(n, i).map(|j| regions[j]).filter(|&r| r != NONE).collect();
        regions[i] = around[rng.below(around.len())];
        left -= 1;
    }
    regions
}

/// Le diocèse `region` reste-t-il d'un seul tenant sans la case `without` ?
fn connected_without(n: usize, regions: &[u8], region: u8, without: usize) -> bool {
    let cells: Vec<usize> = (0..n * n).filter(|&i| i != without && regions[i] == region).collect();
    let Some(&start) = cells.first() else { return false };
    let mut seen = vec![false; n * n];
    seen[start] = true;
    let mut todo = vec![start];
    let mut count = 1;
    while let Some(i) = todo.pop() {
        for j in neighbours4(n, i) {
            if j != without && !seen[j] && regions[j] == region {
                seen[j] = true;
                count += 1;
                todo.push(j);
            }
        }
    }
    count == cells.len()
}

/// Génère une grille de taille `n` : solution unique et résoluble par la logique.
pub fn generate(n: usize, rng: &mut Rng) -> Puzzle {
    assert!((4..=MAX).contains(&n));
    loop {
        let solution = random_solution(n, rng);
        let mut regions = grow_regions(n, &solution, rng);
        let bishops: Vec<usize> = solution.iter().enumerate().map(|(r, &c)| r * n + c as usize).collect();
        // Retouche les frontières jusqu'à ce qu'il ne reste qu'une solution.
        let mut score = count_solutions(n, &regions, 40);
        let mut tries = 0;
        while score > 1 && tries < 4_000 {
            tries += 1;
            let i = rng.below(n * n);
            if bishops.contains(&i) {
                continue;
            }
            let others: Vec<u8> = neighbours4(n, i).map(|j| regions[j]).filter(|&r| r != regions[i]).collect();
            if others.is_empty() || !connected_without(n, &regions, regions[i], i) {
                continue;
            }
            let old = regions[i];
            regions[i] = others[rng.below(others.len())];
            let s = count_solutions(n, &regions, 40);
            if s <= score {
                score = s;
            } else {
                regions[i] = old;
            }
        }
        if score != 1 {
            continue;
        }
        if let Some(difficulty) = logic_solve(n, &regions) {
            return Puzzle { n, regions, solution, difficulty };
        }
    }
}

/// Taille de la grille du défi du jour selon le jour de la semaine
/// (0 = lundi) : plus facile en début de semaine, plus corsé le week-end.
pub fn daily_size(weekday: usize) -> usize {
    [6, 6, 7, 7, 8, 9, 8][weekday % 7]
}

/// La grille du défi du jour : la même pour tout le monde, à partir de la date.
pub fn daily(year: i32, month: u32, day: u32, weekday: usize) -> Puzzle {
    let seed = (year as u64) * 10_000 + month as u64 * 100 + day as u64;
    let mut rng = Rng::new(seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) ^ 0xC0_0C11E);
    generate(daily_size(weekday), &mut rng)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn les_grilles_generees_ont_une_seule_solution() {
        let mut rng = Rng::new(7);
        for n in 5..=8 {
            let p = generate(n, &mut rng);
            assert_eq!(count_solutions(n, &p.regions, 10), 1, "taille {n}");
            let placed: Vec<(usize, usize)> = p.solution.iter().enumerate().map(|(r, &c)| (r, c as usize)).collect();
            assert!(p.is_solved_by(&placed));
            // Chaque diocèse contient exactement un évêque de la solution.
            for d in 0..n as u8 {
                assert_eq!(placed.iter().filter(|&&(r, c)| p.region(r, c) == d).count(), 1);
            }
        }
    }

    #[test]
    fn une_grille_s_ecrit_et_se_relit() {
        let p = generate(6, &mut Rng::new(3));
        assert_eq!(Puzzle::from_line(&p.to_line()), Some(p));
    }

    #[test]
    fn le_defi_du_jour_est_le_meme_pour_tous() {
        assert_eq!(daily(2026, 12, 25, 4), daily(2026, 12, 25, 4));
        assert_ne!(daily(2026, 12, 25, 4).regions, daily(2026, 12, 26, 5).regions);
    }

    #[test]
    fn voisins_et_doublons_sont_des_conflits() {
        let regions = vec![0; 25];
        let bad = conflicts(5, &regions, &[(0, 0), (1, 1)]);
        assert_eq!(bad.len(), 2);
    }
}
