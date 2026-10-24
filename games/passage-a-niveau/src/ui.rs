//! Les écrans : menu (choix du numéro), barre de jeu, fin de partie.
//! Tous les textes en cinq langues avec `tx!`.
use bevy::prelude::*;
use game_common::calendar::Date;
use game_common::langue::Texte;
use game_common::tx;
use game_common::ui::{GameHelp, PANEL, ThemedPanel, pied_de_menu, spawn_button_sized, spawn_gap, spawn_panel_left, spawn_text};
use passage_core::day_number;

use crate::GameState;
use crate::jeu::{CARTES, JeuEvenement, NUMERO_MAX, Partie, Progres, commencer};

/// Aide du jeu (bouton « Aide » du pied de menu).
const AIDE: Texte = tx!(
    "Posez toutes les cartes de rails et de routes pour faire le meilleur score.\n\n\
     Chaque numéro donne toujours les mêmes cartes, dans le même ordre : choisissez un numéro et essayez de battre votre record.\n\n\
     • 1 point par raccord entre deux cartes voisines de la même voie.\n\
     • 1 point par carte de la plus longue voie ferrée et de la plus longue route.\n\
     • 5 points par passage à niveau dont les quatre côtés sont raccordés.\n\
     • -1 point par chemin qui arrive dans le vide (rond rouge).\n\n\
     Clic : poser · clic droit, R, espace ou molette : tourner\n\
     Retour arrière : annuler · Échap : menu\n\
     Au menu, tapez un numéro au clavier.",
    "Place every rail and road card to get the best score.\n\n\
     Each number always gives the same cards in the same order: pick a number and try to beat your best.\n\n\
     • 1 point for each link between two neighbouring cards of the same kind.\n\
     • 1 point per card of the longest railway and of the longest road.\n\
     • 5 points for each level crossing connected on all four sides.\n\
     • -1 point for each path that ends in nothing (red dot).\n\n\
     Click: place · right click, R, space or wheel: rotate\n\
     Backspace: undo · Esc: menu\n\
     In the menu, type a number on the keyboard.",
    "Lege alle Schienen- und Straßenkarten und erreiche die beste Punktzahl.\n\n\
     Jede Nummer ergibt immer dieselben Karten in derselben Reihenfolge: Wähle eine Nummer und versuche, deinen Rekord zu schlagen.\n\n\
     • 1 Punkt pro Anschluss zwischen zwei benachbarten Karten derselben Art.\n\
     • 1 Punkt pro Karte der längsten Bahnstrecke und der längsten Straße.\n\
     • 5 Punkte pro Bahnübergang, der an allen vier Seiten angeschlossen ist.\n\
     • -1 Punkt pro Weg, der im Leeren endet (roter Punkt).\n\n\
     Klick: legen · Rechtsklick, R, Leertaste oder Mausrad: drehen\n\
     Rücktaste: rückgängig · Esc: Menü\n\
     Im Menü kannst du eine Nummer tippen.",
    "Posa tutte le carte di binari e strade per fare il punteggio migliore.\n\n\
     Ogni numero dà sempre le stesse carte nello stesso ordine: scegli un numero e prova a battere il tuo record.\n\n\
     • 1 punto per ogni collegamento tra due carte vicine dello stesso tipo.\n\
     • 1 punto per carta della ferrovia più lunga e della strada più lunga.\n\
     • 5 punti per ogni passaggio a livello collegato su tutti e quattro i lati.\n\
     • -1 punto per ogni percorso che finisce nel vuoto (pallino rosso).\n\n\
     Clic: posare · clic destro, R, spazio o rotella: ruotare\n\
     Backspace: annulla · Esc: menu\n\
     Nel menu, digita un numero con la tastiera.",
    "Coloca todas las cartas de vías y carreteras para conseguir la mejor puntuación.\n\n\
     Cada número da siempre las mismas cartas en el mismo orden: elige un número e intenta superar tu récord.\n\n\
     • 1 punto por cada unión entre dos cartas vecinas del mismo tipo.\n\
     • 1 punto por carta de la vía férrea más larga y de la carretera más larga.\n\
     • 5 puntos por cada paso a nivel conectado por sus cuatro lados.\n\
     • -1 punto por cada camino que acaba en el vacío (punto rojo).\n\n\
     Clic: colocar · clic derecho, R, espacio o rueda: girar\n\
     Retroceso: deshacer · Esc: menú\n\
     En el menú, escribe un número con el teclado.",
);

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameHelp(AIDE))
            .add_systems(OnEnter(GameState::Menu), spawn_menu)
            .add_systems(OnEnter(GameState::Playing), spawn_barre)
            .add_systems(OnEnter(GameState::Fini), spawn_fin)
            .add_systems(Update, (clavier, maj_menu, maj_barre))
            .add_observer(au_clic);
    }
}

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Jouer,
    Ajouter(i64),
    Jour,
    Hasard,
    Rejouer,
    Tourner,
    Annuler,
    Menu,
}

#[derive(Component)]
struct TexteNumero;

#[derive(Component)]
struct TexteRecord;

#[derive(Component)]
struct TexteInfo;

fn bouton(c: &mut Commands, parent: Entity, label: &str, action: Action) -> Entity {
    let (b, t) = spawn_button_sized(c, parent, label, 340.0, 40.0, 19.0);
    c.entity(b).insert(action);
    t
}

fn spawn_menu(mut c: Commands) {
    let panel = spawn_panel_left(&mut c, DespawnOnExit(GameState::Menu));
    spawn_text(&mut c, panel, tx!("PASSAGE À NIVEAU", "LEVEL CROSSING", "BAHNÜBERGANG", "PASSAGGIO A LIVELLO", "PASO A NIVEL").t(), 40.0);
    spawn_text(&mut c, panel, tx!("Rails, routes et un numéro", "Rails, roads and a number", "Schienen, Straßen und eine Nummer", "Binari, strade e un numero", "Vías, carreteras y un número").t(), 20.0);
    spawn_gap(&mut c, panel, 6.0);
    let t = spawn_text(&mut c, panel, "", 30.0);
    c.entity(t).insert(TexteNumero);
    let t = spawn_text(&mut c, panel, "", 17.0);
    c.entity(t).insert(TexteRecord);
    let rangee = c.spawn((Node { column_gap: Val::Px(8.0), ..default() }, ChildOf(panel))).id();
    for (label, delta) in [("-10", -10), ("-1", -1), ("+1", 1), ("+10", 10)] {
        let (b, _) = spawn_button_sized(&mut c, rangee, label, 76.0, 40.0, 19.0);
        c.entity(b).insert(Action::Ajouter(delta));
    }
    bouton(&mut c, panel, tx!("Numéro du jour", "Number of the day", "Nummer des Tages", "Numero del giorno", "Número del día").t(), Action::Jour);
    bouton(&mut c, panel, tx!("Numéro au hasard", "Random number", "Zufällige Nummer", "Numero a caso", "Número al azar").t(), Action::Hasard);
    spawn_gap(&mut c, panel, 4.0);
    bouton(&mut c, panel, tx!("Jouer ce numéro", "Play this number", "Diese Nummer spielen", "Gioca questo numero", "Jugar este número").t(), Action::Jouer);
    spawn_text(&mut c, panel, tx!("(ou tapez un numéro au clavier)", "(or type a number)", "(oder tippe eine Nummer)", "(o digita un numero)", "(o escribe un número)").t(), 15.0);
    pied_de_menu(&mut c, panel);
}

fn maj_menu(progres: Res<Progres>, mut numero: Query<&mut Text, (With<TexteNumero>, Without<TexteRecord>)>, mut record: Query<&mut Text, (With<TexteRecord>, Without<TexteNumero>)>) {
    for mut t in &mut numero {
        t.0 = tx!("Numéro {}", "Number {}", "Nummer {}", "Numero {}", "Número {}").avec(&[&progres.numero]);
    }
    for mut t in &mut record {
        t.0 = match progres.record(progres.numero) {
            Some(r) => tx!("Votre record : {}", "Your best: {}", "Dein Rekord: {}", "Il tuo record: {}", "Tu récord: {}").avec(&[&r]),
            None => tx!("Pas encore joué", "Not played yet", "Noch nicht gespielt", "Non ancora giocato", "Aún sin jugar").t().to_string(),
        };
    }
}

fn spawn_barre(mut c: Commands) {
    let barre = c
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                top: Val::Px(12.0),
                padding: UiRect::horizontal(Val::Px(12.0)),
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::FlexStart,
                flex_wrap: FlexWrap::Wrap,
                row_gap: Val::Px(8.0),
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
            ChildOf(barre),
        ))
        .id();
    let t = spawn_text(&mut c, info, "", 19.0);
    c.entity(t).insert((TexteInfo, TextLayout::justify(Justify::Left)));
    let boutons = c.spawn((Node { column_gap: Val::Px(8.0), ..default() }, ChildOf(barre))).id();
    for (label, action) in [
        (tx!("Tourner", "Rotate", "Drehen", "Ruota", "Girar"), Action::Tourner),
        (tx!("Annuler", "Undo", "Rückgängig", "Annulla", "Deshacer"), Action::Annuler),
        (tx!("Menu", "Menu", "Menü", "Menu", "Menú"), Action::Menu),
    ] {
        let (b, _) = spawn_button_sized(&mut c, boutons, label.t(), 118.0, 42.0, 18.0);
        c.entity(b).insert(action);
    }
}

fn maj_barre(partie: Res<Partie>, progres: Res<Progres>, mut info: Query<&mut Text, With<TexteInfo>>) {
    let s = partie.jeu.score();
    let numero = partie.jeu.number;
    for mut t in &mut info {
        let ligne1 = tx!("Numéro {} · carte {} sur {}", "Number {} · card {} of {}", "Nummer {} · Karte {} von {}", "Numero {} · carta {} di {}", "Número {} · carta {} de {}")
            .avec(&[&numero, &(partie.jeu.played() + 1).min(CARTES), &CARTES]);
        let record = progres.record(numero).map(|r| format!("  ·  {}", tx!("Record : {}", "Best: {}", "Rekord: {}", "Record: {}", "Récord: {}").avec(&[&r]))).unwrap_or_default();
        let gain = partie.gain.map(|g| format!("  ·  {g:+}")).unwrap_or_default();
        let ligne2 = format!("{}{}{}", tx!("Score : {}", "Score: {}", "Punkte: {}", "Punteggio: {}", "Puntos: {}").avec(&[&s.total()]), gain, record);
        let ligne3 = tx!(
            "Raccords {} · voie ferrée {} · route {} · passages {} · vides -{}",
            "Links {} · railway {} · road {} · crossings {} · open ends -{}",
            "Anschlüsse {} · Bahn {} · Straße {} · Übergänge {} · offen -{}",
            "Collegamenti {} · ferrovia {} · strada {} · passaggi {} · vuoti -{}",
            "Uniones {} · vía {} · carretera {} · pasos {} · vacíos -{}"
        )
        .avec(&[&s.links, &s.longest_rail, &s.longest_route, &s.crossings, &s.open_ends]);
        t.0 = format!("{ligne1}\n{ligne2}\n{ligne3}");
    }
}

fn spawn_fin(mut c: Commands, partie: Res<Partie>, progres: Res<Progres>) {
    let s = partie.jeu.score();
    let panel = spawn_panel_left(&mut c, DespawnOnExit(GameState::Fini));
    spawn_text(&mut c, panel, tx!("Terminus !", "End of the line!", "Endstation!", "Capolinea!", "¡Fin de trayecto!").t(), 40.0);
    spawn_text(&mut c, panel, tx!("Numéro {}", "Number {}", "Nummer {}", "Numero {}", "Número {}").avec(&[&partie.jeu.number]), 22.0);
    spawn_text(&mut c, panel, tx!("Score : {}", "Score: {}", "Punkte: {}", "Punteggio: {}", "Puntos: {}").avec(&[&s.total()]), 32.0);
    spawn_text(&mut c, panel, tx!("Étoiles : {} sur 3", "Stars: {} out of 3", "Sterne: {} von 3", "Stelle: {} su 3", "Estrellas: {} de 3").avec(&[&s.stars()]), 20.0);
    if partie.nouveau_record {
        spawn_text(&mut c, panel, tx!("Nouveau record !", "New best!", "Neuer Rekord!", "Nuovo record!", "¡Nuevo récord!").t(), 24.0);
    } else if let Some(r) = progres.record(partie.jeu.number) {
        spawn_text(&mut c, panel, tx!("Record : {}", "Best: {}", "Rekord: {}", "Record: {}", "Récord: {}").avec(&[&r]), 20.0);
    }
    spawn_text(
        &mut c,
        panel,
        tx!(
            "Raccords {} · voie ferrée {} · route {} · passages à niveau {} · bouts dans le vide -{}",
            "Links {} · railway {} · road {} · level crossings {} · open ends -{}",
            "Anschlüsse {} · Bahn {} · Straße {} · Bahnübergänge {} · offene Enden -{}",
            "Collegamenti {} · ferrovia {} · strada {} · passaggi a livello {} · estremità vuote -{}",
            "Uniones {} · vía {} · carretera {} · pasos a nivel {} · extremos vacíos -{}"
        )
        .avec(&[&s.links, &s.longest_rail, &s.longest_route, &s.crossings, &s.open_ends]),
        15.0,
    );
    spawn_gap(&mut c, panel, 4.0);
    bouton(&mut c, panel, tx!("Rejouer ce numéro", "Replay this number", "Diese Nummer nochmal", "Rigioca questo numero", "Repetir este número").t(), Action::Rejouer);
    bouton(&mut c, panel, tx!("Choisir un autre numéro", "Pick another number", "Andere Nummer wählen", "Scegli un altro numero", "Elegir otro número").t(), Action::Menu);
}

fn demarrer(c: &mut Commands, progres: &mut Progres, next: &mut NextState<GameState>, numero: u64) {
    commencer(c, progres, numero);
    next.set(GameState::Playing);
}

/// Exécute une action (bouton ou raccourci clavier).
fn agir(action: Action, c: &mut Commands, progres: &mut Progres, partie: &mut Partie, state: GameState, next: &mut NextState<GameState>, evenements: &mut MessageWriter<JeuEvenement>) {
    match action {
        Action::Jouer => {
            // Une partie du même numéro déjà commencée se reprend.
            if partie.jeu.number == progres.numero && !partie.jeu.is_finished() && partie.jeu.played() > 0 {
                next.set(GameState::Playing);
            } else {
                demarrer(c, progres, next, progres.numero);
            }
        }
        Action::Rejouer => demarrer(c, progres, next, partie.jeu.number),
        Action::Ajouter(d) => {
            let n = (progres.numero as i64 + d).clamp(0, NUMERO_MAX as i64) as u64;
            progres.choisir(n);
        }
        Action::Jour => {
            let d = Date::today();
            progres.choisir(day_number(d.year as u32, d.month, d.day));
        }
        Action::Hasard => progres.choisir(game_common::Rng::from_time().below(1_000_000) as u64),
        Action::Tourner if state == GameState::Playing => {
            partie.tourner();
            evenements.write(JeuEvenement::Tourne);
        }
        Action::Annuler if state == GameState::Playing => partie.annuler(),
        Action::Menu => next.set(GameState::Menu),
        _ => {}
    }
}

fn au_clic(
    clic: On<PointerClick>,
    actions: Query<&Action>,
    mut c: Commands,
    mut progres: ResMut<Progres>,
    mut partie: ResMut<Partie>,
    state: Res<State<GameState>>,
    mut next: ResMut<NextState<GameState>>,
    mut evenements: MessageWriter<JeuEvenement>,
) {
    if let Ok(&action) = actions.get(clic.entity) {
        agir(action, &mut c, &mut progres, &mut partie, *state.get(), &mut next, &mut evenements);
    }
}

const CHIFFRES: [(KeyCode, KeyCode, u64); 10] = [
    (KeyCode::Digit0, KeyCode::Numpad0, 0),
    (KeyCode::Digit1, KeyCode::Numpad1, 1),
    (KeyCode::Digit2, KeyCode::Numpad2, 2),
    (KeyCode::Digit3, KeyCode::Numpad3, 3),
    (KeyCode::Digit4, KeyCode::Numpad4, 4),
    (KeyCode::Digit5, KeyCode::Numpad5, 5),
    (KeyCode::Digit6, KeyCode::Numpad6, 6),
    (KeyCode::Digit7, KeyCode::Numpad7, 7),
    (KeyCode::Digit8, KeyCode::Numpad8, 8),
    (KeyCode::Digit9, KeyCode::Numpad9, 9),
];

fn clavier(
    keys: Res<ButtonInput<KeyCode>>,
    mut c: Commands,
    mut progres: ResMut<Progres>,
    mut partie: ResMut<Partie>,
    state: Res<State<GameState>>,
    mut next: ResMut<NextState<GameState>>,
    mut evenements: MessageWriter<JeuEvenement>,
) {
    let action = match state.get() {
        GameState::Menu => {
            // On tape le numéro au clavier, chiffre après chiffre.
            for &(a, b, v) in &CHIFFRES {
                if keys.any_just_pressed([a, b]) {
                    let n = progres.numero * 10 + v;
                    if n <= NUMERO_MAX {
                        progres.choisir(n);
                    }
                }
            }
            if keys.just_pressed(KeyCode::Backspace) {
                let n = progres.numero / 10;
                progres.choisir(n);
            }
            if keys.any_just_pressed([KeyCode::Enter, KeyCode::NumpadEnter, KeyCode::Space]) { Action::Jouer } else { return }
        }
        GameState::Fini if keys.any_just_pressed([KeyCode::Enter, KeyCode::Space]) => Action::Rejouer,
        GameState::Fini if keys.just_pressed(KeyCode::Escape) => Action::Menu,
        GameState::Playing if keys.just_pressed(KeyCode::Escape) => Action::Menu,
        GameState::Playing if keys.just_pressed(KeyCode::Backspace) || (keys.just_pressed(KeyCode::KeyZ) && keys.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight, KeyCode::SuperLeft, KeyCode::SuperRight])) => Action::Annuler,
        _ => return,
    };
    agir(action, &mut c, &mut progres, &mut partie, *state.get(), &mut next, &mut evenements);
}
