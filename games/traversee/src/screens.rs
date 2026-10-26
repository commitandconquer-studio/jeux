//! Petites animations d'interface pour le menu et l'écran de fin : lettres qui
//! ondulent, cartes qui rebondissent, score qui grimpe, barres qui se remplissent.
use bevy::prelude::*;
use game_common::ui::ThemedText;

use crate::GameState;
use crate::characters::Selection;

pub struct ScreensPlugin;

impl Plugin for ScreensPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Menu), spawn_hero)
            .add_systems(Update, (pop, pulse, wave, count_up, bar_fill, hero.run_if(in_state(GameState::Menu))));
    }
}

/// Apparition en rebondissant (après un petit délai).
#[derive(Component)]
pub struct Pop {
    pub age: f32,
    pub delay: f32,
}

impl Pop {
    pub fn after(delay: f32) -> Self {
        Self { age: 0.0, delay }
    }
}

/// Battement régulier (bouton principal, badge de record).
#[derive(Component)]
pub struct Pulse {
    pub speed: f32,
    pub amount: f32,
}

/// Lettre d'un titre qui ondule.
#[derive(Component)]
pub struct Wave(pub usize);

/// Nombre qui grimpe jusqu'à sa valeur.
#[derive(Component)]
pub struct CountUp {
    pub target: u32,
    pub age: f32,
}

/// Barre qui se remplit jusqu'à `target` (0 à 1).
#[derive(Component)]
pub struct BarFill {
    pub target: f32,
    pub age: f32,
}

fn pop(time: Res<Time<Real>>, mut q: Query<(&mut Pop, &mut UiTransform)>) {
    for (mut p, mut tf) in &mut q {
        p.age += time.delta_secs();
        let t = ((p.age - p.delay) / 0.45).clamp(0.0, 1.0);
        // Rebond en fin de course (ease out back)
        let c = 1.70158;
        let s = if t <= 0.0 { 0.0 } else { 1.0 + (c + 1.0) * (t - 1.0).powi(3) + c * (t - 1.0).powi(2) };
        tf.scale = Vec2::splat(s.max(0.0));
    }
}

fn pulse(time: Res<Time<Real>>, mut q: Query<(&Pulse, &mut UiTransform), Without<Pop>>) {
    for (p, mut tf) in &mut q {
        tf.scale = Vec2::splat(1.0 + (time.elapsed_secs() * p.speed).sin() * p.amount);
    }
}

fn wave(time: Res<Time<Real>>, mut q: Query<(&Wave, &mut UiTransform)>) {
    for (w, mut tf) in &mut q {
        let t = time.elapsed_secs() * 3.2 - w.0 as f32 * 0.45;
        tf.translation = Val2::px(0.0, t.sin().max(0.0) * -14.0);
        tf.rotation = Rot2::radians((t * 0.9).sin() * 0.06);
    }
}

fn count_up(time: Res<Time<Real>>, mut q: Query<(&mut CountUp, &mut Text)>) {
    for (mut c, mut text) in &mut q {
        c.age += time.delta_secs();
        let t = ((c.age - 0.3) / 0.9).clamp(0.0, 1.0);
        let ease = 1.0 - (1.0 - t) * (1.0 - t);
        let n = (c.target as f32 * ease).round() as u32;
        let s = n.to_string();
        if text.0 != s {
            text.0 = s;
        }
    }
}

fn bar_fill(time: Res<Time<Real>>, mut q: Query<(&mut BarFill, &mut Node)>) {
    for (mut b, mut node) in &mut q {
        b.age += time.delta_secs();
        let t = ((b.age - 0.4) / 0.8).clamp(0.0, 1.0);
        let ease = 1.0 - (1.0 - t) * (1.0 - t);
        node.width = percent(100.0 * b.target * ease);
    }
}

/// Une barre de progression : renvoie la barre pleine à animer.
pub fn spawn_bar(commands: &mut Commands, parent: Entity, frac: f32, width: f32, color: Color) {
    let track = commands
        .spawn((
            Node { width: px(width), height: px(16), border: UiRect::all(px(2)), border_radius: BorderRadius::all(px(8)), overflow: Overflow::clip(), ..default() },
            BackgroundColor(Color::srgb(0.86, 0.86, 0.84)),
            BorderColor::all(Color::srgb(0.2, 0.22, 0.28)),
            ChildOf(parent),
        ))
        .id();
    commands.spawn((
        Node { width: percent(0), height: percent(100), ..default() },
        BackgroundColor(color),
        BarFill { target: frac.clamp(0.0, 1.0), age: 0.0 },
        ChildOf(track),
    ));
}

/// Une pastille : étiquette colorée puis texte.
pub fn spawn_chip(commands: &mut Commands, parent: Entity, label: &str, text: &str, color: Color, delay: f32) {
    let chip = commands
        .spawn((
            Node {
                height: px(48),
                border: UiRect::all(px(3)),
                border_radius: BorderRadius::all(px(14)),
                overflow: Overflow::clip(),
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.99, 0.97, 0.9)),
            BorderColor::all(color),
            UiTransform::from_scale(Vec2::ZERO),
            Pop::after(delay),
            ChildOf(parent),
        ))
        .id();
    let tag = commands
        .spawn((Node { height: percent(100), padding: UiRect::horizontal(px(12)), align_items: AlignItems::Center, ..default() }, BackgroundColor(color), ChildOf(chip)))
        .id();
    commands.spawn((Text::new(label), TextFont::from_font_size(16.0), TextColor(Color::WHITE), ThemedText, ChildOf(tag)));
    commands.spawn((
        Node { padding: UiRect::horizontal(px(14)), ..default() },
        Text::new(text),
        TextFont::from_font_size(22.0),
        TextColor(Color::srgb(0.12, 0.14, 0.2)),
        ThemedText,
        ChildOf(chip),
    ));
}

// ---- Le héros du menu -----------------------------------------------------------

/// Le personnage choisi, qui saute sur le côté du menu.
#[derive(Component)]
struct Hero;

fn spawn_hero(mut commands: Commands, selection: Res<Selection>, server: Res<AssetServer>, mut graphs: ResMut<Assets<AnimationGraph>>) {
    let root = commands.spawn((Transform::default(), Visibility::default(), Hero, DespawnOnExit(GameState::Menu))).id();
    crate::kenney::humain(&mut commands, &server, &mut graphs, root, selection.character.index(), selection.chair);
}

fn hero(time: Res<Time>, cam: Res<crate::CamZ>, selection: Res<Selection>, mut q: Query<&mut Transform, With<Hero>>) {
    let t = time.elapsed_secs();
    let phase = (t * 2.4).fract();
    // Petit saut régulier, avec écrasement à l'atterrissage et regard vers l'écran.
    // En fauteuil roulant : pas de saut, il reste au sol.
    let roule = selection.chair > 0;
    let air = if roule { 0.0 } else { (phase * std::f32::consts::PI).sin() };
    let squash = if phase < 0.12 && !roule { 1.0 - phase / 0.12 } else { 0.0 };
    for mut tf in &mut q {
        tf.translation = Vec3::new(4.6, air * 1.1, cam.0 - 2.5 + 2.2);
        tf.rotation = Quat::from_rotation_y(std::f32::consts::PI + (t * 0.8).sin() * 0.5);
        tf.scale = Vec3::new(1.0 + squash * 0.25 + air * -0.1, 1.0 - squash * 0.3 + air * 0.25, 1.0 + squash * 0.25 + air * -0.1) * 2.8;
    }
}
