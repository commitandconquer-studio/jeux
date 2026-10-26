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
