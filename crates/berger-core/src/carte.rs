//! La carte d'un niveau : champ, arbres, rochers, mares, clôtures et enclos,
//! générée de façon déterministe à partir d'une graine.
use crate::v2::{Hasard, V2, sur_segment, v2};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Genre {
    Arbre,
    Pin,
    Rocher,
    Mare,
}

#[derive(Clone, Copy, Debug)]
pub struct Obstacle {
    pub pos: V2,
    pub rayon: f32,
    pub genre: Genre,
    /// Pour varier l'aspect (teinte, rotation) sans changer le jeu.
    pub variante: u8,
}

#[derive(Clone, Copy, Debug)]
pub struct Mur {
    pub a: V2,
    pub b: V2,
}

/// Le côté de l'enclos où se trouve la porte.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cote {
    Nord,
    Est,
    Sud,
    Ouest,
}

impl Cote {
    /// Direction qui sort de l'enclos par ce côté.
    pub fn sortie(self) -> V2 {
        match self {
            Cote::Nord => v2(0.0, -1.0),
            Cote::Est => v2(1.0, 0.0),
            Cote::Sud => v2(0.0, 1.0),
            Cote::Ouest => v2(-1.0, 0.0),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Enclos {
    pub centre: V2,
    pub demi: V2,
    pub cote: Cote,
    /// Largeur de la porte.
    pub porte: f32,
}

impl Enclos {
    /// Le point est-il dans l'enclos (à `marge` du bord) ?
    pub fn dedans(&self, p: V2, marge: f32) -> bool {
        (p.x - self.centre.x).abs() < self.demi.x - marge && (p.y - self.centre.y).abs() < self.demi.y - marge
    }
    /// Le milieu de la porte.
    pub fn entree(&self) -> V2 {
        let n = self.cote.sortie();
        self.centre + v2(n.x * self.demi.x, n.y * self.demi.y)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct DepartLoup {
    pub apparition: f32,
    pub bord: V2,
}

/// Les réglages d'un niveau de la campagne.
#[derive(Clone, Copy, Debug)]
pub struct Niveau {
    /// Demi-largeur et demi-hauteur du champ.
    pub demi: (f32, f32),
    pub moutons: u32,
    pub arbres: u32,
    pub rochers: u32,
    pub mares: u32,
    pub clotures: u32,
    pub loups: u32,
    pub loup_debut: f32,
    pub loup_ecart: f32,
    /// Moutons qu'on peut perdre sans échouer.
    pub pertes_max: u32,
    /// Temps de référence pour la deuxième étoile (secondes).
    pub temps_ref: f32,
    pub entonnoir: bool,
}

#[allow(clippy::too_many_arguments)]
const fn n(
    demi: (f32, f32),
    moutons: u32,
    arbres: u32,
    rochers: u32,
    mares: u32,
    clotures: u32,
    loups: u32,
    pertes_max: u32,
    temps_ref: f32,
) -> Niveau {
    Niveau {
        demi,
        moutons,
        arbres,
        rochers,
        mares,
        clotures,
        loups,
        loup_debut: 18.0,
        loup_ecart: 26.0,
        pertes_max,
        temps_ref,
        entonnoir: moutons <= 12,
    }
}

/// La campagne : douze niveaux de plus en plus difficiles.
pub const NIVEAUX: [Niveau; 12] = [
    n((18.0, 12.0), 5, 3, 0, 0, 0, 0, 0, 60.0),
    n((20.5, 13.0), 7, 5, 2, 0, 0, 0, 0, 70.0),
    n((23.0, 14.5), 8, 6, 3, 1, 0, 0, 0, 80.0),
    n((24.0, 15.5), 10, 6, 4, 1, 2, 0, 0, 95.0),
    n((25.0, 15.5), 10, 7, 4, 2, 2, 1, 1, 100.0),
    n((26.5, 17.0), 12, 8, 5, 2, 3, 1, 1, 115.0),
    n((29.0, 17.0), 14, 9, 5, 2, 3, 1, 1, 130.0),
    n((29.0, 18.0), 14, 10, 6, 3, 4, 2, 2, 140.0),
    n((31.0, 19.0), 16, 10, 6, 3, 5, 2, 2, 155.0),
    n((31.0, 19.0), 18, 12, 7, 3, 5, 2, 2, 170.0),
    n((33.5, 20.5), 20, 12, 8, 4, 6, 3, 3, 190.0),
    n((36.0, 21.5), 24, 14, 9, 4, 6, 3, 3, 215.0),
];

pub fn niveau(i: usize) -> Niveau {
    NIVEAUX[i.min(NIVEAUX.len() - 1)]
}

/// Le défi du jour : un niveau de difficulté moyenne, tiré de la date,
/// le même pour tout le monde. Renvoie le niveau et sa graine.
pub fn niveau_du_jour(jours: i64) -> (Niveau, u64) {
    let graine = (jours.max(0) as u64).wrapping_mul(2_654_435_761) ^ 0x5EED;
    let mut h = Hasard::new(graine);
    let mut nv = NIVEAUX[3 + h.sous(7) as usize];
    nv.loups = nv.loups.min(2);
    (nv, graine)
}

#[derive(Clone, Debug)]
pub struct Carte {
    pub demi: V2,
    pub obstacles: Vec<Obstacle>,
    pub enclos: Enclos,
    /// Les clôtures de l'enclos (avec l'entonnoir de la porte).
    pub murs_enclos: Vec<Mur>,
    /// Les clôtures libres dans le champ.
    pub clotures: Vec<Mur>,
    pub depart_chien: V2,
    pub depart_troupeau: V2,
    pub moutons: Vec<V2>,
    pub loups: Vec<DepartLoup>,
    pub graine: u64,
}

impl Carte {
    pub fn murs(&self) -> impl Iterator<Item = &Mur> {
        self.murs_enclos.iter().chain(self.clotures.iter())
    }

    /// Un cercle de rayon `r` en `p` touche-t-il un obstacle ou une clôture ?
    pub fn bloque(&self, p: V2, r: f32) -> bool {
        self.obstacles.iter().any(|o| o.pos.dist(p) < o.rayon + r)
            || self.murs().any(|m| sur_segment(p, m.a, m.b).dist(p) < r)
            || p.x.abs() > self.demi.x - r
            || p.y.abs() > self.demi.y - r
    }

    /// Le point devant la porte, d'où l'on voit l'enclos.
    pub fn devant_porte(&self, distance: f32) -> V2 {
        self.enclos.entree() + self.enclos.cote.sortie() * distance
    }

    /// Y a-t-il un chemin (pour un mouton) du troupeau jusqu'à la porte ?
    pub fn accessible(&self) -> bool {
        let pas = 0.6;
        let w = ((self.demi.x * 2.0) / pas).ceil() as i32 + 1;
        let h = ((self.demi.y * 2.0) / pas).ceil() as i32 + 1;
        let cell = |p: V2| -> (i32, i32) { (((p.x + self.demi.x) / pas) as i32, ((p.y + self.demi.y) / pas) as i32) };
        let centre = |c: (i32, i32)| v2(c.0 as f32 * pas - self.demi.x, c.1 as f32 * pas - self.demi.y);
        let mut libre = vec![false; (w * h) as usize];
        for cy in 0..h {
            for cx in 0..w {
                libre[(cy * w + cx) as usize] = !self.bloque(centre((cx, cy)), 0.55);
            }
        }
        let depart = cell(self.depart_troupeau);
        let but = cell(self.enclos.entree());
        let mut vu = vec![false; libre.len()];
        let mut pile = vec![depart];
        while let Some((x, y)) = pile.pop() {
            if x < 0 || y < 0 || x >= w || y >= h {
                continue;
            }
            let k = (y * w + x) as usize;
            if vu[k] || !libre[k] {
                continue;
            }
            vu[k] = true;
            if (x - but.0).abs() <= 1 && (y - but.1).abs() <= 1 {
                return true;
            }
            pile.extend([(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]);
        }
        false
    }
}

/// Fabrique la carte d'un niveau. Même niveau + même graine = même carte.
pub fn generer(nv: &Niveau, graine: u64) -> Carte {
    for essai in 0..40u64 {
        let c = essayer(nv, graine.wrapping_add(essai * 7919), essai > 25);
        if c.accessible() {
            return Carte { graine, ..c };
        }
    }
    // Dernier recours : un champ dégagé.
    Carte { graine, ..essayer(&Niveau { arbres: 0, rochers: 0, mares: 0, clotures: 0, ..*nv }, graine, true) }
}

fn essayer(nv: &Niveau, graine: u64, clairsemee: bool) -> Carte {
    let mut h = Hasard::new(graine);
    let demi = v2(nv.demi.0, nv.demi.1);
    let m = nv.moutons as f32;

    // L'enclos : contre un des bords, porte vers l'intérieur du champ.
    let eh = v2(2.4 + m * 0.07, 2.0 + m * 0.055);
    let bord = h.sous(4);
    let (centre, cote) = match bord {
        0 => (v2(h.entre(-0.5, 0.5) * (demi.x - eh.x - 5.0), -demi.y + eh.y + 0.45), Cote::Sud),
        1 => (v2(demi.x - eh.y - 0.45, h.entre(-0.5, 0.5) * (demi.y - eh.x - 3.0)), Cote::Ouest),
        2 => (v2(h.entre(-0.5, 0.5) * (demi.x - eh.x - 5.0), demi.y - eh.y - 0.45), Cote::Nord),
        _ => (v2(-demi.x + eh.y + 0.45, h.entre(-0.5, 0.5) * (demi.y - eh.x - 3.0)), Cote::Est),
    };
    // Sur les bords est et ouest, l'enclos est tourné : largeur et hauteur échangées.
    let eh = if bord % 2 == 1 { v2(eh.y, eh.x) } else { eh };
    let enclos = Enclos { centre, demi: eh, cote, porte: 3.2 };
    let murs_enclos = murs_de_enclos(&enclos, nv.entonnoir);

    // Les zones à garder libres.
    let vers_enclos = -cote.sortie();
    let mut libres: Vec<(V2, f32)> = Vec::new();
    for k in 0..4 {
        libres.push((enclos.entree() + cote.sortie() * (2.5 + k as f32 * 2.8), 2.6));
    }
    let etendue = if bord.is_multiple_of(2) { demi.y } else { demi.x };
    let tangente = vers_enclos.perp();
    let largeur = if bord.is_multiple_of(2) { demi.x } else { demi.y };
    let troupeau = v2(0.0, 0.0) - vers_enclos * (etendue * 0.35) + tangente * h.entre(-0.45, 0.45) * largeur;
    let rayon_troupeau = 2.2 + m.sqrt() * 0.7;
    libres.push((troupeau, rayon_troupeau + 1.5));
    // Le chien démarre sur le côté, assez loin pour ne pas effrayer le troupeau.
    let ecart_chien = rayon_troupeau + 7.0;
    let lim = demi - v2(2.0, 2.0);
    let cote_chien = if (troupeau + tangente * ecart_chien).clamp_in(lim).dist(troupeau) > (troupeau - tangente * ecart_chien).clamp_in(lim).dist(troupeau) { 1.0 } else { -1.0 };
    let chien = (troupeau + tangente * ecart_chien * cote_chien).clamp_in(lim);
    libres.push((chien, 2.0));
    // L'enclos lui-même (avec son entonnoir).
    libres.push((centre, eh.x.max(eh.y) + 2.5));

    let mut occupes: Vec<(V2, f32)> = Vec::new();
    let place = |p: V2, r: f32, ecart: f32, occupes: &[(V2, f32)], libres: &[(V2, f32)]| -> bool {
        p.x.abs() < demi.x - r - 1.0
            && p.y.abs() < demi.y - r - 1.0
            && occupes.iter().all(|&(q, s)| q.dist(p) >= r + s + ecart)
            && libres.iter().all(|&(q, s)| q.dist(p) >= r + s)
    };

    let mut obstacles = Vec::new();
    let reduit = |k: u32| if clairsemee { k / 2 } else { k };
    let genres = [
        (Genre::Mare, reduit(nv.mares), 1.6, 2.6),
        (Genre::Rocher, reduit(nv.rochers), 0.7, 1.3),
        (Genre::Arbre, reduit(nv.arbres), 0.8, 1.1),
    ];
    for (genre, nombre, rmin, rmax) in genres {
        for _ in 0..nombre {
            for _ in 0..80 {
                let r = h.entre(rmin, rmax);
                let p = v2(h.entre(-1.0, 1.0) * demi.x, h.entre(-1.0, 1.0) * demi.y);
                if place(p, r, 1.9, &occupes, &libres) {
                    let genre = if genre == Genre::Arbre && h.f() < 0.4 { Genre::Pin } else { genre };
                    obstacles.push(Obstacle { pos: p, rayon: r, genre, variante: h.sous(4) as u8 });
                    occupes.push((p, r));
                    break;
                }
            }
        }
    }

    let mut clotures = Vec::new();
    for _ in 0..reduit(nv.clotures) {
        for _ in 0..60 {
            let long = h.entre(3.0, 6.0);
            let a = v2(h.entre(-1.0, 1.0) * (demi.x - 4.0), h.entre(-1.0, 1.0) * (demi.y - 4.0));
            let sens = if h.f() < 0.5 { v2(1.0, 0.0) } else { v2(0.0, 1.0) };
            let b = a + sens * long;
            let points: Vec<V2> = (0..=(long / 0.8) as u32).map(|k| a.lerp(b, k as f32 * 0.8 / long)).collect();
            if points.iter().all(|&p| place(p, 0.5, 2.0, &occupes, &libres)) {
                occupes.extend(points.iter().map(|&p| (p, 0.5)));
                clotures.push(Mur { a, b });
                break;
            }
        }
    }

    // Le troupeau : des moutons groupés, hors des obstacles.
    let mut moutons = Vec::new();
    let mut essais = 0;
    while moutons.len() < nv.moutons as usize && essais < 2000 {
        essais += 1;
        let a = h.entre(0.0, std::f32::consts::TAU);
        let d = rayon_troupeau * h.f().sqrt();
        let p = troupeau + V2::from_angle(a) * d;
        let loin_des_autres = moutons.iter().all(|q: &V2| q.dist(p) > 0.9);
        let libre = !obstacles.iter().any(|o| o.pos.dist(p) < o.rayon + 0.8)
            && !clotures.iter().any(|c| sur_segment(p, c.a, c.b).dist(p) < 0.8)
            && p.x.abs() < demi.x - 1.0
            && p.y.abs() < demi.y - 1.0;
        if loin_des_autres && libre {
            moutons.push(p);
        }
    }

    // Les loups arrivent par un bord éloigné de l'enclos.
    let mut loups = Vec::new();
    for k in 0..nv.loups {
        let bord = loop {
            let p = match h.sous(4) {
                0 => v2(h.entre(-1.0, 1.0) * demi.x, -demi.y - 1.0),
                1 => v2(demi.x + 1.0, h.entre(-1.0, 1.0) * demi.y),
                2 => v2(h.entre(-1.0, 1.0) * demi.x, demi.y + 1.0),
                _ => v2(-demi.x - 1.0, h.entre(-1.0, 1.0) * demi.y),
            };
            if p.dist(centre) > 14.0 {
                break p;
            }
        };
        loups.push(DepartLoup { apparition: nv.loup_debut + k as f32 * nv.loup_ecart, bord });
    }

    Carte { demi, obstacles, enclos, murs_enclos, clotures, depart_chien: chien, depart_troupeau: troupeau, moutons, loups, graine }
}

/// Le niveau vitrine (niveau 4 de la campagne), dessiné à la main : l'enclos à l'ouest,
/// une mare devant sa porte, puis une haie avec un passage étroit entre deux rochers
/// que tout le troupeau doit franchir. Pas de loup : on apprend à conduire.
pub fn vitrine() -> (Niveau, Carte) {
    let nv = Niveau { temps_ref: 110.0, ..NIVEAUX[3] };
    let demi = v2(nv.demi.0, nv.demi.1);
    let eh = v2(2.55, 3.1);
    let enclos = Enclos { centre: v2(-demi.x + eh.x + 0.05, 0.0), demi: eh, cote: Cote::Est, porte: 3.2 };
    let murs_enclos = murs_de_enclos(&enclos, true);
    let ob = |x: f32, y: f32, rayon: f32, genre: Genre, variante: u8| Obstacle { pos: v2(x, y), rayon, genre, variante };
    let obstacles = vec![
        // Le passage : deux gros rochers, une haie de chaque côté.
        ob(-7.0, -3.4, 1.3, Genre::Rocher, 0),
        ob(-7.0, 3.4, 1.2, Genre::Rocher, 1),
        // La mare à côté de la porte.
        ob(-13.0, -8.0, 2.3, Genre::Mare, 0),
        ob(-14.0, 9.0, 1.2, Genre::Rocher, 2),
        // Quelques arbres pour contourner.
        ob(-1.0, -9.5, 1.0, Genre::Arbre, 1),
        ob(3.5, 8.5, 1.0, Genre::Pin, 2),
        ob(12.0, -7.5, 1.0, Genre::Arbre, 3),
        ob(17.0, 2.0, 0.9, Genre::Pin, 0),
        ob(5.0, -4.0, 0.8, Genre::Rocher, 1),
        ob(0.0, 3.0, 0.7, Genre::Rocher, 0),
    ];
    let clotures = vec![Mur { a: v2(-7.0, -4.7), b: v2(-7.0, -10.5) }, Mur { a: v2(-7.0, 4.6), b: v2(-7.0, 10.5) }];
    let troupeau = v2(10.0, 0.0);
    let mut h = Hasard::new(4242);
    let mut moutons: Vec<V2> = Vec::new();
    while moutons.len() < nv.moutons as usize {
        let p = troupeau + V2::from_angle(h.entre(0.0, std::f32::consts::TAU)) * (2.6 * h.f().sqrt());
        let libre = !obstacles.iter().any(|o| o.pos.dist(p) < o.rayon + 0.8);
        if libre && moutons.iter().all(|q| q.dist(p) > 0.9) {
            moutons.push(p);
        }
    }
    let carte = Carte { demi, obstacles, enclos, murs_enclos, clotures, depart_chien: v2(17.0, 6.5), depart_troupeau: troupeau, moutons, loups: Vec::new(), graine: 4242 };
    (nv, carte)
}

/// Les quatre côtés de l'enclos, la porte ouverte, et l'entonnoir qui guide
/// les moutons vers elle.
fn murs_de_enclos(e: &Enclos, entonnoir: bool) -> Vec<Mur> {
    let c = e.centre;
    let h = e.demi;
    let coins = [v2(c.x - h.x, c.y - h.y), v2(c.x + h.x, c.y - h.y), v2(c.x + h.x, c.y + h.y), v2(c.x - h.x, c.y + h.y)];
    let cotes = [Cote::Nord, Cote::Est, Cote::Sud, Cote::Ouest];
    let mut murs = Vec::new();
    for i in 0..4 {
        let (a, b) = (coins[i], coins[(i + 1) % 4]);
        if cotes[i] != e.cote {
            murs.push(Mur { a, b });
            continue;
        }
        let milieu = a.lerp(b, 0.5);
        let u = (b - a).norm();
        let g = e.porte / 2.0;
        let (p, q) = (milieu - u * g, milieu + u * g);
        murs.push(Mur { a, b: p });
        murs.push(Mur { a: q, b });
        if entonnoir {
            let n = e.cote.sortie();
            let (s, co) = (0.52f32.sin(), 0.52f32.cos());
            murs.push(Mur { a: p, b: p + n * (co * 3.6) - u * (s * 3.6) });
            murs.push(Mur { a: q, b: q + n * (co * 3.6) + u * (s * 3.6) });
        }
    }
    murs
}
