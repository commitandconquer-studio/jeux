//! Effets visuels : poussière, éclaboussures, éclats, tremblement d'écran.
use bevy::prelude::*;
use game_common::{Rng, flat_material};

use crate::models::Kit;

/// Un effet à jouer à cet endroit.
#[derive(Message, Clone, Copy)]
pub enum Fx {
    /// Petit nuage à l'atterrissage.
    Dust(Vec3),
    /// Ronds dans l'eau quand le crapaud se pose sur un rondin.
    Ripple(Vec3),
}

/// Tremblement de la caméra : 0 (calme) à 1 (fort), retombe tout seul.
#[derive(Resource, Default)]
pub struct Shake(pub f32);

#[derive(Component)]
struct Particle {
    vel: Vec3,
    life: f32,
    max: f32,
    gravity: f32,
    size: f32,
}

#[derive(Resource)]
struct FxMats {
    dust: Handle<StandardMaterial>,
    water: Handle<StandardMaterial>,
}

pub struct FxPlugin;

impl Plugin for FxPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<Fx>()
            .init_resource::<Shake>()
            .add_systems(Startup, setup)
            .add_systems(Update, (spawn_fx, update_particles).chain());
    }
}

fn setup(mut commands: Commands, mut mats: ResMut<Assets<StandardMaterial>>) {
    let mut unlit = |c: Color| mats.add(StandardMaterial { unlit: true, ..flat_material(c) });
    commands.insert_resource(FxMats {
        dust: unlit(Color::srgb(0.96, 0.94, 0.86)),
        water: unlit(Color::srgb(0.75, 0.92, 1.0)),
    });
}

fn unit(rng: &mut Rng) -> f32 {
    rng.below(10_000) as f32 / 5_000.0 - 1.0
}

#[allow(clippy::too_many_arguments)]
fn puff(c: &mut Commands, kit: &Kit, mat: &Handle<StandardMaterial>, pos: Vec3, vel: Vec3, size: f32, life: f32, gravity: f32) {
    c.spawn((
        Mesh3d(kit.cube.clone()),
        MeshMaterial3d(mat.clone()),
        Transform::from_translation(pos).with_scale(Vec3::splat(size)),
        Particle { vel, life, max: life, gravity, size },
        bevy::light::NotShadowCaster,
    ));
}

fn spawn_fx(mut commands: Commands, mut reader: MessageReader<Fx>, kit: Res<Kit>, mats: Res<FxMats>, mut rng: ResMut<Rng>) {
    for fx in reader.read() {
        match *fx {
            Fx::Dust(p) => {
                for _ in 0..6 {
                    let v = Vec3::new(unit(&mut rng), 0.0, unit(&mut rng)).normalize_or_zero() * (0.7 + 0.5 * unit(&mut rng).abs());
                    puff(&mut commands, &kit, &mats.dust, p + v * 0.12 + Vec3::Y * 0.05, v + Vec3::Y * 0.6, 0.12 + 0.06 * unit(&mut rng).abs(), 0.35, 1.5);
                }
            }
            Fx::Ripple(p) => {
                for _ in 0..4 {
                    let v = Vec3::new(unit(&mut rng), 0.0, unit(&mut rng)).normalize_or_zero() * 0.6;
                    puff(&mut commands, &kit, &mats.water, p + Vec3::Y * 0.05, v + Vec3::Y * 1.6, 0.09, 0.4, 7.0);
                }
            }
        }
    }
}

fn update_particles(mut commands: Commands, time: Res<Time>, mut q: Query<(Entity, &mut Particle, &mut Transform)>) {
    let dt = time.delta_secs();
    for (e, mut p, mut tf) in &mut q {
        p.life -= dt;
        if p.life <= 0.0 {
            commands.entity(e).despawn();
            continue;
        }
        p.vel.y -= p.gravity * dt;
        tf.translation += p.vel * dt;
        tf.scale = Vec3::splat(p.size * (p.life / p.max).sqrt());
    }
}
