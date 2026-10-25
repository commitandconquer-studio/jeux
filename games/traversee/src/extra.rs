//! Ce qui donne de la variété à Traversée : véhicules en plus (bus, voiture de
//! sport, pick-up, moto, police, taxi, tracteur), fleurs, champignons, souches,
//! et les mouches à gober.
use std::f32::consts::{FRAC_PI_2, PI};

use bevy::prelude::*;
use game_common::flat_material;

use crate::models::{Kit, part, part_rot, spawn_car};

#[derive(Resource)]
pub struct Extra {
    flowers: Vec<Handle<StandardMaterial>>,
    petal_center: Handle<StandardMaterial>,
    stem: Handle<StandardMaterial>,
    tuft: Handle<StandardMaterial>,
    cap: Handle<StandardMaterial>,
    cream: Handle<StandardMaterial>,
    bark: Handle<StandardMaterial>,
    bark_top: Handle<StandardMaterial>,
    white: Handle<StandardMaterial>,
    red_light: Handle<StandardMaterial>,
    pub blue_light: Handle<StandardMaterial>,
    black_stripe: Handle<StandardMaterial>,
    pub bubble: Handle<StandardMaterial>,
    /// Chemin de métal, plateformes octogonales et cargos de l'espace.
    pub metal: Handle<StandardMaterial>,
    pub metal_top: Handle<StandardMaterial>,
    octagon: Handle<Mesh>,
    bus: Vec<Handle<StandardMaterial>>,
    green: Handle<StandardMaterial>,
    taxi: Handle<StandardMaterial>,
    fly_body: Handle<StandardMaterial>,
    fly_gold: Handle<StandardMaterial>,
    fly_wing: Handle<StandardMaterial>,
    fly_eye: Handle<StandardMaterial>,
    biome: BiomeMats,
    /// Une matière par couleur utilisée par les modèles décrits en pièces.
    colors: Vec<([u8; 3], Handle<StandardMaterial>)>,
    /// Index de la peinture blanche dans la palette des voitures.
    white_car: usize,
    /// Index de la peinture jaune (taxi).
    yellow_car: usize,
}

impl Extra {
    /// La matière unie de cette couleur (toutes celles des modèles sont créées au départ).
    fn color(&self, rgb: [u8; 3]) -> &Handle<StandardMaterial> {
        self.colors.iter().find(|(c, _)| *c == rgb).map_or(&self.white, |(_, h)| h)
    }
}

impl FromWorld for Extra {
    fn from_world(world: &mut World) -> Self {
        let octagon = world.resource_mut::<Assets<Mesh>>().add(Cylinder::new(0.5, 1.0).mesh().resolution(8).build().with_duplicated_vertices().with_computed_flat_normals());
        let mut mats = world.resource_mut::<Assets<StandardMaterial>>();
        let biome = BiomeMats::new(&mut mats);
        let metal = mats.add(flat_material(Color::srgb(0.70, 0.73, 0.78)));
        let metal_top = mats.add(flat_material(Color::srgb(0.88, 0.90, 0.94)));
        let bubble = mats.add(StandardMaterial { unlit: true, alpha_mode: AlphaMode::Blend, ..flat_material(Color::srgba(0.75, 0.92, 1.0, 0.32)) });
        // Toutes les couleurs des modèles décrits en pièces (véhicules et décors).
        let mut colors: Vec<([u8; 3], Handle<StandardMaterial>)> = Vec::new();
        let models = crate::gen_vehicles::GEN.iter().map(|g| (g.c2, g.c3)).chain(
            crate::gen_worlds::THEMES.iter().flat_map(|t| {
                [t.blocks, t.sides, t.deco].into_iter().flatten().chain(t.animals.iter().map(|a| &a.model)).chain([&t.hero, &t.hat]).map(|m| (m.c2, m.c3))
            }),
        )
        .chain(std::iter::once((FROG_GREEN, FROG_GREEN)))
        .chain(SPACE_CARGO.iter().chain(HALLOWEEN_COLORS.iter()).chain(PIRATE_COLORS.iter()).map(|c| (*c, *c)));
        for rgb in models.flat_map(|(a, b)| [a, b]) {
            if !colors.iter().any(|(c, _)| *c == rgb) {
                let col = Color::srgb(rgb[0] as f32 / 255.0, rgb[1] as f32 / 255.0, rgb[2] as f32 / 255.0);
                colors.push((rgb, mats.add(flat_material(col))));
            }
        }
        let mut f = |r: f32, g: f32, b: f32| mats.add(flat_material(Color::srgb(r, g, b)));
        let flowers = vec![f(0.98, 0.45, 0.62), f(1.0, 0.85, 0.25), f(0.97, 0.97, 0.97), f(0.92, 0.25, 0.25), f(0.65, 0.45, 0.92), f(1.0, 0.6, 0.2)];
        let bus = vec![f(0.98, 0.78, 0.15), f(0.25, 0.65, 0.40), f(0.85, 0.25, 0.25), f(0.30, 0.50, 0.85)];
        let tuft = f(0.40, 0.68, 0.30);
        let mut glow = |c: Color, e: LinearRgba| mats.add(StandardMaterial { emissive: e, ..flat_material(c) });
        let fly_gold = glow(Color::srgb(1.0, 0.82, 0.2), LinearRgba::rgb(2.5, 1.7, 0.2));
        let red_light = glow(Color::srgb(1.0, 0.2, 0.2), LinearRgba::rgb(3.0, 0.2, 0.2));
        let blue_light = glow(Color::srgb(0.2, 0.4, 1.0), LinearRgba::rgb(0.2, 0.5, 3.0));
        let mut f = |r: f32, g: f32, b: f32| mats.add(flat_material(Color::srgb(r, g, b)));
        Extra {
            metal,
            metal_top,
            octagon,
            bubble,
            biome,
            colors,
            flowers,
            petal_center: f(1.0, 0.8, 0.2),
            stem: f(0.30, 0.58, 0.26),
            tuft,
            cap: f(0.88, 0.22, 0.20),
            cream: f(0.96, 0.93, 0.82),
            bark: f(0.46, 0.32, 0.22),
            bark_top: f(0.78, 0.60, 0.40),
            white: f(0.96, 0.96, 0.96),
            red_light,
            blue_light,
            black_stripe: f(0.12, 0.12, 0.15),
            bus,
            green: f(0.30, 0.62, 0.30),
            taxi: f(0.99, 0.85, 0.15),
            fly_body: f(0.12, 0.12, 0.16),
            fly_gold,
            fly_wing: f(0.85, 0.93, 1.0),
            fly_eye: f(0.9, 0.15, 0.15),
            white_car: 0,
            yellow_car: 0,
        }
    }
}

/// Ajoute de nouvelles couleurs aux voitures et aux camions (au démarrage).
pub fn extend_palette(mut kit: ResMut<Kit>, mut extra: ResMut<Extra>, mut mats: ResMut<Assets<StandardMaterial>>) {
    let mut f = |r: f32, g: f32, b: f32| mats.add(flat_material(Color::srgb(r, g, b)));
    extra.yellow_car = 2; // la 3e peinture d'origine est jaune
    extra.white_car = kit.m.car.len();
    let more = [
        f(0.96, 0.96, 0.96),
        f(0.95, 0.50, 0.70),
        f(0.55, 0.80, 0.25),
        f(0.16, 0.18, 0.24),
        f(0.20, 0.30, 0.65),
        f(0.45, 0.30, 0.20),
        f(0.85, 0.20, 0.55),
        f(0.55, 0.85, 0.85),
    ];
    kit.m.car.extend(more);
    let cargo = [f(0.85, 0.25, 0.25), f(0.30, 0.65, 0.40), f(0.95, 0.65, 0.20), f(0.35, 0.50, 0.85), f(0.60, 0.40, 0.75)];
    kit.m.cargo.extend(cargo);
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Vehicle {
    Car,
    Truck,
    Bus,
    Sport,
    Pickup,
    Moto,
    Police,
    Taxi,
    Tractor,
    Sheriff,
    /// Un véhicule décrit en pièces (`gen_vehicles.rs`), par son numéro.
    Gen(usize),
    /// Un animal du parc (`gen_worlds::JURA_ANIMALS`) qui traverse la route.
    Beast(usize, usize),
}

impl Vehicle {
    /// Longueur (pour les collisions et l'espacement).
    pub fn len(self) -> f32 {
        match self {
            Vehicle::Car | Vehicle::Police | Vehicle::Taxi => 1.6,
            Vehicle::Sheriff => 1.7,
            Vehicle::Gen(i) => crate::gen_vehicles::GEN[i].len,
            Vehicle::Beast(t, i) => crate::gen_worlds::THEMES[t].animals[i].len,
            Vehicle::Truck => 2.9,
            Vehicle::Bus => 3.6,
            Vehicle::Sport => 1.7,
            Vehicle::Pickup => 2.1,
            Vehicle::Moto => 1.0,
            Vehicle::Tractor => 2.0,
        }
    }
}

/// Un véhicule tiré au sort parmi ceux déjà débloqués (selon les points cumulés).
pub fn pick_vehicle(roll: u32, points: u32) -> Vehicle {
    use crate::progress::{Item, unlocked};
    // TRAVERSEE_VEHICLE=8 : tous les véhicules de la route sont celui-là (essais).
    static FORCED: std::sync::OnceLock<Option<usize>> = std::sync::OnceLock::new();
    if let Some(i) = *FORCED.get_or_init(|| std::env::var("TRAVERSEE_VEHICLE").ok().and_then(|s| s.parse().ok())) {
        return Vehicle::Gen(i.min(crate::gen_vehicles::GEN.len() - 1));
    }
    const BASE: [(Vehicle, u32); 9] = [
        (Vehicle::Car, 38),
        (Vehicle::Truck, 14),
        (Vehicle::Taxi, 6),
        (Vehicle::Moto, 6),
        (Vehicle::Bus, 8),
        (Vehicle::Pickup, 8),
        (Vehicle::Sport, 10),
        (Vehicle::Police, 6),
        (Vehicle::Tractor, 4),
    ];
    let mut open: Vec<(Vehicle, u32)> = BASE.iter().copied().filter(|(v, _)| unlocked(points, Item::Vehicle(*v))).collect();
    if unlocked(points, Item::Vehicle(Vehicle::Sheriff)) {
        open.push((Vehicle::Sheriff, 3));
    }
    // Les véhicules décrits en pièces, plus rares que les classiques
    for (i, g) in crate::gen_vehicles::GEN.iter().enumerate() {
        if unlocked(points, Item::Vehicle(Vehicle::Gen(i))) {
            open.push((Vehicle::Gen(i), if g.class == "bus" { 3 } else { 4 }));
        }
    }
    let total: u32 = open.iter().map(|(_, w)| w).sum();
    let mut r = roll % total;
    for (v, w) in &open {
        if r < *w {
            return *v;
        }
        r -= w;
    }
    Vehicle::Car
}

fn wheel(c: &mut Commands, k: &Kit, parent: Entity, pos: Vec3, radius: f32, width: f32) {
    let axis_z = Quat::from_rotation_x(FRAC_PI_2);
    let d = radius * 2.0;
    part_rot(c, parent, &k.cyl, &k.m.tire, pos, Vec3::new(d, width, d), axis_z);
    let out = pos + Vec3::Z * if pos.z >= 0.0 { 1.0 } else { -1.0 } * (width / 2.0 + 0.005);
    part_rot(c, parent, &k.cyl, &k.m.hub, out, Vec3::new(d * 0.45, 0.02, d * 0.45), axis_z);
}

pub fn spawn_vehicle(c: &mut Commands, k: &Kit, x: &Extra, parent: Entity, v: Vehicle, color: usize) {
    let m = &k.m;
    let paint = &m.car[color % m.car.len()];
    match v {
        Vehicle::Gen(i) => spawn_gen(c, k, x, parent, i, color),
        Vehicle::Beast(t, i) => spawn_animal(c, k, x, parent, t, i),
        Vehicle::Car | Vehicle::Truck => unreachable!("voitures et camions viennent de decor3d"),
        Vehicle::Bus => {
            let body = &x.bus[color % x.bus.len()];
            part(c, parent, &k.cube, &m.dark, Vec3::new(0.0, 0.26, 0.0), Vec3::new(3.5, 0.14, 0.8));
            part(c, parent, &k.cube, body, Vec3::new(0.0, 0.72, 0.0), Vec3::new(3.6, 0.9, 0.9));
            part(c, parent, &k.cube, &x.white, Vec3::new(0.0, 1.2, 0.0), Vec3::new(3.5, 0.06, 0.84));
            for s in [-1.0, 1.0] {
                part(c, parent, &k.cube, &m.glass, Vec3::new(-0.1, 0.92, 0.455 * s), Vec3::new(3.1, 0.3, 0.02));
                part(c, parent, &k.cube, &x.white, Vec3::new(0.0, 0.52, 0.455 * s), Vec3::new(3.6, 0.07, 0.02));
                part(c, parent, &k.cube, &m.headlight, Vec3::new(1.805, 0.42, 0.3 * s), Vec3::new(0.03, 0.1, 0.16));
                part(c, parent, &k.cube, &m.taillight, Vec3::new(-1.805, 0.42, 0.3 * s), Vec3::new(0.03, 0.1, 0.16));
                for wx in [-1.2, 1.15] {
                    wheel(c, k, parent, Vec3::new(wx, 0.21, 0.4 * s), 0.21, 0.14);
                }
            }
            part(c, parent, &k.cube, &m.glass, Vec3::new(1.805, 0.9, 0.0), Vec3::new(0.02, 0.34, 0.76));
        }
        Vehicle::Sport => {
            part(c, parent, &k.cube, &m.dark, Vec3::new(0.0, 0.18, 0.0), Vec3::new(1.7, 0.1, 0.78));
            part(c, parent, &k.cube, paint, Vec3::new(0.0, 0.3, 0.0), Vec3::new(1.7, 0.2, 0.82));
            part(c, parent, &k.cube, &m.glass, Vec3::new(-0.12, 0.5, 0.0), Vec3::new(0.7, 0.2, 0.66));
            part(c, parent, &k.cube, paint, Vec3::new(-0.12, 0.61, 0.0), Vec3::new(0.64, 0.03, 0.6));
            part(c, parent, &k.cube, &x.white, Vec3::new(0.05, 0.405, 0.0), Vec3::new(1.69, 0.01, 0.22));
            part(c, parent, &k.cube, &m.dark, Vec3::new(-0.78, 0.55, 0.0), Vec3::new(0.12, 0.03, 0.8));
            for s in [-1.0, 1.0] {
                part(c, parent, &k.cube, &m.dark, Vec3::new(-0.78, 0.46, 0.3 * s), Vec3::new(0.05, 0.16, 0.04));
                part(c, parent, &k.cube, &m.headlight, Vec3::new(0.855, 0.32, 0.28 * s), Vec3::new(0.03, 0.06, 0.16));
                part(c, parent, &k.cube, &m.taillight, Vec3::new(-0.855, 0.32, 0.28 * s), Vec3::new(0.03, 0.06, 0.16));
                for wx in [-0.55, 0.55] {
                    wheel(c, k, parent, Vec3::new(wx, 0.16, 0.38 * s), 0.17, 0.16);
                }
            }
        }
        Vehicle::Pickup => {
            part(c, parent, &k.cube, &m.dark, Vec3::new(0.0, 0.26, 0.0), Vec3::new(2.1, 0.12, 0.76));
            part(c, parent, &k.cube, paint, Vec3::new(0.82, 0.42, 0.0), Vec3::new(0.46, 0.3, 0.8));
            part(c, parent, &k.cube, paint, Vec3::new(0.3, 0.64, 0.0), Vec3::new(0.64, 0.5, 0.8));
            part(c, parent, &k.cube, &m.glass, Vec3::new(0.3, 0.72, 0.0), Vec3::new(0.66, 0.24, 0.72));
            part(c, parent, &k.cube, &m.dark, Vec3::new(-0.55, 0.38, 0.0), Vec3::new(0.95, 0.05, 0.76));
            for s in [-1.0, 1.0] {
                part(c, parent, &k.cube, paint, Vec3::new(-0.55, 0.5, 0.39 * s), Vec3::new(0.95, 0.24, 0.04));
                part(c, parent, &k.cube, &m.headlight, Vec3::new(1.055, 0.45, 0.28 * s), Vec3::new(0.03, 0.09, 0.15));
                part(c, parent, &k.cube, &m.taillight, Vec3::new(-1.045, 0.45, 0.3 * s), Vec3::new(0.03, 0.09, 0.12));
                for wx in [0.7, -0.65] {
                    wheel(c, k, parent, Vec3::new(wx, 0.2, 0.37 * s), 0.2, 0.14);
                }
            }
            part(c, parent, &k.cube, paint, Vec3::new(-1.03, 0.5, 0.0), Vec3::new(0.04, 0.24, 0.78));
        }
        Vehicle::Moto => {
            part(c, parent, &k.cube, paint, Vec3::new(0.0, 0.34, 0.0), Vec3::new(0.7, 0.18, 0.2));
            part(c, parent, &k.cube, &m.dark, Vec3::new(-0.12, 0.46, 0.0), Vec3::new(0.4, 0.07, 0.16));
            part(c, parent, &k.cube, &m.dark, Vec3::new(0.3, 0.64, 0.0), Vec3::new(0.05, 0.05, 0.42));
            part_rot(c, parent, &k.cube, &m.dark, Vec3::new(0.36, 0.4, 0.0), Vec3::new(0.05, 0.42, 0.05), Quat::from_rotation_z(0.3));
            for wx in [-0.38, 0.4] {
                wheel(c, k, parent, Vec3::new(wx, 0.19, 0.0), 0.19, 0.1);
            }
            part(c, parent, &k.cube, &m.headlight, Vec3::new(0.47, 0.5, 0.0), Vec3::new(0.04, 0.1, 0.12));
            // Le motard : casque et blouson
            part(c, parent, &k.cube, &x.red_light, Vec3::new(-0.08, 0.7, 0.0), Vec3::new(0.22, 0.32, 0.26));
            part(c, parent, &k.ball, &x.white, Vec3::new(0.0, 0.98, 0.0), Vec3::splat(0.28));
            part(c, parent, &k.cube, &m.glass, Vec3::new(0.12, 0.98, 0.0), Vec3::new(0.06, 0.1, 0.2));
        }
        Vehicle::Police => {
            spawn_car(c, k, parent, x.white_car);
            part(c, parent, &k.cube, &x.black_stripe, Vec3::new(0.0, 0.4, 0.0), Vec3::new(1.56, 0.1, 0.8));
            part(c, parent, &k.cube, &m.dark, Vec3::new(-0.12, 0.94, 0.0), Vec3::new(0.3, 0.05, 0.5));
            part(c, parent, &k.cube, &x.red_light, Vec3::new(-0.12, 1.0, -0.15), Vec3::new(0.24, 0.1, 0.18));
            part(c, parent, &k.cube, &x.blue_light, Vec3::new(-0.12, 1.0, 0.15), Vec3::new(0.24, 0.1, 0.18));
        }
        Vehicle::Taxi => {
            spawn_car(c, k, parent, x.yellow_car);
            part(c, parent, &k.cube, &x.taxi, Vec3::new(-0.12, 0.88, 0.0), Vec3::new(0.86, 0.06, 0.74));
            part(c, parent, &k.cube, &x.white, Vec3::new(-0.12, 1.0, 0.0), Vec3::new(0.3, 0.14, 0.2));
            for dz in [-0.12, 0.12] {
                part(c, parent, &k.cube, &x.black_stripe, Vec3::new(0.2, 0.4, dz), Vec3::new(0.16, 0.06, 0.8));
                part(c, parent, &k.cube, &x.black_stripe, Vec3::new(0.2 - 0.32, 0.4, -dz), Vec3::new(0.16, 0.06, 0.8));
            }
        }
        Vehicle::Sheriff => {
            spawn_car(c, k, parent, 4); // orange
            part(c, parent, &k.cube, &x.white, Vec3::new(-0.12, 0.915, 0.0), Vec3::new(0.84, 0.03, 0.72));
            part(c, parent, &k.cube, &x.white, Vec3::new(0.0, 0.445, 0.0), Vec3::new(1.56, 0.04, 0.8));
            for s in [-1.0, 1.0] {
                part(c, parent, &k.cyl, &x.white, Vec3::new(-0.05, 0.43, 0.395 * s), Vec3::new(0.3, 0.02, 0.3));
                part(c, parent, &k.cube, &x.black_stripe, Vec3::new(-0.05, 0.43, 0.406 * s), Vec3::new(0.05, 0.2, 0.01));
                part(c, parent, &k.cube, &x.white, Vec3::new(0.84, 0.3, 0.25 * s), Vec3::new(0.04, 0.22, 0.04));
            }
            part(c, parent, &k.cube, &x.white, Vec3::new(0.86, 0.3, 0.0), Vec3::new(0.04, 0.04, 0.5));
            part(c, parent, &k.cube, &x.red_light, Vec3::new(-0.12, 0.97, 0.0), Vec3::new(0.14, 0.08, 0.3));
        }
        Vehicle::Tractor => {
            part(c, parent, &k.cube, &x.green, Vec3::new(0.62, 0.52, 0.0), Vec3::new(0.85, 0.46, 0.52));
            part(c, parent, &k.cube, &m.dark, Vec3::new(0.98, 0.5, 0.0), Vec3::new(0.04, 0.3, 0.4));
            part(c, parent, &k.cube, &x.green, Vec3::new(-0.2, 0.55, 0.0), Vec3::new(0.7, 0.3, 0.6));
            part(c, parent, &k.cube, &m.glass, Vec3::new(-0.25, 0.95, 0.0), Vec3::new(0.5, 0.5, 0.58));
            part(c, parent, &k.cube, &m.dark, Vec3::new(-0.25, 1.22, 0.0), Vec3::new(0.62, 0.05, 0.7));
            part(c, parent, &k.cyl, &m.dark, Vec3::new(0.85, 0.92, 0.12), Vec3::new(0.07, 0.5, 0.07));
            for s in [-1.0, 1.0] {
                wheel(c, k, parent, Vec3::new(-0.4, 0.36, 0.42 * s), 0.36, 0.2);
                wheel(c, k, parent, Vec3::new(0.65, 0.2, 0.36 * s), 0.2, 0.12);
                part(c, parent, &k.cube, &m.headlight, Vec3::new(1.0, 0.6, 0.18 * s), Vec3::new(0.03, 0.08, 0.1));
            }
        }
    }
}

// ---- Décor de l'herbe ------------------------------------------------------

#[derive(Clone, Copy)]
pub enum Deco {
    Flower,
    Flowers,
    Mushroom,
    Tuft,
    Stump,
}

/// Fleurs, champignons, touffes : purement décoratifs (le crapaud passe dessus).
/// Les souches, elles, bloquent comme un arbre.
pub fn spawn_deco(c: &mut Commands, k: &Kit, x: &Extra, parent: Entity, deco: Deco, pos: Vec3, variant: usize) {
    let tilt = |i: usize| Quat::from_rotation_y(variant as f32 * 0.9 + i as f32 * 2.1);
    match deco {
        Deco::Flower => flower(c, k, x, parent, pos, variant, 1.0),
        Deco::Flowers => {
            for (i, (dx, dz, s)) in [(-0.22, 0.12, 0.9), (0.2, -0.1, 1.05), (0.0, 0.28, 0.8), (0.05, -0.3, 0.95)].into_iter().enumerate() {
                flower(c, k, x, parent, pos + Vec3::new(dx, 0.0, dz), variant + i, s);
            }
        }
        Deco::Mushroom => {
            for (i, (dx, dz, s)) in [(0.0, 0.0, 1.0), (0.25, 0.15, 0.65)].into_iter().enumerate() {
                let p = pos + Vec3::new(dx, 0.0, dz);
                part(c, parent, &k.cyl, &x.cream, p + Vec3::Y * 0.1 * s, Vec3::new(0.12, 0.2, 0.12) * s);
                part(c, parent, &k.blob, &x.cap, p + Vec3::Y * 0.22 * s, Vec3::new(0.34, 0.24, 0.34) * s);
                part_rot(c, parent, &k.cube, &x.white, p + Vec3::new(0.06, 0.32, 0.03) * s, Vec3::splat(0.05) * s, tilt(i));
                part_rot(c, parent, &k.cube, &x.white, p + Vec3::new(-0.07, 0.3, -0.05) * s, Vec3::splat(0.045) * s, tilt(i));
            }
        }
        Deco::Tuft => {
            for i in 0..4 {
                let a = i as f32 * FRAC_PI_2 + variant as f32;
                let off = Vec3::new(a.cos(), 0.0, a.sin()) * 0.07;
                part_rot(c, parent, &k.cone, &x.tuft, pos + off + Vec3::Y * 0.14, Vec3::new(0.09, 0.3, 0.09), Quat::from_rotation_z(a.cos() * 0.25) * Quat::from_rotation_x(a.sin() * 0.25));
            }
        }
        Deco::Stump => {
            part(c, parent, &k.cyl, &x.bark, pos + Vec3::Y * 0.2, Vec3::new(0.55, 0.4, 0.55));
            part(c, parent, &k.cyl, &x.bark_top, pos + Vec3::Y * 0.405, Vec3::new(0.45, 0.02, 0.45));
            part(c, parent, &k.cyl, &x.bark, pos + Vec3::new(0.3, 0.05, 0.1), Vec3::new(0.2, 0.1, 0.2));
            part(c, parent, &k.cap_mushroom(), &x.cap, pos + Vec3::new(-0.28, 0.1, 0.14), Vec3::new(0.2, 0.12, 0.2));
        }
    }
}

trait CapMesh {
    fn cap_mushroom(&self) -> Handle<Mesh>;
}
impl CapMesh for Kit {
    fn cap_mushroom(&self) -> Handle<Mesh> {
        self.blob.clone()
    }
}

fn flower(c: &mut Commands, k: &Kit, x: &Extra, parent: Entity, pos: Vec3, variant: usize, s: f32) {
    let petals = &x.flowers[variant % x.flowers.len()];
    part(c, parent, &k.cyl, &x.stem, pos + Vec3::Y * 0.13 * s, Vec3::new(0.035, 0.26, 0.035) * s);
    for i in 0..5 {
        let a = i as f32 * 2.0 * PI / 5.0;
        part(c, parent, &k.blob, petals, pos + Vec3::new(a.cos() * 0.08, 0.3, a.sin() * 0.08) * s, Vec3::splat(0.1) * s);
    }
    part(c, parent, &k.blob, &x.petal_center, pos + Vec3::Y * 0.31 * s, Vec3::splat(0.09) * s);
}

// ---- Mouches ----------------------------------------------------------------

/// Une mouche à gober : posée sur une case d'herbe, elle vole sur place.
#[derive(Component)]
pub struct Fly {
    pub row: i32,
    pub col: i32,
    pub gold: bool,
    pub phase: f32,
}

impl Fly {
    pub fn points(&self) -> u32 {
        if self.gold { 20 } else { 5 }
    }
}

#[derive(Component)]
pub struct Wing {
    side: f32,
}

pub fn spawn_fly(c: &mut Commands, k: &Kit, x: &Extra, lane: Entity, row: i32, col: i32, gold: bool, phase: f32) {
    let body = if gold { &x.fly_gold } else { &x.fly_body };
    let fly = c
        .spawn((
            Transform::from_xyz(col as f32, 0.6, 0.0),
            Visibility::default(),
            Fly { row, col, gold, phase },
            ChildOf(lane),
        ))
        .id();
    let big = if gold { 1.9 } else { 1.5 };
    part(c, fly, &k.ball, body, Vec3::ZERO, Vec3::new(0.3, 0.22, 0.4) * big);
    part(c, fly, &k.ball, body, Vec3::new(0.0, 0.03, -0.2) * big, Vec3::splat(0.2) * big);
    for s in [-1.0, 1.0] {
        part(c, fly, &k.ball, &x.fly_eye, Vec3::new(0.07 * s, 0.08, -0.3) * big, Vec3::splat(0.1) * big);
        let wing = c
            .spawn((
                Transform::from_xyz(0.1 * s * big, 0.1 * big, 0.02),
                Visibility::default(),
                Wing { side: s },
                ChildOf(fly),
            ))
            .id();
        part(c, wing, &k.cube, &x.fly_wing, Vec3::new(0.16 * s, 0.0, 0.0), Vec3::new(0.3, 0.02, 0.18) * big);
    }
    // Petit éclat doré autour de la mouche en or, pour la repérer de loin.
    if gold {
        part(c, fly, &k.ball, &x.fly_gold, Vec3::new(0.0, 0.5, 0.0), Vec3::splat(0.07));
    }
}

/// Les mouches bourdonnent : elles tournent doucement et battent des ailes.
pub fn animate_flies(time: Res<Time>, mut flies: Query<(&Fly, &mut Transform), Without<Wing>>, mut wings: Query<(&Wing, &mut Transform), Without<Fly>>) {
    let t = time.elapsed_secs();
    for (fly, mut tf) in &mut flies {
        let a = t * 2.2 + fly.phase;
        tf.translation.x = fly.col as f32 + a.cos() * 0.18;
        tf.translation.z = a.sin() * 0.18;
        tf.translation.y = 0.62 + (t * 3.1 + fly.phase).sin() * 0.07;
        tf.rotation = Quat::from_rotation_y(-a + FRAC_PI_2 * 0.0);
    }
    for (wing, mut tf) in &mut wings {
        tf.rotation = Quat::from_rotation_z(wing.side * (t * 70.0).sin() * 0.6);
    }
}

// ---- Décors par monde ------------------------------------------------------

/// Les mondes se suivent tous les 60 rangs : prairie, désert, neige, automne.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Biome {
    Prairie,
    Desert,
    Snow,
    Autumn,
    /// Le Parc Jurassic'Croa (décor décrit en pièces, `gen_worlds.rs`).
    Jurassic,
    /// Nippon Kaboom.
    Nippon,
    /// Cosmos Croâ : la Lune, ses vaisseaux et ses extraterrestres.
    Space,
    /// Far West Croâ : diligences, chevaux, bisons et vieux train.
    FarWest,
    /// Halloween Croâ : cimetière, créatures, train fantôme.
    Halloween,
    /// Néo-Croâ : ville futuriste, robots, monorail.
    Cyber,
    /// Royaume de Croâ : Moyen Âge et dragon.
    Medieval,
    /// Baie des Pirates : plage, ponton, kraken et perroquet.
    Pirate,
}

pub const BIOME_LEN: i32 = 60;

/// Cercueils et citrouilles flottantes d'Halloween.
pub const HALLOWEEN_COLORS: [[u8; 3]; 4] = [[84, 52, 34], [230, 190, 60], [255, 128, 30], [60, 120, 50]];

/// Bois, cordages et fer des tonneaux et radeaux de la baie.
pub const PIRATE_COLORS: [[u8; 3]; 4] = [[160, 105, 58], [58, 42, 26], [220, 200, 150], [196, 57, 43]];

/// Les couleurs des conteneurs des cargos de l'espace.
pub const SPACE_CARGO: [[u8; 3]; 4] = [[232, 67, 58], [255, 138, 42], [55, 163, 255], [244, 246, 250]];

/// La peau verte des héros décrits en pièces (matière `peinture`).
pub const FROG_GREEN: [u8; 3] = [90, 170, 70];

/// Les couleurs d'un monde à thème (sol, eau, route…).
pub struct WorldColors {
    pub grass_a: [u8; 3],
    pub grass_b: [u8; 3],
    pub water: [u8; 3],
    pub road: [u8; 3],
    pub marking: [u8; 3],
    pub log: [u8; 3],
    pub sky: [u8; 3],
}

/// Une voiture du vieux train à vapeur (locomotive, tender, wagons) ; `len` en cases.
pub struct TrainCar {
    pub model: Model,
    pub len: f32,
}

/// Le vieux train du monde : locomotive, tender, puis des wagons.
pub fn spawn_old_train(c: &mut Commands, k: &Kit, x: &Extra, parent: Entity, t: &Theme, wagons: usize) -> f32 {
    let mut order = vec![0usize, 1];
    order.extend((0..wagons).map(|i| 2 + i % 2));
    let gap = 0.15;
    let total: f32 = order.iter().map(|i| t.train[*i].len).sum::<f32>() + gap * (order.len() - 1) as f32;
    let mut front = total / 2.0;
    for i in order {
        let car = &t.train[i];
        let holder = c.spawn((Transform::from_xyz(front - car.len / 2.0, 0.0, 0.0), Visibility::default(), ChildOf(parent))).id();
        spawn_pivoted(c, k, x, holder, &car.model, i);
        front -= car.len + gap;
    }
    total
}

/// Un monde décrit en pièces (`gen_worlds.rs`) : décors, animaux, véhicules, héros et son chapeau.
pub struct Theme {
    /// Le vieux train du monde (vide : le train habituel).
    pub train: &'static [TrainCar],
    /// Sur la route : seulement ses propres véhicules et animaux (pas de voitures ordinaires).
    pub only_own: bool,
    pub blocks: &'static [Model],
    pub sides: &'static [Model],
    pub deco: &'static [Model],
    pub animals: &'static [Animal],
    /// Quel gros obstacle poser, selon le tirage (indices dans `blocks`).
    pub block_mix: &'static [usize],
    /// Décor des bords : indices dans `sides`, ou 100 + indice d'un obstacle.
    pub side_mix: &'static [usize],
    /// Numéros (dans `gen_vehicles::GEN`) des véhicules qui roulent dans ce monde.
    /// Chance (en %) de poser un décor sur chaque case du bord.
    pub side_pct: u32,
    pub vehicles: &'static [usize],
    pub colors: WorldColors,
    pub points: u32,
    pub name: fn() -> &'static str,
    pub banner: fn() -> &'static str,
    pub animal_name: fn(usize) -> &'static str,
    pub hero_name: fn() -> &'static str,
    pub hero: Model,
    pub hat: Model,
}

/// Le monde où commence la partie (choisi au garage, pour jouer ou tester un univers précis).
pub static START_WORLD: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

impl Biome {
    pub const ALL: [Biome; 12] = [Biome::Prairie, Biome::Desert, Biome::Snow, Biome::Autumn, Biome::Jurassic, Biome::Nippon, Biome::Space, Biome::FarWest, Biome::Halloween, Biome::Cyber, Biome::Medieval, Biome::Pirate];

    /// Le monde décrit en pièces, pour les mondes à thème.
    pub fn theme(self) -> Option<&'static Theme> {
        self.theme_index().map(|i| crate::gen_worlds::THEMES[i])
    }

    pub fn theme_index(self) -> Option<usize> {
        match self {
            Biome::Jurassic => Some(0),
            Biome::Nippon => Some(1),
            Biome::Space => Some(3),
            Biome::FarWest => Some(2),
            Biome::Halloween => Some(4),
            Biome::Cyber => Some(5),
            Biome::Medieval => Some(6),
            Biome::Pirate => Some(7),
            _ => None,
        }
    }

    pub fn index(self) -> i32 {
        Biome::ALL.iter().position(|b| *b == self).unwrap_or(0) as i32
    }
}

impl Biome {
    /// Le monde de ce rang ; un monde pas encore débloqué reste de la prairie.
    pub fn of(row: i32, points: u32) -> Self {
        // TRAVERSEE_BIOME=1..3 : voir un monde tout de suite (essais et captures).
        static ENV: std::sync::OnceLock<i32> = std::sync::OnceLock::new();
        let start = *ENV.get_or_init(|| std::env::var("TRAVERSEE_BIOME").ok().and_then(|s| s.parse().ok()).unwrap_or(0)) + START_WORLD.load(std::sync::atomic::Ordering::Relaxed);
        let biome = match (row.max(0) / BIOME_LEN + start) % 12 {
            0 => Biome::Prairie,
            1 => Biome::Desert,
            2 => Biome::Snow,
            3 => Biome::Autumn,
            4 => Biome::Jurassic,
            5 => Biome::Nippon,
            6 => Biome::Space,
            7 => Biome::FarWest,
            8 => Biome::Halloween,
            9 => Biome::Cyber,
            10 => Biome::Medieval,
            _ => Biome::Pirate,
        };
        if biome != Biome::Prairie && !crate::progress::unlocked(points, crate::progress::Item::World(biome)) {
            Biome::Prairie
        } else {
            biome
        }
    }

    pub fn name(self) -> &'static str {
        use game_common::tx;
        match self {
            Biome::Prairie => tx!("La prairie", "The meadow", "Die Wiese", "Il prato", "La pradera"),
            Biome::Desert => tx!("Le désert", "The desert", "Die Wüste", "Il deserto", "El desierto"),
            Biome::Snow => tx!("La neige", "The snow", "Der Schnee", "La neve", "La nieve"),
            Biome::Autumn => tx!("L'automne", "Autumn", "Der Herbst", "L'autunno", "El otoño"),
            Biome::Jurassic | Biome::Nippon | Biome::Space | Biome::FarWest | Biome::Halloween | Biome::Cyber | Biome::Medieval | Biome::Pirate => return (self.theme().unwrap().name)(),
        }
        .t()
    }
}

/// Matériaux des mondes autres que la prairie.
struct BiomeMats {
    sand: [Handle<StandardMaterial>; 2],
    sand_side: Handle<StandardMaterial>,
    snow: [Handle<StandardMaterial>; 2],
    snow_side: Handle<StandardMaterial>,
    autumn: [Handle<StandardMaterial>; 2],
    autumn_side: Handle<StandardMaterial>,
    ice: Handle<StandardMaterial>,
    ice_side: Handle<StandardMaterial>,
    ice_block: Handle<StandardMaterial>,
    ice_top: Handle<StandardMaterial>,
    ice_rock: Handle<StandardMaterial>,
    sandstone: Handle<StandardMaterial>,
    cactus: Handle<StandardMaterial>,
    dry: Handle<StandardMaterial>,
    dead_wood: Handle<StandardMaterial>,
    snow_white: Handle<StandardMaterial>,
    carrot: Handle<StandardMaterial>,
    scarf: Handle<StandardMaterial>,
    leaves: [Handle<StandardMaterial>; 3],
    pumpkin: Handle<StandardMaterial>,
    /// Sol et eau de chaque monde à thème.
    themed: Vec<ThemeMats>,
}

struct ThemeMats {
    road: Handle<StandardMaterial>,
    grass: [Handle<StandardMaterial>; 2],
    side: Handle<StandardMaterial>,
    water: Handle<StandardMaterial>,
    water_side: Handle<StandardMaterial>,
}

impl BiomeMats {
    fn new(mats: &mut Assets<StandardMaterial>) -> Self {
        let mut f = |r: f32, g: f32, b: f32| mats.add(flat_material(Color::srgb(r, g, b)));
        let mut c = |rgb: [u8; 3], k: f32| f(rgb[0] as f32 / 255.0 * k, rgb[1] as f32 / 255.0 * k, rgb[2] as f32 / 255.0 * k);
        let themed = crate::gen_worlds::THEMES
            .iter()
            .map(|t| ThemeMats {
                road: c(t.colors.road, 1.0),
                grass: [c(t.colors.grass_a, 1.0), c(t.colors.grass_b, 1.0)],
                side: c(t.colors.grass_a, 0.8),
                water: c(t.colors.water, 1.0),
                water_side: c(t.colors.water, 0.8),
            })
            .collect();
        Self {
            themed,
            sand: [f(0.94, 0.83, 0.56), f(0.90, 0.79, 0.51)],
            sand_side: f(0.82, 0.69, 0.43),
            snow: [f(0.97, 0.98, 1.0), f(0.92, 0.95, 0.98)],
            snow_side: f(0.82, 0.88, 0.95),
            autumn: [f(0.80, 0.72, 0.36), f(0.75, 0.67, 0.32)],
            autumn_side: f(0.66, 0.56, 0.28),
            ice: f(0.74, 0.90, 0.98),
            ice_side: f(0.66, 0.82, 0.92),
            ice_block: f(0.88, 0.96, 1.0),
            ice_top: f(0.98, 1.0, 1.0),
            ice_rock: f(0.68, 0.84, 0.94),
            sandstone: f(0.80, 0.56, 0.36),
            cactus: f(0.30, 0.65, 0.35),
            dry: f(0.72, 0.64, 0.34),
            dead_wood: f(0.45, 0.36, 0.30),
            snow_white: f(0.97, 0.98, 1.0),
            carrot: f(0.98, 0.55, 0.15),
            scarf: f(0.85, 0.2, 0.25),
            leaves: [f(0.95, 0.55, 0.15), f(0.80, 0.25, 0.15), f(0.96, 0.80, 0.22)],
            pumpkin: f(0.96, 0.52, 0.12),
        }
    }
}

impl Extra {
    /// La piste du monde (terre battue), à la place de la route goudronnée.
    pub fn dirt_road(&self, biome: Biome) -> Option<&Handle<StandardMaterial>> {
        matches!(biome, Biome::FarWest | Biome::Halloween | Biome::Cyber | Biome::Medieval | Biome::Pirate).then(|| &self.biome.themed[biome.theme_index().unwrap()].road)
    }

    /// Sol de l'herbe (deux teintes en alternance) et bords, selon le monde.
    pub fn ground<'a>(&'a self, kit: &'a Kit, biome: Biome, row: i32) -> (&'a Handle<StandardMaterial>, &'a Handle<StandardMaterial>) {
        let i = row.rem_euclid(2) as usize;
        match biome {
            Biome::Prairie => (&kit.m.grass[i], &kit.m.grass_side),
            Biome::Desert => (&self.biome.sand[i], &self.biome.sand_side),
            Biome::Snow => (&self.biome.snow[i], &self.biome.snow_side),
            Biome::Autumn => (&self.biome.autumn[i], &self.biome.autumn_side),
            Biome::Jurassic | Biome::Nippon | Biome::Space | Biome::FarWest | Biome::Halloween | Biome::Cyber | Biome::Medieval | Biome::Pirate => {
                let t = &self.biome.themed[biome.theme_index().unwrap()];
                (&t.grass[i], &t.side)
            }
        }
    }

    /// Eau de la rivière (gelée dans la neige) et ses bords.
    pub fn water<'a>(&'a self, kit: &'a Kit, biome: Biome) -> (&'a Handle<StandardMaterial>, &'a Handle<StandardMaterial>) {
        match biome {
            Biome::Snow => (&self.biome.ice, &self.biome.ice_side),
            Biome::Jurassic | Biome::Nippon | Biome::Space | Biome::FarWest | Biome::Halloween | Biome::Cyber | Biome::Medieval | Biome::Pirate => {
                let t = &self.biome.themed[biome.theme_index().unwrap()];
                (&t.water, &t.water_side)
            }
            _ => (&kit.m.water, &kit.m.water_side),
        }
    }
}

/// Un morceau de banquise qui dérive, à la place du rondin.
pub fn spawn_floe(c: &mut Commands, k: &Kit, x: &Extra, parent: Entity, len: f32) {
    part(c, parent, &k.cube, &x.biome.ice_block, Vec3::new(0.0, -0.06, 0.0), Vec3::new(len, 0.44, 0.84));
    part(c, parent, &k.cube, &x.biome.ice_top, Vec3::new(0.0, 0.17, 0.0), Vec3::new(len - 0.18, 0.03, 0.68));
}

/// Un îlot de glace immobile, à la place du nénuphar.
pub fn spawn_ice_pad(c: &mut Commands, k: &Kit, x: &Extra, parent: Entity) {
    part(c, parent, &k.cube, &x.biome.ice_block, Vec3::new(0.0, -0.17, 0.0), Vec3::new(0.85, 0.22, 0.85));
    part(c, parent, &k.cube, &x.biome.ice_top, Vec3::new(0.0, -0.055, 0.0), Vec3::new(0.7, 0.02, 0.7));
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Scenery {
    /// Gros obstacle sur la zone jouable.
    Block,
    /// Décor des bords (on ne passe pas dessus de toute façon).
    Side,
    /// Petit décor sans collision.
    Deco,
}

/// Cactus, bonhommes de neige, citrouilles… selon le monde. `roll` : 0 à 9.
#[allow(clippy::too_many_arguments)]
pub fn spawn_scenery(c: &mut Commands, k: &Kit, x: &Extra, parent: Entity, biome: Biome, kind: Scenery, pos: Vec3, roll: u32, variant: usize) {
    let b = &x.biome;
    let m = &k.m;
    match (biome, kind) {
        (Biome::Prairie, Scenery::Deco) | (Biome::Prairie, Scenery::Side) | (Biome::Prairie, Scenery::Block) => unreachable!("la prairie reste dans lanes.rs"),
        // ---- Désert
        (Biome::Desert, Scenery::Block) | (Biome::Desert, Scenery::Side) if roll < 6 => {
            part(c, parent, &k.cyl, &b.cactus, pos + Vec3::Y * 0.42, Vec3::new(0.26, 0.84, 0.26));
            part(c, parent, &k.cube, &b.cactus, pos + Vec3::new(0.19, 0.44, 0.0), Vec3::new(0.16, 0.1, 0.12));
            part(c, parent, &k.cyl, &b.cactus, pos + Vec3::new(0.28, 0.58, 0.0), Vec3::new(0.14, 0.36, 0.14));
            if variant % 2 == 0 {
                part(c, parent, &k.cube, &b.cactus, pos + Vec3::new(-0.19, 0.3, 0.0), Vec3::new(0.16, 0.1, 0.12));
                part(c, parent, &k.cyl, &b.cactus, pos + Vec3::new(-0.28, 0.42, 0.0), Vec3::new(0.14, 0.3, 0.14));
            }
            part(c, parent, &k.blob, &m.flower, pos + Vec3::Y * 0.86, Vec3::splat(0.12));
        }
        (Biome::Desert, Scenery::Block) | (Biome::Desert, Scenery::Side) if roll < 9 => {
            let rot = Quat::from_rotation_y(variant as f32 * 1.1);
            part_rot(c, parent, &k.rock, &b.sandstone, pos + Vec3::Y * 0.2, Vec3::new(0.85, 0.6, 0.7), rot);
            part_rot(c, parent, &k.rock, &b.sandstone, pos + Vec3::new(0.3, 0.1, 0.1), Vec3::new(0.4, 0.3, 0.35), rot);
        }
        (Biome::Desert, Scenery::Block) | (Biome::Desert, Scenery::Side) => {
            // Arbre mort
            part(c, parent, &k.cyl, &b.dead_wood, pos + Vec3::Y * 0.45, Vec3::new(0.14, 0.9, 0.14));
            part_rot(c, parent, &k.cube, &b.dead_wood, pos + Vec3::new(0.18, 0.7, 0.0), Vec3::new(0.35, 0.07, 0.07), Quat::from_rotation_z(0.6));
            part_rot(c, parent, &k.cube, &b.dead_wood, pos + Vec3::new(-0.15, 0.55, 0.0), Vec3::new(0.3, 0.06, 0.06), Quat::from_rotation_z(-0.7));
        }
        (Biome::Desert, Scenery::Deco) => {
            for i in 0..5 {
                let a = i as f32 * 1.3 + variant as f32;
                let off = Vec3::new(a.cos(), 0.0, a.sin()) * 0.1;
                part_rot(c, parent, &k.cone, &b.dry, pos + off + Vec3::Y * 0.13, Vec3::new(0.08, 0.28, 0.08), Quat::from_rotation_z(a.cos() * 0.3));
            }
        }
        // ---- Neige
        (Biome::Snow, Scenery::Block) | (Biome::Snow, Scenery::Side) if roll < 4 || roll == 9 => {
            part(c, parent, &k.cyl, &m.trunk, pos + Vec3::Y * 0.2, Vec3::new(0.16, 0.4, 0.16));
            part(c, parent, &k.cone, &m.pine, pos + Vec3::Y * 0.8, Vec3::new(0.8, 0.95, 0.8));
            part(c, parent, &k.cone, &b.snow_white, pos + Vec3::Y * 1.0, Vec3::new(0.52, 0.55, 0.52));
            part(c, parent, &k.cone, &m.pine, pos + Vec3::Y * 1.2, Vec3::new(0.58, 0.7, 0.58));
            part(c, parent, &k.cone, &b.snow_white, pos + Vec3::Y * 1.36, Vec3::new(0.34, 0.34, 0.34));
        }
        (Biome::Snow, Scenery::Block) | (Biome::Snow, Scenery::Side) if roll < 6 => {
            // Bonhomme de neige
            part(c, parent, &k.ball, &b.snow_white, pos + Vec3::Y * 0.27, Vec3::splat(0.56));
            part(c, parent, &k.ball, &b.snow_white, pos + Vec3::Y * 0.7, Vec3::splat(0.42));
            part(c, parent, &k.ball, &b.snow_white, pos + Vec3::Y * 1.02, Vec3::splat(0.32));
            part_rot(c, parent, &k.cone, &b.carrot, pos + Vec3::new(0.0, 1.02, -0.19), Vec3::new(0.07, 0.2, 0.07), Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2));
            for s in [-1.0, 1.0] {
                part(c, parent, &k.ball, &m.tire, pos + Vec3::new(0.07 * s, 1.08, -0.14), Vec3::splat(0.05));
                part_rot(c, parent, &k.cube, &m.trunk, pos + Vec3::new(0.3 * s, 0.76, 0.0), Vec3::new(0.34, 0.04, 0.04), Quat::from_rotation_z(0.5 * s));
            }
            part(c, parent, &k.cyl, &m.tire, pos + Vec3::Y * 1.2, Vec3::new(0.2, 0.16, 0.2));
            part(c, parent, &k.cyl, &m.tire, pos + Vec3::Y * 1.17, Vec3::new(0.3, 0.02, 0.3));
            part(c, parent, &k.cube, &b.scarf, pos + Vec3::new(0.0, 0.88, 0.0), Vec3::new(0.34, 0.06, 0.34));
        }
        (Biome::Snow, Scenery::Block) | (Biome::Snow, Scenery::Side) => {
            let rot = Quat::from_rotation_y(variant as f32 * 1.3);
            part_rot(c, parent, &k.rock, &b.ice_rock, pos + Vec3::Y * 0.2, Vec3::new(0.75, 0.55, 0.65), rot);
            part_rot(c, parent, &k.rock, &b.snow_white, pos + Vec3::Y * 0.4, Vec3::new(0.5, 0.2, 0.42), rot);
        }
        (Biome::Snow, Scenery::Deco) => {
            for (dx, dz, sz) in [(-0.12, 0.05, 0.22), (0.1, -0.08, 0.3), (0.12, 0.14, 0.16)] {
                part(c, parent, &k.blob, &b.snow_white, pos + Vec3::new(dx, 0.07, dz), Vec3::new(sz, sz * 0.5, sz));
            }
            if variant % 3 == 0 {
                part(c, parent, &k.cone, &b.ice_rock, pos + Vec3::new(0.0, 0.18, 0.0), Vec3::new(0.12, 0.36, 0.12));
                part(c, parent, &k.cone, &b.ice_rock, pos + Vec3::new(0.13, 0.12, 0.05), Vec3::new(0.09, 0.24, 0.09));
            }
        }
        // ---- Mondes à thème : modèles décrits en pièces
        (Biome::Jurassic | Biome::Nippon | Biome::Space | Biome::FarWest | Biome::Halloween | Biome::Cyber | Biome::Medieval | Biome::Pirate, kind) => {
            let t = biome.theme().unwrap();
            let pick = (roll as usize * 3 + variant) % 20;
            match kind {
                Scenery::Block => spawn_model(c, k, x, parent, &t.blocks[t.block_mix[pick % t.block_mix.len()]], pos, variant),
                Scenery::Side => match t.side_mix[pick % t.side_mix.len()] {
                    i if i >= 100 => spawn_model(c, k, x, parent, &t.blocks[i - 100], pos, variant),
                    i => spawn_model(c, k, x, parent, &t.sides[i], pos, variant),
                },
                Scenery::Deco => spawn_model(c, k, x, parent, &t.deco[variant % t.deco.len()], pos, variant),
            }
        }
        // ---- Automne
        (Biome::Autumn, Scenery::Block) | (Biome::Autumn, Scenery::Side) if roll < 5 => {
            part(c, parent, &k.cyl, &m.trunk, pos + Vec3::Y * 0.25, Vec3::new(0.18, 0.5, 0.18));
            part(c, parent, &k.blob, &b.leaves[variant % 3], pos + Vec3::Y * 0.8, Vec3::new(0.9, 0.82, 0.9));
            part(c, parent, &k.blob, &b.leaves[(variant + 1) % 3], pos + Vec3::new(0.22, 0.62, 0.1), Vec3::splat(0.45));
        }
        (Biome::Autumn, Scenery::Block) | (Biome::Autumn, Scenery::Side) if roll < 8 => {
            // Citrouille
            part(c, parent, &k.ball, &b.pumpkin, pos + Vec3::Y * 0.22, Vec3::new(0.62, 0.48, 0.62));
            part(c, parent, &k.cube, &m.pad, pos + Vec3::Y * 0.5, Vec3::new(0.06, 0.12, 0.06));
        }
        (Biome::Autumn, Scenery::Block) | (Biome::Autumn, Scenery::Side) => {
            let rot = Quat::from_rotation_y(variant as f32 * 1.3);
            part_rot(c, parent, &k.rock, &m.stone, pos + Vec3::Y * 0.15, Vec3::new(0.7, 0.45, 0.6), rot);
        }
        (Biome::Autumn, Scenery::Deco) => {
            for i in 0..6 {
                let a = i as f32 * 1.05 + variant as f32;
                let col = &b.leaves[(i + variant) % 3];
                part_rot(c, parent, &k.cube, col, pos + Vec3::new(a.cos() * 0.14, 0.02, a.sin() * 0.14), Vec3::new(0.12, 0.02, 0.09), Quat::from_rotation_y(a * 2.0));
            }
        }
    }
}

// ---- Véhicules décrits en pièces (gen_vehicles.rs) ---------------------------

#[derive(Clone, Copy)]
pub enum Shape {
    Cube,
    Cyl,
    Cone,
    Sphere,
}

#[derive(Clone, Copy)]
pub enum Mat {
    Peinture,
    Vitre,
    Sombre,
    Pneu,
    Enjoliveur,
    Phare,
    FeuArriere,
    LumiereRouge,
    LumiereBleue,
    Blanc,
    Noir,
    /// Couleurs propres au modèle (`c2`, `c3`).
    C2,
    C3,
    /// Globe ou dôme transparent.
    Bulle,
}

pub struct Piece {
    pub shape: Shape,
    pub pos: [f32; 3],
    pub size: [f32; 3],
    /// Degrés ; appliquée d'abord autour de X, puis Y, puis Z.
    pub rot: [f32; 3],
    pub mat: Mat,
    pub id: Option<&'static str>,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum AnimKind {
    Blink,
    BlinkAlt,
    Wink,
    Pulse,
    LengthPulse,
    Vibrate,
    Tremble,
    Sway,
    Bob,
    Float,
    PingPongZ,
    DriftBack,
    DriftUp,
    Pendulum,
    PingPongX,
    SpinY,
    Hop,
    Weave,
    Lean,
    Snake,
    BlinkSlow,
    Flutter,
    Flap,
    Fall,
    SpinLocal,
    /// Patte : pivote d'avant en arrière autour du haut ; l'ampleur porte la phase (0 ou 0,5).
    SwingZ,
    /// Aile : bat de haut en bas autour de son attache ; les deux côtés en miroir.
    SwingXBase,
    /// Queue qui frétille (de gauche à droite, ou de haut en bas).
    TailY,
    TailZ,
    TailX,
    /// Tourne en continu sur place (astéroïdes).
    SpinGroup,
    /// Pivote autour du bas du groupe (autour de Z ou de Y).
    SwingZBase,
    SwingYBase,
}

impl AnimKind {
    /// Les animations qui font tourner un groupe de pièces autour d'un point.
    fn pivoted(self) -> bool {
        matches!(self, AnimKind::SwingZ | AnimKind::SwingXBase | AnimKind::TailY | AnimKind::TailZ | AnimKind::TailX | AnimKind::SpinGroup | AnimKind::SwingZBase | AnimKind::SwingYBase)
    }
}

/// Une animation : quelle pièce (`*` = tout le véhicule, `préfixe_*`, `*contient`), laquelle, et son ampleur.
pub struct Anim {
    pub target: &'static str,
    pub kind: AnimKind,
    pub amp: f32,
}

/// Un décor décrit en pièces, avec ses deux couleurs.
pub struct Model {
    pub c2: [u8; 3],
    pub c3: [u8; 3],
    pub pieces: &'static [Piece],
    pub anims: &'static [Anim],
}

/// Pose un décor décrit en pièces ; les pièces animées gardent leur forme de départ (voir `animate_gen`).
/// Plateforme octogonale en métal, à la place du nénuphar.
pub fn spawn_platform(c: &mut Commands, k: &Kit, x: &Extra, parent: Entity) {
    part(c, parent, &x.octagon, &x.metal, Vec3::new(0.0, -0.15, 0.0), Vec3::new(0.9, 0.18, 0.9));
    part(c, parent, &x.octagon, &x.metal_top, Vec3::new(0.0, -0.055, 0.0), Vec3::new(0.7, 0.02, 0.7));
    part(c, parent, &k.ball, &x.blue_light, Vec3::new(0.3, -0.04, 0.3), Vec3::splat(0.07));
    part(c, parent, &k.ball, &x.blue_light, Vec3::new(-0.3, -0.04, -0.3), Vec3::splat(0.07));
}

/// Plateforme holographique, à la place du nénuphar (disque translucide cerclé de néon).
pub fn spawn_hologram_pad(c: &mut Commands, _k: &Kit, x: &Extra, parent: Entity) {
    part(c, parent, &x.octagon, &x.blue_light, Vec3::new(0.0, -0.16, 0.0), Vec3::new(0.95, 0.04, 0.95));
    part(c, parent, &x.octagon, &x.bubble, Vec3::new(0.0, -0.1, 0.0), Vec3::new(0.82, 0.1, 0.82));
    part(c, parent, &x.octagon, &x.metal, Vec3::new(0.0, -0.045, 0.0), Vec3::new(0.5, 0.02, 0.5));
}

/// Barge sombre bordée de néon, à la place du rondin.
pub fn spawn_barge(c: &mut Commands, k: &Kit, x: &Extra, parent: Entity, len: f32) {
    part(c, parent, &k.cube, &k.m.dark, Vec3::new(0.0, -0.08, 0.0), Vec3::new(len, 0.26, 0.74));
    part(c, parent, &k.cube, &x.metal, Vec3::new(0.0, 0.07, 0.0), Vec3::new(len - 0.2, 0.04, 0.58));
    for s in [-1.0, 1.0] {
        part(c, parent, &k.cube, &x.blue_light, Vec3::new(0.0, -0.02, s * 0.38), Vec3::new(len - 0.1, 0.05, 0.03));
        part(c, parent, &k.cube, &x.red_light, Vec3::new(s * (len / 2.0 + 0.01), 0.0, 0.0), Vec3::new(0.03, 0.08, 0.3));
    }
}

/// Tonneau qui flotte sur la mer, à la place du nénuphar.
pub fn spawn_barrel_pad(c: &mut Commands, k: &Kit, x: &Extra, parent: Entity) {
    let wood = x.color(PIRATE_COLORS[0]);
    let dark = x.color(PIRATE_COLORS[1]);
    part(c, parent, &k.cyl, wood, Vec3::new(0.0, -0.1, 0.0), Vec3::new(0.66, 0.5, 0.66));
    part(c, parent, &k.cyl, dark, Vec3::new(0.0, -0.22, 0.0), Vec3::new(0.69, 0.05, 0.69));
    part(c, parent, &k.cyl, dark, Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.69, 0.05, 0.69));
    part(c, parent, &k.cyl, x.color(PIRATE_COLORS[2]), Vec3::new(0.0, 0.16, 0.0), Vec3::new(0.52, 0.02, 0.52));
}

/// Radeau de planches, à la place du rondin : planches liées par des cordes, parfois une caisse dessus.
pub fn spawn_raft(c: &mut Commands, k: &Kit, x: &Extra, parent: Entity, len: f32) {
    let wood = x.color(PIRATE_COLORS[0]);
    let dark = x.color(PIRATE_COLORS[1]);
    let rope = x.color(PIRATE_COLORS[2]);
    part(c, parent, &k.cube, wood, Vec3::new(0.0, -0.05, 0.0), Vec3::new(len, 0.2, 0.72));
    for z in [-0.18, 0.0, 0.18] {
        part(c, parent, &k.cube, dark, Vec3::new(0.0, 0.055, z + 0.09), Vec3::new(len - 0.04, 0.012, 0.015));
    }
    let n = len as usize;
    for i in 0..=n {
        let px = -len / 2.0 + 0.2 + (len - 0.4) * i as f32 / n as f32;
        part(c, parent, &k.cube, rope, Vec3::new(px, 0.06, 0.0), Vec3::new(0.07, 0.03, 0.76));
    }
    if n >= 3 {
        part(c, parent, &k.cube, dark, Vec3::new(len * 0.2, 0.22, 0.0), Vec3::new(0.42, 0.3, 0.42));
        part(c, parent, &k.cube, wood, Vec3::new(len * 0.2, 0.39, 0.0), Vec3::new(0.46, 0.04, 0.46));
    }
}

/// Cercueil qui flotte dans le marais, à la place du rondin.
pub fn spawn_coffin(c: &mut Commands, k: &Kit, x: &Extra, parent: Entity, len: f32) {
    let wood = x.color(HALLOWEEN_COLORS[0]);
    let gold = x.color(HALLOWEEN_COLORS[1]);
    part(c, parent, &k.cube, wood, Vec3::new(0.0, -0.05, 0.0), Vec3::new(len, 0.28, 0.66));
    part(c, parent, &k.cube, wood, Vec3::new(-len * 0.42, -0.05, 0.0), Vec3::new(len * 0.16, 0.28, 0.5));
    part(c, parent, &k.cube, wood, Vec3::new(0.0, 0.1, 0.0), Vec3::new(len - 0.25, 0.04, 0.5));
    part(c, parent, &k.cube, gold, Vec3::new(len * 0.2, 0.13, 0.0), Vec3::new(0.5, 0.03, 0.08));
    part(c, parent, &k.cube, gold, Vec3::new(len * 0.2, 0.13, 0.0), Vec3::new(0.08, 0.03, 0.4));
}

/// Citrouille flottante, à la place du nénuphar.
pub fn spawn_pumpkin_pad(c: &mut Commands, k: &Kit, x: &Extra, parent: Entity) {
    let orange = x.color(HALLOWEEN_COLORS[2]);
    part(c, parent, &k.ball, orange, Vec3::new(0.0, -0.1, 0.0), Vec3::new(0.85, 0.5, 0.85));
    part(c, parent, &k.ball, orange, Vec3::new(0.22, -0.1, 0.0), Vec3::new(0.4, 0.5, 0.8));
    part(c, parent, &k.ball, orange, Vec3::new(-0.22, -0.1, 0.0), Vec3::new(0.4, 0.5, 0.8));
    part(c, parent, &k.cyl, x.color(HALLOWEEN_COLORS[3]), Vec3::new(0.0, 0.18, 0.0), Vec3::new(0.1, 0.16, 0.1));
}

/// Cargo qui dérive dans le vide, à la place du rondin : coque et conteneurs de couleur.
pub fn spawn_cargo(c: &mut Commands, k: &Kit, x: &Extra, parent: Entity, len: f32) {
    part(c, parent, &k.cube, &x.metal, Vec3::new(0.0, -0.1, 0.0), Vec3::new(len, 0.2, 0.84));
    part(c, parent, &k.cube, &x.metal_top, Vec3::new(0.0, 0.0, 0.0), Vec3::new(len - 0.1, 0.02, 0.72));
    let n = len as usize;
    let each = (len - 0.3) / n as f32;
    for i in 0..n {
        let col = x.color(SPACE_CARGO[(i + len as usize) % SPACE_CARGO.len()]);
        part(c, parent, &k.cube, col, Vec3::new(-len / 2.0 + 0.15 + each * (i as f32 + 0.5), 0.2, 0.0), Vec3::new(each - 0.08, 0.34, 0.6));
    }
    for s in [-1.0, 1.0] {
        part(c, parent, &k.ball, &x.blue_light, Vec3::new(s * (len / 2.0 + 0.02), -0.05, 0.0), Vec3::new(0.14, 0.14, 0.4));
    }
}

pub fn spawn_model(c: &mut Commands, k: &Kit, x: &Extra, parent: Entity, m: &Model, pos: Vec3, seed: usize) {
    for (n, p) in m.pieces.iter().enumerate() {
        let tf = piece_transform(p, pos);
        let e = c.spawn((Mesh3d(piece_mesh(k, p.shape).clone()), MeshMaterial3d(piece_material(k, x, p.mat, m.c2, m.c3, usize::MAX).clone()), tf, ChildOf(parent))).id();
        let kinds: Vec<(AnimKind, f32)> = m.anims.iter().filter(|a| matches_target(a.target, p.id)).map(|a| (a.kind, a.amp)).collect();
        if !kinds.is_empty() {
            c.entity(e).insert(GenAnim { kinds, base: tf, phase: (seed * 31 + n) as f32 * 0.37 });
        }
    }
}

/// Un animal décrit en pièces : pattes, ailes et queue pivotent autour de leur attache.
pub struct Animal {
    pub model: Model,
    pub len: f32,
    #[allow(dead_code)]
    pub width: f32,
    pub points: u32,
    pub dangerous: bool,
    pub flying: bool,
}

/// Pose un animal (avant = +X) ; les groupes de pièces animés par rotation sont rangés sous un pivot.
pub fn spawn_animal(c: &mut Commands, k: &Kit, x: &Extra, parent: Entity, theme: usize, i: usize) {
    spawn_pivoted(c, k, x, parent, &crate::gen_worlds::THEMES[theme].animals[i].model, i);
}

/// Un modèle dont certains groupes de pièces (pattes, ailes, queue…) pivotent autour de leur attache.
pub fn spawn_pivoted(c: &mut Commands, k: &Kit, x: &Extra, parent: Entity, m: &Model, i: usize) {
    let body = c.spawn((Transform::default(), Visibility::default(), ChildOf(parent))).id();
    let mut pivots: Vec<(&'static str, Entity, Vec3)> = Vec::new();
    for (n, p) in m.pieces.iter().enumerate() {
        let id = p.id;
        let swing = id.and_then(|id| m.anims.iter().find(|an| an.kind.pivoted() && matches_target(an.target, Some(id))).map(|an| (id, an.kind)));
        let (mut tf, owner) = (piece_transform(p, Vec3::ZERO), body);
        let mut owner = owner;
        if let Some((id, kind)) = swing {
            let at = match pivots.iter().find(|(pid, ..)| *pid == id) {
                Some((_, e, at)) => (*e, *at),
                None => {
                    // Le pivot dépend du groupe entier : on regarde toutes ses pièces.
                    let (mut lo, mut hi) = (Vec3::splat(f32::MAX), Vec3::splat(f32::MIN));
                    for q in m.pieces.iter().filter(|q| q.id == Some(id)) {
                        let (c0, h) = (Vec3::from(q.pos), Vec3::from(q.size) / 2.0);
                        lo = lo.min(c0 - h);
                        hi = hi.max(c0 + h);
                    }
                    let mid = (lo + hi) / 2.0;
                    let pivot = match kind {
                        AnimKind::SwingZ => Vec3::new(mid.x, hi.y, mid.z),
                        AnimKind::SwingZBase | AnimKind::SwingYBase => Vec3::new(mid.x, lo.y, mid.z),
                        AnimKind::SpinGroup => mid,
                        AnimKind::SwingXBase => Vec3::new(mid.x, mid.y, if mid.z > 0.0 { lo.z } else { hi.z }),
                        _ => Vec3::new(hi.x, mid.y, mid.z),
                    };
                    let kinds: Vec<(AnimKind, f32)> = m.anims.iter().filter(|an| matches_target(an.target, Some(id))).map(|an| (an.kind, an.amp)).collect();
                    let e = c.spawn((Transform::from_translation(pivot), Visibility::default(), GenAnim { kinds, base: Transform::from_translation(pivot), phase: 0.0 }, ChildOf(body))).id();
                    pivots.push((id, e, pivot));
                    (e, pivot)
                }
            };
            tf.translation -= at.1;
            owner = at.0;
        }
        let e = c.spawn((Mesh3d(piece_mesh(k, p.shape).clone()), MeshMaterial3d(piece_material(k, x, p.mat, m.c2, m.c3, usize::MAX).clone()), tf, ChildOf(owner))).id();
        if swing.is_none() {
            let kinds: Vec<(AnimKind, f32)> = m.anims.iter().filter(|an| matches_target(an.target, p.id)).map(|an| (an.kind, an.amp)).collect();
            if !kinds.is_empty() {
                c.entity(e).insert(GenAnim { kinds, base: tf, phase: (i * 31 + n) as f32 * 0.37 });
            }
        }
    }
}

fn piece_transform(p: &Piece, pos: Vec3) -> Transform {
    let rot = Quat::from_euler(EulerRot::ZYX, p.rot[2].to_radians(), p.rot[1].to_radians(), p.rot[0].to_radians());
    Transform { translation: pos + Vec3::from(p.pos), rotation: rot, scale: Vec3::from(p.size) }
}

fn piece_mesh(k: &Kit, s: Shape) -> &Handle<Mesh> {
    match s {
        Shape::Cube => &k.cube,
        Shape::Cyl => &k.cyl,
        Shape::Cone => &k.cone,
        Shape::Sphere => &k.ball,
    }
}

fn piece_material<'a>(k: &'a Kit, x: &'a Extra, mat: Mat, c2: [u8; 3], c3: [u8; 3], color: usize) -> &'a Handle<StandardMaterial> {
    match mat {
        // `usize::MAX` : pas de palette de voitures, la peau verte des héros.
        Mat::Peinture if color == usize::MAX => x.color(FROG_GREEN),
        Mat::Peinture => &k.m.car[color % k.m.car.len()],
        Mat::Vitre => &k.m.glass,
        Mat::Sombre => &k.m.dark,
        Mat::Pneu => &k.m.tire,
        Mat::Enjoliveur => &k.m.hub,
        Mat::Phare => &k.m.headlight,
        Mat::FeuArriere => &k.m.taillight,
        Mat::LumiereRouge => &x.red_light,
        Mat::LumiereBleue => &x.blue_light,
        Mat::Blanc => &x.white,
        Mat::Noir => &x.black_stripe,
        Mat::Bulle => &x.bubble,
        Mat::C2 => x.color(c2),
        Mat::C3 => x.color(c3),
    }
}

pub struct Gen {
    pub len: f32,
    pub points: u32,
    pub class: &'static str,
    pub c2: [u8; 3],
    pub c3: [u8; 3],
    pub pieces: &'static [Piece],
    pub anims: &'static [Anim],
}

fn matches_target(target: &str, id: Option<&str>) -> bool {
    let Some(id) = id else {
        return false;
    };
    if let Some(rest) = target.strip_prefix('*') {
        id.contains(rest)
    } else if let Some(prefix) = target.strip_suffix('*') {
        id.starts_with(prefix)
    } else {
        target == id
    }
}

/// Les animations d'une pièce (ou de tout le véhicule), avec la forme de départ.
#[derive(Component)]
pub struct GenAnim {
    kinds: Vec<(AnimKind, f32)>,
    base: Transform,
    phase: f32,
}

pub fn spawn_gen(c: &mut Commands, k: &Kit, x: &Extra, parent: Entity, i: usize, color: usize) {
    let g = &crate::gen_vehicles::GEN[i];
    let body = c.spawn((Transform::default(), Visibility::default(), ChildOf(parent))).id();
    if i == SPEED_BUS {
        c.entity(parent).insert(SpeedBus::default());
    }
    let whole: Vec<(AnimKind, f32)> = g.anims.iter().filter(|a| a.target == "*").map(|a| (a.kind, a.amp)).collect();
    if !whole.is_empty() {
        c.entity(body).insert(GenAnim { kinds: whole, base: Transform::default(), phase: i as f32 * 1.7 });
    }
    for (n, p) in g.pieces.iter().enumerate() {
        let tf = piece_transform(p, Vec3::ZERO);
        let e = c.spawn((Mesh3d(piece_mesh(k, p.shape).clone()), MeshMaterial3d(piece_material(k, x, p.mat, g.c2, g.c3, color).clone()), tf, ChildOf(body))).id();
        let kinds: Vec<(AnimKind, f32)> = g.anims.iter().filter(|a| a.target != "*" && matches_target(a.target, p.id)).map(|a| (a.kind, a.amp)).collect();
        if !kinds.is_empty() {
            c.entity(e).insert(GenAnim { kinds, base: tf, phase: (i * 31 + n) as f32 * 0.37 });
        }
    }
}

/// Fait vivre les pièces animées : clignotements, balancements, scanner…
pub fn animate_gen(time: Res<Time>, mut q: Query<(&GenAnim, &mut Transform, &mut Visibility)>) {
    let t = time.elapsed_secs();
    let tri = |x: f32| (x.rem_euclid(2.0) - 1.0).abs() * 2.0 - 1.0; // va-et-vient de -1 à 1
    for (a, mut tf, mut vis) in &mut q {
        let mut out = a.base;
        let mut visible = true;
        let tt = t + a.phase;
        for (kind, amp) in &a.kinds {
            match kind {
                AnimKind::Blink => visible &= (tt * 3.0) as i32 % 2 == 0,
                AnimKind::BlinkAlt => visible &= (tt * 3.0) as i32 % 2 != 0,
                AnimKind::Wink => visible &= tt.rem_euclid(2.6) < 2.45,
                AnimKind::Pulse => out.scale *= 1.0 + 0.2 * (tt * 5.0).sin(),
                AnimKind::LengthPulse => out.scale.x *= 1.0 + 0.3 * (tt * 7.0).sin(),
                AnimKind::Vibrate => out.translation += Vec3::new((tt * 61.0).sin(), (tt * 53.0).sin(), (tt * 47.0).sin()) * *amp,
                AnimKind::Tremble => {
                    out.rotation *= Quat::from_rotation_z((tt * 40.0).sin() * 0.08);
                    out.translation.y += (tt * 33.0).sin() * 0.01;
                }
                AnimKind::Sway => out.rotation *= Quat::from_rotation_z((tt * 3.0).sin() * 0.14),
                AnimKind::Bob => out.translation.y += (tt * 6.0).sin() * *amp,
                AnimKind::Float => out.translation.y += (tt * 2.0).sin() * *amp,
                AnimKind::PingPongZ => out.translation.z += tri(tt * 1.2) * *amp,
                AnimKind::PingPongX => out.translation.x += tri(tt * 0.8) * *amp,
                AnimKind::DriftBack => {
                    let f = (tt * 0.6).fract();
                    out.translation.x -= f * 0.5;
                    out.scale *= 1.0 - 0.6 * f;
                }
                AnimKind::DriftUp => {
                    let f = (tt * 0.7).fract();
                    out.translation.y += f * 0.3;
                    out.scale *= 1.0 - 0.5 * f;
                }
                AnimKind::Pendulum => out.translation.x += (tt * 3.0).sin() * 0.03,
                AnimKind::SpinY => out.rotation = Quat::from_rotation_y(tt * 6.0) * out.rotation,
                AnimKind::SpinLocal => out.rotation *= Quat::from_rotation_y(tt * 3.0),
                AnimKind::Hop => out.translation.y += (tt * 8.0).sin().abs() * *amp,
                AnimKind::Weave => out.translation.z += (tt * 2.5).sin() * *amp,
                AnimKind::Lean => out.rotation = Quat::from_rotation_x((tt * 1.6).sin() * 0.07) * out.rotation,
                // Les trois Mini serpentent : l'ampleur porte le numéro de phase.
                AnimKind::Snake => out.translation.z += (tt * 3.0 + *amp * 2.1).sin() * 0.14,
                AnimKind::BlinkSlow => visible &= (tt * 1.1) as i32 % 2 == 0,
                AnimKind::Flutter => {
                    out.rotation *= Quat::from_rotation_z((tt * 14.0).sin() * 0.35);
                    out.translation.y += (tt * 9.0).sin() * 0.03;
                }
                AnimKind::Flap => out.rotation *= Quat::from_rotation_z((tt * 16.0).sin() * 0.4),
                AnimKind::SwingZ => out.rotation *= Quat::from_rotation_z((tt * 8.0 + *amp * std::f32::consts::TAU).sin() * 0.6),
                AnimKind::SwingXBase => out.rotation *= Quat::from_rotation_x(-a.base.translation.z.signum() * (tt * 7.0 + *amp * std::f32::consts::TAU).sin() * 0.6),
                AnimKind::TailY => out.rotation *= Quat::from_rotation_y((tt * 5.0).sin() * 0.35),
                AnimKind::TailZ => out.rotation *= Quat::from_rotation_z((tt * 5.0).sin() * 0.2),
                AnimKind::SpinGroup => out.rotation = Quat::from_euler(EulerRot::YXZ, tt * 0.9, tt * 0.6, 0.0),
                AnimKind::TailX => out.rotation *= Quat::from_rotation_x((tt * 5.0).sin() * 0.2),
                AnimKind::SwingZBase => out.rotation *= Quat::from_rotation_z((tt * 3.0 + *amp * std::f32::consts::TAU).sin() * 0.5),
                AnimKind::SwingYBase => out.rotation *= Quat::from_rotation_y((tt * 3.0 + *amp * std::f32::consts::TAU).sin() * 0.6),
                AnimKind::Fall => {
                    // La portière se détache de temps en temps, puis revient d'un coup.
                    let f = ((tt % 4.0 - 3.0) / 0.6).clamp(0.0, 1.0);
                    out.translation.y -= f * 0.12;
                    out.rotation *= Quat::from_rotation_x(f * 0.7);
                }
            }
        }
        *tf = out;
        let want = if visible { Visibility::Inherited } else { Visibility::Hidden };
        if *vis != want {
            *vis = want;
        }
    }
}

// ---- Le Speed-Bus : il explose quand il sort de l'écran -----------------------

/// Numéro du Speed-Bus dans `gen_vehicles::GEN`.
pub const SPEED_BUS: usize = 8;
/// Distance (en x) à laquelle un véhicule sort de l'écran.
const SCREEN_EDGE: f32 = 11.5;

#[derive(Component, Default)]
pub struct SpeedBus {
    exploded: bool,
}

/// Le Speed-Bus saute à la sortie de l'écran, puis reparaît de l'autre côté.
pub fn speed_bus(
    mut q: Query<(&crate::lanes::Mover, &Transform, &GlobalTransform, &mut Visibility, &mut SpeedBus)>,
    frog: Res<crate::frog::Frog>,
    mut fx: MessageWriter<crate::fx::Fx>,
    mut sfx: MessageWriter<crate::audio::Sfx>,
) {
    for (m, tf, gt, mut vis, mut bus) in &mut q {
        let outward = tf.translation.x * m.speed > 0.0;
        if !bus.exploded && outward && tf.translation.x.abs() > SCREEN_EDGE {
            bus.exploded = true;
            *vis = Visibility::Hidden;
            let at = gt.translation() + Vec3::Y * 0.5;
            fx.write(crate::fx::Fx::Boom(at));
            if (m.row - frog.row).abs() <= 7 {
                sfx.write(crate::audio::Sfx::Explosion);
            }
        } else if bus.exploded && !outward {
            // Revenu de l'autre côté : un bus tout neuf.
            bus.exploded = false;
            *vis = Visibility::Inherited;
        }
    }
}
