//! La partie : chargement des niveaux, déplacements, poussées, annuler,
//! animations, victoire, progression sauvegardée.
use std::f32::consts::{FRAC_PI_2, PI};

use bevy::prelude::*;
use game_common::iso_projection;
use game_common::models::{FrogKit, spawn_frog};

use crate::GameState;
use crate::board::{self, BoardPart, Crate, CrateLook, CratePiece, Universe};
use crate::level::{Level, Snapshot, all_levels};

const STEP_TIME: f32 = 0.13;
const CAM_OFFSET: Vec3 = Vec3::new(1.5, 12.0, 7.5);
const SAVE_GAME: &str = "pousse-caisses";

pub struct PlayPlugin;

impl Plugin for PlayPlugin {
    fn build(&self, app: &mut App) {
        let levels = all_levels();
        let unlocked = game_common::save::load(SAVE_GAME, "debloques")
            .and_then(|s| s.parse().ok())
            .unwrap_or(1usize)
            .clamp(1, levels.len());
        let first = levels[0].clone();
        app.init_resource::<FrogKit>()
            .init_resource::<CrateLook>()
            .add_message::<PlayEvent>()
            .insert_resource(Progress { levels, unlocked })
            .insert_resource(Play::new(0, first))
            .add_systems(Startup, setup)
            .add_systems(OnEnter(GameState::Playing), restart_if_needed)
            .add_systems(
                Update,
                (
                    rebuild_board,
                    (read_input, advance).chain().run_if(in_state(GameState::Playing)),
                    animate,
                    follow_camera,
                )
                    .chain(),
            );
    }
}

/// Ce qui se passe pendant la partie (pour les sons et l'interface).
#[derive(Message, Clone, Copy)]
pub enum PlayEvent {
    Step,
    Push,
    /// Une caisse vient d'arriver sur une cible.
    Stored,
    Undo,
    Won,
}

/// Les niveaux et la progression du joueur.
#[derive(Resource)]
pub struct Progress {
    pub levels: Vec<Level>,
    /// Nombre de niveaux accessibles (le suivant du dernier réussi).
    pub unlocked: usize,
}

/// La partie en cours.
#[derive(Resource)]
pub struct Play {
    pub index: usize,
    pub level: Level,
    pub state: Snapshot,
    /// État avant le dernier déplacement (pour l'animation).
    from: Snapshot,
    history: Vec<Snapshot>,
    pub moves: u32,
    /// 0 → 1 pendant l'animation du dernier déplacement.
    anim: f32,
    queued: Option<IVec2>,
    yaw: f32,
    pub won: bool,
    /// Le plateau affiché correspond-il au niveau en cours ?
    built: bool,
    /// Pour le mode captures : directions à jouer automatiquement.
    pub script: Vec<IVec2>,
}

impl Play {
    fn new(index: usize, level: Level) -> Self {
        let state = level.start.clone();
        Play {
            index,
            from: state.clone(),
            state,
            level,
            history: Vec::new(),
            moves: 0,
            anim: 1.0,
            queued: None,
            yaw: 0.0,
            won: false,
            built: false,
            script: Vec::new(),
        }
    }

    pub fn universe(&self) -> Universe {
        Universe::of_level(self.index)
    }
}

/// Demande de (re)chargement d'un niveau, depuis les menus.
pub fn load_level(play: &mut Play, progress: &Progress, index: usize) {
    let index = index.min(progress.levels.len() - 1);
    let script = std::mem::take(&mut play.script);
    *play = Play::new(index, progress.levels[index].clone());
    play.script = script;
}

#[derive(Component)]
struct FrogModel;

fn setup(mut commands: Commands, frog_kit: Res<FrogKit>) {
    commands.spawn((Camera3d::default(), iso_projection(10.0, 10.0), Transform::from_translation(CAM_OFFSET)));
    let frog = spawn_frog(&mut commands, &frog_kit);
    commands.entity(frog).insert(FrogModel);
}

/// En revenant d'une victoire, « Jouer » enchaîne sur le niveau suivant.
fn restart_if_needed(mut play: ResMut<Play>, progress: Res<Progress>) {
    if play.won {
        let next = (play.index + 1).min(progress.levels.len() - 1);
        load_level(&mut play, &progress, next);
    }
}

#[allow(clippy::too_many_arguments)]
fn rebuild_board(
    mut commands: Commands,
    mut play: ResMut<Play>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<StandardMaterial>>,
    look: Res<CrateLook>,
    old: Query<Entity, With<BoardPart>>,
    mut clear: ResMut<ClearColor>,
    mut ambient: ResMut<GlobalAmbientLight>,
    mut suns: Query<&mut DirectionalLight>,
    mut cam: Query<&mut Projection, With<Camera3d>>,
) {
    if play.built {
        return;
    }
    play.built = true;
    for e in &old {
        commands.entity(e).despawn();
    }
    let universe = play.universe();
    board::build(&mut commands, &mut meshes, &mut mats, &look, &play.level, universe, &mut clear, &mut ambient, &mut suns);
    let (w, h) = (play.level.width as f32, play.level.height as f32);
    for mut proj in &mut cam {
        *proj = iso_projection(w + 2.5, h + 3.5);
    }
}

/// Flèches / ZQSD, glisser du doigt, U ou Retour arrière pour annuler, R pour recommencer.
fn read_input(
    keys: Res<ButtonInput<KeyCode>>,
    touches: Res<Touches>,
    mut play: ResMut<Play>,
    mut events: MessageWriter<PlayEvent>,
) {
    if play.won {
        return;
    }
    if keys.just_pressed(KeyCode::KeyU) || keys.just_pressed(KeyCode::Backspace) {
        undo(&mut play, &mut events);
        return;
    }
    if keys.just_pressed(KeyCode::KeyR) {
        restart(&mut play);
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
            play.queued = Some(d);
        }
    }
    for touch in touches.iter_just_released() {
        let delta = touch.position() - touch.start_position();
        if delta.length() > 30.0 {
            play.queued = Some(if delta.x.abs() > delta.y.abs() {
                IVec2::new(delta.x.signum() as i32, 0)
            } else {
                IVec2::new(0, delta.y.signum() as i32)
            });
        }
    }
    if play.queued.is_none() && play.anim >= 1.0 && !play.script.is_empty() {
        let d = play.script.remove(0);
        play.queued = Some(d);
    }
}

pub fn undo(play: &mut Play, events: &mut MessageWriter<PlayEvent>) {
    if let Some(previous) = play.history.pop() {
        play.state = previous.clone();
        play.from = previous;
        play.anim = 1.0;
        play.moves = play.moves.saturating_sub(1);
        play.queued = None;
        events.write(PlayEvent::Undo);
    }
}

pub fn restart(play: &mut Play) {
    let level = play.level.clone();
    let index = play.index;
    *play = Play::new(index, level);
    // Le plateau est déjà construit ; seules les positions changent.
    play.built = true;
}

/// Applique le déplacement demandé quand l'animation précédente est finie.
fn advance(
    mut play: ResMut<Play>,
    mut progress: ResMut<Progress>,
    mut events: MessageWriter<PlayEvent>,
    mut next: ResMut<NextState<GameState>>,
) {
    if play.anim < 1.0 || play.won {
        return;
    }
    let Some(dir) = play.queued.take() else {
        return;
    };
    play.yaw = match (dir.x, dir.y) {
        (0, -1) => 0.0,
        (0, _) => PI,
        (1, _) => -FRAC_PI_2,
        _ => FRAC_PI_2,
    };
    let Some(mv) = play.level.step(&play.state, dir) else {
        return; // mur, ou caisse bloquée
    };
    let before = play.state.clone();
    play.history.push(before.clone());
    play.from = before;
    play.state = mv.state;
    play.anim = 0.0;
    play.moves += 1;
    match mv.pushed {
        Some(i) => {
            events.write(PlayEvent::Push);
            if play.level.targets.contains(&play.state.boxes[i]) {
                events.write(PlayEvent::Stored);
            }
        }
        None => {
            events.write(PlayEvent::Step);
        }
    }
    if play.level.is_solved(&play.state) {
        play.won = true;
        progress.unlocked = progress.unlocked.max(play.index + 2).min(progress.levels.len());
        game_common::save::store(SAVE_GAME, "debloques", &progress.unlocked.to_string());
        events.write(PlayEvent::Won);
        next.set(GameState::Won);
    }
}

/// Saut du crapaud et glissement des caisses ; couleur des caisses rangées.
fn animate(
    time: Res<Time>,
    mut play: ResMut<Play>,
    look: Res<CrateLook>,
    mut frog: Query<&mut Transform, (With<FrogModel>, Without<Crate>)>,
    mut crates: Query<(&Crate, &mut Transform, &Children), Without<FrogModel>>,
    mut pieces: Query<(&CratePiece, &mut MeshMaterial3d<StandardMaterial>)>,
) {
    play.anim = (play.anim + time.delta_secs() / STEP_TIME).min(1.0);
    let t = play.anim;
    let level = &play.level;
    let lerp = |a: IVec2, b: IVec2| board::to_world(level, a.as_vec2().lerp(b.as_vec2(), t));

    for mut tf in &mut frog {
        let s = (t * PI).sin();
        tf.translation = lerp(play.from.player, play.state.player) + Vec3::Y * s * 0.3;
        tf.rotation = Quat::from_rotation_y(play.yaw);
        tf.scale = Vec3::new(1.0 - 0.1 * s, 1.0 + 0.25 * s, 1.0 - 0.1 * s) * 0.95;
    }
    for (c, mut tf, children) in &mut crates {
        let (Some(&from), Some(&to)) = (play.from.boxes.get(c.0), play.state.boxes.get(c.0)) else {
            continue;
        };
        tf.translation = lerp(from, to);
        // La caisse change de couleur une fois posée sur sa cible.
        let stored = t >= 1.0 && level.targets.contains(&to);
        for child in children.iter() {
            if let Ok((piece, mut mat)) = pieces.get_mut(child) {
                let want = match (piece.frame, stored) {
                    (false, false) => &look.wood,
                    (true, false) => &look.frame,
                    (false, true) => &look.wood_ok,
                    (true, true) => &look.frame_ok,
                };
                if mat.0 != *want {
                    mat.0 = want.clone();
                }
            }
        }
    }
}

/// Dans le menu, la caméra tourne doucement autour du plateau.
fn follow_camera(
    time: Res<Time>,
    state: Res<State<GameState>>,
    mut angle: Local<f32>,
    mut q: Query<&mut Transform, With<Camera3d>>,
) {
    let dt = time.delta_secs();
    if matches!(state.get(), GameState::Menu | GameState::Levels) {
        *angle += dt * 0.15;
    } else {
        let a = (*angle + PI).rem_euclid(std::f32::consts::TAU) - PI;
        *angle = a * (1.0 - (dt * 3.0).min(1.0));
    }
    for mut tf in &mut q {
        *tf = Transform::from_translation(Quat::from_rotation_y(*angle) * CAM_OFFSET).looking_at(Vec3::ZERO, Vec3::Y);
    }
}
