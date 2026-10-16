//! Le crapaud : sauts de case en case, rondins, collisions.
use std::f32::consts::{FRAC_PI_2, PI};

use bevy::prelude::*;
use game_common::{Score, tx};

use crate::GameState;
use crate::audio::Sfx;
use crate::lanes::{COLS, Lane, LaneKind, Mover, MoverKind};

const HOP_TIME: f32 = 0.14;
const HOP_HEIGHT: f32 = 0.45;
/// Demi-largeur du crapaud pour les collisions.
const FROG_HALF: f32 = 0.28;

#[derive(Resource, Default)]
pub struct Frog {
    pub pos: Vec3,
    pub row: i32,
    hop: Option<Hop>,
    pub queued: Option<IVec2>,
    /// Rondin ou nénuphar sur lequel il est posé, et décalage par rapport à lui.
    attached: Option<(Entity, f32)>,
    yaw: f32,
    /// Vrai dès le premier saut (la caméra commence alors à avancer).
    pub started: bool,
    hops: u32,
}

struct Hop {
    from: Vec3,
    to: Vec3,
    to_row: i32,
    t: f32,
}

#[derive(Component)]
pub struct FrogModel;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Cause {
    Squashed,
    Drowned,
    CarriedAway,
    TooSlow,
}

impl Cause {
    pub fn message(self) -> &'static str {
        match self {
            Cause::Squashed => tx!("Écrasé !", "Squashed!", "Überfahren!", "Schiacciato!", "¡Aplastado!"),
            Cause::Drowned => tx!(
                "Plouf ! Tombé à l'eau",
                "Splash! Fell in the water",
                "Platsch! Ins Wasser gefallen",
                "Splash! Caduto in acqua",
                "¡Plof! Se cayó al agua"
            ),
            Cause::CarriedAway => tx!(
                "Emporté par le courant",
                "Swept away by the current",
                "Von der Strömung fortgerissen",
                "Trascinato dalla corrente",
                "Arrastrado por la corriente"
            ),
            Cause::TooSlow => tx!("Trop lent !", "Too slow!", "Zu langsam!", "Troppo lento!", "¡Demasiado lento!"),
        }
        .t()
    }
}

#[derive(Resource, Default)]
pub struct Death {
    pub cause: Option<Cause>,
    timer: f32,
}

pub fn die(cause: Cause, death: &mut Death, next: &mut NextState<GameState>) {
    if death.cause.is_none() {
        death.cause = Some(cause);
        death.timer = 0.0;
        next.set(GameState::Dying);
    }
}

/// Hauteur du sol (là où le crapaud se pose) selon le type de voie.
fn ground_height(kind: Option<LaneKind>) -> f32 {
    match kind {
        Some(LaneKind::Road) => -0.02,
        Some(LaneKind::Rail) => 0.05,
        Some(LaneKind::River) => 0.25,
        _ => 0.0,
    }
}

fn lane_at<'a>(lanes: &'a Query<&Lane>, row: i32) -> Option<&'a Lane> {
    lanes.iter().find(|l| l.row == row)
}

/// Clavier (flèches / ZQSD / Espace), clic, et glisser du doigt.
pub fn read_input(
    keys: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    touches: Res<Touches>,
    mut frog: ResMut<Frog>,
) {
    let mut dir = None;
    let bindings = [
        (KeyCode::ArrowUp, KeyCode::KeyW, IVec2::Y),
        (KeyCode::ArrowDown, KeyCode::KeyS, IVec2::NEG_Y),
        (KeyCode::ArrowLeft, KeyCode::KeyA, IVec2::NEG_X),
        (KeyCode::ArrowRight, KeyCode::KeyD, IVec2::X),
    ];
    for (a, b, d) in bindings {
        if keys.just_pressed(a) || keys.just_pressed(b) {
            dir = Some(d);
        }
    }
    if keys.just_pressed(KeyCode::Space) || mouse.just_pressed(MouseButton::Left) {
        dir = Some(IVec2::Y);
    }
    for touch in touches.iter_just_released() {
        let delta = touch.position() - touch.start_position();
        dir = Some(if delta.length() < 30.0 {
            IVec2::Y // simple tape : en avant
        } else if delta.x.abs() > delta.y.abs() {
            IVec2::new(delta.x.signum() as i32, 0)
        } else if delta.y < 0.0 {
            IVec2::Y // l'axe Y de l'écran pointe vers le bas
        } else {
            IVec2::NEG_Y
        });
    }
    if dir.is_some() {
        frog.queued = dir;
    }
}

#[allow(clippy::too_many_arguments)]
pub fn update_frog(
    time: Res<Time>,
    mut frog: ResMut<Frog>,
    lanes: Query<&Lane>,
    movers: Query<(Entity, &Mover, &Transform)>,
    mut score: ResMut<Score>,
    mut death: ResMut<Death>,
    mut next: ResMut<NextState<GameState>>,
    mut sfx: MessageWriter<Sfx>,
) {
    let dt = time.delta_secs();

    // Saut en cours
    if let Some(hop) = frog.hop.as_mut() {
        hop.t += dt / HOP_TIME;
        if hop.t < 1.0 {
            return;
        }
        let (to, to_row) = (hop.to, hop.to_row);
        frog.hop = None;
        frog.pos = to;
        frog.row = to_row;
        if to_row > score.current as i32 {
            score.current = to_row as u32;
            score.best = score.best.max(score.current);
            if score.current % 10 == 0 {
                sfx.write(Sfx::Chime);
            }
        }
        // Atterrissage dans la rivière : sur un rondin / nénuphar, ou à l'eau.
        if lane_at(&lanes, to_row).map(|l| l.kind) == Some(LaneKind::River) {
            let support = movers.iter().find(|(_, m, tf)| {
                m.row == to_row
                    && matches!(m.kind, MoverKind::Log | MoverKind::Pad)
                    && (tf.translation.x - to.x).abs() <= m.half_len + 0.15
            });
            match support {
                Some((e, m, tf)) => {
                    let offset = if m.kind == MoverKind::Pad { 0.0 } else { to.x - tf.translation.x };
                    frog.attached = Some((e, offset));
                    frog.pos.y = if m.kind == MoverKind::Pad { -0.1 } else { 0.25 };
                }
                None => {
                    frog.pos.y = -0.15;
                    die(Cause::Drowned, &mut death, &mut next);
                    sfx.write(Sfx::Splash);
                    return;
                }
            }
        }
    }

    // Porté par un rondin
    if let Some((e, offset)) = frog.attached {
        match movers.get(e) {
            Ok((_, _, tf)) => frog.pos.x = tf.translation.x + offset,
            Err(_) => frog.attached = None,
        }
        if frog.pos.x.abs() > COLS as f32 + 0.6 {
            die(Cause::CarriedAway, &mut death, &mut next);
            sfx.write(Sfx::Splash);
            return;
        }
    }

    // Nouveau saut
    let Some(dir) = frog.queued.take() else {
        return;
    };
    let col = frog.pos.x.round() as i32;
    let target = IVec2::new(col + dir.x, frog.row + dir.y);
    if target.y < 0 || target.x.abs() > COLS {
        return;
    }
    let lane = lane_at(&lanes, target.y);
    if lane.is_some_and(|l| l.kind == LaneKind::Grass && l.blocked.contains(&target.x)) {
        return;
    }
    frog.yaw = match (dir.x, dir.y) {
        (0, 1) => 0.0,
        (0, _) => PI,
        (1, _) => -FRAC_PI_2,
        _ => FRAC_PI_2,
    };
    let to = Vec3::new(target.x as f32, ground_height(lane.map(|l| l.kind)), -target.y as f32);
    let from = frog.pos;
    frog.hop = Some(Hop { from, to, to_row: target.y, t: 0.0 });
    frog.attached = None;
    frog.started = true;
    // Un coassement de temps en temps, sinon le « bloup » du saut.
    frog.hops += 1;
    sfx.write(if frog.hops % 9 == 0 { Sfx::Croak } else { Sfx::Hop });
}

/// Collision avec une voiture, un camion ou un train.
pub fn collide(
    frog: Res<Frog>,
    movers: Query<(&Mover, &Transform)>,
    mut death: ResMut<Death>,
    mut next: ResMut<NextState<GameState>>,
    mut sfx: MessageWriter<Sfx>,
) {
    let (x, row) = match &frog.hop {
        Some(h) if h.t > 0.5 => (h.from.x.lerp(h.to.x, h.t), h.to_row),
        Some(h) => (h.from.x.lerp(h.to.x, h.t), frog.row),
        None => (frog.pos.x, frog.row),
    };
    let hit = movers
        .iter()
        .any(|(m, tf)| m.row == row && m.is_deadly() && (tf.translation.x - x).abs() < m.half_len + FROG_HALF);
    if hit && death.cause.is_none() {
        die(Cause::Squashed, &mut death, &mut next);
        sfx.write(Sfx::Crash);
        sfx.write(Sfx::Honk);
    }
}

/// Après la mort : petite animation, puis écran de fin.
pub fn dying(time: Res<Time>, mut death: ResMut<Death>, mut next: ResMut<NextState<GameState>>) {
    death.timer += time.delta_secs();
    if death.timer > 1.0 {
        next.set(GameState::GameOver);
    }
}

/// Place le modèle 3D : arc de saut, étirement, animations de mort.
pub fn sync_model(time: Res<Time>, frog: Res<Frog>, death: Res<Death>, mut q: Query<&mut Transform, With<FrogModel>>) {
    let mut pos = frog.pos;
    let mut scale = Vec3::ONE;
    let mut yaw = frog.yaw;
    if let Some(h) = &frog.hop {
        let t = h.t.min(1.0);
        let s = (t * PI).sin();
        pos = h.from.lerp(h.to, t) + Vec3::Y * s * HOP_HEIGHT;
        scale = Vec3::new(1.0 - 0.12 * s, 1.0 + 0.3 * s, 1.0 - 0.12 * s);
    } else {
        // Respiration
        scale.y = 1.0 + (time.elapsed_secs() * 3.0).sin() * 0.03;
    }
    match death.cause {
        Some(Cause::Squashed) => scale = Vec3::new(1.45, 0.12, 1.45),
        Some(Cause::Drowned | Cause::CarriedAway) => pos.y -= death.timer.min(1.0) * 0.8,
        Some(Cause::TooSlow) => {
            yaw += death.timer * 12.0;
            scale = Vec3::splat((1.0 - death.timer).max(0.0));
        }
        None => {}
    }
    for mut tf in &mut q {
        tf.translation = pos;
        tf.rotation = Quat::from_rotation_y(yaw);
        tf.scale = scale * 1.2;
    }
}
