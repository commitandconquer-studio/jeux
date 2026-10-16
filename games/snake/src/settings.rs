//! Réglages du jeu, modifiables depuis l'écran « Réglages ».
use bevy::prelude::*;
use game_common::tx;

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
            OptionKind::Universe => (tx!("Univers :", "World:", "Welt:", "Mondo:", "Mundo:"), s.universe.label()),
            OptionKind::Speed => (
                tx!("Vitesse :", "Speed:", "Tempo:", "Velocità:", "Velocidad:"),
                match s.speed {
                    Speed::Lente => tx!("Lente", "Slow", "Langsam", "Lenta", "Lenta"),
                    Speed::Normale => tx!("Normale", "Normal", "Normal", "Normale", "Normal"),
                    Speed::Rapide => tx!("Rapide", "Fast", "Schnell", "Veloce", "Rápida"),
                }
                .t(),
            ),
            OptionKind::Size => (
                tx!("Plateau :", "Board:", "Spielfeld:", "Campo:", "Tablero:"),
                match s.size {
                    BoardSize::Petit => tx!("Petit", "Small", "Klein", "Piccolo", "Pequeño"),
                    BoardSize::Moyen => tx!("Moyen", "Medium", "Mittel", "Medio", "Mediano"),
                    BoardSize::Grand => tx!("Grand", "Large", "Groß", "Grande", "Grande"),
                }
                .t(),
            ),
            OptionKind::Edges => (
                tx!("Bords :", "Edges:", "Ränder:", "Bordi:", "Bordes:"),
                if s.wrap {
                    tx!("Traversables", "Wrap around", "Durchlässig", "Attraversabili", "Atravesables")
                } else {
                    tx!("Murs", "Walls", "Wände", "Muri", "Muros")
                }
                .t(),
            ),
        };
        format!("{}  {}", label.t(), value)
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
