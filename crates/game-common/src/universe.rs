//! Les univers du studio : chaque jeu appartient à l'un d'eux et en reprend
//! l'identité (police, couleurs de l'interface, ambiance).
//!
//! - **Croâ & Cie** : enfantin et sympa — jouets en bois, pastels, animaux.
//! - **Nova** : science-fiction — espace profond, néons, vaisseaux.
//! - **Eurêka** : sciences et maths — cahier quadrillé, feutres, laboratoire.
use bevy::prelude::*;

use crate::{palette, sounds};

#[derive(Resource, Clone, Copy, PartialEq, Eq, Debug)]
pub enum Universe {
    Croa,
    Nova,
    Eureka,
}

/// Style de l'interface d'un univers.
#[derive(Clone, Copy)]
pub struct UiStyle {
    pub font: &'static str,
    pub panel: Color,
    /// Bordure des panneaux (transparente s'il n'y en a pas).
    pub panel_border: Color,
    pub text: Color,
    pub button: Color,
    pub button_hover: Color,
    pub button_border: Color,
    /// Couleur d'accent (titres, jauges…).
    pub accent: Color,
}

impl Universe {
    pub fn name(self) -> &'static str {
        match self {
            Universe::Croa => "Croâ & Cie",
            Universe::Nova => "Nova",
            Universe::Eureka => "Eurêka",
        }
    }

    pub fn style(self) -> UiStyle {
        match self {
            Universe::Croa => UiStyle {
                font: sounds::FONT,
                panel: Color::srgba(0.98, 0.97, 0.94, 0.93),
                panel_border: Color::NONE,
                text: palette::TEXT_DARK,
                button: Color::srgb(0.98, 0.97, 0.93),
                button_hover: Color::srgb(1.0, 0.86, 0.40),
                button_border: palette::TEXT_DARK,
                accent: Color::srgb(0.36, 0.72, 0.30),
            },
            Universe::Nova => UiStyle {
                font: sounds::FONT_NOVA,
                panel: Color::srgba(0.04, 0.07, 0.15, 0.88),
                panel_border: Color::srgb(0.30, 0.85, 1.0),
                text: Color::srgb(0.88, 0.94, 1.0),
                button: Color::srgba(0.08, 0.14, 0.26, 0.95),
                button_hover: Color::srgb(0.12, 0.38, 0.58),
                button_border: Color::srgb(0.30, 0.85, 1.0),
                accent: Color::srgb(1.0, 0.42, 0.25),
            },
            Universe::Eureka => UiStyle {
                font: sounds::FONT_EUREKA,
                panel: Color::srgba(1.0, 0.99, 0.95, 0.96),
                panel_border: Color::srgb(0.20, 0.40, 0.80),
                text: Color::srgb(0.12, 0.16, 0.35),
                button: Color::srgb(1.0, 1.0, 0.98),
                button_hover: Color::srgb(1.0, 0.92, 0.45),
                button_border: Color::srgb(0.20, 0.40, 0.80),
                accent: Color::srgb(0.90, 0.25, 0.30),
            },
        }
    }
}
