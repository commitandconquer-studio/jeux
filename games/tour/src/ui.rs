//! Écrans : accueil, fin de partie, score, et « Parfait ! ».
use bevy::prelude::*;
use game_common::tx;
use game_common::ui::{GameHelp, pied_de_menu, spawn_button, spawn_gap, spawn_hud, spawn_panel_bottom, spawn_panel_left, spawn_text};
use game_common::{Score, palette};

use crate::GameState;
use crate::audio::UiClick;
use crate::tower::TowerEvent;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_hud, setup_praise))
            .insert_resource(GameHelp(tx!(
                "Empilez les blocs le plus haut possible !\n\
                \n\
                Chaque bloc glisse d'avant en arrière :\n\
                posez-le au bon moment. Ce qui dépasse est coupé.\n\
                « Parfait ! » quand il tombe pile :\n\
                trois d'affilée et le bloc regrandit.\n\
                \n\
                Espace, clic ou toucher l'écran : poser\n\
                M : son · Échap : menu",
                "Stack the blocks as high as you can!\n\
                \n\
                Each block slides back and forth:\n\
                drop it at the right moment. Whatever overhangs is cut off.\n\
                \"Perfect!\" when it lands exactly on top:\n\
                three in a row and the block grows back.\n\
                \n\
                Space, click or tap the screen: drop\n\
                M: sound · Esc: menu",
                "Staple die Blöcke so hoch wie möglich!\n\
                \n\
                Jeder Block gleitet hin und her:\n\
                Setze ihn im richtigen Moment ab. Was übersteht, wird abgeschnitten.\n\
                „Perfekt!“, wenn er genau passt:\n\
                dreimal hintereinander und der Block wächst wieder.\n\
                \n\
                Leertaste, Klick oder Tippen: absetzen\n\
                M: Ton · Esc: Menü",
                "Impila i blocchi il più in alto possibile!\n\
                \n\
                Ogni blocco scivola avanti e indietro:\n\
                posalo al momento giusto. Ciò che sporge viene tagliato.\n\
                «Perfetto!» quando cade preciso:\n\
                tre di fila e il blocco torna a crescere.\n\
                \n\
                Spazio, clic o tocco sullo schermo: posa\n\
                M: audio · Esc: menu",
                "¡Apila los bloques lo más alto posible!\n\
                \n\
                Cada bloque se desliza de un lado a otro:\n\
                suéltalo en el momento justo. Lo que sobresale se corta.\n\
                «¡Perfecto!» cuando cae exacto:\n\
                tres seguidos y el bloque vuelve a crecer.\n\
                \n\
                Espacio, clic o tocar la pantalla: soltar\n\
                M: sonido · Esc: menú",
            )))
            .add_systems(OnEnter(GameState::Menu), spawn_menu)
            .add_systems(OnEnter(GameState::GameOver), spawn_game_over)
            .add_systems(Update, (update_hud, praise, keyboard_shortcuts))
            .add_observer(on_click);
    }
}

#[derive(Component, Clone, Copy)]
enum UiAction {
    Play,
    Menu,
}

#[derive(Component)]
struct Hud;

#[derive(Component)]
struct HudText;

/// Texte « Parfait ! » qui s'efface tout seul.
#[derive(Component)]
struct Praise {
    timer: f32,
}

fn button(commands: &mut Commands, parent: Entity, label: &str, action: UiAction) -> Entity {
    let (b, text) = spawn_button(commands, parent, label);
    commands.entity(b).insert(action);
    text
}

fn spawn_menu(mut commands: Commands) {
    let panel = spawn_panel_left(&mut commands, DespawnOnExit(GameState::Menu));
    spawn_text(&mut commands, panel, tx!("TOUR DE BLOCS", "BLOCK TOWER", "BLOCKTURM", "TORRE DI BLOCCHI", "TORRE DE BLOQUES").t(), 60.0);
    spawn_gap(&mut commands, panel, 8.0);
    button(&mut commands, panel, tx!("Jouer", "Play", "Spielen", "Gioca", "Jugar").t(), UiAction::Play);
    pied_de_menu(&mut commands, panel);
}

fn spawn_game_over(mut commands: Commands, score: Res<Score>) {
    let panel = spawn_panel_bottom(&mut commands, DespawnOnExit(GameState::GameOver));
    let title = if score.current > 0 && score.current == score.best {
        tx!("Nouveau record !", "New record!", "Neuer Rekord!", "Nuovo record!", "¡Nuevo récord!")
    } else {
        tx!("La tour s'arrête là !", "The tower ends here!", "Hier endet der Turm!", "La torre finisce qui!", "¡La torre termina aquí!")
    };
    spawn_text(&mut commands, panel, title.t(), 40.0);
    let line = tx!("Hauteur : {}    Record : {}", "Height: {}    Best: {}", "Höhe: {}    Rekord: {}", "Altezza: {}    Record: {}", "Altura: {}    Récord: {}");
    spawn_text(&mut commands, panel, line.avec(&[&score.current, &score.best]), 24.0);
    button(&mut commands, panel, tx!("Rejouer", "Play again", "Nochmal", "Rigioca", "Jugar otra vez").t(), UiAction::Play);
    button(&mut commands, panel, tx!("Menu", "Menu", "Menü", "Menu", "Menú").t(), UiAction::Menu);
}

fn setup_hud(mut commands: Commands) {
    let (hud, text) = spawn_hud(&mut commands);
    commands.entity(hud).insert((Hud, Visibility::Hidden));
    commands.entity(text).insert(HudText);
}

fn setup_praise(mut commands: Commands) {
    let root = commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                top: Val::Percent(18.0),
                justify_content: JustifyContent::Center,
                ..default()
            },
            Pickable::IGNORE,
        ))
        .id();
    let text = spawn_text(&mut commands, root, "", 44.0);
    commands.entity(text).insert(Praise { timer: 0.0 });
}

fn update_hud(
    score: Res<Score>,
    state: Res<State<GameState>>,
    mut hud: Query<&mut Visibility, With<Hud>>,
    mut text: Query<&mut Text, With<HudText>>,
) {
    let playing = *state.get() == GameState::Playing;
    for mut v in &mut hud {
        *v = if playing { Visibility::Inherited } else { Visibility::Hidden };
    }
    // Relu aussi au début de chaque partie (la langue a pu changer).
    if score.is_changed() || state.is_changed() {
        let line = tx!(
            "Hauteur : {}    Record : {}    M : son",
            "Height: {}    Best: {}    M: sound",
            "Höhe: {}    Rekord: {}    M: Ton",
            "Altezza: {}    Record: {}    M: audio",
            "Altura: {}    Récord: {}    M: sonido",
        );
        for mut t in &mut text {
            t.0 = line.avec(&[&score.current, &score.best]);
        }
    }
}

/// Affiche « Parfait ! » (avec la série en cours) puis l'efface en fondu.
fn praise(time: Res<Time>, mut events: MessageReader<TowerEvent>, mut q: Query<(&mut Text, &mut TextColor, &mut Praise)>) {
    for (mut text, mut color, mut p) in &mut q {
        for event in events.read() {
            if let TowerEvent::Placed { perfect: true, combo } = *event {
                text.0 = if combo > 1 {
                    tx!("Parfait ×{} !", "Perfect ×{}!", "Perfekt ×{}!", "Perfetto ×{}!", "¡Perfecto ×{}!").avec(&[&combo])
                } else {
                    tx!("Parfait !", "Perfect!", "Perfekt!", "Perfetto!", "¡Perfecto!").t().into()
                };
                p.timer = 1.0;
            }
        }
        p.timer = (p.timer - time.delta_secs()).max(0.0);
        color.0 = palette::TEXT_DARK.with_alpha(p.timer.min(0.6) / 0.6);
    }
}

fn keyboard_shortcuts(
    keys: Res<ButtonInput<KeyCode>>,
    state: Res<State<GameState>>,
    mut next: ResMut<NextState<GameState>>,
) {
    let confirm = keys.just_pressed(KeyCode::Enter);
    let back = keys.just_pressed(KeyCode::Escape);
    match state.get() {
        GameState::Menu if confirm => next.set(GameState::Playing),
        GameState::Playing if back => next.set(GameState::Menu),
        GameState::GameOver if confirm => next.set(GameState::Playing),
        GameState::GameOver if back => next.set(GameState::Menu),
        _ => {}
    }
}

fn on_click(
    click: On<PointerClick>,
    actions: Query<&UiAction>,
    mut next: ResMut<NextState<GameState>>,
    mut clicks: MessageWriter<UiClick>,
) {
    let Ok(action) = actions.get(click.entity) else {
        return;
    };
    clicks.write(UiClick);
    match *action {
        UiAction::Play => next.set(GameState::Playing),
        UiAction::Menu => next.set(GameState::Menu),
    }
}
