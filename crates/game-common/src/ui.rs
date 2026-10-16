//! Interface commune à tous les jeux : panneaux, boutons, police, survol.
//! Le style (couleurs, police) vient de l'univers du jeu (`Universe`).
use bevy::prelude::*;
use bevy::text::{FontFeatureTag, FontFeatures};

use crate::brand::{self, colors};
use crate::palette;
use crate::universe::{UiStyle, Universe};

/// Fond de panneau de l'univers Croâ & Cie (pour les éléments sans thème).
pub const PANEL: Color = Color::srgba(0.98, 0.97, 0.94, 0.93);

/// Ajoute la police et le style de l'univers, et l'effet de survol des boutons.
pub struct UiKitPlugin {
    pub universe: Universe,
}

impl Plugin for UiKitPlugin {
    fn build(&self, app: &mut App) {
        let style = self.universe.style();
        let server = app.world().resource::<AssetServer>();
        let font = server.load(style.font);
        let logo = StudioLogo { emblem: server.load(brand::EMBLEM_MENU), font: server.load(brand::FONT_LOGO) };
        app.insert_resource(UiFont(font))
            .insert_resource(logo)
            .insert_resource(UiTheme(style))
            .insert_resource(self.universe)
            .add_systems(PostUpdate, (apply_font, apply_theme, fill_logo))
            .add_observer(on_over)
            .add_observer(on_out)
            .add_observer(on_help)
            .add_observer(on_quit);
        if std::env::var_os("CC_OPEN_HELP").is_some() {
            app.add_systems(Update, open_help_for_test);
        }
    }
}

/// Police de l'interface (elle doit contenir les accents, contrairement à la
/// police intégrée à Bevy).
#[derive(Resource)]
pub struct UiFont(pub Handle<Font>);

/// Style de l'univers du jeu en cours.
#[derive(Resource)]
pub struct UiTheme(pub UiStyle);

/// Marque un bouton : couleurs de l'univers et survol.
#[derive(Component)]
pub struct UiButton;

/// Couleur de fond propre à un bouton (gardée par le thème et après le survol).
#[derive(Component, Clone, Copy)]
pub struct ButtonColor(pub Color);

/// Marque un panneau : fond et bordure de l'univers.
#[derive(Component)]
pub struct ThemedPanel;

/// Panneau d'une fenêtre (Options, Aide, Ambiance) : fond de l'univers, mais
/// opaque, pour rester lisible au-dessus du menu.
#[derive(Component)]
pub struct PanneauOpaque;

/// Marque un texte : couleur de l'univers (s'il n'a pas reçu d'autre couleur).
#[derive(Component)]
pub struct ThemedText;

/// Texte qui garde sa propre police (logo du studio…).
#[derive(Component)]
pub struct OwnFont;

fn apply_font(font: Res<UiFont>, mut texts: Query<&mut TextFont, (Added<TextFont>, Without<OwnFont>)>) {
    for mut tf in &mut texts {
        *tf = tf.clone().with_font(font.0.clone());
        // Chiffres alignés : les polices serif (Gelasio) dessinent sinon des
        // chiffres « elzéviriens » qui descendent sous la ligne.
        tf.font_features = FontFeatures::builder().enable(FontFeatureTag::LINING_FIGURES).build();
    }
}

#[allow(clippy::type_complexity)]
fn apply_theme(
    theme: Res<UiTheme>,
    mut panels: Query<(&mut BackgroundColor, &mut BorderColor, Has<PanneauOpaque>), (Added<ThemedPanel>, Without<UiButton>)>,
    mut buttons: Query<(&mut BackgroundColor, &mut BorderColor, Option<&ButtonColor>), (Added<UiButton>, Without<ThemedPanel>)>,
    mut texts: Query<&mut TextColor, Added<ThemedText>>,
) {
    let s = &theme.0;
    for (mut bg, mut border, opaque) in &mut panels {
        bg.0 = if opaque { s.panel.with_alpha(1.0) } else { s.panel };
        *border = BorderColor::all(s.panel_border);
    }
    for (mut bg, mut border, own) in &mut buttons {
        bg.0 = own.map_or(s.button, |c| c.0);
        *border = BorderColor::all(s.button_border);
    }
    for mut color in &mut texts {
        // Un texte recoloré exprès par le jeu garde sa couleur.
        if color.0 == palette::TEXT_DARK {
            color.0 = s.text;
        }
    }
}

fn on_over(over: On<PointerOver>, theme: Res<UiTheme>, mut q: Query<&mut BackgroundColor, With<UiButton>>) {
    if let Ok(mut bg) = q.get_mut(over.entity) {
        bg.0 = theme.0.button_hover;
    }
}

fn on_out(out: On<PointerOut>, theme: Res<UiTheme>, mut q: Query<(&mut BackgroundColor, Option<&ButtonColor>), With<UiButton>>) {
    if let Ok((mut bg, own)) = q.get_mut(out.entity) {
        bg.0 = own.map_or(theme.0.button, |c| c.0);
    }
}

/// Fond plein écran centré avec un panneau arrondi ; renvoie le panneau.
/// `scope` est ajouté à la racine, typiquement `DespawnOnExit(état)`.
pub fn spawn_panel(commands: &mut Commands, scope: impl Bundle) -> Entity {
    panel_at(commands, scope, JustifyContent::Center, AlignItems::Center)
}

/// Comme `spawn_panel`, mais en bas de l'écran (laisse voir la scène).
pub fn spawn_panel_bottom(commands: &mut Commands, scope: impl Bundle) -> Entity {
    panel_at(commands, scope, JustifyContent::Center, AlignItems::FlexEnd)
}

/// Comme `spawn_panel`, mais à gauche de l'écran (la scène reste visible à droite).
pub fn spawn_panel_left(commands: &mut Commands, scope: impl Bundle) -> Entity {
    panel_at(commands, scope, JustifyContent::FlexStart, AlignItems::Center)
}

fn panel_at(commands: &mut Commands, scope: impl Bundle, horizontal: JustifyContent, vertical: AlignItems) -> Entity {
    let root = commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: horizontal,
                align_items: vertical,
                padding: UiRect::all(Val::Px(24.0)),
                ..default()
            },
            scope,
        ))
        .id();
    commands
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                row_gap: Val::Px(14.0),
                padding: UiRect::axes(Val::Px(48.0), Val::Px(32.0)),
                border: UiRect::all(Val::Px(2.0)),
                border_radius: BorderRadius::all(Val::Px(24.0)),
                ..default()
            },
            BackgroundColor(PANEL),
            BorderColor::all(Color::NONE),
            ThemedPanel,
            ChildOf(root),
        ))
        .id()
}

pub fn spawn_text(commands: &mut Commands, parent: Entity, text: impl Into<String>, size: f32) -> Entity {
    commands
        .spawn((
            Text::new(text),
            TextFont::from_font_size(size),
            TextColor(palette::TEXT_DARK),
            TextLayout::justify(Justify::Center),
            Pickable::IGNORE,
            ThemedText,
            ChildOf(parent),
        ))
        .id()
}

/// Espace vertical dans un panneau.
pub fn spawn_gap(commands: &mut Commands, parent: Entity, height: f32) {
    commands.spawn((Node { height: Val::Px(height), ..default() }, ChildOf(parent)));
}

/// Crée un bouton ; renvoie `(bouton, texte)`. Ajoutez votre composant
/// d'action sur le bouton.
pub fn spawn_button(commands: &mut Commands, parent: Entity, label: impl Into<String>) -> (Entity, Entity) {
    spawn_button_sized(commands, parent, label, 320.0, 56.0, 26.0)
}

/// Bouton de taille libre (grilles de niveaux, petits boutons de jeu).
pub fn spawn_button_sized(
    commands: &mut Commands,
    parent: Entity,
    label: impl Into<String>,
    width: f32,
    height: f32,
    font_size: f32,
) -> (Entity, Entity) {
    let button = commands
        .spawn((
            Node {
                width: Val::Px(width),
                height: Val::Px(height),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(3.0)),
                border_radius: BorderRadius::all(Val::Px(14.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.98, 0.97, 0.93)),
            BorderColor::all(palette::TEXT_DARK),
            UiButton,
            ChildOf(parent),
        ))
        .id();
    let text = spawn_text(commands, button, label, font_size);
    (button, text)
}

/// Bandeau d'information en haut à gauche ; renvoie `(bandeau, texte)`.
pub fn spawn_hud(commands: &mut Commands) -> (Entity, Entity) {
    let hud = commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(14.0),
                left: Val::Px(14.0),
                padding: UiRect::axes(Val::Px(18.0), Val::Px(8.0)),
                border: UiRect::all(Val::Px(2.0)),
                border_radius: BorderRadius::all(Val::Px(20.0)),
                ..default()
            },
            BackgroundColor(PANEL),
            BorderColor::all(Color::NONE),
            ThemedPanel,
        ))
        .id();
    let text = spawn_text(commands, hud, "", 24.0);
    (hud, text)
}

/// Emblème et police du logo du studio (chargés une fois pour tous les menus).
#[derive(Resource)]
pub struct StudioLogo {
    pub emblem: Handle<Image>,
    pub font: Handle<Font>,
}

/// Morceaux du logo à compléter par `fill_logo` (image et police).
#[derive(Component)]
enum LogoPart {
    Emblem,
    Word,
}

fn fill_logo(logo: Res<StudioLogo>, mut parts: Query<(&LogoPart, Option<&mut ImageNode>, Option<&mut TextFont>), Added<LogoPart>>) {
    for (part, image, font) in &mut parts {
        match part {
            LogoPart::Emblem => {
                if let Some(mut image) = image {
                    image.image = logo.emblem.clone();
                }
            }
            LogoPart::Word => {
                if let Some(mut font) = font {
                    font.font = logo.font.clone().into();
                }
            }
        }
    }
}

/// Logo du studio : l'emblème CC et « COMMIT AND CONQUER », en ligne.
/// À placer en bas des menus.
pub fn spawn_studio_logo(commands: &mut Commands, parent: Entity) -> Entity {
    let row = commands
        .spawn((
            Node { align_items: AlignItems::Center, column_gap: Val::Px(10.0), margin: UiRect::top(Val::Px(6.0)), ..default() },
            Pickable::IGNORE,
            ChildOf(parent),
        ))
        .id();
    commands.spawn((
        Node { width: Val::Px(52.0), height: Val::Px(57.0), ..default() },
        ImageNode::default(),
        LogoPart::Emblem,
        Pickable::IGNORE,
        ChildOf(row),
    ));
    let words = commands
        .spawn((Node { flex_direction: FlexDirection::Column, align_items: AlignItems::Center, ..default() }, Pickable::IGNORE, ChildOf(row)))
        .id();
    let word = |commands: &mut Commands, parent: Entity, text: &str, size: f32, color: Option<Color>| {
        let mut e = commands.spawn((
            Text::new(text),
            TextFont::from_font_size(size),
            TextColor(color.unwrap_or(palette::TEXT_DARK)),
            OwnFont,
            LogoPart::Word,
            Pickable::IGNORE,
            ChildOf(parent),
        ));
        if color.is_none() {
            e.insert(ThemedText);
        }
    };
    word(commands, words, "COMMIT", 19.0, None);
    // « AND » en vert, entre deux traits.
    let and = commands
        .spawn((Node { align_items: AlignItems::Center, column_gap: Val::Px(4.0), ..default() }, Pickable::IGNORE, ChildOf(words)))
        .id();
    let line = |commands: &mut Commands| {
        commands.spawn((Node { width: Val::Px(20.0), height: Val::Px(2.0), ..default() }, BackgroundColor(colors::GREEN), ChildOf(and)));
    };
    line(commands);
    word(commands, and, "AND", 11.0, Some(colors::GREEN));
    line(commands);
    word(commands, words, "CONQUER", 19.0, None);
    row
}

/// Texte d'aide du jeu (règle et commandes) dans les cinq langues, affiché
/// par le bouton « Aide ».
#[derive(Resource, Clone)]
pub struct GameHelp(pub crate::langue::Texte);

/// Bouton qui ouvre l'aide (voir `GameHelp`).
#[derive(Component)]
pub struct HelpButton;

/// Bouton qui ferme l'aide.
#[derive(Component)]
struct HelpClose;

/// Fond de l'aide (tout l'écran, au-dessus du menu).
#[derive(Component)]
struct HelpOverlay;

/// Bouton « Aide » de taille standard.
pub fn spawn_help_button(commands: &mut Commands, parent: Entity) -> Entity {
    let (button, _) = spawn_button(commands, parent, tx!("Aide", "Help", "Hilfe", "Aiuto", "Ayuda").t());
    commands.entity(button).insert(HelpButton);
    button
}

fn open_help(commands: &mut Commands, help: Option<&GameHelp>) {
    // Le panneau d'aide passe au-dessus du menu et voile la scène.
    let panel = spawn_panel(commands, (HelpOverlay, BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.35)), GlobalZIndex(50)));
    commands.entity(panel).insert(PanneauOpaque);
    spawn_text(commands, panel, tx!("Aide", "Help", "Hilfe", "Aiuto", "Ayuda").t(), 40.0);
    let text = help.map_or("", |h| h.0.t());
    let t = spawn_text(commands, panel, text, 19.0);
    commands.entity(t).insert(TextLayout::justify(Justify::Left));
    spawn_gap(commands, panel, 4.0);
    let (b, _) = spawn_button(commands, panel, tx!("Fermer", "Close", "Schließen", "Chiudi", "Cerrar").t());
    commands.entity(b).insert(HelpClose);
}

/// Test : `CC_OPEN_HELP=1` ouvre l'aide au bout de 2 secondes (pour les captures).
fn open_help_for_test(time: Res<Time>, mut done: Local<bool>, mut commands: Commands, help: Option<Res<GameHelp>>) {
    if !*done && time.elapsed_secs() > 2.0 {
        *done = true;
        open_help(&mut commands, help.as_deref());
    }
}

fn on_help(
    click: On<PointerClick>,
    mut commands: Commands,
    open: Query<(), With<HelpButton>>,
    close: Query<(), With<HelpClose>>,
    overlays: Query<Entity, With<HelpOverlay>>,
    help: Option<Res<GameHelp>>,
) {
    if open.contains(click.entity) && overlays.is_empty() {
        open_help(&mut commands, help.as_deref());
    } else if close.contains(click.entity) {
        for e in &overlays {
            commands.entity(e).despawn();
        }
    }
}

/// Bouton « Quitter » commun (absent dans le navigateur et sur Android).
#[derive(Component)]
pub struct QuitButton;

fn on_quit(click: On<PointerClick>, q: Query<(), With<QuitButton>>, mut exit: MessageWriter<AppExit>) {
    if q.contains(click.entity) {
        exit.write(AppExit::Success);
    }
}

/// Le pied de tous les menus du studio : Options, Aide, Quitter (sauf dans le
/// navigateur et sur Android) et le logo. Les boutons propres au jeu viennent
/// avant. Le jeu fournit son texte d'aide avec la ressource `GameHelp`.
pub fn pied_de_menu(commands: &mut Commands, panel: Entity) {
    crate::options::spawn_options_button(commands, panel);
    spawn_help_button(commands, panel);
    #[cfg(not(any(target_os = "android", target_arch = "wasm32")))]
    {
        let (b, _) = spawn_button(commands, panel, tx!("Quitter", "Quit", "Beenden", "Esci", "Salir").t());
        commands.entity(b).insert(QuitButton);
    }
    spawn_studio_logo(commands, panel);
}
