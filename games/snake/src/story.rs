//! Le mode scénario : des aventures faites de scènes (décor, personnages, sous-titres) et de niveaux, enchaînées.
//! Une aventure est un fichier de `games/snake/scenarios/` (le même format que les niveaux, avec des sections `type = scene`).
use std::collections::HashMap;
use std::sync::LazyLock;

use bevy::prelude::*;

use crate::GameState;
use crate::levels::{Level, Meta, parse_file, preview_text};
use crate::voyage::{Mode, Outcome};

pub struct Scenario {
    pub title: String,
    #[allow(dead_code)] // gardée pour la fiche de l'aventure (éditeur, futur écran de choix)
    pub description: String,
    pub steps: Vec<Level>,
}

fn build(text: &str, fallback_title: &str) -> Result<Scenario, String> {
    let (Meta { title, description }, steps) = parse_file(text)?;
    Ok(Scenario { title: if title.is_empty() { fallback_title.to_owned() } else { title }, description, steps })
}

/// Toutes les aventures. En aperçu (éditeur), seul le fichier d'aperçu.
pub static SCENARIOS: LazyLock<Vec<Scenario>> = LazyLock::new(|| {
    if let Some(text) = preview_text() {
        let problem = match build(&text, "Aperçu") {
            Ok(s) if !s.steps.is_empty() => return vec![s],
            Ok(_) => "aucune étape dans le fichier".to_owned(),
            Err(e) => e,
        };
        warn!("aperçu : {problem}");
        // Repli : une scène qui affiche le problème, pour que ce soit visible dans l'éditeur (et que le jeu ne plante pas).
        let shown: String = problem.chars().filter(|c| !matches!(c, '|' | '\n' | '\r')).collect();
        return vec![build(&format!("[Aperçu illisible]\ntype = scene\nreplique = Aperçu | Le fichier d'aperçu est illisible — {shown}\n"), "Aperçu").expect("aventure de repli")];
    }
    let mut all = Vec::new();
    for (name, text) in [("Le grand voyage", include_str!("../scenarios/aventure.txt"))] {
        all.push(build(text, name).unwrap_or_else(|e| panic!("aventure « {name} » illisible : {e}")));
    }
    all
});

/// Où en est le joueur dans chaque aventure : la prochaine étape, par titre.
#[derive(Resource, Default)]
pub struct StoryProgress {
    next: HashMap<String, usize>,
    /// Meilleur résultat par étape : (aventure, étape) → (étoiles, meilleur temps). Les anciennes sauvegardes n'en ont pas.
    best: HashMap<(String, usize), (u8, f32)>,
}


impl StoryProgress {
    pub fn next_step(&self, title: &str) -> usize {
        self.next.get(title).copied().unwrap_or(0)
    }

    pub fn best(&self, title: &str, step: usize) -> Option<(u8, f32)> {
        self.best.get(&(title.to_owned(), step)).copied()
    }

    pub fn total_stars(&self, title: &str) -> u32 {
        self.best.iter().filter(|((t, _), _)| t == title).map(|(_, (s, _))| *s as u32).sum()
    }

    fn record(&mut self, title: &str, step: usize, stars: u8, time: f32) {
        if crate::levels::preview_active() {
            return;
        }
        let e = self.best.entry((title.to_owned(), step)).or_insert((0, f32::MAX));
        e.0 = e.0.max(stars);
        e.1 = e.1.min(time);
        let text: Vec<String> = self.best.iter().map(|((t, k), (s, tm))| format!("{t}\t{k}\t{s}\t{tm:.1}")).collect();
        game_common::save::store("snake", "scenarios-notes", &text.join("\n"));
    }

    fn set(&mut self, title: &str, step: usize) {
        if crate::levels::preview_active() {
            return;
        }
        self.next.insert(title.to_owned(), step);
        let text: Vec<String> = self.next.iter().map(|(t, s)| format!("{t}\t{s}")).collect();
        game_common::save::store("snake", "scenarios", &text.join("\n"));
    }
}

fn load_progress(mut p: ResMut<StoryProgress>) {
    // Images de presse : on ne montre pas la progression personnelle.
    if std::env::var("SNAKE_PRESSE").is_ok() {
        return;
    }
    if let Some(text) = game_common::save::load("snake", "scenarios") {
        p.next = text.lines().filter_map(|l| l.split_once('\t')).filter_map(|(t, s)| Some((t.to_owned(), s.parse().ok()?))).collect();
    }
    if let Some(text) = game_common::save::load("snake", "scenarios-notes") {
        for l in text.lines() {
            let f: Vec<&str> = l.split('\t').collect();
            if let [t, k, s, tm] = f[..]
                && let (Ok(k), Ok(s), Ok(tm)) = (k.parse::<usize>(), s.parse::<u8>(), tm.parse::<f32>())
            {
                p.best.insert((t.to_owned(), k), (s, tm));
            }
        }
    }
}

/// Passe à l'étape suivante de l'aventure en cours.
pub fn advance(mode: &mut Mode, progress: &mut StoryProgress, next: &mut NextState<GameState>) {
    let Mode::Scenario { scenario, step } = *mode else { return };
    // Aperçu d'une seule étape : on la rejoue au lieu de passer à la suivante.
    let step = if crate::levels::single_step() { step } else { step + 1 };
    if let Some(sc) = SCENARIOS.get(scenario) {
        progress.set(&sc.title, step);
    }
    *mode = Mode::Scenario { scenario, step };
    next.set(GameState::Step);
}

/// Étape de transition : on choisit l'écran de l'étape en cours (scène, niveau ou fin de l'aventure).
fn enter_step(mode: Res<Mode>, mut next: ResMut<NextState<GameState>>) {
    let Mode::Scenario { scenario, step } = *mode else {
        next.set(GameState::Menu);
        return;
    };
    debug!("aventure : étape {}", step + 1);
    match SCENARIOS.get(scenario).and_then(|s| s.steps.get(step)) {
        Some(l) if l.scene.is_some() => next.set(GameState::Scene),
        Some(_) => next.set(GameState::Playing),
        // Fin de l'aventure.
        None => next.set(if crate::levels::preview_active() { GameState::Menu } else { GameState::Scenarios }),
    }
}

/// Un niveau réussi dans une aventure : on enchaîne sans écran de fin.
fn after_win(mut mode: ResMut<Mode>, outcome: Res<Outcome>, mut progress: ResMut<StoryProgress>, mut next: ResMut<NextState<GameState>>, mut banner: ResMut<crate::game::Banner>) {
    if matches!(*mode, Mode::Scenario { .. }) && outcome.won {
        if crate::levels::single_step() {
            banner.text = "Réussi ! Aperçu d'une seule étape : le niveau recommence (décochez « cette étape seulement » pour enchaîner).".to_owned();
            banner.left = 5.0;
        }
        if let Mode::Scenario { scenario, step } = *mode
            && let Some(sc) = SCENARIOS.get(scenario)
            && !crate::levels::single_step()
        {
            progress.record(&sc.title, step, outcome.stars, outcome.time);
        }
        // Un niveau déjà gagné qu'on rejoue : la progression ne bouge pas, retour à la liste.
        if let Mode::Scenario { scenario, step } = *mode
            && !crate::levels::preview_active()
            && SCENARIOS.get(scenario).is_some_and(|sc| step + 1 < progress.next_step(&sc.title))
        {
            next.set(GameState::Scenarios);
            return;
        }
        advance(&mut mode, &mut progress, &mut next);
    }
}

pub struct StoryPlugin;

impl Plugin for StoryPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StoryProgress>()
            .add_systems(Startup, load_progress)
            .add_systems(OnEnter(GameState::Step), enter_step)
            .add_systems(OnEnter(GameState::GameOver), after_win.after(crate::voyage::conclude));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builtin_adventures_are_sound() {
        assert_eq!(SCENARIOS.len(), 1, "une seule aventure : le tutoriel puis dix lieux");
        for sc in SCENARIOS.iter() {
            let levels = sc.steps.iter().filter(|l| l.scene.is_none()).count();
            assert_eq!(levels, 7 + 10 * 6, "{} : le tutoriel (7) et six niveaux par lieu", sc.title);
            let mut names = std::collections::HashSet::new();
            assert!(sc.steps.iter().all(|s| names.insert(s.name.clone())), "{} : des étapes ont le même nom", sc.title);
            for step in &sc.steps {
                if let Some(g) = &step.grid {
                    let p = g.problems(step.tourists);
                    assert!(p.is_empty(), "{} / {} : {p:?}", sc.title, step.name);
                    // Départ valide, sortie voulue présente quand le niveau en a une.
                    assert!(step.layout(0).start_cells().iter().all(|c| step.layout(0).walkable(*c)), "{} : départ", step.name);
                } else if step.scene.is_none() {
                    panic!("{} / {} : niveau sans carte", sc.title, step.name);
                }
                if let Some(scene) = &step.scene {
                    assert!(!scene.lines.is_empty() && scene.actors.len() <= 5, "{} / {} : scène vide ou trop de personnages", sc.title, step.name);
                    assert!(scene.lines.iter().all(|l| l.text.chars().count() <= 200), "{} / {} : réplique trop longue", sc.title, step.name);
                }
            }
        }
    }
}
