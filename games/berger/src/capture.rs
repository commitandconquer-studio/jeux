//! Modes « captures d'écran » (puis le jeu quitte) :
//! - `BERGER_CAPTURE=dossier cargo run -p berger` : le menu, les écrans puis des parties
//!   jouées par le chien automatique (pour vérifier le rendu sans y jouer).
//! - `BERGER_PRESSE=dossier cargo run -p berger` : dossier de presse en 1920×1080, sans
//!   interface en jeu, tout débloqué. Rien n'est sauvegardé.
//! - `BERGER_KIT=dossier cargo run -p berger` : chaque chien, mouton et loup seul sur un fond
//!   magenta uni (`tools/berger-kit.py` en tire des PNG transparents).
use bevy::camera::ClearColorConfig;
use bevy::prelude::*;
use bevy::render::view::screenshot::{Screenshot, save_to_disk};
use bevy::window::PrimaryWindow;
use berger_core::{GenreLoup, TypeMouton};
use game_common::{ISO_OFFSET, iso_projection};

use crate::GameState;
use crate::chiens::ChienVu;
use crate::jeu::{Commencer, Genre, MainCamera};
use crate::vue::{Kit, spawn_belier, spawn_chien, spawn_loup, spawn_mouton};

pub fn pour_la_presse() -> bool {
    std::env::var_os("BERGER_PRESSE").is_some()
}

pub fn mode_kit() -> bool {
    std::env::var_os("BERGER_KIT").is_some()
}

/// Les captures n'écrasent pas la sauvegarde du joueur.
pub fn sans_sauvegarde() -> bool {
    pour_la_presse() || mode_kit() || std::env::var_os("BERGER_CAPTURE").is_some()
}

/// Pas de bandeau de jeu sur les images de presse ni sur le kit.
pub fn sans_interface() -> bool {
    pour_la_presse() || mode_kit()
}

struct Cliche {
    nom: &'static str,
    etat: GameState,
    delai: f32,
    genre: Option<Genre>,
    /// Le chien aboie juste avant la photo (on voit l'onde).
    aboie: bool,
    /// Chien montré à l'écran « Mon chien ».
    chien: usize,
}

const fn c(nom: &'static str, etat: GameState, delai: f32, genre: Option<Genre>) -> Cliche {
    Cliche { nom, etat, delai, genre, aboie: false, chien: 0 }
}

fn cliches(presse: bool) -> Vec<Cliche> {
    let aboie = |mut x: Cliche| {
        x.aboie = true;
        x
    };
    if presse {
        vec![
            c("01-menu", GameState::Menu, 3.0, None),
            aboie(c("02-premier-pre", GameState::Playing, 14.0, Some(Genre::Campagne(0)))),
            c("03-obstacles", GameState::Playing, 25.0, Some(Genre::Campagne(3))),
            c("04-loups", GameState::Playing, 45.0, Some(Genre::Campagne(7))),
            c("05-desert", GameState::Playing, 30.0, Some(Genre::Campagne(14))),
            c("06-grand-pre", GameState::Playing, 30.0, Some(Genre::Campagne(11))),
            c("07-niveaux", GameState::Niveaux, 1.5, None),
            Cliche { chien: 3, ..c("08-mon-chien", GameState::Chiens, 1.5, None) },
        ]
    } else {
        vec![
            c("1-menu", GameState::Menu, 3.0, None),
            c("2-niveaux", GameState::Niveaux, 1.5, None),
            Cliche { chien: 3, ..c("2b-chiens", GameState::Chiens, 1.5, None) },
            aboie(c("3-premier-pre", GameState::Playing, 14.0, Some(Genre::Campagne(0)))),
            c("4-obstacles", GameState::Playing, 25.0, Some(Genre::Campagne(3))),
            c("5-loups", GameState::Playing, 45.0, Some(Genre::Campagne(7))),
            c("6-desert", GameState::Playing, 30.0, Some(Genre::Campagne(14))),
            c("7-grand-pre", GameState::Playing, 30.0, Some(Genre::Campagne(11))),
            // Le chien automatique finit le niveau : la photo est prise à l'écran de fin.
            c("8-fin", GameState::Fin, 1.0, Some(Genre::Campagne(0))),
        ]
    }
}

#[derive(Resource)]
struct Capture {
    dossier: String,
    cliches: Vec<Cliche>,
    i: usize,
    depuis: f32,
    lance: bool,
    pris: bool,
}

pub struct CapturePlugin;

impl Plugin for CapturePlugin {
    fn build(&self, app: &mut App) {
        let presse = std::env::var("BERGER_PRESSE").ok();
        if mode_kit() {
            let dossier = std::env::var("BERGER_KIT").unwrap_or_default();
            app.insert_resource(PriseKit { dossier, i: 0, depuis: 0.0, etat: EtatKit::Prepare, scene: None })
                .add_systems(Update, kit)
                .add_systems(Startup, (sans_synchro, plein_ecran));
        } else if let Some(dossier) = presse.clone().or_else(|| std::env::var("BERGER_CAPTURE").ok()) {
            app.insert_resource(Capture { dossier, cliches: cliches(presse.is_some()), i: 0, depuis: 0.0, lance: false, pris: false })
                .add_systems(Update, capturer)
                .add_systems(Startup, sans_synchro);
            if presse.is_some() {
                app.add_systems(Startup, plein_ecran);
            }
        }
    }
}

/// Fenêtre en 1920×1080 exactement (échelle 1), pour des images de presse.
fn plein_ecran(mut windows: Query<&mut Window, With<PrimaryWindow>>) {
    for mut w in &mut windows {
        // Sans barre de titre : la zone de rendu fait bien 1920×1080.
        w.decorations = false;
        w.resizable = false;
        w.resolution.set_scale_factor_override(Some(1.0));
        w.resolution.set(1920.0, 1080.0);
    }
}

/// Fenêtre cachée ou masquée : sans synchro verticale, le rendu ne se fige pas.
fn sans_synchro(mut windows: Query<&mut Window, With<PrimaryWindow>>) {
    for mut w in &mut windows {
        w.present_mode = bevy::window::PresentMode::AutoNoVsync;
    }
}

#[allow(clippy::too_many_arguments)]
fn capturer(
    time: Res<Time>,
    mut commands: Commands,
    mut cap: ResMut<Capture>,
    state: Res<State<GameState>>,
    mut next: ResMut<NextState<GameState>>,
    mut commencer: MessageWriter<Commencer>,
    mut exit: MessageWriter<AppExit>,
    partie: Option<ResMut<crate::jeu::Partie>>,
    mut vu: ResMut<ChienVu>,
) {
    let t = time.elapsed_secs();
    let Some(cl) = cap.cliches.get(cap.i) else {
        exit.write(AppExit::Success);
        return;
    };
    let (nom, etat, delai, genre, aboie, chien) = (cl.nom, cl.etat, cl.delai, cl.genre, cl.aboie, cl.chien);
    if !cap.lance {
        cap.lance = true;
        cap.depuis = t;
        next.set(etat);
        if etat == GameState::Chiens {
            vu.0 = chien;
        }
        if let Some(genre) = genre {
            // Le chien automatique joue : on voit le troupeau bouger.
            commencer.write(Commencer { genre, demo: false, robot: true });
        }
        return;
    }
    if *state.get() != etat {
        return;
    }
    // Le chien aboie juste avant la photo : on voit l'onde.
    if !cap.pris && aboie && t - cap.depuis >= delai - 0.2 {
        if let Some(mut p) = partie {
            p.bouton_aboyer = true;
        }
    }
    if !cap.pris && t - cap.depuis >= delai {
        cap.pris = true;
        let chemin = format!("{}/{nom}.png", cap.dossier);
        commands.spawn(Screenshot::primary_window()).observe(save_to_disk(chemin));
    } else if cap.pris && t - cap.depuis >= delai + 0.5 {
        cap.i += 1;
        cap.pris = false;
        cap.lance = false;
    }
}

// --- Le kit : chaque animal seul, sur fond magenta -------------------------------------

#[derive(Clone, Copy)]
enum Sujet {
    Chien(usize),
    Mouton(TypeMouton),
    Belier,
    Loup(GenreLoup),
}

const SUJETS: [(&str, Sujet); 13] = [
    ("chien-border", Sujet::Chien(0)),
    ("chien-caramel", Sujet::Chien(1)),
    ("chien-neige", Sujet::Chien(2)),
    ("chien-ardoise", Sujet::Chien(3)),
    ("chien-braise", Sujet::Chien(4)),
    ("chien-dore", Sujet::Chien(5)),
    ("mouton-blanc", Sujet::Mouton(TypeMouton::Blanc)),
    ("mouton-noir", Sujet::Mouton(TypeMouton::Noir)),
    ("agneau", Sujet::Mouton(TypeMouton::Agneau)),
    ("belier", Sujet::Belier),
    ("loup-solitaire", Sujet::Loup(GenreLoup::Solitaire)),
    ("loup-prudent", Sujet::Loup(GenreLoup::Prudent)),
    ("loup-audacieux", Sujet::Loup(GenreLoup::Audacieux)),
];

#[derive(PartialEq)]
enum EtatKit {
    Prepare,
    Pose,
    Photo,
}

#[derive(Resource)]
struct PriseKit {
    dossier: String,
    i: usize,
    depuis: f32,
    etat: EtatKit,
    scene: Option<Entity>,
}

#[allow(clippy::too_many_arguments)]
fn kit(
    time: Res<Time>,
    mut commands: Commands,
    mut prise: ResMut<PriseKit>,
    state: Res<State<GameState>>,
    mut next: ResMut<NextState<GameState>>,
    kit: Res<Kit>,
    mut vu: ResMut<ChienVu>,
    mut cam: Query<(&mut Camera, &mut Transform, &mut Projection), With<MainCamera>>,
    mut exit: MessageWriter<AppExit>,
) {
    let t = time.elapsed_secs();
    if prise.etat == EtatKit::Prepare {
        // Une caméra serrée sur le sujet, fond magenta uni (c'est elle qui fixe la couleur de fond).
        let Ok((mut camera, mut tr, mut proj)) = cam.single_mut() else { return };
        camera.clear_color = ClearColorConfig::Custom(Color::srgb(1.0, 0.0, 1.0));
        let cible = Vec3::new(0.0, 0.6, 0.0);
        *tr = Transform::from_translation(cible + ISO_OFFSET).looking_at(cible, Vec3::Y);
        *proj = iso_projection(3.4, 2.6);
        next.set(GameState::Playing);
        prise.etat = EtatKit::Pose;
        prise.depuis = t;
        return;
    }
    // L'écran de démarrage du studio passe d'abord.
    if *state.get() != GameState::Playing || t < 10.0 {
        return;
    }
    let Some(&(nom, sujet)) = SUJETS.get(prise.i) else {
        if t - prise.depuis > 3.0 {
            exit.write(AppExit::Success);
        }
        return;
    };
    match prise.etat {
        EtatKit::Pose if t - prise.depuis > 0.8 => {
            // Le sujet précédent part une fois sa photo prise.
            if let Some(e) = prise.scene.take() {
                commands.entity(e).despawn();
            }
            if let Sujet::Chien(i) = sujet {
                vu.0 = i;
            }
            let racine = match sujet {
                Sujet::Chien(_) => spawn_chien(&mut commands, &kit),
                Sujet::Mouton(genre) => spawn_mouton(&mut commands, &kit, 0, genre),
                Sujet::Belier => spawn_belier(&mut commands, &kit, 0),
                Sujet::Loup(genre) => spawn_loup(&mut commands, &kit, 0, genre),
            };
            // De trois quarts face à la caméra.
            let lacet = ISO_OFFSET.x.atan2(ISO_OFFSET.z) + 0.5;
            commands.entity(racine).entry::<Transform>().and_modify(move |mut tr| tr.rotation = Quat::from_rotation_y(lacet));
            prise.scene = Some(racine);
            prise.etat = EtatKit::Photo;
            prise.depuis = t;
        }
        EtatKit::Photo if t - prise.depuis > 1.0 => {
            commands.spawn(Screenshot::primary_window()).observe(save_to_disk(format!("{}/{nom}.png", prise.dossier)));
            prise.etat = EtatKit::Pose;
            prise.depuis = t;
            prise.i += 1;
        }
        _ => {}
    }
}
