//! Les univers : plateau, sol, décor et lumière.
use bevy::light::NotShadowCaster;
use bevy::prelude::*;
use game_common::{Rng, flat_material, tx};

use crate::layout::Layout;
use crate::settings::Settings;

/// Hauteur du dessus du sol extérieur.
const GROUND_Y: f32 = -0.2;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum Universe {
    #[default]
    Prairie,
    Desert,
    Cimetiere,
    Ocean,
}

impl Universe {
    const ALL: [Universe; 4] = [Universe::Prairie, Universe::Desert, Universe::Cimetiere, Universe::Ocean];

    pub fn next(self) -> Self {
        let i = Self::ALL.iter().position(|u| *u == self).unwrap_or(0);
        Self::ALL[(i + 1) % Self::ALL.len()]
    }

    pub fn label(self) -> &'static str {
        match self {
            Universe::Prairie => tx!("Prairie", "Meadow", "Wiese", "Prateria", "Pradera"),
            Universe::Desert => tx!("Désert", "Desert", "Wüste", "Deserto", "Desierto"),
            Universe::Cimetiere => tx!("Cimetière", "Graveyard", "Friedhof", "Cimitero", "Cementerio"),
            Universe::Ocean => tx!("Île des pirates", "Pirate island", "Pirateninsel", "Isola dei pirati", "Isla pirata"),
        }
        .t()
    }

    /// Ambiance « Kenney » de Cross or Die : ciel gris-bleu doux, ombres lavande.
    fn theme(self) -> Theme {
        let c = Color::srgb;
        match self {
            Universe::Prairie => Theme {
                sky: c(0.662, 0.692, 0.757),
                tiles: (c(0.56, 0.78, 0.40), c(0.50, 0.72, 0.36)),
                rim: c(0.42, 0.30, 0.22),
                ground: c(0.38, 0.62, 0.32),
                sun: (Color::WHITE, 6_500.0),
                ambient: (c(0.495, 0.482, 0.565), 1000.0),
            },
            Universe::Desert => Theme {
                sky: c(0.93, 0.84, 0.70),
                tiles: (c(0.97, 0.87, 0.62), c(0.93, 0.81, 0.54)),
                rim: c(0.72, 0.52, 0.32),
                ground: c(0.89, 0.73, 0.49),
                sun: (c(1.0, 0.95, 0.85), 7_500.0),
                ambient: (c(0.62, 0.52, 0.45), 900.0),
            },
            Universe::Cimetiere => Theme {
                sky: c(0.42, 0.38, 0.52),
                tiles: (c(0.45, 0.47, 0.50), c(0.40, 0.42, 0.46)),
                rim: c(0.25, 0.24, 0.28),
                ground: c(0.30, 0.34, 0.30),
                sun: (c(0.80, 0.82, 1.0), 3_500.0),
                ambient: (c(0.50, 0.47, 0.68), 1100.0),
            },
            Universe::Ocean => Theme {
                sky: c(0.72, 0.88, 0.95),
                tiles: (c(0.98, 0.91, 0.72), c(0.94, 0.86, 0.64)),
                rim: c(0.60, 0.42, 0.26),
                ground: c(0.22, 0.58, 0.78),
                sun: (Color::WHITE, 6_500.0),
                ambient: (c(0.55, 0.60, 0.70), 1000.0),
            },
        }
    }
}

struct Theme {
    sky: Color,
    tiles: (Color, Color),
    rim: Color,
    ground: Color,
    sun: (Color, f32),
    ambient: (Color, f32),
}

/// Tout ce qui appartient au décor, supprimé quand on change d'univers.
#[derive(Component)]
pub struct WorldPart;

/// Case de la grille → position dans le monde (grille centrée sur l'origine).
pub fn cell_to_world(c: Vec2, y: f32, l: &Layout) -> Vec3 {
    Vec3::new(c.x - (l.w - 1) as f32 / 2.0, y, c.y - (l.h - 1) as f32 / 2.0)
}

/// Reconstruit le décor quand l'univers ou la taille du plateau change.
#[allow(clippy::too_many_arguments)]
pub fn rebuild_world(
    mut commands: Commands,
    settings: Res<Settings>,
    layout: Res<Layout>,
    mut built: Local<Option<(Universe, crate::layout::Shape)>>,
    old: Query<Entity, With<WorldPart>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut clear: ResMut<ClearColor>,
    mut ambient: ResMut<GlobalAmbientLight>,
    mut suns: Query<&mut DirectionalLight>,
    server: Res<AssetServer>,
) {
    let key = (settings.universe, settings.shape);
    if *built == Some(key) {
        return;
    }
    *built = Some(key);
    for e in &old {
        commands.entity(e).despawn();
    }

    let (universe, shape) = key;
    let l = &*layout;
    let theme = universe.theme();
    clear.0 = theme.sky;
    ambient.color = theme.ambient.0;
    ambient.brightness = theme.ambient.1;
    for mut sun in &mut suns {
        sun.color = theme.sun.0;
        sun.illuminance = theme.sun.1;
    }

    let cube = meshes.add(Cuboid::new(1.0, 1.0, 1.0));
    let mut b = Builder { commands: &mut commands, cube, mats: &mut materials, cache: Vec::new(), server: &server };

    // Sol extérieur
    let ground = b.mat(theme.ground);
    let e = b.block_with(ground, Vec3::new(0.0, GROUND_Y - 0.2, 0.0), Vec3::new(90.0, 0.2, 90.0), 0.0);
    b.flat(e);

    // Socle et damier du plateau. Ces pièces plates ne projettent pas d'ombre :
    // sinon elles s'ombrent elles-mêmes et des rayures apparaissent.
    // Le socle (sous chaque case praticable) s'arrête sous le dessus des cases
    // (sinon les deux surfaces se superposent et scintillent).
    for c in l.cells() {
        let color = if (c.x + c.y) % 2 == 0 { theme.tiles.0 } else { theme.tiles.1 };
        let p = cell_to_world(c.as_vec2(), -0.05, l);
        let e = b.block(color, p, Vec3::new(0.96, 0.1, 0.96));
        b.flat(e);
        let e = b.block(theme.rim, Vec3::new(p.x, GROUND_Y, p.z), Vec3::new(1.2, 0.2, 1.2));
        b.flat(e);
    }

    // Décor autour du plateau, y compris dans les creux des formes en L, en U ou en route.
    let mut rng = Rng::new(universe as u64 * 7919 + shape as u64 * 31);
    let reach_x = l.w as f32 / 2.0 + 10.0;
    let reach_z = l.h as f32 / 2.0 + 10.0;
    let mut x = -reach_x;
    while x <= reach_x {
        let mut z = -reach_z;
        while z <= reach_z {
            let jitter = Vec3::new(rng.below(100) as f32 / 100.0 - 0.5, 0.0, rng.below(100) as f32 / 100.0 - 0.5);
            let pos = Vec3::new(x, GROUND_Y, z) + jitter;
            // Position sur la grille de jeu, pour savoir à quelle distance du plateau on se trouve.
            let on_grid = Vec2::new(pos.x + (l.w - 1) as f32 / 2.0, pos.z + (l.h - 1) as f32 / 2.0);
            let free = l.distance_to_walkable(on_grid) > 3.0;
            if free && rng.below(10) < 6 {
                spawn_decor(universe, &mut b, &mut rng, pos);
            }
            z += 3.0;
        }
        x += 3.0;
    }
}

/// Le décor autour du plateau : modèles Kenney (CC0), (fichier, échelle).
fn decor_models(u: Universe) -> &'static [(&'static str, f32)] {
    match u {
        Universe::Prairie => &[
            ("mini-forest/tree", 2.2),
            ("mini-forest/tree-high", 2.2),
            ("mini-forest/tree", 2.8),
            ("mini-forest/tent", 2.0),
            ("nature-kit/rock_largeA", 2.2),
            ("nature-kit/plant_bush", 2.4),
            ("nature-kit/stump_round", 2.4),
            ("nature-kit/flower_redA", 2.5),
            ("nature-kit/flower_yellowB", 2.5),
            ("nature-kit/log_large", 2.2),
        ],
        Universe::Desert => &[
            ("nature-kit/cactus_tall", 2.8),
            ("nature-kit/cactus_short", 2.8),
            ("nature-kit/cactus_tall", 3.4),
            ("pirate-kit/rocks-sand-a", 1.4),
            ("pirate-kit/rocks-sand-b", 1.4),
            ("pirate-kit/rocks-sand-c", 1.4),
            ("survival-kit/tent-canvas", 1.6),
            ("survival-kit/campfire-pit", 1.8),
            ("survival-kit/signpost", 2.4),
        ],
        Universe::Cimetiere => &[
            ("graveyard-kit/pine-fall", 2.4),
            ("graveyard-kit/pine", 2.4),
            ("graveyard-kit/pine-crooked", 2.4),
            ("graveyard-kit/gravestone-cross", 2.4),
            ("graveyard-kit/gravestone-round", 2.4),
            ("graveyard-kit/gravestone-broken", 2.4),
            ("graveyard-kit/crypt-small", 2.2),
            ("graveyard-kit/pumpkin-carved", 2.4),
            ("graveyard-kit/lightpost-single", 2.4),
            ("graveyard-kit/iron-fence", 2.4),
        ],
        Universe::Ocean => &[
            ("pirate-kit/palm-straight", 1.9),
            ("pirate-kit/palm-bend", 1.9),
            ("pirate-kit/palm-detailed-straight", 1.9),
            ("pirate-kit/barrel", 1.6),
            ("pirate-kit/crate", 1.6),
            ("pirate-kit/chest", 1.6),
            ("pirate-kit/rocks-a", 1.3),
            ("pirate-kit/boat-row-small", 1.4),
            ("pirate-kit/ship-small", 1.0),
        ],
    }
}

/// Hauts ou larges : ils cacheraient le plateau s'ils étaient devant (côté caméra).
fn is_tall(file: &str) -> bool {
    ["tree", "pine", "palm", "cactus_tall", "ship", "boat", "crypt", "tent", "lightpost", "log_large", "fence", "rocks", "rock_large"].iter().any(|k| file.contains(k))
}

fn spawn_decor(u: Universe, b: &mut Builder, rng: &mut Rng, pos: Vec3) {
    let models = decor_models(u);
    // La caméra est du côté +x/+z : là, on ne pose que du bas.
    let front = pos.x * 0.4 + pos.z > 0.0;
    let Some(&(file, scale)) = (0..6).map(|_| &models[rng.below(models.len() as u32) as usize]).find(|(f, _)| !front || !is_tall(f)) else {
        return;
    };
    let yaw = rng.below(628) as f32 / 100.0;
    // Sur l'île des pirates, les objets flottent un peu au-dessus de l'eau.
    let pos = if u == Universe::Ocean { pos + Vec3::Y * 0.05 } else { pos };
    b.model(file, pos, scale, yaw);
}

/// Aide à la construction de blocs, avec un cache de matériaux.
struct Builder<'a, 'w, 's> {
    commands: &'a mut Commands<'w, 's>,
    cube: Handle<Mesh>,
    mats: &'a mut Assets<StandardMaterial>,
    cache: Vec<(Color, Handle<StandardMaterial>)>,
    server: &'a AssetServer,
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

    fn model(&mut self, file: &str, at: Vec3, scale: f32, yaw: f32) {
        let scene = self.server.load(GltfAssetLabel::Scene(0).from_asset(format!("kenney/{file}.glb")));
        self.commands.spawn((
            WorldAssetRoot(scene),
            Transform::from_translation(at).with_rotation(Quat::from_rotation_y(yaw)).with_scale(Vec3::splat(scale)),
            WorldPart,
        ));
    }

    fn flat(&mut self, e: Entity) {
        self.commands.entity(e).insert(NotShadowCaster);
    }

}

