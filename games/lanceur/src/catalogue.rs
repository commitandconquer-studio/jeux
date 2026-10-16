//! Les jeux du studio que le lanceur sait installer, rangés par univers.
//!
//! Pour ajouter un jeu : une entrée dans `JEUX` (avec son icône
//! `packaging/icons/<jeu>.png`), et son nom dans `JEUX` de
//! `tools/archives-linux.sh`.
use game_common::langue::Texte;
use game_common::tx;
use game_common::universe::Universe;

pub struct Jeu {
    /// Nom du paquet Cargo, de l'exécutable et de l'archive.
    pub id: &'static str,
    pub nom: Texte,
    pub description: Texte,
    pub univers: Universe,
    /// L'icône, intégrée au lanceur (il l'affiche même sans le jeu).
    pub icone: &'static [u8],
}

macro_rules! icone {
    ($id:literal) => {
        include_bytes!(concat!("../../../packaging/icons/", $id, ".png"))
    };
}

pub const JEUX: &[Jeu] = &[
    Jeu {
        id: "traversee",
        nom: tx!("Traversée", "Crossing", "Überquerung", "Traversata", "Travesía"),
        description: tx!(
            "Aidez le crapaud à traverser routes, rivières et voies ferrées.",
            "Help the toad cross roads, rivers and railways.",
            "Hilf der Kröte über Straßen, Flüsse und Gleise.",
            "Aiuta il rospo ad attraversare strade, fiumi e binari.",
            "Ayuda al sapo a cruzar carreteras, ríos y vías de tren.",
        ),
        univers: Universe::Croa,
        icone: icone!("traversee"),
    },
    Jeu {
        id: "tour",
        nom: tx!("Tour de blocs", "Block Tower", "Blockturm", "Torre di blocchi", "Torre de bloques"),
        description: tx!(
            "Empilez les blocs le plus haut possible.",
            "Stack the blocks as high as you can.",
            "Staple die Blöcke so hoch wie möglich.",
            "Impila i blocchi il più in alto possibile.",
            "Apila los bloques lo más alto posible.",
        ),
        univers: Universe::Croa,
        icone: icone!("tour"),
    },
    Jeu {
        id: "pousse-caisses",
        nom: tx!("Pousse-caisses", "Crate Pusher", "Kistenschieber", "Spingi-casse", "Empujacajas"),
        description: tx!(
            "Le crapaud range les caisses sur les cibles, en 12 niveaux.",
            "The toad pushes crates onto targets, across 12 levels.",
            "Die Kröte schiebt Kisten auf die Ziele, in 12 Levels.",
            "Il rospo spinge le casse sui bersagli, in 12 livelli.",
            "El sapo empuja las cajas a su sitio, en 12 niveles.",
        ),
        univers: Universe::Croa,
        icone: icone!("pousse-caisses"),
    },
    Jeu {
        id: "snake",
        nom: tx!("Snake", "Snake", "Snake", "Snake", "Snake"),
        description: tx!(
            "Le serpent classique en 3D, dans 5 paysages.",
            "The classic snake in 3D, across 5 landscapes.",
            "Die klassische Schlange in 3D, in 5 Landschaften.",
            "Il classico serpente in 3D, in 5 paesaggi.",
            "La serpiente clásica en 3D, en 5 paisajes.",
        ),
        univers: Universe::Croa,
        icone: icone!("snake"),
    },
    Jeu {
        id: "nova",
        nom: tx!("NOVA — Le Dernier Rempart", "NOVA — The Last Bulwark", "NOVA — Das letzte Bollwerk", "NOVA — L'ultimo baluardo", "NOVA — El último bastión"),
        description: tx!(
            "Bâtissez le bouclier de Nova et abattez le croiseur de l'Hégémonie.",
            "Build Nova's shield and bring down the Hegemony's cruiser.",
            "Baue Novas Schild und schieße den Kreuzer der Hegemonie ab.",
            "Costruisci lo scudo di Nova e abbatti l'incrociatore dell'Egemonia.",
            "Construye el escudo de Nova y derriba el crucero de la Hegemonía.",
        ),
        univers: Universe::Nova,
        icone: icone!("nova"),
    },
    Jeu {
        id: "parsecs",
        nom: tx!("12 Parsecs", "12 Parsecs", "12 Parsecs", "12 Parsec", "12 Pársecs"),
        description: tx!(
            "Traversez la ceinture d'astéroïdes et gagnez de nouveaux vaisseaux.",
            "Cross the asteroid belt and win new ships.",
            "Durchquere den Asteroidengürtel und gewinne neue Schiffe.",
            "Attraversa la fascia di asteroidi e vinci nuove navi.",
            "Cruza el cinturón de asteroides y gana nuevas naves.",
        ),
        univers: Universe::Nova,
        icone: icone!("parsecs"),
    },
    Jeu {
        id: "eureka-2048",
        nom: tx!("2048 — Eurêka", "2048 — Eureka", "2048 — Heureka", "2048 — Eureka", "2048 — Eureka"),
        description: tx!(
            "Fusionnez les puissances de 2 jusqu'à 2048.",
            "Merge powers of two all the way to 2048.",
            "Verschmelze Zweierpotenzen bis 2048.",
            "Unisci le potenze di 2 fino a 2048.",
            "Fusiona potencias de 2 hasta llegar a 2048.",
        ),
        univers: Universe::Eureka,
        icone: icone!("eureka-2048"),
    },
    Jeu {
        id: "concile",
        nom: tx!("Le Concile des Évêques", "The Council of Bishops", "Das Konzil der Bischöfe", "Il Concilio dei Vescovi", "El Concilio de los Obispos"),
        description: tx!(
            "Un évêque par diocèse, par rangée et par colonne, et jamais de voisin.",
            "One bishop per diocese, row and column, and never side by side.",
            "Ein Bischof pro Diözese, Zeile und Spalte, und nie nebeneinander.",
            "Un vescovo per diocesi, riga e colonna, e mai vicini.",
            "Un obispo por diócesis, fila y columna, y nunca juntos.",
        ),
        univers: Universe::Eureka,
        icone: icone!("concile"),
    },
    Jeu {
        id: "observatoire",
        nom: tx!("L'Observatoire", "The Observatory", "Die Sternwarte", "L'Osservatorio", "El Observatorio"),
        description: tx!(
            "Soleils et lunes sur la carte du ciel : autant de chaque, jamais trois à la suite.",
            "Suns and moons on the star chart: as many of each, never three in a row.",
            "Sonnen und Monde auf der Himmelskarte: gleich viele, nie drei in Folge.",
            "Soli e lune sulla mappa del cielo: tanti degli uni quanti delle altre, mai tre di fila.",
            "Soles y lunas en la carta celeste: tantos de cada, nunca tres seguidos.",
        ),
        univers: Universe::Eureka,
        icone: icone!("observatoire"),
    },
];

/// Les univers, dans l'ordre d'affichage.
pub const UNIVERS: [Universe; 3] = [Universe::Croa, Universe::Nova, Universe::Eureka];

/// Le lanceur lui-même : il se met à jour comme un jeu.
pub const LANCEUR: &str = "lanceur";

/// Le nom du lanceur (menu d'applications).
pub const NOM_LANCEUR: Texte = tx!(
    "Commit & Conquer",
    "Commit & Conquer",
    "Commit & Conquer",
    "Commit & Conquer",
    "Commit & Conquer",
);

pub const DESCRIPTION_LANCEUR: Texte = tx!(
    "Installe, met à jour et lance les jeux du studio",
    "Installs, updates and launches the studio's games",
    "Installiert, aktualisiert und startet die Spiele des Studios",
    "Installa, aggiorna e avvia i giochi dello studio",
    "Instala, actualiza y abre los juegos del estudio",
);
