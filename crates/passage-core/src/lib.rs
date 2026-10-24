//! Passage à niveau — règles et score, sans affichage.
//!
//! Un paquet de 40 cartes de rails, de routes, de passages à niveau, de
//! gares routières et de culs-de-sac, déterminé par un numéro de partie ;
//! on les pose une à une sur une grille de 7 × 7 où quatre pièces de départ
//! (deux gares, deux parkings) sont déjà posées, et on cherche le meilleur
//! score. Le même numéro donne toujours les mêmes cartes, dans le même ordre.
//!
//! Partagé par le jeu (`games/passage-a-niveau`).

mod rng;

pub use rng::Rng;

/// Côté de la grille.
pub const N: usize = 7;
/// Nombre de cartes d'une partie.
pub const DECK_SIZE: usize = 40;

/// Points par passage à niveau dont les quatre côtés sont raccordés.
pub const CROSSING_BONUS: i32 = 5;
/// Points par gare routière dont toutes les entrées sont raccordées.
pub const STATION_BONUS: i32 = 4;
/// Points par liaison entre les pièces de départ (gares, parkings, ensemble).
pub const START_BONUS: i32 = 6;

/// Les côtés d'une carte, du nord dans le sens des aiguilles d'une montre.
pub const NORTH: u8 = 1;
pub const EAST: u8 = 2;
pub const SOUTH: u8 = 4;
pub const WEST: u8 = 8;

/// Les deux voies.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Network {
    Rail = 0,
    Route = 1,
}

impl Network {
    pub const ALL: [Network; 2] = [Network::Rail, Network::Route];
}

/// Les pièces : celles du paquet, puis les deux pièces de départ.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Piece {
    RailDroit,
    RailCourbe,
    RailT,
    RailCroix,
    RouteDroite,
    RouteCourbe,
    RouteT,
    RouteCroix,
    /// Passage à niveau : un rail et une route qui se croisent sans se raccorder.
    Croisement,
    /// Gare routière à 2 entrées : un rail et une route, en face l'un de l'autre.
    Gare2Droite,
    /// Gare routière à 2 entrées : un rail et une route, en coude.
    Gare2Courbe,
    /// Gare routière à 3 entrées : le rail passe, la route s'y raccorde.
    Gare3Rail,
    /// Gare routière à 3 entrées : la route passe, le rail s'y raccorde.
    Gare3Route,
    /// Gare routière à 4 entrées : rail et route se croisent en se raccordant.
    Gare4,
    /// Cul-de-sac de rail.
    Impasse,
    /// Cul-de-sac de route.
    RondPoint,
    /// Pièce de départ : une gare (entrée de rail).
    GareDepart,
    /// Pièce de départ : un parking (entrée de route).
    Parking,
}

impl Piece {
    /// Les pièces du paquet et leur nombre d'exemplaires (40 en tout).
    pub const DECK: [(Piece, usize); 16] = [
        (Piece::RailDroit, 6),
        (Piece::RailCourbe, 4),
        (Piece::RailT, 2),
        (Piece::RailCroix, 1),
        (Piece::RouteDroite, 6),
        (Piece::RouteCourbe, 4),
        (Piece::RouteT, 2),
        (Piece::RouteCroix, 1),
        (Piece::Croisement, 2),
        (Piece::Gare2Droite, 2),
        (Piece::Gare2Courbe, 1),
        (Piece::Gare3Rail, 1),
        (Piece::Gare3Route, 1),
        (Piece::Gare4, 1),
        (Piece::Impasse, 3),
        (Piece::RondPoint, 3),
    ];

    /// Côtés de rail et de route (sens de base), et si les deux voies sont
    /// raccordées à l'intérieur de la carte (gare routière).
    const fn base(self) -> (u8, u8, bool) {
        match self {
            Piece::RailDroit => (NORTH | SOUTH, 0, false),
            Piece::RailCourbe => (NORTH | EAST, 0, false),
            Piece::RailT => (NORTH | EAST | WEST, 0, false),
            Piece::RailCroix => (NORTH | EAST | SOUTH | WEST, 0, false),
            Piece::RouteDroite => (0, NORTH | SOUTH, false),
            Piece::RouteCourbe => (0, NORTH | EAST, false),
            Piece::RouteT => (0, NORTH | EAST | WEST, false),
            Piece::RouteCroix => (0, NORTH | EAST | SOUTH | WEST, false),
            Piece::Croisement => (NORTH | SOUTH, EAST | WEST, false),
            Piece::Gare2Droite => (NORTH, SOUTH, true),
            Piece::Gare2Courbe => (NORTH, EAST, true),
            Piece::Gare3Rail => (NORTH | SOUTH, EAST, true),
            Piece::Gare3Route => (EAST, NORTH | SOUTH, true),
            Piece::Gare4 => (NORTH | SOUTH, EAST | WEST, true),
            Piece::Impasse | Piece::GareDepart => (NORTH, 0, false),
            Piece::RondPoint | Piece::Parking => (0, NORTH, false),
        }
    }
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
        let (rail, route, _) = self.piece.base();
        rotate_mask(if network == Network::Rail { rail } else { route }, self.rot)
    }

    /// Gare routière : le rail et la route de la carte sont raccordés entre eux.
    pub fn joined(self) -> bool {
        self.piece.base().2
    }
}

/// Les quatre pièces de départ, toujours aux mêmes places : la case, la
/// pièce et son sens (l'entrée regarde vers l'intérieur de la grille).
pub const STARTS: [(usize, Piece, u8); 4] = [
    (7, Piece::GareDepart, 1),  // bord ouest, rangée 2, entrée à l'est
    (41, Piece::GareDepart, 3), // bord est, rangée 6, entrée à l'ouest
    (5, Piece::Parking, 2),     // bord nord, colonne 6, entrée au sud
    (43, Piece::Parking, 0),    // bord sud, colonne 2, entrée au nord
];

/// Le paquet d'une partie : le même numéro donne toujours le même ordre.
pub fn deck(number: u64) -> Vec<Piece> {
    let mut cards: Vec<Piece> = Piece::DECK.iter().flat_map(|&(p, n)| std::iter::repeat_n(p, n)).collect();
    debug_assert_eq!(cards.len(), DECK_SIZE);
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

/// L'état d'un côté d'une carte posée.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SideStatus {
    /// Ce côté est fermé (rien n'en sort).
    Closed,
    /// Ouvert et raccordé à une carte voisine de la même voie.
    Linked,
    /// Ouvert vers une case vide ou le bord de la grille : une carte à venir
    /// peut encore le raccorder (sinon il coûte un point).
    Open,
    /// Ouvert vers une carte qui ne l'accepte pas (côté fermé, ou autre
    /// voie) : c'est un problème que rien ne pourra réparer.
    Blocked,
}

/// Le détail du score d'une grille.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Score {
    /// Raccords entre deux cartes voisines de la même voie.
    pub links: u32,
    /// Cartes du plus grand groupe de voie ferrée relié.
    pub longest_rail: u32,
    /// Cartes du plus grand groupe de route relié.
    pub longest_route: u32,
    /// Passages à niveau dont les quatre côtés sont raccordés.
    pub crossings: u32,
    /// Gares routières dont toutes les entrées sont raccordées.
    pub stations: u32,
    /// Liaisons entre les pièces de départ (0 à 3) : les deux gares par le
    /// rail, les deux parkings par la route, les quatre dans un même réseau.
    pub starts: u32,
    /// Côtés ouverts qui n'arrivent sur rien (case vide, bord, ou une
    /// carte qui ne s'y raccorde pas).
    pub open_ends: u32,
    /// Parmi eux, ceux qui butent contre une carte qui ne les accepte pas
    /// (côté fermé ou autre voie) : plus rien ne pourra les raccorder.
    pub blocked: u32,
}

impl Score {
    pub fn total(&self) -> i32 {
        (self.links + self.longest_rail + self.longest_route) as i32
            + CROSSING_BONUS * self.crossings as i32
            + STATION_BONUS * self.stations as i32
            + START_BONUS * self.starts as i32
            - self.open_ends as i32
    }

    /// Une à trois étoiles (zéro en dessous de la première marche).
    pub fn stars(&self) -> u32 {
        let total = self.total();
        (total >= STARS[0]) as u32 + (total >= STARS[1]) as u32 + (total >= STARS[2]) as u32
    }
}

/// Scores à atteindre pour une, deux et trois étoiles (à régler d'après le
/// joueur glouton : voir le test d'équilibrage).
pub const STARS: [i32; 3] = [20, 32, 45];

/// La grille de jeu.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Board {
    pub cells: Vec<Option<Card>>,
}

impl Default for Board {
    /// Une grille avec ses quatre pièces de départ.
    fn default() -> Self {
        let mut cells = vec![None; N * N];
        for (cell, piece, rot) in STARTS {
            cells[cell] = Some(Card { piece, rot });
        }
        Board { cells }
    }
}

impl Board {
    /// Une grille entièrement vide, sans pièces de départ (pour les essais).
    pub fn empty() -> Board {
        Board { cells: vec![None; N * N] }
    }

    pub fn get(&self, row: usize, col: usize) -> Option<Card> {
        self.cells[row * N + col]
    }

    /// Cette case porte-t-elle une pièce de départ (imposée) ?
    pub fn is_start(cell: usize) -> bool {
        STARTS.iter().any(|&(c, _, _)| c == cell)
    }

    /// Pose une carte sur une case libre. Faux si la case est prise.
    pub fn place(&mut self, cell: usize, card: Card) -> bool {
        if self.cells[cell].is_some() {
            return false;
        }
        self.cells[cell] = Some(card);
        true
    }

    /// Reprend la carte d'une case (jamais une pièce de départ).
    pub fn remove(&mut self, cell: usize) -> Option<Card> {
        if Board::is_start(cell) {
            return None;
        }
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

    /// L'état d'un côté d'une carte, pour une voie.
    pub fn side_status(&self, cell: usize, side: usize, network: Network) -> SideStatus {
        let Some(card) = self.cells[cell] else { return SideStatus::Closed };
        if card.sides(network) & (1 << side) == 0 {
            return SideStatus::Closed;
        }
        match neighbor(cell, side).and_then(|j| self.cells[j]) {
            None => SideStatus::Open,
            Some(other) if other.sides(network) & (1 << ((side + 2) % 4)) != 0 => SideStatus::Linked,
            Some(_) => SideStatus::Blocked,
        }
    }

    /// Les cartes (côté voie) atteintes en suivant les raccords depuis `start`.
    /// Un nœud est une case et une voie ; avec `joined`, les gares routières
    /// relient leur rail à leur route.
    fn reach(&self, start: (usize, Network), joined: bool) -> Vec<bool> {
        let node = |cell: usize, net: Network| cell * 2 + net as usize;
        let mut seen = vec![false; N * N * 2];
        let mut stack = vec![start];
        seen[node(start.0, start.1)] = true;
        while let Some((cell, net)) = stack.pop() {
            let mut next: Vec<(usize, Network)> =
                (0..4).filter(|&s| self.is_linked(cell, s, net)).map(|s| (neighbor(cell, s).unwrap(), net)).collect();
            if joined && self.cells[cell].is_some_and(|c| c.joined()) {
                next.push((cell, if net == Network::Rail { Network::Route } else { Network::Rail }));
            }
            for (j, n) in next {
                if !seen[node(j, n)] {
                    seen[node(j, n)] = true;
                    stack.push((j, n));
                }
            }
        }
        seen
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
                    match self.side_status(cell, side, network) {
                        SideStatus::Linked => link_sides += 1,
                        SideStatus::Blocked => {
                            score.open_ends += 1;
                            score.blocked += 1;
                        }
                        _ => score.open_ends += 1,
                    }
                }
            }
            let largest = self.largest(network);
            match network {
                Network::Rail => score.longest_rail = largest,
                Network::Route => score.longest_route = largest,
            }
        }
        // Chaque raccord a été compté une fois par côté, donc deux fois.
        score.links = link_sides / 2;
        let all_linked = |cell: usize| {
            let card = self.cells[cell].unwrap();
            Network::ALL.iter().all(|&net| (0..4).filter(|&s| card.sides(net) & (1 << s) != 0).all(|s| self.is_linked(cell, s, net)))
        };
        for cell in (0..N * N).filter(|&c| self.cells[c].is_some()) {
            let card = self.cells[cell].unwrap();
            if card.piece == Piece::Croisement && all_linked(cell) {
                score.crossings += 1;
            }
            if card.joined() && all_linked(cell) {
                score.stations += 1;
            }
        }
        // Les pièces de départ : gares, parkings, et les quatre ensemble.
        let (gares, parkings): (Vec<usize>, Vec<usize>) = (
            STARTS.iter().filter(|s| s.1 == Piece::GareDepart).map(|s| s.0).collect(),
            STARTS.iter().filter(|s| s.1 == Piece::Parking).map(|s| s.0).collect(),
        );
        let node = |cell: usize, net: Network| cell * 2 + net as usize;
        let from_gare = self.reach((gares[0], Network::Rail), false);
        if from_gare[node(gares[1], Network::Rail)] {
            score.starts += 1;
        }
        let from_parking = self.reach((parkings[0], Network::Route), false);
        if from_parking[node(parkings[1], Network::Route)] {
            score.starts += 1;
        }
        let mixed = self.reach((gares[0], Network::Rail), true);
        if mixed[node(gares[1], Network::Rail)] && parkings.iter().all(|&p| mixed[node(p, Network::Route)]) {
            score.starts += 1;
        }
        score
    }

    /// La plus longue suite de cartes qu'un convoi peut parcourir sur cette
    /// voie, sans repasser deux fois par la même case : les cases dans
    /// l'ordre du trajet. Vide s'il n'y a aucune carte de cette voie.
    pub fn longest_path(&self, network: Network) -> Vec<usize> {
        let nodes: Vec<usize> = (0..N * N).filter(|&c| self.cells[c].is_some_and(|k| k.sides(network) != 0)).collect();
        let voisines = |cell: usize| -> Vec<usize> {
            (0..4).filter(|&s| self.is_linked(cell, s, network)).map(|s| neighbor(cell, s).unwrap()).collect()
        };
        let adjacence: Vec<Vec<usize>> = (0..N * N).map(voisines).collect();
        // On part d'abord des bouts (une seule voisine), puis du reste (boucles).
        let mut departs: Vec<usize> = nodes.iter().copied().filter(|&c| adjacence[c].len() <= 1).collect();
        departs.extend(nodes.iter().copied().filter(|&c| adjacence[c].len() > 1));
        let mut meilleur: Vec<usize> = Vec::new();
        let mut budget = 300_000usize;
        fn explorer(cell: usize, adjacence: &[Vec<usize>], chemin: &mut Vec<usize>, pris: &mut [bool], meilleur: &mut Vec<usize>, budget: &mut usize) {
            if chemin.len() > meilleur.len() {
                *meilleur = chemin.clone();
            }
            for &j in &adjacence[cell] {
                if *budget == 0 {
                    return;
                }
                if pris[j] {
                    continue;
                }
                *budget -= 1;
                pris[j] = true;
                chemin.push(j);
                explorer(j, adjacence, chemin, pris, meilleur, budget);
                chemin.pop();
                pris[j] = false;
            }
        }
        for start in departs {
            if budget == 0 {
                break;
            }
            let mut pris = [false; N * N];
            pris[start] = true;
            explorer(start, &adjacence, &mut vec![start], &mut pris, &mut meilleur, &mut budget);
        }
        meilleur
    }

    /// Taille du plus grand groupe de cartes raccordées d'une même voie.
    fn largest(&self, network: Network) -> u32 {
        let mut seen = [false; N * N];
        let mut best = 0;
        for start in 0..N * N {
            if seen[start] || self.cells[start].is_none_or(|c| c.sides(network) == 0) {
                continue;
            }
            let reached = self.reach((start, network), false);
            let mut size = 0;
            for cell in 0..N * N {
                if reached[cell * 2 + network as usize] {
                    seen[cell] = true;
                    size += 1;
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
    /// Le joueur a décidé de s'arrêter avant la fin du paquet.
    pub ended: bool,
}

impl Game {
    pub fn new(number: u64) -> Game {
        Game { number, deck: deck(number), board: Board::default(), history: Vec::new(), ended: false }
    }

    /// Arrête la partie maintenant : « c'est bon, j'ai assez de cartes ».
    pub fn finish(&mut self) {
        self.ended = true;
    }

    /// Combien de cartes ont déjà été posées.
    pub fn played(&self) -> usize {
        self.history.len()
    }

    pub fn is_finished(&self) -> bool {
        self.ended || self.played() >= DECK_SIZE
    }

    /// La carte à poser maintenant.
    pub fn current(&self) -> Option<Piece> {
        if self.ended {
            return None;
        }
        self.deck.get(self.played()).copied()
    }

    /// Les deux cartes suivantes (moins s'il n'en reste plus).
    pub fn upcoming(&self) -> &[Piece] {
        if self.ended {
            return &[];
        }
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
