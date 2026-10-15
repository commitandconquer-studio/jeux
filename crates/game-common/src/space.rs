//! Fond spatial de l'univers Nova : étoiles sur plusieurs plans de
//! profondeur (les proches défilent plus vite, brillent plus, sont plus
//! grosses), scintillement, nébuleuses, et petits vaisseaux qui passent au loin.
//!
//! Utilisation : ajouter `SpaceBackdropPlugin` et une ressource `SpaceView`
//! décrivant la zone visible (centre et taille, dans le plan de la scène).
use bevy::asset::RenderAssetUsages;
use bevy::light::NotShadowCaster;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};

use crate::{Rng, flat_material};

pub struct SpaceBackdropPlugin;

impl Plugin for SpaceBackdropPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::srgb(0.015, 0.02, 0.05)))
            .insert_resource(BackdropRng(Rng::new(4_242)))
            .add_systems(PostStartup, spawn_backdrop)
            .add_systems(Update, (add_halos, drift_stars, twinkle, drift_nebulae, launch_ships, fly_ships));
    }
}

/// Zone de la scène à couvrir (centre et largeur × hauteur visibles).
#[derive(Resource, Clone, Copy)]
pub struct SpaceView {
    pub center: Vec3,
    pub size: Vec2,
}

#[derive(Resource)]
struct BackdropRng(Rng);

impl BackdropRng {
    fn range(&mut self, a: f32, b: f32) -> f32 {
        a + (b - a) * self.0.below(10_000) as f32 / 10_000.0
    }
}

/// Un plan d'étoiles : profondeur (pour l'ordre d'affichage), nombre, taille,
/// éclat et vitesse de défilement.
struct Layer {
    z: f32,
    count: usize,
    size: (f32, f32),
    glow: f32,
    speed: f32,
}

const LAYERS: [Layer; 3] = [
    Layer { z: -120.0, count: 320, size: (0.03, 0.07), glow: 1.2, speed: 0.15 },
    Layer { z: -100.0, count: 150, size: (0.07, 0.13), glow: 2.5, speed: 0.45 },
    Layer { z: -80.0, count: 45, size: (0.13, 0.22), glow: 4.5, speed: 1.1 },
];

const STAR_COLORS: [Color; 4] = [
    Color::srgb(1.0, 1.0, 1.0),
    Color::srgb(0.75, 0.85, 1.0),
    Color::srgb(1.0, 0.92, 0.75),
    Color::srgb(1.0, 0.80, 0.85),
];

#[derive(Component)]
struct Star {
    speed: f32,
}

/// Matériau d'étoile qui scintille (éclat de base, phase).
#[derive(Component)]
struct Twinkle {
    material: Handle<StandardMaterial>,
    base: LinearRgba,
    phase: f32,
}

#[derive(Component)]
struct Nebula {
    speed: f32,
}

#[derive(Component)]
struct DistantShip {
    speed: f32,
}

/// Délai avant le prochain passage de vaisseaux.
#[derive(Resource)]
struct ShipTimer(f32);

#[derive(Resource)]
struct ShipLook {
    cube: Handle<Mesh>,
    ball: Handle<Mesh>,
    hull: Handle<StandardMaterial>,
    engines: [Handle<StandardMaterial>; 2],
}

/// Étoile proche à entourer d'un halo (couleur de l'étoile).
#[derive(Component)]
struct NeedsHalo(Color);

/// Texture ronde dégradée (opaque au centre, transparente au bord).
fn soft_disc() -> Image {
    const N: u32 = 128;
    let mut data = Vec::with_capacity((N * N * 4) as usize);
    for y in 0..N {
        for x in 0..N {
            let d = Vec2::new(x as f32 + 0.5, y as f32 + 0.5) / N as f32 * 2.0 - Vec2::ONE;
            let a = (1.0 - d.length()).clamp(0.0, 1.0).powf(2.2);
            data.extend_from_slice(&[255, 255, 255, (a * 255.0) as u8]);
        }
    }
    Image::new(
        Extent3d { width: N, height: N, depth_or_array_layers: 1 },
        TextureDimension::D2,
        data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD,
    )
}

fn halos(commands: &mut Commands, mats: &mut Assets<StandardMaterial>, soft: &Handle<Image>, quad: &Handle<Mesh>) {
    let mut by_color: Vec<(Color, Handle<StandardMaterial>)> = Vec::new();
    for color in STAR_COLORS {
        let m = mats.add(StandardMaterial {
            base_color: color.with_alpha(0.45),
            base_color_texture: Some(soft.clone()),
            alpha_mode: AlphaMode::Add,
            unlit: true,
            ..default()
        });
        by_color.push((color, m));
    }
    commands.insert_resource(HaloLook { quad: quad.clone(), by_color });
}

#[derive(Resource)]
struct HaloLook {
    quad: Handle<Mesh>,
    by_color: Vec<(Color, Handle<StandardMaterial>)>,
}

fn add_halos(mut commands: Commands, look: Option<Res<HaloLook>>, q: Query<(Entity, &NeedsHalo)>) {
    let Some(look) = look else { return };
    for (e, halo) in &q {
        let m = look.by_color.iter().find(|(c, _)| *c == halo.0).map(|(_, m)| m.clone()).unwrap_or_else(|| look.by_color[0].1.clone());
        commands.entity(e).remove::<NeedsHalo>();
        commands.spawn((
            Mesh3d(look.quad.clone()),
            MeshMaterial3d(m),
            Transform::from_xyz(0.0, 0.0, -0.2).with_scale(Vec3::splat(7.0)),
            NotShadowCaster,
            ChildOf(e),
        ));
    }
}

#[allow(clippy::too_many_arguments)]
fn spawn_backdrop(
    mut commands: Commands,
    view: Option<Res<SpaceView>>,
    mut rng: ResMut<BackdropRng>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    let view = view.map(|v| *v).unwrap_or(SpaceView { center: Vec3::ZERO, size: Vec2::new(60.0, 40.0) });
    // On déborde largement de la zone visible (caméra un peu inclinée).
    let half = view.size * 0.8;
    let ball = meshes.add(Sphere::new(0.5).mesh().ico(1).unwrap());
    let cube = meshes.add(Cuboid::new(1.0, 1.0, 1.0));

    for layer in &LAYERS {
        // Quelques matériaux par plan, qui scintillent chacun à leur rythme.
        let materials: Vec<(Handle<StandardMaterial>, LinearRgba)> = (0..6)
            .map(|i| {
                let color = STAR_COLORS[i % STAR_COLORS.len()];
                let base = color.to_linear() * layer.glow;
                let m = mats.add(StandardMaterial { base_color: color, emissive: base, unlit: true, ..default() });
                commands.spawn(Twinkle { material: m.clone(), base, phase: i as f32 * 1.7 });
                (m, base)
            })
            .collect();
        let near = layer.speed > 1.0;
        for _ in 0..layer.count {
            let pos = view.center + Vec3::new(rng.range(-half.x, half.x), rng.range(-half.y, half.y), layer.z);
            let k = rng.0.below(materials.len() as u32) as usize;
            let mut e = commands.spawn((
                Mesh3d(ball.clone()),
                MeshMaterial3d(materials[k].0.clone()),
                Transform::from_translation(pos).with_scale(Vec3::splat(rng.range(layer.size.0, layer.size.1))),
                Star { speed: layer.speed },
                NotShadowCaster,
            ));
            if near {
                e.insert(NeedsHalo(STAR_COLORS[k % STAR_COLORS.len()]));
            }
        }
    }

    // Nébuleuses : nuages doux faits de nombreux voiles dégradés superposés.
    let soft = images.add(soft_disc());
    let quad = meshes.add(Rectangle::new(1.0, 1.0));
    for (i, color) in [Color::srgb(0.50, 0.25, 0.80), Color::srgb(0.15, 0.35, 0.85), Color::srgb(0.10, 0.60, 0.65), Color::srgb(0.80, 0.25, 0.55)]
        .into_iter()
        .enumerate()
    {
        let m = mats.add(StandardMaterial {
            base_color: color.with_alpha(0.16),
            base_color_texture: Some(soft.clone()),
            alpha_mode: AlphaMode::Add,
            unlit: true,
            ..default()
        });
        let cloud = view.center + Vec3::new(rng.range(-half.x, half.x), rng.range(-half.y, half.y), -150.0 - i as f32);
        for k in 0..14 {
            let pos = cloud + Vec3::new(rng.range(-12.0, 12.0), rng.range(-7.0, 7.0), -(k as f32) * 0.05);
            let s = rng.range(7.0, 18.0);
            commands.spawn((
                Mesh3d(quad.clone()),
                MeshMaterial3d(m.clone()),
                Transform::from_translation(pos)
                    .with_rotation(Quat::from_rotation_z(rng.range(0.0, 6.28)))
                    .with_scale(Vec3::new(s * rng.range(1.0, 1.8), s, 1.0)),
                Nebula { speed: 0.06 },
                NotShadowCaster,
            ));
        }
    }
    // Halo lumineux autour des étoiles les plus proches.
    halos(&mut commands, &mut mats, &soft, &quad);

    commands.insert_resource(ShipLook {
        cube,
        ball,
        hull: mats.add(flat_material(Color::srgb(0.22, 0.24, 0.30))),
        engines: [
            mats.add(StandardMaterial { base_color: Color::srgb(0.5, 0.8, 1.0), emissive: LinearRgba::rgb(2.0, 4.0, 8.0), unlit: true, ..default() }),
            mats.add(StandardMaterial { base_color: Color::srgb(1.0, 0.6, 0.3), emissive: LinearRgba::rgb(8.0, 3.0, 1.0), unlit: true, ..default() }),
        ],
    });
    commands.insert_resource(ShipTimer(2.0));
    commands.insert_resource(view);
}

/// Les étoiles défilent vers le bas (on avance dans l'espace) et reviennent en haut.
fn drift_stars(time: Res<Time>, view: Option<Res<SpaceView>>, mut q: Query<(&Star, &mut Transform)>) {
    let Some(view) = view else { return };
    let half = view.size.y * 0.8;
    let dt = time.delta_secs();
    for (star, mut tf) in &mut q {
        tf.translation.y -= star.speed * dt;
        if tf.translation.y < view.center.y - half {
            tf.translation.y += 2.0 * half;
        }
    }
}

fn twinkle(time: Res<Time>, q: Query<&Twinkle>, mut mats: ResMut<Assets<StandardMaterial>>) {
    let t = time.elapsed_secs();
    for tw in &q {
        if let Some(mut m) = mats.get_mut(&tw.material) {
            m.emissive = tw.base * (0.65 + 0.35 * (t * 1.3 + tw.phase).sin());
        }
    }
}

fn drift_nebulae(time: Res<Time>, view: Option<Res<SpaceView>>, mut q: Query<(&Nebula, &mut Transform)>) {
    let Some(view) = view else { return };
    let half = view.size.y * 0.8 + 15.0;
    for (n, mut tf) in &mut q {
        tf.translation.y -= n.speed * time.delta_secs();
        if tf.translation.y < view.center.y - half {
            tf.translation.y += 2.0 * half;
        }
    }
}

/// De temps en temps, un vaisseau (ou une petite formation) traverse au loin.
fn launch_ships(
    mut commands: Commands,
    time: Res<Time>,
    view: Option<Res<SpaceView>>,
    look: Option<Res<ShipLook>>,
    mut timer: Option<ResMut<ShipTimer>>,
    mut rng: ResMut<BackdropRng>,
) {
    let (Some(view), Some(look), Some(timer)) = (view, look, timer.as_mut()) else { return };
    timer.0 -= time.delta_secs();
    if timer.0 > 0.0 {
        return;
    }
    timer.0 = rng.range(4.0, 10.0);
    let dir = if rng.0.below(2) == 0 { 1.0 } else { -1.0 };
    let half = view.size * 0.75;
    let y = view.center.y + rng.range(-half.y * 0.8, half.y * 0.8);
    let scale = rng.range(0.25, 0.7);
    let speed = rng.range(2.0, 5.0) * dir * (0.5 + scale);
    let count = 1 + rng.0.below(3);
    let engine = look.engines[rng.0.below(2) as usize].clone();
    for k in 0..count {
        let offset = Vec3::new(-dir * k as f32 * 2.2 * scale * 2.0, k as f32 * 0.9 * scale * 2.0, 0.0);
        let start = Vec3::new(view.center.x - dir * (half.x + 4.0), y, -90.0) + offset;
        let root = commands
            .spawn((
                Transform::from_translation(start)
                    .with_scale(Vec3::splat(scale))
                    .with_rotation(Quat::from_rotation_y(if dir > 0.0 { 0.0 } else { std::f32::consts::PI })),
                Visibility::default(),
                DistantShip { speed },
            ))
            .id();
        let mut part = |mesh: &Handle<Mesh>, m: &Handle<StandardMaterial>, pos: Vec3, s: Vec3| {
            commands.spawn((Mesh3d(mesh.clone()), MeshMaterial3d(m.clone()), Transform::from_translation(pos).with_scale(s), NotShadowCaster, ChildOf(root)));
        };
        // Petit vaisseau vu de côté : coque, ailes, réacteur et traînée.
        part(&look.cube, &look.hull, Vec3::ZERO, Vec3::new(1.8, 0.35, 0.5));
        part(&look.cube, &look.hull, Vec3::new(0.9, 0.0, 0.0), Vec3::new(0.5, 0.22, 0.4));
        part(&look.cube, &look.hull, Vec3::new(-0.3, -0.1, 0.0), Vec3::new(0.7, 0.08, 1.6));
        part(&look.ball, &engine, Vec3::new(-0.95, 0.0, 0.0), Vec3::splat(0.32));
        part(&look.cube, &engine, Vec3::new(-1.9, 0.0, 0.0), Vec3::new(1.8, 0.06, 0.06));
    }
}

fn fly_ships(mut commands: Commands, time: Res<Time>, view: Option<Res<SpaceView>>, mut q: Query<(Entity, &DistantShip, &mut Transform)>) {
    let Some(view) = view else { return };
    let limit = view.size.x * 0.75 + 8.0;
    for (e, ship, mut tf) in &mut q {
        tf.translation.x += ship.speed * time.delta_secs();
        if (tf.translation.x - view.center.x).abs() > limit {
            commands.entity(e).despawn();
        }
    }
}
