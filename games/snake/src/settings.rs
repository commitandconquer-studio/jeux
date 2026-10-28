//! Réglages du jeu, modifiables depuis l'écran « Réglages ».
use bevy::prelude::*;
use game_common::tx;

use crate::layout::{Layout, Shape};
use crate::progress::{Item, Progress};
use crate::world::Universe;

#[derive(Resource, Clone, Copy, Default)]
pub struct Settings {
    pub universe: Universe,
    pub speed: Speed,
    pub shape: Shape,
    /// `true` : on traverse les bords ; `false` : les bords tuent.
    pub wrap: bool,
    /// Personnage qui mène la chenille (indice dans `people::KINDS`).
    pub leader: usize,
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

impl Settings {
    /// La forme du plateau choisie.
    pub fn layout(&self) -> Layout {
        self.shape.layout()
    }
}

/// Une ligne de l'écran d'options.
#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub enum OptionKind {
    Universe,
    Speed,
    Size,
    Edges,
    Leader,
}

impl OptionKind {
    pub const ALL: [OptionKind; 5] = [OptionKind::Leader, OptionKind::Universe, OptionKind::Speed, OptionKind::Size, OptionKind::Edges];

    pub fn text(self, s: &Settings, _p: &Progress) -> String {
        let (label, value) = match self {
            OptionKind::Universe => (tx!("Paysage :", "Scenery:", "Landschaft:", "Paesaggio:", "Paisaje:"), s.universe.label()),
            OptionKind::Leader => (tx!("Chef de file :", "Leader:", "Anführer:", "Capofila:", "Cabeza de fila:"), crate::people::name(s.leader)),
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
                match s.shape {
                    Shape::Small => tx!("Petit", "Small", "Klein", "Piccolo", "Pequeño"),
                    Shape::Medium => tx!("Moyen", "Medium", "Mittel", "Medio", "Mediano"),
                    Shape::Large => tx!("Grand", "Large", "Groß", "Grande", "Grande"),
                    Shape::Rectangle => tx!("Rectangle", "Rectangle", "Rechteck", "Rettangolo", "Rectángulo"),
                    Shape::L => tx!("En L", "L-shaped", "L-förmig", "A L", "En L"),
                    Shape::U => tx!("En U", "U-shaped", "U-förmig", "A U", "En U"),
                    Shape::Route => tx!("Route sinueuse", "Winding road", "Kurvige Straße", "Strada tortuosa", "Camino sinuoso"),
                    Shape::Alleys => tx!("Allées", "Alleys", "Alleen", "Viali", "Paseos"),
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

    pub fn cycle(self, s: &mut Settings, p: &Progress) {
        match self {
            OptionKind::Universe => {
                // Saute les paysages encore verrouillés.
                let mut u = s.universe.next();
                while !p.has(Item::World(u)) {
                    u = u.next();
                }
                s.universe = u;
            }
            OptionKind::Leader => {
                let pool = p.persons();
                let i = pool.iter().position(|k| *k == s.leader).unwrap_or(0);
                s.leader = pool[(i + 1) % pool.len()];
            }
            OptionKind::Speed => {
                s.speed = match s.speed {
                    Speed::Lente => Speed::Normale,
                    Speed::Normale => Speed::Rapide,
                    Speed::Rapide => Speed::Lente,
                }
            }
            OptionKind::Size => s.shape = s.shape.next(),
            OptionKind::Edges => s.wrap = !s.wrap,
        }
    }
}
