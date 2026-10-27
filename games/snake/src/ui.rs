//! Écrans : accueil, réglages, fin de partie, et affichage du score.
use bevy::prelude::*;
use game_common::langue::Langue;
use game_common::ui::{GameHelp, pied_de_menu, spawn_button, spawn_gap, spawn_hud, spawn_panel, spawn_text};
use game_common::{Score, tx};

use crate::GameState;
use crate::audio::Sfx;
use crate::settings::{OptionKind, Settings};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_hud)
            .insert_resource(GameHelp(tx!(
                "Dirigez le serpent et mangez pour grandir.\n\
Ne vous mordez pas la queue !\n\
\n\
Flèches ou ZQSD : diriger\n\
Entrée ou Espace : valider\n\
M : couper le son · Échap : menu\n\
\n\
Réglages : vitesse, taille du plateau,\n\
bords murs ou traversables.",
                "Steer the snake and eat to grow.\n\
Don't bite your own tail!\n\
\n\
Arrows or WASD: steer\n\
Enter or Space: confirm\n\
M: mute · Esc: menu\n\
\n\
Settings: speed, board size,\n\
solid or wrap-around edges.",
                "Steuere die Schlange und friss, um zu wachsen.\n\
Beiß dir nicht in den Schwanz!\n\
\n\
Pfeiltasten oder WASD: steuern\n\
Eingabe oder Leertaste: bestätigen\n\
M: Ton aus · Esc: Menü\n\
\n\
Einstellungen: Tempo, Spielfeldgröße,\n\
Ränder als Wände oder durchlässig.",
                "Guida il serpente e mangia per crescere.\n\
Non morderti la coda!\n\
\n\
Frecce o WASD: dirigere\n\
Invio o Spazio: confermare\n\
M: disattiva l'audio · Esc: menu\n\
\n\
Impostazioni: velocità, dimensione del campo,\n\
bordi come muri o attraversabili.",
                "Guía la serpiente y come para crecer.\n\
¡No te muerdas la cola!\n\
\n\
Flechas o WASD: dirigir\n\
Intro o Espacio: confirmar\n\
M: silenciar · Esc: menú\n\
\n\
Ajustes: velocidad, tamaño del tablero,\n\
bordes como muros o atravesables."
            )))
            .add_systems(OnEnter(GameState::Menu), spawn_menu)
            .add_systems(OnEnter(GameState::Options), spawn_options)
            .add_systems(OnEnter(GameState::GameOver), spawn_game_over)
            .add_systems(Update, (update_hud, refresh_option_texts, keyboard_shortcuts))
            .add_observer(on_click);
    }
}

/// Ce que fait un bouton quand on clique dessus.
#[derive(Component, Clone, Copy)]
enum UiAction {
    Play,
    Settings,
    Back,
    Menu,
    Cycle(OptionKind),
}

/// Texte d'un bouton d'option, mis à jour quand la valeur change.
#[derive(Component)]
struct OptionText(OptionKind);

#[derive(Component)]
struct Hud;

#[derive(Component)]
struct HudText;

fn button(commands: &mut Commands, parent: Entity, label: impl Into<String>, action: UiAction) -> Entity {
    let (b, text) = spawn_button(commands, parent, label);
    commands.entity(b).insert(action);
    text
}

fn spawn_menu(mut commands: Commands, settings: Res<Settings>) {
    let panel = spawn_panel(&mut commands, DespawnOnExit(GameState::Menu));
    spawn_text(&mut commands, panel, "SNAKE", 72.0);
    spawn_gap(&mut commands, panel, 10.0);
    button(&mut commands, panel, tx!("Jouer", "Play", "Spielen", "Gioca", "Jugar").t(), UiAction::Play);
    let t = button(
        &mut commands,
        panel,
        OptionKind::Universe.text(&settings),
        UiAction::Cycle(OptionKind::Universe),
    );
    commands.entity(t).insert(OptionText(OptionKind::Universe));
    button(
        &mut commands,
        panel,
        tx!("Réglages", "Settings", "Einstellungen", "Impostazioni", "Ajustes").t(),
        UiAction::Settings,
    );
    pied_de_menu(&mut commands, panel);
}

fn spawn_options(mut commands: Commands, settings: Res<Settings>) {
    let panel = spawn_panel(&mut commands, DespawnOnExit(GameState::Options));
    spawn_text(&mut commands, panel, tx!("Réglages", "Settings", "Einstellungen", "Impostazioni", "Ajustes").t(), 48.0);
    for kind in OptionKind::ALL {
        let t = button(&mut commands, panel, kind.text(&settings), UiAction::Cycle(kind));
        commands.entity(t).insert(OptionText(kind));
    }
    spawn_gap(&mut commands, panel, 6.0);
    button(&mut commands, panel, tx!("Retour", "Back", "Zurück", "Indietro", "Volver").t(), UiAction::Back);
}

fn spawn_game_over(mut commands: Commands, score: Res<Score>) {
    let panel = spawn_panel(&mut commands, DespawnOnExit(GameState::GameOver));
    spawn_text(&mut commands, panel, tx!("Perdu !", "Game over!", "Verloren!", "Hai perso!", "¡Has perdido!").t(), 56.0);
    spawn_text(
        &mut commands,
        panel,
        tx!("Score : {}    Record : {}", "Score: {}    Best: {}", "Punkte: {}    Rekord: {}", "Punti: {}    Record: {}", "Puntos: {}    Récord: {}")
            .avec(&[&score.current, &score.best]),
        24.0,
    );
    spawn_gap(&mut commands, panel, 6.0);
    button(&mut commands, panel, tx!("Rejouer", "Play again", "Nochmal", "Rigioca", "Otra vez").t(), UiAction::Play);
    button(&mut commands, panel, tx!("Menu", "Menu", "Menü", "Menu", "Menú").t(), UiAction::Menu);
}

fn setup_hud(mut commands: Commands) {
    let (hud, text) = spawn_hud(&mut commands);
    commands.entity(hud).insert((Hud, Visibility::Hidden));
    commands.entity(text).insert(HudText);
}

fn update_hud(
    score: Res<Score>,
    langue: Res<Langue>,
    state: Res<State<GameState>>,
    mut hud: Query<&mut Visibility, With<Hud>>,
    mut text: Query<&mut Text, With<HudText>>,
) {
    let playing = *state.get() == GameState::Playing;
    for mut v in &mut hud {
        *v = if playing { Visibility::Inherited } else { Visibility::Hidden };
    }
    if score.is_changed() || langue.is_changed() {
        let hud = tx!(
            "Score : {}    Record : {}    Échap : menu    P : pause",
            "Score: {}    Best: {}    Esc: menu    M: sound",
            "Punkte: {}    Rekord: {}    Esc: Menü    M: Ton",
            "Punti: {}    Record: {}    Esc: menu    M: audio",
            "Puntos: {}    Récord: {}    Esc: menú    M: sonido"
        );
        for mut t in &mut text {
            t.0 = hud.avec(&[&score.current, &score.best]);
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
    mut sfx: MessageWriter<Sfx>,
) {
    let Ok(action) = actions.get(click.entity) else {
        return;
    };
    sfx.write(Sfx::Click);
    match *action {
        UiAction::Play => next.set(GameState::Playing),
        UiAction::Settings => next.set(GameState::Options),
        UiAction::Back | UiAction::Menu => next.set(GameState::Menu),
        UiAction::Cycle(kind) => kind.cycle(&mut settings),
    }
}
