//! Règles du Snake : déplacement, nourriture, collisions.
use std::collections::VecDeque;
use std::time::Duration;

use bevy::prelude::*;
use game_common::{Rng, Score, flat_material, palette};

use crate::GameState;
use crate::audio::Sfx;
use crate::settings::Settings;
use crate::world::cell_to_world;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Snake::new(14))
            .insert_resource(Food(IVec2::ZERO))
            .insert_resource(StepTimer(Timer::from_seconds(0.16, TimerMode::Repeating)))
            .add_systems(Startup, setup)
            .add_systems(OnEnter(GameState::Playing), new_game)
            .add_systems(OnEnter(GameState::Menu), clear_board)
            .add_systems(
                Update,
                (
                    (read_input, step).chain().run_if(in_state(GameState::Playing)),
                    sync_segments.run_if(|s: Res<State<GameState>>| {
                        matches!(s.get(), GameState::Playing | GameState::GameOver)
                    }),
                    animate_food,
                )
                    .chain(),
            );
    }
}

/// État logique du serpent (la tête est en premier).
#[derive(Resource)]
struct Snake {
    cells: Vec<IVec2>,
    /// Positions au pas précédent, pour animer le glissement entre deux cases.
    prev: Vec<IVec2>,
    dir: IVec2,
    queued: VecDeque<IVec2>,
}

impl Snake {
    fn new(grid: i32) -> Self {
        let c = grid / 2;
        let cells = vec![IVec2::new(c, c), IVec2::new(c - 1, c), IVec2::new(c - 2, c)];
        Self { prev: cells.clone(), cells, dir: IVec2::X, queued: VecDeque::new() }
    }

    /// Mémorise un virage ; renvoie `true` s'il est accepté.
    fn turn(&mut self, d: IVec2) -> bool {
        let last = self.queued.back().copied().unwrap_or(self.dir);
        let ok = d != last && d != -last && self.queued.len() < 2;
        if ok {
            self.queued.push_back(d);
        }
        ok
    }
}

#[derive(Resource)]
struct Food(IVec2);

#[derive(Resource)]
struct StepTimer(Timer);

#[derive(Resource)]
struct SnakeAssets {
    cube: Handle<Mesh>,
    head: Handle<StandardMaterial>,
    body: Vec<Handle<StandardMaterial>>,
}

#[derive(Component)]
struct Segment(usize);

#[derive(Component)]
struct FoodMarker;

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    let cube = meshes.add(Cuboid::new(1.0, 1.0, 1.0));
    commands.spawn((
        Mesh3d(cube.clone()),
        MeshMaterial3d(materials.add(flat_material(palette::PINK))),
        Transform::from_scale(Vec3::splat(0.55)),
        Visibility::Hidden,
        FoodMarker,
    ));
    let body = [palette::CYAN, palette::LILAC, palette::NAVY, palette::WHITE]
        .into_iter()
        .map(|c| materials.add(flat_material(c)))
        .collect();
    let head = materials.add(flat_material(palette::YELLOW));
    commands.insert_resource(SnakeAssets { cube, head, body });
}

#[allow(clippy::too_many_arguments)]
fn new_game(
    mut commands: Commands,
    settings: Res<Settings>,
    mut snake: ResMut<Snake>,
    mut food: ResMut<Food>,
    mut timer: ResMut<StepTimer>,
    mut score: ResMut<Score>,
    mut rng: ResMut<Rng>,
    segments: Query<Entity, With<Segment>>,
    mut food_vis: Query<&mut Visibility, With<FoodMarker>>,
    mut sfx: MessageWriter<Sfx>,
) {
    sfx.write(Sfx::Start);
    for e in &segments {
        commands.entity(e).despawn();
    }
    *snake = Snake::new(settings.grid());
    food.0 = free_cell(&snake.cells, settings.grid(), &mut rng);
    timer.0 = Timer::from_seconds(settings.speed.step_times().0, TimerMode::Repeating);
    score.reset();
    for mut v in &mut food_vis {
        *v = Visibility::Visible;
    }
}

fn clear_board(
    mut commands: Commands,
    segments: Query<Entity, With<Segment>>,
    mut food_vis: Query<&mut Visibility, With<FoodMarker>>,
) {
    for e in &segments {
        commands.entity(e).despawn();
    }
    for mut v in &mut food_vis {
        *v = Visibility::Hidden;
    }
}

fn read_input(keys: Res<ButtonInput<KeyCode>>, mut snake: ResMut<Snake>, mut sfx: MessageWriter<Sfx>) {
    // Touches physiques : W/A/S/D correspondent à Z/Q/S/D sur un clavier AZERTY.
    let bindings = [
        (KeyCode::ArrowUp, KeyCode::KeyW, IVec2::NEG_Y),
        (KeyCode::ArrowDown, KeyCode::KeyS, IVec2::Y),
        (KeyCode::ArrowLeft, KeyCode::KeyA, IVec2::NEG_X),
        (KeyCode::ArrowRight, KeyCode::KeyD, IVec2::X),
    ];
    for (a, b, d) in bindings {
        if (keys.just_pressed(a) || keys.just_pressed(b)) && snake.turn(d) {
            sfx.write(Sfx::Turn);
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn step(
    time: Res<Time>,
    settings: Res<Settings>,
    mut timer: ResMut<StepTimer>,
    mut snake: ResMut<Snake>,
    mut food: ResMut<Food>,
    mut score: ResMut<Score>,
    mut rng: ResMut<Rng>,
    mut next: ResMut<NextState<GameState>>,
    mut sfx: MessageWriter<Sfx>,
) {
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }
    let grid = settings.grid();
    if let Some(d) = snake.queued.pop_front() {
        snake.dir = d;
    }
    let mut head = snake.cells[0] + snake.dir;
    let outside = head.x < 0 || head.y < 0 || head.x >= grid || head.y >= grid;
    if outside && settings.wrap {
        head = head.rem_euclid(IVec2::splat(grid));
    }
    // La queue va bouger : on peut avancer sur sa case actuelle.
    let body = &snake.cells[..snake.cells.len() - 1];
    if (outside && !settings.wrap) || body.contains(&head) {
        next.set(GameState::GameOver);
        sfx.write(Sfx::GameOver);
        return;
    }

    snake.prev = snake.cells.clone();
    snake.cells.insert(0, head);
    if head == food.0 {
        score.add(1);
        sfx.write(Sfx::Eat(score.current));
        let tail = *snake.prev.last().unwrap();
        snake.prev.push(tail);
        food.0 = free_cell(&snake.cells, grid, &mut rng);
        let min = settings.speed.step_times().1;
        let faster = (timer.0.duration().as_secs_f32() * 0.96).max(min);
        timer.0.set_duration(Duration::from_secs_f32(faster));
    } else {
        snake.cells.pop();
    }
}

fn free_cell(occupied: &[IVec2], grid: i32, rng: &mut Rng) -> IVec2 {
    loop {
        let c = IVec2::new(rng.below(grid as u32) as i32, rng.below(grid as u32) as i32);
        if !occupied.contains(&c) {
            return c;
        }
    }
}

/// Crée/supprime les cubes du serpent et les fait glisser entre les cases.
fn sync_segments(
    mut commands: Commands,
    snake: Res<Snake>,
    timer: Res<StepTimer>,
    settings: Res<Settings>,
    state: Res<State<GameState>>,
    assets: Res<SnakeAssets>,
    mut segments: Query<(Entity, &Segment, &mut Transform)>,
) {
    let grid = settings.grid();
    let t = if *state.get() == GameState::Playing { timer.0.fraction() } else { 1.0 };
    let pos = |i: usize| {
        let from = snake.prev.get(i).copied().unwrap_or(snake.cells[i]).as_vec2();
        let to = snake.cells[i].as_vec2();
        // Saut d'un bord à l'autre : pas de glissement à travers le plateau.
        if from.distance_squared(to) > 1.5 { to } else { from.lerp(to, t) }
    };
    let height = |i: usize| if i == 0 { 0.45 } else { 0.39 };

    let mut present = vec![false; snake.cells.len()];
    for (e, seg, mut tf) in &mut segments {
        if seg.0 >= snake.cells.len() {
            commands.entity(e).despawn();
            continue;
        }
        present[seg.0] = true;
        tf.translation = cell_to_world(pos(seg.0), height(seg.0), grid);
    }
    for (i, _) in present.iter().enumerate().filter(|(_, p)| !**p) {
        let (mat, size) = if i == 0 {
            (assets.head.clone(), 0.9)
        } else {
            (assets.body[(i - 1) % assets.body.len()].clone(), 0.78)
        };
        commands.spawn((
            Mesh3d(assets.cube.clone()),
            MeshMaterial3d(mat),
            Transform::from_translation(cell_to_world(pos(i), height(i), grid)).with_scale(Vec3::splat(size)),
            Segment(i),
        ));
    }
}

fn animate_food(
    time: Res<Time>,
    food: Res<Food>,
    settings: Res<Settings>,
    mut q: Query<&mut Transform, With<FoodMarker>>,
) {
    let s = time.elapsed_secs();
    for mut tf in &mut q {
        tf.translation = cell_to_world(food.0.as_vec2(), 0.55 + (s * 3.0).sin() * 0.12, settings.grid());
        tf.rotation = Quat::from_rotation_y(s * 1.5) * Quat::from_rotation_x(0.6);
    }
}
