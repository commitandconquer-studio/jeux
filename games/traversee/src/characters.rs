//! Les personnages à débloquer (avec les points) et leurs costumes (avec les
//! mouches gobées). Tous partagent la même silhouette trapue que le crapaud :
//! tête vers -Z, posés en y = 0.
use bevy::prelude::*;
use game_common::models::{FrogKit, spawn_frog};
use game_common::{flat_material, tx};

use crate::frog::FrogModel;
use crate::models::{Kit, part, part_rot};
use crate::progress::Progress;

const SAVE: &str = "traversee";

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Character {
    Frog,
    Rabbit,
    Badger,
    Raccoon,
    Hedgehog,
    Fox,
    Panda,
    Duck,
}

/// Les personnages ; les points nécessaires sont dans `progress::UNLOCKS`.
pub const CHARACTERS: [Character; 8] = [
    Character::Frog,
    Character::Rabbit,
    Character::Badger,
    Character::Raccoon,
    Character::Hedgehog,
    Character::Fox,
    Character::Panda,
    Character::Duck,
];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Costume {
    None,
    Cap,
    Shades,
    TopHat,
    Pirate,
    Crown,
    Party,
    Wizard,
}

/// Les accessoires ; les points nécessaires sont dans `progress::UNLOCKS`.
pub const COSTUMES: [Costume; 8] = [
    Costume::None,
    Costume::Cap,
    Costume::Shades,
    Costume::TopHat,
    Costume::Pirate,
    Costume::Crown,
    Costume::Party,
    Costume::Wizard,
];

impl Character {
    pub fn name(self) -> &'static str {
        match self {
            Character::Frog => tx!("Croâ le Crapaud", "Croa the Toad", "Croa die Kröte", "Croa il Rospo", "Croa el Sapo"),
            Character::Rabbit => tx!("Pompon le Lapin", "Pompon the Rabbit", "Pompon der Hase", "Pompon il Coniglio", "Pompón el Conejo"),
            Character::Badger => tx!("Gaston le Blaireau", "Gus the Badger", "Gustav der Dachs", "Gaston il Tasso", "Gastón el Tejón"),
            Character::Raccoon => tx!("Bandit le Raton Laveur", "Bandit the Raccoon", "Bandit der Waschbär", "Bandito il Procione", "Bandido el Mapache"),
            Character::Hedgehog => tx!("Piquette le Hérisson", "Prickles the Hedgehog", "Stachel der Igel", "Spillo il Riccio", "Pinchos el Erizo"),
            Character::Fox => tx!("Roux le Renard", "Rusty the Fox", "Rudi der Fuchs", "Rosso la Volpe", "Rojo el Zorro"),
            Character::Panda => tx!("Bambou le Panda", "Bamboo the Panda", "Bambus der Panda", "Bambù il Panda", "Bambú el Panda"),
            Character::Duck => tx!("Coin-Coin le Canard", "Quackers the Duck", "Quaki die Ente", "Qua-Qua l'Anatra", "Cuac el Pato"),
        }
        .t()
    }

    pub fn from_index(i: usize) -> Self {
        CHARACTERS.get(i).copied().unwrap_or(Character::Frog)
    }

    pub fn index(self) -> usize {
        CHARACTERS.iter().position(|c| *c == self).unwrap_or(0)
    }

    pub fn threshold(self) -> u32 {
        crate::progress::threshold(crate::progress::Item::Character(self))
    }

    /// Haut de la tête : là où se pose le chapeau.
    fn anchor(self) -> Vec3 {
        match self {
            Character::Frog => Vec3::new(0.0, 0.44, -0.06),
            Character::Hedgehog => Vec3::new(0.0, 0.45, -0.3),
            Character::Panda => Vec3::new(0.0, 0.55, -0.3),
            Character::Duck => Vec3::new(0.0, 0.59, -0.3),
            _ => Vec3::new(0.0, 0.51, -0.28),
        }
    }

    /// Hauteur et avant du visage : là où se posent les lunettes.
    fn face(self) -> Vec2 {
        match self {
            Character::Frog => Vec2::new(0.5, -0.45),
            Character::Duck => Vec2::new(0.5, -0.43),
            Character::Hedgehog => Vec2::new(0.38, -0.47),
            Character::Raccoon => Vec2::new(0.4, -0.5),
            Character::Panda => Vec2::new(0.42, -0.51),
            _ => Vec2::new(0.42, -0.49),
        }
    }
}

impl Costume {
    pub fn name(self) -> &'static str {
        match self {
            Costume::None => tx!("Sans costume", "No costume", "Ohne Kostüm", "Nessun costume", "Sin disfraz"),
            Costume::Cap => tx!("Casquette", "Cap", "Kappe", "Berretto", "Gorra"),
            Costume::Shades => tx!("Lunettes de soleil", "Sunglasses", "Sonnenbrille", "Occhiali da sole", "Gafas de sol"),
            Costume::TopHat => tx!("Haut-de-forme", "Top hat", "Zylinder", "Cilindro", "Sombrero de copa"),
            Costume::Pirate => tx!("Chapeau de pirate", "Pirate hat", "Piratenhut", "Cappello da pirata", "Sombrero pirata"),
            Costume::Crown => tx!("Couronne", "Crown", "Krone", "Corona", "Corona"),
            Costume::Party => tx!("Chapeau de fête", "Party hat", "Partyhut", "Cappello da festa", "Gorro de fiesta"),
            Costume::Wizard => tx!("Chapeau de magicien", "Wizard hat", "Zauberhut", "Cappello da mago", "Sombrero de mago"),
        }
        .t()
    }

    pub fn from_index(i: usize) -> Self {
        COSTUMES.get(i).copied().unwrap_or(Costume::None)
    }

    pub fn index(self) -> usize {
        COSTUMES.iter().position(|c| *c == self).unwrap_or(0)
    }

    pub fn threshold(self) -> u32 {
        crate::progress::threshold(crate::progress::Item::Costume(self))
    }
}

/// Le personnage et le costume choisis (sauvegardés).
#[derive(Resource, Clone, Copy)]
pub struct Selection {
    pub character: Character,
    pub costume: Costume,
}

impl Default for Selection {
    fn default() -> Self {
        let idx = |key: &str| game_common::save::load(SAVE, key).and_then(|s| s.parse::<usize>().ok()).unwrap_or(0);
        Self { character: Character::from_index(idx("perso")), costume: Costume::from_index(idx("costume")) }
    }
}

impl Selection {
    pub fn save(&self) {
        if ["TRAVERSEE_CAPTURE", "TRAVERSEE_PERF", "TRAVERSEE_POINTS"].iter().any(|k| std::env::var(k).is_ok()) {
            return;
        }
        game_common::save::store(SAVE, "perso", &self.character.index().to_string());
        game_common::save::store(SAVE, "costume", &self.costume.index().to_string());
    }
}

// ---- Couleurs -----------------------------------------------------------------

#[derive(Clone, Copy)]
#[repr(usize)]
enum C {
    White,
    Cream,
    Pink,
    Black,
    Dark,
    Rabbit,
    Badger,
    Raccoon,
    Hedgehog,
    Spike,
    Fox,
    Duck,
    DuckDark,
    Beak,
    Blue,
    BlueDark,
    Gold,
    Purple,
    PinkHat,
    RedHat,
    Star,
}

const COLORS: [[f32; 3]; 21] = [
    [0.97, 0.97, 0.97],
    [0.96, 0.92, 0.82],
    [0.98, 0.62, 0.68],
    [0.07, 0.07, 0.09],
    [0.25, 0.25, 0.28],
    [0.96, 0.94, 0.92],
    [0.56, 0.58, 0.62],
    [0.58, 0.55, 0.53],
    [0.62, 0.46, 0.32],
    [0.34, 0.24, 0.18],
    [0.93, 0.50, 0.14],
    [1.0, 0.84, 0.2],
    [0.9, 0.7, 0.1],
    [1.0, 0.55, 0.12],
    [0.25, 0.45, 0.85],
    [0.15, 0.3, 0.65],
    [1.0, 0.8, 0.2],
    [0.45, 0.28, 0.7],
    [0.98, 0.5, 0.7],
    [0.85, 0.2, 0.25],
    [1.0, 0.9, 0.3],
];

#[derive(Resource)]
pub struct Cast(Vec<Handle<StandardMaterial>>);

impl FromWorld for Cast {
    fn from_world(world: &mut World) -> Self {
        let mut mats = world.resource_mut::<Assets<StandardMaterial>>();
        Cast(COLORS.iter().map(|c| mats.add(flat_material(Color::srgb(c[0], c[1], c[2])))).collect())
    }
}

impl Cast {
    fn m(&self, c: C) -> &Handle<StandardMaterial> {
        &self.0[c as usize]
    }
}

/// Petit pinceau : pose des pièces sur la racine d'un personnage.
struct Brush<'a, 'w, 's> {
    c: &'a mut Commands<'w, 's>,
    root: Entity,
    kit: &'a Kit,
    cast: &'a Cast,
}

type V = [f32; 3];

impl Brush<'_, '_, '_> {
    fn cube(&mut self, col: C, p: V, s: V) {
        part(self.c, self.root, &self.kit.cube, self.cast.m(col), Vec3::from(p), Vec3::from(s));
    }
    fn cube_rot(&mut self, col: C, p: V, s: V, r: Quat) {
        part_rot(self.c, self.root, &self.kit.cube, self.cast.m(col), Vec3::from(p), Vec3::from(s), r);
    }
    fn ball(&mut self, col: C, p: V, d: f32) {
        part(self.c, self.root, &self.kit.ball, self.cast.m(col), Vec3::from(p), Vec3::splat(d));
    }
    fn ball_s(&mut self, col: C, p: V, s: V) {
        part(self.c, self.root, &self.kit.ball, self.cast.m(col), Vec3::from(p), Vec3::from(s));
    }
    fn cone(&mut self, col: C, p: V, s: V, r: Quat) {
        part_rot(self.c, self.root, &self.kit.cone, self.cast.m(col), Vec3::from(p), Vec3::from(s), r);
    }
    fn cyl(&mut self, col: C, p: V, s: V) {
        part(self.c, self.root, &self.kit.cyl, self.cast.m(col), Vec3::from(p), Vec3::from(s));
    }
    /// Deux yeux ronds noirs à reflet blanc.
    fn eyes(&mut self, x: f32, y: f32, z: f32, d: f32) {
        for s in [-1.0, 1.0] {
            self.ball(C::White, [x * s, y, z], d * 1.5);
            self.ball(C::Black, [x * s, y, z - d * 0.5], d);
        }
    }
}

/// Fabrique le personnage et son costume ; renvoie la racine.
#[allow(clippy::too_many_arguments)]
pub fn spawn_character(
    c: &mut Commands,
    ch: Character,
    costume: Costume,
    kit: &Kit,
    frog_kit: &FrogKit,
    cast: &Cast,
    mats: &mut Assets<StandardMaterial>,
    parent: Entity,
) -> Entity {
    let root = if ch == Character::Frog {
        let f = spawn_frog(c, frog_kit);
        c.entity(f).insert(ChildOf(parent));
        crate::frog::decorate(c, f, kit, mats);
        f
    } else {
        c.spawn((Transform::default(), Visibility::default(), ChildOf(parent))).id()
    };
    let mut b = Brush { c, root, kit, cast };
    match ch {
        Character::Frog => {}
        Character::Rabbit => {
            b.cube(C::Rabbit, [0.0, 0.27, 0.05], [0.6, 0.34, 0.6]);
            b.cube(C::Rabbit, [0.0, 0.36, -0.3], [0.5, 0.3, 0.34]);
            for s in [-1.0, 1.0] {
                b.cube(C::Rabbit, [0.13 * s, 0.76, -0.27], [0.1, 0.5, 0.08]);
                b.cube(C::Pink, [0.13 * s, 0.76, -0.315], [0.05, 0.4, 0.02]);
                b.cube(C::Rabbit, [0.24 * s, 0.06, 0.05], [0.16, 0.1, 0.3]);
                b.cube(C::Rabbit, [0.2 * s, 0.05, -0.38], [0.1, 0.1, 0.14]);
            }
            b.eyes(0.12, 0.43, -0.47, 0.05);
            b.ball(C::Pink, [0.0, 0.36, -0.485], 0.07);
            b.cube(C::White, [0.0, 0.28, -0.475], [0.08, 0.08, 0.02]);
            b.ball(C::White, [0.0, 0.3, 0.4], 0.24);
        }
        Character::Badger => {
            b.cube(C::Badger, [0.0, 0.27, 0.05], [0.62, 0.34, 0.62]);
            b.cube(C::Black, [0.0, 0.12, 0.05], [0.58, 0.1, 0.6]);
            b.cube(C::White, [0.0, 0.36, -0.3], [0.5, 0.3, 0.36]);
            for s in [-1.0, 1.0] {
                b.cube(C::Black, [0.14 * s, 0.362, -0.3], [0.1, 0.302, 0.362]);
                b.ball(C::Black, [0.2 * s, 0.54, -0.2], 0.12);
                b.cube(C::Black, [0.24 * s, 0.05, -0.36], [0.12, 0.1, 0.14]);
                b.cube(C::Black, [0.26 * s, 0.07, 0.2], [0.16, 0.14, 0.26]);
            }
            b.eyes(0.14, 0.43, -0.48, 0.05);
            b.cube(C::White, [0.0, 0.31, -0.46], [0.14, 0.12, 0.1]);
            b.ball(C::Black, [0.0, 0.34, -0.5], 0.09);
            b.cube(C::Badger, [0.0, 0.3, 0.4], [0.14, 0.14, 0.12]);
        }
        Character::Raccoon => {
            b.cube(C::Raccoon, [0.0, 0.27, 0.05], [0.6, 0.34, 0.6]);
            b.cube(C::Raccoon, [0.0, 0.36, -0.3], [0.52, 0.3, 0.36]);
            b.cube(C::Cream, [0.0, 0.3, -0.455], [0.26, 0.14, 0.06]);
            b.cube(C::Black, [0.0, 0.41, -0.47], [0.5, 0.1, 0.04]);
            b.eyes(0.12, 0.41, -0.5, 0.045);
            b.ball(C::Black, [0.0, 0.33, -0.5], 0.07);
            for s in [-1.0, 1.0] {
                b.ball(C::Raccoon, [0.2 * s, 0.55, -0.22], 0.14);
                b.ball(C::Black, [0.2 * s, 0.55, -0.27], 0.07);
                b.cube(C::Black, [0.2 * s, 0.04, -0.38], [0.1, 0.08, 0.12]);
                b.cube(C::Black, [0.26 * s, 0.06, 0.2], [0.16, 0.12, 0.26]);
            }
            // Queue rayée qui monte derrière
            for i in 0..6 {
                let col = if i % 2 == 0 { C::Raccoon } else { C::Dark };
                b.cube(col, [0.0, 0.3 + i as f32 * 0.07, 0.38 + i as f32 * 0.08], [0.17, 0.17, 0.11]);
            }
        }
        Character::Hedgehog => {
            b.cube(C::Hedgehog, [0.0, 0.26, 0.08], [0.62, 0.36, 0.62]);
            for (i, (x, z)) in [(-0.2, -0.02), (0.0, -0.04), (0.2, -0.02), (-0.2, 0.15), (0.0, 0.15), (0.2, 0.15), (-0.2, 0.32), (0.0, 0.34), (0.2, 0.32)].into_iter().enumerate() {
                let tilt = Quat::from_rotation_x(0.35) * Quat::from_rotation_y(i as f32);
                b.cone(C::Spike, [x, 0.5, z], [0.18, 0.34, 0.18], tilt);
            }
            for s in [-1.0, 1.0] {
                for z in [0.1, 0.3] {
                    b.cone(C::Spike, [0.32 * s, 0.34, z], [0.16, 0.26, 0.16], Quat::from_rotation_z(-0.9 * s));
                }
                b.ball(C::Cream, [0.16 * s, 0.5, -0.25], 0.1);
                b.cube(C::Cream, [0.2 * s, 0.05, -0.38], [0.1, 0.1, 0.12]);
            }
            b.cube(C::Cream, [0.0, 0.3, -0.33], [0.42, 0.3, 0.3]);
            b.eyes(0.1, 0.38, -0.46, 0.04);
            b.ball(C::Black, [0.0, 0.3, -0.5], 0.1);
        }
        Character::Fox => {
            b.cube(C::Fox, [0.0, 0.27, 0.05], [0.6, 0.34, 0.6]);
            b.cube(C::White, [0.0, 0.2, -0.22], [0.3, 0.2, 0.2]);
            b.cube(C::Fox, [0.0, 0.36, -0.3], [0.5, 0.3, 0.34]);
            b.cube(C::White, [0.0, 0.3, -0.48], [0.24, 0.14, 0.14]);
            b.ball(C::Black, [0.0, 0.33, -0.56], 0.07);
            b.eyes(0.12, 0.43, -0.47, 0.05);
            for s in [-1.0, 1.0] {
                b.cone(C::Fox, [0.17 * s, 0.66, -0.22], [0.2, 0.32, 0.12], Quat::from_rotation_z(-0.12 * s));
                b.cone(C::Black, [0.17 * s, 0.63, -0.25], [0.1, 0.2, 0.06], Quat::from_rotation_z(-0.12 * s));
                b.cube(C::White, [0.22 * s, 0.3, -0.44], [0.1, 0.1, 0.1]);
                b.cube(C::Black, [0.2 * s, 0.05, -0.38], [0.1, 0.1, 0.14]);
                b.cube(C::Black, [0.26 * s, 0.07, 0.2], [0.16, 0.14, 0.26]);
            }
            b.cube_rot(C::Fox, [0.0, 0.36, 0.52], [0.2, 0.2, 0.5], Quat::from_rotation_x(0.35));
            b.cube_rot(C::White, [0.0, 0.5, 0.74], [0.2, 0.2, 0.14], Quat::from_rotation_x(0.35));
        }
        Character::Panda => {
            b.cube(C::White, [0.0, 0.27, 0.05], [0.62, 0.34, 0.62]);
            b.cube(C::Black, [0.0, 0.3, -0.1], [0.64, 0.22, 0.12]);
            b.cube(C::White, [0.0, 0.38, -0.3], [0.54, 0.32, 0.38]);
            for s in [-1.0, 1.0] {
                b.ball(C::Black, [0.2 * s, 0.58, -0.28], 0.15);
                b.cube(C::Black, [0.13 * s, 0.41, -0.495], [0.12, 0.13, 0.02]);
                b.cube(C::Black, [0.27 * s, 0.1, -0.32], [0.14, 0.2, 0.14]);
                b.cube(C::Black, [0.28 * s, 0.1, 0.28], [0.18, 0.2, 0.26]);
            }
            b.eyes(0.13, 0.42, -0.51, 0.03);
            b.ball(C::Black, [0.0, 0.34, -0.5], 0.08);
        }
        Character::Duck => {
            b.cube(C::Duck, [0.0, 0.27, 0.05], [0.6, 0.36, 0.62]);
            b.cube(C::Duck, [0.0, 0.43, -0.3], [0.42, 0.34, 0.34]);
            b.cube(C::Beak, [0.0, 0.37, -0.5], [0.28, 0.07, 0.2]);
            b.eyes(0.12, 0.5, -0.43, 0.045);
            b.cube(C::Duck, [0.0, 0.4, 0.38], [0.2, 0.14, 0.12]);
            for s in [-1.0, 1.0] {
                b.cube(C::DuckDark, [0.32 * s, 0.3, 0.1], [0.06, 0.2, 0.3]);
                b.cube(C::Beak, [0.14 * s, 0.02, -0.3], [0.16, 0.04, 0.2]);
            }
        }
    }
    // Costume
    let a = ch.anchor();
    let f = ch.face();
    match costume {
        Costume::None => {}
        Costume::Cap => {
            b.ball_s(C::Blue, [a.x, a.y + 0.05, a.z], [0.4, 0.22, 0.4]);
            b.cube(C::BlueDark, [a.x, a.y + 0.01, a.z - 0.2], [0.3, 0.03, 0.2]);
        }
        Costume::Shades => {
            b.cube(C::Black, [0.0, f.x, f.y], [0.46, 0.04, 0.03]);
            for s in [-1.0, 1.0] {
                b.cube(C::Black, [0.12 * s, f.x, f.y - 0.005], [0.16, 0.1, 0.04]);
            }
        }
        Costume::TopHat => {
            b.cyl(C::Black, [a.x, a.y + 0.02, a.z], [0.5, 0.04, 0.5]);
            b.cyl(C::Black, [a.x, a.y + 0.22, a.z], [0.32, 0.38, 0.32]);
            b.cyl(C::RedHat, [a.x, a.y + 0.08, a.z], [0.335, 0.07, 0.335]);
        }
        Costume::Pirate => {
            b.ball_s(C::Black, [a.x, a.y + 0.08, a.z], [0.5, 0.14, 0.3]);
            b.ball_s(C::Black, [a.x, a.y + 0.14, a.z], [0.3, 0.16, 0.26]);
            b.ball(C::White, [a.x, a.y + 0.12, a.z - 0.18], 0.09);
            b.cube(C::White, [a.x, a.y + 0.06, a.z - 0.185], [0.14, 0.03, 0.01]);
        }
        Costume::Crown => {
            b.cyl(C::Gold, [a.x, a.y + 0.05, a.z], [0.34, 0.1, 0.34]);
            for i in 0..5 {
                let t = i as f32 * std::f32::consts::TAU / 5.0;
                b.cone(C::Gold, [a.x + t.cos() * 0.13, a.y + 0.16, a.z + t.sin() * 0.13], [0.09, 0.16, 0.09], Quat::IDENTITY);
            }
            b.ball(C::RedHat, [a.x, a.y + 0.06, a.z - 0.17], 0.07);
        }
        Costume::Party => {
            b.cone(C::PinkHat, [a.x, a.y + 0.22, a.z], [0.3, 0.46, 0.3], Quat::IDENTITY);
            b.ball(C::Star, [a.x, a.y + 0.46, a.z], 0.09);
            for (dy, dx) in [(0.1, 0.1), (0.2, -0.06), (0.15, -0.1)] {
                b.cube(C::Star, [a.x + dx, a.y + dy, a.z - 0.1], [0.05, 0.05, 0.03]);
            }
        }
        Costume::Wizard => {
            b.cyl(C::Purple, [a.x, a.y + 0.02, a.z], [0.56, 0.04, 0.56]);
            b.cone(C::Purple, [a.x, a.y + 0.32, a.z], [0.36, 0.62, 0.36], Quat::from_rotation_z(0.12));
            b.cyl(C::Gold, [a.x, a.y + 0.07, a.z], [0.37, 0.05, 0.37]);
            for (dy, dx) in [(0.22, 0.04), (0.34, -0.03)] {
                b.cube(C::Star, [a.x + dx, a.y + dy, a.z - 0.13], [0.06, 0.06, 0.03]);
            }
        }
    }
    root
}

/// Remet à jour le modèle du joueur quand le personnage ou le costume change.
#[allow(clippy::too_many_arguments)]
pub fn rebuild_player(
    mut commands: Commands,
    selection: Res<Selection>,
    kit: Res<Kit>,
    frog_kit: Res<FrogKit>,
    cast: Res<Cast>,
    mut mats: ResMut<Assets<StandardMaterial>>,
    model: Query<Entity, With<FrogModel>>,
    frog: Res<crate::frog::Frog>,
    progress: Res<Progress>,
    score: Res<game_common::Score>,
    extra: Res<crate::extra::Extra>,
    server: Res<AssetServer>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
    death: Res<crate::frog::Death>,
    mut was_dead: Local<bool>,
    mut built: Local<Option<(Character, Costume, Option<usize>)>>,
) {
    // Le héros Kenney meurt en s'allongeant (animation « die ») : à la partie suivante on refait le modèle, il repart debout.
    if *was_dead && death.cause.is_none() {
        *built = None;
    }
    *was_dead = death.cause.is_some();
    // Dans un monde à thème, on joue le héros du monde (avec son chapeau).
    let theme = crate::extra::Biome::of(frog.max_row_i(), progress.total + score.current).theme_index();
    let want = (selection.character, selection.costume, theme);
    if *built == Some(want) {
        return;
    }
    let Ok(root) = model.single() else {
        return;
    };
    *built = Some(want);
    commands.entity(root).despawn_related::<Children>();
    if let Some(t) = theme.filter(|_| !crate::space::kenney()) {
        // Les héros du monde regardent vers +X ; les personnages vers -Z.
        let holder = commands.spawn((Transform::from_rotation(Quat::from_rotation_y(std::f32::consts::FRAC_PI_2)), Visibility::default(), ChildOf(root))).id();
        let th = crate::gen_worlds::THEMES[t];
        crate::extra::spawn_pivoted(&mut commands, &kit, &extra, holder, &th.hero, 7);
        crate::extra::spawn_pivoted(&mut commands, &kit, &extra, holder, &th.hat, 8);
        return;
    }
    if crate::space::kenney() {
        crate::kenney::humain(&mut commands, &server, &mut graphs, root, want.0.index());
        return;
    }
    spawn_character(&mut commands, want.0, want.1, &kit, &frog_kit, &cast, &mut mats, root);
}

pub fn unlocked_character(p: &Progress, ch: Character) -> bool {
    p.has(crate::progress::Item::Character(ch))
}

pub fn unlocked_costume(p: &Progress, co: Costume) -> bool {
    p.has(crate::progress::Item::Costume(co))
}
