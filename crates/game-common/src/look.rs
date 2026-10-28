//! Générateur d'apparences : à partir d'un numéro, choisit la couleur de peau, de cheveux et de
//! vêtements d'un personnage. Le même numéro donne toujours la même apparence, et des numéros
//! voisins donnent des apparences très différentes (une file de 100 personnes reste variée).
//! Ne sait rien du rendu : chaque jeu applique ces couleurs à ses modèles.

/// Couleur sRGB, 0–255.
pub type Rgb = [u8; 3];

/// Teintes de peau, du plus clair au plus foncé.
pub const SKINS: [Rgb; 8] = [
    [255, 224, 196],
    [241, 194, 153],
    [224, 172, 120],
    [198, 134, 87],
    [160, 100, 62],
    [124, 76, 48],
    [92, 56, 36],
    [66, 40, 28],
];

/// Cheveux naturels (les plus fréquents) puis quelques teintes de fantaisie.
pub const HAIRS: [Rgb; 12] = [
    [35, 30, 30],
    [70, 45, 30],
    [110, 70, 40],
    [140, 90, 50],
    [225, 190, 110],
    [240, 220, 150],
    [190, 85, 40],
    [160, 160, 165],
    [235, 235, 235],
    [70, 110, 200],
    [230, 110, 170],
    [90, 190, 120],
];

pub const TOPS: [Rgb; 16] = [
    [214, 69, 65],
    [243, 156, 18],
    [247, 202, 24],
    [76, 175, 80],
    [38, 166, 154],
    [52, 152, 219],
    [44, 62, 120],
    [142, 68, 173],
    [236, 135, 190],
    [245, 245, 245],
    [149, 165, 166],
    [45, 45, 52],
    [139, 94, 60],
    [160, 224, 200],
    [190, 170, 230],
    [212, 160, 40],
];

pub const BOTTOMS: [Rgb; 8] = [
    [40, 52, 96],
    [38, 38, 44],
    [120, 124, 130],
    [110, 76, 50],
    [196, 176, 130],
    [76, 120, 180],
    [48, 90, 64],
    [120, 40, 50],
];

pub const SHOES: [Rgb; 4] = [[240, 240, 240], [35, 35, 40], [100, 66, 42], [200, 60, 55]];

/// Les couleurs d'un personnage.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Look {
    pub skin: Rgb,
    pub hair: Rgb,
    pub top: Rgb,
    pub bottom: Rgb,
    pub shoes: Rgb,
}

/// Mélange de bits (splitmix64) : des numéros voisins donnent des résultats sans rapport.
fn mix(mut x: u64) -> u64 {
    x = x.wrapping_add(0x9E37_79B9_7F4A_7C15);
    x = (x ^ (x >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    x = (x ^ (x >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    x ^ (x >> 31)
}

/// Une valeur dans `0..n`, tirée du numéro et d'un grain propre à chaque rôle.
fn pick(id: u64, salt: u64, n: usize) -> usize {
    (mix(id ^ salt.wrapping_mul(0xA24B_AED4_963E_E407)) % n as u64) as usize
}

impl Look {
    /// L'apparence du personnage n° `id`.
    pub fn from_id(id: u64) -> Self {
        // Cheveux : surtout naturels (les 9 premiers), une fois sur six une teinte de fantaisie.
        let hair = if pick(id, 2, 6) == 0 { 9 + pick(id, 3, 3) } else { pick(id, 4, 9) };
        let top = pick(id, 5, TOPS.len());
        let mut bottom = pick(id, 6, BOTTOMS.len());
        // Haut et bas jamais de la même couleur.
        if BOTTOMS[bottom] == TOPS[top] {
            bottom = (bottom + 1) % BOTTOMS.len();
        }
        Look {
            skin: SKINS[pick(id, 1, SKINS.len())],
            hair: HAIRS[hair],
            top: TOPS[top],
            bottom: BOTTOMS[bottom],
            shoes: SHOES[pick(id, 7, SHOES.len())],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn same_id_same_look() {
        assert_eq!(Look::from_id(42), Look::from_id(42));
    }

    #[test]
    fn a_hundred_ids_are_varied() {
        let looks: HashSet<Look> = (0..100).map(Look::from_id).collect();
        assert!(looks.len() >= 95, "seulement {} apparences", looks.len());
        let skins: HashSet<Rgb> = (0..100).map(|i| Look::from_id(i).skin).collect();
        assert_eq!(skins.len(), SKINS.len(), "toutes les teintes de peau doivent apparaître");
        let hairs: HashSet<Rgb> = (0..100).map(|i| Look::from_id(i).hair).collect();
        assert!(hairs.len() >= 9);
    }

    #[test]
    fn top_and_bottom_differ() {
        assert!((0..2000).all(|i| {
            let l = Look::from_id(i);
            l.top != l.bottom
        }));
    }

    #[test]
    fn fantasy_hair_is_rare() {
        let fantasy = (0..1200).filter(|i| HAIRS[9..].contains(&Look::from_id(*i).hair)).count();
        assert!((100..320).contains(&fantasy), "{fantasy} cheveux de fantaisie sur 1200");
    }
}
