//! Progression : le total des invités attrapés, toutes parties confondues, débloque peu à peu
//! les éléments du plateau, les paysages et les personnages. Sauvegardé entre deux parties.
use bevy::prelude::*;
use game_common::{Score, tx};

use crate::GameState;
use crate::board::Feature;
use crate::people;
use crate::world::Universe;

const SAVE: &str = "snake";

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Item {
    Feature(Feature),
    World(Universe),
    /// Un personnage (indice dans `people::KINDS`) : invité et chef de file possible.
    Person(usize),
}

/// Total d'invités attrapés nécessaire pour chaque déblocage, dans l'ordre.
pub const UNLOCKS: &[(u32, Item)] = &[
    (10, Item::Feature(Feature::Obstacle)),
    (20, Item::World(Universe::Desert)),
    (30, Item::Person(6)),
    (40, Item::Feature(Feature::Speeder)),
    (55, Item::Person(7)),
    (70, Item::Feature(Feature::Hole)),
    (90, Item::World(Universe::Cimetiere)),
    (110, Item::Person(8)),
    (130, Item::Feature(Feature::Teleport)),
    (155, Item::Person(9)),
    (180, Item::Feature(Feature::Belt)),
    (210, Item::World(Universe::Ocean)),
    (240, Item::Person(10)),
    (275, Item::Person(11)),
    (320, Item::Person(12)),
    (430, Item::Person(13)),
    (500, Item::Person(14)),
    (600, Item::Person(15)),
];

#[derive(Resource, Default)]
pub struct Progress {
    pub total: u32,
    /// Déblocages obtenus pendant la dernière partie.
    pub fresh: Vec<Item>,
    /// Paysages débloqués en finissant un lieu du Voyage.
    pub places: Vec<Universe>,
    /// Tout est ouvert (captures d'écran, essais).
    open: bool,
}

impl Progress {
    fn needed(item: Item) -> u32 {
        UNLOCKS.iter().find(|(_, i)| *i == item).map_or(0, |(n, _)| *n)
    }

    pub fn has(&self, item: Item) -> bool {
        if self.open {
            return true;
        }
        // Un paysage dont le lieu existe dans le Voyage s'y gagne ; les autres suivent le total.
        if let Item::World(u) = item
            && crate::levels::LEVELS.iter().any(|l| l.place.scenery() == u)
        {
            return u == Universe::Prairie || self.places.contains(&u);
        }
        self.total >= Self::needed(item)
    }

    pub fn allows(&self, f: Feature) -> bool {
        self.has(Item::Feature(f))
    }

    /// Les personnages disponibles.
    pub fn persons(&self) -> Vec<usize> {
        (0..people::KINDS.len()).filter(|k| self.has(Item::Person(*k))).collect()
    }

    /// Prochain déblocage : (seuil, élément).
    pub fn next(&self) -> Option<(u32, Item)> {
        if self.open {
            return None;
        }
        UNLOCKS.iter().find(|(n, _)| *n > self.total).copied()
    }

    /// Texte « Prochain : X (encore N) ».
    pub fn next_text(&self) -> Option<String> {
        let (n, item) = self.next()?;
        Some(tx!("Prochain déblocage : {} (encore {})", "Next unlock: {} ({} to go)", "Nächste Freischaltung: {} (noch {})", "Prossimo sblocco: {} (ancora {})", "Próximo desbloqueo: {} (faltan {})").avec(&[&label(item), &(n - self.total)]))
    }
}

pub fn label(item: Item) -> &'static str {
    match item {
        Item::Feature(Feature::Obstacle) => tx!("Obstacles", "Obstacles", "Hindernisse", "Ostacoli", "Obstáculos").t(),
        Item::Feature(Feature::Portal) => tx!("Portails", "Portals", "Portale", "Portali", "Portales").t(),
        Item::Feature(Feature::Hole) => tx!("Trous", "Holes", "Löcher", "Buche", "Hoyos").t(),
        Item::Feature(Feature::Teleport) => tx!("Téléporteurs", "Teleporters", "Teleporter", "Teletrasporti", "Teletransportadores").t(),
        Item::Feature(Feature::Speeder) => tx!("Accélérateurs", "Speed pads", "Turbo-Felder", "Acceleratori", "Aceleradores").t(),
        Item::Feature(Feature::Plank) => tx!("Pontons", "Docks", "Stege", "Moli", "Muelles").t(),
        Item::Feature(Feature::Train) => tx!("Trains", "Trains", "Züge", "Treni", "Trenes").t(),
        Item::Feature(Feature::Rival) => tx!("Guides rivaux", "Rival guides", "Konkurrenzführer", "Guide rivali", "Guías rivales").t(),
        Item::Feature(Feature::Gate) => tx!("Barrières", "Barriers", "Schranken", "Barriere", "Barreras").t(),
        Item::Feature(Feature::Belt) => tx!("Tapis roulants", "Conveyor belts", "Förderbänder", "Nastri trasportatori", "Cintas transportadoras").t(),
        Item::World(u) => u.label(),
        Item::Person(k) => people::name(k),
    }
}

pub struct ProgressPlugin;

impl Plugin for ProgressPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Progress>()
            .add_systems(Startup, load)
            .add_systems(OnEnter(GameState::Playing), |mut p: ResMut<Progress>| p.fresh.clear())
            .add_systems(OnExit(GameState::Playing), bank);
    }
}

fn load(mut p: ResMut<Progress>) {
    p.total = game_common::save::load(SAVE, "total").and_then(|v| v.parse().ok()).unwrap_or(0);
    p.open = std::env::var("SNAKE_CAPTURE").is_ok() || std::env::var("SNAKE_PRESSE").is_ok() || std::env::var("SNAKE_TOUT").is_ok();
}

/// Les invités de la partie s'ajoutent au total ; on note ce qui vient de se débloquer.
fn bank(mut p: ResMut<Progress>, score: Res<Score>) {
    let before = p.total;
    p.total += score.current;
    game_common::save::store(SAVE, "total", &p.total.to_string());
    p.fresh = UNLOCKS.iter().filter(|(n, _)| *n > before && *n <= p.total).map(|(_, i)| *i).collect();
}
