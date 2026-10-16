//! Les réglages propres à 12 Parsecs (bouton « Réglages » du menu) :
//! la façon de piloter et le tir automatique. Les Options communes (langue,
//! son, musique, volumes, ambiance) sont dans le pied du menu.
use bevy::prelude::*;

const SAVE: &str = "parsecs";

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Pilotage {
    /// Tourner et accélérer, avec de l'inertie (comme le jeu d'origine).
    Classique,
    /// Le vaisseau va où l'on pousse, il pointe toujours vers le haut.
    Direct,
}

#[derive(Resource, Clone, Copy, Debug)]
pub struct Reglages {
    pub pilotage: Pilotage,
    pub tir_auto: bool,
}

impl Reglages {
    pub fn charger() -> Reglages {
        let v = game_common::save::load(SAVE, "reglages").unwrap_or_default();
        let mut m = v.split_whitespace();
        // Par défaut : pilotage direct, plus simple pour commencer ; tir auto
        // sur le web et le tactile.
        let pilotage = if m.next() == Some("classique") { Pilotage::Classique } else { Pilotage::Direct };
        let tir_auto = match m.next() {
            Some(x) => x == "1",
            None => cfg!(any(target_arch = "wasm32", target_os = "android")),
        };
        Reglages { pilotage, tir_auto }
    }

    pub fn enregistrer(&self) {
        let p = if self.pilotage == Pilotage::Classique { "classique" } else { "direct" };
        game_common::save::store(SAVE, "reglages", &format!("{p} {}", if self.tir_auto { 1 } else { 0 }));
    }
}
