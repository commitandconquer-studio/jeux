//! Les voies : génération infinie, véhicules, rondins et trains.
use std::f32::consts::PI;

use bevy::audio::Volume;
use bevy::prelude::*;
use game_common::Rng;

use crate::audio::{Mutable, Sfx, SoundOn, Sounds};
use crate::frog::Frog;
use crate::extra::{Biome, Deco, Extra, Scenery, Vehicle, pick_vehicle, spawn_deco, spawn_fly, spawn_floe, spawn_ice_pad, spawn_scenery, spawn_vehicle};
use crate::models::{Kit, Prop, part, spawn_car, spawn_log, spawn_pad, spawn_prop, spawn_train, spawn_truck};

/// Colonnes jouables : de -COLS à +COLS.
pub const COLS: i32 = 4;
/// Demi-largeur du décor de chaque voie.
const HALF: f32 = 18.0;
/// Les véhicules et rondins réapparaissent de l'autre côté au-delà de ±WRAP.
const WRAP: f32 = 16.0;
/// Combien de voies on génère devant et on garde derrière la caméra.
const AHEAD: i32 = 24;
const BEHIND: i32 = 12;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum LaneKind {
    Grass,
    Road,
    River,
    Rail,
}

/// Une voie (une rangée), placée en z = -row.
#[derive(Component)]
pub struct Lane {
    pub row: i32,
    pub kind: LaneKind,
    /// Colonnes occupées par un obstacle (arbre, rocher) sur l'herbe.
    pub blocked: Vec<i32>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MoverKind {
    Car,
    Log,
    Pad,
    Train,
}

/// Tout ce qui se déplace sur une voie (ou flotte, pour les nénuphars).
#[derive(Component)]
pub struct Mover {
    pub row: i32,
    pub speed: f32,
    pub half_len: f32,
    pub kind: MoverKind,
}

impl Mover {
    pub fn is_deadly(&self) -> bool {
        matches!(self.kind, MoverKind::Car | MoverKind::Train)
    }
}

/// Voie ferrée : le feu clignote, puis le train passe.
#[derive(Component)]
pub struct RailLane {
    train: Entity,
    light: Entity,
    /// Sonnerie en cours pendant que le feu clignote.
    bell: Option<Entity>,
    dir: f32,
    timer: f32,
    phase: RailPhase,
}

impl RailLane {
    pub fn light(&self) -> Entity {
        self.light
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum RailPhase {
    Waiting,
    Warning,
    Passing,
}

/// Générateur de voies.
#[derive(Resource)]
pub struct LaneGen {
    next_row: i32,
    rng: Rng,
    queue: Vec<LaneKind>,
    prev: LaneKind,
    reset: bool,
    /// Colonne que chaque voie garde ouverte : de quoi toujours pouvoir avancer.
    corridor: i32,
    /// La rivière précédente avait des rondins (le crapaud arrive n'importe où).
    prev_logs: bool,
}

impl LaneGen {
    pub fn new(seed: u64) -> Self {
        Self { next_row: -8, rng: Rng::new(seed), queue: Vec::new(), prev: LaneKind::Grass, reset: true, corridor: 0, prev_logs: false }
    }

    /// Repart de zéro (appliqué au prochain passage de `ensure_lanes`).
    pub fn request_reset(&mut self, seed: u64) {
        *self = Self::new(seed);
    }

    fn rand(&mut self) -> f32 {
        self.rng.below(10_000) as f32 / 10_000.0
    }

    fn next_kind(&mut self, row: i32, points: u32) -> LaneKind {
        if row < 3 {
            return LaneKind::Grass;
        }
        if self.queue.is_empty() {
            let d = difficulty(row);
            let r = self.rand();
            let (kind, max) = if r < 0.30 - 0.1 * d {
                (LaneKind::Grass, 2)
            } else if r < 0.68 {
                (LaneKind::Road, 2 + (d * 3.0) as u32)
            } else if r < 0.88 {
                (LaneKind::River, 2 + (d * 2.0) as u32)
            } else if crate::progress::unlocked(points, crate::progress::Item::Trains) {
                (LaneKind::Rail, 1)
            } else {
                (LaneKind::Road, 2)
            };
            let n = 1 + self.rng.below(max);
            self.queue.extend(std::iter::repeat_n(kind, n as usize));
        }
        self.queue.pop().unwrap()
    }
}

/// De 0 (début) à 1 (au bout de 150 rangées).
fn difficulty(row: i32) -> f32 {
    (row as f32 / 150.0).clamp(0.0, 1.0)
}

/// Génère les voies devant la caméra et supprime celles loin derrière.
pub fn ensure_lanes(
    mut commands: Commands,
    kit: Res<Kit>,
    extra: Res<Extra>,
    progress: Res<crate::progress::Progress>,
    score: Res<game_common::Score>,
    mut generator: ResMut<LaneGen>,
    cam: Res<crate::CamZ>,
    lanes: Query<(Entity, &Lane)>,
) {
    if generator.reset {
        generator.reset = false;
        for (e, _) in &lanes {
            commands.entity(e).despawn();
        }
        return;
    }
    let cam_row = (-cam.0).round() as i32;
    while generator.next_row <= cam_row + AHEAD {
        spawn_lane(&mut commands, &kit, &extra, &mut generator, progress.total + score.current);
    }
    for (e, lane) in &lanes {
        if lane.row < cam_row - BEHIND {
            commands.entity(e).despawn();
        }
    }
}

fn spawn_lane(c: &mut Commands, kit: &Kit, x: &Extra, g: &mut LaneGen, points: u32) {
    let row = g.next_row;
    g.next_row += 1;
    let kind = g.next_kind(row, points);
    let biome = Biome::of(row, points);
    let after_road = g.prev == LaneKind::Road;
    let after_river = g.prev == LaneKind::River;
    let prev_logs = std::mem::take(&mut g.prev_logs);
    g.prev = kind;

    let root = c.spawn((Transform::from_xyz(0.0, 0.0, -row as f32), Visibility::default())).id();
    let m = &kit.m;
    let center_w = (COLS * 2 + 1) as f32;
    let side_w = HALF - center_w / 2.0;
    let side_x = center_w / 2.0 + side_w / 2.0;
    // Sol en trois morceaux : la zone jouable au centre, plus claire que les côtés.
    let ground = |c: &mut Commands, top: f32, center: &Handle<StandardMaterial>, side: &Handle<StandardMaterial>| {
        let y = top - 0.1;
        part(c, root, &kit.cube, center, Vec3::new(0.0, y, 0.0), Vec3::new(center_w, 0.2, 1.0));
        for s in [-1.0, 1.0] {
            part(c, root, &kit.cube, side, Vec3::new(side_x * s, y, 0.0), Vec3::new(side_w, 0.2, 1.0));
        }
    };

    let mut blocked = Vec::new();
    match kind {
        LaneKind::Grass => {
            let (grass, side) = x.ground(kit, biome, row);
            ground(c, 0.0, grass, side);
            // Obstacles dans la zone jouable
            if row < 0 {
                blocked = (-COLS..=COLS).collect();
            } else if row > 0 {
                let count = if row < 3 { 1 } else { g.rng.below(4) };
                for _ in 0..count {
                    let col = g.rng.below((COLS * 2 + 1) as u32) as i32 - COLS;
                    if !(row < 3 && col == 0) && !blocked.contains(&col) {
                        blocked.push(col);
                    }
                }
            }
            if row >= 0 {
                // Après une rivière, peu d'obstacles : le crapaud arrive où le courant l'a mené.
                if after_river {
                    blocked.truncate(1);
                }
                // Le couloir reste ouvert, et peut glisser d'une colonne sur l'herbe.
                let c0 = g.corridor;
                let mut c1 = c0;
                if g.rand() < 0.35 {
                    c1 = (c0 + if g.rng.below(2) == 0 { -1 } else { 1 }).clamp(-COLS, COLS);
                }
                blocked.retain(|b| *b != c0 && *b != c1);
                g.corridor = c1;
            }
            for &col in &blocked {
                let variant = g.rng.below(6) as usize;
                let at = Vec3::new(col as f32, 0.0, 0.0);
                if biome != Biome::Prairie {
                    let roll = g.rng.below(10);
                    spawn_scenery(c, kit, x, root, biome, Scenery::Block, at, roll, variant);
                    continue;
                }
                match g.rng.below(10) {
                    0..=3 => spawn_prop(c, kit, root, Prop::Tree, at, variant),
                    4..=6 => spawn_prop(c, kit, root, Prop::Pine, at, variant),
                    7..=8 => spawn_prop(c, kit, root, Prop::Rock, at, variant),
                    _ => spawn_deco(c, kit, x, root, Deco::Stump, at, variant),
                }
            }
            // Fleurs, champignons et touffes dans la zone jouable (on passe dessus)
            if row > 0 {
                for _ in 0..g.rng.below(4) {
                    let col = g.rng.below((COLS * 2 + 1) as u32) as i32 - COLS;
                    if blocked.contains(&col) {
                        continue;
                    }
                    if biome != Biome::Prairie {
                        let variant = g.rng.below(12) as usize;
                        let jitter = Vec3::new(g.rand() * 0.3 - 0.15, 0.0, g.rand() * 0.3 - 0.15);
                        spawn_scenery(c, kit, x, root, biome, Scenery::Deco, Vec3::new(col as f32, 0.0, 0.0) + jitter, 0, variant);
                        continue;
                    }
                    let deco = match g.rng.below(10) {
                        0..=2 => Deco::Flower,
                        3..=4 => Deco::Flowers,
                        5 => Deco::Mushroom,
                        _ => Deco::Tuft,
                    };
                    let variant = g.rng.below(12) as usize;
                    let jitter = Vec3::new(g.rand() * 0.3 - 0.15, 0.0, g.rand() * 0.3 - 0.15);
                    spawn_deco(c, kit, x, root, deco, Vec3::new(col as f32, 0.0, 0.0) + jitter, variant);
                }
                // Une mouche à gober, parfois dorée
                if row >= 3 && crate::progress::unlocked(points, crate::progress::Item::Flies) && g.rand() < 0.3 {
                    let col = g.rng.below((COLS * 2 + 1) as u32) as i32 - COLS;
                    if !blocked.contains(&col) {
                        let gold = crate::progress::unlocked(points, crate::progress::Item::GoldFlies) && g.rand() < 0.12;
                        let phase = g.rand() * 6.28;
                        spawn_fly(c, kit, x, root, row, col, gold, phase);
                    }
                }
            }
            // Décor sur les côtés
            for col in (COLS + 1)..(HALF as i32) {
                for s in [-1, 1] {
                    let at = Vec3::new((col * s) as f32, 0.0, 0.0);
                    let variant = g.rng.below(12) as usize;
                    if biome != Biome::Prairie {
                        if g.rng.below(10) < 4 {
                            let roll = g.rng.below(10);
                            spawn_scenery(c, kit, x, root, biome, Scenery::Side, at, roll, variant);
                        } else if g.rng.below(10) < 3 {
                            spawn_scenery(c, kit, x, root, biome, Scenery::Deco, at, 0, variant);
                        }
                        continue;
                    }
                    if g.rng.below(10) < 4 {
                        let prop = match g.rng.below(10) {
                            0..=3 => Prop::Tree,
                            4..=6 => Prop::Pine,
                            7 => Prop::Rock,
                            _ => Prop::Bush,
                        };
                        spawn_prop(c, kit, root, prop, at, variant);
                    } else if g.rng.below(10) < 2 {
                        let deco = match g.rng.below(4) {
                            0 => Deco::Flower,
                            1 => Deco::Flowers,
                            2 => Deco::Mushroom,
                            _ => Deco::Tuft,
                        };
                        spawn_deco(c, kit, x, root, deco, at, variant);
                    }
                }
            }
        }
        LaneKind::Road => {
            ground(c, -0.02, &m.road, &m.road_side);
            if after_road {
                // Pointillés entre deux voies de route
                let mut x = -HALF;
                while x < HALF {
                    part(c, root, &kit.cube, &m.line, Vec3::new(x, -0.015, 0.5), Vec3::new(0.7, 0.01, 0.07));
                    x += 1.6;
                }
            }
            let d = difficulty(row);
            let dir = if g.rng.below(2) == 0 { 1.0 } else { -1.0 };
            let speed = (1.8 + g.rand() * 1.6 + d * 3.0) * dir;
            let mut x_pos = -WRAP + g.rand() * 3.0;
            loop {
                let mut vehicle = pick_vehicle(g.rng.below(1000), points);
                // Dans le parc, surtout les véhicules du parc (ceux déjà gagnés).
                if biome == Biome::Jurassic && g.rng.below(100) < 65 {
                    let open: Vec<usize> = crate::gen_worlds::JURA_VEHICLES.iter().copied().filter(|i| crate::progress::unlocked(points, crate::progress::Item::Vehicle(Vehicle::Gen(*i)))).collect();
                    if !open.is_empty() {
                        vehicle = Vehicle::Gen(open[g.rng.below(open.len() as u32) as usize]);
                    }
                }
                let len = vehicle.len();
                if x_pos + len > WRAP - 2.0 {
                    break;
                }
                let color = g.rng.below(24) as usize;
                let mover = spawn_mover(c, root, row, x_pos + len / 2.0, speed, len, MoverKind::Car, dir);
                match vehicle {
                    Vehicle::Car => spawn_car(c, kit, mover, color),
                    Vehicle::Truck => spawn_truck(c, kit, mover, color),
                    v => spawn_vehicle(c, kit, x, mover, v, color),
                }
                x_pos += len + 2.6 + g.rand() * (5.5 - 2.0 * d);
            }
        }
        LaneKind::River => {
            let (water, water_side) = x.water(kit, biome);
            ground(c, -0.15, water, water_side);
            let frozen = biome == Biome::Snow;
            if g.rng.below(4) == 0 {
                // Nénuphars immobiles : toujours un sur le couloir, et beaucoup
                // après une rivière à rondins (le crapaud n'est pas sur le couloir).
                let dense = if prev_logs { 8 } else { 4 };
                let mut count = 0;
                for col in -COLS..=COLS {
                    if col == g.corridor || g.rng.below(10) < dense {
                        let mover = spawn_mover(c, root, row, col as f32, 0.0, 0.8, MoverKind::Pad, 1.0);
                        if frozen {
                            spawn_ice_pad(c, kit, x, mover);
                        } else {
                            spawn_pad(c, kit, mover, g.rng.below(3) == 0);
                        }
                        count += 1;
                    }
                }
                if count == 0 {
                    let mover = spawn_mover(c, root, row, 0.0, 0.0, 0.8, MoverKind::Pad, 1.0);
                    if frozen {
                        spawn_ice_pad(c, kit, x, mover);
                    } else {
                        spawn_pad(c, kit, mover, true);
                    }
                }
            } else {
                g.prev_logs = true;
                let d = difficulty(row);
                let dir = if g.rng.below(2) == 0 { 1.0 } else { -1.0 };
                let speed = (1.0 + g.rand() * 1.2 + d * 1.5) * dir;
                let mut x_pos = -WRAP + g.rand() * 2.0;
                loop {
                    let len = [2.0, 3.0, 4.0][g.rng.below(3) as usize];
                    if x_pos + len > WRAP - 1.0 {
                        break;
                    }
                    let mover = spawn_mover(c, root, row, x_pos + len / 2.0, speed, len, MoverKind::Log, 1.0);
                    if frozen {
                        spawn_floe(c, kit, x, mover, len);
                    } else {
                        spawn_log(c, kit, mover, len);
                    }
                    x_pos += len + 1.2 + g.rand() * (2.6 - d);
                }
            }
        }
        LaneKind::Rail => {
            ground(c, 0.0, &m.gravel, &m.gravel);
            let mut x = -HALF;
            while x < HALF {
                part(c, root, &kit.cube, &m.sleeper, Vec3::new(x, 0.025, 0.0), Vec3::new(0.18, 0.05, 0.85));
                x += 0.7;
            }
            for z in [-0.28, 0.28] {
                part(c, root, &kit.cube, &m.rail, Vec3::new(0.0, 0.085, z), Vec3::new(HALF * 2.0, 0.07, 0.07));
            }
            // Feu de signalisation au bord de la zone jouable
            let px = COLS as f32 + 1.0;
            part(c, root, &kit.cyl, &m.dark, Vec3::new(px, 0.6, 0.42), Vec3::new(0.09, 1.2, 0.09));
            let light = part(c, root, &kit.ball, &m.signal_off, Vec3::new(px, 1.28, 0.42), Vec3::splat(0.26));
            let dir = if g.rng.below(2) == 0 { 1.0 } else { -1.0 };
            let train = spawn_mover(c, root, row, 0.0, 0.0, 1.0, MoverKind::Train, dir);
            let len = spawn_train(c, kit, train, 2 + g.rng.below(3) as usize);
            c.entity(train).insert((
                Mover { row, speed: 0.0, half_len: len / 2.0, kind: MoverKind::Train },
                Transform::from_xyz(-dir * (WRAP + len / 2.0), 0.0, 0.0)
                    .with_rotation(Quat::from_rotation_y(if dir > 0.0 { 0.0 } else { PI })),
            ));
            c.entity(root).insert(RailLane {
                train,
                light,
                bell: None,
                dir,
                timer: 1.0 + g.rand() * 4.0,
                phase: RailPhase::Waiting,
            });
        }
    }
    c.entity(root).insert(Lane { row, kind, blocked });
}

#[allow(clippy::too_many_arguments)]
fn spawn_mover(c: &mut Commands, lane: Entity, row: i32, x: f32, speed: f32, len: f32, kind: MoverKind, dir: f32) -> Entity {
    // Les véhicules regardent dans leur sens de marche.
    let facing = if speed < 0.0 || dir < 0.0 { PI } else { 0.0 };
    c.spawn((
        Transform::from_xyz(x, 0.0, 0.0).with_rotation(Quat::from_rotation_y(facing)),
        Visibility::default(),
        Mover { row, speed, half_len: len / 2.0, kind },
        ChildOf(lane),
    ))
    .id()
}

pub fn move_movers(time: Res<Time>, mut q: Query<(&Mover, &mut Transform)>) {
    let dt = time.delta_secs();
    for (m, mut tf) in &mut q {
        tf.translation.x += m.speed * dt;
        if m.kind != MoverKind::Train {
            if tf.translation.x > WRAP {
                tf.translation.x -= 2.0 * WRAP;
            } else if tf.translation.x < -WRAP {
                tf.translation.x += 2.0 * WRAP;
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub fn run_rails(
    mut commands: Commands,
    time: Res<Time>,
    kit: Res<Kit>,
    sounds: Res<Sounds>,
    sound_on: Res<SoundOn>,
    frog: Res<Frog>,
    mut sfx: MessageWriter<Sfx>,
    mut rng: ResMut<Rng>,
    mut rails: Query<(Entity, &mut RailLane, &Lane)>,
    mut trains: Query<(&mut Mover, &mut Transform)>,
    mut lights: Query<&mut MeshMaterial3d<StandardMaterial>>,
) {
    let dt = time.delta_secs();
    for (lane_entity, mut rail, lane) in &mut rails {
        // Volume selon la distance au crapaud ; silence au-delà de 8 rangées.
        let volume = (1.0 - (lane.row - frog.row).abs() as f32 / 8.0).max(0.0);
        let Ok((mut mover, mut tf)) = trains.get_mut(rail.train) else {
            continue;
        };
        let lit = match rail.phase {
            RailPhase::Waiting => {
                rail.timer -= dt;
                if rail.timer <= 0.0 {
                    rail.phase = RailPhase::Warning;
                    rail.timer = 1.3;
                    if sound_on.0 && volume > 0.0 {
                        let bell = commands
                            .spawn((
                                AudioPlayer::new(sounds.bell.clone()),
                                PlaybackSettings::LOOP.with_volume(Volume::Linear(0.6 * volume)),
                                Mutable,
                                ChildOf(lane_entity),
                            ))
                            .id();
                        rail.bell = Some(bell);
                    }
                }
                false
            }
            RailPhase::Warning => {
                rail.timer -= dt;
                if rail.timer <= 0.0 {
                    rail.phase = RailPhase::Passing;
                    mover.speed = rail.dir * 30.0;
                    if let Some(bell) = rail.bell.take() {
                        commands.entity(bell).despawn();
                    }
                    if volume > 0.0 {
                        sfx.write(Sfx::Train(volume));
                    }
                }
                (rail.timer * 7.0) as i32 % 2 == 0
            }
            RailPhase::Passing => {
                if tf.translation.x * rail.dir > WRAP + mover.half_len {
                    rail.phase = RailPhase::Waiting;
                    rail.timer = 2.0 + rng.below(400) as f32 / 100.0;
                    mover.speed = 0.0;
                    tf.translation.x = -rail.dir * (WRAP + mover.half_len);
                }
                true
            }
        };
        if let Ok(mut mat) = lights.get_mut(rail.light) {
            let want = if lit { &kit.m.signal_on } else { &kit.m.signal_off };
            if mat.0 != *want {
                mat.0 = want.clone();
            }
        }
    }
}
