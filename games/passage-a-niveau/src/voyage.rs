//! La séquence de fin : le petit train de bois (une locomotive et deux
//! wagons, en low-poly) parcourt la plus longue voie ferrée posée, et la
//! camionnette du facteur la plus longue route. La caméra s'incline pour les
//! regarder rouler ; le convoi repart du début quand il est arrivé.
use std::f32::consts::FRAC_PI_2;

use bevy::light::NotShadowCaster;
use bevy::prelude::*;
use passage_core::{Board, Network, neighbor};

use crate::GameState;
use crate::jeu::Partie;
use crate::plateau::{DESSUS, Look, pos_case};

/// Vitesse des véhicules, en cases par seconde.
const VITESSE: f32 = 2.2;
/// Pause en bout de voie avant de repartir, en cases (de distance équivalente).
const PAUSE: f32 = 3.0;
const PAS: usize = 8;

pub struct VoyagePlugin;

impl Plugin for VoyagePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Fini), partir)
            .add_systems(Update, (rouler, fumee).run_if(in_state(GameState::Fini)));
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

#[derive(Component)]
struct Vehicule {
    reseau: Network,
    /// Distance derrière la tête du convoi.
    retard: f32,
}

#[derive(Component)]
struct Locomotive;

#[derive(Component)]
struct Fumee(f32);

fn partir(mut commands: Commands, partie: Res<Partie>, look: Res<Look>) {
    let board = &partie.jeu.board;
    let rail = tracer(board, &board.longest_path(Network::Rail), Network::Rail, DESSUS + 0.04);
    let route = tracer(board, &board.longest_path(Network::Route), Network::Route, DESSUS + 0.02);
    if !rail.points.is_empty() {
        let loco = vehicule(&mut commands, &look, Network::Rail, 0.0);
        commands.entity(loco).insert(Locomotive);
        for (k, couleur) in [&look.jaune, &look.bois].into_iter().enumerate() {
            let wagon = vehicule(&mut commands, &look, Network::Rail, 0.9 * (k + 1) as f32);
            wagon_caisse(&mut commands, wagon, &look, couleur);
        }
    }
    if !route.points.is_empty() {
        let camion = vehicule(&mut commands, &look, Network::Route, 0.0);
        camionnette(&mut commands, camion, &look);
    }
    commands.insert_resource(Voyage { rail, route, t: 0.0 });
}

fn bloc(c: &mut Commands, parent: Entity, look: &Look, mat: &Handle<StandardMaterial>, centre: Vec3, taille: Vec3) {
    c.spawn((Mesh3d(look.cube.clone()), MeshMaterial3d(mat.clone()), Transform::from_translation(centre).with_scale(taille), NotShadowCaster, ChildOf(parent)));
}

fn cylindre(c: &mut Commands, parent: Entity, look: &Look, mat: &Handle<StandardMaterial>, centre: Vec3, taille: Vec3, couche: bool) {
    let rot = if couche { Quat::from_rotation_x(FRAC_PI_2) } else { Quat::IDENTITY };
    c.spawn((Mesh3d(look.disque.clone()), MeshMaterial3d(mat.clone()), Transform::from_translation(centre).with_rotation(rot).with_scale(taille), NotShadowCaster, ChildOf(parent)));
}

/// Un véhicule vide : la locomotive (le train), ou un châssis de wagon ou de camionnette.
fn vehicule(c: &mut Commands, look: &Look, reseau: Network, retard: f32) -> Entity {
    let racine = c.spawn((Transform::default(), Visibility::default(), Vehicule { reseau, retard }, DespawnOnExit(GameState::Fini))).id();
    if reseau == Network::Rail {
        bloc(c, racine, look, &look.noir, Vec3::new(0.0, 0.09, 0.0), Vec3::new(0.3, 0.07, 0.62));
        for (x, z) in [(-0.17, -0.2), (0.17, -0.2), (-0.17, 0.2), (0.17, 0.2)] {
            bloc(c, racine, look, &look.noir, Vec3::new(x, 0.05, z), Vec3::new(0.05, 0.1, 0.13));
        }
        if retard == 0.0 {
            // La locomotive : chaudière, cheminée, cabine et toit.
            cylindre(c, racine, look, &look.rouge, Vec3::new(0.0, 0.25, -0.12), Vec3::new(0.26, 0.5, 0.26), true);
            cylindre(c, racine, look, &look.noir, Vec3::new(0.0, 0.43, -0.27), Vec3::new(0.1, 0.2, 0.1), false);
            bloc(c, racine, look, &look.rouge, Vec3::new(0.0, 0.28, 0.24), Vec3::new(0.3, 0.28, 0.26));
            bloc(c, racine, look, &look.noir, Vec3::new(0.0, 0.44, 0.24), Vec3::new(0.34, 0.04, 0.3));
        }
    }
    racine
}

fn wagon_caisse(c: &mut Commands, wagon: Entity, look: &Look, couleur: &Handle<StandardMaterial>) {
    bloc(c, wagon, look, couleur, Vec3::new(0.0, 0.21, 0.0), Vec3::new(0.3, 0.2, 0.56));
}

fn camionnette(c: &mut Commands, camion: Entity, look: &Look) {
    bloc(c, camion, look, &look.jaune, Vec3::new(0.0, 0.2, 0.1), Vec3::new(0.34, 0.22, 0.46));
    bloc(c, camion, look, &look.blanc, Vec3::new(0.0, 0.18, -0.26), Vec3::new(0.34, 0.18, 0.22));
    bloc(c, camion, look, &look.noir, Vec3::new(0.0, 0.22, -0.375), Vec3::new(0.28, 0.08, 0.02));
    for (x, z) in [(-0.19, -0.2), (0.19, -0.2), (-0.19, 0.22), (0.19, 0.22)] {
        bloc(c, camion, look, &look.noir, Vec3::new(x, 0.06, z), Vec3::new(0.06, 0.12, 0.12));
    }
}

/// Les véhicules roulent ; la tête du convoi avance, les wagons la suivent.
fn rouler(time: Res<Time>, mut voyage: ResMut<Voyage>, mut q: Query<(&Vehicule, &mut Transform)>) {
    voyage.t += time.delta_secs();
    for (v, mut tr) in &mut q {
        let trace = if v.reseau == Network::Rail { &voyage.rail } else { &voyage.route };
        let longueur = trace.longueur();
        let cycle = (longueur + PAUSE) / VITESSE;
        let tete = ((voyage.t % cycle) * VITESSE).min(longueur);
        let Some((pos, dir)) = trace.echantillon((tete - v.retard).max(0.0)) else { continue };
        let regard = if dir == Vec3::ZERO { Vec3::NEG_Z } else { dir };
        *tr = Transform::from_translation(pos).looking_to(regard, Vec3::Y);
    }
}

/// De petits cubes de fumée blanche montent de la cheminée de la locomotive.
fn fumee(
    mut commands: Commands,
    time: Res<Time>,
    look: Res<Look>,
    mut prochaine: Local<f32>,
    loco: Query<&Transform, (With<Locomotive>, Without<Fumee>)>,
    mut bouffees: Query<(Entity, &mut Fumee, &mut Transform)>,
) {
    let dt = time.delta_secs();
    *prochaine -= dt;
    if *prochaine <= 0.0
        && let Ok(tr) = loco.single()
    {
        *prochaine = 0.22;
        let cheminee = tr.translation + tr.rotation * Vec3::new(0.0, 0.55, -0.27);
        commands.spawn((
            Mesh3d(look.cube.clone()),
            MeshMaterial3d(look.blanc.clone()),
            Transform::from_translation(cheminee).with_scale(Vec3::splat(0.08)),
            NotShadowCaster,
            Fumee(0.0),
            DespawnOnExit(GameState::Fini),
        ));
    }
    for (e, mut f, mut tr) in &mut bouffees {
        f.0 += dt;
        if f.0 > 1.3 {
            commands.entity(e).despawn();
            continue;
        }
        tr.translation.y += 0.35 * dt;
        tr.rotate_y(1.5 * dt);
        let taille = 0.08 + 0.16 * (f.0 / 1.3) * (1.0 - f.0 / 1.3).sqrt().max(0.2);
        tr.scale = Vec3::splat(taille);
    }
}

