//! Les langues des jeux : français, anglais, allemand, italien, espagnol.
//!
//! Un texte traduit s'écrit avec la macro `tx!` (dans cet ordre : français,
//! anglais, allemand, italien, espagnol), puis `.t()` donne la version dans
//! la langue choisie :
//!
//! ```ignore
//! let jouer = tx!("Jouer", "Play", "Spielen", "Gioca", "Jugar").t();
//! let niveau = tx!("Niveau {}", "Level {}", "Level {}", "Livello {}", "Nivel {}").avec(&[&n]);
//! ```
//!
//! La langue est celle du système au premier lancement, puis celle choisie
//! dans les Options (commune à tous les jeux du studio, « studio-langue »).
use std::fmt::Display;
use std::sync::atomic::{AtomicU8, Ordering};

use bevy::prelude::*;

#[derive(Resource, Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Langue {
    #[default]
    Fr,
    En,
    De,
    It,
    Es,
}

impl Langue {
    pub const TOUTES: [Langue; 5] = [Langue::Fr, Langue::En, Langue::De, Langue::It, Langue::Es];

    /// Code ISO (`fr`, `en`…).
    pub fn code(self) -> &'static str {
        match self {
            Langue::Fr => "fr",
            Langue::En => "en",
            Langue::De => "de",
            Langue::It => "it",
            Langue::Es => "es",
        }
    }

    /// Le nom de la langue, dans cette langue (pour le choix des Options).
    pub fn nom(self) -> &'static str {
        match self {
            Langue::Fr => "Français",
            Langue::En => "English",
            Langue::De => "Deutsch",
            Langue::It => "Italiano",
            Langue::Es => "Español",
        }
    }

    pub fn depuis_code(code: &str) -> Option<Langue> {
        let c = code.trim().to_lowercase();
        Langue::TOUTES.into_iter().find(|l| c.starts_with(l.code()))
    }

    pub fn suivante(self) -> Langue {
        let i = Langue::TOUTES.iter().position(|l| *l == self).unwrap_or(0);
        Langue::TOUTES[(i + 1) % Langue::TOUTES.len()]
    }

    /// La langue enregistrée, sinon celle du système, sinon l'anglais
    /// (le français si le système est en français).
    pub fn initiale() -> Langue {
        if let Some(l) = std::env::var("CC_LANGUE").ok().as_deref().and_then(Langue::depuis_code) {
            return l;
        }
        if let Some(l) = crate::save::load("studio", "langue").as_deref().and_then(Langue::depuis_code) {
            return l;
        }
        systeme().unwrap_or(Langue::En)
    }

    pub fn enregistrer(self) {
        crate::save::store("studio", "langue", self.code());
    }
}

/// La langue du système (variables LANGUAGE, LC_ALL, LC_MESSAGES, LANG ; dans
/// le navigateur, sa langue).
fn systeme() -> Option<Langue> {
    #[cfg(target_arch = "wasm32")]
    {
        let nav = web_sys::window()?.navigator();
        return nav.language().as_deref().and_then(Langue::depuis_code);
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        ["LANGUAGE", "LC_ALL", "LC_MESSAGES", "LANG"]
            .iter()
            .filter_map(|v| std::env::var(v).ok())
            .flat_map(|v| v.split(':').map(str::to_string).collect::<Vec<_>>())
            .find_map(|v| Langue::depuis_code(&v))
    }
}

// La langue courante, lisible partout (même hors d'un système Bevy).
static ACTUELLE: AtomicU8 = AtomicU8::new(0);

/// La langue en cours.
pub fn actuelle() -> Langue {
    Langue::TOUTES[ACTUELLE.load(Ordering::Relaxed) as usize % 5]
}

/// Change la langue tout de suite (la ressource `Langue` suit au prochain passage).
pub fn definir_maintenant(l: Langue) {
    definir(l);
}

fn definir(l: Langue) {
    let i = Langue::TOUTES.iter().position(|x| *x == l).unwrap_or(0);
    ACTUELLE.store(i as u8, Ordering::Relaxed);
}

/// « : » avec l'espace qui le précède en français seulement (« Son : activé »,
/// mais « Sound: on »).
pub fn deux_points() -> &'static str {
    if actuelle() == Langue::Fr { " : " } else { ": " }
}

/// Un texte dans les cinq langues.
#[derive(Clone, Copy, Debug)]
pub struct Texte {
    pub fr: &'static str,
    pub en: &'static str,
    pub de: &'static str,
    pub it: &'static str,
    pub es: &'static str,
}

impl Texte {
    /// Dans une langue donnée.
    pub fn en(&self, l: Langue) -> &'static str {
        match l {
            Langue::Fr => self.fr,
            Langue::En => self.en,
            Langue::De => self.de,
            Langue::It => self.it,
            Langue::Es => self.es,
        }
    }

    /// Dans la langue en cours.
    pub fn t(&self) -> &'static str {
        self.en(actuelle())
    }

    /// Dans la langue en cours, chaque `{}` remplacé par une valeur, dans l'ordre.
    pub fn avec(&self, valeurs: &[&dyn Display]) -> String {
        let mut sortie = String::new();
        let mut morceaux = self.t().split("{}");
        if let Some(premier) = morceaux.next() {
            sortie.push_str(premier);
        }
        for (i, m) in morceaux.enumerate() {
            if let Some(v) = valeurs.get(i) {
                sortie.push_str(&v.to_string());
            }
            sortie.push_str(m);
        }
        sortie
    }
}

/// Un texte en cinq langues : `tx!(français, anglais, allemand, italien, espagnol)`.
#[macro_export]
macro_rules! tx {
    ($fr:expr, $en:expr, $de:expr, $it:expr, $es:expr $(,)?) => {
        $crate::langue::Texte { fr: $fr, en: $en, de: $de, it: $it, es: $es }
    };
}

/// La langue a changé (dans les Options) : les jeux reconstruisent leur menu.
#[derive(Message, Clone, Copy)]
pub struct LangueChangee;

pub struct LanguePlugin;

impl Plugin for LanguePlugin {
    fn build(&self, app: &mut App) {
        let l = Langue::initiale();
        definir(l);
        app.insert_resource(l).add_message::<LangueChangee>().add_systems(PreUpdate, suivre);
    }
}

/// La ressource `Langue` est la référence : on recopie sa valeur pour `actuelle()`.
fn suivre(l: Res<Langue>) {
    if l.is_changed() {
        definir(*l);
    }
}

/// À ajouter dans chaque jeu : quand la langue change, le menu est rebâti
/// (on y repasse), et tous ses textes sont dans la nouvelle langue.
///
/// ```ignore
/// app.add_systems(Update, game_common::langue::rebatir_menu(GameState::Menu));
/// ```
pub fn rebatir_menu<S: bevy::state::state::FreelyMutableState + Clone>(menu: S) -> impl FnMut(MessageReader<LangueChangee>, ResMut<NextState<S>>) {
    move |mut changees: MessageReader<LangueChangee>, mut next: ResMut<NextState<S>>| {
        if changees.read().count() > 0 {
            next.set(menu.clone());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn codes() {
        assert_eq!(Langue::depuis_code("de_DE.UTF-8"), Some(Langue::De));
        assert_eq!(Langue::depuis_code("es-ES"), Some(Langue::Es));
        assert_eq!(Langue::depuis_code("pt_BR"), None);
    }

    #[test]
    fn remplace() {
        let t = tx!("Niveau {} sur {}", "Level {} of {}", "Level {} von {}", "Livello {} di {}", "Nivel {} de {}");
        definir(Langue::It);
        assert_eq!(t.avec(&[&3, &12]), "Livello 3 di 12");
        definir(Langue::Fr);
    }
}
