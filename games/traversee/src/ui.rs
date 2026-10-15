//! Écrans : accueil, fin de partie, et affichage du score.
use bevy::prelude::*;
use game_common::Score;
use game_common::ui::{UiKitPlugin, spawn_button, spawn_gap, spawn_hud, spawn_panel, spawn_text};

use crate::GameState;
use crate::audio::{Sfx, SoundOn};
use crate::frog::Death;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(UiKitPlugin { font_path: game_common::sounds::FONT })
            .add_systems(Startup, setup_hud)
            .add_systems(OnEnter(GameState::Menu), spawn_menu)
            .add_systems(OnEnter(GameState::GameOver), spawn_game_over)
            .add_systems(Update, (update_hud, update_sound_label, keyboard_shortcuts))
            .add_observer(on_click);
    }
}

#[derive(Component, Clone, Copy)]
enum UiAction {
    Play,
    Menu,
    ToggleSound,
    #[cfg_attr(any(target_os = "android", target_arch = "wasm32"), allow(dead_code))]
    Quit,
}

#[derive(Component)]
struct Hud;

#[derive(Component)]
struct HudText;

/// Texte du bouton « Son », mis à jour quand on l'active ou le coupe.
#[derive(Component)]
struct SoundLabel;

fn button(commands: &mut Commands, parent: Entity, label: &str, action: UiAction) -> Entity {
    let (b, text) = spawn_button(commands, parent, label);
    commands.entity(b).insert(action);
    text
}

fn spawn_menu(mut commands: Commands, sound: Res<SoundOn>) {
    let panel = spawn_panel(&mut commands, DespawnOnExit(GameState::Menu));
    spawn_text(&mut commands, panel, "TRAVERSÉE", 64.0);
    spawn_text(&mut commands, panel, "un jeu Commit & Conquer", 20.0);
    spawn_gap(&mut commands, panel, 4.0);
    spawn_text(&mut commands, panel, "Aidez le crapaud à traverser !", 22.0);
    spawn_text(&mut commands, panel, "Flèches ou ZQSD · Espace · ou glissez du doigt", 16.0);
    spawn_gap(&mut commands, panel, 8.0);
    button(&mut commands, panel, "Jouer", UiAction::Play);
    let label = button(&mut commands, panel, sound.label(), UiAction::ToggleSound);
    commands.entity(label).insert(SoundLabel);
    #[cfg(not(any(target_os = "android", target_arch = "wasm32")))]
    button(&mut commands, panel, "Quitter", UiAction::Quit);
}

fn spawn_game_over(mut commands: Commands, score: Res<Score>, death: Res<Death>) {
    let panel = spawn_panel(&mut commands, DespawnOnExit(GameState::GameOver));
    let title = death.cause.map(|c| c.message()).unwrap_or("Perdu !");
    spawn_text(&mut commands, panel, title, 48.0);
    spawn_text(&mut commands, panel, format!("Score : {}    Record : {}", score.current, score.best), 24.0);
    spawn_gap(&mut commands, panel, 6.0);
    button(&mut commands, panel, "Rejouer", UiAction::Play);
    button(&mut commands, panel, "Menu", UiAction::Menu);
}

fn setup_hud(mut commands: Commands) {
    let (hud, text) = spawn_hud(&mut commands);
    commands.entity(hud).insert((Hud, Visibility::Hidden));
    commands.entity(text).insert(HudText);
}

fn update_hud(
    score: Res<Score>,
    state: Res<State<GameState>>,
    mut hud: Query<&mut Visibility, With<Hud>>,
    mut text: Query<&mut Text, With<HudText>>,
) {
    let show = matches!(state.get(), GameState::Playing | GameState::Dying);
    for mut v in &mut hud {
        *v = if show { Visibility::Inherited } else { Visibility::Hidden };
    }
    if score.is_changed() {
        for mut t in &mut text {
            t.0 = format!("Score : {}    Record : {}    M : son", score.current, score.best);
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
    mut next: ResMut<NextState<GameState>>,
) {
    let confirm = keys.just_pressed(KeyCode::Enter) || keys.just_pressed(KeyCode::Space);
    let back = keys.just_pressed(KeyCode::Escape);
    match state.get() {
        GameState::Menu if confirm => next.set(GameState::Playing),
        GameState::Playing if back => next.set(GameState::GameOver),
        GameState::GameOver if confirm => next.set(GameState::Playing),
        GameState::GameOver if back => next.set(GameState::Menu),
        _ => {}
    }
}

fn on_click(
    click: On<PointerClick>,
    actions: Query<&UiAction>,
    mut next: ResMut<NextState<GameState>>,
    mut sound: ResMut<SoundOn>,
    mut sfx: MessageWriter<Sfx>,
    mut exit: MessageWriter<AppExit>,
) {
    let Ok(action) = actions.get(click.entity) else {
        return;
    };
    sfx.write(Sfx::Click);
    match *action {
        UiAction::Play => next.set(GameState::Playing),
        UiAction::Menu => next.set(GameState::Menu),
        UiAction::ToggleSound => sound.0 = !sound.0,
        UiAction::Quit => {
            exit.write(AppExit::Success);
        }
    }
}
