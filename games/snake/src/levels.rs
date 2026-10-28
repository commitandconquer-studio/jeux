//! Les niveaux du mode Voyage, lus dans `games/snake/levels/*.txt` (voir l'en-tête de chaque fichier).
//! Un niveau = un lieu, une forme de plateau, un objectif et un rythme.
use std::sync::LazyLock;

use bevy::prelude::IVec2;

use crate::board::Feature;
use crate::layout::{Layout, Shape};
use crate::world::Universe;

#[derive(Clone, Debug)]
pub struct Level {
    pub name: String,
    pub place: Place,
    pub shape: Form,
    /// Touristes à recruter.
    pub tourists: u32,
    /// Une sortie s'ouvre quand l'objectif est atteint : il faut que toute la file y passe.
    pub exit: bool,
    /// Durée d'une case, en secondes.
    pub pace: f32,
    /// Temps de la 2e étoile (la 3e demande 25 % de moins).
    pub par: f32,
    /// Cases spéciales posées au départ : (genre, nombre). Leur place est tirée au sort mais toujours la même pour un niveau.
    pub cases: Vec<(Feature, u32)>,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Form {
    Square(i32),
    Rect(i32, i32),
    Shape(Shape),
}

/// Les lieux du Voyage. Ceux qui n'ont pas encore leur décor empruntent celui d'un autre paysage.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Place {
    Garden,
    Desert,
    Ruins,
    Graveyard,
    Pirates,
    Park,
    City,
    Station,
    Harbour,
    Medieval,
}

impl Place {
    pub const ALL: [Place; 10] = [Place::Garden, Place::Desert, Place::Ruins, Place::Graveyard, Place::Pirates, Place::Park, Place::City, Place::Station, Place::Harbour, Place::Medieval];

    pub fn scenery(self) -> Universe {
        match self {
            Place::Garden => Universe::Prairie,
            Place::Desert => Universe::Desert,
            Place::Ruins => Universe::Ruines,
            Place::Graveyard => Universe::Cimetiere,
            Place::Pirates => Universe::Ocean,
            Place::Park => Universe::Parc,
            Place::City => Universe::Ville,
            Place::Station => Universe::Gare,
            Place::Harbour => Universe::Port,
            Place::Medieval => Universe::Medieval,
        }
    }

    pub fn label(self) -> &'static str {
        use game_common::tx;
        match self {
            Place::Garden => tx!("Jardin public", "Public garden", "Stadtpark", "Giardino pubblico", "Jardín público"),
            Place::Desert => tx!("Désert", "Desert", "Wüste", "Deserto", "Desierto"),
            Place::Ruins => tx!("Ruines antiques", "Ancient ruins", "Antike Ruinen", "Rovine antiche", "Ruinas antiguas"),
            Place::Graveyard => tx!("Cimetière", "Graveyard", "Friedhof", "Cimitero", "Cementerio"),
            Place::Pirates => tx!("Île des pirates", "Pirate island", "Pirateninsel", "Isola dei pirati", "Isla pirata"),
            Place::Park => tx!("Parc de loisirs", "Theme park", "Freizeitpark", "Parco divertimenti", "Parque de atracciones"),
            Place::City => tx!("Centre-ville", "Downtown", "Innenstadt", "Centro città", "Centro urbano"),
            Place::Station => tx!("Gare", "Train station", "Bahnhof", "Stazione", "Estación"),
            Place::Harbour => tx!("Port", "Harbour", "Hafen", "Porto", "Puerto"),
            Place::Medieval => tx!("Village médiéval", "Medieval village", "Mittelalterdorf", "Villaggio medievale", "Aldea medieval"),
        }
        .t()
    }
}

impl Level {
    pub fn layout(&self, id: u32) -> Layout {
        let mut l = match self.shape {
            Form::Square(n) => Layout::rect(n, n),
            Form::Rect(w, h) => Layout::rect(w, h),
            Form::Shape(s) => s.layout(),
        };
        l.id = 1000 + id;
        l
    }
}

fn parse_shape(v: &str) -> Option<Form> {
    let mut it = v.split_whitespace();
    let kind = it.next()?.to_lowercase();
    let arg = it.next();
    Some(match kind.as_str() {
        "carre" => Form::Square(arg?.parse().ok()?),
        "rectangle" => {
            let (w, h) = arg?.split_once('x')?;
            Form::Rect(w.parse().ok()?, h.parse().ok()?)
        }
        "l" => Form::Shape(Shape::L),
        "u" => Form::Shape(Shape::U),
        "route" => Form::Shape(Shape::Route),
        "allees" => Form::Shape(Shape::Alleys),
        _ => return None,
    })
}

fn parse_place(v: &str) -> Option<Place> {
    Some(match v {
        "jardin" => Place::Garden,
        "desert" => Place::Desert,
        "ruines" => Place::Ruins,
        "cimetiere" => Place::Graveyard,
        "pirates" => Place::Pirates,
        "parc" => Place::Park,
        "ville" => Place::City,
        "gare" => Place::Station,
        "port" => Place::Harbour,
        "medieval" => Place::Medieval,
        _ => return None,
    })
}

/// Lit un fichier de niveaux ; renvoie la liste, ou le message de la première erreur.
pub fn parse(text: &str) -> Result<Vec<Level>, String> {
    let mut levels: Vec<Level> = Vec::new();
    for (n, raw) in text.lines().enumerate() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let err = |m: &str| format!("ligne {} : {m}", n + 1);
        if let Some(name) = line.strip_prefix('[').and_then(|l| l.strip_suffix(']')) {
            levels.push(Level { name: name.to_owned(), place: Place::Garden, shape: Form::Square(14), tourists: 6, exit: false, pace: 0.3, par: 0.0, cases: Vec::new() });
            continue;
        }
        let Some((key, value)) = line.split_once('=') else { return Err(err("« clé = valeur » attendu")) };
        let (key, value) = (key.trim(), value.trim());
        let Some(level) = levels.last_mut() else { return Err(err("clé avant le premier [niveau]")) };
        match key {
            "lieu" => level.place = parse_place(value).ok_or_else(|| err("lieu inconnu"))?,
            "forme" => level.shape = parse_shape(value).ok_or_else(|| err("forme inconnue"))?,
            "touristes" => level.tourists = value.parse().map_err(|_| err("nombre attendu"))?,
            "sortie" => level.exit = value == "oui",
            "pas" => level.pace = value.parse().map_err(|_| err("nombre attendu"))?,
            "temps" => level.par = value.parse().map_err(|_| err("nombre attendu"))?,
            "blocs" | "trous" | "teleporteurs" | "accelerateurs" | "tapis" | "barrieres" | "rivaux" | "trains" | "pontons" | "herses" => {
                let n = value.parse().map_err(|_| err("nombre attendu"))?;
                let f = match key {
                    "blocs" => Feature::Obstacle,
                    "trous" => Feature::Hole,
                    "teleporteurs" => Feature::Teleport,
                    "accelerateurs" => Feature::Speeder,
                    "barrieres" | "herses" => Feature::Gate,
                    "rivaux" => Feature::Rival,
                    "trains" => Feature::Train,
                    "pontons" => Feature::Plank,
                    _ => Feature::Belt,
                };
                level.cases.push((f, n));
            }
            _ => return Err(err("clé inconnue")),
        }
    }
    for l in &mut levels {
        // Par défaut : une quinzaine de cases par touriste, plus la sortie.
        if l.par == 0.0 {
            l.par = l.tourists as f32 * l.pace * 14.0 + if l.exit { 15.0 } else { 0.0 };
        }
    }
    Ok(levels)
}

/// Tous les niveaux, dans l'ordre du Voyage.
pub static LEVELS: LazyLock<Vec<Level>> = LazyLock::new(|| {
    let mut all = Vec::new();
    for text in [include_str!("../levels/jardin.txt"), include_str!("../levels/desert.txt"), include_str!("../levels/ruines.txt"), include_str!("../levels/cimetiere.txt"), include_str!("../levels/pirates.txt"), include_str!("../levels/parc.txt"), include_str!("../levels/ville.txt"), include_str!("../levels/gare.txt"), include_str!("../levels/port.txt"), include_str!("../levels/medieval.txt")] {
        all.extend(parse(text).unwrap_or_else(|e| panic!("niveaux illisibles : {e}")));
    }
    all
});

/// Cases où l'on peut mettre la sortie : praticables, loin de la tête.
pub fn exit_candidates(layout: &Layout, from: IVec2) -> Vec<IVec2> {
    let mut cells: Vec<IVec2> = layout.cells().filter(|c| (c.x - from.x).abs() + (c.y - from.y).abs() >= 6).collect();
    cells.sort_by_key(|c| -((c.x - from.x).abs() + (c.y - from.y).abs()));
    cells
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn built_in_levels_are_valid() {
        let all = &*LEVELS;
        assert!(all.len() >= 10);
        for (i, l) in all.iter().enumerate() {
            let layout = l.layout(i as u32);
            assert!(layout.start_cells().iter().all(|c| layout.walkable(*c)), "{} : départ bloqué", l.name);
            // De la place pour recruter tout le monde avec la file de départ.
            assert!(layout.cells().count() as u32 > l.tourists + 3 + 10, "{} : trop étroit", l.name);
            if l.exit {
                assert!(!exit_candidates(&layout, layout.start).is_empty(), "{} : pas de place pour la sortie", l.name);
            }
        }
    }

    #[test]
    fn first_six_levels_get_harder_gradually() {
        // Le terrain ne grandit jamais et les touristes ne diminuent jamais.
        let all = &*LEVELS;
        for w in all[..6].windows(2) {
            let cells = |l: &Level| l.layout(0).cells().count();
            assert!(cells(&w[1]) <= cells(&w[0]));
            assert!(w[1].tourists >= w[0].tourists);
        }
    }

    #[test]
    fn parser_reports_errors() {
        assert!(parse("[A]\nforme = triangle").is_err());
        assert!(parse("lieu = jardin").is_err());
    }
}
