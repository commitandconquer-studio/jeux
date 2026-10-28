//! Un chien automatique : il sert aux tests (chaque niveau a une solution) et
//! aux captures d'écran. Il se place derrière le mouton le plus loin de la
//! porte et le pousse, en chassant les loups quand ils menacent.
use crate::monde::{Commande, EtatBelier, EtatLoup, Monde};
use crate::v2::{V2, v2};

pub fn commande(m: &Monde) -> Commande {
    let mut c = commande_de_base(m);
    // Un bélier qui baisse les cornes : un aboiement au bon moment annule la charge.
    let chien = m.chien.pos;
    if m.beliers.iter().any(|b| matches!(b.etat, EtatBelier::Vigilance | EtatBelier::Avertit) && b.pos.dist(chien) < m.reg.portee_aboiement - 2.0) {
        c.aboyer = true;
    }
    c
}

fn commande_de_base(m: &Monde) -> Commande {
    let chien = m.chien.pos;
    let porte = m.carte.enclos.entree();
    let dehors = m.carte.enclos.cote.sortie();
    let libres: Vec<V2> = m.moutons.iter().filter(|s| !s.perdu && !s.abrite).map(|s| s.pos).collect();
    if libres.is_empty() {
        return Commande::default();
    }

    // Un loup emporte un mouton : on court le lui reprendre.
    if let Some(l) = m.loups.iter().find(|l| l.etat == EtatLoup::Repart && l.porte.is_some()) {
        let d = l.pos.dist(chien);
        return Commande { dir: chemin(&m.carte, chien, l.pos) + evite(m, chien) * 0.4, course: d > 3.0, aboyer: d < m.reg.portee_aboiement - 1.5 };
    }

    // Un loup rôde près de moutons libres : on court le chasser.
    let menace = m.loups.iter().filter(|l| l.etat == EtatLoup::Rode).min_by(|a, b| a.pos.dist(chien).total_cmp(&b.pos.dist(chien)));
    if let Some(l) = menace {
        let proche = libres.iter().any(|p| p.dist(l.pos) < 11.0);
        if proche {
            let d = l.pos.dist(chien);
            return Commande {
                dir: chemin(&m.carte, chien, l.pos) + evite(m, chien) * 0.5,
                course: d > 4.0,
                aboyer: d < m.reg.portee_aboiement - 1.5,
            };
        }
    }

    // Rassembler (pousser le plus isolé vers le groupe), puis conduire le groupe à la porte.
    // Les moutons isolés reviennent d'eux-mêmes : on s'occupe du groupe.
    let en_groupe: Vec<V2> = libres.iter().copied().filter(|p| libres.iter().any(|q| q != p && q.dist(*p) < 5.5)).collect();
    let libres = if en_groupe.is_empty() { libres } else { en_groupe };
    let centre = libres.iter().fold(V2::ZERO, |s, &p| s + p) / libres.len() as f32;
    let ecart = libres.iter().map(|p| p.dist(centre)).fold(0.0, f32::max);
    // Dans l'axe de la porte : on entre. Sinon on ramène d'abord le troupeau dans l'axe.
    let vers_pour = |p: V2| -> V2 {
        let rel = p - porte;
        let profondeur = rel.dot(dehors);
        let lateral = rel.dot(dehors.perp()).abs();
        if (profondeur < 7.0 && lateral < 1.2 + profondeur.max(0.0) * 0.4) || (profondeur < 1.5 && lateral < 5.0) {
            porte - dehors * 3.0
        } else {
            porte + dehors * 7.5
        }
    };
    let eparpille = ecart > 4.5 || m.carte.bloque(centre, 0.6);
    let (cible, sens) = if eparpille {
        let loin = *libres.iter().max_by(|a, b| a.dist(porte).total_cmp(&b.dist(porte))).unwrap();
        (loin, chemin(&m.carte, loin, vers_pour(loin)))
    } else {
        (centre, chemin(&m.carte, centre, vers_pour(centre)))
    };
    // Derrière le troupeau ; si la place manque (bord, arbre), on s'écarte de l'axe.
    let distance = if eparpille { 4.2 } else { 3.2 + ecart * 0.35 };
    let mut poste = cible - sens * distance;
    for angle in [0.0f32, 0.5, -0.5, 1.0, -1.0, 1.5, -1.5] {
        let (c, sn) = (angle.cos(), angle.sin());
        let d = v2(sens.x * c - sens.y * sn, sens.x * sn + sens.y * c);
        let p = cible - d * distance;
        if !m.carte.bloque(p, 0.8) && !m.carte.enclos.dedans(p, -2.5) {
            poste = p;
            break;
        }
    }
    // Du mauvais côté du troupeau : on le contourne en cercle, hors de sa zone de peur.
    let rel = chien - cible;
    let vers_poste = poste - cible;
    let ecart_angle = (rel.x * vers_poste.y - rel.y * vers_poste.x).atan2(rel.dot(vers_poste));
    let rayon_orbite = 7.0 + ecart * 0.5;
    let dir = if ecart_angle.abs() > 0.6 && rel.len() < rayon_orbite + 6.0 {
        // Un relais plus loin sur le cercle ; le parcours en largeur contourne les clôtures.
        let tourne: f32 = if ecart_angle > 0.0 { 0.7 } else { -0.7 };
        let (c, sn) = (tourne.cos(), tourne.sin());
        let u = rel.norm();
        let relais = cible + v2(u.x * c - u.y * sn, u.x * sn + u.y * c) * rayon_orbite;
        chemin(&m.carte, chien, relais.clamp_in(m.carte.demi - v2(1.5, 1.5)))
    } else if chien.dist(poste) > 1.3 {
        chemin(&m.carte, chien, poste)
    } else {
        sens * 0.55
    };
    // Jamais dans l'enclos : on garde la porte depuis l'extérieur.
    let dir = if m.carte.enclos.dedans(chien, -1.0) { dehors } else { dir };
    let course = chien.dist(poste) > 9.0;
    Commande { dir: (dir + evite(m, chien) * 1.6).limit(1.0), course, aboyer: false }
}

/// Évite de se coincer contre un arbre ou une clôture.
fn evite(m: &Monde, p: V2) -> V2 {
    let mut f = V2::ZERO;
    for o in &m.carte.obstacles {
        let d = p - o.pos;
        let e = d.len() - o.rayon;
        if e < 2.2 {
            f += d.norm() * ((2.2 - e) / 2.2);
        }
    }
    for mur in m.carte.murs() {
        let d = p - crate::v2::sur_segment(p, mur.a, mur.b);
        if d.len() < 1.6 {
            f += d.norm() * ((1.6 - d.len()) / 1.6);
        }
    }
    v2(f.x, f.y)
}

/// Direction à prendre pour aller de `depuis` à `but` en contournant les
/// obstacles (parcours en largeur sur une grille, depuis le but).
fn chemin(carte: &crate::carte::Carte, depuis: V2, but: V2) -> V2 {
    const PAS: f32 = 0.7;
    let w = (carte.demi.x * 2.0 / PAS) as i32 + 2;
    let h = (carte.demi.y * 2.0 / PAS) as i32 + 2;
    let cell = |p: V2| ((((p.x + carte.demi.x) / PAS) as i32).clamp(0, w - 1), (((p.y + carte.demi.y) / PAS) as i32).clamp(0, h - 1));
    let centre = |c: (i32, i32)| v2(c.0 as f32 * PAS - carte.demi.x + PAS / 2.0, c.1 as f32 * PAS - carte.demi.y + PAS / 2.0);
    let libre = |c: (i32, i32)| !carte.bloque(centre(c), 0.75);
    let d0 = cell(depuis);
    let mut dist = vec![i32::MAX; (w * h) as usize];
    let mut file = std::collections::VecDeque::new();
    let g = cell(but);
    dist[(g.1 * w + g.0) as usize] = 0;
    file.push_back(g);
    while let Some((x, y)) = file.pop_front() {
        let dc = dist[(y * w + x) as usize];
        if (x, y) == d0 {
            break;
        }
        for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1), (1, 1), (1, -1), (-1, 1), (-1, -1)] {
            let (nx, ny) = (x + dx, y + dy);
            if nx < 0 || ny < 0 || nx >= w || ny >= h {
                continue;
            }
            let k = (ny * w + nx) as usize;
            if dist[k] != i32::MAX || !libre((nx, ny)) {
                continue;
            }
            dist[k] = dc + 1;
            file.push_back((nx, ny));
        }
    }
    // Le voisin le plus proche du but.
    let mut meilleur = (i32::MAX, d0);
    for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1), (1, 1), (1, -1), (-1, 1), (-1, -1)] {
        let (nx, ny) = (d0.0 + dx, d0.1 + dy);
        if nx < 0 || ny < 0 || nx >= w || ny >= h {
            continue;
        }
        let k = dist[(ny * w + nx) as usize];
        if k < meilleur.0 {
            meilleur = (k, (nx, ny));
        }
    }
    if meilleur.0 == i32::MAX {
        return (but - depuis).norm();
    }
    (centre(meilleur.1) - depuis).norm()
}
