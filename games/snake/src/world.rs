//! Les univers : plateau, sol, décor et lumière.
use bevy::light::NotShadowCaster;
use bevy::prelude::*;
use game_common::{Rng, flat_material, palette, tx};

use crate::settings::Settings;

/// Hauteur du dessus du sol extérieur.
const GROUND_Y: f32 = -0.2;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum Universe {
    #[default]
    Prairie,
    Desert,
    Banquise,
    Ocean,
    Volcan,
}

impl Universe {
    const ALL: [Universe; 5] =
        [Universe::Prairie, Universe::Desert, Universe::Banquise, Universe::Ocean, Universe::Volcan];

    pub fn next(self) -> Self {
        let i = Self::ALL.iter().position(|u| *u == self).unwrap_or(0);
        Self::ALL[(i + 1) % Self::ALL.len()]
    }

    pub fn label(self) -> &'static str {
        match self {
            Universe::Prairie => tx!("Prairie", "Meadow", "Wiese", "Prateria", "Pradera"),
            Universe::Desert => tx!("Désert", "Desert", "Wüste", "Deserto", "Desierto"),
            Universe::Banquise => tx!("Banquise", "Ice floe", "Packeis", "Banchisa", "Banquisa"),
            Universe::Ocean => tx!("Océan", "Ocean", "Ozean", "Oceano", "Océano"),
            Universe::Volcan => tx!("Volcan", "Volcano", "Vulkan", "Vulcano", "Volcán"),
        }
        .t()
    }

    fn theme(self) -> Theme {
        let c = Color::srgb;
        match self {
            Universe::Prairie => Theme {
                sky: palette::SKY,
                tiles: (palette::GRASS_LIGHT, palette::GRASS),
                rim: palette::FOLIAGE_DARK,
                ground: palette::PATH,
                ground_glow: None,
                sun: (Color::WHITE, 6_000.0),
                ambient: 900.0,
            },
            Universe::Desert => Theme {
                sky: c(0.98, 0.90, 0.76),
                tiles: (c(0.97, 0.87, 0.62), c(0.93, 0.81, 0.54)),
                rim: c(0.80, 0.62, 0.38),
                ground: c(0.89, 0.73, 0.49),
                ground_glow: None,
                sun: (c(1.0, 0.95, 0.85), 7_000.0),
                ambient: 900.0,
            },
            Universe::Banquise => Theme {
                sky: c(0.86, 0.93, 0.98),
                tiles: (c(0.90, 0.96, 1.0), c(0.79, 0.90, 0.97)),
                rim: c(0.55, 0.72, 0.86),
                ground: c(0.96, 0.97, 1.0),
                ground_glow: None,
                sun: (c(0.92, 0.96, 1.0), 5_500.0),
                ambient: 1_000.0,
            },
            Universe::Ocean => Theme {
                sky: c(0.80, 0.92, 0.96),
                tiles: (c(0.98, 0.91, 0.72), c(0.94, 0.86, 0.64)),
                rim: c(0.85, 0.75, 0.52),
                ground: c(0.22, 0.60, 0.80),
                ground_glow: None,
                sun: (Color::WHITE, 6_500.0),
                ambient: 900.0,
            },
            Universe::Volcan => Theme {
                sky: c(0.20, 0.15, 0.17),
                tiles: (c(0.36, 0.33, 0.35), c(0.29, 0.27, 0.30)),
                rim: c(0.16, 0.14, 0.16),
                ground: c(0.95, 0.35, 0.08),
                ground_glow: Some(LinearRgba::rgb(2.0, 0.45, 0.05)),
                sun: (c(1.0, 0.72, 0.52), 4_000.0),
                ambient: 450.0,
            },
        }
    }
}

struct Theme {
    sky: Color,
    tiles: (Color, Color),
    rim: Color,
    ground: Color,
    ground_glow: Option<LinearRgba>,
    sun: (Color, f32),
    ambient: f32,
}

/// Tout ce qui appartient au décor, supprimé quand on change d'univers.
#[derive(Component)]
pub struct WorldPart;

/// Petit mouvement de flottaison (bouées, vagues, lave).
#[derive(Component)]
pub struct Bob {
    base_y: f32,
    phase: f32,
    amp: f32,
}

/// Case de la grille → position dans le monde (grille centrée sur l'origine).
pub fn cell_to_world(c: Vec2, y: f32, grid: i32) -> Vec3 {
    let off = (grid - 1) as f32 / 2.0;
    Vec3::new(c.x - off, y, c.y - off)
}

/// Reconstruit le décor quand l'univers ou la taille du plateau change.
#[allow(clippy::too_many_arguments)]
pub fn rebuild_world(
    mut commands: Commands,
    settings: Res<Settings>,
    mut built: Local<Option<(Universe, i32)>>,
    old: Query<Entity, With<WorldPart>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut clear: ResMut<ClearColor>,
    mut ambient: ResMut<GlobalAmbientLight>,
    mut suns: Query<&mut DirectionalLight>,
) {
    let key = (settings.universe, settings.grid());
    if *built == Some(key) {
        return;
    }
    *built = Some(key);
    for e in &old {
        commands.entity(e).despawn();
    }

    let (universe, grid) = key;
    let theme = universe.theme();
    clear.0 = theme.sky;
    ambient.brightness = theme.ambient;
    for mut sun in &mut suns {
        sun.color = theme.sun.0;
        sun.illuminance = theme.sun.1;
    }

    let cube = meshes.add(Cuboid::new(1.0, 1.0, 1.0));
    let mut b = Builder { commands: &mut commands, cube, mats: &mut materials, cache: Vec::new() };

    // Sol extérieur
    let ground = match theme.ground_glow {
        Some(glow) => b.glow(theme.ground, glow),
        None => b.mat(theme.ground),
    };
    let e = b.block_with(ground, Vec3::new(0.0, GROUND_Y - 0.2, 0.0), Vec3::new(90.0, 0.2, 90.0), 0.0);
    b.flat(e);

    // Socle et damier du plateau. Ces pièces plates ne projettent pas d'ombre :
    // sinon elles s'ombrent elles-mêmes et des rayures apparaissent.
    let g = grid as f32;
    // Le socle s'arrête sous le dessus des cases (sinon les deux surfaces se
    // superposent et scintillent).
    let e = b.block(theme.rim, Vec3::new(0.0, GROUND_Y, 0.0), Vec3::new(g + 0.3, 0.2, g + 0.3));
    b.flat(e);
    for x in 0..grid {
        for z in 0..grid {
            let color = if (x + z) % 2 == 0 { theme.tiles.0 } else { theme.tiles.1 };
            let p = cell_to_world(Vec2::new(x as f32, z as f32), -0.05, grid);
            let e = b.block(color, p, Vec3::new(0.96, 0.1, 0.96));
            b.flat(e);
        }
    }

    // Décor sur un anneau autour du plateau
    let mut rng = Rng::new(universe as u64 * 7919 + grid as u64);
    let half = g / 2.0;
    let reach = half + 10.0;
    let mut x = -reach;
    while x <= reach {
        let mut z = -reach;
        while z <= reach {
            let jitter = Vec3::new(rng.below(100) as f32 / 100.0 - 0.5, 0.0, rng.below(100) as f32 / 100.0 - 0.5);
            let pos = Vec3::new(x, GROUND_Y, z) + jitter;
            let inside = pos.x.abs() < half + 1.8 && pos.z.abs() < half + 1.8;
            if !inside && rng.below(10) < 6 {
                spawn_decor(universe, &mut b, &mut rng, pos);
            }
            z += 3.0;
        }
        x += 3.0;
    }
}

fn spawn_decor(u: Universe, b: &mut Builder, rng: &mut Rng, pos: Vec3) {
    let c = Color::srgb;
    let yaw = rng.below(4) as f32 * 0.4;
    match (u, rng.below(3)) {
        // Prairie : immeubles, arbres
        (Universe::Prairie, 0) => {
            let (w, h) = (1.8 + rng.below(2) as f32 * 0.6, 1.0 + rng.below(3) as f32);
            b.block(palette::WALL, pos, Vec3::new(w, h, w));
            b.block(palette::ROOF, pos + Vec3::Y * h, Vec3::new(w, 0.9, w));
        }
        (Universe::Prairie, 1) => {
            b.block(palette::FOLIAGE_DARK, pos, Vec3::new(2.4, 0.9, 2.4));
            b.block(palette::FOLIAGE, pos + Vec3::Y * 0.9, Vec3::splat(1.1));
        }
        (Universe::Prairie, _) => {
            b.block(c(0.50, 0.36, 0.25), pos, Vec3::new(0.25, 0.6, 0.25));
            b.block(palette::FOLIAGE, pos + Vec3::Y * 0.6, Vec3::splat(0.9));
        }

        // Désert : cactus, pyramides, rochers
        (Universe::Desert, 0) => {
            let green = c(0.36, 0.62, 0.36);
            let h = 1.4 + rng.below(3) as f32 * 0.4;
            b.block(green, pos, Vec3::new(0.5, h, 0.5));
            b.block(green, pos + Vec3::new(-0.45, 0.6, 0.0), Vec3::new(0.45, 0.3, 0.3));
            b.block(green, pos + Vec3::new(-0.6, 0.6, 0.0), Vec3::new(0.3, 0.7, 0.3));
            b.block(green, pos + Vec3::new(0.45, 0.9, 0.0), Vec3::new(0.45, 0.3, 0.3));
            b.block(green, pos + Vec3::new(0.6, 0.9, 0.0), Vec3::new(0.3, 0.5, 0.3));
        }
        (Universe::Desert, 1) => {
            for (i, w) in [3.2, 2.4, 1.6, 0.8].into_iter().enumerate() {
                b.block(c(0.86, 0.69, 0.43), pos + Vec3::Y * (i as f32 * 0.6), Vec3::new(w, 0.6, w));
            }
        }
        (Universe::Desert, _) => {
            b.block_rot(c(0.66, 0.48, 0.34), pos, Vec3::new(1.4, 0.7, 1.1), yaw);
        }

        // Banquise : sapins enneigés, igloos, blocs de glace
        (Universe::Banquise, 0) => {
            b.block(c(0.45, 0.32, 0.22), pos, Vec3::new(0.3, 0.5, 0.3));
            for (i, w) in [1.6, 1.2, 0.8].into_iter().enumerate() {
                let y = 0.5 + i as f32 * 0.55;
                b.block(c(0.20, 0.42, 0.33), pos + Vec3::Y * y, Vec3::new(w, 0.45, w));
                b.block(Color::WHITE, pos + Vec3::Y * (y + 0.45), Vec3::new(w * 0.9, 0.1, w * 0.9));
            }
        }
        (Universe::Banquise, 1) => {
            let snow = c(0.97, 0.98, 1.0);
            b.block(snow, pos, Vec3::new(2.0, 1.0, 2.0));
            b.block(snow, pos + Vec3::Y, Vec3::new(1.3, 0.6, 1.3));
            b.block(c(0.85, 0.90, 0.96), pos + Vec3::new(0.0, 0.0, 1.2), Vec3::new(0.7, 0.6, 0.5));
        }
        (Universe::Banquise, _) => {
            let ice = c(0.70, 0.88, 0.97);
            b.block_rot(ice, pos, Vec3::splat(1.2), yaw);
            if rng.below(2) == 0 {
                b.block_rot(ice, pos + Vec3::Y * 1.2, Vec3::splat(0.7), yaw + 0.5);
            }
        }

        // Océan : îlots à palmier, bouées, vagues
        (Universe::Ocean, 0) => {
            b.block(c(0.96, 0.88, 0.66), pos, Vec3::new(2.6, 0.4, 2.6));
            b.block(c(0.55, 0.40, 0.26), pos + Vec3::Y * 0.4, Vec3::new(0.25, 1.6, 0.25));
            let leaf = c(0.30, 0.66, 0.35);
            b.block_rot(leaf, pos + Vec3::Y * 2.0, Vec3::new(2.0, 0.15, 0.5), yaw);
            b.block_rot(leaf, pos + Vec3::Y * 2.0, Vec3::new(0.5, 0.15, 2.0), yaw);
        }
        (Universe::Ocean, 1) => {
            let e1 = b.block(c(0.90, 0.25, 0.25), pos, Vec3::new(0.5, 0.5, 0.5));
            let e2 = b.block(Color::WHITE, pos + Vec3::Y * 0.5, Vec3::new(0.5, 0.25, 0.5));
            let phase = rng.below(628) as f32 / 100.0;
            b.bob(e1, pos.y, phase, 0.12);
            b.bob(e2, pos.y + 0.5, phase, 0.12);
        }
        (Universe::Ocean, _) => {
            let e = b.block_rot(c(0.55, 0.80, 0.92), pos, Vec3::new(1.6, 0.12, 0.6), yaw);
            b.bob(e, pos.y, rng.below(628) as f32 / 100.0, 0.08);
        }

        // Volcan : orgues basaltiques, cônes, roches incandescentes
        (Universe::Volcan, 0) => {
            let dark = c(0.20, 0.19, 0.21);
            for i in 0..3 {
                let h = 1.0 + rng.below(4) as f32 * 0.6;
                b.block(dark, pos + Vec3::new(i as f32 * 0.8 - 0.8, 0.0, 0.0), Vec3::new(0.75, h, 0.75));
            }
        }
        (Universe::Volcan, 1) => {
            for (i, w) in [3.0, 2.2, 1.4].into_iter().enumerate() {
                b.block(c(0.25, 0.22, 0.23), pos + Vec3::Y * (i as f32 * 0.7), Vec3::new(w, 0.7, w));
            }
            let lava = b.glow(c(1.0, 0.45, 0.1), LinearRgba::rgb(4.0, 1.0, 0.1));
            b.block_with(lava, pos + Vec3::Y * 2.1, Vec3::new(0.8, 0.15, 0.8), 0.0);
        }
        (Universe::Volcan, _) => {
            let ember = b.glow(c(1.0, 0.5, 0.15), LinearRgba::rgb(3.0, 0.8, 0.1));
            let e = b.block_with(ember, pos, Vec3::splat(0.6), yaw);
            b.bob(e, pos.y, rng.below(628) as f32 / 100.0, 0.15);
        }
    }
}

/// Aide à la construction de blocs, avec un cache de matériaux.
struct Builder<'a, 'w, 's> {
    commands: &'a mut Commands<'w, 's>,
    cube: Handle<Mesh>,
    mats: &'a mut Assets<StandardMaterial>,
    cache: Vec<(Color, Handle<StandardMaterial>)>,
}

impl Builder<'_, '_, '_> {
    fn mat(&mut self, color: Color) -> Handle<StandardMaterial> {
        if let Some((_, h)) = self.cache.iter().find(|(c, _)| *c == color) {
            return h.clone();
        }
        let h = self.mats.add(flat_material(color));
        self.cache.push((color, h.clone()));
        h
    }

    fn glow(&mut self, color: Color, emissive: LinearRgba) -> Handle<StandardMaterial> {
        self.mats.add(StandardMaterial { emissive, ..flat_material(color) })
    }

    /// Pose un bloc dont `base` est le centre de la face inférieure.
    fn block(&mut self, color: Color, base: Vec3, size: Vec3) -> Entity {
        self.block_rot(color, base, size, 0.0)
    }

    fn block_rot(&mut self, color: Color, base: Vec3, size: Vec3, yaw: f32) -> Entity {
        let m = self.mat(color);
        self.block_with(m, base, size, yaw)
    }

    fn block_with(&mut self, mat: Handle<StandardMaterial>, base: Vec3, size: Vec3, yaw: f32) -> Entity {
        self.commands
            .spawn((
                Mesh3d(self.cube.clone()),
                MeshMaterial3d(mat),
                Transform::from_translation(base + Vec3::Y * size.y / 2.0)
                    .with_rotation(Quat::from_rotation_y(yaw))
                    .with_scale(size),
                WorldPart,
            ))
            .id()
    }

    fn flat(&mut self, e: Entity) {
        self.commands.entity(e).insert(NotShadowCaster);
    }

    fn bob(&mut self, e: Entity, bottom_y: f32, phase: f32, amp: f32) {
        self.commands.entity(e).insert(Bob { base_y: bottom_y, phase, amp });
    }
}

pub fn animate_bobs(time: Res<Time>, mut q: Query<(&Bob, &mut Transform)>) {
    let t = time.elapsed_secs();
    for (bob, mut tf) in &mut q {
        tf.translation.y = bob.base_y + tf.scale.y / 2.0 + (t * 1.8 + bob.phase).sin() * bob.amp;
    }
}
