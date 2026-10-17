//! Le paquet, la main, la partie et le score.

use crate::carte::{Carte, Case, Forme, Rotation};
use crate::grille::{Grille, Refus};
use crate::rng::Rng;

/// Le nombre de cartes du paquet.
pub const TAILLE_PAQUET: usize = 70;
/// Le nombre de cartes en main au départ.
pub const MAIN_DEPART: usize = 3;

/// Le paquet d'une partie : 70 cartes, toujours la même composition
/// (30 % de bouchons, 40 % à deux sorties, 18 % à trois, 12 % à quatre et
/// cinq), mélangée selon la graine.
pub fn paquet(graine: u64) -> Vec<Forme> {
    let toutes = Forme::toutes();
    let classe = |n: u32| toutes.iter().copied().filter(|f| f.nb_sorties() == n).collect::<Vec<_>>();
    let mut cartes = Vec::with_capacity(TAILLE_PAQUET);
    let mut ajouter = |formes: Vec<Forme>, nombre: usize| {
        for i in 0..nombre {
            cartes.push(formes[i % formes.len()]);
        }
    };
    ajouter(classe(1), 21);
    ajouter(classe(2), 28);
    ajouter(classe(3), 13);
    ajouter(classe(4).into_iter().chain(classe(5)).collect(), 8);
    debug_assert_eq!(cartes.len(), TAILLE_PAQUET);
    Rng::new(graine).shuffle(&mut cartes);
    cartes
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Score {
    pub points: u32,
    /// Plus rien ne fuit.
    pub bouche: bool,
    pub posees: u32,
    /// Ouvertures qui fuient encore.
    pub fuites: u32,
    /// Bouchons posés.
    pub bouchons: u32,
}

/// Moins on pose de cartes, plus on marque ; un réseau entièrement bouché
/// vaut bien plus qu'un réseau qui fuit encore.
pub fn score(grille: &Grille, posees: u32) -> Score {
    let bouche = grille.bouchee();
    let fuites = grille.fuites() as u32;
    let bouchons = grille.cartes().iter().filter(|c| c.forme.est_bouchon()).count() as u32;
    let points = if bouche {
        (1000u32.saturating_sub(10 * posees)).max(100)
    } else {
        100u32.saturating_sub(posees) + 5 * bouchons
    };
    Score { points, bouche, posees, fuites, bouchons }
}

#[derive(Clone, Debug)]
pub struct Partie {
    pub grille: Grille,
    /// Les cartes encore à piocher, la prochaine en premier.
    paquet: Vec<Forme>,
    pub main: Vec<Forme>,
    pub posees: u32,
}

impl Partie {
    pub fn nouvelle(graine: u64) -> Partie {
        let mut paquet = paquet(graine);
        paquet.reverse();
        let mut partie = Partie { grille: Grille::nouvelle(), paquet, main: Vec::new(), posees: 0 };
        for _ in 0..MAIN_DEPART {
            partie.piocher();
        }
        partie
    }

    /// Les prochaines cartes du paquet, dans l'ordre où elles arriveront.
    pub fn a_venir(&self) -> impl Iterator<Item = Forme> + '_ {
        self.paquet.iter().rev().copied()
    }

    pub fn restantes(&self) -> usize {
        self.paquet.len()
    }

    /// Prend la carte suivante du paquet dans la main. `false` si le paquet est vide.
    pub fn piocher(&mut self) -> bool {
        match self.paquet.pop() {
            Some(f) => {
                self.main.push(f);
                true
            }
            None => false,
        }
    }

    /// La carte `i` de la main a-t-elle une place ?
    pub fn posable(&self, i: usize) -> bool {
        self.main.get(i).is_some_and(|&f| self.grille.posable(f))
    }

    /// Aucune carte de la main n'a de place.
    pub fn bloquee(&self) -> bool {
        !(0..self.main.len()).any(|i| self.posable(i))
    }

    /// Tant que rien n'est posable et qu'il reste des cartes, pioche une
    /// carte de plus : la main grandit. Rend le nombre de cartes piochées.
    pub fn debloquer(&mut self) -> usize {
        let mut piochees = 0;
        while self.bloquee() && self.piocher() {
            piochees += 1;
        }
        piochees
    }

    /// Pose la carte `i` de la main, puis repioche une carte si possible.
    pub fn poser(&mut self, i: usize, rotation: Rotation, ancre: Case) -> Result<(), Refus> {
        let forme = *self.main.get(i).ok_or(Refus::CaseOccupee)?;
        self.grille.poser(Carte::new(forme, rotation, ancre))?;
        self.main.remove(i);
        self.posees += 1;
        self.piocher();
        Ok(())
    }

    /// Plus rien à poser ni à piocher.
    pub fn terminee(&self) -> bool {
        self.paquet.is_empty() && self.bloquee()
    }

    pub fn score(&self) -> Score {
        score(&self.grille, self.posees)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn le_paquet_a_70_cartes_bien_reparties() {
        let p = paquet(1);
        assert_eq!(p.len(), TAILLE_PAQUET);
        let n = |k: u32| p.iter().filter(|f| f.nb_sorties() == k).count();
        assert_eq!((n(1), n(2), n(3), n(4) + n(5)), (21, 28, 13, 8));
        assert_eq!(n(1), 21, "30 % de bouchons");
        assert!(!p.contains(&Forme::FUITE));
        // Chaque bouchon 7 fois.
        for s in 1..=3 {
            assert_eq!(p.iter().filter(|&&f| f == Forme::de(&[s])).count(), 7);
        }
    }

    #[test]
    fn la_graine_fixe_l_ordre_du_paquet() {
        assert_eq!(paquet(42), paquet(42));
        assert_ne!(paquet(42), paquet(43));
        let mut trie = paquet(42);
        trie.sort();
        let mut trie2 = paquet(43);
        trie2.sort();
        assert_eq!(trie, trie2, "même composition, seul l'ordre change");
    }

    #[test]
    fn une_partie_commence_avec_trois_cartes() {
        let p = Partie::nouvelle(7);
        assert_eq!(p.main.len(), 3);
        assert_eq!(p.restantes(), 67);
        assert_eq!(p.a_venir().count(), 67);
        assert_eq!(p.a_venir().next(), Some(paquet(7)[3]));
        assert!(!p.terminee());
    }

    #[test]
    fn poser_une_carte_repioche() {
        let mut p = Partie::nouvelle(7);
        let i = (0..3).find(|&i| p.posable(i)).expect("une carte posable au départ");
        let pose = p.grille.poses_possibles(p.main[i])[0];
        let prochaine = p.a_venir().next().unwrap();
        p.poser(i, pose.rotation, pose.ancre).unwrap();
        assert_eq!(p.posees, 1);
        assert_eq!(p.main.len(), 3);
        assert_eq!(p.main[2], prochaine);
        assert_eq!(p.restantes(), 66);
        assert_eq!(p.grille.cartes().len(), 2);
    }

    #[test]
    fn une_pose_refusee_ne_change_rien() {
        let mut p = Partie::nouvelle(7);
        assert_eq!(p.poser(0, Rotation::R0, (20, 20)), Err(Refus::SansEau));
        assert_eq!(p.posees, 0);
        assert_eq!(p.main.len(), 3);
        assert_eq!(p.restantes(), 67);
    }

    #[test]
    fn la_main_grandit_quand_rien_n_est_posable() {
        // Un réseau entièrement bouché : plus rien n'est posable, on pioche jusqu'au bout.
        let mut p = Partie::nouvelle(3);
        for b in [
            Carte::new(Forme::de(&[1]), Rotation::R180, (0, 2)),
            Carte::new(Forme::de(&[1]), Rotation::R180, (1, 2)),
            Carte::new(Forme::de(&[1]), Rotation::R0, (0, -2)),
            Carte::new(Forme::de(&[1]), Rotation::R0, (1, -2)),
            Carte::new(Forme::de(&[1]), Rotation::R90, (-2, 0)),
            Carte::new(Forme::de(&[1]), Rotation::R270, (3, 0)),
        ] {
            p.grille.poser(b).unwrap();
        }
        assert!(p.bloquee());
        let piochees = p.debloquer();
        assert_eq!(piochees, 67);
        assert_eq!(p.main.len(), 70);
        assert_eq!(p.restantes(), 0);
        assert!(p.terminee());
        assert!(p.score().bouche);
    }

    #[test]
    fn le_score_recompense_les_reseaux_bouches_et_courts() {
        let mut g = Grille::nouvelle();
        assert_eq!(score(&g, 0).points, 100, "rien posé, six fuites : 100 − 0, aucun bouchon");
        assert_eq!(score(&g, 30).points, 70);
        assert_eq!(score(&g, 300).points, 0);
        for b in [
            Carte::new(Forme::de(&[1]), Rotation::R180, (0, 2)),
            Carte::new(Forme::de(&[1]), Rotation::R180, (1, 2)),
            Carte::new(Forme::de(&[1]), Rotation::R0, (0, -2)),
        ] {
            g.poser(b).unwrap();
        }
        let s = score(&g, 3);
        assert_eq!((s.bouche, s.fuites, s.bouchons, s.points), (false, 3, 3, 97 + 15));
        for b in [
            Carte::new(Forme::de(&[1]), Rotation::R0, (1, -2)),
            Carte::new(Forme::de(&[1]), Rotation::R90, (-2, 0)),
            Carte::new(Forme::de(&[1]), Rotation::R270, (3, 0)),
        ] {
            g.poser(b).unwrap();
        }
        let s = score(&g, 6);
        assert_eq!((s.bouche, s.fuites, s.points), (true, 0, 940));
        assert_eq!(score(&g, 95).points, 100, "jamais moins de 100 quand c'est bouché");
    }

    #[test]
    fn une_partie_jouee_au_hasard_se_termine_toujours() {
        for graine in 1..=20 {
            let mut p = Partie::nouvelle(graine);
            let mut rng = Rng::new(graine);
            let mut tours = 0;
            while !p.terminee() {
                p.debloquer();
                if p.bloquee() {
                    break;
                }
                let posables: Vec<usize> = (0..p.main.len()).filter(|&i| p.posable(i)).collect();
                let i = posables[rng.below(posables.len())];
                let poses = p.grille.poses_possibles(p.main[i]);
                let pose = poses[rng.below(poses.len())];
                p.poser(i, pose.rotation, pose.ancre).unwrap();
                tours += 1;
                assert!(tours <= TAILLE_PAQUET);
            }
            assert!(p.terminee());
            assert_eq!(p.restantes(), 0);
            assert_eq!(p.posees as usize + p.main.len(), TAILLE_PAQUET, "chaque carte est posée ou en main");
            // Le réseau n'a jamais de raccord faux : chaque ouverture donne sur du vide ou sur une ouverture.
            for c in p.grille.cartes() {
                for (case, dir) in c.ouvertures() {
                    let v = dir.voisine(case);
                    assert!(p.grille.libre(v) || p.grille.cote(v, dir.opposee()) == Some(true));
                }
            }
            // L'eau atteint toutes les cartes.
            assert!(p.grille.arrivee_eau().iter().all(|a| a.is_some()));
        }
    }
}
