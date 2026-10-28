//! Les écrans : menu, niveaux, bandeau de jeu, fin de partie. Tous les textes
//! en cinq langues avec `tx!`.
use berger_core::{Fin, NIVEAUX};
use bevy::prelude::*;
use game_common::calendar::Date;
use game_common::langue::Texte;
use game_common::tx;
use game_common::ui::{GameHelp, PANEL, ThemedPanel, spawn_button_sized, spawn_panel_left, spawn_text};

use crate::GameState;
use crate::jeu::{Commencer, Genre, Partie, Progression};

/// Aide du jeu (bouton « Aide » du pied de menu).
const AIDE: Texte = tx!(
    "Vous êtes le chien de berger : rassemblez les moutons et rentrez-les dans l'enclos.\n\n\
     Les moutons fuient le chien et restent groupés. Placez-vous derrière eux, du côté opposé à l'enclos, et avancez : ils partent dans l'autre sens.\n\n\
     Flèches ou ZQSD : courir · Maj : sprint (le souffle s'épuise)\n\
     Souris ou doigt : le chien va vers le pointeur · V : mode souris (il suit sans cliquer, le clic aboie)\n\
     Espace, B ou clic droit : aboyer (affole les moutons et fait fuir les loups)\n\
     Échap : menu · R : recommencer · M : son · C : caméra fixe ou mobile\n\n\
     Les loups attaquent les moutons loin de l'enclos : fonce les faire fuir ! Trois étoiles : rentrer tout le troupeau, sans en perdre, avant le temps de référence.",
    "You are the sheepdog: gather the sheep and bring them into the pen.\n\n\
     Sheep run from the dog and stay together. Get behind them, on the side away from the pen, and move forward: they will head the other way.\n\n\
     Arrow keys or WASD: run · Shift: sprint (you run out of breath)\n\
     Mouse or finger: the dog goes to the pointer · V: mouse mode (it follows without clicking, click barks)\n\
     Space, B or right click: bark (scares the sheep and drives wolves away)\n\
     Esc: menu · R: restart · M: sound · C: fixed or moving camera\n\n\
     Wolves attack sheep far from the pen: rush over and scare them off! Three stars: bring in the whole flock, losing none, before the reference time.",
    "Du bist der Schäferhund: Treibe die Schafe zusammen und ins Gehege.\n\n\
     Die Schafe laufen vor dem Hund weg und bleiben beisammen. Stell dich hinter sie, auf die vom Gehege abgewandte Seite, und geh vor: Sie laufen in die andere Richtung.\n\n\
     Pfeiltasten oder WASD: rennen · Umschalt: Sprint (der Atem geht aus)\n\
     Maus oder Finger: Der Hund läuft zum Zeiger · V: Mausmodus (folgt ohne Klick, Klick = bellen)\n\
     Leertaste, B oder Rechtsklick: bellen (erschreckt die Schafe und vertreibt Wölfe)\n\
     Esc: Menü · R: neu starten · M: Ton · C: feste oder bewegte Kamera\n\n\
     Wölfe greifen Schafe fern vom Gehege an: Lauf hin und jage sie fort! Drei Sterne: die ganze Herde einbringen, ohne Verluste, vor der Richtzeit.",
    "Sei il cane pastore: raduna le pecore e portale nel recinto.\n\n\
     Le pecore scappano dal cane e restano unite. Mettiti dietro di loro, dal lato opposto al recinto, e avanza: andranno dall'altra parte.\n\n\
     Frecce o WASD: correre · Maiusc: scatto (il fiato finisce)\n\
     Mouse o dito: il cane va verso il puntatore · V: modalità mouse (segue senza cliccare, il clic abbaia)\n\
     Spazio, B o clic destro: abbaiare (spaventa le pecore e mette in fuga i lupi)\n\
     Esc: menu · R: ricomincia · M: audio · C: camera fissa o mobile\n\n\
     I lupi attaccano le pecore lontano dal recinto: corri a farli fuggire! Tre stelle: portare dentro tutto il gregge, senza perderne, prima del tempo di riferimento.",
    "Eres el perro pastor: reúne las ovejas y llévalas al corral.\n\n\
     Las ovejas huyen del perro y se mantienen juntas. Colócate detrás de ellas, en el lado opuesto al corral, y avanza: irán hacia el otro lado.\n\n\
     Flechas o WASD: correr · Mayús: sprint (se acaba el aliento)\n\
     Ratón o dedo: el perro va hacia el puntero · V: modo ratón (sigue sin hacer clic, el clic ladra)\n\
     Espacio, B o clic derecho: ladrar (asusta a las ovejas y ahuyenta a los lobos)\n\
     Esc: menú · R: reiniciar · M: sonido · C: cámara fija o móvil\n\n\
     Los lobos atacan a las ovejas lejos del corral: ¡corre a espantarlos! Tres estrellas: meter todo el rebaño, sin perder ninguna, antes del tiempo de referencia.",
);

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameHelp(AIDE))
            .add_systems(OnEnter(GameState::Menu), spawn_menu)
            .add_systems(OnEnter(GameState::Playing), spawn_bandeau)
            .add_systems(Update, (clavier, maj_bandeau))
            .add_observer(au_clic)
            .add_observer(survol_entre)
            .add_observer(survol_sort);
    }
}

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Continuer,
    Niveaux,
    Jour,
    Hasard,
    Choisir(usize),
    Suivant,
    Rejouer,
    Menu,
    Aboyer,
}

#[derive(Component)]
struct TexteInfo;
#[derive(Component)]
struct TexteAlerte;
#[derive(Component)]
struct BoiteAlerte;
#[derive(Component)]
struct TexteIntro;
#[derive(Component)]
struct BoiteIntro;
#[derive(Component)]
struct BarreSouffle;
#[derive(Component)]
struct BarreAboi;

pub fn temps(secs: f32) -> String {
    let s = secs as u32;
    format!("{}:{:02}", s / 60, s % 60)
}

pub(crate) fn titre_partie(genre: Genre) -> String {
    match genre {
        Genre::Campagne(i) => tx!("Niveau {}", "Level {}", "Level {}", "Livello {}", "Nivel {}").avec(&[&(i + 1)]),
        Genre::Jour(_) => tx!("Pré du jour", "Daily meadow", "Wiese des Tages", "Prato del giorno", "Prado del día").t().to_string(),
        Genre::Hasard(..) => tx!("Pré au hasard", "Random meadow", "Zufallswiese", "Prato a caso", "Prado al azar").t().to_string(),
    }
}

/// Le menu façon « carte en bois » : un titre sur une planche, des planches à icône pour chaque entrée.
#[derive(Component)]
pub(crate) struct Survol;

fn survol_entre(over: On<PointerOver>, mut q: Query<&mut UiTransform, With<Survol>>) {
    if let Ok(mut t) = q.get_mut(over.entity) {
        t.scale = Vec2::splat(1.05);
    }
}

fn survol_sort(out: On<PointerOut>, mut q: Query<&mut UiTransform, With<Survol>>) {
    if let Ok(mut t) = q.get_mut(out.entity) {
        t.scale = Vec2::ONE;
    }
}

const CREME: Color = Color::srgb(1.0, 0.96, 0.84);
const LARGEUR_PLANCHE: f32 = 262.0;
const HAUTEUR_PLANCHE: f32 = 50.5;

/// Une planche de bois avec son icône (l'image) et le texte à droite de l'icône.
fn planche(c: &mut Commands, parent: Entity, server: &AssetServer, image: &str, texte: &str) -> Entity {
    let b = c
        .spawn((
            Node { width: px(LARGEUR_PLANCHE), height: px(HAUTEUR_PLANCHE), ..default() },
            ImageNode::new(server.load(format!("branding/berger/bouton-{image}.png"))),
            UiTransform::default(),
            Survol,
            ChildOf(parent),
        ))
        .id();
    let taille = if texte.chars().count() > 14 { 16.0 } else { 21.0 };
    // Le texte occupe la planche à droite de l'icône.
    c.spawn((
        Node { position_type: PositionType::Absolute, left: percent(27), right: percent(9), top: px(0), bottom: px(0), justify_content: JustifyContent::Center, align_items: AlignItems::Center, ..default() },
        Pickable::IGNORE,
        ChildOf(b),
        children![(
            Text::new(texte),
            TextFont::from_font_size(taille),
            TextColor(CREME),
            TextShadow { offset: Vec2::new(0.0, 2.0), color: Color::srgba(0.25, 0.1, 0.0, 0.75) },
            TextLayout::justify(Justify::Center),
            Pickable::IGNORE,
        )],
    ));
    b
}

fn spawn_menu(mut c: Commands, server: Res<AssetServer>, progression: Res<Progression>, partie: Option<Res<Partie>>) {
    let panel = spawn_panel_left(&mut c, DespawnOnExit(GameState::Menu));
    // Pas de panneau : les planches flottent devant le champ.
    c.entity(panel).remove::<ThemedPanel>().insert((
        Node { flex_direction: FlexDirection::Column, align_items: AlignItems::Center, row_gap: px(3), width: px(LARGEUR_PLANCHE + 40.0), padding: UiRect::all(px(6)), ..default() },
        BackgroundColor(Color::NONE),
    ));
    // Le titre : le chien sur sa planche (le texte suit la langue).
    let titre = c.spawn((Node { width: px(176), height: px(165), ..default() }, ImageNode::new(server.load("branding/berger/titre.png")), Pickable::IGNORE, ChildOf(panel))).id();
    c.spawn((
        Node { position_type: PositionType::Absolute, left: px(14), right: px(14), top: px(110), height: px(44), justify_content: JustifyContent::Center, align_items: AlignItems::Center, ..default() },
        Pickable::IGNORE,
        ChildOf(titre),
        children![(
            Text::new(tx!("Le chien de berger", "The sheepdog", "Der Schäferhund", "Il cane pastore", "El perro pastor").t()),
            TextFont::from_font_size(16.0),
            TextColor(CREME),
            TextShadow { offset: Vec2::new(0.0, 2.0), color: Color::srgba(0.25, 0.1, 0.0, 0.85) },
            TextLayout::justify(Justify::Center),
            Pickable::IGNORE,
        )],
    ));
    let en_cours = partie.as_ref().is_some_and(|p| p.en_cours());
    let jouer = if en_cours { tx!("Continuer", "Continue", "Weiter", "Continua", "Continuar").t() } else { tx!("Jouer", "Play", "Spielen", "Gioca", "Jugar").t() };
    let b = planche(&mut c, panel, &server, "jouer", jouer);
    c.entity(b).insert(Action::Continuer);
    let b = planche(&mut c, panel, &server, "niveaux", tx!("Niveaux", "Levels", "Level", "Livelli", "Niveles").t());
    c.entity(b).insert(Action::Niveaux);
    let jour = if progression.jour_reussi() {
        tx!("Pré du jour : réussi", "Daily meadow: done", "Wiese des Tages: geschafft", "Prato del giorno: fatto", "Prado del día: hecho").t()
    } else {
        tx!("Pré du jour", "Daily meadow", "Wiese des Tages", "Prato del giorno", "Prado del día").t()
    };
    let b = planche(&mut c, panel, &server, "chien", tx!("Mon chien", "My dog", "Mein Hund", "Il mio cane", "Mi perro").t());
    c.entity(b).insert(crate::bois::ActionChien::Ouvrir);
    let b = planche(&mut c, panel, &server, "jour", jour);
    c.entity(b).insert(Action::Jour);
    let b = planche(&mut c, panel, &server, "hasard", tx!("Pré au hasard", "Random meadow", "Zufallswiese", "Prato a caso", "Prado al azar").t());
    c.entity(b).insert(Action::Hasard);
    let b = planche(&mut c, panel, &server, "options", tx!("Options", "Options", "Optionen", "Opzioni", "Opciones").t());
    c.entity(b).insert(game_common::options::OptionsBouton);
    let b = planche(&mut c, panel, &server, "aide", tx!("Aide", "Help", "Hilfe", "Aiuto", "Ayuda").t());
    c.entity(b).insert(game_common::ui::HelpButton);
    #[cfg(not(any(target_os = "android", target_arch = "wasm32")))]
    {
        let b = planche(&mut c, panel, &server, "quitter", tx!("Quitter", "Quit", "Beenden", "Esci", "Salir").t());
        c.entity(b).insert(game_common::ui::QuitButton);
    }
    // Étoiles gagnées et logo du studio, sur une petite carte crème.
    let pied = c
        .spawn((
            Node { flex_direction: FlexDirection::Row, align_items: AlignItems::Center, column_gap: px(12), padding: UiRect::axes(px(12), px(2)), border_radius: BorderRadius::all(px(18)), ..default() },
            BackgroundColor(Color::srgba(1.0, 0.97, 0.88, 0.85)),
            Pickable::IGNORE,
            ChildOf(panel),
        ))
        .id();
    c.spawn((ImageNode::new(server.load("branding/hud/etoile.png")), Node { width: px(24), height: px(24), ..default() }, Pickable::IGNORE, ChildOf(pied)));
    spawn_text(&mut c, pied, &format!("{} / {}", progression.total(), NIVEAUX.len() * 3), 18.0);
    game_common::ui::spawn_studio_logo(&mut c, pied);
}

fn spawn_bandeau(mut c: Commands) {
    if crate::capture::sans_interface() {
        return;
    }
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
            Pickable::IGNORE,
            DespawnOnExit(GameState::Playing),
        ))
        .id();
    let info = c
        .spawn((
            Node { padding: UiRect::axes(Val::Px(16.0), Val::Px(8.0)), border: UiRect::all(Val::Px(2.0)), border_radius: BorderRadius::all(Val::Px(14.0)), ..default() },
            BackgroundColor(PANEL),
            ThemedPanel,
            Pickable::IGNORE,
            ChildOf(barre),
        ))
        .id();
    let t = spawn_text(&mut c, info, "", 19.0);
    c.entity(t).insert((TexteInfo, TextLayout::justify(Justify::Left)));
    let boutons = c.spawn((Node { column_gap: Val::Px(8.0), ..default() }, ChildOf(barre))).id();
    for (label, action) in [(tx!("Aboyer", "Bark", "Bellen", "Abbaia", "Ladrar"), Action::Aboyer), (tx!("Menu", "Menu", "Menü", "Menu", "Menú"), Action::Menu)] {
        let (b, _) = spawn_button_sized(&mut c, boutons, label.t(), 118.0, 42.0, 18.0);
        c.entity(b).insert(action);
    }

    // En bas : le souffle et la voix du chien.
    let bas = c
        .spawn((
            Node { position_type: PositionType::Absolute, bottom: Val::Px(16.0), width: Val::Percent(100.0), justify_content: JustifyContent::Center, ..default() },
            Pickable::IGNORE,
            DespawnOnExit(GameState::Playing),
        ))
        .id();
    let boite = c
        .spawn((
            Node { padding: UiRect::axes(Val::Px(16.0), Val::Px(8.0)), column_gap: Val::Px(14.0), border: UiRect::all(Val::Px(2.0)), border_radius: BorderRadius::all(Val::Px(14.0)), align_items: AlignItems::Center, ..default() },
            BackgroundColor(PANEL),
            ThemedPanel,
            Pickable::IGNORE,
            ChildOf(bas),
        ))
        .id();
    for (nom, couleur, souffle) in [
        (tx!("Souffle", "Stamina", "Atem", "Fiato", "Aliento"), Color::srgb(0.30, 0.68, 0.32), true),
        (tx!("Aboiement", "Bark", "Bellen", "Abbaio", "Ladrido"), Color::srgb(0.28, 0.58, 0.88), false),
    ] {
        let t = spawn_text(&mut c, boite, nom.t(), 15.0);
        c.entity(t).insert(TextLayout::justify(Justify::Left));
        let fond = c
            .spawn((Node { width: Val::Px(130.0), height: Val::Px(14.0), border_radius: BorderRadius::all(Val::Px(7.0)), overflow: Overflow::clip(), ..default() }, BackgroundColor(Color::srgb(0.82, 0.80, 0.76)), Pickable::IGNORE, ChildOf(boite)))
            .id();
        let rempli = c.spawn((Node { width: Val::Percent(100.0), height: Val::Percent(100.0), ..default() }, BackgroundColor(couleur), Pickable::IGNORE, ChildOf(fond))).id();
        if souffle {
            c.entity(rempli).insert(BarreSouffle);
        } else {
            c.entity(rempli).insert(BarreAboi);
        }
    }

    // L'alerte « loup » et l'annonce du niveau, au milieu en haut.
    let milieu = c
        .spawn((
            Node { position_type: PositionType::Absolute, top: Val::Px(74.0), width: Val::Percent(100.0), flex_direction: FlexDirection::Column, align_items: AlignItems::Center, row_gap: Val::Px(10.0), ..default() },
            Pickable::IGNORE,
            DespawnOnExit(GameState::Playing),
        ))
        .id();
    let alerte = c
        .spawn((
            Node { padding: UiRect::axes(Val::Px(22.0), Val::Px(8.0)), border_radius: BorderRadius::all(Val::Px(16.0)), ..default() },
            BackgroundColor(Color::srgb(0.86, 0.18, 0.16)),
            Visibility::Hidden,
            BoiteAlerte,
            Pickable::IGNORE,
            ChildOf(milieu),
        ))
        .id();
    let t = spawn_text(&mut c, alerte, tx!("Un loup rôde !", "A wolf is prowling!", "Ein Wolf schleicht herum!", "Un lupo si aggira!", "¡Un lobo ronda!").t(), 28.0);
    c.entity(t).insert((TexteAlerte, TextColor(Color::WHITE)));
    let intro = c
        .spawn((
            Node { padding: UiRect::axes(Val::Px(30.0), Val::Px(14.0)), flex_direction: FlexDirection::Column, align_items: AlignItems::Center, border_radius: BorderRadius::all(Val::Px(20.0)), ..default() },
            BackgroundColor(PANEL),
            ThemedPanel,
            Visibility::Hidden,
            BoiteIntro,
            Pickable::IGNORE,
            ChildOf(milieu),
        ))
        .id();
    let t = spawn_text(&mut c, intro, "", 24.0);
    c.entity(t).insert(TexteIntro);
}

fn maj_bandeau(
    partie: Option<Res<Partie>>,
    mut info: Query<&mut Text, (With<TexteInfo>, Without<TexteIntro>)>,
    mut alerte: Query<&mut Visibility, (With<BoiteAlerte>, Without<BoiteIntro>)>,
    mut intro: Query<&mut Visibility, (With<BoiteIntro>, Without<BoiteAlerte>)>,
    mut texte_intro: Query<&mut Text, (With<TexteIntro>, Without<TexteInfo>)>,
    mut souffle: Query<&mut Node, (With<BarreSouffle>, Without<BarreAboi>)>,
    mut aboi: Query<&mut Node, (With<BarreAboi>, Without<BarreSouffle>)>,
) {
    let Some(partie) = partie else { return };
    let m = &partie.monde;
    for mut t in &mut info {
        let l1 = titre_partie(partie.genre);
        let l2 = tx!("À l'abri : {} / {}", "Safe: {} / {}", "In Sicherheit: {} / {}", "Al sicuro: {} / {}", "A salvo: {} / {}").avec(&[&m.abrites(), &m.vivants()]);
        let reste = temps(m.temps_restant());
        let mut l3 = tx!("Temps : {}", "Time: {}", "Zeit: {}", "Tempo: {}", "Tiempo: {}").avec(&[&reste]);
        if m.niveau.pertes_max > 0 || m.perdus() > 0 {
            l3 += &format!("  ·  {}", tx!("Perdus : {} / {}", "Lost: {} / {}", "Verloren: {} / {}", "Perse: {} / {}", "Perdidas: {} / {}").avec(&[&m.perdus(), &m.niveau.pertes_max]));
        }
        t.0 = format!("{l1}\n{l2}\n{l3}");
    }
    for mut v in &mut alerte {
        *v = if partie.alerte > 0.0 && (partie.alerte * 3.0) as u32 % 2 == 0 { Visibility::Inherited } else { Visibility::Hidden };
    }
    for mut v in &mut intro {
        *v = if partie.intro > 0.0 { Visibility::Inherited } else { Visibility::Hidden };
    }
    if partie.intro > 0.0 {
        for mut t in &mut texte_intro {
            let mut s = tx!("Rentrez les {} moutons dans l'enclos", "Bring the {} sheep into the pen", "Treibe die {} Schafe ins Gehege", "Porta le {} pecore nel recinto", "Lleva las {} ovejas al corral").avec(&[&m.moutons.len()]);
            if m.niveau.loups > 0 {
                s += &format!("\n{}", tx!("Attention aux loups !", "Beware of wolves!", "Vorsicht vor Wölfen!", "Attenzione ai lupi!", "¡Cuidado con los lobos!").t());
            }
            t.0 = s;
        }
    }
    for mut n in &mut souffle {
        n.width = Val::Percent(m.chien.endurance * 100.0);
    }
    for mut n in &mut aboi {
        n.width = Val::Percent((1.0 - m.chien.recharge / 1.8).clamp(0.0, 1.0) * 100.0);
    }
}

/// Exécute une action (bouton ou raccourci clavier).
fn agir(action: Action, partie: Option<&mut Partie>, progression: &Progression, commencer: &mut MessageWriter<Commencer>, next: &mut NextState<GameState>) {
    let neuve = |genre: Genre| Commencer { genre, demo: false, robot: false };
    match action {
        Action::Continuer => {
            if !partie.is_some_and(|p| p.en_cours()) {
                commencer.write(neuve(Genre::Campagne(progression.suivant())));
            }
            next.set(GameState::Playing);
        }
        Action::Niveaux => next.set(GameState::Niveaux),
        Action::Jour => {
            commencer.write(neuve(Genre::Jour(Date::today().days())));
            next.set(GameState::Playing);
        }
        Action::Hasard => {
            let niveau = progression.suivant().max(2).min(NIVEAUX.len() - 1);
            let graine = game_common::Rng::from_time().next_u64() >> 20;
            commencer.write(neuve(Genre::Hasard(niveau, graine)));
            next.set(GameState::Playing);
        }
        Action::Choisir(i) => {
            commencer.write(neuve(Genre::Campagne(i)));
            next.set(GameState::Playing);
        }
        Action::Suivant => {
            if let Some(Partie { genre: Genre::Campagne(i), .. }) = partie.as_deref() {
                commencer.write(neuve(Genre::Campagne((i + 1).min(NIVEAUX.len() - 1))));
                next.set(GameState::Playing);
            }
        }
        Action::Rejouer => {
            if let Some(p) = partie {
                commencer.write(neuve(p.genre));
                next.set(GameState::Playing);
            }
        }
        Action::Menu => next.set(GameState::Menu),
        Action::Aboyer => {
            if let Some(p) = partie {
                p.bouton_aboyer = true;
            }
        }
    }
}

fn au_clic(clic: On<PointerClick>, actions: Query<&Action>, partie: Option<ResMut<Partie>>, progression: Res<Progression>, mut commencer: MessageWriter<Commencer>, mut next: ResMut<NextState<GameState>>) {
    if let Ok(&action) = actions.get(clic.entity) {
        let mut partie = partie;
        agir(action, partie.as_deref_mut(), &progression, &mut commencer, &mut next);
    }
}

fn clavier(keys: Res<ButtonInput<KeyCode>>, partie: Option<ResMut<Partie>>, progression: Res<Progression>, state: Res<State<GameState>>, mut commencer: MessageWriter<Commencer>, mut next: ResMut<NextState<GameState>>) {
    let mut partie = partie;
    let action = match state.get() {
        GameState::Menu if keys.any_just_pressed([KeyCode::Enter, KeyCode::Space]) => Action::Continuer,
        GameState::Niveaux | GameState::Chiens if keys.just_pressed(KeyCode::Escape) => Action::Menu,
        GameState::Fin if keys.just_pressed(KeyCode::Escape) => Action::Menu,
        GameState::Fin if keys.any_just_pressed([KeyCode::Enter, KeyCode::Space]) => {
            if partie.as_ref().is_some_and(|p| p.monde.fin == Some(Fin::Victoire) && matches!(p.genre, Genre::Campagne(i) if i + 1 < NIVEAUX.len())) {
                Action::Suivant
            } else {
                Action::Rejouer
            }
        }
        GameState::Playing if keys.just_pressed(KeyCode::Escape) => Action::Menu,
        GameState::Playing if keys.just_pressed(KeyCode::KeyR) => Action::Rejouer,
        _ => return,
    };
    agir(action, partie.as_deref_mut(), &progression, &mut commencer, &mut next);
}
