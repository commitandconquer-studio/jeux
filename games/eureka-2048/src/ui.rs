//! Écrans : accueil, barre de jeu, faits « Eurêka ! », victoire, fin de partie.
use bevy::prelude::*;
use game_common::langue::Texte;
use game_common::ui::{GameHelp, pied_de_menu};
use game_common::ui::{PANEL, ThemedPanel, spawn_button, spawn_button_sized, spawn_gap, spawn_panel_left, spawn_text};
use game_common::universe::Universe;
use game_common::{Rng, tx};

use crate::GameState;
use crate::audio::UiClick;
use crate::play::{Game, PlayEvent};

/// Aide du jeu (bouton « Aide » du menu).
const HELP: Texte = tx!(
    "Faites glisser les tuiles : deux nombres égaux\n\
fusionnent et doublent. Atteindrez-vous 2048 ?\n\
\n\
Flèches, ZQSD ou glisser du doigt : faire glisser\n\
U ou Retour arrière : annuler le dernier coup\n\
M : son · Échap : menu",
    "Slide the tiles: two equal numbers\n\
merge and double. Can you reach 2048?\n\
\n\
Arrows, WASD or swipe: slide the tiles\n\
U or Backspace: undo the last move\n\
M: sound · Esc: menu",
    "Schiebe die Kacheln: Zwei gleiche Zahlen\n\
verschmelzen und verdoppeln sich. Schaffst du 2048?\n\
\n\
Pfeiltasten, WASD oder Wischen: schieben\n\
U oder Rücktaste: letzten Zug zurücknehmen\n\
M: Ton · Esc: Menü",
    "Fai scorrere le tessere: due numeri uguali\n\
si fondono e raddoppiano. Riuscirai a fare 2048?\n\
\n\
Frecce, WASD o scorrimento del dito: far scorrere\n\
U o Backspace: annulla l'ultima mossa\n\
M: audio · Esc: menu",
    "Desliza las fichas: dos números iguales\n\
se fusionan y se duplican. ¿Llegarás a 2048?\n\
\n\
Flechas, WASD o deslizar el dedo: deslizar\n\
U o Retroceso: deshacer el último movimiento\n\
M: sonido · Esc: menú",
);

const NEW_GAME: Texte = tx!("Nouvelle partie", "New game", "Neues Spiel", "Nuova partita", "Nueva partida");
const MENU: Texte = tx!("Menu", "Menu", "Menü", "Menu", "Menú");
const SCORES: Texte = tx!(
    "Score : {}    Record : {}",
    "Score: {}    Best: {}",
    "Punkte: {}    Rekord: {}",
    "Punteggio: {}    Record: {}",
    "Puntos: {}    Récord: {}",
);

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameHelp(HELP))
            .add_systems(OnEnter(GameState::Menu), spawn_menu)
            .add_systems(OnEnter(GameState::Playing), spawn_hud)
            .add_systems(OnEnter(GameState::Won), spawn_won)
            .add_systems(OnEnter(GameState::Over), spawn_over)
            .add_systems(Update, (update_hud, show_facts, fade_facts, keyboard))
            .add_observer(on_click);
    }
}

/// Petits faits scientifiques sur les puissances de 2.
fn fact(value: u32) -> Option<Texte> {
    Some(match value {
        8 => tx!(
            "8 = 2 × 2 × 2 : c'est 2 puissance 3, un cube de 2 de côté !",
            "8 = 2 × 2 × 2: that's 2 to the power of 3, a cube with sides of 2!",
            "8 = 2 × 2 × 2: Das ist 2 hoch 3, ein Würfel mit der Kantenlänge 2!",
            "8 = 2 × 2 × 2: è 2 alla terza, un cubo di lato 2!",
            "8 = 2 × 2 × 2: es 2 elevado a 3, ¡un cubo de lado 2!",
        ),
        16 => tx!(
            "16 = 2 puissance 4. Les informaticiens comptent souvent en base 16 : l'hexadécimal.",
            "16 = 2 to the power of 4. Programmers often count in base 16: hexadecimal.",
            "16 = 2 hoch 4. Informatiker rechnen oft zur Basis 16: im Hexadezimalsystem.",
            "16 = 2 alla quarta. Gli informatici contano spesso in base 16: l'esadecimale.",
            "16 = 2 elevado a 4. Los informáticos cuentan a menudo en base 16: el hexadecimal.",
        ),
        32 => tx!(
            "32 = 2 puissance 5. C'est le nombre de dents d'un adulte !",
            "32 = 2 to the power of 5. That's how many teeth an adult has!",
            "32 = 2 hoch 5. So viele Zähne hat ein Erwachsener!",
            "32 = 2 alla quinta. È il numero dei denti di un adulto!",
            "32 = 2 elevado a 5. ¡Es el número de dientes de un adulto!",
        ),
        64 => tx!(
            "64 = 2 puissance 6 : le nombre de cases d'un échiquier.",
            "64 = 2 to the power of 6: the number of squares on a chessboard.",
            "64 = 2 hoch 6: die Zahl der Felder eines Schachbretts.",
            "64 = 2 alla sesta: il numero di caselle di una scacchiera.",
            "64 = 2 elevado a 6: el número de casillas de un tablero de ajedrez.",
        ),
        128 => tx!(
            "128 = 2 puissance 7. Le code ASCII des premiers ordinateurs comptait 128 caractères.",
            "128 = 2 to the power of 7. The ASCII code of early computers had 128 characters.",
            "128 = 2 hoch 7. Der ASCII-Code der ersten Computer umfasste 128 Zeichen.",
            "128 = 2 alla settima. Il codice ASCII dei primi computer contava 128 caratteri.",
            "128 = 2 elevado a 7. El código ASCII de los primeros ordenadores tenía 128 caracteres.",
        ),
        256 => tx!(
            "256 = 2 puissance 8 : un octet peut prendre 256 valeurs différentes.",
            "256 = 2 to the power of 8: a byte can hold 256 different values.",
            "256 = 2 hoch 8: Ein Byte kann 256 verschiedene Werte annehmen.",
            "256 = 2 all'ottava: un byte può assumere 256 valori diversi.",
            "256 = 2 elevado a 8: un byte puede tomar 256 valores distintos.",
        ),
        512 => tx!(
            "512 = 2 puissance 9. Chaque fusion double la tuile : la croissance exponentielle !",
            "512 = 2 to the power of 9. Each merge doubles the tile: exponential growth!",
            "512 = 2 hoch 9. Jede Verschmelzung verdoppelt die Kachel: exponentielles Wachstum!",
            "512 = 2 alla nona. Ogni fusione raddoppia la tessera: la crescita esponenziale!",
            "512 = 2 elevado a 9. Cada fusión duplica la ficha: ¡el crecimiento exponencial!",
        ),
        1024 => tx!(
            "1024 = 2 puissance 10 : c'est le « kibi » de l'informatique (1 Kio = 1024 octets).",
            "1024 = 2 to the power of 10: the \"kibi\" of computing (1 KiB = 1024 bytes).",
            "1024 = 2 hoch 10: das „Kibi“ der Informatik (1 KiB = 1024 Byte).",
            "1024 = 2 alla decima: il «kibi» dell'informatica (1 KiB = 1024 byte).",
            "1024 = 2 elevado a 10: el «kibi» de la informática (1 KiB = 1024 bytes).",
        ),
        2048 => tx!(
            "2048 = 2 puissance 11. Eurêka !",
            "2048 = 2 to the power of 11. Eureka!",
            "2048 = 2 hoch 11. Heureka!",
            "2048 = 2 all'undicesima. Eureka!",
            "2048 = 2 elevado a 11. ¡Eureka!",
        ),
        4096 => tx!(
            "4096 = 2 puissance 12. Archimède serait fier de vous !",
            "4096 = 2 to the power of 12. Archimedes would be proud of you!",
            "4096 = 2 hoch 12. Archimedes wäre stolz auf dich!",
            "4096 = 2 alla dodicesima. Archimede sarebbe fiero di te!",
            "4096 = 2 elevado a 12. ¡Arquímedes estaría orgulloso de ti!",
        ),
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
}

#[derive(Component)]
struct ScoreText;
#[derive(Component)]
struct FactCard(f32);

fn button(c: &mut Commands, parent: Entity, label: impl Into<String>, action: UiAction) -> Entity {
    let (b, text) = spawn_button(c, parent, label);
    c.entity(b).insert(action);
    text
}

fn spawn_menu(mut c: Commands, game: Res<Game>) {
    let panel = spawn_panel_left(&mut c, DespawnOnExit(GameState::Menu));
    spawn_text(&mut c, panel, "2048", 96.0);
    spawn_text(&mut c, panel, tx!("les puissances de 2", "the powers of 2", "die Zweierpotenzen", "le potenze di 2", "las potencias de 2").t(), 26.0);
    spawn_text(&mut c, panel, tx!("un jeu Eurêka", "a Eureka game", "ein Eureka-Spiel", "un gioco Eureka", "un juego Eureka").t(), 17.0);
    spawn_gap(&mut c, panel, 6.0);
    let in_progress = game.score > 0;
    let play = if in_progress {
        tx!("Continuer", "Continue", "Weiter", "Continua", "Continuar")
    } else {
        tx!("Jouer", "Play", "Spielen", "Gioca", "Jugar")
    };
    button(&mut c, panel, play.t(), UiAction::Play);
    if in_progress {
        button(&mut c, panel, NEW_GAME.t(), UiAction::NewGame);
    }
    pied_de_menu(&mut c, panel);
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
    let undo = tx!("Annuler", "Undo", "Rückgängig", "Annulla", "Deshacer");
    for (label, action) in [(undo.t(), UiAction::Undo), (NEW_GAME.t(), UiAction::NewGame), (MENU.t(), UiAction::Menu)] {
        let width = if label.chars().count() > 8 { 190.0 } else { 130.0 };
        let (b, _) = spawn_button_sized(&mut c, buttons, label, width, 44.0, 19.0);
        c.entity(b).insert(action);
    }
}

fn spawn_won(mut c: Commands, game: Res<Game>) {
    let panel = spawn_panel_left(&mut c, DespawnOnExit(GameState::Won));
    spawn_text(&mut c, panel, tx!("Eurêka ! 2048 !", "Eureka! 2048!", "Heureka! 2048!", "Eureka! 2048!", "¡Eureka! ¡2048!").t(), 48.0);
    spawn_text(&mut c, panel, SCORES.avec(&[&game.score, &game.best]), 22.0);
    button(&mut c, panel, tx!("Continuer plus loin", "Keep going", "Weiterspielen", "Continua ancora", "Seguir jugando").t(), UiAction::Continue);
    button(&mut c, panel, NEW_GAME.t(), UiAction::NewGame);
}

fn spawn_over(mut c: Commands, game: Res<Game>, cards: Query<Entity, With<FactCard>>) {
    for e in &cards {
        c.entity(e).despawn();
    }
    let panel = spawn_panel_left(&mut c, DespawnOnExit(GameState::Over));
    let record = game.score > 0 && game.score >= game.best;
    let title = if record {
        tx!("Nouveau record !", "New record!", "Neuer Rekord!", "Nuovo record!", "¡Nuevo récord!")
    } else {
        tx!("Plus aucun coup possible !", "No moves left!", "Kein Zug mehr möglich!", "Nessuna mossa possibile!", "¡No quedan movimientos!")
    };
    spawn_text(&mut c, panel, title.t(), 40.0);
    let biggest = tx!("Plus grande tuile : {}", "Biggest tile: {}", "Größte Kachel: {}", "Tessera più grande: {}", "Ficha más grande: {}");
    spawn_text(&mut c, panel, format!("{}\n{}", SCORES.avec(&[&game.score, &game.best]), biggest.avec(&[&game.grid.max_value()])), 20.0);
    if game.can_undo() {
        let undo = tx!("Annuler le dernier coup", "Undo last move", "Letzten Zug zurücknehmen", "Annulla l'ultima mossa", "Deshacer la última jugada");
        let (b, _) = spawn_button_sized(&mut c, panel, undo.t(), 380.0, 56.0, 24.0);
        c.entity(b).insert(UiAction::Undo);
    }
    button(&mut c, panel, NEW_GAME.t(), UiAction::NewGame);
    button(&mut c, panel, MENU.t(), UiAction::Menu);
}

fn update_hud(game: Res<Game>, mut q: Query<&mut Text, With<ScoreText>>) {
    for mut t in &mut q {
        let s = tx!(
            "Score : {}   ·   Record : {}",
            "Score: {}   ·   Best: {}",
            "Punkte: {}   ·   Rekord: {}",
            "Punteggio: {}   ·   Record: {}",
            "Puntos: {}   ·   Récord: {}",
        )
        .avec(&[&game.score, &game.best]);
        if t.0 != s {
            t.0 = s;
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
        let title = spawn_text(&mut c, inner, tx!("Eurêka !", "Eureka!", "Heureka!", "Eureka!", "¡Eureka!").t(), 24.0);
        c.entity(title).insert(TextColor(Universe::Eureka.style().accent));
        spawn_text(&mut c, inner, text.t(), 19.0);
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

fn on_click(
    click: On<PointerClick>,
    actions: Query<&UiAction>,
    mut game: ResMut<Game>,
    mut rng: ResMut<Rng>,
    mut next: ResMut<NextState<GameState>>,
    mut clicks: MessageWriter<UiClick>,
    mut events: MessageWriter<PlayEvent>,
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
    }
}
