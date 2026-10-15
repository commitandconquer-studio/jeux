//! Personnages et objets 3D partagés entre les jeux.
use bevy::prelude::*;

use crate::flat_material;

/// Formes et couleurs du crapaud (créées une fois, partagées).
#[derive(Resource)]
pub struct FrogKit {
    cube: Handle<Mesh>,
    ball: Handle<Mesh>,
    skin: Handle<StandardMaterial>,
    dark: Handle<StandardMaterial>,
    belly: Handle<StandardMaterial>,
    white: Handle<StandardMaterial>,
    black: Handle<StandardMaterial>,
}

impl FromWorld for FrogKit {
    fn from_world(world: &mut World) -> Self {
        let mut meshes = world.resource_mut::<Assets<Mesh>>();
        let cube = meshes.add(Cuboid::new(1.0, 1.0, 1.0));
        let ball = meshes.add(Sphere::new(0.5).mesh().uv(16, 12));
        let mut mats = world.resource_mut::<Assets<StandardMaterial>>();
        let mut m = |r, g, b| mats.add(flat_material(Color::srgb(r, g, b)));
        FrogKit {
            cube,
            ball,
            skin: m(0.36, 0.72, 0.30),
            dark: m(0.22, 0.50, 0.20),
            belly: m(0.86, 0.90, 0.55),
            white: m(0.97, 0.97, 0.97),
            black: m(0.06, 0.06, 0.08),
        }
    }
}

fn part(c: &mut Commands, parent: Entity, mesh: &Handle<Mesh>, mat: &Handle<StandardMaterial>, pos: Vec3, scale: Vec3) {
    c.spawn((
        Mesh3d(mesh.clone()),
        MeshMaterial3d(mat.clone()),
        Transform::from_translation(pos).with_scale(scale),
        ChildOf(parent),
    ));
}

/// Le crapaud de Commit & Conquer, tête vers -Z, posé en y = 0.
/// Renvoie l'entité racine (à déplacer / tourner / étirer).
pub fn spawn_frog(c: &mut Commands, k: &FrogKit) -> Entity {
    let root = c.spawn((Transform::default(), Visibility::default())).id();
    // Corps trapu, ventre clair, tête plate
    part(c, root, &k.cube, &k.skin, Vec3::new(0.0, 0.27, 0.05), Vec3::new(0.62, 0.32, 0.62));
    part(c, root, &k.cube, &k.belly, Vec3::new(0.0, 0.12, 0.0), Vec3::new(0.56, 0.1, 0.62));
    part(c, root, &k.cube, &k.skin, Vec3::new(0.0, 0.32, -0.3), Vec3::new(0.56, 0.24, 0.3));
    // Bouche
    part(c, root, &k.cube, &k.dark, Vec3::new(0.0, 0.24, -0.452), Vec3::new(0.42, 0.03, 0.02));
    for s in [-1.0, 1.0] {
        // Gros yeux ronds qui dépassent
        part(c, root, &k.ball, &k.skin, Vec3::new(0.17 * s, 0.45, -0.3), Vec3::splat(0.22));
        part(c, root, &k.ball, &k.white, Vec3::new(0.17 * s, 0.49, -0.34), Vec3::splat(0.18));
        part(c, root, &k.ball, &k.black, Vec3::new(0.17 * s, 0.5, -0.42), Vec3::splat(0.08));
        // Pattes arrière repliées et pattes avant
        part(c, root, &k.cube, &k.dark, Vec3::new(0.36 * s, 0.12, 0.18), Vec3::new(0.2, 0.18, 0.42));
        part(c, root, &k.cube, &k.dark, Vec3::new(0.42 * s, 0.03, -0.02), Vec3::new(0.22, 0.05, 0.18));
        part(c, root, &k.cube, &k.skin, Vec3::new(0.26 * s, 0.1, -0.34), Vec3::new(0.11, 0.2, 0.11));
        part(c, root, &k.cube, &k.dark, Vec3::new(0.27 * s, 0.02, -0.4), Vec3::new(0.16, 0.04, 0.12));
    }
    // Taches sur le dos
    for (x, z, s) in [(-0.13, 0.12, 0.12), (0.12, 0.22, 0.1), (0.06, -0.02, 0.08)] {
        part(c, root, &k.cube, &k.dark, Vec3::new(x, 0.435, z), Vec3::new(s, 0.02, s));
    }
    root
}
