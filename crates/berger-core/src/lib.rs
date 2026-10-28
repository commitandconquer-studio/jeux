//! Le noyau du Chien de berger, sans affichage : la carte (arbres, rochers,
//! mares, clôtures, enclos), les moutons (des boids qui fuient le chien),
//! les loups et un chien automatique. Voir `docs/jeux/chien-de-berger.md`.
pub mod carte;
pub mod monde;
pub mod robot;
pub mod v2;

pub use carte::{Carte, Cote, Genre, Mur, NIVEAUX, Niveau, generer, niveau, niveau_du_jour};
pub use monde::{Commande, EtatLoup, EtatMouton, Evenement, Fin, Monde, PAS_FIXE, Reglages};
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
    fn le_pas_fixe_ne_depend_pas_de_la_frequence_d_affichage() {
        let nv = NIVEAUX[0];
        let (mut a, mut b) = (Monde::nouveau(generer(&nv, 3), nv), Monde::nouveau(generer(&nv, 3), nv));
        let cmd = Commande { dir: v2(1.0, 0.0), course: true, aboyer: false };
        for _ in 0..120 {
            a.pas(1.0 / 60.0, cmd);
        }
        for _ in 0..60 {
            b.pas(1.0 / 30.0, cmd);
        }
        assert!(a.chien.pos.dist(b.chien.pos) < 0.01);
        assert!((a.temps - b.temps).abs() < 0.02);
    }

    #[test]
    fn l_onde_touche_chaque_mouton_une_fois_et_de_proche_en_proche() {
        let nv = NIVEAUX[0];
        let mut m = Monde::nouveau(generer(&nv, 3), nv);
        let chien = m.chien.pos;
        m.moutons[0].pos = chien + v2(2.0, 0.0);
        m.moutons[1].pos = chien + v2(8.0, 0.0);
        let mut touches = Vec::new();
        m.pas(1.0 / 60.0, Commande { aboyer: true, ..Commande::default() });
        for _ in 0..90 {
            m.pas(1.0 / 60.0, Commande::default());
            for e in &m.evenements {
                if let Evenement::Sursaut(i) = e {
                    touches.push(*i);
                }
            }
        }
        let pos = |k: usize| touches.iter().position(|&i| i == k);
        assert_eq!(touches.iter().filter(|&&i| i == 0).count(), 1);
        assert!(pos(0).unwrap() < pos(1).unwrap(), "{touches:?}");
    }

    #[test]
    fn les_moutons_affoles_ne_restent_pas_coinces() {
        // Le robot joue le niveau le plus encombré ; aucun mouton ne reste affolé et immobile longtemps.
        let nv = NIVEAUX[10];
        let mut m = Monde::nouveau(generer(&nv, 5), nv);
        let mut pire: f32 = 0.0;
        for _ in 0..(90 * 60) {
            m.pas(1.0 / 60.0, robot::commande(&m));
            pire = pire.max(m.moutons.iter().map(|s| s.coince).fold(0.0, f32::max));
            if m.fin.is_some() {
                break;
            }
        }
        assert!(pire < 1.3, "un mouton est resté coincé {pire:.1} s");
    }

    #[test]
    fn un_mouton_calme_broute_ou_flane() {
        let nv = NIVEAUX[1];
        let mut m = Monde::nouveau(generer(&nv, 5), nv);
        for _ in 0..(5 * 60) {
            m.pas(1.0 / 60.0, Commande::default());
        }
        assert!(m.moutons.iter().all(|s| matches!(s.etat, EtatMouton::Broute | EtatMouton::Flane | EtatMouton::Troupeau)));
    }

    #[test]
    fn un_mouton_rentre_reste_dans_l_enclos() {
        let nv = NIVEAUX[0];
        let mut m = Monde::nouveau(generer(&nv, 3), nv);
        let e = m.carte.enclos;
        for (i, s) in m.moutons.iter_mut().enumerate() {
            s.pos = e.centre + v2(i as f32 * 0.3 - 0.6, 0.0);
        }
        // Le chien fonce sur la porte, dans tous les sens.
        for t in 0..(20 * 60) {
            m.chien.pos = e.entree() + e.cote.sortie() * (1.5 + (t as f32 * 0.02).sin() * 1.0);
            m.pas(1.0 / 60.0, Commande { aboyer: t % 120 == 0, ..Commande::default() });
        }
        assert_eq!(m.abrites(), m.moutons.len());
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
