//! Les cases spéciales du plateau : obstacles, trous, téléporteurs, accélérateurs, tapis roulants.
use std::collections::HashMap;
use std::f32::consts::{FRAC_PI_2, FRAC_PI_4};

use bevy::light::NotShadowCaster;
use bevy::prelude::*;
use game_common::{Rng, flat_material};

use crate::layout::Layout;
use crate::world::{Universe, cell_to_world};

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Tile {
    /// Pierre, cactus, tonneau… selon le paysage (indice du modèle).
    Obstacle(u8),
    Hole,
    /// Les deux cases de même numéro sont reliées.
    Teleport(u8),
    Speeder,
    /// Pousse vers la direction indiquée.
    Belt(IVec2),
    /// La sortie d'un niveau du Voyage.
    Exit,
    Portal,
    /// Barrière qui s'ouvre et se ferme au rythme des pas (décalage de phase).
    Gate(u8),
    /// Ponton fragile : un trou après le passage.
    Plank,
    /// Une pierre va tomber ici au pas indiqué (la case clignote), puis elle restera comme un bloc.
    Rubble(u32),
    /// Chemin du guide rival (vrai : le long de x).
    Lane(bool),
}

/// Ce que le jeu sait poser sur le plateau.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Feature {
    Obstacle,
    Hole,
    Teleport,
    Speeder,
    Belt,
    /// Portail de paysage (mode infini) : change le décor, pas le plateau.
    Portal,
    Gate,
    /// Un guide rival qui traverse le plateau en emmenant les touristes qu'il touche.
    Rival,
    /// Un train mortel qui traverse le plateau.
    Train,
    /// Ponton : s'effondre (devient un trou) dès que toute la file l'a quitté.
    Plank,
    /// Éboulement programmé (nombre à venir) : des pierres tomberont une à une et resteront.
    Cave,
}

/// Un guide rival et ses deux touristes : il traverse tout le plateau le long d'une ligne de cases, puis revient après une pause.
pub struct Rival {
    /// Les cases du chemin, dans le sens de la marche (la première est l'entrée).
    pub cells: Vec<IVec2>,
    pub offset: u32,
    /// Un vrai train (mortel, 5 wagons) au lieu d'un guide et ses deux touristes.
    pub train: bool,
}

impl Rival {
    pub fn cars(&self) -> i32 {
        if self.train { 5 } else { 3 }
    }

    /// Pas pendant lesquels l'entrée clignote avant son arrivée.
    pub const WARN: u32 = 10;

    pub fn period(&self) -> u32 {
        self.cells.len() as u32 + self.cars() as u32 + 14
    }

    /// Indice (sur le chemin) du guide rival au pas `clock`.
    pub fn front(&self, clock: u32) -> i32 {
        ((clock + self.offset) % self.period()) as i32
    }

    /// Il arrive dans quelques pas : l'entrée clignote.
    pub fn warning(&self, clock: u32) -> bool {
        self.front(clock) as u32 + Self::WARN >= self.period()
    }

    pub fn hits(&self, c: IVec2, clock: u32) -> bool {
        let f = self.front(clock);
        self.cells.iter().position(|p| *p == c).is_some_and(|i| (f - self.cars() + 1..=f).contains(&(i as i32)))
    }
}

/// Nombre de pas pendant lesquels une barrière reste ouverte (puis fermée).
pub const GATE_PERIOD: u32 = 4;

#[derive(Resource, Default)]
pub struct Board {
    pub tiles: HashMap<IVec2, Tile>,
    pub teleports: u8,
    /// Nombre de pas faits (rythme des barrières).
    pub clock: u32,
    pub rivals: Vec<Rival>,
    /// Éboulements encore à venir (grottes).
    pub falls_left: u32,
}

impl Board {
    pub fn clear(&mut self) {
        self.tiles.clear();
        self.teleports = 0;
        self.clock = 0;
        self.rivals.clear();
        self.falls_left = 0;
    }

    /// Un rival occupe-t-il cette case au pas `clock` ?
    /// Un train (mortel) occupe-t-il cette case ?
    pub fn train_hits(&self, c: IVec2, clock: u32) -> bool {
        self.rivals.iter().any(|t| t.train && t.hits(c, clock))
    }

    pub fn rival_hits(&self, c: IVec2, clock: u32) -> bool {
        self.rivals.iter().any(|t| !t.train && t.hits(c, clock))
    }

    /// La barrière de cette case est-elle fermée au pas `clock` ?
    pub fn gate_closed(phase: u8, clock: u32) -> bool {
        (clock / GATE_PERIOD + phase as u32) % 2 == 1
    }

    /// Cette case est-elle un mur au pas `clock` (barrière fermée) ?
    pub fn shut(&self, c: IVec2, clock: u32) -> bool {
        matches!(self.tiles.get(&c), Some(Tile::Gate(ph)) if Self::gate_closed(*ph, clock))
    }

    pub fn partner(&self, from: IVec2, id: u8) -> Option<IVec2> {
        self.tiles.iter().find(|(c, t)| **t == Tile::Teleport(id) && **c != from).map(|(c, _)| *c)
    }

    /// Nombre de cases de ce genre déjà posées.
    pub fn count(&self, f: Feature) -> usize {
        if f == Feature::Cave {
            return self.falls_left as usize;
        }
        if matches!(f, Feature::Rival | Feature::Train) {
            return self.rivals.iter().filter(|r| r.train == (f == Feature::Train)).count();
        }
        self.tiles
            .values()
            .filter(|t| {
                matches!(
                    (f, t),
                    (Feature::Obstacle, Tile::Obstacle(_))
                        | (Feature::Hole, Tile::Hole)
                        | (Feature::Teleport, Tile::Teleport(_))
                        | (Feature::Speeder, Tile::Speeder)
                        | (Feature::Belt, Tile::Belt(_))
                        | (Feature::Portal, Tile::Portal) | (Feature::Gate, Tile::Gate(_)) | (Feature::Plank, Tile::Plank)
                )
            })
            .count()
    }

    /// Pose un élément à un endroit libre, loin de la tête. `false` si aucune place n'a été trouvée.
    pub fn place(&mut self, f: Feature, busy: &[IVec2], head: IVec2, layout: &Layout, universe: Universe, rng: &mut Rng) -> bool {
        let pick = |rng: &mut Rng| IVec2::new(rng.below(layout.w as u32) as i32, rng.below(layout.h as u32) as i32);
        for _ in 0..80 {
            let c = pick(rng);
            if !layout.walkable(c) || !self.is_free(c, busy, head) {
                continue;
            }
            match f {
                Feature::Obstacle | Feature::Hole => {
                    if self.next_to_blocker(c) {
                        continue;
                    }
                    let t = if f == Feature::Hole { Tile::Hole } else { Tile::Obstacle(rng.below(obstacle_models(universe).len() as u32) as u8) };
                    self.tiles.insert(c, t);
                    return true;
                }
                Feature::Speeder => {
                    self.tiles.insert(c, Tile::Speeder);
                    return true;
                }
                Feature::Rival | Feature::Train => {
                    let horizontal = rng.below(2) == 0;
                    // Il passe au milieu du plateau, pas le long d'un bord où il ne gênerait personne.
                    let (pos, size) = if horizontal { (c.y, layout.h) } else { (c.x, layout.w) };
                    if pos < size / 4 || pos > size - size / 4 {
                        continue;
                    }
                    let line: Vec<IVec2> = if horizontal { (0..layout.w).map(|x| IVec2::new(x, c.y)).collect() } else { (0..layout.h).map(|y| IVec2::new(c.x, y)).collect() };
                    let line: Vec<IVec2> = line.into_iter().filter(|p| layout.walkable(*p)).collect();
                    // Le chemin doit traverser tout le plateau d'un seul tenant, loin de la tête.
                    let straight = line.windows(2).all(|w| (w[1] - w[0]).abs().element_sum() == 1);
                    let mut line = line;
                    if rng.below(2) == 0 {
                        line.reverse();
                    }
                    if line.len() >= 8 && straight && line.iter().all(|p| self.is_free(*p, busy, head) && !busy.contains(p)) {
                        for p in &line {
                            self.tiles.insert(*p, Tile::Lane(horizontal));
                        }
                        let offset = rng.below(line.len() as u32 + 6);
                        self.rivals.push(Rival { cells: line, offset, train: f == Feature::Train });
                        return true;
                    }
                }
                Feature::Gate => {
                    self.tiles.insert(c, Tile::Gate(rng.below(2) as u8));
                    return true;
                }
                Feature::Portal => {
                    self.tiles.insert(c, Tile::Portal);
                    return true;
                }
                Feature::Teleport => {
                    let d = pick(rng);
                    if d == c || !layout.walkable(d) || !self.is_free(d, busy, head) || c.as_vec2().distance(d.as_vec2()) < 4.0 {
                        continue;
                    }
                    let id = self.teleports;
                    self.teleports += 1;
                    self.tiles.insert(c, Tile::Teleport(id));
                    self.tiles.insert(d, Tile::Teleport(id));
                    return true;
                }
                Feature::Cave => {
                    self.falls_left += 1;
                    return true;
                }
                Feature::Plank => {
                    let dir = [IVec2::X, IVec2::Y][rng.below(2) as usize];
                    let cells: Vec<IVec2> = (0..3).map(|i| c + dir * i).collect();
                    // Les deux bouts doivent rester praticables : le ponton ne doit pas enfermer la file.
                    if cells.iter().all(|p| layout.walkable(*p) && self.is_free(*p, busy, head) && !self.next_to_blocker(*p)) && layout.walkable(c - dir) && layout.walkable(c + dir * 3) {
                        for p in cells {
                            self.tiles.insert(p, Tile::Plank);
                        }
                        return true;
                    }
                }
                Feature::Belt => {
                    let dir = [IVec2::X, IVec2::NEG_X, IVec2::Y, IVec2::NEG_Y][rng.below(4) as usize];
                    let cells: Vec<IVec2> = (0..4).map(|i| c + dir * i).collect();
                    let beyond = c + dir * 4;
                    if cells.iter().all(|p| layout.walkable(*p) && self.is_free(*p, busy, head)) && layout.walkable(beyond) {
                        for p in cells {
                            self.tiles.insert(p, Tile::Belt(dir));
                        }
                        return true;
                    }
                }
            }
        }
        false
    }

    pub fn is_free(&self, c: IVec2, busy: &[IVec2], head: IVec2) -> bool {
        !self.tiles.contains_key(&c) && !busy.contains(&c) && (c - head).abs().element_sum() >= 4
    }

    pub fn next_to_blocker(&self, c: IVec2) -> bool {
        (-1..=1).any(|dx| {
            (-1..=1).any(|dy| matches!(self.tiles.get(&(c + IVec2::new(dx, dy))), Some(Tile::Obstacle(_) | Tile::Hole)))
        })
    }
}

/// Les modèles Kenney (CC0) des obstacles, selon le paysage : (fichier, échelle).
pub fn obstacle_models(u: Universe) -> &'static [(&'static str, f32)] {
    match u {
        Universe::Prairie => &[("nature-kit/rock_largeA", 1.8), ("nature-kit/rock_largeC", 1.8), ("nature-kit/stump_round", 2.2), ("nature-kit/plant_bush", 2.0)],
        Universe::Desert => &[("nature-kit/cactus_short", 2.2), ("nature-kit/cactus_tall", 1.8), ("survival-kit/rock-sand-a", 2.5), ("survival-kit/rock-sand-b", 2.5)],
        Universe::Ruines => &[("mini-arena/column-damaged", 2.4), ("prototype-kit/column-low", 2.4), ("nature-kit/rock_largeA", 1.8), ("mini-arena/bricks", 2.0)],
        Universe::Cimetiere => &[("graveyard-kit/gravestone-cross", 2.4), ("graveyard-kit/rocks", 2.0), ("graveyard-kit/pumpkin", 2.4), ("graveyard-kit/gravestone-broken", 2.4)],
        Universe::Ocean => &[("survival-kit/barrel", 2.5), ("nature-kit/rock_largeE", 1.8), ("nature-kit/log", 2.0)],
        Universe::Parc => &[("coaster-kit/bench", 2.2), ("coaster-kit/trash", 2.4), ("coaster-kit/flowers", 2.4), ("fantasy-town-kit/stall-bench", 2.0)],
        Universe::Ville => &[("city-kit-suburban/tree-large", 2.0), ("city-kit-suburban/tree-small", 2.2), ("nature-kit/plant_bush", 2.0), ("nature-kit/rock_largeA", 1.8), ("special/pond", 1.0), ("special/pond", 1.0)],
        Universe::Gare => &[("city-kit-industrial/shipping-container-a", 1.0), ("city-kit-industrial/detail-tank", 1.2), ("city-kit-roads/construction-barrier", 2.0)],
        Universe::Port => &[("pirate-kit/barrel", 1.6), ("pirate-kit/crate", 1.6), ("city-kit-industrial/shipping-container-a", 1.0)],
        Universe::Cave => &[("nature-kit/rock_largeA", 1.8), ("nature-kit/rock_largeB", 1.8), ("nature-kit/rock_largeD", 1.6), ("nature-kit/mushroom_redGroup", 2.4)],
        Universe::Medieval => &[("fantasy-town-kit/cart", 1.6), ("castle-kit/rocks-small", 1.8), ("castle-kit/tree-trunk", 1.8), ("fantasy-town-kit/planks", 1.8)],
    }
}

pub const TELEPORT_COLORS: [Color; 4] = [
    Color::srgb(0.25, 0.55, 1.0),
    Color::srgb(1.0, 0.55, 0.15),
    Color::srgb(0.3, 0.85, 0.45),
    Color::srgb(0.85, 0.35, 0.9),
];

#[derive(Component)]
pub struct TileVisual;

#[derive(Component)]
pub struct Spin(f32);

#[derive(Component)]
pub struct Scroll(f32);

#[derive(Component)]
pub struct Pulse(f32);

/// Reconstruit les éléments visibles quand le plateau change.
pub fn sync_tiles(
    mut commands: Commands,
    board: Res<Board>,
    scenery: Res<crate::voyage::Scenery>,
    layout: Res<Layout>,
    server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<StandardMaterial>>,
    old: Query<Entity, With<TileVisual>>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
    mut count: Local<usize>,
) {
    if !board.is_changed() && !scenery.is_changed() {
        return;
    }
    for e in &old {
        commands.entity(e).despawn();
    }
    let grid = &*layout;
    let disc = meshes.add(Cylinder::new(0.46, 0.05));
    let plate = meshes.add(Cuboid::new(0.94, 0.06, 0.94));
    let cube = meshes.add(Cuboid::new(1.0, 1.0, 1.0));
    let torus = meshes.add(Torus::new(0.22, 0.36));
    let mut mk = |c: Color, k: f32| {
        let l = c.to_linear();
        mats.add(StandardMaterial { emissive: LinearRgba::rgb(l.red * k, l.green * k, l.blue * k), ..flat_material(c) })
    };
    let hole_rim = mk(Color::srgb(0.35, 0.25, 0.18), 0.0);
    let hole = mk(Color::srgb(0.04, 0.03, 0.05), 0.0);
    let belt = mk(Color::srgb(0.22, 0.24, 0.30), 0.0);
    let chevron = mk(Color::srgb(1.0, 0.85, 0.2), 1.2);
    let gate_base = mk(Color::srgb(0.30, 0.30, 0.34), 0.0);
    let gate_shut = mk(Color::srgb(0.95, 0.25, 0.20), 0.8);
    let gate_warn = mk(Color::srgb(1.0, 0.75, 0.15), 1.0);
    let gate_open = mk(Color::srgb(0.35, 0.85, 0.45), 0.4);
    let rail_bed = mk(Color::srgb(0.60, 0.62, 0.66), 0.0);
    let rail_steel = mk(Color::WHITE, 0.0);
    let rail_lamp = mk(Color::srgb(1.0, 0.25, 0.15), 2.0);
    let pond_edge = mk(Color::srgb(0.62, 0.60, 0.52), 0.0);
    let pond = mk(Color::srgb(0.28, 0.58, 0.82), 0.25);
    let rubble_shadow = mk(Color::srgb(0.20, 0.17, 0.18), 0.0);
    let rubble_warn = mk(Color::srgb(1.0, 0.35, 0.15), 1.8);
    let plank_wood = mk(Color::srgb(0.58, 0.45, 0.30), 0.0);
    let plank_dark = mk(Color::srgb(0.36, 0.30, 0.24), 0.0);
    let speed_plate = mk(Color::srgb(1.0, 0.8, 0.1), 0.6);
    let speed_core = mk(Color::WHITE, 2.0);
    *count = board.tiles.len();
    // Les guides rivaux : un guide et deux touristes en marche, et un signal clignotant à l'entrée.
    for (n, t) in board.rivals.iter().enumerate() {
        let dir = if t.cells.len() > 1 { t.cells[1] - t.cells[0] } else { IVec2::X };
        let yaw = Quat::from_rotation_y((dir.x as f32).atan2(dir.y as f32));
        if t.train {
            let cars = ["train-kit/train-diesel-a", "train-kit/train-carriage-box", "train-kit/train-carriage-container-red", "train-kit/train-carriage-tank", "train-kit/train-carriage-box"];
            for (k, file) in cars.iter().enumerate() {
                let scene = server.load(GltfAssetLabel::Scene(0).from_asset(format!("kenney/{file}.glb")));
                commands.spawn((
                    WorldAssetRoot(scene),
                    Transform::from_translation(Vec3::new(0.0, -50.0, 0.0)).with_rotation(yaw).with_scale(Vec3::new(0.75, 0.6, 0.4)),
                    Visibility::Hidden,
                    RivalCar { rival: n, car: k as i32, dir },
                    TileVisual,
                ));
            }
        } else {
            for (k, kind) in [3usize, 5, 8].into_iter().enumerate() {
                let root = commands.spawn((Transform::from_translation(Vec3::new(0.0, -50.0, 0.0)).with_rotation(yaw), Visibility::Hidden, RivalCar { rival: n, car: k as i32, dir }, TileVisual)).id();
                crate::people::spawn_model(&mut commands, &server, &mut graphs, root, kind, crate::people::Pose::Walk, 1.0, (n * 7 + k * 13 + 5) as u32, k == 0);
            }
        }
        let at_entry = cell_to_world(t.cells[0].as_vec2(), 0.12, grid);
        commands.spawn((Mesh3d(disc.clone()), MeshMaterial3d(rail_lamp.clone()), Transform::from_translation(at_entry).with_scale(Vec3::new(1.1, 1.0, 1.1)), Visibility::Hidden, RivalMarker(n), NotShadowCaster, TileVisual));
        let post = cell_to_world(t.cells[0].as_vec2(), 0.7, grid);
        commands.spawn((Mesh3d(cube.clone()), MeshMaterial3d(rail_lamp.clone()), Transform::from_translation(post).with_scale(Vec3::new(0.2, 1.2, 0.2)), Visibility::Hidden, RivalMarker(n), NotShadowCaster, TileVisual));
    }

    for (&cell, &tile) in &board.tiles {
        let at = |y: f32| Transform::from_translation(cell_to_world(cell.as_vec2(), y + 0.05, grid));
        let spawn = |commands: &mut Commands, m: Handle<Mesh>, mat: Handle<StandardMaterial>, tf: Transform, parent: Option<Entity>| {
            let mut e = commands.spawn((Mesh3d(m), MeshMaterial3d(mat), tf, NotShadowCaster));
            if let Some(p) = parent {
                e.insert(ChildOf(p));
            } else {
                e.insert(TileVisual);
            }
            e.id()
        };
        match tile {
            Tile::Obstacle(v) => {
                let models = obstacle_models(scenery.0);
                let (file, scale) = models[v as usize % models.len()];
                // Un étang : un disque d'eau, pas un modèle.
                if file == "special/pond" {
                    spawn(&mut commands, disc.clone(), pond_edge.clone(), at(0.0).with_scale(Vec3::new(2.0, 1.0, 2.0)), None);
                    spawn(&mut commands, disc.clone(), pond.clone(), at(0.02).with_scale(Vec3::new(1.8, 1.0, 1.8)), None);
                    continue;
                }
                let scene = server.load(GltfAssetLabel::Scene(0).from_asset(format!("kenney/{file}.glb")));
                let yaw = (cell.x * 7 + cell.y * 13) as f32 * 0.9;
                commands.spawn((
                    WorldAssetRoot(scene),
                    at(0.0).with_scale(Vec3::splat(scale)).with_rotation(Quat::from_rotation_y(yaw)),
                    TileVisual,
                ));
            }
            Tile::Portal => {
                let m = mk(Color::srgb(0.7, 0.35, 1.0), 1.8);
                spawn(&mut commands, disc.clone(), m.clone(), at(0.01).with_scale(Vec3::new(1.1, 1.0, 1.1)), None);
                let ring = spawn(&mut commands, torus.clone(), mk(Color::WHITE, 1.6), at(0.45).with_scale(Vec3::splat(1.5)).with_rotation(Quat::from_rotation_x(FRAC_PI_2)), None);
                commands.entity(ring).insert(Pulse(1.0));
                let gem = spawn(&mut commands, cube.clone(), m, at(0.5).with_scale(Vec3::splat(0.3)), None);
                commands.entity(gem).insert(Spin(2.5));
            }
            Tile::Exit => {
                let m = mk(Color::srgb(0.2, 0.85, 0.4), 1.4);
                spawn(&mut commands, disc.clone(), m.clone(), at(0.01).with_scale(Vec3::new(1.15, 1.0, 1.15)), None);
                let post = spawn(&mut commands, cube.clone(), m.clone(), at(0.55).with_scale(Vec3::new(0.12, 1.1, 0.12)), None);
                commands.entity(post).insert(Pulse(0.0));
                let sign = spawn(&mut commands, cube.clone(), mk(Color::WHITE, 1.0), at(1.15).with_scale(Vec3::new(0.7, 0.35, 0.1)), None);
                commands.entity(sign).insert(Spin(1.2));
            }
            Tile::Gate(phase) => {
                let root = commands.spawn((at(0.0), Visibility::default(), TileVisual)).id();
                spawn(&mut commands, plate.clone(), gate_base.clone(), Transform::IDENTITY, Some(root));
                let kinds = [(&gate_shut, 0u8, Vec3::new(0.9, 0.8, 0.14), 0.4), (&gate_warn, 1, Vec3::new(0.9, 0.3, 0.14), 0.15), (&gate_open, 2, Vec3::new(0.9, 0.05, 0.14), 0.05)];
                for (m, kind, size, y) in kinds {
                    let e = spawn(&mut commands, cube.clone(), m.clone(), Transform::from_xyz(0.0, y, 0.0).with_scale(size), Some(root));
                    commands.entity(e).insert(GateBar { phase, kind });
                }
            }
            Tile::Lane(horizontal) => {
                let yaw = if horizontal { FRAC_PI_2 } else { 0.0 };
                let root = commands.spawn((at(0.0).with_rotation(Quat::from_rotation_y(yaw)), Visibility::default(), TileVisual)).id();
                spawn(&mut commands, plate.clone(), rail_bed.clone(), Transform::IDENTITY, Some(root));
                spawn(&mut commands, cube.clone(), rail_steel.clone(), Transform::from_xyz(0.0, 0.05, 0.0).with_scale(Vec3::new(0.08, 0.02, 0.5)), Some(root));
            }
            Tile::Hole => {
                spawn(&mut commands, disc.clone(), hole_rim.clone(), at(0.0).with_scale(Vec3::new(1.08, 1.0, 1.08)), None);
                spawn(&mut commands, disc.clone(), hole.clone(), at(0.02).with_scale(Vec3::new(0.9, 1.0, 0.9)), None);
            }
            Tile::Teleport(id) => {
                let c = TELEPORT_COLORS[id as usize % TELEPORT_COLORS.len()];
                let m = mk(c, 1.6);
                spawn(&mut commands, disc.clone(), m.clone(), at(0.01), None);
                let ring = spawn(&mut commands, torus.clone(), mk(Color::WHITE, 1.4), at(0.28).with_rotation(Quat::from_rotation_x(FRAC_PI_2 * 0.35)), None);
                commands.entity(ring).insert(Spin(2.2));
                let gem = spawn(&mut commands, cube.clone(), m, at(0.62).with_scale(Vec3::splat(0.22)), None);
                commands.entity(gem).insert(Spin(-3.0));
            }
            Tile::Speeder => {
                spawn(&mut commands, plate.clone(), speed_plate.clone(), at(0.0), None);
                let core = spawn(&mut commands, cube.clone(), speed_core.clone(), at(0.06).with_scale(Vec3::new(0.34, 0.05, 0.34)).with_rotation(Quat::from_rotation_y(FRAC_PI_4)), None);
                commands.entity(core).insert(Pulse(cell.x as f32 * 0.7));
            }
            Tile::Rubble(_) => {
                spawn(&mut commands, disc.clone(), rubble_shadow.clone(), at(0.0).with_scale(Vec3::new(1.0, 1.0, 1.0)), None);
                let core = spawn(&mut commands, cube.clone(), rubble_warn.clone(), at(0.06).with_scale(Vec3::new(0.4, 0.05, 0.4)).with_rotation(Quat::from_rotation_y(FRAC_PI_4)), None);
                commands.entity(core).insert(Pulse(cell.x as f32 * 0.9));
            }
            Tile::Plank => {
                // Planche pourrie : trois lattes de travers, usées, avec des jours entre elles.
                for (k, z) in [-0.32_f32, 0.0, 0.32].into_iter().enumerate() {
                    let tilt = ((cell.x * 5 + cell.y * 3 + k as i32 * 7) % 5 - 2) as f32 * 0.05;
                    let mut t = at(0.0);
                    t.translation.z += z;
                    let mat = if (cell.x + cell.y + k as i32) % 3 == 0 { plank_dark.clone() } else { plank_wood.clone() };
                    spawn(&mut commands, cube.clone(), mat, t.with_rotation(Quat::from_rotation_y(tilt)).with_scale(Vec3::new(0.92, 0.07, 0.27)), None);
                }
            }
            Tile::Belt(d) => {
                let yaw = (d.x as f32).atan2(d.y as f32);
                let root = commands.spawn((at(0.0).with_rotation(Quat::from_rotation_y(yaw)), Visibility::default(), TileVisual)).id();
                spawn(&mut commands, plate.clone(), belt.clone(), Transform::IDENTITY, Some(root));
                for i in 0..2 {
                    for s in [-1.0_f32, 1.0] {
                        let e = spawn(
                            &mut commands,
                            cube.clone(),
                            chevron.clone(),
                            Transform::from_xyz(0.0, 0.05, 0.0).with_scale(Vec3::new(0.32, 0.03, 0.07)).with_rotation(Quat::from_rotation_y(s * FRAC_PI_4)),
                            Some(root),
                        );
                        commands.entity(e).insert((Scroll(i as f32 * 0.5), SideOffset(s * 0.1)));
                    }
                }
            }
        }
    }
}

#[derive(Component)]
pub struct SideOffset(f32);

#[derive(Component)]
pub struct RivalCar {
    rival: usize,
    car: i32,
    dir: IVec2,
}

#[derive(Component)]
pub struct RivalMarker(usize);

/// Fait avancer les guides rivaux d'une case par pas, et clignoter l'entrée avant leur arrivée.
pub fn animate_rivals(
    time: Res<Time>,
    board: Res<Board>,
    layout: Res<Layout>,
    mut cars: Query<(&RivalCar, &mut Transform, &mut Visibility), Without<RivalMarker>>,
    mut marks: Query<(&RivalMarker, &mut Visibility), Without<RivalCar>>,
) {
    let blink = (time.elapsed_secs() * 7.0).sin() > 0.0;
    for (l, mut v) in &mut marks {
        let on = board.rivals.get(l.0).is_some_and(|t| t.warning(board.clock)) && blink;
        *v = if on { Visibility::Inherited } else { Visibility::Hidden };
    }
    for (c, mut tf, mut v) in &mut cars {
        let Some(t) = board.rivals.get(c.rival) else { continue };
        let i = t.front(board.clock) - c.car;
        if i < -1 || i > t.cells.len() as i32 {
            *v = Visibility::Hidden;
            continue;
        }
        *v = Visibility::Inherited;
        // Au-delà des extrémités, on prolonge en ligne droite : il entre et sort du décor en marchant.
        let target = cell_to_world(t.cells[0].as_vec2() + c.dir.as_vec2() * i as f32, 0.05, &layout);
        let jump = tf.translation.distance(target) > 2.5;
        tf.translation = if jump { target } else { tf.translation.lerp(target, (time.delta_secs() * 14.0).min(1.0)) };
    }
}

#[derive(Component)]
pub struct GateBar {
    phase: u8,
    /// 0 fermée, 1 sur le point de se fermer, 2 ouverte.
    kind: u8,
}

/// Montre l'état des barrières : fermée (rouge, haute), prête à fermer (orange), ouverte (verte, basse).
pub fn animate_gates(board: Res<Board>, mut bars: Query<(&GateBar, &mut Visibility)>) {
    for (b, mut v) in &mut bars {
        let closed = Board::gate_closed(b.phase, board.clock);
        // Au prochain pas, elle sera fermée : on prévient.
        let soon = !closed && Board::gate_closed(b.phase, board.clock + 1);
        let kind = if closed { 0 } else if soon { 1 } else { 2 };
        *v = if kind == b.kind { Visibility::Inherited } else { Visibility::Hidden };
    }
}

pub fn animate_tiles(
    time: Res<Time>,
    mut spins: Query<(&Spin, &mut Transform), (Without<Scroll>, Without<Pulse>)>,
    mut scrolls: Query<(&Scroll, &SideOffset, &mut Transform), (Without<Spin>, Without<Pulse>)>,
    mut pulses: Query<(&Pulse, &mut Transform), (Without<Spin>, Without<Scroll>)>,
) {
    let t = time.elapsed_secs();
    for (s, mut tf) in &mut spins {
        tf.rotate_y(s.0 * time.delta_secs());
    }
    for (s, side, mut tf) in &mut scrolls {
        // Les chevrons glissent le long du tapis (de l'arrière vers l'avant).
        tf.translation.z = (t * 0.9 + s.0).fract() - 0.5;
        tf.translation.x = side.0;
    }
    for (p, mut tf) in &mut pulses {
        let k = 0.34 + 0.12 * (t * 6.0 + p.0).sin();
        tf.scale = Vec3::new(k, 0.05, k);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rival_warns_before_entering_and_covers_three_cells() {
        let r = Rival { cells: (0..10).map(|x| IVec2::new(x, 0)).collect(), offset: 0, train: false };
        // Au pas 0 il entre : seule la première case est occupée.
        assert!(r.hits(IVec2::new(0, 0), 0) && !r.hits(IVec2::new(1, 0), 0));
        // Au pas 4 il occupe les cases 2 à 4.
        assert!(r.hits(IVec2::new(2, 0), 4) && r.hits(IVec2::new(4, 0), 4) && !r.hits(IVec2::new(5, 0), 4));
        // L'entrée clignote juste avant son arrivée, mais pas juste après son départ.
        assert!(r.warning(r.period() - 1) && !r.warning(5));
        // Il est parti avant la fin du cycle.
        assert!((0..10).all(|x| !r.hits(IVec2::new(x, 0), 14)));
    }
}
