//! Installation des jeux sur ce PC, sans droits administrateur, au même
//! endroit que `tools/install-linux.sh` : `~/.local/share/commit-and-conquer`.
//!
//! Les archives viennent des « Releases » GitHub du studio
//! (`.github/workflows/versions.yml`) ; `catalogue.json` y donne la version,
//! la taille et l'empreinte de chacune. `CC_DEPOT=http://…` lit une autre
//! adresse (pour essayer avec des archives fabriquées sur le PC).
//!
//! Le téléchargement se fait dans un fil à part : l'interface lit l'état
//! partagé (`Partage`) à chaque image.
use std::collections::HashMap;
use std::fs;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::{Arc, Mutex, MutexGuard};

use bevy::prelude::*;
use game_common::langue::Texte;
use sha2::{Digest, Sha256};

use crate::catalogue::{self, JEUX, LANCEUR};

/// Les archives de la dernière version publiée.
const DEPOT: &str = "https://github.com/commitandconquer-studio/jeux/releases/latest/download";

fn depot() -> String {
    std::env::var("CC_DEPOT").unwrap_or_else(|_| DEPOT.into()).trim_end_matches('/').to_string()
}

fn donnees() -> PathBuf {
    std::env::var_os("XDG_DATA_HOME")
        .map(PathBuf::from)
        .filter(|p| p.is_absolute())
        .unwrap_or_else(|| PathBuf::from(std::env::var_os("HOME").unwrap_or_default()).join(".local/share"))
}

/// Où vivent les jeux installés.
pub fn dossier() -> PathBuf {
    donnees().join("commit-and-conquer")
}

/// Une archive publiée.
#[derive(Clone, Debug)]
pub struct Distant {
    pub version: String,
    pub archive: String,
    pub taille: u64,
    pub sha256: String,
}

#[derive(Clone, Debug)]
pub enum Catalogue {
    Recherche,
    Pret(HashMap<String, Distant>),
    HorsLigne,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Echec {
    Reseau,
    Archive,
    Disque,
}

impl Echec {
    pub fn texte(self) -> Texte {
        match self {
            Echec::Reseau => game_common::tx!(
                "Téléchargement impossible",
                "Download failed",
                "Download fehlgeschlagen",
                "Download non riuscito",
                "Descarga fallida",
            ),
            Echec::Archive => game_common::tx!("Archive abîmée", "Damaged archive", "Beschädigtes Archiv", "Archivio danneggiato", "Archivo dañado"),
            Echec::Disque => game_common::tx!(
                "Installation impossible",
                "Installation failed",
                "Installation fehlgeschlagen",
                "Installazione non riuscita",
                "Instalación fallida",
            ),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Tache {
    /// Part téléchargée, de 0 à 1.
    Telechargement(f32),
    Installation,
    Echec(Echec),
}

#[derive(Default)]
pub struct Etat {
    pub catalogue: Option<Catalogue>,
    /// Version installée de chaque jeu (`?` si installé sans version, par
    /// exemple avec `tools/install-linux.sh`).
    pub installes: HashMap<String, String>,
    pub taches: HashMap<String, Tache>,
    /// Le lanceur vient d'être mis à jour : il faut le relancer.
    pub relancer: bool,
}

impl Etat {
    pub fn distant(&self, id: &str) -> Option<&Distant> {
        match &self.catalogue {
            Some(Catalogue::Pret(c)) => c.get(id),
            _ => None,
        }
    }

    /// Une version plus récente que celle installée est publiée.
    pub fn mise_a_jour(&self, id: &str) -> Option<&Distant> {
        let installee = self.installes.get(id)?;
        self.distant(id).filter(|d| plus_recente(&d.version, installee))
    }

    pub fn occupe(&self, id: &str) -> bool {
        matches!(self.taches.get(id), Some(Tache::Telechargement(_) | Tache::Installation))
    }
}

/// L'état partagé entre l'interface et les fils de téléchargement.
#[derive(Resource, Clone, Default)]
pub struct Partage(Arc<Mutex<Etat>>);

impl Partage {
    pub fn lire(&self) -> MutexGuard<'_, Etat> {
        self.0.lock().unwrap_or_else(|e| e.into_inner())
    }
}

/// `a` est-elle plus récente que `b` ? (« 0.10.0 » > « 0.9.2 » ; une version
/// inconnue « ? » est plus ancienne que tout.)
pub fn plus_recente(a: &str, b: &str) -> bool {
    let nombres = |v: &str| -> Option<Vec<u64>> { v.split(['.', '-']).take(3).map(|n| n.parse().ok()).collect() };
    match (nombres(a), nombres(b)) {
        (Some(a), Some(b)) => a > b,
        (Some(_), None) => true,
        _ => false,
    }
}

fn version_installee(dossier: &Path, id: &str) -> Option<String> {
    if !dossier.join(id).is_file() {
        return None;
    }
    let v = fs::read_to_string(dossier.join(format!("{id}.version"))).unwrap_or_default();
    let v = v.trim();
    Some(if v.is_empty() { "?".into() } else { v.into() })
}

/// Relit les jeux installés.
pub fn relire_installes(partage: &Partage) {
    let d = dossier();
    let installes = JEUX
        .iter()
        .map(|j| j.id)
        .chain([LANCEUR])
        .filter_map(|id| version_installee(&d, id).map(|v| (id.to_string(), v)))
        .collect();
    partage.lire().installes = installes;
}

/// Télécharge le catalogue de la dernière version (dans un fil à part).
pub fn chercher_catalogue(partage: &Partage) {
    partage.lire().catalogue = Some(Catalogue::Recherche);
    let partage = partage.clone();
    std::thread::spawn(move || {
        let catalogue = match lire_catalogue() {
            Ok(c) => Catalogue::Pret(c),
            Err(e) => {
                warn!("Catalogue indisponible : {e}");
                Catalogue::HorsLigne
            }
        };
        partage.lire().catalogue = Some(catalogue);
    });
}

fn lire_catalogue() -> Result<HashMap<String, Distant>, String> {
    let texte = ureq::get(format!("{}/catalogue.json", depot()))
        .call()
        .map_err(|e| e.to_string())?
        .into_body()
        .read_to_string()
        .map_err(|e| e.to_string())?;
    let json: serde_json::Value = serde_json::from_str(&texte).map_err(|e| e.to_string())?;
    let version = json["version"].as_str().ok_or("version absente")?.to_string();
    let jeux = json["jeux"].as_object().ok_or("liste des jeux absente")?;
    Ok(jeux
        .iter()
        .filter_map(|(id, j)| {
            Some((
                id.clone(),
                Distant {
                    version: version.clone(),
                    archive: j["archive"].as_str()?.to_string(),
                    taille: j["taille"].as_u64()?,
                    sha256: j["sha256"].as_str()?.to_lowercase(),
                },
            ))
        })
        .collect())
}

/// Installe ou met à jour un jeu (dans un fil à part).
pub fn installer(partage: &Partage, id: &'static str) {
    let distant = {
        let mut etat = partage.lire();
        if etat.occupe(id) {
            return;
        }
        let Some(d) = etat.distant(id).cloned() else {
            etat.taches.insert(id.into(), Tache::Echec(Echec::Reseau));
            return;
        };
        etat.taches.insert(id.into(), Tache::Telechargement(0.0));
        d
    };
    let partage = partage.clone();
    std::thread::spawn(move || {
        let fin = telecharger_et_installer(&partage, id, &distant);
        relire_installes(&partage);
        let mut etat = partage.lire();
        match fin {
            Ok(()) => {
                etat.taches.remove(id);
                info!("{id} {} installé", distant.version);
                if id == LANCEUR {
                    etat.relancer = true;
                }
            }
            Err(echec) => {
                etat.taches.insert(id.into(), Tache::Echec(echec));
            }
        }
    });
}

fn telecharger_et_installer(partage: &Partage, id: &str, distant: &Distant) -> Result<(), Echec> {
    let dossier = dossier();
    let attente = dossier.join(".telechargement");
    let archive = attente.join(&distant.archive);
    let deballe = attente.join(id);
    fs::create_dir_all(&attente).map_err(disque)?;

    // 1. Téléchargement, avec la progression et l'empreinte au fil de l'eau.
    let reponse = ureq::get(format!("{}/{}", depot(), distant.archive)).call().map_err(|e| {
        warn!("{id} : {e}");
        Echec::Reseau
    })?;
    let mut lecteur = reponse.into_body().into_reader();
    let mut fichier = fs::File::create(&archive).map_err(disque)?;
    let mut empreinte = Sha256::new();
    let mut recu = 0u64;
    let mut morceau = vec![0u8; 64 * 1024];
    loop {
        let n = lecteur.read(&mut morceau).map_err(|e| {
            warn!("{id} : {e}");
            Echec::Reseau
        })?;
        if n == 0 {
            break;
        }
        fichier.write_all(&morceau[..n]).map_err(disque)?;
        empreinte.update(&morceau[..n]);
        recu += n as u64;
        let part = (recu as f32 / distant.taille.max(1) as f32).min(1.0);
        partage.lire().taches.insert(id.into(), Tache::Telechargement(part));
    }
    drop(fichier);
    let calculee: String = empreinte.finalize().iter().map(|o| format!("{o:02x}")).collect();
    if recu != distant.taille || calculee != distant.sha256 {
        warn!("{id} : archive inattendue ({recu} octets, empreinte {calculee})");
        let _ = fs::remove_file(&archive);
        return Err(Echec::Archive);
    }

    // 2. Décompression à part, puis mise en place : un jeu à moitié copié ne
    // remplace jamais celui qui marche.
    partage.lire().taches.insert(id.into(), Tache::Installation);
    let _ = fs::remove_dir_all(&deballe);
    fs::create_dir_all(&deballe).map_err(disque)?;
    let gz = flate2::read::GzDecoder::new(fs::File::open(&archive).map_err(disque)?);
    // `unpack` refuse les chemins qui sortiraient du dossier (« ../ »).
    tar::Archive::new(gz).unpack(&deballe).map_err(|e| {
        warn!("{id} : {e}");
        Echec::Archive
    })?;
    if !deballe.join(id).is_file() {
        return Err(Echec::Archive);
    }
    deplacer(&deballe, &dossier).map_err(disque)?;
    let _ = fs::remove_dir_all(&deballe);
    let _ = fs::remove_file(&archive);

    if let Some(jeu) = JEUX.iter().find(|j| j.id == id) {
        entree_de_menu(&dossier, id, &jeu.nom, &jeu.description).map_err(disque)?;
    } else if id == LANCEUR {
        entree_de_menu(&dossier, LANCEUR, &catalogue::NOM_LANCEUR, &catalogue::DESCRIPTION_LANCEUR).map_err(disque)?;
    }
    Ok(())
}

fn disque(e: io::Error) -> Echec {
    warn!("Disque : {e}");
    Echec::Disque
}

/// Déplace le contenu de `de` dans `vers`, en remplaçant les fichiers (un
/// renommage remplace même l'exécutable d'un jeu ouvert).
fn deplacer(de: &Path, vers: &Path) -> io::Result<()> {
    fs::create_dir_all(vers)?;
    for entree in fs::read_dir(de)? {
        let entree = entree?;
        let cible = vers.join(entree.file_name());
        if entree.file_type()?.is_dir() {
            deplacer(&entree.path(), &cible)?;
        } else {
            fs::rename(entree.path(), &cible)?;
        }
    }
    Ok(())
}

/// Copie le contenu de `de` dans `vers` (le dossier d'origine reste).
fn copier(de: &Path, vers: &Path) -> io::Result<()> {
    fs::create_dir_all(vers)?;
    for entree in fs::read_dir(de)? {
        let entree = entree?;
        let cible = vers.join(entree.file_name());
        if entree.file_type()?.is_dir() {
            copier(&entree.path(), &cible)?;
        } else {
            remplacer_par_copie(&entree.path(), &cible)?;
        }
    }
    Ok(())
}

/// Copie à côté puis renomme : marche aussi sur un exécutable en cours.
fn remplacer_par_copie(de: &Path, vers: &Path) -> io::Result<()> {
    let provisoire = vers.with_extension("nouveau");
    fs::copy(de, &provisoire)?;
    fs::rename(&provisoire, vers)
}

fn fichier_desktop(id: &str) -> PathBuf {
    let nom = if id == LANCEUR { "commit-and-conquer.desktop".to_string() } else { format!("commit-and-conquer-{id}.desktop") };
    donnees().join("applications").join(nom)
}

/// L'entrée du menu d'applications (catégorie Jeux), dans les cinq langues.
fn entree_de_menu(dossier: &Path, id: &str, nom: &Texte, description: &Texte) -> io::Result<()> {
    let chemin = fichier_desktop(id);
    fs::create_dir_all(chemin.parent().unwrap_or(dossier))?;
    let d = dossier.display();
    let studio = game_common::tx!(
        "un jeu Commit & Conquer",
        "a Commit & Conquer game",
        "ein Spiel von Commit & Conquer",
        "un gioco Commit & Conquer",
        "un juego de Commit & Conquer",
    );
    let langues = [("en", nom.en, description.en, studio.en), ("de", nom.de, description.de, studio.de), ("it", nom.it, description.it, studio.it), ("es", nom.es, description.es, studio.es)];
    let comment = |desc: &str, studio: &str| if id == LANCEUR { desc.trim_end_matches('.').to_string() } else { format!("{} — {studio}", desc.trim_end_matches('.')) };
    let mut texte = format!(
        "[Desktop Entry]\nType=Application\nName={}\nComment={}\n",
        nom.fr,
        comment(description.fr, studio.fr)
    );
    for (code, n, desc, s) in langues {
        texte += &format!("Name[{code}]={n}\nComment[{code}]={}\n", comment(desc, s));
    }
    texte += &format!("Exec=\"{d}/{id}\"\nPath={d}\nIcon={d}/{id}.png\nTerminal=false\nCategories=Game;\n");
    fs::write(&chemin, texte)?;
    let _ = Command::new("update-desktop-database").arg(chemin.parent().unwrap_or(dossier)).output();
    Ok(())
}

/// Lance un jeu installé, dans son dossier (il y trouve `assets/`).
pub fn lancer(id: &str) -> bool {
    let d = dossier();
    match Command::new(d.join(id)).current_dir(&d).spawn() {
        Ok(_) => true,
        Err(e) => {
            warn!("Lancement de {id} impossible : {e}");
            false
        }
    }
}

/// Premier lancement depuis l'archive décompressée (dans Téléchargements…) :
/// le lanceur s'installe dans le dossier des jeux et dans le menu
/// d'applications. Déjà installé : il rafraîchit seulement son entrée.
pub fn installer_le_lanceur() {
    if std::env::var_os("CARGO_MANIFEST_DIR").is_some() {
        return; // `cargo run` : on développe, on n'installe rien.
    }
    let Some(ici) = std::env::current_exe().ok().and_then(|e| e.parent().map(Path::to_path_buf)) else {
        return;
    };
    let dest = dossier();
    let deja_installe = fs::canonicalize(&ici).ok().is_some_and(|i| fs::canonicalize(&dest).ok() == Some(i));
    if !deja_installe {
        if !ici.join(format!("{LANCEUR}.version")).is_file() {
            return;
        }
        let copie = || -> io::Result<()> {
            fs::create_dir_all(&dest)?;
            for f in [LANCEUR.to_string(), format!("{LANCEUR}.png"), format!("{LANCEUR}.version")] {
                remplacer_par_copie(&ici.join(&f), &dest.join(&f))?;
            }
            copier(&ici.join("assets"), &dest.join("assets"))
        };
        if let Err(e) = copie() {
            warn!("Installation du lanceur impossible : {e}");
            return;
        }
        info!("Lanceur installé dans {}", dest.display());
    }
    if let Err(e) = entree_de_menu(&dest, LANCEUR, &catalogue::NOM_LANCEUR, &catalogue::DESCRIPTION_LANCEUR) {
        warn!("Entrée de menu du lanceur impossible : {e}");
    }
}

/// Après sa propre mise à jour : ouvre la nouvelle version.
pub fn relancer_le_lanceur() {
    let d = dossier();
    if let Err(e) = Command::new(d.join(LANCEUR)).current_dir(&d).spawn() {
        warn!("Relance du lanceur impossible : {e}");
    }
}
