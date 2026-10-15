//! Écrans : accueil, fin de partie, score, et « Parfait ! ».
use bevy::prelude::*;
use game_common::audio::SoundOn;
use game_common::ui::{UiKitPlugin, spawn_button, spawn_gap, spawn_hud, spawn_panel_bottom, spawn_panel_left, spawn_text};
use game_common::{Score, palette};

use crate::GameState;
use crate::audio::UiClick;
use crate::tower::TowerEvent;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(UiKitPlugin { universe: game_common::universe::Universe::Croa })
            .add_systems(Startup, (setup_hud, setup_praise))
            .add_systems(OnEnter(GameState::Menu), spawn_menu)
            .add_systems(OnEnter(GameState::GameOver), spawn_game_over)
            .add_systems(Update, (update_hud, update_sound_label, praise, keyboard_shortcuts))
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

#[derive(Component)]
struct SoundLabel;

/// Texte « Parfait ! » qui s'efface tout seul.
#[derive(Component)]
struct Praise {
    timer: f32,
}

fn button(commands: &mut Commands, parent: Entity, label: &str, action: UiAction) -> Entity {
    let (b, text) = spawn_button(commands, parent, label);
    commands.entity(b).insert(action);
    text
}

fn spawn_menu(mut commands: Commands, sound: Res<SoundOn>) {
    let panel = spawn_panel_left(&mut commands, DespawnOnExit(GameState::Menu));
    spawn_text(&mut commands, panel, "TOUR DE BLOCS", 60.0);
    spawn_text(&mut commands, panel, "un jeu Commit & Conquer", 20.0);
    spawn_gap(&mut commands, panel, 4.0);
    spawn_text(&mut commands, panel, "Empilez les blocs le plus haut possible !", 22.0);
    spawn_text(&mut commands, panel, "Espace, clic ou toucher l'écran pour poser", 16.0);
    spawn_gap(&mut commands, panel, 8.0);
    button(&mut commands, panel, "Jouer", UiAction::Play);
    let t = button(&mut commands, panel, sound.label(), UiAction::ToggleSound);
    commands.entity(t).insert(SoundLabel);
    #[cfg(not(any(target_os = "android", target_arch = "wasm32")))]
    button(&mut commands, panel, "Quitter", UiAction::Quit);
}

fn spawn_game_over(mut commands: Commands, score: Res<Score>) {
    let panel = spawn_panel_bottom(&mut commands, DespawnOnExit(GameState::GameOver));
    let title = if score.current > 0 && score.current == score.best { "Nouveau record !" } else { "La tour s'arrête là !" };
    spawn_text(&mut commands, panel, title, 40.0);
    spawn_text(&mut commands, panel, format!("Hauteur : {}    Record : {}", score.current, score.best), 24.0);
    button(&mut commands, panel, "Rejouer", UiAction::Play);
    button(&mut commands, panel, "Menu", UiAction::Menu);
}

fn setup_hud(mut commands: Commands) {
    let (hud, text) = spawn_hud(&mut commands);
    commands.entity(hud).insert((Hud, Visibility::Hidden));
    commands.entity(text).insert(HudText);
}

fn setup_praise(mut commands: Commands) {
    let root = commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                top: Val::Percent(18.0),
                justify_content: JustifyContent::Center,
                ..default()
            },
            Pickable::IGNORE,
        ))
        .id();
    let text = spawn_text(&mut commands, root, "", 44.0);
    commands.entity(text).insert(Praise { timer: 0.0 });
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
            t.0 = format!("Hauteur : {}    Record : {}    M : son", score.current, score.best);
        }
    }
}

/// Affiche « Parfait ! » (avec la série en cours) puis l'efface en fondu.
fn praise(time: Res<Time>, mut events: MessageReader<TowerEvent>, mut q: Query<(&mut Text, &mut TextColor, &mut Praise)>) {
    for (mut text, mut color, mut p) in &mut q {
        for event in events.read() {
            if let TowerEvent::Placed { perfect: true, combo } = *event {
                text.0 = if combo > 1 { format!("Parfait ×{combo} !") } else { "Parfait !".into() };
                p.timer = 1.0;
            }
        }
        p.timer = (p.timer - time.delta_secs()).max(0.0);
        color.0 = palette::TEXT_DARK.with_alpha(p.timer.min(0.6) / 0.6);
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
    let confirm = keys.just_pressed(KeyCode::Enter);
    let back = keys.just_pressed(KeyCode::Escape);
    match state.get() {
        GameState::Menu if confirm => next.set(GameState::Playing),
        GameState::Playing if back => next.set(GameState::Menu),
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
    mut clicks: MessageWriter<UiClick>,
    mut exit: MessageWriter<AppExit>,
) {
    let Ok(action) = actions.get(click.entity) else {
        return;
    };
    clicks.write(UiClick);
    match *action {
        UiAction::Play => next.set(GameState::Playing),
        UiAction::Menu => next.set(GameState::Menu),
        UiAction::ToggleSound => sound.0 = !sound.0,
        UiAction::Quit => {
            exit.write(AppExit::Success);
        }
    }
}
