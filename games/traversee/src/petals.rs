//! Pétales de cerisier qui flottent sur l'écran dans Nippon Kaboom.
use bevy::light::NotShadowCaster;
use bevy::prelude::*;
use game_common::{Rng, Score, flat_material};

use crate::GameState;
use crate::extra::Biome;
use crate::frog::Frog;
use crate::models::Kit;
use crate::progress::Progress;

pub struct PetalsPlugin;

impl Plugin for PetalsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PetalMats>().add_systems(Update, petals.run_if(|s: Res<State<GameState>>| *s.get() != GameState::Garage));
    }
}

#[derive(Resource)]
struct PetalMats([Handle<StandardMaterial>; 2]);

impl FromWorld for PetalMats {
    fn from_world(world: &mut World) -> Self {
        let mut mats = world.resource_mut::<Assets<StandardMaterial>>();
        let mut f = |c: Color| mats.add(StandardMaterial { unlit: true, ..flat_material(c) });
        PetalMats([f(Color::srgb(1.0, 0.72, 0.82)), f(Color::srgb(1.0, 0.86, 0.91))])
    }
}

#[derive(Component)]
struct Petal {
    vel: Vec3,
    age: f32,
    phase: f32,
}

#[allow(clippy::too_many_arguments)]
fn petals(
    mut commands: Commands,
    time: Res<Time>,
    kit: Res<Kit>,
    mats: Res<PetalMats>,
    frog: Res<Frog>,
    progress: Res<Progress>,
    score: Res<Score>,
    cam: Res<crate::CamZ>,
    mut rng: ResMut<Rng>,
    mut acc: Local<f32>,
    mut q: Query<(Entity, &mut Petal, &mut Transform)>,
) {
    let dt = time.delta_secs();
    let now = time.elapsed_secs();
    for (e, mut p, mut tf) in &mut q {
        p.age += dt;
        // Elles dérivent en tournoyant doucement, puis se posent.
        tf.translation += (p.vel + Vec3::new((now * 1.3 + p.phase).sin() * 0.5, 0.0, (now * 1.7 + p.phase).cos() * 0.4)) * dt;
        tf.rotation = Quat::from_euler(EulerRot::XYZ, now * 2.0 + p.phase, now * 1.5 + p.phase, 0.0);
        if tf.translation.y < 0.15 || p.age > 14.0 {
            commands.entity(e).despawn();
        }
    }
    let in_japan = Biome::of(frog.max_row_i(), progress.total + score.current) == Biome::Nippon;
    if !in_japan {
        return;
    }
    *acc += dt * 14.0;
    while *acc >= 1.0 {
        *acc -= 1.0;
        let r = |rng: &mut Rng| rng.below(10_000) as f32 / 10_000.0;
        let at = Vec3::new(frog.pos.x + r(&mut rng) * 26.0 - 9.0, 5.0 + r(&mut rng) * 2.5, cam.0 - 3.0 + r(&mut rng) * 12.0);
        let vel = Vec3::new(-0.9 - r(&mut rng) * 0.8, -0.55 - r(&mut rng) * 0.35, 0.1);
        commands.spawn((
            Mesh3d(kit.cube.clone()),
            MeshMaterial3d(mats.0[rng.below(2) as usize].clone()),
            Transform::from_translation(at).with_scale(Vec3::new(0.14, 0.02, 0.1)),
            Petal { vel, age: 0.0, phase: r(&mut rng) * 6.28 },
            NotShadowCaster,
        ));
    }
}
