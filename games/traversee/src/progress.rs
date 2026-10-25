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

/// Tous les objets, dans l'ordre d'affichage au garage.
pub const ALL: [Item; 13] = [
    Item::Vehicle(Vehicle::Car),
    Item::Vehicle(Vehicle::Truck),
    Item::Vehicle(Vehicle::Taxi),
    Item::Trains,
    Item::Vehicle(Vehicle::Moto),
    Item::Vehicle(Vehicle::Bus),
    Item::Vehicle(Vehicle::Pickup),
    Item::Vehicle(Vehicle::Sport),
    Item::Vehicle(Vehicle::Police),
    Item::Vehicle(Vehicle::Tractor),
    Item::Vehicle(Vehicle::Future),
    Item::Vehicle(Vehicle::Sheriff),
    Item::Vehicle(Vehicle::Fantastic),
];

impl Item {
    /// Le petit nom de chaque véhicule.
    pub fn name(self) -> &'static str {
        match self {
            Item::Trains => tx!("Le Rapide Rouge", "The Red Rocket", "Der Rote Blitz", "Il Rapido Rosso", "El Rayo Rojo"),
            Item::Vehicle(v) => match v {
                Vehicle::Car => tx!("Pépette", "Penny", "Püppi", "Pepetta", "Pepita"),
                Vehicle::Truck => tx!("Gros Bidon", "Big Barrel", "Dicker Brummer", "Gran Botte", "Gran Bidón"),
                Vehicle::Taxi => tx!("Taxi Citron", "Lemon Cab", "Zitronen-Taxi", "Taxi Limone", "Taxi Limón"),
                Vehicle::Moto => tx!("Zim-Zim", "Zoomy", "Flitzer", "Sfreccia", "Zumbi"),
                Vehicle::Bus => tx!("Le Bus Rigolo", "The Jolly Bus", "Der Lustige Bus", "Il Bus Allegro", "El Bus Alegre"),
                Vehicle::Pickup => tx!("Rocky le Pick-up", "Rocky the Pickup", "Rocky der Pick-up", "Rocky il Pick-up", "Rocky la Camioneta"),
                Vehicle::Sport => tx!("Éclair Fou", "Mad Flash", "Irrer Blitz", "Lampo Matto", "Rayo Loco"),
                Vehicle::Police => tx!("Sirène Max", "Siren Max", "Sirenen-Max", "Sirena Max", "Sirena Max"),
                Vehicle::Tractor => tx!("Gustave le Tracteur", "Gus the Tractor", "Traktor-Gustav", "Gustavo il Trattore", "Gustavo el Tractor"),
                Vehicle::Future => tx!("Le Voyageur du Temps", "The Time Traveler", "Der Zeitreisende", "Il Viaggiatore del Tempo", "El Viajero del Tiempo"),
                Vehicle::Sheriff => tx!("Shérif Casse-Cou", "Sheriff Daredevil", "Sheriff Draufgänger", "Sceriffo Spericolato", "Sheriff Temerario"),
                Vehicle::Fantastic => tx!("Scanner le Fantastique", "Scanner the Fantastic", "Scanner der Fantastische", "Scanner il Fantastico", "Scanner el Fantástico"),
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
    /// Mouches gobées au total (elles débloquent les costumes).
    pub flies: u32,
    /// Personnages et costumes débloqués par la dernière partie.
    pub fresh_characters: Vec<crate::characters::Character>,
    pub fresh_costumes: Vec<crate::characters::Costume>,
    /// Objets débloqués par la dernière partie.
    pub fresh: Vec<Item>,
}

impl Default for Progress {
    fn default() -> Self {
        // TRAVERSEE_POINTS=9999 : essayer tous les véhicules sans toucher à la sauvegarde.
        let forced = std::env::var("TRAVERSEE_POINTS").ok().and_then(|s| s.parse().ok());
        let total = forced.or_else(|| game_common::save::load(SAVE, "total").and_then(|s| s.parse().ok())).unwrap_or(0);
        let flies = forced.map(|_| 9999).or_else(|| game_common::save::load(SAVE, "mouches").and_then(|s| s.parse().ok())).unwrap_or(0);
        Self { total, flies, fresh_characters: Vec::new(), fresh_costumes: Vec::new(), fresh: Vec::new() }
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
fn commit(mut progress: ResMut<Progress>, score: Res<Score>, frog: Res<crate::frog::Frog>) {
    let before = progress.total;
    let flies_before = progress.flies;
    progress.flies += frog.eaten();
    progress.total += score.current;
    let after = progress.total;
    let flies_after = progress.flies;
    progress.fresh_characters = crate::characters::CHARACTERS.iter().filter(|(_, t)| before < *t && *t <= after).map(|(c, _)| *c).collect();
    progress.fresh_costumes = crate::characters::COSTUMES.iter().filter(|(_, t)| flies_before < *t && *t <= flies_after).map(|(c, _)| *c).collect();
    progress.fresh = UNLOCKS.iter().filter(|(t, _)| before < *t && *t <= after).map(|(_, i)| *i).collect();
    // Les modes d'essai (captures, mesures, points forcés) ne touchent pas la sauvegarde.
    if ["TRAVERSEE_CAPTURE", "TRAVERSEE_PERF", "TRAVERSEE_POINTS"].iter().any(|k| std::env::var(k).is_ok()) {
        return;
    }
    game_common::save::store(SAVE, "total", &after.to_string());
    game_common::save::store(SAVE, "mouches", &flies_after.to_string());
    game_common::save::store(SAVE, "best", &score.best.to_string());
}
