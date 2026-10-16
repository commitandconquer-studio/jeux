//! Écrans : accueil, barre de jeu, faits « Eurêka ! », victoire, fin de partie.
use bevy::prelude::*;
use game_common::Rng;
use game_common::audio::SoundOn;
use game_common::ui::{GameHelp, spawn_help_button, spawn_studio_logo};
use game_common::ui::{
    PANEL, ThemedPanel, UiKitPlugin, spawn_button, spawn_button_sized, spawn_gap, spawn_panel_left,
    spawn_text,
};
use game_common::universe::Universe;

use crate::GameState;
use crate::audio::UiClick;
use crate::play::{Game, PlayEvent};

/// Aide du jeu (bouton « Aide » du menu).
const HELP: &str = "Faites glisser les tuiles : deux nombres égaux\n\
fusionnent et doublent. Atteindrez-vous 2048 ?\n\
\n\
Flèches, ZQSD ou glisser du doigt : faire glisser\n\
U ou Retour arrière : annuler le dernier coup\n\
M : son · Échap : menu";

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(UiKitPlugin { universe: Universe::Eureka })
            .insert_resource(GameHelp(HELP))
            .add_systems(OnEnter(GameState::Menu), spawn_menu)
            .add_systems(OnEnter(GameState::Playing), spawn_hud)
            .add_systems(OnEnter(GameState::Won), spawn_won)
            .add_systems(OnEnter(GameState::Over), spawn_over)
            .add_systems(Update, (update_hud, update_sound_label, show_facts, fade_facts, keyboard))
            .add_observer(on_click);
    }
}

/// Petits faits scientifiques sur les puissances de 2.
fn fact(value: u32) -> Option<&'static str> {
    Some(match value {
        8 => "8 = 2 × 2 × 2 : c'est 2 puissance 3, un cube de 2 de côté !",
        16 => "16 = 2 puissance 4. Les informaticiens comptent souvent en base 16 : l'hexadécimal.",
        32 => "32 = 2 puissance 5. C'est le nombre de dents d'un adulte !",
        64 => "64 = 2 puissance 6 : le nombre de cases d'un échiquier.",
        128 => "128 = 2 puissance 7. Le code ASCII des premiers ordinateurs comptait 128 caractères.",
        256 => "256 = 2 puissance 8 : un octet peut prendre 256 valeurs différentes.",
        512 => "512 = 2 puissance 9. Chaque fusion double la tuile : la croissance exponentielle !",
        1024 => "1024 = 2 puissance 10 : c'est le « kibi » de l'informatique (1 Kio = 1024 octets).",
        2048 => "2048 = 2 puissance 11. Eurêka !",
        4096 => "4096 = 2 puissance 12. Archimède serait fier de vous !",
        _ => return None,
    })
}

#[derive(Component, Clone, Copy)]
enum UiAction {
    Play,
    NewGame,
    Undo,
    Continue,
    Menu,
    ToggleSound,
    #[cfg_attr(any(target_os = "android", target_arch = "wasm32"), allow(dead_code))]
    Quit,
}

#[derive(Component)]
struct ScoreText;
#[derive(Component)]
struct SoundLabel;
#[derive(Component)]
struct FactCard(f32);

fn button(c: &mut Commands, parent: Entity, label: impl Into<String>, action: UiAction) -> Entity {
    let (b, text) = spawn_button(c, parent, label);
    c.entity(b).insert(action);
    text
}

fn spawn_menu(mut c: Commands, game: Res<Game>, sound: Res<SoundOn>) {
    let panel = spawn_panel_left(&mut c, DespawnOnExit(GameState::Menu));
    spawn_text(&mut c, panel, "2048", 96.0);
    spawn_text(&mut c, panel, "les puissances de 2", 26.0);
    spawn_text(&mut c, panel, "un jeu Eurêka", 17.0);
    spawn_gap(&mut c, panel, 6.0);
    let in_progress = game.score > 0;
    button(&mut c, panel, if in_progress { "Continuer" } else { "Jouer" }, UiAction::Play);
    if in_progress {
        button(&mut c, panel, "Nouvelle partie", UiAction::NewGame);
    }
    let t = button(&mut c, panel, sound.label(), UiAction::ToggleSound);
    c.entity(t).insert(SoundLabel);
    spawn_help_button(&mut c, panel);
    #[cfg(not(any(target_os = "android", target_arch = "wasm32")))]
    button(&mut c, panel, "Quitter", UiAction::Quit);
    spawn_studio_logo(&mut c, panel);
}

fn spawn_hud(mut c: Commands) {
    let bar = c
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
    let info = c
        .spawn((
            Node {
                padding: UiRect::axes(Val::Px(18.0), Val::Px(8.0)),
                border: UiRect::all(Val::Px(2.0)),
                border_radius: BorderRadius::all(Val::Px(16.0)),
                ..default()
            },
            BackgroundColor(PANEL),
            ThemedPanel,
            ChildOf(bar),
        ))
        .id();
    let t = spawn_text(&mut c, info, "", 24.0);
    c.entity(t).insert(ScoreText);
    let buttons = c.spawn((Node { column_gap: Val::Px(8.0), ..default() }, ChildOf(bar))).id();
    for (label, action) in [("Annuler", UiAction::Undo), ("Nouvelle partie", UiAction::NewGame), ("Menu", UiAction::Menu)] {
        let (b, _) = spawn_button_sized(&mut c, buttons, label, if label.len() > 8 { 190.0 } else { 130.0 }, 44.0, 19.0);
        c.entity(b).insert(action);
    }
}

fn spawn_won(mut c: Commands, game: Res<Game>) {
    let panel = spawn_panel_left(&mut c, DespawnOnExit(GameState::Won));
    spawn_text(&mut c, panel, "Eurêka ! 2048 !", 48.0);
    spawn_text(&mut c, panel, format!("Score : {}    Record : {}", game.score, game.best), 22.0);
    button(&mut c, panel, "Continuer plus loin", UiAction::Continue);
    button(&mut c, panel, "Nouvelle partie", UiAction::NewGame);
}

fn spawn_over(mut c: Commands, game: Res<Game>, cards: Query<Entity, With<FactCard>>) {
    for e in &cards {
        c.entity(e).despawn();
    }
    let panel = spawn_panel_left(&mut c, DespawnOnExit(GameState::Over));
    let record = game.score > 0 && game.score >= game.best;
    spawn_text(&mut c, panel, if record { "Nouveau record !" } else { "Plus aucun coup possible !" }, 40.0);
    spawn_text(&mut c, panel, format!("Score : {}    Record : {}\nPlus grande tuile : {}", game.score, game.best, game.grid.max_value()), 20.0);
    if game.can_undo() {
        button(&mut c, panel, "Annuler le dernier coup", UiAction::Undo);
    }
    button(&mut c, panel, "Nouvelle partie", UiAction::NewGame);
    button(&mut c, panel, "Menu", UiAction::Menu);
}

fn update_hud(game: Res<Game>, mut q: Query<&mut Text, With<ScoreText>>) {
    for mut t in &mut q {
        let s = format!("Score : {}   ·   Record : {}", game.score, game.best);
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

/// Une fiche « Eurêka ! » apparaît quand on atteint une nouvelle puissance de 2.
fn show_facts(mut c: Commands, mut events: MessageReader<PlayEvent>, old: Query<Entity, With<FactCard>>) {
    for e in events.read() {
        let PlayEvent::Fact(v) = *e else { continue };
        let Some(text) = fact(v) else { continue };
        for o in &old {
            c.entity(o).despawn();
        }
        let card = c
            .spawn((
                Node {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(22.0),
                    width: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                Pickable::IGNORE,
                FactCard(5.0),
            ))
            .id();
        let inner = c
            .spawn((
                Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    max_width: Val::Px(640.0),
                    padding: UiRect::axes(Val::Px(22.0), Val::Px(12.0)),
                    border: UiRect::all(Val::Px(2.0)),
                    border_radius: BorderRadius::all(Val::Px(16.0)),
                    ..default()
                },
                BackgroundColor(PANEL),
                ThemedPanel,
                ChildOf(card),
            ))
            .id();
        let title = spawn_text(&mut c, inner, "Eurêka !", 24.0);
        c.entity(title).insert(TextColor(Universe::Eureka.style().accent));
        spawn_text(&mut c, inner, text, 19.0);
    }
}

fn fade_facts(mut c: Commands, time: Res<Time>, mut q: Query<(Entity, &mut FactCard)>) {
    for (e, mut card) in &mut q {
        card.0 -= time.delta_secs();
        if card.0 <= 0.0 {
            c.entity(e).despawn();
        }
    }
}

fn keyboard(keys: Res<ButtonInput<KeyCode>>, state: Res<State<GameState>>, mut next: ResMut<NextState<GameState>>) {
    let confirm = keys.just_pressed(KeyCode::Enter) || keys.just_pressed(KeyCode::Space);
    match state.get() {
        GameState::Menu if confirm => next.set(GameState::Playing),
        GameState::Playing if keys.just_pressed(KeyCode::Escape) => next.set(GameState::Menu),
        GameState::Won if confirm => next.set(GameState::Playing),
        _ => {}
    }
}

#[allow(clippy::too_many_arguments)]
fn on_click(
    click: On<PointerClick>,
    actions: Query<&UiAction>,
    mut game: ResMut<Game>,
    mut rng: ResMut<Rng>,
    mut next: ResMut<NextState<GameState>>,
    mut sound: ResMut<SoundOn>,
    mut clicks: MessageWriter<UiClick>,
    mut events: MessageWriter<PlayEvent>,
    mut exit: MessageWriter<AppExit>,
) {
    let Ok(action) = actions.get(click.entity) else { return };
    clicks.write(UiClick);
    match *action {
        UiAction::Play | UiAction::Continue => next.set(GameState::Playing),
        UiAction::NewGame => {
            game.new_game(&mut rng);
            next.set(GameState::Playing);
        }
        UiAction::Undo => {
            if game.undo() {
                events.write(PlayEvent::Undo);
                next.set(GameState::Playing);
            }
        }
        UiAction::Menu => next.set(GameState::Menu),
        UiAction::ToggleSound => sound.0 = !sound.0,
        UiAction::Quit => {
            exit.write(AppExit::Success);
        }
    }
}
