//! Écrans : accueil, fin de partie, et affichage du score.
use bevy::prelude::*;
use game_common::Score;
use game_common::tx;
use game_common::ui::{GameHelp, ThemedText, pied_de_menu, spawn_button, spawn_button_sized, spawn_gap, spawn_hud, spawn_panel, spawn_panel_left, spawn_text};

use crate::screens::{CountUp, Pop, Pulse, Wave, spawn_bar, spawn_chip};

use crate::GameState;
use crate::audio::Sfx;
use crate::frog::Death;
use crate::progress::Progress;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_hud)
            .insert_resource(GameHelp(tx!(
                "Aidez le crapaud à aller le plus loin possible !\n\
\n\
Herbe : arbres et rochers bloquent le passage.\n\
Routes : attention aux voitures et aux camions.\n\
Rivières : sautez sur les rondins et les nénuphars.\n\
Voies ferrées : le feu clignote avant le train.\n\
La caméra avance toute seule : ne traînez pas !\n\
\n\
Flèches ou ZQSD, Espace ou clic : avancer\n\
Tactile : glisser du doigt\n\
M : son · Échap : menu",
                "Help the toad get as far as possible!\n\
\n\
Grass: trees and rocks block the way.\n\
Roads: watch out for cars and trucks.\n\
Rivers: jump onto the logs and water lilies.\n\
Railways: the light flashes before the train comes.\n\
The camera moves forward on its own: don't dawdle!\n\
\n\
Arrows or WASD, Space or click: move forward\n\
Touch: swipe with your finger\n\
M: sound · Esc: menu",
                "Hilf der Kröte, so weit wie möglich zu kommen!\n\
\n\
Wiese: Bäume und Felsen versperren den Weg.\n\
Straßen: Vorsicht vor Autos und Lastwagen.\n\
Flüsse: Spring auf die Baumstämme und Seerosen.\n\
Bahngleise: Das Licht blinkt, bevor der Zug kommt.\n\
Die Kamera fährt von selbst weiter: Trödle nicht!\n\
\n\
Pfeiltasten oder WASD, Leertaste oder Klick: vorwärts\n\
Touchscreen: mit dem Finger wischen\n\
M: Ton · Esc: Menü",
                "Aiuta il rospo ad andare il più lontano possibile!\n\
\n\
Prato: alberi e rocce bloccano il passaggio.\n\
Strade: attenzione alle auto e ai camion.\n\
Fiumi: salta sui tronchi e sulle ninfee.\n\
Ferrovie: la luce lampeggia prima del treno.\n\
La telecamera avanza da sola: non perdere tempo!\n\
\n\
Frecce o WASD, Spazio o clic: avanti\n\
Touch: scorri con il dito\n\
M: audio · Esc: menu",
                "¡Ayuda al sapo a llegar lo más lejos posible!\n\
\n\
Hierba: los árboles y las rocas bloquean el paso.\n\
Carreteras: cuidado con los coches y los camiones.\n\
Ríos: salta sobre los troncos y los nenúfares.\n\
Vías del tren: la luz parpadea antes de que pase el tren.\n\
La cámara avanza sola: ¡no te entretengas!\n\
\n\
Flechas o WASD, Espacio o clic: avanzar\n\
Táctil: desliza el dedo\n\
M: sonido · Esc: menú"
            )))
            .add_systems(OnEnter(GameState::Menu), spawn_menu)
            .add_systems(OnEnter(GameState::GameOver), spawn_game_over)
            .add_systems(Update, (update_hud, keyboard_shortcuts))
            .add_observer(on_click);
    }
}

#[derive(Component, Clone, Copy)]
enum UiAction {
    Play,
    Garage,
    Menu,
}

#[derive(Component)]
struct Hud;

#[derive(Component)]
struct HudText;

fn button(commands: &mut Commands, parent: Entity, label: &str, action: UiAction) -> Entity {
    let (b, text) = spawn_button(commands, parent, label);
    commands.entity(b).insert(action);
    text
}

/// Couleurs des lettres du titre.
const TITLE_COLORS: [Color; 6] = [
    Color::srgb(0.30, 0.68, 0.30),
    Color::srgb(0.95, 0.55, 0.15),
    Color::srgb(0.25, 0.50, 0.90),
    Color::srgb(0.90, 0.30, 0.35),
    Color::srgb(0.60, 0.40, 0.85),
    Color::srgb(0.95, 0.75, 0.15),
];

/// Ligne « Prochain : … à N pts » suivie de sa barre de progression.
fn spawn_next_goal(commands: &mut Commands, parent: Entity, progress: &Progress, size: f32, width: f32) {
    let Some((at, item)) = progress.next() else {
        return;
    };
    let line = tx!("Prochain : {} — {} pts", "Next: {} — {} pts", "Nächstes: {} — {} Pkt.", "Prossimo: {} — {} pt", "Siguiente: {} — {} pts").avec(&[&item.name(), &at]);
    spawn_text(commands, parent, line, size);
    spawn_bar(commands, parent, progress.next_fraction(), width, item.color());
}

fn spawn_menu(mut commands: Commands, progress: Res<Progress>, score: Res<Score>) {
    let panel = spawn_panel_left(&mut commands, DespawnOnExit(GameState::Menu));
    commands.entity(panel).insert(UiTransform::from_scale(Vec2::ZERO)).insert(Pop::after(0.05));
    // Titre : une lettre par texte, pour les faire onduler
    let row = commands.spawn((Node { flex_direction: FlexDirection::Row, column_gap: px(1), ..default() }, ChildOf(panel))).id();
    for (i, ch) in tx!("TRAVERSÉE", "CROSSING", "ÜBERQUERUNG", "TRAVERSATA", "TRAVESÍA").t().chars().enumerate() {
        commands.spawn((
            Text::new(ch.to_string()),
            TextFont::from_font_size(78.0),
            TextColor(TITLE_COLORS[i % TITLE_COLORS.len()]),
            ThemedText,
            UiTransform::default(),
            Wave(i),
            ChildOf(row),
        ));
    }
    if score.best > 0 {
        spawn_text(&mut commands, panel, tx!("Record : {}", "Best: {}", "Rekord: {}", "Record: {}", "Récord: {}").avec(&[&score.best]), 24.0);
    }
    spawn_next_goal(&mut commands, panel, &progress, 18.0, 360.0);
    let total = crate::progress::UNLOCKS.len();
    let line = tx!("Déblocages : {} / {}", "Unlocked: {} / {}", "Freigeschaltet: {} / {}", "Sbloccati: {} / {}", "Desbloqueos: {} / {}").avec(&[&progress.count(), &total]);
    spawn_text(&mut commands, panel, line, 16.0);
    spawn_gap(&mut commands, panel, 6.0);
    let (play, _) = spawn_button_sized(&mut commands, panel, tx!("Jouer", "Play", "Spielen", "Gioca", "Jugar").t(), 360.0, 76.0, 36.0);
    commands.entity(play).insert((
        UiAction::Play,
        BackgroundColor(Color::srgb(1.0, 0.86, 0.3)),
        UiTransform::default(),
        Pulse { speed: 3.0, amount: 0.03 },
    ));
    button(&mut commands, panel, tx!("Garage", "Garage", "Garage", "Garage", "Garaje").t(), UiAction::Garage);
    pied_de_menu(&mut commands, panel);
}

/// « Score : 12    Record : 40 », dans la langue en cours.
fn score_line(score: &Score) -> String {
    tx!(
        "Score : {}    Record : {}",
        "Score: {}    Best: {}",
        "Punkte: {}    Rekord: {}",
        "Punti: {}    Record: {}",
        "Puntos: {}    Récord: {}"
    )
    .avec(&[&score.current, &score.best])
}

pub fn spawn_game_over(mut commands: Commands, score: Res<Score>, death: Res<Death>, progress: Res<Progress>, mut sfx: MessageWriter<Sfx>) {
    let panel = spawn_panel(&mut commands, DespawnOnExit(GameState::GameOver));
    commands.entity(panel).insert(UiTransform::from_scale(Vec2::ZERO)).insert(Pop::after(0.0));
    let title = death.cause.map(|c| c.message()).unwrap_or(tx!("Perdu !", "Game over!", "Verloren!", "Hai perso!", "¡Has perdido!").t());
    spawn_text(&mut commands, panel, title, 46.0);
    // Le score grimpe jusqu'à sa valeur
    let row = commands.spawn((Node { align_items: AlignItems::FlexEnd, column_gap: px(10), ..default() }, ChildOf(panel))).id();
    commands.spawn((
        Text::new("0"),
        TextFont::from_font_size(96.0),
        TextColor(Color::srgb(0.12, 0.14, 0.2)),
        ThemedText,
        CountUp { target: score.current, age: 0.0 },
        ChildOf(row),
    ));
    commands.spawn((Text::new("pts"), TextFont::from_font_size(30.0), TextColor(Color::srgb(0.4, 0.4, 0.45)), ThemedText, Node { margin: UiRect::bottom(px(14)), ..default() }, ChildOf(row)));
    if progress.new_record {
        let badge = commands
            .spawn((
                Node { padding: UiRect::axes(px(22), px(8)), border_radius: BorderRadius::all(px(16)), ..default() },
                BackgroundColor(Color::srgb(1.0, 0.82, 0.2)),
                UiTransform::default(),
                Pulse { speed: 6.0, amount: 0.06 },
                ChildOf(panel),
            ))
            .id();
        commands.spawn((
            Text::new(tx!("NOUVEAU RECORD !", "NEW RECORD!", "NEUER REKORD!", "NUOVO RECORD!", "¡NUEVO RÉCORD!").t()),
            TextFont::from_font_size(28.0),
            TextColor(Color::srgb(0.35, 0.2, 0.0)),
            ThemedText,
            ChildOf(badge),
        ));
    } else {
        spawn_text(&mut commands, panel, tx!("Record : {}", "Best: {}", "Rekord: {}", "Record: {}", "Récord: {}").avec(&[&score.best]), 22.0);
    }
    // Ce que cette partie vient de débloquer
    for (i, item) in progress.fresh.iter().enumerate() {
        spawn_chip(&mut commands, panel, item.kind(), item.name(), item.color(), 0.7 + i as f32 * 0.3);
    }
    spawn_next_goal(&mut commands, panel, &progress, 18.0, 400.0);
    sfx.write(if progress.new_record || !progress.fresh.is_empty() { Sfx::Win } else { Sfx::GameOver });
    spawn_gap(&mut commands, panel, 6.0);
    let (again, _) = spawn_button_sized(&mut commands, panel, tx!("Rejouer", "Play again", "Nochmal", "Rigioca", "Volver a jugar").t(), 320.0, 64.0, 30.0);
    commands.entity(again).insert((
        UiAction::Play,
        BackgroundColor(Color::srgb(1.0, 0.86, 0.3)),
        UiTransform::default(),
        Pulse { speed: 3.0, amount: 0.025 },
    ));
    button(&mut commands, panel, tx!("Menu", "Menu", "Menü", "Menu", "Menú").t(), UiAction::Menu);
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
            let son = tx!("M : son", "M: sound", "M: Ton", "M: audio", "M: sonido").t();
            t.0 = format!("{}    {}", score_line(&score), son);
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
    mut sfx: MessageWriter<Sfx>,
) {
    let Ok(action) = actions.get(click.entity) else {
        return;
    };
    sfx.write(Sfx::Click);
    match *action {
        UiAction::Play => next.set(GameState::Playing),
        UiAction::Garage => next.set(GameState::Garage),
        UiAction::Menu => next.set(GameState::Menu),
    }
}
