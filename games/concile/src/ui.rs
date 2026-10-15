//! Écrans : accueil, Livre des Conciles, barre de jeu, victoire.
use bevy::prelude::*;
use concile_core::daily;
use game_common::audio::SoundOn;
use game_common::calendar::{Date, feast_on};
use game_common::ui::{
    ButtonColor, PANEL, ThemedPanel, UiButton, UiKitPlugin, spawn_button_sized, spawn_gap, spawn_panel, spawn_panel_left,
    spawn_text,
};
use game_common::universe::Universe;

use crate::GameState;
use crate::audio::UiClick;
use crate::play::{Book, Kind, Play, PlayEvent, Progress, start};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(UiKitPlugin { universe: Universe::Eureka })
            .add_systems(OnEnter(GameState::Menu), spawn_menu)
            .add_systems(OnEnter(GameState::Book), spawn_book)
            .add_systems(OnEnter(GameState::Playing), spawn_hud)
            .add_systems(OnEnter(GameState::Won), spawn_won)
            .add_systems(Update, (update_hud, update_labels, keyboard))
            .add_observer(on_click);
    }
}

#[derive(Component, Clone, Copy)]
enum UiAction {
    Continue,
    Book,
    Pick(usize),
    Daily,
    ToggleNovice,
    ToggleFeasts,
    ToggleSound,
    Undo,
    Clear,
    Hint,
    Next,
    Menu,
    #[cfg_attr(any(target_os = "android", target_arch = "wasm32"), allow(dead_code))]
    Quit,
}

#[derive(Component)]
struct InfoText;
#[derive(Component)]
struct HintText;
#[derive(Component)]
struct HintBox;
/// Libellé d'un bouton de réglage, mis à jour quand on clique.
#[derive(Component)]
enum Label {
    Novice,
    Feasts,
    Sound,
}

fn button(c: &mut Commands, parent: Entity, label: impl Into<String>, action: UiAction) -> Entity {
    let (b, text) = spawn_button_sized(c, parent, label, 340.0, 46.0, 21.0);
    c.entity(b).insert(action);
    text
}

fn mode_text(novice: bool) -> &'static str {
    if novice { "Mode : Novice (aidé)" } else { "Mode : Normal" }
}

fn feasts_text(on: bool) -> &'static str {
    if on { "Fêtes : affichées" } else { "Fêtes : masquées" }
}

fn spawn_menu(mut c: Commands, progress: Res<Progress>, sound: Res<SoundOn>, book: Res<Book>) {
    let panel = spawn_panel_left(&mut c, DespawnOnExit(GameState::Menu));
    // Panneau plus serré : il y a beaucoup de choix.
    c.entity(panel).insert(Node {
        flex_direction: FlexDirection::Column,
        align_items: AlignItems::Center,
        row_gap: Val::Px(8.0),
        padding: UiRect::axes(Val::Px(32.0), Val::Px(20.0)),
        border: UiRect::all(Val::Px(2.0)),
        border_radius: BorderRadius::all(Val::Px(24.0)),
        ..default()
    });
    spawn_text(&mut c, panel, "Le Concile\ndes Évêques", 44.0);
    spawn_text(&mut c, panel, "un jeu Eurêka · Commit & Conquer", 16.0);
    let today = Date::today();
    if progress.feasts
        && let Some(f) = feast_on(today)
    {
        let t = spawn_text(&mut c, panel, f.greeting(), 19.0);
        c.entity(t).insert(TextColor(Universe::Eureka.style().accent));
    }
    spawn_gap(&mut c, panel, 4.0);
    let done = progress.solved.iter().filter(|&&s| s).count();
    let next = progress.next_unsolved().map(|i| i + 1).unwrap_or(book.0.len());
    button(&mut c, panel, format!("Continuer : n° {next}"), UiAction::Continue);
    button(&mut c, panel, format!("Livre des Conciles · {done}/{}", book.0.len()), UiAction::Book);
    let daily = if progress.daily_solved_today() { "Défi du jour : réussi !".to_string() } else { "Défi du jour".to_string() };
    button(&mut c, panel, daily, UiAction::Daily);
    let t = button(&mut c, panel, mode_text(progress.novice), UiAction::ToggleNovice);
    c.entity(t).insert(Label::Novice);
    let t = button(&mut c, panel, feasts_text(progress.feasts), UiAction::ToggleFeasts);
    c.entity(t).insert(Label::Feasts);
    let t = button(&mut c, panel, sound.label(), UiAction::ToggleSound);
    c.entity(t).insert(Label::Sound);
    #[cfg(not(any(target_os = "android", target_arch = "wasm32")))]
    button(&mut c, panel, "Quitter", UiAction::Quit);
    let streak = if progress.streak > 1 { format!(" · série : {} jours", progress.streak) } else { String::new() };
    spawn_text(&mut c, panel, format!("{}{streak}", capitalize(&today.french())), 15.0);
    spawn_text(&mut c, panel, "Un évêque par diocèse, rangée et colonne,\njamais deux côte à côte (même en diagonale).", 14.0);
}

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    c.next().map(|f| f.to_uppercase().collect::<String>() + c.as_str()).unwrap_or_default()
}

/// Le Livre : une rangée de grilles par taille.
fn spawn_book(mut c: Commands, book: Res<Book>, progress: Res<Progress>) {
    let panel = spawn_panel(&mut c, DespawnOnExit(GameState::Book));
    spawn_text(&mut c, panel, "Le Livre des Conciles", 38.0);
    let mut i = 0;
    while i < book.0.len() {
        let n = book.0[i].n;
        let row = c.spawn((Node { column_gap: Val::Px(6.0), align_items: AlignItems::Center, ..default() }, ChildOf(panel))).id();
        let label = c.spawn((Node { width: Val::Px(84.0), ..default() }, ChildOf(row))).id();
        spawn_text(&mut c, label, format!("{n} × {n}"), 18.0);
        while i < book.0.len() && book.0[i].n == n {
            let text = format!("{}", i + 1);
            let (b, t) = spawn_button_sized(&mut c, row, text, 42.0, 36.0, 15.0);
            c.entity(b).insert(UiAction::Pick(i));
            if progress.solved[i] {
                // Grille réussie : chiffre vert sur fond vert pâle.
                c.entity(t).insert(TextColor(Color::srgb(0.15, 0.45, 0.20)));
                c.entity(b).insert(ButtonColor(Color::srgb(0.80, 0.93, 0.78)));
            }
            i += 1;
        }
    }
    spawn_gap(&mut c, panel, 4.0);
    button(&mut c, panel, "Retour", UiAction::Menu);
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
                align_items: AlignItems::FlexStart,
                ..default()
            },
            DespawnOnExit(GameState::Playing),
        ))
        .id();
    let info = c
        .spawn((
            Node { padding: UiRect::axes(Val::Px(16.0), Val::Px(8.0)), border: UiRect::all(Val::Px(2.0)), border_radius: BorderRadius::all(Val::Px(14.0)), ..default() },
            BackgroundColor(PANEL),
            ThemedPanel,
            ChildOf(bar),
        ))
        .id();
    let t = spawn_text(&mut c, info, "", 20.0);
    c.entity(t).insert((InfoText, TextLayout::justify(Justify::Left)));
    let buttons = c.spawn((Node { column_gap: Val::Px(8.0), ..default() }, ChildOf(bar))).id();
    for (label, action) in [("Annuler", UiAction::Undo), ("Effacer", UiAction::Clear), ("Indice", UiAction::Hint), ("Menu", UiAction::Menu)] {
        let (b, _) = spawn_button_sized(&mut c, buttons, label, 118.0, 42.0, 18.0);
        c.entity(b).insert(action);
    }
    // Explication des indices, en bas de l'écran.
    let foot = c
        .spawn((
            Node { position_type: PositionType::Absolute, bottom: Val::Px(18.0), width: Val::Percent(100.0), justify_content: JustifyContent::Center, ..default() },
            Pickable::IGNORE,
            DespawnOnExit(GameState::Playing),
        ))
        .id();
    let card = c
        .spawn((
            Node { max_width: Val::Px(660.0), padding: UiRect::axes(Val::Px(18.0), Val::Px(10.0)), border: UiRect::all(Val::Px(2.0)), border_radius: BorderRadius::all(Val::Px(14.0)), ..default() },
            BackgroundColor(PANEL),
            ThemedPanel,
            Visibility::Hidden,
            HintBox,
            ChildOf(foot),
        ))
        .id();
    let t = spawn_text(&mut c, card, "", 18.0);
    c.entity(t).insert(HintText);
}

fn format_time(secs: f32) -> String {
    let s = secs as u32;
    format!("{}:{:02}", s / 60, s % 60)
}

fn update_hud(
    play: Res<Play>,
    progress: Res<Progress>,
    mut info: Query<&mut Text, (With<InfoText>, Without<HintText>)>,
    mut hint: Query<&mut Text, (With<HintText>, Without<InfoText>)>,
    mut hint_box: Query<&mut Visibility, With<HintBox>>,
) {
    let n = play.n();
    let title = match play.kind {
        Kind::Book(i) => format!("Concile n° {} · {n} × {n}", i + 1),
        Kind::Daily(d) => format!("Défi du {} · {n} × {n}", d.french()),
    };
    let mode = if progress.novice { "Novice" } else { "Normal" };
    for mut t in &mut info {
        let s = format!("{title} · {mode}\nSablier : {}   Évêques : {}/{n}", format_time(play.time), play.bishops().len());
        if t.0 != s {
            t.0 = s;
        }
    }
    let text = play.hint.as_ref().map(|(_, s, _)| s.clone());
    for mut t in &mut hint {
        let s = text.clone().unwrap_or_default();
        if t.0 != s {
            t.0 = s;
        }
    }
    for mut v in &mut hint_box {
        *v = if text.is_some() { Visibility::Inherited } else { Visibility::Hidden };
    }
}

fn spawn_won(mut c: Commands, play: Res<Play>, progress: Res<Progress>, book: Res<Book>) {
    let panel = spawn_panel_left(&mut c, DespawnOnExit(GameState::Won));
    spawn_text(&mut c, panel, "Le concile\npeut commencer !", 44.0);
    let line = match play.kind {
        Kind::Book(i) => {
            let best = progress.best[i].map(|b| format_time(b as f32)).unwrap_or_default();
            format!("Concile n° {} réussi en {}\nMeilleur temps : {best}", i + 1, format_time(play.time))
        }
        Kind::Daily(_) => {
            let streak = if progress.streak > 1 { format!("\nSérie : {} jours de suite !", progress.streak) } else { String::new() };
            format!("Défi du jour réussi en {}{streak}", format_time(play.time))
        }
    };
    spawn_text(&mut c, panel, line, 20.0);
    spawn_gap(&mut c, panel, 4.0);
    if matches!(play.kind, Kind::Book(i) if i + 1 < book.0.len()) {
        button(&mut c, panel, "Concile suivant", UiAction::Next);
    }
    button(&mut c, panel, "Menu", UiAction::Menu);
}

fn update_labels(progress: Res<Progress>, sound: Res<SoundOn>, mut q: Query<(&Label, &mut Text)>) {
    if !progress.is_changed() && !sound.is_changed() {
        return;
    }
    for (label, mut t) in &mut q {
        t.0 = match label {
            Label::Novice => mode_text(progress.novice),
            Label::Feasts => feasts_text(progress.feasts),
            Label::Sound => sound.label(),
        }
        .into();
    }
}

fn keyboard(
    keys: Res<ButtonInput<KeyCode>>,
    state: Res<State<GameState>>,
    mut play: ResMut<Play>,
    mut next: ResMut<NextState<GameState>>,
    mut events: MessageWriter<PlayEvent>,
) {
    match state.get() {
        GameState::Playing => {
            if keys.just_pressed(KeyCode::Escape) {
                next.set(GameState::Menu);
            }
            if keys.just_pressed(KeyCode::KeyU) || keys.just_pressed(KeyCode::Backspace) {
                play.undo();
            }
            if keys.just_pressed(KeyCode::KeyH) {
                play.compute_hint();
                events.write(PlayEvent::Hint);
            }
        }
        GameState::Book if keys.just_pressed(KeyCode::Escape) => next.set(GameState::Menu),
        _ => {}
    }
}

/// Lance la grille `i` du Livre.
fn open_book(c: &mut Commands, book: &Book, i: usize, next: &mut NextState<GameState>) {
    start(c, book.0[i].clone(), Kind::Book(i));
    next.set(GameState::Playing);
}

#[allow(clippy::too_many_arguments)]
fn on_click(
    click: On<PointerClick>,
    actions: Query<&UiAction, With<UiButton>>,
    mut c: Commands,
    book: Res<Book>,
    mut play: ResMut<Play>,
    mut progress: ResMut<Progress>,
    mut next: ResMut<NextState<GameState>>,
    mut sound: ResMut<SoundOn>,
    mut clicks: MessageWriter<UiClick>,
    mut events: MessageWriter<PlayEvent>,
    mut exit: MessageWriter<AppExit>,
) {
    let Ok(action) = actions.get(click.entity) else { return };
    clicks.write(UiClick);
    match *action {
        UiAction::Continue => open_book(&mut c, &book, progress.next_unsolved().unwrap_or(0), &mut next),
        UiAction::Book => next.set(GameState::Book),
        UiAction::Pick(i) => open_book(&mut c, &book, i, &mut next),
        UiAction::Daily => {
            let d = Date::today();
            start(&mut c, daily(d.year, d.month, d.day, d.weekday()), Kind::Daily(d));
            next.set(GameState::Playing);
        }
        UiAction::ToggleNovice => {
            progress.novice = !progress.novice;
            progress.store();
        }
        UiAction::ToggleFeasts => {
            progress.feasts = !progress.feasts;
            progress.store();
        }
        UiAction::ToggleSound => sound.0 = !sound.0,
        UiAction::Undo => {
            play.undo();
        }
        UiAction::Clear => play.clear(),
        UiAction::Hint => {
            play.compute_hint();
            events.write(PlayEvent::Hint);
        }
        UiAction::Next => {
            if let Kind::Book(i) = play.kind {
                open_book(&mut c, &book, (i + 1).min(book.0.len() - 1), &mut next);
            }
        }
        UiAction::Menu => next.set(GameState::Menu),
        UiAction::Quit => {
            exit.write(AppExit::Success);
        }
    }
}
