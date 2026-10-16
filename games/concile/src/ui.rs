//! Écrans : accueil, Livre des Conciles, barre de jeu, victoire.
use bevy::prelude::*;
use concile_core::daily;
use game_common::calendar::Date;
use game_common::langue::{Langue, actuelle};
use game_common::tx;
use game_common::ui::{
    ButtonColor, GameHelp, PANEL, ThemedPanel, UiButton, pied_de_menu, spawn_button_sized, spawn_gap, spawn_panel, spawn_panel_left,
    spawn_text,
};
use game_common::universe::Universe;

use crate::GameState;
use crate::audio::UiClick;
use crate::play::{Book, Kind, Play, PlayEvent, Progress, start};

/// Aide du jeu (bouton « Aide » du menu).
const HELP: game_common::langue::Texte = tx!(
    "Placez un évêque par diocèse (région de couleur),\n\
par rangée et par colonne. Deux évêques ne se\n\
touchent jamais, même en diagonale.\n\
\n\
Clic : croix, puis évêque, puis case vide\n\
Glisser : barrer plusieurs cases\n\
Mode Novice : poser un évêque hachure d'office\n\
son diocèse, sa rangée, sa colonne et ses voisines.\n\
\n\
H : indice · U ou Retour arrière : annuler\n\
Échap : menu",
    "Place one bishop in each diocese (coloured region),\n\
each row and each column. Two bishops never\n\
touch, not even diagonally.\n\
\n\
Click: cross, then bishop, then empty square\n\
Drag: cross out several squares\n\
Novice mode: placing a bishop automatically shades\n\
its diocese, row, column and neighbouring squares.\n\
\n\
H: hint · U or Backspace: undo\n\
Esc: menu",
    "Setze einen Bischof in jede Diözese (farbige Region),\n\
jede Zeile und jede Spalte. Zwei Bischöfe berühren\n\
sich nie, auch nicht diagonal.\n\
\n\
Klick: Kreuz, dann Bischof, dann leeres Feld\n\
Ziehen: mehrere Felder streichen\n\
Anfängermodus: Ein gesetzter Bischof schraffiert\n\
sofort seine Diözese, Zeile, Spalte und Nachbarfelder.\n\
\n\
H: Tipp · U oder Rücktaste: rückgängig\n\
Esc: Menü",
    "Metti un vescovo per diocesi (regione colorata),\n\
per riga e per colonna. Due vescovi non si\n\
toccano mai, nemmeno in diagonale.\n\
\n\
Clic: croce, poi vescovo, poi casella vuota\n\
Trascina: barra più caselle\n\
Modalità Novizio: mettere un vescovo tratteggia\n\
subito la sua diocesi, riga, colonna e caselle vicine.\n\
\n\
H: indizio · U o Backspace: annulla\n\
Esc: menu",
    "Coloca un obispo por diócesis (región de color),\n\
por fila y por columna. Dos obispos nunca se\n\
tocan, ni siquiera en diagonal.\n\
\n\
Clic: cruz, luego obispo, luego casilla vacía\n\
Arrastrar: tachar varias casillas\n\
Modo Novato: al colocar un obispo se sombrean\n\
su diócesis, su fila, su columna y sus vecinas.\n\
\n\
H: pista · U o Retroceso: deshacer\n\
Esc: menú",
);

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameHelp(HELP))
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
    Undo,
    Clear,
    Hint,
    Next,
    Menu,
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
}

fn button(c: &mut Commands, parent: Entity, label: impl Into<String>, action: UiAction) -> Entity {
    let (b, text) = spawn_button_sized(c, parent, label, 340.0, 42.0, 20.0);
    c.entity(b).insert(action);
    text
}

fn mode_text(novice: bool) -> &'static str {
    if novice {
        tx!("Mode : Novice (aidé)", "Mode: Novice (assisted)", "Modus: Anfänger (mit Hilfe)", "Modalità: Novizio (aiutato)", "Modo: Novato (con ayuda)").t()
    } else {
        tx!("Mode : Normal", "Mode: Normal", "Modus: Normal", "Modalità: Normale", "Modo: Normal").t()
    }
}

fn feasts_text(on: bool) -> &'static str {
    if on {
        tx!("Fêtes : affichées", "Feasts: shown", "Feste: angezeigt", "Feste: mostrate", "Fiestas: visibles").t()
    } else {
        tx!("Fêtes : masquées", "Feasts: hidden", "Feste: ausgeblendet", "Feste: nascoste", "Fiestas: ocultas").t()
    }
}

/// La date dans la langue en cours : « vendredi 16 octobre 2026 »,
/// « Friday 16 October 2026 », « Freitag, 16. Oktober 2026 »…
fn date_text(d: Date) -> String {
    let wd = d.weekday();
    let m = d.month as usize - 1;
    let (y, n) = (d.year, d.day);
    match actuelle() {
        Langue::Fr => d.french(),
        Langue::En => {
            const DAYS: [&str; 7] = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"];
            const MONTHS: [&str; 12] =
                ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
            format!("{} {n} {} {y}", DAYS[wd], MONTHS[m])
        }
        Langue::De => {
            const DAYS: [&str; 7] = ["Montag", "Dienstag", "Mittwoch", "Donnerstag", "Freitag", "Samstag", "Sonntag"];
            const MONTHS: [&str; 12] =
                ["Januar", "Februar", "März", "April", "Mai", "Juni", "Juli", "August", "September", "Oktober", "November", "Dezember"];
            format!("{}, {n}. {} {y}", DAYS[wd], MONTHS[m])
        }
        Langue::It => {
            const DAYS: [&str; 7] = ["lunedì", "martedì", "mercoledì", "giovedì", "venerdì", "sabato", "domenica"];
            const MONTHS: [&str; 12] = [
                "gennaio", "febbraio", "marzo", "aprile", "maggio", "giugno", "luglio", "agosto", "settembre", "ottobre", "novembre", "dicembre",
            ];
            let n = if n == 1 { "1º".to_string() } else { n.to_string() };
            format!("{} {n} {} {y}", DAYS[wd], MONTHS[m])
        }
        Langue::Es => {
            const DAYS: [&str; 7] = ["lunes", "martes", "miércoles", "jueves", "viernes", "sábado", "domingo"];
            const MONTHS: [&str; 12] = [
                "enero", "febrero", "marzo", "abril", "mayo", "junio", "julio", "agosto", "septiembre", "octubre", "noviembre", "diciembre",
            ];
            format!("{}, {n} de {} de {y}", DAYS[wd], MONTHS[m])
        }
    }
}

fn spawn_menu(mut c: Commands, progress: Res<Progress>, book: Res<Book>) {
    let panel = spawn_panel_left(&mut c, DespawnOnExit(GameState::Menu));
    // Panneau plus serré : il y a beaucoup de choix.
    c.entity(panel).insert(Node {
        flex_direction: FlexDirection::Column,
        align_items: AlignItems::Center,
        row_gap: Val::Px(6.0),
        padding: UiRect::axes(Val::Px(28.0), Val::Px(14.0)),
        border: UiRect::all(Val::Px(2.0)),
        border_radius: BorderRadius::all(Val::Px(24.0)),
        ..default()
    });
    spawn_text(
        &mut c,
        panel,
        tx!("Le Concile des Évêques", "The Council of Bishops", "Das Konzil der Bischöfe", "Il Concilio dei Vescovi", "El Concilio de los Obispos").t(),
        36.0,
    );
    spawn_text(&mut c, panel, tx!("un jeu Eurêka", "a Eurêka game", "ein Eurêka-Spiel", "un gioco Eurêka", "un juego Eurêka").t(), 15.0);
    let today = Date::today();
    let streak = if progress.streak > 1 {
        tx!(" · série : {} jours", " · streak: {} days", " · Serie: {} Tage", " · serie: {} giorni", " · racha: {} días").avec(&[&progress.streak])
    } else {
        String::new()
    };
    spawn_text(&mut c, panel, format!("{}{streak}", capitalize(&date_text(today))), 15.0);
    if progress.feasts
        && let Some(f) = game_common::ambiance::fete_du_jour()
    {
        let t = spawn_text(&mut c, panel, f.greeting(), 18.0);
        c.entity(t).insert(TextColor(Universe::Eureka.style().accent));
    }
    spawn_gap(&mut c, panel, 2.0);
    let done = progress.solved.iter().filter(|&&s| s).count();
    let next = progress.next_unsolved().map(|i| i + 1).unwrap_or(book.0.len());
    button(&mut c, panel, tx!("Continuer : n° {}", "Continue: no. {}", "Weiter: Nr. {}", "Continua: n. {}", "Continuar: n.º {}").avec(&[&next]), UiAction::Continue);
    button(
        &mut c,
        panel,
        tx!("Livre des Conciles · {}/{}", "Book of Councils · {}/{}", "Buch der Konzile · {}/{}", "Libro dei Concili · {}/{}", "Libro de los Concilios · {}/{}")
            .avec(&[&done, &book.0.len()]),
        UiAction::Book,
    );
    let daily = if progress.daily_solved_today() {
        tx!("Défi du jour : réussi !", "Daily challenge: solved!", "Tagesrätsel: gelöst!", "Sfida del giorno: risolta!", "Reto del día: ¡resuelto!")
    } else {
        tx!("Défi du jour", "Daily challenge", "Tagesrätsel", "Sfida del giorno", "Reto del día")
    };
    button(&mut c, panel, daily.t(), UiAction::Daily);
    let t = button(&mut c, panel, mode_text(progress.novice), UiAction::ToggleNovice);
    c.entity(t).insert(Label::Novice);
    let t = button(&mut c, panel, feasts_text(progress.feasts), UiAction::ToggleFeasts);
    c.entity(t).insert(Label::Feasts);
    pied_de_menu(&mut c, panel);
}

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    c.next().map(|f| f.to_uppercase().collect::<String>() + c.as_str()).unwrap_or_default()
}

/// Le Livre : une rangée de grilles par taille.
fn spawn_book(mut c: Commands, book: Res<Book>, progress: Res<Progress>) {
    let panel = spawn_panel(&mut c, DespawnOnExit(GameState::Book));
    spawn_text(
        &mut c,
        panel,
        tx!("Le Livre des Conciles", "The Book of Councils", "Das Buch der Konzile", "Il Libro dei Concili", "El Libro de los Concilios").t(),
        38.0,
    );
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
    button(&mut c, panel, tx!("Retour", "Back", "Zurück", "Indietro", "Volver").t(), UiAction::Menu);
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
    for (label, action) in [
        (tx!("Annuler", "Undo", "Rückgängig", "Annulla", "Deshacer"), UiAction::Undo),
        (tx!("Effacer", "Clear", "Löschen", "Cancella", "Borrar"), UiAction::Clear),
        (tx!("Indice", "Hint", "Tipp", "Indizio", "Pista"), UiAction::Hint),
        (tx!("Menu", "Menu", "Menü", "Menu", "Menú"), UiAction::Menu),
    ] {
        let (b, _) = spawn_button_sized(&mut c, buttons, label.t(), 128.0, 42.0, 18.0);
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
        Kind::Book(i) => tx!("Concile n° {}", "Council no. {}", "Konzil Nr. {}", "Concilio n. {}", "Concilio n.º {}").avec(&[&(i + 1)]),
        Kind::Daily(d) => tx!("Défi du {}", "Challenge of {}", "Rätsel vom {}", "Sfida di {}", "Reto del {}").avec(&[&date_text(d)]),
    };
    let mode = if progress.novice {
        tx!("Novice", "Novice", "Anfänger", "Novizio", "Novato")
    } else {
        tx!("Normal", "Normal", "Normal", "Normale", "Normal")
    };
    for mut t in &mut info {
        let s = tx!(
            "{} · {} × {} · {}\nSablier : {}   Évêques : {}/{}",
            "{} · {} × {} · {}\nHourglass: {}   Bishops: {}/{}",
            "{} · {} × {} · {}\nSanduhr: {}   Bischöfe: {}/{}",
            "{} · {} × {} · {}\nClessidra: {}   Vescovi: {}/{}",
            "{} · {} × {} · {}\nReloj de arena: {}   Obispos: {}/{}",
        )
        .avec(&[&title, &n, &n, &mode.t(), &format_time(play.time), &play.bishops().len(), &n]);
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
    spawn_text(
        &mut c,
        panel,
        tx!(
            "Le concile\npeut commencer !",
            "The council\ncan begin!",
            "Das Konzil\nkann beginnen!",
            "Il concilio\npuò cominciare!",
            "¡El concilio\npuede comenzar!",
        )
        .t(),
        44.0,
    );
    let line = match play.kind {
        Kind::Book(i) => {
            let best = progress.best[i].map(|b| format_time(b as f32)).unwrap_or_default();
            tx!(
                "Concile n° {} réussi en {}\nMeilleur temps : {}",
                "Council no. {} solved in {}\nBest time: {}",
                "Konzil Nr. {} gelöst in {}\nBestzeit: {}",
                "Concilio n. {} risolto in {}\nTempo migliore: {}",
                "Concilio n.º {} resuelto en {}\nMejor tiempo: {}",
            )
            .avec(&[&(i + 1), &format_time(play.time), &best])
        }
        Kind::Daily(_) => {
            let streak = if progress.streak > 1 {
                tx!(
                    "\nSérie : {} jours de suite !",
                    "\nStreak: {} days in a row!",
                    "\nSerie: {} Tage in Folge!",
                    "\nSerie: {} giorni di fila!",
                    "\nRacha: ¡{} días seguidos!",
                )
                .avec(&[&progress.streak])
            } else {
                String::new()
            };
            tx!(
                "Défi du jour réussi en {}",
                "Daily challenge solved in {}",
                "Tagesrätsel gelöst in {}",
                "Sfida del giorno risolta in {}",
                "Reto del día resuelto en {}",
            )
            .avec(&[&format_time(play.time)])
                + &streak
        }
    };
    spawn_text(&mut c, panel, line, 20.0);
    spawn_gap(&mut c, panel, 4.0);
    if matches!(play.kind, Kind::Book(i) if i + 1 < book.0.len()) {
        button(&mut c, panel, tx!("Concile suivant", "Next council", "Nächstes Konzil", "Concilio successivo", "Siguiente concilio").t(), UiAction::Next);
    }
    button(&mut c, panel, tx!("Menu", "Menu", "Menü", "Menu", "Menú").t(), UiAction::Menu);
}

fn update_labels(progress: Res<Progress>, mut q: Query<(&Label, &mut Text)>) {
    if !progress.is_changed() {
        return;
    }
    for (label, mut t) in &mut q {
        t.0 = match label {
            Label::Novice => mode_text(progress.novice),
            Label::Feasts => feasts_text(progress.feasts),
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
    mut clicks: MessageWriter<UiClick>,
    mut events: MessageWriter<PlayEvent>,
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
    }
}
