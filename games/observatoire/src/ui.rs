//! Les écrans : menu, Livre des Constellations, barre de jeu (sablier,
//! indice), victoire. Tous les textes en cinq langues avec `tx!`.
use bevy::prelude::*;
use game_common::calendar::Date;
use game_common::langue::Texte;
use game_common::tx;
use game_common::ui::{ButtonColor, GameHelp, PANEL, ThemedPanel, pied_de_menu, spawn_button_sized, spawn_gap, spawn_panel, spawn_panel_left, spawn_text};
use observatoire_core::du_jour;

use crate::GameState;
use crate::jeu::{Genre, JeuEvenement, Livre, Partie, Progression, commencer};

/// Aide du jeu (bouton « Aide » du pied de menu).
const AIDE: Texte = tx!(
    "Remplissez la carte du ciel de soleils et de lunes.\n\n\
     1. Autant de soleils que de lunes dans chaque rangée et chaque colonne.\n\
     2. Jamais plus de deux symboles pareils côte à côte.\n\
     3. « = » : les deux cases sont pareilles ; « × » : elles sont différentes.\n\n\
     Clic : vide, soleil, lune · clic droit : dans l'autre sens\n\
     Ctrl+Z : annuler · H : indice · Échap : menu\n\n\
     Mode Novice : les erreurs sont signalées tout de suite.",
    "Fill the star chart with suns and moons.\n\n\
     1. As many suns as moons in every row and every column.\n\
     2. Never more than two identical symbols side by side.\n\
     3. “=”: both cells are the same; “×”: they are different.\n\n\
     Click: empty, sun, moon · right click: the other way\n\
     Ctrl+Z: undo · H: hint · Esc: menu\n\n\
     Novice mode: mistakes are shown right away.",
    "Fülle die Sternkarte mit Sonnen und Monden.\n\n\
     1. Gleich viele Sonnen wie Monde in jeder Zeile und Spalte.\n\
     2. Nie mehr als zwei gleiche Symbole nebeneinander.\n\
     3. „=“: beide Felder sind gleich; „ד: sie sind verschieden.\n\n\
     Klick: leer, Sonne, Mond · Rechtsklick: andersherum\n\
     Strg+Z: rückgängig · H: Tipp · Esc: Menü\n\n\
     Modus Anfänger: Fehler werden sofort angezeigt.",
    "Riempi la carta del cielo di soli e di lune.\n\n\
     1. Tanti soli quante lune in ogni riga e ogni colonna.\n\
     2. Mai più di due simboli uguali vicini.\n\
     3. «=»: le due caselle sono uguali; «×»: sono diverse.\n\n\
     Clic: vuoto, sole, luna · clic destro: al contrario\n\
     Ctrl+Z: annulla · H: indizio · Esc: menu\n\n\
     Modalità Principiante: gli errori sono segnalati subito.",
    "Llena la carta celeste de soles y lunas.\n\n\
     1. Tantos soles como lunas en cada fila y cada columna.\n\
     2. Nunca más de dos símbolos iguales seguidos.\n\
     3. «=»: las dos casillas son iguales; «×»: son distintas.\n\n\
     Clic: vacío, sol, luna · clic derecho: al revés\n\
     Ctrl+Z: deshacer · H: pista · Esc: menú\n\n\
     Modo Novato: los errores se señalan enseguida.",
);

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameHelp(AIDE))
            .add_systems(OnEnter(GameState::Menu), spawn_menu)
            .add_systems(OnEnter(GameState::Livre), spawn_livre)
            .add_systems(OnEnter(GameState::Playing), spawn_barre)
            .add_systems(OnEnter(GameState::Gagne), spawn_gagne)
            .add_systems(Update, (clavier, maj_barre))
            .add_observer(au_clic);
    }
}

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Continuer,
    Livre,
    Jour,
    Mode,
    Choisir(usize),
    Suivante,
    Menu,
    Annuler,
    Effacer,
    Indice,
}

#[derive(Component)]
struct TexteInfo;

#[derive(Component)]
struct BoiteIndice;

#[derive(Component)]
struct TexteIndice;

fn bouton(c: &mut Commands, parent: Entity, label: &str, action: Action) -> Entity {
    let (b, t) = spawn_button_sized(c, parent, label, 340.0, 40.0, 19.0);
    c.entity(b).insert(action);
    t
}

pub fn temps(secs: f32) -> String {
    let s = secs as u32;
    format!("{}:{:02}", s / 60, s % 60)
}

fn mode(novice: bool) -> &'static str {
    if novice {
        tx!("Mode : Novice", "Mode: Novice", "Modus: Anfänger", "Modalità: Principiante", "Modo: Novato").t()
    } else {
        tx!("Mode : Normal", "Mode: Normal", "Modus: Normal", "Modalità: Normale", "Modo: Normal").t()
    }
}

fn spawn_menu(mut c: Commands, livre: Res<Livre>, progression: Res<Progression>, partie: Option<Res<Partie>>) {
    let panel = spawn_panel_left(&mut c, DespawnOnExit(GameState::Menu));
    spawn_text(&mut c, panel, tx!("L'OBSERVATOIRE", "THE OBSERVATORY", "DIE STERNWARTE", "L'OSSERVATORIO", "EL OBSERVATORIO").t(), 42.0);
    spawn_text(&mut c, panel, tx!("Soleils et lunes", "Suns and moons", "Sonnen und Monde", "Soli e lune", "Soles y lunas").t(), 22.0);
    let faites = progression.resolues.iter().filter(|r| **r).count();
    spawn_text(
        &mut c,
        panel,
        tx!("{} constellations sur {}", "{} constellations out of {}", "{} von {} Sternbildern", "{} costellazioni su {}", "{} constelaciones de {}").avec(&[&faites, &livre.0.len()]),
        16.0,
    );
    let en_cours = partie.as_ref().is_some_and(|p| !p.gagne && p.cases != p.grille.donnees);
    let jouer = if en_cours {
        tx!("Continuer", "Continue", "Weiter", "Continua", "Continuar").t()
    } else {
        tx!("Jouer", "Play", "Spielen", "Gioca", "Jugar").t()
    };
    bouton(&mut c, panel, jouer, Action::Continuer);
    bouton(&mut c, panel, tx!("Livre des Constellations", "Book of Constellations", "Buch der Sternbilder", "Libro delle Costellazioni", "Libro de las Constelaciones").t(), Action::Livre);
    let jour = if progression.jour_reussi() {
        tx!("Grille du jour : réussie", "Daily grid: solved", "Tagesgitter: gelöst", "Griglia del giorno: risolta", "Cuadrícula del día: resuelta").t()
    } else {
        tx!("Grille du jour", "Daily grid", "Tagesgitter", "Griglia del giorno", "Cuadrícula del día").t()
    };
    bouton(&mut c, panel, jour, Action::Jour);
    if progression.serie > 1 && progression.jour_fait.is_some_and(|d| d.plus(1) >= Date::today()) {
        spawn_text(&mut c, panel, tx!("Série : {} jours", "Streak: {} days", "Serie: {} Tage", "Serie: {} giorni", "Racha: {} días").avec(&[&progression.serie]), 16.0);
    }
    bouton(&mut c, panel, mode(progression.novice), Action::Mode);
    // Les boutons du jeu d'abord, puis le pied commun : Options, Aide, Quitter, logo.
    pied_de_menu(&mut c, panel);
}

/// Le Livre : les grilles par taille, dix par ligne.
fn spawn_livre(mut c: Commands, livre: Res<Livre>, progression: Res<Progression>) {
    let panel = spawn_panel(&mut c, DespawnOnExit(GameState::Livre));
    spawn_text(&mut c, panel, tx!("Le Livre des Constellations", "The Book of Constellations", "Das Buch der Sternbilder", "Il Libro delle Costellazioni", "El Libro de las Constelaciones").t(), 34.0);
    let mut i = 0;
    while i < livre.0.len() {
        let n = livre.0[i].n;
        let rangee = c.spawn((Node { column_gap: Val::Px(10.0), align_items: AlignItems::FlexStart, ..default() }, ChildOf(panel))).id();
        let etiquette = c.spawn((Node { width: Val::Px(70.0), padding: UiRect::top(Val::Px(6.0)), ..default() }, ChildOf(rangee))).id();
        spawn_text(&mut c, etiquette, format!("{n} × {n}"), 17.0);
        let grille = c.spawn((Node { width: Val::Px(10.0 * 48.0), flex_wrap: FlexWrap::Wrap, column_gap: Val::Px(4.0), row_gap: Val::Px(4.0), ..default() }, ChildOf(rangee))).id();
        while i < livre.0.len() && livre.0[i].n == n {
            let (b, t) = spawn_button_sized(&mut c, grille, format!("{}", i + 1), 44.0, 32.0, 15.0);
            c.entity(b).insert(Action::Choisir(i));
            if progression.resolues[i] {
                // Constellation complète : doré sur fond crème.
                c.entity(t).insert(TextColor(Color::srgb(0.55, 0.38, 0.05)));
                c.entity(b).insert(ButtonColor(Color::srgb(0.98, 0.88, 0.55)));
            }
            i += 1;
        }
    }
    spawn_gap(&mut c, panel, 2.0);
    bouton(&mut c, panel, tx!("Retour", "Back", "Zurück", "Indietro", "Volver").t(), Action::Menu);
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
        (tx!("Annuler", "Undo", "Rückgängig", "Annulla", "Deshacer"), Action::Annuler),
        (tx!("Effacer", "Clear", "Löschen", "Cancella", "Borrar"), Action::Effacer),
        (tx!("Indice", "Hint", "Tipp", "Indizio", "Pista"), Action::Indice),
        (tx!("Menu", "Menu", "Menü", "Menu", "Menú"), Action::Menu),
    ] {
        let (b, _) = spawn_button_sized(&mut c, boutons, label.t(), 118.0, 42.0, 18.0);
        c.entity(b).insert(action);
    }
    // L'explication de l'indice, en bas de l'écran.
    let pied = c
        .spawn((
            Node { position_type: PositionType::Absolute, bottom: Val::Px(18.0), width: Val::Percent(100.0), justify_content: JustifyContent::Center, ..default() },
            Pickable::IGNORE,
            DespawnOnExit(GameState::Playing),
        ))
        .id();
    let carte = c
        .spawn((
            Node { max_width: Val::Px(660.0), padding: UiRect::axes(Val::Px(18.0), Val::Px(10.0)), border: UiRect::all(Val::Px(2.0)), border_radius: BorderRadius::all(Val::Px(14.0)), ..default() },
            BackgroundColor(PANEL),
            ThemedPanel,
            Visibility::Hidden,
            BoiteIndice,
            ChildOf(pied),
        ))
        .id();
    let t = spawn_text(&mut c, carte, "", 18.0);
    c.entity(t).insert(TexteIndice);
}

fn titre_grille(genre: Genre, n: usize) -> String {
    match genre {
        Genre::Livre(i) => tx!("Constellation {} · {} × {}", "Constellation {} · {} × {}", "Sternbild {} · {} × {}", "Costellazione {} · {} × {}", "Constelación {} · {} × {}").avec(&[&(i + 1), &n, &n]),
        Genre::Jour(_) => tx!("Grille du jour · {} × {}", "Daily grid · {} × {}", "Tagesgitter · {} × {}", "Griglia del giorno · {} × {}", "Cuadrícula del día · {} × {}").avec(&[&n, &n]),
    }
}

fn maj_barre(
    partie: Option<Res<Partie>>,
    progression: Res<Progression>,
    mut info: Query<&mut Text, (With<TexteInfo>, Without<TexteIndice>)>,
    mut boite: Query<&mut Visibility, With<BoiteIndice>>,
    mut indice: Query<&mut Text, (With<TexteIndice>, Without<TexteInfo>)>,
) {
    let Some(partie) = partie else { return };
    for mut t in &mut info {
        let ligne2 = tx!("Temps : {}", "Time: {}", "Zeit: {}", "Tempo: {}", "Tiempo: {}").avec(&[&temps(partie.temps)]);
        t.0 = format!("{}\n{}  ·  {}", titre_grille(partie.genre, partie.n()), ligne2, mode(progression.novice));
    }
    for mut v in &mut boite {
        *v = if partie.indice.is_some() { Visibility::Inherited } else { Visibility::Hidden };
    }
    if let Some(ind) = &partie.indice {
        for mut t in &mut indice {
            if t.0 != ind.texte {
                t.0 = ind.texte.clone();
            }
        }
    }
}

fn spawn_gagne(mut c: Commands, partie: Res<Partie>, progression: Res<Progression>) {
    let panel = spawn_panel_left(&mut c, DespawnOnExit(GameState::Gagne));
    spawn_text(&mut c, panel, tx!("Constellation complète !", "Constellation complete!", "Sternbild vollständig!", "Costellazione completa!", "¡Constelación completa!").t(), 34.0);
    spawn_text(&mut c, panel, titre_grille(partie.genre, partie.n()), 18.0);
    spawn_text(&mut c, panel, tx!("Temps : {}", "Time: {}", "Zeit: {}", "Tempo: {}", "Tiempo: {}").avec(&[&temps(partie.temps)]), 22.0);
    match partie.genre {
        Genre::Livre(i) => {
            if let Some(b) = progression.meilleurs[i] {
                spawn_text(&mut c, panel, tx!("Meilleur temps : {}", "Best time: {}", "Bestzeit: {}", "Tempo migliore: {}", "Mejor tiempo: {}").avec(&[&temps(b as f32)]), 18.0);
            }
        }
        Genre::Jour(_) => {
            spawn_text(&mut c, panel, tx!("Série : {} jours", "Streak: {} days", "Serie: {} Tage", "Serie: {} giorni", "Racha: {} días").avec(&[&progression.serie]), 18.0);
        }
    }
    spawn_gap(&mut c, panel, 4.0);
    if progression.suivante().is_some() {
        bouton(&mut c, panel, tx!("Constellation suivante", "Next constellation", "Nächstes Sternbild", "Costellazione successiva", "Siguiente constelación").t(), Action::Suivante);
    }
    bouton(&mut c, panel, tx!("Livre des Constellations", "Book of Constellations", "Buch der Sternbilder", "Libro delle Costellazioni", "Libro de las Constelaciones").t(), Action::Livre);
    bouton(&mut c, panel, tx!("Menu", "Menu", "Menü", "Menu", "Menú").t(), Action::Menu);
}

/// Exécute une action (bouton ou raccourci clavier).
#[allow(clippy::too_many_arguments)]
fn agir(
    action: Action,
    c: &mut Commands,
    livre: &Livre,
    progression: &mut Progression,
    partie: Option<&mut Partie>,
    state: GameState,
    next: &mut NextState<GameState>,
    evenements: &mut MessageWriter<JeuEvenement>,
) {
    let suivante = |c: &mut Commands, progression: &Progression| {
        let i = progression.suivante().unwrap_or(0);
        commencer(c, livre.0[i].clone(), Genre::Livre(i));
    };
    match action {
        Action::Continuer => {
            if partie.is_none_or(|p| p.gagne) {
                suivante(c, progression);
            }
            next.set(GameState::Playing);
        }
        Action::Suivante => {
            suivante(c, progression);
            next.set(GameState::Playing);
        }
        Action::Livre => next.set(GameState::Livre),
        Action::Choisir(i) => {
            commencer(c, livre.0[i].clone(), Genre::Livre(i));
            next.set(GameState::Playing);
        }
        Action::Jour => {
            let d = Date::today();
            let en_cours = partie.as_ref().is_some_and(|p| p.genre == Genre::Jour(d) && !p.gagne);
            if !en_cours {
                commencer(c, du_jour(d.year, d.month, d.day), Genre::Jour(d));
            }
            next.set(GameState::Playing);
        }
        Action::Mode => {
            progression.novice = !progression.novice;
            progression.enregistrer();
            if let Some(p) = partie {
                p.redessiner();
            }
            // Le menu est rebâti avec le nouveau libellé.
            next.set(GameState::Menu);
        }
        Action::Menu => next.set(GameState::Menu),
        Action::Annuler | Action::Effacer | Action::Indice if state == GameState::Playing => {
            let Some(p) = partie else { return };
            match action {
                Action::Annuler => p.annuler(),
                Action::Effacer => {
                    p.effacer();
                    evenements.write(JeuEvenement::Efface);
                }
                _ => {
                    p.calculer_indice();
                    if p.indice.is_some() {
                        evenements.write(JeuEvenement::Indice);
                    }
                }
            }
        }
        _ => {}
    }
}

#[allow(clippy::too_many_arguments)]
fn au_clic(
    clic: On<PointerClick>,
    actions: Query<&Action>,
    mut c: Commands,
    livre: Res<Livre>,
    mut progression: ResMut<Progression>,
    mut partie: Option<ResMut<Partie>>,
    state: Res<State<GameState>>,
    mut next: ResMut<NextState<GameState>>,
    mut evenements: MessageWriter<JeuEvenement>,
) {
    if let Ok(&action) = actions.get(clic.entity) {
        agir(action, &mut c, &livre, &mut progression, partie.as_deref_mut(), *state.get(), &mut next, &mut evenements);
    }
}

#[allow(clippy::too_many_arguments)]
fn clavier(
    keys: Res<ButtonInput<KeyCode>>,
    mut c: Commands,
    livre: Res<Livre>,
    mut progression: ResMut<Progression>,
    mut partie: Option<ResMut<Partie>>,
    state: Res<State<GameState>>,
    mut next: ResMut<NextState<GameState>>,
    mut evenements: MessageWriter<JeuEvenement>,
) {
    let ctrl = keys.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight, KeyCode::SuperLeft, KeyCode::SuperRight]);
    let action = match state.get() {
        GameState::Menu if keys.any_just_pressed([KeyCode::Enter, KeyCode::Space]) => Action::Continuer,
        GameState::Livre | GameState::Gagne if keys.just_pressed(KeyCode::Escape) => Action::Menu,
        GameState::Gagne if keys.any_just_pressed([KeyCode::Enter, KeyCode::Space]) => Action::Suivante,
        GameState::Playing if keys.just_pressed(KeyCode::Escape) => Action::Menu,
        GameState::Playing if keys.just_pressed(KeyCode::KeyH) => Action::Indice,
        GameState::Playing if (ctrl && keys.just_pressed(KeyCode::KeyZ)) || keys.just_pressed(KeyCode::Backspace) => Action::Annuler,
        _ => return,
    };
    agir(action, &mut c, &livre, &mut progression, partie.as_deref_mut(), *state.get(), &mut next, &mut evenements);
}
