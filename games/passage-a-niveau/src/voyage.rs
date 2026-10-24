//! La séquence de fin : le petit train de bois (une locomotive et deux
//! wagons, en low-poly) parcourt la plus longue voie ferrée posée, et la
//! camionnette du facteur la plus longue route. La caméra s'incline pour les
//! regarder rouler ; le convoi repart du début quand il est arrivé.
use std::f32::consts::FRAC_PI_2;

use bevy::prelude::*;
use game_common::decor3d::{Kit, spawn_car, spawn_train_car};
use game_common::models::{FrogKit, spawn_frog};
use passage_core::{Board, Network, neighbor};

use crate::GameState;
use crate::jeu::Partie;
use crate::plateau::{DESSUS, pos_case};

/// Attente avant le départ : la caméra finit de passer à la vue isométrique.
const DELAI: f32 = 2.2;
/// Vitesse des véhicules, en cases par seconde.
const VITESSE: f32 = 2.2;
/// Pause en bout de voie avant de repartir, en cases (de distance équivalente).
const PAUSE: f32 = 3.0;
const PAS: usize = 8;
/// Distance entre deux éléments du train (longueur d'un élément et un petit jeu).
const ECART: f32 = 0.95;

pub struct VoyagePlugin;

impl Plugin for VoyagePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Fini), partir)
            .add_systems(Update, rouler.run_if(in_state(GameState::Fini)));
    }
}

/// Un trajet : des points rapprochés et la distance parcourue jusqu'à chacun.
pub struct Trace {
    points: Vec<Vec3>,
    cumul: Vec<f32>,
}

impl Trace {
    fn new(points: Vec<Vec3>) -> Trace {
        let mut nets: Vec<Vec3> = Vec::new();
        for p in points {
            if nets.last().is_none_or(|q| q.distance(p) > 1e-4) {
                nets.push(p);
            }
        }
        let mut cumul = vec![0.0];
        for w in nets.windows(2) {
            cumul.push(cumul.last().unwrap() + w[0].distance(w[1]));
        }
        Trace { points: nets, cumul }
    }

    pub fn longueur(&self) -> f32 {
        self.cumul.last().copied().unwrap_or(0.0)
    }

    /// La position à la distance `d` du départ, et la direction de marche.
    pub fn echantillon(&self, d: f32) -> Option<(Vec3, Vec3)> {
        let premier = *self.points.first()?;
        if self.points.len() < 2 {
            return Some((premier, Vec3::NEG_Z));
        }
        let d = d.clamp(0.0, self.longueur());
        let i = self.cumul.partition_point(|&c| c <= d).clamp(1, self.points.len() - 1);
        let (a, b) = (self.points[i - 1], self.points[i]);
        let t = if self.cumul[i] > self.cumul[i - 1] { (d - self.cumul[i - 1]) / (self.cumul[i] - self.cumul[i - 1]) } else { 0.0 };
        Some((a.lerp(b, t), (b - a).normalize_or_zero()))
    }
}

fn dir3(side: usize) -> Vec3 {
    match side {
        0 => Vec3::NEG_Z,
        1 => Vec3::X,
        2 => Vec3::Z,
        _ => Vec3::NEG_X,
    }
}

/// Les points du parcours d'une case, du côté d'entrée au côté de sortie :
/// tout droit, ou en quart de cercle autour du coin commun.
fn points_de_case(base: Vec3, entree: Option<usize>, sortie: Option<usize>) -> Vec<Vec3> {
    let ligne = |a: Vec3, b: Vec3| (0..=PAS).map(|k| a.lerp(b, k as f32 / PAS as f32)).collect::<Vec<_>>();
    match (entree, sortie) {
        (Some(a), Some(b)) if (a + 2) % 4 == b => ligne(base + dir3(a) * 0.5, base + dir3(b) * 0.5),
        (Some(a), Some(b)) => {
            let coin = base + (dir3(a) + dir3(b)) * 0.5;
            let (va, vb) = (base + dir3(a) * 0.5 - coin, base + dir3(b) * 0.5 - coin);
            (0..=PAS).map(|k| {
                let t = k as f32 / PAS as f32 * FRAC_PI_2;
                coin + va * t.cos() + vb * t.sin()
            }).collect()
        }
        (Some(a), None) => ligne(base + dir3(a) * 0.5, base),
        (None, Some(b)) => ligne(base, base + dir3(b) * 0.5),
        (None, None) => vec![base],
    }
}

/// Le tracé d'une suite de cases d'une voie : on y entre par un bout libre et on en ressort par l'autre.
pub fn tracer(board: &Board, chemin: &[usize], network: Network, hauteur: f32) -> Trace {
    let mut points = Vec::new();
    for (i, &cell) in chemin.iter().enumerate() {
        let carte = board.cells[cell].unwrap();
        let ouvert = |s: usize| carte.sides(network) & (1 << s) != 0;
        let libre = |s: usize| ouvert(s) && !board.is_linked(cell, s, network);
        let vers = |autre: usize| (0..4).find(|&s| neighbor(cell, s) == Some(autre));
        let mut sortie = chemin.get(i + 1).and_then(|&n| vers(n));
        let mut entree = if i > 0 { vers(chemin[i - 1]) } else { None };
        if i == 0 {
            entree = (0..4).find(|&s| libre(s) && Some(s) != sortie);
        }
        if i + 1 == chemin.len() {
            sortie = (0..4).find(|&s| libre(s) && Some(s) != entree);
        }
        let base = pos_case(cell) - Vec3::Y * DESSUS + Vec3::Y * hauteur;
        points.extend(points_de_case(base, entree, sortie));
    }
    Trace::new(points)
}

#[derive(Resource)]
struct Voyage {
    rail: Trace,
    route: Trace,
    t: f32,
}

/// Le chef de gare : il apparaît en même temps que le convoi.
#[derive(Component)]
struct ChefDeGare;

#[derive(Component)]
struct Vehicule {
    reseau: Network,
    /// Distance derrière la tête du convoi.
    retard: f32,
}

fn partir(mut commands: Commands, partie: Res<Partie>, kit: Res<Kit>, frog: Res<FrogKit>) {
    let board = &partie.jeu.board;
    let rail = tracer(board, &board.longest_path(Network::Rail), Network::Rail, DESSUS + 0.04);
    let route = tracer(board, &board.longest_path(Network::Route), Network::Route, DESSUS + 0.02);
    if !rail.points.is_empty() {
        // La locomotive, puis deux wagons : les modèles de La Traversée, en petit.
        for k in 0..3 {
            vehicule(&mut commands, &kit, Network::Rail, ECART * k as f32, k == 0);
        }
    }
    if !route.points.is_empty() {
        vehicule(&mut commands, &kit, Network::Route, 0.0, false);
    }
    // Le crapaud, chef de gare, regarde passer son train depuis la prairie.
    let crapaud = spawn_frog(&mut commands, &frog);
    commands.entity(crapaud).insert((
        Transform::from_xyz(4.6, 0.0, 2.6).with_rotation(Quat::from_rotation_y(FRAC_PI_2 + 0.3)).with_scale(Vec3::splat(1.3)),
        ChefDeGare,
        DespawnOnExit(GameState::Fini),
    ));
    commands.insert_resource(Voyage { rail, route, t: -DELAI });
}

/// Un véhicule : la locomotive, un wagon (voie ferrée) ou la camionnette
/// jaune (route), avec les modèles de La Traversée réduits à la taille d'une case.
fn vehicule(c: &mut Commands, kit: &Kit, reseau: Network, retard: f32, locomotive: bool) {
    let racine = c.spawn((Transform::default(), Visibility::default(), Vehicule { reseau, retard }, DespawnOnExit(GameState::Fini))).id();
    // Les modèles regardent vers +X ; le convoi avance vers -Z.
    let echelle = if reseau == Network::Rail { 0.3 } else { 0.4 };
    let modele = c.spawn((Transform::from_rotation(Quat::from_rotation_y(FRAC_PI_2)).with_scale(Vec3::splat(echelle)), Visibility::default(), ChildOf(racine))).id();
    match reseau {
        Network::Rail => spawn_train_car(c, kit, modele, locomotive),
        Network::Route => spawn_car(c, kit, modele, 2),
    }
}

/// L'apparition : de rien à la taille normale, juste avant le départ.
fn apparition(t: f32) -> f32 {
    let x = ((t + 0.7) / 0.7).clamp(0.0, 1.0);
    x * x * (3.0 - 2.0 * x)
}

/// Les véhicules roulent ; la tête du convoi avance, les wagons la suivent.
fn rouler(
    time: Res<Time>,
    mut voyage: ResMut<Voyage>,
    mut q: Query<(&Vehicule, &mut Transform), Without<ChefDeGare>>,
    mut chef: Query<&mut Transform, With<ChefDeGare>>,
) {
    voyage.t += time.delta_secs();
    let pop = apparition(voyage.t);
    for mut tr in &mut chef {
        tr.scale = Vec3::splat(1.3 * pop);
    }
    for (v, mut tr) in &mut q {
        let trace = if v.reseau == Network::Rail { &voyage.rail } else { &voyage.route };
        let longueur = trace.longueur();
        let cycle = (longueur + PAUSE) / VITESSE;
        let tete = ((voyage.t.max(0.0) % cycle) * VITESSE).min(longueur);
        let Some((pos, dir)) = trace.echantillon((tete - v.retard).max(0.0)) else { continue };
        let regard = if dir == Vec3::ZERO { Vec3::NEG_Z } else { dir };
        *tr = Transform::from_translation(pos).looking_to(regard, Vec3::Y).with_scale(Vec3::splat(pop));
    }
}
