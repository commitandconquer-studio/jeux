//! Les niveaux du mode Voyage, lus dans `games/snake/levels/*.txt` (voir l'en-tête de chaque fichier).
//! Un niveau = un lieu, une forme de plateau, un objectif et un rythme.
use std::sync::LazyLock;

use bevy::prelude::{IVec2, warn};

use crate::board::Feature;
use crate::grid::{self, Grid};
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
    /// Carte dessinée à la main (éditeur) : elle remplace la forme, et fixe l'emplacement des cases.
    pub grid: Option<Grid>,
    /// Direction du départ (utile pour les cartes dessinées).
    pub dir: IVec2,
    /// Paysage choisi (`paysage = …`) ; sinon celui du lieu.
    pub universe: Option<Universe>,
    /// Une scène (décor, personnages, dialogues) au lieu d'un niveau jouable : `type = scene`.
    pub scene: Option<Scene>,
}

/// Cadrage de la caméra d'une scène (perspective, pas isométrique).
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum Cam {
    /// Tout le décor.
    Large,
    #[default]
    Medium,
    /// Gros plan sur les personnages.
    Close,
    /// Vue de dessus, un peu en recul.
    High,
    /// Contre-plongée : on les voit d'en bas.
    Low,
}

impl Cam {
    pub fn parse(v: &str) -> Option<Cam> {
        Some(match v {
            "large" => Cam::Large,
            "moyen" => Cam::Medium,
            "gros" => Cam::Close,
            "plongee" => Cam::High,
            "bas" => Cam::Low,
            _ => return None,
        })
    }
}

/// Un personnage posé sur la scène.
#[derive(Clone, Debug)]
pub struct Actor {
    #[allow(dead_code)]
    pub name: String,
    /// Indice dans `people::KINDS`.
    pub kind: usize,
    /// Numéro d'apparence (couleurs de peau, cheveux, vêtements).
    pub look: u32,
    pub x: f32,
    pub z: f32,
    pub pose: crate::people::Pose,
    pub guide: bool,
}

/// Une réplique, affichée en sous-titre.
#[derive(Clone, Debug)]
pub struct Line {
    pub who: String,
    pub text: String,
    /// Nouveau cadrage à partir de cette réplique.
    pub camera: Option<Cam>,
}

#[derive(Clone, Debug, Default)]
pub struct Scene {
    pub decor: Option<Universe>,
    pub camera: Cam,
    pub actors: Vec<Actor>,
    pub lines: Vec<Line>,
}

impl Level {
    pub fn scenery(&self) -> Universe {
        self.universe.unwrap_or_else(|| self.place.scenery())
    }
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
    Cave,
    /// Niveaux spéciaux (éditeur) : tout est permis, le paysage se choisit (`paysage = …`). Absent des onglets du Voyage.
    Special,
}

impl Place {
    pub const ALL: [Place; 11] = [Place::Garden, Place::Desert, Place::Ruins, Place::Graveyard, Place::Pirates, Place::Park, Place::City, Place::Station, Place::Harbour, Place::Medieval, Place::Cave];

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
            Place::Cave => Universe::Cave,
            Place::Special => Universe::Prairie,
        }
    }

    /// Le nom du lieu dans les fichiers de niveaux.
    pub fn key(self) -> &'static str {
        match self {
            Place::Garden => "jardin",
            Place::Desert => "desert",
            Place::Ruins => "ruines",
            Place::Graveyard => "cimetiere",
            Place::Pirates => "pirates",
            Place::Park => "parc",
            Place::City => "ville",
            Place::Station => "gare",
            Place::Harbour => "port",
            Place::Medieval => "medieval",
            Place::Cave => "grottes",
            Place::Special => "special",
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
            Place::Cave => tx!("Grottes", "Caves", "Höhlen", "Grotte", "Cuevas"),
            Place::Special => tx!("Niveaux spéciaux", "Special levels", "Spezielle Level", "Livelli speciali", "Niveles especiales"),
        }
        .t()
    }
}

impl Level {
    pub fn layout(&self, id: u32) -> Layout {
        let mut l = match (&self.grid, self.shape) {
            (Some(g), _) => g.layout(),
            (None, shape) => match shape {
            Form::Square(n) => Layout::rect(n, n),
            Form::Rect(w, h) => Layout::rect(w, h),
            Form::Shape(s) => s.layout(),
            },
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
        "grottes" => Place::Cave,
        "special" => Place::Special,
        _ => return None,
    })
}

/// Lit un fichier de niveaux ; renvoie la liste, ou le message de la première erreur.
pub fn parse(text: &str) -> Result<Vec<Level>, String> {
    parse_file(text).map(|(_, levels)| levels)
}

/// Titre et description d'une aventure (lignes `titre = …` avant la première section).
#[derive(Clone, Debug, Default)]
pub struct Meta {
    pub title: String,
    pub description: String,
}

fn parse_position(v: &str) -> Option<(f32, f32)> {
    let (name, back) = v.strip_suffix("-fond").map_or((v, false), |n| (n, true));
    let x = match name {
        "gauche2" => -3.4,
        "gauche" => -1.7,
        "centre" => 0.0,
        "droite" => 1.7,
        "droite2" => 3.4,
        _ => {
            let (x, z) = v.split_once(',')?;
            return Some((x.trim().parse().ok()?, z.trim().parse().ok()?));
        }
    };
    Some((x, if back { 1.6 } else { 0.0 }))
}

/// Lit un fichier de niveaux et de scènes ; renvoie aussi son titre.
pub fn parse_file(text: &str) -> Result<(Meta, Vec<Level>), String> {
    let mut meta = Meta::default();
    let mut levels: Vec<Level> = Vec::new();
    // Lignes de la carte en cours de lecture (après « carte: »), par niveau.
    let mut maps: Vec<Vec<String>> = Vec::new();
    let mut in_map = false;
    for (n, raw) in text.lines().enumerate() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') {
            in_map = false;
            continue;
        }
        let err = |m: &str| format!("ligne {} : {m}", n + 1);
        if let Some(name) = line.strip_prefix('[').and_then(|l| l.strip_suffix(']')) {
            levels.push(Level { name: name.to_owned(), place: Place::Garden, shape: Form::Square(14), tourists: 6, exit: false, pace: 0.3, par: 0.0, cases: Vec::new(), grid: None, dir: IVec2::X, universe: None, scene: None });
            maps.push(Vec::new());
            in_map = false;
            continue;
        }
        if line == "carte:" {
            if levels.is_empty() {
                return Err(err("carte avant le premier [niveau]"));
            }
            in_map = true;
            continue;
        }
        if in_map {
            maps.last_mut().expect("un niveau est ouvert").push(line.to_owned());
            continue;
        }
        let Some((key, value)) = line.split_once('=') else { return Err(err("« clé = valeur » attendu")) };
        let (key, value) = (key.trim(), value.trim());
        let Some(level) = levels.last_mut() else {
            match key {
                "titre" => meta.title = value.to_owned(),
                "description" => meta.description = value.to_owned(),
                _ => return Err(err("clé avant le premier [niveau] (seuls « titre » et « description » sont permis)")),
            }
            continue;
        };
        match key {
            "type" if value == "scene" => level.scene = Some(Scene::default()),
            "type" if value == "niveau" => level.scene = None,
            "type" => return Err(err("type attendu : scene ou niveau")),
            "decor" | "camera" | "personnage" | "replique" => {
                let Some(scene) = level.scene.as_mut() else { return Err(err("cette clé demande « type = scene » plus haut dans la section")) };
                match key {
                    "decor" => scene.decor = Some(parse_place(value).ok_or_else(|| err("décor inconnu (jardin, desert, ruines…)"))?.scenery()),
                    "camera" => scene.camera = Cam::parse(value).ok_or_else(|| err("caméra attendue : large, moyen, gros, plongee ou bas"))?,
                    "personnage" => {
                        // Nom | modèle | apparence | position | animation | rôle
                        let f: Vec<&str> = value.split('|').map(str::trim).collect();
                        if f.len() < 4 {
                            return Err(err("personnage = Nom | modèle | apparence | position [| animation | guide]"));
                        }
                        let kind: usize = f[1].parse().map_err(|_| err("modèle : un numéro (0 à 18)"))?;
                        // Apparence : un numéro (0 à 95), ou des couleurs choisies : peau.cheveux.haut.bas.chaussures (indices).
                        let look: u32 = if f[2].contains('.') {
                            let c: Vec<usize> = f[2].split('.').filter_map(|v| v.parse().ok()).collect();
                            use game_common::look::{BOTTOMS, HAIRS, SHOES, SKINS, TOPS, parts_code};
                            if c.len() != 5 || c[0] >= SKINS.len() || c[1] >= HAIRS.len() || c[2] >= TOPS.len() || c[3] >= BOTTOMS.len() || c[4] >= SHOES.len() {
                                return Err(err("couleurs : peau.cheveux.haut.bas.chaussures (0-7, 0-11, 0-15, 0-7, 0-3)"));
                            }
                            parts_code(c[0], c[1], c[2], c[3], c[4]) as u32
                        } else {
                            f[2].parse().map_err(|_| err("apparence : un numéro (0 à 95) ou peau.cheveux.haut.bas.chaussures"))?
                        };
                        let (x, z) = parse_position(f[3]).ok_or_else(|| err("position : gauche2, gauche, centre, droite, droite2 (+ « -fond »), ou x,z"))?;
                        let pose = crate::people::Pose::from_name(match f.get(4).copied() {
                            Some("") | None => "repos",
                            Some(n) => n,
                        })
                        .ok_or_else(|| err("animation inconnue (repos, salut, marche, court, saute, mort… voir l'éditeur)"))?;
                        scene.actors.push(Actor { name: f[0].to_owned(), kind, look, x, z, pose, guide: f.get(5).copied() == Some("guide") });
                    }
                    _ => {
                        // Qui | texte [| caméra]
                        let mut it = value.splitn(3, '|').map(str::trim);
                        let (who, text) = (it.next().unwrap_or(""), it.next().ok_or_else(|| err("replique = Qui | texte"))?);
                        let camera = match it.next() {
                            Some(c) => Some(Cam::parse(c).ok_or_else(|| err("caméra attendue : large, moyen, gros, plongee ou bas"))?),
                            None => None,
                        };
                        scene.lines.push(Line { who: who.to_owned(), text: text.to_owned(), camera });
                    }
                }
            }
            "lieu" => level.place = parse_place(value).ok_or_else(|| err("lieu inconnu"))?,
            "forme" => level.shape = parse_shape(value).ok_or_else(|| err("forme inconnue"))?,
            "touristes" => level.tourists = value.parse().map_err(|_| err("nombre attendu"))?,
            "sortie" => level.exit = value == "oui",
            "paysage" => level.universe = Some(parse_place(value).ok_or_else(|| err("paysage inconnu (jardin, desert, ruines…)"))?.scenery()),
            "depart" => level.dir = grid::parse_dir(value).ok_or_else(|| err("direction attendue : est, ouest, nord ou sud"))?,
            "pas" => level.pace = value.parse().map_err(|_| err("nombre attendu"))?,
            "temps" => level.par = value.parse().map_err(|_| err("nombre attendu"))?,
            "blocs" | "trous" | "teleporteurs" | "accelerateurs" | "tapis" | "barrieres" | "rivaux" | "trains" | "pontons" | "herses" | "eboulements" => {
                let n = value.parse().map_err(|_| err("nombre attendu"))?;
                let f = match key {
                    "blocs" => Feature::Obstacle,
                    "trous" => Feature::Hole,
                    "teleporteurs" => Feature::Teleport,
                    "accelerateurs" => Feature::Speeder,
                    "barrieres" | "herses" => Feature::Gate,
                    "eboulements" => Feature::Cave,
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
    for (l, rows) in levels.iter_mut().zip(&maps) {
        if !rows.is_empty() {
            l.grid = Some(Grid::from_rows(rows, l.dir).map_err(|e| format!("niveau « {} » : {e}", l.name))?);
        }
    }
    for l in &mut levels {
        // Par défaut : une quinzaine de cases par touriste, plus la sortie.
        if l.par == 0.0 {
            l.par = l.tourists as f32 * l.pace * 14.0 + if l.exit { 15.0 } else { 0.0 };
        }
    }
    Ok((meta, levels))
}

/// Niveau demandé par l'éditeur en ligne : `?editeur=N` dans l'adresse de la page (ou `SNAKE_EDITEUR=N` en natif). 1 = premier niveau du fichier.
pub fn editor_request() -> Option<usize> {
    static R: LazyLock<Option<usize>> = LazyLock::new(|| {
        game_common::save::url_param("editeur").or_else(|| std::env::var("SNAKE_EDITEUR").ok()).and_then(|v| v.parse().ok()).map(|n: usize| n.max(1))
    });
    *R
}

/// Aperçu d'une seule étape (`&seul=1` dans l'adresse, ou `SNAKE_SEUL=1`) : elle recommence au lieu d'enchaîner la suivante.
pub fn single_step() -> bool {
    static R: LazyLock<bool> = LazyLock::new(|| game_common::save::url_param("seul").is_some_and(|v| v == "1") || std::env::var("SNAKE_SEUL").is_ok());
    *R
}

/// Aperçu d'un fichier de niveaux (éditeur) : rien ne doit être enregistré (étoiles, records, total).
pub fn preview_active() -> bool {
    editor_request().is_some() || std::env::var("SNAKE_FICHIER").is_ok()
}

/// Le texte du fichier d'aperçu : celui de l'éditeur en ligne, ou `SNAKE_FICHIER` en natif.
pub fn preview_text() -> Option<String> {
    if editor_request().is_some() {
        return game_common::save::load("snake", "editeur");
    }
    let path = std::env::var("SNAKE_FICHIER").ok()?;
    Some(std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{path} : {e}")))
}

/// Tous les niveaux, dans l'ordre du Voyage.
pub static LEVELS: LazyLock<Vec<Level>> = LazyLock::new(|| {
    // Aperçu d'un fichier (éditeur en ligne ou SNAKE_FICHIER) : ses niveaux jouables remplacent ceux du Voyage.
    if let Some(text) = preview_text() {
        match parse(&text) {
            Ok(levels) => {
                let playable: Vec<Level> = levels.into_iter().filter(|l| l.scene.is_none()).collect();
                if !playable.is_empty() {
                    return playable;
                }
            }
            Err(e) => warn!("aperçu : fichier illisible : {e}"),
        }
    }
    let mut all = Vec::new();
    for text in [include_str!("../levels/jardin.txt"), include_str!("../levels/desert.txt"), include_str!("../levels/ruines.txt"), include_str!("../levels/cimetiere.txt"), include_str!("../levels/pirates.txt"), include_str!("../levels/parc.txt"), include_str!("../levels/ville.txt"), include_str!("../levels/gare.txt"), include_str!("../levels/port.txt"), include_str!("../levels/medieval.txt"), include_str!("../levels/grottes.txt")] {
        all.extend(parse(text).unwrap_or_else(|e| panic!("niveaux illisibles : {e}")));
    }
    all
});

/// Écrit tous les niveaux en cartes dessinées (un fichier par lieu) : le point de départ pour les retoucher dans l'éditeur.
/// Les cases tirées au sort (blocs, trous, tapis…) sont posées exactement comme dans le jeu ; rivaux, trains et éboulements restent des réglages.
pub fn export(dir: &str) -> std::io::Result<()> {
    use crate::board::{Board, Tile};
    use game_common::Rng;
    std::fs::create_dir_all(dir)?;
    for place in Place::ALL {
        let mut out = format!("# {} — niveaux au format carte (voir l'en-tête de grid.rs ou l'éditeur).\n", place.label());
        for (i, level) in LEVELS.iter().enumerate().filter(|(_, l)| l.place == place) {
            let layout = level.layout(i as u32);
            let mut board = Board::default();
            let mut rng = Rng::new(0xC0FFEE + i as u64 * 7919);
            let busy = layout.start_cells();
            for (feature, n) in &level.cases {
                for _ in 0..*n {
                    board.place(*feature, &busy, layout.start, &layout, place.scenery(), &mut rng);
                }
            }
            out += &format!("\n[{}]\nlieu = {}\ntouristes = {}\npas = {}\n", level.name, place.key(), level.tourists, level.pace);
            if level.exit {
                out += "sortie = oui\n";
            }
            out += &format!("temps = {}\ndepart = {}\n", level.par.round(), grid::dir_name(layout.dir));
            for (f, n) in &level.cases {
                let key = match f {
                    Feature::Rival => "rivaux",
                    Feature::Train => "trains",
                    Feature::Cave => "eboulements",
                    _ => continue,
                };
                out += &format!("{key} = {n}\n");
            }
            out += "carte:\n";
            for y in 0..layout.h {
                let row: String = (0..layout.w)
                    .map(|x| {
                        let c = IVec2::new(x, y);
                        if !layout.walkable(c) {
                            '0'
                        } else if c == layout.start {
                            'S'
                        } else {
                            board.tiles.get(&c).and_then(Grid::char_of).unwrap_or('1')
                        }
                    })
                    .collect();
                out += &row;
                out.push('\n');
            }
            let _ = Tile::Hole;
        }
        std::fs::write(format!("{dir}/{}.txt", place.key()), out)?;
    }
    Ok(())
}

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
    fn reads_drawn_levels() {
        let text = "[Dessiné]\nlieu = jardin\ntouristes = 3\nsortie = oui\ndepart = est\ncarte:\n0011111100\n11S1111111\n1111A11A1E\n0011111100\n";
        let l = &parse(text).unwrap()[0];
        let layout = l.layout(0);
        assert_eq!((layout.w, layout.h, layout.start), (10, 4, IVec2::new(2, 1)));
        assert!(!layout.walkable(IVec2::new(0, 0)));
        assert_eq!(l.grid.as_ref().unwrap().exit(), Some(IVec2::new(9, 2)));
        assert!(parse("[X]\ncarte:\n111\n111").is_err(), "pas de départ");
    }

    #[test]
    fn special_levels_choose_their_scenery() {
        let text = "[À moi]\nlieu = special\npaysage = grottes\ntouristes = 3\ncarte:\n0011111100\n11S1111111\n1111A11A11\n0011111100\n";
        let l = &parse(text).unwrap()[0];
        assert_eq!((l.place, l.scenery()), (Place::Special, Universe::Cave));
        // Sans « paysage », un niveau normal prend le paysage de son lieu.
        let l = &parse("[X]\nlieu = ruines\ncarte:\n0011111100\n11S1111111\n0011111100\n").unwrap()[0];
        assert_eq!(l.scenery(), Universe::Ruines);
        assert!(parse("[X]\npaysage = mars").is_err());
    }

    #[test]
    fn reads_a_scene() {
        let text = "titre = Essai\ndescription = Une aventure\n[Intro]\ntype = scene\ndecor = ville\ncamera = gros\npersonnage = Léa | 1 | 12 | gauche-fond | salut | guide\npersonnage = Max | 2 | 5 | 1.5,0.5\nreplique = Léa | Bonjour !\nreplique = Max | Salut | bas\n[Niveau]\nlieu = jardin\ncarte:\n0011111100\n11S1111111\n0011111100\n";
        let (meta, levels) = parse_file(text).unwrap();
        assert_eq!((meta.title.as_str(), levels.len()), ("Essai", 2));
        let scene = levels[0].scene.as_ref().unwrap();
        assert_eq!((scene.camera, scene.actors.len(), scene.lines.len()), (Cam::Close, 2, 2));
        assert!(scene.actors[0].guide && scene.actors[0].z > 0.0 && scene.actors[1].x == 1.5);
        assert_eq!(scene.lines[1].camera, Some(Cam::Low));
        assert!(levels[1].scene.is_none() && levels[1].grid.is_some());
        assert!(parse("[X]\nreplique = A | b").is_err(), "réplique sans type = scene");
        assert!(parse("[X]\ntype = scene\npersonnage = A | 1").is_err());
        // Couleurs choisies une à une.
        let l = &parse("[X]\ntype = scene\npersonnage = A | 1 | 3.5.2.1.0 | centre").unwrap()[0];
        assert_eq!(l.scene.as_ref().unwrap().actors[0].look, 1000 + (((3 * 12 + 5) * 16 + 2) * 8 + 1) as u32 * 4);
        assert!(parse("[X]\ntype = scene\npersonnage = A | 1 | 9.0.0.0.0 | centre").is_err());
    }

    #[test]
    fn exported_levels_read_back() {
        let dir = std::env::temp_dir().join("snake-export-test");
        export(dir.to_str().unwrap()).unwrap();
        for place in Place::ALL {
            let text = std::fs::read_to_string(dir.join(format!("{}.txt", place.key()))).unwrap();
            let levels = parse(&text).unwrap_or_else(|e| panic!("{} : {e}", place.key()));
            assert_eq!(levels.len(), LEVELS.iter().filter(|l| l.place == place).count());
            assert!(levels.iter().all(|l| l.grid.is_some()));
        }
    }

    #[test]
    fn parser_reports_errors() {
        assert!(parse("[A]\nforme = triangle").is_err());
        assert!(parse("lieu = jardin").is_err());
    }
}
