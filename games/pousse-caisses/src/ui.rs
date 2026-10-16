//! Écrans : accueil, choix du niveau, barre de jeu, niveau réussi.
use bevy::prelude::*;
use game_common::langue::Texte;
use game_common::tx;
use game_common::ui::{
    GameHelp, PANEL, UiButton, pied_de_menu, spawn_button, spawn_button_sized, spawn_gap, spawn_panel, spawn_panel_bottom,
    spawn_panel_left, spawn_text,
};

use crate::GameState;
use crate::audio::UiClick;
use crate::play::{Play, PlayEvent, Progress, load_level, restart, undo};

/// Aide du jeu (bouton « Aide » du menu).
const HELP: Texte = tx!(
    "Rangez toutes les caisses sur les cibles !\n\
Le crapaud pousse les caisses, il ne peut pas les tirer.\n\
\n\
Flèches ou ZQSD, ou glisser du doigt : se déplacer\n\
U ou Retour arrière : annuler · R : recommencer\n\
M : son · Échap : menu",
    "Push every crate onto a target!\n\
The toad pushes the crates, it cannot pull them.\n\
\n\
Arrow keys or WASD, or swipe: move\n\
U or Backspace: undo · R: restart\n\
M: sound · Esc: menu",
    "Schiebe alle Kisten auf die Zielfelder!\n\
Die Kröte schiebt die Kisten, ziehen kann sie sie nicht.\n\
\n\
Pfeiltasten oder WASD, oder wischen: bewegen\n\
U oder Rücktaste: rückgängig · R: neu starten\n\
M: Ton · Esc: Menü",
    "Metti tutte le casse sui bersagli!\n\
Il rospo spinge le casse, ma non può tirarle.\n\
\n\
Frecce o WASD, o scorri col dito: muoviti\n\
U o Backspace: annulla · R: ricomincia\n\
M: audio · Esc: menu",
    "¡Coloca todas las cajas sobre las dianas!\n\
El sapo empuja las cajas, pero no puede tirar de ellas.\n\
\n\
Flechas o WASD, o deslizar el dedo: moverse\n\
U o Retroceso: deshacer · R: reiniciar\n\
M: sonido · Esc: menú",
);

/// Nom d'un niveau (écrit en français dans `levels.txt`) dans la langue en cours.
pub fn level_name(name: &str) -> &str {
    let t = match name {
        "Premiers pas" => tx!("Premiers pas", "First Steps", "Erste Schritte", "Primi passi", "Primeros pasos"),
        "Le virage" => tx!("Le virage", "The Bend", "Die Kurve", "La curva", "La curva"),
        "Deux caisses" => tx!("Deux caisses", "Two Crates", "Zwei Kisten", "Due casse", "Dos cajas"),
        "La rangée" => tx!("La rangée", "The Row", "Die Reihe", "La fila", "La hilera"),
        "Le détour" => tx!("Le détour", "The Detour", "Der Umweg", "La deviazione", "El rodeo"),
        "Les quatre vents" => tx!("Les quatre vents", "The Four Winds", "Die vier Winde", "I quattro venti", "Los cuatro vientos"),
        "La place" => tx!("La place", "The Square", "Der Platz", "La piazza", "La plaza"),
        "Les quatre coins" => tx!("Les quatre coins", "The Four Corners", "Die vier Ecken", "I quattro angoli", "Las cuatro esquinas"),
        "La croix" => tx!("La croix", "The Cross", "Das Kreuz", "La croce", "La cruz"),
        "L'entrepôt" => tx!("L'entrepôt", "The Warehouse", "Das Lagerhaus", "Il magazzino", "El almacén"),
        "Le serpentin" => tx!("Le serpentin", "The Serpentine", "Die Schlangenlinie", "La serpentina", "La serpentina"),
        "Le grand final" => tx!("Le grand final", "The Grand Finale", "Das große Finale", "Il gran finale", "El gran final"),
        _ => return name,
    };
    t.t()
}

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameHelp(HELP))
            .add_systems(OnEnter(GameState::Menu), spawn_menu)
            .add_systems(OnEnter(GameState::Levels), spawn_levels)
            .add_systems(OnEnter(GameState::Playing), spawn_hud)
            .add_systems(OnEnter(GameState::Won), spawn_won)
            .add_systems(Update, (update_hud, keyboard_shortcuts))
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
}

#[derive(Component)]
struct HudText;

fn button(commands: &mut Commands, parent: Entity, label: impl Into<String>, action: UiAction) -> Entity {
    let (b, text) = spawn_button(commands, parent, label);
    commands.entity(b).insert(action);
    text
}

const CONTINUE: Texte = tx!("Continuer : niveau {}", "Continue: level {}", "Weiter: Level {}", "Continua: livello {}", "Continuar: nivel {}");
const CHOOSE: Texte = tx!("Choisir un niveau", "Choose a level", "Level wählen", "Scegli un livello", "Elegir un nivel");
const MENU: Texte = tx!("Menu", "Menu", "Menü", "Menu", "Menú");

fn spawn_menu(mut commands: Commands, progress: Res<Progress>) {
    let panel = spawn_panel_left(&mut commands, DespawnOnExit(GameState::Menu));
    spawn_text(&mut commands, panel, tx!("POUSSE-CAISSES", "CRATE PUSHER", "KISTENSCHIEBER", "SPINGI-CASSE", "EMPUJACAJAS").t(), 56.0);
    spawn_gap(&mut commands, panel, 8.0);
    let total = progress.levels.len();
    let n = if progress.unlocked >= total && total > 0 { total } else { progress.unlocked };
    let label = CONTINUE.avec(&[&n]);
    button(&mut commands, panel, label, UiAction::Continue);
    button(&mut commands, panel, CHOOSE.t(), UiAction::ChooseLevel);
    pied_de_menu(&mut commands, panel);
}

/// Grille des niveaux : ceux pas encore débloqués sont grisés.
fn spawn_levels(mut commands: Commands, progress: Res<Progress>) {
    let panel = spawn_panel(&mut commands, DespawnOnExit(GameState::Levels));
    spawn_text(&mut commands, panel, CHOOSE.t(), 40.0);
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
        let label = if open { format!("{} · {}", i + 1, level_name(&level.name)) } else { format!("{} · ???", i + 1) };
        let (b, _) = spawn_button_sized(&mut commands, grid, label, 300.0, 46.0, 19.0);
        if open {
            commands.entity(b).insert(UiAction::Pick(i));
        } else {
            commands.entity(b).remove::<UiButton>().insert(BackgroundColor(Color::srgb(0.82, 0.82, 0.80)));
        }
    }
    spawn_gap(&mut commands, panel, 4.0);
    button(&mut commands, panel, tx!("Retour", "Back", "Zurück", "Indietro", "Volver").t(), UiAction::Back);
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
    let undo = tx!("Annuler", "Undo", "Rückgängig", "Annulla", "Deshacer");
    let restart = tx!("Recommencer", "Restart", "Neustart", "Ricomincia", "Reiniciar");
    for (label, action) in [(undo, UiAction::Undo), (restart, UiAction::Restart), (MENU, UiAction::Menu)] {
        let (b, _) = spawn_button_sized(&mut commands, buttons, label.t(), 150.0, 44.0, 18.0);
        commands.entity(b).insert(action);
    }
}

fn spawn_won(mut commands: Commands, play: Res<Play>, progress: Res<Progress>) {
    let panel = spawn_panel_bottom(&mut commands, DespawnOnExit(GameState::Won));
    let last = play.index + 1 >= progress.levels.len();
    let title = if last {
        tx!(
            "Bravo, tous les niveaux sont réussis !",
            "Well done, every level is solved!",
            "Bravo, alle Level geschafft!",
            "Bravo, hai superato tutti i livelli!",
            "¡Bravo, has superado todos los niveles!"
        )
    } else {
        tx!("Niveau réussi !", "Level complete!", "Level geschafft!", "Livello superato!", "¡Nivel superado!")
    };
    spawn_text(&mut commands, panel, title.t(), 36.0);
    let result = if play.moves > 1 {
        tx!("« {} » en {} coups", "“{}” in {} moves", "„{}“ in {} Zügen", "«{}» in {} mosse", "«{}» en {} movimientos")
    } else {
        tx!("« {} » en {} coup", "“{}” in {} move", "„{}“ in {} Zug", "«{}» in {} mossa", "«{}» en {} movimiento")
    };
    spawn_text(&mut commands, panel, result.avec(&[&level_name(&play.level.name), &play.moves]), 22.0);
    if !last {
        button(&mut commands, panel, tx!("Niveau suivant", "Next level", "Nächstes Level", "Livello successivo", "Siguiente nivel").t(), UiAction::Next);
    }
    button(&mut commands, panel, tx!("Rejouer ce niveau", "Replay this level", "Level wiederholen", "Rigioca questo livello", "Repetir este nivel").t(), UiAction::Replay);
    button(&mut commands, panel, MENU.t(), UiAction::Menu);
}

fn update_hud(play: Res<Play>, progress: Res<Progress>, mut text: Query<&mut Text, With<HudText>>) {
    for mut t in &mut text {
        let s = tx!(
            "Niveau {}/{} · {} · Coups : {}",
            "Level {}/{} · {} · Moves: {}",
            "Level {}/{} · {} · Züge: {}",
            "Livello {}/{} · {} · Mosse: {}",
            "Nivel {}/{} · {} · Movimientos: {}"
        )
        .avec(&[&(play.index + 1), &progress.levels.len(), &level_name(&play.level.name), &play.moves]);
        if t.0 != s {
            t.0 = s;
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

fn on_click(
    click: On<PointerClick>,
    actions: Query<&UiAction>,
    mut play: ResMut<Play>,
    progress: Res<Progress>,
    mut next: ResMut<NextState<GameState>>,
    mut clicks: MessageWriter<UiClick>,
    mut events: MessageWriter<PlayEvent>,
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
    }
}
