//! Progression : au début, presque rien ; le total des points de toutes les
//! parties débloque peu à peu les mouches, les véhicules, les trains, les mondes,
//! les personnages et leurs accessoires. Sauvegardé entre deux parties.
use std::sync::LazyLock;

use bevy::prelude::*;
use game_common::{Score, tx};

use crate::GameState;
use crate::characters::Character;
use crate::extra::{Biome, Vehicle};

const SAVE: &str = "traversee";

/// Ce qui se débloque.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Item {
    Vehicle(Vehicle),
    World(Biome),
    Character(Character),
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

/// Points cumulés nécessaires, dans l'ordre d'arrivée : une nouveauté tous les 250 points, et les grosses
/// (premier monde, aigle) aux paliers de 1 000 à 2 000. Valeurs finales, sans étirement.
const BASE_UNLOCKS: &[(u32, Item)] = &[
    (20, Item::Flies),
    (250, Item::Vehicle(Vehicle::Truck)),
    (500, Item::Character(Character::Rabbit)),
    (750, Item::Vehicle(Vehicle::Taxi)),
    (1000, Item::Vehicle(Vehicle::Moto)),
    (1250, Item::World(Biome::Desert)),
    (1500, Item::Vehicle(Vehicle::Bus)),
    (1750, Item::Character(Character::Badger)),
    (2000, Item::Eagle),
    (2250, Item::GoldFlies),
    (2500, Item::Vehicle(Vehicle::Pickup)),
    (2750, Item::Character(Character::Raccoon)),
    (3000, Item::World(Biome::Snow)),
    (3250, Item::Character(Character::Noe)),
    (3500, Item::Vehicle(Vehicle::Sport)),
    (3750, Item::Character(Character::Ines)),
    (4000, Item::Vehicle(Vehicle::Police)),
    (4250, Item::Character(Character::Hedgehog)),
    (4500, Item::World(Biome::Autumn)),
    (4750, Item::Vehicle(Vehicle::Tractor)),
    (5000, Item::Character(Character::Sacha)),
    (5250, Item::Character(Character::Fox)),
    (5500, Item::Character(Character::Panda)),
    (5750, Item::Character(Character::Maya)),
    (6000, Item::Character(Character::Duck)),
    (6250, Item::Vehicle(Vehicle::Sheriff)),
];

/// Tous les déblocages, par ordre de points : ceux ci-dessus et les véhicules décrits en pièces.
pub static UNLOCKS: LazyLock<Vec<(u32, Item)>> = LazyLock::new(|| {
    let mut all: Vec<(u32, Item)> = BASE_UNLOCKS.to_vec();
    for b in Biome::ALL {
        if let Some(t) = b.theme() {
            all.push((rescale(t.points), Item::World(b)));
        }
    }
    all.sort_by_key(|(t, _)| *t);
    all
});


impl Item {
    /// Le nom à afficher (petit nom des véhicules, nom des mondes, des personnages…).
    pub fn name(self) -> &'static str {
        match self {
            Item::World(b) => return b.name(),
            Item::Character(c) => return c.name(),
            Item::Flies => tx!("La nourriture", "Food", "Das Essen", "Il cibo", "La comida"),
            Item::Eagle => tx!("L'aigle (attention !)", "The eagle (watch out!)", "Der Adler (Vorsicht!)", "L'aquila (attenzione!)", "El águila (¡cuidado!)"),
            Item::GoldFlies => tx!("La nourriture dorée", "Golden food", "Das goldene Essen", "Il cibo dorato", "La comida dorada"),
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
            Item::Vehicle(_) => tx!("VÉHICULE", "VEHICLE", "FAHRZEUG", "VEICOLO", "VEHÍCULO"),
            Item::World(_) => tx!("MONDE", "WORLD", "WELT", "MONDO", "MUNDO"),
            Item::Character(_) => tx!("PERSONNAGE", "CHARACTER", "FIGUR", "PERSONAGGIO", "PERSONAJE"),
            Item::Flies | Item::GoldFlies | Item::Eagle => tx!("NOUVEAUTÉ", "NEW", "NEU", "NOVITÀ", "NOVEDAD"),
        }
        .t()
    }

    /// La phrase du pop-up quand on vient de le gagner.
    pub fn gagne(self) -> &'static str {
        match self {
            Item::Vehicle(_) => tx!("Nouveau modèle gagné !", "New model unlocked!", "Neues Modell freigeschaltet!", "Nuovo modello sbloccato!", "¡Nuevo modelo desbloqueado!"),
            Item::World(_) => tx!("Nouveau monde gagné !", "New world unlocked!", "Neue Welt freigeschaltet!", "Nuovo mondo sbloccato!", "¡Nuevo mundo desbloqueado!"),
            Item::Character(_) => tx!("Nouveau personnage gagné !", "New character unlocked!", "Neue Figur freigeschaltet!", "Nuovo personaggio sbloccato!", "¡Nuevo personaje desbloqueado!"),
            Item::Flies | Item::GoldFlies | Item::Eagle => tx!("Nouveauté débloquée !", "New unlocked!", "Neuheit freigeschaltet!", "Novità sbloccata!", "¡Novedad desbloqueada!"),
        }
        .t()
    }

    pub fn color(self) -> Color {
        match self {
            Item::Vehicle(_) => Color::srgb(0.25, 0.5, 0.9),
            Item::World(_) => Color::srgb(0.3, 0.7, 0.35),
            Item::Character(_) => Color::srgb(0.95, 0.55, 0.15),
            Item::Flies | Item::GoldFlies | Item::Eagle => Color::srgb(0.2, 0.7, 0.7),
        }
    }
}

fn test_mode() -> bool {
    ["TRAVERSEE_CAPTURE", "TRAVERSEE_PERF", "TRAVERSEE_POINTS"].iter().any(|k| std::env::var(k).is_ok())
}
