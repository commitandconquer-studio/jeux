//! Le bureau d'Eurêka : feuille quadrillée, plateau, objets de laboratoire.
use std::f32::consts::{FRAC_PI_2, PI, TAU};

use bevy::light::NotShadowCaster;
use bevy::prelude::*;
use game_common::{flat_material, iso_projection};

use crate::GameState;
use crate::grid::SIZE;

/// Écart entre deux cases du plateau.
pub const CELL: f32 = 1.15;
const LOOK: Vec3 = Vec3::new(0.0, 0.0, 0.4);
const CAM_OFFSET: Vec3 = Vec3::new(0.0, 12.0, 5.0);

pub struct DeskPlugin;

impl Plugin for DeskPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::srgb(0.93, 0.91, 0.86)))
            .add_systems(Startup, setup)
            .add_systems(Update, (spin_atom, follow_camera));
    }
}

/// Case du plateau → position dans le monde (dessus du plateau en y = 0).
pub fn cell_pos(p: IVec2) -> Vec3 {
    let off = (SIZE - 1) as f32 / 2.0;
    Vec3::new((p.x as f32 - off) * CELL, 0.0, (p.y as f32 - off) * CELL)
}

#[derive(Component)]
struct Atom;

#[derive(Component)]
struct Electron {
    ring: f32,
    speed: f32,
}

fn put(c: &mut Commands, mesh: &Handle<Mesh>, mat: &Handle<StandardMaterial>, pos: Vec3, scale: Vec3, rot: Quat) -> Entity {
    c.spawn((Mesh3d(mesh.clone()), MeshMaterial3d(mat.clone()), Transform { translation: pos, rotation: rot, scale })).id()
}

fn faceted(mesh: Mesh) -> Mesh {
    mesh.with_duplicated_vertices().with_computed_flat_normals()
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut mats: ResMut<Assets<StandardMaterial>>) {
    commands.spawn((Camera3d::default(), iso_projection(14.0, 9.5), Transform::from_translation(LOOK + CAM_OFFSET).looking_at(LOOK, Vec3::Y)));

    let cube = meshes.add(Cuboid::new(1.0, 1.0, 1.0));
    let cyl = meshes.add(Cylinder::new(0.5, 1.0).mesh().resolution(24).build());
    let hex = meshes.add(faceted(Cylinder::new(0.5, 1.0).mesh().resolution(6).build()));
    let cone = meshes.add(Cone { radius: 0.5, height: 1.0 }.mesh().resolution(24).build());
    let ball = meshes.add(Sphere::new(0.5).mesh().uv(20, 14));
    let ring = meshes.add(Torus::new(0.96, 1.0).mesh().minor_resolution(8).major_resolution(48).build());
    let mut m = |r, g, b| mats.add(flat_material(Color::srgb(r, g, b)));
    let paper = m(0.98, 0.97, 0.92);
    let line = m(0.62, 0.74, 0.92);
    let margin = m(0.92, 0.45, 0.45);
    let tray = m(0.78, 0.83, 0.92);
    let slot = m(0.87, 0.90, 0.96);
    let wood = m(0.92, 0.80, 0.55);
    let ink = m(0.25, 0.25, 0.30);
    let yellow = m(0.98, 0.80, 0.20);
    let pink = m(0.98, 0.62, 0.66);
    let metal = m(0.75, 0.76, 0.80);
    let red = m(0.90, 0.30, 0.30);
    let blue = m(0.30, 0.45, 0.90);
    let glass = mats.add(StandardMaterial {
        base_color: Color::srgba(0.80, 0.92, 1.0, 0.30),
        alpha_mode: AlphaMode::Blend,
        perceptual_roughness: 0.1,
        reflectance: 0.6,
        ..default()
    });
    let potion = mats.add(StandardMaterial {
        emissive: LinearRgba::rgb(0.2, 0.9, 0.4),
        ..flat_material(Color::srgb(0.35, 0.90, 0.45))
    });
    let orbit = mats.add(StandardMaterial { emissive: LinearRgba::rgb(0.3, 0.6, 1.2), ..flat_material(Color::srgb(0.45, 0.65, 1.0)) });

    let q = Quat::IDENTITY;

    // Feuille de papier quadrillé, avec sa marge rouge.
    let e = put(&mut commands, &cube, &paper, Vec3::new(0.0, -0.2, 0.0), Vec3::new(44.0, 0.1, 32.0), q);
    commands.entity(e).insert(NotShadowCaster);
    for i in -22..=22 {
        let e = put(&mut commands, &cube, &line, Vec3::new(i as f32 * 0.8, -0.149, 0.0), Vec3::new(0.025, 0.002, 32.0), q);
        commands.entity(e).insert(NotShadowCaster);
    }
    for j in -16..=16 {
        let e = put(&mut commands, &cube, &line, Vec3::new(0.0, -0.149, j as f32 * 0.8), Vec3::new(44.0, 0.002, 0.025), q);
        commands.entity(e).insert(NotShadowCaster);
    }
    let e = put(&mut commands, &cube, &margin, Vec3::new(-7.6, -0.147, 0.0), Vec3::new(0.06, 0.002, 32.0), q);
    commands.entity(e).insert(NotShadowCaster);

    // Plateau et ses 16 cases.
    let w = SIZE as f32 * CELL + 0.35;
    put(&mut commands, &cube, &tray, Vec3::new(0.0, -0.15, 0.0), Vec3::new(w, 0.3, w), q);
    for y in 0..SIZE {
        for x in 0..SIZE {
            let e = put(&mut commands, &cube, &slot, cell_pos(IVec2::new(x, y)) - Vec3::Y * 0.005, Vec3::new(1.0, 0.01, 1.0), q);
            commands.entity(e).insert(NotShadowCaster);
        }
    }

    // Fiole d'Erlenmeyer : verre, potion verte lumineuse.
    let flask = Vec3::new(-5.4, -0.15, -1.6);
    put(&mut commands, &cone, &potion, flask + Vec3::Y * 0.45, Vec3::new(1.45, 0.9, 1.45), q);
    put(&mut commands, &cone, &glass, flask + Vec3::Y * 0.8, Vec3::new(1.7, 1.6, 1.7), q);
    put(&mut commands, &cyl, &glass, flask + Vec3::Y * 1.85, Vec3::new(0.5, 0.7, 0.5), q);

    // Crayon : corps hexagonal, bois taillé, mine, gomme et bague métallique.
    let pencil = commands.spawn((Transform::from_xyz(4.6, 0.05, 2.7).with_rotation(Quat::from_rotation_y(0.35)), Visibility::default())).id();
    let along_x = Quat::from_rotation_z(FRAC_PI_2);
    for (mesh, mat, x, s) in [
        (&hex, &yellow, 0.0, Vec3::new(0.32, 3.2, 0.32)),
        (&cone, &wood, -1.85, Vec3::new(0.32, 0.5, 0.32)),
        (&cone, &ink, -2.05, Vec3::new(0.12, 0.18, 0.12)),
        (&cyl, &metal, 1.72, Vec3::new(0.33, 0.25, 0.33)),
        (&cyl, &pink, 1.98, Vec3::new(0.3, 0.3, 0.3)),
    ] {
        let rot = if mesh == &cone { Quat::from_rotation_z(FRAC_PI_2) } else { along_x };
        commands.spawn((Mesh3d(mesh.clone()), MeshMaterial3d(mat.clone()), Transform { translation: Vec3::new(x, 0.16, 0.0), rotation: rot, scale: s }, ChildOf(pencil)));
    }

    // Règle graduée.
    let ruler = commands.spawn((Transform::from_xyz(-4.9, -0.12, 2.9).with_rotation(Quat::from_rotation_y(-0.2)), Visibility::default())).id();
    commands.spawn((Mesh3d(cube.clone()), MeshMaterial3d(wood.clone()), Transform::from_scale(Vec3::new(5.0, 0.06, 0.8)), ChildOf(ruler)));
    for k in 0..=20 {
        let long = if k % 4 == 0 { 0.3 } else { 0.15 };
        commands.spawn((
            Mesh3d(cube.clone()),
            MeshMaterial3d(ink.clone()),
            Transform::from_xyz(-2.4 + k as f32 * 0.24, 0.035, -0.4 + long / 2.0).with_scale(Vec3::new(0.025, 0.01, long)),
            ChildOf(ruler),
        ));
    }

    // Gomme.
    put(&mut commands, &cube, &pink, Vec3::new(-6.2, 0.0, 0.7), Vec3::new(0.9, 0.35, 0.5), Quat::from_rotation_y(0.4));

    // Atome : noyau de protons et neutrons, trois orbites, électrons qui tournent.
    let atom = commands.spawn((Transform::from_xyz(5.4, 1.5, -1.6), Visibility::default(), Atom)).id();
    for (i, p) in [Vec3::new(0.15, 0.1, 0.0), Vec3::new(-0.15, 0.05, 0.1), Vec3::new(0.0, -0.12, -0.1), Vec3::new(0.05, 0.15, -0.12), Vec3::new(-0.1, -0.1, 0.12)].iter().enumerate() {
        let mat = if i % 2 == 0 { &red } else { &blue };
        commands.spawn((Mesh3d(ball.clone()), MeshMaterial3d(mat.clone()), Transform::from_translation(*p).with_scale(Vec3::splat(0.3)), ChildOf(atom)));
    }
    for k in 0..3 {
        let tilt = Quat::from_rotation_y(k as f32 * PI / 3.0) * Quat::from_rotation_x(1.2);
        commands.spawn((Mesh3d(ring.clone()), MeshMaterial3d(orbit.clone()), Transform::from_rotation(tilt).with_scale(Vec3::splat(1.1)), ChildOf(atom)));
        commands.spawn((
            Mesh3d(ball.clone()),
            MeshMaterial3d(orbit.clone()),
            Transform::from_scale(Vec3::splat(0.16)),
            Electron { ring: k as f32, speed: 1.6 + k as f32 * 0.5 },
            ChildOf(atom),
        ));
    }
}

fn spin_atom(time: Res<Time>, mut atoms: Query<&mut Transform, (With<Atom>, Without<Electron>)>, mut electrons: Query<(&Electron, &mut Transform), Without<Atom>>) {
    let t = time.elapsed_secs();
    for mut tf in &mut atoms {
        tf.rotation = Quat::from_rotation_y(t * 0.3);
        tf.translation.y = 1.5 + (t * 1.2).sin() * 0.12;
    }
    for (e, mut tf) in &mut electrons {
        let tilt = Quat::from_rotation_y(e.ring * PI / 3.0) * Quat::from_rotation_x(1.2);
        let a = t * e.speed + e.ring * TAU / 3.0;
        // Le tore est couché dans le plan XZ avant inclinaison.
        tf.translation = tilt * Vec3::new(a.cos(), 0.0, a.sin()) * 1.1;
    }
}

/// Dans le menu, la caméra glisse pour laisser le plateau à droite du panneau.
fn follow_camera(time: Res<Time>, state: Res<State<GameState>>, mut shift: Local<f32>, mut q: Query<&mut Transform, With<Camera3d>>) {
    let side_panel = matches!(state.get(), GameState::Menu | GameState::Won | GameState::Over);
    let target = if side_panel { -3.8 } else { 0.0 };
    *shift += (target - *shift) * (time.delta_secs() * 3.0).min(1.0);
    for mut tf in &mut q {
        let look = LOOK + Vec3::X * *shift;
        *tf = Transform::from_translation(look + CAM_OFFSET).looking_at(look, Vec3::Y);
    }
}
