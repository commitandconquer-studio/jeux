//! Écrans : accueil, fin de partie, et affichage du score.
use bevy::prelude::*;
use game_common::Score;
use game_common::tx;
use game_common::ui::{GameHelp, HelpButton, QuitButton, ThemedPanel, ThemedText, spawn_panel_left, spawn_studio_logo};

use crate::screens::{CountUp, Pop, Pulse, spawn_bar, spawn_chip};

use crate::GameState;
use crate::audio::Sfx;
use crate::frog::Death;
use crate::progress::Progress;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_hud)
            .insert_resource(GameHelp(tx!(
                "Aidez votre personnage à aller le plus loin possible !\n\
\n\
Herbe : arbres et rochers bloquent le passage.\n\
Routes : attention aux voitures et aux camions.\n\
Rivières : sautez sur les rondins et les nénuphars.\n\
Voies ferrées : le feu clignote avant le train.\n\
La caméra avance toute seule : ne traînez pas !\n\
\n\
Flèches ou ZQSD, Espace ou clic : avancer\n\
Tactile : glisser du doigt\n\
M : son · Échap : menu",
                "Help your character get as far as possible!\n\
\n\
Grass: trees and rocks block the way.\n\
Roads: watch out for cars and trucks.\n\
Rivers: jump onto the logs and water lilies.\n\
Railways: the light flashes before the train comes.\n\
The camera moves forward on its own: don't dawdle!\n\
\n\
Arrows or WASD, Space or click: move forward\n\
Touch: swipe with your finger\n\
M: sound · Esc: menu",
                "Hilf deiner Figur, so weit wie möglich zu kommen!\n\
\n\
Wiese: Bäume und Felsen versperren den Weg.\n\
Straßen: Vorsicht vor Autos und Lastwagen.\n\
Flüsse: Spring auf die Baumstämme und Seerosen.\n\
Bahngleise: Das Licht blinkt, bevor der Zug kommt.\n\
Die Kamera fährt von selbst weiter: Trödle nicht!\n\
\n\
Pfeiltasten oder WASD, Leertaste oder Klick: vorwärts\n\
Touchscreen: mit dem Finger wischen\n\
M: Ton · Esc: Menü",
                "Aiuta il tuo personaggio ad andare il più lontano possibile!\n\
\n\
Prato: alberi e rocce bloccano il passaggio.\n\
Strade: attenzione alle auto e ai camion.\n\
Fiumi: salta sui tronchi e sulle ninfee.\n\
Ferrovie: la luce lampeggia prima del treno.\n\
La telecamera avanza da sola: non perdere tempo!\n\
\n\
Frecce o WASD, Spazio o clic: avanti\n\
Touch: scorri con il dito\n\
M: audio · Esc: menu",
                "¡Ayuda a tu personaje a llegar lo más lejos posible!\n\
\n\
Hierba: los árboles y las rocas bloquean el paso.\n\
Carreteras: cuidado con los coches y los camiones.\n\
Ríos: salta sobre los troncos y los nenúfares.\n\
Vías del tren: la luz parpadea antes de que pase el tren.\n\
La cámara avanza sola: ¡no te entretengas!\n\
\n\
Flechas o WASD, Espacio o clic: avanzar\n\
Táctil: desliza el dedo\n\
M: sonido · Esc: menú"
            )))
            .add_systems(OnEnter(GameState::Menu), spawn_menu)
            .add_systems(OnEnter(GameState::GameOver), spawn_game_over)
            .add_systems(Update, (update_hud, keyboard_shortcuts))
            .add_observer(on_click)
            .add_observer(survol_entre)
            .add_observer(survol_sort);
    }
}

#[derive(Component, Clone, Copy)]
enum UiAction {
    Play,
    Garage,
    Menu,
    /// Le bouton menu du bandeau de jeu : met fin à la partie, comme Échap.
    Stop,
}

#[derive(Component)]
struct Hud;


/// Couleurs des lettres du titre.

/// Ligne « Prochain : … à N pts » suivie de sa barre de progression.
/// Un gros bouton coloré du menu : dégradé, bord plus sombre, icône blanche et texte blanc ombré.
#[allow(clippy::too_many_arguments)]
fn gros_bouton(commands: &mut Commands, parent: Entity, texte: &str, icone: Option<Handle<Image>>, couleur: Color, largeur: f32, hauteur: f32, taille: f32) -> Entity {
    let c = couleur.to_srgba();
    let clair = Color::srgb((c.red + 0.22).min(1.0), (c.green + 0.22).min(1.0), (c.blue + 0.22).min(1.0));
    let sombre = Color::srgb(c.red * 0.55, c.green * 0.55, c.blue * 0.55);
    let bouton = commands
        .spawn((
            Node {
                width: px(largeur),
                height: px(hauteur),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                column_gap: px(14),
                border: UiRect::all(px(4)),
                border_radius: BorderRadius::all(px(hauteur * 0.34)),
                ..default()
            },
            BackgroundColor(couleur),
            BackgroundGradient::from(LinearGradient {
                angle: std::f32::consts::PI,
                stops: vec![ColorStop::new(clair, percent(0)), ColorStop::new(couleur, percent(55)), ColorStop::new(couleur, percent(100))],
                ..default()
            }),
            BorderColor::all(sombre),
            UiTransform::default(),
            Survol,
            ChildOf(parent),
        ))
        .id();
    if let Some(icone) = icone {
        let cote = hauteur * 0.62;
        commands.spawn((ImageNode::new(icone), Node { width: px(cote), height: px(cote), ..default() }, Pickable::IGNORE, ChildOf(bouton)));
    }
    commands.spawn((
        Text::new(texte),
        TextFont::from_font_size(taille),
        TextColor(Color::WHITE),
        TextShadow { offset: Vec2::new(0.0, 2.5), color: Color::srgba(0.0, 0.0, 0.0, 0.35) },
        TextLayout::justify(Justify::Center),
        Pickable::IGNORE,
        ChildOf(bouton),
    ));
    bouton
}

/// Le bouton grossit un peu sous le pointeur.
#[derive(Component)]
struct Survol;

fn survol_entre(over: On<PointerOver>, mut q: Query<&mut UiTransform, With<Survol>>) {
    if let Ok(mut t) = q.get_mut(over.entity) {
        t.scale = Vec2::splat(1.04);
    }
}

fn survol_sort(out: On<PointerOut>, mut q: Query<&mut UiTransform, With<Survol>>) {
    if let Ok(mut t) = q.get_mut(out.entity) {
        t.scale = Vec2::ONE;
    }
}

fn spawn_menu(mut commands: Commands, progress: Res<Progress>, score: Res<Score>, server: Res<AssetServer>, stats: Res<crate::exploits::Stats>) {
    let panel = spawn_panel_left(&mut commands, DespawnOnExit(GameState::Menu));
    // Panneau « verre » bleu translucide qui se fond vers le rose (et non plus le panneau crème du thème).
    commands
        .entity(panel)
        .remove::<ThemedPanel>()
        .insert((
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                row_gap: px(6),
                width: px(500),
                padding: UiRect::axes(px(30), px(18)),
                border: UiRect::all(px(3)),
                border_radius: BorderRadius::all(px(34)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.7, 0.82, 1.0, 0.8)),
            BackgroundGradient::from(LinearGradient {
                angle: std::f32::consts::PI,
                stops: vec![ColorStop::new(Color::srgba(0.56, 0.76, 1.0, 0.84), percent(0)), ColorStop::new(Color::srgba(0.78, 0.86, 1.0, 0.8), percent(55)), ColorStop::new(Color::srgba(1.0, 0.88, 0.94, 0.8), percent(100))],
                ..default()
            }),
            BorderColor::all(Color::srgba(1.0, 1.0, 1.0, 0.75)),
        ))
        .insert(UiTransform::from_scale(Vec2::ZERO))
        .insert(Pop::after(0.05));
    // Le logo « Cross or Die » (le même dans toutes les langues).
    commands.spawn((ImageNode::new(server.load("branding/cross-or-die.png")), Node { width: px(300), height: px(100), ..default() }, ChildOf(panel)));

    // Carte « Record » : couronne, record, prochain déblocage, barre et compte des déblocages.
    let carte = commands
        .spawn((
            Node { flex_direction: FlexDirection::Column, align_items: AlignItems::Center, row_gap: px(6), width: px(440), padding: UiRect::axes(px(18), px(10)), border_radius: BorderRadius::all(px(24)), ..default() },
            BackgroundColor(Color::srgba(1.0, 1.0, 1.0, 0.72)),
            ChildOf(panel),
        ))
        .id();
    let haut = commands.spawn((Node { flex_direction: FlexDirection::Row, align_items: AlignItems::Center, column_gap: px(16), width: percent(100), ..default() }, ChildOf(carte))).id();
    commands.spawn((ImageNode::new(server.load("branding/menu/couronne.png")), Node { width: px(58), height: px(58), ..default() }, Pickable::IGNORE, ChildOf(haut)));
    let textes = commands.spawn((Node { flex_direction: FlexDirection::Column, align_items: AlignItems::FlexStart, ..default() }, ChildOf(haut))).id();
    let sombre = Color::srgb(0.16, 0.18, 0.27);
    let gris = Color::srgb(0.45, 0.47, 0.56);
    commands.spawn((
        Text::new(tx!("Record : {}", "Best: {}", "Rekord: {}", "Record: {}", "Récord: {}").avec(&[&score.best])),
        TextFont::from_font_size(30.0),
        TextColor(sombre),
        Pickable::IGNORE,
        ChildOf(textes),
    ));
    if let Some((at, item)) = progress.next() {
        commands.spawn((
            Text::new(tx!("Prochain : {} — {} pts", "Next: {} — {} pts", "Nächstes: {} — {} Pkt.", "Prossimo: {} — {} pt", "Siguiente: {} — {} pts").avec(&[&item.name(), &at])),
            TextFont::from_font_size(18.0),
            TextColor(gris),
            Pickable::IGNORE,
            ChildOf(textes),
        ));
        spawn_bar(&mut commands, carte, progress.next_fraction(), 400.0, item.color());
    }
    let total = crate::progress::UNLOCKS.len();
    commands.spawn((
        Text::new(tx!("Déblocages : {} / {}", "Unlocked: {} / {}", "Freigeschaltet: {} / {}", "Sbloccati: {} / {}", "Desbloqueos: {} / {}").avec(&[&progress.count(), &total])),
        TextFont::from_font_size(18.0),
        TextColor(sombre),
        Pickable::IGNORE,
        ChildOf(carte),
    ));
    commands.spawn((
        Text::new(tx!("Livraisons : {}   Exploits : {} / {}", "Deliveries: {}   Achievements: {} / {}", "Lieferungen: {}   Erfolge: {} / {}", "Consegne: {}   Imprese: {} / {}", "Entregas: {}   Logros: {} / {}").avec(&[&stats.livraisons, &stats.exploits_gagnes(), &crate::exploits::NOMBRE])),
        TextFont::from_font_size(18.0),
        TextColor(sombre),
        Pickable::IGNORE,
        ChildOf(carte),
    ));

    // Les boutons.
    let jouer = gros_bouton(&mut commands, panel, tx!("Jouer", "Play", "Spielen", "Gioca", "Jugar").t(), Some(server.load("branding/menu/jouer.png")), Color::srgb(0.2, 0.72, 0.27), 420.0, 64.0, 40.0);
    commands.entity(jouer).insert((UiAction::Play, Pulse { speed: 3.0, amount: 0.03 })).remove::<Survol>();
    let collection = gros_bouton(&mut commands, panel, tx!("Collection", "Collection", "Sammlung", "Collezione", "Colección").t(), Some(server.load("branding/menu/collection.png")), Color::srgb(0.16, 0.5, 0.96), 380.0, 48.0, 28.0);
    commands.entity(collection).insert(UiAction::Garage);
    let options = gros_bouton(&mut commands, panel, tx!("Options", "Options", "Optionen", "Opzioni", "Opciones").t(), Some(server.load("branding/menu/options.png")), Color::srgb(0.56, 0.34, 0.86), 380.0, 48.0, 28.0);
    commands.entity(options).insert(game_common::options::OptionsBouton);
    let aide = gros_bouton(&mut commands, panel, tx!("Aide", "Help", "Hilfe", "Aiuto", "Ayuda").t(), Some(server.load("branding/menu/aide.png")), Color::srgb(0.98, 0.62, 0.12), 380.0, 48.0, 28.0);
    commands.entity(aide).insert(HelpButton);
    #[cfg(not(any(target_os = "android", target_arch = "wasm32")))]
    {
        let quitter = gros_bouton(&mut commands, panel, tx!("Quitter", "Quit", "Beenden", "Esci", "Salir").t(), None, Color::srgb(0.86, 0.32, 0.34), 200.0, 36.0, 20.0);
        commands.entity(quitter).insert(QuitButton);
    }
    // Filet et logo du studio.
    commands.spawn((Node { width: px(400), height: px(2), margin: UiRect::top(px(2)), ..default() }, BackgroundColor(Color::srgba(1.0, 1.0, 1.0, 0.7)), ChildOf(panel)));
    spawn_studio_logo(&mut commands, panel);
}

/// « Score : 12    Record : 40 », dans la langue en cours.
/// Un texte posé à un endroit précis du panneau (position et taille en % du panneau).
#[allow(clippy::too_many_arguments)]
fn texte_abs(commands: &mut Commands, parent: Entity, texte: impl Into<String>, (x, y, l, h): (f32, f32, f32, f32), taille: f32, couleur: Color, justif: JustifyContent, ombre: Option<Color>) -> Entity {
    let boite = commands
        .spawn((
            Node { position_type: PositionType::Absolute, left: percent(x), top: percent(y), width: percent(l), height: percent(h), justify_content: justif, align_items: AlignItems::Center, ..default() },
            Pickable::IGNORE,
            ChildOf(parent),
        ))
        .id();
    let t = commands
        .spawn((
            Text::new(texte),
            TextFont::from_font_size(taille),
            TextColor(couleur),
            TextLayout::justify(match justif {
                JustifyContent::FlexStart => Justify::Left,
                JustifyContent::FlexEnd => Justify::Right,
                _ => Justify::Center,
            }),
            Pickable::IGNORE,
            ThemedText,
            ChildOf(boite),
        ))
        .id();
    if let Some(c) = ombre {
        commands.entity(t).insert(TextShadow { offset: Vec2::splat(3.0), color: c });
    }
    t
}

pub fn spawn_game_over(mut commands: Commands, score: Res<Score>, death: Res<Death>, progress: Res<Progress>, mut sfx: MessageWriter<Sfx>, mut rng: ResMut<game_common::Rng>, server: Res<AssetServer>) {
    let brun = Color::srgb(0.35, 0.2, 0.08);
    let racine = commands
        .spawn((
            Node { width: percent(100), height: percent(100), flex_direction: FlexDirection::Column, justify_content: JustifyContent::Center, align_items: AlignItems::Center, row_gap: px(6), ..default() },
            DespawnOnExit(GameState::GameOver),
        ))
        .id();
    // Le panneau du livreur K.O. (image 1000 x 824) : tout est posé dessus en pourcentages.
    let panneau = commands
        .spawn((
            ImageNode::new(server.load("branding/panneau-fin.png")),
            Node { width: px(800), height: px(659), ..default() },
            UiTransform::from_scale(Vec2::ZERO),
            Pop::after(0.0),
            ChildOf(racine),
        ))
        .id();
    let title = death.cause.map(|c| crate::annonces::mort(c, death.train, &mut rng)).unwrap_or(tx!("Perdu !", "Game over!", "Verloren!", "Hai perso!", "¡Has perdido!").t());
    let taille = match title.chars().count() {
        0..=18 => 46.0,
        19..=34 => 38.0,
        _ => 32.0,
    };
    texte_abs(&mut commands, panneau, title, (15.6, 25.5, 70.0, 18.5), taille, Color::WHITE, JustifyContent::Center, Some(Color::srgba(0.35, 0.05, 0.02, 0.85)));
    // Score
    texte_abs(&mut commands, panneau, "Score", (39.0, 45.0, 20.0, 5.0), 24.0, Color::srgb(0.4, 0.36, 0.34), JustifyContent::Center, None);
    let rang = commands
        .spawn((
            Node { position_type: PositionType::Absolute, left: percent(30.0), top: percent(50.0), width: percent(38.0), height: percent(14.0), justify_content: JustifyContent::Center, align_items: AlignItems::FlexEnd, column_gap: px(10), ..default() },
            Pickable::IGNORE,
            ChildOf(panneau),
        ))
        .id();
    commands.spawn((
        Text::new("0"),
        TextFont::from_font_size(76.0),
        TextColor(Color::srgb(1.0, 0.82, 0.2)),
        TextShadow { offset: Vec2::splat(4.0), color: Color::srgba(0.25, 0.1, 0.0, 0.9) },
        ThemedText,
        CountUp { target: score.current, age: 0.0 },
        ChildOf(rang),
    ));
    commands.spawn((Text::new("pts"), TextFont::from_font_size(28.0), TextColor(Color::srgb(0.4, 0.36, 0.34)), ThemedText, Node { margin: UiRect::bottom(px(14)), ..default() }, ChildOf(rang)));
    // Record
    let (etiquette, couleur_rec) = if progress.new_record {
        (tx!("NOUVEAU !", "NEW!", "NEU!", "NUOVO!", "¡NUEVO!").t(), Color::srgb(0.8, 0.4, 0.0))
    } else {
        (tx!("Record", "Best", "Rekord", "Record", "Récord").t(), brun)
    };
    texte_abs(&mut commands, panneau, etiquette, (77.0, 51.2, 14.3, 4.5), 17.0, couleur_rec, JustifyContent::Center, None);
    texte_abs(&mut commands, panneau, score.best.to_string(), (77.0, 55.6, 14.3, 6.0), 34.0, brun, JustifyContent::Center, None);
    if progress.new_record {
        texte_abs(
            &mut commands,
            panneau,
            tx!(
                "Incroyable ! Vous êtes notre employé du mois.",
                "Incredible! You are our employee of the month.",
                "Unglaublich! Sie sind unser Mitarbeiter des Monats.",
                "Incredibile! Siete il nostro dipendente del mese.",
                "¡Increíble! Es nuestro empleado del mes."
            )
            .t(),
            (14.0, 62.6, 72.0, 4.5),
            20.0,
            Color::srgb(0.3, 0.25, 0.22),
            JustifyContent::Center,
            None,
        );
    }
    // Prochain déblocage : la barre du panneau est remplacée par la nôtre.
    if let Some((at, item)) = progress.next() {
        let piste = commands
            .spawn((
                Node { position_type: PositionType::Absolute, left: percent(23.0), top: percent(73.8), width: percent(63.5), height: percent(4.2), border_radius: BorderRadius::all(px(8)), overflow: Overflow::clip(), ..default() },
                BackgroundColor(Color::srgb(0.83, 0.83, 0.8)),
                Pickable::IGNORE,
                ChildOf(panneau),
            ))
            .id();
        commands.spawn((
            Node { width: percent(progress.next_fraction().clamp(0.0, 1.0) * 100.0), height: percent(100), ..default() },
            BackgroundColor(item.color()),
            Pickable::IGNORE,
            ChildOf(piste),
        ));
        let ligne = tx!("Prochain : {}", "Next: {}", "Nächstes: {}", "Prossimo: {}", "Siguiente: {}").avec(&[&item.name()]);
        texte_abs(&mut commands, panneau, ligne, (23.0, 68.6, 44.0, 4.8), 21.0, brun, JustifyContent::FlexStart, None);
        let cumul = format!("{} / {} pts", progress.live.min(at), at);
        texte_abs(&mut commands, panneau, cumul, (62.0, 68.6, 24.5, 4.8), 20.0, Color::srgb(0.42, 0.38, 0.36), JustifyContent::FlexEnd, None);
    }
    // Boutons : posés sur ceux du panneau.
    let bouton = |commands: &mut Commands, x: f32, texte: &str, action: UiAction, couleur: Color| {
        let b = commands
            .spawn((
                Node { position_type: PositionType::Absolute, left: percent(x), top: percent(83.6), width: percent(30.8), height: percent(9.9), justify_content: JustifyContent::Center, align_items: AlignItems::Center, ..default() },
                action,
                UiTransform::default(),
                Survol,
                ChildOf(panneau),
            ))
            .id();
        commands.spawn((Text::new(texte), TextFont::from_font_size(30.0), TextColor(couleur), Pickable::IGNORE, ThemedText, ChildOf(b)));
        b
    };
    let rejouer = bouton(&mut commands, 17.3, tx!("Rejouer", "Play again", "Nochmal", "Rigioca", "Volver a jugar").t(), UiAction::Play, Color::srgb(0.38, 0.22, 0.02));
    commands.entity(rejouer).insert(Pulse { speed: 3.0, amount: 0.02 });
    bouton(&mut commands, 51.8, tx!("Menu", "Menu", "Menü", "Menu", "Menú").t(), UiAction::Menu, Color::srgb(0.25, 0.22, 0.2));
    // Ce que cette partie vient de débloquer : sous le panneau.
    if !progress.fresh.is_empty() {
        let ligne = commands.spawn((Node { column_gap: px(10), ..default() }, ChildOf(racine))).id();
        for (i, item) in progress.fresh.iter().enumerate() {
            spawn_chip(&mut commands, ligne, item.kind(), item.name(), item.color(), 0.7 + i as f32 * 0.3);
        }
    }
    sfx.write(if progress.new_record || !progress.fresh.is_empty() { Sfx::Win } else { Sfx::GameOver });
}

#[derive(Component)]
struct HudScore;
#[derive(Component)]
struct HudRecord;
#[derive(Component)]
struct HudSuivant;
#[derive(Component)]
struct HudCompte;
#[derive(Component)]
struct HudBarre;
/// Ligne « prochain déblocage » (cachée quand tout est débloqué).
#[derive(Component)]
struct HudLigneSuivant;

const CARTE: Color = Color::srgb(0.99, 0.97, 0.92);
const ENCRE: Color = Color::srgb(0.12, 0.14, 0.2);
const DOUX: Color = Color::srgb(0.46, 0.46, 0.5);

fn hud_texte(commands: &mut Commands, parent: Entity, texte: &str, taille: f32, couleur: Color) -> Entity {
    commands.spawn((Text::new(texte), TextFont::from_font_size(taille), TextColor(couleur), Pickable::IGNORE, ThemedText, ChildOf(parent))).id()
}

fn hud_icone(commands: &mut Commands, parent: Entity, image: Handle<Image>, cote: f32) {
    commands.spawn((ImageNode::new(image), Node { width: px(cote), height: px(cote), flex_shrink: 0.0, ..default() }, Pickable::IGNORE, ChildOf(parent)));
}

/// Bandeau de jeu : score et record, prochain déblocage, bouton menu.
fn setup_hud(mut commands: Commands, server: Res<AssetServer>) {
    let carte = commands
        .spawn((
            Node { position_type: PositionType::Absolute, top: px(14), left: px(14), width: px(420), flex_direction: FlexDirection::Column, row_gap: px(8), padding: UiRect::axes(px(18), px(12)), border_radius: BorderRadius::all(px(22)), ..default() },
            BackgroundColor(CARTE),
            BoxShadow::new(Color::srgba(0.0, 0.0, 0.0, 0.25), px(0), px(4), px(0), px(10)),
            GlobalZIndex(10),
            Pickable::IGNORE,
            Hud,
            Visibility::Hidden,
        ))
        .id();
    let haut = commands.spawn((Node { align_items: AlignItems::Center, column_gap: px(10), ..default() }, Pickable::IGNORE, ChildOf(carte))).id();
    let bloc = |commands: &mut Commands, icone: &'static str, titre: &str| -> Entity {
        hud_icone(commands, haut, server.load(icone), 40.0);
        let col = commands.spawn((Node { flex_direction: FlexDirection::Column, min_width: px(70), ..default() }, Pickable::IGNORE, ChildOf(haut))).id();
        hud_texte(commands, col, titre, 15.0, DOUX);
        hud_texte(commands, col, "0", 30.0, ENCRE)
    };
    let score = bloc(&mut commands, "branding/hud/etoile.png", tx!("Score", "Score", "Punkte", "Punti", "Puntos").t());
    commands.entity(score).insert(HudScore);
    commands.spawn((Node { width: px(2), height: px(44), margin: UiRect::horizontal(px(6)), ..default() }, BackgroundColor(Color::srgb(0.85, 0.83, 0.78)), Pickable::IGNORE, ChildOf(haut)));
    let record = bloc(&mut commands, "branding/hud/trophee.png", tx!("Record", "Best", "Rekord", "Record", "Récord").t());
    commands.entity(record).insert(HudRecord);
    // Prochain déblocage
    let ligne = commands.spawn((Node { flex_direction: FlexDirection::Column, row_gap: px(5), ..default() }, Pickable::IGNORE, HudLigneSuivant, ChildOf(carte))).id();
    let titres = commands.spawn((Node { justify_content: JustifyContent::SpaceBetween, align_items: AlignItems::Center, ..default() }, Pickable::IGNORE, ChildOf(ligne))).id();
    let gauche = commands.spawn((Node { align_items: AlignItems::Center, column_gap: px(7), ..default() }, Pickable::IGNORE, ChildOf(titres))).id();
    hud_icone(&mut commands, gauche, server.load("branding/hud/cadenas.png"), 18.0);
    let suivant = hud_texte(&mut commands, gauche, "", 15.0, ENCRE);
    commands.entity(suivant).insert(HudSuivant);
    let compte = hud_texte(&mut commands, titres, "", 15.0, DOUX);
    commands.entity(compte).insert(HudCompte);
    let piste = commands.spawn((Node { width: percent(100), height: px(12), border_radius: BorderRadius::all(px(6)), overflow: Overflow::clip(), ..default() }, BackgroundColor(Color::srgb(0.85, 0.83, 0.8)), Pickable::IGNORE, ChildOf(ligne))).id();
    commands.spawn((Node { width: percent(0), height: percent(100), ..default() }, BackgroundColor(Color::srgb(1.0, 0.65, 0.1)), Pickable::IGNORE, HudBarre, ChildOf(piste)));

    // Bouton menu (trois barres) : même effet que la touche Échap.
    let menu = commands
        .spawn((
            Node { position_type: PositionType::Absolute, top: px(14), right: px(14), width: px(62), height: px(62), flex_direction: FlexDirection::Column, justify_content: JustifyContent::Center, align_items: AlignItems::Center, row_gap: px(6), border_radius: BorderRadius::all(px(16)), ..default() },
            BackgroundColor(CARTE),
            BoxShadow::new(Color::srgba(0.0, 0.0, 0.0, 0.25), px(0), px(4), px(0), px(10)),
            GlobalZIndex(10),
            UiTransform::default(),
            Survol,
            UiAction::Stop,
            Hud,
            Visibility::Hidden,
        ))
        .id();
    for _ in 0..3 {
        commands.spawn((Node { width: px(30), height: px(5), border_radius: BorderRadius::all(px(3)), ..default() }, BackgroundColor(ENCRE), Pickable::IGNORE, ChildOf(menu)));
    }
}

#[allow(clippy::too_many_arguments)]
fn update_hud(
    score: Res<Score>,
    progress: Res<Progress>,
    state: Res<State<GameState>>,
    mut hud: Query<&mut Visibility, With<Hud>>,
    mut scores: Query<&mut Text, (With<HudScore>, Without<HudRecord>, Without<HudSuivant>, Without<HudCompte>)>,
    mut records: Query<&mut Text, (With<HudRecord>, Without<HudScore>, Without<HudSuivant>, Without<HudCompte>)>,
    mut suivants: Query<&mut Text, (With<HudSuivant>, Without<HudScore>, Without<HudRecord>, Without<HudCompte>)>,
    mut comptes: Query<&mut Text, (With<HudCompte>, Without<HudScore>, Without<HudRecord>, Without<HudSuivant>)>,
    mut barres: Query<&mut Node, With<HudBarre>>,
    mut lignes: Query<&mut Node, (With<HudLigneSuivant>, Without<HudBarre>)>,
) {
    let show = matches!(state.get(), GameState::Playing | GameState::Dying);
    for mut v in &mut hud {
        *v = if show { Visibility::Inherited } else { Visibility::Hidden };
    }
    if !show {
        return;
    }
    for mut t in &mut scores {
        t.0 = score.current.to_string();
    }
    for mut t in &mut records {
        t.0 = score.best.to_string();
    }
    let cumul = progress.total + score.current;
    let suite = crate::progress::UNLOCKS.iter().filter(|(t, _)| cumul < *t).min_by_key(|(t, _)| *t).copied();
    for mut n in &mut lignes {
        n.display = if suite.is_some() { Display::Flex } else { Display::None };
    }
    if let Some((at, item)) = suite {
        let fraction = cumul as f32 / at.max(1) as f32;
        for mut t in &mut suivants {
            t.0 = tx!("Prochain déblocage : {}", "Next unlock: {}", "Nächste Freischaltung: {}", "Prossimo sblocco: {}", "Próximo desbloqueo: {}").avec(&[&item.name()]);
        }
        for mut t in &mut comptes {
            t.0 = format!("{} / {}", cumul, at);
        }
        for mut n in &mut barres {
            n.width = percent(fraction.clamp(0.0, 1.0) * 100.0);
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
        GameState::Playing if back => next.set(GameState::GameOver),
        GameState::GameOver if confirm => next.set(GameState::Playing),
        GameState::GameOver if back => next.set(GameState::Menu),
        _ => {}
    }
}

fn on_click(
    click: On<PointerClick>,
    actions: Query<&UiAction>,
    mut next: ResMut<NextState<GameState>>,
    mut sfx: MessageWriter<Sfx>,
) {
    let Ok(action) = actions.get(click.entity) else {
        return;
    };
    sfx.write(Sfx::Click);
    match *action {
        UiAction::Play => next.set(GameState::Playing),
        UiAction::Garage => next.set(GameState::Garage),
        UiAction::Menu => next.set(GameState::Menu),
        UiAction::Stop => next.set(GameState::GameOver),
    }
}
