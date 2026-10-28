//! Les jeux du studio que le lanceur sait installer, .
//!
//! Pour ajouter un jeu : une entrée dans `JEUX` (avec son icône
//! `packaging/icons/<jeu>.png`), et son nom dans `JEUX` de
//! `tools/archives-linux.sh`.
use game_common::langue::Texte;
use game_common::tx;

pub struct Jeu {
    /// Nom du paquet Cargo, de l'exécutable et de l'archive.
    pub id: &'static str,
    pub nom: Texte,
    pub description: Texte,
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
        nom: tx!("Cross or Die", "Cross or Die", "Cross or Die", "Cross or Die", "Cross or Die"),
        description: tx!(
            "Le pire service de livraison du multivers : livrez vos colis et survivez aux routes, aux trains et aux aigles.",
            "The worst delivery service in the multiverse: deliver your parcels and survive roads, trains and eagles.",
            "Der schlechteste Lieferdienst des Multiversums: liefere deine Pakete und überlebe Straßen, Züge und Adler.",
            "Il peggior servizio di consegne del multiverso: consegna i tuoi pacchi e sopravvivi a strade, treni e aquile.",
            "El peor servicio de reparto del multiverso: entrega tus paquetes y sobrevive a carreteras, trenes y águilas.",
        ),
        icone: icone!("traversee"),
    },
    Jeu {
        id: "snake",
        nom: tx!("Suivez le guide !", "Follow the Guide!", "Folgt dem Führer!", "Seguite la guida!", "¡Sigue al guía!"),
        description: tx!(
            "Guidez une file de touristes à travers 11 lieux : chaque invité recruté s'accroche derrière vous.",
            "Lead a line of tourists through 11 places: every guest you recruit joins in behind you.",
            "Führe eine Touristenschlange durch 11 Orte: jeder Gast reiht sich hinter dir ein.",
            "Guida una fila di turisti in 11 luoghi: ogni ospite reclutato si attacca dietro di te.",
            "Guía una fila de turistas por 11 lugares: cada invitado se une detrás de ti.",
        ),
        icone: icone!("snake"),
    },
    Jeu {
        id: "berger",
        nom: tx!("Le Chien de berger", "The Sheepdog", "Der Schäferhund", "Il Cane pastore", "El Perro pastor"),
        description: tx!(
            "Rassemblez le troupeau et rentrez-le dans l'enclos, avant que les loups n'arrivent.",
            "Gather the flock and bring it into the pen before the wolves arrive.",
            "Treibe die Herde ins Gehege, bevor die Wölfe kommen.",
            "Raduna il gregge e portalo nel recinto prima che arrivino i lupi.",
            "Reúne el rebaño y llévalo al corral antes de que lleguen los lobos.",
        ),
        icone: icone!("berger"),
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
        icone: icone!("tour"),
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
        icone: icone!("eureka-2048"),
    },
];

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
