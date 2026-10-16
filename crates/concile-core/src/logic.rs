//! Solveur « logique » : il raisonne comme un joueur, sans jamais deviner.
//! Sert à refuser les grilles qui demanderaient de tenter sa chance, à noter
//! leur difficulté, et à donner des indices.

/// Les techniques de déduction, de la plus simple à la plus subtile.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Technique {
    /// Une rangée, une colonne ou un diocèse n'a plus qu'une case possible.
    Single,
    /// Les cases possibles d'un diocèse tiennent dans une seule rangée (ou
    /// colonne), ou l'inverse : le reste de cette rangée est exclu.
    Confinement,
    /// Poser un évêque ici viderait complètement une rangée, une colonne ou
    /// un diocèse : cette case est donc exclue.
    Attack,
    /// 2 ou 3 diocèses enfermés dans autant de rangées (ou colonnes).
    Group,
}

impl Technique {
    fn cost(self) -> u32 {
        match self {
            Technique::Single => 1,
            Technique::Confinement => 3,
            Technique::Attack => 6,
            Technique::Group => 10,
        }
    }

    /// Explication courte, pour les indices, dans la langue `langue` :
    /// 0 français, 1 anglais, 2 allemand, 3 italien, 4 espagnol (le français
    /// pour toute autre valeur).
    pub fn explain(self, langue: usize) -> &'static str {
        let textes: [&'static str; 5] = match self {
            Technique::Single => [
                "Il ne reste qu'une case possible ici.",
                "Only one possible square is left here.",
                "Hier bleibt nur noch ein mögliches Feld.",
                "Qui resta una sola casella possibile.",
                "Aquí solo queda una casilla posible.",
            ],
            Technique::Confinement => [
                "Ce diocèse est enfermé dans une seule rangée ou colonne : le reste de celle-ci est exclu.",
                "This diocese is confined to a single row or column: the rest of that line is ruled out.",
                "Diese Diözese liegt ganz in einer Zeile oder Spalte: Der Rest dieser Linie ist ausgeschlossen.",
                "Questa diocesi è chiusa in una sola riga o colonna: il resto di quella linea è escluso.",
                "Esta diócesis está encerrada en una sola fila o columna: el resto de esa línea queda descartado.",
            ],
            Technique::Attack => [
                "Un évêque ici priverait un autre diocèse (ou une rangée) de toute place.",
                "A bishop here would leave another diocese (or a row) with no room at all.",
                "Ein Bischof hier ließe einer anderen Diözese (oder einer Zeile) keinen Platz mehr.",
                "Un vescovo qui toglierebbe ogni posto a un'altra diocesi (o a una riga).",
                "Un obispo aquí dejaría a otra diócesis (o a una fila) sin ningún sitio.",
            ],
            Technique::Group => [
                "Plusieurs diocèses se partagent exactement autant de rangées : les autres cases de ces rangées sont exclues.",
                "Several dioceses share exactly as many rows: the other squares of those rows are ruled out.",
                "Mehrere Diözesen teilen sich genau so viele Zeilen: Die übrigen Felder dieser Zeilen sind ausgeschlossen.",
                "Più diocesi si dividono esattamente altrettante righe: le altre caselle di quelle righe sono escluse.",
                "Varias diócesis comparten exactamente tantas filas: las demás casillas de esas filas quedan descartadas.",
            ],
        };
        textes.get(langue).copied().unwrap_or(textes[0])
    }
}

/// Une déduction : une case où poser un évêque, ou des cases à exclure.
#[derive(Clone, Debug)]
pub struct Deduction {
    pub technique: Technique,
    pub place: Option<usize>,
    pub exclude: Vec<usize>,
}

struct State<'a> {
    n: usize,
    regions: &'a [u8],
    cand: Vec<bool>,
    placed: Vec<bool>,
    /// Rangées, colonnes et diocèses : listes de cases.
    units: Vec<Vec<usize>>,
}

impl<'a> State<'a> {
    fn new(n: usize, regions: &'a [u8]) -> Self {
        let mut units = Vec::new();
        for r in 0..n {
            units.push((0..n).map(|c| r * n + c).collect());
        }
        for c in 0..n {
            units.push((0..n).map(|r| r * n + c).collect());
        }
        for d in 0..n as u8 {
            units.push((0..n * n).filter(|&i| regions[i] == d).collect());
        }
        State { n, regions, cand: vec![true; n * n], placed: vec![false; n * n], units }
    }

    /// Cases exclues si l'on pose un évêque en `i`.
    fn shadow(&self, i: usize) -> Vec<usize> {
        let n = self.n;
        let (r, c) = (i / n, i % n);
        (0..n * n)
            .filter(|&j| {
                let (r2, c2) = (j / n, j % n);
                j != i && (r2 == r || c2 == c || self.regions[j] == self.regions[i] || (r.abs_diff(r2) <= 1 && c.abs_diff(c2) <= 1))
            })
            .collect()
    }

    fn place(&mut self, i: usize) {
        self.placed[i] = true;
        self.cand[i] = false;
        for j in self.shadow(i) {
            self.cand[j] = false;
        }
    }

    fn unit_done(&self, u: &[usize]) -> bool {
        u.iter().any(|&i| self.placed[i])
    }

    fn unit_cands(&self, u: &[usize]) -> Vec<usize> {
        u.iter().copied().filter(|&i| self.cand[i]).collect()
    }

    fn solved(&self) -> bool {
        self.placed.iter().filter(|&&p| p).count() == self.n
    }

    fn stuck(&self) -> bool {
        self.units.iter().any(|u| !self.unit_done(u) && self.unit_cands(u).is_empty())
    }

    /// Cherche la déduction la plus simple possible.
    fn deduce(&self) -> Option<Deduction> {
        let n = self.n;
        // 1. Case unique
        for u in &self.units {
            if !self.unit_done(u) {
                let c = self.unit_cands(u);
                if c.len() == 1 {
                    return Some(Deduction { technique: Technique::Single, place: Some(c[0]), exclude: Vec::new() });
                }
            }
        }
        // 2. Confinement : un groupe (rangée, colonne, diocèse) dont toutes
        //    les cases possibles tombent dans un même autre groupe.
        for (a, ua) in self.units.iter().enumerate() {
            if self.unit_done(ua) {
                continue;
            }
            let ca = self.unit_cands(ua);
            for (b, ub) in self.units.iter().enumerate() {
                if a == b || self.unit_done(ub) || ca.iter().any(|i| !ub.contains(i)) {
                    continue;
                }
                let exclude: Vec<usize> = ub.iter().copied().filter(|&j| self.cand[j] && !ua.contains(&j)).collect();
                if !exclude.is_empty() {
                    return Some(Deduction { technique: Technique::Confinement, place: None, exclude });
                }
            }
        }
        // 3. Attaque : poser ici viderait un autre groupe.
        let mut exclude = Vec::new();
        for i in (0..n * n).filter(|&i| self.cand[i]) {
            let shadow = self.shadow(i);
            let kills = self.units.iter().any(|u| {
                !u.contains(&i) && !self.unit_done(u) && self.unit_cands(u).iter().all(|j| shadow.contains(j))
            });
            if kills {
                exclude.push(i);
            }
        }
        if !exclude.is_empty() {
            return Some(Deduction { technique: Technique::Attack, place: None, exclude });
        }
        // 4. Groupes de 2 ou 3 diocèses enfermés dans autant de rangées
        //    (ou colonnes) — et l'inverse.
        let lines: [(std::ops::Range<usize>, std::ops::Range<usize>); 4] = [
            (2 * n..3 * n, 0..n),     // diocèses → rangées
            (2 * n..3 * n, n..2 * n), // diocèses → colonnes
            (0..n, 2 * n..3 * n),     // rangées → diocèses
            (n..2 * n, 2 * n..3 * n), // colonnes → diocèses
        ];
        for (from, to) in lines {
            let open: Vec<usize> = from.clone().filter(|&u| !self.unit_done(&self.units[u])).collect();
            for k in 2..=3 {
                for combo in combinations(&open, k) {
                    let cells: Vec<usize> = combo.iter().flat_map(|&u| self.unit_cands(&self.units[u])).collect();
                    let targets: Vec<usize> = to.clone().filter(|&t| cells.iter().any(|i| self.units[t].contains(i))).collect();
                    if targets.len() != k {
                        continue;
                    }
                    let exclude: Vec<usize> = targets
                        .iter()
                        .flat_map(|&t| self.units[t].iter().copied())
                        .filter(|&j| self.cand[j] && !cells.contains(&j))
                        .collect();
                    if !exclude.is_empty() {
                        return Some(Deduction { technique: Technique::Group, place: None, exclude });
                    }
                }
            }
        }
        None
    }

    fn apply(&mut self, d: &Deduction) {
        if let Some(i) = d.place {
            self.place(i);
        }
        for &j in &d.exclude {
            self.cand[j] = false;
        }
    }
}

fn combinations(items: &[usize], k: usize) -> Vec<Vec<usize>> {
    if k == 0 {
        return vec![Vec::new()];
    }
    let mut out = Vec::new();
    for (i, &x) in items.iter().enumerate() {
        for mut rest in combinations(&items[i + 1..], k - 1) {
            rest.insert(0, x);
            out.push(rest);
        }
    }
    out
}

/// Résout par la logique seule. Renvoie la difficulté, ou `None` si la
/// grille demande de deviner (ou n'a pas de solution).
pub fn logic_solve(n: usize, regions: &[u8]) -> Option<u32> {
    let mut s = State::new(n, regions);
    let mut score = 0;
    let mut hardest = Technique::Single;
    while !s.solved() {
        if s.stuck() {
            return None;
        }
        let d = s.deduce()?;
        score += d.technique.cost();
        hardest = hardest.max(d.technique);
        s.apply(&d);
    }
    Some(score + hardest.cost() * 10)
}

/// Prochaine déduction utile au joueur, à partir de ses évêques et de ses
/// croix (supposés justes) : on saute les déductions qu'il a déjà faites.
pub fn next_deduction(n: usize, regions: &[u8], placed: &[usize], crossed: &[usize]) -> Option<Deduction> {
    let mut s = State::new(n, regions);
    for &i in placed {
        s.place(i);
    }
    for &i in crossed {
        s.cand[i] = false;
    }
    let known = |s: &State, i: usize| s.placed[i] || crossed.contains(&i) || !s.cand[i];
    for _ in 0..(n * n) {
        let d = s.deduce()?;
        // Utile si elle pose un évêque, ou exclut une case pas encore barrée.
        let new_place = d.place.filter(|&i| !s.placed[i]);
        let new_exclude: Vec<usize> = d.exclude.iter().copied().filter(|&i| !known(&s, i) || (s.cand[i] && !crossed.contains(&i))).collect();
        if new_place.is_some() || !new_exclude.is_empty() {
            return Some(Deduction { technique: d.technique, place: new_place, exclude: new_exclude });
        }
        s.apply(&d);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Rng, generate};

    #[test]
    fn la_logique_resout_les_grilles_generees() {
        let mut rng = Rng::new(11);
        for n in 5..=9 {
            let p = generate(n, &mut rng);
            assert!(logic_solve(n, &p.regions).is_some(), "taille {n}");
        }
    }

    #[test]
    fn chaque_technique_parle_cinq_langues() {
        for t in [Technique::Single, Technique::Confinement, Technique::Attack, Technique::Group] {
            assert_eq!(t.explain(9), t.explain(0));
            assert!((1..5).all(|l| t.explain(l) != t.explain(0)));
        }
    }

    #[test]
    fn un_indice_est_toujours_juste() {
        let p = generate(7, &mut Rng::new(5));
        let d = next_deduction(7, &p.regions, &[], &[]).expect("au moins une déduction");
        if let Some(i) = d.place {
            assert!(p.is_bishop(i / 7, i % 7));
        }
        for i in d.exclude {
            assert!(!p.is_bishop(i / 7, i % 7));
        }
    }
}
