//! La simulation : le chien, les moutons (des boids qui fuient le chien et
//! les loups) et les loups. Un pas de temps à la fois, sans affichage.
use crate::carte::{Carte, GenreLoup, Niveau, TypeMouton};
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
    // Les béliers.
    pub belier_vigilance_r: f32,
    pub belier_vitesse_charge: f32,
    pub belier_avertissement: f32,
    pub belier_charge: f32,
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
            belier_vigilance_r: 7.0,
            belier_vitesse_charge: 9.5,
            belier_avertissement: 1.0,
            belier_charge: 0.7,
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
    /// D'où part l'onde, et son numéro (chaque cible n'est touchée qu'une fois par onde).
    pub onde_origine: V2,
    pub onde_id: u32,
    /// Temps restant de l'aboiement : le chien se plante sur ses pattes et ralentit.
    pub aboie: f32,
    pub court: bool,
    /// Temps restant où le chien est sonné (après un coup de bélier).
    pub etourdi: f32,
}

/// Ce que fait un mouton (dérivé de sa peur et de ses envies, pour l'animation et les tests).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EtatMouton {
    /// Il broute, immobile.
    Broute,
    /// Il flâne.
    Flane,
    /// Il suit le troupeau, ou le rejoint.
    Troupeau,
    /// Il a repéré une menace et se met en garde.
    Alerte,
    /// Il fuit.
    Fuit,
    /// Il retrouve son calme.
    Calme,
    /// Il est dans l'enclos.
    Abrite,
}

#[derive(Clone, Debug)]
pub struct Mouton {
    pub genre: TypeMouton,
    pub etat: EtatMouton,
    /// Le loup qui l'emporte (il peut encore être libéré).
    pub porte_par: Option<usize>,
    /// Temps pendant lequel il est affolé sans avancer (coincé).
    pub coince: f32,
    pub pos: V2,
    pub vel: V2,
    pub cap: f32,
    pub repos: f32,
    /// De 0 (tranquille) à 1 (affolé).
    pub panique: f32,
    pub perdu: bool,
    pub abrite: bool,
    pub phase: f32,
    pub onde_vue: u32,
    /// Temps restant du sursaut (l'onde vient de passer) : sert à l'animation.
    pub sursaut: f32,
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
    pub genre: GenreLoup,
    /// Le mouton qu'il emporte.
    pub porte: Option<usize>,
    pub pos: V2,
    pub vel: V2,
    pub etat: EtatLoup,
    pub timer: f32,
    pub cible: Option<usize>,
    pub morsure: f32,
    pub apparition: f32,
    pub bord: V2,
    pub phase: f32,
    pub onde_vue: u32,
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
    /// L'onde d'aboiement vient d'atteindre un mouton.
    Sursaut(usize),
    Mange(usize),
    LoupArrive(usize),
    LoupFuit(usize),
    /// Un loup emporte un mouton : il est encore temps de le délivrer.
    Capture(usize),
    /// Le mouton est délivré.
    Libere(usize),
    /// Le bélier s'arrête et baisse les cornes : la charge arrive.
    BelierAvertit(usize),
    BelierCharge(usize),
    /// Un aboiement a calmé le bélier.
    BelierCalme(usize),
    /// Le bélier a touché le chien.
    ChienPercute,
    Fin(Fin),
}

/// Les phases d'un bélier : calme, vigilance, avertissement (il baisse les cornes), charge, récupération.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EtatBelier {
    Calme,
    Vigilance,
    Avertit,
    Charge,
    Recupere,
}

#[derive(Clone, Debug)]
pub struct Belier {
    pub pos: V2,
    pub vel: V2,
    pub cap: f32,
    pub etat: EtatBelier,
    pub timer: f32,
    /// Temps avant qu'il puisse s'énerver de nouveau.
    pub repos: f32,
    pub direction: V2,
    pub phase: f32,
    pub onde_vue: u32,
}

pub struct Monde {
    pub carte: Carte,
    pub niveau: Niveau,
    pub chien: Chien,
    pub moutons: Vec<Mouton>,
    pub loups: Vec<Loup>,
    pub beliers: Vec<Belier>,
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
            .zip(carte.types.iter())
            .map(|(&p, &genre)| Mouton {
                genre,
                pos: p,
                vel: V2::ZERO,
                cap: hasard.entre(0.0, std::f32::consts::TAU),
                repos: hasard.entre(0.0, 4.0),
                panique: 0.0,
                perdu: false,
                abrite: false,
                phase: hasard.entre(0.0, std::f32::consts::TAU),
                onde_vue: 0,
                sursaut: 0.0,
                etat: EtatMouton::Broute,
                porte_par: None,
                coince: 0.0,
            })
            .collect();
        let beliers = carte
            .beliers
            .iter()
            .map(|&p| Belier { pos: p, vel: V2::ZERO, cap: 0.0, etat: EtatBelier::Calme, timer: 0.0, repos: 2.0, direction: V2::ZERO, phase: 0.0, onde_vue: 0 })
            .collect();
        let loups = carte
            .loups
            .iter()
            .map(|l| Loup {
                genre: l.genre,
                porte: None,
                pos: l.bord,
                vel: V2::ZERO,
                etat: EtatLoup::Absent,
                timer: 0.0,
                cible: None,
                morsure: 0.0,
                apparition: l.apparition,
                bord: l.bord,
                phase: hasard.entre(0.0, std::f32::consts::TAU),
                onde_vue: 0,
            })
            .collect();
        let chien = Chien { pos: carte.depart_chien, vel: V2::ZERO, cap: 0.0, endurance: 1.0, essouffle: false, recharge: 0.0, onde: 0.0, onde_origine: V2::ZERO, onde_id: 0, aboie: 0.0, court: false, etourdi: 0.0 };
        Monde { carte, niveau, chien, moutons, loups, beliers, temps: 0.0, fin: None, evenements: Vec::new(), reg: Reglages::default(), tenu: 0.0, reste: 0.0, hasard }
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
        self.pas_beliers(dt);
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
        let vitesse = if c.aboie > 0.0 { vitesse * 0.45 } else { vitesse };
        let vitesse = if c.etourdi > 0.0 { vitesse * 0.35 } else { vitesse };
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
        c.aboie = (c.aboie - dt).max(0.0);
        c.etourdi = (c.etourdi - dt).max(0.0);
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
            self.chien.onde_origine = self.chien.pos;
            self.chien.onde_id += 1;
            self.chien.aboie = 0.28;
            self.evenements.push(Evenement::Aboiement);
        }
        self.propager_onde();
    }

    /// L'onde grandit : chaque animal qu'elle atteint réagit, une seule fois par onde.
    fn propager_onde(&mut self) {
        let (rayon, origine, id) = (self.chien.onde, self.chien.onde_origine, self.chien.onde_id);
        if rayon <= 0.0 {
            return;
        }
        let portee = self.reg.portee_aboiement;
        for (i, m) in self.moutons.iter_mut().enumerate() {
            let d = m.pos.dist(origine);
            if m.perdu || m.onde_vue == id || d > rayon.min(portee) {
                continue;
            }
            m.onde_vue = id;
            m.sursaut = 0.4;
            let t = 1.0 - d / portee;
            m.vel += (m.pos - origine).norm() * (1.5 + 3.5 * t);
            m.panique = m.panique.max(0.5 + 0.5 * t);
            self.evenements.push(Evenement::Sursaut(i));
        }
        for (i, b) in self.beliers.iter_mut().enumerate() {
            if b.onde_vue == id || b.pos.dist(origine) > rayon.min(portee) {
                continue;
            }
            b.onde_vue = id;
            // Une charge déjà lancée ne s'arrête pas ; avant, l'aboiement la fait avorter.
            if b.etat != EtatBelier::Charge {
                b.etat = EtatBelier::Calme;
                b.repos = 5.0;
                b.vel = (b.pos - origine).norm() * 3.5;
                self.evenements.push(Evenement::BelierCalme(i));
            }
        }
        for i in 0..self.loups.len() {
            let l = &mut self.loups[i];
            if l.onde_vue == id || !matches!(l.etat, EtatLoup::Rode | EtatLoup::Fuit | EtatLoup::Repart) || l.pos.dist(origine) > rayon.min(portee) {
                continue;
            }
            l.onde_vue = id;
            if l.etat == EtatLoup::Repart {
                self.lacher(i);
                continue;
            }
            l.etat = EtatLoup::Fuit;
            l.timer = 4.5;
            l.cible = None;
            l.morsure = 0.0;
            self.evenements.push(Evenement::LoupFuit(i));
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
        let vivant: Vec<bool> = self.moutons.iter().map(|m| !m.perdu && m.porte_par.is_none()).collect();
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
            let genre = self.moutons[i].genre;

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
                acc += ali * 1.4 + coh.norm() * (coh.len() / 3.0).min(1.0) * 1.7 * genre.cohesion();
            }

            // Un mouton isolé rejoint le troupeau : ils sont grégaires.
            if voisins == 0.0 {
                acc += (centre_troupeau - p).norm() * 2.4;
            }

            // La peur : le chien (plus il court, plus il fait peur) et les loups.
            let mut menace: f32 = 0.0;
            let rayon_peur = (self.reg.peur_chien_r + vitesse_chien * 0.25) * genre.peur();
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

            // Dans l'enclos : moins de peur du chien (on est à l'abri), on s'éloigne de la porte et on s'étale doucement.
            if self.moutons[i].abrite {
                let e = self.carte.enclos;
                acc -= d.norm() * (16.0 * (1.0 - dist / rayon_peur).max(0.0) + 2.5) * if dist < rayon_peur { 0.6 } else { 0.0 };
                let a_la_porte = (p - e.entree()).len();
                if a_la_porte < 2.2 {
                    acc += (e.centre - p).norm() * (2.2 - a_la_porte) * 4.0;
                }
                menace *= 0.4;
            } else if self.carte.enclos.dedans(p, -0.15) {
                // Sur le seuil : un mouton qui vient d'entrer s'avance tout seul à l'intérieur.
                acc += (self.carte.enclos.centre - p).norm() * 2.5;
            }

            // Obstacles et clôtures : on les contourne.
            acc += Self::evitement(&self.carte, p, genre.rayon(), 1.6) * 9.0;

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
            let vmax = (1.2 + m.panique * 3.0) * genre.vitesse();
            let mut vel = (v + acc * dt) * (1.0 - 2.4 * dt).max(0.0);
            vel = vel.limit(vmax);
            let mut pos = p + vel * dt;
            Self::collisions(&self.carte, &mut pos, &mut vel, genre.rayon(), genre.rayon() * 0.8);
            m.pos = pos;
            m.vel = vel;
            m.sursaut = (m.sursaut - dt).max(0.0);
            m.phase += dt * (1.5 + vel.len() * 2.2);

            // Coincé contre un obstacle alors qu'il est affolé : un petit écart le dégage.
            if m.panique > 0.3 && vel.len() < 0.25 {
                m.coince += dt;
                if m.coince > 1.2 {
                    m.coince = 0.0;
                    let a = self.hasard.entre(0.0, std::f32::consts::TAU);
                    m.vel += V2::from_angle(a) * 2.5 + (self.carte.enclos.centre - m.pos).norm() * 0.5;
                }
            } else {
                m.coince = 0.0;
            }
            m.etat = if m.abrite {
                EtatMouton::Abrite
            } else if m.panique > 0.55 {
                EtatMouton::Fuit
            } else if menace > 0.02 || m.panique > 0.3 {
                EtatMouton::Alerte
            } else if m.panique > 0.08 {
                EtatMouton::Calme
            } else if voisins == 0.0 {
                EtatMouton::Troupeau
            } else if m.repos > 0.0 || vel.len() < 0.2 {
                EtatMouton::Broute
            } else if vel.len() > 0.6 {
                EtatMouton::Troupeau
            } else {
                EtatMouton::Flane
            };

            // Sécurisé quand il a passé la porte d'au moins 0,7 ; il le reste tant qu'il ne ressort pas.
            // (Contre les clôtures, un mouton ne peut pas être plus près que 0,3 : la marge latérale est petite.)
            let e = self.carte.enclos;
            let passe = (e.entree() - pos).dot(e.cote.sortie());
            let dedans = if m.abrite { e.dedans(pos, -0.4) } else { e.dedans(pos, 0.25) && passe >= 0.7 };
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
                    // Le chien qui s'approche, ou un aboiement, lui fait lâcher sa proie.
                    if self.loups[i].pos.dist(chien) < 3.0 {
                        self.lacher(i);
                        continue;
                    }
                    let carte_demi = self.carte.demi;
                    let charge = self.loups[i].porte.is_some();
                    let l = &mut self.loups[i];
                    let sortie = if (l.pos.x.abs() - carte_demi.x).abs() < (l.pos.y.abs() - carte_demi.y).abs() {
                        v2(l.pos.x.signum() * (carte_demi.x + 3.0), l.pos.y)
                    } else {
                        v2(l.pos.x, l.pos.y.signum() * (carte_demi.y + 3.0))
                    };
                    let dir = ((sortie - l.pos).norm() + Self::evitement(&self.carte, l.pos, 0.5, 2.0)).norm();
                    // Chargé d'un mouton, il va moins vite.
                    let v = if charge { self.reg.vitesse_loup * 0.8 } else { self.reg.vitesse_fuite_loup };
                    l.vel = l.vel.lerp(dir * v, (dt * 6.0).min(1.0));
                    if l.pos.x.abs() > carte_demi.x + 1.5 || l.pos.y.abs() > carte_demi.y + 1.5 {
                        l.etat = EtatLoup::Absent;
                        l.apparition = temps + self.niveau.loup_ecart * 0.8;
                        l.bord = l.pos.clamp_in(carte_demi + v2(1.0, 1.0));
                        // Sorti du champ avec sa proie : le mouton est perdu.
                        if let Some(c) = l.porte.take() {
                            self.moutons[c].perdu = true;
                            self.moutons[c].porte_par = None;
                            self.evenements.push(Evenement::Mange(c));
                        }
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
            if let Some(c) = l.porte {
                let derriere = if vel.len() > 0.2 { vel.norm() * -0.7 } else { V2::ZERO };
                self.moutons[c].pos = pos + derriere;
                self.moutons[c].vel = V2::ZERO;
            }
        }
    }

    /// Le loup lâche sa proie et s'enfuit.
    fn lacher(&mut self, i: usize) {
        let chien = self.chien.pos;
        let l = &mut self.loups[i];
        l.etat = EtatLoup::Fuit;
        l.timer = 4.5;
        l.cible = None;
        l.morsure = 0.0;
        if let Some(c) = l.porte.take() {
            let m = &mut self.moutons[c];
            m.porte_par = None;
            m.panique = 1.0;
            m.vel = (m.pos - chien).norm() * 3.0;
            self.evenements.push(Evenement::Libere(c));
        }
        self.evenements.push(Evenement::LoupFuit(i));
    }

    fn pas_beliers(&mut self, dt: f32) {
        let chien = self.chien.pos;
        let libres: Vec<V2> = self.moutons.iter().filter(|m| !m.perdu && !m.abrite).map(|m| m.pos).collect();
        let centre = if libres.is_empty() { None } else { Some(libres.iter().fold(V2::ZERO, |s, &p| s + p) / libres.len() as f32) };
        for i in 0..self.beliers.len() {
            let reg = self.reg;
            let b = &mut self.beliers[i];
            b.repos = (b.repos - dt).max(0.0);
            let d = b.pos.dist(chien);
            let vers_chien = (chien - b.pos).norm();
            match b.etat {
                EtatBelier::Calme => {
                    // Il reste avec le troupeau, sans le mener.
                    let voulu = match centre {
                        Some(c) if c.dist(b.pos) > 3.5 => (c - b.pos).norm() * 1.1,
                        _ => V2::ZERO,
                    };
                    let evite = Self::evitement(&self.carte, b.pos, 0.6, 1.8);
                    b.vel = b.vel.lerp(voulu + evite * 1.5, (dt * 3.0).min(1.0));
                    if d < reg.belier_vigilance_r && b.repos <= 0.0 {
                        b.etat = EtatBelier::Vigilance;
                        b.timer = 1.0;
                    }
                }
                EtatBelier::Vigilance => {
                    b.vel = b.vel.lerp(V2::ZERO, (dt * 6.0).min(1.0));
                    b.direction = vers_chien;
                    b.timer -= dt;
                    if d > reg.belier_vigilance_r + 2.0 {
                        b.etat = EtatBelier::Calme;
                    } else if b.timer <= 0.0 {
                        b.etat = EtatBelier::Avertit;
                        b.timer = reg.belier_avertissement;
                        self.evenements.push(Evenement::BelierAvertit(i));
                    }
                }
                EtatBelier::Avertit => {
                    b.vel = V2::ZERO;
                    b.direction = vers_chien;
                    b.timer -= dt;
                    if b.timer <= 0.0 {
                        // Il fonce là où le chien se trouve maintenant : on peut l'éviter en bougeant.
                        b.etat = EtatBelier::Charge;
                        b.timer = reg.belier_charge;
                        self.evenements.push(Evenement::BelierCharge(i));
                    }
                }
                EtatBelier::Charge => {
                    b.vel = b.direction * reg.belier_vitesse_charge;
                    b.timer -= dt;
                    if d < 1.0 {
                        let dir = b.direction;
                        self.chien.vel += dir * 9.0;
                        self.chien.etourdi = 0.7;
                        self.evenements.push(Evenement::ChienPercute);
                        b.etat = EtatBelier::Recupere;
                        b.timer = 1.6;
                    } else if b.timer <= 0.0 {
                        b.etat = EtatBelier::Recupere;
                        b.timer = 1.6;
                    }
                    // Les moutons sur son passage s'écartent.
                    let (bp, bd) = (b.pos, b.direction);
                    for m in self.moutons.iter_mut().filter(|m| !m.perdu && !m.abrite) {
                        let dm = m.pos.dist(bp);
                        if dm < 2.5 {
                            m.vel += (m.pos - bp).norm() * 2.0 + bd * 1.0;
                            m.panique = m.panique.max(0.8);
                        }
                    }
                }
                EtatBelier::Recupere => {
                    b.vel = b.vel.lerp(V2::ZERO, (dt * 5.0).min(1.0));
                    b.timer -= dt;
                    if b.timer <= 0.0 {
                        b.etat = EtatBelier::Calme;
                        b.repos = 3.5;
                    }
                }
            }
            let b = &mut self.beliers[i];
            let (mut pos, mut vel) = (b.pos + b.vel * dt, b.vel);
            let avant = vel.len();
            Self::collisions(&self.carte, &mut pos, &mut vel, 0.6, 0.5);
            if b.etat == EtatBelier::Charge && vel.len() < avant * 0.5 {
                // Contre un arbre ou une clôture : la charge s'arrête.
                b.etat = EtatBelier::Recupere;
                b.timer = 1.6;
            }
            b.pos = pos;
            b.vel = vel;
            if vel.len() > 0.3 || matches!(b.etat, EtatBelier::Vigilance | EtatBelier::Avertit) {
                let v = if vel.len() > 0.3 { vel } else { b.direction };
                b.cap = v.x.atan2(v.y);
            }
            b.phase += dt * (1.0 + vel.len() * 1.6);
        }
    }

    fn loup_chasse(&mut self, i: usize, dt: f32) {
        // Il vise le mouton libre le plus proche (les moutons de l'enclos sont à l'abri).
        let lp = self.loups[i].pos;
        let valide = |m: &Mouton| !m.perdu && !m.abrite && m.porte_par.is_none();
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
        let chien = self.chien.pos;
        let mut vers = self.moutons[c].pos + self.moutons[c].vel * 0.25;
        let l = &mut self.loups[i];
        if l.genre == GenreLoup::Prudent && l.pos.dist(vers) > 4.0 {
            // Il tourne autour : il vise le côté du mouton opposé au chien.
            vers += (self.moutons[c].pos - chien).norm() * 3.0;
        }
        let dist = l.pos.dist(vers);
        let dir = ((vers - l.pos).norm() + Self::evitement(&self.carte, l.pos, 0.5, 1.8) * 1.4).norm();
        let mut vitesse = if dist < 3.5 { self.reg.vitesse_loup * 1.15 } else { self.reg.vitesse_loup };
        if l.genre == GenreLoup::Audacieux && l.pos.dist(chien) > 10.0 {
            vitesse *= 1.25;
        }
        l.vel = l.vel.lerp(dir * vitesse, (dt * 5.0).min(1.0));
        if self.moutons[c].pos.dist(l.pos) < 1.0 {
            l.morsure += dt;
            if l.morsure > 0.7 {
                // Il l'attrape et l'emporte vers le bord : on peut le lui reprendre.
                self.moutons[c].porte_par = Some(i);
                self.moutons[c].vel = V2::ZERO;
                self.evenements.push(Evenement::Capture(c));
                l.porte = Some(c);
                l.etat = EtatLoup::Repart;
                l.cible = None;
                l.morsure = 0.0;
            }
        } else {
            l.morsure = (l.morsure - dt).max(0.0);
        }
    }
}

