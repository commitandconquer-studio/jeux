//! Les chiens à débloquer : le même chien, six pelages. Chaque pelage s'ouvre
//! avec un nombre d'étoiles gagnées dans la campagne.
use bevy::prelude::*;
use game_common::langue::Texte;
use game_common::tx;

use crate::jeu::Progression;
use crate::vue::Kit;

pub struct Chien {
    pub nom: Texte,
    /// Étoiles à avoir gagnées pour l'ouvrir.
    pub seuil: u32,
    /// Corps, tête, oreilles, queue.
    pub pelage: Color,
    /// Poitrail, museau, pattes, bout de queue.
    pub accent: Color,
}

pub const CHIENS: [Chien; 6] = [
    Chien { nom: tx!("Border", "Border", "Border", "Border", "Border"), seuil: 0, pelage: Color::srgb(0.09, 0.09, 0.11), accent: Color::srgb(0.96, 0.96, 0.95) },
    Chien { nom: tx!("Caramel", "Caramel", "Karamell", "Caramello", "Caramelo"), seuil: 4, pelage: Color::srgb(0.72, 0.46, 0.22), accent: Color::srgb(0.97, 0.90, 0.76) },
    Chien { nom: tx!("Neige", "Snow", "Schnee", "Neve", "Nieve"), seuil: 9, pelage: Color::srgb(0.95, 0.95, 0.93), accent: Color::srgb(0.72, 0.76, 0.82) },
    Chien { nom: tx!("Ardoise", "Slate", "Schiefer", "Ardesia", "Pizarra"), seuil: 15, pelage: Color::srgb(0.36, 0.42, 0.52), accent: Color::srgb(0.93, 0.93, 0.92) },
    Chien { nom: tx!("Braise", "Ember", "Glut", "Brace", "Brasa"), seuil: 22, pelage: Color::srgb(0.13, 0.1, 0.1), accent: Color::srgb(0.78, 0.42, 0.18) },
    Chien { nom: tx!("Doré", "Goldie", "Goldi", "Dorato", "Dorado"), seuil: 30, pelage: Color::srgb(0.9, 0.7, 0.3), accent: Color::srgb(1.0, 0.97, 0.88) },
];

/// Le chien affiché à l'écran « Mon chien » (et non encore choisi).
#[derive(Resource, Default)]
pub struct ChienVu(pub usize);

pub struct ChiensPlugin;

impl Plugin for ChiensPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ChienVu>().add_systems(Update, peindre);
    }
}

/// Repeint le chien de la scène (matières propres au chien) : le chien choisi,
/// ou celui qu'on regarde à l'écran « Mon chien ».
fn peindre(
    progression: Res<Progression>,
    vu: Res<ChienVu>,
    state: Res<State<crate::GameState>>,
    kit: Res<Kit>,
    mut mats: ResMut<Assets<StandardMaterial>>,
    mut fait: Local<Option<usize>>,
) {
    let voulu = if matches!(*state.get(), crate::GameState::Chiens | crate::GameState::Playing) && crate::capture::mode_kit() || *state.get() == crate::GameState::Chiens { vu.0 } else { progression.chien };
    if *fait == Some(voulu) {
        return;
    }
    let c = &CHIENS[voulu.min(CHIENS.len() - 1)];
    if let Some(mut m) = mats.get_mut(&kit.chien_pelage) {
        m.base_color = c.pelage;
    }
    if let Some(mut m) = mats.get_mut(&kit.chien_accent) {
        m.base_color = c.accent;
    }
    *fait = Some(voulu);
}
