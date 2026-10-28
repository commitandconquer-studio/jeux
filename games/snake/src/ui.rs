//! Écrans : accueil, réglages, fin de partie, et affichage du score.
use bevy::prelude::*;
use game_common::langue::Langue;
use game_common::ui::{GameHelp, spawn_button, spawn_gap, spawn_hud, spawn_panel, spawn_text};
use game_common::{Score, tx};

use crate::GameState;
use crate::audio::Sfx;
use crate::game::{Banner, Paused};
use crate::progress::{Progress, label};
use crate::levels::{LEVELS, Place};
use crate::settings::{OptionKind, Settings};
use crate::voyage::{Mode, Outcome, Run, VoyageProgress};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_hud)
            .insert_resource(GameHelp(tx!(
                "Menez la chenille comme dans un bal populaire :\n\
attrapez les invités, ils s'accrochent derrière vous.\n\
Ne vous mordez pas la queue !\n\
\n\
Flèches ou ZQSD (ou glisser le doigt) : diriger\n\
P ou Espace : pause · Échap : menu · M : son\n\
\n\
Téléporteurs : entrez dans l'un, sortez de l'autre.\n\
Accélérateurs : un coup de vitesse.\n\
Tapis roulants : impossible d'en sortir.\n\
Pierres et trous : attention !\n\
\n\
Plus vous attrapez d'invités, plus vous débloquez\n\
de personnages, de paysages et de cases.",
                "Lead the conga line like at a village dance:\n\
catch the guests, they join the line behind you.\n\
Don't bite your own tail!\n\
\n\
Arrows or WASD (or swipe): steer\n\
P or Space: pause · Esc: menu · M: mute\n\
\n\
Teleporters: go in one, come out of the other.\n\
Speed pads: a burst of speed.\n\
Conveyor belts: you can't leave them.\n\
Rocks and holes: watch out!\n\
\n\
The more guests you catch, the more characters,\n\
sceneries and tiles you unlock.",
                "Führe die Polonäse wie auf dem Dorffest:\n\
fang die Gäste, sie reihen sich hinter dir ein.\n\
Beiß dir nicht in den Schwanz!\n\
\n\
Pfeiltasten oder WASD (oder wischen): steuern\n\
P oder Leertaste: Pause · Esc: Menü · M: Ton aus\n\
\n\
Teleporter: rein in den einen, raus aus dem anderen.\n\
Turbo-Felder: ein Schub Tempo.\n\
Förderbänder: kein Entkommen.\n\
Steine und Löcher: Vorsicht!\n\
\n\
Je mehr Gäste du fängst, desto mehr Figuren,\n\
Landschaften und Felder schaltest du frei.",
                "Guida il trenino come a una festa di paese:\n\
prendi gli invitati, si attaccano dietro di te.\n\
Non morderti la coda!\n\
\n\
Frecce o WASD (o scorri col dito): dirigere\n\
P o Spazio: pausa · Esc: menu · M: audio\n\
\n\
Teletrasporti: entri in uno, esci dall'altro.\n\
Acceleratori: una sferzata di velocità.\n\
Nastri trasportatori: non se ne esce.\n\
Sassi e buche: attenzione!\n\
\n\
Più invitati prendi, più personaggi,\n\
paesaggi e caselle sblocchi.",
                "Guía el trenecito como en una fiesta de pueblo:\n\
atrapa a los invitados, se enganchan detrás de ti.\n\
¡No te muerdas la cola!\n\
\n\
Flechas o WASD (o deslizar el dedo): dirigir\n\
P o Espacio: pausa · Esc: menú · M: silenciar\n\
\n\
Teletransportadores: entras por uno, sales por el otro.\n\
Aceleradores: un empujón de velocidad.\n\
Cintas transportadoras: no se puede salir.\n\
Piedras y hoyos: ¡cuidado!\n\
\n\
Cuantos más invitados atrapes, más personajes,\n\
paisajes y casillas desbloqueas."
            )))
            .add_systems(OnEnter(GameState::Menu), spawn_menu)
            .add_systems(OnEnter(GameState::Options), spawn_options)
            .insert_resource(VoyageTab { selected: Place::Garden, built: None })
            .add_systems(OnEnter(GameState::Voyage), (pick_tab, spawn_voyage).chain())
            .add_systems(Update, redraw_voyage)
            .add_observer(plank_over)
            .add_observer(plank_out)
            .add_systems(OnEnter(GameState::GameOver), spawn_game_over)
            .add_systems(Update, (update_hud, update_banner, refresh_option_texts, keyboard_shortcuts))
            .add_observer(on_click);
    }
}

/// Ce que fait un bouton quand on clique dessus.
#[derive(Component, Clone, Copy)]
enum UiAction {
    /// Mode infini.
    Play,
    /// Recommencer le niveau en cours.
    Replay,
    /// Carte du Voyage.
    Voyage,
    /// Liste des aventures.
    Scenarios,
    Level(usize),
    Tab(Place),
    Locked,
    Next,
    Settings,
    Back,
    Menu,
    Cycle(OptionKind),
}

/// Texte d'un bouton d'option, mis à jour quand la valeur change.
#[derive(Component)]
struct OptionText(OptionKind);

#[derive(Component)]
struct Hud;

#[derive(Component)]
struct HudText;

fn button(commands: &mut Commands, parent: Entity, label: impl Into<String>, action: UiAction) -> Entity {
    let (b, text) = spawn_button(commands, parent, label);
    commands.entity(b).insert(action);
    text
}

/// Un panneau de bois du menu (image du kit UI) : icône à gauche, titre et sous-titre à droite.
#[derive(Component)]
struct Plank;

/// Pose un panneau cliquable ; renvoie `(panneau, texte du titre)`.
#[allow(clippy::too_many_arguments)]
pub(crate) fn plank(commands: &mut Commands, server: &AssetServer, parent: Entity, board: &str, icon: &str, width: f32, label: impl Into<String>, subtitle: Option<&str>) -> (Entity, Entity) {
    let cream = Color::srgb(1.0, 0.96, 0.84);
    let b = commands
        .spawn((
            Node { width: Val::Px(width), height: Val::Px(width * 0.26), align_items: AlignItems::Center, padding: UiRect::axes(Val::Px(width * 0.07), Val::Px(0.0)), column_gap: Val::Px(width * 0.035), ..default() },
            ImageNode::new(server.load(format!("ui/suivez/{board}.png"))).with_color(Color::srgb(0.93, 0.93, 0.93)),
            Plank,
            ChildOf(parent),
        ))
        .id();
    commands.spawn((Node { width: Val::Px(width * 0.14), height: Val::Px(width * 0.14), ..default() }, ImageNode::new(server.load(format!("ui/suivez/{icon}.png"))), Pickable::IGNORE, ChildOf(b)));
    let col = commands.spawn((Node { flex_direction: FlexDirection::Column, ..default() }, Pickable::IGNORE, ChildOf(b))).id();
    let t = spawn_text(commands, col, label, (width * 0.075).clamp(16.0, 27.0));
    commands.entity(t).insert((TextColor(cream), TextShadow { offset: Vec2::splat(2.0), color: Color::srgba(0.12, 0.07, 0.04, 0.9) }, TextLayout::justify(Justify::Left)));
    if let Some(sub) = subtitle {
        let st = spawn_text(commands, col, sub, (width * 0.042).clamp(12.0, 15.0));
        commands.entity(st).insert((TextColor(Color::srgb(1.0, 0.92, 0.74)), TextLayout::justify(Justify::Left)));
    }
    (b, t)
}

/// Les panneaux s'éclaircissent sous la souris.
fn plank_over(over: On<PointerOver>, mut q: Query<&mut ImageNode, With<Plank>>) {
    if let Ok(mut img) = q.get_mut(over.entity) {
        img.color = Color::WHITE;
    }
}

fn plank_out(out: On<PointerOut>, mut q: Query<&mut ImageNode, With<Plank>>) {
    if let Ok(mut img) = q.get_mut(out.entity) {
        img.color = Color::srgb(0.93, 0.93, 0.93);
    }
}

fn spawn_menu(mut commands: Commands, settings: Res<Settings>, progress: Res<Progress>, server: Res<AssetServer>) {
    let root = commands
        .spawn((
            Node { width: Val::Percent(100.0), height: Val::Percent(100.0), flex_direction: FlexDirection::Column, align_items: AlignItems::Center, justify_content: JustifyContent::Center, row_gap: Val::Px(8.0), padding: UiRect::all(Val::Px(16.0)), ..default() },
            DespawnOnExit(GameState::Menu),
        ))
        .id();
    commands.spawn((Node { width: Val::Px(380.0), height: Val::Px(191.0), margin: UiRect::bottom(Val::Px(6.0)), ..default() }, ImageNode::new(server.load("ui/suivez/logo_complet.png")), Pickable::IGNORE, ChildOf(root)));
    let (b, _) = plank(&mut commands, &server, root, "panneau_bois_dore_large", "icone_carte", 340.0, tx!("Mode scénario", "Story mode", "Story-Modus", "Modalità storia", "Modo historia").t(), Some(tx!("Des aventures à vivre", "Adventures to live", "Abenteuer erleben", "Avventure da vivere", "Aventuras por vivir").t()));
    commands.entity(b).insert(UiAction::Scenarios);
    let (b, _) = plank(&mut commands, &server, root, "panneau_bois_fonce_large", "icone_infini", 340.0, tx!("Mode infini", "Endless mode", "Endlos-Modus", "Modalità infinita", "Modo infinito").t(), Some(tx!("Toujours plus nombreux !", "More and more guests!", "Immer mehr Gäste!", "Sempre più numerosi!", "¡Cada vez más!").t()));
    commands.entity(b).insert(UiAction::Play);
    let (b, t) = plank(&mut commands, &server, root, "panneau_bois_fonce_large", "icone_jumelles", 340.0, OptionKind::Universe.text(&settings, &progress), None);
    commands.entity(b).insert(UiAction::Cycle(OptionKind::Universe));
    commands.entity(t).insert(OptionText(OptionKind::Universe));
    // Petits panneaux deux par deux : Réglages / Options, puis Aide / Quitter.
    let row = |commands: &mut Commands| commands.spawn((Node { column_gap: Val::Px(10.0), ..default() }, ChildOf(root))).id();
    let r1 = row(&mut commands);
    let (b, _) = plank(&mut commands, &server, r1, "panneau_bois_fonce_large", "icone_reglages", 230.0, tx!("Réglages", "Settings", "Einstellungen", "Impostazioni", "Ajustes").t(), None);
    commands.entity(b).insert(UiAction::Settings);
    let (b, _) = plank(&mut commands, &server, r1, "panneau_bois_fonce_large", "icone_boussole", 230.0, tx!("Options", "Options", "Optionen", "Opzioni", "Opciones").t(), None);
    commands.entity(b).insert(game_common::options::OptionsBouton);
    let r2 = row(&mut commands);
    let (b, _) = plank(&mut commands, &server, r2, "panneau_bois_fonce_large", "icone_aide", 230.0, tx!("Aide", "Help", "Hilfe", "Aiuto", "Ayuda").t(), None);
    commands.entity(b).insert(game_common::ui::HelpButton);
    #[cfg(not(any(target_os = "android", target_arch = "wasm32")))]
    {
        let (b, _) = plank(&mut commands, &server, r2, "panneau_bois_fonce_large", "icone_quitter", 230.0, tx!("Quitter", "Quit", "Beenden", "Esci", "Salir").t(), None);
        commands.entity(b).insert(game_common::ui::QuitButton);
    }
    // Total d'invités et prochain déblocage, en bas à gauche.
    let tag = commands
        .spawn((
            Node { position_type: PositionType::Absolute, left: Val::Px(20.0), bottom: Val::Px(20.0), flex_direction: FlexDirection::Column, padding: UiRect::axes(Val::Px(18.0), Val::Px(10.0)), border_radius: BorderRadius::all(Val::Px(16.0)), ..default() },
            BackgroundColor(Color::srgba(0.16, 0.18, 0.24, 0.85)),
            ChildOf(root),
        ))
        .id();
    let light = |commands: &mut Commands, t: Entity| {
        commands.entity(t).insert((TextColor(Color::srgb(0.97, 0.97, 0.97)), TextLayout::justify(Justify::Left)));
    };
    let t = spawn_text(&mut commands, tag, tx!("Invités attrapés : {}", "Guests caught: {}", "Gefangene Gäste: {}", "Invitati presi: {}", "Invitados atrapados: {}").avec(&[&progress.total]), 20.0);
    light(&mut commands, t);
    if let Some(next) = progress.next_text() {
        let t = spawn_text(&mut commands, tag, next, 15.0);
        light(&mut commands, t);
    }
    // Le logo du studio sur un bloc clair : sur le décor il serait illisible.
    let block = commands
        .spawn((
            Node { position_type: PositionType::Absolute, right: Val::Px(20.0), bottom: Val::Px(20.0), padding: UiRect::axes(Val::Px(16.0), Val::Px(8.0)), border_radius: BorderRadius::all(Val::Px(16.0)), ..default() },
            BackgroundColor(Color::srgb(0.98, 0.96, 0.9)),
            ChildOf(root),
        ))
        .id();
    game_common::ui::spawn_studio_logo(&mut commands, block);
}

fn stars_text(n: u8) -> String {
    (0..3).map(|i| if i < n { '*' } else { '-' }).collect()
}

/// L'onglet (le lieu) affiché sur la carte du Voyage.
#[derive(Resource)]
struct VoyageTab {
    selected: Place,
    built: Option<Place>,
}

#[derive(Component)]
struct VoyagePanel;

/// À l'arrivée sur la carte : on ouvre le lieu du niveau qu'on vient de jouer.
fn pick_tab(mode: Res<Mode>, mut tab: ResMut<VoyageTab>) {
    if let Mode::Voyage(i) = *mode
        && let Some(level) = LEVELS.get(i)
    {
        tab.selected = level.place;
    }
}

/// La carte du Voyage : un onglet par lieu, puis 12 niveaux en cartes (4 colonnes × 3 lignes).
fn spawn_voyage(mut commands: Commands, voyage: Res<VoyageProgress>, mut tab: ResMut<VoyageTab>, mode: Res<Mode>, server: Res<AssetServer>) {
    tab.built = Some(tab.selected);
    let img = |name: &str| server.load(format!("ui/suivez/{name}.png"));
    let root = commands
        .spawn((
            Node { width: Val::Percent(100.0), height: Val::Percent(100.0), flex_direction: FlexDirection::Column, align_items: AlignItems::Center, justify_content: JustifyContent::Center, row_gap: Val::Px(6.0), padding: UiRect::all(Val::Px(10.0)), ..default() },
            DespawnOnExit(GameState::Voyage),
            VoyagePanel,
        ))
        .id();
    // Décors aux coins (derrière le reste).
    commands.spawn((Node { position_type: PositionType::Absolute, left: Val::Px(0.0), bottom: Val::Px(0.0), width: Val::Px(260.0), height: Val::Px(124.0), ..default() }, ImageNode::new(img("decor_bas_gauche")), Pickable::IGNORE, ChildOf(root)));
    commands.spawn((Node { position_type: PositionType::Absolute, right: Val::Px(0.0), bottom: Val::Px(0.0), width: Val::Px(260.0), height: Val::Px(124.0), ..default() }, ImageNode { flip_x: true, ..ImageNode::new(img("decor_bas_gauche")) }, Pickable::IGNORE, ChildOf(root)));
    commands.spawn((Node { width: Val::Px(360.0), height: Val::Px(111.0), ..default() }, ImageNode::new(img("logo_selection")).with_rect(Rect::new(0.0, 0.0, 764.0, 236.0)), Pickable::IGNORE, ChildOf(root)));
    let board = commands
        .spawn((
            Node { flex_direction: FlexDirection::Column, align_items: AlignItems::Center, row_gap: Val::Px(8.0), padding: UiRect::all(Val::Px(12.0)), border: UiRect::all(Val::Px(4.0)), border_radius: BorderRadius::all(Val::Px(18.0)), ..default() },
            BackgroundColor(Color::srgb(0.66, 0.43, 0.24)),
            BorderColor::all(Color::srgb(0.43, 0.27, 0.14)),
            ChildOf(root),
        ))
        .id();
    // Onglets : un par lieu (en deux lignes si l'écran est étroit).
    let tabs = commands
        .spawn((Node { flex_wrap: FlexWrap::Wrap, justify_content: JustifyContent::Center, column_gap: Val::Px(6.0), row_gap: Val::Px(4.0), width: Val::Px(940.0), ..default() }, ChildOf(board)))
        .id();
    for place in Place::ALL {
        let levels: Vec<usize> = LEVELS.iter().enumerate().filter(|(_, l)| l.place == place).map(|(i, _)| i).collect();
        let open = levels.first().is_some_and(|i| voyage.unlocked(*i));
        let selected = place == tab.selected;
        let b = commands
            .spawn((
                Node { width: Val::Px(128.0), height: Val::Px(36.0), justify_content: JustifyContent::Center, align_items: AlignItems::Center, ..default() },
                ImageNode::new(img(if selected { "panneau_bois_dore_large" } else { "panneau_bois_fonce_large" })).with_color(if open { Color::WHITE } else { Color::srgb(0.5, 0.5, 0.5) }),
                if open { UiAction::Tab(place) } else { UiAction::Locked },
                ChildOf(tabs),
            ))
            .id();
        if open && !selected {
            commands.entity(b).insert(Plank);
        }
        let t = spawn_text(&mut commands, b, place.label(), 12.0);
        commands.entity(t).insert(TextColor(if selected { Color::srgb(0.30, 0.17, 0.05) } else { Color::srgb(1.0, 0.96, 0.84) }));
    }
    // Contenu : à gauche le panneau du lieu, à droite les 12 cartes.
    let content = commands.spawn((Node { column_gap: Val::Px(14.0), align_items: AlignItems::Center, ..default() }, ChildOf(board))).id();
    let levels: Vec<usize> = LEVELS.iter().enumerate().filter(|(_, l)| l.place == tab.selected).map(|(i, _)| i).collect();
    let side = commands.spawn((Node { width: Val::Px(210.0), flex_direction: FlexDirection::Column, align_items: AlignItems::Center, row_gap: Val::Px(8.0), ..default() }, ChildOf(content))).id();
    let sign = commands
        .spawn((
            Node { width: Val::Px(210.0), height: Val::Px(84.0), justify_content: JustifyContent::Center, align_items: AlignItems::Center, padding: UiRect::all(Val::Px(18.0)), ..default() },
            ImageNode::new(img("panneau_bois_dore_large")),
            ChildOf(side),
        ))
        .id();
    let t = spawn_text(&mut commands, sign, tab.selected.label(), 19.0);
    commands.entity(t).insert(TextColor(Color::srgb(0.30, 0.17, 0.05)));
    let done = levels.iter().filter(|i| voyage.stars(**i) > 0).count();
    let t = spawn_text(&mut commands, side, tx!("Niveaux réussis : {} / {}", "Levels done: {} / {}", "Geschafft: {} / {}", "Livelli superati: {} / {}", "Niveles superados: {} / {}").avec(&[&done, &levels.len()]), 16.0);
    commands.entity(t).insert(TextColor(Color::srgb(1.0, 0.96, 0.84)));
    let t = spawn_text(&mut commands, side, tx!("Étoiles : {}", "Stars: {}", "Sterne: {}", "Stelle: {}", "Estrellas: {}").avec(&[&voyage.total_stars()]), 16.0);
    commands.entity(t).insert(TextColor(Color::srgb(1.0, 0.9, 0.55)));
    let grid = commands
        .spawn((Node { flex_wrap: FlexWrap::Wrap, justify_content: JustifyContent::Center, column_gap: Val::Px(8.0), row_gap: Val::Px(8.0), width: Val::Px(4.0 * 148.0 + 3.0 * 8.0), ..default() }, ChildOf(content)))
        .id();
    let playing = if let Mode::Voyage(i) = *mode { Some(i) } else { None };
    for slot in 0..12 {
        let level = levels.get(slot).copied();
        let open = level.is_some_and(|i| voyage.unlocked(i));
        let skin = match level {
            Some(i) if open && playing == Some(i) => "carte_vierge_selectionne",
            Some(_) if open => "carte_vierge_disponible",
            _ => "carte_vierge_verrouille",
        };
        let card = commands
            .spawn((
                Node { width: Val::Px(148.0), height: Val::Px(128.0), flex_direction: FlexDirection::Column, align_items: AlignItems::Center, justify_content: JustifyContent::Center, row_gap: Val::Px(2.0), padding: UiRect::axes(Val::Px(10.0), Val::Px(8.0)), ..default() },
                ImageNode::new(img(skin)),
                match level {
                    Some(i) if open => UiAction::Level(i),
                    _ => UiAction::Locked,
                },
                ChildOf(grid),
            ))
            .id();
        if open {
            commands.entity(card).insert(Plank);
            let badge = commands
                .spawn((
                    Node { position_type: PositionType::Absolute, left: Val::Px(-4.0), top: Val::Px(-4.0), width: Val::Px(26.0), height: Val::Px(26.0), justify_content: JustifyContent::Center, align_items: AlignItems::Center, border: UiRect::all(Val::Px(2.0)), border_radius: BorderRadius::MAX, ..default() },
                    BackgroundColor(Color::srgb(1.0, 0.82, 0.2)),
                    BorderColor::all(Color::srgb(0.85, 0.5, 0.1)),
                    Pickable::IGNORE,
                    ChildOf(card),
                ))
                .id();
            let t = spawn_text(&mut commands, badge, format!("{}", slot + 1), 15.0);
            commands.entity(t).insert(TextColor(Color::srgb(0.3, 0.17, 0.05)));
            let stars = commands.spawn((Node { column_gap: Val::Px(2.0), ..default() }, Pickable::IGNORE, ChildOf(card))).id();
            let n = voyage.stars(level.expect("niveau ouvert"));
            for k in 0..3 {
                commands.spawn((Node { width: Val::Px(30.0), height: Val::Px(30.0), ..default() }, ImageNode::new(img(if k < n { "icone_etoile_doree" } else { "icone_etoile_grise" })), Pickable::IGNORE, ChildOf(stars)));
            }
            let t = spawn_text(&mut commands, card, LEVELS[level.expect("niveau ouvert")].name.clone(), 13.0);
            commands.entity(t).insert(TextColor(Color::srgb(0.30, 0.17, 0.05)));
        } else if level.is_some() {
            commands.spawn((Node { width: Val::Px(46.0), height: Val::Px(46.0), ..default() }, ImageNode::new(img("icone_cadenas")), Pickable::IGNORE, ChildOf(card)));
            let t = spawn_text(&mut commands, card, format!("{}", slot + 1), 15.0);
            commands.entity(t).insert(TextColor(Color::srgb(0.75, 0.78, 0.85)));
        }
    }
    let (b, _) = plank(&mut commands, &server, root, "panneau_bois_fonce_large", "fleche_precedent", 220.0, tx!("Retour", "Back", "Zurück", "Indietro", "Volver").t(), None);
    commands.entity(b).insert(UiAction::Menu);
}

/// Changement d'onglet : on redessine la carte.
fn redraw_voyage(
    mut commands: Commands,
    voyage: Res<VoyageProgress>,
    tab: ResMut<VoyageTab>,
    mode: Res<Mode>,
    server: Res<AssetServer>,
    panels: Query<Entity, With<VoyagePanel>>,
    state: Res<State<GameState>>,
) {
    if *state.get() != GameState::Voyage || tab.built == Some(tab.selected) {
        return;
    }
    for e in &panels {
        commands.entity(e).despawn();
    }
    spawn_voyage(commands, voyage, tab, mode, server);
}

fn spawn_options(mut commands: Commands, settings: Res<Settings>, progress: Res<Progress>) {
    let panel = spawn_panel(&mut commands, DespawnOnExit(GameState::Options));
    spawn_text(&mut commands, panel, tx!("Réglages", "Settings", "Einstellungen", "Impostazioni", "Ajustes").t(), 48.0);
    for kind in OptionKind::ALL {
        let t = button(&mut commands, panel, kind.text(&settings, &progress), UiAction::Cycle(kind));
        commands.entity(t).insert(OptionText(kind));
    }
    spawn_gap(&mut commands, panel, 6.0);
    button(&mut commands, panel, tx!("Retour", "Back", "Zurück", "Indietro", "Volver").t(), UiAction::Back);
}

fn spawn_game_over(mut commands: Commands, score: Res<Score>, progress: Res<Progress>, mode: Res<Mode>, outcome: Res<Outcome>, run: Res<Run>) {
    let panel = spawn_panel(&mut commands, DespawnOnExit(GameState::GameOver));
    // Aventure : un niveau réussi enchaîne tout de suite ; un niveau raté se rejoue.
    if let (Mode::Scenario { .. }, Some(level)) = (*mode, mode.level()) {
        if outcome.won {
            return;
        }
        spawn_text(&mut commands, panel, tx!("Raté !", "Missed!", "Verpasst!", "Mancato!", "¡Fallaste!").t(), 56.0);
        spawn_text(
            &mut commands,
            panel,
            tx!("Touristes recrutés : {} sur {}", "Tourists recruited: {} of {}", "Touristen gewonnen: {} von {}", "Turisti reclutati: {} su {}", "Turistas reclutados: {} de {}")
                .avec(&[&run.recruited, &level.tourists]),
            24.0,
        );
        spawn_gap(&mut commands, panel, 6.0);
        button(&mut commands, panel, tx!("Rejouer", "Play again", "Nochmal", "Rigioca", "Otra vez").t(), UiAction::Replay);
        button(&mut commands, panel, tx!("Mode scénario", "Story mode", "Story-Modus", "Modalità storia", "Modo historia").t(), UiAction::Scenarios);
        return;
    }
    if let Mode::Voyage(i) = *mode {
        let level = &LEVELS[i];
        if outcome.won {
            spawn_text(&mut commands, panel, tx!("Niveau réussi !", "Level complete!", "Geschafft!", "Livello superato!", "¡Nivel superado!").t(), 48.0);
            spawn_text(&mut commands, panel, format!("{}  {}", level.name, stars_text(outcome.stars)), 28.0);
            spawn_text(
                &mut commands,
                panel,
                tx!("Temps : {} s    Objectif des 2 étoiles : {} s", "Time: {} s    2-star goal: {} s", "Zeit: {} s    Ziel für 2 Sterne: {} s", "Tempo: {} s    Obiettivo 2 stelle: {} s", "Tiempo: {} s    Objetivo 2 estrellas: {} s")
                    .avec(&[&(outcome.time.round() as u32), &(level.par.round() as u32)]),
                20.0,
            );
            if outcome.stars > outcome.best_before && outcome.best_before > 0 {
                spawn_text(&mut commands, panel, tx!("Nouveau meilleur résultat !", "New best!", "Neue Bestleistung!", "Nuovo record!", "¡Nuevo récord!").t(), 22.0);
            }
        } else {
            spawn_text(&mut commands, panel, tx!("Raté !", "Missed!", "Verpasst!", "Mancato!", "¡Fallaste!").t(), 56.0);
            spawn_text(
                &mut commands,
                panel,
                tx!("Touristes recrutés : {} sur {}", "Tourists recruited: {} of {}", "Touristen gewonnen: {} von {}", "Turisti reclutati: {} su {}", "Turistas reclutados: {} de {}")
                    .avec(&[&run.recruited, &level.tourists]),
                24.0,
            );
        }
        spawn_gap(&mut commands, panel, 6.0);
        if outcome.won && i + 1 < LEVELS.len() {
            button(&mut commands, panel, tx!("Niveau suivant", "Next level", "Nächstes Level", "Livello successivo", "Siguiente nivel").t(), UiAction::Next);
        }
        button(&mut commands, panel, tx!("Rejouer", "Play again", "Nochmal", "Rigioca", "Otra vez").t(), UiAction::Replay);
        button(&mut commands, panel, tx!("Voyage", "Journey", "Reise", "Viaggio", "Viaje").t(), UiAction::Voyage);
        return;
    }
    spawn_text(&mut commands, panel, tx!("Perdu !", "Game over!", "Verloren!", "Hai perso!", "¡Has perdido!").t(), 56.0);
    spawn_text(
        &mut commands,
        panel,
        tx!("Score : {}    Record : {}", "Score: {}    Best: {}", "Punkte: {}    Rekord: {}", "Punti: {}    Record: {}", "Puntos: {}    Récord: {}")
            .avec(&[&score.current, &score.best]),
        24.0,
    );
    for item in &progress.fresh {
        spawn_text(&mut commands, panel, tx!("Débloqué : {}", "Unlocked: {}", "Freigeschaltet: {}", "Sbloccato: {}", "Desbloqueado: {}").avec(&[&label(*item)]), 24.0);
    }
    if let Some(t) = progress.next_text() {
        spawn_text(&mut commands, panel, t, 18.0);
    }
    spawn_gap(&mut commands, panel, 6.0);
    button(&mut commands, panel, tx!("Rejouer", "Play again", "Nochmal", "Rigioca", "Otra vez").t(), UiAction::Play);
    button(&mut commands, panel, tx!("Menu", "Menu", "Menü", "Menu", "Menú").t(), UiAction::Menu);
}

#[derive(Component)]
struct BannerText;

fn setup_hud(mut commands: Commands) {
    commands.spawn((
        Text::new(""),
        TextFont::from_font_size(30.0),
        TextColor(game_common::palette::TEXT_DARK),
        TextLayout::justify(Justify::Center),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(84.0),
            align_self: AlignSelf::Center,
            left: Val::Percent(50.0),
            padding: UiRect::axes(Val::Px(18.0), Val::Px(6.0)),
            border_radius: BorderRadius::all(Val::Px(18.0)),
            ..default()
        },
        BackgroundColor(Color::NONE),
        game_common::ui::ThemedText,
        BannerText,
    ));
    let (hud, text) = spawn_hud(&mut commands);
    commands.entity(hud).insert((Hud, Visibility::Hidden));
    commands.entity(text).insert(HudText);
}

fn update_banner(demo: Res<crate::game::Demo>, banner: Res<Banner>, paused: Res<Paused>, state: Res<State<GameState>>, mut q: Query<(&mut Text, &mut BackgroundColor), With<BannerText>>) {
    let playing = *state.get() == GameState::Playing;
    let t = if !playing || demo.press {
        String::new()
    } else if paused.0 {
        tx!("PAUSE", "PAUSED", "PAUSE", "PAUSA", "PAUSA").t().to_owned()
    } else if banner.left > 0.0 {
        banner.text.clone()
    } else {
        String::new()
    };
    for (mut text, mut bg) in &mut q {
        if text.0 != t {
            bg.0 = if t.is_empty() { Color::NONE } else { Color::srgba(0.98, 0.96, 0.9, 0.92) };
            text.0 = t.clone();
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn update_hud(
    score: Res<Score>,
    langue: Res<Langue>,
    state: Res<State<GameState>>,
    demo: Res<crate::game::Demo>,
    mode: Res<Mode>,
    run: Res<Run>,
    mut hud: Query<&mut Visibility, With<Hud>>,
    mut text: Query<&mut Text, With<HudText>>,
) {
    let playing = *state.get() == GameState::Playing && !demo.press;
    for mut v in &mut hud {
        *v = if playing { Visibility::Inherited } else { Visibility::Hidden };
    }
    if !playing {
        return;
    }
    let s = if let (Mode::Scenario { scenario, step }, Some(level)) = (*mode, mode.level()) {
        let secs = run.elapsed as u32;
        let total = crate::story::SCENARIOS.get(scenario).map_or(0, |s| s.steps.len());
        format!("Étape {}/{} · Touristes {}/{} · {}:{:02} · P : pause", step + 1, total, run.recruited.min(level.tourists), level.tourists, secs / 60, secs % 60)
    } else if let (Mode::Voyage(i), Some(level)) = (*mode, mode.level()) {
        let secs = run.elapsed as u32;
        let clock = format!("{}:{:02}", secs / 60, secs % 60);
        tx!(
            "Niveau {} · Touristes {}/{} · {} · P : pause",
            "Level {} · Tourists {}/{} · {} · P: pause",
            "Level {} · Touristen {}/{} · {} · P: Pause",
            "Livello {} · Turisti {}/{} · {} · P: pausa",
            "Nivel {} · Turistas {}/{} · {} · P: pausa"
        )
        .avec(&[&(i + 1), &run.recruited.min(level.tourists), &level.tourists, &clock])
    } else {
        tx!(
            "Score : {}    Record : {}    Échap : menu    P : pause",
            "Score: {}    Best: {}    Esc: menu    P: pause",
            "Punkte: {}    Rekord: {}    Esc: Menü    P: Pause",
            "Punti: {}    Record: {}    Esc: menu    P: pausa",
            "Puntos: {}    Récord: {}    Esc: menú    P: pausa"
        )
        .avec(&[&score.current, &score.best])
    };
    let _ = &langue;
    for mut t in &mut text {
        if t.0 != s {
            t.0 = s.clone();
        }
    }
}

fn refresh_option_texts(settings: Res<Settings>, progress: Res<Progress>, mut q: Query<(&mut Text, &OptionText)>) {
    if settings.is_changed() {
        for (mut text, opt) in &mut q {
            text.0 = opt.0.text(&settings, &progress);
        }
    }
}

fn keyboard_shortcuts(
    keys: Res<ButtonInput<KeyCode>>,
    state: Res<State<GameState>>,
    mut next: ResMut<NextState<GameState>>,
    mode: Res<Mode>,
) {
    let confirm = keys.just_pressed(KeyCode::Enter) || keys.just_pressed(KeyCode::Space);
    let back = keys.just_pressed(KeyCode::Escape);
    match state.get() {
        GameState::Menu if confirm => next.set(GameState::Playing),
        GameState::Options if back => next.set(GameState::Menu),
        GameState::Voyage if back => next.set(GameState::Menu),
        GameState::Playing if back => next.set(match *mode {
            Mode::Voyage(_) => GameState::Voyage,
            Mode::Scenario { .. } if crate::levels::preview_active() => GameState::Menu,
            Mode::Scenario { .. } => GameState::Scenarios,
            Mode::Infinite => GameState::Menu,
        }),
        GameState::GameOver if confirm => next.set(GameState::Playing),
        GameState::GameOver if back => next.set(match *mode {
            Mode::Voyage(_) => GameState::Voyage,
            Mode::Scenario { .. } if crate::levels::preview_active() => GameState::Menu,
            Mode::Scenario { .. } => GameState::Scenarios,
            Mode::Infinite => GameState::Menu,
        }),
        _ => {}
    }
}

fn on_click(
    click: On<PointerClick>,
    actions: Query<&UiAction>,
    mut settings: ResMut<Settings>,
    progress: Res<Progress>,
    mut mode: ResMut<Mode>,
    mut tab: ResMut<VoyageTab>,
    mut next: ResMut<NextState<GameState>>,
    mut sfx: MessageWriter<Sfx>,
) {
    let Ok(action) = actions.get(click.entity) else {
        return;
    };
    sfx.write(Sfx::Click);
    match *action {
        UiAction::Play => {
            *mode = Mode::Infinite;
            next.set(GameState::Playing);
        }
        UiAction::Replay => next.set(GameState::Playing),
        UiAction::Voyage => next.set(GameState::Voyage),
        UiAction::Scenarios => next.set(GameState::Scenarios),
        UiAction::Level(i) => {
            *mode = Mode::Voyage(i);
            next.set(GameState::Playing);
        }
        UiAction::Next => {
            if let Mode::Voyage(i) = *mode {
                *mode = Mode::Voyage(i + 1);
            }
            next.set(GameState::Playing);
        }
        UiAction::Tab(place) => tab.selected = place,
        UiAction::Locked => {}
        UiAction::Settings => next.set(GameState::Options),
        UiAction::Back | UiAction::Menu => next.set(GameState::Menu),
        UiAction::Cycle(kind) => kind.cycle(&mut settings, &progress),
    }
}
