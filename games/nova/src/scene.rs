//! La scène : espace étoilé, planète, croiseur de l'Hégémonie, cadre du
//! bouclier, blocs d'énergie, rayons du canon à ions, débris.
use std::f32::consts::FRAC_PI_2;

use bevy::light::NotShadowCaster;
use bevy::prelude::*;
use game_common::space::{SpaceBackdropPlugin, SpaceView};
use game_common::{Rng, flat_material, iso_projection};

use crate::GameState;
use crate::board::{Cell, HEIGHT, Piece, WIDTH};
use crate::game::{Game, Mode, NovaEvent};
use crate::story::CHAPTERS;

/// Point visé par la caméra.
const LOOK: Vec3 = Vec3::new(0.0, 14.5, 0.0);
const CAM_OFFSET: Vec3 = Vec3::new(7.0, 4.0, 40.0);
/// Position du croiseur au-dessus du bouclier.
const CRUISER_POS: Vec3 = Vec3::new(0.0, 25.5, -5.0);
/// Aperçus : pièces suivantes à droite, réserve à gauche.
pub const NEXT_POS: [Vec3; 3] = [Vec3::new(10.0, 17.0, 0.0), Vec3::new(10.0, 13.0, 0.0), Vec3::new(10.0, 9.0, 0.0)];
pub const HOLD_POS: Vec3 = Vec3::new(-10.0, 17.0, 0.0);

pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Look>()
            .add_plugins(SpaceBackdropPlugin)
            .insert_resource(SpaceView { center: LOOK, size: Vec2::new(64.0, 40.0) })
            .add_systems(Startup, setup)
            .add_systems(
                Update,
                (sync_board, sync_piece, sync_previews, effects, animate_fx, animate_cruiser, chapter_backdrop, shake_camera),
            );
    }
}

/// Formes et matériaux.
#[derive(Resource)]
pub struct Look {
    cube: Handle<Mesh>,
    ball: Handle<Mesh>,
    blocks: Vec<Handle<StandardMaterial>>,
    damaged: Handle<StandardMaterial>,
    ghost: Handle<StandardMaterial>,
    beam: Handle<StandardMaterial>,
    debris: Vec<Handle<StandardMaterial>>,
    fire: Handle<StandardMaterial>,
    hull: Handle<StandardMaterial>,
    planet: Handle<StandardMaterial>,
}

/// Couleurs des blocs d'énergie, une par forme.
pub fn kind_color(kind: u8) -> Color {
    [
        Color::srgb(0.55, 0.85, 1.0),  // I : bleu glacier
        Color::srgb(1.0, 0.78, 0.30),  // O : ambre
        Color::srgb(0.85, 0.45, 0.95), // T : magenta
        Color::srgb(0.55, 0.92, 0.45), // S : vert lime
        Color::srgb(1.0, 0.48, 0.42),  // Z : corail
        Color::srgb(0.55, 0.55, 1.0),  // J : violet
        Color::srgb(0.30, 0.85, 0.75), // L : turquoise
    ][kind as usize % 7]
}

fn glowing(color: Color, glow: f32) -> StandardMaterial {
    StandardMaterial { emissive: color.to_linear() * glow, ..flat_material(color) }
}

impl FromWorld for Look {
    fn from_world(world: &mut World) -> Self {
        let mut meshes = world.resource_mut::<Assets<Mesh>>();
        let cube = meshes.add(Cuboid::new(1.0, 1.0, 1.0));
        let ball = meshes.add(Sphere::new(0.5).mesh().ico(2).unwrap());
        let mut mats = world.resource_mut::<Assets<StandardMaterial>>();
        let blocks = (0..7).map(|k| mats.add(glowing(kind_color(k), 0.35))).collect();
        let debris = (0..7).map(|k| mats.add(glowing(kind_color(k), 2.0))).collect();
        Look {
            cube,
            ball,
            blocks,
            damaged: mats.add(StandardMaterial {
                emissive: LinearRgba::rgb(0.25, 0.02, 0.0),
                ..flat_material(Color::srgb(0.30, 0.30, 0.34))
            }),
            ghost: mats.add(StandardMaterial {
                base_color: Color::srgba(0.8, 0.95, 1.0, 0.18),
                alpha_mode: AlphaMode::Blend,
                unlit: true,
                ..default()
            }),
            beam: mats.add(StandardMaterial {
                base_color: Color::srgb(0.6, 0.95, 1.0),
                emissive: LinearRgba::rgb(3.0, 8.0, 10.0),
                unlit: true,
                ..default()
            }),
            debris,
            fire: mats.add(StandardMaterial {
                base_color: Color::srgb(1.0, 0.4, 0.2),
                emissive: LinearRgba::rgb(10.0, 2.5, 0.5),
                unlit: true,
                ..default()
            }),
            hull: mats.add(flat_material(Color::srgb(0.46, 0.48, 0.54))),
            planet: mats.add(flat_material(CHAPTERS[0].planet)),
        }
    }
}

/// Case du bouclier → position dans le monde.
pub fn cell_pos(p: IVec2) -> Vec3 {
    Vec3::new(p.x as f32 - (WIDTH - 1) as f32 / 2.0, p.y as f32 + 0.5, 0.0)
}

#[derive(Component)]
struct BoardBlock;

#[derive(Component)]
struct ActiveBlock(usize);

#[derive(Component)]
struct GhostBlock(usize);

/// Bloc d'aperçu : emplacement (0..3 = suivantes, 3 = réserve) et numéro.
#[derive(Component)]
struct PreviewBlock(usize, usize);

#[derive(Component)]
pub struct Cruiser;

#[derive(Component)]
struct Planet;

/// Effet temporaire (rayon, débris, explosion) qui disparaît tout seul.
#[derive(Component)]
struct Fx {
    life: f32,
    max: f32,
    vel: Vec3,
    shrink: bool,
}

/// Secousse de la caméra (tir ennemi, explosion).
#[derive(Resource, Default)]
struct Shake(f32);

/// Éclair blanc sur le croiseur quand il est touché.
#[derive(Resource, Default)]
struct HitFlash(f32);

fn setup(mut commands: Commands, look: Res<Look>, mut meshes: ResMut<Assets<Mesh>>, mut mats: ResMut<Assets<StandardMaterial>>) {
    commands.insert_resource(Shake::default());
    commands.insert_resource(HitFlash::default());
    commands.insert_resource(GlobalAmbientLight { color: Color::srgb(0.8, 0.85, 1.0), brightness: 500.0, ..default() });
    // Zone visible : le bouclier, les aperçus, et le croiseur avec de la marge au-dessus.
    commands.spawn((Camera3d::default(), iso_projection(34.0, 36.0), Transform::from_translation(LOOK + CAM_OFFSET).looking_at(LOOK, Vec3::Y)));

    // Planète à défendre (le fond étoilé vient de `game_common::space`)
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(0.5).mesh().ico(4).unwrap())),
        MeshMaterial3d(look.planet.clone()),
        Transform::from_xyz(-22.0, -8.0, -55.0).with_scale(Vec3::splat(34.0)),
        NotShadowCaster,
        Planet,
    ));

    // Cadre du bouclier : fond quadrillé, piliers avec bandes lumineuses, socle.
    let w = WIDTH as f32;
    let h = HEIGHT as f32;
    let back = mats.add(flat_material(Color::srgb(0.05, 0.07, 0.14)));
    let grid = mats.add(StandardMaterial { base_color: Color::srgb(0.2, 0.4, 0.6), emissive: LinearRgba::rgb(0.05, 0.12, 0.2), unlit: true, ..default() });
    let metal = mats.add(flat_material(Color::srgb(0.42, 0.45, 0.52)));
    let strip = mats.add(glowing(Color::srgb(0.3, 0.85, 1.0), 3.0));
    let mut put = |m: &Handle<StandardMaterial>, pos: Vec3, scale: Vec3, shadow: bool| {
        let mut e = commands.spawn((Mesh3d(look.cube.clone()), MeshMaterial3d(m.clone()), Transform::from_translation(pos).with_scale(scale)));
        if !shadow {
            e.insert(NotShadowCaster);
        }
    };
    put(&back, Vec3::new(0.0, h / 2.0, -0.75), Vec3::new(w, h, 0.3), false);
    for x in 0..=WIDTH {
        put(&grid, Vec3::new(x as f32 - w / 2.0, h / 2.0, -0.58), Vec3::new(0.03, h, 0.02), false);
    }
    for y in 0..=HEIGHT {
        put(&grid, Vec3::new(0.0, y as f32, -0.58), Vec3::new(w, 0.03, 0.02), false);
    }
    for s in [-1.0, 1.0] {
        put(&metal, Vec3::new(s * (w / 2.0 + 0.35), h / 2.0, -0.1), Vec3::new(0.7, h + 1.0, 1.6), true);
        put(&strip, Vec3::new(s * (w / 2.0 + 0.35), h / 2.0, 0.71), Vec3::new(0.12, h, 0.02), false);
        // Émetteurs du canon en haut des piliers
        put(&strip, Vec3::new(s * (w / 2.0 + 0.35), h + 0.8, 0.0), Vec3::new(0.9, 0.5, 0.9), false);
    }
    put(&metal, Vec3::new(0.0, -0.4, -0.1), Vec3::new(w + 1.4, 0.8, 1.8), true);
    put(&strip, Vec3::new(0.0, -0.4, 0.81), Vec3::new(w + 1.0, 0.1, 0.02), false);
    // Socles des aperçus
    for pos in NEXT_POS.iter().chain([&HOLD_POS]) {
        put(&back, *pos + Vec3::new(0.0, 0.0, -0.75), Vec3::new(4.6, 3.6, 0.3), false);
        put(&strip, *pos + Vec3::new(0.0, -1.85, -0.6), Vec3::new(4.6, 0.08, 0.05), false);
    }

    // Blocs de la pièce en cours, de son ombre, des aperçus.
    for i in 0..4 {
        commands.spawn((Mesh3d(look.cube.clone()), MeshMaterial3d(look.blocks[0].clone()), Transform::default(), Visibility::Hidden, ActiveBlock(i)));
        commands.spawn((Mesh3d(look.cube.clone()), MeshMaterial3d(look.ghost.clone()), Transform::default(), Visibility::Hidden, GhostBlock(i), NotShadowCaster));
        for slot in 0..4 {
            commands.spawn((Mesh3d(look.cube.clone()), MeshMaterial3d(look.blocks[0].clone()), Transform::default(), Visibility::Hidden, PreviewBlock(slot, i)));
        }
    }

    spawn_cruiser(&mut commands, &look, &mut mats);
}

/// Croiseur de l'Hégémonie : coque massive en marteau, tour de commandement,
/// réacteurs bleus à l'arrière, feux rouges.
fn spawn_cruiser(commands: &mut Commands, look: &Look, mats: &mut Assets<StandardMaterial>) {
    let root = commands.spawn((Transform::from_translation(CRUISER_POS), Visibility::default(), Cruiser)).id();
    let dark = mats.add(flat_material(Color::srgb(0.30, 0.31, 0.36)));
    let window = mats.add(glowing(Color::srgb(1.0, 0.85, 0.5), 2.0));
    let engine = mats.add(glowing(Color::srgb(0.4, 0.7, 1.0), 6.0));
    let red = mats.add(glowing(Color::srgb(1.0, 0.15, 0.1), 5.0));
    let mut part = |m: &Handle<StandardMaterial>, mesh: &Handle<Mesh>, pos: Vec3, scale: Vec3, rot: Quat| {
        commands.spawn((Mesh3d(mesh.clone()), MeshMaterial3d(m.clone()), Transform { translation: pos, rotation: rot, scale }, ChildOf(root)));
    };
    let q = Quat::IDENTITY;
    part(&look.hull, &look.cube, Vec3::ZERO, Vec3::new(15.0, 1.7, 4.0), q);
    part(&dark, &look.cube, Vec3::new(1.5, 1.2, 0.0), Vec3::new(9.0, 1.0, 3.0), q);
    part(&look.hull, &look.cube, Vec3::new(3.5, 2.3, 0.0), Vec3::new(2.4, 1.4, 1.8), q);
    part(&window, &look.cube, Vec3::new(3.5, 2.5, 0.91), Vec3::new(2.0, 0.22, 0.02), q);
    // Proue en marteau
    part(&dark, &look.cube, Vec3::new(-8.0, 0.0, 0.0), Vec3::new(2.2, 3.0, 5.0), q);
    part(&look.hull, &look.cube, Vec3::new(-9.3, 0.0, 0.0), Vec3::new(0.6, 2.2, 4.2), q);
    // Canons sous la coque
    for x in [-4.0, 0.0, 4.0] {
        part(&dark, &look.cube, Vec3::new(x, -1.2, 0.6), Vec3::new(1.4, 0.7, 1.2), q);
        part(&red, &look.cube, Vec3::new(x - 0.8, -1.3, 0.6), Vec3::new(0.5, 0.15, 0.15), q);
    }
    // Réacteurs
    let along_x = Quat::from_rotation_z(FRAC_PI_2);
    for z in [-1.2, 0.0, 1.2] {
        part(&dark, &look.cube, Vec3::new(8.0, 0.0, z), Vec3::new(1.2, 1.0, 1.0), along_x);
        part(&engine, &look.ball, Vec3::new(8.7, 0.0, z), Vec3::new(0.6, 0.8, 0.8), q);
    }
    // Feux de position
    for (x, y) in [(-7.0, 1.6), (0.0, 0.9), (6.5, 0.9), (3.5, 3.1)] {
        part(&red, &look.ball, Vec3::new(x, y, 1.6), Vec3::splat(0.22), q);
    }
}

/// Redessine les blocs posés quand le bouclier change.
fn sync_board(mut commands: Commands, mut game: ResMut<Game>, look: Res<Look>, old: Query<Entity, With<BoardBlock>>) {
    if !game.dirty {
        return;
    }
    game.dirty = false;
    for e in &old {
        commands.entity(e).despawn();
    }
    for (p, cell) in game.board.filled() {
        let mat = match cell {
            Cell::Block(k) => look.blocks[k as usize].clone(),
            _ => look.damaged.clone(),
        };
        commands.spawn((Mesh3d(look.cube.clone()), MeshMaterial3d(mat), Transform::from_translation(cell_pos(p)).with_scale(Vec3::splat(0.94)), BoardBlock));
    }
}

/// Pièce en cours et son ombre (là où elle tombera).
fn sync_piece(
    game: Res<Game>,
    look: Res<Look>,
    mut active: Query<(&ActiveBlock, &mut Transform, &mut Visibility, &mut MeshMaterial3d<StandardMaterial>), Without<GhostBlock>>,
    mut ghosts: Query<(&GhostBlock, &mut Transform, &mut Visibility), Without<ActiveBlock>>,
) {
    let piece = game.piece.filter(|_| game.ending.is_none());
    let ghost = piece.map(|p| game.board.drop_position(&p));
    for (b, mut tf, mut vis, mut mat) in &mut active {
        match piece {
            Some(p) if p.cells()[b.0].y < HEIGHT => {
                *vis = Visibility::Inherited;
                *tf = Transform::from_translation(cell_pos(p.cells()[b.0])).with_scale(Vec3::splat(0.94));
                mat.0 = look.blocks[p.kind.index() as usize].clone();
            }
            _ => *vis = Visibility::Hidden,
        }
    }
    for (g, mut tf, mut vis) in &mut ghosts {
        match ghost {
            Some(p) if p.cells()[g.0].y < HEIGHT => {
                *vis = Visibility::Inherited;
                *tf = Transform::from_translation(cell_pos(p.cells()[g.0])).with_scale(Vec3::splat(0.9));
            }
            _ => *vis = Visibility::Hidden,
        }
    }
}

/// Aperçus des 3 pièces suivantes et de la réserve.
fn sync_previews(game: Res<Game>, look: Res<Look>, mut q: Query<(&PreviewBlock, &mut Transform, &mut Visibility, &mut MeshMaterial3d<StandardMaterial>)>) {
    for (pb, mut tf, mut vis, mut mat) in &mut q {
        let (kind, center) = if pb.0 < 3 { (game.next.get(pb.0).copied(), NEXT_POS[pb.0]) } else { (game.hold, HOLD_POS) };
        let Some(kind) = kind else {
            *vis = Visibility::Hidden;
            continue;
        };
        let cells = Piece { kind, rot: 0, pos: IVec2::ZERO }.cells();
        let mid = cells.iter().map(|c| c.as_vec2()).sum::<Vec2>() / 4.0;
        let c = (cells[pb.1].as_vec2() - mid) * 0.8;
        *vis = Visibility::Inherited;
        *tf = Transform::from_translation(center + Vec3::new(c.x, c.y, 0.0)).with_scale(Vec3::splat(0.75));
        mat.0 = look.blocks[kind.index() as usize].clone();
    }
}

/// Rayons du canon à ions, débris des lignes, explosion du croiseur.
fn effects(
    mut commands: Commands,
    mut events: MessageReader<NovaEvent>,
    look: Res<Look>,
    mut rng: ResMut<Rng>,
    mut shake: ResMut<Shake>,
    mut flash: ResMut<HitFlash>,
    game: Res<Game>,
) {
    for event in events.read() {
        match event {
            NovaEvent::Lines(rows) => {
                let with_cruiser = game.mode != Mode::Endless;
                for &row in rows {
                    // Débris : les blocs de la ligne éclatent.
                    for x in 0..WIDTH {
                        let pos = cell_pos(IVec2::new(x, row));
                        let vel = Vec3::new(rng.below(200) as f32 / 40.0 - 2.5, rng.below(200) as f32 / 30.0, rng.below(200) as f32 / 40.0);
                        commands.spawn((
                            Mesh3d(look.cube.clone()),
                            MeshMaterial3d(look.debris[rng.below(7) as usize].clone()),
                            Transform::from_translation(pos).with_scale(Vec3::splat(0.35)),
                            Fx { life: 0.9, max: 0.9, vel, shrink: true },
                            NotShadowCaster,
                        ));
                    }
                    // Rayon : de la ligne jusqu'au croiseur.
                    if with_cruiser {
                        let y0 = row as f32 + 0.5;
                        let len = CRUISER_POS.y - y0;
                        commands.spawn((
                            Mesh3d(look.cube.clone()),
                            MeshMaterial3d(look.beam.clone()),
                            Transform::from_xyz(0.0, y0 + len / 2.0, -2.0).with_scale(Vec3::new(0.5 + rows.len() as f32 * 0.25, len, 0.5)),
                            Fx { life: 0.35, max: 0.35, vel: Vec3::ZERO, shrink: true },
                            NotShadowCaster,
                        ));
                    }
                }
                if with_cruiser {
                    flash.0 = 1.0;
                }
            }
            NovaEvent::EnemyShot => {
                shake.0 = 0.5;
                // Tir rouge du croiseur vers le bas du bouclier
                let len = CRUISER_POS.y;
                commands.spawn((
                    Mesh3d(look.cube.clone()),
                    MeshMaterial3d(look.fire.clone()),
                    Transform::from_xyz(-4.0, len / 2.0, -1.5).with_scale(Vec3::new(0.6, len, 0.6)),
                    Fx { life: 0.4, max: 0.4, vel: Vec3::ZERO, shrink: true },
                    NotShadowCaster,
                ));
            }
            NovaEvent::CruiserDestroyed => {
                shake.0 = 1.5;
                for _ in 0..80 {
                    let vel = Vec3::new(rng.below(200) as f32 / 10.0 - 10.0, rng.below(200) as f32 / 14.0 - 4.0, rng.below(200) as f32 / 20.0 - 5.0);
                    let mat = if rng.below(3) == 0 { look.hull.clone() } else { look.fire.clone() };
                    commands.spawn((
                        Mesh3d(look.cube.clone()),
                        MeshMaterial3d(mat),
                        Transform::from_translation(CRUISER_POS + vel * 0.1).with_scale(Vec3::splat(0.3 + rng.below(60) as f32 / 100.0)),
                        Fx { life: 2.2, max: 2.2, vel, shrink: true },
                        NotShadowCaster,
                    ));
                }
            }
            _ => {}
        }
    }
}

fn animate_fx(mut commands: Commands, time: Res<Time>, mut q: Query<(Entity, &mut Fx, &mut Transform)>) {
    let dt = time.delta_secs();
    for (e, mut fx, mut tf) in &mut q {
        fx.life -= dt;
        if fx.life <= 0.0 {
            commands.entity(e).despawn();
            continue;
        }
        if fx.vel != Vec3::ZERO {
            fx.vel.y -= 9.0 * dt;
            tf.translation += fx.vel * dt;
            tf.rotate(Quat::from_rotation_z(dt * 4.0) * Quat::from_rotation_x(dt * 3.0));
        }
        if fx.shrink {
            let k = (fx.life / fx.max).sqrt();
            if fx.vel == Vec3::ZERO {
                // Rayon : s'amincit
                tf.scale.x *= 0.9f32.powf(dt * 60.0).max(k);
                tf.scale.z = tf.scale.x;
            } else {
                tf.scale = Vec3::splat(0.35 * k + 0.05);
            }
        }
    }
}

/// Le croiseur flotte, clignote quand il est touché, et disparaît détruit.
fn animate_cruiser(
    time: Res<Time>,
    game: Res<Game>,
    state: Res<State<GameState>>,
    mut flash: ResMut<HitFlash>,
    look: Res<Look>,
    mut mats: ResMut<Assets<StandardMaterial>>,
    mut q: Query<(&mut Transform, &mut Visibility), With<Cruiser>>,
) {
    let t = time.elapsed_secs();
    flash.0 = (flash.0 - time.delta_secs() * 4.0).max(0.0);
    let warning = (game.warning > 0.0) as i32 as f32 * ((t * 10.0).sin() * 0.5 + 0.5);
    if let Some(mut m) = mats.get_mut(&look.hull) {
        m.emissive = LinearRgba::rgb(flash.0 * 4.0 + warning * 1.5, flash.0 * 4.0, flash.0 * 4.0);
    }
    let show = game.mode != Mode::Endless && !matches!(state.get(), GameState::Victory) && (game.hull > 0.0 || game.ending.is_some_and(|e| e > 2.2));
    for (mut tf, mut vis) in &mut q {
        *vis = if show { Visibility::Inherited } else { Visibility::Hidden };
        tf.translation = CRUISER_POS + Vec3::new((t * 0.3).sin() * 1.5, (t * 0.8).sin() * 0.3, 0.0);
    }
}

/// La planète change selon le chapitre.
fn chapter_backdrop(game: Res<Game>, look: Res<Look>, mut mats: ResMut<Assets<StandardMaterial>>, mut last: Local<Option<Mode>>) {
    if *last == Some(game.mode) {
        return;
    }
    *last = Some(game.mode);
    let (color, glow) = match game.mode {
        Mode::Campaign(i) => (CHAPTERS[i].planet, CHAPTERS[i].planet_glow),
        Mode::Endless => (Color::srgb(0.55, 0.40, 0.75), false),
    };
    if let Some(mut m) = mats.get_mut(&look.planet) {
        m.base_color = color;
        m.emissive = if glow { color.to_linear() * 0.8 } else { LinearRgba::BLACK };
    }
}

fn shake_camera(time: Res<Time>, mut shake: ResMut<Shake>, mut q: Query<&mut Transform, With<Camera3d>>) {
    shake.0 = (shake.0 - time.delta_secs()).max(0.0);
    let t = time.elapsed_secs();
    let offset = Vec3::new((t * 53.0).sin(), (t * 47.0).cos(), 0.0) * shake.0 * 0.4;
    for mut tf in &mut q {
        *tf = Transform::from_translation(LOOK + CAM_OFFSET + offset).looking_at(LOOK + offset, Vec3::Y);
    }
}
