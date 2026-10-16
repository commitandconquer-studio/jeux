//! Écrans : accueil, options, fin de partie, et affichage du score.
use bevy::prelude::*;
use game_common::Score;
use game_common::ui::{GameHelp, spawn_help_button, spawn_studio_logo};
use game_common::ui::{UiKitPlugin, spawn_button, spawn_gap, spawn_hud, spawn_panel, spawn_text};

use game_common::audio::SoundOn;

use crate::GameState;
use crate::audio::Sfx;
use crate::settings::{OptionKind, Settings};

/// Aide du jeu (bouton « Aide » du menu).
const HELP: &str = "Dirigez le serpent et mangez pour grandir.\n\
Ne vous mordez pas la queue !\n\
\n\
Flèches ou ZQSD : diriger\n\
Entrée ou Espace : valider\n\
M : couper le son · Échap : menu\n\
\n\
Options : vitesse, taille du plateau,\n\
bords murs ou traversables.";

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(UiKitPlugin { universe: game_common::universe::Universe::Croa })
            .add_systems(Startup, setup_hud)
            .insert_resource(GameHelp(HELP))
            .add_systems(OnEnter(GameState::Menu), spawn_menu)
            .add_systems(OnEnter(GameState::Options), spawn_options)
            .add_systems(OnEnter(GameState::GameOver), spawn_game_over)
            .add_systems(Update, (update_hud, refresh_option_texts, update_sound_label, keyboard_shortcuts))
            .add_observer(on_click);
    }
}

/// Ce que fait un bouton quand on clique dessus.
#[derive(Component, Clone, Copy)]
enum UiAction {
    Play,
    Options,
    Back,
    Menu,
    Cycle(OptionKind),
    ToggleSound,
    #[cfg_attr(any(target_os = "android", target_arch = "wasm32"), allow(dead_code))]
    Quit,
}

/// Texte d'un bouton d'option, mis à jour quand la valeur change.
#[derive(Component)]
struct OptionText(OptionKind);

#[derive(Component)]
struct Hud;

#[derive(Component)]
struct HudText;

/// Texte du bouton « Son ».
#[derive(Component)]
struct SoundLabel;

fn button(commands: &mut Commands, parent: Entity, label: impl Into<String>, action: UiAction) -> Entity {
    let (b, text) = spawn_button(commands, parent, label);
    commands.entity(b).insert(action);
    text
}

fn spawn_menu(mut commands: Commands, settings: Res<Settings>, sound: Res<SoundOn>) {
    let panel = spawn_panel(&mut commands, DespawnOnExit(GameState::Menu));
    spawn_text(&mut commands, panel, "SNAKE", 72.0);
    spawn_gap(&mut commands, panel, 10.0);
    button(&mut commands, panel, "Jouer", UiAction::Play);
    let t = button(
        &mut commands,
        panel,
        OptionKind::Universe.text(&settings),
        UiAction::Cycle(OptionKind::Universe),
    );
    commands.entity(t).insert(OptionText(OptionKind::Universe));
    let t = button(&mut commands, panel, sound.label(), UiAction::ToggleSound);
    commands.entity(t).insert(SoundLabel);
    button(&mut commands, panel, "Options", UiAction::Options);
    spawn_help_button(&mut commands, panel);
    #[cfg(not(any(target_os = "android", target_arch = "wasm32")))]
    button(&mut commands, panel, "Quitter", UiAction::Quit);
    spawn_studio_logo(&mut commands, panel);
}

fn spawn_options(mut commands: Commands, settings: Res<Settings>) {
    let panel = spawn_panel(&mut commands, DespawnOnExit(GameState::Options));
    spawn_text(&mut commands, panel, "Options", 48.0);
    for kind in OptionKind::ALL {
        let t = button(&mut commands, panel, kind.text(&settings), UiAction::Cycle(kind));
        commands.entity(t).insert(OptionText(kind));
    }
    spawn_gap(&mut commands, panel, 6.0);
    button(&mut commands, panel, "Retour", UiAction::Back);
}

fn spawn_game_over(mut commands: Commands, score: Res<Score>) {
    let panel = spawn_panel(&mut commands, DespawnOnExit(GameState::GameOver));
    spawn_text(&mut commands, panel, "Perdu !", 56.0);
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
    let playing = *state.get() == GameState::Playing;
    for mut v in &mut hud {
        *v = if playing { Visibility::Inherited } else { Visibility::Hidden };
    }
    if score.is_changed() {
        for mut t in &mut text {
            t.0 = format!("Score : {}    Record : {}    Échap : menu    M : son", score.current, score.best);
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

fn refresh_option_texts(settings: Res<Settings>, mut q: Query<(&mut Text, &OptionText)>) {
    if settings.is_changed() {
        for (mut text, opt) in &mut q {
            text.0 = opt.0.text(&settings);
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
        GameState::Options if back => next.set(GameState::Menu),
        GameState::Playing if back => next.set(GameState::Menu),
        GameState::GameOver if confirm => next.set(GameState::Playing),
        GameState::GameOver if back => next.set(GameState::Menu),
        _ => {}
    }
}

fn on_click(
    click: On<PointerClick>,
    actions: Query<&UiAction>,
    mut settings: ResMut<Settings>,
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
        UiAction::ToggleSound => sound.0 = !sound.0,
        UiAction::Play => next.set(GameState::Playing),
        UiAction::Options => next.set(GameState::Options),
        UiAction::Back | UiAction::Menu => next.set(GameState::Menu),
        UiAction::Cycle(kind) => kind.cycle(&mut settings),
        UiAction::Quit => {
            exit.write(AppExit::Success);
        }
    }
}
