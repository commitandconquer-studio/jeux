//! Les règles de 2048, sans rien d'affiché : glisser, fusionner, score.
//!
//! Coordonnées : x de gauche à droite, y de haut en bas (comme à l'écran).
use bevy::math::IVec2;
use game_common::Rng;

pub const SIZE: i32 = 4;

/// Une tuile, avec un numéro unique pour suivre ses déplacements à l'écran.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Tile {
    pub id: u32,
    pub value: u32,
}

/// Une fusion : `kept` double et reste, `absorbed` glisse sur elle et disparaît.
#[derive(Clone, Copy, Debug)]
pub struct Merge {
    pub kept: u32,
    pub absorbed: u32,
    pub value: u32,
}

#[derive(Default, Debug)]
pub struct MoveResult {
    pub changed: bool,
    pub merges: Vec<Merge>,
    /// Points gagnés (somme des tuiles créées par fusion).
    pub points: u32,
}

#[derive(Clone, Debug)]
pub struct Grid {
    cells: [[Option<Tile>; SIZE as usize]; SIZE as usize],
    next_id: u32,
}

impl Default for Grid {
    fn default() -> Self {
        Grid { cells: [[None; SIZE as usize]; SIZE as usize], next_id: 1 }
    }
}

impl Grid {
    pub fn get(&self, p: IVec2) -> Option<Tile> {
        self.cells[p.y as usize][p.x as usize]
    }

    fn set(&mut self, p: IVec2, t: Option<Tile>) {
        self.cells[p.y as usize][p.x as usize] = t;
    }

    /// Toutes les tuiles avec leur position.
    pub fn tiles(&self) -> impl Iterator<Item = (IVec2, Tile)> + '_ {
        (0..SIZE).flat_map(move |y| (0..SIZE).filter_map(move |x| self.get(IVec2::new(x, y)).map(|t| (IVec2::new(x, y), t))))
    }

    pub fn position_of(&self, id: u32) -> Option<IVec2> {
        self.tiles().find(|(_, t)| t.id == id).map(|(p, _)| p)
    }

    pub fn max_value(&self) -> u32 {
        self.tiles().map(|(_, t)| t.value).max().unwrap_or(0)
    }

    /// Pose une tuile (pour les tests et le départ).
    pub fn place(&mut self, p: IVec2, value: u32) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        self.set(p, Some(Tile { id, value }));
        id
    }

    /// Fait apparaître un 2 (ou parfois un 4) sur une case vide.
    pub fn spawn_random(&mut self, rng: &mut Rng) -> Option<(IVec2, Tile)> {
        let empty: Vec<IVec2> = (0..SIZE).flat_map(|y| (0..SIZE).map(move |x| IVec2::new(x, y))).filter(|&p| self.get(p).is_none()).collect();
        if empty.is_empty() {
            return None;
        }
        let p = empty[rng.below(empty.len() as u32) as usize];
        let value = if rng.below(10) == 0 { 4 } else { 2 };
        let id = self.place(p, value);
        Some((p, Tile { id, value }))
    }

    /// Fait tout glisser dans la direction `dir`, en fusionnant les paires.
    pub fn slide(&mut self, dir: IVec2) -> MoveResult {
        let mut result = MoveResult::default();
        for line in 0..SIZE {
            // Cases de la ligne, en partant du bord vers lequel on pousse.
            let cells: Vec<IVec2> = (0..SIZE)
                .map(|k| match (dir.x, dir.y) {
                    (1, _) => IVec2::new(SIZE - 1 - k, line),
                    (-1, _) => IVec2::new(k, line),
                    (_, 1) => IVec2::new(line, SIZE - 1 - k),
                    _ => IVec2::new(line, k),
                })
                .collect();
            let tiles: Vec<Tile> = cells.iter().filter_map(|&p| self.get(p)).collect();
            let mut packed: Vec<(Tile, bool)> = Vec::new(); // (tuile, déjà fusionnée ?)
            for t in tiles {
                match packed.last_mut() {
                    Some((last, false)) if last.value == t.value => {
                        last.value *= 2;
                        result.points += last.value;
                        result.merges.push(Merge { kept: last.id, absorbed: t.id, value: last.value });
                        packed.last_mut().unwrap().1 = true;
                    }
                    _ => packed.push((t, false)),
                }
            }
            for (k, &p) in cells.iter().enumerate() {
                let new = packed.get(k).map(|(t, _)| *t);
                if self.get(p) != new {
                    result.changed = true;
                }
                self.set(p, new);
            }
        }
        result
    }

    /// Reste-t-il un coup possible ?
    pub fn can_move(&self) -> bool {
        self.tiles().count() < (SIZE * SIZE) as usize
            || self.tiles().any(|(p, t)| [IVec2::X, IVec2::Y].iter().any(|&d| {
                let q = p + d;
                q.x < SIZE && q.y < SIZE && self.get(q).map(|u| u.value) == Some(t.value)
            }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn row(grid: &Grid, y: i32) -> Vec<u32> {
        (0..SIZE).map(|x| grid.get(IVec2::new(x, y)).map(|t| t.value).unwrap_or(0)).collect()
    }

    #[test]
    fn deux_tuiles_egales_fusionnent() {
        let mut g = Grid::default();
        g.place(IVec2::new(0, 0), 2);
        g.place(IVec2::new(3, 0), 2);
        let r = g.slide(IVec2::NEG_X);
        assert_eq!(row(&g, 0), vec![4, 0, 0, 0]);
        assert_eq!(r.points, 4);
        assert!(r.changed);
    }

    #[test]
    fn une_tuile_ne_fusionne_qu_une_fois() {
        let mut g = Grid::default();
        for x in 0..4 {
            g.place(IVec2::new(x, 0), 2);
        }
        g.slide(IVec2::NEG_X);
        assert_eq!(row(&g, 0), vec![4, 4, 0, 0]);
    }

    #[test]
    fn trois_egales_fusionnent_du_cote_pousse() {
        let mut g = Grid::default();
        g.place(IVec2::new(0, 1), 2);
        g.place(IVec2::new(1, 1), 2);
        g.place(IVec2::new(2, 1), 2);
        g.slide(IVec2::X);
        assert_eq!(row(&g, 1), vec![0, 0, 2, 4]);
    }

    #[test]
    fn glisser_vers_le_bas() {
        let mut g = Grid::default();
        g.place(IVec2::new(2, 0), 8);
        g.place(IVec2::new(2, 2), 8);
        g.slide(IVec2::Y);
        assert_eq!(g.get(IVec2::new(2, 3)).map(|t| t.value), Some(16));
        assert_eq!(g.get(IVec2::new(2, 0)), None);
    }

    #[test]
    fn rien_ne_bouge_contre_le_mur() {
        let mut g = Grid::default();
        g.place(IVec2::new(0, 0), 2);
        g.place(IVec2::new(1, 0), 4);
        assert!(!g.slide(IVec2::NEG_X).changed);
    }

    #[test]
    fn grille_pleine_sans_paire_est_bloquee() {
        let mut g = Grid::default();
        let mut v = 2;
        for y in 0..4 {
            for x in 0..4 {
                g.place(IVec2::new(x, y), if (x + y) % 2 == 0 { v } else { v * 2 });
            }
            v *= 4;
        }
        assert!(!g.can_move());
    }
}
