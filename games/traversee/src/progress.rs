//! Progression : le total des points de toutes les parties débloque peu à peu
//! de nouveaux véhicules et les trains. Sauvegardé entre deux parties.
use bevy::prelude::*;
use game_common::{Score, tx};

use crate::GameState;
use crate::extra::Vehicle;

const SAVE: &str = "traversee";

/// Ce qui se débloque.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Item {
    Vehicle(Vehicle),
    Trains,
}

/// Points cumulés nécessaires, dans l'ordre d'arrivée.
pub const UNLOCKS: [(u32, Item); 12] = [
    (25, Item::Vehicle(Vehicle::Truck)),
    (60, Item::Vehicle(Vehicle::Taxi)),
    (100, Item::Trains),
    (160, Item::Vehicle(Vehicle::Moto)),
    (240, Item::Vehicle(Vehicle::Bus)),
    (340, Item::Vehicle(Vehicle::Pickup)),
    (460, Item::Vehicle(Vehicle::Sport)),
    (600, Item::Vehicle(Vehicle::Police)),
    (760, Item::Vehicle(Vehicle::Tractor)),
    (950, Item::Vehicle(Vehicle::Future)),
    (1200, Item::Vehicle(Vehicle::Sheriff)),
    (1500, Item::Vehicle(Vehicle::Fantastic)),
];

impl Item {
    pub fn name(self) -> &'static str {
        match self {
            Item::Trains => tx!("Les trains", "Trains", "Züge", "I treni", "Los trenes"),
            Item::Vehicle(v) => match v {
                Vehicle::Truck => tx!("Camion", "Truck", "Lastwagen", "Camion", "Camión"),
                Vehicle::Taxi => tx!("Taxi", "Taxi", "Taxi", "Taxi", "Taxi"),
                Vehicle::Moto => tx!("Moto", "Motorbike", "Motorrad", "Moto", "Moto"),
                Vehicle::Bus => tx!("Bus", "Bus", "Bus", "Autobus", "Autobús"),
                Vehicle::Pickup => tx!("Pick-up", "Pickup", "Pick-up", "Pick-up", "Camioneta"),
                Vehicle::Sport => tx!("Voiture de sport", "Sports car", "Sportwagen", "Auto sportiva", "Coche deportivo"),
                Vehicle::Police => tx!("Voiture de police", "Police car", "Polizeiwagen", "Auto della polizia", "Coche de policía"),
                Vehicle::Tractor => tx!("Tracteur", "Tractor", "Traktor", "Trattore", "Tractor"),
                Vehicle::Future => tx!("Le bolide du futur", "The car from the future", "Das Auto aus der Zukunft", "L'auto dal futuro", "El coche del futuro"),
                Vehicle::Sheriff => tx!("Le bolide du shérif", "The sheriff's racer", "Der Sheriff-Flitzer", "Il bolide dello sceriffo", "El bólido del sheriff"),
                Vehicle::Fantastic => tx!("La voiture fantastique", "The talking car", "Das Wunderauto", "L'auto fantastica", "El coche fantástico"),
                Vehicle::Car => tx!("Voiture", "Car", "Auto", "Auto", "Coche"),
            },
        }
        .t()
    }
}

/// Seuil d'un objet, ou 0 s'il est disponible dès le départ.
pub fn threshold(item: Item) -> u32 {
    UNLOCKS.iter().find(|(_, i)| *i == item).map_or(0, |(t, _)| *t)
}

pub fn unlocked(points: u32, item: Item) -> bool {
    points >= threshold(item)
}

#[derive(Resource)]
pub struct Progress {
    /// Points de toutes les parties terminées.
    pub total: u32,
    /// Objets débloqués par la dernière partie.
    pub fresh: Vec<Item>,
}

impl Default for Progress {
    fn default() -> Self {
        // TRAVERSEE_POINTS=9999 : essayer tous les véhicules sans toucher à la sauvegarde.
        let forced = std::env::var("TRAVERSEE_POINTS").ok().and_then(|s| s.parse().ok());
        let total = forced.or_else(|| game_common::save::load(SAVE, "total").and_then(|s| s.parse().ok())).unwrap_or(0);
        Self { total, fresh: Vec::new() }
    }
}

impl Progress {
    pub fn count(&self) -> usize {
        1 + UNLOCKS.iter().filter(|(t, _)| self.total >= *t).count()
    }

    /// Prochain objet à débloquer et son seuil.
    pub fn next(&self) -> Option<(u32, Item)> {
        UNLOCKS.iter().find(|(t, _)| self.total < *t).copied()
    }
}

pub struct ProgressPlugin;

impl Plugin for ProgressPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Progress>()
            .add_systems(Startup, load_best)
            // Avant l'écran de fin, qui affiche ce qui vient d'être débloqué.
            .add_systems(OnEnter(GameState::GameOver), commit.before(crate::ui::spawn_game_over));
    }
}

fn load_best(mut score: ResMut<Score>) {
    if let Some(best) = game_common::save::load(SAVE, "best").and_then(|s| s.parse().ok()) {
        score.best = best;
    }
}

/// Fin de partie : les points s'ajoutent au total ; ce qui passe un seuil se débloque.
fn commit(mut progress: ResMut<Progress>, score: Res<Score>) {
    let before = progress.total;
    progress.total += score.current;
    let after = progress.total;
    progress.fresh = UNLOCKS.iter().filter(|(t, _)| before < *t && *t <= after).map(|(_, i)| *i).collect();
    // Les modes d'essai (captures, mesures, points forcés) ne touchent pas la sauvegarde.
    if ["TRAVERSEE_CAPTURE", "TRAVERSEE_PERF", "TRAVERSEE_POINTS"].iter().any(|k| std::env::var(k).is_ok()) {
        return;
    }
    game_common::save::store(SAVE, "total", &after.to_string());
    game_common::save::store(SAVE, "best", &score.best.to_string());
}
