//! Sauvegarde très simple (progression, records) : une valeur texte par clé,
//! dans `~/.local/share/commit-and-conquer/save/<jeu>-<clé>.txt`.
//! En cas de problème (dossier absent, plateforme sans disque…), on ignore.
use std::path::PathBuf;

fn path(game: &str, key: &str) -> Option<PathBuf> {
    let data = std::env::var_os("XDG_DATA_HOME")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".local/share")))
        .or_else(|| std::env::var_os("APPDATA").map(PathBuf::from))?;
    Some(data.join("commit-and-conquer/save").join(format!("{game}-{key}.txt")))
}

/// Lit une valeur sauvegardée.
pub fn load(game: &str, key: &str) -> Option<String> {
    std::fs::read_to_string(path(game, key)?).ok().map(|s| s.trim().to_string())
}

/// Enregistre une valeur (silencieusement ignoré en cas d'erreur).
pub fn store(game: &str, key: &str, value: &str) {
    if let Some(p) = path(game, key) {
        if let Some(dir) = p.parent() {
            let _ = std::fs::create_dir_all(dir);
        }
        let _ = std::fs::write(p, value);
    }
}
