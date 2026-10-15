//! Décorations des fêtes posées sur la table du concile, selon la date du
//! jour (voir `game_common::calendar`). Première série : Noël, Avent, Pâques,
//! Rameaux, Halloween, Épiphanie, Saint-Nicolas.
use std::f32::consts::{FRAC_PI_2, TAU};

use bevy::light::NotShadowCaster;
use bevy::prelude::*;
use game_common::calendar::{Date, Feast, feast_on};
use game_common::{Rng, flat_material};

use crate::board::Look;
use crate::play::Progress;

pub struct FeastPlugin;

impl Plugin for FeastPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (decorate, fall_snow, fly_bats));
    }
}

#[derive(Component)]
struct Decoration;

#[derive(Component)]
struct Snowflake(f32);

#[derive(Component)]
struct Bat {
    radius: f32,
    phase: f32,
    height: f32,
}

#[derive(Component)]
struct Wing(f32);

/// Pose (ou retire) les décorations quand la fête ou le réglage change.
fn decorate(
    mut commands: Commands,
    progress: Res<Progress>,
    look: Res<Look>,
    mut mats: ResMut<Assets<StandardMaterial>>,
    mut current: Local<Option<Option<Feast>>>,
    old: Query<Entity, With<Decoration>>,
) {
    let feast = if progress.feasts { feast_on(Date::today()) } else { None };
    if *current == Some(feast) {
        return;
    }
    *current = Some(feast);
    for e in &old {
        commands.entity(e).despawn();
    }
    // Couleurs des sceaux : rouge d'ordinaire, orange à Halloween, mitre dorée à la Saint-Nicolas.
    if let Some(mut m) = mats.get_mut(&look.wax) {
        m.base_color = if feast == Some(Feast::Halloween) { Color::srgb(0.95, 0.50, 0.10) } else { Color::srgb(0.66, 0.13, 0.16) };
    }
    if let Some(mut m) = mats.get_mut(&look.wax_relief) {
        m.base_color = if feast == Some(Feast::SaintNicholas) { Color::srgb(0.95, 0.78, 0.25) } else { Color::srgb(0.96, 0.89, 0.76) };
    }
    let Some(feast) = feast else { return };
    let mut d = Deco { commands: &mut commands, look: &look, mats: &mut mats };
    match feast {
        Feast::Christmas => d.christmas(),
        Feast::Advent(k) => d.advent_wreath(k),
        Feast::Easter => d.easter(),
        Feast::PalmSunday => d.palms(),
        Feast::Halloween => d.halloween(),
        Feast::Epiphany => d.galette(),
        _ => {}
    }
}

struct Deco<'a, 'w, 's> {
    commands: &'a mut Commands<'w, 's>,
    look: &'a Look,
    mats: &'a mut Assets<StandardMaterial>,
}

impl Deco<'_, '_, '_> {
    fn mat(&mut self, c: Color) -> Handle<StandardMaterial> {
        self.mats.add(flat_material(c))
    }

    fn glow(&mut self, c: Color, k: f32) -> Handle<StandardMaterial> {
        self.mats.add(StandardMaterial { emissive: c.to_linear() * k, ..flat_material(c) })
    }

    fn put(&mut self, mesh: &Handle<Mesh>, mat: &Handle<StandardMaterial>, pos: Vec3, scale: Vec3, rot: Quat) -> Entity {
        self.commands.spawn((Mesh3d(mesh.clone()), MeshMaterial3d(mat.clone()), Transform { translation: pos, rotation: rot, scale }, Decoration)).id()
    }

    /// Sapin, étoile, boules, et neige qui tombe.
    fn christmas(&mut self) {
        let (cone, ball, cyl, q) = (self.look.cone.clone(), self.look.ball.clone(), self.look.cyl.clone(), Quat::IDENTITY);
        let base = Vec3::new(-10.5, 0.0, 0.5);
        let (green, trunk) = (self.mat(Color::srgb(0.16, 0.45, 0.25)), self.mat(Color::srgb(0.40, 0.25, 0.15)));
        self.put(&cyl, &trunk, base + Vec3::Y * 0.4, Vec3::new(0.5, 0.8, 0.5), q);
        for (k, w) in [3.2, 2.5, 1.8].iter().enumerate() {
            self.put(&cone, &green, base + Vec3::Y * (1.6 + k as f32 * 1.1), Vec3::new(*w, 2.0, *w), q);
        }
        let star = self.glow(Color::srgb(1.0, 0.85, 0.3), 3.0);
        self.put(&ball, &star, base + Vec3::Y * 4.6, Vec3::splat(0.6), q);
        let colors = [Color::srgb(0.9, 0.2, 0.2), Color::srgb(0.95, 0.8, 0.2), Color::srgb(0.2, 0.5, 0.95), Color::srgb(0.9, 0.9, 0.95)];
        for k in 0..10 {
            let a = k as f32 * 2.4;
            let y = 1.4 + (k % 5) as f32 * 0.55;
            let r = 1.45 - (y - 1.4) * 0.38;
            let m = self.glow(colors[k % 4], 0.6);
            self.put(&ball, &m, base + Vec3::new(a.cos() * r, y, a.sin() * r), Vec3::splat(0.3), q);
        }
        // Neige
        let snow = self.glow(Color::WHITE, 0.6);
        let mut rng = Rng::new(25);
        for _ in 0..160 {
            let pos = Vec3::new(rng.below(300) as f32 / 10.0 - 15.0, rng.below(140) as f32 / 10.0, rng.below(200) as f32 / 10.0 - 10.0);
            let e = self.put(&ball, &snow, pos, Vec3::splat(0.09 + rng.below(8) as f32 / 100.0), q);
            self.commands.entity(e).insert((Snowflake(0.6 + rng.below(80) as f32 / 100.0), NotShadowCaster));
        }
    }

    /// Couronne de l'Avent : une bougie allumée de plus chaque semaine.
    fn advent_wreath(&mut self, lit: u8) {
        let (cyl, cone, ball, q) = (self.look.cyl.clone(), self.look.cone.clone(), self.look.ball.clone(), Quat::IDENTITY);
        let base = Vec3::new(-10.5, 0.0, 0.5);
        let leaf = self.mat(Color::srgb(0.15, 0.40, 0.22));
        for k in 0..18 {
            let a = k as f32 / 18.0 * TAU;
            self.put(&ball, &leaf, base + Vec3::new(a.cos() * 1.6, 0.25, a.sin() * 1.6), Vec3::new(0.8, 0.5, 0.8), Quat::from_rotation_y(a));
        }
        let (purple, pink) = (self.mat(Color::srgb(0.45, 0.25, 0.60)), self.mat(Color::srgb(0.90, 0.55, 0.70)));
        let flame = self.glow(Color::srgb(1.0, 0.75, 0.3), 6.0);
        for k in 0..4u8 {
            let a = k as f32 * FRAC_PI_2 + 0.4;
            let p = base + Vec3::new(a.cos() * 1.6, 0.0, a.sin() * 1.6);
            let wax = if k == 2 { pink.clone() } else { purple.clone() };
            self.put(&cyl, &wax, p + Vec3::Y * 1.0, Vec3::new(0.38, 1.6, 0.38), q);
            if k < lit {
                self.put(&cone, &flame, p + Vec3::Y * 2.0, Vec3::new(0.18, 0.4, 0.18), q);
            }
        }
    }

    /// Œufs colorés et cloche de Pâques.
    fn easter(&mut self) {
        let (ball, cone, q) = (self.look.ball.clone(), self.look.cone.clone(), Quat::IDENTITY);
        let colors = [
            Color::srgb(0.95, 0.55, 0.65),
            Color::srgb(0.55, 0.75, 0.95),
            Color::srgb(0.95, 0.85, 0.35),
            Color::srgb(0.60, 0.85, 0.55),
            Color::srgb(0.75, 0.60, 0.90),
            Color::srgb(0.95, 0.65, 0.35),
        ];
        let spots = [(-10.0, 2.5), (-11.2, 1.6), (-9.6, -0.8), (10.8, 0.6), (11.4, -1.0), (-11.0, -2.6), (10.2, 2.4)];
        for (k, (x, z)) in spots.iter().enumerate() {
            let m = self.mat(colors[k % colors.len()]);
            self.put(&ball, &m, Vec3::new(*x, 0.45, *z), Vec3::new(0.7, 0.9, 0.7), Quat::from_rotation_z(0.2 * (k as f32 - 3.0)));
        }
        let gold = self.mat(Color::srgb(0.90, 0.72, 0.25));
        let bell = Vec3::new(-10.5, 2.4, -0.5);
        self.put(&cone, &gold, bell, Vec3::new(1.8, 1.8, 1.8), q);
        self.put(&ball, &gold, bell - Vec3::Y * 1.0, Vec3::splat(0.45), q);
        let ribbon = self.mat(Color::srgb(0.85, 0.20, 0.25));
        self.put(&self.look.cube.clone(), &ribbon, bell + Vec3::Y * 0.95, Vec3::new(1.2, 0.25, 0.3), q);
    }

    /// Branches de buis et palmes des Rameaux.
    fn palms(&mut self) {
        let cube = self.look.cube.clone();
        let (stem, leaf) = (self.mat(Color::srgb(0.45, 0.55, 0.25)), self.mat(Color::srgb(0.30, 0.60, 0.25)));
        for (k, (x, z, yaw)) in [(-10.5, 1.0, 0.3), (-10.0, -1.5, -0.4), (10.5, 1.8, 2.8)].iter().enumerate() {
            let rot = Quat::from_rotation_y(*yaw);
            let base = Vec3::new(*x, 0.1, *z);
            self.put(&cube, &stem, base, Vec3::new(4.0, 0.08, 0.08), rot);
            for j in 0..9 {
                let t = j as f32 / 8.0 * 3.4 - 1.6;
                for s in [-1.0, 1.0] {
                    let along = rot * Vec3::X * t;
                    let r = rot * Quat::from_rotation_y(s * (0.7 + k as f32 * 0.05));
                    self.put(&cube, &leaf, base + along + r * Vec3::X * 0.35, Vec3::new(0.8 - (t.abs() * 0.12), 0.04, 0.16), r);
                }
            }
        }
    }

    /// Citrouilles et chauves-souris qui tournent au-dessus de la table.
    fn halloween(&mut self) {
        let (ball, cyl, cube, q) = (self.look.ball.clone(), self.look.cyl.clone(), self.look.cube.clone(), Quat::IDENTITY);
        let (orange, stem) = (self.mat(Color::srgb(0.95, 0.50, 0.10)), self.mat(Color::srgb(0.30, 0.40, 0.15)));
        let face = self.glow(Color::srgb(1.0, 0.8, 0.2), 4.0);
        for (x, z, s) in [(-10.5, 0.5, 2.0), (-11.0, -2.4, 1.3), (10.8, 2.2, 1.5)] {
            let p = Vec3::new(x, s * 0.4, z);
            self.put(&ball, &orange, p, Vec3::new(s, s * 0.8, s), q);
            self.put(&cyl, &stem, p + Vec3::Y * s * 0.45, Vec3::new(0.15 * s, 0.25 * s, 0.15 * s), q);
            // Yeux et bouche lumineux
            for ex in [-0.2, 0.2] {
                self.put(&cube, &face, p + Vec3::new(ex * s, 0.1 * s, s * 0.47), Vec3::new(0.14 * s, 0.14 * s, 0.05), Quat::from_rotation_z(0.785));
            }
            self.put(&cube, &face, p + Vec3::new(0.0, -0.12 * s, s * 0.47), Vec3::new(0.4 * s, 0.07 * s, 0.05), q);
        }
        let black = self.mat(Color::srgb(0.08, 0.06, 0.10));
        for k in 0..5 {
            let root = self
                .commands
                .spawn((Transform::default(), Visibility::default(), Bat { radius: 6.0 + k as f32, phase: k as f32 * 1.3, height: 4.0 + (k % 3) as f32 }, Decoration))
                .id();
            self.commands.spawn((Mesh3d(ball.clone()), MeshMaterial3d(black.clone()), Transform::from_scale(Vec3::splat(0.3)), ChildOf(root)));
            for s in [-1.0, 1.0] {
                self.commands.spawn((
                    Mesh3d(cube.clone()),
                    MeshMaterial3d(black.clone()),
                    Transform::from_xyz(0.35 * s, 0.0, 0.0).with_scale(Vec3::new(0.6, 0.04, 0.3)),
                    Wing(s),
                    ChildOf(root),
                ));
            }
        }
    }

    /// Galette des rois et couronne en papier doré.
    fn galette(&mut self) {
        let (cyl, cone, q) = (self.look.cyl.clone(), self.look.cone.clone(), Quat::IDENTITY);
        let (pastry, top) = (self.mat(Color::srgb(0.85, 0.62, 0.30)), self.mat(Color::srgb(0.78, 0.52, 0.22)));
        let g = Vec3::new(-10.5, 0.0, 1.0);
        self.put(&cyl, &pastry, g + Vec3::Y * 0.15, Vec3::new(3.0, 0.3, 3.0), q);
        self.put(&cyl, &top, g + Vec3::Y * 0.31, Vec3::new(2.7, 0.02, 2.7), q);
        let gold = self.mat(Color::srgb(0.95, 0.80, 0.25));
        let crown = Vec3::new(-10.3, 0.0, -2.4);
        self.put(&cyl, &gold, crown + Vec3::Y * 0.2, Vec3::new(1.6, 0.4, 1.6), q);
        for k in 0..8 {
            let a = k as f32 / 8.0 * TAU;
            self.put(&cone, &gold, crown + Vec3::new(a.cos() * 0.72, 0.65, a.sin() * 0.72), Vec3::new(0.3, 0.5, 0.12), Quat::from_rotation_y(-a));
        }
    }
}

fn fall_snow(time: Res<Time>, mut q: Query<(&Snowflake, &mut Transform)>) {
    let t = time.elapsed_secs();
    for (s, mut tf) in &mut q {
        tf.translation.y -= s.0 * time.delta_secs();
        tf.translation.x += (t * s.0 + tf.translation.z).sin() * 0.004;
        if tf.translation.y < 0.0 {
            tf.translation.y += 14.0;
        }
    }
}

fn fly_bats(time: Res<Time>, mut bats: Query<(&Bat, &mut Transform), Without<Wing>>, mut wings: Query<(&Wing, &mut Transform), Without<Bat>>) {
    let t = time.elapsed_secs();
    for (b, mut tf) in &mut bats {
        let a = t * 0.5 + b.phase;
        tf.translation = Vec3::new(a.cos() * b.radius, b.height + (t * 2.0 + b.phase).sin() * 0.4, a.sin() * b.radius * 0.6);
        tf.rotation = Quat::from_rotation_y(-a);
    }
    for (w, mut tf) in &mut wings {
        tf.rotation = Quat::from_rotation_z(w.0 * (t * 14.0).sin() * 0.7);
    }
}
