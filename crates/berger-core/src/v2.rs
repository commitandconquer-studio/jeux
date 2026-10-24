//! Un vecteur plat minimal (`x` : est, `y` : sud sur la carte), sans dépendance.
use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub, SubAssign};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct V2 {
    pub x: f32,
    pub y: f32,
}

pub const fn v2(x: f32, y: f32) -> V2 {
    V2 { x, y }
}

impl V2 {
    pub const ZERO: V2 = v2(0.0, 0.0);

    pub fn len(self) -> f32 {
        self.x.hypot(self.y)
    }
    pub fn len2(self) -> f32 {
        self.x * self.x + self.y * self.y
    }
    pub fn dot(self, o: V2) -> f32 {
        self.x * o.x + self.y * o.y
    }
    pub fn dist(self, o: V2) -> f32 {
        (self - o).len()
    }
    /// Le même sens, longueur 1 (ou zéro si le vecteur est nul).
    pub fn norm(self) -> V2 {
        let l = self.len();
        if l < 1e-6 { V2::ZERO } else { self / l }
    }
    /// Tourné d'un quart de tour.
    pub fn perp(self) -> V2 {
        v2(-self.y, self.x)
    }
    pub fn from_angle(a: f32) -> V2 {
        v2(a.cos(), a.sin())
    }
    pub fn angle(self) -> f32 {
        self.y.atan2(self.x)
    }
    /// Raccourci à `max` s'il est plus long.
    pub fn limit(self, max: f32) -> V2 {
        let l = self.len();
        if l > max { self * (max / l) } else { self }
    }
    /// Ramené dans le rectangle `[-lim, lim]`.
    pub fn clamp_in(self, lim: V2) -> V2 {
        v2(self.x.clamp(-lim.x, lim.x), self.y.clamp(-lim.y, lim.y))
    }
    pub fn lerp(self, o: V2, t: f32) -> V2 {
        self + (o - self) * t
    }
}

impl Add for V2 {
    type Output = V2;
    fn add(self, o: V2) -> V2 {
        v2(self.x + o.x, self.y + o.y)
    }
}
impl Sub for V2 {
    type Output = V2;
    fn sub(self, o: V2) -> V2 {
        v2(self.x - o.x, self.y - o.y)
    }
}
impl Mul<f32> for V2 {
    type Output = V2;
    fn mul(self, k: f32) -> V2 {
        v2(self.x * k, self.y * k)
    }
}
impl Div<f32> for V2 {
    type Output = V2;
    fn div(self, k: f32) -> V2 {
        v2(self.x / k, self.y / k)
    }
}
impl Neg for V2 {
    type Output = V2;
    fn neg(self) -> V2 {
        v2(-self.x, -self.y)
    }
}
impl AddAssign for V2 {
    fn add_assign(&mut self, o: V2) {
        *self = *self + o;
    }
}
impl SubAssign for V2 {
    fn sub_assign(&mut self, o: V2) {
        *self = *self - o;
    }
}

/// Point le plus proche de `p` sur le segment `[a, b]`.
pub fn sur_segment(p: V2, a: V2, b: V2) -> V2 {
    let ab = b - a;
    let l2 = ab.len2();
    if l2 < 1e-9 {
        return a;
    }
    a + ab * ((p - a).dot(ab) / l2).clamp(0.0, 1.0)
}

/// Générateur pseudo-aléatoire minimal (xorshift), déterministe.
#[derive(Clone, Debug)]
pub struct Hasard(u64);

impl Hasard {
    pub fn new(graine: u64) -> Self {
        let mut h = Hasard(graine.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1);
        for _ in 0..4 {
            h.next_u64();
        }
        h
    }
    pub fn next_u64(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }
    /// Réel dans `0..1`.
    pub fn f(&mut self) -> f32 {
        (self.next_u64() >> 40) as f32 / (1u64 << 24) as f32
    }
    /// Réel dans `a..b`.
    pub fn entre(&mut self, a: f32, b: f32) -> f32 {
        a + (b - a) * self.f()
    }
    /// Entier dans `0..max`.
    pub fn sous(&mut self, max: u32) -> u32 {
        (self.next_u64() % max.max(1) as u64) as u32
    }
}
