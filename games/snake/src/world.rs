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
                tiles: (c(0.50, 0.74, 0.38), c(0.45, 0.68, 0.34)),
                rim: c(0.36, 0.30, 0.22),
                ground: c(0.74, 0.72, 0.68),
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

/// Un véhicule qui fait le tour du parc sur l'avenue : rectangle `(x0, y0, x1, y1)` en cases de la grille, sans logique de jeu.
#[derive(Component)]
pub struct Traffic {
    rect: (f32, f32, f32, f32),
    clockwise: bool,
    speed: f32,
    phase: f32,
}

pub fn animate_traffic(time: Res<Time>, layout: Res<Layout>, mut q: Query<(&Traffic, &mut Transform)>) {
    let t = time.elapsed_secs();
    for (v, mut tf) in &mut q {
        let (x0, y0, x1, y1) = v.rect;
        let (w, h) = (x1 - x0, y1 - y0);
        let per = 2.0 * (w + h);
        let s = (v.phase + t * v.speed).rem_euclid(per);
        // Sens horaire sur la grille ; l'autre voie fait le tour à l'envers.
        let s = if v.clockwise { s } else { per - s };
        let (p, d) = if s < w {
            (Vec2::new(x0 + s, y0), Vec2::X)
        } else if s < w + h {
            (Vec2::new(x1, y0 + s - w), Vec2::Y)
        } else if s < 2.0 * w + h {
            (Vec2::new(x1 - (s - w - h), y1), Vec2::NEG_X)
        } else {
            (Vec2::new(x0, y1 - (s - 2.0 * w - h)), Vec2::NEG_Y)
        };
        let d = if v.clockwise { d } else { -d };
        tf.translation = cell_to_world(p, 0.05, &layout);
        tf.rotation = Quat::from_rotation_y(d.x.atan2(d.y));
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

    // Centre-ville : un grand parc entouré d'une avenue (voitures, tramway) et d'un horizon de tours.
    if universe == Universe::Ville {
        let mut rng = Rng::new(universe as u64 * 7919 + shape as u64 * 31);
        let (w, h) = (l.w as f32, l.h as f32);
        let (ex0, ey0, ex1, ey1) = (-0.5, -0.5, w - 0.5, h - 0.5);
        // Avenue : des tuiles de route Kenney (droites et virages) en boucle autour du parc ; l'axe est à `centre` du bord.
        let centre = 3.5;
        let (rx0, ry0, rx1, ry1) = (ex0 - centre, ey0 - centre, ex1 + centre, ey1 + centre);
        let road = 3.0;
        let mut tile = |b: &mut Builder, file: &str, p: Vec2, size: Vec3, yaw: f32| {
            let scene = b.server.load(GltfAssetLabel::Scene(0).from_asset(format!("kenney/{file}.glb")));
            b.commands.spawn((WorldAssetRoot(scene), Transform::from_translation(cell_to_world(p, GROUND_Y + 0.2, l)).with_rotation(Quat::from_rotation_y(yaw)).with_scale(size), NotShadowCaster, WorldPart));
        };
        // Quatre virages aux coins (chacun relie deux côtés) ; l'orientation de la tuile de base va vers +x et +z.
        for (x, y, yaw) in [(rx0, ry0, 0.0), (rx1, ry0, FRAC_PI_2), (rx1, ry1, 2.0 * FRAC_PI_2), (rx0, ry1, 3.0 * FRAC_PI_2)] {
            tile(&mut b, "city-kit-roads/road-bend", Vec2::new(x, y), Vec3::splat(road), yaw);
        }
        // Les côtés : des tuiles droites étirées pour remplir exactement l'espace entre deux virages.
        let nx = (((rx1 - rx0) - road) / road).ceil().max(1.0);
        let ny = (((ry1 - ry0) - road) / road).ceil().max(1.0);
        let (lx, ly) = (((rx1 - rx0) - road) / nx, ((ry1 - ry0) - road) / ny);
        for i in 0..nx as i32 {
            let x = rx0 + road / 2.0 + lx * (i as f32 + 0.5);
            for y in [ry0, ry1] {
                tile(&mut b, "city-kit-roads/road-straight", Vec2::new(x, y), Vec3::new(lx, road, road), 0.0);
            }
        }
        for i in 0..ny as i32 {
            let y = ry0 + road / 2.0 + ly * (i as f32 + 0.5);
            for x in [rx0, rx1] {
                tile(&mut b, "city-kit-roads/road-straight", Vec2::new(x, y), Vec3::new(ly, road, road), FRAC_PI_2);
            }
        }
        let _ = &mut tile;
        // Circulation : deux voies de voitures et un tramway, qui font le tour du parc.
        let cars = ["car-kit/sedan", "car-kit/taxi", "car-kit/suv", "car-kit/van", "car-kit/hatchback-sports", "car-kit/police", "car-kit/delivery", "car-kit/truck", "car-kit/sedan-sports", "car-kit/suv-luxury"];
        let trams = ["train-kit/train-electric-city-a", "train-kit/train-electric-subway-a", "train-kit/train-electric-subway-a", "train-kit/train-electric-city-a"];
        let lane = |d: f32| (ex0 - d, ey0 - d, ex1 + d, ey1 + d);
        let per = |r: (f32, f32, f32, f32)| 2.0 * ((r.2 - r.0) + (r.3 - r.1));
        let mut vehicle = |b: &mut Builder, file: &str, scale: Vec3, v: Traffic| {
            let scene = b.server.load(GltfAssetLabel::Scene(0).from_asset(format!("kenney/{file}.glb")));
            b.commands.spawn((WorldAssetRoot(scene), Transform::from_scale(scale), v, WorldPart));
        };
        let (a, c) = (lane(centre - 0.7), lane(centre + 0.7));
        for (i, t) in trams.iter().enumerate() {
            vehicle(&mut b, t, Vec3::new(0.6, 0.5, 0.52), Traffic { rect: a, clockwise: true, speed: 2.2, phase: 12.0 - i as f32 * 1.35 });
        }
        for i in 0..6 {
            let file = cars[rng.below(cars.len() as u32) as usize];
            vehicle(&mut b, file, Vec3::splat(0.5), Traffic { rect: a, clockwise: true, speed: 2.2, phase: 18.0 + i as f32 * per(a) / 6.0 });
        }
        for i in 0..7 {
            let file = cars[rng.below(cars.len() as u32) as usize];
            vehicle(&mut b, file, Vec3::splat(0.5), Traffic { rect: c, clockwise: false, speed: 3.0, phase: i as f32 * per(c) / 7.0 + rng.below(30) as f32 / 10.0 });
        }
        let _ = &mut vehicle;
        // Bordure du parc : arbres (côtés et fond) et mobilier bas (côté caméra).
        let tall = ["city-kit-suburban/tree-large", "city-kit-suburban/tree-small", "city-kit-suburban/tree-large"];
        let lowp = ["coaster-kit/bench", "city-kit-suburban/planter", "coaster-kit/flowers", "nature-kit/plant_bush"];
        let edge = |b: &mut Builder, p: Vec2, rng: &mut Rng| {
            let at = cell_to_world(p, 0.0, l);
            let front = at.x * 0.4 + at.z > 0.0;
            let file = if front { lowp[rng.below(lowp.len() as u32) as usize] } else { tall[rng.below(tall.len() as u32) as usize] };
            b.model(file, at, if front { 1.8 } else { 2.2 }, rng.below(628) as f32 / 100.0);
        };
        let mut x = ex0 - 0.5;
        while x < ex1 + 1.0 {
            for y in [ey0 - 0.9, ey1 + 0.9] {
                edge(&mut b, Vec2::new(x, y), &mut rng);
            }
            x += 1.7;
        }
        let mut y = ey0 + 0.8;
        while y < ey1 {
            for x in [ex0 - 0.9, ex1 + 0.9] {
                edge(&mut b, Vec2::new(x, y), &mut rng);
            }
            y += 1.7;
        }
        // L'horizon : plusieurs rangées de tours autour de l'avenue (basses du côté caméra, pour garder la vue).
        let low = ["city-kit-commercial/building-a", "city-kit-commercial/building-d", "city-kit-commercial/building-g", "city-kit-commercial/building-k", "city-kit-commercial/building-c", "city-kit-commercial/building-h", "city-kit-commercial/building-b", "city-kit-commercial/building-e"];
        let towers = ["city-kit-commercial/building-skyscraper-a", "city-kit-commercial/building-skyscraper-b", "city-kit-commercial/building-skyscraper-c", "city-kit-commercial/building-skyscraper-d", "city-kit-commercial/building-skyscraper-e"];
        let first = centre + 3.2;
        for row in 0..3 {
            let d = first + row as f32 * 3.6;
            let (rx0, ry0, rx1, ry1) = (ex0 - d, ey0 - d, ex1 + d, ey1 + d);
            let put = |b: &mut Builder, p: Vec2, rng: &mut Rng| {
                let at = cell_to_world(p, 0.0, l);
                let front = at.x * 0.4 + at.z > 0.0;
                if front {
                    b.model(low[rng.below(low.len() as u32) as usize], at, 1.5 + rng.below(6) as f32 / 10.0, rng.below(4) as f32 * FRAC_PI_2);
                } else if rng.below(4) > 0 {
                    b.model(towers[rng.below(towers.len() as u32) as usize], at, 1.9 + rng.below(12) as f32 / 10.0, rng.below(4) as f32 * FRAC_PI_2);
                } else {
                    b.model(low[rng.below(low.len() as u32) as usize], at, 1.8, rng.below(4) as f32 * FRAC_PI_2);
                }
            };
            let mut x = rx0;
            while x <= rx1 {
                put(&mut b, Vec2::new(x, ry0), &mut rng);
                put(&mut b, Vec2::new(x, ry1), &mut rng);
                x += 3.4;
            }
            let mut y = ry0 + 3.4;
            while y < ry1 {
                put(&mut b, Vec2::new(rx0, y), &mut rng);
                put(&mut b, Vec2::new(rx1, y), &mut rng);
                y += 3.4;
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

