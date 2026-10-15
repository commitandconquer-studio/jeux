//! La partie : coups, animations des tuiles (glisser, fusionner, apparaître),
//! chiffres affichés sur les tuiles, faits scientifiques, fin de partie.
use bevy::prelude::*;
use game_common::{Rng, flat_material};

use crate::GameState;
use crate::desk::cell_pos;
use crate::grid::Grid;

const SLIDE_TIME: f32 = 0.11;
const SAVE: &str = "eureka-2048";

pub struct PlayPlugin;

impl Plugin for PlayPlugin {
    fn build(&self, app: &mut App) {
        let best = game_common::save::load(SAVE, "record").and_then(|s| s.parse().ok()).unwrap_or(0);
        app.add_message::<PlayEvent>()
            .init_resource::<TileLook>()
            .insert_resource(Game { best, ..Game::default() })
            .add_systems(Startup, |mut game: ResMut<Game>, mut rng: ResMut<Rng>| game.new_game(&mut rng))
            .add_systems(
                Update,
                (
                    (read_input, autopilot, play_move).chain().run_if(in_state(GameState::Playing)),
                    animate,
                    sync_tiles,
                    place_labels,
                )
                    .chain(),
            );
    }
}

#[derive(Message, Clone, Copy)]
pub enum PlayEvent {
    Slide,
    Merge(u32),
    /// Nouvelle plus grande tuile : un petit fait scientifique.
    Fact(u32),
    Undo,
}

#[derive(Resource, Default)]
pub struct Game {
    pub grid: Grid,
    pub score: u32,
    pub best: u32,
    history: Option<(Grid, u32)>,
    /// 0 → 1 pendant le glissement ; ≥ 1 au repos.
    anim: f32,
    queued: Option<IVec2>,
    /// Tuiles absorbées par une fusion (elles disparaissent après le glissement).
    doomed: Vec<u32>,
    /// Le plateau affiché doit être reconstruit (nouvelle partie, annulation).
    rebuild: bool,
    /// Plus grande tuile déjà annoncée.
    max_seen: u32,
    /// 2048 atteint et déjà fêté.
    pub won: bool,
    pub autopilot: bool,
    auto_timer: f32,
}

impl Game {
    pub fn new_game(&mut self, rng: &mut Rng) {
        let best = self.best;
        let autopilot = self.autopilot;
        *self = Game { best, autopilot, anim: 1.0, rebuild: true, max_seen: 4, ..Game::default() };
        self.grid.spawn_random(rng);
        self.grid.spawn_random(rng);
    }

    pub fn undo(&mut self) -> bool {
        match self.history.take() {
            Some((grid, score)) => {
                self.grid = grid;
                self.score = score;
                self.rebuild = true;
                self.anim = 1.0;
                true
            }
            None => false,
        }
    }

    pub fn can_undo(&self) -> bool {
        self.history.is_some()
    }
}

/// Couleurs « feutres » des tuiles, de 2 à 2048 et au-delà.
fn tile_color(value: u32) -> Color {
    let c = Color::srgb;
    match value {
        2 => c(0.98, 0.96, 0.86),
        4 => c(0.99, 0.90, 0.68),
        8 => c(1.0, 0.75, 0.45),
        16 => c(1.0, 0.60, 0.40),
        32 => c(0.98, 0.45, 0.42),
        64 => c(0.92, 0.32, 0.40),
        128 => c(0.96, 0.80, 0.28),
        256 => c(0.55, 0.80, 0.40),
        512 => c(0.35, 0.72, 0.88),
        1024 => c(0.45, 0.52, 0.92),
        2048 => c(0.76, 0.45, 0.92),
        _ => c(0.22, 0.24, 0.38),
    }
}

fn text_color(value: u32) -> Color {
    if value <= 4 { Color::srgb(0.30, 0.28, 0.35) } else { Color::WHITE }
}

/// Hauteur d'une tuile : les grands nombres sont plus épais.
fn tile_height(value: u32) -> f32 {
    0.22 + value.max(2).ilog2() as f32 * 0.03
}

#[derive(Resource)]
struct TileLook {
    cube: Handle<Mesh>,
    /// Matériau par puissance de 2 (indice = log2).
    mats: Vec<Handle<StandardMaterial>>,
}

impl FromWorld for TileLook {
    fn from_world(world: &mut World) -> Self {
        let cube = world.resource_mut::<Assets<Mesh>>().add(Cuboid::new(1.0, 1.0, 1.0));
        let mut assets = world.resource_mut::<Assets<StandardMaterial>>();
        let mats = (0..=17)
            .map(|k| {
                let v = 1u32 << k;
                let glow = if v >= 2048 { 0.5 } else { 0.0 };
                assets.add(StandardMaterial { emissive: tile_color(v).to_linear() * glow, ..flat_material(tile_color(v)) })
            })
            .collect();
        TileLook { cube, mats }
    }
}

impl TileLook {
    fn mat(&self, value: u32) -> Handle<StandardMaterial> {
        self.mats[(value.max(1).ilog2() as usize).min(self.mats.len() - 1)].clone()
    }
}

/// Une tuile affichée.
#[derive(Component)]
struct TileView {
    id: u32,
    value: u32,
    from: Vec3,
    to: Vec3,
    /// Rebond de fusion / grossissement à l'apparition (0 → 1).
    pop: f32,
    appear: f32,
}

/// Chiffre affiché au-dessus d'une tuile.
#[derive(Component)]
struct TileLabel(Entity);

fn read_input(keys: Res<ButtonInput<KeyCode>>, touches: Res<Touches>, mut game: ResMut<Game>, mut events: MessageWriter<PlayEvent>) {
    if game.autopilot {
        return;
    }
    if (keys.just_pressed(KeyCode::KeyU) || keys.just_pressed(KeyCode::Backspace)) && game.undo() {
        events.write(PlayEvent::Undo);
        return;
    }
    let bindings = [
        (KeyCode::ArrowUp, KeyCode::KeyW, IVec2::NEG_Y),
        (KeyCode::ArrowDown, KeyCode::KeyS, IVec2::Y),
        (KeyCode::ArrowLeft, KeyCode::KeyA, IVec2::NEG_X),
        (KeyCode::ArrowRight, KeyCode::KeyD, IVec2::X),
    ];
    for (a, b, d) in bindings {
        if keys.just_pressed(a) || keys.just_pressed(b) {
            game.queued = Some(d);
        }
    }
    for t in touches.iter_just_released() {
        let d = t.position() - t.start_position();
        if d.length() > 30.0 {
            game.queued = Some(if d.x.abs() > d.y.abs() { IVec2::new(d.x.signum() as i32, 0) } else { IVec2::new(0, d.y.signum() as i32) });
        }
    }
}

/// Pilote automatique (mode captures) : garde les gros nombres dans un coin.
fn autopilot(time: Res<Time>, mut game: ResMut<Game>) {
    if !game.autopilot || game.queued.is_some() {
        return;
    }
    game.auto_timer += time.delta_secs();
    if game.auto_timer < 0.16 {
        return;
    }
    game.auto_timer = 0.0;
    for d in [IVec2::Y, IVec2::NEG_X, IVec2::X, IVec2::NEG_Y] {
        if game.grid.clone().slide(d).changed {
            game.queued = Some(d);
            return;
        }
    }
}

fn play_move(mut game: ResMut<Game>, mut events: MessageWriter<PlayEvent>, mut tiles: Query<&mut TileView>) {
    if game.anim < 1.0 {
        return;
    }
    let Some(dir) = game.queued.take() else { return };
    let before = (game.grid.clone(), game.score);
    let result = game.grid.slide(dir);
    if !result.changed {
        return;
    }
    game.history = Some(before);
    game.score += result.points;
    game.best = game.best.max(game.score);
    game.anim = 0.0;
    events.write(PlayEvent::Slide);
    // Chaque tuile glisse vers sa nouvelle case ; les absorbées vers leur partenaire.
    let absorbed: Vec<(u32, u32)> = result.merges.iter().map(|m| (m.absorbed, m.kept)).collect();
    for mut tv in &mut tiles {
        let target = absorbed.iter().find(|(a, _)| *a == tv.id).map(|(_, k)| *k).unwrap_or(tv.id);
        if let Some(p) = game.grid.position_of(target) {
            tv.from = tv.to;
            tv.to = cell_pos(p);
        }
    }
    game.doomed = absorbed.iter().map(|(a, _)| *a).collect();
    for m in &result.merges {
        events.write(PlayEvent::Merge(m.value));
    }
}

/// Fin du glissement : fusions, nouvelle tuile, faits, fin de partie.
#[allow(clippy::too_many_arguments)]
fn animate(
    mut commands: Commands,
    time: Res<Time>,
    mut game: ResMut<Game>,
    mut rng: ResMut<Rng>,
    look: Res<TileLook>,
    state: Res<State<GameState>>,
    mut next: ResMut<NextState<GameState>>,
    mut events: MessageWriter<PlayEvent>,
    mut tiles: Query<(Entity, &mut TileView, &mut Transform, &mut MeshMaterial3d<StandardMaterial>)>,
) {
    let dt = time.delta_secs();
    let was_moving = game.anim < 1.0;
    game.anim = (game.anim + dt / SLIDE_TIME).min(1.0);
    let t = game.anim;
    let ease = 1.0 - (1.0 - t) * (1.0 - t);

    if was_moving && t >= 1.0 {
        // Les tuiles absorbées disparaissent ; les autres prennent leur valeur.
        for (e, mut tv, _, mut mat) in &mut tiles {
            if game.doomed.contains(&tv.id) {
                commands.entity(e).despawn();
                continue;
            }
            if let Some(p) = game.grid.position_of(tv.id) {
                let value = game.grid.get(p).map(|t| t.value).unwrap_or(tv.value);
                if value != tv.value {
                    tv.value = value;
                    tv.pop = 0.0;
                    mat.0 = look.mat(value);
                }
            }
        }
        game.doomed.clear();
        if let Some((p, tile)) = game.grid.spawn_random(&mut rng) {
            spawn_tile(&mut commands, &look, p, tile.id, tile.value, true);
        }
        let max = game.grid.max_value();
        if max > game.max_seen {
            game.max_seen = max;
            events.write(PlayEvent::Fact(max));
        }
        if game.best > 0 {
            game_common::save::store(SAVE, "record", &game.best.to_string());
        }
        if *state.get() == GameState::Playing {
            if max >= 2048 && !game.won {
                game.won = true;
                next.set(GameState::Won);
            } else if !game.grid.can_move() {
                next.set(GameState::Over);
            }
        }
    }

    for (_, mut tv, mut tf, _) in &mut tiles {
        tv.pop = (tv.pop + dt * 5.0).min(1.0);
        tv.appear = (tv.appear + dt * 6.0).min(1.0);
        let h = tile_height(tv.value);
        let bounce = 1.0 + (tv.pop * std::f32::consts::PI).sin() * 0.22;
        let grow = tv.appear * tv.appear * (3.0 - 2.0 * tv.appear);
        tf.translation = tv.from.lerp(tv.to, ease) + Vec3::Y * h / 2.0;
        tf.scale = Vec3::new(0.98 * bounce, h * bounce, 0.98 * bounce) * grow;
    }
}

fn spawn_tile(commands: &mut Commands, look: &TileLook, p: IVec2, id: u32, value: u32, appear: bool) {
    let pos = cell_pos(p);
    commands.spawn((
        Mesh3d(look.cube.clone()),
        MeshMaterial3d(look.mat(value)),
        Transform::from_translation(pos).with_scale(Vec3::ZERO),
        TileView { id, value, from: pos, to: pos, pop: 1.0, appear: if appear { 0.0 } else { 1.0 } },
    ));
}

/// Nouvelle partie ou annulation : on redessine tout le plateau.
fn sync_tiles(mut commands: Commands, mut game: ResMut<Game>, look: Res<TileLook>, tiles: Query<Entity, With<TileView>>) {
    if !game.rebuild {
        return;
    }
    game.rebuild = false;
    game.doomed.clear();
    for e in &tiles {
        commands.entity(e).despawn();
    }
    for (p, t) in game.grid.tiles() {
        spawn_tile(&mut commands, &look, p, t.id, t.value, true);
    }
}

/// Écrit le chiffre de chaque tuile, à l'écran, juste au-dessus d'elle.
fn place_labels(
    mut commands: Commands,
    cam: Query<(&Camera, &GlobalTransform)>,
    tiles: Query<(Entity, &TileView, &Transform)>,
    mut labels: Query<(Entity, &TileLabel, &mut Node, &mut Text, &mut TextFont, &mut TextColor, &ComputedNode)>,
) {
    let Ok((camera, cam_tf)) = cam.single() else { return };
    let mut labelled = Vec::new();
    for (e, label, mut node, mut text, mut font, mut color, computed) in &mut labels {
        let Ok((_, tv, tf)) = tiles.get(label.0) else {
            commands.entity(e).despawn();
            continue;
        };
        labelled.push(label.0);
        let top = tf.translation + Vec3::Y * tf.scale.y / 2.0;
        // Largeur d'une case à l'écran (fixe : ne dépend pas des animations).
        let (Ok(center), Ok(edge)) = (camera.world_to_viewport(cam_tf, top), camera.world_to_viewport(cam_tf, top + Vec3::X * 0.5)) else {
            continue;
        };
        let tile_px = (edge.x - center.x).abs() * 2.0;
        let digits = tv.value.to_string();
        // Taille arrondie : chaque taille différente crée de nouveaux caractères
        // en mémoire graphique, il ne faut pas en changer à chaque image.
        let size = (tile_px * match digits.len() { 1 | 2 => 0.5, 3 => 0.4, 4 => 0.31, _ => 0.25 } / 4.0).round() * 4.0;
        if text.0 != digits {
            text.0 = digits;
        }
        let size = bevy::text::FontSize::Px(size.max(8.0));
        if font.font_size != size {
            font.font_size = size;
        }
        color.0 = text_color(tv.value);
        let s = computed.size() * computed.inverse_scale_factor();
        node.left = Val::Px(center.x - s.x / 2.0);
        node.top = Val::Px(center.y - s.y / 2.0);
    }
    for (e, _, _) in &tiles {
        if !labelled.contains(&e) {
            commands.spawn((
                Text::new(""),
                TextFont::from_font_size(10.0),
                TextColor(Color::WHITE),
                Node { position_type: PositionType::Absolute, ..default() },
                Pickable::IGNORE,
                GlobalZIndex(-1),
                TileLabel(e),
            ));
        }
    }
}
