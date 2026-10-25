//! Effets visuels : poussière, éclaboussures, éclats, tremblement d'écran.
use bevy::prelude::*;
use game_common::ui::ThemedText;
use game_common::{Rng, flat_material, tx};

use crate::GameState;
use crate::frog::{Cause, Death, Frog};
use crate::models::Kit;

/// Un effet à jouer à cet endroit.
#[derive(Message, Clone, Copy)]
pub enum Fx {
    /// Petit nuage à l'atterrissage.
    Dust(Vec3),
    /// Ronds dans l'eau quand le crapaud se pose sur un rondin.
    Ripple(Vec3),
    /// Grosse gerbe d'eau (noyade).
    Splash(Vec3),
    /// Éclats verts, plumes et étoiles (écrasé).
    Burst(Vec3),
    /// Une mouche gobée : étincelles et points gagnés.
    Gulp(Vec3, u32),
    /// Un nouveau monde commence.
    World(crate::extra::Biome),
    /// Explosion (le Speed-Bus).
    Boom(Vec3),
    /// Plumes qui volent (l'aigle).
    Feathers(Vec3),
    /// Un déblocage vient d'être gagné.
    Unlock(crate::progress::Item),
}

/// Éclair blanc plein écran qui s'éteint tout seul.
#[derive(Resource, Default)]
struct Flash(f32);

/// Coup de zoom de la caméra à l'impact : 1 (zoomée) retombe à 0.
#[derive(Resource, Default)]
pub struct Punch(pub f32);

/// Onde de choc : un anneau qui s'élargit en s'amincissant.
#[derive(Component)]
struct Ring {
    life: f32,
    max: f32,
    size: f32,
}

/// Mot géant façon bande dessinée (« SPLAT ! »).
#[derive(Component)]
struct PopText {
    age: f32,
    size: f32,
}

#[derive(Component)]
struct FlashNode;

/// Tremblement de la caméra : 0 (calme) à 1 (fort), retombe tout seul.
#[derive(Resource, Default)]
pub struct Shake(pub f32);

#[derive(Component)]
struct Particle {
    vel: Vec3,
    life: f32,
    max: f32,
    gravity: f32,
    size: f32,
}

#[derive(Resource)]
struct FxMats {
    dust: Handle<StandardMaterial>,
    water: Handle<StandardMaterial>,
    drop: Handle<StandardMaterial>,
    skin: Handle<StandardMaterial>,
    feather: Handle<StandardMaterial>,
    star: Handle<StandardMaterial>,
    fire: Handle<StandardMaterial>,
    smoke: Handle<StandardMaterial>,
    ring: Handle<Mesh>,
}

pub struct FxPlugin;

impl Plugin for FxPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<Fx>()
            .init_resource::<Shake>()
            .init_resource::<Flash>()
            .init_resource::<Punch>()
            .add_systems(Startup, setup)
            .add_systems(OnEnter(GameState::Dying), death_fx)
            .add_systems(OnExit(GameState::Dying), |mut t: ResMut<Time<Virtual>>| t.set_relative_speed(1.0))
            .add_systems(Update, (spawn_fx, update_particles, update_rings, update_flash, update_pop, update_rise, update_toasts).chain());
    }
}

fn setup(mut commands: Commands, mut mats: ResMut<Assets<StandardMaterial>>, mut meshes: ResMut<Assets<Mesh>>) {
    let mut unlit = |c: Color| mats.add(StandardMaterial { unlit: true, ..flat_material(c) });
    commands.insert_resource(FxMats {
        dust: unlit(Color::srgb(0.96, 0.94, 0.86)),
        water: unlit(Color::srgb(0.75, 0.92, 1.0)),
        drop: unlit(Color::srgb(0.45, 0.75, 1.0)),
        skin: unlit(Color::srgb(0.40, 0.76, 0.34)),
        feather: unlit(Color::srgb(1.0, 0.98, 0.92)),
        star: unlit(Color::srgb(1.0, 0.85, 0.25)),
        fire: unlit(Color::srgb(1.0, 0.45, 0.08)),
        smoke: unlit(Color::srgb(0.22, 0.22, 0.25)),
        ring: meshes.add(Torus::new(0.46, 0.5)),
    });
    commands.spawn((
        Node { position_type: PositionType::Absolute, width: percent(100), height: percent(100), ..default() },
        BackgroundColor(Color::srgba(1.0, 1.0, 1.0, 0.0)),
        GlobalZIndex(50),
        Pickable::IGNORE,
        FlashNode,
    ));
}

fn kit_ring(m: &FxMats) -> Handle<Mesh> {
    m.ring.clone()
}

fn meshes_h(m: &FxMats) -> Handle<Mesh> {
    m.ring.clone()
}

fn ring(c: &mut Commands, mesh: &Handle<Mesh>, mat: &Handle<StandardMaterial>, pos: Vec3, size: f32, life: f32) {
    c.spawn((
        Mesh3d(mesh.clone()),
        MeshMaterial3d(mat.clone()),
        Transform::from_translation(pos).with_scale(Vec3::splat(0.2)),
        Ring { life, max: life, size },
        bevy::light::NotShadowCaster,
    ));
}

fn unit(rng: &mut Rng) -> f32 {
    rng.below(10_000) as f32 / 5_000.0 - 1.0
}

#[allow(clippy::too_many_arguments)]
fn puff(c: &mut Commands, kit: &Kit, mat: &Handle<StandardMaterial>, pos: Vec3, vel: Vec3, size: f32, life: f32, gravity: f32) {
    c.spawn((
        Mesh3d(kit.cube.clone()),
        MeshMaterial3d(mat.clone()),
        Transform::from_translation(pos).with_scale(Vec3::splat(size)),
        Particle { vel, life, max: life, gravity, size },
        bevy::light::NotShadowCaster,
    ));
}

fn spawn_fx(
    mut commands: Commands,
    mut reader: MessageReader<Fx>,
    kit: Res<Kit>,
    mats: Res<FxMats>,
    mut rng: ResMut<Rng>,
    toasts: Query<&Toast>,
    mut sfx: MessageWriter<crate::audio::Sfx>,
) {
    let mut queued = toasts.iter().count();
    for fx in reader.read() {
        match *fx {
            Fx::Dust(p) => {
                for _ in 0..6 {
                    let v = Vec3::new(unit(&mut rng), 0.0, unit(&mut rng)).normalize_or_zero() * (0.7 + 0.5 * unit(&mut rng).abs());
                    puff(&mut commands, &kit, &mats.dust, p + v * 0.12 + Vec3::Y * 0.05, v + Vec3::Y * 0.6, 0.2 + 0.08 * unit(&mut rng).abs(), 0.4, 1.5);
                }
            }
            Fx::Splash(p) => {
                ring(&mut commands, &meshes_h(&mats), &mats.drop, p + Vec3::Y * 0.05, 3.2, 0.7);
                ring(&mut commands, &meshes_h(&mats), &mats.water, p + Vec3::Y * 0.04, 2.0, 0.5);
                for _ in 0..44 {
                    let v = Vec3::new(unit(&mut rng) * 1.6, 4.5 + 3.5 * unit(&mut rng).abs(), unit(&mut rng) * 1.6);
                    puff(&mut commands, &kit, &mats.drop, p + Vec3::Y * 0.1, v, 0.22 + 0.1 * unit(&mut rng).abs(), 0.7, 11.0);
                }
                for _ in 0..12 {
                    let v = Vec3::new(unit(&mut rng), 0.0, unit(&mut rng)).normalize_or_zero() * 1.8;
                    puff(&mut commands, &kit, &mats.water, p + Vec3::Y * 0.05, v, 0.2, 0.5, 0.0);
                }
            }
            Fx::Burst(p) => {
                ring(&mut commands, &meshes_h(&mats), &mats.feather, p + Vec3::Y * 0.1, 4.0, 0.45);
                ring(&mut commands, &meshes_h(&mats), &mats.star, p + Vec3::Y * 0.12, 2.4, 0.35);
                for _ in 0..40 {
                    let v = Vec3::new(unit(&mut rng) * 4.0, 3.0 + 4.0 * unit(&mut rng).abs(), unit(&mut rng) * 4.0);
                    puff(&mut commands, &kit, &mats.skin, p + Vec3::Y * 0.2, v, 0.22 + 0.16 * unit(&mut rng).abs(), 0.8, 10.0);
                }
                for _ in 0..18 {
                    let v = Vec3::new(unit(&mut rng) * 2.4, 1.5 + unit(&mut rng).abs(), unit(&mut rng) * 1.6);
                    puff(&mut commands, &kit, &mats.feather, p + Vec3::Y * 0.3, v, 0.24, 1.1, 2.0);
                }
                for _ in 0..12 {
                    let v = Vec3::new(unit(&mut rng) * 3.0, 3.5 + unit(&mut rng).abs(), unit(&mut rng) * 2.0);
                    puff(&mut commands, &kit, &mats.star, p + Vec3::Y * 0.4, v, 0.32, 0.9, 6.0);
                }
            }
            Fx::Gulp(p, points) => {
                for _ in 0..14 {
                    let v = Vec3::new(unit(&mut rng) * 2.0, 1.5 + 2.0 * unit(&mut rng).abs(), unit(&mut rng) * 2.0);
                    puff(&mut commands, &kit, &mats.star, p, v, 0.14, 0.6, 5.0);
                }
                ring(&mut commands, &kit_ring(&mats), &mats.star, Vec3::new(p.x, p.y - 0.3, p.z), 1.6, 0.35);
                points_text(&mut commands, points);
            }
            Fx::Unlock(item) => {
                spawn_toast(&mut commands, item, queued as f32 * 1.1);
                queued += 1;
                sfx.write(crate::audio::Sfx::Win);
            }
            Fx::Boom(p) => {
                // Boule de feu, fumée noire, éclats jaunes et onde de choc.
                for _ in 0..34 {
                    let v = Vec3::new(unit(&mut rng) * 4.5, 1.0 + 4.0 * unit(&mut rng).abs(), unit(&mut rng) * 3.0);
                    puff(&mut commands, &kit, &mats.fire, p, v, 0.35 + 0.3 * unit(&mut rng).abs(), 0.9, 2.5);
                }
                for _ in 0..16 {
                    let v = Vec3::new(unit(&mut rng) * 3.0, 1.5 + 3.0 * unit(&mut rng).abs(), unit(&mut rng) * 2.0);
                    puff(&mut commands, &kit, &mats.smoke, p + Vec3::Y * 0.3, v, 0.5, 1.6, 0.6);
                }
                for _ in 0..12 {
                    let v = Vec3::new(unit(&mut rng) * 6.0, 2.0 + 4.0 * unit(&mut rng).abs(), unit(&mut rng) * 4.0);
                    puff(&mut commands, &kit, &mats.star, p, v, 0.22, 0.8, 7.0);
                }
                ring(&mut commands, &kit_ring(&mats), &mats.fire, p - Vec3::Y * 0.4, 6.0, 0.55);
            }
            Fx::Feathers(p) => {
                for _ in 0..22 {
                    let v = Vec3::new(unit(&mut rng) * 2.5, 1.5 + 2.5 * unit(&mut rng).abs(), unit(&mut rng) * 2.5);
                    puff(&mut commands, &kit, &mats.feather, p, v, 0.2, 1.3, 2.5);
                }
            }
            Fx::World(b) => {
                let line = tx!("Bienvenue : {}", "Welcome: {}", "Willkommen: {}", "Benvenuti: {}", "Bienvenido: {}").avec(&[&b.name()]);
                pop(&mut commands, &line, Color::srgb(1.0, 1.0, 1.0), 64.0);
            }
            Fx::Ripple(p) => {
                for _ in 0..4 {
                    let v = Vec3::new(unit(&mut rng), 0.0, unit(&mut rng)).normalize_or_zero() * 0.6;
                    puff(&mut commands, &kit, &mats.water, p + Vec3::Y * 0.05, v + Vec3::Y * 1.6, 0.14, 0.4, 7.0);
                }
            }
        }
    }
}

fn update_particles(mut commands: Commands, time: Res<Time>, mut q: Query<(Entity, &mut Particle, &mut Transform)>) {
    let dt = time.delta_secs();
    for (e, mut p, mut tf) in &mut q {
        p.life -= dt;
        if p.life <= 0.0 {
            commands.entity(e).despawn();
            continue;
        }
        p.vel.y -= p.gravity * dt;
        tf.translation += p.vel * dt;
        tf.scale = Vec3::splat(p.size * (p.life / p.max).sqrt());
    }
}

/// Au moment de mourir : effet selon la cause, tremblement, éclair, ralenti.
fn death_fx(
    mut commands: Commands,
    frog: Res<Frog>,
    death: Res<Death>,
    mut fx: MessageWriter<Fx>,
    mut shake: ResMut<Shake>,
    mut flash: ResMut<Flash>,
    mut punch: ResMut<Punch>,
    mut virt: ResMut<Time<Virtual>>,
) {
    let p = frog.visual_pos();
    match death.cause {
        Some(Cause::Squashed) => {
            fx.write(Fx::Burst(p));
            shake.0 = 1.0;
            flash.0 = 0.8;
            punch.0 = 1.0;
            virt.set_relative_speed(0.04);
            pop(&mut commands, tx!("SPLATCH !", "SPLAT!", "KLATSCH!", "SPLAT!", "¡PLAF!").t(), Color::srgb(1.0, 0.85, 0.2), 120.0);
        }
        Some(Cause::Drowned | Cause::CarriedAway) => {
            fx.write(Fx::Splash(Vec3::new(p.x, 0.0, p.z)));
            shake.0 = 0.5;
            punch.0 = 0.6;
            pop(&mut commands, tx!("PLOUF !", "SPLASH!", "PLATSCH!", "SPLASH!", "¡CHOF!").t(), Color::srgb(0.45, 0.8, 1.0), 120.0);
        }
        Some(Cause::Eagle) => {
            fx.write(Fx::Feathers(p + Vec3::Y * 0.4));
            shake.0 = 0.4;
            pop(&mut commands, tx!("ENLEVÉ !", "SNATCHED!", "GEKRALLT!", "RAPITO!", "¡RAPTADO!").t(), Color::srgb(1.0, 0.7, 0.3), 100.0);
        }
        Some(Cause::TooSlow) => {
            fx.write(Fx::Burst(p));
            shake.0 = 0.3;
        }
        None => {}
    }
}

fn update_flash(time: Res<Time<Real>>, mut flash: ResMut<Flash>, mut q: Query<&mut BackgroundColor, With<FlashNode>>) {
    flash.0 = (flash.0 - time.delta_secs() * 3.0).max(0.0);
    for mut c in &mut q {
        c.0 = Color::srgba(1.0, 1.0, 1.0, flash.0);
    }
}

fn pop(commands: &mut Commands, word: &str, color: Color, size: f32) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                width: percent(100),
                top: percent(14),
                justify_content: JustifyContent::Center,
                ..default()
            },
            GlobalZIndex(40),
            Pickable::IGNORE,
            DespawnOnExit(GameState::Dying),
        ))
        .with_children(|p| {
            p.spawn((
                Text::new(word),
                TextFont::from_font_size(10.0),
                TextColor(color),
                TextShadow { offset: Vec2::splat(5.0), color: Color::srgba(0.1, 0.05, 0.0, 0.8) },
                Pickable::IGNORE,
                ThemedText,
                PopText { age: 0.0, size },
            ));
        });
}

/// Le mot jaillit en grossissant, rebondit, puis s'efface.
fn update_pop(time: Res<Time<Real>>, mut q: Query<(&mut PopText, &mut TextFont, &mut TextColor)>) {
    for (mut p, mut font, mut color) in &mut q {
        p.age += time.delta_secs();
        let a = p.age;
        let k = if a < 0.12 {
            0.2 + a / 0.12 * 1.2
        } else if a < 0.3 {
            1.4 - (a - 0.12) / 0.18 * 0.4
        } else {
            1.0
        };
        font.font_size = FontSize::Px((p.size * k).max(1.0));
        let fade = if a > 0.9 { (1.0 - (a - 0.9) / 0.3).max(0.0) } else { 1.0 };
        color.0 = color.0.with_alpha(fade);
    }
}

fn update_rings(mut commands: Commands, time: Res<Time>, mut q: Query<(Entity, &mut Ring, &mut Transform)>) {
    let dt = time.delta_secs();
    for (e, mut r, mut tf) in &mut q {
        r.life -= dt;
        if r.life <= 0.0 {
            commands.entity(e).despawn();
            continue;
        }
        let t = 1.0 - r.life / r.max;
        let out = 1.0 - (1.0 - t) * (1.0 - t); // part vite, finit doucement
        let s = 0.2 + (r.size - 0.2) * out;
        tf.scale = Vec3::new(s, (1.0 - t).max(0.05), s);
    }
}

/// Texte « +5 » qui monte et s'efface sous le score.
#[derive(Component)]
struct Rise {
    age: f32,
}

fn points_text(commands: &mut Commands, points: u32) {
    let gold = points >= 20;
    commands.spawn((
        Text::new(format!("+{points}")),
        TextFont::from_font_size(if gold { 72.0 } else { 54.0 }),
        TextColor(if gold { Color::srgb(1.0, 0.85, 0.2) } else { Color::srgb(0.55, 0.95, 0.4) }),
        TextShadow { offset: Vec2::splat(3.0), color: Color::srgba(0.0, 0.0, 0.0, 0.7) },
        TextLayout::justify(Justify::Center),
        Node { position_type: PositionType::Absolute, width: percent(100), top: percent(12), ..default() },
        GlobalZIndex(35),
        Pickable::IGNORE,
        ThemedText,
        Rise { age: 0.0 },
    ));
}

fn update_rise(mut commands: Commands, time: Res<Time<Real>>, mut q: Query<(Entity, &mut Rise, &mut Node, &mut TextColor)>) {
    for (e, mut r, mut node, mut color) in &mut q {
        r.age += time.delta_secs();
        if r.age > 1.0 {
            commands.entity(e).despawn();
            continue;
        }
        node.top = percent(12.0 - 6.0 * r.age);
        color.0 = color.0.with_alpha((1.0 - (r.age - 0.5).max(0.0) * 2.0).clamp(0.0, 1.0));
    }
}

/// L'annonce d'un déblocage : une carte qui glisse du haut de l'écran.
#[derive(Component)]
struct Toast {
    age: f32,
}

const TOAST_LEN: f32 = 3.4;

fn spawn_toast(commands: &mut Commands, item: crate::progress::Item, delay: f32) {
    let color = item.color();
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: percent(50),
                margin: UiRect::left(px(-250)),
                top: px(-120),
                width: px(500),
                height: px(84),
                border: UiRect::all(px(4)),
                border_radius: BorderRadius::all(px(18)),
                overflow: Overflow::clip(),
                ..default()
            },
            BackgroundColor(Color::srgb(0.99, 0.97, 0.9)),
            BorderColor::all(color),
            GlobalZIndex(60),
            Pickable::IGNORE,
            Toast { age: -delay },
            Visibility::Hidden,
        ))
        .with_children(|p| {
            p.spawn((
                Node { width: px(150), justify_content: JustifyContent::Center, align_items: AlignItems::Center, ..default() },
                BackgroundColor(color),
                Pickable::IGNORE,
            ))
            .with_children(|b| {
                b.spawn((Text::new(item.kind()), TextFont::from_font_size(20.0), TextColor(Color::WHITE), Pickable::IGNORE, ThemedText));
            });
            p.spawn((
                Node { flex_grow: 1.0, flex_direction: FlexDirection::Column, justify_content: JustifyContent::Center, padding: UiRect::horizontal(px(14)), ..default() },
                Pickable::IGNORE,
            ))
            .with_children(|t| {
                t.spawn((
                    Text::new(tx!("Débloqué !", "Unlocked!", "Freigeschaltet!", "Sbloccato!", "¡Desbloqueado!").t()),
                    TextFont::from_font_size(16.0),
                    TextColor(Color::srgb(0.4, 0.4, 0.45)),
                    Pickable::IGNORE,
                    ThemedText,
                ));
                t.spawn((Text::new(item.name()), TextFont::from_font_size(26.0), TextColor(Color::srgb(0.12, 0.14, 0.2)), Pickable::IGNORE, ThemedText));
            });
        });
}

fn update_toasts(mut commands: Commands, time: Res<Time<Real>>, mut q: Query<(Entity, &mut Toast, &mut Node, &mut Visibility)>) {
    for (e, mut t, mut node, mut vis) in &mut q {
        t.age += time.delta_secs();
        if t.age < 0.0 {
            continue;
        }
        *vis = Visibility::Inherited;
        if t.age > TOAST_LEN {
            commands.entity(e).despawn();
            continue;
        }
        // Descend vite, rebondit un peu, reste, puis remonte.
        let slide_in = (t.age / 0.4).min(1.0);
        let ease = 1.0 - (1.0 - slide_in).powi(3);
        let bounce = if t.age < 0.7 { (1.0 - (t.age - 0.4).max(0.0) / 0.3) * (t.age * 30.0).sin() * 6.0 } else { 0.0 };
        let out = ((t.age - (TOAST_LEN - 0.4)) / 0.4).clamp(0.0, 1.0);
        let top = -120.0 + (120.0 + 96.0) * ease + bounce - out * out * 240.0;
        node.top = px(top);
    }
}
