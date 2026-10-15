//! La tour : bloc qui glisse, coupe de ce qui dépasse, chutes, « Parfait ».
use std::f32::consts::{PI, TAU};

use bevy::prelude::*;
use game_common::{Rng, Score, flat_material, iso_projection};

use crate::GameState;

/// Épaisseur d'un bloc.
pub const BLOCK_H: f32 = 0.5;
/// Taille du bloc de départ.
const BASE: f32 = 3.0;
/// Le bloc glisse de -RANGE à +RANGE.
const RANGE: f32 = 4.5;
/// Écart toléré pour un « Parfait ».
const PERFECT: f32 = 0.12;
/// Après ce nombre de « Parfait » d'affilée, le bloc regrandit.
const GROW_AFTER: u32 = 3;
const CAM_OFFSET: Vec3 = Vec3::new(10.0, 8.5, 10.0);

pub struct TowerPlugin;

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<TowerEvent>()
            .init_resource::<Autopilot>()
            .init_resource::<Shapes>()
            .insert_resource(Tower {
                top_center: Vec2::ZERO,
                top_size: Vec2::splat(BASE),
                level: 0,
                moving: None,
                phase: 0.0,
                axis: Axis::X,
                combo: 0,
                hue0: 200.0,
                cam_y: 1.0,
                zoom: 10.0,
                cam_angle: 0.0,
                cam_shift: 0.0,
            })
            .add_systems(Startup, setup_scene)
            .add_systems(OnEnter(GameState::Menu), (reset, build_demo).chain())
            .add_systems(OnEnter(GameState::Playing), reset)
            .add_systems(
                Update,
                (
                    (spawn_next, slide, place).chain().run_if(in_state(GameState::Playing)),
                    fall,
                    follow_camera,
                    tint_sky,
                ),
            );
    }
}

/// Ce qui vient de se passer sur la tour (pour les sons et l'interface).
#[derive(Message, Clone, Copy)]
pub enum TowerEvent {
    Placed { perfect: bool, combo: u32 },
    Missed,
}

#[derive(Clone, Copy, PartialEq)]
enum Axis {
    X,
    Z,
}

impl Axis {
    fn get(self, v: Vec2) -> f32 {
        match self {
            Axis::X => v.x,
            Axis::Z => v.y,
        }
    }

    fn set(self, v: &mut Vec2, value: f32) {
        match self {
            Axis::X => v.x = value,
            Axis::Z => v.y = value,
        }
    }

    fn unit(self) -> Vec3 {
        match self {
            Axis::X => Vec3::X,
            Axis::Z => Vec3::Z,
        }
    }
}

#[derive(Resource)]
pub struct Tower {
    /// Centre (x, z) et taille (x, z) du bloc du dessus.
    top_center: Vec2,
    top_size: Vec2,
    /// Nombre de blocs posés au-dessus du socle.
    pub level: u32,
    moving: Option<Entity>,
    /// Avancement du bloc qui glisse (aller-retour).
    phase: f32,
    axis: Axis,
    pub combo: u32,
    hue0: f32,
    cam_y: f32,
    zoom: f32,
    cam_angle: f32,
    cam_shift: f32,
}

/// Pour le mode captures : pose automatiquement quand l'écart vaut la cible.
#[derive(Resource, Default)]
pub struct Autopilot(pub Option<f32>);

#[derive(Component)]
struct Block;

#[derive(Component)]
struct Moving;

#[derive(Component)]
struct Falling {
    vel: Vec3,
    spin: Vec3,
}

#[derive(Resource)]
struct Shapes {
    cube: Handle<Mesh>,
}

impl FromWorld for Shapes {
    fn from_world(world: &mut World) -> Self {
        Shapes { cube: world.resource_mut::<Assets<Mesh>>().add(Cuboid::new(1.0, 1.0, 1.0)) }
    }
}

fn block_color(hue0: f32, level: u32) -> Color {
    Color::hsl((hue0 + level as f32 * 7.0) % 360.0, 0.55, 0.62)
}

fn block_transform(center: Vec2, size: Vec2, level: u32) -> Transform {
    Transform::from_xyz(center.x, level as f32 * BLOCK_H + BLOCK_H / 2.0, center.y)
        .with_scale(Vec3::new(size.x, BLOCK_H, size.y))
}

fn faceted(mesh: Mesh) -> Mesh {
    mesh.with_duplicated_vertices().with_computed_flat_normals()
}

/// Caméra, et petite île flottante sur laquelle repose la tour.
fn setup_scene(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut mats: ResMut<Assets<StandardMaterial>>) {
    commands.spawn((Camera3d::default(), iso_projection(10.0, 10.0), Transform::from_translation(CAM_OFFSET)));

    let disc = meshes.add(faceted(Cylinder::new(0.5, 1.0).mesh().resolution(9).build()));
    let cone = meshes.add(faceted(Cone { radius: 0.5, height: 1.0 }.mesh().resolution(9).build()));
    let blob = meshes.add(faceted(Sphere::new(0.5).mesh().ico(1).unwrap()));
    let mut mat = |r, g, b| mats.add(flat_material(Color::srgb(r, g, b)));
    let (grass, dirt, rock, leaf, leaf2, pine, trunk) = (
        mat(0.55, 0.80, 0.40),
        mat(0.55, 0.40, 0.30),
        mat(0.45, 0.35, 0.30),
        mat(0.40, 0.70, 0.35),
        mat(0.55, 0.78, 0.35),
        mat(0.22, 0.50, 0.35),
        mat(0.50, 0.35, 0.24),
    );
    let mut put = |mesh: &Handle<Mesh>, m: &Handle<StandardMaterial>, pos: Vec3, scale: Vec3, rot: Quat| {
        commands.spawn((
            Mesh3d(mesh.clone()),
            MeshMaterial3d(m.clone()),
            Transform { translation: pos, rotation: rot, scale },
        ));
    };
    // Île : dessus d'herbe, terre en cône renversé, rochers
    put(&disc, &grass, Vec3::new(0.0, -0.25, 0.0), Vec3::new(10.0, 0.5, 10.0), Quat::IDENTITY);
    put(&cone, &dirt, Vec3::new(0.0, -2.5, 0.0), Vec3::new(9.6, 4.0, 9.6), Quat::from_rotation_x(PI));
    put(&cone, &rock, Vec3::new(1.2, -4.6, 0.8), Vec3::new(3.0, 2.0, 3.0), Quat::from_rotation_x(PI));
    // Buissons et sapins autour du socle
    for i in 0..9 {
        let a = i as f32 / 9.0 * TAU + 0.3;
        let r = 3.6 + (i % 3) as f32 * 0.4;
        let p = Vec3::new(a.cos() * r, 0.0, a.sin() * r);
        if i % 3 == 0 {
            put(&disc, &trunk, p + Vec3::Y * 0.2, Vec3::new(0.18, 0.4, 0.18), Quat::IDENTITY);
            put(&cone, &pine, p + Vec3::Y * 0.85, Vec3::new(0.9, 1.1, 0.9), Quat::IDENTITY);
        } else {
            let m = if i % 2 == 0 { &leaf } else { &leaf2 };
            put(&blob, m, p + Vec3::Y * 0.25, Vec3::new(0.8, 0.6, 0.8), Quat::from_rotation_y(a));
        }
    }
}

/// Repart de zéro : seul le socle reste.
fn reset(
    mut commands: Commands,
    mut tower: ResMut<Tower>,
    mut score: ResMut<Score>,
    mut rng: ResMut<Rng>,
    shapes: Res<Shapes>,
    mut mats: ResMut<Assets<StandardMaterial>>,
    blocks: Query<Entity, Or<(With<Block>, With<Falling>)>>,
) {
    for e in &blocks {
        commands.entity(e).despawn();
    }
    tower.top_center = Vec2::ZERO;
    tower.top_size = Vec2::splat(BASE);
    tower.level = 0;
    tower.moving = None;
    tower.combo = 0;
    tower.axis = Axis::X;
    tower.hue0 = rng.below(360) as f32;
    score.reset();
    commands.spawn((
        Mesh3d(shapes.cube.clone()),
        MeshMaterial3d(mats.add(flat_material(block_color(tower.hue0, 0)))),
        block_transform(Vec2::ZERO, tower.top_size, 0),
        Block,
    ));
}

/// Dans le menu : une petite tour déjà construite, pour donner envie.
fn build_demo(
    mut commands: Commands,
    mut tower: ResMut<Tower>,
    mut rng: ResMut<Rng>,
    shapes: Res<Shapes>,
    mut mats: ResMut<Assets<StandardMaterial>>,
) {
    let mut size = Vec2::splat(BASE);
    let mut center = Vec2::ZERO;
    for level in 1..=12 {
        let axis = if level % 2 == 1 { Axis::X } else { Axis::Z };
        if rng.below(3) == 0 {
            let cut = 0.1 + rng.below(25) as f32 / 100.0;
            let len = axis.get(size) - cut;
            axis.set(&mut size, len);
            let c = axis.get(center) + if rng.below(2) == 0 { cut / 2.0 } else { -cut / 2.0 };
            axis.set(&mut center, c);
        }
        commands.spawn((
            Mesh3d(shapes.cube.clone()),
            MeshMaterial3d(mats.add(flat_material(block_color(tower.hue0, level)))),
            block_transform(center, size, level),
            Block,
        ));
    }
    tower.level = 12;
}

fn slide_offset(phase: f32) -> f32 {
    let p = phase.rem_euclid(4.0 * RANGE);
    if p < 2.0 * RANGE { -RANGE + p } else { 3.0 * RANGE - p }
}

/// Fait apparaître le bloc suivant, au bord, sur l'axe suivant.
fn spawn_next(
    mut commands: Commands,
    mut tower: ResMut<Tower>,
    shapes: Res<Shapes>,
    mut mats: ResMut<Assets<StandardMaterial>>,
) {
    if tower.moving.is_some() {
        return;
    }
    let level = tower.level + 1;
    tower.axis = if level % 2 == 1 { Axis::X } else { Axis::Z };
    tower.phase = 0.0;
    let mut center = tower.top_center;
    tower.axis.set(&mut center, -RANGE);
    let e = commands
        .spawn((
            Mesh3d(shapes.cube.clone()),
            MeshMaterial3d(mats.add(flat_material(block_color(tower.hue0, level)))),
            block_transform(center, tower.top_size, level),
            Block,
            Moving,
        ))
        .id();
    tower.moving = Some(e);
}

fn speed(level: u32) -> f32 {
    (3.0 + level as f32 * 0.06).min(7.0)
}

fn slide(time: Res<Time>, mut tower: ResMut<Tower>, mut q: Query<&mut Transform, With<Moving>>) {
    tower.phase += speed(tower.level) * time.delta_secs();
    let offset = slide_offset(tower.phase);
    let mut center = tower.top_center;
    tower.axis.set(&mut center, offset);
    for mut tf in &mut q {
        tf.translation.x = center.x;
        tf.translation.z = center.y;
    }
}

/// Pose le bloc : on garde la partie au-dessus de la tour, le reste tombe.
#[allow(clippy::too_many_arguments)]
fn place(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    touches: Res<Touches>,
    autopilot: Res<Autopilot>,
    mut tower: ResMut<Tower>,
    mut score: ResMut<Score>,
    shapes: Res<Shapes>,
    mut q: Query<(&mut Transform, &MeshMaterial3d<StandardMaterial>), With<Moving>>,
    mut events: MessageWriter<TowerEvent>,
    mut next: ResMut<NextState<GameState>>,
) {
    let Some(e) = tower.moving else {
        return;
    };
    let Ok((mut tf, mat)) = q.get_mut(e) else {
        return;
    };
    let axis = tower.axis;
    let prev = axis.get(tower.top_center);
    let d = slide_offset(tower.phase) - prev;

    let pressed = keys.just_pressed(KeyCode::Space)
        || mouse.just_pressed(MouseButton::Left)
        || touches.any_just_pressed()
        || autopilot.0.is_some_and(|target| (d - target).abs() < 0.05);
    if !pressed {
        return;
    }

    let len = axis.get(tower.top_size);
    let mut center = tower.top_center;
    let mut size = tower.top_size;
    let perfect = d.abs() < PERFECT;
    if perfect {
        tower.combo += 1;
        if tower.combo >= GROW_AFTER {
            axis.set(&mut size, (len + 0.15).min(BASE));
        }
    } else {
        tower.combo = 0;
        let overlap = len - d.abs();
        if overlap <= 0.0 {
            // Raté : le bloc entier tombe.
            commands.entity(e).remove::<Moving>().insert(Falling { vel: axis.unit() * d.signum() * 1.5, spin: Vec3::new(1.2, 0.0, 0.8) });
            tower.moving = None;
            events.write(TowerEvent::Missed);
            next.set(GameState::GameOver);
            return;
        }
        axis.set(&mut size, overlap);
        axis.set(&mut center, prev + d / 2.0);
        // Morceau coupé qui tombe
        let mut cut_center = center;
        axis.set(&mut cut_center, prev + d / 2.0 + d.signum() * (overlap + d.abs()) / 2.0);
        let mut cut_size = size;
        axis.set(&mut cut_size, d.abs());
        commands.spawn((
            Mesh3d(shapes.cube.clone()),
            MeshMaterial3d(mat.0.clone()),
            block_transform(cut_center, cut_size, tower.level + 1),
            Falling { vel: axis.unit() * d.signum() * 1.2, spin: axis.unit().cross(Vec3::Y) * -d.signum() * 2.0 },
        ));
    }

    tower.level += 1;
    *tf = block_transform(center, size, tower.level);
    commands.entity(e).remove::<Moving>();
    tower.top_center = center;
    tower.top_size = size;
    tower.moving = None;
    score.current = tower.level;
    score.best = score.best.max(score.current);
    events.write(TowerEvent::Placed { perfect, combo: tower.combo });
}

fn fall(mut commands: Commands, time: Res<Time>, mut q: Query<(Entity, &mut Falling, &mut Transform)>) {
    let dt = time.delta_secs();
    for (e, mut f, mut tf) in &mut q {
        f.vel.y -= 18.0 * dt;
        tf.translation += f.vel * dt;
        tf.rotate(Quat::from_scaled_axis(f.spin * dt));
        if tf.translation.y < -40.0 {
            commands.entity(e).despawn();
        }
    }
}

/// La caméra monte avec la tour ; tourne doucement dans le menu ;
/// recule pour montrer toute la tour à la fin.
fn follow_camera(
    time: Res<Time>,
    state: Res<State<GameState>>,
    mut tower: ResMut<Tower>,
    mut q: Query<(&mut Transform, &mut Projection), With<Camera3d>>,
) {
    let dt = time.delta_secs();
    let top = tower.level as f32 * BLOCK_H;
    let (target_y, zoom, shift_x) = match state.get() {
        // Fin : toute la tour, remontée dans l'image pour laisser la place au panneau du bas.
        GameState::GameOver => {
            let zoom = (top * 1.25 + 8.0).max(12.0);
            (top * 0.5 - zoom * 0.2, zoom, 0.0)
        }
        // Menu : la tour à droite, le panneau à gauche.
        GameState::Menu => {
            let zoom = (top + 6.0).max(10.0);
            (top * 0.5 + 1.0, zoom, zoom * 0.45)
        }
        GameState::Playing => (top + 1.0, 10.0, 0.0),
    };
    tower.cam_shift += (shift_x - tower.cam_shift) * (dt * 3.0).min(1.0);
    tower.cam_y += (target_y - tower.cam_y) * (dt * 3.0).min(1.0);
    tower.zoom += (zoom - tower.zoom) * (dt * 2.5).min(1.0);
    if *state.get() == GameState::Menu {
        tower.cam_angle = (tower.cam_angle + dt * 0.25) % TAU;
    } else {
        let a = (tower.cam_angle + PI).rem_euclid(TAU) - PI;
        tower.cam_angle = a * (1.0 - (dt * 3.0).min(1.0));
    }
    let look = Vec3::Y * tower.cam_y;
    for (mut tf, mut proj) in &mut q {
        *tf = Transform::from_translation(look + Quat::from_rotation_y(tower.cam_angle) * CAM_OFFSET).looking_at(look, Vec3::Y);
        // Décale la caméra vers la gauche : la tour apparaît à droite de l'écran.
        let left = -tf.right().as_vec3();
        tf.translation += left * tower.cam_shift;
        *proj = iso_projection(tower.zoom, tower.zoom);
    }
}

/// Le ciel prend une teinte pâle assortie aux blocs.
fn tint_sky(tower: Res<Tower>, mut clear: ResMut<ClearColor>) {
    if tower.is_changed() {
        clear.0 = Color::hsl((tower.hue0 + tower.level as f32 * 7.0) % 360.0, 0.45, 0.88);
    }
}
