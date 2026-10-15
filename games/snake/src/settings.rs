//! Options du jeu, modifiables depuis l'écran d'options.
use bevy::prelude::*;

use crate::world::Universe;

#[derive(Resource, Clone, Copy, Default)]
pub struct Settings {
    pub universe: Universe,
    pub speed: Speed,
    pub size: BoardSize,
    /// `true` : on traverse les bords ; `false` : les bords tuent.
    pub wrap: bool,
}

impl Settings {
    pub fn grid(&self) -> i32 {
        self.size.cells()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum Speed {
    Lente,
    #[default]
    Normale,
    Rapide,
}

impl Speed {
    /// (durée d'un pas au départ, durée minimale) en secondes.
    pub fn step_times(self) -> (f32, f32) {
        match self {
            Speed::Lente => (0.22, 0.10),
            Speed::Normale => (0.16, 0.07),
            Speed::Rapide => (0.11, 0.05),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum BoardSize {
    Petit,
    #[default]
    Moyen,
    Grand,
}

impl BoardSize {
    pub fn cells(self) -> i32 {
        match self {
            BoardSize::Petit => 10,
            BoardSize::Moyen => 14,
            BoardSize::Grand => 18,
        }
    }
}

/// Une ligne de l'écran d'options.
#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub enum OptionKind {
    Universe,
    Speed,
    Size,
    Edges,
}

impl OptionKind {
    pub const ALL: [OptionKind; 4] = [OptionKind::Universe, OptionKind::Speed, OptionKind::Size, OptionKind::Edges];

    pub fn text(self, s: &Settings) -> String {
        let (label, value) = match self {
            OptionKind::Universe => ("Univers", s.universe.label()),
            OptionKind::Speed => (
                "Vitesse",
                match s.speed {
                    Speed::Lente => "Lente",
                    Speed::Normale => "Normale",
                    Speed::Rapide => "Rapide",
                },
            ),
            OptionKind::Size => (
                "Plateau",
                match s.size {
                    BoardSize::Petit => "Petit",
                    BoardSize::Moyen => "Moyen",
                    BoardSize::Grand => "Grand",
                },
            ),
            OptionKind::Edges => ("Bords", if s.wrap { "Traversables" } else { "Murs" }),
        };
        format!("{label} :  {value}")
    }

    pub fn cycle(self, s: &mut Settings) {
        match self {
            OptionKind::Universe => s.universe = s.universe.next(),
            OptionKind::Speed => {
                s.speed = match s.speed {
                    Speed::Lente => Speed::Normale,
                    Speed::Normale => Speed::Rapide,
                    Speed::Rapide => Speed::Lente,
                }
            }
            OptionKind::Size => {
                s.size = match s.size {
                    BoardSize::Petit => BoardSize::Moyen,
                    BoardSize::Moyen => BoardSize::Grand,
                    BoardSize::Grand => BoardSize::Petit,
                }
            }
            OptionKind::Edges => s.wrap = !s.wrap,
        }
    }
}
