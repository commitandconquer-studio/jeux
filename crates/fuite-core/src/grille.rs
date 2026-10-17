//! La grille infinie, les règles de pose et l'eau.
//!
//! La grille est un dictionnaire de cases occupées : elle grandit à mesure
//! qu'on pose des cartes. Elle naît avec la fuite, couchée au centre.

use std::collections::{HashMap, VecDeque};

use crate::carte::{Carte, Case, Direction, Forme, Rotation};

/// Pourquoi une pose est refusée.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Refus {
    /// Une des deux cases est déjà prise.
    CaseOccupee,
    /// La carte ne touche aucune ouverture d'eau du réseau.
    SansEau,
    /// Un bord ne concorde pas avec une carte voisine (ouverture contre mur) :
    /// ce serait une fuite irréparable.
    Raccord,
}

#[derive(Clone, Debug)]
pub struct Grille {
    cartes: Vec<Carte>,
    /// Pour chaque case occupée, l'indice de sa carte dans `cartes`.
    cases: HashMap<Case, usize>,
}

impl Default for Grille {
    fn default() -> Self {
        Grille::nouvelle()
    }
}

impl Grille {
    /// La fuite seule, couchée sur les cases (0, 0) et (1, 0).
    pub fn nouvelle() -> Grille {
        let mut g = Grille { cartes: Vec::new(), cases: HashMap::new() };
        g.inserer(Carte::new(Forme::FUITE, Rotation::R90, (0, 0)));
        g
    }

    fn inserer(&mut self, carte: Carte) {
        let i = self.cartes.len();
        for case in carte.cases() {
            self.cases.insert(case, i);
        }
        self.cartes.push(carte);
    }

    /// Toutes les cartes posées, la fuite en premier.
    pub fn cartes(&self) -> &[Carte] {
        &self.cartes
    }

    pub fn fuite(&self) -> &Carte {
        &self.cartes[0]
    }

    pub fn libre(&self, case: Case) -> bool {
        !self.cases.contains_key(&case)
    }

    /// La carte qui occupe une case.
    pub fn carte_en(&self, case: Case) -> Option<&Carte> {
        self.cases.get(&case).map(|&i| &self.cartes[i])
    }

    /// Le côté d'une case occupée : `Some(true)` ouvert, `Some(false)` mur,
    /// `None` si la case est libre. Le côté entre les deux cases d'une même
    /// carte compte comme ouvert (l'eau y passe).
    pub fn cote(&self, case: Case, direction: Direction) -> Option<bool> {
        let carte = self.carte_en(case)?;
        if carte.cases().contains(&direction.voisine(case)) {
            return Some(true);
        }
        carte.cotes().iter().find(|c| c.0 == case && c.1 == direction).map(|c| c.2)
    }

    /// Les ouvertures du réseau qui donnent sur du vide : (case, direction).
    /// C'est là que l'eau déborde, et là qu'on peut poser.
    pub fn ouvertures(&self) -> Vec<(Case, Direction)> {
        self.cartes.iter().flat_map(|c| c.ouvertures()).filter(|(case, dir)| self.libre(dir.voisine(*case))).collect()
    }

    /// Le nombre d'ouvertures qui fuient.
    pub fn fuites(&self) -> usize {
        self.ouvertures().len()
    }

    /// Plus rien ne fuit : le réseau est entièrement bouché.
    pub fn bouchee(&self) -> bool {
        self.fuites() == 0
    }

    /// Peut-on poser cette carte ? Les deux cases doivent être libres, elle
    /// doit toucher au moins une ouverture d'eau, et chaque bord contre une
    /// carte voisine doit concorder (ouvert contre ouvert, mur contre mur).
    pub fn verifier(&self, carte: &Carte) -> Result<(), Refus> {
        if carte.cases().iter().any(|&c| !self.libre(c)) {
            return Err(Refus::CaseOccupee);
        }
        let mut touche_eau = false;
        for (case, direction, ouvert) in carte.cotes() {
            if let Some(voisin_ouvert) = self.cote(direction.voisine(case), direction.opposee()) {
                if voisin_ouvert != ouvert {
                    return Err(Refus::Raccord);
                }
                if ouvert {
                    touche_eau = true;
                }
            }
        }
        if touche_eau { Ok(()) } else { Err(Refus::SansEau) }
    }

    pub fn poser(&mut self, carte: Carte) -> Result<(), Refus> {
        self.verifier(&carte)?;
        self.inserer(carte);
        Ok(())
    }

    /// Toutes les poses valides d'une forme, dans toutes les rotations.
    pub fn poses_possibles(&self, forme: Forme) -> Vec<Carte> {
        let mut poses = Vec::new();
        for (case, dir) in self.ouvertures() {
            let visee = dir.voisine(case);
            for rotation in Rotation::TOUTES {
                for (dx, dy) in Carte::decalages(rotation) {
                    let carte = Carte::new(forme, rotation, (visee.0 - dx, visee.1 - dy));
                    if self.verifier(&carte).is_ok() && !poses.contains(&carte) {
                        poses.push(carte);
                    }
                }
            }
        }
        poses
    }

    /// Une forme a-t-elle au moins une pose valide ?
    pub fn posable(&self, forme: Forme) -> bool {
        !self.poses_possibles(forme).is_empty()
    }

    /// Les coins de la zone occupée : (min, max) inclus. Sert au cadrage.
    pub fn bornes(&self) -> (Case, Case) {
        let mut min = (i32::MAX, i32::MAX);
        let mut max = (i32::MIN, i32::MIN);
        for &(x, y) in self.cases.keys() {
            min = (min.0.min(x), min.1.min(y));
            max = (max.0.max(x), max.1.max(y));
        }
        (min, max)
    }

    /// L'étape à laquelle l'eau arrive dans chaque carte (indice de
    /// `cartes()`) : 0 pour la fuite, 1 pour ses voisines, etc. Une carte
    /// que l'eau n'atteint pas (impossible par construction) vaut `None`.
    pub fn arrivee_eau(&self) -> Vec<Option<u32>> {
        let mut arrivee = vec![None; self.cartes.len()];
        let mut file = VecDeque::new();
        arrivee[0] = Some(0);
        file.push_back(0);
        while let Some(i) = file.pop_front() {
            let etape = arrivee[i].unwrap();
            for (case, dir) in self.cartes[i].ouvertures() {
                let voisine = dir.voisine(case);
                let Some(&j) = self.cases.get(&voisine) else { continue };
                if self.cote(voisine, dir.opposee()) == Some(true) && arrivee[j].is_none() {
                    arrivee[j] = Some(etape + 1);
                    file.push_back(j);
                }
            }
        }
        arrivee
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn f(s: &[u8]) -> Forme {
        Forme::de(s)
    }

    #[test]
    fn la_grille_nait_avec_la_fuite_qui_fuit_de_six_cotes() {
        let g = Grille::nouvelle();
        assert_eq!(g.cartes().len(), 1);
        assert_eq!(g.fuite().cases(), [(0, 0), (1, 0)]);
        assert_eq!(g.fuites(), 6);
        assert!(!g.bouchee());
        assert_eq!(g.cote((0, 0), Direction::Nord), Some(true));
        assert_eq!(g.cote((0, 0), Direction::Est), Some(true), "entre les deux cases de la fuite, l'eau passe");
        assert_eq!(g.cote((0, 5), Direction::Nord), None);
    }

    #[test]
    fn un_bouchon_se_pose_contre_une_ouverture() {
        let mut g = Grille::nouvelle();
        // Le bouchon « 1 » renversé : son ouverture regarde vers le sud, sur la fuite.
        let bouchon = Carte::new(f(&[1]), Rotation::R180, (0, 2));
        assert_eq!(g.verifier(&bouchon), Ok(()));
        g.poser(bouchon).unwrap();
        assert_eq!(g.fuites(), 5);
        assert_eq!(g.cartes().len(), 2);
    }

    #[test]
    fn on_ne_pose_pas_sur_une_case_prise() {
        let g = Grille::nouvelle();
        assert_eq!(g.verifier(&Carte::new(f(&[4]), Rotation::R0, (0, 0))), Err(Refus::CaseOccupee));
    }

    #[test]
    fn on_ne_pose_pas_loin_de_l_eau_ni_mur_contre_mur() {
        let g = Grille::nouvelle();
        assert_eq!(g.verifier(&Carte::new(f(&[1]), Rotation::R0, (10, 10))), Err(Refus::SansEau));
        // Couchée à gauche de la fuite, ouverte vers l'ouest (loin de l'eau) et
        // un mur contre l'ouverture ouest de la fuite : raccord faux.
        assert_eq!(g.verifier(&Carte::new(f(&[4]), Rotation::R90, (-2, 0))), Err(Refus::Raccord));
        // Même place, ouverte vers l'est : elle se raccorde.
        assert_eq!(g.verifier(&Carte::new(f(&[1]), Rotation::R90, (-2, 0))), Ok(()));
    }

    #[test]
    fn un_mur_contre_une_ouverture_est_une_fuite_irreparable() {
        let g = Grille::nouvelle();
        // Couchée juste au-dessus de la fuite : sa case gauche (0, 1) ouvre
        // vers le sud (sortie 3), mais sa case droite (1, 1) y met un mur
        // face à l'ouverture nord de (1, 0).
        let mauvaise = Carte::new(f(&[3]), Rotation::R90, (0, 1));
        assert_eq!(g.verifier(&mauvaise), Err(Refus::Raccord));
        // Avec les deux sorties 2 et 3 (toutes deux vers le sud), ça concorde.
        let bonne = Carte::new(f(&[2, 3]), Rotation::R90, (0, 1));
        assert_eq!(g.verifier(&bonne), Ok(()));
    }

    #[test]
    fn les_bords_doivent_concorder_avec_toutes_les_voisines() {
        let mut g = Grille::nouvelle();
        // Un tuyau droit vertical au-dessus de la case (0, 0) : ouvert en bas et en haut.
        g.poser(Carte::new(f(&[1, 4]), Rotation::R0, (0, 1))).unwrap();
        // Un autre à sa droite, au-dessus de (1, 0), avec une ouverture vers l'ouest (sortie 5)
        // face au mur est du premier : refusé.
        assert_eq!(g.verifier(&Carte::new(f(&[4, 5]), Rotation::R0, (1, 1))), Err(Refus::Raccord));
        // Sans cette ouverture : accepté.
        assert_eq!(g.verifier(&Carte::new(f(&[4]), Rotation::R0, (1, 1))), Ok(()));
    }

    #[test]
    fn un_reseau_entierement_bouche() {
        let mut g = Grille::nouvelle();
        let bouchons = [
            Carte::new(f(&[1]), Rotation::R180, (0, 2)),  // au-dessus de (0, 0)
            Carte::new(f(&[1]), Rotation::R180, (1, 2)),  // au-dessus de (1, 0)
            Carte::new(f(&[1]), Rotation::R0, (0, -2)),   // sous (0, 0)
            Carte::new(f(&[1]), Rotation::R0, (1, -2)),   // sous (1, 0)
            Carte::new(f(&[1]), Rotation::R90, (-2, 0)),  // à gauche, ouvert vers l'est
            Carte::new(f(&[1]), Rotation::R270, (3, 0)),  // à droite, ouvert vers l'ouest
        ];
        for b in bouchons {
            g.poser(b).unwrap_or_else(|e| panic!("{b:?} : {e:?}"));
        }
        assert!(g.bouchee());
        assert_eq!(g.fuites(), 0);
        assert!(!g.posable(f(&[1])), "plus aucune place");
        assert_eq!(g.bornes(), ((-2, -2), (3, 2)));
    }

    #[test]
    fn l_eau_avance_de_proche_en_proche() {
        let mut g = Grille::nouvelle();
        g.poser(Carte::new(f(&[1, 4]), Rotation::R0, (0, 1))).unwrap(); // étape 1
        g.poser(Carte::new(f(&[1, 4]), Rotation::R0, (0, 3))).unwrap(); // étape 2
        g.poser(Carte::new(f(&[1]), Rotation::R180, (0, 6))).unwrap(); // étape 3, bouchon
        assert_eq!(g.arrivee_eau(), vec![Some(0), Some(1), Some(2), Some(3)]);
    }

    #[test]
    fn toutes_les_poses_possibles_d_un_bouchon_au_depart() {
        let g = Grille::nouvelle();
        let poses = g.poses_possibles(f(&[1]));
        // Six ouvertures ; devant chacune, le bouchon peut se présenter
        // debout ou couché, mais seules certaines rotations concordent.
        assert!(!poses.is_empty());
        assert!(poses.iter().all(|p| g.verifier(p).is_ok()));
        assert!(poses.contains(&Carte::new(f(&[1]), Rotation::R180, (0, 2))));
        // Une forme sans sortie compatible n'a aucune pose… sauf si tout est ouvert.
        assert!(g.posable(Forme::FUITE));
    }
}
