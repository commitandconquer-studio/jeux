//! La simulation : le chien, les moutons (des boids qui fuient le chien et
//! les loups) et les loups. Un pas de temps à la fois, sans affichage.
use crate::carte::{Carte, Niveau};
use crate::v2::{Hasard, V2, sur_segment, v2};

/// Pas de simulation fixe : le jeu se déroule pareil quelle que soit la fréquence d'affichage.
pub const PAS_FIXE: f32 = 1.0 / 60.0;

/// Tous les réglages de gameplay au même endroit (distances en mètres, durées en secondes).
#[derive(Clone, Copy, Debug)]
pub struct Reglages {
    // Le chien.
    pub vitesse_chien: f32,
    pub vitesse_course: f32,
    pub rayon_chien: f32,
    /// Vitesse de rattrapage de la vitesse voulue (plus grand = plus nerveux).
    pub reactivite_chien: f32,
    pub duree_course: f32,
    pub duree_recuperation: f32,
    // L'aboiement.
    pub portee_aboiement: f32,
    pub recharge_aboiement: f32,
    pub vitesse_onde: f32,
    // Les moutons.
    pub rayon_mouton: f32,
    pub separation_r: f32,
    pub voisinage_r: f32,
    pub peur_chien_r: f32,
    pub peur_loup_r: f32,
    // Les loups.
    pub vitesse_loup: f32,
    pub vitesse_fuite_loup: f32,
}

impl Default for Reglages {
    fn default() -> Self {
        Reglages {
            vitesse_chien: 5.2,
            vitesse_course: 8.2,
            rayon_chien: 0.45,
            reactivite_chien: 11.0,
            duree_course: 3.2,
            duree_recuperation: 6.0,
            portee_aboiement: 10.0,
            recharge_aboiement: 1.8,
            vitesse_onde: 26.0,
            rayon_mouton: 0.38,
            separation_r: 1.5,
            voisinage_r: 5.0,
            peur_chien_r: 4.8,
            peur_loup_r: 8.0,
            vitesse_loup: 5.4,
            vitesse_fuite_loup: 6.4,
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Commande {
    /// Direction voulue (longueur ≤ 1).
    pub dir: V2,
    pub course: bool,
    pub aboyer: bool,
}

#[derive(Clone, Debug)]
pub struct Chien {
    pub pos: V2,
    pub vel: V2,
    pub cap: f32,
    /// De 0 (épuisé) à 1.
    pub endurance: f32,
    pub essouffle: bool,
    pub recharge: f32,
    /// Rayon de l'onde d'aboiement en cours (0 = aucune).
    pub onde: f32,
    pub court: bool,
}

#[derive(Clone, Debug)]
pub struct Mouton {
    pub pos: V2,
    pub vel: V2,
    pub cap: f32,
    pub repos: f32,
    /// De 0 (tranquille) à 1 (affolé).
    pub panique: f32,
    pub perdu: bool,
    pub abrite: bool,
    pub phase: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EtatLoup {
    Absent,
    Rode,
    Fuit,
    Repart,
}

#[derive(Clone, Debug)]
pub struct Loup {
    pub pos: V2,
    pub vel: V2,
    pub etat: EtatLoup,
    pub timer: f32,
    pub cible: Option<usize>,
    pub morsure: f32,
    pub apparition: f32,
    pub bord: V2,
    pub phase: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Fin {
    Victoire,
    Pertes,
    Temps,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Evenement {
    Aboiement,
    Bele(usize),
    Entre(usize),
    Sorti(usize),
    Mange(usize),
    LoupArrive(usize),
    LoupFuit(usize),
    Fin(Fin),
}

pub struct Monde {
    pub carte: Carte,
    pub niveau: Niveau,
    pub chien: Chien,
    pub moutons: Vec<Mouton>,
    pub loups: Vec<Loup>,
    pub temps: f32,
    pub fin: Option<Fin>,
    pub evenements: Vec<Evenement>,
    pub reg: Reglages,
    /// Temps pendant lequel tous les moutons restants sont dans l'enclos.
    tenu: f32,
    /// Temps pas encore simulé (le pas fixe consomme ce reste).
    reste: f32,
    hasard: Hasard,
}

impl Monde {
    pub fn nouveau(carte: Carte, niveau: Niveau) -> Monde {
        let mut hasard = Hasard::new(carte.graine ^ 0xB0B);
        let moutons = carte
            .moutons
            .iter()
            .map(|&p| Mouton {
                pos: p,
                vel: V2::ZERO,
                cap: hasard.entre(0.0, std::f32::consts::TAU),
                repos: hasard.entre(0.0, 4.0),
                panique: 0.0,
                perdu: false,
                abrite: false,
                phase: hasard.entre(0.0, std::f32::consts::TAU),
            })
            .collect();
        let loups = carte
            .loups
            .iter()
            .map(|l| Loup {
                pos: l.bord,
                vel: V2::ZERO,
                etat: EtatLoup::Absent,
                timer: 0.0,
                cible: None,
                morsure: 0.0,
                apparition: l.apparition,
                bord: l.bord,
                phase: hasard.entre(0.0, std::f32::consts::TAU),
            })
            .collect();
        let chien = Chien { pos: carte.depart_chien, vel: V2::ZERO, cap: 0.0, endurance: 1.0, essouffle: false, recharge: 0.0, onde: 0.0, court: false };
        Monde { carte, niveau, chien, moutons, loups, temps: 0.0, fin: None, evenements: Vec::new(), reg: Reglages::default(), tenu: 0.0, reste: 0.0, hasard }
    }

    pub fn abrites(&self) -> usize {
        self.moutons.iter().filter(|m| !m.perdu && m.abrite).count()
    }
    pub fn perdus(&self) -> usize {
        self.moutons.iter().filter(|m| m.perdu).count()
    }
    pub fn vivants(&self) -> usize {
        self.moutons.len() - self.perdus()
    }
    pub fn temps_restant(&self) -> f32 {
        (self.temps_limite() - self.temps).max(0.0)
    }
    pub fn temps_limite(&self) -> f32 {
        self.niveau.temps_ref * 2.4
    }

    /// Étoiles gagnées (0 tant que la partie n'est pas gagnée).
    pub fn etoiles(&self) -> u32 {
        if self.fin != Some(Fin::Victoire) {
            return 0;
        }
        1 + (self.temps <= self.niveau.temps_ref) as u32 + (self.perdus() == 0) as u32
    }

    /// Fait avancer le monde de `dt` secondes, par pas fixes de `PAS_FIXE`.
    /// Les événements des pas sont dans `evenements` (vidés à chaque appel).
    pub fn pas(&mut self, dt: f32, cmd: Commande) {
        self.evenements.clear();
        if self.fin.is_some() {
            return;
        }
        self.reste += dt.min(0.1);
        let mut cmd = cmd;
        while self.reste >= PAS_FIXE - 1e-6 && self.fin.is_none() {
            self.reste -= PAS_FIXE;
            self.un_pas(PAS_FIXE, cmd);
            // L'aboiement est une pression : il ne se répète pas d'un pas à l'autre.
            cmd.aboyer = false;
        }
    }

    fn un_pas(&mut self, dt: f32, cmd: Commande) {
        self.temps += dt;
        self.pas_chien(dt, cmd);
        self.pas_loups(dt);
        self.pas_moutons(dt);
        self.verifier_fin(dt);
    }

    fn verifier_fin(&mut self, dt: f32) {
        let vivants = self.vivants();
        if self.perdus() as u32 > self.niveau.pertes_max {
            self.terminer(Fin::Pertes);
        } else if self.temps >= self.temps_limite() {
            self.terminer(Fin::Temps);
        } else if vivants > 0 && self.abrites() == vivants {
            self.tenu += dt;
            if self.tenu >= 1.2 {
                self.terminer(Fin::Victoire);
            }
        } else {
            self.tenu = 0.0;
        }
    }

    fn terminer(&mut self, f: Fin) {
        self.fin = Some(f);
        self.evenements.push(Evenement::Fin(f));
    }

    fn pas_chien(&mut self, dt: f32, cmd: Commande) {
        let c = &mut self.chien;
        let veut = cmd.dir.limit(1.0);
        let bouge = veut.len2() > 0.01;
        let court = cmd.course && bouge && !c.essouffle && c.endurance > 0.0;
        c.court = court;
        let vitesse = if court { self.reg.vitesse_course } else { self.reg.vitesse_chien };
        c.vel = c.vel.lerp(veut * vitesse, (dt * self.reg.reactivite_chien).min(1.0));
        c.pos += c.vel * dt;
        if court {
            c.endurance = (c.endurance - dt / self.reg.duree_course).max(0.0);
            if c.endurance <= 0.0 {
                c.essouffle = true;
            }
        } else {
            c.endurance = (c.endurance + dt / self.reg.duree_recuperation).min(1.0);
            if c.essouffle && c.endurance >= 0.35 {
                c.essouffle = false;
            }
        }
        if c.vel.len() > 0.4 {
            c.cap = c.vel.angle();
        }
        c.recharge = (c.recharge - dt).max(0.0);
        if c.onde > 0.0 {
            c.onde += dt * self.reg.vitesse_onde;
            if c.onde > self.reg.portee_aboiement + 2.0 {
                c.onde = 0.0;
            }
        }
        let (mut pos, mut vel) = (c.pos, c.vel);
        Self::collisions(&self.carte, &mut pos, &mut vel, self.reg.rayon_chien, 0.3);
        self.chien.pos = pos;
        self.chien.vel = vel;

        if cmd.aboyer && self.chien.recharge <= 0.0 {
            self.chien.recharge = self.reg.recharge_aboiement;
            self.chien.onde = 0.01;
            self.evenements.push(Evenement::Aboiement);
            let chien = self.chien.pos;
            for m in self.moutons.iter_mut().filter(|m| !m.perdu) {
                let d = m.pos.dist(chien);
                if d < self.reg.portee_aboiement {
                    let t = 1.0 - d / self.reg.portee_aboiement;
                    m.vel += (m.pos - chien).norm() * (1.5 + 3.5 * t);
                    m.panique = m.panique.max(0.5 + 0.5 * t);
                }
            }
            for (i, l) in self.loups.iter_mut().enumerate() {
                if matches!(l.etat, EtatLoup::Rode | EtatLoup::Fuit) && l.pos.dist(chien) < self.reg.portee_aboiement {
                    l.etat = EtatLoup::Fuit;
                    l.timer = 4.5;
                    l.cible = None;
                    l.morsure = 0.0;
                    self.evenements.push(Evenement::LoupFuit(i));
                }
            }
        }
    }

    /// Repousse un disque hors des obstacles, des clôtures et des bords du champ.
    fn collisions(carte: &Carte, pos: &mut V2, vel: &mut V2, rayon: f32, rayon_mur: f32) {
        for o in &carte.obstacles {
            let d = *pos - o.pos;
            let min = o.rayon + rayon;
            let l = d.len();
            if l < min {
                let n = if l < 1e-4 { v2(1.0, 0.0) } else { d / l };
                *pos = o.pos + n * min;
                let vn = vel.dot(n);
                if vn < 0.0 {
                    *vel -= n * vn;
                }
            }
        }
        for m in carte.murs() {
            let q = sur_segment(*pos, m.a, m.b);
            let d = *pos - q;
            let l = d.len();
            if l < rayon_mur {
                let n = if l < 1e-4 { (m.b - m.a).perp().norm() } else { d / l };
                *pos = q + n * rayon_mur;
                let vn = vel.dot(n);
                if vn < 0.0 {
                    *vel -= n * vn;
                }
            }
        }
        let lim = carte.demi - v2(rayon, rayon);
        if pos.x.abs() > lim.x {
            pos.x = pos.x.clamp(-lim.x, lim.x);
            vel.x = 0.0;
        }
        if pos.y.abs() > lim.y {
            pos.y = pos.y.clamp(-lim.y, lim.y);
            vel.y = 0.0;
        }
    }

    /// Force qui éloigne `pos` des obstacles et clôtures proches (pour guider un animal).
    fn evitement(carte: &Carte, pos: V2, rayon: f32, portee: f32) -> V2 {
        let mut f = V2::ZERO;
        for o in &carte.obstacles {
            let d = pos - o.pos;
            let ecart = d.len() - o.rayon - rayon;
            if ecart < portee {
                f += d.norm() * ((portee - ecart) / portee);
            }
        }
        for m in carte.murs() {
            let d = pos - sur_segment(pos, m.a, m.b);
            let ecart = d.len() - rayon;
            if ecart < portee * 0.7 {
                f += d.norm() * ((portee * 0.7 - ecart) / (portee * 0.7));
            }
        }
        let lim = carte.demi;
        let marge = portee;
        if pos.x > lim.x - marge {
            f.x -= (pos.x - (lim.x - marge)) / marge * 0.6;
        }
        if pos.x < -lim.x + marge {
            f.x += (-lim.x + marge - pos.x) / marge * 0.6;
        }
        if pos.y > lim.y - marge {
            f.y -= (pos.y - (lim.y - marge)) / marge * 0.6;
        }
        if pos.y < -lim.y + marge {
            f.y += (-lim.y + marge - pos.y) / marge * 0.6;
        }
        f
    }

    fn pas_moutons(&mut self, dt: f32) {
        let n = self.moutons.len();
        let pos: Vec<V2> = self.moutons.iter().map(|m| m.pos).collect();
        let vel: Vec<V2> = self.moutons.iter().map(|m| m.vel).collect();
        let vivant: Vec<bool> = self.moutons.iter().map(|m| !m.perdu).collect();
        let chien = self.chien.pos;
        let vitesse_chien = self.chien.vel.len();
        // Le troupeau « libre » : les moutons de l'enclos n'attirent plus les égarés (ils se colleraient à la clôture).
        let libres: Vec<V2> = (0..n).filter(|&i| vivant[i] && !self.moutons[i].abrite).map(|i| pos[i]).collect();
        let centre_troupeau = if libres.is_empty() { V2::ZERO } else { libres.iter().fold(V2::ZERO, |s, &p| s + p) / libres.len() as f32 };
        let loups: Vec<V2> = self.loups.iter().filter(|l| matches!(l.etat, EtatLoup::Rode)).map(|l| l.pos).collect();

        for i in 0..n {
            if !vivant[i] {
                continue;
            }
            let (p, v) = (pos[i], vel[i]);

            // Les trois règles des boids : séparation, alignement, cohésion.
            let (mut sep, mut ali, mut coh) = (V2::ZERO, V2::ZERO, V2::ZERO);
            let mut voisins = 0.0;
            for j in 0..n {
                if j == i || !vivant[j] {
                    continue;
                }
                let d = pos[j] - p;
                let dist = d.len();
                if dist < self.reg.separation_r && dist > 1e-4 {
                    sep -= d / dist * (1.0 - dist / self.reg.separation_r);
                }
                if dist < self.reg.voisinage_r {
                    ali += vel[j];
                    coh += pos[j];
                    voisins += 1.0;
                }
            }
            let mut acc = sep * 16.0;
            if voisins > 0.0 {
                ali = ali / voisins - v;
                coh = coh / voisins - p;
                acc += ali * 1.4 + coh.norm() * (coh.len() / 3.0).min(1.0) * 1.7;
            }

            // Un mouton isolé rejoint le troupeau : ils sont grégaires.
            if voisins == 0.0 {
                acc += (centre_troupeau - p).norm() * 2.4;
            }

            // La peur : le chien (plus il court, plus il fait peur) et les loups.
            let mut menace: f32 = 0.0;
            let rayon_peur = self.reg.peur_chien_r + vitesse_chien * 0.25;
            let d = p - chien;
            let dist = d.len();
            if dist < rayon_peur {
                let t = 1.0 - dist / rayon_peur;
                acc += d.norm() * (16.0 * t + 2.5);
                menace = menace.max(t.sqrt());
            }
            for &lp in &loups {
                let d = p - lp;
                let dist = d.len();
                if dist < self.reg.peur_loup_r {
                    let t = 1.0 - dist / self.reg.peur_loup_r;
                    acc += d.norm() * (26.0 * t + 4.0);
                    menace = 1.0;
                }
            }

            // Obstacles et clôtures : on les contourne.
            acc += Self::evitement(&self.carte, p, self.reg.rayon_mouton, 1.6) * 9.0;

            // Tranquille : il broute, par moments il se déplace un peu.
            let m = &mut self.moutons[i];
            if menace < 0.02 && m.panique < 0.1 {
                m.cap += (self.hasard.f() - 0.5) * dt * 3.0;
                let voulu = if m.repos > 0.0 {
                    m.repos -= dt;
                    V2::ZERO
                } else {
                    if self.hasard.f() < dt * 0.25 {
                        m.repos = self.hasard.entre(2.0, 6.0);
                    }
                    V2::from_angle(m.cap) * 0.5
                };
                acc += (voulu - v) * 2.0;
            }

            let vers = if menace > m.panique { 9.0 } else { 1.1 };
            m.panique += (menace - m.panique) * (dt * vers).min(1.0);
            let vmax = 1.2 + m.panique * 3.0;
            let mut vel = (v + acc * dt) * (1.0 - 2.4 * dt).max(0.0);
            vel = vel.limit(vmax);
            let mut pos = p + vel * dt;
            Self::collisions(&self.carte, &mut pos, &mut vel, self.reg.rayon_mouton, self.reg.rayon_mouton * 0.8);
            m.pos = pos;
            m.vel = vel;
            m.phase += dt * (1.5 + vel.len() * 2.2);

            let dedans = self.carte.enclos.dedans(pos, 0.1);
            if dedans != m.abrite {
                m.abrite = dedans;
                self.evenements.push(if dedans { Evenement::Entre(i) } else { Evenement::Sorti(i) });
            }
            let proba = if m.panique > 0.3 { 0.35 } else { 0.03 };
            if self.hasard.f() < dt * proba {
                self.evenements.push(Evenement::Bele(i));
            }
        }
    }

    fn pas_loups(&mut self, dt: f32) {
        let chien = self.chien.pos;
        let temps = self.temps;
        for i in 0..self.loups.len() {
            match self.loups[i].etat {
                EtatLoup::Absent => {
                    let l = &mut self.loups[i];
                    if temps >= l.apparition {
                        l.pos = l.bord;
                        l.vel = V2::ZERO;
                        l.etat = EtatLoup::Rode;
                        l.cible = None;
                        l.morsure = 0.0;
                        self.evenements.push(Evenement::LoupArrive(i));
                    }
                    continue;
                }
                EtatLoup::Rode => {
                    // Le chien trop près lui fait peur.
                    if self.loups[i].pos.dist(chien) < 2.8 {
                        let l = &mut self.loups[i];
                        l.etat = EtatLoup::Fuit;
                        l.timer = 3.5;
                        l.cible = None;
                        l.morsure = 0.0;
                        self.evenements.push(Evenement::LoupFuit(i));
                        continue;
                    }
                    self.loup_chasse(i, dt);
                }
                EtatLoup::Fuit => {
                    let l = &mut self.loups[i];
                    l.timer -= dt;
                    let d = l.pos - chien;
                    let dir = (d.norm() + Self::evitement(&self.carte, l.pos, 0.5, 2.0) * 1.2).norm();
                    l.vel = l.vel.lerp(dir * self.reg.vitesse_fuite_loup, (dt * 6.0).min(1.0));
                    if l.timer <= 0.0 {
                        l.etat = EtatLoup::Rode;
                    }
                }
                EtatLoup::Repart => {
                    let carte_demi = self.carte.demi;
                    let l = &mut self.loups[i];
                    let sortie = if (l.pos.x.abs() - carte_demi.x).abs() < (l.pos.y.abs() - carte_demi.y).abs() {
                        v2(l.pos.x.signum() * (carte_demi.x + 3.0), l.pos.y)
                    } else {
                        v2(l.pos.x, l.pos.y.signum() * (carte_demi.y + 3.0))
                    };
                    let dir = ((sortie - l.pos).norm() + Self::evitement(&self.carte, l.pos, 0.5, 2.0)).norm();
                    l.vel = l.vel.lerp(dir * self.reg.vitesse_fuite_loup, (dt * 6.0).min(1.0));
                    if l.pos.x.abs() > carte_demi.x + 1.5 || l.pos.y.abs() > carte_demi.y + 1.5 {
                        l.etat = EtatLoup::Absent;
                        l.apparition = temps + self.niveau.loup_ecart * 0.8;
                        l.bord = l.pos.clamp_in(carte_demi + v2(1.0, 1.0));
                    }
                }
            }
            let (mut pos, mut vel) = (self.loups[i].pos + self.loups[i].vel * dt, self.loups[i].vel);
            if self.loups[i].etat != EtatLoup::Repart || self.carte.demi.x - pos.x.abs() > 0.5 && self.carte.demi.y - pos.y.abs() > 0.5 {
                // Hors du champ (arrivée et départ), les murs invisibles du bord ne comptent pas.
                if pos.x.abs() < self.carte.demi.x && pos.y.abs() < self.carte.demi.y {
                    Self::collisions(&self.carte, &mut pos, &mut vel, 0.5, 0.45);
                } else {
                    for o in &self.carte.obstacles {
                        let d = pos - o.pos;
                        if d.len() < o.rayon + 0.5 {
                            pos = o.pos + d.norm() * (o.rayon + 0.5);
                        }
                    }
                }
            }
            let l = &mut self.loups[i];
            l.pos = pos;
            l.vel = vel;
            l.phase += dt * (1.0 + vel.len() * 2.0);
        }
    }

    fn loup_chasse(&mut self, i: usize, dt: f32) {
        // Il vise le mouton libre le plus proche (les moutons de l'enclos sont à l'abri).
        let lp = self.loups[i].pos;
        let valide = |m: &Mouton| !m.perdu && !m.abrite;
        let cible = match self.loups[i].cible {
            Some(c) if valide(&self.moutons[c]) && self.hasard.f() > dt * 0.6 => Some(c),
            _ => self
                .moutons
                .iter()
                .enumerate()
                .filter(|(_, m)| valide(m))
                .min_by(|a, b| a.1.pos.dist(lp).total_cmp(&b.1.pos.dist(lp)))
                .map(|(k, _)| k),
        };
        self.loups[i].cible = cible;
        let Some(c) = cible else {
            let l = &mut self.loups[i];
            l.vel = l.vel.lerp(V2::ZERO, (dt * 4.0).min(1.0));
            return;
        };
        let vers = self.moutons[c].pos + self.moutons[c].vel * 0.25;
        let l = &mut self.loups[i];
        let dist = l.pos.dist(vers);
        let dir = ((vers - l.pos).norm() + Self::evitement(&self.carte, l.pos, 0.5, 1.8) * 1.4).norm();
        let vitesse = if dist < 3.5 { self.reg.vitesse_loup * 1.15 } else { self.reg.vitesse_loup };
        l.vel = l.vel.lerp(dir * vitesse, (dt * 5.0).min(1.0));
        if self.moutons[c].pos.dist(l.pos) < 1.0 {
            l.morsure += dt;
            if l.morsure > 0.7 {
                self.moutons[c].perdu = true;
                self.evenements.push(Evenement::Mange(c));
                l.etat = EtatLoup::Repart;
                l.cible = None;
                l.morsure = 0.0;
            }
        } else {
            l.morsure = (l.morsure - dt).max(0.0);
        }
    }
}

