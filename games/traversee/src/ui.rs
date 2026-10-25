//! Écrans : accueil, fin de partie, et affichage du score.
use bevy::prelude::*;
use game_common::Score;
use game_common::tx;
use game_common::ui::{GameHelp, pied_de_menu, spawn_button, spawn_gap, spawn_hud, spawn_panel, spawn_text};

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

fn spawn_menu(mut commands: Commands, progress: Res<Progress>) {
    let panel = spawn_panel(&mut commands, DespawnOnExit(GameState::Menu));
    spawn_text(&mut commands, panel, tx!("TRAVERSÉE", "CROSSING", "ÜBERQUERUNG", "TRAVERSATA", "TRAVESÍA").t(), 64.0);
    let total = crate::progress::UNLOCKS.len() + 1;
    let line = tx!("Véhicules débloqués : {} / {}", "Vehicles unlocked: {} / {}", "Fahrzeuge freigeschaltet: {} / {}", "Veicoli sbloccati: {} / {}", "Vehículos desbloqueados: {} / {}")
        .avec(&[&progress.count(), &total]);
    spawn_text(&mut commands, panel, line, 20.0);
    spawn_gap(&mut commands, panel, 8.0);
    button(&mut commands, panel, tx!("Jouer", "Play", "Spielen", "Gioca", "Jugar").t(), UiAction::Play);
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

pub fn spawn_game_over(mut commands: Commands, score: Res<Score>, death: Res<Death>, progress: Res<Progress>) {
    let panel = spawn_panel(&mut commands, DespawnOnExit(GameState::GameOver));
    let title = death.cause.map(|c| c.message()).unwrap_or(tx!("Perdu !", "Game over!", "Verloren!", "Hai perso!", "¡Has perdido!").t());
    spawn_text(&mut commands, panel, title, 48.0);
    spawn_text(&mut commands, panel, score_line(&score), 24.0);
    // Ce que cette partie vient de débloquer, puis le prochain objectif.
    for item in &progress.fresh {
        let line = tx!("Débloqué : {} !", "Unlocked: {}!", "Freigeschaltet: {}!", "Sbloccato: {}!", "¡Desbloqueado: {}!").avec(&[&item.name()]);
        spawn_text(&mut commands, panel, line, 26.0);
    }
    if let Some((at, item)) = progress.next() {
        let line = tx!(
            "Total {} pts — prochain : {} à {} pts",
            "Total {} pts — next: {} at {} pts",
            "Gesamt {} Pkt. — nächstes: {} bei {} Pkt.",
            "Totale {} pt — prossimo: {} a {} pt",
            "Total {} pts — siguiente: {} a {} pts"
        )
        .avec(&[&progress.total, &item.name(), &at]);
        spawn_text(&mut commands, panel, line, 18.0);
    }
    spawn_gap(&mut commands, panel, 6.0);
    button(&mut commands, panel, tx!("Rejouer", "Play again", "Nochmal", "Rigioca", "Volver a jugar").t(), UiAction::Play);
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
        UiAction::Menu => next.set(GameState::Menu),
    }
}
