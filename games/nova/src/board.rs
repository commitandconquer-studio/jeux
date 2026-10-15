//! Les règles du bouclier (le puits), sans rien d'affiché : pièces, rotation,
//! lignes complètes, rangées endommagées par l'ennemi.
use bevy::math::IVec2;
use game_common::Rng;

pub const WIDTH: i32 = 12;
pub const HEIGHT: i32 = 20;
/// Lignes cachées au-dessus du puits, où apparaissent les pièces.
const SPAWN_ROOM: i32 = 3;

/// Les 7 formes de 4 blocs.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Kind {
    I,
    O,
    T,
    S,
    Z,
    J,
    L,
}

impl Kind {
    pub const ALL: [Kind; 7] = [Kind::I, Kind::O, Kind::T, Kind::S, Kind::Z, Kind::J, Kind::L];

    /// Cases occupées dans l'orientation de départ (y vers le haut).
    fn shape(self) -> [IVec2; 4] {
        let c = |x, y| IVec2::new(x, y);
        match self {
            Kind::I => [c(-1, 0), c(0, 0), c(1, 0), c(2, 0)],
            Kind::O => [c(0, 0), c(1, 0), c(0, 1), c(1, 1)],
            Kind::T => [c(-1, 0), c(0, 0), c(1, 0), c(0, 1)],
            Kind::S => [c(-1, 0), c(0, 0), c(0, 1), c(1, 1)],
            Kind::Z => [c(-1, 1), c(0, 1), c(0, 0), c(1, 0)],
            Kind::J => [c(-1, 1), c(-1, 0), c(0, 0), c(1, 0)],
            Kind::L => [c(-1, 0), c(0, 0), c(1, 0), c(1, 1)],
        }
    }

    pub fn index(self) -> u8 {
        Kind::ALL.iter().position(|k| *k == self).unwrap() as u8
    }
}

/// Contenu d'une case du bouclier.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Cell {
    Empty,
    /// Bloc d'énergie posé (indice de la forme, pour la couleur).
    Block(u8),
    /// Bloc endommagé envoyé par un tir ennemi.
    Damaged,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Piece {
    pub kind: Kind,
    /// Nombre de quarts de tour (sens horaire).
    pub rot: u8,
    pub pos: IVec2,
}

impl Piece {
    pub fn new(kind: Kind) -> Self {
        Piece { kind, rot: 0, pos: IVec2::new(WIDTH / 2 - 1, HEIGHT) }
    }

    /// Cases occupées par la pièce dans le puits.
    pub fn cells(&self) -> [IVec2; 4] {
        self.kind.shape().map(|mut c| {
            if self.kind != Kind::O {
                for _ in 0..self.rot % 4 {
                    c = IVec2::new(c.y, -c.x); // quart de tour horaire
                }
            }
            c + self.pos
        })
    }
}

#[derive(Clone)]
pub struct Board {
    cells: Vec<Cell>,
}

impl Default for Board {
    fn default() -> Self {
        Board { cells: vec![Cell::Empty; (WIDTH * (HEIGHT + SPAWN_ROOM)) as usize] }
    }
}

impl Board {
    fn idx(p: IVec2) -> Option<usize> {
        let inside = p.x >= 0 && p.x < WIDTH && p.y >= 0 && p.y < HEIGHT + SPAWN_ROOM;
        inside.then(|| (p.y * WIDTH + p.x) as usize)
    }

    pub fn get(&self, p: IVec2) -> Cell {
        Self::idx(p).map(|i| self.cells[i]).unwrap_or(Cell::Damaged)
    }

    fn set(&mut self, p: IVec2, c: Cell) {
        if let Some(i) = Self::idx(p) {
            self.cells[i] = c;
        }
    }

    /// La pièce est-elle dans le puits, sans chevaucher de bloc ?
    pub fn fits(&self, piece: &Piece) -> bool {
        piece.cells().iter().all(|&c| self.get(c) == Cell::Empty)
    }

    /// Déplace si possible.
    pub fn shifted(&self, piece: &Piece, d: IVec2) -> Option<Piece> {
        let moved = Piece { pos: piece.pos + d, ..*piece };
        self.fits(&moved).then_some(moved)
    }

    /// Tourne d'un quart de tour ; si ça cogne, essaie de décaler un peu
    /// (contre un mur ou un bloc), comme on le ferait à la main.
    pub fn rotated(&self, piece: &Piece) -> Option<Piece> {
        let turned = Piece { rot: (piece.rot + 1) % 4, ..*piece };
        [IVec2::ZERO, IVec2::X, IVec2::NEG_X, IVec2::new(2, 0), IVec2::new(-2, 0), IVec2::Y]
            .into_iter()
            .map(|d| Piece { pos: turned.pos + d, ..turned })
            .find(|p| self.fits(p))
    }

    /// Position d'arrivée en tombant tout droit.
    pub fn drop_position(&self, piece: &Piece) -> Piece {
        let mut p = *piece;
        while let Some(lower) = self.shifted(&p, IVec2::NEG_Y) {
            p = lower;
        }
        p
    }

    /// Pose la pièce et retire les lignes complètes ; renvoie leurs numéros
    /// (du bas vers le haut, avant suppression).
    pub fn lock(&mut self, piece: &Piece) -> Vec<i32> {
        for c in piece.cells() {
            self.set(c, Cell::Block(piece.kind.index()));
        }
        let full: Vec<i32> = (0..HEIGHT + SPAWN_ROOM)
            .filter(|&y| (0..WIDTH).all(|x| self.get(IVec2::new(x, y)) != Cell::Empty))
            .collect();
        for &y in full.iter().rev() {
            self.remove_row(y);
        }
        full
    }

    fn remove_row(&mut self, row: i32) {
        for y in row..HEIGHT + SPAWN_ROOM {
            for x in 0..WIDTH {
                let above = self.get(IVec2::new(x, y + 1));
                let above = if y + 1 >= HEIGHT + SPAWN_ROOM { Cell::Empty } else { above };
                self.set(IVec2::new(x, y), above);
            }
        }
    }

    /// Le tir ennemi pousse une rangée de blocs endommagés par le bas, avec
    /// un trou. Renvoie `false` si le bouclier déborde (perdu).
    pub fn add_damaged_row(&mut self, hole: i32) -> bool {
        let overflow = (0..WIDTH).any(|x| self.get(IVec2::new(x, HEIGHT - 1)) != Cell::Empty);
        for y in (1..HEIGHT + SPAWN_ROOM).rev() {
            for x in 0..WIDTH {
                let below = self.get(IVec2::new(x, y - 1));
                self.set(IVec2::new(x, y), below);
            }
        }
        for x in 0..WIDTH {
            self.set(IVec2::new(x, 0), if x == hole { Cell::Empty } else { Cell::Damaged });
        }
        !overflow
    }

    /// Des blocs dépassent-ils du haut du puits ?
    pub fn overflowing(&self) -> bool {
        (HEIGHT..HEIGHT + SPAWN_ROOM).any(|y| (0..WIDTH).any(|x| self.get(IVec2::new(x, y)) != Cell::Empty))
    }

    /// Toutes les cases non vides visibles.
    pub fn filled(&self) -> impl Iterator<Item = (IVec2, Cell)> + '_ {
        (0..HEIGHT).flat_map(move |y| {
            (0..WIDTH).filter_map(move |x| {
                let p = IVec2::new(x, y);
                let c = self.get(p);
                (c != Cell::Empty).then_some((p, c))
            })
        })
    }
}

/// Tirage « par sac » : les 7 formes, mélangées, puis un nouveau sac.
pub struct Bag {
    queue: Vec<Kind>,
}

impl Bag {
    pub fn new() -> Self {
        Bag { queue: Vec::new() }
    }

    pub fn next(&mut self, rng: &mut Rng) -> Kind {
        if self.queue.is_empty() {
            let mut bag = Kind::ALL.to_vec();
            for i in (1..bag.len()).rev() {
                bag.swap(i, rng.below(i as u32 + 1) as usize);
            }
            self.queue = bag;
        }
        self.queue.pop().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fill_row_except(board: &mut Board, y: i32, hole: i32) {
        for x in 0..WIDTH {
            if x != hole {
                board.set(IVec2::new(x, y), Cell::Damaged);
            }
        }
    }

    #[test]
    fn la_piece_tombe_jusqu_au_fond() {
        let board = Board::default();
        let p = board.drop_position(&Piece::new(Kind::O));
        assert_eq!(p.cells().iter().map(|c| c.y).min(), Some(0));
    }

    #[test]
    fn une_ligne_complete_disparait() {
        let mut board = Board::default();
        fill_row_except(&mut board, 0, 5);
        fill_row_except(&mut board, 1, 5);
        board.set(IVec2::new(3, 2), Cell::Damaged);
        // Une barre verticale dans le trou complète les deux lignes du bas.
        let mut i = Piece { kind: Kind::I, rot: 1, pos: IVec2::new(5, 10) };
        assert!(board.fits(&i));
        i = board.drop_position(&i);
        let cleared = board.lock(&i);
        assert_eq!(cleared, vec![0, 1]);
        // Le bloc de la ligne 2 est descendu en ligne 0.
        assert_eq!(board.get(IVec2::new(3, 0)), Cell::Damaged);
    }

    #[test]
    fn la_rotation_se_decale_contre_le_mur() {
        let board = Board::default();
        // Barre verticale collée au mur de gauche : en tournant elle dépasserait.
        let i = Piece { kind: Kind::I, rot: 1, pos: IVec2::new(0, 5) };
        assert!(board.fits(&i));
        let r = board.rotated(&i).expect("la rotation doit trouver une place");
        assert!(r.cells().iter().all(|c| c.x >= 0));
    }

    #[test]
    fn quatre_rotations_reviennent_au_depart() {
        for kind in Kind::ALL {
            let p = Piece { kind, rot: 0, pos: IVec2::new(5, 5) };
            let mut q = p;
            for _ in 0..4 {
                q = Piece { rot: (q.rot + 1) % 4, ..q };
            }
            assert_eq!(p.cells(), q.cells());
        }
    }

    #[test]
    fn le_tir_ennemi_ajoute_une_rangee_trouee() {
        let mut board = Board::default();
        board.set(IVec2::new(0, 0), Cell::Block(0));
        assert!(board.add_damaged_row(4));
        assert_eq!(board.get(IVec2::new(4, 0)), Cell::Empty);
        assert_eq!(board.get(IVec2::new(0, 0)), Cell::Damaged);
        assert_eq!(board.get(IVec2::new(0, 1)), Cell::Block(0));
    }

    #[test]
    fn un_sac_contient_les_sept_formes() {
        let mut rng = Rng::new(3);
        let mut bag = Bag::new();
        let mut seen: Vec<Kind> = (0..7).map(|_| bag.next(&mut rng)).collect();
        seen.sort_by_key(|k| k.index());
        assert_eq!(seen, Kind::ALL.to_vec());
    }
}
