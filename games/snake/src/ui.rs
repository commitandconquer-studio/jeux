//! Écrans : accueil, réglages, fin de partie, et affichage du score.
use bevy::prelude::*;
use game_common::langue::Langue;
use game_common::ui::{GameHelp, pied_de_menu, spawn_button, spawn_gap, spawn_hud, spawn_panel, spawn_text};
use game_common::{Score, tx};

use crate::GameState;
use crate::audio::Sfx;
use crate::game::{Banner, Paused};
use crate::progress::{Progress, label};
use crate::levels::LEVELS;
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
            .add_systems(OnEnter(GameState::Voyage), spawn_voyage)
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
    Level(usize),
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

fn spawn_menu(mut commands: Commands, settings: Res<Settings>, progress: Res<Progress>) {
    let panel = spawn_panel(&mut commands, DespawnOnExit(GameState::Menu));
    spawn_text(&mut commands, panel, tx!("À LA QUEUE LEU LEU", "CONGA LINE", "POLONÄSE", "TRENINO", "TRENECITO").t(), 46.0);
    spawn_gap(&mut commands, panel, 10.0);
    button(&mut commands, panel, tx!("Voyage", "Journey", "Reise", "Viaggio", "Viaje").t(), UiAction::Voyage);
    button(&mut commands, panel, tx!("Mode infini", "Endless mode", "Endlos-Modus", "Modalità infinita", "Modo infinito").t(), UiAction::Play);
    let t = button(
        &mut commands,
        panel,
        OptionKind::Universe.text(&settings, &progress),
        UiAction::Cycle(OptionKind::Universe),
    );
    commands.entity(t).insert(OptionText(OptionKind::Universe));
    button(
        &mut commands,
        panel,
        tx!("Réglages", "Settings", "Einstellungen", "Impostazioni", "Ajustes").t(),
        UiAction::Settings,
    );
    spawn_gap(&mut commands, panel, 4.0);
    spawn_text(
        &mut commands,
        panel,
        tx!("Invités attrapés au total : {}", "Guests caught in total: {}", "Gefangene Gäste insgesamt: {}", "Invitati presi in totale: {}", "Invitados atrapados en total: {}").avec(&[&progress.total]),
        18.0,
    );
    if let Some(t) = progress.next_text() {
        spawn_text(&mut commands, panel, t, 18.0);
    }
    pied_de_menu(&mut commands, panel);
}

fn stars_text(n: u8) -> String {
    (0..3).map(|i| if i < n { '*' } else { '-' }).collect()
}

/// La carte du Voyage : les niveaux, leurs étoiles, ceux qui sont encore fermés.
fn spawn_voyage(mut commands: Commands, voyage: Res<VoyageProgress>) {
    let panel = spawn_panel(&mut commands, DespawnOnExit(GameState::Voyage));
    spawn_text(&mut commands, panel, tx!("Voyage", "Journey", "Reise", "Viaggio", "Viaje").t(), 44.0);
    spawn_text(
        &mut commands,
        panel,
        tx!("Étoiles : {}", "Stars: {}", "Sterne: {}", "Stelle: {}", "Estrellas: {}").avec(&[&voyage.total_stars()]),
        20.0,
    );
    let mut place = None;
    let mut grid = None;
    for (i, level) in LEVELS.iter().enumerate() {
        if place != Some(level.place) {
            place = Some(level.place);
            spawn_text(&mut commands, panel, level.place.label(), 26.0);
            grid = Some(
                commands
                    .spawn((
                        Node {
                            flex_wrap: FlexWrap::Wrap,
                            justify_content: JustifyContent::Center,
                            column_gap: Val::Px(8.0),
                            row_gap: Val::Px(2.0),
                            max_width: Val::Px(900.0),
                            ..default()
                        },
                        ChildOf(panel),
                    ))
                    .id(),
            );
        }
        let open = voyage.unlocked(i);
        let label = if open {
            format!("{} · {}  {}", i + 1, level.name, stars_text(voyage.stars(i)))
        } else {
            format!("{} · ?", i + 1)
        };
        button(&mut commands, grid.unwrap_or(panel), label, if open { UiAction::Level(i) } else { UiAction::Locked });
    }
    spawn_gap(&mut commands, panel, 6.0);
    button(&mut commands, panel, tx!("Retour", "Back", "Zurück", "Indietro", "Volver").t(), UiAction::Menu);
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
    let s = if let (Mode::Voyage(i), Some(level)) = (*mode, mode.level()) {
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
        GameState::Playing if back => next.set(if matches!(*mode, Mode::Voyage(_)) { GameState::Voyage } else { GameState::Menu }),
        GameState::GameOver if confirm => next.set(GameState::Playing),
        GameState::GameOver if back => next.set(if matches!(*mode, Mode::Voyage(_)) { GameState::Voyage } else { GameState::Menu }),
        _ => {}
    }
}

fn on_click(
    click: On<PointerClick>,
    actions: Query<&UiAction>,
    mut settings: ResMut<Settings>,
    progress: Res<Progress>,
    mut mode: ResMut<Mode>,
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
        UiAction::Locked => {}
        UiAction::Settings => next.set(GameState::Options),
        UiAction::Back | UiAction::Menu => next.set(GameState::Menu),
        UiAction::Cycle(kind) => kind.cycle(&mut settings, &progress),
    }
}
