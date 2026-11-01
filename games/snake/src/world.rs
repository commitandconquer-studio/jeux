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
    Medieval,
    Cave,
}

impl Universe {
    pub const ALL: [Universe; 11] = [Universe::Prairie, Universe::Desert, Universe::Ruines, Universe::Cimetiere, Universe::Ocean, Universe::Parc, Universe::Ville, Universe::Gare, Universe::Port, Universe::Medieval, Universe::Cave];

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
            Universe::Medieval => tx!("Village médiéval", "Medieval village", "Mittelalterdorf", "Villaggio medievale", "Aldea medieval"),
            Universe::Cave => tx!("Grottes", "Caves", "Höhlen", "Grotte", "Cuevas"),
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
            Universe::Medieval => Theme {
                sky: c(0.80, 0.84, 0.90),
                tiles: (c(0.74, 0.70, 0.62), c(0.68, 0.64, 0.56)),
                rim: c(0.40, 0.34, 0.28),
                ground: c(0.46, 0.64, 0.34),
                sun: (c(1.0, 0.95, 0.85), 6_800.0),
                ambient: (c(0.56, 0.54, 0.62), 1000.0),
            },
            Universe::Cave => Theme {
                sky: c(0.16, 0.14, 0.22),
                tiles: (c(0.46, 0.42, 0.44), c(0.40, 0.36, 0.40)),
                rim: c(0.22, 0.20, 0.24),
                ground: c(0.22, 0.20, 0.25),
                sun: (c(0.80, 0.86, 1.0), 3_800.0),
                ambient: (c(0.52, 0.46, 0.70), 1200.0),
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

/// Un véhicule qui roule en boucle sur une rue (hors du plateau) : départ, direction et longueur en cases de la grille, sans logique de jeu.
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
        tf.rotation = Quat::from_rotation_y(v.dir.x.atan2(v.dir.y));
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
        // Quatre rues larges qui se croisent aux quatre coins du parc et partent dans tous les sens (carrefours Kenney).
        let road = 5.0;
        let axis = 2.0 + road / 2.0; // axe de la rue : un trottoir de 2 cases entre le plateau et la chaussée
        let reach = 34.0;
        let (vx_a, vx_b, hy_a, hy_b) = (ex0 - axis, ex1 + axis, ey0 - axis, ey1 + axis);
        let (xmin, xmax, ymin, ymax) = (vx_a - reach, vx_b + reach, hy_a - reach, hy_b + reach);
        let tile = |b: &mut Builder, file: &str, p: Vec2, size: Vec3, yaw: f32| {
            let scene = b.server.load(GltfAssetLabel::Scene(0).from_asset(format!("kenney/{file}.glb")));
            b.commands.spawn((WorldAssetRoot(scene), Transform::from_translation(cell_to_world(p, GROUND_Y + 0.2, l)).with_rotation(Quat::from_rotation_y(yaw)).with_scale(size), NotShadowCaster, WorldPart));
        };
        // Un tronçon de rue droite : des tuiles étirées pour remplir exactement l'espace entre deux carrefours.
        let strip = |b: &mut Builder, from: f32, to: f32, fixed: f32, horizontal: bool| {
            let n = ((to - from) / road).ceil().max(1.0);
            let len = (to - from) / n;
            for k in 0..n as i32 {
                let c = from + len * (k as f32 + 0.5);
                let (p, yaw) = if horizontal { (Vec2::new(c, fixed), 0.0) } else { (Vec2::new(fixed, c), FRAC_PI_2) };
                tile(b, "city-kit-roads/road-straight", p, Vec3::new(len, road, road), yaw);
            }
        };
        for y in [hy_a, hy_b] {
            strip(&mut b, xmin, vx_a - road / 2.0, y, true);
            strip(&mut b, vx_a + road / 2.0, vx_b - road / 2.0, y, true);
            strip(&mut b, vx_b + road / 2.0, xmax, y, true);
        }
        for x in [vx_a, vx_b] {
            strip(&mut b, ymin, hy_a - road / 2.0, x, false);
            strip(&mut b, hy_a + road / 2.0, hy_b - road / 2.0, x, false);
            strip(&mut b, hy_b + road / 2.0, ymax, x, false);
        }
        let cross = |b: &mut Builder, p: Vec2| {
            let scene = b.server.load(GltfAssetLabel::Scene(0).from_asset("kenney/city-kit-roads/road-crossroad.glb".to_owned()));
            b.commands.spawn((WorldAssetRoot(scene), Transform::from_translation(cell_to_world(p, GROUND_Y + 0.2, l)).with_scale(Vec3::splat(road)), NotShadowCaster, WorldPart));
        };
        for (x, y) in [(vx_a, hy_a), (vx_b, hy_a), (vx_a, hy_b), (vx_b, hy_b)] {
            cross(&mut b, Vec2::new(x, y));
        }
        // Circulation : deux voies par rue (un sens chacune), des voitures à l'échelle des personnages, et un tramway dans la rue du fond.
        let cars = ["car-kit/sedan", "car-kit/taxi", "car-kit/suv", "car-kit/van", "car-kit/hatchback-sports", "car-kit/police", "car-kit/delivery", "car-kit/truck", "car-kit/sedan-sports", "car-kit/suv-luxury"];
        let trams = ["train-kit/train-electric-city-a", "train-kit/train-electric-subway-a", "train-kit/train-electric-subway-a", "train-kit/train-electric-city-a"];
        let mut vehicle = |b: &mut Builder, file: &str, scale: Vec3, v: Traffic| {
            let scene = b.server.load(GltfAssetLabel::Scene(0).from_asset(format!("kenney/{file}.glb")));
            b.commands.spawn((WorldAssetRoot(scene), Transform::from_scale(scale), v, WorldPart));
        };
        let lane_off = 1.3;
        let mut first_lane = true;
        for (fixed, horizontal) in [(hy_a, true), (hy_b, true), (vx_a, false), (vx_b, false)] {
            let (from, len) = if horizontal { (xmin, xmax - xmin) } else { (ymin, ymax - ymin) };
            for (sign, dirsign) in [(-1.0_f32, 1.0_f32), (1.0, -1.0)] {
                let (start, dir) = match (horizontal, dirsign > 0.0) {
                    (true, true) => (Vec2::new(from, fixed + sign * lane_off), Vec2::X),
                    (true, false) => (Vec2::new(from + len, fixed + sign * lane_off), Vec2::NEG_X),
                    (false, true) => (Vec2::new(fixed + sign * lane_off, from), Vec2::Y),
                    (false, false) => (Vec2::new(fixed + sign * lane_off, from + len), Vec2::NEG_Y),
                };
                if first_lane {
                    // Le tramway roule dans la rue du fond.
                    first_lane = false;
                    for (i, t) in trams.iter().enumerate() {
                        vehicle(&mut b, t, Vec3::new(1.0, 0.9, 0.95), Traffic { start, dir, len, speed: 2.4, phase: 40.0 - i as f32 * 2.6 });
                    }
                    continue;
                }
                let speed = 2.6 + rng.below(18) as f32 / 10.0;
                for i in 0..3 {
                    let file = cars[rng.below(cars.len() as u32) as usize];
                    vehicle(&mut b, file, Vec3::splat(0.95), Traffic { start, dir, len, speed, phase: i as f32 * len / 3.0 + rng.below(100) as f32 / 10.0 });
                }
            }
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
        let first = axis + road / 2.0 + 2.0;
        for row in 0..3 {
            let d = first + row as f32 * 3.6;
            let (rx0, ry0, rx1, ry1) = (ex0 - d, ey0 - d, ex1 + d, ey1 + d);
            let put = |b: &mut Builder, p: Vec2, rng: &mut Rng| {
                // Pas d'immeuble au milieu d'une rue qui part vers l'extérieur.
                if (p.x - vx_a).abs() < 3.6 || (p.x - vx_b).abs() < 3.6 || (p.y - hy_a).abs() < 3.6 || (p.y - hy_b).abs() < 3.6 {
                    return;
                }
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

    // Village médiéval : une cour pavée, des remparts et des tours aux côtés du fond (le côté caméra reste ouvert), des étals devant.
    if universe == Universe::Medieval {
        let (w, h) = (l.w as f32, l.h as f32);
        let (ex0, ey0, ex1, ey1) = (-0.5, -0.5, w - 0.5, h - 0.5);
        let (d, piece) = (1.4, 1.8);
        let cobble = b.mat(Color::srgb(0.66, 0.62, 0.55));
        let at = cell_to_world(Vec2::new((ex0 + ex1) / 2.0, (ey0 + ey1) / 2.0), GROUND_Y, l);
        let e = b.block_with(cobble, at, Vec3::new(w + 2.0 * d, 0.2, h + 2.0 * d), 0.0);
        b.flat(e);
        // Rempart : des pans de mur le long des deux côtés du fond.
        let mut x = ex0 - d;
        while x <= ex1 + d {
            b.model("castle-kit/wall", cell_to_world(Vec2::new(x, ey0 - d), GROUND_Y + 0.2, l), piece, 0.0);
            x += piece;
        }
        let mut y = ey0 - d;
        while y <= ey1 + d {
            b.model("castle-kit/wall", cell_to_world(Vec2::new(ex0 - d, y), GROUND_Y + 0.2, l), piece, FRAC_PI_2);
            y += piece;
        }
        // Tours : hautes au fond et sur les côtés, petites devant (elles cacheraient le plateau).
        let tower = |b: &mut Builder, p: Vec2, floors: i32| {
            let base = cell_to_world(p, GROUND_Y + 0.2, l);
            let s = 2.2;
            b.model("castle-kit/tower-square-base", base, s, 0.0);
            for f in 0..floors {
                b.model("castle-kit/tower-square-mid-windows", base + Vec3::Y * (1.01 * s * (f + 1) as f32), s, 0.0);
            }
            b.model("castle-kit/tower-square-roof", base + Vec3::Y * (1.01 * s * (floors + 1) as f32), s, 0.0);
        };
        tower(&mut b, Vec2::new(ex0 - d, ey0 - d), 3);
        tower(&mut b, Vec2::new(ex1 + d, ey0 - d), 1);
        tower(&mut b, Vec2::new(ex0 - d, ey1 + d), 1);
        tower(&mut b, Vec2::new(ex1 + d, ey1 + d), 0);
        // Devant : une clôture basse et des fanions.
        let mut x = ex0;
        while x < ex1 {
            b.model("fantasy-town-kit/fence", cell_to_world(Vec2::new(x, ey1 + d), GROUND_Y + 0.2, l), 1.8, 0.0);
            x += 1.8;
        }
        let mut y = ey0;
        while y < ey1 {
            b.model("fantasy-town-kit/fence", cell_to_world(Vec2::new(ex1 + d, y), GROUND_Y + 0.2, l), 1.8, FRAC_PI_2);
            y += 1.8;
        }
    }

    // Grotte : des parois de roche tout autour du fond et des côtés (le côté caméra reste ouvert).
    if universe == Universe::Cave {
        let (w, h) = (l.w as f32, l.h as f32);
        let (ex0, ey0, ex1, ey1) = (-0.5, -0.5, w - 0.5, h - 0.5);
        let (d, piece) = (1.6, 2.8);
        let mut x = ex0 - d;
        while x <= ex1 + d + piece {
            b.model("modular-cave-kit/template-wall", cell_to_world(Vec2::new(x + piece / 2.0, ey0 - d), GROUND_Y, l), 0.7, 0.0);
            x += piece;
        }
        let mut y = ey0 - d;
        while y <= ey1 + d + piece {
            b.model("modular-cave-kit/template-wall", cell_to_world(Vec2::new(ex0 - d, y + piece / 2.0), GROUND_Y, l), 0.7, FRAC_PI_2);
            y += piece;
        }
    }

    // Prairie : une clairière au milieu de la forêt. Autour du plateau, une pelouse dégagée avec quelques touffes de fleurs
    // bien placées et un petit campement au fond ; puis la forêt, dense, qui remplit tout le bord de l'écran.
    if universe == Universe::Prairie {
        let mut rng = Rng::new(universe as u64 * 7919 + shape as u64 * 31);
        let (cx, cz) = ((l.w - 1) as f32 / 2.0, (l.h - 1) as f32 / 2.0);
        let reach = (l.w.max(l.h) as f32) / 2.0 + 36.0;
        let dist = |p: Vec3| l.distance_to_walkable(Vec2::new(p.x + cx, p.z + cz));
        let front = |p: Vec3| p.x * 0.4 + p.z > 0.0;
        let facing = |p: Vec3, rng: &mut Rng| (-p.x).atan2(-p.z) + (rng.below(100) as f32 / 100.0 - 0.5) * 0.5;
        // La forêt : un semis serré en quinconce, arbres un peu plus grands à mesure qu'on s'éloigne.
        // Côté caméra elle commence plus loin pour ne pas cacher le plateau.
        let (step, mut row) = (2.6_f32, 0);
        let mut z = -reach;
        while z <= reach {
            let mut x = -reach + if row % 2 == 0 { 0.0 } else { step / 2.0 };
            while x <= reach {
                let p = Vec3::new(x + (rng.below(100) as f32 / 100.0 - 0.5) * 1.2, GROUND_Y, z + (rng.below(100) as f32 / 100.0 - 0.5) * 1.2);
                let d = dist(p);
                let edge = if front(p) { 11.0 } else { 6.5 };
                if d >= edge {
                    let t = ((d - edge) / 22.0).min(1.0);
                    let file = if rng.below(3) == 0 { "mini-forest/tree-high" } else { "mini-forest/tree" };
                    b.model(file, p, 2.2 + t * 1.6 + rng.below(40) as f32 / 100.0, rng.below(628) as f32 / 100.0);
                } else if d >= edge - 2.2 && rng.below(3) == 0 {
                    // La lisière : souches, champignons et buissons au pied des premiers arbres.
                    let file = ["nature-kit/stump_round", "nature-kit/mushroom_redGroup", "nature-kit/plant_bush", "nature-kit/mushroom_tanGroup"][rng.below(4) as usize];
                    b.model(file, p, 2.0, facing(p, &mut rng));
                }
                x += step;
            }
            z += step;
            row += 1;
        }
        // Les fleurs : des touffes de trois, en anneau régulier autour du plateau.
        let flowers = ["nature-kit/flower_redA", "nature-kit/flower_yellowB", "nature-kit/flower_purpleA"];
        let n = ((l.w + l.h) as f32 / 3.0).round().max(6.0) as i32;
        for i in 0..n {
            let a = i as f32 / n as f32 * std::f32::consts::TAU + 0.3;
            let (sx, sz) = (cx + 3.5, cz + 3.5);
            let centre = Vec3::new(a.cos() * sx, GROUND_Y, a.sin() * sz);
            if dist(centre) < 1.8 {
                continue;
            }
            let file = flowers[i as usize % flowers.len()];
            b.model(file, centre, 3.0, rng.below(628) as f32 / 100.0);
            for k in 0..4 {
                let off = (k as f32) * std::f32::consts::FRAC_PI_2 + 0.4;
                let p = centre + Vec3::new(off.cos() * 1.1, 0.0, off.sin() * 1.1);
                b.model(file, p, 3.0, rng.below(628) as f32 / 100.0);
            }
        }
        // Le campement : au fond, deux tentes face au plateau, un feu entre elles, un rondin et une terre battue.
        let back = Vec3::new(-0.37, 0.0, -0.93);
        let side = Vec3::new(0.93, 0.0, -0.37);
        let half = (cx * back.x.abs() + cz * back.z.abs()) + 5.5;
        let camp = back * half;
        b.model("mini-forest/patch-dirt", camp + Vec3::Y * GROUND_Y, 2.8, 0.0);
        let yaw = (-camp.x).atan2(-camp.z);
        b.model("mini-forest/tent", camp + Vec3::Y * GROUND_Y - side * 2.6 - back * 1.0, 2.2, yaw + 0.4);
        b.model("survival-kit/tent-canvas", camp + Vec3::Y * GROUND_Y + side * 2.6 - back * 1.0, 2.0, yaw - 0.4);
        b.model("survival-kit/campfire-pit", camp + Vec3::Y * GROUND_Y + back * 0.6, 1.8, 0.0);
        b.model("nature-kit/log_large", camp + Vec3::Y * GROUND_Y - back * 1.4, 1.8, yaw + FRAC_PI_2);
        return;
    }

    // Désert : du sable nu autour du plateau, quelques grands cactus bien espacés, un bivouac avec trois palmiers au fond,
    // et seulement quelques rochers, au loin. Les creux de la figure (centre vide) reçoivent de petits cactus, pas des rochers.
    if universe == Universe::Desert {
        let mut rng = Rng::new(universe as u64 * 7919 + shape as u64 * 31);
        let (cx, cz) = ((l.w - 1) as f32 / 2.0, (l.h - 1) as f32 / 2.0);
        let reach = (l.w.max(l.h) as f32) / 2.0 + 36.0;
        let dist = |p: Vec3| l.distance_to_walkable(Vec2::new(p.x + cx, p.z + cz));
        let front = |p: Vec3| p.x * 0.4 + p.z > 0.0;
        // Les cases vides à l'intérieur de la figure : un petit cactus par carré de trois cases, pas plus.
        for y in 0..l.h {
            for x in 0..l.w {
                let c = IVec2::new(x, y);
                if l.walkable(c) || x % 3 != 1 || y % 3 != 1 {
                    continue;
                }
                let p = cell_to_world(c.as_vec2() + Vec2::new(rng.below(40) as f32 / 100.0 - 0.2, rng.below(40) as f32 / 100.0 - 0.2), GROUND_Y, l);
                b.model("nature-kit/cactus_short", p, 1.3 + rng.below(40) as f32 / 100.0, rng.below(628) as f32 / 100.0);
            }
        }
        // Les grands cactus : un anneau régulier, bien espacés, un petit cactus au pied de chacun.
        let n = ((l.w + l.h) as f32 / 4.0).round().max(5.0) as i32;
        for i in 0..n {
            let a = i as f32 / n as f32 * std::f32::consts::TAU + 0.7;
            let centre = Vec3::new(a.cos() * (cx + 5.5), GROUND_Y, a.sin() * (cz + 5.5));
            if dist(centre) < 3.0 || front(centre) && a.sin() > 0.6 {
                continue;
            }
            b.model("nature-kit/cactus_tall", centre, 2.6, rng.below(628) as f32 / 100.0);
            b.model("nature-kit/cactus_short", centre + Vec3::new(1.1, 0.0, 0.5), 1.6, rng.below(628) as f32 / 100.0);
        }
        // Au loin : des rochers de sable, rares et gros, en quinconce ; côté caméra, plus loin encore.
        let (step, mut row) = (4.0_f32, 0);
        let mut z = -reach;
        while z <= reach {
            let mut x = -reach + if row % 2 == 0 { 0.0 } else { step / 2.0 };
            while x <= reach {
                let p = Vec3::new(x + (rng.below(100) as f32 / 100.0 - 0.5) * 2.0, GROUND_Y, z + (rng.below(100) as f32 / 100.0 - 0.5) * 2.0);
                let d = dist(p);
                let edge = if front(p) { 16.0 } else { 12.0 };
                if d >= edge && rng.below(5) == 0 {
                    let t = ((d - edge) / 20.0).min(1.0);
                    let file = ["pirate-kit/rocks-sand-a", "pirate-kit/rocks-sand-b", "pirate-kit/rocks-sand-c"][rng.below(3) as usize];
                    b.model(file, p, 2.0 + t * 2.0, rng.below(628) as f32 / 100.0);
                } else if d >= edge - 4.0 && rng.below(4) == 0 {
                    b.model("nature-kit/cactus_tall", p, 2.4, rng.below(628) as f32 / 100.0);
                }
                x += step;
            }
            z += step;
            row += 1;
        }
        // Le bivouac : au fond, une tente face au plateau, un feu, un panneau, et trois palmiers derrière (l'oasis).
        let back = Vec3::new(-0.37, 0.0, -0.93);
        let side = Vec3::new(0.93, 0.0, -0.37);
        let camp = back * ((cx * back.x.abs() + cz * back.z.abs()) + 6.0);
        let yaw = (-camp.x).atan2(-camp.z);
        let g = Vec3::Y * GROUND_Y;
        b.model("survival-kit/tent-canvas", camp + g - side * 2.2, 2.0, yaw + 0.3);
        b.model("survival-kit/campfire-pit", camp + g + side * 0.6 + back * 0.4, 1.8, 0.0);
        b.model("survival-kit/signpost", camp + g + side * 2.6 - back * 0.8, 2.0, yaw - 0.5);
        for (k, file) in ["pirate-kit/palm-straight", "pirate-kit/palm-detailed-bend", "pirate-kit/palm-bend"].iter().enumerate() {
            b.model(file, camp + g + back * 3.2 + side * ((k as f32 - 1.0) * 2.8), 2.4, rng.below(628) as f32 / 100.0);
        }
        return;
    }

    // Décor autour du plateau, par bandes : petit près du plateau, de plus en plus gros en s'éloignant (profondeur homogène).
    // Chaque bande ne sert que quelques modèles, rangés dans un ordre régulier, tournés vers le centre.
    let mut rng = Rng::new(universe as u64 * 7919 + shape as u64 * 31);
    let [near, mid, far] = bands(universe);
    let (cx, cz) = ((l.w - 1) as f32 / 2.0, (l.h - 1) as f32 / 2.0);
    let reach = (l.w.max(l.h) as f32) / 2.0 + 34.0;
    // (modèles, pas de la grille, distance minimale, distance maximale, échelle, saute les grands objets côté caméra)
    for (list, step, lo, hi, k, low_front) in [(near, 2.2_f32, 1.8_f32, 6.0_f32, 0.8_f32, true), (mid, 3.8, 6.0, 14.0, 1.0, true), (far, 9.0, 14.0, 32.0, 2.2, true)] {
        if list.is_empty() {
            continue;
        }
        let (mut ix, mut x) = (0usize, -reach);
        while x <= reach {
            let (mut iz, mut z) = (0usize, -reach);
            while z <= reach {
                let jitter = |rng: &mut Rng| (rng.below(100) as f32 / 100.0 - 0.5) * step * 0.35;
                let pos = Vec3::new(x + jitter(&mut rng), GROUND_Y, z + jitter(&mut rng));
                let d = l.distance_to_walkable(Vec2::new(pos.x + cx, pos.z + cz));
                if d >= lo && d < hi {
                    let (file, scale) = list[(ix * 7 + iz * 13) % list.len()];
                    // Côté caméra, rien de haut qui cacherait le plateau.
                    let front = pos.x * 0.4 + pos.z > 0.0;
                    if !(low_front && front && is_tall(file)) {
                        // Tourné vers le centre, à un petit écart près.
                        let yaw = (-pos.x).atan2(-pos.z) + (rng.below(100) as f32 / 100.0 - 0.5) * 0.4;
                        let pos = if matches!(universe, Universe::Ocean | Universe::Port) { pos + Vec3::Y * 0.05 } else { pos };
                        b.model(file, pos, scale * k, yaw);
                    }
                }
                z += step;
                iz += 1;
            }
            x += step;
            ix += 1;
        }
    }
}

/// Les trois bandes du décor autour du plateau (près, au milieu, au loin) : modèles Kenney (CC0), (fichier, échelle de base).
/// Les échelles de bande (petit, normal, gros) sont appliquées par `rebuild_world`.
fn bands(u: Universe) -> [&'static [(&'static str, f32)]; 3] {
    match u {
        Universe::Prairie => [&[], &[], &[]],
        Universe::Desert => [&[], &[], &[]],
        Universe::Ruines => [
            &[("mini-arena/bricks", 2.0), ("prototype-kit/column-low", 2.6), ("mini-arena/column-damaged", 2.6)],
            &[("prototype-kit/column", 2.6), ("mini-arena/statue", 2.6), ("prototype-kit/column-rounded", 2.6), ("fantasy-town-kit/fountain-round", 2.2), ("mini-arena/trophy", 2.4)],
            &[("fantasy-town-kit/wall-arch", 2.2), ("fantasy-town-kit/pillar-stone", 2.6), ("mini-arena/column", 2.6)],
        ],
        Universe::Cimetiere => [
            &[("graveyard-kit/pumpkin-carved", 2.4), ("graveyard-kit/gravestone-round", 2.4), ("graveyard-kit/gravestone-broken", 2.4)],
            &[("graveyard-kit/gravestone-cross", 2.4), ("graveyard-kit/lightpost-single", 2.4), ("graveyard-kit/iron-fence", 2.4), ("graveyard-kit/pine-crooked", 2.4)],
            &[("graveyard-kit/crypt-small", 2.2), ("graveyard-kit/pine", 2.4), ("graveyard-kit/pine-fall", 2.4)],
        ],
        Universe::Ocean => [
            &[("pirate-kit/barrel", 1.6), ("pirate-kit/crate", 1.6), ("pirate-kit/chest", 1.6)],
            &[("pirate-kit/boat-row-small", 1.4), ("pirate-kit/palm-straight", 1.9), ("pirate-kit/rocks-a", 1.3), ("pirate-kit/palm-bend", 1.9)],
            &[("pirate-kit/ship-small", 1.0), ("pirate-kit/palm-detailed-straight", 1.9), ("pirate-kit/rocks-a", 1.3)],
        ],
        Universe::Parc => [
            &[("coaster-kit/bench", 2.2), ("coaster-kit/flowers", 2.4), ("coaster-kit/trash", 2.2)],
            &[("coaster-kit/stall-food", 2.0), ("coaster-kit/stall-drinks", 2.0), ("coaster-kit/tree", 2.2), ("coaster-kit/stall-information", 2.0), ("fantasy-town-kit/lantern", 2.4)],
            &[("coaster-kit/support-large", 1.6), ("coaster-kit/park-entrance", 1.2), ("coaster-kit/tree-large", 2.0), ("coaster-kit/station", 1.0)],
        ],
        Universe::Ville => [&[], &[], &[]],
        Universe::Gare => [
            &[("city-kit-roads/construction-cone", 2.0), ("city-kit-industrial/shipping-container-a", 1.0), ("city-kit-industrial/detail-tank", 1.4)],
            &[("train-kit/train-carriage-box", 0.6), ("city-kit-industrial/building-a", 1.3), ("city-kit-industrial/shipping-container-b", 1.2), ("city-kit-industrial/building-c", 1.3)],
            &[("city-kit-industrial/chimney-medium", 1.2), ("city-kit-industrial/water-tower", 1.2), ("city-kit-industrial/building-e", 1.3)],
        ],
        Universe::Port => [
            &[("pirate-kit/barrel", 1.6), ("pirate-kit/crate", 1.6)],
            &[("city-kit-industrial/shipping-container-a", 1.2), ("pirate-kit/boat-row-small", 1.4), ("city-kit-industrial/shipping-container-c", 1.2)],
            &[("pirate-kit/ship-small", 1.0), ("city-kit-industrial/detail-tank-large", 1.2), ("city-kit-industrial/water-tower", 1.2)],
        ],
        Universe::Medieval => [
            &[("fantasy-town-kit/fence", 1.8), ("fantasy-town-kit/planks", 1.8), ("castle-kit/tree-small", 2.0)],
            &[("fantasy-town-kit/stall-red", 1.6), ("fantasy-town-kit/stall-green", 1.6), ("fantasy-town-kit/cart", 1.6), ("mini-forest/tent", 2.0), ("fantasy-town-kit/lantern", 2.2)],
            &[("castle-kit/tower-hexagon-base", 2.0), ("castle-kit/tree-large", 2.4), ("fantasy-town-kit/windmill", 1.2)],
        ],
        Universe::Cave => [
            &[("nature-kit/rock_smallA", 2.6), ("nature-kit/mushroom_redGroup", 3.0)],
            &[("nature-kit/rock_largeA", 2.4), ("nature-kit/mushroom_tanTall", 3.0), ("nature-kit/rock_largeB", 2.4), ("nature-kit/mushroom_redTall", 3.0), ("nature-kit/rock_largeD", 2.4)],
            &[("nature-kit/rock_largeC", 2.4), ("nature-kit/rock_largeA", 2.4), ("nature-kit/rock_largeD", 2.4)],
        ],
    }
}

/// Hauts ou larges : ils cacheraient le plateau s'ils étaient devant (côté caméra).
fn is_tall(file: &str) -> bool {
    ["tree", "pine", "palm", "cactus_tall", "prototype-kit/column", "mini-arena/column", "wall-arch", "pillar-stone", "statue", "ship", "boat", "crypt", "tent", "lightpost", "log_large", "fence", "rocks", "rock_large", "building", "tower", "castle-kit/wall", "windmill", "water-tower", "chimney", "traffic-light", "stall", "support", "park-entrance"].iter().any(|k| file.contains(k))
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

