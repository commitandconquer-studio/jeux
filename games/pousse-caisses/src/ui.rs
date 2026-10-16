//! Écrans : accueil, choix du niveau, barre de jeu, niveau réussi.
use bevy::prelude::*;
use game_common::audio::SoundOn;
use game_common::ui::{GameHelp, spawn_help_button, spawn_studio_logo};
use game_common::ui::{
    PANEL, UiButton, UiKitPlugin, spawn_button, spawn_button_sized, spawn_gap, spawn_panel, spawn_panel_bottom,
    spawn_panel_left, spawn_text,
};

use crate::GameState;
use crate::audio::UiClick;
use crate::play::{Play, PlayEvent, Progress, load_level, restart, undo};

/// Aide du jeu (bouton « Aide » du menu).
const HELP: &str = "Rangez toutes les caisses sur les cibles !\n\
Le crapaud pousse les caisses, il ne peut pas les tirer.\n\
\n\
Flèches ou ZQSD, ou glisser du doigt : se déplacer\n\
U ou Retour arrière : annuler · R : recommencer\n\
M : son · Échap : menu";

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(UiKitPlugin { universe: game_common::universe::Universe::Croa })
            .insert_resource(GameHelp(HELP))
            .add_systems(OnEnter(GameState::Menu), spawn_menu)
            .add_systems(OnEnter(GameState::Levels), spawn_levels)
            .add_systems(OnEnter(GameState::Playing), spawn_hud)
            .add_systems(OnEnter(GameState::Won), spawn_won)
            .add_systems(Update, (update_hud, update_sound_label, keyboard_shortcuts))
            .add_observer(on_click);
    }
}

#[derive(Component, Clone, Copy)]
enum UiAction {
    Continue,
    ChooseLevel,
    Pick(usize),
    Back,
    Undo,
    Restart,
    Next,
    Replay,
    Menu,
    ToggleSound,
    #[cfg_attr(any(target_os = "android", target_arch = "wasm32"), allow(dead_code))]
    Quit,
}

#[derive(Component)]
struct HudText;

#[derive(Component)]
struct SoundLabel;

fn button(commands: &mut Commands, parent: Entity, label: impl Into<String>, action: UiAction) -> Entity {
    let (b, text) = spawn_button(commands, parent, label);
    commands.entity(b).insert(action);
    text
}

fn spawn_menu(mut commands: Commands, progress: Res<Progress>, sound: Res<SoundOn>) {
    let panel = spawn_panel_left(&mut commands, DespawnOnExit(GameState::Menu));
    spawn_text(&mut commands, panel, "POUSSE-CAISSES", 56.0);
    spawn_gap(&mut commands, panel, 8.0);
    let total = progress.levels.len();
    let label = if progress.unlocked >= total && total > 0 {
        format!("Continuer : niveau {total}")
    } else {
        format!("Continuer : niveau {}", progress.unlocked)
    };
    button(&mut commands, panel, label, UiAction::Continue);
    button(&mut commands, panel, "Choisir un niveau", UiAction::ChooseLevel);
    let t = button(&mut commands, panel, sound.label(), UiAction::ToggleSound);
    commands.entity(t).insert(SoundLabel);
    spawn_help_button(&mut commands, panel);
    #[cfg(not(any(target_os = "android", target_arch = "wasm32")))]
    button(&mut commands, panel, "Quitter", UiAction::Quit);
    spawn_studio_logo(&mut commands, panel);
}

/// Grille des niveaux : ceux pas encore débloqués sont grisés.
fn spawn_levels(mut commands: Commands, progress: Res<Progress>) {
    let panel = spawn_panel(&mut commands, DespawnOnExit(GameState::Levels));
    spawn_text(&mut commands, panel, "Choisir un niveau", 40.0);
    let grid = commands
        .spawn((
            Node {
                display: Display::Grid,
                grid_template_columns: RepeatedGridTrack::flex(2, 1.0),
                row_gap: Val::Px(10.0),
                column_gap: Val::Px(10.0),
                ..default()
            },
            ChildOf(panel),
        ))
        .id();
    for (i, level) in progress.levels.iter().enumerate() {
        let open = i < progress.unlocked;
        let label = if open { format!("{} · {}", i + 1, level.name) } else { format!("{} · ???", i + 1) };
        let (b, _) = spawn_button_sized(&mut commands, grid, label, 300.0, 46.0, 19.0);
        if open {
            commands.entity(b).insert(UiAction::Pick(i));
        } else {
            commands.entity(b).remove::<UiButton>().insert(BackgroundColor(Color::srgb(0.82, 0.82, 0.80)));
        }
    }
    spawn_gap(&mut commands, panel, 4.0);
    button(&mut commands, panel, "Retour", UiAction::Back);
}

/// Barre du haut pendant la partie : niveau, coups, et boutons utiles au doigt.
fn spawn_hud(mut commands: Commands) {
    let bar = commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                top: Val::Px(12.0),
                padding: UiRect::horizontal(Val::Px(12.0)),
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                ..default()
            },
            DespawnOnExit(GameState::Playing),
        ))
        .id();
    let info = commands
        .spawn((
            Node {
                padding: UiRect::axes(Val::Px(18.0), Val::Px(8.0)),
                border_radius: BorderRadius::all(Val::Px(20.0)),
                ..default()
            },
            BackgroundColor(PANEL),
            ChildOf(bar),
        ))
        .id();
    let text = spawn_text(&mut commands, info, "", 22.0);
    commands.entity(text).insert(HudText);
    let buttons = commands
        .spawn((Node { column_gap: Val::Px(8.0), ..default() }, ChildOf(bar)))
        .id();
    for (label, action) in [("Annuler", UiAction::Undo), ("Recommencer", UiAction::Restart), ("Menu", UiAction::Menu)] {
        let (b, _) = spawn_button_sized(&mut commands, buttons, label, 150.0, 44.0, 18.0);
        commands.entity(b).insert(action);
    }
}

fn spawn_won(mut commands: Commands, play: Res<Play>, progress: Res<Progress>) {
    let panel = spawn_panel_bottom(&mut commands, DespawnOnExit(GameState::Won));
    let last = play.index + 1 >= progress.levels.len();
    let title = if last { "Bravo, tous les niveaux sont réussis !" } else { "Niveau réussi !" };
    spawn_text(&mut commands, panel, title, 36.0);
    let plural = if play.moves > 1 { "s" } else { "" };
    spawn_text(&mut commands, panel, format!("« {} » en {} coup{plural}", play.level.name, play.moves), 22.0);
    if !last {
        button(&mut commands, panel, "Niveau suivant", UiAction::Next);
    }
    button(&mut commands, panel, "Rejouer ce niveau", UiAction::Replay);
    button(&mut commands, panel, "Menu", UiAction::Menu);
}

fn update_hud(play: Res<Play>, progress: Res<Progress>, mut text: Query<&mut Text, With<HudText>>) {
    for mut t in &mut text {
        let s = format!("Niveau {}/{} · {} · Coups : {}", play.index + 1, progress.levels.len(), play.level.name, play.moves);
        if t.0 != s {
            t.0 = s;
        }
    }
}

fn update_sound_label(sound: Res<SoundOn>, mut labels: Query<&mut Text, With<SoundLabel>>) {
    if sound.is_changed() {
        for mut t in &mut labels {
            t.0 = sound.label().into();
        }
    }
}

fn keyboard_shortcuts(
    keys: Res<ButtonInput<KeyCode>>,
    state: Res<State<GameState>>,
    mut play: ResMut<Play>,
    progress: Res<Progress>,
    mut next: ResMut<NextState<GameState>>,
) {
    let confirm = keys.just_pressed(KeyCode::Enter) || keys.just_pressed(KeyCode::Space);
    let back = keys.just_pressed(KeyCode::Escape);
    match state.get() {
        GameState::Menu if confirm => {
            load_level(&mut play, &progress, progress.unlocked - 1);
            next.set(GameState::Playing);
        }
        GameState::Levels | GameState::Playing if back => next.set(GameState::Menu),
        GameState::Won if confirm => next.set(GameState::Playing),
        GameState::Won if back => next.set(GameState::Menu),
        _ => {}
    }
}

#[allow(clippy::too_many_arguments)]
fn on_click(
    click: On<PointerClick>,
    actions: Query<&UiAction>,
    mut play: ResMut<Play>,
    progress: Res<Progress>,
    mut next: ResMut<NextState<GameState>>,
    mut sound: ResMut<SoundOn>,
    mut clicks: MessageWriter<UiClick>,
    mut events: MessageWriter<PlayEvent>,
    mut exit: MessageWriter<AppExit>,
) {
    let Ok(action) = actions.get(click.entity) else {
        return;
    };
    clicks.write(UiClick);
    match *action {
        UiAction::Continue => {
            load_level(&mut play, &progress, progress.unlocked - 1);
            next.set(GameState::Playing);
        }
        UiAction::ChooseLevel => next.set(GameState::Levels),
        UiAction::Pick(i) => {
            load_level(&mut play, &progress, i);
            next.set(GameState::Playing);
        }
        UiAction::Back | UiAction::Menu => next.set(GameState::Menu),
        UiAction::Undo => undo(&mut play, &mut events),
        UiAction::Restart => restart(&mut play),
        UiAction::Next => next.set(GameState::Playing),
        UiAction::Replay => {
            restart(&mut play);
            next.set(GameState::Playing);
        }
        UiAction::ToggleSound => sound.0 = !sound.0,
        UiAction::Quit => {
            exit.write(AppExit::Success);
        }
    }
}
