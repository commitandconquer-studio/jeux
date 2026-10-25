//! L'aigle : de temps en temps, une colonne se teinte de rouge pour prévenir,
//! puis un aigle la parcourt de bout en bout. S'il attrape le crapaud au
//! passage, il l'emporte.
use std::f32::consts::PI;

use bevy::light::NotShadowCaster;
use bevy::prelude::*;
use game_common::{Rng, Score, flat_material};

use crate::GameState;
use crate::audio::Sfx;
use crate::frog::{Cause, Death, Frog, die};
use crate::lanes::COLS;
use crate::models::{Kit, part, part_rot};
use crate::progress::{Item, Progress, unlocked};

const SPEED: f32 = 15.0;
const HEIGHT: f32 = 3.2;
const WARNING: f32 = 1.7;
/// Immobile aussi longtemps : l'aigle fond sur le crapaud.
const IDLE_LIMIT: f32 = 4.0;

#[derive(Clone, Copy)]
enum Phase {
    Idle,
    /// La colonne clignote ; l'aigle n'est pas encore là.
    Warning { col: i32, dir: f32, t: f32 },
    /// L'aigle vole le long de la colonne.
    Flying { col: i32, dir: f32, z: f32 },
}

#[derive(Resource)]
pub struct Eagle {
    phase: Phase,
    timer: f32,
    /// Position de l'aigle quand il tient le crapaud.
    grip: Option<Vec3>,
    /// Dernière case du crapaud et depuis combien de temps il n'a pas bougé.
    idle: (i32, i32, f32),
}

impl Default for Eagle {
    fn default() -> Self {
        Self { phase: Phase::Idle, timer: 25.0, grip: None, idle: (0, 0, 0.0) }
    }
}

impl Eagle {
    /// Où l'aigle emporte le crapaud, s'il l'a attrapé.
    pub fn grip(&self) -> Option<Vec3> {
        self.grip
    }
}

#[derive(Component)]
struct EagleVisual;

#[derive(Component)]
struct Shadow;

#[derive(Component)]
struct Strip;

#[derive(Component)]
struct Wing(f32);

#[derive(Resource)]
struct EagleKit {
    body: Handle<StandardMaterial>,
    dark: Handle<StandardMaterial>,
    white: Handle<StandardMaterial>,
    beak: Handle<StandardMaterial>,
    shadow: Handle<StandardMaterial>,
    warn_on: Handle<StandardMaterial>,
    warn_off: Handle<StandardMaterial>,
}

impl FromWorld for EagleKit {
    fn from_world(world: &mut World) -> Self {
        let mut mats = world.resource_mut::<Assets<StandardMaterial>>();
        let mut f = |r: f32, g: f32, b: f32| mats.add(flat_material(Color::srgb(r, g, b)));
        let body = f(0.50, 0.33, 0.20);
        let dark = f(0.30, 0.20, 0.13);
        let white = f(0.97, 0.97, 0.95);
        let beak = f(1.0, 0.78, 0.15);
        let mut blend = |c: Color| mats.add(StandardMaterial { unlit: true, alpha_mode: AlphaMode::Blend, ..flat_material(c) });
        let shadow = blend(Color::srgba(0.05, 0.08, 0.05, 0.4));
        let warn_on = blend(Color::srgba(1.0, 0.12, 0.1, 0.62));
        let warn_off = blend(Color::srgba(1.0, 0.12, 0.1, 0.22));
        Self { body, dark, white, beak, shadow, warn_on, warn_off }
    }
}

pub struct EaglePlugin;

impl Plugin for EaglePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Eagle>()
            .init_resource::<EagleKit>()
            .add_systems(OnEnter(GameState::Playing), reset)
            .add_systems(
                Update,
                (run_eagle, flap, blink).run_if(|s: Res<State<GameState>>| matches!(s.get(), GameState::Playing | GameState::Dying)),
            );
    }
}

fn reset(
    mut commands: Commands,
    mut eagle: ResMut<Eagle>,
    mut rng: ResMut<Rng>,
    q: Query<Entity, Or<(With<EagleVisual>, With<Strip>)>>,
    frog: Res<Frog>,
) {
    // Reprise après la mort : on ne touche à rien (la partie continue). Neuf seulement au départ.
    if frog.started {
        return;
    }
    for e in &q {
        commands.entity(e).despawn();
    }
    *eagle = Eagle::default();
    eagle.timer = if std::env::var("TRAVERSEE_EAGLE").is_ok() { 2.0 } else { 22.0 + (rng.below(800) as f32) / 100.0 };
}

fn unit(rng: &mut Rng) -> f32 {
    rng.below(10_000) as f32 / 10_000.0
}

#[allow(clippy::too_many_arguments)]
fn run_eagle(
    mut commands: Commands,
    time: Res<Time>,
    state: Res<State<GameState>>,
    mut eagle: ResMut<Eagle>,
    kit: Res<Kit>,
    ek: Res<EagleKit>,
    mut rng: ResMut<Rng>,
    frog: Res<Frog>,
    progress: Res<Progress>,
    score: Res<Score>,
    cam: Res<crate::CamZ>,
    extra: Res<crate::extra::Extra>,
    mut death: ResMut<Death>,
    mut next: ResMut<NextState<GameState>>,
    mut sfx: MessageWriter<Sfx>,
    mut visuals: Query<(Entity, &mut Transform, Has<Shadow>, Has<Strip>), Or<(With<EagleVisual>, With<Strip>)>>,
) {
    let dt = time.delta_secs();
    let playing = *state.get() == GameState::Playing;
    let points = progress.total + score.current;
    match eagle.phase {
        Phase::Idle => {
            // Le compte à rebours ne tourne qu'une fois la partie bien lancée.
            let test = std::env::var("TRAVERSEE_EAGLE").is_ok();
            if !(playing && (frog.started || test) && unlocked(points, Item::Eagle)) {
                return;
            }
            // Immobile trop longtemps : il arrive tout de suite, sur sa colonne.
            let here = (frog.row, frog.pos.x.round() as i32);
            if here != (eagle.idle.0, eagle.idle.1) {
                eagle.idle = (here.0, here.1, 0.0);
            } else {
                eagle.idle.2 += dt;
            }
            let punish = frog.started && eagle.idle.2 >= IDLE_LIMIT;
            eagle.timer -= dt;
            if eagle.timer > 0.0 && !punish {
                return;
            }
            eagle.idle.2 = 0.0;
            // Le plus souvent sur la colonne du crapaud, parfois ailleurs.
            let col = if punish || unit(&mut rng) < 0.7 { frog.pos.x.round() as i32 } else { rng.below((COLS * 2 + 1) as u32) as i32 - COLS };
            let dir = if unit(&mut rng) < 0.5 { 1.0 } else { -1.0 };
            eagle.phase = Phase::Warning { col, dir, t: 0.0 };
            if !game_common::url_flag("nostrip") {
            commands.spawn((
                Mesh3d(kit.cube.clone()),
                MeshMaterial3d(ek.warn_on.clone()),
                Transform::from_xyz(col as f32, 0.05, cam.0).with_scale(Vec3::new(0.92, 0.02, 44.0)),
                Strip,
                NotShadowCaster,
            ));
            }
            if !game_common::url_flag("noalarm") {
                sfx.write(Sfx::Alarm);
            }
        }
        Phase::Warning { col, dir, t } => {
            let t = t + dt;
            for (_, mut tf, _, strip) in &mut visuals {
                if strip {
                    tf.translation.z = cam.0;
                }
            }
            if t < WARNING {
                eagle.phase = Phase::Warning { col, dir, t };
                return;
            }
            // L'aigle arrive par le haut ou le bas de l'écran.
            let z = if dir > 0.0 { cam.0 - 15.0 } else { cam.0 + 13.0 };
            let park = crate::extra::Biome::of(frog.max_row_i(), progress.total + score.current).theme_index();
            if !game_common::url_flag("nobird") {
                spawn_eagle(&mut commands, &kit, &extra, &ek, col, dir, z, park);
            }
            for (e, _, _, strip) in &visuals {
                if strip {
                    commands.entity(e).despawn();
                }
            }
            eagle.phase = Phase::Flying { col, dir, z };
            if !game_common::url_flag("noscreech") {
                sfx.write(Sfx::Screech);
            }
        }
        Phase::Flying { col, dir, z } => {
            let z = z + dir * SPEED * dt;
            let held = eagle.grip.is_some();
            let pos = Vec3::new(col as f32, HEIGHT, z);
            for (_, mut tf, shadow, _) in &mut visuals {
                if shadow {
                    tf.translation = Vec3::new(pos.x + 0.5, 0.045, pos.z + 0.5);
                } else {
                    tf.translation = pos;
                }
            }
            if held {
                eagle.grip = Some(pos);
            } else if playing && death.cause.is_none() {
                let at = frog.visual_pos();
                if (at.x - col as f32).abs() < 0.65 && (at.z - z).abs() < 1.0 {
                    eagle.grip = Some(pos);
                    die(Cause::Eagle, &mut death, &mut next);
                    sfx.write(Sfx::Screech);
                }
            }
            let gone = if dir > 0.0 { z > cam.0 + 16.0 } else { z < cam.0 - 18.0 };
            if gone {
                for (e, ..) in &visuals {
                    commands.entity(e).despawn();
                }
                eagle.phase = Phase::Idle;
                eagle.grip = None;
                let d = (frog.max_row_f() / 150.0).clamp(0.0, 1.0);
                eagle.timer = 13.0 + unit(&mut rng) * 12.0 - 6.0 * d;
            } else {
                eagle.phase = Phase::Flying { col, dir, z };
            }
        }
    }
}

fn spawn_eagle(c: &mut Commands, k: &Kit, x: &crate::extra::Extra, e: &EagleKit, col: i32, dir: f32, z: f32, theme: Option<usize>) {
    // Modèle tourné vers -Z, puis pivoté selon le sens du vol.
    let root = c
        .spawn((
            Transform::from_xyz(col as f32, HEIGHT, z).with_rotation(Quat::from_rotation_y(if dir > 0.0 { PI } else { 0.0 })).with_scale(Vec3::splat(1.5)),
            Visibility::default(),
            EagleVisual,
        ))
        .id();
    if let Some(t) = theme {
        // Dans un monde à thème, c'est son animal volant (ptéro, carpe…) qui fond sur le crapaud (modèle tourné vers +X).
        if let Some(i) = crate::gen_worlds::THEMES[t].animals.iter().position(|a| a.flying) {
            let holder = c.spawn((Transform::from_xyz(0.0, -0.6, 0.0).with_rotation(Quat::from_rotation_y(PI / 2.0)), Visibility::default(), ChildOf(root))).id();
            crate::extra::spawn_animal(c, k, x, holder, t, i);
            shadow(c, k, e, col, z);
            return;
        }
    }
    part(c, root, &k.ball, &e.body, Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.5, 0.42, 1.1));
    part(c, root, &k.ball, &e.white, Vec3::new(0.0, 0.1, -0.55), Vec3::splat(0.34));
    part_rot(c, root, &k.cone, &e.beak, Vec3::new(0.0, 0.07, -0.78), Vec3::new(0.12, 0.2, 0.12), Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2));
    for s in [-1.0, 1.0] {
        part(c, root, &k.ball, &e.dark, Vec3::new(0.08 * s, 0.17, -0.65), Vec3::splat(0.07));
    }
    part(c, root, &k.cube, &e.white, Vec3::new(0.0, 0.02, 0.62), Vec3::new(0.34, 0.06, 0.4));
    part(c, root, &k.cube, &e.dark, Vec3::new(0.0, 0.02, 0.86), Vec3::new(0.3, 0.05, 0.1));
    for s in [-1.0, 1.0] {
        part(c, root, &k.cube, &e.beak, Vec3::new(0.12 * s, -0.28, -0.1), Vec3::new(0.06, 0.1, 0.18));
        let wing = c
            .spawn((Transform::from_xyz(0.18 * s, 0.08, 0.0), Visibility::default(), Wing(s), ChildOf(root)))
            .id();
        part(c, wing, &k.cube, &e.body, Vec3::new(0.85 * s, 0.0, 0.1), Vec3::new(1.7, 0.06, 0.62));
        part(c, wing, &k.cube, &e.dark, Vec3::new(1.55 * s, 0.0, 0.18), Vec3::new(0.6, 0.05, 0.5));
    }
    shadow(c, k, e, col, z);
}

fn shadow(c: &mut Commands, k: &Kit, e: &EagleKit, col: i32, z: f32) {
    // Son ombre sur la route
    c.spawn((
        Mesh3d(k.cyl.clone()),
        MeshMaterial3d(e.shadow.clone()),
        Transform::from_xyz(col as f32 + 0.5, 0.045, z + 0.5).with_scale(Vec3::new(1.4, 0.01, 2.6)),
        EagleVisual,
        Shadow,
        NotShadowCaster,
    ));
}

fn flap(time: Res<Time>, mut q: Query<(&Wing, &mut Transform)>) {
    let a = (time.elapsed_secs() * 9.0).sin() * 0.55;
    for (w, mut tf) in &mut q {
        tf.rotation = Quat::from_rotation_z(w.0 * a);
    }
}

/// La bande d'avertissement clignote.
fn blink(time: Res<Time>, ek: Res<EagleKit>, mut q: Query<&mut MeshMaterial3d<StandardMaterial>, With<Strip>>) {
    if game_common::url_flag("noblink") {
        return;
    }
    let on = (time.elapsed_secs() * 7.0) as i32 % 2 == 0;
    for mut m in &mut q {
        m.0 = if on { ek.warn_on.clone() } else { ek.warn_off.clone() };
    }
}
