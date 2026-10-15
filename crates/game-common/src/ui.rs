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
    let root = commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
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
    let button = commands
        .spawn((
            Node {
                width: Val::Px(320.0),
                height: Val::Px(56.0),
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
    let text = spawn_text(commands, button, label, 26.0);
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
