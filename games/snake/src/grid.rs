//! Niveaux dessinés à la main : une carte de caractères (un par case), lue depuis un fichier texte.
//! Légende (aussi celle de l'éditeur `tools/editeur-niveaux.html`) :
//! `0` vide (non jouable), `1` case libre, `2`…`7` blocs (modèle 0…5 du paysage), `o` trou, `p` ponton,
//! `z` accélérateur, `h`/`H` herse (deux phases), `>` `<` `^` `v` tapis, `A`…`D` téléporteurs (deux de chaque),
//! `S` départ, `E` sortie (si le niveau a une sortie).
use bevy::prelude::*;

use crate::board::Tile;
use crate::layout::Layout;

#[derive(Clone, Debug)]
pub struct Grid {
    pub w: i32,
    pub h: i32,
    pub rows: Vec<Vec<char>>,
    /// Direction du départ.
    pub dir: IVec2,
}

/// Les caractères que la carte accepte.
pub const LEGEND: &str = "01234567ophHz<>^vSEABCD";

impl Grid {
    pub fn from_rows(rows: &[String], dir: IVec2) -> Result<Grid, String> {
        let rows: Vec<Vec<char>> = rows.iter().map(|r| r.chars().collect()).collect();
        let h = rows.len() as i32;
        let w = rows.first().map_or(0, |r| r.len()) as i32;
        if w == 0 || h == 0 {
            return Err("carte vide".into());
        }
        if rows.iter().any(|r| r.len() as i32 != w) {
            return Err("toutes les lignes de la carte doivent avoir la même longueur".into());
        }
        if let Some(c) = rows.iter().flatten().find(|c| !LEGEND.contains(**c)) {
            return Err(format!("caractère inconnu dans la carte : « {c} »"));
        }
        let g = Grid { w, h, rows, dir };
        if g.count('S') != 1 {
            return Err("la carte doit contenir un seul départ « S »".into());
        }
        if g.count('E') > 1 {
            return Err("la carte contient plusieurs sorties « E »".into());
        }
        for t in ['A', 'B', 'C', 'D'] {
            if !matches!(g.count(t), 0 | 2) {
                return Err(format!("le téléporteur « {t} » doit apparaître exactement deux fois"));
            }
        }
        let l = g.layout();
        if !l.start_cells().iter().all(|c| l.walkable(*c)) {
            return Err("le départ n'a pas deux cases libres derrière lui (dans le sens opposé à « depart »)".into());
        }
        if !l.walkable(l.start + l.dir) {
            return Err("un mur juste devant le départ".into());
        }
        Ok(g)
    }

    fn count(&self, ch: char) -> usize {
        self.rows.iter().flatten().filter(|c| **c == ch).count()
    }

    fn find(&self, ch: char) -> Option<IVec2> {
        self.cells().find(|(_, c)| *c == ch).map(|(p, _)| p)
    }

    fn cells(&self) -> impl Iterator<Item = (IVec2, char)> + '_ {
        self.rows.iter().enumerate().flat_map(|(y, r)| r.iter().enumerate().map(move |(x, c)| (IVec2::new(x as i32, y as i32), *c)))
    }

    pub fn exit(&self) -> Option<IVec2> {
        self.find('E')
    }

    pub fn layout(&self) -> Layout {
        let mask = self.rows.iter().flatten().map(|c| *c != '0').collect();
        Layout::from_mask(self.w, self.h, mask, self.find('S').unwrap_or_default(), self.dir)
    }

    /// Les cases spéciales de la carte, et le nombre de paires de téléporteurs.
    pub fn tiles(&self) -> (Vec<(IVec2, Tile)>, u8) {
        let mut out = Vec::new();
        let mut pairs = 0;
        for (p, c) in self.cells() {
            let t = match c {
                '2'..='7' => Tile::Obstacle(c as u8 - b'2'),
                'o' => Tile::Hole,
                'p' => Tile::Plank,
                'z' => Tile::Speeder,
                'h' => Tile::Gate(0),
                'H' => Tile::Gate(1),
                '>' => Tile::Belt(IVec2::X),
                '<' => Tile::Belt(IVec2::NEG_X),
                '^' => Tile::Belt(IVec2::NEG_Y),
                'v' => Tile::Belt(IVec2::Y),
                'A'..='D' => {
                    let id = c as u8 - b'A';
                    pairs = pairs.max(id + 1);
                    Tile::Teleport(id)
                }
                _ => continue,
            };
            out.push((p, t));
        }
        (out, pairs)
    }

    /// Le caractère d'une case spéciale (pour exporter un niveau existant).
    pub fn char_of(t: &Tile) -> Option<char> {
        Some(match t {
            Tile::Obstacle(v) => (b'2' + (*v).min(5)) as char,
            Tile::Hole => 'o',
            Tile::Plank => 'p',
            Tile::Speeder => 'z',
            Tile::Gate(0) => 'h',
            Tile::Gate(_) => 'H',
            Tile::Belt(d) if *d == IVec2::X => '>',
            Tile::Belt(d) if *d == IVec2::NEG_X => '<',
            Tile::Belt(d) if *d == IVec2::NEG_Y => '^',
            Tile::Belt(_) => 'v',
            Tile::Teleport(id) => (b'A' + (*id).min(3)) as char,
            _ => return None,
        })
    }
}

pub fn dir_name(d: IVec2) -> &'static str {
    match (d.x, d.y) {
        (-1, 0) => "ouest",
        (0, -1) => "nord",
        (0, 1) => "sud",
        _ => "est",
    }
}

pub fn parse_dir(v: &str) -> Option<IVec2> {
    Some(match v {
        "est" => IVec2::X,
        "ouest" => IVec2::NEG_X,
        "nord" => IVec2::NEG_Y,
        "sud" => IVec2::Y,
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rows(s: &str) -> Vec<String> {
        s.lines().map(|l| l.trim().to_owned()).filter(|l| !l.is_empty()).collect()
    }

    #[test]
    fn reads_a_small_map() {
        let g = Grid::from_rows(&rows("0111110\n1S11111\n1111A1A\n0222110"), IVec2::X);
        // Le départ est en bord de carte : pas de place derrière lui.
        assert!(g.is_err());
        let g = Grid::from_rows(&rows("001111111100\n111S111111A1\n1111111111A1\n002211111100"), IVec2::X).unwrap();
        assert_eq!(g.layout().start, IVec2::new(3, 1));
        let (tiles, pairs) = g.tiles();
        assert_eq!(pairs, 1);
        assert_eq!(tiles.iter().filter(|(_, t)| matches!(t, Tile::Teleport(0))).count(), 2);
        assert_eq!(tiles.iter().filter(|(_, t)| matches!(t, Tile::Obstacle(_))).count(), 2);
    }

    #[test]
    fn rejects_bad_maps() {
        assert!(Grid::from_rows(&rows("111\n11"), IVec2::X).is_err());
        assert!(Grid::from_rows(&rows("1x1"), IVec2::X).is_err());
        assert!(Grid::from_rows(&rows("11111\n11111"), IVec2::X).is_err(), "pas de départ");
        assert!(Grid::from_rows(&rows("1111A\n11S11"), IVec2::X).is_err(), "téléporteur seul");
    }
}
