//! Interface commune à tous les jeux : panneaux, boutons, police, survol.
use bevy::prelude::*;

use crate::palette;

pub const PANEL: Color = Color::srgba(0.98, 0.97, 0.94, 0.93);
const BUTTON: Color = Color::srgb(0.98, 0.97, 0.93);
const BUTTON_HOVER: Color = Color::srgb(1.0, 0.86, 0.40);

/// Ajoute la police de l'interface et l'effet de survol des boutons.
///
/// `font_path` est relatif au dossier `assets/` du jeu.
pub struct UiKitPlugin {
    pub font_path: &'static str,
}

impl Plugin for UiKitPlugin {
    fn build(&self, app: &mut App) {
        let font = app.world().resource::<AssetServer>().load(self.font_path);
        app.insert_resource(UiFont(font))
            .add_systems(PostUpdate, apply_font)
            .add_observer(on_over)
            .add_observer(on_out);
    }
}

/// Police de l'interface (elle doit contenir les accents, contrairement à la
/// police intégrée à Bevy).
#[derive(Resource)]
pub struct UiFont(pub Handle<Font>);

/// Marque un bouton : couleur au survol.
#[derive(Component)]
pub struct UiButton;

fn apply_font(font: Res<UiFont>, mut texts: Query<&mut TextFont, Added<TextFont>>) {
    for mut tf in &mut texts {
        *tf = tf.clone().with_font(font.0.clone());
    }
}

fn on_over(over: On<PointerOver>, mut q: Query<&mut BackgroundColor, With<UiButton>>) {
    if let Ok(mut bg) = q.get_mut(over.entity) {
        bg.0 = BUTTON_HOVER;
    }
}

fn on_out(out: On<PointerOut>, mut q: Query<&mut BackgroundColor, With<UiButton>>) {
    if let Ok(mut bg) = q.get_mut(out.entity) {
        bg.0 = BUTTON;
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
                border_radius: BorderRadius::all(Val::Px(24.0)),
                ..default()
            },
            BackgroundColor(PANEL),
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
            BackgroundColor(BUTTON),
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
                border_radius: BorderRadius::all(Val::Px(20.0)),
                ..default()
            },
            BackgroundColor(PANEL),
        ))
        .id();
    let text = spawn_text(commands, hud, "", 24.0);
    (hud, text)
}
