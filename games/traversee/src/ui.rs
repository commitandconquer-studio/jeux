//! Écrans : accueil, fin de partie, et affichage du score.
use bevy::prelude::*;
use game_common::Score;
use game_common::tx;
use game_common::ui::{GameHelp, HelpButton, QuitButton, ThemedPanel, ThemedText, spawn_button, spawn_button_sized, spawn_gap, spawn_hud, spawn_panel, spawn_panel_left, spawn_studio_logo, spawn_text};

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
}

#[derive(Component)]
struct Hud;

#[derive(Component)]
struct HudText;

fn button(commands: &mut Commands, parent: Entity, label: &str, action: UiAction) -> Entity {
    let (b, text) = spawn_button(commands, parent, label);
    commands.entity(b).insert(action);
    text
}

/// Couleurs des lettres du titre.

/// Ligne « Prochain : … à N pts » suivie de sa barre de progression.
fn spawn_next_goal(commands: &mut Commands, parent: Entity, progress: &Progress, size: f32, width: f32) {
    let Some((at, item)) = progress.next() else {
        return;
    };
    let line = tx!("Prochain : {} — {} pts", "Next: {} — {} pts", "Nächstes: {} — {} Pkt.", "Prossimo: {} — {} pt", "Siguiente: {} — {} pts").avec(&[&item.name(), &at]);
    spawn_text(commands, parent, line, size);
    spawn_bar(commands, parent, progress.next_fraction(), width, item.color());
}

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

fn spawn_menu(mut commands: Commands, progress: Res<Progress>, score: Res<Score>, server: Res<AssetServer>) {
    let panel = spawn_panel_left(&mut commands, DespawnOnExit(GameState::Menu));
    // Panneau « verre » bleu translucide qui se fond vers le rose (et non plus le panneau crème du thème).
    commands
        .entity(panel)
        .remove::<ThemedPanel>()
        .insert((
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                row_gap: px(9),
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
    commands.spawn((ImageNode::new(server.load("branding/cross-or-die.png")), Node { width: px(360), height: px(120), ..default() }, ChildOf(panel)));

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

    // Les boutons.
    let jouer = gros_bouton(&mut commands, panel, tx!("Jouer", "Play", "Spielen", "Gioca", "Jugar").t(), Some(server.load("branding/menu/jouer.png")), Color::srgb(0.2, 0.72, 0.27), 420.0, 72.0, 40.0);
    commands.entity(jouer).insert((UiAction::Play, Pulse { speed: 3.0, amount: 0.03 })).remove::<Survol>();
    let collection = gros_bouton(&mut commands, panel, tx!("Collection", "Collection", "Sammlung", "Collezione", "Colección").t(), Some(server.load("branding/menu/collection.png")), Color::srgb(0.16, 0.5, 0.96), 380.0, 56.0, 28.0);
    commands.entity(collection).insert(UiAction::Garage);
    let options = gros_bouton(&mut commands, panel, tx!("Options", "Options", "Optionen", "Opzioni", "Opciones").t(), Some(server.load("branding/menu/options.png")), Color::srgb(0.56, 0.34, 0.86), 380.0, 56.0, 28.0);
    commands.entity(options).insert(game_common::options::OptionsBouton);
    let aide = gros_bouton(&mut commands, panel, tx!("Aide", "Help", "Hilfe", "Aiuto", "Ayuda").t(), Some(server.load("branding/menu/aide.png")), Color::srgb(0.98, 0.62, 0.12), 380.0, 56.0, 28.0);
    commands.entity(aide).insert(HelpButton);
    #[cfg(not(any(target_os = "android", target_arch = "wasm32")))]
    {
        let quitter = gros_bouton(&mut commands, panel, tx!("Quitter", "Quit", "Beenden", "Esci", "Salir").t(), None, Color::srgb(0.86, 0.32, 0.34), 220.0, 42.0, 20.0);
        commands.entity(quitter).insert(QuitButton);
    }
    // Filet et logo du studio.
    commands.spawn((Node { width: px(400), height: px(2), margin: UiRect::top(px(2)), ..default() }, BackgroundColor(Color::srgba(1.0, 1.0, 1.0, 0.7)), ChildOf(panel)));
    spawn_studio_logo(&mut commands, panel);
}

/// « Score : 12    Record : 40 », dans la langue en cours.
fn score_line(score: &Score) -> String {
    tx!(
        "Score : {}    Record : {}",
        "Score: {}    Best: {}",
        "Punkte: {}    Rekord: {}",
        "Punti: {}    Record: {}",
        "Puntos: {}    Récord: {}"
    )
    .avec(&[&score.current, &score.best])
}

pub fn spawn_game_over(mut commands: Commands, score: Res<Score>, death: Res<Death>, progress: Res<Progress>, mut sfx: MessageWriter<Sfx>) {
    let panel = spawn_panel(&mut commands, DespawnOnExit(GameState::GameOver));
    commands.entity(panel).insert(UiTransform::from_scale(Vec2::ZERO)).insert(Pop::after(0.0));
    let title = death.cause.map(|c| c.message()).unwrap_or(tx!("Perdu !", "Game over!", "Verloren!", "Hai perso!", "¡Has perdido!").t());
    spawn_text(&mut commands, panel, title, 46.0);
    // Le score grimpe jusqu'à sa valeur
    let row = commands.spawn((Node { align_items: AlignItems::FlexEnd, column_gap: px(10), ..default() }, ChildOf(panel))).id();
    commands.spawn((
        Text::new("0"),
        TextFont::from_font_size(96.0),
        TextColor(Color::srgb(0.12, 0.14, 0.2)),
        ThemedText,
        CountUp { target: score.current, age: 0.0 },
        ChildOf(row),
    ));
    commands.spawn((Text::new("pts"), TextFont::from_font_size(30.0), TextColor(Color::srgb(0.4, 0.4, 0.45)), ThemedText, Node { margin: UiRect::bottom(px(14)), ..default() }, ChildOf(row)));
    if progress.new_record {
        let badge = commands
            .spawn((
                Node { padding: UiRect::axes(px(22), px(8)), border_radius: BorderRadius::all(px(16)), ..default() },
                BackgroundColor(Color::srgb(1.0, 0.82, 0.2)),
                UiTransform::default(),
                Pulse { speed: 6.0, amount: 0.06 },
                ChildOf(panel),
            ))
            .id();
        commands.spawn((
            Text::new(tx!("NOUVEAU RECORD !", "NEW RECORD!", "NEUER REKORD!", "NUOVO RECORD!", "¡NUEVO RÉCORD!").t()),
            TextFont::from_font_size(28.0),
            TextColor(Color::srgb(0.35, 0.2, 0.0)),
            ThemedText,
            ChildOf(badge),
        ));
    } else {
        spawn_text(&mut commands, panel, tx!("Record : {}", "Best: {}", "Rekord: {}", "Record: {}", "Récord: {}").avec(&[&score.best]), 22.0);
    }
    // Ce que cette partie vient de débloquer
    for (i, item) in progress.fresh.iter().enumerate() {
        spawn_chip(&mut commands, panel, item.kind(), item.name(), item.color(), 0.7 + i as f32 * 0.3);
    }
    spawn_next_goal(&mut commands, panel, &progress, 18.0, 400.0);
    sfx.write(if progress.new_record || !progress.fresh.is_empty() { Sfx::Win } else { Sfx::GameOver });
    spawn_gap(&mut commands, panel, 6.0);
    let (again, _) = spawn_button_sized(&mut commands, panel, tx!("Rejouer", "Play again", "Nochmal", "Rigioca", "Volver a jugar").t(), 320.0, 64.0, 30.0);
    commands.entity(again).insert((
        UiAction::Play,
        BackgroundColor(Color::srgb(1.0, 0.86, 0.3)),
        UiTransform::default(),
        Pulse { speed: 3.0, amount: 0.025 },
    ));
    button(&mut commands, panel, tx!("Menu", "Menu", "Menü", "Menu", "Menú").t(), UiAction::Menu);
}

fn setup_hud(mut commands: Commands) {
    let (hud, text) = spawn_hud(&mut commands);
    commands.entity(hud).insert((Hud, Visibility::Hidden));
    commands.entity(text).insert(HudText);
}

fn update_hud(
    score: Res<Score>,
    state: Res<State<GameState>>,
    mut hud: Query<&mut Visibility, With<Hud>>,
    mut text: Query<&mut Text, With<HudText>>,
) {
    let show = matches!(state.get(), GameState::Playing | GameState::Dying);
    for mut v in &mut hud {
        *v = if show { Visibility::Inherited } else { Visibility::Hidden };
    }
    if score.is_changed() {
        for mut t in &mut text {
            let son = tx!("M : son", "M: sound", "M: Ton", "M: audio", "M: sonido").t();
            t.0 = format!("{}    {}", score_line(&score), son);
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
    }
}
