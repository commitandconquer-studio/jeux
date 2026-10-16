//! Les vaisseaux du hangar, leurs livrées et la progression du joueur
//! (points cumulés, vaisseaux et livrées gagnés, record, défi du jour).
//!
//! Vaisseaux originaux du studio : ils évoquent les grandes familles de la
//! science-fiction sans en copier ni les noms ni les silhouettes.
use bevy::prelude::*;
use game_common::calendar::Date;
use game_common::langue::Texte;
use game_common::{flat_material, tx};

const SAVE: &str = "parsecs";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Vaisseau {
    Faucheuse,
    Bourdon,
    Aiguillon,
    CargoLibre,
}

/// Façon de tirer de chaque vaisseau.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Tir {
    /// Deux tirs parallèles.
    Double,
    /// Un tir qui s'oriente doucement vers le rocher le plus proche.
    Aimante,
    /// Un tir unique, très rapide et fréquent.
    Rapide,
    /// Trois tirs en éventail.
    Eventail,
}

/// Caractéristiques d'un vaisseau en jeu.
pub struct Fiche {
    pub nom: &'static str,
    pub description: Texte,
    /// Points cumulés (toutes parties confondues) pour le gagner.
    pub palier: u64,
    /// Vitesse maximale, poussée, rotation (radians/s).
    pub vitesse: f32,
    pub poussee: f32,
    pub rotation: f32,
    /// Délai entre deux tirs (s).
    pub cadence: f32,
    pub tir: Tir,
    /// Points de bouclier.
    pub bouclier: f32,
    /// Rayon de collision.
    pub rayon: f32,
}

impl Vaisseau {
    pub const TOUS: [Vaisseau; 4] = [Vaisseau::Faucheuse, Vaisseau::Bourdon, Vaisseau::Aiguillon, Vaisseau::CargoLibre];

    pub fn fiche(self) -> Fiche {
        match self {
            Vaisseau::Faucheuse => Fiche {
                nom: "Faucheuse",
                description: tx!(
                    "Chasseur équilibré, ailes en croix. Tir double.",
                    "Balanced fighter with crossed wings. Twin shot.",
                    "Ausgewogener Jäger mit Kreuzflügeln. Doppelschuss.",
                    "Caccia equilibrato ad ali incrociate. Doppio colpo.",
                    "Caza equilibrado de alas cruzadas. Disparo doble."
                ),
                palier: 0,
                vitesse: 13.0,
                poussee: 22.0,
                rotation: 4.2,
                cadence: 0.22,
                tir: Tir::Double,
                bouclier: 100.0,
                rayon: 0.8,
            },
            Vaisseau::Bourdon => Fiche {
                nom: "Bourdon",
                description: tx!(
                    "Petit drone agile : il se faufile partout. Tir aimanté.",
                    "Small nimble drone: it slips through anything. Homing shot.",
                    "Kleine wendige Drohne: schlüpft überall durch. Zielsuchender Schuss.",
                    "Piccolo drone agile: si infila ovunque. Colpo calamitato.",
                    "Pequeño dron ágil: se cuela por todas partes. Disparo guiado."
                ),
                palier: 5_000,
                vitesse: 15.0,
                poussee: 30.0,
                rotation: 5.4,
                cadence: 0.28,
                tir: Tir::Aimante,
                bouclier: 70.0,
                rayon: 0.55,
            },
            Vaisseau::Aiguillon => Fiche {
                nom: "Aiguillon",
                description: tx!(
                    "Intercepteur très rapide mais fragile. Tir rapide.",
                    "Very fast but fragile interceptor. Rapid fire.",
                    "Sehr schneller, aber zerbrechlicher Abfangjäger. Schnellfeuer.",
                    "Intercettore velocissimo ma fragile. Fuoco rapido.",
                    "Interceptor muy rápido pero frágil. Fuego rápido."
                ),
                palier: 20_000,
                vitesse: 19.0,
                poussee: 34.0,
                rotation: 5.0,
                cadence: 0.11,
                tir: Tir::Rapide,
                bouclier: 60.0,
                rayon: 0.7,
            },
            Vaisseau::CargoLibre => Fiche {
                nom: "Cargo Libre",
                description: tx!(
                    "Vieux cargo rapiécé : lent, mais increvable. Tir en éventail.",
                    "Patched-up old freighter: slow, but tough as nails. Spread shot.",
                    "Alter geflickter Frachter: langsam, aber unverwüstlich. Fächerschuss.",
                    "Vecchio cargo rattoppato: lento, ma indistruttibile. Colpo a ventaglio.",
                    "Viejo carguero remendado: lento, pero indestructible. Disparo en abanico."
                ),
                palier: 50_000,
                vitesse: 10.0,
                poussee: 15.0,
                rotation: 3.2,
                cadence: 0.34,
                tir: Tir::Eventail,
                bouclier: 180.0,
                rayon: 1.05,
            },
        }
    }

    pub fn index(self) -> usize {
        Vaisseau::TOUS.iter().position(|v| *v == self).unwrap_or(0)
    }

    /// Les trois livrées du vaisseau : (nom, coque, ailes et détails, cockpit).
    pub fn livrees(self) -> [(Texte, Color, Color, Color); 3] {
        let c = |r: f32, g: f32, b: f32| Color::srgb(r, g, b);
        match self {
            Vaisseau::Faucheuse => [
                (tx!("Escadrille", "Squadron", "Staffel", "Squadriglia", "Escuadrilla"), c(0.88, 0.89, 0.92), c(0.85, 0.30, 0.25), c(0.20, 0.55, 0.90)),
                (tx!("Nuit polaire", "Polar night", "Polarnacht", "Notte polare", "Noche polar"), c(0.20, 0.24, 0.32), c(0.55, 0.85, 1.00), c(0.85, 0.95, 1.00)),
                (tx!("Or de Nova", "Nova gold", "Nova-Gold", "Oro di Nova", "Oro de Nova"), c(0.95, 0.80, 0.30), c(0.30, 0.20, 0.12), c(1.00, 0.45, 0.20)),
            ],
            Vaisseau::Bourdon => [
                (tx!("Atelier", "Workshop", "Werkstatt", "Officina", "Taller"), c(0.95, 0.75, 0.20), c(0.20, 0.20, 0.24), c(0.40, 0.90, 1.00)),
                (tx!("Menthe", "Mint", "Minze", "Menta", "Menta"), c(0.55, 0.88, 0.70), c(0.95, 0.97, 0.95), c(0.15, 0.45, 0.35)),
                (tx!("Braise", "Ember", "Glut", "Brace", "Brasa"), c(0.85, 0.25, 0.20), c(0.25, 0.10, 0.10), c(1.00, 0.80, 0.30)),
            ],
            Vaisseau::Aiguillon => [
                (tx!("Acier", "Steel", "Stahl", "Acciaio", "Acero"), c(0.60, 0.64, 0.70), c(0.18, 0.20, 0.25), c(0.90, 0.25, 0.25)),
                (tx!("Givre", "Frost", "Raureif", "Brina", "Escarcha"), c(0.88, 0.95, 1.00), c(0.35, 0.60, 0.90), c(0.20, 0.30, 0.55)),
                (tx!("Nébuleuse", "Nebula", "Nebel", "Nebulosa", "Nebulosa"), c(0.55, 0.30, 0.75), c(0.95, 0.45, 0.80), c(0.60, 1.00, 0.90)),
            ],
            Vaisseau::CargoLibre => [
                (tx!("Rouille", "Rust", "Rost", "Ruggine", "Óxido"), c(0.62, 0.55, 0.45), c(0.75, 0.40, 0.20), c(0.95, 0.85, 0.50)),
                (tx!("Contrebande", "Smuggler", "Schmuggler", "Contrabbando", "Contrabando"), c(0.30, 0.32, 0.30), c(0.85, 0.70, 0.25), c(0.40, 1.00, 0.50)),
                (tx!("Colonie", "Colony", "Kolonie", "Colonia", "Colonia"), c(0.92, 0.92, 0.88), c(0.20, 0.50, 0.80), c(1.00, 0.55, 0.20)),
            ],
        }
    }
}

/// Points à faire avec un vaisseau pour gagner ses livrées 2 et 3.
pub const PALIERS_LIVREE: [u64; 3] = [0, 2_000, 10_000];

/// La progression du joueur, enregistrée.
#[derive(Resource, Clone)]
pub struct Progression {
    /// Points cumulés sur toutes les parties.
    pub total: u64,
    /// Points cumulés avec chaque vaisseau (pour ses livrées).
    pub par_vaisseau: [u64; 4],
    pub choisi: Vaisseau,
    /// Livrée choisie pour chaque vaisseau.
    pub livree: [usize; 4],
    pub record: u64,
    /// Plus grand nombre de parsecs franchis.
    pub meilleur_parsec: u32,
    pub defi_fait: Option<Date>,
    pub serie: u32,
}

impl Progression {
    pub fn charger() -> Progression {
        let n = |cle: &str| game_common::save::load(SAVE, cle).and_then(|v| v.parse::<u64>().ok()).unwrap_or(0);
        let liste = |cle: &str| -> [u64; 4] {
            let mut t = [0; 4];
            if let Some(v) = game_common::save::load(SAVE, cle) {
                for (i, x) in v.split_whitespace().take(4).enumerate() {
                    t[i] = x.parse().unwrap_or(0);
                }
            }
            t
        };
        let livree = liste("livrees").map(|x| (x as usize).min(2));
        let choisi = Vaisseau::TOUS[(n("choisi") as usize).min(3)];
        let mut p = Progression {
            total: n("total"),
            par_vaisseau: liste("par-vaisseau"),
            choisi,
            livree,
            record: n("record"),
            meilleur_parsec: n("parsec") as u32,
            defi_fait: game_common::save::load(SAVE, "defi").and_then(|d| Date::parse(&d)),
            serie: n("serie") as u32,
        };
        // Sécurité : jamais un vaisseau choisi qui ne serait pas gagné.
        if !p.gagne(p.choisi) {
            p.choisi = Vaisseau::Faucheuse;
        }
        p
    }

    pub fn enregistrer(&self) {
        let s = |cle: &str, v: String| game_common::save::store(SAVE, cle, &v);
        let liste = |t: [u64; 4]| t.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" ");
        s("total", self.total.to_string());
        s("par-vaisseau", liste(self.par_vaisseau));
        s("choisi", self.choisi.index().to_string());
        s("livrees", liste(self.livree.map(|x| x as u64)));
        s("record", self.record.to_string());
        s("parsec", self.meilleur_parsec.to_string());
        if let Some(d) = self.defi_fait {
            s("defi", format!("{:04}-{:02}-{:02}", d.year, d.month, d.day));
        }
        s("serie", self.serie.to_string());
    }

    pub fn gagne(&self, v: Vaisseau) -> bool {
        self.total >= v.fiche().palier
    }

    pub fn livree_gagnee(&self, v: Vaisseau, i: usize) -> bool {
        self.gagne(v) && self.par_vaisseau[v.index()] >= PALIERS_LIVREE[i]
    }

    /// Le prochain vaisseau à gagner, s'il en reste un.
    pub fn prochain(&self) -> Option<Vaisseau> {
        Vaisseau::TOUS.into_iter().find(|v| !self.gagne(*v))
    }

    /// Ajoute les points d'une partie ; renvoie les vaisseaux et livrées gagnés.
    pub fn ajouter(&mut self, points: u64) -> Vec<Gain> {
        let avant = self.clone();
        self.total += points;
        self.par_vaisseau[self.choisi.index()] += points;
        let mut gains = Vec::new();
        for v in Vaisseau::TOUS {
            if !avant.gagne(v) && self.gagne(v) {
                gains.push(Gain::Vaisseau(v));
            }
            for i in 1..3 {
                if avant.gagne(v) && !avant.livree_gagnee(v, i) && self.livree_gagnee(v, i) {
                    gains.push(Gain::Livree(v, i));
                }
            }
        }
        gains
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Gain {
    Vaisseau(Vaisseau),
    Livree(Vaisseau, usize),
}

// ---- Les modèles 3D ---------------------------------------------------------

/// Construit le vaisseau (pointe vers +Y, dans le plan XY), enfant de `parent`.
/// `ombre` : silhouette sombre (vaisseau pas encore gagné).
pub fn construire(
    commands: &mut Commands,
    parent: Entity,
    v: Vaisseau,
    livree: usize,
    ombre: bool,
    meshes: &mut Assets<Mesh>,
    mats: &mut Assets<StandardMaterial>,
) {
    let (_, coque, detail, vitre) = v.livrees()[livree.min(2)];
    let noir = Color::srgb(0.06, 0.07, 0.10);
    let (coque, detail, vitre) = if ombre { (noir, noir, noir) } else { (coque, detail, vitre) };
    let m_coque = mats.add(flat_material(coque));
    let m_detail = mats.add(flat_material(detail));
    let m_vitre = mats.add(StandardMaterial { emissive: if ombre { LinearRgba::BLACK } else { vitre.to_linear() * 0.6 }, ..flat_material(vitre) });
    let m_reacteur = mats.add(StandardMaterial { base_color: Color::srgb(1.0, 0.6, 0.2), emissive: if ombre { LinearRgba::BLACK } else { LinearRgba::rgb(4.0, 1.6, 0.4) }, unlit: true, ..default() });
    let mut piece = |commands: &mut Commands, mesh: Mesh, mat: &Handle<StandardMaterial>, pos: Vec3, rot: Quat, echelle: Vec3| {
        commands.spawn((
            Mesh3d(meshes.add(mesh)),
            MeshMaterial3d(mat.clone()),
            Transform { translation: pos, rotation: rot, scale: echelle },
            ChildOf(parent),
        ));
    };
    let cone = |r: f32, h: f32, n: u32| Cone { radius: r, height: h }.mesh().resolution(n).build();
    let boite = |x: f32, y: f32, z: f32| Mesh::from(Cuboid::new(x, y, z));
    let id = Quat::IDENTITY;
    let un = Vec3::ONE;
    match v {
        Vaisseau::Faucheuse => {
            // Fuselage effilé, quatre ailes en croix légèrement écartées, deux canons.
            piece(commands, cone(0.32, 2.2, 6), &m_coque, Vec3::new(0.0, 0.2, 0.0), id, un);
            piece(commands, boite(0.5, 0.7, 0.4), &m_coque, Vec3::new(0.0, -0.75, 0.0), id, un);
            for (sx, sz) in [(1.0, 1.0), (-1.0, 1.0), (1.0, -1.0), (-1.0, -1.0)] {
                let rot = Quat::from_rotation_y(sx * sz * 0.22);
                piece(commands, boite(1.3, 0.55, 0.06), &m_detail, Vec3::new(sx * 0.85, -0.65, sz * 0.12), rot, un);
                piece(commands, boite(0.08, 0.9, 0.08), &m_coque, Vec3::new(sx * 1.45, -0.45, sz * 0.15), id, un);
            }
            piece(commands, Sphere::new(0.2).mesh().ico(0).unwrap(), &m_vitre, Vec3::new(0.0, 0.0, 0.25), id, Vec3::new(1.0, 1.4, 0.7));
            piece(commands, cone(0.18, 0.4, 5), &m_reacteur, Vec3::new(0.0, -1.2, 0.0), Quat::from_rotation_z(std::f32::consts::PI), un);
        }
        Vaisseau::Bourdon => {
            // Corps rond, deux bras porteurs de moteurs, une antenne.
            piece(commands, Sphere::new(0.45).mesh().ico(1).unwrap(), &m_coque, Vec3::ZERO, id, Vec3::new(1.0, 1.1, 0.8));
            piece(commands, Sphere::new(0.22).mesh().ico(0).unwrap(), &m_vitre, Vec3::new(0.0, 0.28, 0.28), id, un);
            for sx in [-1.0, 1.0] {
                piece(commands, boite(0.6, 0.12, 0.12), &m_detail, Vec3::new(sx * 0.6, -0.1, 0.0), id, un);
                piece(commands, Cylinder::new(0.18, 0.35).mesh().resolution(6).build(), &m_detail, Vec3::new(sx * 0.95, -0.15, 0.0), id, un);
                piece(commands, cone(0.12, 0.3, 5), &m_reacteur, Vec3::new(sx * 0.95, -0.45, 0.0), Quat::from_rotation_z(std::f32::consts::PI), un);
            }
            piece(commands, boite(0.04, 0.4, 0.04), &m_detail, Vec3::new(0.15, 0.6, 0.0), Quat::from_rotation_z(-0.3), un);
        }
        Vaisseau::Aiguillon => {
            // Cabine ronde au centre, deux grandes ailes plates et anguleuses.
            piece(commands, Sphere::new(0.38).mesh().ico(1).unwrap(), &m_coque, Vec3::ZERO, id, un);
            piece(commands, Sphere::new(0.24).mesh().ico(0).unwrap(), &m_vitre, Vec3::new(0.0, 0.12, 0.25), id, un);
            for sx in [-1.0, 1.0] {
                piece(commands, boite(0.7, 0.14, 0.14), &m_coque, Vec3::new(sx * 0.5, 0.0, 0.0), id, un);
                piece(commands, cone(0.75, 1.9, 3), &m_detail, Vec3::new(sx * 0.95, -0.1, 0.0), Quat::from_rotation_y(std::f32::consts::FRAC_PI_2), Vec3::new(1.0, 1.0, 0.12));
            }
            piece(commands, cone(0.14, 0.3, 5), &m_reacteur, Vec3::new(0.0, -0.45, 0.0), Quat::from_rotation_z(std::f32::consts::PI), un);
        }
        Vaisseau::CargoLibre => {
            // Coque large et plate, cabine décalée sur le côté, deux mâchoires à l'avant.
            piece(commands, Cylinder::new(1.0, 0.45).mesh().resolution(8).build(), &m_coque, Vec3::new(0.0, -0.2, 0.0), Quat::from_rotation_x(std::f32::consts::FRAC_PI_2), un);
            for sx in [-1.0, 1.0] {
                piece(commands, boite(0.45, 1.1, 0.35), &m_coque, Vec3::new(sx * 0.42, 0.95, 0.0), id, un);
            }
            piece(commands, Cylinder::new(0.28, 0.5).mesh().resolution(6).build(), &m_detail, Vec3::new(0.95, 0.35, 0.05), Quat::from_rotation_x(std::f32::consts::FRAC_PI_2), un);
            piece(commands, Sphere::new(0.18).mesh().ico(0).unwrap(), &m_vitre, Vec3::new(0.95, 0.35, 0.3), id, un);
            piece(commands, boite(1.1, 0.25, 0.5), &m_detail, Vec3::new(0.0, -0.95, 0.0), id, un);
            for sx in [-0.4, 0.0, 0.4] {
                piece(commands, cone(0.14, 0.35, 5), &m_reacteur, Vec3::new(sx, -1.25, 0.0), Quat::from_rotation_z(std::f32::consts::PI), un);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn neuve() -> Progression {
        Progression { total: 0, par_vaisseau: [0; 4], choisi: Vaisseau::Faucheuse, livree: [0; 4], record: 0, meilleur_parsec: 0, defi_fait: None, serie: 0 }
    }

    #[test]
    fn on_commence_avec_un_seul_vaisseau() {
        let p = neuve();
        assert!(p.gagne(Vaisseau::Faucheuse));
        assert!(!p.gagne(Vaisseau::Bourdon));
        assert_eq!(p.prochain(), Some(Vaisseau::Bourdon));
    }

    #[test]
    fn les_points_cumules_debloquent() {
        let mut p = neuve();
        assert_eq!(p.ajouter(3_000), vec![Gain::Livree(Vaisseau::Faucheuse, 1)]);
        let gains = p.ajouter(2_500);
        assert!(gains.contains(&Gain::Vaisseau(Vaisseau::Bourdon)));
        assert_eq!(p.total, 5_500);
        // Les livrées comptent les points faits avec ce vaisseau-là.
        p.choisi = Vaisseau::Bourdon;
        assert!(!p.livree_gagnee(Vaisseau::Bourdon, 1));
        p.ajouter(2_000);
        assert!(p.livree_gagnee(Vaisseau::Bourdon, 1));
    }
}
