//! Essai « Kenney » (`TRAVERSEE_KENNEY=1`) : les voitures du Car Kit de Kenney (CC0, `assets/kenney/car-kit`)
//! remplacent les voitures en pièces sur les routes des mondes sans thème.
use crate::extra::Vehicle;
use bevy::asset::RenderAssetUsages;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};

const CARS: [&str; 6] = ["sedan", "hatchback-sports", "suv", "sedan-sports", "van", "suv-luxury"];
const TRUCKS: [&str; 4] = ["truck", "delivery", "garbage-truck", "firetruck"];

/// Pose un modèle Kenney à la place du véhicule ; renvoie `false` s'il n'y en a pas pour ce véhicule.
pub fn car(c: &mut Commands, server: &AssetServer, e: Option<&Eclairage>, parent: Entity, v: Vehicle, color: usize) -> bool {
    let (nom, echelle) = match v {
        Vehicle::Car => (CARS[color % CARS.len()], 0.8),
        Vehicle::Truck => (TRUCKS[color % TRUCKS.len()], 0.8),
        Vehicle::Police => ("police", 0.8),
        Vehicle::Taxi => ("taxi", 0.8),
        Vehicle::Sport => ("race", 0.8),
        Vehicle::Tractor => ("tractor", 0.8),
        Vehicle::Pickup => ("truck-flat", 0.8),
        Vehicle::Bus => ("ambulance", 0.8),
        Vehicle::Moto => ("kart-oobi", 0.7),
        Vehicle::Sheriff => ("police", 0.8),
        _ => return false,
    };
    let scene = server.load(GltfAssetLabel::Scene(0).from_asset(format!("kenney/car-kit/{nom}.glb")));
    // Les modèles Kenney regardent vers +Z ; le jeu avance vers +X.
    c.spawn((WorldAssetRoot(scene), Transform::from_rotation(Quat::from_rotation_y(std::f32::consts::FRAC_PI_2)).with_scale(Vec3::splat(echelle)), ChildOf(parent)));
    // Les phares ne s'allument que la nuit (invisibles le jour) ; pas dans la Collection.
    if let Some(e) = e {
        phares(c, e, parent, v.len());
    }
    true
}

/// Arbres et buissons du Mini Forest (`assets/kenney/mini-forest`) ; `false` s'il n'y en a pas.
pub fn prop(c: &mut Commands, server: &AssetServer, parent: Entity, p: crate::models::Prop, at: Vec3, variant: usize) -> bool {
    use crate::models::Prop;
    let nom = match p {
        Prop::Tree => "tree",
        Prop::Pine => "tree-high",
        // Les rochers du Mini Forest sortent noirs et ceux du Nature Kit turquoise et orange dans Bevy : on garde ceux du jeu (gris).
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
        Biome::Medieval => &MEDIEVAL_SIEGE,
        Biome::Space => &SPACE_TRAFFIC,
        Biome::FarWest => &FARWEST_ANIMAUX,
        Biome::Cyber => &CYBER_TRAFIC,
        _ => &[],
    }
}

/// Pose le n° `i` du trafic du monde.
pub fn trafic(c: &mut Commands, server: &AssetServer, graphs: &mut Assets<AnimationGraph>, e: Option<&Eclairage>, parent: Entity, biome: crate::extra::Biome, i: usize) -> bool {
    use crate::extra::Biome;
    match biome {
        Biome::Pirate => pirate_vehicle(c, server, parent, i),
        Biome::Halloween => marcheur(c, server, graphs, parent, i),
        Biome::Medieval => engin(c, server, parent, i),
        Biome::Space => vaisseau(c, server, parent, i),
        Biome::FarWest => animal_ouest(c, server, graphs, parent, i),
        Biome::Cyber => voiture_neon(c, server, e, parent, i),
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
        Biome::Medieval => medieval_scenery(c, server, parent, kind, at, roll, variant),
        Biome::Space => space_scenery(c, server, parent, kind, at, roll, variant),
        Biome::FarWest => farwest_scenery(c, server, parent, kind, at, roll, variant),
        Biome::Cyber => cyber_scenery(c, server, parent, kind, at, roll, variant),
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
    /// Vitesse au sol (cases par seconde) pour laquelle l'animation est à vitesse normale ; 0 : toujours à vitesse normale.
    /// Au-dessus ou en dessous, l'animation est accélérée ou ralentie pour que les pas suivent le déplacement.
    v_ref: f32,
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
        Marcheur { graph: graphs.add(graph), node, v_ref: 0.0 },
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
    movers: Query<&crate::lanes::Mover>,
) {
    for (e, mut lecteur) in &mut lecteurs {
        let mut cur = e;
        loop {
            if let Ok(m) = marcheurs.get(cur) {
                commands.entity(e).insert(AnimationGraphHandle(m.graph.clone()));
                // Vitesse de l'animation proportionnelle à celle du déplacement (celle du véhicule qui porte l'animal).
                let mut vitesse = 1.0;
                if m.v_ref > 0.0 {
                    let mut haut = cur;
                    loop {
                        if let Ok(mv) = movers.get(haut) {
                            vitesse = (mv.speed.abs() / m.v_ref).clamp(0.25, 1.6);
                            break;
                        }
                        match parents.get(haut) {
                            Ok(p) => haut = p.parent(),
                            Err(_) => break,
                        }
                    }
                }
                lecteur.play(m.node).repeat().set_speed(vitesse);
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

// ---- Moyen Âge (Castle Kit) ----

/// Les machines de siège qui roulent sur le chemin : (modèle, échelle, longueur).
pub const MEDIEVAL_SIEGE: [(&str, f32, f32); 5] = [
    ("siege-ram", 0.9, 1.9),
    ("siege-catapult", 0.85, 1.8),
    ("siege-ballista", 1.0, 1.3),
    ("siege-trebuchet", 0.75, 1.5),
    ("siege-tower", 0.65, 1.3),
];

/// Une machine de siège ; ces modèles sont déjà tournés vers +X (arrière du côté -X), roues sous le sol de 0,2.
fn engin(c: &mut Commands, server: &AssetServer, parent: Entity, i: usize) -> bool {
    let (nom, echelle, _) = MEDIEVAL_SIEGE[i];
    let scene = server.load(GltfAssetLabel::Scene(0).from_asset(format!("kenney/castle-kit/{nom}.glb")));
    c.spawn((WorldAssetRoot(scene), Transform::from_xyz(0.0, 0.2 * echelle, 0.0).with_scale(Vec3::splat(echelle)), ChildOf(parent)));
    true
}

fn medieval_scenery(c: &mut Commands, server: &AssetServer, parent: Entity, kind: crate::extra::Scenery, at: Vec3, roll: u32, variant: usize) -> bool {
    use crate::extra::Scenery;
    let (nom, echelle) = match kind {
        Scenery::Block | Scenery::Side => match roll {
            0..=3 => (["tree-large", "tree-small"][variant % 2], 1.0),
            4 => (["rocks-large", "rocks-small"][variant % 2], 0.9),
            5 => ("tower-square", 0.8),
            6 => (["wall", "wall-pillar"][variant % 2], 0.9),
            7 => (["flag-pennant", "flag", "flag-wide"][variant % 3], 1.3),
            8 => (["tree-trunk", "tree-log"][variant % 2], 1.0),
            _ => ("tower-hexagon-base", 0.8),
        },
        Scenery::Deco => (["tree-trunk", "rocks-small", "tree-log"][variant % 3], 0.5),
    };
    let scene = server.load(GltfAssetLabel::Scene(0).from_asset(format!("kenney/castle-kit/{nom}.glb")));
    let tour = Quat::from_rotation_y(variant as f32 * 1.3);
    c.spawn((WorldAssetRoot(scene), Transform::from_translation(at).with_rotation(tour).with_scale(Vec3::splat(echelle)), ChildOf(parent)));
    true
}

/// Souche flottante sur laquelle on saute (dessus à ≈ 0,16).
pub fn medieval_pad(c: &mut Commands, server: &AssetServer, parent: Entity) {
    let scene = server.load(GltfAssetLabel::Scene(0).from_asset("kenney/castle-kit/tree-trunk.glb"));
    let echelle = 1.6;
    c.spawn((WorldAssetRoot(scene), Transform::from_xyz(0.0, 0.16 - 0.38 * echelle, 0.0).with_scale(Vec3::splat(echelle)), ChildOf(parent)));
}

/// Radeau de rondins : trois rangées de `tree-log` (1 de long, 0,25 de large) bout à bout sur `len` cases.
pub fn medieval_raft(c: &mut Commands, server: &AssetServer, parent: Entity, len: f32) {
    let n = len.round().max(1.0) as usize;
    let pas = len / n as f32;
    let scene: Handle<WorldAsset> = server.load(GltfAssetLabel::Scene(0).from_asset("kenney/castle-kit/tree-log.glb"));
    for i in 0..n {
        let x = -len / 2.0 + pas * (i as f32 + 0.5);
        for z in [-0.26, 0.0, 0.26] {
            c.spawn((
                WorldAssetRoot(scene.clone()),
                Transform::from_xyz(x, 0.06 - 0.17, z).with_rotation(Quat::from_rotation_y(std::f32::consts::FRAC_PI_2)).with_scale(Vec3::new(1.0, 1.0, pas)),
                ChildOf(parent),
            ));
        }
    }
}

/// Une cavalerie au galop à la place du train du Moyen Âge : une file de chevaliers (cheval du Prototype Kit,
/// soldat du Mini Arena assis dessus). Renvoie la longueur de la file.
pub fn cavalerie(c: &mut Commands, server: &AssetServer, graphs: &mut Assets<AnimationGraph>, parent: Entity, wagons: usize) -> f32 {
    const PAS: f32 = 2.5;
    let n = wagons * 2 + 2;
    let len = n as f32 * PAS;
    for i in 0..n {
        let x = len / 2.0 - PAS / 2.0 - i as f32 * PAS;
        // Le cheval regarde vers +X ; le soldat (qui regarde vers +Z) est tourné de même et assis sur le dos.
        anime(c, server, graphs, parent, "kenney/prototype-kit/animal-horse.glb", 1, Transform::from_xyz(x, 0.3, 0.0).with_scale(Vec3::splat(2.0)), 0.0);
        anime(c, server, graphs, parent, "kenney/mini-arena/character-soldier.glb", 7, Transform::from_xyz(x, 0.55, 0.0).with_rotation(Quat::from_rotation_y(std::f32::consts::FRAC_PI_2)).with_scale(Vec3::splat(0.7)), 0.0);
    }
    len
}

/// Un modèle animé en boucle (`anim` : numéro de l'animation dans le GLB).
fn anime(c: &mut Commands, server: &AssetServer, graphs: &mut Assets<AnimationGraph>, parent: Entity, fichier: &str, anim: usize, tf: Transform, v_ref: f32) {
    let scene = server.load(GltfAssetLabel::Scene(0).from_asset(fichier.to_string()));
    let (graph, node) = AnimationGraph::from_clip(server.load(GltfAssetLabel::Animation(anim).from_asset(fichier.to_string())));
    c.spawn((WorldAssetRoot(scene), tf, Marcheur { graph: graphs.add(graph), node, v_ref }, ChildOf(parent)));
}

// ---- Espace (Space Kit) ----

/// Les GLB du Space Kit ont leur modèle décalé de (2, 0, 1,5) dans la scène : on le recentre dans un porteur,
/// qui reçoit la vraie position, rotation et échelle.
fn space_modele(c: &mut Commands, scene: Handle<WorldAsset>, parent: Entity, tf: Transform) {
    let porteur = c.spawn((tf, Visibility::default(), ChildOf(parent))).id();
    c.spawn((WorldAssetRoot(scene), Transform::from_xyz(-2.0, 0.0, -1.5), ChildOf(porteur)));
}

/// Ce qui file sur la voie de métal : (modèle, échelle, longueur).
pub const SPACE_TRAFFIC: [(&str, f32, f32); 8] = [
    ("craft_speederA", 0.5, 1.2),
    ("craft_speederB", 0.5, 1.2),
    ("craft_racer", 0.5, 1.1),
    ("craft_cargoA", 0.5, 1.3),
    ("craft_cargoB", 0.5, 1.3),
    ("craft_miner", 0.45, 1.3),
    ("alien", 0.5, 0.8),
    ("rover", 2.5, 0.9),
];

/// Un vaisseau (ou un extraterrestre, un rover) : les modèles du Space Kit regardent vers -Z, le jeu avance vers +X.
fn vaisseau(c: &mut Commands, server: &AssetServer, parent: Entity, i: usize) -> bool {
    let (nom, echelle, _) = SPACE_TRAFFIC[i];
    let scene = server.load(GltfAssetLabel::Scene(0).from_asset(format!("kenney/space-kit/{nom}.glb")));
    space_modele(c, scene, parent, Transform::from_xyz(0.0, 0.1, 0.0).with_rotation(Quat::from_rotation_y(-std::f32::consts::FRAC_PI_2)).with_scale(Vec3::splat(echelle)));
    true
}

fn space_scenery(c: &mut Commands, server: &AssetServer, parent: Entity, kind: crate::extra::Scenery, at: Vec3, roll: u32, variant: usize) -> bool {
    use crate::extra::Scenery;
    let (nom, echelle) = match kind {
        Scenery::Block | Scenery::Side => match roll {
            0..=3 => (["meteor", "meteor_detailed", "rock_largeA", "rock_largeB"][variant % 4], 1.0),
            4..=5 => (["rock_crystalsLargeA", "rock_crystalsLargeB"][variant % 2], 1.0),
            6 => ("satelliteDish", 1.1),
            7 => ("turret_single", 1.1),
            8 => (["structure", "hangar_smallA"][variant % 2], 0.8),
            _ => ("barrels", 1.4),
        },
        Scenery::Deco => (["rocks_smallA", "rocks_smallB", "crater"][variant % 3], 0.6),
    };
    let scene = server.load(GltfAssetLabel::Scene(0).from_asset(format!("kenney/space-kit/{nom}.glb")));
    let tour = Quat::from_rotation_y(variant as f32 * 1.3);
    space_modele(c, scene, parent, Transform::from_translation(at).with_rotation(tour).with_scale(Vec3::splat(echelle)));
    true
}

/// Plateforme flottant dans le vide, sur laquelle on saute.
pub fn space_pad(c: &mut Commands, server: &AssetServer, parent: Entity) {
    let scene = server.load(GltfAssetLabel::Scene(0).from_asset("kenney/space-kit/platform_small.glb"));
    space_modele(c, scene, parent, Transform::from_xyz(0.0, 0.06 - 0.1 * 0.8, 0.0).with_scale(Vec3::splat(0.8)));
}

/// Plateforme longue qui glisse dans le vide : des `platform_long` (2 de long, 1 de large) bout à bout.
pub fn space_cargo(c: &mut Commands, server: &AssetServer, parent: Entity, len: f32) {
    const LARGEUR: f32 = 0.75;
    const MODULE: f32 = 2.0 * LARGEUR;
    let n = (len / MODULE).round().max(1.0) as usize;
    let pas = len / n as f32;
    let scene: Handle<WorldAsset> = server.load(GltfAssetLabel::Scene(0).from_asset("kenney/space-kit/platform_long.glb"));
    for i in 0..n {
        let x = -len / 2.0 + pas * (i as f32 + 0.5);
        space_modele(c, scene.clone(), parent, Transform::from_xyz(x, 0.06 - 0.1 * LARGEUR, 0.0).with_scale(Vec3::new(LARGEUR * pas / MODULE, LARGEUR, LARGEUR)));
    }
}

/// Un monorail à la place du train de l'espace : motrice, wagons, queue (1 de long chacun). Renvoie la longueur.
pub fn monorail(c: &mut Commands, server: &AssetServer, parent: Entity, wagons: usize) -> f32 {
    let n = wagons + 2;
    let len = n as f32;
    let wagon = ["monorail_trainPassenger", "monorail_trainCargo", "monorail_trainBox"];
    for i in 0..n {
        let nom = if i == 0 { "monorail_trainFront" } else if i == n - 1 { "monorail_trainEnd" } else { wagon[i % wagon.len()] };
        let x = len / 2.0 - 0.5 - i as f32;
        let scene: Handle<WorldAsset> = server.load(GltfAssetLabel::Scene(0).from_asset(format!("kenney/space-kit/{nom}.glb")));
        // Comme les vaisseaux : -Z vers +X.
        space_modele(c, scene, parent, Transform::from_xyz(x, 0.3, 0.0).with_rotation(Quat::from_rotation_y(-std::f32::consts::FRAC_PI_2)).with_scale(Vec3::splat(1.2)));
    }
    len
}

// ---- Cadeaux (Holiday Kit) ----

/// Le cadeau brille : les matières de son modèle deviennent lumineuses (couleurs conservées) dès que le modèle est chargé.
#[derive(Component)]
pub struct Brille(pub f32);

/// Une étincelle qui scintille autour du cadeau.
#[derive(Component)]
pub struct Etincelle {
    phase: f32,
    rayon: f32,
}

/// La lumière du cadeau, qui pulse doucement.
#[derive(Component)]
pub struct LueurCadeau {
    phase: f32,
    base: f32,
    dore: bool,
}

/// Les couleurs de la lumière des bonus : blanc, rouge, bleu, vert, jaune, rose.
const TEINTES_LUEUR: [Color; 6] = [
    Color::srgb(1.0, 1.0, 1.0),
    Color::srgb(1.0, 0.25, 0.22),
    Color::srgb(0.25, 0.5, 1.0),
    Color::srgb(0.3, 1.0, 0.4),
    Color::srgb(1.0, 0.85, 0.2),
    Color::srgb(1.0, 0.4, 0.85),
];

/// Marque la nourriture dorée : sa matière est teintée d'or en plus de briller.
#[derive(Component)]
pub struct Dore;

/// Les bonus de chaque monde : quatre mets, tirés au hasard (modèles du Food Kit, tous inscrits dans -1..1).
fn nourritures(biome: crate::extra::Biome) -> [&'static str; 4] {
    use crate::extra::Biome::*;
    match biome {
        Prairie => ["apple", "carrot", "strawberry", "corn"],
        Desert => ["watermelon", "pineapple", "banana", "coconut"],
        Snow => ["cupcake", "ice-cream", "ginger-bread", "chocolate"],
        Autumn => ["pumpkin", "mushroom", "grapes", "pear"],
        Jurassic => ["banana", "coconut", "broccoli", "mushroom"],
        Nippon => ["sushi-salmon", "maki-salmon", "rice-ball", "dim-sum"],
        Space | Cyber => ["pizza", "burger", "fries", "soda"],
        FarWest => ["burger", "hot-dog", "corn", "taco"],
        Halloween => ["pumpkin", "candy-bar", "donut", "mushroom"],
        Medieval => ["bread", "cheese", "whole-ham", "mincemeat-pie"],
        Pirate => ["fish", "cherries", "pineapple", "coconut"],
    }
}

/// Centre et plus grande dimension de chaque modèle du Food Kit (ils ne sont pas à la même échelle ni centrés) : on les ramène à la même taille.
const TAILLES: [(&str, [f32; 3], f32); 36] = [
    ("apple", [0.000, 0.095, 0.000], 0.196),
    ("banana", [0.000, 0.098, 0.000], 0.630),
    ("bread", [0.000, 0.020, 0.000], 0.442),
    ("broccoli", [0.000, 0.211, 0.000], 0.422),
    ("burger", [0.000, 0.145, 0.000], 0.404),
    ("candy-bar", [0.000, 0.033, 0.000], 0.292),
    ("carrot", [0.000, 0.358, 0.000], 0.716),
    ("cheese", [0.000, 0.110, 0.000], 0.680),
    ("cherries", [0.000, 0.085, 0.000], 0.171),
    ("chocolate", [0.000, 0.017, 0.000], 0.292),
    ("coconut", [0.000, 0.124, 0.000], 0.248),
    ("corn", [-0.007, 0.158, -0.006], 0.343),
    ("cupcake", [0.000, 0.212, 0.000], 0.424),
    ("dim-sum", [0.000, 0.060, 0.000], 0.186),
    ("donut", [0.000, 0.044, 0.000], 0.283),
    ("eggplant", [0.000, 0.203, 0.000], 0.405),
    ("fish", [0.000, 0.161, 0.000], 0.621),
    ("fries", [0.001, 0.199, 0.000], 0.398),
    ("ginger-bread", [0.000, 0.015, 0.000], 0.260),
    ("grapes", [0.000, 0.211, 0.000], 0.423),
    ("hot-dog", [0.000, 0.076, 0.000], 0.586),
    ("ice-cream", [0.000, 0.232, 0.000], 0.463),
    ("maki-salmon", [0.000, 0.035, 0.000], 0.120),
    ("mincemeat-pie", [0.000, 0.084, 0.000], 0.613),
    ("mushroom", [0.000, 0.070, 0.000], 0.144),
    ("pear", [0.000, 0.135, 0.000], 0.271),
    ("pineapple", [0.000, 0.236, 0.000], 0.472),
    ("pizza", [0.000, 0.022, 0.000], 0.842),
    ("pumpkin", [0.000, 0.115, 0.000], 0.419),
    ("rice-ball", [0.000, 0.065, 0.000], 0.131),
    ("soda", [0.000, 0.216, 0.000], 0.432),
    ("strawberry", [0.000, 0.091, 0.000], 0.182),
    ("sushi-salmon", [0.000, 0.045, 0.000], 0.201),
    ("taco", [0.000, 0.131, 0.000], 0.426),
    ("watermelon", [0.000, 0.238, 0.000], 0.476),
    ("whole-ham", [0.000, 0.220, 0.000], 0.640),
];

/// Un bonus de points à ramasser (le doré vaut plus) : un mets du monde, qui brille, tourne lentement, avec une vraie
/// lumière chaude autour de lui et des étincelles. Il garde le composant `Fly` : ramassage et points comme les mouches.
#[allow(clippy::too_many_arguments)]
pub fn cadeau(c: &mut Commands, k: &crate::models::Kit, x: &crate::extra::Extra, server: &AssetServer, lane: Entity, row: i32, col: i32, gold: bool, phase: f32, biome: crate::extra::Biome) {
    let nom = nourritures(biome)[(phase * 10.0) as usize % 4];
    let scene = server.load(GltfAssetLabel::Scene(0).from_asset(format!("kenney/food-kit/{nom}.glb")));
    let fly = c
        .spawn((
            Transform::from_xyz(col as f32, 0.7, 0.0),
            Visibility::default(),
            crate::extra::Fly { row, col, gold, phase },
            crate::extra::Tourne,
            ChildOf(lane),
        ))
        .id();
    let (_, centre, taille) = TAILLES.iter().find(|t| t.0 == nom).copied().unwrap_or((nom, [0.0; 3], 1.0));
    let echelle = (if gold { 1.0 } else { 0.75 }) / taille;
    // Le pivot gonfle et dégonfle comme un ressort : « viens me prendre ! »
    let pivot = c.spawn((Transform::default(), Visibility::default(), Ressort { phase }, ChildOf(fly))).id();
    let mut modele = c.spawn((WorldAssetRoot(scene), Transform::from_translation(-Vec3::from(centre) * echelle).with_scale(Vec3::splat(echelle)), Brille(0.5), ChildOf(pivot)));
    if gold {
        modele.insert(Dore);
    }
    let couleur = if gold { Color::srgb(1.0, 0.8, 0.25) } else { Color::srgb(1.0, 0.93, 0.7) };
    let base = if gold { 160_000.0 } else { 110_000.0 };
    c.spawn((PointLight { color: couleur, intensity: base, range: 4.5, shadow_maps_enabled: false, ..default() }, Transform::from_xyz(0.0, 0.5, 0.0), LueurCadeau { phase, base, dore: gold }, ChildOf(fly)));
    // Pas de disque visible : la lumière colorée du bonus éclaire l'herbe autour de lui, et les étincelles tournent.
    for i in 0..6 {
        let a = i as f32 * 1.047 + phase;
        c.spawn((
            Mesh3d(k.ball.clone()),
            MeshMaterial3d(x.arcs[i % x.arcs.len()].1.clone()),
            Transform::from_xyz(a.cos() * 0.6, 0.0, a.sin() * 0.6).with_scale(Vec3::splat(0.08)),
            Etincelle { phase: a, rayon: 0.6 + 0.12 * (i % 2) as f32 },
            ChildOf(fly),
        ));
    }
}

/// Fait gonfler et dégonfler les bonus, en rebond (ressort).
#[derive(Component)]
pub struct Ressort {
    phase: f32,
}

pub fn ressorts(time: Res<Time>, mut q: Query<(&Ressort, &mut Transform)>) {
    let t = time.elapsed_secs();
    for (r, mut tf) in &mut q {
        let s = 1.0 + 0.2 * (t * 4.5 + r.phase).sin();
        tf.scale = Vec3::new(s, 2.0 - s, s);
    }
}

/// Fait scintiller : les étincelles tournent autour du cadeau en grossissant et rapetissant, la lumière pulse.
pub fn scintiller(time: Res<Time>, mut etincelles: Query<(&Etincelle, &mut Transform), Without<LueurCadeau>>, mut lueurs: Query<(&LueurCadeau, &mut PointLight)>) {
    let t = time.elapsed_secs();
    for (e, mut tf) in &mut etincelles {
        let a = t * 1.6 + e.phase;
        tf.translation = Vec3::new(a.cos() * e.rayon, (t * 2.3 + e.phase * 3.0).sin() * 0.25, a.sin() * e.rayon);
        tf.scale = Vec3::splat(0.03 + 0.05 * ((t * 5.0 + e.phase * 2.0).sin() * 0.5 + 0.5));
    }
    for (l, mut light) in &mut lueurs {
        light.intensity = l.base * (0.8 + 0.25 * (t * 3.0 + l.phase).sin());
        if !l.dore {
            light.color = TEINTES_LUEUR[((t * 2.0 + l.phase) as usize) % TEINTES_LUEUR.len()];
        }
    }
}

/// Rend lumineuses les matières des modèles marqués `Brille` : la couleur du modèle sert aussi d'émission.
pub fn faire_briller(
    racines: Query<(Entity, &Brille, Has<Dore>)>,
    enfants: Query<&Children>,
    mut meshes: Query<&mut MeshMaterial3d<StandardMaterial>, Without<Brille>>,
    mut mats: ResMut<Assets<StandardMaterial>>,
    mut faites: Local<std::collections::HashMap<(AssetId<StandardMaterial>, u32), Handle<StandardMaterial>>>,
) {
    for (racine, force, dore) in &racines {
        for e in enfants.iter_descendants(racine) {
            let Ok(mut m) = meshes.get_mut(e) else { continue };
            let origine = m.0.id();
            if faites.values().any(|h| h.id() == origine) {
                continue; // déjà une matière lumineuse
            }
            let cle = (origine, force.0.to_bits() + dore as u32);
            let lumineuse = match faites.get(&cle) {
                Some(h) => h.clone(),
                None => {
                    let Some(base) = mats.get(&m.0).cloned() else { continue };
                    let or = if dore { Color::srgb(1.0, 0.78, 0.25) } else { Color::WHITE };
                    let h = mats.add(StandardMaterial { emissive: if dore { LinearRgba::rgb(force.0, force.0 * 0.8, force.0 * 0.3) } else { LinearRgba::rgb(force.0, force.0, force.0) }, emissive_texture: base.base_color_texture.clone(), base_color: if dore { or } else { base.base_color }, ..base });
                    faites.insert(cle, h.clone());
                    h
                }
            };
            m.0 = lumineuse;
        }
    }
}

// ---- Trains (Train Kit) ----

/// Un train Kenney à la place du train en pièces : locomotive + `wagons` + 1 wagons, la tête vers +X. Renvoie la longueur.
/// Les modèles du Train Kit sont longs de 2,6 le long de Z et regardent vers +Z (hauteur des roues : -0,36) ; on les réduit à 0,7.
pub fn train(c: &mut Commands, server: &AssetServer, parent: Entity, wagons: usize, biome: crate::extra::Biome) -> f32 {
    use crate::extra::Biome;
    const ECHELLE: f32 = 0.7;
    const PAS: f32 = 1.95;
    let (loco, wagon): (&str, &[&str]) = match biome {
        Biome::Pirate => ("train-locomotive-b", &["train-carriage-lumber", "train-carriage-flatbed-wood", "train-carriage-dirt", "train-carriage-wood"]),
        Biome::Halloween => ("train-locomotive-c", &["train-carriage-box", "train-carriage-dirt", "train-carriage-tank"]),
        _ => ("train-locomotive-a", &["train-carriage-coal", "train-carriage-box", "train-carriage-wood", "train-carriage-lumber", "train-carriage-tank", "train-carriage-flatbed", "train-carriage-container-red", "train-carriage-container-blue"]),
    };
    let n = wagons + 1;
    let len = n as f32 * PAS;
    for i in 0..n {
        let nom = if i == 0 { loco } else { wagon[(i - 1 + wagons) % wagon.len()] };
        let x = len / 2.0 - PAS / 2.0 - i as f32 * PAS;
        let scene: Handle<WorldAsset> = server.load(GltfAssetLabel::Scene(0).from_asset(format!("kenney/train-kit/{nom}.glb")));
        c.spawn((
            WorldAssetRoot(scene),
            Transform::from_xyz(x, 0.34, 0.0).with_rotation(Quat::from_rotation_y(std::f32::consts::FRAC_PI_2)).with_scale(Vec3::splat(ECHELLE)),
            ChildOf(parent),
        ));
    }
    len
}

/// Les rails du Train Kit (`railroad-straight`, 1 de large, 4 de long le long de Z, posé à y = -1 dans son fichier) mis bout à bout
/// sur toute la voie, de -`reach` à `reach`.
pub fn rails(c: &mut Commands, server: &AssetServer, parent: Entity, reach: f32) {
    const ECHELLE: f32 = 0.7; // comme les trains : les roues tombent sur les rails
    const MODULE: f32 = 4.0 * ECHELLE;
    let n = (2.0 * reach / MODULE).ceil() as usize;
    let scene: Handle<WorldAsset> = server.load(GltfAssetLabel::Scene(0).from_asset("kenney/train-kit/railroad-straight.glb"));
    for i in 0..n {
        let x = -reach + MODULE * (i as f32 + 0.5);
        let porteur = c
            .spawn((
                Transform::from_xyz(x, ECHELLE * 1.0, 0.0).with_rotation(Quat::from_rotation_y(std::f32::consts::FRAC_PI_2)).with_scale(Vec3::splat(ECHELLE)),
                Visibility::default(),
                ChildOf(parent),
            ))
            .id();
        // Le modèle va de z = 0 à z = 4 : on le centre.
        c.spawn((WorldAssetRoot(scene.clone()), Transform::from_xyz(0.0, 0.0, -2.0), ChildOf(porteur)));
    }
}

// ---- Prairie (Nature Kit) ----

/// Un modèle du Nature Kit, posé en `at` (ses modèles sont décalés de -0,05 en hauteur dans leur fichier).
fn nature(c: &mut Commands, server: &AssetServer, parent: Entity, nom: &str, at: Vec3, tour: f32, echelle: f32) {
    let scene: Handle<WorldAsset> = server.load(GltfAssetLabel::Scene(0).from_asset(format!("kenney/nature-kit/{nom}.glb")));
    c.spawn((WorldAssetRoot(scene), Transform::from_translation(at + Vec3::Y * 0.05 * echelle).with_rotation(Quat::from_rotation_y(tour)).with_scale(Vec3::splat(echelle)), ChildOf(parent)));
}

const FLEURS: [&str; 9] = ["flower_redA", "flower_yellowA", "flower_purpleA", "flower_redB", "flower_yellowB", "flower_purpleB", "flower_redC", "flower_yellowC", "flower_purpleC"];

/// Fleurs, champignons, touffes d'herbe et souches de la prairie.
pub fn prairie_deco(c: &mut Commands, server: &AssetServer, parent: Entity, d: crate::extra::Deco, at: Vec3, variant: usize) -> bool {
    use crate::extra::Deco;
    let tour = variant as f32 * 0.9;
    match d {
        Deco::Flower => nature(c, server, parent, FLEURS[variant % 9], at, tour, 2.2),
        Deco::Flowers => {
            for (i, (dx, dz)) in [(-0.14, -0.08), (0.12, -0.1), (0.0, 0.14)].into_iter().enumerate() {
                nature(c, server, parent, FLEURS[(variant + i * 4) % 9], at + Vec3::new(dx, 0.0, dz), tour + i as f32, 1.8);
            }
        }
        Deco::Mushroom => nature(c, server, parent, ["mushroom_red", "mushroom_redGroup", "mushroom_tan", "mushroom_tanGroup", "mushroom_redTall", "mushroom_tanTall"][variant % 6], at, tour, 2.2),
        // Les touffes d'herbe du Nature Kit sortent turquoise : on garde celles du jeu.
        Deco::Tuft => return false,
        Deco::Stump => nature(c, server, parent, ["stump_round", "stump_old", "stump_square", "stump_roundDetailed", "stump_oldTall", "stump_squareDetailed"][variant % 6], at, tour, 2.0),
    }
    true
}

/// Nénuphar bien rond sur lequel on saute (dessus à ≈ 0,16), parfois avec sa petite fleur de Kenney.
pub fn prairie_pad(c: &mut Commands, server: &AssetServer, x: &crate::extra::Extra, parent: Entity, fleur: bool) {
    // Feuille : un disque vert (0,78 de large) avec un disque plus clair dessus.
    c.spawn((Mesh3d(x.rond.clone()), MeshMaterial3d(x.pad_vert.clone()), Transform::from_xyz(0.0, 0.1, 0.0).with_scale(Vec3::new(0.78, 0.12, 0.78)), ChildOf(parent)));
    c.spawn((Mesh3d(x.rond.clone()), MeshMaterial3d(x.pad_vert_clair.clone()), Transform::from_xyz(0.0, 0.165, 0.0).with_scale(Vec3::new(0.62, 0.02, 0.62)), ChildOf(parent)));
    if fleur {
        nature(c, server, parent, "flower_redA", Vec3::new(0.05, 0.17, 0.0), 0.5, 1.4);
    }
}

/// Rondin de `len` cases : des `log_large` (1 de long le long de X) mis bout à bout.
pub fn prairie_log(c: &mut Commands, server: &AssetServer, parent: Entity, len: f32) {
    const ECHELLE: f32 = 1.3;
    let n = (len / ECHELLE).round().max(1.0) as usize;
    let pas = len / n as f32;
    let scene: Handle<WorldAsset> = server.load(GltfAssetLabel::Scene(0).from_asset("kenney/nature-kit/log_large.glb"));
    for i in 0..n {
        let x = -len / 2.0 + pas * (i as f32 + 0.5);
        c.spawn((
            WorldAssetRoot(scene.clone()),
            Transform::from_xyz(x, 0.06 - 0.42 * ECHELLE + 0.05, 0.0).with_scale(Vec3::new(pas, ECHELLE, ECHELLE)),
            ChildOf(parent),
        ));
    }
}

// ---- La nuit : lampe du héros et phares ----

/// Les faisceaux de lumière (quadrilatère + dégradé) pour les phares des voitures et la lampe du héros.
#[derive(Resource)]
pub struct Eclairage {
    quad: Handle<Mesh>,
    jaune: Handle<StandardMaterial>,
    rouge: Handle<StandardMaterial>,
    lampe: Handle<StandardMaterial>,
}

/// Un faisceau : n'est visible que le soir et la nuit.
#[derive(Component)]
pub struct Faisceau;

/// La lampe que porte le héros, qui éclaire devant lui la nuit.
#[derive(Component)]
pub struct LampeHeros;

/// Dégradé d'un faisceau : opaque près de la source (côté x = 0), transparent au bout et sur les bords.
fn degrade(r: u8, g: u8, b: u8, force: f32) -> Image {
    let (w, h) = (64usize, 32usize);
    let mut data = Vec::with_capacity(w * h * 4);
    for j in 0..h {
        let v = (j as f32 + 0.5) / h as f32 * 2.0 - 1.0; // -1..1 en travers
        for i in 0..w {
            let u = (i as f32 + 0.5) / w as f32; // 0..1 le long
            let long = (1.0 - u).powf(1.6);
            // En cône : étroit près de la source, large au bout.
            let demi = 0.3 + 0.7 * u;
            let bord = (1.0 - (v / demi) * (v / demi)).clamp(0.0, 1.0).powf(0.7);
            data.extend_from_slice(&[r, g, b, (255.0 * force * long * bord) as u8]);
        }
    }
    Image::new(Extent3d { width: w as u32, height: h as u32, depth_or_array_layers: 1 }, TextureDimension::D2, data, TextureFormat::Rgba8UnormSrgb, RenderAssetUsages::default())
}

impl FromWorld for Eclairage {
    fn from_world(world: &mut World) -> Self {
        let quad = world.resource_mut::<Assets<Mesh>>().add(Plane3d::default().mesh().size(1.0, 1.0));
        let mut images = world.resource_mut::<Assets<Image>>();
        let (img_j, img_r, img_l) = (images.add(degrade(255, 232, 110, 0.55)), images.add(degrade(255, 40, 30, 0.28)), images.add(degrade(255, 244, 205, 0.4)));
        let mut mats = world.resource_mut::<Assets<StandardMaterial>>();
        let mut faisceau = |img: Handle<Image>| mats.add(StandardMaterial { base_color_texture: Some(img), unlit: true, alpha_mode: AlphaMode::Blend, cull_mode: None, ..default() });
        Self { quad, jaune: faisceau(img_j), rouge: faisceau(img_r), lampe: faisceau(img_l) }
    }
}

/// Les phares d'une voiture de longueur `len` (tournée vers +X) : faisceau jaune devant, léger feu rouge derrière.
pub fn phares(c: &mut Commands, e: &Eclairage, parent: Entity, len: f32) {
    let devant = len / 2.0;
    // Devant : 2,6 de long, 1,5 de large, posé juste au-dessus de la route.
    c.spawn((Mesh3d(e.quad.clone()), MeshMaterial3d(e.jaune.clone()), Transform::from_xyz(devant + 1.3, 0.045, 0.0).with_scale(Vec3::new(2.6, 1.0, 1.5)), Visibility::Hidden, Faisceau, ChildOf(parent)));
    // Derrière : petit et léger (le dégradé s'éloigne de la voiture : on le retourne).
    c.spawn((Mesh3d(e.quad.clone()), MeshMaterial3d(e.rouge.clone()), Transform::from_xyz(-devant - 0.4, 0.045, 0.0).with_scale(Vec3::new(-0.8, 1.0, 1.0)), Visibility::Hidden, Faisceau, ChildOf(parent)));
}

/// Les faisceaux apparaissent le soir et la nuit.
pub fn eclairage_nuit(amb: Res<game_common::ambiance::Ambiance>, mut q: Query<&mut Visibility, With<Faisceau>>) {
    use game_common::ambiance::Moment;
    let sombre = matches!(amb.moment, Moment::Soir | Moment::Nuit);
    let voulu = if sombre { Visibility::Inherited } else { Visibility::Hidden };
    for mut v in &mut q {
        if *v != voulu {
            *v = voulu;
        }
    }
}

/// La lampe du héros : un projecteur chaud qui suit le personnage et regarde devant lui (vers -Z), plus son faisceau visible au sol.
pub fn lampe_heros(
    mut commands: Commands,
    amb: Res<game_common::ambiance::Ambiance>,
    frog: Res<crate::frog::Frog>,
    e: Res<Eclairage>,
    time: Res<Time>,
    mut lampes: Query<(&mut Transform, &mut SpotLight), With<LampeHeros>>,
    mut porteurs: Query<&mut Transform, (With<PorteurLampe>, Without<LampeHeros>)>,
    mut faisceaux: Query<&mut Visibility, (With<LampeFaisceau>, Without<Faisceau>)>,
    mut cree: Local<bool>,
    mut cap: Local<f32>,
) {
    use game_common::ambiance::Moment;
    if !*cree {
        *cree = true;
        *cap = std::f32::consts::FRAC_PI_2; // au départ il regarde vers le haut de l'écran
        // Le projecteur lui-même.
        commands.spawn((
            SpotLight { color: Color::srgb(1.0, 0.92, 0.72), intensity: 900_000.0, range: 10.0, outer_angle: 0.55, inner_angle: 0.2, shadow_maps_enabled: false, ..default() },
            Transform::default(),
            LampeHeros,
        ));
        // Son faisceau visible : un porteur qui suit le héros, tourné pour que +X de ses enfants aille vers -Z.
        let porteur = commands.spawn((Transform::from_rotation(Quat::from_rotation_y(std::f32::consts::FRAC_PI_2)), Visibility::default(), PorteurLampe)).id();
        commands.spawn((Mesh3d(e.quad.clone()), MeshMaterial3d(e.lampe.clone()), Transform::from_xyz(1.9, 0.05, 0.0).with_scale(Vec3::new(3.8, 1.0, 2.4)), Visibility::Hidden, LampeFaisceau, ChildOf(porteur)));
        return;
    }
    let sombre = matches!(amb.moment, Moment::Soir | Moment::Nuit);
    let force = match amb.moment {
        Moment::Nuit => 1.0,
        Moment::Soir => 0.5,
        _ => 0.0,
    };
    // La lumière suit le regard du personnage (haut, bas, gauche, droite) : le cap tourne vite mais sans à-coup, par le chemin le plus court.
    let devant = Quat::from_rotation_y(frog.yaw) * Vec3::NEG_Z;
    let voulu = (-devant.z).atan2(devant.x);
    let mut ecart = (voulu - *cap + std::f32::consts::PI).rem_euclid(std::f32::consts::TAU) - std::f32::consts::PI;
    ecart *= (time.delta_secs() * 14.0).min(1.0);
    *cap += ecart;
    let dir = Vec3::new(cap.cos(), 0.0, -cap.sin());
    for (mut tf, mut light) in &mut lampes {
        let p = frog.pos;
        *tf = Transform::from_translation(p + Vec3::new(0.0, 1.2, 0.0) - dir * 0.1).looking_at(p + dir * 3.5, Vec3::Y);
        light.intensity = 900_000.0 * force;
    }
    for mut tf in &mut porteurs {
        tf.translation = frog.pos + dir * 0.0;
        tf.rotation = Quat::from_rotation_y(*cap);
    }
    let voulu = if sombre { Visibility::Inherited } else { Visibility::Hidden };
    for mut v in &mut faisceaux {
        if *v != voulu {
            *v = voulu;
        }
    }
}

/// Le faisceau visible de la lampe du héros, et le porteur qui le suit.
#[derive(Component)]
pub struct LampeFaisceau;

#[derive(Component)]
pub struct PorteurLampe;

// ---- Far West (Prototype Kit, Survival Kit, Fantasy Town Kit, Nature Kit) ----

/// Les animaux qui traversent la piste : (modèle du Prototype Kit, échelle, longueur).
pub const FARWEST_ANIMAUX: [(&str, f32, f32); 3] = [("animal-horse", 1.5, 1.05), ("animal-bison", 1.6, 1.05), ("animal-dog", 1.6, 0.8)];

/// Un animal animé du Prototype Kit : le cheval et le chien galopent (animation « run »), le bison marche.
fn animal_ouest(c: &mut Commands, server: &AssetServer, graphs: &mut Assets<AnimationGraph>, parent: Entity, i: usize) -> bool {
    let (nom, echelle, _) = FARWEST_ANIMAUX[i];
    // (animation, rotation en Y pour qu'il regarde vers +X, hauteur du bas des pattes sous l'origine)
    let (anim, tour, pattes) = match nom {
        // Les animaux du Prototype Kit ont la tête vers -X : on les retourne tous.
        "animal-horse" => (1, std::f32::consts::PI, 0.31),
        "animal-bison" => (0, std::f32::consts::PI, 0.31),
        _ => (1, std::f32::consts::PI, 0.21),
    };
    // Vitesse à laquelle le pas de l'animation colle au sol : le cheval et le chien galopent vite, le bison marche.
    let v_ref = if anim == 1 { 4.5 } else { 2.0 };
    anime(c, server, graphs, parent, &format!("kenney/prototype-kit/{nom}.glb"), anim, Transform::from_xyz(0.0, pattes * echelle, 0.0).with_rotation(Quat::from_rotation_y(tour)).with_scale(Vec3::splat(echelle)), v_ref);
    true
}

fn farwest_scenery(c: &mut Commands, server: &AssetServer, parent: Entity, kind: crate::extra::Scenery, at: Vec3, roll: u32, variant: usize) -> bool {
    use crate::extra::Scenery;
    let tour = variant as f32 * 1.3;
    let poser = |c: &mut Commands, kit: &str, nom: &str, echelle: f32| {
        let scene: Handle<WorldAsset> = server.load(GltfAssetLabel::Scene(0).from_asset(format!("kenney/{kit}/{nom}.glb")));
        c.spawn((WorldAssetRoot(scene), Transform::from_translation(at).with_rotation(Quat::from_rotation_y(tour)).with_scale(Vec3::splat(echelle)), ChildOf(parent)));
    };
    match kind {
        Scenery::Block | Scenery::Side => match roll {
            // Les cactus du Nature Kit sont décalés de -0,05 dans leur fichier.
            0..=3 => nature(c, server, parent, ["cactus_tall", "cactus_short"][variant % 2], at, tour, 1.7),
            4..=5 => poser(c, "survival-kit", ["rock-sand-a", "rock-sand-b", "rock-sand-c"][variant % 3], 1.6),
            6 => poser(c, "survival-kit", "tent-canvas", 1.6),
            7 => poser(c, "fantasy-town-kit", ["stall", "cart"][variant % 2], 1.3),
            8 => poser(c, "survival-kit", ["barrel", "chest"][variant % 2], 1.7),
            _ => poser(c, "survival-kit", "signpost", 1.8),
        },
        // Peu de détails : des petits cailloux, un feu de camp éteint de temps en temps.
        Scenery::Deco => match variant % 6 {
            0 => poser(c, "survival-kit", "campfire-pit", 1.5),
            1 | 2 | 3 => poser(c, "survival-kit", ["rock-sand-a", "rock-sand-b", "rock-sand-c"][variant % 3], 0.5),
            _ => nature(c, server, parent, "cactus_short", at, tour, 0.9),
        },
    }
    true
}

/// Tonneau flottant sur lequel on saute (dessus à ≈ 0,16).
pub fn farwest_pad(c: &mut Commands, server: &AssetServer, parent: Entity) {
    let scene = server.load(GltfAssetLabel::Scene(0).from_asset("kenney/survival-kit/barrel.glb"));
    let echelle = 1.7;
    c.spawn((WorldAssetRoot(scene), Transform::from_xyz(0.0, 0.16 - 0.34 * echelle, 0.0).with_scale(Vec3::splat(echelle)), ChildOf(parent)));
}

// ---- Ville futuriste (City Kit, Space Kit, Car Kit) ----

/// Ce qui roule ou plane sur l'autoroute : (modèle « kit/nom », échelle, longueur).
pub const CYBER_TRAFIC: [(&str, f32, f32); 4] = [("space-kit/craft_speederA", 0.5, 1.2), ("space-kit/craft_speederB", 0.5, 1.2), ("space-kit/craft_racer", 0.5, 1.1), ("car-kit/race-future", 0.8, 1.6)];

/// Une voiture du futur : speeder volant du Space Kit (légèrement au-dessus de la route) ou bolide du Car Kit, avec phares la nuit.
fn voiture_neon(c: &mut Commands, server: &AssetServer, e: Option<&Eclairage>, parent: Entity, i: usize) -> bool {
    let (nom, echelle, len) = CYBER_TRAFIC[i];
    let scene = server.load(GltfAssetLabel::Scene(0).from_asset(format!("kenney/{nom}.glb")));
    if nom.starts_with("space-kit/") {
        space_modele(c, scene, parent, Transform::from_xyz(0.0, 0.12, 0.0).with_rotation(Quat::from_rotation_y(-std::f32::consts::FRAC_PI_2)).with_scale(Vec3::splat(echelle)));
    } else {
        c.spawn((WorldAssetRoot(scene), Transform::from_rotation(Quat::from_rotation_y(std::f32::consts::FRAC_PI_2)).with_scale(Vec3::splat(echelle)), ChildOf(parent)));
    }
    if let Some(e) = e {
        phares(c, e, parent, len);
    }
    true
}

fn cyber_scenery(c: &mut Commands, server: &AssetServer, parent: Entity, kind: crate::extra::Scenery, at: Vec3, roll: u32, variant: usize) -> bool {
    use crate::extra::Scenery;
    let tour = (variant % 4) as f32 * std::f32::consts::FRAC_PI_2;
    let poser = |c: &mut Commands, kit: &str, nom: &str, echelle: f32| {
        let scene: Handle<WorldAsset> = server.load(GltfAssetLabel::Scene(0).from_asset(format!("kenney/{kit}/{nom}.glb")));
        // La ville est éclairée : les modèles brillent doucement (leurs couleurs servent aussi d'émission).
        c.spawn((WorldAssetRoot(scene), Transform::from_translation(at).with_rotation(Quat::from_rotation_y(tour)).with_scale(Vec3::splat(echelle)), Brille(0.45), ChildOf(parent)));
    };
    let immeubles = ["building-a", "building-b", "building-c", "building-d", "building-e", "building-f", "building-g", "building-h", "building-i", "building-j", "building-k", "building-l", "building-m", "building-n"];
    let gratte_ciel = ["building-skyscraper-a", "building-skyscraper-b", "building-skyscraper-c", "building-skyscraper-d", "building-skyscraper-e"];
    match kind {
        Scenery::Block | Scenery::Side => match roll {
            0..=3 => poser(&mut *c, "city-kit-commercial", immeubles[variant % immeubles.len()], 1.0),
            4..=5 => poser(&mut *c, "city-kit-commercial", gratte_ciel[variant % gratte_ciel.len()], 0.75),
            6 => poser(&mut *c, "city-kit-industrial", ["shipping-container-a", "shipping-container-b", "shipping-container-c"][variant % 3], 0.8),
            7 => poser(&mut *c, "city-kit-industrial", ["detail-tank", "chimney-medium", "water-tower"][variant % 3], 0.8),
            8 => poser(&mut *c, "city-kit-industrial", "solar-panel-landscape-group", 1.0),
            _ => poser(&mut *c, "city-kit-roads", ["light-square-double", "sign-highway"][variant % 2], 1.6),
        },
        Scenery::Deco => poser(&mut *c, "city-kit-roads", ["construction-cone", "construction-barrier", "light-square"][variant % 3], if variant % 3 == 2 { 1.6 } else { 1.4 }),
    }
    true
}
