//! Le menu et le bandeau de jeu. Tous les textes en cinq langues avec `tx!`.
use bevy::prelude::*;
use game_common::Score;
use game_common::tx;
use game_common::ui::{GameHelp, pied_de_menu, spawn_button, spawn_hud, spawn_panel, spawn_text};

use crate::GameState;

/// Aide du jeu (bouton « Aide » du pied de menu).
const AIDE: game_common::langue::Texte = tx!(
    "Le crapaud ramasse les pièces de la prairie.\n\nFlèches ou ZQSD : se déplacer\nÉchap : menu · M : son",
    "The toad picks up the coins in the meadow.\n\nArrow keys or WASD: move\nEsc: menu · M: sound",
    "Die Kröte sammelt die Münzen auf der Wiese.\n\nPfeiltasten oder WASD: bewegen\nEsc: Menü · M: Ton",
    "Il rospo raccoglie le monete nel prato.\n\nFrecce o WASD: muoversi\nEsc: menu · M: audio",
    "El sapo recoge las monedas del prado.\n\nFlechas o WASD: moverse\nEsc: menú · M: sonido",
);

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameHelp(AIDE))
            .add_systems(OnEnter(GameState::Menu), spawn_menu)
            .add_systems(OnEnter(GameState::Playing), spawn_bandeau)
            .add_systems(Update, (jouer_au_clavier, revenir_au_menu, maj_bandeau))
            .add_observer(au_clic);
    }
}

#[derive(Component)]
struct BoutonJouer;

#[derive(Component)]
struct TexteScore;

fn spawn_menu(mut commands: Commands, score: Res<Score>) {
    let panel = spawn_panel(&mut commands, DespawnOnExit(GameState::Menu));
    spawn_text(&mut commands, panel, tx!("MODÈLE", "TEMPLATE", "VORLAGE", "MODELLO", "PLANTILLA").t(), 64.0);
    if score.best > 0 {
        spawn_text(&mut commands, panel, tx!("Record : {}", "Best: {}", "Rekord: {}", "Record: {}", "Récord: {}").avec(&[&score.best]), 20.0);
    }
    let (b, _) = spawn_button(&mut commands, panel, tx!("Jouer", "Play", "Spielen", "Gioca", "Jugar").t());
    commands.entity(b).insert(BoutonJouer);
    // Les boutons du jeu d'abord, puis le pied commun : Options, Aide, Quitter, logo.
    pied_de_menu(&mut commands, panel);
}

fn au_clic(clic: On<PointerClick>, jouer: Query<(), With<BoutonJouer>>, mut next: ResMut<NextState<GameState>>) {
    if jouer.contains(clic.entity) {
        next.set(GameState::Playing);
    }
}

fn jouer_au_clavier(keys: Res<ButtonInput<KeyCode>>, state: Res<State<GameState>>, mut next: ResMut<NextState<GameState>>) {
    if *state.get() == GameState::Menu && keys.any_just_pressed([KeyCode::Enter, KeyCode::Space]) {
        next.set(GameState::Playing);
    }
}

fn revenir_au_menu(keys: Res<ButtonInput<KeyCode>>, state: Res<State<GameState>>, mut next: ResMut<NextState<GameState>>) {
    if *state.get() == GameState::Playing && keys.just_pressed(KeyCode::Escape) {
        next.set(GameState::Menu);
    }
}

fn spawn_bandeau(mut commands: Commands) {
    let (hud, texte) = spawn_hud(&mut commands);
    commands.entity(hud).insert(DespawnOnExit(GameState::Playing));
    commands.entity(texte).insert(TexteScore);
}

fn maj_bandeau(score: Res<Score>, mut q: Query<&mut Text, With<TexteScore>>) {
    for mut t in &mut q {
        t.0 = tx!("Pièces : {}    Record : {}", "Coins: {}    Best: {}", "Münzen: {}    Rekord: {}", "Monete: {}    Record: {}", "Monedas: {}    Récord: {}")
            .avec(&[&score.current, &score.best]);
    }
}
