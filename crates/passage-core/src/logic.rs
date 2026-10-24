//! Solveur « logique » : il raisonne comme un joueur, sans jamais deviner.
//! Sert à refuser les grilles qui demanderaient de tenter sa chance, à noter
//! leur difficulté, et à donner des indices. Une recherche exhaustive, qui
//! s'appuie sur les mêmes déductions, compte les solutions : c'est elle qui
//! garantit qu'une grille n'en a qu'une.

use crate::{CROIX, Kind, Network, Puzzle, RAIL, ROUTE, VIDE, check_structure, neighbors};

/// Les techniques de déduction, de la plus simple à la plus subtile.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Technique {
    /// Le chiffre d'une rangée ou d'une colonne est atteint (le reste est
    /// vide) ou ne peut l'être qu'avec toutes les cases restantes.
    Compte,
    /// Une case de la voie a besoin de deux voisines (une seule pour une
    /// gare), pas une de plus, pas une de moins.
    Voisins,
    /// Autour d'un passage à niveau, les deux voies se croisent à angle droit.
    Croisement,
    /// La voie est déjà complète, ou refermerait une boucle.
    Boucle,
}

impl Technique {
    fn cost(self) -> u32 {
        match self {
            Technique::Compte => 1,
            Technique::Voisins => 2,
            Technique::Croisement => 3,
            Technique::Boucle => 4,
        }
    }

    /// Explication courte, pour les indices, dans la langue `langue` :
    /// 0 français, 1 anglais, 2 allemand, 3 italien, 4 espagnol (le français
    /// pour toute autre valeur).
    pub fn explain(self, langue: usize) -> &'static str {
        let textes: [&'static str; 5] = match self {
            Technique::Compte => [
                "Ce chiffre est atteint, ou il faut toutes les cases restantes : on peut conclure sur cette rangée ou cette colonne.",
                "This number is reached, or every remaining square is needed: the rest of this row or column follows.",
                "Diese Zahl ist erreicht, oder es werden alle übrigen Felder gebraucht: der Rest dieser Zeile oder Spalte folgt daraus.",
                "Questo numero è raggiunto, oppure servono tutte le caselle rimaste: il resto di questa riga o colonna ne segue.",
                "Este número ya se alcanzó, o hacen falta todas las casillas que quedan: el resto de esta fila o columna se deduce.",
            ],
            Technique::Voisins => [
                "Une voie a besoin de deux voisines, ni plus ni moins (une seule pour une gare).",
                "A track needs exactly two neighbours (only one for a station).",
                "Eine Strecke braucht genau zwei Nachbarn (ein Bahnhof nur einen).",
                "Un tracciato ha bisogno di due vicine, né più né meno (una sola per una stazione).",
                "Una vía necesita exactamente dos vecinas (solo una si es una estación).",
            ],
            Technique::Croisement => [
                "Au passage à niveau, le rail et la route se croisent à angle droit.",
                "At a level crossing, the rail and the road cross at a right angle.",
                "Am Bahnübergang kreuzen sich Schiene und Straße im rechten Winkel.",
                "Al passaggio a livello, binario e strada si incrociano ad angolo retto.",
                "En el paso a nivel, la vía y la carretera se cruzan en ángulo recto.",
            ],
            Technique::Boucle => [
                "Cette voie est déjà complète, ou cette case la refermerait sur elle-même.",
                "This track is already complete, or this square would close it into a loop.",
                "Diese Strecke ist schon vollständig, oder dieses Feld würde sie zur Schleife schließen.",
                "Questo tracciato è già completo, oppure questa casella lo chiuderebbe ad anello.",
                "Esta vía ya está completa, o esta casilla la cerraría en un bucle.",
            ],
        };
        textes.get(langue).copied().unwrap_or(textes[0])
    }
}

/// Une conclusion du solveur : sur cette case, ces types sont exclus.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Deduction {
    pub cell: usize,
    /// Les types de case (bits `VIDE`, `RAIL`, `ROUTE`, `CROIX`) qui ne sont plus possibles.
    pub removed: u8,
    pub technique: Technique,
}

struct Contradiction;

struct Solver<'a> {
    p: &'a Puzzle,
    /// Types encore possibles pour chaque case (masque de bits).
    dom: Vec<u8>,
    log: Vec<Deduction>,
    /// S'arrêter à la première déduction (pour les indices).
    one: bool,
}

impl<'a> Solver<'a> {
    fn new(p: &'a Puzzle) -> Option<Self> {
        let n = p.n;
        let mut dom: Vec<u8> = (0..n * n)
            .map(|i| match (p.givens[i], p.solution[i]) {
                (true, kind) => kind.bit(),
                (false, Kind::Croisement) => CROIX,
                _ => VIDE | RAIL | ROUTE,
            })
            .collect();
        for net in Network::ALL {
            for g in p.gates[net as usize] {
                let i = g.cell(n);
                dom[i] &= net.pure_kind().bit();
                if dom[i] == 0 {
                    return None;
                }
            }
        }
        Some(Solver { p, dom, log: Vec::new(), one: false })
    }

    fn stop(&self) -> bool {
        self.one && !self.log.is_empty()
    }

    fn restrict(&mut self, i: usize, keep: u8, technique: Technique) -> Result<(), Contradiction> {
        let old = self.dom[i];
        let new = old & keep;
        if new == 0 {
            return Err(Contradiction);
        }
        if new != old {
            self.dom[i] = new;
            self.log.push(Deduction { cell: i, removed: old & !new, technique });
        }
        Ok(())
    }

    fn is_m(&self, i: usize, net: Network) -> bool {
        self.dom[i] & !net.member_mask() == 0
    }

    fn can_m(&self, i: usize, net: Network) -> bool {
        self.dom[i] & net.member_mask() != 0
    }

    fn is_gate(&self, i: usize, net: Network) -> bool {
        self.p.gates[net as usize].iter().any(|g| g.cell(self.p.n) == i)
    }

    /// Applique toutes les déductions jusqu'à ce qu'il n'y en ait plus.
    fn propagate(&mut self) -> Result<(), Contradiction> {
        loop {
            let before = self.log.len();
            self.rule_counts()?;
            self.rule_degree()?;
            self.rule_crossing()?;
            self.rule_paths()?;
            if self.log.len() == before || self.stop() {
                return Ok(());
            }
        }
    }

    fn rule_counts(&mut self) -> Result<(), Contradiction> {
        let n = self.p.n;
        for net in Network::ALL {
            let m = net.member_mask();
            for line in 0..2 * n {
                if self.stop() {
                    return Ok(());
                }
                let (target, cells): (usize, Vec<usize>) = if line < n {
                    (self.p.row_counts[net as usize][line] as usize, (0..n).map(|c| line * n + c).collect())
                } else {
                    let c = line - n;
                    (self.p.col_counts[net as usize][c] as usize, (0..n).map(|r| r * n + c).collect())
                };
                let sure = cells.iter().filter(|&&i| self.is_m(i, net)).count();
                let maybe = cells.iter().filter(|&&i| self.can_m(i, net)).count();
                if sure > target || maybe < target {
                    return Err(Contradiction);
                }
                if maybe == sure {
                    continue;
                }
                let keep = if sure == target {
                    !m
                } else if maybe == target {
                    m
                } else {
                    continue;
                };
                for &i in &cells {
                    if self.can_m(i, net) && !self.is_m(i, net) {
                        self.restrict(i, keep, Technique::Compte)?;
                        if self.stop() {
                            return Ok(());
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn rule_degree(&mut self) -> Result<(), Contradiction> {
        let n = self.p.n;
        for net in Network::ALL {
            let m = net.member_mask();
            for i in 0..n * n {
                if self.stop() {
                    return Ok(());
                }
                let nbs: Vec<usize> = neighbors(n, i).into_iter().flatten().collect();
                let sure = nbs.iter().filter(|&&j| self.is_m(j, net)).count();
                let maybe = nbs.iter().filter(|&&j| self.can_m(j, net)).count();
                if self.is_m(i, net) {
                    if self.dom[i] == CROIX {
                        continue;
                    }
                    let needed = if self.is_gate(i, net) { 1 } else { 2 };
                    if sure > needed || maybe < needed {
                        return Err(Contradiction);
                    }
                    if maybe == sure {
                        continue;
                    }
                    let keep = if sure == needed {
                        !m
                    } else if maybe == needed {
                        m
                    } else {
                        continue;
                    };
                    for &j in &nbs {
                        if self.can_m(j, net) && !self.is_m(j, net) {
                            self.restrict(j, keep, Technique::Voisins)?;
                            if self.stop() {
                                return Ok(());
                            }
                        }
                    }
                } else if self.can_m(i, net) && sure > 2 {
                    self.restrict(i, !m, Technique::Voisins)?;
                }
            }
        }
        Ok(())
    }

    fn rule_crossing(&mut self) -> Result<(), Contradiction> {
        let n = self.p.n;
        for i in 0..n * n {
            if self.stop() {
                return Ok(());
            }
            if self.dom[i] != CROIX {
                continue;
            }
            let nbs = neighbors(n, i);
            if nbs.iter().any(Option::is_none) {
                return Err(Contradiction);
            }
            let nb = nbs.map(|j| j.unwrap());
            // `vertical` : le rail passe au nord et au sud, la route à l'est
            // et à l'ouest ; sinon c'est l'inverse.
            let feasible = |s: &Self, vertical: bool| {
                let (rail, route) = if vertical { ([0, 2], [1, 3]) } else { ([1, 3], [0, 2]) };
                rail.iter().all(|&d| s.dom[nb[d]] & RAIL != 0)
                    && route.iter().all(|&d| s.dom[nb[d]] & ROUTE != 0)
            };
            let (v, h) = (feasible(self, true), feasible(self, false));
            let vertical = match (v, h) {
                (false, false) => return Err(Contradiction),
                (true, false) => true,
                (false, true) => false,
                (true, true) => continue,
            };
            let (rail, route) = if vertical { ([0, 2], [1, 3]) } else { ([1, 3], [0, 2]) };
            for d in rail {
                self.restrict(nb[d], RAIL, Technique::Croisement)?;
            }
            for d in route {
                self.restrict(nb[d], ROUTE, Technique::Croisement)?;
            }
        }
        Ok(())
    }

    fn rule_paths(&mut self) -> Result<(), Contradiction> {
        let n = self.p.n;
        for net in Network::ALL {
            let m = net.member_mask();
            let mut comp = vec![usize::MAX; n * n];
            let mut count = 0;
            for start in 0..n * n {
                if comp[start] != usize::MAX || !self.is_m(start, net) {
                    continue;
                }
                let mut stack = vec![start];
                comp[start] = count;
                while let Some(i) = stack.pop() {
                    for j in neighbors(n, i).into_iter().flatten() {
                        if comp[j] == usize::MAX && self.is_m(j, net) {
                            comp[j] = count;
                            stack.push(j);
                        }
                    }
                }
                count += 1;
            }
            for id in 0..count {
                let cells: Vec<usize> = (0..n * n).filter(|&i| comp[i] == id).collect();
                let gates = cells.iter().filter(|&&i| self.is_gate(i, net)).count();
                let closed = cells
                    .iter()
                    .all(|&i| neighbors(n, i).into_iter().flatten().filter(|&j| self.is_m(j, net)).count() == 2);
                if gates == 0 && closed {
                    return Err(Contradiction);
                }
                if gates == 2 {
                    for i in 0..n * n {
                        if comp[i] == id {
                            continue;
                        }
                        if self.is_m(i, net) {
                            return Err(Contradiction);
                        }
                        if self.can_m(i, net) {
                            self.restrict(i, !m, Technique::Boucle)?;
                            if self.stop() {
                                return Ok(());
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn solved(&self) -> Option<Vec<Kind>> {
        self.dom.iter().map(|&d| Kind::from_bit(d)).collect()
    }
}

/// Toutes les solutions de la grille (au plus `limit`), en cherchant
/// exhaustivement. Une bonne grille n'en a qu'une.
pub fn solutions(p: &Puzzle, limit: usize) -> Vec<Vec<Kind>> {
    let mut out = Vec::new();
    if let Some(solver) = Solver::new(p) {
        search(p, solver.dom, limit, &mut out);
    }
    out
}

fn search(p: &Puzzle, dom: Vec<u8>, limit: usize, out: &mut Vec<Vec<Kind>>) {
    if out.len() >= limit {
        return;
    }
    let mut s = Solver { p, dom, log: Vec::new(), one: false };
    if s.propagate().is_err() {
        return;
    }
    let pick = (0..s.dom.len())
        .filter(|&i| s.dom[i].count_ones() > 1)
        .min_by_key(|&i| s.dom[i].count_ones());
    match pick {
        None => {
            if let Some(cells) = s.solved() {
                if check_structure(p.n, &p.gates, &cells).is_ok() && p.check(&cells).is_ok() {
                    out.push(cells);
                }
            }
        }
        Some(i) => {
            for bit in [VIDE, RAIL, ROUTE, CROIX] {
                if s.dom[i] & bit != 0 {
                    let mut dom = s.dom.clone();
                    dom[i] = bit;
                    search(p, dom, limit, out);
                }
            }
        }
    }
}

/// Résout la grille par la seule logique (sans jamais deviner). Renvoie la
/// difficulté si elle y arrive, `None` si la logique seule ne suffit pas.
pub fn logic_solve(p: &Puzzle) -> Option<u32> {
    let mut s = Solver::new(p)?;
    s.propagate().ok()?;
    let cells = s.solved()?;
    (cells == p.solution).then(|| s.log.iter().map(|d| d.technique.cost()).sum())
}

/// La prochaine déduction que peut faire le joueur, d'après ce qu'il a déjà
/// posé (`None` pour une case encore inconnue). `None` si rien n'est
/// déductible, ou si ce qu'il a posé contient une erreur.
pub fn next_deduction(p: &Puzzle, known: &[Option<Kind>]) -> Option<Deduction> {
    let mut s = Solver::new(p)?;
    for (i, k) in known.iter().enumerate() {
        if let Some(k) = k {
            s.dom[i] &= k.bit();
            if s.dom[i] == 0 {
                return None;
            }
        }
    }
    s.one = true;
    s.propagate().ok()?;
    s.log.first().copied()
}
