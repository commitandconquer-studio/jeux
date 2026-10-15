//! Le plateau en 3D : univers, sol, murs, cibles, caisses.
use std::f32::consts::FRAC_PI_2;

use bevy::light::NotShadowCaster;
use bevy::prelude::*;
use game_common::{flat_material, sounds};

use crate::level::Level;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Universe {
    Prairie,
    Desert,
    Banquise,
    Ocean,
    Volcan,
}

impl Universe {
    /// Les niveaux traversent les univers dans l'ordre, puis recommencent.
    pub fn of_level(index: usize) -> Self {
        [Universe::Prairie, Universe::Desert, Universe::Banquise, Universe::Ocean, Universe::Volcan][index % 5]
    }

    pub fn music(self) -> &'static str {
        match self {
            Universe::Prairie => sounds::MUSIC_PRAIRIE,
            Universe::Desert => sounds::MUSIC_DESERT,
            Universe::Banquise => sounds::MUSIC_BANQUISE,
            Universe::Ocean => sounds::MUSIC_OCEAN,
            Universe::Volcan => sounds::MUSIC_VOLCAN,
        }
    }

    fn theme(self) -> Theme {
        let c = Color::srgb;
        match self {
            Universe::Prairie => Theme {
                sky: c(0.80, 0.90, 0.95),
                ground: c(0.68, 0.84, 0.55),
                tiles: (c(0.90, 0.86, 0.68), c(0.85, 0.80, 0.62)),
                wall: c(0.22, 0.46, 0.26),
                cap: c(0.32, 0.58, 0.32),
                target: c(0.98, 0.80, 0.22),
                glow: None,
                sun: 6_000.0,
                ambient: 900.0,
            },
            Universe::Desert => Theme {
                sky: c(0.98, 0.90, 0.76),
                ground: c(0.88, 0.72, 0.48),
                tiles: (c(0.96, 0.86, 0.62), c(0.92, 0.80, 0.55)),
                wall: c(0.80, 0.60, 0.40),
                cap: c(0.88, 0.70, 0.48),
                target: c(0.90, 0.35, 0.30),
                glow: None,
                sun: 7_000.0,
                ambient: 900.0,
            },
            Universe::Banquise => Theme {
                sky: c(0.86, 0.93, 0.98),
                ground: c(0.95, 0.97, 1.0),
                tiles: (c(0.86, 0.94, 1.0), c(0.79, 0.89, 0.97)),
                wall: c(0.62, 0.82, 0.95),
                cap: c(0.98, 0.99, 1.0),
                target: c(0.35, 0.55, 0.95),
                glow: None,
                sun: 5_500.0,
                ambient: 1_000.0,
            },
            Universe::Ocean => Theme {
                sky: c(0.80, 0.92, 0.96),
                ground: c(0.25, 0.60, 0.82),
                tiles: (c(0.97, 0.90, 0.70), c(0.93, 0.86, 0.64)),
                wall: c(0.55, 0.57, 0.62),
                cap: c(0.66, 0.68, 0.72),
                target: c(0.95, 0.45, 0.55),
                glow: None,
                sun: 6_500.0,
                ambient: 900.0,
            },
            Universe::Volcan => Theme {
                sky: c(0.22, 0.16, 0.18),
                ground: c(0.95, 0.35, 0.08),
                tiles: (c(0.38, 0.35, 0.37), c(0.32, 0.30, 0.33)),
                wall: c(0.18, 0.16, 0.18),
                cap: c(0.26, 0.23, 0.25),
                target: c(1.0, 0.80, 0.20),
                glow: Some(LinearRgba::rgb(2.0, 0.45, 0.05)),
                sun: 4_500.0,
                ambient: 500.0,
            },
        }
    }
}

struct Theme {
    sky: Color,
    ground: Color,
    tiles: (Color, Color),
    wall: Color,
    cap: Color,
    target: Color,
    /// Lueur du sol extérieur (lave).
    glow: Option<LinearRgba>,
    sun: f32,
    ambient: f32,
}

/// Tout ce qui appartient au plateau en cours (supprimé au changement de niveau).
#[derive(Component)]
pub struct BoardPart;

/// La caisse numéro `0` (indice dans `State::boxes`).
#[derive(Component)]
pub struct Crate(pub usize);

/// Matériaux d'une caisse, normale ou rangée sur une cible.
#[derive(Resource)]
pub struct CrateLook {
    pub wood: Handle<StandardMaterial>,
    pub frame: Handle<StandardMaterial>,
    pub wood_ok: Handle<StandardMaterial>,
    pub frame_ok: Handle<StandardMaterial>,
}

/// Les pièces d'une caisse dont la couleur change quand elle est rangée.
#[derive(Component)]
pub struct CratePiece {
    pub frame: bool,
}

impl FromWorld for CrateLook {
    fn from_world(world: &mut World) -> Self {
        let mut mats = world.resource_mut::<Assets<StandardMaterial>>();
        let mut m = |r, g, b| mats.add(flat_material(Color::srgb(r, g, b)));
        CrateLook {
            wood: m(0.80, 0.58, 0.35),
            frame: m(0.55, 0.36, 0.20),
            wood_ok: m(0.62, 0.82, 0.42),
            frame_ok: m(0.36, 0.56, 0.26),
        }
    }
}

/// Case du niveau → position dans le monde (plateau centré sur l'origine).
pub fn to_world(level: &Level, p: Vec2) -> Vec3 {
    Vec3::new(p.x - (level.width - 1) as f32 / 2.0, 0.0, p.y - (level.height - 1) as f32 / 2.0)
}

#[allow(clippy::too_many_arguments)]
pub fn build(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    mats: &mut Assets<StandardMaterial>,
    look: &CrateLook,
    level: &Level,
    universe: Universe,
    clear: &mut ClearColor,
    ambient: &mut GlobalAmbientLight,
    suns: &mut Query<&mut DirectionalLight>,
) {
    let th = universe.theme();
    clear.0 = th.sky;
    ambient.brightness = th.ambient;
    for mut sun in suns.iter_mut() {
        sun.illuminance = th.sun;
    }

    let cube = meshes.add(Cuboid::new(1.0, 1.0, 1.0));
    let disc = meshes.add(Cylinder::new(0.5, 1.0).mesh().resolution(20).build());
    let mut mat = |c: Color| mats.add(flat_material(c));
    let (tile_a, tile_b, wall, cap) = (mat(th.tiles.0), mat(th.tiles.1), mat(th.wall), mat(th.cap));
    let target = mats.add(StandardMaterial { emissive: th.target.to_linear() * 0.6, ..flat_material(th.target) });
    let ground = mats.add(StandardMaterial { emissive: th.glow.unwrap_or(LinearRgba::BLACK), ..flat_material(th.ground) });

    let mut spawn = |mesh: &Handle<Mesh>, m: &Handle<StandardMaterial>, pos: Vec3, scale: Vec3, flat: bool| {
        let mut e = commands.spawn((Mesh3d(mesh.clone()), MeshMaterial3d(m.clone()), Transform::from_translation(pos).with_scale(scale), BoardPart));
        if flat {
            e.insert(NotShadowCaster);
        }
    };

    // Sol extérieur
    spawn(&cube, &ground, Vec3::new(0.0, -0.35, 0.0), Vec3::new(80.0, 0.2, 80.0), true);
    // Cases de sol en damier, et cibles
    for &p in &level.floor {
        let tile = if (p.x + p.y) % 2 == 0 { &tile_a } else { &tile_b };
        spawn(&cube, tile, to_world(level, p.as_vec2()) - Vec3::Y * 0.1, Vec3::new(1.0, 0.2, 1.0), true);
    }
    for &t in &level.targets {
        spawn(&disc, &target, to_world(level, t.as_vec2()) + Vec3::Y * 0.01, Vec3::new(0.62, 0.02, 0.62), true);
    }
    // Murs, avec un chapeau d'une autre teinte
    for &w in &level.walls {
        let p = to_world(level, w.as_vec2());
        // Murs bas : ils ne doivent pas cacher les cibles et caisses derrière eux.
        spawn(&cube, &wall, p + Vec3::Y * 0.15, Vec3::new(1.0, 0.5, 1.0), false);
        spawn(&cube, &cap, p + Vec3::Y * 0.43, Vec3::new(0.92, 0.06, 0.92), false);
    }

    // Caisses en bois : corps, montants aux coins, croisillon sur le dessus.
    for (i, &b) in level.start.boxes.iter().enumerate() {
        let root = commands
            .spawn((Transform::from_translation(to_world(level, b.as_vec2())), Visibility::default(), Crate(i), BoardPart))
            .id();
        let mut piece = |pos: Vec3, scale: Vec3, rot: Quat, frame: bool| {
            let m = if frame { &look.frame } else { &look.wood };
            commands.spawn((
                Mesh3d(cube.clone()),
                MeshMaterial3d(m.clone()),
                Transform { translation: pos, rotation: rot, scale },
                CratePiece { frame },
                ChildOf(root),
            ));
        };
        piece(Vec3::Y * 0.42, Vec3::splat(0.8), Quat::IDENTITY, false);
        for (x, z) in [(-1.0, -1.0), (-1.0, 1.0), (1.0, -1.0), (1.0, 1.0)] {
            piece(Vec3::new(x * 0.38, 0.42, z * 0.38), Vec3::new(0.1, 0.84, 0.1), Quat::IDENTITY, true);
        }
        for a in [FRAC_PI_2 / 2.0, -FRAC_PI_2 / 2.0] {
            piece(Vec3::Y * 0.83, Vec3::new(1.0, 0.04, 0.1), Quat::from_rotation_y(a), true);
        }
    }
}
