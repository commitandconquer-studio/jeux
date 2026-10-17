//! Les cartes : une forme (ses sorties), une rotation, et la géométrie d'une
//! carte posée sur la grille.
//!
//! Une carte debout occupe deux cases, l'une sur l'autre. Ses six côtés sont
//! numérotés dans le sens des aiguilles d'une montre :
//!
//! ```text
//!         1
//!      ┌─────┐
//!    6 │     │ 2
//!      │     │
//!    5 │     │ 3
//!      └─────┘
//!         4
//! ```
//!
//! Tourner la carte de 180° échange 1↔4, 2↔5 et 3↔6 : deux formes qui se
//! déduisent ainsi l'une de l'autre sont la **même carte**. Tourner de 90° la
//! couche à l'horizontale. On ne retourne jamais une carte en miroir.

/// Une case de la grille : (colonne, rangée), la rangée croissant vers le haut.
pub type Case = (i32, i32);

/// Les sorties d'une carte debout : le bit `n - 1` est levé si le côté `n`
/// (de 1 à 6) est ouvert. Une seule sortie = un bouchon.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub struct Forme(u8);

impl Forme {
    /// La fuite : ouverte sur ses six côtés. Jamais dans le paquet.
    pub const FUITE: Forme = Forme(0b11_1111);

    /// Une forme à partir de ses sorties (de 1 à 6).
    pub fn de(sorties: &[u8]) -> Forme {
        let mut bits = 0;
        for &s in sorties {
            assert!((1..=6).contains(&s), "sortie {s} hors de 1..=6");
            bits |= 1 << (s - 1);
        }
        Forme(bits)
    }

    /// Le côté `sortie` (de 1 à 6) est-il ouvert ?
    pub fn ouverte(self, sortie: u8) -> bool {
        self.0 & (1 << (sortie - 1)) != 0
    }

    /// Les sorties ouvertes, de 1 à 6, dans l'ordre.
    pub fn sorties(self) -> impl Iterator<Item = u8> {
        (1..=6).filter(move |&s| self.ouverte(s))
    }

    pub fn nb_sorties(self) -> u32 {
        self.0.count_ones()
    }

    /// Un bouchon : une seule sortie, l'eau entre et s'arrête.
    pub fn est_bouchon(self) -> bool {
        self.nb_sorties() == 1
    }

    /// La même carte tournée de 180° : 1↔4, 2↔5, 3↔6.
    pub fn demi_tour(self) -> Forme {
        let bas = self.0 & 0b111; // sorties 1, 2, 3
        let haut = self.0 >> 3; // sorties 4, 5, 6
        Forme(bas << 3 | haut)
    }

    /// La forme de référence parmi les deux orientations d'une même carte :
    /// celle dont les numéros de sorties sont les plus petits (« 1-6 » plutôt
    /// que « 3-4 »), comme dans le cahier des charges.
    pub fn canonique(self) -> Forme {
        let autre = self.demi_tour();
        if autre.sorties().collect::<Vec<_>>() < self.sorties().collect::<Vec<_>>() { autre } else { self }
    }

    /// Les 34 cartes possibles (à rotation près), hors la vide et la fuite :
    /// 3 bouchons, 9 à deux sorties, 10 à trois, 9 à quatre, 3 à cinq.
    /// Triées par nombre de sorties, puis par numéros.
    pub fn toutes() -> Vec<Forme> {
        let mut formes: Vec<Forme> = (1..0b11_1111).map(Forme).filter(|f| f.canonique() == *f).collect();
        formes.sort_by_key(|f| (f.nb_sorties(), f.sorties().collect::<Vec<_>>()));
        formes
    }

    /// « 1-2-3 ».
    pub fn nom(self) -> String {
        self.sorties().map(|s| s.to_string()).collect::<Vec<_>>().join("-")
    }
}

/// Un quart de tour dans le sens des aiguilles d'une montre, répété.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub enum Rotation {
    #[default]
    R0,
    R90,
    R180,
    R270,
}

impl Rotation {
    pub const TOUTES: [Rotation; 4] = [Rotation::R0, Rotation::R90, Rotation::R180, Rotation::R270];

    /// Nombre de quarts de tour.
    pub fn quarts(self) -> u8 {
        match self {
            Rotation::R0 => 0,
            Rotation::R90 => 1,
            Rotation::R180 => 2,
            Rotation::R270 => 3,
        }
    }

    pub fn de_quarts(quarts: u8) -> Rotation {
        Rotation::TOUTES[(quarts % 4) as usize]
    }

    /// Un quart de tour de plus (touche R, clic droit).
    pub fn suivante(self) -> Rotation {
        Rotation::de_quarts(self.quarts() + 1)
    }

    /// La carte est-elle couchée à l'horizontale ?
    pub fn horizontale(self) -> bool {
        matches!(self, Rotation::R90 | Rotation::R270)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Direction {
    Nord,
    Est,
    Sud,
    Ouest,
}

impl Direction {
    pub const TOUTES: [Direction; 4] = [Direction::Nord, Direction::Est, Direction::Sud, Direction::Ouest];

    /// Le déplacement d'une case dans cette direction.
    pub fn delta(self) -> (i32, i32) {
        match self {
            Direction::Nord => (0, 1),
            Direction::Est => (1, 0),
            Direction::Sud => (0, -1),
            Direction::Ouest => (-1, 0),
        }
    }

    pub fn opposee(self) -> Direction {
        match self {
            Direction::Nord => Direction::Sud,
            Direction::Est => Direction::Ouest,
            Direction::Sud => Direction::Nord,
            Direction::Ouest => Direction::Est,
        }
    }

    /// Tournée d'autant de quarts de tour, dans le sens des aiguilles d'une montre.
    pub fn tournee(self, rotation: Rotation) -> Direction {
        let i = Direction::TOUTES.iter().position(|d| *d == self).unwrap();
        Direction::TOUTES[(i + rotation.quarts() as usize) % 4]
    }

    pub fn voisine(self, case: Case) -> Case {
        let (dx, dy) = self.delta();
        (case.0 + dx, case.1 + dy)
    }
}

/// Une carte posée (ou qu'on s'apprête à poser) : sa forme, sa rotation, et
/// son ancre, la case qui est **en bas** quand la carte est debout.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Carte {
    pub forme: Forme,
    pub rotation: Rotation,
    pub ancre: Case,
}

/// Les six côtés d'une carte debout : (case locale, direction), pour les
/// sorties 1 à 6. La case locale `(0, 1)` est le haut, `(0, 0)` le bas.
const COTES_DEBOUT: [((i32, i32), Direction); 6] = [
    ((0, 1), Direction::Nord),
    ((0, 1), Direction::Est),
    ((0, 0), Direction::Est),
    ((0, 0), Direction::Sud),
    ((0, 0), Direction::Ouest),
    ((0, 1), Direction::Ouest),
];

/// Un décalage local tourné d'un quart de tour dans le sens des aiguilles
/// d'une montre, répété : (x, y) → (y, −x).
fn tourner(local: (i32, i32), rotation: Rotation) -> (i32, i32) {
    let mut p = local;
    for _ in 0..rotation.quarts() {
        p = (p.1, -p.0);
    }
    p
}

impl Carte {
    pub fn new(forme: Forme, rotation: Rotation, ancre: Case) -> Carte {
        Carte { forme, rotation, ancre }
    }

    fn placer(&self, local: (i32, i32)) -> Case {
        let (dx, dy) = tourner(local, self.rotation);
        (self.ancre.0 + dx, self.ancre.1 + dy)
    }

    /// Les deux cases occupées : l'ancre, puis l'autre.
    pub fn cases(&self) -> [Case; 2] {
        [self.placer((0, 0)), self.placer((0, 1))]
    }

    /// Les décalages qui, soustraits à une case visée, donnent les ancres
    /// possibles pour qu'une carte de cette rotation la couvre.
    pub fn decalages(rotation: Rotation) -> [(i32, i32); 2] {
        [tourner((0, 0), rotation), tourner((0, 1), rotation)]
    }

    /// Les six côtés extérieurs, dans l'ordre des sorties 1 à 6 :
    /// (case, direction, ouvert).
    pub fn cotes(&self) -> [(Case, Direction, bool); 6] {
        let mut cotes = [((0, 0), Direction::Nord, false); 6];
        for (i, (local, dir)) in COTES_DEBOUT.iter().enumerate() {
            cotes[i] = (self.placer(*local), dir.tournee(self.rotation), self.forme.ouverte(i as u8 + 1));
        }
        cotes
    }

    /// Les côtés ouverts seulement : (case, direction).
    pub fn ouvertures(&self) -> impl Iterator<Item = (Case, Direction)> {
        self.cotes().into_iter().filter(|c| c.2).map(|c| (c.0, c.1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn les_34_formes_sont_exactement_celles_du_cahier_des_charges() {
        let attendues = [
            "1", "2", "3", // bouchons
            "1-4", "2-5", "3-6", "1-2", "1-6", "1-3", "1-5", "2-3", "2-6", // deux sorties
            "1-2-3", "1-2-4", "1-2-5", "1-2-6", "1-3-4", "1-3-5", "1-3-6", "1-5-6", "2-3-5", "2-3-6", // trois
            "1-2-3-4", "1-2-3-5", "1-2-3-6", "1-2-4-5", "1-2-4-6", "1-2-5-6", "1-3-4-6", "1-3-5-6", "2-3-5-6", // quatre
            "1-2-3-4-5", "1-2-3-4-6", "1-2-3-5-6", // cinq
        ];
        let toutes = Forme::toutes();
        assert_eq!(toutes.len(), 34);
        let noms: std::collections::BTreeSet<String> = toutes.iter().map(|f| f.nom()).collect();
        let attendus: std::collections::BTreeSet<String> = attendues.iter().map(|s| s.to_string()).collect();
        assert_eq!(noms, attendus);
        let par_nombre = |n| toutes.iter().filter(|f| f.nb_sorties() == n).count();
        assert_eq!((par_nombre(1), par_nombre(2), par_nombre(3), par_nombre(4), par_nombre(5)), (3, 9, 10, 9, 3));
        assert!(!toutes.contains(&Forme::FUITE));
        assert!(!toutes.contains(&Forme(0)));
    }

    #[test]
    fn un_demi_tour_echange_1_4_2_5_3_6() {
        assert_eq!(Forme::de(&[1]).demi_tour(), Forme::de(&[4]));
        assert_eq!(Forme::de(&[1, 2]).demi_tour(), Forme::de(&[4, 5]));
        assert_eq!(Forme::de(&[2, 3, 6]).demi_tour(), Forme::de(&[5, 6, 3]));
        assert_eq!(Forme::de(&[1, 4]).demi_tour(), Forme::de(&[1, 4]));
        assert_eq!(Forme::FUITE.demi_tour(), Forme::FUITE);
        // Deux orientations d'une même carte ont la même forme de référence.
        assert_eq!(Forme::de(&[4, 5]).canonique(), Forme::de(&[1, 2]));
        assert_eq!(Forme::de(&[5, 6, 3]).canonique(), Forme::de(&[2, 3, 6]).canonique());
    }

    #[test]
    fn une_carte_debout_a_ses_cotes_dans_le_bon_ordre() {
        let c = Carte::new(Forme::FUITE, Rotation::R0, (5, 5));
        assert_eq!(c.cases(), [(5, 5), (5, 6)]);
        let cotes = c.cotes();
        assert_eq!(cotes[0], ((5, 6), Direction::Nord, true));
        assert_eq!(cotes[1], ((5, 6), Direction::Est, true));
        assert_eq!(cotes[2], ((5, 5), Direction::Est, true));
        assert_eq!(cotes[3], ((5, 5), Direction::Sud, true));
        assert_eq!(cotes[4], ((5, 5), Direction::Ouest, true));
        assert_eq!(cotes[5], ((5, 6), Direction::Ouest, true));
    }

    #[test]
    fn un_quart_de_tour_couche_la_carte_vers_la_droite() {
        // Le haut (sortie 1) passe à droite, ouvert vers l'est.
        let c = Carte::new(Forme::de(&[1]), Rotation::R90, (0, 0));
        assert_eq!(c.cases(), [(0, 0), (1, 0)]);
        assert_eq!(c.ouvertures().collect::<Vec<_>>(), vec![((1, 0), Direction::Est)]);
        assert!(c.rotation.horizontale());
        // La sortie 2 (droite du haut) regarde alors vers le sud.
        let c = Carte::new(Forme::de(&[2]), Rotation::R90, (0, 0));
        assert_eq!(c.ouvertures().collect::<Vec<_>>(), vec![((1, 0), Direction::Sud)]);
    }

    #[test]
    fn un_demi_tour_renverse_la_carte() {
        let c = Carte::new(Forme::de(&[1]), Rotation::R180, (0, 2));
        assert_eq!(c.cases(), [(0, 2), (0, 1)]);
        assert_eq!(c.ouvertures().collect::<Vec<_>>(), vec![((0, 1), Direction::Sud)]);
        // Même chose qu'une carte « 4 » debout dont l'ancre est en (0, 1).
        let d = Carte::new(Forme::de(&[4]), Rotation::R0, (0, 1));
        assert_eq!(d.ouvertures().collect::<Vec<_>>(), c.ouvertures().collect::<Vec<_>>());
    }

    #[test]
    fn la_fuite_couchee_a_deux_sorties_en_haut_deux_en_bas_une_de_chaque_cote() {
        let fuite = Carte::new(Forme::FUITE, Rotation::R90, (0, 0));
        let compte = |d: Direction| fuite.ouvertures().filter(|o| o.1 == d).count();
        assert_eq!((compte(Direction::Nord), compte(Direction::Sud), compte(Direction::Est), compte(Direction::Ouest)), (2, 2, 1, 1));
    }

    #[test]
    fn quatre_quarts_de_tour_reviennent_au_depart() {
        let mut r = Rotation::R0;
        for _ in 0..4 {
            r = r.suivante();
        }
        assert_eq!(r, Rotation::R0);
        assert_eq!(Direction::Nord.tournee(Rotation::R270), Direction::Ouest);
        assert_eq!(Direction::Sud.opposee(), Direction::Nord);
    }
}
