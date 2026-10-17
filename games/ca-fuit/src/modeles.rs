//! Les modèles 3D dont on tire les sprites (`sprites.rs`) : les tuyaux de
//! zinc (gouttières ouvertes sur le dessus, pour voir l'eau), la fuite, le
//! décor de la pâture. Le jeu lui-même ne les affiche pas : il est en 2D.
//!
//! Une case fait 1 × 1 ; la case (x, y) de la grille est au monde en
//! (x, 0, −y) : « haut » de la grille = nord.
use std::ops::Neg;

use bevy::prelude::*;
use fuite_core::{Carte, Case, Direction};
use game_common::{flat_material, palette};

pub const CASE: f32 = 1.0;
/// Largeur intérieure d'un tuyau.
const LARGEUR: f32 = 0.34;
/// Épaisseur des parois.
const PAROI: f32 = 0.07;
/// Hauteur des parois (et de l'eau quand le tuyau est plein).
pub const HAUTEUR: f32 = 0.22;
const SOL: f32 = 0.06;

pub fn monde(case: Case) -> Vec3 {
    Vec3::new(case.0 as f32 * CASE, 0.0, -(case.1 as f32) * CASE)
}

fn axe(d: Direction) -> Vec3 {
    match d {
        Direction::Nord => Vec3::NEG_Z,
        Direction::Est => Vec3::X,
        Direction::Sud => Vec3::Z,
        Direction::Ouest => Vec3::NEG_X,
    }
}

/// Formes et matériaux partagés (créés une fois).
#[derive(Resource)]
pub struct Look {
    pub cube: Handle<Mesh>,
    pub disque: Handle<Mesh>,
    pub zinc: Handle<StandardMaterial>,
    pub zinc_sombre: Handle<StandardMaterial>,
    pub eau: Handle<StandardMaterial>,
    pub flaque: Handle<StandardMaterial>,
    pub terre: Handle<StandardMaterial>,
    pub herbe_claire: Handle<StandardMaterial>,
    pub feuillage: Handle<StandardMaterial>,
    pub tronc: Handle<StandardMaterial>,
    pub fleur: Handle<StandardMaterial>,
}

impl FromWorld for Look {
    fn from_world(world: &mut World) -> Self {
        let mut meshes = world.resource_mut::<Assets<Mesh>>();
        let cube = meshes.add(Cuboid::new(1.0, 1.0, 1.0));
        let disque = meshes.add(Cylinder::new(0.5, 1.0).mesh().resolution(14));
        let mut mats = world.resource_mut::<Assets<StandardMaterial>>();
        let mut plat = |c: Color| mats.add(flat_material(c));
        let zinc = plat(Color::srgb(0.64, 0.68, 0.70));
        let zinc_sombre = plat(Color::srgb(0.42, 0.46, 0.49));
        let herbe_claire = plat(palette::GRASS_LIGHT);
        let feuillage = plat(palette::FOLIAGE);
        let tronc = plat(Color::srgb(0.45, 0.32, 0.20));
        let terre = plat(Color::srgb(0.48, 0.36, 0.24));
        let fleur = plat(palette::YELLOW);
        let mut translucide = |c: Color, a: f32| {
            mats.add(StandardMaterial { base_color: c.with_alpha(a), alpha_mode: AlphaMode::Blend, perceptual_roughness: 0.7, ..default() })
        };
        let eau = translucide(Color::srgb(0.22, 0.58, 0.92), 0.92);
        let flaque = translucide(Color::srgb(0.30, 0.60, 0.90), 0.75);
        Look { cube, disque, zinc, zinc_sombre, eau, flaque, terre, herbe_claire, feuillage, tronc, fleur }
    }
}

fn bloc(c: &mut Commands, parent: Entity, look: &Look, mat: &Handle<StandardMaterial>, pos: Vec3, taille: Vec3) -> Entity {
    c.spawn((Mesh3d(look.cube.clone()), MeshMaterial3d(mat.clone()), Transform::from_translation(pos).with_scale(taille), ChildOf(parent))).id()
}

/// L'eau d'un tuyau : un bloc qui monte avec le remplissage (`niveau` de 0 à 1).
#[derive(Component)]
pub struct Eau {
    pub base: Vec3,
    pub taille: Vec3,
}

fn eau(c: &mut Commands, parent: Entity, look: &Look, pos: Vec3, taille: Vec3, rot: Quat) {
    let base = Vec3::new(pos.x, SOL, pos.z);
    c.spawn((
        Mesh3d(look.cube.clone()),
        MeshMaterial3d(look.eau.clone()),
        Transform::from_translation(base).with_rotation(rot).with_scale(Vec3::new(taille.x, 0.001, taille.z)),
        Visibility::Hidden,
        Eau { base, taille: Vec3::new(taille.x, HAUTEUR - 0.03, taille.z) },
        ChildOf(parent),
    ));
}

/// Un tronçon droit de gouttière : le fond, les deux parois et l'eau,
/// centré en `centre`, tourné de `yaw` autour de la verticale, long de `longueur`.
#[allow(clippy::too_many_arguments)]
fn troncon(c: &mut Commands, parent: Entity, look: &Look, paroi: &Handle<StandardMaterial>, fond: &Handle<StandardMaterial>, centre: Vec3, yaw: f32, longueur: f32, avec_eau: bool) {
    let rot = Quat::from_rotation_y(yaw);
    let lateral = rot * Vec3::Z;
    c.spawn((Mesh3d(look.cube.clone()), MeshMaterial3d(fond.clone()), Transform::from_translation(centre + Vec3::Y * SOL / 2.0).with_rotation(rot).with_scale(Vec3::new(longueur, SOL, LARGEUR)), ChildOf(parent)));
    for s in [-1.0, 1.0] {
        c.spawn((
            Mesh3d(look.cube.clone()),
            MeshMaterial3d(paroi.clone()),
            Transform::from_translation(centre + lateral * s * (LARGEUR / 2.0 + PAROI / 2.0) + Vec3::Y * HAUTEUR / 2.0).with_rotation(rot).with_scale(Vec3::new(longueur, HAUTEUR, PAROI)),
            ChildOf(parent),
        ));
    }
    if avec_eau {
        eau(c, parent, look, centre, Vec3::new(longueur, 0.0, LARGEUR), rot);
    }
}

/// Un coude arrondi entre deux côtés voisins d'une case : un quart de cercle
/// autour du coin, du milieu d'un bord au milieu de l'autre.
#[allow(clippy::too_many_arguments)]
fn coude(c: &mut Commands, parent: Entity, look: &Look, paroi: &Handle<StandardMaterial>, fond: &Handle<StandardMaterial>, centre: Vec3, a: Direction, b: Direction, avec_eau: bool) {
    let coin = centre + (axe(a) + axe(b)) * (CASE / 2.0);
    let r = CASE / 2.0;
    let n = 6;
    let pas = std::f32::consts::FRAC_PI_2 / n as f32;
    // De la direction −a (vers le centre depuis le bord a) à la direction −b.
    let depart = (-axe(a)).xz();
    let arrivee = (-axe(b)).xz();
    let sens = (depart.x * arrivee.y - depart.y * arrivee.x).signum();
    for k in 0..n {
        let t = (k as f32 + 0.5) * pas;
        let (cos, sin) = (t.cos(), t.sin());
        // Rotation de `depart` d'un angle t (dans le sens qui mène à `arrivee`).
        let dir = Vec2::new(depart.x * cos - sens * depart.y * sin, sens * depart.x * sin + depart.y * cos);
        let radial = Vec3::new(dir.x, 0.0, dir.y);
        let tangente = Vec3::new(-dir.y, 0.0, dir.x);
        let yaw = tangente.z.atan2(tangente.x).neg();
        let rot = Quat::from_rotation_y(yaw);
        let milieu = coin + radial * r;
        let longueur = r * pas * 1.35;
        c.spawn((Mesh3d(look.cube.clone()), MeshMaterial3d(fond.clone()), Transform::from_translation(milieu + Vec3::Y * SOL / 2.0).with_rotation(rot).with_scale(Vec3::new(longueur, SOL, LARGEUR)), ChildOf(parent)));
        for s in [-1.0, 1.0] {
            let rayon = r + s * (LARGEUR / 2.0 + PAROI / 2.0);
            c.spawn((
                Mesh3d(look.cube.clone()),
                MeshMaterial3d(paroi.clone()),
                Transform::from_translation(coin + radial * rayon + Vec3::Y * HAUTEUR / 2.0).with_rotation(rot).with_scale(Vec3::new(rayon * pas * 1.35, HAUTEUR, PAROI)),
                ChildOf(parent),
            ));
        }
        if avec_eau {
            eau(c, parent, look, milieu, Vec3::new(longueur, 0.0, LARGEUR), rot);
        }
    }
}

/// Dessine une carte (ses deux cases et ses tuyaux) sous `parent`. Avec
/// `fantome`, un seul matériau translucide (l'aperçu du curseur) et pas d'eau.
pub fn dessiner_carte(c: &mut Commands, parent: Entity, look: &Look, carte: &Carte, fantome: Option<&Handle<StandardMaterial>>) {
    let paroi = fantome.unwrap_or(&look.zinc);
    let fond = fantome.unwrap_or(&look.zinc_sombre);
    let avec_eau = fantome.is_none();
    let cases = carte.cases();
    for case in cases {
        let centre = monde(case);
        let ouverts: Vec<Direction> = Direction::TOUTES
            .into_iter()
            .filter(|&d| cases.contains(&d.voisine(case)) || carte.cotes().iter().any(|(ca, di, o)| *ca == case && *di == d && *o))
            .collect();
        let yaw_de = |d: Direction| match d {
            Direction::Est | Direction::Ouest => 0.0,
            Direction::Nord | Direction::Sud => std::f32::consts::FRAC_PI_2,
        };
        match ouverts.as_slice() {
            // Deux côtés opposés : un tuyau droit qui traverse la case.
            [a, b] if *b == a.opposee() => troncon(c, parent, look, paroi, fond, centre, yaw_de(*a), CASE, avec_eau),
            // Deux côtés voisins : un coude arrondi.
            [a, b] => coude(c, parent, look, paroi, fond, centre, *a, *b, avec_eau),
            // Un bouchon (un seul côté) ou un carrefour : un bout de tuyau vers
            // chaque côté ouvert, et un mur qui ferme chaque côté fermé.
            _ => {
                for d in Direction::TOUTES {
                    let a = axe(d);
                    let lateral = Vec3::new(a.z.abs(), 0.0, a.x.abs());
                    if ouverts.contains(&d) {
                        let longueur = CASE / 2.0 - LARGEUR / 2.0;
                        troncon(c, parent, look, paroi, fond, centre + a * (LARGEUR / 2.0 + longueur / 2.0), yaw_de(d), longueur, avec_eau);
                    } else {
                        let t = a.abs() * PAROI + lateral * (LARGEUR + 2.0 * PAROI) + Vec3::Y * HAUTEUR;
                        bloc(c, parent, look, paroi, centre + a * (LARGEUR / 2.0 + PAROI / 2.0) + Vec3::Y * HAUTEUR / 2.0, t);
                    }
                }
                bloc(c, parent, look, fond, centre + Vec3::Y * SOL / 2.0, Vec3::new(LARGEUR, SOL, LARGEUR));
                if avec_eau {
                    eau(c, parent, look, centre, Vec3::new(LARGEUR, 0.0, LARGEUR), Quat::IDENTITY);
                }
            }
        }
    }
}

/// La fuite : la carte toute ouverte, plus le tuyau crevé qui jaillit et la
/// terre retournée autour.
pub fn dessiner_fuite(c: &mut Commands, parent: Entity, look: &Look, carte: &Carte) {
    let cases = carte.cases();
    let centre = (monde(cases[0]) + monde(cases[1])) / 2.0;
    c.spawn((
        Mesh3d(look.disque.clone()),
        MeshMaterial3d(look.terre.clone()),
        Transform::from_translation(centre + Vec3::Y * 0.01).with_scale(Vec3::new(3.2, 0.02, 2.4)),
        ChildOf(parent),
    ));
    dessiner_carte(c, parent, look, carte, None);
    // Le bout de canalisation qui sort de terre, cassé, et le jet.
    bloc(c, parent, look, &look.zinc_sombre, centre + Vec3::Y * 0.35, Vec3::new(0.30, 0.5, 0.30));
    bloc(c, parent, look, &look.zinc, centre + Vec3::Y * 0.62, Vec3::new(0.38, 0.08, 0.38));
    for (i, (dx, dz, h)) in [(0.0, 0.0, 0.9), (0.12, -0.08, 0.7), (-0.1, 0.1, 0.55)].iter().enumerate() {
        bloc(c, parent, look, &look.eau, centre + Vec3::new(*dx, 0.62 + h / 2.0, *dz), Vec3::new(0.12 - i as f32 * 0.02, *h, 0.12 - i as f32 * 0.02));
    }
}

/// Un arbre : tronc et deux cônes de feuillage.
pub fn dessiner_arbre(c: &mut Commands, racine: Entity, look: &Look, meshes: &mut Assets<Mesh>, taille: f32) {
    let cone = meshes.add(Cone { radius: 0.8, height: 2.0 }.mesh().resolution(7));
    bloc(c, racine, look, &look.tronc, Vec3::Y * 0.4 * taille, Vec3::new(0.25, 0.8, 0.25) * taille);
    c.spawn((Mesh3d(cone.clone()), MeshMaterial3d(look.feuillage.clone()), Transform::from_translation(Vec3::Y * 1.5 * taille).with_scale(Vec3::splat(taille)), ChildOf(racine)));
    c.spawn((Mesh3d(cone), MeshMaterial3d(look.feuillage.clone()), Transform::from_translation(Vec3::Y * 2.3 * taille).with_scale(Vec3::splat(taille * 0.7)), ChildOf(racine)));
}

/// Une touffe d'herbe (trois brins), avec ou sans fleur.
pub fn dessiner_touffe(c: &mut Commands, racine: Entity, look: &Look, meshes: &mut Assets<Mesh>, graine: usize, fleur: bool) {
    let touffe = meshes.add(Cone { radius: 0.18, height: 0.45 }.mesh().resolution(5));
    for k in 0..3 {
        let a = k as f32 * 2.1 + graine as f32;
        c.spawn((
            Mesh3d(touffe.clone()),
            MeshMaterial3d(look.herbe_claire.clone()),
            Transform::from_xyz(a.cos() * 0.18, 0.2, a.sin() * 0.18).with_rotation(Quat::from_rotation_z(0.25 * (k as f32 - 1.0))),
            ChildOf(racine),
        ));
    }
    if fleur {
        c.spawn((Mesh3d(look.disque.clone()), MeshMaterial3d(look.fleur.clone()), Transform::from_xyz(0.25, 0.3, -0.2).with_scale(Vec3::new(0.14, 0.03, 0.14)), ChildOf(racine)));
    }
}
