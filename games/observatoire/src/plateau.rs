//! Le décor : le bureau de l'astronome, la nuit. Une table en bois, une carte
//! du ciel bleu nuit cerclée de laiton (la grille), une lunette de cuivre, un
//! globe céleste, un compas et une chandelle. Les soleils sont des médaillons
//! dorés, les lunes des croissants d'argent ; les signes « = » et « × » sont
//! gravés en laiton entre les cases.
use std::f32::consts::{FRAC_PI_2, FRAC_PI_4, TAU};

use bevy::light::NotShadowCaster;
use bevy::prelude::*;
use game_common::{Rng, flat_material};
use observatoire_core::{LUNE, SOLEIL, Signe, Signes};

/// Hauteur du dessus des cases.
pub const DESSUS: f32 = 0.08;

/// Ce qui dépend de la grille en cours (refait à chaque nouvelle grille).
#[derive(Component)]
pub struct DeLaGrille;

/// Une marque posée sur une case (soleil, lune, erreur, indice).
#[derive(Component)]
pub struct Marque;

/// La flamme de la chandelle (et sa lumière) qui vacillent.
#[derive(Component)]
pub struct Flamme;

#[derive(Resource)]
pub struct Look {
    pub cube: Handle<Mesh>,
    pub disque: Handle<Mesh>,
    pub cone: Handle<Mesh>,
    pub boule: Handle<Mesh>,
    pub case: Handle<StandardMaterial>,
    pub case_donnee: Handle<StandardMaterial>,
    pub or: Handle<StandardMaterial>,
    pub or_vif: Handle<StandardMaterial>,
    pub argent: Handle<StandardMaterial>,
    pub laiton: Handle<StandardMaterial>,
    pub erreur: Handle<StandardMaterial>,
    pub indice: Handle<StandardMaterial>,
    pub indice_pale: Handle<StandardMaterial>,
    pub etoile: Handle<StandardMaterial>,
}

impl FromWorld for Look {
    fn from_world(world: &mut World) -> Self {
        let mut meshes = world.resource_mut::<Assets<Mesh>>();
        let cube = meshes.add(Cuboid::new(1.0, 1.0, 1.0));
        let disque = meshes.add(Cylinder::new(0.5, 1.0).mesh().resolution(16));
        let cone = meshes.add(Cone { radius: 0.5, height: 1.0 }.mesh().resolution(4));
        let boule = meshes.add(Sphere::new(0.5).mesh().ico(1).unwrap());
        let mut mats = world.resource_mut::<Assets<StandardMaterial>>();
        let lumineux = |c: Color, f: f32| StandardMaterial { emissive: c.to_linear() * f, ..flat_material(c) };
        Look {
            cube,
            disque,
            cone,
            boule,
            case: mats.add(flat_material(Color::srgb(0.10, 0.15, 0.30))),
            case_donnee: mats.add(flat_material(Color::srgb(0.17, 0.22, 0.38))),
            or: mats.add(StandardMaterial { metallic: 0.6, perceptual_roughness: 0.35, ..lumineux(Color::srgb(0.98, 0.76, 0.25), 0.35) }),
            or_vif: mats.add(lumineux(Color::srgb(1.0, 0.85, 0.35), 1.2)),
            argent: mats.add(StandardMaterial { metallic: 0.5, perceptual_roughness: 0.3, ..lumineux(Color::srgb(0.85, 0.88, 0.95), 0.25) }),
            laiton: mats.add(StandardMaterial { metallic: 0.8, perceptual_roughness: 0.4, ..flat_material(Color::srgb(0.78, 0.60, 0.30)) }),
            erreur: mats.add(StandardMaterial { base_color: Color::srgba(1.0, 0.25, 0.2, 0.55), emissive: LinearRgba::rgb(1.2, 0.15, 0.1), alpha_mode: AlphaMode::Blend, unlit: true, ..default() }),
            indice: mats.add(StandardMaterial { base_color: Color::srgb(0.45, 0.95, 0.85), emissive: LinearRgba::rgb(0.4, 1.4, 1.1), unlit: true, ..default() }),
            indice_pale: mats.add(StandardMaterial { base_color: Color::srgba(0.45, 0.95, 0.85, 0.35), emissive: LinearRgba::rgb(0.15, 0.5, 0.4), alpha_mode: AlphaMode::Blend, unlit: true, ..default() }),
            etoile: mats.add(StandardMaterial { base_color: Color::srgb(1.0, 0.97, 0.85), emissive: LinearRgba::rgb(2.5, 2.4, 2.0), unlit: true, ..default() }),
        }
    }
}

/// Centre d'une case (la grille est dans le plan XZ, centrée sur l'origine).
pub fn pos_case(n: usize, i: usize) -> Vec3 {
    let d = (n as f32 - 1.0) / 2.0;
    Vec3::new((i % n) as f32 - d, DESSUS, (i / n) as f32 - d)
}

fn piece(c: &mut Commands, mesh: &Handle<Mesh>, mat: &Handle<StandardMaterial>, t: Transform) -> Entity {
    c.spawn((Mesh3d(mesh.clone()), MeshMaterial3d(mat.clone()), t)).id()
}

/// La table, la lunette, le globe, le compas et la chandelle (une fois pour toutes).
pub fn bureau(c: &mut Commands, look: &Look, meshes: &mut Assets<Mesh>, mats: &mut Assets<StandardMaterial>) {
    let bois = [Color::srgb(0.36, 0.22, 0.13), Color::srgb(0.40, 0.25, 0.15), Color::srgb(0.33, 0.20, 0.12)].map(|col| mats.add(flat_material(col)));
    for k in 0..9 {
        let z = -12.0 + k as f32 * 3.0;
        piece(c, &look.cube, &bois[k % 3], Transform::from_xyz(0.0, -0.32, z).with_scale(Vec3::new(40.0, 0.5, 2.9)));
    }
    // La lunette de cuivre sur son trépied, en haut à droite.
    let cuivre = mats.add(StandardMaterial { metallic: 0.8, perceptual_roughness: 0.35, ..flat_material(Color::srgb(0.80, 0.48, 0.26)) });
    let noir = mats.add(flat_material(Color::srgb(0.08, 0.08, 0.10)));
    let lunette = c.spawn((Transform::from_xyz(7.8, 0.9, -3.8).with_rotation(Quat::from_rotation_y(-0.6) * Quat::from_rotation_z(0.35)), Visibility::default())).id();
    for (r, l, x, m) in [(0.28, 3.0, 0.0, &cuivre), (0.36, 0.9, 1.6, &cuivre), (0.2, 0.6, -1.7, &noir)] {
        let e = piece(c, &look.disque, m, Transform::from_xyz(x, 0.0, 0.0).with_rotation(Quat::from_rotation_z(FRAC_PI_2)).with_scale(Vec3::new(r * 2.0, l, r * 2.0)));
        c.entity(e).insert(ChildOf(lunette));
    }
    for a in [0.0, TAU / 3.0, 2.0 * TAU / 3.0] {
        let pied = Transform::from_xyz(7.8 + a.cos() * 0.6, 0.2, -3.8 + a.sin() * 0.6).with_rotation(Quat::from_rotation_arc(Vec3::Y, Vec3::new(a.cos() * 0.4, 1.0, a.sin() * 0.4).normalize())).with_scale(Vec3::new(0.08, 1.4, 0.08));
        piece(c, &look.cube, &noir, pied);
    }
    // Le globe céleste et son méridien de laiton, en bas à gauche.
    let globe = mats.add(flat_material(Color::srgb(0.16, 0.28, 0.55)));
    piece(c, &look.boule, &globe, Transform::from_xyz(-7.6, 1.1, 3.4).with_scale(Vec3::splat(1.9)));
    let meridien = meshes.add(Torus { minor_radius: 0.05, major_radius: 1.1 }.mesh().minor_resolution(5).major_resolution(28));
    c.spawn((Mesh3d(meridien), MeshMaterial3d(look.laiton.clone()), Transform::from_xyz(-7.6, 1.1, 3.4).with_rotation(Quat::from_rotation_x(FRAC_PI_2) * Quat::from_rotation_y(0.4))));
    piece(c, &look.disque, &look.laiton, Transform::from_xyz(-7.6, 0.0, 3.4).with_scale(Vec3::new(1.2, 0.15, 1.2)));
    // Le compas, ouvert, à côté de la carte.
    for s in [-1.0, 1.0] {
        piece(c, &look.cube, &look.laiton, Transform::from_xyz(-7.2 + s * 0.3, 0.05, -4.0).with_rotation(Quat::from_rotation_y(s * 0.25)).with_scale(Vec3::new(0.08, 0.06, 2.0)));
    }
    // La chandelle, à droite en bas, qui éclaire la carte.
    let cire = mats.add(flat_material(Color::srgb(0.95, 0.92, 0.82)));
    piece(c, &look.disque, &look.laiton, Transform::from_xyz(7.4, 0.0, 3.6).with_scale(Vec3::new(1.0, 0.1, 1.0)));
    piece(c, &look.disque, &cire, Transform::from_xyz(7.4, 0.6, 3.6).with_scale(Vec3::new(0.35, 1.1, 0.35)));
    c.spawn((
        Mesh3d(look.cone.clone()),
        MeshMaterial3d(look.or_vif.clone()),
        Transform::from_xyz(7.4, 1.3, 3.6).with_scale(Vec3::new(0.14, 0.3, 0.14)),
        PointLight { color: Color::srgb(1.0, 0.75, 0.45), intensity: 160_000.0, range: 18.0, shadow_maps_enabled: false, ..default() },
        NotShadowCaster,
        Flamme,
    ));
}

/// La carte du ciel : le disque bleu nuit cerclé de laiton, les cases, les
/// petites étoiles décoratives et les signes entre les cases.
pub fn carte(c: &mut Commands, look: &Look, meshes: &mut Assets<Mesh>, n: usize, donnees: &[u8], signes: &Signes) {
    let r = n as f32 * 0.5 + 1.4;
    c.spawn((
        Mesh3d(look.disque.clone()),
        MeshMaterial3d(look.case.clone()),
        Transform::from_xyz(0.0, -0.06, 0.0).with_scale(Vec3::new(r * 2.0, 0.12, r * 2.0)),
        DeLaGrille,
    ));
    c.spawn((
        Mesh3d(meshes.add(Torus { minor_radius: 0.12, major_radius: r }.mesh().minor_resolution(6).major_resolution(48))),
        MeshMaterial3d(look.laiton.clone()),
        Transform::from_xyz(0.0, 0.0, 0.0),
        DeLaGrille,
    ));
    // Petites étoiles semées sur le pourtour de la carte.
    let mut rng = Rng::new(n as u64 * 77);
    for _ in 0..40 {
        let a = rng.below(10_000) as f32 / 10_000.0 * TAU;
        let d = n as f32 * 0.5 * 1.42 + rng.below(1_000) as f32 / 1_000.0 * 0.9;
        if d > r - 0.2 {
            continue;
        }
        let s = 0.04 + rng.below(100) as f32 / 100.0 * 0.06;
        c.spawn((Mesh3d(look.boule.clone()), MeshMaterial3d(look.etoile.clone()), Transform::from_xyz(a.cos() * d, 0.02, a.sin() * d).with_scale(Vec3::splat(s)), NotShadowCaster, DeLaGrille));
    }
    // Les cases : un peu plus claires et cerclées de laiton quand elles sont données.
    for i in 0..n * n {
        let p = pos_case(n, i);
        let donnee = donnees[i] != 0;
        c.spawn((
            Mesh3d(look.cube.clone()),
            MeshMaterial3d(if donnee { look.case_donnee.clone() } else { look.case.clone() }),
            Transform::from_translation(p - Vec3::Y * 0.05).with_scale(Vec3::new(0.92, 0.1, 0.92)),
            DeLaGrille,
        ));
    }
    // Le quadrillage, en fils de laiton.
    let d = n as f32 / 2.0;
    for k in 0..=n {
        let x = k as f32 - d;
        for (pos, ech) in [(Vec3::new(x, DESSUS - 0.01, 0.0), Vec3::new(0.03, 0.02, n as f32)), (Vec3::new(0.0, DESSUS - 0.01, x), Vec3::new(n as f32, 0.02, 0.03))] {
            c.spawn((Mesh3d(look.cube.clone()), MeshMaterial3d(look.laiton.clone()), Transform::from_translation(pos).with_scale(ech), NotShadowCaster, DeLaGrille));
        }
    }
    // Les signes, à cheval sur la frontière entre deux cases.
    for (a, b, s) in signes.tous() {
        let milieu = (pos_case(n, a) + pos_case(n, b)) / 2.0 + Vec3::Y * 0.03;
        signe(c, look, milieu, s);
    }
}

fn signe(c: &mut Commands, look: &Look, p: Vec3, s: Signe) {
    // Une pastille de laiton sous le signe, pour bien le voir.
    c.spawn((Mesh3d(look.disque.clone()), MeshMaterial3d(look.laiton.clone()), Transform::from_translation(p).with_scale(Vec3::new(0.3, 0.03, 0.3)), NotShadowCaster, DeLaGrille));
    let haut = p + Vec3::Y * 0.025;
    match s {
        // « = » se lit toujours à l'endroit, quelle que soit la frontière.
        Signe::Egal => {
            for dz in [-0.045, 0.045] {
                c.spawn((Mesh3d(look.cube.clone()), MeshMaterial3d(look.case.clone()), Transform::from_translation(haut + Vec3::new(0.0, 0.0, dz)).with_scale(Vec3::new(0.16, 0.02, 0.035)), NotShadowCaster, DeLaGrille));
            }
        }
        Signe::Different => {
            for a in [FRAC_PI_4, -FRAC_PI_4] {
                c.spawn((Mesh3d(look.cube.clone()), MeshMaterial3d(look.case.clone()), Transform::from_translation(haut).with_rotation(Quat::from_rotation_y(a)).with_scale(Vec3::new(0.19, 0.02, 0.04)), NotShadowCaster, DeLaGrille));
            }
        }
        Signe::Aucun => {}
    }
}

/// Pose le symbole `v` sur la case `i` (médaillon solaire ou croissant de lune).
pub fn symbole(c: &mut Commands, look: &Look, n: usize, i: usize, v: u8, donnee: bool) {
    let p = pos_case(n, i) + Vec3::Y * 0.02;
    let e = c.spawn((Transform::from_translation(p), Visibility::default(), Marque, DeLaGrille)).id();
    let enfant = |c: &mut Commands, mesh: &Handle<Mesh>, mat: &Handle<StandardMaterial>, t: Transform| {
        c.spawn((Mesh3d(mesh.clone()), MeshMaterial3d(mat.clone()), t, NotShadowCaster, ChildOf(e)));
    };
    match v {
        SOLEIL => {
            enfant(c, &look.disque, &look.or, Transform::from_scale(Vec3::new(0.42, 0.08, 0.42)));
            for k in 0..8 {
                let a = k as f32 * TAU / 8.0;
                let d = Vec3::new(a.cos(), 0.0, a.sin());
                enfant(c, &look.cone, &look.or, Transform::from_translation(d * 0.3).with_rotation(Quat::from_rotation_arc(Vec3::Y, d)).with_scale(Vec3::new(0.11, 0.16, 0.05)));
            }
        }
        LUNE => {
            // Un disque d'argent, et un disque couleur de nuit décalé : un croissant.
            enfant(c, &look.disque, &look.argent, Transform::from_scale(Vec3::new(0.58, 0.08, 0.58)));
            let fond = if donnee { &look.case_donnee } else { &look.case };
            enfant(c, &look.disque, fond, Transform::from_xyz(0.15, 0.015, -0.12).with_scale(Vec3::new(0.5, 0.08, 0.5)));
        }
        _ => {}
    }
    if donnee {
        // Les symboles donnés ont un petit anneau de laiton : on ne peut pas les changer.
        enfant(c, &look.disque, &look.laiton, Transform::from_xyz(0.0, -0.02, 0.0).with_scale(Vec3::new(0.78, 0.03, 0.78)));
    }
}

/// Fait vaciller la chandelle.
pub fn vaciller(time: Res<Time>, mut q: Query<(&mut PointLight, &mut Transform), With<Flamme>>) {
    let t = time.elapsed_secs();
    for (mut l, mut tr) in &mut q {
        let f = 1.0 + 0.08 * (t * 7.3).sin() + 0.05 * (t * 13.1).sin();
        l.intensity = 160_000.0 * f;
        tr.scale = Vec3::new(0.14, 0.3 * f, 0.14);
    }
}
