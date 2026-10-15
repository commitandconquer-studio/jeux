//! Modèles 3D low-poly construits à partir de formes simples :
//! crapaud, voitures, camions, train, rondins, nénuphars, arbres, rochers.
use std::f32::consts::FRAC_PI_2;

use bevy::prelude::*;
use game_common::flat_material;

/// Formes de base et matériaux partagés par tous les modèles.
#[derive(Resource)]
pub struct Kit {
    pub cube: Handle<Mesh>,
    /// Cylindre unitaire (rayon 0,5, hauteur 1) à facettes.
    pub cyl: Handle<Mesh>,
    /// Cône unitaire à facettes.
    pub cone: Handle<Mesh>,
    /// Sphère à facettes (feuillages).
    pub blob: Handle<Mesh>,
    /// Sphère très grossière (rochers).
    pub rock: Handle<Mesh>,
    /// Sphère lisse (yeux).
    pub ball: Handle<Mesh>,
    pub m: Mats,
}

pub struct Mats {
    pub grass: [Handle<StandardMaterial>; 2],
    pub grass_side: Handle<StandardMaterial>,
    pub road: Handle<StandardMaterial>,
    pub road_side: Handle<StandardMaterial>,
    pub line: Handle<StandardMaterial>,
    pub water: Handle<StandardMaterial>,
    pub water_side: Handle<StandardMaterial>,
    pub gravel: Handle<StandardMaterial>,
    pub sleeper: Handle<StandardMaterial>,
    pub rail: Handle<StandardMaterial>,
    pub dark: Handle<StandardMaterial>,
    pub white: Handle<StandardMaterial>,
    pub black: Handle<StandardMaterial>,
    pub frog: Handle<StandardMaterial>,
    pub frog_dark: Handle<StandardMaterial>,
    pub frog_belly: Handle<StandardMaterial>,
    pub glass: Handle<StandardMaterial>,
    pub tire: Handle<StandardMaterial>,
    pub hub: Handle<StandardMaterial>,
    pub headlight: Handle<StandardMaterial>,
    pub taillight: Handle<StandardMaterial>,
    pub car: Vec<Handle<StandardMaterial>>,
    pub cargo: Vec<Handle<StandardMaterial>>,
    pub train: Handle<StandardMaterial>,
    pub wagon: Handle<StandardMaterial>,
    pub stripe: Handle<StandardMaterial>,
    pub trunk: Handle<StandardMaterial>,
    pub leaf: [Handle<StandardMaterial>; 3],
    pub pine: Handle<StandardMaterial>,
    pub stone: Handle<StandardMaterial>,
    pub log: Handle<StandardMaterial>,
    pub log_end: Handle<StandardMaterial>,
    pub pad: Handle<StandardMaterial>,
    pub flower: Handle<StandardMaterial>,
    pub signal_off: Handle<StandardMaterial>,
    pub signal_on: Handle<StandardMaterial>,
}

fn faceted(mesh: Mesh) -> Mesh {
    mesh.with_duplicated_vertices().with_computed_flat_normals()
}

impl FromWorld for Kit {
    fn from_world(world: &mut World) -> Self {
        let mut meshes = world.resource_mut::<Assets<Mesh>>();
        let cube = meshes.add(Cuboid::new(1.0, 1.0, 1.0));
        let cyl = meshes.add(faceted(Cylinder::new(0.5, 1.0).mesh().resolution(10).build()));
        let cone = meshes.add(faceted(Cone { radius: 0.5, height: 1.0 }.mesh().resolution(7).build()));
        let blob = meshes.add(faceted(Sphere::new(0.5).mesh().ico(1).unwrap()));
        let rock = meshes.add(faceted(Sphere::new(0.5).mesh().ico(0).unwrap()));
        let ball = meshes.add(Sphere::new(0.5).mesh().uv(16, 12));

        let mut mats = world.resource_mut::<Assets<StandardMaterial>>();
        let signal_on = mats.add(StandardMaterial {
            emissive: LinearRgba::rgb(6.0, 0.4, 0.3),
            ..flat_material(Color::srgb(1.0, 0.2, 0.2))
        });
        let mut flat =|r: f32, g: f32, b: f32| mats.add(flat_material(Color::srgb(r, g, b)));
        let m = Mats {
            grass: [flat(0.62, 0.85, 0.40), flat(0.57, 0.80, 0.37)],
            grass_side: flat(0.48, 0.70, 0.33),
            road: flat(0.36, 0.37, 0.43),
            road_side: flat(0.30, 0.31, 0.36),
            line: flat(0.95, 0.95, 0.90),
            water: flat(0.33, 0.68, 0.90),
            water_side: flat(0.27, 0.58, 0.80),
            gravel: flat(0.62, 0.58, 0.54),
            sleeper: flat(0.45, 0.33, 0.24),
            rail: flat(0.70, 0.72, 0.76),
            dark: flat(0.18, 0.18, 0.22),
            white: flat(0.97, 0.97, 0.97),
            black: flat(0.06, 0.06, 0.08),
            frog: flat(0.36, 0.72, 0.30),
            frog_dark: flat(0.22, 0.50, 0.20),
            frog_belly: flat(0.86, 0.90, 0.55),
            glass: flat(0.26, 0.38, 0.52),
            tire: flat(0.12, 0.12, 0.14),
            hub: flat(0.75, 0.76, 0.80),
            headlight: flat(1.0, 0.97, 0.80),
            taillight: flat(0.90, 0.15, 0.15),
            car: vec![
                flat(0.93, 0.33, 0.30),
                flat(0.25, 0.55, 0.90),
                flat(0.98, 0.78, 0.20),
                flat(0.60, 0.40, 0.85),
                flat(0.95, 0.55, 0.20),
                flat(0.20, 0.70, 0.65),
            ],
            cargo: vec![flat(0.94, 0.94, 0.92), flat(0.85, 0.80, 0.70), flat(0.70, 0.78, 0.85)],
            train: flat(0.85, 0.20, 0.20),
            wagon: flat(0.25, 0.42, 0.75),
            stripe: flat(0.98, 0.85, 0.25),
            trunk: flat(0.50, 0.35, 0.24),
            leaf: [flat(0.30, 0.62, 0.30), flat(0.42, 0.72, 0.32), flat(0.55, 0.75, 0.30)],
            pine: flat(0.20, 0.48, 0.32),
            stone: flat(0.62, 0.63, 0.66),
            log: flat(0.55, 0.36, 0.22),
            log_end: flat(0.80, 0.62, 0.42),
            pad: flat(0.30, 0.65, 0.30),
            flower: flat(0.98, 0.55, 0.75),
            signal_off: flat(0.35, 0.10, 0.10),
            signal_on,
        };
        Kit { cube, cyl, cone, blob, rock, ball, m }
    }
}

/// Ajoute une pièce (forme + matériau) comme enfant de `parent`.
pub fn part(
    commands: &mut Commands,
    parent: Entity,
    mesh: &Handle<Mesh>,
    mat: &Handle<StandardMaterial>,
    pos: Vec3,
    scale: Vec3,
) -> Entity {
    part_rot(commands, parent, mesh, mat, pos, scale, Quat::IDENTITY)
}

pub fn part_rot(
    commands: &mut Commands,
    parent: Entity,
    mesh: &Handle<Mesh>,
    mat: &Handle<StandardMaterial>,
    pos: Vec3,
    scale: Vec3,
    rot: Quat,
) -> Entity {
    commands
        .spawn((
            Mesh3d(mesh.clone()),
            MeshMaterial3d(mat.clone()),
            Transform { translation: pos, rotation: rot, scale },
            ChildOf(parent),
        ))
        .id()
}

/// Roue : cylindre couché selon l'axe Z, avec enjoliveur.
fn wheel(c: &mut Commands, k: &Kit, parent: Entity, pos: Vec3, radius: f32) {
    let axis_z = Quat::from_rotation_x(FRAC_PI_2);
    let d = radius * 2.0;
    part_rot(c, parent, &k.cyl, &k.m.tire, pos, Vec3::new(d, 0.14, d), axis_z);
    let out = pos + Vec3::Z * pos.z.signum() * 0.075;
    part_rot(c, parent, &k.cyl, &k.m.hub, out, Vec3::new(d * 0.45, 0.02, d * 0.45), axis_z);
}

/// Le crapaud, tête vers -Z. Renvoie l'entité racine.
pub fn spawn_frog(c: &mut Commands, k: &Kit) -> Entity {
    let root = c.spawn((Transform::default(), Visibility::default())).id();
    let m = &k.m;
    // Corps trapu, ventre clair, tête plate
    part(c, root, &k.cube, &m.frog, Vec3::new(0.0, 0.27, 0.05), Vec3::new(0.62, 0.32, 0.62));
    part(c, root, &k.cube, &m.frog_belly, Vec3::new(0.0, 0.12, 0.0), Vec3::new(0.56, 0.1, 0.62));
    part(c, root, &k.cube, &m.frog, Vec3::new(0.0, 0.32, -0.3), Vec3::new(0.56, 0.24, 0.3));
    // Bouche
    part(c, root, &k.cube, &m.frog_dark, Vec3::new(0.0, 0.24, -0.452), Vec3::new(0.42, 0.03, 0.02));
    // Gros yeux ronds qui dépassent
    for s in [-1.0, 1.0] {
        part(c, root, &k.ball, &m.frog, Vec3::new(0.17 * s, 0.45, -0.3), Vec3::splat(0.22));
        part(c, root, &k.ball, &m.white, Vec3::new(0.17 * s, 0.49, -0.34), Vec3::splat(0.18));
        part(c, root, &k.ball, &m.black, Vec3::new(0.17 * s, 0.5, -0.42), Vec3::splat(0.08));
        // Pattes arrière repliées et pattes avant
        part(c, root, &k.cube, &m.frog_dark, Vec3::new(0.36 * s, 0.12, 0.18), Vec3::new(0.2, 0.18, 0.42));
        part(c, root, &k.cube, &m.frog_dark, Vec3::new(0.42 * s, 0.03, -0.02), Vec3::new(0.22, 0.05, 0.18));
        part(c, root, &k.cube, &m.frog, Vec3::new(0.26 * s, 0.1, -0.34), Vec3::new(0.11, 0.2, 0.11));
        part(c, root, &k.cube, &m.frog_dark, Vec3::new(0.27 * s, 0.02, -0.4), Vec3::new(0.16, 0.04, 0.12));
    }
    // Taches sur le dos
    for (x, z, s) in [(-0.13, 0.12, 0.12), (0.12, 0.22, 0.1), (0.06, -0.02, 0.08)] {
        part(c, root, &k.cube, &m.frog_dark, Vec3::new(x, 0.435, z), Vec3::new(s, 0.02, s));
    }
    root
}

/// Voiture orientée vers +X. Longueur 1,6.
pub fn spawn_car(c: &mut Commands, k: &Kit, parent: Entity, color: usize) {
    let m = &k.m;
    let paint = &m.car[color % m.car.len()];
    part(c, parent, &k.cube, &m.dark, Vec3::new(0.0, 0.21, 0.0), Vec3::new(1.6, 0.12, 0.8));
    part(c, parent, &k.cube, paint, Vec3::new(0.0, 0.4, 0.0), Vec3::new(1.54, 0.34, 0.78));
    part(c, parent, &k.cube, &m.glass, Vec3::new(-0.12, 0.71, 0.0), Vec3::new(0.82, 0.3, 0.7));
    part(c, parent, &k.cube, paint, Vec3::new(-0.12, 0.88, 0.0), Vec3::new(0.86, 0.06, 0.74));
    for s in [-1.0, 1.0] {
        part(c, parent, &k.cube, &m.headlight, Vec3::new(0.775, 0.44, 0.26 * s), Vec3::new(0.03, 0.09, 0.15));
        part(c, parent, &k.cube, &m.taillight, Vec3::new(-0.775, 0.44, 0.26 * s), Vec3::new(0.03, 0.08, 0.15));
        for x in [-0.5, 0.5] {
            wheel(c, k, parent, Vec3::new(x, 0.19, 0.37 * s), 0.19);
        }
    }
}

/// Camion orienté vers +X. Longueur 2,9.
pub fn spawn_truck(c: &mut Commands, k: &Kit, parent: Entity, color: usize) {
    let m = &k.m;
    let paint = &m.car[color % m.car.len()];
    let cargo = &m.cargo[color % m.cargo.len()];
    part(c, parent, &k.cube, &m.dark, Vec3::new(0.0, 0.26, 0.0), Vec3::new(2.9, 0.14, 0.72));
    part(c, parent, &k.cube, paint, Vec3::new(1.02, 0.66, 0.0), Vec3::new(0.82, 0.78, 0.82));
    part(c, parent, &k.cube, &m.glass, Vec3::new(1.43, 0.8, 0.0), Vec3::new(0.02, 0.3, 0.68));
    part(c, parent, &k.cube, cargo, Vec3::new(-0.44, 0.86, 0.0), Vec3::new(1.98, 1.06, 0.88));
    for s in [-1.0, 1.0] {
        part(c, parent, &k.cube, &m.glass, Vec3::new(1.08, 0.82, 0.415 * s), Vec3::new(0.42, 0.26, 0.02));
        part(c, parent, &k.cube, &m.headlight, Vec3::new(1.435, 0.42, 0.27 * s), Vec3::new(0.03, 0.1, 0.15));
        part(c, parent, &k.cube, &m.taillight, Vec3::new(-1.445, 0.42, 0.3 * s), Vec3::new(0.03, 0.1, 0.12));
        for x in [1.0, -0.3, -1.0] {
            wheel(c, k, parent, Vec3::new(x, 0.21, 0.37 * s), 0.21);
        }
    }
}

/// Train orienté vers +X : locomotive + `wagons` voitures. Longueur 3,1 par élément.
pub fn spawn_train(c: &mut Commands, k: &Kit, parent: Entity, wagons: usize) -> f32 {
    let m = &k.m;
    let n = wagons + 1;
    let len = n as f32 * 3.1;
    for i in 0..n {
        let x = len / 2.0 - 1.55 - i as f32 * 3.1;
        let body = if i == 0 { &m.train } else { &m.wagon };
        part(c, parent, &k.cube, body, Vec3::new(x, 0.85, 0.0), Vec3::new(2.95, 1.05, 0.92));
        part(c, parent, &k.cube, &m.dark, Vec3::new(x, 1.41, 0.0), Vec3::new(2.85, 0.08, 0.84));
        part(c, parent, &k.cube, &m.dark, Vec3::new(x, 0.25, 0.0), Vec3::new(2.7, 0.14, 0.7));
        for s in [-1.0, 1.0] {
            part(c, parent, &k.cube, &m.glass, Vec3::new(x, 1.0, 0.465 * s), Vec3::new(2.4, 0.26, 0.02));
            part(c, parent, &k.cube, &m.stripe, Vec3::new(x, 0.6, 0.465 * s), Vec3::new(2.95, 0.08, 0.02));
            for dx in [-1.0, 1.0] {
                wheel(c, k, parent, Vec3::new(x + dx, 0.2, 0.36 * s), 0.18);
            }
        }
    }
    // Avant de la locomotive : pare-brise, bande jaune, phare
    let front = len / 2.0;
    part(c, parent, &k.cube, &m.glass, Vec3::new(front + 0.005, 1.0, 0.0), Vec3::new(0.02, 0.3, 0.7));
    part(c, parent, &k.cube, &m.stripe, Vec3::new(front + 0.005, 0.55, 0.0), Vec3::new(0.02, 0.25, 0.92));
    part(c, parent, &k.cube, &m.headlight, Vec3::new(front + 0.02, 0.75, 0.0), Vec3::new(0.03, 0.12, 0.2));
    len
}

/// Rondin couché selon l'axe X.
pub fn spawn_log(c: &mut Commands, k: &Kit, parent: Entity, len: f32) {
    let axis_x = Quat::from_rotation_z(FRAC_PI_2);
    part_rot(c, parent, &k.cyl, &k.m.log, Vec3::new(0.0, -0.05, 0.0), Vec3::new(0.6, len, 0.6), axis_x);
    for s in [-1.0, 1.0] {
        let pos = Vec3::new(s * (len / 2.0 + 0.005), -0.05, 0.0);
        part_rot(c, parent, &k.cyl, &k.m.log_end, pos, Vec3::new(0.48, 0.02, 0.48), axis_x);
    }
}

pub fn spawn_pad(c: &mut Commands, k: &Kit, parent: Entity, flower: bool) {
    part(c, parent, &k.cyl, &k.m.pad, Vec3::new(0.0, -0.13, 0.0), Vec3::new(0.85, 0.04, 0.85));
    if flower {
        part(c, parent, &k.blob, &k.m.flower, Vec3::new(0.18, -0.06, 0.15), Vec3::splat(0.16));
    }
}

#[derive(Clone, Copy)]
pub enum Prop {
    Tree,
    Pine,
    Rock,
    Bush,
}

/// Arbre, sapin, rocher ou buisson posé au sol en `pos`.
pub fn spawn_prop(c: &mut Commands, k: &Kit, parent: Entity, prop: Prop, pos: Vec3, variant: usize) {
    let m = &k.m;
    match prop {
        Prop::Tree => {
            part(c, parent, &k.cyl, &m.trunk, pos + Vec3::Y * 0.25, Vec3::new(0.18, 0.5, 0.18));
            let leaf = &m.leaf[variant % 3];
            part(c, parent, &k.blob, leaf, pos + Vec3::Y * 0.8, Vec3::new(0.85, 0.8, 0.85));
        }
        Prop::Pine => {
            part(c, parent, &k.cyl, &m.trunk, pos + Vec3::Y * 0.2, Vec3::new(0.16, 0.4, 0.16));
            part(c, parent, &k.cone, &m.pine, pos + Vec3::Y * 0.8, Vec3::new(0.8, 0.95, 0.8));
            part(c, parent, &k.cone, &m.pine, pos + Vec3::Y * 1.2, Vec3::new(0.58, 0.7, 0.58));
        }
        Prop::Rock => {
            let rot = Quat::from_rotation_y(variant as f32 * 1.3);
            part_rot(c, parent, &k.rock, &m.stone, pos + Vec3::Y * 0.15, Vec3::new(0.7, 0.45, 0.6), rot);
        }
        Prop::Bush => {
            part(c, parent, &k.blob, &m.leaf[variant % 3], pos + Vec3::Y * 0.2, Vec3::new(0.6, 0.45, 0.6));
        }
    }
}
