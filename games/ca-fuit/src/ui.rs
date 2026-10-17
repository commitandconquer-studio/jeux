//! L'interface : le menu, le bandeau des compteurs (ivoire à bord bleu nuit,
//! pictogrammes et grands chiffres), la main en bas (cartes ivoire, la carte
//! choisie en halo cyan, les cartes imposables grisées), la colonne « À
//! venir » à droite et le bilan de fin de partie. Tous les textes en cinq
//! langues avec `tx!`.
use bevy::picking::hover::Hovered;
use bevy::prelude::*;
use bevy::window::{MonitorSelection, PrimaryWindow, WindowMode};
use fuite_core::{Forme, Rotation};
use game_common::langue::Texte;
use game_common::tx;
use game_common::ui::{GameHelp, pied_de_menu, spawn_button, spawn_gap, spawn_panel, spawn_panel_left, spawn_text};

use crate::GameState;
use crate::jeu::{Images, Jeu, Record, TAILLE_CARTE};

/// Les couleurs de la direction artistique.
const IVOIRE: Color = Color::srgb(0.99, 0.95, 0.87);
const NUIT: Color = Color::srgb(0.055, 0.106, 0.227);
const CYAN: Color = Color::srgb(0.26, 0.80, 1.0);
const GRIS_SEPARATEUR: Color = Color::srgba(0.34, 0.46, 0.58, 0.5);

const AIDE: Texte = tx!(
    "Une fuite au milieu de la pâture ! Raccordez des tuyaux pour tout boucher.\n\n\
     L'eau avance toute seule dans ce qui est relié ; ce qui sort sur du vide déborde, sans défaite.\n\
     Une carte se pose contre une ouverture d'eau, et chaque bord doit concorder avec ses voisines : \
     ouverture contre ouverture, mur contre mur.\n\
     Rien de posable ? On pioche une carte de plus. Fin quand il n'y a plus rien à poser.\n\
     Moins de cartes = plus de points ; tout boucher vaut bien plus.\n\n\
     Clic ou Entrée : poser · clic droit ou R : tourner · 1 à 9, Tab : choisir\n\
     Flèches : viser · molette : zoom · F11 : plein écran · Échap : menu · M : son",
    "A leak in the middle of the pasture! Connect pipes to plug it all.\n\n\
     Water flows by itself through whatever is connected; what spills into the void overflows, no defeat.\n\
     A card goes against a water opening, and every edge must match its neighbours: \
     opening to opening, wall to wall.\n\
     Nothing to play? Draw one more card. The game ends when nothing can be placed.\n\
     Fewer cards = more points; plugging everything is worth far more.\n\n\
     Click or Enter: place · right click or R: rotate · 1 to 9, Tab: choose\n\
     Arrows: aim · wheel: zoom · F11: full screen · Esc: menu · M: sound",
    "Ein Leck mitten auf der Weide! Verbinde Rohre, um alles zu stopfen.\n\n\
     Das Wasser fließt von allein durch alles, was verbunden ist; was ins Leere läuft, läuft über – keine Niederlage.\n\
     Eine Karte liegt an einer Wasseröffnung an, und jede Kante muss zu den Nachbarn passen: \
     Öffnung an Öffnung, Wand an Wand.\n\
     Nichts legbar? Eine Karte mehr ziehen. Ende, wenn nichts mehr gelegt werden kann.\n\
     Weniger Karten = mehr Punkte; alles zu stopfen zählt viel mehr.\n\n\
     Klick oder Eingabe: legen · Rechtsklick oder R: drehen · 1 bis 9, Tab: wählen\n\
     Pfeile: zielen · Mausrad: Zoom · F11: Vollbild · Esc: Menü · M: Ton",
    "Una perdita in mezzo al pascolo! Collega i tubi per tappare tutto.\n\n\
     L'acqua avanza da sola in ciò che è collegato; ciò che esce nel vuoto trabocca, senza sconfitta.\n\
     Una carta si posa contro un'apertura d'acqua, e ogni bordo deve combaciare con i vicini: \
     apertura contro apertura, muro contro muro.\n\
     Niente da posare? Si pesca una carta in più. Fine quando non c'è più nulla da posare.\n\
     Meno carte = più punti; tappare tutto vale molto di più.\n\n\
     Clic o Invio: posare · clic destro o R: ruotare · 1–9, Tab: scegliere\n\
     Frecce: mirare · rotella: zoom · F11: schermo intero · Esc: menu · M: audio",
    "¡Una fuga en medio del prado! Conecta tuberías para taparlo todo.\n\n\
     El agua avanza sola por lo que está conectado; lo que sale al vacío se desborda, sin derrota.\n\
     Una carta se coloca contra una abertura de agua, y cada borde debe encajar con sus vecinas: \
     abertura contra abertura, muro contra muro.\n\
     ¿Nada que colocar? Se roba una carta más. Fin cuando no queda nada que colocar.\n\
     Menos cartas = más puntos; taparlo todo vale mucho más.\n\n\
     Clic o Intro: colocar · clic derecho o R: girar · 1 a 9, Tab: elegir\n\
     Flechas: apuntar · rueda: zoom · F11: pantalla completa · Esc: menú · M: sonido",
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
    PleinEcran,
}

/// Un compteur du bandeau.
#[derive(Component, Clone, Copy, PartialEq, Eq)]
enum Compteur {
    Posees,
    Restantes,
    Fuites,
    Record,
}

#[derive(Component)]
struct Main;

#[derive(Component)]
struct AVenir;

fn spawn_menu(mut commands: Commands, record: Res<Record>) {
    let panel = spawn_panel(&mut commands, DespawnOnExit(GameState::Menu));
    spawn_text(&mut commands, panel, tx!("ÇA FUIT !", "IT'S LEAKING!", "ES TROPFT!", "C'È UNA PERDITA!", "¡HAY UNA FUGA!").t(), 60.0);
    spawn_text(
        &mut commands,
        panel,
        tx!("Le crapaud plombier et ses tuyaux de cuivre", "The plumber toad and his copper pipes", "Die Klempnerkröte und ihre Kupferrohre", "Il rospo idraulico e i suoi tubi di rame", "El sapo fontanero y sus tuberías de cobre").t(),
        20.0,
    );
    if record.0 > 0 {
        spawn_text(&mut commands, panel, tx!("Record : {}", "Best: {}", "Rekord: {}", "Record: {}", "Récord: {}").avec(&[&record.0]), 20.0);
    }
    spawn_gap(&mut commands, panel, 6.0);
    let (b, _) = spawn_button(&mut commands, panel, tx!("Jouer", "Play", "Spielen", "Gioca", "Jugar").t());
    commands.entity(b).insert(Action::Jouer);
    let (b, _) = spawn_button(&mut commands, panel, tx!("Plein écran (F11)", "Full screen (F11)", "Vollbild (F11)", "Schermo intero (F11)", "Pantalla completa (F11)").t());
    commands.entity(b).insert(Action::PleinEcran);
    pied_de_menu(&mut commands, panel);
}

fn au_clic(clic: On<PointerClick>, actions: Query<&Action>, mut jeu: ResMut<Jeu>, mut next: ResMut<NextState<GameState>>, mut fenetres: Query<&mut Window, With<PrimaryWindow>>, capture: Option<Res<crate::capture::Capture>>) {
    if capture.is_some() {
        return;
    }
    match actions.get(clic.entity) {
        Ok(Action::Jouer) => next.set(GameState::Playing),
        Ok(Action::Menu) => next.set(GameState::Menu),
        Ok(Action::Choisir(i)) => jeu.choisir(*i),
        Ok(Action::PleinEcran) => basculer_plein_ecran(&mut fenetres),
        Err(_) => {}
    }
}

/// Fenêtre ou plein écran (sans bordure, sur l'écran courant) ; mémorisé.
fn basculer_plein_ecran(fenetres: &mut Query<&mut Window, With<PrimaryWindow>>) {
    for mut f in fenetres.iter_mut() {
        let plein = !matches!(f.mode, WindowMode::Windowed);
        f.mode = if plein { WindowMode::Windowed } else { WindowMode::BorderlessFullscreen(MonitorSelection::Current) };
        game_common::save::store("ca-fuit", "plein-ecran", if plein { "0" } else { "1" });
    }
}

fn clavier(keys: Res<ButtonInput<KeyCode>>, state: Res<State<GameState>>, mut next: ResMut<NextState<GameState>>, mut fenetres: Query<&mut Window, With<PrimaryWindow>>, capture: Option<Res<crate::capture::Capture>>) {
    if capture.is_some() {
        return;
    }
    if keys.just_pressed(KeyCode::F11) {
        basculer_plein_ecran(&mut fenetres);
    }
    match state.get() {
        GameState::Menu if keys.any_just_pressed([KeyCode::Enter, KeyCode::Space]) => next.set(GameState::Playing),
        GameState::Playing if keys.just_pressed(KeyCode::Escape) => next.set(GameState::Menu),
        GameState::Fin if keys.just_pressed(KeyCode::Escape) => next.set(GameState::Menu),
        GameState::Fin if keys.any_just_pressed([KeyCode::Enter, KeyCode::Space]) => next.set(GameState::Playing),
        _ => {}
    }
}

/// Un panneau ivoire à bord bleu nuit, qui attrape le pointeur pour qu'un
/// clic dessus ne pose pas de carte.
fn panneau(commands: &mut Commands, node: Node, bord: f32, scope: impl Bundle) -> Entity {
    commands
        .spawn((
            Node { border: UiRect::all(Val::Px(bord)), border_radius: BorderRadius::all(Val::Px(16.0)), ..node },
            BackgroundColor(IVOIRE),
            BorderColor::all(NUIT),
            BoxShadow::new(Color::srgba(0.0, 0.0, 0.0, 0.25), Val::Px(3.0), Val::Px(4.0), Val::Px(0.0), Val::Px(4.0)),
            Hovered::default(),
            scope,
        ))
        .id()
}

/// Un texte dans la couleur de la DA (la police du studio s'applique toute seule).
fn texte(commands: &mut Commands, parent: Entity, s: impl Into<String>, taille: f32, couleur: Color) -> Entity {
    commands.spawn((Text::new(s), TextFont::from_font_size(taille), TextColor(couleur), Pickable::IGNORE, ChildOf(parent))).id()
}

fn spawn_partie(mut commands: Commands, images: Res<Images>) {
    // Le bandeau des compteurs, en haut à gauche.
    let bandeau = panneau(
        &mut commands,
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Px(12.0),
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            column_gap: Val::Px(14.0),
            padding: UiRect::axes(Val::Px(16.0), Val::Px(8.0)),
            ..default()
        },
        3.0,
        DespawnOnExit(GameState::Playing),
    );
    let compteurs = [
        (Compteur::Posees, images.pictos.cle.clone(), tx!("Posées", "Placed", "Gelegt", "Posate", "Colocadas")),
        (Compteur::Restantes, images.pictos.tuyau.clone(), tx!("Restantes", "Left", "Übrig", "Rimaste", "Quedan")),
        (Compteur::Fuites, images.pictos.goutte.clone(), tx!("Fuites", "Leaks", "Lecks", "Perdite", "Fugas")),
        (Compteur::Record, images.pictos.trophee.clone(), tx!("Record", "Best", "Rekord", "Record", "Récord")),
    ];
    for (k, (compteur, picto, libelle)) in compteurs.into_iter().enumerate() {
        if k > 0 {
            commands.spawn((Node { width: Val::Px(2.0), height: Val::Px(36.0), ..default() }, BackgroundColor(GRIS_SEPARATEUR), ChildOf(bandeau)));
        }
        let groupe = commands.spawn((Node { flex_direction: FlexDirection::Row, align_items: AlignItems::Center, column_gap: Val::Px(8.0), ..default() }, ChildOf(bandeau))).id();
        commands.spawn((ImageNode::new(picto), Node { width: Val::Px(34.0), height: Val::Px(34.0), ..default() }, Pickable::IGNORE, ChildOf(groupe)));
        let colonne = commands.spawn((Node { flex_direction: FlexDirection::Column, align_items: AlignItems::FlexStart, ..default() }, ChildOf(groupe))).id();
        texte(&mut commands, colonne, libelle.t(), 13.0, Color::srgba(0.2, 0.28, 0.42, 1.0));
        let n = texte(&mut commands, colonne, "0", 26.0, NUIT);
        commands.entity(n).insert(compteur);
    }
    // La main, en bas au centre : des cartes posées directement sur l'herbe.
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(16.0),
            left: Val::Percent(50.0),
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            column_gap: Val::Px(14.0),
            ..default()
        },
        UiTransform::from_translation(Val2::new(Val::Percent(-50.0), Val::ZERO)),
        DespawnOnExit(GameState::Playing),
        Main,
    ));
    // Les prochains tuyaux, à droite, discrets.
    let colonne = panneau(
        &mut commands,
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(90.0),
            right: Val::Px(12.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            row_gap: Val::Px(4.0),
            padding: UiRect::all(Val::Px(8.0)),
            ..default()
        },
        2.0,
        (DespawnOnExit(GameState::Playing), AVenir),
    );
    commands.entity(colonne).insert(BackgroundColor(IVOIRE.with_alpha(0.85)));
    texte(&mut commands, colonne, tx!("À venir", "Next", "Nächste", "In arrivo", "Siguientes").t(), 13.0, Color::srgba(0.2, 0.28, 0.42, 1.0));
}

/// Une carte de la main ou de la pioche : une boîte ivoire arrondie à bord
/// bleu nuit, avec le sprite du plateau dedans (et son ombre). `cote` =
/// largeur d'une case en pixels. La carte choisie a un halo cyan ; une carte
/// imposable est grisée.
#[allow(clippy::too_many_arguments)]
fn tuile(commands: &mut Commands, parent: Entity, images: &Images, forme: Forme, rotation: Rotation, cote: f32, choisie: bool, posable: bool, action: Option<Action>) -> Entity {
    let (image, _, ombre, demi_tour) = images.carte(forme);
    let rotation = Rotation::de_quarts(rotation.quarts() + demi_tour.quarts());
    let boite_px = cote * TAILLE_CARTE.y + 6.0;
    let (bord, fond, bord_px) = match (choisie, posable) {
        (true, _) => (CYAN, IVOIRE, 4.0),
        (false, true) => (NUIT, IVOIRE, 3.0),
        (false, false) => (Color::srgba(0.3, 0.33, 0.4, 0.6), Color::srgba(0.85, 0.84, 0.8, 0.8), 3.0),
    };
    let mut ombre_boite = BoxShadow::new(Color::srgba(0.0, 0.0, 0.0, 0.25), Val::Px(3.0), Val::Px(4.0), Val::Px(0.0), Val::Px(4.0));
    if choisie {
        ombre_boite.0.push(ShadowStyle { color: CYAN.with_alpha(0.75), x_offset: Val::ZERO, y_offset: Val::ZERO, spread_radius: Val::Px(4.0), blur_radius: Val::Px(14.0) });
    }
    let boite = commands
        .spawn((
            Node {
                width: Val::Px(boite_px),
                height: Val::Px(boite_px),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(bord_px)),
                border_radius: BorderRadius::all(Val::Px(14.0)),
                overflow: Overflow::clip(),
                ..default()
            },
            BorderColor::all(bord),
            BackgroundColor(fond),
            ombre_boite,
            Hovered::default(),
            ChildOf(parent),
        ))
        .id();
    if let Some(a) = action {
        commands.entity(boite).insert(a);
    }
    let teinte = if posable { Color::WHITE } else { Color::srgba(0.6, 0.6, 0.6, 0.7) };
    let tourne = UiTransform { rotation: Rot2::degrees(-(rotation.quarts() as f32) * 90.0), ..default() };
    let taille = Node { position_type: PositionType::Absolute, width: Val::Px(cote * TAILLE_CARTE.x), height: Val::Px(cote * TAILLE_CARTE.y), ..default() };
    commands.spawn((ImageNode::new(ombre).with_color(teinte), taille.clone(), tourne.clone(), Pickable::IGNORE, ChildOf(boite)));
    commands.spawn((ImageNode::new(image).with_color(teinte), taille, tourne, Pickable::IGNORE, ChildOf(boite)));
    boite
}

fn maj_bandeau(jeu: Res<Jeu>, record: Res<Record>, mut q: Query<(&Compteur, &mut Text)>) {
    let s = jeu.partie.score();
    for (c, mut t) in &mut q {
        let v = match c {
            Compteur::Posees => s.posees as usize,
            Compteur::Restantes => jeu.partie.restantes(),
            Compteur::Fuites => s.fuites as usize,
            Compteur::Record => record.0 as usize,
        };
        let texte = v.to_string();
        if t.0 != texte {
            t.0 = texte;
        }
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
    let cote = if n <= 4 { 44.0 } else { (44.0 * 4.0 / n as f32).max(24.0) };
    for (i, &forme) in jeu.partie.main.iter().enumerate() {
        let choisie = i == jeu.choix;
        // Les cartes de la main se tiennent couchées ; la carte choisie montre sa rotation.
        let rotation = if choisie { jeu.rotation } else { Rotation::R90 };
        tuile(&mut commands, panneau, &images, forme, rotation, cote, choisie, jeu.partie.posable(i), Some(Action::Choisir(i)));
    }
}

/// Les prochains tuyaux : cinq, ou bien plus quand on survole la colonne.
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
        let boite = tuile(&mut commands, panneau, &images, forme, Rotation::R90, 18.0, false, true, None);
        commands.entity(boite).insert(TuileAVenir);
    }
    if jeu.partie.restantes() > nombre {
        let e = texte(&mut commands, panneau, "…", 14.0, NUIT);
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
