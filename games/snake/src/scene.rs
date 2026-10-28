//! L'écran de scène (décor, personnages, caméra de présentation, sous-titres) et la liste des aventures.
use bevy::prelude::*;
use game_common::ui::{spawn_gap, spawn_text};

use crate::GameState;
use crate::audio::Sfx;
use crate::levels::{Cam, Scene};
use crate::people::{self, Pose};
use crate::story::{SCENARIOS, StoryProgress, advance};
use crate::ui::plank;
use crate::voyage::Mode;
use crate::world::cell_to_world;

/// Où la caméra doit être (elle s'y rend en douceur).
#[derive(Resource, Default)]
pub struct SceneCamera {
    pub pos: Vec3,
    pub look: Vec3,
}

/// La réplique affichée.
#[derive(Resource, Default)]
struct SceneRun {
    line: usize,
    /// Les sous-titres sont à remettre à jour (entrée dans une scène).
    dirty: bool,
}

#[derive(Component)]
struct SubtitleName;

#[derive(Component)]
struct SubtitleText;

#[derive(Component)]
struct SubtitleCount;

#[derive(Component)]
struct PrevLabel;

#[derive(Component)]
struct NextLabel;

#[derive(Component, Clone, Copy, PartialEq, Eq)]
enum SceneBtn {
    Prev,
    Next,
    Skip,
}

/// Un bouton de la liste des aventures : (aventure, recommencer depuis le début).
#[derive(Component)]
struct ScenarioBtn(usize, bool);

#[derive(Component)]
struct ScenarioBack;

/// Le cadrage d'une scène : où est la caméra et vers quoi elle regarde, selon les personnages.
fn frame(scene: &Scene, cam: Cam) -> SceneCamera {
    let n = scene.actors.len().max(1) as f32;
    let (cx, cz) = (scene.actors.iter().map(|a| a.x).sum::<f32>() / n, scene.actors.iter().map(|a| a.z).sum::<f32>() / n);
    let spread = scene.actors.iter().map(|a| (a.x - cx).abs()).fold(0.0, f32::max) + 1.4;
    let k = (spread / 2.2).max(1.0);
    let c = Vec3::new(cx, 0.0, cz);
    let (offset, aim) = match cam {
        Cam::Large => (Vec3::new(0.0, 2.8, 9.5), Vec3::new(0.0, 0.5, 0.0)),
        Cam::Medium => (Vec3::new(0.0, 1.8, 5.4 * k), Vec3::new(0.0, 0.4, 0.0)),
        Cam::Close => (Vec3::new(0.0, 1.4, 3.0 * k), Vec3::new(0.0, 0.55, 0.0)),
        Cam::High => (Vec3::new(0.0, 5.5, 5.5 * k), Vec3::new(0.0, 0.0, 0.0)),
        Cam::Low => (Vec3::new(0.0, 0.35, 4.4 * k), Vec3::new(0.0, 0.9, 0.0)),
    };
    SceneCamera { pos: c + offset, look: c + aim }
}

/// Le cadrage de la réplique `line` : le dernier changement de caméra jusque-là.
fn camera_for(scene: &Scene, line: usize) -> Cam {
    scene.lines.iter().take(line + 1).rev().find_map(|l| l.camera).unwrap_or(scene.camera)
}

#[derive(Component)]
struct StageActor;

#[allow(clippy::too_many_arguments)]
fn spawn_scene(
    mut commands: Commands,
    mode: Res<Mode>,
    layout: Res<crate::layout::Layout>,
    server: Res<AssetServer>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
    mut cam: ResMut<SceneCamera>,
    mut run: ResMut<SceneRun>,
) {
    let Some(scene) = mode.scene() else { return };
    run.line = 0;
    run.dirty = true;
    *cam = frame(scene, camera_for(scene, 0));
    // Les personnages, tournés l'un vers l'autre (légèrement) et vers la caméra.
    let n = scene.actors.len().max(1) as f32;
    let cx = scene.actors.iter().map(|a| a.x).sum::<f32>() / n;
    for a in &scene.actors {
        let pos = cell_to_world(Vec2::new((layout.w as f32 - 1.0) / 2.0, (layout.h as f32 - 1.0) / 2.0), 0.05, &layout) + Vec3::new(a.x, 0.0, a.z);
        let root = commands
            .spawn((
                Transform::from_translation(pos).with_rotation(Quat::from_rotation_y((cx - a.x) * 0.12)),
                Visibility::default(),
                StageActor,
                DespawnOnExit(GameState::Scene),
            ))
            .id();
        let pose = if a.pose == Pose::Walk { Pose::Walk } else { a.pose };
        people::spawn_model_with(&mut commands, &server, &mut graphs, root, a.kind, pose, 1.0, a.look, a.guide, &a.acc);
    }
    // Lumière d'appoint de face, sans ombre : sinon, avec le soleil dans le dos, les visages restent sombres.
    commands.spawn((
        DirectionalLight { illuminance: 4200.0, color: Color::srgb(1.0, 0.97, 0.92), shadow_maps_enabled: false, ..default() },
        Transform::from_xyz(1.5, 3.5, 9.0).looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Y),
        DespawnOnExit(GameState::Scene),
    ));
    // Sous-titres en bas de l'écran.
    let root = commands
        .spawn((
            Node { position_type: PositionType::Absolute, left: Val::Px(0.0), right: Val::Px(0.0), bottom: Val::Px(0.0), flex_direction: FlexDirection::Column, align_items: AlignItems::Center, padding: UiRect::all(Val::Px(14.0)), row_gap: Val::Px(8.0), ..default() },
            DespawnOnExit(GameState::Scene),
        ))
        .id();
    let bar = commands
        .spawn((
            Node { width: Val::Percent(88.0), max_width: Val::Px(1100.0), flex_direction: FlexDirection::Column, row_gap: Val::Px(4.0), padding: UiRect::axes(Val::Px(26.0), Val::Px(14.0)), border_radius: BorderRadius::all(Val::Px(18.0)), min_height: Val::Px(120.0), ..default() },
            BackgroundColor(Color::srgba(0.09, 0.07, 0.06, 0.82)),
            ChildOf(root),
        ))
        .id();
    let name = spawn_text(&mut commands, bar, "", 24.0);
    commands.entity(name).insert((SubtitleName, TextColor(Color::srgb(1.0, 0.84, 0.35)), TextLayout::justify(Justify::Left)));
    let text = spawn_text(&mut commands, bar, "", 29.0);
    commands.entity(text).insert((SubtitleText, TextColor(Color::srgb(1.0, 0.97, 0.9)), TextLayout::justify(Justify::Left)));
    let buttons = commands.spawn((Node { column_gap: Val::Px(12.0), align_items: AlignItems::Center, ..default() }, ChildOf(root))).id();
    let (b, t) = plank(&mut commands, &server, buttons, "panneau_bois_fonce_large", "fleche_precedent", 200.0, "Précédent", None);
    commands.entity(b).insert(SceneBtn::Prev);
    commands.entity(t).insert(PrevLabel);
    let count = spawn_text(&mut commands, buttons, "", 20.0);
    commands.entity(count).insert((SubtitleCount, TextColor(Color::srgb(1.0, 1.0, 1.0))));
    let (b, t) = plank(&mut commands, &server, buttons, "panneau_bois_dore_large", "fleche_suivant", 200.0, "Suivant", None);
    commands.entity(b).insert(SceneBtn::Next);
    commands.entity(t).insert(NextLabel);
    // « Passer » en haut à droite.
    let skip = commands
        .spawn((
            Node { position_type: PositionType::Absolute, right: Val::Px(16.0), top: Val::Px(14.0), ..default() },
            DespawnOnExit(GameState::Scene),
        ))
        .id();
    let (b, _) = plank(&mut commands, &server, skip, "panneau_bois_fonce_large", "fleche_suivant", 170.0, "Passer", None);
    commands.entity(b).insert(SceneBtn::Skip);
}

/// Met à jour les sous-titres et le cadrage quand la réplique change.
#[allow(clippy::too_many_arguments)]
fn update_scene(
    mode: Res<Mode>,
    mut run: ResMut<SceneRun>,
    mut cam: ResMut<SceneCamera>,
    mut name: Query<&mut Text, (With<SubtitleName>, Without<SubtitleText>, Without<SubtitleCount>, Without<PrevLabel>, Without<NextLabel>)>,
    mut text: Query<&mut Text, (With<SubtitleText>, Without<SubtitleName>, Without<SubtitleCount>, Without<PrevLabel>, Without<NextLabel>)>,
    mut count: Query<&mut Text, (With<SubtitleCount>, Without<SubtitleName>, Without<SubtitleText>, Without<PrevLabel>, Without<NextLabel>)>,
    mut next_label: Query<&mut Text, (With<NextLabel>, Without<SubtitleName>, Without<SubtitleText>, Without<SubtitleCount>, Without<PrevLabel>)>,
    mut seen: Local<Option<(usize, usize)>>,
) {
    let Some(scene) = mode.scene() else { return };
    let key = (run.line, scene.lines.len());
    if *seen == Some(key) && !run.dirty {
        return;
    }
    *seen = Some(key);
    run.dirty = false;
    let line = scene.lines.get(run.line);
    for mut t in &mut name {
        t.0 = line.map_or(String::new(), |l| l.who.clone());
    }
    for mut t in &mut text {
        t.0 = line.map_or(String::new(), |l| l.text.clone());
    }
    for mut t in &mut count {
        t.0 = if scene.lines.is_empty() { String::new() } else { format!("{} / {}", run.line + 1, scene.lines.len()) };
    }
    let last = run.line + 1 >= scene.lines.len();
    for mut t in &mut next_label {
        t.0 = if last { "Continuer".to_owned() } else { "Suivant".to_owned() };
    }
    *cam = frame(scene, camera_for(scene, run.line));
}

fn finish(mode: &mut Mode, progress: &mut StoryProgress, next: &mut NextState<GameState>) {
    advance(mode, progress, next);
}

#[allow(clippy::too_many_arguments)]
fn scene_click(
    click: On<PointerClick>,
    buttons: Query<&SceneBtn>,
    mut run: ResMut<SceneRun>,
    mut mode: ResMut<Mode>,
    mut progress: ResMut<StoryProgress>,
    mut next: ResMut<NextState<GameState>>,
    mut sfx: MessageWriter<Sfx>,
) {
    let Ok(b) = buttons.get(click.entity) else { return };
    sfx.write(Sfx::Click);
    let Some(scene) = mode.scene() else { return };
    match b {
        SceneBtn::Prev => run.line = run.line.saturating_sub(1),
        SceneBtn::Next => {
            if run.line + 1 < scene.lines.len() {
                run.line += 1;
            } else {
                finish(&mut mode, &mut progress, &mut next);
            }
        }
        SceneBtn::Skip => finish(&mut mode, &mut progress, &mut next),
    }
}

fn scene_keys(
    keys: Res<ButtonInput<KeyCode>>,
    mut run: ResMut<SceneRun>,
    mut mode: ResMut<Mode>,
    mut progress: ResMut<StoryProgress>,
    mut next: ResMut<NextState<GameState>>,
) {
    let Some(scene) = mode.scene() else { return };
    if keys.just_pressed(KeyCode::Escape) {
        next.set(if crate::levels::preview_active() { GameState::Menu } else { GameState::Scenarios });
    } else if keys.just_pressed(KeyCode::ArrowLeft) {
        run.line = run.line.saturating_sub(1);
    } else if keys.just_pressed(KeyCode::ArrowRight) || keys.just_pressed(KeyCode::Enter) || keys.just_pressed(KeyCode::Space) {
        if run.line + 1 < scene.lines.len() {
            run.line += 1;
        } else {
            finish(&mut mode, &mut progress, &mut next);
        }
    }
}

// ---------------------------------------------------------------- liste des aventures

fn spawn_scenarios(mut commands: Commands, server: Res<AssetServer>, progress: Res<StoryProgress>) {
    let root = commands
        .spawn((
            Node { width: Val::Percent(100.0), height: Val::Percent(100.0), flex_direction: FlexDirection::Column, align_items: AlignItems::Center, justify_content: JustifyContent::Center, row_gap: Val::Px(10.0), padding: UiRect::all(Val::Px(16.0)), ..default() },
            DespawnOnExit(GameState::Scenarios),
        ))
        .id();
    let title = spawn_text(&mut commands, root, "Mode scénario", 48.0);
    commands.entity(title).insert(TextColor(Color::srgb(1.0, 0.96, 0.84)));
    spawn_gap(&mut commands, root, 6.0);
    for (i, sc) in SCENARIOS.iter().enumerate() {
        let done = progress.next_step(&sc.title);
        let status = if done == 0 {
            sc.description.clone()
        } else if done >= sc.steps.len() {
            "Terminée — rejouer".to_owned()
        } else {
            format!("Reprendre : étape {} sur {}", done + 1, sc.steps.len())
        };
        let row = commands.spawn((Node { column_gap: Val::Px(10.0), align_items: AlignItems::Center, ..default() }, ChildOf(root))).id();
        let (b, _) = plank(&mut commands, &server, row, "panneau_bois_dore_large", "icone_carte", 460.0, sc.title.clone(), Some(&status));
        commands.entity(b).insert(ScenarioBtn(i, false));
        if done > 0 && done < sc.steps.len() {
            let (b, _) = plank(&mut commands, &server, row, "panneau_bois_fonce_large", "icone_infini", 200.0, "Recommencer", None);
            commands.entity(b).insert(ScenarioBtn(i, true));
        }
    }
    spawn_gap(&mut commands, root, 8.0);
    let (b, _) = plank(&mut commands, &server, root, "panneau_bois_fonce_large", "fleche_precedent", 230.0, "Retour", None);
    commands.entity(b).insert(ScenarioBack);
}

#[allow(clippy::too_many_arguments)]
fn scenarios_click(
    click: On<PointerClick>,
    buttons: Query<&ScenarioBtn>,
    back: Query<(), With<ScenarioBack>>,
    progress: Res<StoryProgress>,
    mut mode: ResMut<Mode>,
    mut next: ResMut<NextState<GameState>>,
    mut sfx: MessageWriter<Sfx>,
) {
    if back.contains(click.entity) {
        sfx.write(Sfx::Click);
        next.set(GameState::Menu);
        return;
    }
    let Ok(ScenarioBtn(i, restart)) = buttons.get(click.entity) else { return };
    sfx.write(Sfx::Click);
    let sc = &SCENARIOS[*i];
    let saved = progress.next_step(&sc.title);
    let step = if *restart || saved >= sc.steps.len() { 0 } else { saved };
    *mode = Mode::Scenario { scenario: *i, step };
    next.set(GameState::Step);
}

fn scenarios_keys(keys: Res<ButtonInput<KeyCode>>, mut next: ResMut<NextState<GameState>>) {
    if keys.just_pressed(KeyCode::Escape) {
        next.set(GameState::Menu);
    }
}

pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SceneCamera>()
            .init_resource::<SceneRun>()
            .add_systems(OnEnter(GameState::Scene), (crate::voyage::sync_layout, spawn_scene).chain())
            .add_systems(Update, (update_scene, scene_keys).run_if(in_state(GameState::Scene)))
            .add_systems(OnEnter(GameState::Scenarios), spawn_scenarios)
            .add_systems(Update, scenarios_keys.run_if(in_state(GameState::Scenarios)))
            .add_observer(scene_click)
            .add_observer(scenarios_click);
    }
}
