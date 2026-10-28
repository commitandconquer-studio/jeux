//! La forme du plateau : grille de `w`×`h` cases dont certaines seulement sont praticables
//! (carré, rectangle, L, U, routes…). Tout le jeu passe par cette forme : aucune règle ne suppose
//! un plateau carré. La logique ne dépend ni du rendu ni du moteur, elle se teste seule.
use bevy::prelude::*;

#[derive(Resource, Clone, Debug)]
pub struct Layout {
    pub w: i32,
    pub h: i32,
    mask: Vec<bool>,
    /// Case de la tête au départ et direction ; le corps s'étend derrière.
    pub start: IVec2,
    pub dir: IVec2,
    /// Identifie la forme (pour savoir quand reconstruire le décor).
    pub id: u32,
}

impl Default for Layout {
    fn default() -> Self {
        Layout::rect(14, 14)
    }
}

impl Layout {
    /// Rectangle plein ; départ au centre, vers l'est.
    pub fn rect(w: i32, h: i32) -> Self {
        Layout { w, h, mask: vec![true; (w * h) as usize], start: IVec2::new(w / 2, h / 2), dir: IVec2::X, id: 0 }
    }

    /// Forme dessinée case par case (ligne après ligne).
    pub fn from_mask(w: i32, h: i32, mask: Vec<bool>, start: IVec2, dir: IVec2) -> Self {
        Layout { w, h, mask, start, dir, id: 0 }
    }

    /// Union de rectangles `(x0, y0, x1, y1)` (bornes comprises).
    pub fn from_rects(w: i32, h: i32, rects: &[(i32, i32, i32, i32)], start: IVec2) -> Self {
        let mut l = Layout { w, h, mask: vec![false; (w * h) as usize], start, dir: IVec2::X, id: 0 };
        for &(x0, y0, x1, y1) in rects {
            for y in y0..=y1 {
                for x in x0..=x1 {
                    l.set(IVec2::new(x, y), true);
                }
            }
        }
        l
    }

    /// Chemin épaissi de 2 cases le long d'une ligne brisée (points aux coins).
    pub fn route(w: i32, h: i32, corners: &[(i32, i32)], start: IVec2) -> Self {
        let rects: Vec<_> = corners
            .windows(2)
            .map(|p| (p[0].0.min(p[1].0), p[0].1.min(p[1].1), p[0].0.max(p[1].0) + 1, p[0].1.max(p[1].1) + 1))
            .collect();
        Self::from_rects(w, h, &rects, start)
    }

    fn set(&mut self, c: IVec2, v: bool) {
        if self.inside(c) {
            let i = (c.y * self.w + c.x) as usize;
            self.mask[i] = v;
        }
    }

    pub fn inside(&self, c: IVec2) -> bool {
        c.x >= 0 && c.y >= 0 && c.x < self.w && c.y < self.h
    }

    /// La case existe et on peut y marcher.
    pub fn walkable(&self, c: IVec2) -> bool {
        self.inside(c) && self.mask[(c.y * self.w + c.x) as usize]
    }

    /// Toutes les cases praticables.
    pub fn cells(&self) -> impl Iterator<Item = IVec2> + '_ {
        (0..self.h).flat_map(move |y| (0..self.w).map(move |x| IVec2::new(x, y))).filter(|c| self.walkable(*c))
    }

    /// Plateau plein (les bords « traversables » n'ont de sens que là).
    pub fn is_full(&self) -> bool {
        self.mask.iter().all(|m| *m)
    }

    /// Les trois cases du départ : tête, puis deux membres derrière elle.
    pub fn start_cells(&self) -> Vec<IVec2> {
        (0..3).map(|i| self.start - self.dir * i).collect()
    }

    /// Distance (en cases) de `c` à la case praticable la plus proche.
    pub fn distance_to_walkable(&self, c: Vec2) -> f32 {
        self.cells().map(|p| p.as_vec2().distance(c)).fold(f32::MAX, f32::min)
    }

    /// Cases praticables atteignables depuis `from` sans quitter la forme.
    #[cfg(test)]
    pub fn reachable(&self, from: IVec2) -> usize {
        let mut seen = vec![false; self.mask.len()];
        let mut stack = vec![from];
        let mut n = 0;
        while let Some(c) = stack.pop() {
            if !self.walkable(c) || seen[(c.y * self.w + c.x) as usize] {
                continue;
            }
            seen[(c.y * self.w + c.x) as usize] = true;
            n += 1;
            stack.extend([IVec2::X, IVec2::NEG_X, IVec2::Y, IVec2::NEG_Y].map(|d| c + d));
        }
        n
    }
}

/// Les formes proposées.
#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub enum Shape {
    Small,
    #[default]
    Medium,
    Large,
    Rectangle,
    L,
    U,
    Route,
    Alleys,
}

impl Shape {
    pub const ALL: [Shape; 8] = [Shape::Small, Shape::Medium, Shape::Large, Shape::Rectangle, Shape::L, Shape::U, Shape::Route, Shape::Alleys];

    pub fn next(self) -> Shape {
        let i = Self::ALL.iter().position(|s| *s == self).unwrap_or(0);
        Self::ALL[(i + 1) % Self::ALL.len()]
    }

    pub fn layout(self) -> Layout {
        let mut l = self.build();
        l.id = self as u32 + 1;
        l
    }

    fn build(self) -> Layout {
        let at = IVec2::new;
        match self {
            Shape::Small => Layout::rect(10, 10),
            Shape::Medium => Layout::rect(14, 14),
            Shape::Large => Layout::rect(18, 18),
            Shape::Rectangle => Layout::rect(20, 12),
            // Un L : un bras vertical à gauche, une grande base en bas.
            Shape::L => Layout::from_rects(16, 16, &[(0, 0, 7, 15), (0, 8, 15, 15)], at(4, 12)),
            // Un U : deux bras reliés par une base.
            Shape::U => Layout::from_rects(16, 14, &[(0, 0, 4, 13), (11, 0, 15, 13), (0, 9, 15, 13)], at(7, 11)),
            // Une route de deux cases de large qui serpente et se referme sur elle-même.
            Shape::Route => Layout::route(
                22,
                14,
                &[(1, 1), (15, 1), (15, 5), (6, 5), (6, 9), (19, 9), (19, 12), (1, 12), (1, 1)],
                at(5, 1),
            ),
            // Un réseau d'allées : anneau, croisement central, quatre boucles.
            Shape::Alleys => Layout::from_rects(
                22,
                14,
                &[(0, 0, 21, 1), (0, 12, 21, 13), (0, 0, 1, 13), (20, 0, 21, 13), (0, 6, 21, 7), (10, 0, 11, 13)],
                at(5, 6),
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_shape_is_sound() {
        for s in Shape::ALL {
            let l = s.layout();
            // La tête et les deux membres de départ marchent sur des cases praticables.
            assert!(l.start_cells().iter().all(|c| l.walkable(*c)), "{s:?} : départ sur une case bloquée");
            // Tout le plateau est d'un seul tenant.
            assert_eq!(l.reachable(l.start), l.cells().count(), "{s:?} : cases isolées");
            // La tête a de la place devant elle.
            assert!(l.walkable(l.start + l.dir), "{s:?} : mur devant le départ");
        }
    }

    #[test]
    fn non_square_shapes_exist() {
        assert!(!Shape::L.layout().is_full());
        assert!(!Shape::Route.layout().is_full());
        let r = Shape::Rectangle.layout();
        assert!(r.w != r.h && r.is_full());
    }

    #[test]
    fn route_has_no_dead_end() {
        // Chaque case de la route a au moins deux voisines praticables (pas de cul-de-sac).
        let l = Shape::Route.layout();
        for c in l.cells() {
            let n = [IVec2::X, IVec2::NEG_X, IVec2::Y, IVec2::NEG_Y].iter().filter(|d| l.walkable(c + **d)).count();
            assert!(n >= 2, "cul-de-sac en {c}");
        }
    }
}
