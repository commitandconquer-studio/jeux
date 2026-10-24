//! Le noyau du Chien de berger, sans affichage : la carte (arbres, rochers,
//! mares, clôtures, enclos), les moutons (des boids qui fuient le chien),
//! les loups et un chien automatique. Voir `docs/jeux/chien-de-berger.md`.
pub mod carte;
pub mod monde;
pub mod robot;
pub mod v2;

pub use carte::{Carte, Cote, Genre, Mur, NIVEAUX, Niveau, generer, niveau, niveau_du_jour};
pub use monde::{Commande, EtatLoup, Evenement, Fin, Monde};
pub use v2::{V2, v2};

#[cfg(test)]
mod tests {
    use super::*;

    /// Joue un niveau avec le chien automatique ; renvoie (fin, temps, étoiles).
    fn jouer(nv: &Niveau, graine: u64, robot: bool) -> (Option<Fin>, f32, u32, usize) {
        let mut m = Monde::nouveau(generer(nv, graine), *nv);
        let mut t = 0.0;
        while m.fin.is_none() && t < 150.0 {
            let cmd = if robot { robot::commande(&m) } else { Commande::default() };
            m.pas(1.0 / 60.0, cmd);
            t += 1.0 / 60.0;
        }
        (m.fin, m.temps, m.etoiles(), m.perdus())
    }

    #[test]
    fn toutes_les_cartes_ont_un_chemin_jusqu_a_la_porte() {
        for (i, nv) in NIVEAUX.iter().enumerate() {
            for graine in 0..40 {
                let c = generer(nv, graine * 31 + i as u64);
                assert!(c.accessible(), "niveau {} graine {graine}", i + 1);
                assert_eq!(c.moutons.len(), nv.moutons as usize, "niveau {} graine {graine} : moutons", i + 1);
                assert!(!c.bloque(c.depart_chien, 0.45), "niveau {} graine {graine} : chien coincé", i + 1);
                assert!(c.moutons.iter().all(|&p| !c.enclos.dedans(p, 0.0)), "mouton dans l'enclos au départ");
            }
        }
    }

    #[test]
    fn la_carte_est_deterministe() {
        let a = generer(&NIVEAUX[6], 99);
        let b = generer(&NIVEAUX[6], 99);
        assert_eq!(a.moutons.len(), b.moutons.len());
        assert!(a.moutons.iter().zip(&b.moutons).all(|(p, q)| p == q));
        assert_eq!(a.obstacles.len(), b.obstacles.len());
    }

    #[test]
    fn sans_chien_les_moutons_restent_groupes_et_calmes() {
        let nv = NIVEAUX[1];
        let mut m = Monde::nouveau(generer(&nv, 5), nv);
        for _ in 0..(20 * 60) {
            m.pas(1.0 / 60.0, Commande::default());
        }
        assert!(m.moutons.iter().all(|s| s.panique < 0.1));
        let c = m.moutons.iter().fold(V2::ZERO, |s, p| s + p.pos) / m.moutons.len() as f32;
        assert!(m.moutons.iter().all(|s| s.pos.dist(c) < 9.0), "le troupeau s'est dispersé");
    }

    #[test]
    fn le_chien_fait_fuir_les_moutons() {
        let nv = NIVEAUX[0];
        let mut m = Monde::nouveau(generer(&nv, 3), nv);
        m.chien.pos = m.moutons[0].pos + v2(2.0, 0.0);
        let avant = m.moutons[0].pos;
        for _ in 0..60 {
            m.pas(1.0 / 60.0, Commande::default());
        }
        assert!(m.moutons[0].pos.dist(avant) > 1.5);
        assert!(m.moutons[0].panique > 0.5);
    }

    #[test]
    fn un_loup_sans_chien_mange_un_mouton() {
        let nv = Niveau { loups: 1, loup_debut: 1.0, pertes_max: 99, ..NIVEAUX[4] };
        let (fin, _, _, perdus) = jouer(&nv, 7, false);
        assert!(perdus >= 1, "{fin:?}");
    }

    #[test]
    fn le_robot_gagne_les_premiers_niveaux() {
        for i in [0usize, 1, 2, 3] {
            let mut gagnes = 0;
            for graine in 0..6 {
                let (fin, t, e, _) = jouer(&NIVEAUX[i], graine * 13 + 1, true);
                println!("niveau {} graine {graine} : {fin:?} en {t:.0} s, {e} étoile(s)", i + 1);
                gagnes += (fin == Some(Fin::Victoire)) as u32;
            }
            assert!(gagnes >= 1, "niveau {} : {gagnes}/6 seulement", i + 1);
        }
    }

    #[test]
    fn le_robot_protege_le_troupeau_des_loups() {
        for i in [4usize, 7] {
            let mut sauf = 0;
            for graine in 0..4 {
                let (fin, t, _, p) = jouer(&NIVEAUX[i], graine * 17 + 2, true);
                println!("niveau {} graine {graine} : {fin:?} en {t:.0} s, {p} perdu(s)", i + 1);
                sauf += (p as u32 <= NIVEAUX[i].pertes_max) as u32;
            }
            assert!(sauf >= 2, "niveau {} : le robot perd trop de moutons", i + 1);
        }
    }
}
