//! L'interface : le menu, le bandeau des compteurs, la main en bas, la
//! colonne « À venir » à droite et le bilan de fin de partie. Tous les
//! textes en cinq langues avec `tx!`.
use bevy::picking::hover::Hovered;
use bevy::prelude::*;
use fuite_core::{Forme, Rotation};
use game_common::langue::Texte;
use game_common::tx;
use game_common::ui::{GameHelp, PANEL, ThemedPanel, pied_de_menu, spawn_button, spawn_gap, spawn_hud, spawn_panel, spawn_panel_left, spawn_text};

use crate::GameState;
use crate::jeu::{Images, Jeu, Record};

const AIDE: Texte = tx!(
    "Une fuite au milieu de la pâture ! Raccordez des tuyaux pour tout boucher.\n\n\
     L'eau avance toute seule dans ce qui est relié ; ce qui sort sur du vide déborde, sans défaite.\n\
     Une carte se pose contre une ouverture d'eau, et chaque bord doit concorder avec ses voisines : \
     ouverture contre ouverture, mur contre mur.\n\
     Rien de posable ? On pioche une carte de plus. Fin quand il n'y a plus rien à poser.\n\
     Moins de cartes = plus de points ; tout boucher vaut bien plus.\n\n\
     Clic ou Entrée : poser · clic droit ou R : tourner · 1 à 9, Tab : choisir\n\
     Flèches : viser · molette : zoom · Échap : menu · M : son",
    "A leak in the middle of the pasture! Connect pipes to plug it all.\n\n\
     Water flows by itself through whatever is connected; what spills into the void overflows, no defeat.\n\
     A card goes against a water opening, and every edge must match its neighbours: \
     opening to opening, wall to wall.\n\
     Nothing to play? Draw one more card. The game ends when nothing can be placed.\n\
     Fewer cards = more points; plugging everything is worth far more.\n\n\
     Click or Enter: place · right click or R: rotate · 1 to 9, Tab: choose\n\
     Arrows: aim · wheel: zoom · Esc: menu · M: sound",
    "Ein Leck mitten auf der Weide! Verbinde Rohre, um alles zu stopfen.\n\n\
     Das Wasser fließt von allein durch alles, was verbunden ist; was ins Leere läuft, läuft über – keine Niederlage.\n\
     Eine Karte liegt an einer Wasseröffnung an, und jede Kante muss zu den Nachbarn passen: \
     Öffnung an Öffnung, Wand an Wand.\n\
     Nichts legbar? Eine Karte mehr ziehen. Ende, wenn nichts mehr gelegt werden kann.\n\
     Weniger Karten = mehr Punkte; alles zu stopfen zählt viel mehr.\n\n\
     Klick oder Eingabe: legen · Rechtsklick oder R: drehen · 1 bis 9, Tab: wählen\n\
     Pfeile: zielen · Mausrad: Zoom · Esc: Menü · M: Ton",
    "Una perdita in mezzo al pascolo! Collega i tubi per tappare tutto.\n\n\
     L'acqua avanza da sola in ciò che è collegato; ciò che esce nel vuoto trabocca, senza sconfitta.\n\
     Una carta si posa contro un'apertura d'acqua, e ogni bordo deve combaciare con i vicini: \
     apertura contro apertura, muro contro muro.\n\
     Niente da posare? Si pesca una carta in più. Fine quando non c'è più nulla da posare.\n\
     Meno carte = più punti; tappare tutto vale molto di più.\n\n\
     Clic o Invio: posare · clic destro o R: ruotare · 1–9, Tab: scegliere\n\
     Frecce: mirare · rotella: zoom · Esc: menu · M: audio",
    "¡Una fuga en medio del prado! Conecta tuberías para taparlo todo.\n\n\
     El agua avanza sola por lo que está conectado; lo que sale al vacío se desborda, sin derrota.\n\
     Una carta se coloca contra una abertura de agua, y cada borde debe encajar con sus vecinas: \
     abertura contra abertura, muro contra muro.\n\
     ¿Nada que colocar? Se roba una carta más. Fin cuando no queda nada que colocar.\n\
     Menos cartas = más puntos; taparlo todo vale mucho más.\n\n\
     Clic o Intro: colocar · clic derecho o R: girar · 1 a 9, Tab: elegir\n\
     Flechas: apuntar · rueda: zoom · Esc: menú · M: sonido",
);

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameHelp(AIDE))
            .add_systems(OnEnter(GameState::Menu), spawn_menu)
            .add_systems(OnEnter(GameState::Playing), spawn_partie)
            .add_systems(OnEnter(GameState::Fin), spawn_bilan)
            .add_systems(Update, (clavier, maj_bandeau, maj_main, maj_a_venir))
            .add_observer(au_clic);
    }
}

#[derive(Component, Clone, Copy, PartialEq, Eq)]
enum Action {
    Jouer,
    Menu,
    /// Choisir la carte `i` de la main.
    Choisir(usize),
}

#[derive(Component)]
struct TexteBandeau;

#[derive(Component)]
struct Main;

#[derive(Component)]
struct AVenir;

const CHOISIE: Color = Color::srgb(0.98, 0.80, 0.20);

fn spawn_menu(mut commands: Commands, record: Res<Record>) {
    let panel = spawn_panel(&mut commands, DespawnOnExit(GameState::Menu));
    spawn_text(&mut commands, panel, tx!("ÇA FUIT !", "IT'S LEAKING!", "ES TROPFT!", "C'È UNA PERDITA!", "¡HAY UNA FUGA!").t(), 60.0);
    spawn_text(
        &mut commands,
        panel,
        tx!("Le crapaud plombier et ses tuyaux de zinc", "The plumber toad and his zinc pipes", "Die Klempnerkröte und ihre Zinkrohre", "Il rospo idraulico e i suoi tubi di zinco", "El sapo fontanero y sus tuberías de zinc").t(),
        20.0,
    );
    if record.0 > 0 {
        spawn_text(&mut commands, panel, tx!("Record : {}", "Best: {}", "Rekord: {}", "Record: {}", "Récord: {}").avec(&[&record.0]), 20.0);
    }
    spawn_gap(&mut commands, panel, 6.0);
    let (b, _) = spawn_button(&mut commands, panel, tx!("Jouer", "Play", "Spielen", "Gioca", "Jugar").t());
    commands.entity(b).insert(Action::Jouer);
    pied_de_menu(&mut commands, panel);
}

fn au_clic(clic: On<PointerClick>, actions: Query<&Action>, mut jeu: ResMut<Jeu>, mut next: ResMut<NextState<GameState>>) {
    match actions.get(clic.entity) {
        Ok(Action::Jouer) => next.set(GameState::Playing),
        Ok(Action::Menu) => next.set(GameState::Menu),
        Ok(Action::Choisir(i)) => jeu.choisir(*i),
        Err(_) => {}
    }
}

fn clavier(keys: Res<ButtonInput<KeyCode>>, state: Res<State<GameState>>, mut next: ResMut<NextState<GameState>>) {
    match state.get() {
        GameState::Menu if keys.any_just_pressed([KeyCode::Enter, KeyCode::Space]) => next.set(GameState::Playing),
        GameState::Playing if keys.just_pressed(KeyCode::Escape) => next.set(GameState::Menu),
        GameState::Fin if keys.just_pressed(KeyCode::Escape) => next.set(GameState::Menu),
        GameState::Fin if keys.any_just_pressed([KeyCode::Enter, KeyCode::Space]) => next.set(GameState::Playing),
        _ => {}
    }
}

/// Un panneau du jeu (fond clair arrondi), qui attrape le pointeur pour
/// qu'un clic dessus ne pose pas de carte.
fn panneau(commands: &mut Commands, node: Node, scope: impl Bundle) -> Entity {
    commands
        .spawn((
            Node { padding: UiRect::all(Val::Px(10.0)), border: UiRect::all(Val::Px(2.0)), border_radius: BorderRadius::all(Val::Px(18.0)), ..node },
            BackgroundColor(PANEL),
            BorderColor::all(Color::NONE),
            ThemedPanel,
            Hovered::default(),
            scope,
        ))
        .id()
}

fn spawn_partie(mut commands: Commands) {
    let (hud, texte) = spawn_hud(&mut commands);
    commands.entity(hud).insert((DespawnOnExit(GameState::Playing), Hovered::default()));
    commands.entity(texte).insert(TexteBandeau);
    // La main, en bas au centre.
    panneau(
        &mut commands,
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(14.0),
            left: Val::Percent(50.0),
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            column_gap: Val::Px(10.0),
            ..default()
        },
        (DespawnOnExit(GameState::Playing), Main, UiTransform::from_translation(Val2::new(Val::Percent(-50.0), Val::ZERO))),
    );
    // Les prochains tuyaux, à droite.
    let colonne = panneau(
        &mut commands,
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(70.0),
            right: Val::Px(14.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            row_gap: Val::Px(6.0),
            ..default()
        },
        (DespawnOnExit(GameState::Playing), AVenir),
    );
    spawn_text(&mut commands, colonne, tx!("À venir", "Next", "Nächste", "In arrivo", "Siguientes").t(), 16.0);
}

/// Une carte, avec le même sprite que sur le plateau. `cote` = largeur d'une
/// case en pixels. La carte choisie est entourée de jaune et tournée.
fn tuile(commands: &mut Commands, parent: Entity, images: &Images, forme: Forme, rotation: Rotation, cote: f32, choisie: bool, action: Option<Action>) -> Entity {
    let (image, _, demi_tour) = images.carte(forme);
    let rotation = Rotation::de_quarts(rotation.quarts() + demi_tour.quarts());
    let boite = commands
        .spawn((
            Node {
                width: Val::Px(cote * 2.0 + 8.0),
                height: Val::Px(cote * 2.0 + 8.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(3.0)),
                border_radius: BorderRadius::all(Val::Px(10.0)),
                ..default()
            },
            BorderColor::all(if choisie { CHOISIE } else { Color::NONE }),
            BackgroundColor(if choisie { Color::srgba(0.98, 0.80, 0.20, 0.15) } else { Color::NONE }),
            Hovered::default(),
            ChildOf(parent),
        ))
        .id();
    if let Some(a) = action {
        commands.entity(boite).insert(a);
    }
    commands.spawn((
        ImageNode::new(image),
        Node { width: Val::Px(cote), height: Val::Px(cote * 2.0), ..default() },
        UiTransform { rotation: Rot2::degrees(-(rotation.quarts() as f32) * 90.0), ..default() },
        Pickable::IGNORE,
        ChildOf(boite),
    ));
    boite
}

fn maj_bandeau(jeu: Res<Jeu>, record: Res<Record>, mut q: Query<&mut Text, With<TexteBandeau>>) {
    for mut t in &mut q {
        let s = jeu.partie.score();
        t.0 = tx!(
            "Posées : {}   Paquet : {}   Fuites : {}   Record : {}",
            "Placed: {}   Deck: {}   Leaks: {}   Best: {}",
            "Gelegt: {}   Stapel: {}   Lecks: {}   Rekord: {}",
            "Posate: {}   Mazzo: {}   Perdite: {}   Record: {}",
            "Colocadas: {}   Mazo: {}   Fugas: {}   Récord: {}"
        )
        .avec(&[&s.posees, &jeu.partie.restantes(), &s.fuites, &record.0]);
    }
}

/// La main : redessinée quand elle change (pose, pioche, choix, rotation).
fn maj_main(mut commands: Commands, mut jeu: ResMut<Jeu>, images: Res<Images>, main: Query<(Entity, Option<&Children>), With<Main>>) {
    if !jeu.main_changee {
        return;
    }
    let Ok((panneau, enfants)) = main.single() else { return };
    jeu.main_changee = false;
    for e in enfants.into_iter().flatten() {
        commands.entity(*e).despawn();
    }
    let n = jeu.partie.main.len();
    let cote = if n <= 4 { 56.0 } else { (56.0 * 4.0 / n as f32).max(30.0) };
    for (i, &forme) in jeu.partie.main.iter().enumerate() {
        let choisie = i == jeu.choix;
        let rotation = if choisie { jeu.rotation } else { Rotation::R0 };
        tuile(&mut commands, panneau, &images, forme, rotation, cote, choisie, Some(Action::Choisir(i)));
    }
}

/// Les prochains tuyaux : six, ou bien plus quand on survole la colonne.
fn maj_a_venir(mut commands: Commands, jeu: Res<Jeu>, images: Res<Images>, colonne: Query<(Entity, &Hovered, &Children), With<AVenir>>, tuiles: Query<Entity, With<TuileAVenir>>, mut montre: Local<Option<(usize, Vec<Forme>)>>) {
    let Ok((panneau, survol, _)) = colonne.single() else { return };
    let nombre = if survol.get() { 12 } else { 5 };
    let voulues: Vec<Forme> = jeu.partie.a_venir().take(nombre).collect();
    if montre.as_ref().is_some_and(|(n, v)| *n == nombre && *v == voulues) {
        return;
    }
    *montre = Some((nombre, voulues.clone()));
    for e in &tuiles {
        commands.entity(e).despawn();
    }
    for forme in voulues {
        let boite = tuile(&mut commands, panneau, &images, forme, Rotation::R0, 26.0, false, None);
        commands.entity(boite).insert(TuileAVenir);
    }
    if jeu.partie.restantes() > nombre {
        let e = spawn_text(&mut commands, panneau, "…", 16.0);
        commands.entity(e).insert(TuileAVenir);
    }
}

#[derive(Component)]
struct TuileAVenir;

fn spawn_bilan(mut commands: Commands, jeu: Res<Jeu>, record: Res<Record>) {
    let s = jeu.partie.score();
    // Sur le côté : on voit encore le réseau.
    let panel = spawn_panel_left(&mut commands, DespawnOnExit(GameState::Fin));
    let titre = if s.bouche {
        tx!("Plus une goutte !", "Not a drop left!", "Kein Tropfen mehr!", "Non una goccia!", "¡Ni una gota!")
    } else {
        tx!("Ça fuit encore…", "Still leaking…", "Es tropft noch…", "Perde ancora…", "Sigue goteando…")
    };
    spawn_text(&mut commands, panel, titre.t(), 40.0);
    spawn_text(&mut commands, panel, tx!("{} points", "{} points", "{} Punkte", "{} punti", "{} puntos").avec(&[&s.points]), 30.0);
    spawn_text(
        &mut commands,
        panel,
        tx!("{} cartes posées · {} bouchons · {} fuites", "{} cards placed · {} plugs · {} leaks", "{} Karten gelegt · {} Stopfen · {} Lecks", "{} carte posate · {} tappi · {} perdite", "{} cartas colocadas · {} tapones · {} fugas")
            .avec(&[&s.posees, &s.bouchons, &s.fuites]),
        18.0,
    );
    if s.points >= record.0 && s.points > 0 {
        spawn_text(&mut commands, panel, tx!("Nouveau record !", "New best!", "Neuer Rekord!", "Nuovo record!", "¡Nuevo récord!").t(), 20.0);
    } else {
        spawn_text(&mut commands, panel, tx!("Record : {}", "Best: {}", "Rekord: {}", "Record: {}", "Récord: {}").avec(&[&record.0]), 18.0);
    }
    spawn_gap(&mut commands, panel, 6.0);
    let (b, _) = spawn_button(&mut commands, panel, tx!("Rejouer", "Play again", "Nochmal", "Rigioca", "Jugar otra vez").t());
    commands.entity(b).insert(Action::Jouer);
    let (b, _) = spawn_button(&mut commands, panel, tx!("Menu", "Menu", "Menü", "Menu", "Menú").t());
    commands.entity(b).insert(Action::Menu);
}
