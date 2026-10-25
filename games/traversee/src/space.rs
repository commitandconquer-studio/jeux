//! Le vide de Cosmos Croâ : étoiles et planètes lointaines qui défilent à des
//! vitesses différentes (effet de profondeur), et ciel noir.
use bevy::light::NotShadowCaster;
use bevy::prelude::*;
use game_common::{Rng, Score, flat_material};

use crate::extra::Biome;
use crate::frog::Frog;
use crate::models::Kit;
use crate::progress::Progress;

/// Le joueur est-il dans Cosmos Croâ ? (mis à jour à chaque image)
static IN_SPACE: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

pub fn in_space() -> bool {
    IN_SPACE.load(std::sync::atomic::Ordering::Relaxed)
}

pub struct SpacePlugin;

impl Plugin for SpacePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_sky).add_systems(Update, update_sky);
    }
}

/// Un astre du fond : `k` = part du mouvement de la caméra qu'il suit (proche de 1 = très loin, il bouge à peine).
#[derive(Component)]
struct Star {
    base: Vec2,
    k: f32,
    y: f32,
}

const RADIUS: f32 = 34.0;
const DEEP: Color = Color::srgb(0.02, 0.025, 0.06);

/// Le fond noir du vide (l'ambiance du studio teinte le ciel : on pose un grand plan sombre dessous).
#[derive(Component)]
struct Backdrop;

fn spawn_sky(mut commands: Commands, kit: Res<Kit>, mut mats: ResMut<Assets<StandardMaterial>>, mut rng: ResMut<Rng>) {
    let unlit = |c: Color| StandardMaterial { unlit: true, ..flat_material(c) };
    let white = mats.add(unlit(Color::srgb(1.0, 1.0, 1.0)));
    let blue = mats.add(unlit(Color::srgb(0.65, 0.8, 1.0)));
    let gold = mats.add(unlit(Color::srgb(1.0, 0.9, 0.6)));
    let deep = mats.add(unlit(DEEP));
    commands.spawn((
        Mesh3d(kit.cube.clone()),
        MeshMaterial3d(deep),
        Transform::from_xyz(0.0, -90.0, 0.0).with_scale(Vec3::new(600.0, 1.0, 600.0)),
        Visibility::Hidden,
        Backdrop,
        NotShadowCaster,
    ));
    let r = |rng: &mut Rng| rng.below(10_000) as f32 / 10_000.0;
    // Trois couches d'étoiles : lointaines (fines, quasi immobiles), moyennes, proches (grosses, rapides).
    for (count, k, y, size) in [(130, 0.96, -55.0, 0.09), (90, 0.85, -28.0, 0.11), (45, 0.7, -12.0, 0.15)] {
        for i in 0..count {
            let base = Vec2::new(r(&mut rng) * 2.0 * RADIUS - RADIUS, r(&mut rng) * 2.0 * RADIUS - RADIUS);
            let mat = [&white, &blue, &gold][i % 3].clone();
            let s = size * (0.6 + r(&mut rng) * 0.9);
            commands.spawn((
                Mesh3d(kit.cube.clone()),
                MeshMaterial3d(mat),
                Transform::from_xyz(0.0, y, 0.0).with_scale(Vec3::splat(s)),
                Visibility::Hidden,
                Star { base, k, y },
                NotShadowCaster,
            ));
        }
    }
    // Quelques planètes très lointaines, pas plus.
    for (x, z, d, col) in [(-15.0, 6.0, 7.0, (0.95, 0.55, 0.25)), (17.0, -14.0, 4.5, (0.35, 0.55, 0.95)), (-22.0, -22.0, 3.0, (0.8, 0.8, 0.85))] {
        let m = mats.add(unlit(Color::srgb(col.0, col.1, col.2)));
        commands.spawn((
            Mesh3d(kit.ball.clone()),
            MeshMaterial3d(m),
            Transform::from_xyz(0.0, -40.0, 0.0).with_scale(Vec3::splat(d)),
            Visibility::Hidden,
            Star { base: Vec2::new(x, z), k: 0.97, y: -40.0 },
            NotShadowCaster,
        ));
    }
}

#[allow(clippy::too_many_arguments)]
fn update_sky(
    frog: Res<Frog>,
    progress: Res<Progress>,
    score: Res<Score>,
    cam: Res<crate::CamZ>,
    mut stars: Query<(&Star, &mut Transform, &mut Visibility), Without<Backdrop>>,
    mut backdrop: Query<(&mut Transform, &mut Visibility), With<Backdrop>>,
    state: Res<State<crate::GameState>>,
    mut amb: ResMut<game_common::ambiance::Ambiance>,
    mut saved: Local<Option<(game_common::ambiance::Moment, Option<game_common::ambiance::Meteo>)>>,
) {
    let space = Biome::of(frog.max_row_i(), progress.total + score.current) == Biome::Space;
    IN_SPACE.store(space, std::sync::atomic::Ordering::Relaxed);
    // Dans le vide et au garage : pas de coucher de soleil ni de pluie, une lumière neutre.
    if space || *state.get() == crate::GameState::Garage {
        if saved.is_none() {
            *saved = Some((amb.moment, amb.meteo));
        }
        amb.moment = game_common::ambiance::Moment::Jour;
        amb.meteo = None;
    } else if let Some((moment, meteo)) = saved.take() {
        amb.moment = moment;
        amb.meteo = meteo;
    }
    for (mut tf, mut v) in &mut backdrop {
        tf.translation.x = frog.pos.x * 0.4;
        tf.translation.z = cam.0;
        let want = if space { Visibility::Inherited } else { Visibility::Hidden };
        if *v != want {
            *v = want;
        }
    }
    let vis = if space { Visibility::Inherited } else { Visibility::Hidden };
    // La caméra avance vers -Z : les astres lointains la suivent presque, les proches défilent.
    let (cx, cz) = (frog.pos.x * 0.4, cam.0);
    for (s, mut tf, mut v) in &mut stars {
        if *v != vis {
            *v = vis;
        }
        if !space {
            continue;
        }
        let wrap = |a: f32| (a + RADIUS).rem_euclid(2.0 * RADIUS) - RADIUS;
        let rel = Vec2::new(wrap(s.base.x - (1.0 - s.k) * cx), wrap(s.base.y - (1.0 - s.k) * cz));
        tf.translation = Vec3::new(cx + rel.x, s.y, cz + rel.y);
    }
}
