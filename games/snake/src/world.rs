//! Les univers : plateau, sol, décor et lumière.
use bevy::light::NotShadowCaster;
use std::f32::consts::FRAC_PI_2;

use bevy::prelude::*;
use game_common::{Rng, flat_material, tx};

use crate::layout::Layout;

/// Hauteur du dessus du sol extérieur.
const GROUND_Y: f32 = -0.2;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum Universe {
    #[default]
    Prairie,
    Desert,
    Ruines,
    Cimetiere,
    Ocean,
    Parc,
    Ville,
    Gare,
    Port,
}

impl Universe {
    pub const ALL: [Universe; 9] = [Universe::Prairie, Universe::Desert, Universe::Ruines, Universe::Cimetiere, Universe::Ocean, Universe::Parc, Universe::Ville, Universe::Gare, Universe::Port];

    pub fn next(self) -> Self {
        let i = Self::ALL.iter().position(|u| *u == self).unwrap_or(0);
        Self::ALL[(i + 1) % Self::ALL.len()]
    }

    pub fn label(self) -> &'static str {
        match self {
            Universe::Prairie => tx!("Prairie", "Meadow", "Wiese", "Prateria", "Pradera"),
            Universe::Desert => tx!("Désert", "Desert", "Wüste", "Deserto", "Desierto"),
            Universe::Ruines => tx!("Ruines antiques", "Ancient ruins", "Antike Ruinen", "Rovine antiche", "Ruinas antiguas"),
            Universe::Cimetiere => tx!("Cimetière", "Graveyard", "Friedhof", "Cimitero", "Cementerio"),
            Universe::Ocean => tx!("Île des pirates", "Pirate island", "Pirateninsel", "Isola dei pirati", "Isla pirata"),
            Universe::Parc => tx!("Parc de loisirs", "Theme park", "Freizeitpark", "Parco divertimenti", "Parque de atracciones"),
            Universe::Ville => tx!("Centre-ville", "Downtown", "Innenstadt", "Centro città", "Centro urbano"),
            Universe::Gare => tx!("Gare", "Train station", "Bahnhof", "Stazione", "Estación"),
            Universe::Port => tx!("Port", "Harbour", "Hafen", "Porto", "Puerto"),
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
            Universe::Ruines => Theme {
                sky: c(0.78, 0.85, 0.93),
                tiles: (c(0.92, 0.90, 0.84), c(0.86, 0.84, 0.77)),
                rim: c(0.62, 0.58, 0.50),
                ground: c(0.80, 0.76, 0.62),
                sun: (c(1.0, 0.96, 0.88), 7_000.0),
                ambient: (c(0.58, 0.58, 0.66), 1000.0),
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
            Universe::Parc => Theme {
                sky: c(0.80, 0.84, 0.95),
                tiles: (c(0.95, 0.82, 0.86), c(0.90, 0.74, 0.80)),
                rim: c(0.55, 0.38, 0.50),
                ground: c(0.55, 0.78, 0.50),
                sun: (Color::WHITE, 6_800.0),
                ambient: (c(0.56, 0.52, 0.66), 1000.0),
            },
            Universe::Ville => Theme {
                sky: c(0.70, 0.76, 0.86),
                tiles: (c(0.40, 0.41, 0.45), c(0.37, 0.38, 0.42)),
                rim: c(0.26, 0.27, 0.30),
                ground: c(0.34, 0.35, 0.38),
                sun: (Color::WHITE, 6_500.0),
                ambient: (c(0.52, 0.52, 0.62), 1000.0),
            },
            Universe::Gare => Theme {
                sky: c(0.76, 0.80, 0.88),
                tiles: (c(0.78, 0.76, 0.72), c(0.72, 0.70, 0.66)),
                rim: c(0.42, 0.36, 0.32),
                ground: c(0.55, 0.52, 0.46),
                sun: (c(1.0, 0.96, 0.9), 6_800.0),
                ambient: (c(0.54, 0.52, 0.62), 1000.0),
            },
            Universe::Port => Theme {
                sky: c(0.74, 0.84, 0.92),
                tiles: (c(0.72, 0.56, 0.36), c(0.66, 0.50, 0.32)),
                rim: c(0.38, 0.28, 0.20),
                ground: c(0.18, 0.42, 0.58),
                sun: (Color::WHITE, 6_800.0),
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

/// Un véhicule qui roule en boucle sur une rue au-delà du plateau (cases de la grille, pas de logique de jeu).
#[derive(Component)]
pub struct Traffic {
    start: Vec2,
    dir: Vec2,
    len: f32,
    speed: f32,
    phase: f32,
}

pub fn animate_traffic(time: Res<Time>, layout: Res<Layout>, mut q: Query<(&Traffic, &mut Transform)>) {
    let t = time.elapsed_secs();
    for (v, mut tf) in &mut q {
        let s = (v.phase + t * v.speed).rem_euclid(v.len);
        tf.translation = cell_to_world(v.start + v.dir * s, 0.05, &layout);
    }
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
    layout: Res<Layout>,
    scenery: Res<crate::voyage::Scenery>,
    mut built: Local<Option<(Universe, u32)>>,
    old: Query<Entity, With<WorldPart>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut clear: ResMut<ClearColor>,
    mut ambient: ResMut<GlobalAmbientLight>,
    mut suns: Query<&mut DirectionalLight>,
    server: Res<AssetServer>,
) {
    let key = (scenery.0, layout.id);
    if *built == Some(key) {
        return;
    }
    *built = Some(key);
    for e in &old {
        commands.entity(e).despawn();
    }

    let (universe, shape) = (key.0, key.1);
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

    // Ville : des pâtés de maisons (immeubles bas ou petits parcs côté caméra) entre les rues, qui se prolongent au-delà du plateau.
    if l.blocks > 0 {
        let mut rng = Rng::new(universe as u64 * 7919 + shape as u64 * 31);
        let pitch = crate::layout::CITY_BLOCK + crate::layout::CITY_STREET;
        let street = crate::layout::CITY_STREET;
        let pavement = b.mat(Color::srgb(0.80, 0.78, 0.74));
        let lawn = b.mat(Color::srgb(0.50, 0.72, 0.38));
        let line = b.mat(Color::srgb(0.95, 0.93, 0.85));
        let low = ["city-kit-commercial/building-a", "city-kit-commercial/building-d", "city-kit-commercial/building-g", "city-kit-commercial/building-k", "city-kit-commercial/building-c", "city-kit-commercial/building-h", "city-kit-commercial/building-b"];
        let towers = ["city-kit-commercial/building-skyscraper-b", "city-kit-commercial/building-skyscraper-d", "city-kit-commercial/building-skyscraper-a", "city-kit-commercial/building-skyscraper-c"];
        let parks = ["city-kit-suburban/tree-small", "city-kit-suburban/tree-large", "city-kit-suburban/planter", "coaster-kit/bench", "coaster-kit/flowers"];
        // Traits de rue : un pointillé au milieu de chaque rue du plateau.
        for k in 0..=l.blocks {
            let c = (k * pitch) as f32 + 0.5;
            for t in (0..l.w).filter(|t| t % 2 == 0) {
                for (x, y, yaw) in [(t as f32, c, 0.0), (c, t as f32, FRAC_PI_2)] {
                    if l.walkable(IVec2::new(x as i32, y as i32)) {
                        let e = b.block_rot(Color::srgb(0.95, 0.93, 0.85), cell_to_world(Vec2::new(x, y), 0.05, l), Vec3::new(0.5, 0.01, 0.07), yaw);
                        b.flat(e);
                    }
                }
            }
        }
        let _ = line;
        // Circulation : des voitures sur les rues autour du plateau, et un tramway sur la rue du fond.
        let cars = ["car-kit/sedan", "car-kit/taxi", "car-kit/suv", "car-kit/van", "car-kit/hatchback-sports", "car-kit/police", "car-kit/delivery", "car-kit/truck", "car-kit/sedan-sports", "car-kit/suv-luxury"];
        let trams = ["train-kit/train-electric-city-a", "train-kit/train-electric-subway-a", "train-kit/train-electric-subway-a", "train-kit/train-electric-city-a"];
        let (span, from) = (l.w as f32 + 40.0, -20.0);
        let vehicle = |b: &mut Builder, file: &str, scale: Vec3, start: Vec2, dir: Vec2, speed: f32, phase: f32| {
            let scene = b.server.load(GltfAssetLabel::Scene(0).from_asset(format!("kenney/{file}.glb")));
            let yaw = dir.x.atan2(dir.y);
            b.commands.spawn((WorldAssetRoot(scene), Transform::from_rotation(Quat::from_rotation_y(yaw)).with_scale(scale), Traffic { start, dir, len: span, speed, phase }, WorldPart));
        };
        for k in [-1, l.blocks + 1] {
            for lane in 0..2 {
                let row = (k * pitch + lane) as f32;
                let sign = if lane == 0 { 1.0 } else { -1.0 };
                let speed = 2.5 + rng.below(20) as f32 / 10.0;
                for horizontal in [true, false] {
                    // Le tramway roule sur la rue du fond, dans un seul sens.
                    if horizontal && k == -1 && lane == 0 {
                        for (i, t) in trams.iter().enumerate() {
                            vehicle(&mut b, t, Vec3::new(0.6, 0.5, 0.52), Vec2::new(from, row), Vec2::X, 2.2, 18.0 - i as f32 * 1.35);
                        }
                        continue;
                    }
                    let (start, dir) = if horizontal { (Vec2::new(if sign > 0.0 { from } else { from + span }, row), Vec2::new(sign, 0.0)) } else { (Vec2::new(row, if sign > 0.0 { from } else { from + span }), Vec2::new(0.0, sign)) };
                    for i in 0..3 {
                        let file = cars[rng.below(cars.len() as u32) as usize];
                        vehicle(&mut b, file, Vec3::splat(0.5), start, dir, speed, i as f32 * span / 3.0 + rng.below(100) as f32 / 10.0);
                    }
                }
            }
        }
        let ring = 2;
        for by in -ring..l.blocks + ring {
            for bx in -ring..l.blocks + ring {
                let (x0, y0) = (bx * pitch + street, by * pitch + street);
                let inside = (0..l.blocks).contains(&bx) && (0..l.blocks).contains(&by);
                let centre = cell_to_world(Vec2::new(x0 as f32 + 1.5, y0 as f32 + 1.5), 0.0, l);
                // Du côté de la caméra, rien de haut : on garde la vue sur la rue.
                let front = centre.x * 0.4 + centre.z > 0.0;
                let park = front && (inside || rng.below(5) == 0);
                let (mat, base) = (if park { lawn.clone() } else { pavement.clone() }, if inside { -0.05 } else { GROUND_Y });
                let h = if inside { 0.1 } else { 0.25 };
                let e = b.block_with(mat, Vec3::new(centre.x, base, centre.z), Vec3::new(3.9, h, 3.9), 0.0);
                b.flat(e);
                for (dx, dy) in [(0.0, 0.0), (2.0, 0.0), (0.0, 2.0), (2.0, 2.0)] {
                    let at = cell_to_world(Vec2::new(x0 as f32 + 0.5 + dx, y0 as f32 + 0.5 + dy), 0.05, l);
                    let yaw = rng.below(4) as f32 * FRAC_PI_2;
                    if park {
                        if rng.below(3) > 0 {
                            b.model(parks[rng.below(parks.len() as u32) as usize], at, 1.8, yaw);
                        }
                    } else if !front && (dx, dy) == (0.0, 0.0) && !inside && rng.below(3) > 0 {
                        // Quartier d'affaires : une tour par pâté, qui remplit tout le pâté.
                        let centre = cell_to_world(Vec2::new(x0 as f32 + 1.5, y0 as f32 + 1.5), 0.05, l);
                        b.model(towers[rng.below(towers.len() as u32) as usize], centre, 2.3, yaw);
                        break;
                    } else if !front && inside && rng.below(3) == 0 {
                        b.model(towers[rng.below(towers.len() as u32) as usize], at, 1.2, yaw);
                    } else {
                        b.model(low[rng.below(low.len() as u32) as usize], at, if front { 1.0 } else { 1.25 }, yaw);
                    }
                }
                // Un lampadaire au coin du pâté.
                if rng.below(2) == 0 {
                    b.model("city-kit-roads/light-square", cell_to_world(Vec2::new(x0 as f32 - 0.5, y0 as f32 - 0.5), 0.05, l), 1.8, 0.0);
                }
            }
        }
        return;
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
        Universe::Ruines => &[
            ("prototype-kit/column", 2.6),
            ("prototype-kit/column-rounded", 2.6),
            ("prototype-kit/column-low", 2.6),
            ("mini-arena/column", 2.6),
            ("mini-arena/column-damaged", 2.6),
            ("mini-arena/statue", 2.6),
            ("mini-arena/trophy", 2.4),
            ("fantasy-town-kit/pillar-stone", 2.6),
            ("fantasy-town-kit/wall-arch", 2.2),
            ("fantasy-town-kit/fountain-round", 2.2),
            ("nature-kit/rock_largeA", 2.2),
            ("nature-kit/cactus_short", 2.4),
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
        Universe::Parc => &[
            ("coaster-kit/tree", 2.2),
            ("coaster-kit/tree-large", 2.0),
            ("coaster-kit/flowers", 2.4),
            ("coaster-kit/bench", 2.2),
            ("coaster-kit/trash", 2.2),
            ("coaster-kit/stall-food", 2.0),
            ("coaster-kit/stall-drinks", 2.0),
            ("coaster-kit/stall-information", 2.0),
            ("coaster-kit/support-large", 1.6),
            ("coaster-kit/park-entrance", 1.2),
            ("fantasy-town-kit/lantern", 2.4),
            ("fantasy-town-kit/fountain-round", 2.2),
        ],
        Universe::Ville => &[
            ("city-kit-commercial/building-a", 1.3),
            ("city-kit-commercial/building-d", 1.3),
            ("city-kit-commercial/building-g", 1.3),
            ("city-kit-commercial/building-k", 1.3),
            ("city-kit-commercial/building-skyscraper-b", 1.2),
            ("city-kit-commercial/building-skyscraper-d", 1.2),
            ("city-kit-roads/light-square", 2.0),
            ("city-kit-roads/traffic-light", 2.0),
            ("city-kit-roads/construction-cone", 2.0),
            ("city-kit-suburban/tree-small", 1.8),
            ("city-kit-suburban/planter", 2.0),
        ],
        Universe::Gare => &[
            ("city-kit-industrial/building-a", 1.3),
            ("city-kit-industrial/building-c", 1.3),
            ("city-kit-industrial/building-e", 1.3),
            ("city-kit-industrial/shipping-container-a", 1.2),
            ("city-kit-industrial/shipping-container-b", 1.2),
            ("city-kit-industrial/shipping-container-c", 1.2),
            ("city-kit-industrial/water-tower", 1.2),
            ("city-kit-industrial/chimney-medium", 1.2),
            ("city-kit-industrial/detail-tank", 1.4),
            ("city-kit-roads/light-square", 2.0),
            ("city-kit-roads/construction-cone", 2.0),
            ("city-kit-suburban/tree-small", 1.8),
        ],
        Universe::Port => &[
            ("city-kit-industrial/shipping-container-a", 1.2),
            ("city-kit-industrial/shipping-container-b", 1.2),
            ("city-kit-industrial/shipping-container-c", 1.2),
            ("city-kit-industrial/detail-tank-large", 1.2),
            ("pirate-kit/boat-row-small", 1.4),
            ("pirate-kit/ship-small", 1.0),
            ("pirate-kit/barrel", 1.6),
            ("pirate-kit/crate", 1.6),
            ("city-kit-roads/light-square", 2.0),
        ],
    }
}

/// Hauts ou larges : ils cacheraient le plateau s'ils étaient devant (côté caméra).
fn is_tall(file: &str) -> bool {
    ["tree", "pine", "palm", "cactus_tall", "prototype-kit/column", "mini-arena/column", "wall-arch", "pillar-stone", "statue", "ship", "boat", "crypt", "tent", "lightpost", "log_large", "fence", "rocks", "rock_large", "building", "water-tower", "chimney", "traffic-light", "stall", "support", "park-entrance"].iter().any(|k| file.contains(k))
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
    let pos = if matches!(u, Universe::Ocean | Universe::Port) { pos + Vec3::Y * 0.05 } else { pos };
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

