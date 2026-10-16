//! Sauvegarde très simple (progression, records) : une valeur texte par clé,
//! dans `~/.local/share/commit-and-conquer/save/<jeu>-<clé>.txt`.
//! Dans le navigateur : le stockage local, clé `cc-save-<jeu>-<clé>`.
//! En cas de problème (dossier absent, plateforme sans disque…), on ignore.
#[cfg(not(target_arch = "wasm32"))]
use std::path::PathBuf;

#[cfg(target_arch = "wasm32")]
fn stockage() -> Option<web_sys::Storage> {
    web_sys::window()?.local_storage().ok()?
}

/// Lit une valeur sauvegardée.
#[cfg(target_arch = "wasm32")]
pub fn load(game: &str, key: &str) -> Option<String> {
    stockage()?.get_item(&format!("cc-save-{game}-{key}")).ok()?
}

/// Enregistre une valeur (silencieusement ignoré en cas d'erreur).
#[cfg(target_arch = "wasm32")]
pub fn store(game: &str, key: &str, value: &str) {
    if let Some(s) = stockage() {
        let _ = s.set_item(&format!("cc-save-{game}-{key}"), value);
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn path(game: &str, key: &str) -> Option<PathBuf> {
    let data = std::env::var_os("XDG_DATA_HOME")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".local/share")))
        .or_else(|| std::env::var_os("APPDATA").map(PathBuf::from))?;
    Some(data.join("commit-and-conquer/save").join(format!("{game}-{key}.txt")))
}

/// Lit une valeur sauvegardée.
#[cfg(not(target_arch = "wasm32"))]
pub fn load(game: &str, key: &str) -> Option<String> {
    std::fs::read_to_string(path(game, key)?).ok().map(|s| s.trim().to_string())
}

/// Enregistre une valeur (silencieusement ignoré en cas d'erreur).
#[cfg(not(target_arch = "wasm32"))]
pub fn store(game: &str, key: &str, value: &str) {
    if let Some(p) = path(game, key) {
        if let Some(dir) = p.parent() {
            let _ = std::fs::create_dir_all(dir);
        }
        let _ = std::fs::write(p, value);
    }
}
