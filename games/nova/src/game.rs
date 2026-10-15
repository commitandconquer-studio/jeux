//! La partie : commandes, gravité, pose des blocs, canon à ions, riposte.
use std::collections::VecDeque;

use bevy::prelude::*;
use game_common::Rng;

use crate::GameState;
use crate::board::{Bag, Board, HEIGHT, Kind, Piece, WIDTH};
use crate::story::CHAPTERS;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<NovaEvent>()
            .insert_resource(Game::new(Mode::Endless, &mut Rng::new(1)))
            .init_resource::<TouchState>()
            .add_systems(OnEnter(GameState::Playing), start_game)
            .add_systems(OnEnter(GameState::Menu), start_demo)
            .add_systems(
                Update,
                (controls, autopilot, gravity, enemy_fire, destroyed)
                    .chain()
                    .run_if(|s: Res<State<GameState>>| matches!(s.get(), GameState::Playing | GameState::Menu)),
            );
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Mode {
    /// Chapitre de l'histoire (indice dans `CHAPTERS`).
    Campaign(usize),
    /// Score infini, sans croiseur.
    Endless,
}

/// Ce qui vient de se passer (sons, effets, interface).
#[derive(Message, Clone, Debug)]
pub enum NovaEvent {
    Rotate,
    Lock,
    HardDrop,
    Hold,
    /// Lignes complètes (numéros des rangées, avant suppression).
    Lines(Vec<i32>),
    Warning,
    EnemyShot,
    CruiserDestroyed,
    Defeat,
}

#[derive(Resource)]
pub struct Game {
    pub mode: Mode,
    pub board: Board,
    pub piece: Option<Piece>,
    pub next: VecDeque<Kind>,
    pub hold: Option<Kind>,
    can_hold: bool,
    bag: Bag,
    fall_timer: f32,
    lock_timer: f32,
    lock_resets: u32,
    das: f32,
    pub score: u32,
    pub lines: u32,
    pub hull: f32,
    pub hull_max: f32,
    fire_timer: f32,
    /// > 0 : alarme en cours, tir ennemi dans ce nombre de secondes.
    pub warning: f32,
    /// Le plateau a changé : à redessiner.
    pub dirty: bool,
    /// Après la destruction du croiseur : petite pause avant la suite.
    pub ending: Option<f32>,
    /// Le pilote automatique joue (menu, captures).
    pub autopilot: bool,
    autopilot_timer: f32,
    half_announced: bool,
}

impl Game {
    pub fn new(mode: Mode, rng: &mut Rng) -> Self {
        let (hull, fire) = match mode {
            Mode::Campaign(i) => (CHAPTERS[i].hull, CHAPTERS[i].fire_every),
            Mode::Endless => (0.0, f32::INFINITY),
        };
        let mut bag = Bag::new();
        let next = (0..3).map(|_| bag.next(rng)).collect();
        Game {
            mode,
            board: Board::default(),
            piece: None,
            next,
            hold: None,
            can_hold: true,
            bag,
            fall_timer: 0.0,
            lock_timer: 0.0,
            lock_resets: 0,
            das: 0.0,
            score: 0,
            lines: 0,
            hull,
            hull_max: hull,
            fire_timer: fire,
            warning: 0.0,
            dirty: true,
            ending: None,
            autopilot: false,
            autopilot_timer: 0.0,
            half_announced: false,
        }
    }

    pub fn chapter(&self) -> Option<usize> {
        match self.mode {
            Mode::Campaign(i) => Some(i),
            Mode::Endless => None,
        }
    }

    /// Secondes entre deux descentes d'un cran (accélère avec les lignes).
    fn fall_interval(&self) -> f32 {
        let base = self.chapter().map(|i| CHAPTERS[i].gravity).unwrap_or(0.9);
        (base * 0.95f32.powi((self.lines / 5) as i32)).max(0.08)
    }

    fn spawn(&mut self, kind: Kind, rng: &mut Rng) -> bool {
        let piece = Piece::new(kind);
        self.next.push_back(self.bag.next(rng));
        self.fall_timer = 0.0;
        self.lock_timer = 0.0;
        self.lock_resets = 0;
        let ok = self.board.fits(&piece);
        self.piece = Some(piece);
        ok
    }

    /// Un déplacement réussi au sol relance un peu le délai de pose.
    fn moved(&mut self, piece: Piece) {
        self.piece = Some(piece);
        if self.board.shifted(&piece, IVec2::NEG_Y).is_none() && self.lock_resets < 12 {
            self.lock_timer = 0.0;
            self.lock_resets += 1;
        }
    }

    pub fn try_shift(&mut self, dx: i32) -> bool {
        let Some(p) = self.piece else { return false };
        match self.board.shifted(&p, IVec2::new(dx, 0)) {
            Some(q) => {
                self.moved(q);
                true
            }
            None => false,
        }
    }

    pub fn try_rotate(&mut self) -> bool {
        let Some(p) = self.piece else { return false };
        match self.board.rotated(&p) {
            Some(q) => {
                self.moved(q);
                true
            }
            None => false,
        }
    }

    pub fn try_hold(&mut self, rng: &mut Rng) -> bool {
        let Some(p) = self.piece else { return false };
        if !self.can_hold {
            return false;
        }
        self.can_hold = false;
        let kind = match self.hold.replace(p.kind) {
            Some(k) => k,
            None => self.next.pop_front().unwrap(),
        };
        self.spawn(kind, rng);
        true
    }

    /// Pose la pièce : lignes, dégâts au croiseur, pièce suivante.
    fn lock(&mut self, rng: &mut Rng, events: &mut MessageWriter<NovaEvent>, next_state: &mut NextState<GameState>) {
        let Some(p) = self.piece.take() else { return };
        let rows = self.board.lock(&p);
        self.dirty = true;
        self.can_hold = true;
        events.write(NovaEvent::Lock);
        let n = rows.len() as u32;
        if n > 0 {
            let chapter_bonus = self.chapter().map(|i| i as u32 + 1).unwrap_or(1);
            self.score += [0, 100, 300, 500, 800][n as usize] * chapter_bonus;
            self.lines += n;
            if self.mode != Mode::Endless {
                self.hull = (self.hull - [0.0, 10.0, 24.0, 40.0, 60.0][n as usize]).max(0.0);
            }
            events.write(NovaEvent::Lines(rows));
            if self.hull <= 0.0 && self.mode != Mode::Endless && self.ending.is_none() {
                self.ending = Some(2.5);
                events.write(NovaEvent::CruiserDestroyed);
                return;
            }
        }
        let kind = self.next.pop_front().unwrap();
        if !self.spawn(kind, rng) || self.board.overflowing() {
            self.lose(events, next_state);
        }
    }

    fn lose(&mut self, events: &mut MessageWriter<NovaEvent>, next_state: &mut NextState<GameState>) {
        if self.autopilot {
            // Démo : on recommence discrètement.
            let mode = self.mode;
            let mut rng = Rng::new(self.score as u64 + 7);
            *self = Game { autopilot: true, ..Game::new(mode, &mut rng) };
            return;
        }
        self.piece = None;
        events.write(NovaEvent::Defeat);
        next_state.set(GameState::Defeat);
    }

    pub fn half_hull(&mut self) -> bool {
        if !self.half_announced && self.hull_max > 0.0 && self.hull <= self.hull_max / 2.0 {
            self.half_announced = true;
            return true;
        }
        false
    }
}

/// Le chapitre choisi avant de lancer la partie.
#[derive(Resource, Clone, Copy)]
pub struct NextMode(pub Mode);

fn start_game(mut commands: Commands, mode: Option<Res<NextMode>>, mut rng: ResMut<Rng>) {
    let mode = mode.map(|m| m.0).unwrap_or(Mode::Endless);
    commands.insert_resource(Game::new(mode, &mut rng));
}

/// Dans le menu, le pilote automatique joue en fond.
fn start_demo(mut commands: Commands, mut rng: ResMut<Rng>) {
    commands.insert_resource(Game { autopilot: true, ..Game::new(Mode::Endless, &mut rng) });
}

/// Suivi d'un doigt sur l'écran tactile.
#[derive(Resource, Default)]
struct TouchState {
    start: Vec2,
    last_x: f32,
    moved: bool,
    dropped: bool,
    held_since: f32,
}

#[allow(clippy::too_many_arguments)]
fn controls(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    touches: Res<Touches>,
    mut touch: ResMut<TouchState>,
    mut game: ResMut<Game>,
    mut rng: ResMut<Rng>,
    mut events: MessageWriter<NovaEvent>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if game.autopilot || game.piece.is_none() || game.ending.is_some() {
        return;
    }
    let dt = time.delta_secs();
    let pressed = |a: &[KeyCode]| a.iter().any(|k| keys.pressed(*k));
    let just = |a: &[KeyCode]| a.iter().any(|k| keys.just_pressed(*k));

    // Gauche / droite avec répétition quand on garde la touche enfoncée.
    let left = [KeyCode::ArrowLeft, KeyCode::KeyA];
    let right = [KeyCode::ArrowRight, KeyCode::KeyD];
    let dir = pressed(&right) as i32 - pressed(&left) as i32;
    if just(&left) || just(&right) {
        game.try_shift(dir);
        game.das = 0.16;
    } else if dir != 0 {
        game.das -= dt;
        if game.das <= 0.0 {
            game.try_shift(dir);
            game.das = 0.05;
        }
    }
    if just(&[KeyCode::ArrowUp, KeyCode::KeyW, KeyCode::KeyX]) && game.try_rotate() {
        events.write(NovaEvent::Rotate);
    }
    if just(&[KeyCode::KeyC, KeyCode::ShiftLeft, KeyCode::ShiftRight]) && game.try_hold(&mut rng) {
        events.write(NovaEvent::Hold);
    }
    if just(&[KeyCode::Space]) {
        hard_drop(&mut game, &mut rng, &mut events, &mut next_state);
        return;
    }

    // Tactile : glisser = déplacer, toucher = tourner, glisser vers le bas
    // = poser d'un coup, vers le haut = réserve.
    const COLUMN_PX: f32 = 55.0;
    for t in touches.iter_just_pressed() {
        *touch = TouchState { start: t.position(), last_x: t.position().x, held_since: time.elapsed_secs(), ..default() };
    }
    for t in touches.iter() {
        let p = t.position();
        while p.x - touch.last_x > COLUMN_PX {
            game.try_shift(1);
            touch.last_x += COLUMN_PX;
            touch.moved = true;
        }
        while touch.last_x - p.x > COLUMN_PX {
            game.try_shift(-1);
            touch.last_x -= COLUMN_PX;
            touch.moved = true;
        }
        if !touch.dropped && p.y - touch.start.y > 140.0 {
            touch.dropped = true;
            hard_drop(&mut game, &mut rng, &mut events, &mut next_state);
            return;
        }
    }
    for t in touches.iter_just_released() {
        let d = t.position() - touch.start;
        if touch.moved || touch.dropped {
            continue;
        }
        if d.y < -120.0 {
            if game.try_hold(&mut rng) {
                events.write(NovaEvent::Hold);
            }
        } else if d.length() < 25.0 && time.elapsed_secs() - touch.held_since < 0.35 && game.try_rotate() {
            events.write(NovaEvent::Rotate);
        }
    }
}

fn hard_drop(game: &mut Game, rng: &mut Rng, events: &mut MessageWriter<NovaEvent>, next_state: &mut NextState<GameState>) {
    let Some(p) = game.piece else { return };
    let landed = game.board.drop_position(&p);
    game.score += 2 * (p.pos.y - landed.pos.y).max(0) as u32;
    game.piece = Some(landed);
    events.write(NovaEvent::HardDrop);
    game.lock(rng, events, next_state);
}

/// La pièce descend toute seule ; au sol, elle se pose après un court délai.
fn gravity(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut game: ResMut<Game>,
    mut rng: ResMut<Rng>,
    mut events: MessageWriter<NovaEvent>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if game.ending.is_some() {
        return;
    }
    if game.piece.is_none() {
        let kind = game.next.pop_front().unwrap();
        game.spawn(kind, &mut rng);
        return;
    }
    let dt = time.delta_secs();
    let soft = !game.autopilot && (keys.pressed(KeyCode::ArrowDown) || keys.pressed(KeyCode::KeyS));
    let interval = game.fall_interval() / if soft { 15.0 } else { 1.0 };
    game.fall_timer += dt;
    while game.fall_timer >= interval {
        game.fall_timer -= interval;
        let p = game.piece.unwrap();
        if let Some(lower) = game.board.shifted(&p, IVec2::NEG_Y) {
            game.piece = Some(lower);
            if soft {
                game.score += 1;
            }
        } else {
            break;
        }
    }
    let p = game.piece.unwrap();
    if game.board.shifted(&p, IVec2::NEG_Y).is_none() {
        game.lock_timer += dt;
        if game.lock_timer >= 0.45 || game.lock_resets >= 12 {
            game.lock(&mut rng, &mut events, &mut next_state);
        }
    }
}

/// Le croiseur riposte : alarme, puis une rangée endommagée par le bas.
fn enemy_fire(
    time: Res<Time>,
    mut game: ResMut<Game>,
    mut rng: ResMut<Rng>,
    mut events: MessageWriter<NovaEvent>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let Mode::Campaign(chapter) = game.mode else { return };
    if game.ending.is_some() || game.autopilot && game.piece.is_none() {
        return;
    }
    const WARNING: f32 = 2.0;
    let dt = time.delta_secs();
    game.fire_timer -= dt;
    if game.fire_timer <= WARNING && game.warning <= 0.0 && game.fire_timer > 0.0 {
        game.warning = game.fire_timer;
        events.write(NovaEvent::Warning);
    }
    if game.warning > 0.0 {
        game.warning = (game.warning - dt).max(0.0);
    }
    if game.fire_timer <= 0.0 {
        game.fire_timer = CHAPTERS[chapter].fire_every;
        game.warning = 0.0;
        let hole = rng.below(WIDTH as u32) as i32;
        let ok = game.board.add_damaged_row(hole);
        game.dirty = true;
        events.write(NovaEvent::EnemyShot);
        // La pièce en cours est poussée vers le haut si besoin.
        if let Some(mut p) = game.piece {
            while !game.board.fits(&p) && p.pos.y < HEIGHT + 2 {
                p.pos.y += 1;
            }
            game.piece = Some(p);
        }
        if !ok {
            game.lose(&mut events, &mut next_state);
        }
    }
}

/// Après l'explosion du croiseur : la suite de l'histoire.
fn destroyed(time: Res<Time>, mut game: ResMut<Game>, mut next_state: ResMut<NextState<GameState>>) {
    if let Some(t) = game.ending.as_mut() {
        *t -= time.delta_secs();
        if *t <= 0.0 {
            game.ending = None;
            next_state.set(GameState::Victory);
        }
    }
}

/// Pilote automatique : essaie toutes les positions et garde la meilleure.
fn autopilot(
    time: Res<Time>,
    mut game: ResMut<Game>,
    mut rng: ResMut<Rng>,
    mut events: MessageWriter<NovaEvent>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if !game.autopilot || game.ending.is_some() {
        return;
    }
    let Some(piece) = game.piece else { return };
    game.autopilot_timer += time.delta_secs();
    if game.autopilot_timer < 0.35 {
        return;
    }
    game.autopilot_timer = 0.0;
    if let Some(best) = best_placement(&game.board, &piece) {
        game.piece = Some(best);
        hard_drop(&mut game, &mut rng, &mut events, &mut next_state);
    }
}

fn best_placement(board: &Board, piece: &Piece) -> Option<Piece> {
    let mut best: Option<(f32, Piece)> = None;
    let mut p = *piece;
    for _ in 0..4 {
        for dx in -(WIDTH)..=WIDTH {
            let Some(moved) = board.shifted(&p, IVec2::new(dx, 0)) else { continue };
            let landed = board.drop_position(&moved);
            let mut b = board.clone();
            let lines = b.lock(&landed).len() as f32;
            let score = lines * 8.0 - evaluate(&b);
            if best.is_none_or(|(s, _)| score > s) {
                best = Some((score, landed));
            }
        }
        match board.rotated(&p) {
            Some(r) => p = r,
            None => break,
        }
    }
    best.map(|(_, p)| p)
}

/// Pénalités : hauteur, trous, irrégularités.
fn evaluate(b: &Board) -> f32 {
    let mut heights = [0i32; WIDTH as usize];
    let mut holes = 0;
    for x in 0..WIDTH {
        let mut top = 0;
        for y in (0..HEIGHT).rev() {
            if b.get(IVec2::new(x, y)) != crate::board::Cell::Empty {
                top = top.max(y + 1);
            } else if top > 0 {
                holes += 1;
            }
        }
        heights[x as usize] = top;
    }
    let total: i32 = heights.iter().sum();
    let bumpy: i32 = heights.windows(2).map(|w| (w[0] - w[1]).abs()).sum();
    total as f32 * 0.5 + holes as f32 * 4.0 + bumpy as f32 * 0.3
}
