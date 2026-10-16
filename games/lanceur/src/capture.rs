//! Mode capture : `LANCEUR_CAPTURE=dossier cargo run -p lanceur` enregistre
//! l'écran du lanceur et quitte (pour vérifier le rendu). Avec
//! `CC_OPEN_OPTIONS=1` ou `CC_OPEN_HELP=1`, la fenêtre correspondante est ouverte.
//!
//! `LANCEUR_INSTALLER=snake,tour` installe aussi ces jeux dès que le catalogue
//! est lu, capture le téléchargement en cours puis l'écran une fois les jeux
//! installés (essai de bout en bout, avec `CC_DEPOT` et `XDG_DATA_HOME`).
use bevy::prelude::*;
use bevy::render::view::screenshot::{Screenshot, save_to_disk};

use crate::catalogue::{JEUX, LANCEUR};
use crate::installation::{self, Catalogue, Partage};

#[derive(Resource)]
struct Capture {
    dossier: String,
    a_installer: Vec<&'static str>,
    etape: Etape,
    depuis: f32,
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Etape {
    Accueil,
    Lancement,
    Telechargement,
    Installe,
    Fin,
}

pub struct CapturePlugin;

impl Plugin for CapturePlugin {
    fn build(&self, app: &mut App) {
        if let Ok(dossier) = std::env::var("LANCEUR_CAPTURE") {
            let demandes = std::env::var("LANCEUR_INSTALLER").unwrap_or_default();
            let a_installer =
                JEUX.iter().map(|j| j.id).chain([LANCEUR]).filter(|id| demandes.split(',').any(|d| d.trim() == *id)).collect();
            app.insert_resource(Capture { dossier, a_installer, etape: Etape::Accueil, depuis: 0.0 }).add_systems(Update, capturer);
        }
    }
}

fn capturer(time: Res<Time>, mut commands: Commands, mut cap: ResMut<Capture>, partage: Res<Partage>, mut exit: MessageWriter<AppExit>) {
    let t = time.elapsed_secs();
    let dossier = cap.dossier.clone();
    let cliche = |commands: &mut Commands, nom: &str| {
        commands.spawn(Screenshot::primary_window()).observe(save_to_disk(format!("{dossier}/{nom}.png")));
    };
    let (pret, occupe) = {
        let etat = partage.lire();
        (matches!(etat.catalogue, Some(Catalogue::Pret(_) | Catalogue::HorsLigne)), cap.a_installer.iter().any(|id| etat.occupe(id)))
    };
    match cap.etape {
        Etape::Accueil if pret && t >= 4.0 => {
            cliche(&mut commands, "1-lanceur");
            cap.etape = if cap.a_installer.is_empty() { Etape::Fin } else { Etape::Lancement };
            cap.depuis = t;
        }
        Etape::Lancement if t - cap.depuis >= 0.5 => {
            for id in cap.a_installer.clone() {
                installation::installer(&partage, id);
            }
            cap.etape = Etape::Telechargement;
            cap.depuis = t;
        }
        Etape::Telechargement if t - cap.depuis >= 0.4 => {
            cliche(&mut commands, "2-telechargement");
            cap.etape = Etape::Installe;
            cap.depuis = t;
        }
        // On attend la fin des installations (deux minutes au plus).
        Etape::Installe if (!occupe && t - cap.depuis >= 1.0) || t - cap.depuis > 120.0 => {
            cliche(&mut commands, "3-installe");
            cap.etape = Etape::Fin;
            cap.depuis = t;
        }
        Etape::Fin if t - cap.depuis >= 1.0 => {
            exit.write(AppExit::Success);
        }
        _ => {}
    }
}
