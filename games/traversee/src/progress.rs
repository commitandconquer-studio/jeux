//! Progression : au début, presque rien ; le total des points de toutes les
//! parties débloque peu à peu les mouches, les véhicules, les trains, les mondes,
//! les personnages et leurs accessoires. Sauvegardé entre deux parties.
use std::sync::LazyLock;

use bevy::prelude::*;
use game_common::{Score, tx};

use crate::GameState;
use crate::characters::{Character, Costume};
use crate::extra::{Biome, Vehicle};

const SAVE: &str = "traversee";

/// Ce qui se débloque.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Item {
    Vehicle(Vehicle),
    Trains,
    World(Biome),
    Character(Character),
    Costume(Costume),
    Flies,
    GoldFlies,
    Eagle,
}

/// Les seuils des tables sont des repères ; on les étire pour que les déblocages se méritent :
/// les premiers restent rapides, les derniers demandent beaucoup de parties.
pub fn rescale(t: u32) -> u32 {
    let t = t as f32;
    (t * 1.5 + t * t / 1500.0).round() as u32
}

/// Points cumulés nécessaires, dans l'ordre d'arrivée.
const BASE_UNLOCKS: &[(u32, Item)] = &[
    (15, Item::Flies),
    (25, Item::Vehicle(Vehicle::Truck)),
    (40, Item::Costume(Costume::Cap)),
    (60, Item::Vehicle(Vehicle::Taxi)),
    (80, Item::Character(Character::Rabbit)),
    (100, Item::Trains),
    (130, Item::Costume(Costume::Shades)),
    (160, Item::Vehicle(Vehicle::Moto)),
    (180, Item::Eagle),
    (200, Item::World(Biome::Desert)),
    (240, Item::Vehicle(Vehicle::Bus)),
    (280, Item::Character(Character::Badger)),
    (330, Item::Costume(Costume::TopHat)),
    (350, Item::GoldFlies),
    (380, Item::Vehicle(Vehicle::Pickup)),
    (440, Item::World(Biome::Snow)),
    (500, Item::Character(Character::Raccoon)),
    (560, Item::Vehicle(Vehicle::Sport)),
    (640, Item::Costume(Costume::Pirate)),
    (720, Item::Vehicle(Vehicle::Police)),
    (800, Item::World(Biome::Autumn)),
    (900, Item::Character(Character::Hedgehog)),
    (1000, Item::Vehicle(Vehicle::Tractor)),
    (1100, Item::Costume(Costume::Crown)),
    (1250, Item::Character(Character::Fox)),
    (1600, Item::Costume(Costume::Party)),
    (1800, Item::Character(Character::Panda)),
    (2000, Item::Vehicle(Vehicle::Sheriff)),
    (2300, Item::Costume(Costume::Wizard)),
    (2600, Item::Character(Character::Duck)),
];

/// Tous les déblocages, par ordre de points : ceux ci-dessus et les véhicules décrits en pièces.
pub static UNLOCKS: LazyLock<Vec<(u32, Item)>> = LazyLock::new(|| {
    let mut all: Vec<(u32, Item)> = BASE_UNLOCKS.iter().map(|(t, i)| (rescale(*t), *i)).collect();
    for b in Biome::ALL {
        if let Some(t) = b.theme() {
            all.push((rescale(t.points), Item::World(b)));
        }
    }
    all.sort_by_key(|(t, _)| *t);
    all
});

/// Les véhicules, dans l'ordre d'affichage au garage.
pub static ALL: LazyLock<Vec<Item>> = LazyLock::new(|| {
    let mut all = vec![
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
        Item::Vehicle(Vehicle::Sheriff),
    ];
    all
});

impl Item {
    /// Le nom à afficher (petit nom des véhicules, nom des mondes, des personnages…).
    pub fn name(self) -> &'static str {
        match self {
            Item::Trains => tx!("Le Rapide Rouge", "The Red Rocket", "Der Rote Blitz", "Il Rapido Rosso", "El Rayo Rojo"),
            Item::World(b) => return b.name(),
            Item::Character(c) => return c.name(),
            Item::Costume(c) => return c.name(),
            Item::Flies => tx!("Les mouches", "The flies", "Die Fliegen", "Le mosche", "Las moscas"),
            Item::Eagle => tx!("L'aigle (attention !)", "The eagle (watch out!)", "Der Adler (Vorsicht!)", "L'aquila (attenzione!)", "El águila (¡cuidado!)"),
            Item::GoldFlies => tx!("Les mouches dorées", "The golden flies", "Die goldenen Fliegen", "Le mosche dorate", "Las moscas doradas"),
            Item::Vehicle(Vehicle::Gen(i)) => return crate::gen_vehicles::name(i),
            Item::Vehicle(v) => match v {
                Vehicle::Gen(_) => unreachable!(),
                Vehicle::Beast(t, i) => return (crate::gen_worlds::THEMES[t].animal_name)(i),
                Vehicle::Car => tx!("Pépette", "Penny", "Püppi", "Pepetta", "Pepita"),
                Vehicle::Truck => tx!("Gros Bidon", "Big Barrel", "Dicker Brummer", "Gran Botte", "Gran Bidón"),
                Vehicle::Taxi => tx!("Taxi Citron", "Lemon Cab", "Zitronen-Taxi", "Taxi Limone", "Taxi Limón"),
                Vehicle::Moto => tx!("Zim-Zim", "Zoomy", "Flitzer", "Sfreccia", "Zumbi"),
                Vehicle::Bus => tx!("Le Bus Rigolo", "The Jolly Bus", "Der Lustige Bus", "Il Bus Allegro", "El Bus Alegre"),
                Vehicle::Pickup => tx!("Rocky le Pick-up", "Rocky the Pickup", "Rocky der Pick-up", "Rocky il Pick-up", "Rocky la Camioneta"),
                Vehicle::Sport => tx!("Éclair Fou", "Mad Flash", "Irrer Blitz", "Lampo Matto", "Rayo Loco"),
                Vehicle::Police => tx!("Sirène Max", "Siren Max", "Sirenen-Max", "Sirena Max", "Sirena Max"),
                Vehicle::Tractor => tx!("Gustave le Tracteur", "Gus the Tractor", "Traktor-Gustav", "Gustavo il Trattore", "Gustavo el Tractor"),
                Vehicle::Sheriff => tx!("Shérif Casse-Cou", "Sheriff Daredevil", "Sheriff Draufgänger", "Sceriffo Spericolato", "Sheriff Temerario"),
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
    /// Meilleur score d'avant la dernière partie, et si elle l'a battu.
    pub best_before: u32,
    pub new_record: bool,
    /// Points atteints en cours de partie : ce qui vient d'être gagné est déjà utilisable.
    pub live: u32,
}

impl Default for Progress {
    fn default() -> Self {
        // TRAVERSEE_POINTS=9999 : essayer tout sans toucher à la sauvegarde.
        let forced = std::env::var("TRAVERSEE_POINTS").ok().and_then(|s| s.parse().ok());
        let total = forced.or_else(|| game_common::save::load(SAVE, "total").and_then(|s| s.parse().ok())).unwrap_or(0);
        let best_before = game_common::save::load(SAVE, "best").and_then(|s| s.parse().ok()).unwrap_or(0);
        Self { total, fresh: Vec::new(), best_before, new_record: false, live: 0 }
    }
}

impl Progress {
    /// Nombre d'objets gagnés jusqu'ici.
    pub fn count(&self) -> usize {
        UNLOCKS.iter().filter(|(t, _)| self.points() >= *t).count()
    }

    /// Prochain objet à débloquer et son seuil.
    pub fn next(&self) -> Option<(u32, Item)> {
        UNLOCKS.iter().filter(|(t, _)| self.points() < *t).min_by_key(|(t, _)| *t).copied()
    }

    /// Avancement vers le prochain objet (0 à 1), seuil précédent et suivant.
    pub fn next_fraction(&self) -> f32 {
        let prev = UNLOCKS.iter().filter(|(t, _)| self.total >= *t).map(|(t, _)| *t).max().unwrap_or(0);
        match self.next() {
            Some((t, _)) => (self.total - prev) as f32 / (t - prev).max(1) as f32,
            None => 1.0,
        }
    }

    /// Points pris en compte : le total, ou plus si la partie en cours est déjà plus loin.
    pub fn points(&self) -> u32 {
        self.total.max(self.live)
    }

    pub fn has(&self, item: Item) -> bool {
        unlocked(self.points(), item)
    }
}

pub struct ProgressPlugin;

impl Plugin for ProgressPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Progress>()
            .add_systems(Startup, load_best)
            .add_systems(Update, announce.run_if(in_state(GameState::Playing)))
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
    progress.new_record = score.current > progress.best_before && score.current > 0;
    progress.best_before = progress.best_before.max(score.current);
    progress.total += score.current;
    let after = progress.total;
    progress.fresh = UNLOCKS.iter().filter(|(t, _)| before < *t && *t <= after).map(|(_, i)| *i).collect();
    // Les modes d'essai (captures, mesures, points forcés) ne touchent pas la sauvegarde.
    if test_mode() {
        return;
    }
    game_common::save::store(SAVE, "total", &after.to_string());
    game_common::save::store(SAVE, "best", &score.best.to_string());
}

/// Pendant la partie : dès qu'un seuil est franchi, on l'annonce à l'écran.
fn announce(
    mut progress: ResMut<Progress>,
    score: Res<Score>,
    mut last: Local<Option<u32>>,
    mut fx: MessageWriter<crate::fx::Fx>,
) {
    let now = progress.total + score.current;
    let before = last.unwrap_or(now).min(now);
    if before < now {
        for (t, item) in UNLOCKS.iter() {
            if before < *t && *t <= now {
                fx.write(crate::fx::Fx::Unlock(*item));
            }
        }
        // On le garde tout de suite : même si on ferme le jeu avant la fin de la partie.
        progress.live = now;
        if !test_mode() {
            game_common::save::store(SAVE, "total", &now.to_string());
        }
    }
    *last = Some(now);
}

impl Item {
    /// Le genre de nouveauté, pour l'étiquette de l'annonce.
    pub fn kind(self) -> &'static str {
        match self {
            Item::Vehicle(_) | Item::Trains => tx!("VÉHICULE", "VEHICLE", "FAHRZEUG", "VEICOLO", "VEHÍCULO"),
            Item::World(_) => tx!("MONDE", "WORLD", "WELT", "MONDO", "MUNDO"),
            Item::Character(_) => tx!("PERSONNAGE", "CHARACTER", "FIGUR", "PERSONAGGIO", "PERSONAJE"),
            Item::Costume(_) => tx!("ACCESSOIRE", "ACCESSORY", "ZUBEHÖR", "ACCESSORIO", "ACCESORIO"),
            Item::Flies | Item::GoldFlies | Item::Eagle => tx!("NOUVEAUTÉ", "NEW", "NEU", "NOVITÀ", "NOVEDAD"),
        }
        .t()
    }

    pub fn color(self) -> Color {
        match self {
            Item::Vehicle(_) | Item::Trains => Color::srgb(0.25, 0.5, 0.9),
            Item::World(_) => Color::srgb(0.3, 0.7, 0.35),
            Item::Character(_) => Color::srgb(0.95, 0.55, 0.15),
            Item::Costume(_) => Color::srgb(0.6, 0.4, 0.85),
            Item::Flies | Item::GoldFlies | Item::Eagle => Color::srgb(0.2, 0.7, 0.7),
        }
    }
}

fn test_mode() -> bool {
    ["TRAVERSEE_CAPTURE", "TRAVERSEE_PERF", "TRAVERSEE_POINTS"].iter().any(|k| std::env::var(k).is_ok())
}
