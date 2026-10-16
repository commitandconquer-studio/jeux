//! Écrans : accueil, chapitres, dialogues, barre de jeu, victoire, défaite.
use bevy::prelude::*;
use game_common::langue::Texte;
use game_common::tx;
use game_common::ui::{GameHelp, pied_de_menu};
use game_common::ui::{
    PANEL, ThemedPanel, UiButton, spawn_button, spawn_button_sized, spawn_gap, spawn_panel, spawn_panel_bottom,
    spawn_panel_left, spawn_text,
};

use crate::GameState;
use crate::audio::UiClick;
use crate::game::{Game, Mode, NextMode, NovaEvent};
use crate::scene::{HOLD_POS, NEXT_POS};
use crate::story::{BOULON_FOUR, BOULON_HALF, BOULON_NOM, BOULON_WARNING, CHAPTERS, Line};

const SAVE: &str = "nova";

/// Aide du jeu (bouton « Aide » du menu).
const HELP: Texte = tx!(
    "Les blocs d'énergie tombent dans le bouclier.\n\
Chaque rangée complète tire le canon à ions sur le croiseur :\n\
plusieurs rangées d'un coup, tir plus puissant !\n\
Le croiseur riposte avec des rangées endommagées.\n\
\n\
Gauche/droite : déplacer · Haut : tourner\n\
Bas : accélérer · Espace : poser · C : réserve\n\
M : son · Échap : menu\n\
\n\
Tactile : glisser pour déplacer, toucher pour tourner,\n\
vers le bas pour poser, vers le haut pour la réserve.",
    "Energy blocks fall into the shield.\n\
Every full row fires the ion cannon at the cruiser:\n\
several rows at once make a stronger shot!\n\
The cruiser strikes back with damaged rows.\n\
\n\
Left/right: move · Up: rotate\n\
Down: speed up · Space: drop · C: hold\n\
M: sound · Esc: menu\n\
\n\
Touch: slide to move, tap to rotate,\n\
swipe down to drop, swipe up to hold.",
    "Energieblöcke fallen in den Schild.\n\
Jede volle Reihe feuert die Ionenkanone auf den Kreuzer:\n\
mehrere Reihen auf einmal, stärkerer Schuss!\n\
Der Kreuzer schlägt mit beschädigten Reihen zurück.\n\
\n\
Links/rechts: bewegen · Oben: drehen\n\
Unten: beschleunigen · Leertaste: absetzen · C: Reserve\n\
M: Ton · Esc: Menü\n\
\n\
Touch: wischen zum Bewegen, tippen zum Drehen,\n\
nach unten zum Absetzen, nach oben für die Reserve.",
    "I blocchi di energia cadono nello scudo.\n\
Ogni fila completa spara il cannone a ioni sull'incrociatore:\n\
più file in un colpo, tiro più potente!\n\
L'incrociatore risponde con file danneggiate.\n\
\n\
Sinistra/destra: muovere · Su: ruotare\n\
Giù: accelerare · Spazio: posare · C: riserva\n\
M: audio · Esc: menu\n\
\n\
Touch: trascina per muovere, tocca per ruotare,\n\
verso il basso per posare, verso l'alto per la riserva.",
    "Los bloques de energía caen en el escudo.\n\
Cada fila completa dispara el cañón de iones contra el crucero:\n\
¡varias filas de golpe, disparo más potente!\n\
El crucero responde con filas dañadas.\n\
\n\
Izquierda/derecha: mover · Arriba: girar\n\
Abajo: acelerar · Espacio: soltar · C: reserva\n\
M: sonido · Esc: menú\n\
\n\
Táctil: desliza para mover, toca para girar,\n\
hacia abajo para soltar, hacia arriba para la reserva.",
);

const MENU: Texte = tx!("Menu", "Menu", "Menü", "Menu", "Menú");
const BOUCLIER_CEDE: Texte = tx!(
    "Le bouclier a cédé !",
    "The shield gave way!",
    "Der Schild ist gebrochen!",
    "Lo scudo ha ceduto!",
    "¡El escudo ha cedido!",
);
const MODE_INFINI: Texte = tx!("Mode infini", "Endless mode", "Endlosmodus", "Modalità infinita", "Modo infinito");

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        let unlocked = game_common::save::load(SAVE, "chapitres").and_then(|s| s.parse().ok()).unwrap_or(1usize).clamp(1, CHAPTERS.len());
        let record = game_common::save::load(SAVE, "record").and_then(|s| s.parse().ok()).unwrap_or(0u32);
        app.insert_resource(Progress { unlocked, record })
            .insert_resource(Dialog::default())
            .insert_resource(GameHelp(HELP))
            .add_systems(OnEnter(GameState::Menu), spawn_menu)
            .add_systems(OnEnter(GameState::Chapters), spawn_chapters)
            .add_systems(OnEnter(GameState::Story), spawn_dialog)
            .add_systems(OnEnter(GameState::Playing), spawn_hud)
            .add_systems(OnEnter(GameState::Victory), start_outro)
            .add_systems(OnEnter(GameState::Defeat), spawn_defeat)
            .add_systems(
                Update,
                (update_hud, place_preview_labels, boulon_remarks, fade_toasts, update_dialog, keyboard),
            )
            .add_observer(on_click);
    }
}

/// Chapitres débloqués et record du mode infini (sauvegardés).
#[derive(Resource)]
pub struct Progress {
    pub unlocked: usize,
    pub record: u32,
}

/// Dialogue en cours.
#[derive(Resource, Default)]
pub struct Dialog {
    lines: &'static [Line],
    index: usize,
}

#[derive(Component, Clone, Copy)]
enum UiAction {
    Campaign,
    Chapters,
    Chapter(usize),
    Endless,
    Back,
    Menu,
    NextLine,
    NextChapter,
    Retry,
}

#[derive(Component)]
struct InfoText;
#[derive(Component)]
struct HullBar;
#[derive(Component)]
struct HullFill;
#[derive(Component)]
struct HullText;
#[derive(Component)]
struct WarningText;
#[derive(Component)]
struct PreviewLabel(bool);
#[derive(Component)]
struct DialogSpeaker;
#[derive(Component)]
struct DialogText;
#[derive(Component)]
struct DialogBox;
#[derive(Component)]
struct Toast(f32);

fn button(c: &mut Commands, parent: Entity, label: impl Into<String>, action: UiAction) -> Entity {
    let (b, text) = spawn_button(c, parent, label);
    c.entity(b).insert(action);
    text
}

fn spawn_menu(mut c: Commands, progress: Res<Progress>) {
    let panel = spawn_panel_left(&mut c, DespawnOnExit(GameState::Menu));
    spawn_text(&mut c, panel, "NOVA", 64.0);
    spawn_text(&mut c, panel, tx!("Le Dernier Rempart", "The Last Bastion", "Das letzte Bollwerk", "L'ultimo baluardo", "El último bastión").t(), 30.0);
    spawn_gap(&mut c, panel, 0.0);
    let campagne = tx!("Campagne : chapitre {}", "Campaign: chapter {}", "Kampagne: Kapitel {}", "Campagna: capitolo {}", "Campaña: capítulo {}");
    button(&mut c, panel, campagne.avec(&[&progress.unlocked]), UiAction::Campaign);
    button(&mut c, panel, tx!("Chapitres", "Chapters", "Kapitel", "Capitoli", "Capítulos").t(), UiAction::Chapters);
    let record = if progress.record > 0 {
        tx!("{} (record {})", "{} (best {})", "{} (Rekord {})", "{} (record {})", "{} (récord {})").avec(&[&MODE_INFINI.t(), &progress.record])
    } else {
        MODE_INFINI.t().into()
    };
    button(&mut c, panel, record, UiAction::Endless);
    pied_de_menu(&mut c, panel);
}

fn spawn_chapters(mut c: Commands, progress: Res<Progress>) {
    let panel = spawn_panel(&mut c, DespawnOnExit(GameState::Chapters));
    spawn_text(&mut c, panel, tx!("Chapitres", "Chapters", "Kapitel", "Capitoli", "Capítulos").t(), 40.0);
    for (i, ch) in CHAPTERS.iter().enumerate() {
        let open = i < progress.unlocked;
        let label = if open { format!("{} · {}", i + 1, ch.name.t()) } else { format!("{} · ???", i + 1) };
        let (b, _) = spawn_button_sized(&mut c, panel, label, 320.0, 50.0, 22.0);
        if open {
            c.entity(b).insert(UiAction::Chapter(i));
        } else {
            c.entity(b).remove::<UiButton>().insert(BackgroundColor(Color::srgb(0.82, 0.82, 0.80)));
        }
    }
    spawn_gap(&mut c, panel, 4.0);
    button(&mut c, panel, tx!("Retour", "Back", "Zurück", "Indietro", "Volver").t(), UiAction::Back);
}

/// Lance un chapitre : son introduction, puis la partie.
fn start_chapter(c: &mut Commands, dialog: &mut Dialog, next: &mut NextState<GameState>, chapter: usize) {
    c.insert_resource(NextMode(Mode::Campaign(chapter)));
    *dialog = Dialog { lines: CHAPTERS[chapter].intro, index: 0 };
    next.set(GameState::Story);
}

/// Boîte de dialogue en bas de l'écran : qui parle, ce qu'il dit, « Suite ».
fn spawn_dialog_box(c: &mut Commands, scope: impl Bundle) {
    let panel = spawn_panel_bottom(c, scope);
    c.entity(panel).insert((DialogBox, Node {
        flex_direction: FlexDirection::Column,
        align_items: AlignItems::FlexStart,
        row_gap: Val::Px(10.0),
        padding: UiRect::axes(Val::Px(32.0), Val::Px(22.0)),
        width: Val::Px(760.0),
        border: UiRect::all(Val::Px(2.0)),
        max_width: Val::Percent(95.0),
        border_radius: BorderRadius::all(Val::Px(20.0)),
        ..default()
    }));
    let speaker = spawn_text(c, panel, "", 24.0);
    c.entity(speaker).insert(DialogSpeaker);
    let text = spawn_text(c, panel, "", 22.0);
    c.entity(text).insert((DialogText, TextLayout::justify(Justify::Left)));
    let row = c.spawn((Node { width: Val::Percent(100.0), justify_content: JustifyContent::FlexEnd, ..default() }, ChildOf(panel))).id();
    let (b, _) = spawn_button_sized(c, row, tx!("Suite ›", "Next ›", "Weiter ›", "Avanti ›", "Seguir ›").t(), 160.0, 44.0, 20.0);
    c.entity(b).insert(UiAction::NextLine);
}

fn spawn_dialog(mut c: Commands) {
    spawn_dialog_box(&mut c, DespawnOnExit(GameState::Story));
}

/// Victoire : la conclusion du chapitre, puis l'écran de fin de chapitre.
fn start_outro(mut c: Commands, game: Res<Game>, mut dialog: ResMut<Dialog>, mut progress: ResMut<Progress>) {
    let Some(chapter) = game.chapter() else { return };
    progress.unlocked = progress.unlocked.max(chapter + 2).min(CHAPTERS.len());
    game_common::save::store(SAVE, "chapitres", &progress.unlocked.to_string());
    *dialog = Dialog { lines: CHAPTERS[chapter].outro, index: 0 };
    spawn_dialog_box(&mut c, DespawnOnExit(GameState::Victory));
}

fn spawn_victory_panel(c: &mut Commands, game: &Game) {
    let panel = spawn_panel(c, DespawnOnExit(GameState::Victory));
    let last = game.chapter() == Some(CHAPTERS.len() - 1);
    if last {
        spawn_text(c, panel, tx!("Nova est libre !", "Nova is free!", "Nova ist frei!", "Nova è libera!", "¡Nova es libre!").t(), 48.0);
        let merci = tx!(
            "Merci d'avoir défendu la colonie.\nLe mode infini vous attend pour battre des records.",
            "Thank you for defending the colony.\nEndless mode awaits you for record runs.",
            "Danke, dass du die Kolonie verteidigt hast.\nIm Endlosmodus kannst du jetzt Rekorde jagen.",
            "Grazie per aver difeso la colonia.\nLa modalità infinita ti aspetta per battere i record.",
            "Gracias por defender la colonia.\nEl modo infinito te espera para batir récords.",
        );
        spawn_text(c, panel, merci.t(), 20.0);
    } else {
        spawn_text(c, panel, tx!("Chapitre réussi !", "Chapter complete!", "Kapitel geschafft!", "Capitolo superato!", "¡Capítulo superado!").t(), 44.0);
    }
    let bilan = tx!("Score : {}    Lignes : {}", "Score: {}    Lines: {}", "Punkte: {}    Reihen: {}", "Punti: {}    Righe: {}", "Puntos: {}    Líneas: {}");
    spawn_text(c, panel, bilan.avec(&[&game.score, &game.lines]), 22.0);
    spawn_gap(c, panel, 4.0);
    if last {
        button(c, panel, MODE_INFINI.t(), UiAction::Endless);
    } else {
        button(c, panel, tx!("Chapitre suivant", "Next chapter", "Nächstes Kapitel", "Capitolo successivo", "Capítulo siguiente").t(), UiAction::NextChapter);
    }
    button(c, panel, MENU.t(), UiAction::Menu);
}

fn spawn_defeat(mut c: Commands, game: Res<Game>, mut progress: ResMut<Progress>) {
    let panel = spawn_panel_bottom(&mut c, DespawnOnExit(GameState::Defeat));
    if game.mode == Mode::Endless {
        let new = game.score > progress.record;
        if new {
            progress.record = game.score;
            game_common::save::store(SAVE, "record", &game.score.to_string());
        }
        let titre = if new { tx!("Nouveau record !", "New record!", "Neuer Rekord!", "Nuovo record!", "¡Nuevo récord!").t() } else { BOUCLIER_CEDE.t() };
        spawn_text(&mut c, panel, titre, 40.0);
        let bilan = tx!("Score : {}    Record : {}", "Score: {}    Best: {}", "Punkte: {}    Rekord: {}", "Punti: {}    Record: {}", "Puntos: {}    Récord: {}");
        spawn_text(&mut c, panel, bilan.avec(&[&game.score, &progress.record]), 22.0);
    } else {
        spawn_text(&mut c, panel, BOUCLIER_CEDE.t(), 40.0);
        let replique = tx!(
            "{} : « On recommence ? Cette fois, je ne tremble pas. »",
            "{}: \"Again? This time I won't shake.\"",
            "{}: „Noch mal? Diesmal zittere ich nicht.“",
            "{}: «Riproviamo? Stavolta non tremo.»",
            "{}: «¿Otra vez? Esta vez no tiemblo.»",
        );
        spawn_text(&mut c, panel, replique.avec(&[&BOULON_NOM.t()]), 20.0);
    }
    button(&mut c, panel, tx!("Réessayer", "Retry", "Nochmal", "Riprova", "Reintentar").t(), UiAction::Retry);
    button(&mut c, panel, MENU.t(), UiAction::Menu);
}

/// Barre de jeu : infos à gauche, coque du croiseur en haut, alarme, étiquettes.
fn spawn_hud(mut c: Commands, mode: Option<Res<NextMode>>) {
    let scope = || DespawnOnExit(GameState::Playing);
    let info = c
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(14.0),
                left: Val::Px(14.0),
                padding: UiRect::axes(Val::Px(18.0), Val::Px(10.0)),
                border_radius: BorderRadius::all(Val::Px(16.0)),
                ..default()
            },
            BackgroundColor(PANEL),
            ThemedPanel,
            scope(),
        ))
        .id();
    let t = spawn_text(&mut c, info, "", 20.0);
    c.entity(t).insert((InfoText, TextLayout::justify(Justify::Left)));

    if matches!(mode.map(|m| m.0), Some(Mode::Campaign(_))) {
        let bar = c
            .spawn((
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(14.0),
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    row_gap: Val::Px(4.0),
                    ..default()
                },
                Pickable::IGNORE,
                HullBar,
                scope(),
            ))
            .id();
        let label = spawn_text(&mut c, bar, "", 18.0);
        c.entity(label).insert((HullText, TextColor(Color::srgb(0.95, 0.95, 1.0))));
        let back = c
            .spawn((
                Node { width: Val::Px(420.0), height: Val::Px(16.0), border_radius: BorderRadius::all(Val::Px(8.0)), ..default() },
                BackgroundColor(Color::srgba(1.0, 1.0, 1.0, 0.15)),
                ChildOf(bar),
            ))
            .id();
        c.spawn((
            Node { width: Val::Percent(100.0), height: Val::Percent(100.0), border_radius: BorderRadius::all(Val::Px(8.0)), ..default() },
            BackgroundColor(Color::srgb(1.0, 0.42, 0.25)),
            HullFill,
            ChildOf(back),
        ));
        let warn = spawn_text(&mut c, bar, "", 26.0);
        c.entity(warn).insert((WarningText, TextColor(Color::srgb(1.0, 0.35, 0.3))));
    }
    for hold in [false, true] {
        c.spawn((
            Text::new(if hold {
                tx!("Réserve", "Hold", "Reserve", "Riserva", "Reserva").t()
            } else {
                tx!("Suivant", "Next", "Nächster", "Prossimo", "Siguiente").t()
            }),
            TextFont::from_font_size(18.0),
            TextColor(Color::srgb(0.75, 0.9, 1.0)),
            Node { position_type: PositionType::Absolute, ..default() },
            Pickable::IGNORE,
            PreviewLabel(hold),
            scope(),
        ));
    }
}

fn update_hud(
    game: Res<Game>,
    time: Res<Time>,
    mut info: Query<&mut Text, (With<InfoText>, Without<HullText>, Without<WarningText>)>,
    mut hull_text: Query<&mut Text, (With<HullText>, Without<InfoText>, Without<WarningText>)>,
    mut warn: Query<&mut Text, (With<WarningText>, Without<InfoText>, Without<HullText>)>,
    mut fill: Query<&mut Node, With<HullFill>>,
) {
    let title = match game.mode {
        Mode::Campaign(i) => tx!("Chapitre {} · {}", "Chapter {} · {}", "Kapitel {} · {}", "Capitolo {} · {}", "Capítulo {} · {}")
            .avec(&[&(i + 1), &CHAPTERS[i].name.t()]),
        Mode::Endless => tx!("{} · vitesse {}", "{} · speed {}", "{} · Tempo {}", "{} · velocità {}", "{} · velocidad {}")
            .avec(&[&MODE_INFINI.t(), &(game.lines / 5 + 1)]),
    };
    let infos = tx!("{}\nScore : {}\nLignes : {}", "{}\nScore: {}\nLines: {}", "{}\nPunkte: {}\nReihen: {}", "{}\nPunti: {}\nRighe: {}", "{}\nPuntos: {}\nLíneas: {}");
    for mut t in &mut info {
        t.0 = infos.avec(&[&title, &game.score, &game.lines]);
    }
    let pct = if game.hull_max > 0.0 { game.hull / game.hull_max * 100.0 } else { 0.0 };
    for mut t in &mut hull_text {
        t.0 = tx!(
            "Croiseur de l'Hégémonie — coque {} %",
            "Hegemony cruiser — hull {}%",
            "Kreuzer der Hegemonie — Hülle {} %",
            "Incrociatore dell'Egemonia — scafo {}%",
            "Crucero de la Hegemonía — casco {} %",
        )
        .avec(&[&format!("{:.0}", pct.ceil())]);
    }
    for mut n in &mut fill {
        n.width = Val::Percent(pct);
    }
    let blink = (time.elapsed_secs() * 6.0).sin() > 0.0;
    for mut t in &mut warn {
        t.0 = if game.warning > 0.0 && blink { tx!("! Tir ennemi imminent !", "! Enemy fire incoming !", "! Feindlicher Schuss droht !", "! Fuoco nemico imminente !", "¡ Disparo enemigo inminente !").t().into() } else { String::new() };
    }
}

/// Place « Suivant » et « Réserve » au-dessus des aperçus 3D, quelle que
/// soit la taille de la fenêtre.
fn place_preview_labels(cam: Query<(&Camera, &GlobalTransform)>, mut labels: Query<(&PreviewLabel, &mut Node, &ComputedNode)>) {
    let Ok((camera, cam_tf)) = cam.single() else { return };
    for (label, mut node, computed) in &mut labels {
        let world = if label.0 { HOLD_POS } else { NEXT_POS[0] } + Vec3::new(0.0, 2.4, 0.0);
        if let Ok(p) = camera.world_to_viewport(cam_tf, world) {
            let w = computed.size().x * computed.inverse_scale_factor();
            node.left = Val::Px(p.x - w / 2.0);
            node.top = Val::Px(p.y - 12.0);
        }
    }
}

/// Boulon commente la partie dans une petite bulle.
fn boulon_remarks(mut c: Commands, mut events: MessageReader<NovaEvent>, mut game: ResMut<Game>, state: Res<State<GameState>>, old: Query<Entity, With<Toast>>) {
    if *state.get() != GameState::Playing {
        events.clear();
        return;
    }
    let mut say = None;
    for e in events.read() {
        match e {
            NovaEvent::Lines(rows) if rows.len() == 4 => say = Some(BOULON_FOUR),
            NovaEvent::Warning => say = Some(BOULON_WARNING),
            _ => {}
        }
    }
    if game.half_hull() {
        say = Some(BOULON_HALF);
    }
    let Some(text) = say else { return };
    for e in &old {
        c.entity(e).despawn();
    }
    let bubble = c
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px(20.0),
                left: Val::Px(20.0),
                max_width: Val::Px(420.0),
                padding: UiRect::axes(Val::Px(16.0), Val::Px(10.0)),
                border_radius: BorderRadius::all(Val::Px(16.0)),
                ..default()
            },
            BackgroundColor(PANEL),
            ThemedPanel,
            Pickable::IGNORE,
            Toast(3.5),
            DespawnOnExit(GameState::Playing),
        ))
        .id();
    let t = spawn_text(&mut c, bubble, tx!("{} : « {} »", "{}: \"{}\"", "{}: „{}“", "{}: «{}»", "{}: «{}»").avec(&[&BOULON_NOM.t(), &text.t()]), 18.0);
    c.entity(t).insert(TextLayout::justify(Justify::Left));
}

fn fade_toasts(mut c: Commands, time: Res<Time>, mut q: Query<(Entity, &mut Toast)>) {
    for (e, mut toast) in &mut q {
        toast.0 -= time.delta_secs();
        if toast.0 <= 0.0 {
            c.entity(e).despawn();
        }
    }
}

fn update_dialog(
    dialog: Res<Dialog>,
    mut speaker: Query<(&mut Text, &mut TextColor), (With<DialogSpeaker>, Without<DialogText>)>,
    mut text: Query<&mut Text, (With<DialogText>, Without<DialogSpeaker>)>,
) {
    let Some(&(who, line)) = dialog.lines.get(dialog.index) else { return };
    for (mut t, mut color) in &mut speaker {
        t.0 = who.name().into();
        color.0 = who.color();
    }
    for mut t in &mut text {
        t.0 = line.t().into();
    }
}

/// Passe à la réplique suivante ; à la fin, lance la partie ou affiche le bilan.
fn advance_dialog(
    c: &mut Commands,
    dialog: &mut Dialog,
    state: &GameState,
    next: &mut NextState<GameState>,
    game: &Game,
    boxes: &Query<Entity, With<DialogBox>>,
) {
    dialog.index += 1;
    if dialog.index < dialog.lines.len() {
        return;
    }
    match state {
        GameState::Story => next.set(GameState::Playing),
        GameState::Victory => {
            for e in boxes {
                c.entity(e).despawn();
            }
            spawn_victory_panel(c, game);
        }
        _ => {}
    }
}

#[allow(clippy::too_many_arguments)]
fn keyboard(
    mut c: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    state: Res<State<GameState>>,
    mut next: ResMut<NextState<GameState>>,
    mut dialog: ResMut<Dialog>,
    progress: Res<Progress>,
    game: Res<Game>,
    boxes: Query<Entity, With<DialogBox>>,
) {
    let confirm = keys.just_pressed(KeyCode::Enter) || keys.just_pressed(KeyCode::Space);
    let back = keys.just_pressed(KeyCode::Escape);
    match state.get() {
        GameState::Menu if keys.just_pressed(KeyCode::Enter) => start_chapter(&mut c, &mut dialog, &mut next, progress.unlocked - 1),
        GameState::Story | GameState::Victory if confirm && dialog.index < dialog.lines.len() => {
            advance_dialog(&mut c, &mut dialog, state.get(), &mut next, &game, &boxes)
        }
        GameState::Chapters | GameState::Playing | GameState::Defeat if back => next.set(GameState::Menu),
        GameState::Defeat if keys.just_pressed(KeyCode::Enter) => next.set(GameState::Playing),
        _ => {}
    }
}

#[allow(clippy::too_many_arguments)]
fn on_click(
    click: On<PointerClick>,
    actions: Query<&UiAction>,
    mut c: Commands,
    state: Res<State<GameState>>,
    mut next: ResMut<NextState<GameState>>,
    mut dialog: ResMut<Dialog>,
    progress: Res<Progress>,
    game: Res<Game>,
    boxes: Query<Entity, With<DialogBox>>,
    mut clicks: MessageWriter<UiClick>,
) {
    let Ok(action) = actions.get(click.entity) else { return };
    clicks.write(UiClick);
    match *action {
        UiAction::Campaign => start_chapter(&mut c, &mut dialog, &mut next, progress.unlocked - 1),
        UiAction::Chapters => next.set(GameState::Chapters),
        UiAction::Chapter(i) => start_chapter(&mut c, &mut dialog, &mut next, i),
        UiAction::Endless => {
            c.insert_resource(NextMode(Mode::Endless));
            next.set(GameState::Playing);
        }
        UiAction::Back | UiAction::Menu => next.set(GameState::Menu),
        UiAction::NextLine => advance_dialog(&mut c, &mut dialog, state.get(), &mut next, &game, &boxes),
        UiAction::NextChapter => {
            let i = game.chapter().map(|i| i + 1).unwrap_or(0).min(CHAPTERS.len() - 1);
            start_chapter(&mut c, &mut dialog, &mut next, i);
        }
        UiAction::Retry => next.set(GameState::Playing),
    }
}
