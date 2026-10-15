//! L'histoire : les 5 chapitres, leurs réglages et les dialogues.
//!
//! Univers original Commit & Conquer : la colonie libre de Nova résiste à
//! l'Hégémonie. Isa Valen, ingénieure en chef, bâtit le Bouclier de Nova ;
//! Boulon, petit drone de maintenance, l'accompagne (et commente).
use bevy::prelude::Color;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Speaker {
    Isa,
    Boulon,
}

impl Speaker {
    pub fn name(self) -> &'static str {
        match self {
            Speaker::Isa => "Isa Valen",
            Speaker::Boulon => "Boulon",
        }
    }

    pub fn color(self) -> Color {
        match self {
            Speaker::Isa => Color::srgb(0.20, 0.45, 0.80),
            Speaker::Boulon => Color::srgb(0.85, 0.50, 0.10),
        }
    }
}

pub type Line = (Speaker, &'static str);

pub struct Chapter {
    pub name: &'static str,
    pub planet: Color,
    /// La planète brille (lave).
    pub planet_glow: bool,
    /// Points de coque du croiseur ennemi.
    pub hull: f32,
    /// Secondes entre deux descentes d'un cran au début.
    pub gravity: f32,
    /// Secondes entre deux tirs ennemis.
    pub fire_every: f32,
    pub intro: &'static [Line],
    pub outro: &'static [Line],
}

use Speaker::{Boulon, Isa};

pub const CHAPTERS: [Chapter; 5] = [
    Chapter {
        name: "Nova Prime",
        planet: Color::srgb(0.30, 0.62, 0.42),
        planet_glow: false,
        hull: 100.0,
        gravity: 0.9,
        fire_every: 40.0,
        intro: &[
            (Isa, "Boulon, réveille-toi. Un croiseur de l'Hégémonie vient de sortir de l'hyperespace."),
            (Boulon, "Un croiseur ? Un gros ? Avec des canons ? Je peux aller me cacher dans la soute ?"),
            (Isa, "Le Bouclier de Nova est notre seule défense. Chaque rangée d'énergie complète charge le canon à ions."),
            (Isa, "Plus on complète de rangées d'un coup, plus le tir est puissant. On tient bon."),
        ],
        outro: &[
            (Boulon, "Il… il recule ! Il fume ! On l'a eu !"),
            (Isa, "Ce n'était qu'un éclaireur. L'Hégémonie en enverra d'autres. Cap sur Kessa."),
        ],
    },
    Chapter {
        name: "Kessa",
        planet: Color::srgb(0.85, 0.64, 0.38),
        planet_glow: false,
        hull: 150.0,
        gravity: 0.75,
        fire_every: 32.0,
        intro: &[
            (Isa, "Les raffineries de Kessa alimentent toute la colonie. Si elles tombent, Nova gèle."),
            (Boulon, "J'ai du sable dans les engrenages. Rien de grave. Enfin, j'espère."),
            (Isa, "Attention : leurs tirs abîment le bas du bouclier. Garde-le propre."),
        ],
        outro: &[
            (Isa, "Les raffineries sont sauves. Mais leurs croiseurs deviennent plus rapides."),
            (Boulon, "Plus rapides ? Moi aussi je peux être rapide. Surtout pour fuir."),
        ],
    },
    Chapter {
        name: "Hélion",
        planet: Color::srgb(0.75, 0.88, 0.98),
        planet_glow: false,
        hull: 200.0,
        gravity: 0.62,
        fire_every: 26.0,
        intro: &[
            (Boulon, "Il fait moins 140 degrés. Mes capteurs ont froid. Est-ce que les capteurs peuvent avoir froid ?"),
            (Isa, "Concentre-toi. Ils tirent plus souvent ici : garde le bouclier bas."),
        ],
        outro: &[
            (Boulon, "La lune de glace tient ! Et moi aussi, à peu près."),
            (Isa, "Prochaine étape : Thalassa. Les familles évacuées comptent sur nous."),
        ],
    },
    Chapter {
        name: "Thalassa",
        planet: Color::srgb(0.20, 0.50, 0.85),
        planet_glow: false,
        hull: 260.0,
        gravity: 0.52,
        fire_every: 22.0,
        intro: &[
            (Isa, "Thalassa abrite les familles évacuées. Pas question de reculer."),
            (Boulon, "Pour une fois, je suis d'accord. On tient. Et on gagne. Dans cet ordre."),
        ],
        outro: &[
            (Isa, "Thalassa est sauvée. Il est temps de porter le combat chez eux."),
            (Boulon, "Chez eux ? Vous voulez dire… à la Forge ? Oh non. Oh non non non."),
        ],
    },
    Chapter {
        name: "La Forge",
        planet: Color::srgb(0.95, 0.38, 0.10),
        planet_glow: true,
        hull: 340.0,
        gravity: 0.42,
        fire_every: 18.0,
        intro: &[
            (Isa, "Voilà la Forge, le chantier de l'Hégémonie. Et leur vaisseau amiral, le Souverain."),
            (Boulon, "Il est… très gros. Très, très gros."),
            (Isa, "Plus il est gros, plus il est facile à toucher. Chargez le canon."),
        ],
        outro: &[
            (Boulon, "Le Souverain se replie ! Toute la flotte se replie !"),
            (Isa, "Nova est libre. Beau travail, Boulon."),
            (Boulon, "C'est surtout vous. Moi, j'ai juste un peu tremblé."),
        ],
    },
];

/// Petites remarques de Boulon pendant la partie.
pub const BOULON_FOUR: &str = "Quatre rangées d'un coup ! Ils ont dû le sentir passer !";
pub const BOULON_WARNING: &str = "Ils chargent leurs canons ! Attention en bas !";
pub const BOULON_HALF: &str = "Leur coque est à moitié fondue. Continuez !";
