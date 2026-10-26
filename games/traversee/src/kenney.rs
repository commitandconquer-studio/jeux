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

/// Ce qui roule sur le ponton de la baie des pirates : (modèle, échelle, longueur).
pub const PIRATE_VEHICLES: [(&str, f32, f32); 5] = [
    ("cannon-mobile", 0.45, 1.2),
    ("barrel", 0.5, 0.8),
    ("barrel", 0.5, 0.8),
    ("cannon-mobile", 0.45, 1.2),
    ("barrel", 0.6, 0.9),
];

/// Un tonneau qui roule : tourne autour de l'axe en travers de la route selon la vitesse de son véhicule.
#[derive(Component)]
pub struct Rouler {
    rayon: f32,
}

/// Pose le véhicule n° `i` du ponton : canon sur roues, ou tonneau couché qui roule.
pub fn pirate_vehicle(c: &mut Commands, server: &AssetServer, parent: Entity, i: usize) -> bool {
    let (nom, echelle, _) = PIRATE_VEHICLES[i];
    let scene = server.load(GltfAssetLabel::Scene(0).from_asset(format!("kenney/pirate-kit/{nom}.glb")));
    if nom == "barrel" {
        // Le tonneau (debout, origine au pied, hauteur 1,23, rayon 0,67) est couché sur le côté et centré sur l'axe de roulement.
        let rayon = 0.67 * echelle;
        let tenant = c.spawn((Transform::from_xyz(0.0, rayon, 0.0), Visibility::default(), Rouler { rayon }, ChildOf(parent))).id();
        let couche = Quat::from_rotation_x(std::f32::consts::FRAC_PI_2);
        c.spawn((WorldAssetRoot(scene), Transform::from_translation(couche * Vec3::new(0.0, -0.615 * echelle, 0.0)).with_rotation(couche).with_scale(Vec3::splat(echelle)), ChildOf(tenant)));
    } else {
        // Comme les voitures : les modèles Kenney regardent vers +Z, le jeu avance vers +X.
        c.spawn((WorldAssetRoot(scene), Transform::from_rotation(Quat::from_rotation_y(std::f32::consts::FRAC_PI_2)).with_scale(Vec3::splat(echelle)), ChildOf(parent)));
    }
    true
}

/// Fait tourner les tonneaux selon la vitesse du véhicule qui les porte (le véhicule est retourné quand il va vers -X : le sens de rotation suit).
pub fn rouler(time: Res<Time>, mut q: Query<(&mut Transform, &Rouler, &ChildOf)>, movers: Query<&crate::lanes::Mover>) {
    for (mut tf, r, parent) in &mut q {
        if let Ok(m) = movers.get(parent.parent()) {
            tf.rotate_z(-m.speed.abs() * time.delta_secs() / r.rayon);
        }
    }
}

/// Décor de la baie : palmiers, rochers de sable, caisses, tonneaux, canons… `roll` : 0 à 9.
pub fn pirate_scenery(c: &mut Commands, server: &AssetServer, parent: Entity, kind: crate::extra::Scenery, at: Vec3, roll: u32, variant: usize) -> bool {
    use crate::extra::Scenery;
    let (nom, echelle) = match kind {
        Scenery::Block | Scenery::Side => match roll {
            0..=3 => (["palm-straight", "palm-bend", "palm-detailed-straight", "palm-detailed-bend"][variant % 4], 0.3),
            4..=5 => (["rocks-sand-a", "rocks-sand-b", "rocks-sand-c"][variant % 3], 0.22),
            6 => (["crate", "crate-bottles"][variant % 2], 0.5),
            7 => ("barrel", 0.45),
            8 => (["cannon", "chest"][variant % 2], 0.45),
            _ => ("flag-pirate-high", 0.3),
        },
        Scenery::Deco => (["grass-plant", "bottle", "cannon-ball"][variant % 3], 0.4),
    };
    let scene = server.load(GltfAssetLabel::Scene(0).from_asset(format!("kenney/pirate-kit/{nom}.glb")));
    let tour = Quat::from_rotation_y(variant as f32 * 1.3);
    c.spawn((WorldAssetRoot(scene), Transform::from_translation(at).with_rotation(tour).with_scale(Vec3::splat(echelle)), ChildOf(parent)));
    true
}

/// Tonneau flottant sur lequel on saute (le dessus affleure à ≈ 0,16 comme les autres nénuphars).
pub fn pirate_pad(c: &mut Commands, server: &AssetServer, parent: Entity) {
    let scene = server.load(GltfAssetLabel::Scene(0).from_asset("kenney/pirate-kit/barrel.glb"));
    let echelle = 0.45;
    c.spawn((WorldAssetRoot(scene), Transform::from_xyz(0.0, 0.16 - 1.23 * echelle, 0.0).with_scale(Vec3::splat(echelle)), ChildOf(parent)));
}

/// Radeau de `len` cases : des plateformes à planches (`platform-planks`) mises bout à bout.
pub fn pirate_raft(c: &mut Commands, server: &AssetServer, parent: Entity, len: f32) {
    const LARGEUR: f32 = 0.4; // échelle : 1,9 de large -> 0,76 (une voie)
    const MODULE: f32 = 2.7 * LARGEUR; // longueur d'une plateforme une fois mise à l'échelle
    let n = (len / MODULE).round().max(1.0) as usize;
    let pas = len / n as f32;
    let scene: Handle<WorldAsset> = server.load(GltfAssetLabel::Scene(0).from_asset("kenney/pirate-kit/platform-planks.glb"));
    for i in 0..n {
        let x = -len / 2.0 + pas * (i as f32 + 0.5);
        // Le dessus du radeau affleure à ≈ 0,06 ; les modèles Kenney regardent vers +Z, le jeu avance vers +X.
        c.spawn((
            WorldAssetRoot(scene.clone()),
            Transform::from_xyz(x, 0.06 - 0.43 * LARGEUR, 0.0).with_rotation(Quat::from_rotation_y(std::f32::consts::FRAC_PI_2)).with_scale(Vec3::new(LARGEUR, LARGEUR, LARGEUR * pas / MODULE)),
            ChildOf(parent),
        ));
    }
}

// ---- Halloween (Graveyard Kit) ----

/// Ce qui roule ou marche sur la route d'un monde : (modèle, échelle, longueur). Vide : le monde garde ses véhicules.
pub fn trafic_modeles(biome: crate::extra::Biome) -> &'static [(&'static str, f32, f32)] {
    use crate::extra::Biome;
    match biome {
        Biome::Pirate => &PIRATE_VEHICLES,
        Biome::Halloween => &HALLOWEEN_WALKERS,
        _ => &[],
    }
}

/// Pose le n° `i` du trafic du monde.
pub fn trafic(c: &mut Commands, server: &AssetServer, graphs: &mut Assets<AnimationGraph>, parent: Entity, biome: crate::extra::Biome, i: usize) -> bool {
    use crate::extra::Biome;
    match biome {
        Biome::Pirate => pirate_vehicle(c, server, parent, i),
        Biome::Halloween => marcheur(c, server, graphs, parent, i),
        _ => false,
    }
}

/// Le décor du monde (obstacles, bords, petits détails) ; `false` s'il n'y a pas de modèle Kenney pour ce monde.
#[allow(clippy::too_many_arguments)]
pub fn decor(c: &mut Commands, server: &AssetServer, parent: Entity, biome: crate::extra::Biome, kind: crate::extra::Scenery, at: Vec3, roll: u32, variant: usize) -> bool {
    use crate::extra::Biome;
    match biome {
        Biome::Pirate => pirate_scenery(c, server, parent, kind, at, roll, variant),
        Biome::Halloween => halloween_scenery(c, server, parent, kind, at, roll, variant),
        _ => false,
    }
}

/// Les morts-vivants qui traversent la route : (modèle, échelle, longueur).
pub const HALLOWEEN_WALKERS: [(&str, f32, f32); 5] = [
    ("character-zombie", 1.4, 0.8),
    ("character-skeleton", 1.4, 0.8),
    ("character-ghost", 1.4, 0.8),
    ("character-vampire", 1.4, 0.8),
    ("character-zombie", 1.5, 0.8),
];

/// Un personnage du Graveyard Kit qui marche : l'animation « walk » à lancer quand son squelette apparaît.
#[derive(Component)]
pub struct Marcheur {
    graph: Handle<AnimationGraph>,
    node: AnimationNodeIndex,
}

fn marcheur(c: &mut Commands, server: &AssetServer, graphs: &mut Assets<AnimationGraph>, parent: Entity, i: usize) -> bool {
    let (nom, echelle, _) = HALLOWEEN_WALKERS[i];
    let fichier = format!("kenney/graveyard-kit/{nom}.glb");
    let scene = server.load(GltfAssetLabel::Scene(0).from_asset(fichier.clone()));
    let (graph, node) = AnimationGraph::from_clip(server.load(GltfAssetLabel::Animation(2).from_asset(fichier)));
    // Les personnages Kenney regardent vers +Z, le jeu avance vers +X.
    c.spawn((
        WorldAssetRoot(scene),
        Transform::from_rotation(Quat::from_rotation_y(std::f32::consts::FRAC_PI_2)).with_scale(Vec3::splat(echelle)),
        Marcheur { graph: graphs.add(graph), node },
        ChildOf(parent),
    ));
    true
}

/// Lance la marche des personnages dès que leur squelette est chargé.
pub fn animer_marcheurs(
    mut commands: Commands,
    mut lecteurs: Query<(Entity, &mut AnimationPlayer), Added<AnimationPlayer>>,
    parents: Query<&ChildOf>,
    marcheurs: Query<&Marcheur>,
) {
    for (e, mut lecteur) in &mut lecteurs {
        let mut cur = e;
        loop {
            if let Ok(m) = marcheurs.get(cur) {
                commands.entity(e).insert(AnimationGraphHandle(m.graph.clone()));
                lecteur.play(m.node).repeat();
                break;
            }
            match parents.get(cur) {
                Ok(p) => cur = p.parent(),
                Err(_) => break,
            }
        }
    }
}

fn halloween_scenery(c: &mut Commands, server: &AssetServer, parent: Entity, kind: crate::extra::Scenery, at: Vec3, roll: u32, variant: usize) -> bool {
    use crate::extra::Scenery;
    let (nom, echelle) = match kind {
        Scenery::Block | Scenery::Side => match roll {
            0..=2 => (["pine", "pine-crooked", "pine-fall", "pine-fall-crooked"][variant % 4], 0.8),
            3..=5 => (["gravestone-bevel", "gravestone-cross", "gravestone-round", "gravestone-roof", "gravestone-wide", "gravestone-decorative", "gravestone-broken", "gravestone-cross-large"][variant % 8], 1.3),
            6 => (["cross", "cross-wood", "pillar-obelisk"][variant % 3], 0.9),
            7 => ("crypt", 1.0),
            8 => (["rocks-tall", "trunk-long", "hay-bale"][variant % 3], 1.0),
            _ => (["lightpost-single", "fire-basket"][variant % 2], 1.0),
        },
        Scenery::Deco => (["pumpkin", "pumpkin-carved", "candle-multiple", "urn-round", "shovel-dirt", "debris"][variant % 6], 1.2),
    };
    let scene = server.load(GltfAssetLabel::Scene(0).from_asset(format!("kenney/graveyard-kit/{nom}.glb")));
    let tour = Quat::from_rotation_y(variant as f32 * 1.3);
    c.spawn((WorldAssetRoot(scene), Transform::from_translation(at).with_rotation(tour).with_scale(Vec3::splat(echelle)), ChildOf(parent)));
    true
}

/// Citrouille flottante sur laquelle on saute (dessus à ≈ 0,16).
pub fn halloween_pad(c: &mut Commands, server: &AssetServer, parent: Entity) {
    let scene = server.load(GltfAssetLabel::Scene(0).from_asset("kenney/graveyard-kit/pumpkin.glb"));
    let echelle = 1.9;
    c.spawn((WorldAssetRoot(scene), Transform::from_xyz(0.0, 0.16 - 0.32 * echelle, 0.0).with_scale(Vec3::splat(echelle)), ChildOf(parent)));
}

/// Cercueils flottants mis bout à bout sur `len` cases.
pub fn halloween_coffins(c: &mut Commands, server: &AssetServer, parent: Entity, len: f32) {
    const ECHELLE: f32 = 1.1;
    let n = (len / (0.84 * ECHELLE)).round().max(1.0) as usize;
    let pas = len / n as f32;
    for i in 0..n {
        let nom = if (i + len as usize) % 2 == 0 { "coffin" } else { "coffin-old" };
        let scene: Handle<WorldAsset> = server.load(GltfAssetLabel::Scene(0).from_asset(format!("kenney/graveyard-kit/{nom}.glb")));
        let x = -len / 2.0 + pas * (i as f32 + 0.5);
        c.spawn((
            WorldAssetRoot(scene),
            Transform::from_xyz(x, 0.06 - 0.2 * ECHELLE, 0.0).with_rotation(Quat::from_rotation_y(std::f32::consts::FRAC_PI_2)).with_scale(Vec3::new(ECHELLE, ECHELLE, ECHELLE * pas / (0.84 * ECHELLE))),
            ChildOf(parent),
        ));
    }
}
