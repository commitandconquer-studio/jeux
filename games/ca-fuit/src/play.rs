//! Le jeu : le crapaud (flèches ou ZQSD) ramasse les pièces sur la prairie.
use bevy::prelude::*;
use game_common::models::{FrogKit, spawn_frog};
use game_common::{Rng, Score, flat_material, iso_camera, palette};

use crate::GameState;

pub struct PlayPlugin;

impl Plugin for PlayPlugin {
    fn build(&self, app: &mut App) {
        // Le record est enregistré par game_common::save (fichier ou navigateur).
        let best = game_common::save::load("ca-fuit", "record").and_then(|v| v.parse().ok()).unwrap_or(0);
        app.insert_resource(Score { current: 0, best })
            .init_resource::<FrogKit>()
            .add_message::<Ramassee>()
            .add_systems(Startup, setup)
            .add_systems(OnEnter(GameState::Playing), (reset, spawn_piece))
            .add_systems(Update, (deplacer, ramasser).chain().run_if(in_state(GameState::Playing)));
    }
}

/// Une pièce vient d'être ramassée (le son l'écoute).
#[derive(Message)]
pub struct Ramassee;

#[derive(Component)]
pub struct Crapaud;

#[derive(Component)]
struct Piece;

#[derive(Resource)]
struct Hasard(Rng);

const TAILLE: f32 = 6.0;

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut mats: ResMut<Assets<StandardMaterial>>, kit: Res<FrogKit>) {
    commands.spawn(iso_camera(Vec3::ZERO, 16.0, 14.0));
    // La prairie en couleurs de la palette commune : les saisons la repeignent.
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(TAILLE * 2.0 + 1.0, 0.4, TAILLE * 2.0 + 1.0))),
        MeshMaterial3d(mats.add(flat_material(palette::GRASS))),
        Transform::from_xyz(0.0, -0.2, 0.0),
    ));
    for (x, z) in [(-5.0, -5.0), (5.5, -4.0), (-4.5, 5.0), (5.0, 5.5)] {
        commands.spawn((
            Mesh3d(meshes.add(Cone { radius: 0.7, height: 1.8 }.mesh().resolution(6))),
            MeshMaterial3d(mats.add(flat_material(palette::FOLIAGE))),
            Transform::from_xyz(x, 0.9, z),
        ));
    }
    let crapaud = spawn_frog(&mut commands, &kit);
    commands.entity(crapaud).insert((Crapaud, Transform::from_scale(Vec3::splat(1.5))));
    commands.insert_resource(Hasard(Rng::from_time()));
}

fn reset(mut score: ResMut<Score>, mut q: Query<&mut Transform, With<Crapaud>>) {
    score.current = 0;
    for mut t in &mut q {
        t.translation = Vec3::ZERO;
    }
}

fn spawn_piece(
    mut commands: Commands,
    mut hasard: ResMut<Hasard>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<StandardMaterial>>,
    pieces: Query<(), With<Piece>>,
) {
    if !pieces.is_empty() {
        return;
    }
    let mut au_hasard = || hasard.0.below(11) as f32 - 5.0;
    commands.spawn((
        Mesh3d(meshes.add(Cylinder::new(0.35, 0.12).mesh().resolution(8))),
        MeshMaterial3d(mats.add(flat_material(palette::YELLOW))),
        Transform::from_xyz(au_hasard(), 0.4, au_hasard()).with_rotation(Quat::from_rotation_x(1.2)),
        Piece,
        DespawnOnExit(GameState::Playing),
    ));
}

fn deplacer(keys: Res<ButtonInput<KeyCode>>, time: Res<Time>, mut q: Query<&mut Transform, With<Crapaud>>) {
    let mut d = Vec3::ZERO;
    // Touches physiques : ZQSD sur AZERTY = WASD sur QWERTY.
    if keys.any_pressed([KeyCode::ArrowUp, KeyCode::KeyW]) { d.z -= 1.0; }
    if keys.any_pressed([KeyCode::ArrowDown, KeyCode::KeyS]) { d.z += 1.0; }
    if keys.any_pressed([KeyCode::ArrowLeft, KeyCode::KeyA]) { d.x -= 1.0; }
    if keys.any_pressed([KeyCode::ArrowRight, KeyCode::KeyD]) { d.x += 1.0; }
    for mut t in &mut q {
        if d != Vec3::ZERO {
            t.translation = (t.translation + d.normalize() * 5.0 * time.delta_secs()).clamp(Vec3::splat(-TAILLE), Vec3::splat(TAILLE));
            t.look_to(d, Vec3::Y);
        }
    }
}

fn ramasser(
    mut commands: Commands,
    mut score: ResMut<Score>,
    crapaud: Query<&Transform, With<Crapaud>>,
    pieces: Query<(Entity, &Transform), With<Piece>>,
    mut evenements: MessageWriter<Ramassee>,
) {
    let Ok(c) = crapaud.single() else { return };
    for (e, p) in &pieces {
        if c.translation.xz().distance(p.translation.xz()) < 0.8 {
            commands.entity(e).despawn();
            score.current += 1;
            if score.current > score.best {
                score.best = score.current;
                game_common::save::store("ca-fuit", "record", &score.best.to_string());
            }
            evenements.write(Ramassee);
            commands.run_system_cached(spawn_piece);
        }
    }
}
