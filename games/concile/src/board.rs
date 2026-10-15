//! La salle du concile : table, carte des diocèses en parchemin, sceaux de
//! cire, croix à la plume, hachures du mode Novice, objets sur la table.
use std::f32::consts::{FRAC_PI_2, FRAC_PI_4, PI};

use bevy::asset::RenderAssetUsages;
use bevy::light::NotShadowCaster;
use bevy::mesh::PrimitiveTopology;
use bevy::prelude::*;
use game_common::{Rng, flat_material};

/// Couleurs aquarelle des diocèses.
const WATERCOLORS: [Color; 10] = [
    Color::srgb(0.90, 0.74, 0.45), // ocre
    Color::srgb(0.62, 0.78, 0.52), // vert prairie
    Color::srgb(0.58, 0.72, 0.88), // bleu ciel
    Color::srgb(0.92, 0.60, 0.52), // terre cuite
    Color::srgb(0.78, 0.64, 0.86), // lilas
    Color::srgb(0.95, 0.88, 0.50), // jaune paille
    Color::srgb(0.48, 0.76, 0.74), // turquoise
    Color::srgb(0.95, 0.70, 0.82), // rose
    Color::srgb(0.70, 0.70, 0.48), // olive
    Color::srgb(0.66, 0.66, 0.74), // ardoise
];

/// Hauteur du dessus des cases.
pub const TOP: f32 = 0.08;

/// Tout ce qui dépend de la grille en cours (refait à chaque nouvelle grille).
#[derive(Component)]
pub struct MapPart;

/// Une case de la carte et ses marques.
#[derive(Component)]
pub struct CellView(pub usize);

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub enum MarkKind {
    Cross,
    Hatch,
    Seal,
    Hint,
}

/// Le sceau d'un évêque (pour le faire rougir et trembler en cas de conflit).
#[derive(Component)]
pub struct SealBody;

/// La flamme de la chandelle (et sa lumière) qui vacillent.
#[derive(Component)]
pub struct Flame;

#[derive(Resource)]
pub struct Look {
    pub cube: Handle<Mesh>,
    pub cyl: Handle<Mesh>,
    pub cone: Handle<Mesh>,
    pub ball: Handle<Mesh>,
    pub ink: Handle<StandardMaterial>,
    pub ink_pale: Handle<StandardMaterial>,
    pub wax: Handle<StandardMaterial>,
    pub wax_angry: Handle<StandardMaterial>,
    pub wax_relief: Handle<StandardMaterial>,
    pub gold: Handle<StandardMaterial>,
    pub hint: Handle<StandardMaterial>,
    pub parchment: Handle<StandardMaterial>,
}

impl FromWorld for Look {
    fn from_world(world: &mut World) -> Self {
        let mut meshes = world.resource_mut::<Assets<Mesh>>();
        let cube = meshes.add(Cuboid::new(1.0, 1.0, 1.0));
        let cyl = meshes.add(Cylinder::new(0.5, 1.0).mesh().resolution(10).build().with_duplicated_vertices().with_computed_flat_normals());
        let cone = meshes.add(Cone { radius: 0.5, height: 1.0 }.mesh().resolution(8).build().with_duplicated_vertices().with_computed_flat_normals());
        let ball = meshes.add(Sphere::new(0.5).mesh().ico(1).unwrap().with_duplicated_vertices().with_computed_flat_normals());
        let mut mats = world.resource_mut::<Assets<StandardMaterial>>();
        let mut m = |r, g, b| mats.add(flat_material(Color::srgb(r, g, b)));
        Look {
            cube,
            cyl,
            cone,
            ball,
            ink: m(0.33, 0.21, 0.12),
            ink_pale: m(0.62, 0.50, 0.36),
            wax: m(0.66, 0.13, 0.16),
            wax_angry: mats.add(StandardMaterial { emissive: LinearRgba::rgb(1.5, 0.1, 0.05), ..flat_material(Color::srgb(0.95, 0.15, 0.10)) }),
            wax_relief: mats.add(flat_material(Color::srgb(0.96, 0.89, 0.76))),
            gold: mats.add(flat_material(Color::srgb(0.90, 0.72, 0.25))),
            hint: mats.add(StandardMaterial { emissive: LinearRgba::rgb(2.0, 1.5, 0.3), ..flat_material(Color::srgb(1.0, 0.85, 0.30)) }),
            parchment: mats.add(flat_material(Color::srgb(0.91, 0.84, 0.67))),
        }
    }
}

/// Case → position du centre de la case (carte centrée sur l'origine).
pub fn cell_pos(n: usize, i: usize) -> Vec3 {
    let off = (n as f32 - 1.0) / 2.0;
    Vec3::new((i % n) as f32 - off, TOP, (i / n) as f32 - off)
}

/// Feuille de parchemin low-poly : plan découpé en triangles, légèrement
/// bosselé, avec des facettes bien visibles.
fn parchment_mesh(size: f32, seed: u64) -> Mesh {
    let k = 12;
    let mut rng = Rng::new(seed);
    let mut heights = vec![0.0f32; (k + 1) * (k + 1)];
    for (idx, h) in heights.iter_mut().enumerate() {
        let (x, z) = (idx % (k + 1), idx / (k + 1));
        let edge = x == 0 || z == 0 || x == k || z == k;
        *h = rng.below(100) as f32 / 100.0 * if edge { 0.06 } else { 0.035 };
    }
    let p = |x: usize, z: usize| {
        let s = size / k as f32;
        [x as f32 * s - size / 2.0, heights[z * (k + 1) + x], z as f32 * s - size / 2.0]
    };
    let mut positions = Vec::new();
    for z in 0..k {
        for x in 0..k {
            let (a, b, c, d) = (p(x, z), p(x + 1, z), p(x, z + 1), p(x + 1, z + 1));
            positions.extend_from_slice(&[a, c, b, b, c, d]);
        }
    }
    // Chaque triangle a ses propres sommets : facettes bien nettes.
    Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default())
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
        .with_computed_flat_normals()
}

fn put(c: &mut Commands, mesh: &Handle<Mesh>, mat: &Handle<StandardMaterial>, pos: Vec3, scale: Vec3, rot: Quat) -> Entity {
    c.spawn((Mesh3d(mesh.clone()), MeshMaterial3d(mat.clone()), Transform { translation: pos, rotation: rot, scale })).id()
}

/// Décor fixe : table en bois, objets du concile, chandelle.
pub fn spawn_hall(commands: &mut Commands, look: &Look, mats: &mut Assets<StandardMaterial>) {
    let mut m = |r, g, b| mats.add(flat_material(Color::srgb(r, g, b)));
    let wood = m(0.45, 0.28, 0.16);
    let wood_dark = m(0.36, 0.22, 0.12);
    let glass = m(0.18, 0.20, 0.28);
    let feather = m(0.96, 0.95, 0.92);
    let candle = m(0.95, 0.91, 0.80);
    let brass = m(0.78, 0.62, 0.30);
    let sand = m(0.93, 0.80, 0.52);
    let flame = mats.add(StandardMaterial { emissive: LinearRgba::rgb(8.0, 4.0, 1.0), ..flat_material(Color::srgb(1.0, 0.75, 0.3)) });
    let lens = mats.add(StandardMaterial {
        base_color: Color::srgba(0.85, 0.95, 1.0, 0.25),
        alpha_mode: AlphaMode::Blend,
        perceptual_roughness: 0.1,
        ..default()
    });

    let q = Quat::IDENTITY;
    // Table : grandes planches
    for k in -6..=6 {
        let mat = if k % 2 == 0 { &wood } else { &wood_dark };
        let e = put(commands, &look.cube, mat, Vec3::new(0.0, -0.35, k as f32 * 2.0), Vec3::new(40.0, 0.5, 1.96), q);
        commands.entity(e).insert(NotShadowCaster);
    }
    // Encrier et plume
    let ink_pos = Vec3::new(-8.2, 0.0, -3.5);
    put(commands, &look.cyl, &glass, ink_pos + Vec3::Y * 0.45, Vec3::new(1.1, 0.9, 1.1), q);
    put(commands, &look.cyl, &glass, ink_pos + Vec3::Y * 1.0, Vec3::new(0.5, 0.25, 0.5), q);
    let tilt = Quat::from_rotation_z(-0.5) * Quat::from_rotation_y(0.4);
    put(commands, &look.cone, &feather, ink_pos + Vec3::new(0.7, 1.9, 0.0), Vec3::new(0.35, 2.4, 0.12), tilt);
    // Chandelle allumée
    let c = Vec3::new(8.0, 0.0, -3.0);
    put(commands, &look.cyl, &brass, c + Vec3::Y * 0.08, Vec3::new(1.4, 0.16, 1.4), q);
    put(commands, &look.cyl, &candle, c + Vec3::Y * 1.0, Vec3::new(0.45, 1.8, 0.45), q);
    commands.spawn((
        Mesh3d(look.cone.clone()),
        MeshMaterial3d(flame),
        Transform::from_translation(c + Vec3::Y * 2.1).with_scale(Vec3::new(0.22, 0.45, 0.22)),
        NotShadowCaster,
        Flame,
    ));
    commands.spawn((
        PointLight { color: Color::srgb(1.0, 0.75, 0.45), intensity: 120_000.0, range: 20.0, shadow_maps_enabled: false, ..default() },
        Transform::from_translation(c + Vec3::Y * 2.6),
        Flame,
    ));
    // Sablier
    let h = Vec3::new(8.3, 0.0, 3.2);
    put(commands, &look.cube, &wood_dark, h + Vec3::Y * 0.05, Vec3::new(1.2, 0.1, 1.2), q);
    put(commands, &look.cube, &wood_dark, h + Vec3::Y * 1.95, Vec3::new(1.2, 0.1, 1.2), q);
    put(commands, &look.cone, &glass, h + Vec3::Y * 0.55, Vec3::new(0.9, 0.9, 0.9), Quat::from_rotation_x(PI));
    put(commands, &look.cone, &glass, h + Vec3::Y * 1.45, Vec3::new(0.9, 0.9, 0.9), q);
    put(commands, &look.cone, &sand, h + Vec3::Y * 0.3, Vec3::new(0.6, 0.4, 0.6), q);
    for (x, z) in [(-0.5, -0.5), (0.5, -0.5), (-0.5, 0.5), (0.5, 0.5)] {
        put(commands, &look.cyl, &wood_dark, h + Vec3::new(x, 1.0, z), Vec3::new(0.08, 1.9, 0.08), q);
    }
    // Loupe
    let l = Vec3::new(-8.0, 0.05, 3.4);
    put(commands, &look.cyl, &brass, l, Vec3::new(1.8, 0.08, 1.8), q);
    put(commands, &look.cyl, &lens, l + Vec3::Y * 0.03, Vec3::new(1.55, 0.06, 1.55), q);
    put(commands, &look.cyl, &wood, l + Vec3::new(1.5, 0.0, 0.6), Vec3::new(0.18, 1.4, 0.18), Quat::from_rotation_z(FRAC_PI_2) * Quat::from_rotation_x(-0.4));
}

/// Construit la carte d'une grille : parchemin, diocèses, frontières, cases.
pub fn spawn_map(commands: &mut Commands, look: &Look, meshes: &mut Assets<Mesh>, mats: &mut Assets<StandardMaterial>, n: usize, regions: &[u8]) {
    let size = n as f32 + 2.2;
    commands.spawn((
        Mesh3d(meshes.add(parchment_mesh(size, n as u64 * 31 + regions[0] as u64))),
        MeshMaterial3d(look.parchment.clone()),
        Transform::from_xyz(0.0, -0.05, 0.0),
        MapPart,
    ));
    let tiles: Vec<Handle<StandardMaterial>> = WATERCOLORS.iter().map(|&c| mats.add(flat_material(c))).collect();
    let region = |i: usize| regions[i];
    for i in 0..n * n {
        let p = cell_pos(n, i);
        commands.spawn((
                Mesh3d(look.cube.clone()),
                MeshMaterial3d(tiles[region(i) as usize % tiles.len()].clone()),
                Transform::from_translation(p - Vec3::Y * 0.04).with_scale(Vec3::new(1.0, 0.08, 1.0)),
                NotShadowCaster,
                MapPart,
            ))
;
        spawn_marks(commands, look, p, i);
    }
    // Frontières à l'encre entre diocèses, pointillés à l'intérieur.
    let edge = |commands: &mut Commands, a: Vec3, b: Vec3, thick: bool| {
        let mid = (a + b) / 2.0 + Vec3::Y * 0.012;
        let len = a.distance(b);
        let horizontal = (b - a).x.abs() > 0.5;
        if thick {
            let scale = if horizontal { Vec3::new(len + 0.08, 0.025, 0.08) } else { Vec3::new(0.08, 0.025, len + 0.08) };
            commands.spawn((Mesh3d(look.cube.clone()), MeshMaterial3d(look.ink.clone()), Transform::from_translation(mid).with_scale(scale), NotShadowCaster, MapPart));
        } else {
            for k in 0..3 {
                let t = (k as f32 + 0.5) / 3.0;
                let pos = a.lerp(b, t) + Vec3::Y * 0.01;
                let scale = if horizontal { Vec3::new(0.14, 0.012, 0.025) } else { Vec3::new(0.025, 0.012, 0.14) };
                commands.spawn((Mesh3d(look.cube.clone()), MeshMaterial3d(look.ink_pale.clone()), Transform::from_translation(pos).with_scale(scale), NotShadowCaster, MapPart));
            }
        }
    };
    let off = n as f32 / 2.0;
    let corner = |x: usize, z: usize| Vec3::new(x as f32 - off, TOP, z as f32 - off);
    for r in 0..n {
        for c in 0..n {
            let i = r * n + c;
            // bord droit et bord bas de chaque case
            if c + 1 < n {
                edge(commands, corner(c + 1, r), corner(c + 1, r + 1), region(i) != region(i + 1));
            }
            if r + 1 < n {
                edge(commands, corner(c, r + 1), corner(c + 1, r + 1), region(i) != region(i + n));
            }
        }
    }
    // Cadre de la carte
    for (a, b) in [(corner(0, 0), corner(n, 0)), (corner(0, n), corner(n, n)), (corner(0, 0), corner(0, n)), (corner(n, 0), corner(n, n))] {
        edge(commands, a, b, true);
    }
    // Rose des vents dessinée dans un coin du parchemin
    let rose = Vec3::new(off + 0.55, TOP - 0.03, -off - 0.55);
    for k in 0..4 {
        let rot = Quat::from_rotation_y(k as f32 * FRAC_PI_2);
        let mat = if k == 0 { &look.wax } else { &look.ink };
        commands.spawn((
            Mesh3d(look.cone.clone()),
            MeshMaterial3d(mat.clone()),
            Transform::from_translation(rose + rot * Vec3::new(0.0, 0.0, -0.22))
                .with_rotation(rot * Quat::from_rotation_x(-FRAC_PI_2))
                .with_scale(Vec3::new(0.14, 0.4, 0.03)),
            NotShadowCaster,
            MapPart,
        ));
    }
}

/// Les marques possibles d'une case (cachées par défaut) : croix, hachures,
/// sceau d'évêque, surbrillance d'indice.
fn spawn_marks(commands: &mut Commands, look: &Look, p: Vec3, i: usize) {
    let mark = |commands: &mut Commands, kind: MarkKind, root_pos: Vec3| {
        commands.spawn((Transform::from_translation(root_pos), Visibility::Hidden, kind, CellView(i), MapPart)).id()
    };
    // Croix griffonnée à la plume
    let cross = mark(commands, MarkKind::Cross, p);
    for a in [FRAC_PI_4, -FRAC_PI_4] {
        commands.spawn((
            Mesh3d(look.cube.clone()),
            MeshMaterial3d(look.ink.clone()),
            Transform::from_xyz(0.0, 0.015, 0.0).with_rotation(Quat::from_rotation_y(a)).with_scale(Vec3::new(0.5, 0.02, 0.06)),
            NotShadowCaster,
            ChildOf(cross),
        ));
    }
    // Hachures pâles (cases exclues automatiquement en mode Novice)
    let hatch = mark(commands, MarkKind::Hatch, p);
    for k in -1..=1 {
        commands.spawn((
            Mesh3d(look.cube.clone()),
            MeshMaterial3d(look.ink_pale.clone()),
            Transform::from_xyz(k as f32 * 0.22, 0.008, k as f32 * 0.22)
                .with_rotation(Quat::from_rotation_y(FRAC_PI_4))
                .with_scale(Vec3::new(0.035, 0.012, 0.75 - (k as f32).abs() * 0.3)),
            NotShadowCaster,
            ChildOf(hatch),
        ));
    }
    // Sceau de cire rouge frappé d'une mitre
    let seal = mark(commands, MarkKind::Seal, p);
    commands.spawn((Mesh3d(look.cyl.clone()), MeshMaterial3d(look.wax.clone()), Transform::from_xyz(0.0, 0.07, 0.0).with_scale(Vec3::new(0.78, 0.14, 0.78)), SealBody, ChildOf(seal)));
    commands.spawn((Mesh3d(look.cube.clone()), MeshMaterial3d(look.wax_relief.clone()), Transform::from_xyz(0.0, 0.15, 0.05).with_scale(Vec3::new(0.3, 0.03, 0.26)), ChildOf(seal)));
    commands.spawn((
        Mesh3d(look.cone.clone()),
        MeshMaterial3d(look.wax_relief.clone()),
        Transform::from_xyz(0.0, 0.15, -0.14).with_rotation(Quat::from_rotation_x(-FRAC_PI_2)).with_scale(Vec3::new(0.3, 0.2, 0.03)),
        ChildOf(seal),
    ));
    commands.spawn((Mesh3d(look.cube.clone()), MeshMaterial3d(look.gold.clone()), Transform::from_xyz(0.0, 0.17, -0.02).with_scale(Vec3::new(0.035, 0.02, 0.26)), ChildOf(seal)));
    commands.spawn((Mesh3d(look.cube.clone()), MeshMaterial3d(look.gold.clone()), Transform::from_xyz(0.0, 0.17, -0.06).with_scale(Vec3::new(0.16, 0.02, 0.035)), ChildOf(seal)));
    // Surbrillance d'indice : un cadre doré autour de la case
    let hint = mark(commands, MarkKind::Hint, p);
    for (x, z, sx, sz) in [(0.0, -0.46, 0.96, 0.06), (0.0, 0.46, 0.96, 0.06), (-0.46, 0.0, 0.06, 0.96), (0.46, 0.0, 0.06, 0.96)] {
        commands.spawn((Mesh3d(look.cube.clone()), MeshMaterial3d(look.hint.clone()), Transform::from_xyz(x, 0.03, z).with_scale(Vec3::new(sx, 0.04, sz)), NotShadowCaster, ChildOf(hint)));
    }
}

pub fn flicker(time: Res<Time>, mut q: Query<(&mut Transform, Option<&mut PointLight>), With<Flame>>) {
    let t = time.elapsed_secs();
    let k = 1.0 + (t * 11.0).sin() * 0.06 + (t * 23.0).sin() * 0.04;
    for (mut tf, light) in &mut q {
        match light {
            Some(mut l) => l.intensity = 120_000.0 * k,
            None => tf.scale = Vec3::new(0.22, 0.45 * k, 0.22),
        }
    }
}
