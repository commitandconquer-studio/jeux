//! La partie : grilles du Livre et défi du jour, marques au clic ou au doigt,
//! mode Novice, conflits, indices, chronomètre, victoire, sauvegarde.
use bevy::prelude::*;
use concile_core::{Puzzle, conflicts, next_deduction};
use game_common::calendar::Date;
use game_common::{iso_projection, save};

use crate::GameState;
use crate::board::{self, CellView, Look, MapPart, MarkKind, SealBody};

const SAVE: &str = "concile";
const BOOK_TXT: &str = include_str!("../puzzles.txt");
const CAM_OFFSET: Vec3 = Vec3::new(0.0, 15.0, 4.5);

pub struct PlayPlugin;

impl Plugin for PlayPlugin {
    fn build(&self, app: &mut App) {
        let book: Vec<Puzzle> = BOOK_TXT.lines().filter(|l| !l.starts_with('#') && !l.trim().is_empty()).filter_map(Puzzle::from_line).collect();
        let progress = Progress::load(book.len());
        let first = book[progress.next_unsolved().unwrap_or(0)].clone();
        app.init_resource::<Look>()
            .add_message::<PlayEvent>()
            .insert_resource(Book(book))
            .insert_resource(progress)
            .insert_resource(Play::new(first, Kind::Book(0)))
            .add_systems(Startup, setup)
            .add_systems(
                Update,
                (
                    rebuild_map,
                    (pointer, tick, victory).chain().run_if(in_state(GameState::Playing)),
                    show_marks,
                    shake_conflicts,
                    board::flicker,
                    camera,
                ),
            );
    }
}

/// Toutes les grilles de la campagne, dans l'ordre.
#[derive(Resource)]
pub struct Book(pub Vec<Puzzle>);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Kind {
    /// Grille numéro `i` du Livre des Conciles.
    Book(usize),
    /// Défi du jour.
    Daily(Date),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Mark {
    Empty,
    Cross,
    Bishop,
}

#[derive(Message, Clone, Copy)]
pub enum PlayEvent {
    Cross,
    Bishop,
    Erase,
    Conflict,
    Hint,
    Won,
}

/// Réglages et progression du joueur (sauvegardés).
#[derive(Resource)]
pub struct Progress {
    pub solved: Vec<bool>,
    /// Meilleur temps (secondes) par grille du Livre.
    pub best: Vec<Option<u32>>,
    pub novice: bool,
    pub feasts: bool,
    /// Dernier défi du jour réussi, et série de jours consécutifs.
    pub daily_done: Option<Date>,
    pub streak: u32,
}

impl Progress {
    fn load(len: usize) -> Self {
        let solved: Vec<bool> = save::load(SAVE, "resolus").map(|s| s.chars().map(|c| c == '1').collect()).unwrap_or_default();
        let best: Vec<Option<u32>> = save::load(SAVE, "temps").map(|s| s.split(',').map(|t| t.parse().ok()).collect()).unwrap_or_default();
        Progress {
            solved: (0..len).map(|i| solved.get(i).copied().unwrap_or(false)).collect(),
            best: (0..len).map(|i| best.get(i).copied().flatten()).collect(),
            novice: save::load(SAVE, "novice").map(|s| s == "1").unwrap_or(true),
            feasts: save::load(SAVE, "fetes").map(|s| s == "1").unwrap_or(true),
            daily_done: save::load(SAVE, "defi").and_then(|s| Date::parse(&s)),
            streak: save::load(SAVE, "serie").and_then(|s| s.parse().ok()).unwrap_or(0),
        }
    }

    pub fn store(&self) {
        save::store(SAVE, "resolus", &self.solved.iter().map(|&b| if b { '1' } else { '0' }).collect::<String>());
        save::store(SAVE, "temps", &self.best.iter().map(|t| t.map(|v| v.to_string()).unwrap_or_default()).collect::<Vec<_>>().join(","));
        save::store(SAVE, "novice", if self.novice { "1" } else { "0" });
        save::store(SAVE, "fetes", if self.feasts { "1" } else { "0" });
        if let Some(d) = self.daily_done {
            save::store(SAVE, "defi", &format!("{:04}-{:02}-{:02}", d.year, d.month, d.day));
        }
        save::store(SAVE, "serie", &self.streak.to_string());
    }

    pub fn next_unsolved(&self) -> Option<usize> {
        self.solved.iter().position(|&s| !s)
    }

    pub fn daily_solved_today(&self) -> bool {
        self.daily_done == Some(Date::today())
    }
}

/// La partie en cours.
#[derive(Resource)]
pub struct Play {
    pub puzzle: Puzzle,
    pub kind: Kind,
    pub marks: Vec<Mark>,
    history: Vec<Vec<Mark>>,
    pub time: f32,
    pub won: bool,
    built: bool,
    /// Indice affiché : cases en surbrillance, explication, temps restant.
    pub hint: Option<(Vec<usize>, String, f32)>,
    /// Glisser pour barrer : marque appliquée aux cases survolées.
    drag: Option<Mark>,
    last_cell: Option<usize>,
}

impl Play {
    pub fn new(puzzle: Puzzle, kind: Kind) -> Self {
        let n = puzzle.n;
        Play { puzzle, kind, marks: vec![Mark::Empty; n * n], history: Vec::new(), time: 0.0, won: false, built: false, hint: None, drag: None, last_cell: None }
    }

    pub fn n(&self) -> usize {
        self.puzzle.n
    }

    pub fn bishops(&self) -> Vec<usize> {
        (0..self.marks.len()).filter(|&i| self.marks[i] == Mark::Bishop).collect()
    }

    fn placed(&self) -> Vec<(usize, usize)> {
        let n = self.n();
        self.bishops().into_iter().map(|i| (i / n, i % n)).collect()
    }

    /// Évêques en conflit avec un autre.
    pub fn conflicts(&self) -> Vec<usize> {
        let n = self.n();
        conflicts(n, &self.puzzle.regions, &self.placed()).into_iter().map(|(r, c)| r * n + c).collect()
    }

    /// Cases exclues d'office par les évêques posés (mode Novice).
    pub fn shadowed(&self, i: usize) -> bool {
        let n = self.n();
        let (r, c) = (i / n, i % n);
        self.marks[i] != Mark::Bishop
            && self.bishops().into_iter().any(|b| {
                let (br, bc) = (b / n, b % n);
                br == r || bc == c || self.puzzle.regions[b] == self.puzzle.regions[i] || (br.abs_diff(r) <= 1 && bc.abs_diff(c) <= 1)
            })
    }

    fn remember(&mut self) {
        self.history.push(self.marks.clone());
        if self.history.len() > 300 {
            self.history.remove(0);
        }
    }

    pub fn undo(&mut self) -> bool {
        match self.history.pop() {
            Some(m) => {
                self.marks = m;
                true
            }
            None => false,
        }
    }

    pub fn clear(&mut self) {
        if self.marks.iter().any(|&m| m != Mark::Empty) {
            self.remember();
            self.marks.iter_mut().for_each(|m| *m = Mark::Empty);
        }
    }

    /// Indice : d'abord les erreurs, sinon la prochaine déduction logique.
    pub fn compute_hint(&mut self) {
        let n = self.n();
        let wrong: Vec<usize> = self.bishops().into_iter().filter(|&i| !self.puzzle.is_bishop(i / n, i % n)).collect();
        let hint = if !wrong.is_empty() {
            (wrong, "Cet évêque n'est pas à sa place : retirez-le.".to_string())
        } else {
            let bad_cross: Vec<usize> = (0..n * n).filter(|&i| self.marks[i] == Mark::Cross && self.puzzle.is_bishop(i / n, i % n)).collect();
            if !bad_cross.is_empty() {
                (bad_cross, "Cette croix cache un évêque : effacez-la.".to_string())
            } else {
                let crossed: Vec<usize> = (0..n * n).filter(|&i| self.marks[i] == Mark::Cross).collect();
                match next_deduction(n, &self.puzzle.regions, &self.bishops(), &crossed) {
                    Some(d) => match d.place {
                        Some(i) => (vec![i], format!("Un évêque peut être posé ici. {}", d.technique.explain())),
                        None => (d.exclude, format!("Ces cases sont à barrer. {}", d.technique.explain())),
                    },
                    None => (Vec::new(), "Tout ce qui est posé est juste : continuez !".to_string()),
                }
            }
        };
        self.hint = Some((hint.0, hint.1, 7.0));
    }
}

#[derive(Component)]
struct MainCamera;

fn setup(mut commands: Commands, look: Res<Look>, mut mats: ResMut<Assets<StandardMaterial>>) {
    commands.spawn((Camera3d::default(), iso_projection(12.0, 10.0), Transform::from_translation(CAM_OFFSET).looking_at(Vec3::ZERO, Vec3::Y), MainCamera));
    board::spawn_hall(&mut commands, &look, &mut mats);
}

/// Construit la carte quand la grille change.
fn rebuild_map(
    mut commands: Commands,
    mut play: ResMut<Play>,
    look: Res<Look>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<StandardMaterial>>,
    old: Query<Entity, With<MapPart>>,
    mut cam: Query<&mut Projection, With<MainCamera>>,
) {
    if play.built {
        return;
    }
    play.built = true;
    for e in &old {
        commands.entity(e).despawn();
    }
    let n = play.n();
    board::spawn_map(&mut commands, &look, &mut meshes, &mut mats, n, &play.puzzle.regions);
    for mut p in &mut cam {
        *p = iso_projection(n as f32 + 6.0, n as f32 + 3.6);
    }
}

/// Case sous le pointeur (souris ou doigt).
fn cell_under(play: &Play, cam: &Camera, cam_tf: &GlobalTransform, screen: Vec2) -> Option<usize> {
    let ray = cam.viewport_to_world(cam_tf, screen).ok()?;
    let t = ray.intersect_plane(Vec3::Y * board::TOP, InfinitePlane3d::new(Vec3::Y))?;
    let p = ray.get_point(t);
    let n = play.n();
    let off = n as f32 / 2.0;
    let (c, r) = ((p.x + off).floor(), (p.z + off).floor());
    (c >= 0.0 && r >= 0.0 && (c as usize) < n && (r as usize) < n).then(|| r as usize * n + c as usize)
}

/// Un clic (ou un toucher) fait tourner la case : vide → croix → évêque → vide.
/// En gardant appuyé, on barre (ou on efface) toutes les cases survolées.
#[allow(clippy::too_many_arguments)]
fn pointer(
    mouse: Res<ButtonInput<MouseButton>>,
    touches: Res<Touches>,
    windows: Query<&Window>,
    cam: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut play: ResMut<Play>,
    mut events: MessageWriter<PlayEvent>,
) {
    if play.won {
        return;
    }
    let Ok((camera, cam_tf)) = cam.single() else { return };
    let (pressed, held, pos) = if let Some(t) = touches.iter().next() {
        (touches.just_pressed(t.id()), true, Some(t.position()))
    } else {
        let pos = windows.single().ok().and_then(|w| w.cursor_position());
        (mouse.just_pressed(MouseButton::Left), mouse.pressed(MouseButton::Left), pos)
    };
    if !held {
        play.drag = None;
        play.last_cell = None;
        return;
    }
    let Some(cell) = pos.and_then(|p| cell_under(&play, camera, cam_tf, p)) else { return };
    let before = play.marks[cell];
    if pressed {
        play.remember();
        let after = match before {
            Mark::Empty => Mark::Cross,
            Mark::Cross => Mark::Bishop,
            Mark::Bishop => Mark::Empty,
        };
        play.marks[cell] = after;
        // Glisser ensuite : on barre si on vient de barrer, on efface si on vient d'effacer.
        play.drag = match after {
            Mark::Cross => Some(Mark::Cross),
            Mark::Empty => Some(Mark::Empty),
            Mark::Bishop => None,
        };
        play.last_cell = Some(cell);
        play.hint = None;
        events.write(match after {
            Mark::Cross => PlayEvent::Cross,
            Mark::Bishop => PlayEvent::Bishop,
            Mark::Empty => PlayEvent::Erase,
        });
        if after == Mark::Bishop && play.conflicts().contains(&cell) {
            events.write(PlayEvent::Conflict);
        }
    } else if let Some(paint) = play.drag {
        if play.last_cell != Some(cell) && before != Mark::Bishop && before != paint {
            play.marks[cell] = paint;
            events.write(if paint == Mark::Cross { PlayEvent::Cross } else { PlayEvent::Erase });
        }
        play.last_cell = Some(cell);
    }
}

/// Tous les évêques sont bien placés : le concile peut commencer.
fn victory(mut play: ResMut<Play>, mut progress: ResMut<Progress>, mut events: MessageWriter<PlayEvent>, mut next: ResMut<NextState<GameState>>) {
    let placed: Vec<(usize, usize)> = play.placed();
    if play.won || !play.puzzle.is_solved_by(&placed) {
        return;
    }
    play.won = true;
    play.hint = None;
    let secs = play.time.round() as u32;
    match play.kind {
        Kind::Book(i) => {
            progress.solved[i] = true;
            progress.best[i] = Some(progress.best[i].map_or(secs, |b| b.min(secs)));
        }
        Kind::Daily(d) => {
            if progress.daily_done != Some(d) {
                let consecutive = progress.daily_done.is_some_and(|p| p.plus(1) == d);
                progress.streak = if consecutive { progress.streak + 1 } else { 1 };
                progress.daily_done = Some(d);
            }
        }
    }
    progress.store();
    events.write(PlayEvent::Won);
    next.set(GameState::Won);
}

fn tick(time: Res<Time>, mut play: ResMut<Play>) {
    if !play.won {
        play.time += time.delta_secs();
    }
    if let Some((_, _, t)) = play.hint.as_mut() {
        *t -= time.delta_secs();
        if *t <= 0.0 {
            play.hint = None;
        }
    }
}

/// Affiche les marques de chaque case selon l'état de la partie.
fn show_marks(play: Res<Play>, progress: Res<Progress>, mut q: Query<(&CellView, &MarkKind, &mut Visibility)>) {
    for (cell, kind, mut vis) in &mut q {
        let i = cell.0;
        if i >= play.marks.len() {
            continue;
        }
        let show = match kind {
            MarkKind::Cross => play.marks[i] == Mark::Cross,
            MarkKind::Seal => play.marks[i] == Mark::Bishop,
            MarkKind::Hatch => progress.novice && play.marks[i] == Mark::Empty && play.shadowed(i),
            MarkKind::Hint => play.hint.as_ref().is_some_and(|(cells, _, _)| cells.contains(&i)),
        };
        let want = if show { Visibility::Inherited } else { Visibility::Hidden };
        if *vis != want {
            *vis = want;
        }
    }
}

/// Les évêques en conflit rougissent et tremblent (ils se disputent !).
fn shake_conflicts(
    time: Res<Time>,
    play: Res<Play>,
    look: Res<Look>,
    seals: Query<(&CellView, &MarkKind, &Children)>,
    mut bodies: Query<(&mut MeshMaterial3d<StandardMaterial>, &mut Transform), With<SealBody>>,
) {
    let bad = play.conflicts();
    let t = time.elapsed_secs();
    for (cell, kind, children) in &seals {
        if *kind != MarkKind::Seal {
            continue;
        }
        let angry = bad.contains(&cell.0);
        for child in children.iter() {
            if let Ok((mut mat, mut tf)) = bodies.get_mut(child) {
                let want = if angry { &look.wax_angry } else { &look.wax };
                if mat.0 != *want {
                    mat.0 = want.clone();
                }
                tf.translation.x = if angry { (t * 40.0 + cell.0 as f32).sin() * 0.04 } else { 0.0 };
            }
        }
    }
}

/// Vue du dessus, légèrement inclinée ; dans les menus, la carte se décale à droite.
fn camera(time: Res<Time>, state: Res<State<GameState>>, mut shift: Local<f32>, mut q: Query<&mut Transform, With<MainCamera>>, play: Res<Play>) {
    let side = matches!(state.get(), GameState::Menu | GameState::Won);
    let target = if side { -(play.n() as f32 * 0.55 + 1.5) } else { 0.0 };
    *shift += (target - *shift) * (time.delta_secs() * 3.0).min(1.0);
    for mut tf in &mut q {
        let look = Vec3::X * *shift;
        *tf = Transform::from_translation(look + CAM_OFFSET).looking_at(look, Vec3::Y);
    }
}

/// Lance une grille (depuis les menus).
pub fn start(commands: &mut Commands, puzzle: Puzzle, kind: Kind) {
    commands.insert_resource(Play::new(puzzle, kind));
}
