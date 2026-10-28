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
}

/// Ce que le jeu sait poser sur le plateau.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Feature {
    Obstacle,
    Hole,
    Teleport,
    Speeder,
    Belt,
}

#[derive(Resource, Default)]
pub struct Board {
    pub tiles: HashMap<IVec2, Tile>,
    pub teleports: u8,
}

impl Board {
    pub fn clear(&mut self) {
        self.tiles.clear();
        self.teleports = 0;
    }

    pub fn partner(&self, from: IVec2, id: u8) -> Option<IVec2> {
        self.tiles.iter().find(|(c, t)| **t == Tile::Teleport(id) && **c != from).map(|(c, _)| *c)
    }

    /// Nombre de cases de ce genre déjà posées.
    pub fn count(&self, f: Feature) -> usize {
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

    fn is_free(&self, c: IVec2, busy: &[IVec2], head: IVec2) -> bool {
        !self.tiles.contains_key(&c) && !busy.contains(&c) && (c - head).abs().element_sum() >= 4
    }

    fn next_to_blocker(&self, c: IVec2) -> bool {
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
        Universe::Cimetiere => &[("graveyard-kit/gravestone-cross", 2.4), ("graveyard-kit/rocks", 2.0), ("graveyard-kit/pumpkin", 2.4), ("graveyard-kit/gravestone-broken", 2.4)],
        Universe::Ocean => &[("survival-kit/barrel", 2.5), ("nature-kit/rock_largeE", 1.8), ("nature-kit/log", 2.0)],
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
    mut count: Local<usize>,
) {
    if !board.is_changed() {
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
    let speed_plate = mk(Color::srgb(1.0, 0.8, 0.1), 0.6);
    let speed_core = mk(Color::WHITE, 2.0);
    *count = board.tiles.len();

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
                let scene = server.load(GltfAssetLabel::Scene(0).from_asset(format!("kenney/{file}.glb")));
                let yaw = (cell.x * 7 + cell.y * 13) as f32 * 0.9;
                commands.spawn((
                    WorldAssetRoot(scene),
                    at(0.0).with_scale(Vec3::splat(scale)).with_rotation(Quat::from_rotation_y(yaw)),
                    TileVisual,
                ));
            }
            Tile::Exit => {
                let m = mk(Color::srgb(0.2, 0.85, 0.4), 1.4);
                spawn(&mut commands, disc.clone(), m.clone(), at(0.01).with_scale(Vec3::new(1.15, 1.0, 1.15)), None);
                let post = spawn(&mut commands, cube.clone(), m.clone(), at(0.55).with_scale(Vec3::new(0.12, 1.1, 0.12)), None);
                commands.entity(post).insert(Pulse(0.0));
                let sign = spawn(&mut commands, cube.clone(), mk(Color::WHITE, 1.0), at(1.15).with_scale(Vec3::new(0.7, 0.35, 0.1)), None);
                commands.entity(sign).insert(Spin(1.2));
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
