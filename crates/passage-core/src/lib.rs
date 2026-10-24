//! Passage à niveau — règles, solveurs et générateur, sans affichage.
//!
//! Une grille N × N, deux voies à tracer : une **voie ferrée** et une
//! **route**, chacune d'une gare (ou d'un village) de départ à une
//! d'arrivée, continue, sans bifurcation et sans jamais se toucher
//! elle-même. Elles ne se croisent qu'aux **passages à niveau**, des cases
//! marquées où elles se coupent à angle droit. Les chiffres des bords
//! donnent le nombre de cases de rail et de route de chaque rangée et de
//! chaque colonne. Chaque grille a une seule solution, trouvable par la
//! logique.
//!
//! Partagé par le jeu (`games/passage-a-niveau`) et le générateur de
//! grilles (`tools/passage-gen`).

mod logic;
mod rng;

pub use logic::{Deduction, Technique, logic_solve, next_deduction, solutions};
pub use rng::Rng;

/// Taille maximale d'une grille.
pub const MAX: usize = 9;

/// Les quatre côtés d'une case : nord, est, sud, ouest (rangée, colonne).
pub const DIRS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

/// Ce qu'une case contient dans la solution (et dans la partie).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Vide,
    Rail,
    Route,
    /// Passage à niveau : un rail et une route qui se croisent.
    Croisement,
}

/// Les types de case comme masques de bits, pour les domaines du solveur.
pub const VIDE: u8 = 1;
pub const RAIL: u8 = 2;
pub const ROUTE: u8 = 4;
pub const CROIX: u8 = 8;

impl Kind {
    pub const fn bit(self) -> u8 {
        match self {
            Kind::Vide => VIDE,
            Kind::Rail => RAIL,
            Kind::Route => ROUTE,
            Kind::Croisement => CROIX,
        }
    }

    pub fn from_bit(bit: u8) -> Option<Kind> {
        match bit {
            VIDE => Some(Kind::Vide),
            RAIL => Some(Kind::Rail),
            ROUTE => Some(Kind::Route),
            CROIX => Some(Kind::Croisement),
            _ => None,
        }
    }

    pub fn to_char(self) -> char {
        match self {
            Kind::Vide => '.',
            Kind::Rail => 'r',
            Kind::Route => 'o',
            Kind::Croisement => 'x',
        }
    }

    pub fn from_char(c: char) -> Option<Kind> {
        match c {
            '.' => Some(Kind::Vide),
            'r' => Some(Kind::Rail),
            'o' => Some(Kind::Route),
            'x' => Some(Kind::Croisement),
            _ => None,
        }
    }
}

/// Les deux voies : la voie ferrée et la route.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Network {
    Rail = 0,
    Route = 1,
}

impl Network {
    pub const ALL: [Network; 2] = [Network::Rail, Network::Route];

    /// Les types de case qui font partie de cette voie (le passage à niveau
    /// fait partie des deux).
    pub const fn member_mask(self) -> u8 {
        match self {
            Network::Rail => RAIL | CROIX,
            Network::Route => ROUTE | CROIX,
        }
    }

    /// Le type de case d'une gare (ou d'un village) de cette voie.
    pub const fn pure_kind(self) -> Kind {
        match self {
            Network::Rail => Kind::Rail,
            Network::Route => Kind::Route,
        }
    }
}

/// Une entrée de voie sur un bord de la grille : la case et le côté (0 nord,
/// 1 est, 2 sud, 3 ouest) par lequel la voie arrive de l'extérieur.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Gate {
    pub row: usize,
    pub col: usize,
    pub side: usize,
}

impl Gate {
    pub fn cell(self, n: usize) -> usize {
        self.row * n + self.col
    }

    /// Le côté est-il bien sur un bord de la grille ?
    fn on_border(self, n: usize) -> bool {
        match self.side {
            0 => self.row == 0,
            1 => self.col == n - 1,
            2 => self.row == n - 1,
            3 => self.col == 0,
            _ => false,
        }
    }
}

/// Pourquoi une grille remplie n'est pas une solution.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fault {
    /// Une gare n'est pas sur un bord, ou pas sur une case de sa voie.
    Gate(Network),
    /// Une case de la voie n'a pas le bon nombre de voisines : elle
    /// bifurque, se touche elle-même, ou s'arrête.
    Degree { cell: usize, network: Network },
    /// Un passage à niveau où les deux voies ne se croisent pas à angle droit.
    Crossing(usize),
    /// La voie n'est pas d'un seul tenant entre ses deux gares.
    Broken(Network),
    /// Une rangée ou une colonne n'a pas le bon nombre de cases.
    Count,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Puzzle {
    pub n: usize,
    /// Les deux gares de chaque voie, indexées par `Network as usize`.
    pub gates: [[Gate; 2]; 2],
    /// La solution, rangée par rangée.
    pub solution: Vec<Kind>,
    /// Les cases dévoilées au départ.
    pub givens: Vec<bool>,
    /// Nombre de cases de chaque voie par rangée : `row_counts[réseau][rangée]`.
    pub row_counts: [Vec<u8>; 2],
    /// Nombre de cases de chaque voie par colonne.
    pub col_counts: [Vec<u8>; 2],
    /// Difficulté estimée (plus c'est grand, plus c'est dur).
    pub difficulty: u32,
}

/// Les voisines d'une case (nord, est, sud, ouest), `None` hors de la grille.
pub fn neighbors(n: usize, cell: usize) -> [Option<usize>; 4] {
    let (r, c) = ((cell / n) as isize, (cell % n) as isize);
    DIRS.map(|(dr, dc)| {
        let (nr, nc) = (r + dr, c + dc);
        (nr >= 0 && nc >= 0 && (nr as usize) < n && (nc as usize) < n).then(|| nr as usize * n + nc as usize)
    })
}

fn is_member(kind: Kind, network: Network) -> bool {
    kind.bit() & network.member_mask() != 0
}

/// Vérifie que `cells` forme bien deux voies valides entre les gares données,
/// sans regarder les chiffres des bords.
pub fn check_structure(n: usize, gates: &[[Gate; 2]; 2], cells: &[Kind]) -> Result<(), Fault> {
    for network in Network::ALL {
        let [a, b] = gates[network as usize];
        for g in [a, b] {
            if g.row >= n || g.col >= n || !g.on_border(n) || cells[g.cell(n)] != network.pure_kind() {
                return Err(Fault::Gate(network));
            }
        }
        if a.cell(n) == b.cell(n) {
            return Err(Fault::Gate(network));
        }
    }
    for network in Network::ALL {
        let ends = gates[network as usize].map(|g| g.cell(n));
        let mut total = 0;
        for (i, &kind) in cells.iter().enumerate() {
            if !is_member(kind, network) {
                continue;
            }
            total += 1;
            let degree = neighbors(n, i).iter().flatten().filter(|&&j| is_member(cells[j], network)).count();
            let needed = if ends.contains(&i) { 1 } else { 2 };
            if degree != needed {
                return Err(Fault::Degree { cell: i, network });
            }
        }
        // On suit la voie de la première gare : elle doit atteindre la
        // seconde en passant par toutes les cases.
        let (mut prev, mut cur, mut seen) = (usize::MAX, ends[0], 1);
        while cur != ends[1] {
            let next = neighbors(n, cur)
                .iter()
                .flatten()
                .copied()
                .find(|&j| j != prev && is_member(cells[j], network));
            let Some(next) = next else { return Err(Fault::Broken(network)) };
            (prev, cur) = (cur, next);
            seen += 1;
            if seen > total {
                return Err(Fault::Broken(network));
            }
        }
        if seen != total {
            return Err(Fault::Broken(network));
        }
    }
    for (i, &kind) in cells.iter().enumerate() {
        if kind != Kind::Croisement {
            continue;
        }
        let nbs = neighbors(n, i);
        if nbs.iter().any(Option::is_none) {
            return Err(Fault::Crossing(i));
        }
        let side_mask = |network: Network| {
            (0..4).filter(|&d| is_member(cells[nbs[d].unwrap()], network)).fold(0u8, |m, d| m | (1 << d))
        };
        let (rail, route) = (side_mask(Network::Rail), side_mask(Network::Route));
        const VERTICAL: u8 = 0b0101;
        const HORIZONTAL: u8 = 0b1010;
        if !((rail == VERTICAL && route == HORIZONTAL) || (rail == HORIZONTAL && route == VERTICAL)) {
            return Err(Fault::Crossing(i));
        }
    }
    Ok(())
}

impl Puzzle {
    /// Construit une grille à partir de sa solution ; les chiffres des bords
    /// en sont déduits. Refuse une solution qui ne respecte pas les règles.
    pub fn new(
        n: usize,
        gates: [[Gate; 2]; 2],
        solution: Vec<Kind>,
        givens: Vec<bool>,
        difficulty: u32,
    ) -> Result<Puzzle, Fault> {
        assert!(n <= MAX && solution.len() == n * n && givens.len() == n * n);
        check_structure(n, &gates, &solution)?;
        let count = |network: Network, cells: &mut dyn Iterator<Item = usize>| {
            cells.filter(|&i| is_member(solution[i], network)).count() as u8
        };
        let row_counts = Network::ALL.map(|net| (0..n).map(|r| count(net, &mut (0..n).map(|c| r * n + c))).collect());
        let col_counts = Network::ALL.map(|net| (0..n).map(|c| count(net, &mut (0..n).map(|r| r * n + c))).collect());
        Ok(Puzzle { n, gates, solution, givens, row_counts, col_counts, difficulty })
    }

    pub fn kind(&self, row: usize, col: usize) -> Kind {
        self.solution[row * self.n + col]
    }

    /// Cette case est-elle un passage à niveau (marqué dès le départ) ?
    pub fn is_crossing(&self, cell: usize) -> bool {
        self.solution[cell] == Kind::Croisement
    }

    /// Une ligne de texte : `n gares-rail gares-route solution dévoilées difficulté`.
    /// Une gare s'écrit en trois chiffres (rangée, colonne, côté), deux gares
    /// sont séparées par un point, par exemple `203.451`.
    pub fn to_line(&self) -> String {
        let gates = |net: Network| {
            self.gates[net as usize].map(|g| format!("{}{}{}", g.row, g.col, g.side)).join(".")
        };
        let solution: String = self.solution.iter().map(|k| k.to_char()).collect();
        let givens: String = self.givens.iter().map(|&g| if g { '1' } else { '0' }).collect();
        format!(
            "{} {} {} {} {} {}",
            self.n,
            gates(Network::Rail),
            gates(Network::Route),
            solution,
            givens,
            self.difficulty
        )
    }

    pub fn from_line(line: &str) -> Option<Puzzle> {
        let mut parts = line.split_whitespace();
        let n: usize = parts.next()?.parse().ok()?;
        let mut gates = [[Gate { row: 0, col: 0, side: 0 }; 2]; 2];
        for net in Network::ALL {
            let text = parts.next()?;
            let mut it = text.split('.');
            for k in 0..2 {
                let d: Vec<usize> = it.next()?.chars().map(|c| c.to_digit(10).map(|v| v as usize)).collect::<Option<_>>()?;
                let [row, col, side] = d[..] else { return None };
                gates[net as usize][k] = Gate { row, col, side };
            }
        }
        let solution: Vec<Kind> = parts.next()?.chars().map(Kind::from_char).collect::<Option<_>>()?;
        let givens: Vec<bool> = parts.next()?.chars().map(|c| c == '1').collect();
        let difficulty = parts.next()?.parse().ok()?;
        if n == 0 || n > MAX || solution.len() != n * n || givens.len() != n * n {
            return None;
        }
        Puzzle::new(n, gates, solution, givens, difficulty).ok()
    }

    /// La grille remplie par le joueur est-elle la solution ? (La solution
    /// est unique : il n'y en a pas d'autre.)
    pub fn is_solved_by(&self, cells: &[Kind]) -> bool {
        cells == self.solution.as_slice()
    }

    /// Vérifie une grille remplie : règles de tracé, puis chiffres des bords.
    pub fn check(&self, cells: &[Kind]) -> Result<(), Fault> {
        check_structure(self.n, &self.gates, cells)?;
        let n = self.n;
        for net in Network::ALL {
            for k in 0..n {
                let row = (0..n).filter(|&c| is_member(cells[k * n + c], net)).count();
                let col = (0..n).filter(|&r| is_member(cells[r * n + k], net)).count();
                if row != self.row_counts[net as usize][k] as usize || col != self.col_counts[net as usize][k] as usize {
                    return Err(Fault::Count);
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests;
