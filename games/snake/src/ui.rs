//! Écrans : accueil, réglages, fin de partie, et affichage du score.
use bevy::prelude::*;
use game_common::langue::Langue;
use game_common::ui::{GameHelp, pied_de_menu, spawn_button, spawn_gap, spawn_hud, spawn_panel, spawn_text};
use game_common::{Score, tx};

use crate::GameState;
use crate::audio::Sfx;
use crate::game::{Banner, Paused};
use crate::progress::{Progress, label};
use crate::settings::{OptionKind, Settings};

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
            .add_systems(OnEnter(GameState::GameOver), spawn_game_over)
            .add_systems(Update, (update_hud, update_banner, refresh_option_texts, keyboard_shortcuts))
            .add_observer(on_click);
    }
}

/// Ce que fait un bouton quand on clique dessus.
#[derive(Component, Clone, Copy)]
enum UiAction {
    Play,
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
    button(&mut commands, panel, tx!("Jouer", "Play", "Spielen", "Gioca", "Jugar").t(), UiAction::Play);
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

fn spawn_game_over(mut commands: Commands, score: Res<Score>, progress: Res<Progress>) {
    let panel = spawn_panel(&mut commands, DespawnOnExit(GameState::GameOver));
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

fn update_banner(banner: Res<Banner>, paused: Res<Paused>, state: Res<State<GameState>>, mut q: Query<(&mut Text, &mut BackgroundColor), With<BannerText>>) {
    let playing = *state.get() == GameState::Playing;
    let t = if !playing {
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

fn update_hud(
    score: Res<Score>,
    langue: Res<Langue>,
    state: Res<State<GameState>>,
    mut hud: Query<&mut Visibility, With<Hud>>,
    mut text: Query<&mut Text, With<HudText>>,
) {
    let playing = *state.get() == GameState::Playing;
    for mut v in &mut hud {
        *v = if playing { Visibility::Inherited } else { Visibility::Hidden };
    }
    if score.is_changed() || langue.is_changed() {
        let hud = tx!(
            "Score : {}    Record : {}    Échap : menu    P : pause",
            "Score: {}    Best: {}    Esc: menu    M: sound",
            "Punkte: {}    Rekord: {}    Esc: Menü    M: Ton",
            "Punti: {}    Record: {}    Esc: menu    M: audio",
            "Puntos: {}    Récord: {}    Esc: menú    M: sonido"
        );
        for mut t in &mut text {
            t.0 = hud.avec(&[&score.current, &score.best]);
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
) {
    let confirm = keys.just_pressed(KeyCode::Enter) || keys.just_pressed(KeyCode::Space);
    let back = keys.just_pressed(KeyCode::Escape);
    match state.get() {
        GameState::Menu if confirm => next.set(GameState::Playing),
        GameState::Options if back => next.set(GameState::Menu),
        GameState::Playing if back => next.set(GameState::Menu),
        GameState::GameOver if confirm => next.set(GameState::Playing),
        GameState::GameOver if back => next.set(GameState::Menu),
        _ => {}
    }
}

fn on_click(
    click: On<PointerClick>,
    actions: Query<&UiAction>,
    mut settings: ResMut<Settings>,
    progress: Res<Progress>,
    mut next: ResMut<NextState<GameState>>,
    mut sfx: MessageWriter<Sfx>,
) {
    let Ok(action) = actions.get(click.entity) else {
        return;
    };
    sfx.write(Sfx::Click);
    match *action {
        UiAction::Play => next.set(GameState::Playing),
        UiAction::Settings => next.set(GameState::Options),
        UiAction::Back | UiAction::Menu => next.set(GameState::Menu),
        UiAction::Cycle(kind) => kind.cycle(&mut settings, &progress),
    }
}
