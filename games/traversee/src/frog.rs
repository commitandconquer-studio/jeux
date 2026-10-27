//! Le crapaud : sauts de case en case, rondins, collisions.
use std::f32::consts::{FRAC_PI_2, PI};

use bevy::prelude::*;
use game_common::{Score, tx};

use crate::GameState;
use crate::audio::Sfx;
use crate::fx::Fx;
use crate::lanes::{COLS, Lane, LaneKind, Mover, MoverKind};

const HOP_TIME: f32 = 0.14;
const HOP_HEIGHT: f32 = 0.45;
/// Demi-largeur du crapaud pour les collisions.
const FROG_HALF: f32 = 0.26;
/// Taille du modèle.
const FROG_SCALE: f32 = 1.15;

#[derive(Resource, Default)]
pub struct Frog {
    pub pos: Vec3,
    pub row: i32,
    pub hop: Option<Hop>,
    pub queued: Option<IVec2>,
    /// Rondin ou nénuphar sur lequel il est posé, et décalage par rapport à lui.
    attached: Option<(Entity, f32)>,
    /// Cap du personnage : 0 vers le haut de l'écran (-Z), puis ±π/2 et π pour les autres directions.
    pub yaw: f32,
    /// Vrai dès le premier saut (la caméra commence alors à avancer).
    pub started: bool,
    hops: u32,
    /// Rangée la plus lointaine atteinte, et points gagnés avec les mouches.
    max_row: i32,
    bonus: u32,
    /// Temps restant de l'écrasement à l'atterrissage.
    land: f32,
}

const LAND_TIME: f32 = 0.14;
/// Durée de l'image figée quand un véhicule percute le crapaud.
const IMPACT_FREEZE: f32 = 0.12;
/// Durée du vol du crapaud projeté.
const FLIGHT_TIME: f32 = 0.8;

pub struct Hop {
    from: Vec3,
    to: Vec3,
    to_row: i32,
    t: f32,
}

impl Frog {
    /// Où se trouve le crapaud à l'écran (au sol, en plein saut compris).
    pub fn max_row_i(&self) -> i32 {
        self.max_row
    }

    /// Rangée la plus lointaine atteinte (pour régler la difficulté).
    pub fn max_row_f(&self) -> f32 {
        self.max_row as f32
    }

    pub fn visual_pos(&self) -> Vec3 {
        match &self.hop {
            Some(h) => h.from.lerp(h.to, h.t.min(1.0)),
            None => self.pos,
        }
    }
}

#[derive(Component)]
pub struct FrogModel;

/// Ombre ou halo posé sous le crapaud (pour le repérer entre les véhicules).
#[derive(Component)]
pub struct FrogShadow {
    halo: bool,
}

/// Détails propres à Traversée, ajoutés au crapaud commun : dos rayé, joues
/// roses, narines, reflets dans les yeux, orteils, gorge claire.
pub fn decorate(commands: &mut Commands, frog: Entity, kit: &crate::models::Kit, mats: &mut Assets<StandardMaterial>) {
    let mut m = |r: f32, g: f32, b: f32| mats.add(game_common::flat_material(Color::srgb(r, g, b)));
    let stripe = m(0.62, 0.86, 0.42);
    let cheek = m(0.98, 0.58, 0.62);
    let dark = m(0.14, 0.34, 0.14);
    let toe = m(0.28, 0.58, 0.24);
    let white = m(1.0, 1.0, 1.0);
    let throat = m(0.92, 0.95, 0.66);
    let (cube, ball) = (&kit.cube, &kit.ball);
    crate::models::part(commands, frog, cube, &stripe, Vec3::new(0.0, 0.436, 0.1), Vec3::new(0.1, 0.02, 0.42));
    crate::models::part(commands, frog, cube, &throat, Vec3::new(0.0, 0.2, -0.455), Vec3::new(0.3, 0.1, 0.03));
    for s in [-1.0, 1.0] {
        crate::models::part(commands, frog, ball, &cheek, Vec3::new(0.23 * s, 0.28, -0.4), Vec3::new(0.1, 0.1, 0.05));
        crate::models::part(commands, frog, cube, &dark, Vec3::new(0.07 * s, 0.445, -0.435), Vec3::splat(0.03));
        crate::models::part(commands, frog, ball, &white, Vec3::new(0.145 * s, 0.545, -0.44), Vec3::splat(0.04));
        // Orteils : trois par patte avant, trois par patte arrière
        for dx in [-0.055, 0.0, 0.055] {
            crate::models::part(commands, frog, cube, &toe, Vec3::new(0.27 * s + dx, 0.02, -0.5), Vec3::new(0.04, 0.035, 0.09));
            crate::models::part(commands, frog, cube, &toe, Vec3::new(0.43 * s + dx, 0.02, -0.14), Vec3::new(0.04, 0.035, 0.09));
        }
        // Tache sur chaque flanc
        crate::models::part(commands, frog, cube, &dark, Vec3::new(0.315 * s, 0.3, 0.12), Vec3::new(0.02, 0.1, 0.12));
    }
}

pub fn spawn_shadow(commands: &mut Commands, kit: &crate::models::Kit, mats: &mut Assets<StandardMaterial>) {
    let blend = |mats: &mut Assets<StandardMaterial>, c: Color| {
        mats.add(StandardMaterial { unlit: true, alpha_mode: AlphaMode::Blend, ..game_common::flat_material(c) })
    };
    let dark = blend(mats, Color::srgba(0.05, 0.1, 0.05, 0.45));
    for (mat, halo) in [(dark, false)] {
        commands.spawn((
            Mesh3d(kit.cyl.clone()),
            MeshMaterial3d(mat),
            Transform::from_scale(Vec3::ZERO),
            FrogShadow { halo },
            bevy::light::NotShadowCaster,
        ));
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Cause {
    Squashed,
    Drowned,
    /// Tombé dans le vide (Cosmos Croâ).
    Void,
    CarriedAway,
    TooSlow,
    Eagle,
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
            Cause::Void => tx!(
                "Perdu dans le vide !",
                "Lost in the void!",
                "Im Nichts verloren!",
                "Perso nel vuoto!",
                "¡Perdido en el vacío!"
            ),
            Cause::CarriedAway => tx!(
                "Emporté par le courant",
                "Swept away by the current",
                "Von der Strömung fortgerissen",
                "Trascinato dalla corrente",
                "Arrastrado por la corriente"
            ),
            Cause::Eagle => tx!(
                "Enlevé par l'aigle !",
                "Carried off by the eagle!",
                "Vom Adler geholt!",
                "Rapito dall'aquila!",
                "¡Se lo llevó el águila!"
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
    /// Où le choc projette le crapaud (selon la vitesse et le sens du véhicule).
    pub fling: Vec3,
    /// Hauteur du vol.
    pub fling_height: f32,
    /// La voiture qui a tué (pour la faire exploser en mode gore) et où.
    pub killer: Option<(Entity, Vec3)>,
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
    mut fx: MessageWriter<Fx>,
    mut commands: Commands,
    flies: Query<(Entity, &crate::extra::Fly)>,
    progress: Res<crate::progress::Progress>,
) {
    let dt = time.delta_secs();
    frog.land = (frog.land - dt).max(0.0);

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
        frog.land = LAND_TIME;
        if to_row > frog.max_row {
            frog.max_row = to_row;
            if to_row % 10 == 0 {
                sfx.write(Sfx::Chime);
            }
            // Un nouveau monde commence : on le dit à l'écran.
            if to_row > 0 && to_row % crate::extra::BIOME_LEN == 0 {
                let points = progress.total + frog.max_row as u32 + frog.bonus;
                let world = crate::extra::Biome::of(to_row, points);
                if world != crate::extra::Biome::Prairie {
                    sfx.write(Sfx::Start);
                    fx.write(Fx::World(world));
                }
            }
        }
        // Une mouche sur la case : on la gobe.
        let col = to.x.round() as i32;
        if let Some((e, fly)) = flies.iter().find(|(_, f)| f.row == to_row && f.col == col) {
            commands.entity(e).despawn();
            frog.bonus += fly.points();
            sfx.write(if fly.gold { Sfx::Power } else { Sfx::Eat });
            fx.write(Fx::Gulp(Vec3::new(to.x, to.y + 0.5, to.z), fly.points()));
        }
        score.current = frog.max_row as u32 + frog.bonus;
        score.best = score.best.max(score.current);
        // Atterrissage dans la rivière : sur un rondin / nénuphar, ou à l'eau.
        if lane_at(&lanes, to_row).map(|l| l.kind) == Some(LaneKind::River) {
            let support = movers.iter().find(|(_, m, tf)| {
                m.row == to_row
                    && matches!(m.kind, MoverKind::Log | MoverKind::Pad)
                    && (tf.translation.x - to.x).abs() <= m.half_len + 0.15
            });
            match support {
                Some((e, m, tf)) => {
                    fx.write(Fx::Ripple(Vec3::new(to.x, 0.2, to.z)));
                    let offset = if m.kind == MoverKind::Pad { 0.0 } else { to.x - tf.translation.x };
                    frog.attached = Some((e, offset));
                    frog.pos.y = if m.kind == MoverKind::Pad { -0.1 } else { 0.25 };
                }
                None => {
                    frog.pos.y = -0.15;
                    if crate::space::in_space() {
                        die(Cause::Void, &mut death, &mut next);
                        sfx.write(Sfx::Fall);
                    } else {
                        die(Cause::Drowned, &mut death, &mut next);
                        sfx.write(Sfx::Splash);
                    }
                    return;
                }
            }
        } else {
            fx.write(Fx::Dust(to));
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
    fx.write(Fx::Dust(from));
    // Un coassement de temps en temps, sinon le « bloup » du saut.
    frog.hops += 1;
    sfx.write(if frog.hops % 9 == 0 { Sfx::Croak } else { Sfx::Hop });
}

/// Collision avec une voiture, un camion ou un train.
pub fn collide(
    frog: Res<Frog>,
    movers: Query<(Entity, &Mover, &Transform)>,
    mut death: ResMut<Death>,
    mut next: ResMut<NextState<GameState>>,
    mut sfx: MessageWriter<Sfx>,
    reglages: Res<game_common::audio::Reglages>,
) {
    let (x, row) = match &frog.hop {
        Some(h) if h.t > 0.5 => (h.from.x.lerp(h.to.x, h.t), h.to_row),
        Some(h) => (h.from.x.lerp(h.to.x, h.t), frog.row),
        None => (frog.pos.x, frog.row),
    };
    let hit = movers
        .iter()
        .find(|(_, m, tf)| m.row == row && m.is_deadly() && (tf.translation.x - x).abs() < m.half_len + FROG_HALF);
    if let Some((car, m, tf)) = hit
        && death.cause.is_none()
    {
        die(Cause::Squashed, &mut death, &mut next);
        // Un train l'envoie bien plus loin et plus haut qu'une voiture.
        let train = m.kind == MoverKind::Train;
        death.fling = Vec3::new(m.speed.signum() * if train { 9.0 } else { 3.6 }, 0.0, -0.8);
        death.fling_height = if train { 5.5 } else { 3.2 };
        death.killer = Some((car, Vec3::new(tf.translation.x, 0.5, -(m.row as f32))));
        match reglages.style_mort {
            0 => {
                sfx.write(Sfx::Chime);
            }
            _ => {
                sfx.write(Sfx::Crash);
                sfx.write(Sfx::Honk);
            }
        }
    }
}

/// Après la mort : petite animation, puis écran de fin. Un appui saute l'attente.
/// Le temps réel compte : le ralenti de l'impact ne rallonge pas la pause.
pub fn dying(
    time: Res<Time<Real>>,
    keys: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    touches: Res<Touches>,
    mut death: ResMut<Death>,
    mut virt: ResMut<Time<Virtual>>,
    mut next: ResMut<NextState<GameState>>,
) {
    death.timer += time.delta_secs();
    if death.cause == Some(Cause::Squashed) {
        // Image figée à l'impact, puis ralenti, puis retour à la normale.
        virt.set_relative_speed(match death.timer {
            t if t < IMPACT_FREEZE => 0.04,
            t if t < 0.7 => 0.3,
            _ => 1.0,
        });
    } else {
        virt.set_relative_speed(1.0);
    }
    let pressed = keys.get_just_pressed().next().is_some()
        || mouse.just_pressed(MouseButton::Left)
        || touches.iter_just_pressed().next().is_some();
    if death.timer > 1.4 || (death.timer > 0.6 && pressed) {
        next.set(GameState::GameOver);
    }
}

/// Place le modèle 3D : arc de saut, étirement, écrasement à l'atterrissage,
/// animations de mort, et l'ombre posée au sol.
pub fn sync_model(
    time: Res<Time>,
    frog: Res<Frog>,
    death: Res<Death>,
    mut q: Query<&mut Transform, With<FrogModel>>,
    mut shadows: Query<(&mut Transform, &FrogShadow), Without<FrogModel>>,
    eagle: Res<crate::eagle::Eagle>,
) {
    let mut pos = frog.pos;
    let mut ground = frog.pos;
    let mut scale = Vec3::ONE;
    let mut yaw = frog.yaw;
    let mut lift = 0.0;
    let mut tumble = Vec3::ZERO;
    if let Some(h) = &frog.hop {
        let t = h.t.min(1.0);
        let s = (t * PI).sin();
        ground = h.from.lerp(h.to, t);
        lift = s;
        pos = ground + Vec3::Y * s * HOP_HEIGHT;
        scale = Vec3::new(1.0 - 0.12 * s, 1.0 + 0.3 * s, 1.0 - 0.12 * s);
    } else if frog.land > 0.0 {
        // Petit écrasement à l'arrivée : le crapaud « pèse ».
        let k = (frog.land / LAND_TIME * PI).sin();
        scale = Vec3::new(1.0 + 0.16 * k, 1.0 - 0.22 * k, 1.0 + 0.16 * k);
    } else {
        // Respiration
        scale.y = 1.0 + (time.elapsed_secs() * 3.0).sin() * 0.03;
    }
    let mut shadow_on = true;
    match death.cause {
        Some(Cause::Squashed) => {
            let t = death.timer;
            if t < IMPACT_FREEZE {
                // Écrasé net sur place, le temps d'un battement.
                scale = Vec3::new(1.4, 0.4, 1.4);
            } else {
                // Puis projeté en l'air en tournoyant, et retombe à plat.
                let a = ((t - IMPACT_FREEZE) / FLIGHT_TIME).min(1.0);
                pos += death.fling * a + Vec3::Y * 4.0 * a * (1.0 - a) * death.fling_height;
                tumble = Vec3::new(a * 17.0, a * 9.0, a * 12.0) * (1.0 - a * 0.1);
                scale = if a < 1.0 { Vec3::ONE } else { Vec3::new(1.45, 0.12, 1.45) };
                if a >= 1.0 {
                    tumble = Vec3::ZERO;
                    pos.y = ground.y;
                    yaw = 0.0;
                }
            }
        }
        Some(Cause::Drowned | Cause::CarriedAway) => {
            pos.y -= death.timer.min(1.0) * 0.8;
            shadow_on = false;
        }
        Some(Cause::Void) => {
            // Il tombe, de plus en plus vite, en tournant lentement.
            pos.y -= death.timer * death.timer * 14.0;
            tumble = Vec3::new(death.timer * 3.0, death.timer * 2.0, death.timer * 4.0);
            scale = Vec3::splat((1.0 - death.timer * 0.5).max(0.3));
            shadow_on = false;
        }
        Some(Cause::Eagle) => {
            // Dans les serres de l'aigle : il se débat en l'air.
            if let Some(g) = eagle.grip() {
                pos = g + Vec3::new(0.0, -0.55, 0.0);
            }
            tumble = Vec3::new((death.timer * 9.0).sin() * 0.5, death.timer * 6.0, (death.timer * 7.0).cos() * 0.4);
            scale = Vec3::splat(0.85);
            shadow_on = false;
        }
        Some(Cause::TooSlow) => {
            yaw += death.timer * 12.0;
            scale = Vec3::splat((1.0 - death.timer).max(0.0));
            shadow_on = false;
        }
        None => {}
    }
    for mut tf in &mut q {
        tf.translation = pos;
        tf.rotation = Quat::from_rotation_y(yaw) * Quat::from_euler(EulerRot::XYZ, tumble.x, tumble.y, tumble.z);
        tf.scale = scale * FROG_SCALE;
    }
    for (mut tf, shadow) in &mut shadows {
        let size = if shadow_on { (if shadow.halo { 1.6 } else { 1.0 }) * (1.0 - 0.35 * lift) } else { 0.0 };
        tf.translation = Vec3::new(ground.x, ground.y + if shadow.halo { 0.02 } else { 0.03 }, ground.z);
        tf.scale = Vec3::new(size, 0.01, size);
    }
}
