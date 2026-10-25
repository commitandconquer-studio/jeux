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
    blue_light: Handle<StandardMaterial>,
    black_stripe: Handle<StandardMaterial>,
    bus: Vec<Handle<StandardMaterial>>,
    green: Handle<StandardMaterial>,
    taxi: Handle<StandardMaterial>,
    fly_body: Handle<StandardMaterial>,
    fly_gold: Handle<StandardMaterial>,
    fly_wing: Handle<StandardMaterial>,
    fly_eye: Handle<StandardMaterial>,
    silver: Handle<StandardMaterial>,
    glow_blue: Handle<StandardMaterial>,
    glow_orange: Handle<StandardMaterial>,
    /// Index de la peinture blanche dans la palette des voitures.
    white_car: usize,
    /// Index de la peinture jaune (taxi).
    yellow_car: usize,
}

impl FromWorld for Extra {
    fn from_world(world: &mut World) -> Self {
        let mut mats = world.resource_mut::<Assets<StandardMaterial>>();
        let mut f = |r: f32, g: f32, b: f32| mats.add(flat_material(Color::srgb(r, g, b)));
        let flowers = vec![f(0.98, 0.45, 0.62), f(1.0, 0.85, 0.25), f(0.97, 0.97, 0.97), f(0.92, 0.25, 0.25), f(0.65, 0.45, 0.92), f(1.0, 0.6, 0.2)];
        let bus = vec![f(0.98, 0.78, 0.15), f(0.25, 0.65, 0.40), f(0.85, 0.25, 0.25), f(0.30, 0.50, 0.85)];
        let tuft = f(0.40, 0.68, 0.30);
        let mut glow = |c: Color, e: LinearRgba| mats.add(StandardMaterial { emissive: e, ..flat_material(c) });
        let fly_gold = glow(Color::srgb(1.0, 0.82, 0.2), LinearRgba::rgb(2.5, 1.7, 0.2));
        let red_light = glow(Color::srgb(1.0, 0.2, 0.2), LinearRgba::rgb(3.0, 0.2, 0.2));
        let glow_blue = glow(Color::srgb(0.6, 0.9, 1.0), LinearRgba::rgb(0.8, 2.5, 4.0));
        let glow_orange = glow(Color::srgb(1.0, 0.6, 0.2), LinearRgba::rgb(4.0, 1.6, 0.2));
        let blue_light = glow(Color::srgb(0.2, 0.4, 1.0), LinearRgba::rgb(0.2, 0.5, 3.0));
        let mut f = |r: f32, g: f32, b: f32| mats.add(flat_material(Color::srgb(r, g, b)));
        Extra {
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
            silver: f(0.78, 0.80, 0.84),
            glow_blue,
            glow_orange,
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
    Future,
    Sheriff,
    Fantastic,
}

impl Vehicle {
    /// Longueur (pour les collisions et l'espacement).
    pub fn len(self) -> f32 {
        match self {
            Vehicle::Car | Vehicle::Police | Vehicle::Taxi => 1.6,
            Vehicle::Future => 1.8,
            Vehicle::Sheriff | Vehicle::Fantastic => 1.7,
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
    const WEIGHTS: [(Vehicle, u32); 12] = [
        (Vehicle::Car, 38),
        (Vehicle::Truck, 14),
        (Vehicle::Taxi, 6),
        (Vehicle::Moto, 6),
        (Vehicle::Bus, 8),
        (Vehicle::Pickup, 8),
        (Vehicle::Sport, 10),
        (Vehicle::Police, 6),
        (Vehicle::Tractor, 4),
        (Vehicle::Future, 3),
        (Vehicle::Sheriff, 3),
        (Vehicle::Fantastic, 3),
    ];
    let open: Vec<(Vehicle, u32)> =
        WEIGHTS.iter().copied().filter(|(v, _)| crate::progress::unlocked(points, crate::progress::Item::Vehicle(*v))).collect();
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
        Vehicle::Future => {
            // Bolide argenté, en coin, avec des lignes lumineuses à l'arrière.
            part(c, parent, &k.cube, &m.dark, Vec3::new(0.0, 0.2, 0.0), Vec3::new(1.78, 0.1, 0.78));
            part(c, parent, &k.cube, &x.silver, Vec3::new(-0.05, 0.36, 0.0), Vec3::new(1.8, 0.24, 0.84));
            part_rot(c, parent, &k.cube, &x.silver, Vec3::new(0.62, 0.5, 0.0), Vec3::new(0.7, 0.04, 0.82), Quat::from_rotation_z(-0.12));
            part(c, parent, &k.cube, &m.glass, Vec3::new(-0.2, 0.56, 0.0), Vec3::new(0.72, 0.2, 0.72));
            part(c, parent, &k.cube, &x.silver, Vec3::new(-0.2, 0.67, 0.0), Vec3::new(0.66, 0.03, 0.68));
            part(c, parent, &k.cube, &x.black_stripe, Vec3::new(-0.91, 0.45, 0.0), Vec3::new(0.04, 0.12, 0.72));
            part(c, parent, &k.cube, &x.glow_blue, Vec3::new(-0.915, 0.38, 0.0), Vec3::new(0.02, 0.035, 0.72));
            for s in [-1.0, 1.0] {
                part(c, parent, &k.cube, &x.glow_orange, Vec3::new(-0.92, 0.5, 0.28 * s), Vec3::new(0.03, 0.05, 0.14));
                part(c, parent, &k.cube, &x.glow_blue, Vec3::new(-0.2, 0.37, 0.426 * s), Vec3::new(1.1, 0.02, 0.01));
                part(c, parent, &k.cube, &m.headlight, Vec3::new(0.9, 0.38, 0.28 * s), Vec3::new(0.03, 0.06, 0.16));
                for wx in [-0.55, 0.58] {
                    wheel(c, k, parent, Vec3::new(wx, 0.17, 0.4 * s), 0.17, 0.16);
                }
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
        Vehicle::Fantastic => {
            spawn_car(c, k, parent, x.white_car + 3); // presque noir
            part(c, parent, &k.cube, &x.black_stripe, Vec3::new(0.8, 0.46, 0.0), Vec3::new(0.03, 0.08, 0.62));
            let eye = part(c, parent, &k.cube, &x.red_light, Vec3::new(0.83, 0.46, 0.0), Vec3::new(0.02, 0.04, 0.1));
            c.entity(eye).insert(Scanner);
            part(c, parent, &k.cube, &x.black_stripe, Vec3::new(-0.76, 0.6, 0.0), Vec3::new(0.06, 0.03, 0.74));
            part(c, parent, &k.cube, &x.red_light, Vec3::new(-0.12, 0.915, 0.0), Vec3::new(0.84, 0.01, 0.01));
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

/// Le voyant rouge qui balaie le museau de la voiture fantastique.
#[derive(Component)]
pub struct Scanner;

pub fn animate_scanner(time: Res<Time>, mut q: Query<&mut Transform, With<Scanner>>) {
    let t = time.elapsed_secs() * 3.2;
    // Va-et-vient régulier (triangle), de bord à bord du museau.
    let tri = (t % 2.0 - 1.0).abs() * 2.0 - 1.0;
    for mut tf in &mut q {
        tf.translation.z = tri * 0.26;
    }
}
