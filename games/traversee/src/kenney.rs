//! Essai « Kenney » (`TRAVERSEE_KENNEY=1`) : les voitures du Car Kit de Kenney (CC0, `assets/kenney/car-kit`)
//! remplacent les voitures en pièces sur les routes des mondes sans thème.
use crate::extra::Vehicle;
use bevy::prelude::*;

const CARS: [&str; 6] = ["sedan", "hatchback-sports", "suv", "sedan-sports", "van", "suv-luxury"];
const TRUCKS: [&str; 4] = ["truck", "delivery", "garbage-truck", "firetruck"];

/// Pose un modèle Kenney à la place du véhicule ; renvoie `false` s'il n'y en a pas pour ce véhicule.
pub fn car(c: &mut Commands, server: &AssetServer, parent: Entity, v: Vehicle, color: usize) -> bool {
    let (nom, echelle) = match v {
        Vehicle::Car => (CARS[color % CARS.len()], 0.8),
        Vehicle::Truck => (TRUCKS[color % TRUCKS.len()], 0.8),
        Vehicle::Police => ("police", 0.8),
        Vehicle::Taxi => ("taxi", 0.8),
        Vehicle::Sport => ("race", 0.8),
        Vehicle::Tractor => ("tractor", 0.8),
        Vehicle::Pickup => ("truck-flat", 0.8),
        _ => return false,
    };
    let scene = server.load(GltfAssetLabel::Scene(0).from_asset(format!("kenney/car-kit/{nom}.glb")));
    // Les modèles Kenney regardent vers +Z ; le jeu avance vers +X.
    c.spawn((WorldAssetRoot(scene), Transform::from_rotation(Quat::from_rotation_y(std::f32::consts::FRAC_PI_2)).with_scale(Vec3::splat(echelle)), ChildOf(parent)));
    true
}

/// Arbres et buissons du Mini Forest (`assets/kenney/mini-forest`) ; `false` s'il n'y en a pas.
pub fn prop(c: &mut Commands, server: &AssetServer, parent: Entity, p: crate::models::Prop, at: Vec3, variant: usize) -> bool {
    use crate::models::Prop;
    let nom = match p {
        Prop::Tree => "tree",
        Prop::Pine => "tree-high",
        // Les rochers Kenney sortent noirs dans Bevy : on garde ceux du jeu (gris).
        Prop::Rock => return false,
        Prop::Bush => "plant",
    };
    let scene = server.load(GltfAssetLabel::Scene(0).from_asset(format!("kenney/mini-forest/{nom}.glb")));
    let tour = Quat::from_rotation_y(variant as f32 * 1.57);
    c.spawn((WorldAssetRoot(scene), Transform::from_translation(at).with_rotation(tour), ChildOf(parent)));
    true
}

/// Un personnage Mini Characters : le graphe d'animation (idle, saut, mort) à poser quand son `AnimationPlayer` apparaît.
#[derive(Component)]
pub struct Humain {
    graph: Handle<AnimationGraph>,
    idle: AnimationNodeIndex,
    saut: AnimationNodeIndex,
    mort: AnimationNodeIndex,
}

/// Animation en cours sur un squelette Kenney (pour ne la relancer que quand elle change).
#[derive(Component, PartialEq, Clone, Copy)]
pub enum Etat {
    Idle,
    Saut,
    Mort,
}

/// Le héros jouable Kenney (`character-male-a`…) ; regarde vers -Z comme les autres personnages du jeu.
pub fn humain(c: &mut Commands, server: &AssetServer, graphs: &mut Assets<AnimationGraph>, parent: Entity, index: usize) {
    let lettre = (b'a' + (index / 2 % 6) as u8) as char;
    let sexe = if index % 2 == 0 { "male" } else { "female" };
    let fichier = format!("kenney/mini-characters/character-{sexe}-{lettre}.glb");
    let scene = server.load(GltfAssetLabel::Scene(0).from_asset(fichier.clone()));
    let clip = |i: usize| server.load(GltfAssetLabel::Animation(i).from_asset(fichier.clone()));
    // Dans les GLB : 1 = idle, 4 = jump, 9 = die.
    let mut graph = AnimationGraph::new();
    let idle = graph.add_clip(clip(1), 1.0, graph.root);
    let saut = graph.add_clip(clip(4), 1.0, graph.root);
    let mort = graph.add_clip(clip(9), 1.0, graph.root);
    c.spawn((
        WorldAssetRoot(scene),
        Transform::from_rotation(Quat::from_rotation_y(std::f32::consts::PI)).with_scale(Vec3::splat(0.8)),
        Humain { graph: graphs.add(graph), idle, saut, mort },
        ChildOf(parent),
    ));
}

/// Pose le graphe d'animation dès que le squelette est chargé, puis choisit l'animation selon le crapaud :
/// mort, saut en cours, ou repos.
pub fn animer_humains(
    mut commands: Commands,
    mut lecteurs: Query<(Entity, &mut AnimationPlayer, Option<&mut Etat>)>,
    parents: Query<&ChildOf>,
    humains: Query<&Humain>,
    frog: Res<crate::frog::Frog>,
    death: Res<crate::frog::Death>,
) {
    let voulu = if death.cause.is_some() {
        Etat::Mort
    } else if frog.hop.is_some() {
        Etat::Saut
    } else {
        Etat::Idle
    };
    for (e, mut lecteur, etat) in &mut lecteurs {
        let mut cur = e;
        let humain = loop {
            if let Ok(h) = humains.get(cur) {
                break Some(h);
            }
            match parents.get(cur) {
                Ok(p) => cur = p.parent(),
                Err(_) => break None,
            }
        };
        let Some(h) = humain else { continue };
        if etat.is_none() {
            commands.entity(e).insert((AnimationGraphHandle(h.graph.clone()), Etat::Idle));
            lecteur.play(h.idle).repeat();
            continue;
        }
        let mut etat = etat.unwrap();
        if *etat == voulu {
            continue;
        }
        *etat = voulu;
        lecteur.stop_all();
        match voulu {
            Etat::Idle => {
                lecteur.play(h.idle).repeat();
            }
            // Le saut du jeu ne dure que 0,14 s : on accélère le clip.
            Etat::Saut => {
                lecteur.play(h.saut).set_speed(4.0);
            }
            Etat::Mort => {
                lecteur.play(h.mort);
            }
        }
    }
}
