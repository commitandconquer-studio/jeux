//! Règles de la chenille : déplacement, invités à attraper, collisions.
use std::collections::VecDeque;
use std::f32::consts::{PI, TAU};
use std::time::Duration;

use bevy::prelude::*;
use game_common::{Rng, Score, flat_material, palette};

use crate::GameState;
use crate::audio::Sfx;
use crate::board::{Board, Feature, Tile};
use crate::dress::LOOKS;
use crate::people::{self, Pose};
use crate::progress::Progress;
use crate::settings::Settings;
use crate::layout::Layout;
use crate::voyage::{Mode, Outcome, Run, Scenery, Voyage, stars_for};
use crate::world::cell_to_world;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Conga::new(&Layout::default(), 0))
            .insert_resource(Food { cell: IVec2::ZERO, kind: 1, look: 0, spawned: true, active: true })
            .init_resource::<Demo>()
            .insert_resource(Paused(false))
            .insert_resource(Boost(0))
            .insert_resource(Crash::default())
            .insert_resource(Banner::default())
            .init_resource::<Board>()
            .insert_resource(StepTimer(Timer::from_seconds(0.16, TimerMode::Repeating)))
            .add_systems(Startup, (setup, load_best, crate::dress::setup))
            .add_systems(OnEnter(GameState::Playing), (new_game, new_run).chain())
            .add_systems(OnEnter(GameState::Menu), clear_board)
            .add_systems(OnEnter(GameState::GameOver), (people::all_die, save_best))
            .add_systems(
                Update,
                (
                    (crate::board::sync_tiles, crate::board::animate_tiles),
                    (read_input, autopilot, step, exit_sequence, finish_crash).chain().run_if(in_state(GameState::Playing)),
                    (sync_guest, sync_segments).run_if(|s: Res<State<GameState>>| {
                        matches!(s.get(), GameState::Playing | GameState::GameOver)
                    }),
                    (animate_guest, animate_pop, people::start_animations, crate::dress::dress, people::attach_guide),
                )
                    .chain(),
            );
    }
}

/// Réglages d'essai et de captures (voir `capture.rs`).
#[derive(Resource)]
pub struct Demo {
    /// File de cette longueur au départ (`SNAKE_LONG=n`).
    pub long: Option<usize>,
    /// Cases spéciales déjà posées.
    pub features: bool,
    /// Dossier de presse : partie figée, sans interface.
    pub press: bool,
    /// Pilote automatique (`SNAKE_AUTO=n` : n fois plus vite), pour essayer les niveaux.
    pub auto: Option<f32>,
}

impl Default for Demo {
    fn default() -> Self {
        let press = std::env::var("SNAKE_PRESSE").is_ok();
        Self {
            long: std::env::var("SNAKE_LONG").ok().and_then(|v| v.parse().ok()),
            features: press || std::env::var("SNAKE_CAPTURE").is_ok(),
            press,
            auto: std::env::var("SNAKE_AUTO").ok().and_then(|v| v.parse().ok()),
        }
    }
}

/// `true` quand la partie est en pause.
#[derive(Resource)]
pub struct Paused(pub bool);

/// Choc en cours : la tête avance jusqu'au mur (ou à l'obstacle) avant que la partie ne s'arrête.
#[derive(Resource, Default)]
struct Crash {
    left: f32,
    dir: IVec2,
    active: bool,
}

const CRASH_TIME: f32 = 0.14;

/// Pas restants d'élan donné par un accélérateur.
#[derive(Resource)]
struct Boost(u32);

/// Annonce affichée à l'écran quelques secondes (nouvel élément sur le plateau).
#[derive(Resource, Default)]
pub struct Banner {
    pub text: String,
    pub left: f32,
}

/// Ordre dans lequel les éléments arrivent, tous les `LEVEL_EVERY` invités.
const SEQUENCE: [Feature; 16] = [
    Feature::Obstacle,
    Feature::Speeder,
    Feature::Hole,
    Feature::Teleport,
    Feature::Belt,
    Feature::Obstacle,
    Feature::Speeder,
    Feature::Teleport,
    Feature::Hole,
    Feature::Obstacle,
    Feature::Belt,
    Feature::Obstacle,
    Feature::Teleport,
    Feature::Hole,
    Feature::Speeder,
    Feature::Obstacle,
];
const LEVEL_EVERY: u32 = 5;

fn cap(f: Feature) -> usize {
    match f {
        Feature::Obstacle => 10,
        Feature::Hole => 4,
        Feature::Teleport => 6,
        Feature::Speeder => 4,
        Feature::Belt => 8,
    }
}

pub fn feature_name(f: Feature) -> &'static str {
    use game_common::tx;
    match f {
        Feature::Obstacle => tx!("Des obstacles !", "Obstacles!", "Hindernisse!", "Ostacoli!", "¡Obstáculos!"),
        Feature::Hole => tx!("Attention aux trous !", "Mind the holes!", "Vorsicht, Löcher!", "Attenzione alle buche!", "¡Cuidado con los hoyos!"),
        Feature::Teleport => tx!("Téléporteurs !", "Teleporters!", "Teleporter!", "Teletrasporti!", "¡Teletransportadores!"),
        Feature::Speeder => tx!("Accélérateurs !", "Speed pads!", "Turbo-Felder!", "Acceleratori!", "¡Aceleradores!"),
        Feature::Belt => tx!("Tapis roulants !", "Conveyor belts!", "Förderbänder!", "Nastri trasportatori!", "¡Cintas transportadoras!"),
    }
    .t()
}

/// La chenille : la tête en premier, puis les invités attrapés.
#[derive(Resource)]
struct Conga {
    cells: Vec<IVec2>,
    /// Positions au pas précédent, pour animer le glissement entre deux cases.
    prev: Vec<IVec2>,
    /// Modèle de chaque membre (indice dans `people::KINDS`).
    kinds: Vec<usize>,
    /// Numéro d'apparence de chaque membre (couleurs de peau, cheveux, vêtements).
    looks: Vec<u32>,
    /// Pour un membre arrivé par un téléporteur : la case d'entrée (il y disparaît, puis ressort à la sienne).
    vias: Vec<Option<IVec2>>,
    dir: IVec2,
    queued: VecDeque<IVec2>,
}

impl Conga {
    fn new(layout: &Layout, leader: usize) -> Self {
        let pal = |k: usize| (k + 1) % 6;
        let cells = layout.start_cells();
        let kinds = vec![leader, pal(leader), pal(pal(leader) + 2)];
        let looks = vec![leader as u32 * 7 + 3, 11, 58];
        Self { prev: cells.clone(), vias: vec![None; cells.len()], cells, kinds, looks, dir: layout.dir, queued: VecDeque::new() }
    }

    /// Mémorise un virage ; renvoie `true` s'il est accepté.
    fn turn(&mut self, d: IVec2) -> bool {
        let last = self.queued.back().copied().unwrap_or(self.dir);
        let ok = d != last && d != -last && self.queued.len() < 3;
        if ok {
            self.queued.push_back(d);
        }
        ok
    }
}

/// L'invité à attraper.
#[derive(Resource)]
struct Food {
    cell: IVec2,
    kind: usize,
    look: u32,
    /// `false` : plus d'invité à attraper (objectif atteint).
    active: bool,
    /// `false` : son modèle reste à poser.
    spawned: bool,
}

#[derive(Resource)]
struct StepTimer(Timer);

#[derive(Resource)]
struct GuestAssets {
    ring: Handle<Mesh>,
    ring_mat: Handle<StandardMaterial>,
}

/// Un membre de la chenille (racine du modèle).
#[derive(Component)]
struct Segment {
    index: usize,
    yaw: f32,
}

#[derive(Component)]
struct Guest;

#[derive(Component)]
struct Ring;

/// Petit rebond à l'arrivée d'un nouveau membre.
#[derive(Component)]
struct Pop(f32);

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    commands.insert_resource(GuestAssets {
        ring: meshes.add(Cylinder::new(0.42, 0.04)),
        ring_mat: materials.add(flat_material(palette::PINK)),
    });
}

fn load_best(mut score: ResMut<Score>) {
    if let Some(b) = game_common::save::load("snake", "best").and_then(|v| v.parse().ok()) {
        score.best = b;
    }
}

fn save_best(score: Res<Score>) {
    game_common::save::store("snake", "best", &score.best.to_string());
}

#[allow(clippy::too_many_arguments)]
fn new_game(
    mut commands: Commands,
    settings: Res<Settings>,
    layout: Res<Layout>,
    mut conga: ResMut<Conga>,
    mut food: ResMut<Food>,
    mut timer: ResMut<StepTimer>,
    mut score: ResMut<Score>,
    mut paused: ResMut<Paused>,
    mut board: ResMut<Board>,
    mut boost: ResMut<Boost>,
    mut crash: ResMut<Crash>,
    progress: Res<Progress>,
    mut rng: ResMut<Rng>,
    stuff: Query<Entity, Or<(With<Segment>, With<Guest>)>>,
    mut sfx: MessageWriter<Sfx>,
) {
    sfx.write(Sfx::Start);
    for e in &stuff {
        commands.entity(e).despawn();
    }
    paused.0 = false;
    board.clear();
    boost.0 = 0;
    *crash = Crash::default();
    *conga = Conga::new(&layout, settings.leader);
    food.cell = free_cell(&conga.cells, &board, &layout, &mut rng);
    food.kind = guest_kind(&mut rng, &progress);
    food.look = rng.below(LOOKS);
    food.spawned = false;
    timer.0 = Timer::from_seconds(settings.speed.step_times().0, TimerMode::Repeating);
    score.reset();
}

/// Démarre le niveau (Voyage) ou la partie infinie : rythme, objectif, cases du niveau.
#[allow(clippy::too_many_arguments)]
fn new_run(
    mode: Res<Mode>,
    mut run: ResMut<Run>,
    mut outcome: ResMut<Outcome>,
    mut timer: ResMut<StepTimer>,
    mut food: ResMut<Food>,
    progress: Res<crate::voyage::VoyageProgress>,
    unlocked: Res<Progress>,
    layout: Res<Layout>,
    scenery: Res<Scenery>,
    demo: Res<Demo>,
    mut conga: ResMut<Conga>,
    mut board: ResMut<Board>,
    mut paused: ResMut<Paused>,
    mut rng: ResMut<Rng>,
) {
    let progress_v = progress;
    let progress = unlocked;
    *run = Run::default();
    *outcome = Outcome::default();
    food.active = true;
    // Captures d'écran : tous les éléments sont déjà posés.
    if demo.features {
        let busy = conga.cells.clone();
        for f in [Feature::Obstacle, Feature::Obstacle, Feature::Obstacle, Feature::Hole, Feature::Teleport, Feature::Teleport, Feature::Speeder, Feature::Belt, Feature::Belt] {
            board.place(f, &busy, conga.cells[0], &layout, scenery.0, &mut rng);
        }
    }
    // Essai : une très longue file (SNAKE_LONG=n) pour vérifier la variété des apparences.
    if let Some(n) = demo.long {
        let mut path = Vec::new();
        for y in 0..layout.h {
            let xs: Vec<i32> = if y % 2 == 0 { (0..layout.w).collect() } else { (0..layout.w).rev().collect() };
            path.extend(xs.into_iter().map(|x| IVec2::new(x, y)).filter(|c| layout.walkable(*c)));
        }
        path.truncate(n.min(path.len()));
        path.reverse();
        let pool = progress.persons();
        conga.cells = path.clone();
        conga.prev = path;
        conga.kinds = (0..conga.cells.len()).map(|_| pool[rng.below(pool.len() as u32) as usize]).collect();
        conga.looks = (0..conga.cells.len() as u32).map(|i| i * 5 % LOOKS).collect();
        conga.vias = vec![None; conga.cells.len()];
        conga.dir = if conga.cells.len() > 1 { conga.cells[0] - conga.cells[1] } else { IVec2::X };
        timer.0 = Timer::from_seconds(5.0, TimerMode::Repeating);
    }
    paused.0 = demo.press;
    if let Mode::Voyage(i) = *mode {
        outcome.best_before = progress_v.stars(i);
        if let Some(level) = mode.level() {
            timer.0 = Timer::from_seconds(level.pace / demo.auto.unwrap_or(1.0), TimerMode::Repeating);
        }
    }
}

fn clear_board(mut commands: Commands, stuff: Query<Entity, Or<(With<Segment>, With<Guest>)>>) {
    for e in &stuff {
        commands.entity(e).despawn();
    }
}

/// Un invité au hasard parmi les personnages débloqués.
fn guest_kind(rng: &mut Rng, progress: &Progress) -> usize {
    let pool = progress.persons();
    pool[rng.below(pool.len() as u32) as usize]
}

/// Clavier, manette, glissement du doigt ; pause avec P, Espace ou la manette.
/// Pilote automatique de test : va vers l'invité (ou la sortie) en évitant murs, corps et obstacles.
fn autopilot(
    demo: Res<Demo>,
    layout: Res<Layout>,
    board: Res<Board>,
    food: Res<Food>,
    run: Res<Run>,
    mut conga: ResMut<Conga>,
) {
    if demo.auto.is_none() || !conga.queued.is_empty() {
        return;
    }
    let target = run.exit.unwrap_or(food.cell);
    let head = conga.cells[0];
    let body = &conga.cells[..conga.cells.len() - 1];
    let safe = |c: IVec2| layout.walkable(c) && !body.contains(&c) && !matches!(board.tiles.get(&c), Some(Tile::Obstacle(_) | Tile::Hole));
    let best = [IVec2::X, IVec2::NEG_X, IVec2::Y, IVec2::NEG_Y]
        .into_iter()
        .filter(|d| *d != -conga.dir && safe(head + *d))
        .min_by_key(|d| ((head + *d) - target).abs().element_sum());
    if let Some(d) = best
        && d != conga.dir
    {
        conga.queued.push_back(d);
    }
}

fn read_input(
    keys: Res<ButtonInput<KeyCode>>,
    pads: Query<&Gamepad>,
    touches: Res<Touches>,
    mut swiped: Local<Vec<u64>>,
    mut paused: ResMut<Paused>,
    mut conga: ResMut<Conga>,
    mut sfx: MessageWriter<Sfx>,
) {
    if keys.just_pressed(KeyCode::KeyP) || keys.just_pressed(KeyCode::Space) || pads.iter().any(|p| p.just_pressed(GamepadButton::Start)) {
        paused.0 = !paused.0;
    }
    // Touches physiques : W/A/S/D correspondent à Z/Q/S/D sur un clavier AZERTY.
    let bindings = [
        (KeyCode::ArrowUp, KeyCode::KeyW, GamepadButton::DPadUp, IVec2::NEG_Y),
        (KeyCode::ArrowDown, KeyCode::KeyS, GamepadButton::DPadDown, IVec2::Y),
        (KeyCode::ArrowLeft, KeyCode::KeyA, GamepadButton::DPadLeft, IVec2::NEG_X),
        (KeyCode::ArrowRight, KeyCode::KeyD, GamepadButton::DPadRight, IVec2::X),
    ];
    let mut wanted: Vec<IVec2> = Vec::new();
    for (a, b, pad, d) in bindings {
        if keys.just_pressed(a) || keys.just_pressed(b) || pads.iter().any(|p| p.just_pressed(pad)) {
            wanted.push(d);
        }
    }
    // Glissement : un virage dès que le doigt a parcouru assez de chemin.
    for t in touches.iter() {
        if swiped.contains(&t.id()) {
            continue;
        }
        let v = t.position() - t.start_position();
        if v.length() > 28.0 {
            swiped.push(t.id());
            wanted.push(if v.x.abs() > v.y.abs() { IVec2::new(v.x.signum() as i32, 0) } else { IVec2::new(0, v.y.signum() as i32) });
        }
    }
    swiped.retain(|id| touches.get_pressed(*id).is_some());
    if paused.0 {
        return;
    }
    for d in wanted {
        if conga.turn(d) {
            sfx.write(Sfx::Turn);
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn step(
    time: Res<Time>,
    settings: Res<Settings>,
    layout: Res<Layout>,
    paused: Res<Paused>,
    mut timer: ResMut<StepTimer>,
    mut conga: ResMut<Conga>,
    mut food: ResMut<Food>,
    mut score: ResMut<Score>,
    mut rng: ResMut<Rng>,
    mut board: ResMut<Board>,
    mut boost: ResMut<Boost>,
    mut banner: ResMut<Banner>,
    progress: Res<Progress>,
    mut crash: ResMut<Crash>,
    mut voyage: Voyage,
    mut sfx: MessageWriter<Sfx>,
) {
    banner.left = (banner.left - time.delta_secs()).max(0.0);
    if paused.0 || crash.active {
        return;
    }
    voyage.run.elapsed += time.delta_secs();
    voyage.run.stepped = false;
    // L'élan d'un accélérateur fait avancer le temps deux fois plus vite.
    let dt = if boost.0 > 0 { time.delta() * 2 } else { time.delta() };
    if !timer.0.tick(dt).just_finished() {
        return;
    }
    voyage.run.stepped = true;
    // Défilé de sortie : `exit_sequence` fait avancer la file.
    if voyage.run.exiting {
        return;
    }
    boost.0 = boost.0.saturating_sub(1);
    // Un tapis roulant impose sa direction (sauf à contresens).
    if let Some(Tile::Belt(d)) = board.tiles.get(&conga.cells[0])
        && *d != -conga.dir
    {
        conga.dir = *d;
        conga.queued.clear();
    } else if let Some(d) = conga.queued.pop_front() {
        conga.dir = d;
    }
    let mut head = conga.cells[0] + conga.dir;
    // Hors de la forme du plateau (bord ou case bloquée) : mur, ou passage de l'autre côté si les bords sont traversables.
    let mut outside = !layout.walkable(head);
    if outside && settings.wrap && layout.is_full() {
        head = head.rem_euclid(IVec2::new(layout.w, layout.h));
        outside = false;
    }
    let deadly = matches!(board.tiles.get(&head), Some(Tile::Obstacle(_) | Tile::Hole));
    let mut via = None;
    if let Some(Tile::Teleport(id)) = board.tiles.get(&head).copied()
        && let Some(exit) = board.partner(head, id)
    {
        via = Some(head);
        head = exit;
        sfx.write(Sfx::Teleport);
    }
    // La queue va bouger : on peut avancer sur sa case actuelle.
    let body = &conga.cells[..conga.cells.len() - 1];
    if (outside && !settings.wrap) || deadly || body.contains(&head) {
        *crash = Crash { left: CRASH_TIME, dir: conga.dir, active: true };
        sfx.write(Sfx::GameOver);
        return;
    }

    conga.prev = conga.cells.clone();
    conga.cells.insert(0, head);
    conga.vias.insert(0, via);
    if voyage.run.exit == Some(head) {
        voyage.run.exiting = true;
        info!("voyage : la file passe la sortie ({} personnes)", conga.cells.len());
    }
    if food.active && head == food.cell {
        score.add(1);
        voyage.run.recruited += 1;
        sfx.write(Sfx::Eat(score.current));
        let tail = *conga.prev.last().unwrap();
        conga.prev.push(tail);
        let kind = food.kind;
        conga.kinds.push(kind);
        let look = food.look;
        conga.looks.push(look);
        food.cell = free_cell(&conga.cells, &board, &layout, &mut rng);
        food.kind = guest_kind(&mut rng, &progress);
        food.look = rng.below(LOOKS);
        food.spawned = false;
        if let Some(level) = voyage.mode.level() {
            // Voyage : objectif atteint → la sortie s'ouvre (ou c'est gagné tout de suite).
            if voyage.run.recruited >= level.tourists {
                food.active = false;
                if level.exit {
                    let spots = crate::levels::exit_candidates(&layout, head);
                    let spot = spots.into_iter().find(|c| !conga.cells.contains(c) && !board.tiles.contains_key(c));
                    if let Some(c) = spot {
                        board.tiles.insert(c, Tile::Exit);
                        voyage.run.exit = Some(c);
                        banner.text = game_common::tx!("La sortie est ouverte !", "The exit is open!", "Der Ausgang ist offen!", "L'uscita è aperta!", "¡La salida está abierta!").t().to_owned();
                        banner.left = 4.0;
                        sfx.write(Sfx::Spawn);
                    }
                } else {
                    let time = voyage.run.elapsed;
                    *voyage.outcome = Outcome { won: true, stars: stars_for(level, time), time, best_before: voyage.outcome.best_before };
                    voyage.next.set(GameState::GameOver);
                    sfx.write(Sfx::Win);
                }
            }
        } else {
            if score.current % LEVEL_EVERY == 0 {
                level_up(&mut board, &progress, &conga, food.cell, &settings, &layout, &mut banner, &mut sfx, &mut rng, score.current / LEVEL_EVERY);
            }
            let min = settings.speed.step_times().1;
            let faster = (timer.0.duration().as_secs_f32() * 0.96).max(min);
            timer.0.set_duration(Duration::from_secs_f32(faster));
        }
    } else {
        conga.cells.pop();
        conga.vias.pop();
    }
    if board.tiles.get(&head) == Some(&Tile::Speeder) {
        boost.0 = 5;
        sfx.write(Sfx::Boost);
    }
}

/// Défilé de sortie : à chaque case, le premier de la file passe la porte et les autres avancent.
#[allow(clippy::too_many_arguments)]
fn exit_sequence(
    mut commands: Commands,
    mut conga: ResMut<Conga>,
    mut voyage: Voyage,
    mut segments: Query<(Entity, &mut Segment)>,
    mut sfx: MessageWriter<Sfx>,
) {
    if !voyage.run.exiting || !voyage.run.stepped {
        return;
    }
    voyage.run.stepped = false;
    let old = conga.cells.clone();
    conga.cells.pop();
    conga.vias.pop();
    conga.vias.remove(0);
    conga.kinds.remove(0);
    conga.looks.remove(0);
    conga.prev = old[1..].to_vec();
    for (e, mut seg) in &mut segments {
        if seg.index == 0 {
            commands.entity(e).despawn();
        } else {
            seg.index -= 1;
        }
    }
    if conga.cells.is_empty() {
        if let Some(level) = voyage.mode.level() {
            let time = voyage.run.elapsed;
            *voyage.outcome = Outcome { won: true, stars: stars_for(level, time), time, best_before: voyage.outcome.best_before };
        }
        voyage.next.set(GameState::GameOver);
        sfx.write(Sfx::Win);
    } else {
        sfx.write(Sfx::Turn);
    }
}

/// Un nouvel élément arrive sur le plateau (le premier de la liste que le joueur a débloqué et qui a encore de la place).
#[allow(clippy::too_many_arguments)]
fn level_up(board: &mut Board, progress: &Progress, conga: &Conga, food: IVec2, settings: &Settings, layout: &Layout, banner: &mut Banner, sfx: &mut MessageWriter<Sfx>, rng: &mut Rng, level: u32) {
    let mut busy = conga.cells.clone();
    busy.push(food);
    for k in 0..SEQUENCE.len() {
        let f = SEQUENCE[(level as usize - 1 + k) % SEQUENCE.len()];
        if !progress.allows(f) || board.count(f) >= cap(f) {
            continue;
        }
        if board.place(f, &busy, conga.cells[0], layout, settings.universe, rng) {
            banner.text = format!("{} · {}", game_common::tx!("Niveau {}", "Level {}", "Level {}", "Livello {}", "Nivel {}").avec(&[&(level + 1)]), feature_name(f));
            banner.left = 3.0;
            sfx.write(Sfx::Spawn);
            return;
        }
    }
}

/// Laisse la tête finir sa glissade contre l'obstacle, puis termine la partie.
fn finish_crash(time: Res<Time>, mut crash: ResMut<Crash>, mut next: ResMut<NextState<GameState>>) {
    if !crash.active {
        return;
    }
    crash.left -= time.delta_secs();
    if crash.left <= 0.0 {
        crash.active = false;
        next.set(GameState::GameOver);
    }
}

fn free_cell(occupied: &[IVec2], board: &Board, layout: &Layout, rng: &mut Rng) -> IVec2 {
    loop {
        let c = IVec2::new(rng.below(layout.w as u32) as i32, rng.below(layout.h as u32) as i32);
        if layout.walkable(c) && !occupied.contains(&c) && !board.tiles.contains_key(&c) {
            return c;
        }
    }
}

/// Pose le nouvel invité (et son anneau) quand il a changé de place.
fn sync_guest(
    mut commands: Commands,
    mut food: ResMut<Food>,
    layout: Res<Layout>,
    server: Res<AssetServer>,
    assets: Res<GuestAssets>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
    old: Query<Entity, With<Guest>>,
) {
    if food.spawned {
        return;
    }
    food.spawned = true;
    for e in &old {
        commands.entity(e).despawn();
    }
    if !food.active {
        return;
    }
    let root = commands
        .spawn((
            Transform::from_translation(cell_to_world(food.cell.as_vec2(), 0.05, &layout))
                .with_rotation(Quat::from_rotation_y(PI * 0.75))
                .with_scale(Vec3::splat(0.01)),
            Visibility::default(),
            Guest,
            Pop(0.0),
        ))
        .id();
    commands.spawn((
        Mesh3d(assets.ring.clone()),
        MeshMaterial3d(assets.ring_mat.clone()),
        Transform::from_xyz(0.0, 0.03, 0.0),
        bevy::light::NotShadowCaster,
        Ring,
        ChildOf(root),
    ));
    people::spawn_model(&mut commands, &server, &mut graphs, root, food.kind, Pose::Wave, 1.0, food.look, false);
}

fn animate_guest(time: Res<Time>, mut rings: Query<&mut Transform, With<Ring>>) {
    let s = 1.0 + (time.elapsed_secs() * 5.0).sin() * 0.08;
    for mut tf in &mut rings {
        tf.scale = Vec3::new(s, 1.0, s);
    }
}

/// Apparition avec un petit rebond.
fn animate_pop(time: Res<Time>, mut commands: Commands, mut q: Query<(Entity, &mut Pop, &mut Transform)>) {
    for (e, mut pop, mut tf) in &mut q {
        pop.0 += time.delta_secs() / 0.35;
        let t = pop.0.min(1.0);
        // Dépasse un peu 1 puis revient (« back out »).
        let c = 1.70158;
        let s = 1.0 + (c + 1.0) * (t - 1.0).powi(3) + c * (t - 1.0).powi(2);
        tf.scale = Vec3::splat(s.max(0.01));
        if pop.0 >= 1.0 {
            commands.entity(e).remove::<Pop>();
            tf.scale = Vec3::ONE;
        }
    }
}

/// Crée/supprime les membres de la chenille, les fait glisser entre les cases et les tourne dans le sens de la marche.
#[allow(clippy::too_many_arguments)]
fn sync_segments(
    mut commands: Commands,
    time: Res<Time>,
    conga: Res<Conga>,
    timer: Res<StepTimer>,
    layout: Res<Layout>,
    state: Res<State<GameState>>,
    crash: Res<Crash>,
    server: Res<AssetServer>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
    mut segments: Query<(Entity, &mut Segment, &mut Transform, Has<Pop>)>,
) {
    let grid = &*layout;
    let t = if *state.get() == GameState::Playing && !crash.active { timer.0.fraction() } else { 1.0 };
    // Choc : la tête avance d'une demi-case, jusqu'au mur.
    let nudge = if crash.active { crash.dir.as_vec2() * 0.5 * (1.0 - crash.left / CRASH_TIME).clamp(0.0, 1.0) } else { Vec2::ZERO };
    let via = |i: usize| conga.vias.get(i).copied().flatten();
    // Téléporteur : le membre rentre dans la case d'entrée (il rapetisse), puis ressort de l'autre côté (il regrandit).
    let size_of = |i: usize| match via(i) {
        Some(_) if t < 1.0 => (if t < 0.5 { 1.0 - 2.0 * t } else { 2.0 * t - 1.0 }).max(0.02),
        _ => 1.0,
    };
    let pos = |i: usize| {
        let from = conga.prev.get(i).copied().unwrap_or(conga.cells[i]).as_vec2();
        let to = conga.cells[i].as_vec2();
        if let Some(v) = via(i)
            && t < 1.0
        {
            return if t < 0.5 { from.lerp(v.as_vec2(), t * 2.0) } else { to };
        }
        // Saut d'un bord à l'autre : pas de glissement à travers le plateau.
        let p = if from.distance_squared(to) > 1.5 { to } else { from.lerp(to, t) };
        if i == 0 { p + nudge } else { p }
    };
    // Direction de marche d'un membre : de la case précédente vers la case actuelle.
    let heading = |i: usize| {
        if let Some(v) = via(i) {
            let d = v - conga.prev.get(i).copied().unwrap_or(v);
            return if t < 0.5 && d != IVec2::ZERO && d.abs().max_element() <= 1 { Some(d) } else { None };
        }
        let d = conga.cells[i] - conga.prev.get(i).copied().unwrap_or(conga.cells[i]);
        if d == IVec2::ZERO || d.abs().max_element() > 1 { None } else { Some(d) }
    };
    let dt = time.delta_secs();

    let mut present = vec![false; conga.cells.len()];
    for (e, mut seg, mut tf, popping) in &mut segments {
        if seg.index >= conga.cells.len() {
            commands.entity(e).despawn();
            continue;
        }
        present[seg.index] = true;
        tf.translation = cell_to_world(pos(seg.index), 0.05, grid);
        if let Some(d) = heading(seg.index) {
            let target = (d.x as f32).atan2(d.y as f32);
            let delta = (target - seg.yaw + PI).rem_euclid(TAU) - PI;
            seg.yaw += delta * (1.0 - (-dt * 16.0).exp());
        }
        tf.rotation = Quat::from_rotation_y(seg.yaw);
        if !popping {
            tf.scale = Vec3::splat(size_of(seg.index));
        }
    }
    for (i, _) in present.iter().enumerate().filter(|(_, p)| !**p) {
        let yaw = heading(i).map_or(PI / 2.0, |d| (d.x as f32).atan2(d.y as f32));
        let size = if i == 0 { 1.12 } else { 1.0 };
        let mut root = commands.spawn((
            Transform::from_translation(cell_to_world(pos(i), 0.05, grid)).with_rotation(Quat::from_rotation_y(yaw)),
            Visibility::default(),
            Segment { index: i, yaw },
        ));
        // Les trois membres de départ sont déjà là ; les suivants apparaissent en rebondissant.
        if i >= 3 {
            root.insert((Pop(0.0), Transform::from_translation(cell_to_world(pos(i), 0.05, grid)).with_scale(Vec3::splat(0.01))));
        }
        let id = root.id();
        people::spawn_model(&mut commands, &server, &mut graphs, id, conga.kinds[i], Pose::Walk, size, conga.looks[i], i == 0);
    }
}
