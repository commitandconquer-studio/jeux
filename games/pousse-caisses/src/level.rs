//! Niveaux et règles du pousse-caisses (indépendants de l'affichage).
use std::collections::{HashSet, VecDeque};

use bevy::math::IVec2;

const LEVELS_TXT: &str = include_str!("../levels.txt");

/// Un niveau tel que dessiné dans `levels.txt`. Coordonnées : x vers la
/// droite, y vers le bas (ligne du fichier).
#[derive(Clone, Debug)]
pub struct Level {
    pub name: String,
    pub width: i32,
    pub height: i32,
    pub walls: HashSet<IVec2>,
    /// Cases de sol à l'intérieur des murs (accessibles au crapaud).
    pub floor: HashSet<IVec2>,
    pub targets: Vec<IVec2>,
    pub start: Snapshot,
}

/// Position du crapaud et des caisses.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Snapshot {
    pub player: IVec2,
    pub boxes: Vec<IVec2>,
}

/// Résultat d'un déplacement accepté.
pub struct Move {
    pub state: Snapshot,
    /// Indice de la caisse poussée, s'il y en a une.
    pub pushed: Option<usize>,
}

pub const DIRS: [IVec2; 4] = [IVec2::NEG_Y, IVec2::Y, IVec2::NEG_X, IVec2::X];

/// Tous les niveaux du fichier, dans l'ordre.
pub fn all_levels() -> Vec<Level> {
    let mut levels = Vec::new();
    let mut name = None;
    let mut rows: Vec<&str> = Vec::new();
    let mut flush = |name: &mut Option<String>, rows: &mut Vec<&str>| {
        if let Some(n) = name.take() {
            levels.push(Level::parse(&n, rows));
        }
        rows.clear();
    };
    for line in LEVELS_TXT.lines() {
        if line.starts_with(';') {
            continue;
        }
        if let Some(n) = line.strip_prefix("= ") {
            flush(&mut name, &mut rows);
            name = Some(n.trim().to_string());
        } else if !line.trim().is_empty() {
            rows.push(line);
        }
    }
    flush(&mut name, &mut rows);
    levels
}

impl Level {
    fn parse(name: &str, rows: &[&str]) -> Level {
        let mut walls = HashSet::new();
        let mut targets = Vec::new();
        let mut boxes = Vec::new();
        let mut player = IVec2::ZERO;
        let mut width = 0;
        for (y, row) in rows.iter().enumerate() {
            width = width.max(row.chars().count() as i32);
            for (x, c) in row.chars().enumerate() {
                let p = IVec2::new(x as i32, y as i32);
                match c {
                    '#' => {
                        walls.insert(p);
                    }
                    '.' => targets.push(p),
                    '$' => boxes.push(p),
                    '*' => {
                        boxes.push(p);
                        targets.push(p);
                    }
                    '@' => player = p,
                    '+' => {
                        player = p;
                        targets.push(p);
                    }
                    _ => {}
                }
            }
        }
        // Le sol = ce qu'on atteint depuis le crapaud sans traverser de mur.
        let height = rows.len() as i32;
        let mut floor = HashSet::from([player]);
        let mut todo = vec![player];
        while let Some(p) = todo.pop() {
            for d in DIRS {
                let q = p + d;
                let inside = q.x >= 0 && q.y >= 0 && q.x < width && q.y < height;
                if inside && !walls.contains(&q) && floor.insert(q) {
                    todo.push(q);
                }
            }
        }
        Level { name: name.to_string(), width, height, walls, floor, targets, start: Snapshot { player, boxes } }
    }

    fn free(&self, p: IVec2, boxes: &[IVec2]) -> bool {
        self.floor.contains(&p) && !boxes.contains(&p)
    }

    /// Essaie de déplacer le crapaud d'une case (en poussant une caisse si besoin).
    pub fn step(&self, s: &Snapshot, dir: IVec2) -> Option<Move> {
        let to = s.player + dir;
        if !self.floor.contains(&to) {
            return None;
        }
        match s.boxes.iter().position(|&b| b == to) {
            None => Some(Move { state: Snapshot { player: to, boxes: s.boxes.clone() }, pushed: None }),
            Some(i) => {
                let beyond = to + dir;
                if !self.free(beyond, &s.boxes) {
                    return None;
                }
                let mut boxes = s.boxes.clone();
                boxes[i] = beyond;
                Some(Move { state: Snapshot { player: to, boxes }, pushed: Some(i) })
            }
        }
    }

    pub fn is_solved(&self, s: &Snapshot) -> bool {
        s.boxes.iter().all(|b| self.targets.contains(b))
    }

    /// Nombre minimal de poussées pour résoudre le niveau (None = impossible).
    /// Recherche en largeur sur les poussées ; sert aux tests.
    #[allow(dead_code)]
    pub fn min_pushes(&self, limit: usize) -> Option<usize> {
        let canon = |s: &Snapshot| -> (Vec<IVec2>, IVec2) {
            let mut boxes = s.boxes.clone();
            boxes.sort_by_key(|b| (b.y, b.x));
            let reach = self.reachable(s.player, &boxes);
            let corner = *reach.iter().min_by_key(|p| (p.y, p.x)).unwrap();
            (boxes, corner)
        };
        let mut seen = HashSet::from([canon(&self.start)]);
        let mut queue = VecDeque::from([(self.start.clone(), 0usize)]);
        while let Some((s, pushes)) = queue.pop_front() {
            if self.is_solved(&s) {
                return Some(pushes);
            }
            if seen.len() > limit {
                return None;
            }
            let reach = self.reachable(s.player, &s.boxes);
            for (i, &b) in s.boxes.iter().enumerate() {
                for d in DIRS {
                    let from = b - d;
                    let to = b + d;
                    if !reach.contains(&from) || !self.free(to, &s.boxes) || self.dead_corner(to) {
                        continue;
                    }
                    let mut boxes = s.boxes.clone();
                    boxes[i] = to;
                    let next = Snapshot { player: b, boxes };
                    if seen.insert(canon(&next)) {
                        queue.push_back((next, pushes + 1));
                    }
                }
            }
        }
        None
    }

    fn reachable(&self, from: IVec2, boxes: &[IVec2]) -> HashSet<IVec2> {
        let mut seen = HashSet::from([from]);
        let mut todo = vec![from];
        while let Some(p) = todo.pop() {
            for d in DIRS {
                let q = p + d;
                if self.free(q, boxes) && seen.insert(q) {
                    todo.push(q);
                }
            }
        }
        seen
    }

    /// Une caisse poussée dans un coin (hors cible) ne pourra plus jamais bouger.
    fn dead_corner(&self, p: IVec2) -> bool {
        if self.targets.contains(&p) {
            return false;
        }
        let wall = |d: IVec2| !self.floor.contains(&(p + d));
        (wall(IVec2::NEG_Y) || wall(IVec2::Y)) && (wall(IVec2::NEG_X) || wall(IVec2::X))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tous_les_niveaux_sont_bien_formes() {
        for level in all_levels() {
            assert_eq!(level.start.boxes.len(), level.targets.len(), "« {} » : autant de caisses que de cibles", level.name);
            assert!(!level.start.boxes.is_empty(), "« {} » : au moins une caisse", level.name);
            for b in &level.start.boxes {
                assert!(level.floor.contains(b), "« {} » : caisse hors du niveau en {b}", level.name);
            }
            for t in &level.targets {
                assert!(level.floor.contains(t), "« {} » : cible hors du niveau en {t}", level.name);
            }
        }
    }

    #[test]
    fn tous_les_niveaux_ont_une_solution() {
        for (i, level) in all_levels().iter().enumerate() {
            let pushes = level.min_pushes(3_000_000);
            println!("Niveau {:>2} « {} » : {:?} poussées", i + 1, level.name, pushes);
            assert!(pushes.is_some(), "Le niveau {} « {} » n'a pas de solution", i + 1, level.name);
        }
    }
}
