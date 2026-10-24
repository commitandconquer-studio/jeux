//! Passage à niveau — règles et score, sans affichage.
//!
//! Un paquet de 40 cartes de rails et de routes, déterminé par un numéro
//! de partie ; on les pose une à une sur une grille de 7 × 7 et on cherche
//! le meilleur score. Le même numéro donne toujours les mêmes cartes, dans
//! le même ordre.
//!
//! Partagé par le jeu (`games/passage-a-niveau`).

mod rng;

pub use rng::Rng;

/// Côté de la grille.
pub const N: usize = 7;
/// Nombre de cartes d'une partie.
pub const DECK_SIZE: usize = 40;
/// Cartes de chaque sorte dans le paquet.
pub const PER_PIECE: usize = DECK_SIZE / Piece::ALL.len();

/// Points par passage à niveau dont les quatre côtés sont raccordés.
pub const CROSSING_BONUS: i32 = 5;

/// Les côtés d'une carte, du nord dans le sens des aiguilles d'une montre.
pub const NORTH: u8 = 1;
pub const EAST: u8 = 2;
pub const SOUTH: u8 = 4;
pub const WEST: u8 = 8;

/// Les deux voies.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Network {
    Rail,
    Route,
}

impl Network {
    pub const ALL: [Network; 2] = [Network::Rail, Network::Route];
}

/// Les cinq pièces de voie.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Piece {
    RailDroit,
    RailCourbe,
    RouteDroite,
    RouteCourbe,
    /// Passage à niveau : un rail et une route qui se croisent à angle droit.
    Croisement,
}

impl Piece {
    pub const ALL: [Piece; 5] =
        [Piece::RailDroit, Piece::RailCourbe, Piece::RouteDroite, Piece::RouteCourbe, Piece::Croisement];
}

/// Fait tourner un masque de côtés de `quarts` quarts de tour, dans le sens
/// des aiguilles d'une montre.
fn rotate_mask(mask: u8, quarts: u8) -> u8 {
    (0..quarts % 4).fold(mask, |m, _| ((m << 1) | (m >> 3)) & 0b1111)
}

/// Une carte posée : sa pièce et son sens (nombre de quarts de tour).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Card {
    pub piece: Piece,
    pub rot: u8,
}

impl Card {
    pub fn new(piece: Piece) -> Card {
        Card { piece, rot: 0 }
    }

    /// La même carte, tournée d'un quart de tour.
    pub fn rotated(self) -> Card {
        Card { rot: (self.rot + 1) % 4, ..self }
    }

    /// Les côtés ouverts de cette voie sur cette carte (masque de bits).
    pub fn sides(self, network: Network) -> u8 {
        let base = match (self.piece, network) {
            (Piece::RailDroit, Network::Rail) | (Piece::RouteDroite, Network::Route) => NORTH | SOUTH,
            (Piece::RailCourbe, Network::Rail) | (Piece::RouteCourbe, Network::Route) => NORTH | EAST,
            (Piece::Croisement, Network::Rail) => NORTH | SOUTH,
            (Piece::Croisement, Network::Route) => EAST | WEST,
            _ => 0,
        };
        rotate_mask(base, self.rot)
    }
}

/// Le paquet d'une partie : le même numéro donne toujours le même ordre.
pub fn deck(number: u64) -> Vec<Piece> {
    let mut cards: Vec<Piece> = Piece::ALL.iter().flat_map(|&p| std::iter::repeat_n(p, PER_PIECE)).collect();
    Rng::new(number).shuffle(&mut cards);
    cards
}

/// Le numéro « du jour » : la date écrite en un seul nombre (20261024).
pub fn day_number(year: u32, month: u32, day: u32) -> u64 {
    year as u64 * 10_000 + month as u64 * 100 + day as u64
}

/// La case voisine dans la direction `side` (0 nord, 1 est, 2 sud, 3 ouest).
pub fn neighbor(cell: usize, side: usize) -> Option<usize> {
    let (r, c) = (cell / N, cell % N);
    match side {
        0 => r.checked_sub(1).map(|r| r * N + c),
        1 => (c + 1 < N).then(|| r * N + c + 1),
        2 => (r + 1 < N).then(|| (r + 1) * N + c),
        _ => c.checked_sub(1).map(|c| r * N + c),
    }
}

/// Le détail du score d'une grille.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Score {
    /// Raccords entre deux cartes voisines de la même voie.
    pub links: u32,
    /// Cartes de la plus longue voie ferrée d'un seul tenant.
    pub longest_rail: u32,
    /// Cartes de la plus longue route d'un seul tenant.
    pub longest_route: u32,
    /// Passages à niveau dont les quatre côtés sont raccordés.
    pub crossings: u32,
    /// Côtés ouverts qui n'arrivent sur rien (case vide, bord, ou une
    /// carte qui ne s'y raccorde pas).
    pub open_ends: u32,
}

impl Score {
    pub fn total(&self) -> i32 {
        (self.links + self.longest_rail + self.longest_route) as i32 + CROSSING_BONUS * self.crossings as i32
            - self.open_ends as i32
    }

    /// Une à trois étoiles (zéro en dessous de la première marche).
    pub fn stars(&self) -> u32 {
        let total = self.total();
        (total >= STARS[0]) as u32 + (total >= STARS[1]) as u32 + (total >= STARS[2]) as u32
    }
}

/// Scores à atteindre pour une, deux et trois étoiles.
/// Réglés d'après le joueur glouton (moyenne 35 sur 200 numéros) ; à ajuster à l'essai.
pub const STARS: [i32; 3] = [25, 40, 55];

/// La grille de jeu.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Board {
    pub cells: Vec<Option<Card>>,
}

impl Default for Board {
    fn default() -> Self {
        Board { cells: vec![None; N * N] }
    }
}

impl Board {
    pub fn get(&self, row: usize, col: usize) -> Option<Card> {
        self.cells[row * N + col]
    }

    /// Pose une carte sur une case libre. Faux si la case est prise.
    pub fn place(&mut self, cell: usize, card: Card) -> bool {
        if self.cells[cell].is_some() {
            return false;
        }
        self.cells[cell] = Some(card);
        true
    }

    pub fn remove(&mut self, cell: usize) -> Option<Card> {
        self.cells[cell].take()
    }

    /// Ce côté de cette case est-il raccordé à une carte voisine de la même voie ?
    pub fn is_linked(&self, cell: usize, side: usize, network: Network) -> bool {
        let Some(card) = self.cells[cell] else { return false };
        if card.sides(network) & (1 << side) == 0 {
            return false;
        }
        neighbor(cell, side)
            .and_then(|j| self.cells[j])
            .is_some_and(|other| other.sides(network) & (1 << ((side + 2) % 4)) != 0)
    }

    pub fn score(&self) -> Score {
        let mut score = Score::default();
        let mut link_sides = 0;
        for network in Network::ALL {
            for (cell, card) in self.cells.iter().enumerate() {
                let Some(card) = card else { continue };
                for side in 0..4 {
                    if card.sides(network) & (1 << side) == 0 {
                        continue;
                    }
                    if self.is_linked(cell, side, network) {
                        link_sides += 1;
                    } else {
                        score.open_ends += 1;
                    }
                }
            }
            let longest = self.longest(network);
            match network {
                Network::Rail => score.longest_rail = longest,
                Network::Route => score.longest_route = longest,
            }
        }
        // Chaque raccord a été compté une fois par côté, donc deux fois.
        score.links = link_sides / 2;
        score.crossings = (0..N * N)
            .filter(|&cell| {
                self.cells[cell].is_some_and(|c| c.piece == Piece::Croisement)
                    && Network::ALL.iter().all(|&net| {
                        (0..4).filter(|&s| self.cells[cell].unwrap().sides(net) & (1 << s) != 0)
                            .all(|s| self.is_linked(cell, s, net))
                    })
            })
            .count() as u32;
        score
    }

    /// Taille du plus grand groupe de cartes raccordées d'une même voie.
    fn longest(&self, network: Network) -> u32 {
        let mut seen = [false; N * N];
        let mut best = 0;
        for start in 0..N * N {
            let on_network = |i: usize| self.cells[i].is_some_and(|c| c.sides(network) != 0);
            if seen[start] || !on_network(start) {
                continue;
            }
            let (mut stack, mut size) = (vec![start], 0);
            seen[start] = true;
            while let Some(i) = stack.pop() {
                size += 1;
                for side in 0..4 {
                    if !self.is_linked(i, side, network) {
                        continue;
                    }
                    let j = neighbor(i, side).unwrap();
                    if !seen[j] {
                        seen[j] = true;
                        stack.push(j);
                    }
                }
            }
            best = best.max(size);
        }
        best
    }
}

/// Une partie en cours : un numéro, le paquet, la grille, l'historique.
#[derive(Clone, Debug)]
pub struct Game {
    pub number: u64,
    pub deck: Vec<Piece>,
    pub board: Board,
    /// Cases où l'on a posé les cartes, dans l'ordre (pour annuler).
    pub history: Vec<usize>,
}

impl Game {
    pub fn new(number: u64) -> Game {
        Game { number, deck: deck(number), board: Board::default(), history: Vec::new() }
    }

    /// Combien de cartes ont déjà été posées.
    pub fn played(&self) -> usize {
        self.history.len()
    }

    pub fn is_finished(&self) -> bool {
        self.played() >= DECK_SIZE
    }

    /// La carte à poser maintenant.
    pub fn current(&self) -> Option<Piece> {
        self.deck.get(self.played()).copied()
    }

    /// Les deux cartes suivantes (moins s'il n'en reste plus).
    pub fn upcoming(&self) -> &[Piece] {
        let from = (self.played() + 1).min(DECK_SIZE);
        &self.deck[from..(from + 2).min(DECK_SIZE)]
    }

    /// Pose la carte du dessus sur `cell`, tournée de `rot` quarts de tour.
    /// Faux si la partie est finie ou si la case est prise.
    pub fn play(&mut self, cell: usize, rot: u8) -> bool {
        let Some(piece) = self.current() else { return false };
        if !self.board.place(cell, Card { piece, rot: rot % 4 }) {
            return false;
        }
        self.history.push(cell);
        true
    }

    /// Reprend la dernière carte posée.
    pub fn undo(&mut self) -> bool {
        match self.history.pop() {
            Some(cell) => {
                self.board.remove(cell);
                true
            }
            None => false,
        }
    }

    pub fn score(&self) -> Score {
        self.board.score()
    }
}

#[cfg(test)]
mod tests;
