//! Le panneau « Ambiance » des menus, commun à tous les jeux : le joueur
//! active ou non l'ambiance, le moment de la journée, la saison (ou en choisit
//! une), la fête du moment et la météo locale.
//!
//! La météo locale, sur PC : le joueur saisit sa ville, que l'on envoie au
//! service libre Open-Meteo (avec l'outil `curl`, présent sur Linux, macOS et
//! Windows 10+). Dans le navigateur, c'est le choix fait sur le site (bouton
//! nuage) qui compte.
use std::sync::{Arc, Mutex};

use bevy::input::InputSystems;
use bevy::input::keyboard::{Key, KeyboardInput};
use bevy::prelude::*;

use crate::ambiance::{Ambiance, ChoixSaison, Meteo, Preferences, Saison, web};
use crate::ui::{spawn_button, spawn_button_sized, spawn_gap, spawn_panel, spawn_text};

/// Bouton qui ouvre le panneau « Ambiance » (à mettre dans les menus).
#[derive(Component)]
pub struct AmbianceBouton;

/// Bouton « Ambiance » de taille standard.
pub fn spawn_ambiance_button(commands: &mut Commands, parent: Entity) -> Entity {
    let (b, _) = spawn_button(commands, parent, tx!("Ambiance", "Atmosphere", "Stimmung", "Atmosfera", "Ambiente").t());
    commands.entity(b).insert(AmbianceBouton);
    b
}

#[derive(Component)]
struct Panneau;

#[derive(Component, Clone, Copy)]
enum Choix {
    Active,
    Moment,
    Saison,
    Fete,
    Meteo,
    Ville,
    Fermer,
}

#[derive(Component)]
struct TexteVille;

#[derive(Component)]
struct TexteEtat;

/// Saisie de la ville en cours, et message d'état sous le panneau.
#[derive(Resource, Default)]
pub(crate) struct Saisie {
    active: bool,
    texte: String,
    etat: String,
}

/// Réponse d'une recherche météo lancée en arrière-plan (ville, temps).
#[derive(Resource, Default, Clone)]
pub struct MeteoRecue(Arc<Mutex<Option<Result<(String, Meteo), String>>>>);

pub(crate) struct PanneauPlugin;

impl Plugin for PanneauPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Saisie>()
            .init_resource::<MeteoRecue>()
            .add_observer(au_clic)
            .add_systems(PreUpdate, saisir.after(InputSystems))
            .add_systems(Update, (reponse_meteo, rafraichir).chain())
            .add_systems(Startup, meteo_au_lancement);
        if std::env::var_os("CC_OPEN_AMBIANCE").is_some() {
            app.add_systems(Update, ouvrir_pour_test);
        }
    }
}

/// Test : `CC_OPEN_AMBIANCE=1` ouvre le panneau au bout de 2 secondes (pour les captures).
fn ouvrir_pour_test(time: Res<Time>, mut fait: Local<bool>, mut commands: Commands, prefs: Res<Preferences>, saisie: Res<Saisie>) {
    if !*fait && time.elapsed_secs() > 2.0 {
        *fait = true;
        ouvrir(&mut commands, &prefs, &saisie);
    }
}

fn oui(v: bool, masculin: bool) -> &'static str {
    match (v, masculin) {
        (true, true) => tx!("activé", "on", "an", "attivo", "activado"),
        (false, true) => tx!("désactivé", "off", "aus", "disattivato", "desactivado"),
        (true, false) => tx!("activée", "on", "an", "attiva", "activada"),
        (false, false) => tx!("désactivée", "off", "aus", "disattivata", "desactivada"),
    }
    .t()
}

fn nom_saison(s: Saison) -> &'static str {
    match s {
        Saison::Printemps => tx!("printemps", "spring", "Frühling", "primavera", "primavera"),
        Saison::Ete => tx!("été", "summer", "Sommer", "estate", "verano"),
        Saison::Automne => tx!("automne", "autumn", "Herbst", "autunno", "otoño"),
        Saison::Hiver => tx!("hiver", "winter", "Winter", "inverno", "invierno"),
    }
    .t()
}

fn nom_meteo(m: Meteo) -> &'static str {
    match m {
        Meteo::Clair => tx!("ciel dégagé", "clear sky", "klarer Himmel", "cielo sereno", "cielo despejado"),
        Meteo::Nuageux => tx!("nuageux", "cloudy", "bewölkt", "nuvoloso", "nublado"),
        Meteo::Pluie => tx!("pluie", "rain", "Regen", "pioggia", "lluvia"),
        Meteo::Neige => tx!("neige", "snow", "Schnee", "neve", "nieve"),
        Meteo::Orage => tx!("orage", "thunderstorm", "Gewitter", "temporale", "tormenta"),
        Meteo::Brouillard => tx!("brouillard", "fog", "Nebel", "nebbia", "niebla"),
    }
    .t()
}

/// Ouvre le panneau depuis les Options (bouton « Ambiance… »).
pub(crate) fn ouvrir_depuis_options(mut commands: Commands, prefs: Res<Preferences>, mut saisie: ResMut<Saisie>) {
    saisie.etat.clear();
    ouvrir(&mut commands, &prefs, &saisie);
}

fn ouvrir(commands: &mut Commands, p: &Preferences, saisie: &Saisie) {
    let panel = spawn_panel(commands, (Panneau, BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.35)), GlobalZIndex(50)));
    commands.entity(panel).insert(crate::ui::PanneauOpaque);
    commands.entity(panel).insert(Node {
        flex_direction: FlexDirection::Column,
        align_items: AlignItems::Center,
        row_gap: Val::Px(8.0),
        padding: UiRect::axes(Val::Px(32.0), Val::Px(22.0)),
        border: UiRect::all(Val::Px(2.0)),
        border_radius: BorderRadius::all(Val::Px(24.0)),
        max_width: Val::Px(520.0),
        ..default()
    });
    spawn_text(commands, panel, tx!("Ambiance", "Atmosphere", "Stimmung", "Atmosfera", "Ambiente").t(), 36.0);
    spawn_text(commands, panel, tx!("Le décor suit l'heure, la saison, les fêtes\net la météo de chez vous.", "The scenery follows the time of day, the season,\nthe holidays and your local weather.", "Die Kulisse folgt Tageszeit, Jahreszeit,\nFesttagen und dem Wetter bei Ihnen.", "Lo scenario segue l'ora, la stagione,\nle feste e il meteo di casa tua.", "El decorado sigue la hora, la estación,\nlas fiestas y el tiempo de tu zona.").t(), 16.0);
    spawn_gap(commands, panel, 4.0);
    let vraie = Saison::de_la_date(crate::calendar::Date::today());
    let saison = match p.saison {
        ChoixSaison::Auto => tx!("automatique ({})", "automatic ({})", "automatisch ({})", "automatica ({})", "automática ({})").avec(&[&nom_saison(vraie)]),
        ChoixSaison::Fixe(s) => nom_saison(s).to_string(),
        ChoixSaison::Aucune => oui(false, false).to_string(),
    };
    let lignes = [
        (Choix::Active, tx!("Ambiance : {}", "Atmosphere: {}", "Stimmung: {}", "Atmosfera: {}", "Ambiente: {}").avec(&[&oui(p.active, false)])),
        (Choix::Moment, tx!("Moment de la journée : {}", "Time of day: {}", "Tageszeit: {}", "Ora del giorno: {}", "Momento del día: {}").avec(&[&oui(p.moment, true)])),
        (Choix::Saison, tx!("Saison : {}", "Season: {}", "Jahreszeit: {}", "Stagione: {}", "Estación: {}").avec(&[&saison])),
        (Choix::Fete, tx!("Fête du moment : {}", "Holiday of the day: {}", "Festtag: {}", "Festa del giorno: {}", "Fiesta del día: {}").avec(&[&oui(p.fete, false)])),
        (Choix::Meteo, tx!("Météo locale : {}", "Local weather: {}", "Lokales Wetter: {}", "Meteo locale: {}", "Tiempo local: {}").avec(&[&oui(p.meteo, false)])),
    ];
    for (i, (choix, texte)) in lignes.into_iter().enumerate() {
        let (b, t) = spawn_button_sized(commands, panel, texte, 400.0, 44.0, 19.0);
        commands.entity(b).insert(choix);
        // Ambiance désactivée : les réglages détaillés sont grisés.
        if i > 0 && !p.active {
            commands.entity(t).insert(TextColor(Color::srgba(0.4, 0.4, 0.45, 0.6)));
        }
    }
    if p.meteo && p.active {
        if cfg!(target_arch = "wasm32") {
            spawn_text(commands, panel, tx!("Dans le navigateur, la météo est celle\nchoisie sur le site (bouton nuage).", "In the browser, the weather is the one\nchosen on the website (cloud button).", "Im Browser gilt das auf der Website\ngewählte Wetter (Wolken-Schaltfläche).", "Nel browser, il meteo è quello\nscelto sul sito (pulsante nuvola).", "En el navegador, el tiempo es el\nelegido en la web (botón de la nube).").t(), 15.0);
        } else {
            let ville = if saisie.active {
                format!("{}{}_", ville_titre(), saisie.texte)
            } else {
                format!("{}{}", ville_titre(), ville_enregistree().map_or(tx!("à choisir", "not set", "nicht gewählt", "da scegliere", "por elegir").t().to_string(), |v| v.0))
            };
            let (b, t) = spawn_button_sized(commands, panel, ville, 400.0, 44.0, 19.0);
            commands.entity(b).insert(Choix::Ville);
            commands.entity(t).insert(TexteVille);
            spawn_text(commands, panel, tx!("Votre ville est envoyée au service libre Open-Meteo\npour connaître le temps qu'il fait. Rien d'autre.", "Your city is sent to the free Open-Meteo service\nto get the current weather. Nothing else.", "Ihre Stadt wird an den freien Dienst Open-Meteo\ngesendet, um das Wetter abzufragen. Sonst nichts.", "La tua città viene inviata al servizio libero Open-Meteo\nper conoscere il meteo. Nient'altro.", "Tu ciudad se envía al servicio libre Open-Meteo\npara conocer el tiempo. Nada más.").t(), 13.0);
        }
    }
    let etat = spawn_text(commands, panel, saisie.etat.clone(), 15.0);
    commands.entity(etat).insert(TexteEtat);
    spawn_gap(commands, panel, 2.0);
    let (b, _) = spawn_button(commands, panel, tx!("Fermer", "Close", "Schließen", "Chiudi", "Cerrar").t());
    commands.entity(b).insert(Choix::Fermer);
}

#[allow(clippy::too_many_arguments)]
fn au_clic(
    clic: On<PointerClick>,
    mut commands: Commands,
    boutons: Query<(), With<AmbianceBouton>>,
    choix: Query<&Choix>,
    panneaux: Query<Entity, With<Panneau>>,
    mut prefs: ResMut<Preferences>,
    mut saisie: ResMut<Saisie>,
    recue: Res<MeteoRecue>,
) {
    let fermer = |commands: &mut Commands| panneaux.iter().for_each(|e| commands.entity(e).despawn());
    if boutons.contains(clic.entity) {
        if panneaux.is_empty() {
            saisie.etat.clear();
            ouvrir(&mut commands, &prefs, &saisie);
        }
        return;
    }
    let Ok(c) = choix.get(clic.entity) else { return };
    let mut p = *prefs;
    match c {
        Choix::Fermer => {
            saisie.active = false;
            fermer(&mut commands);
            return;
        }
        Choix::Active => p.active = !p.active,
        Choix::Moment => p.moment = !p.moment,
        Choix::Saison => p.saison_suivante(),
        Choix::Fete => p.fete = !p.fete,
        Choix::Meteo => {
            p.meteo = !p.meteo;
            // Première activation sur PC : on demande la ville.
            if p.meteo && !cfg!(target_arch = "wasm32") {
                match ville_enregistree() {
                    None => {
                        saisie.active = true;
                        saisie.texte.clear();
                        saisie.etat = tape_ta_ville();
                    }
                    Some((nom, lat, lon)) => chercher_meteo(&recue, Some((nom, lat, lon)), None, &mut saisie),
                }
            }
        }
        Choix::Ville => {
            saisie.active = true;
            saisie.texte.clear();
            saisie.etat = tape_ta_ville();
        }
    }
    p.enregistrer();
    *prefs = p;
    fermer(&mut commands);
    ouvrir(&mut commands, &p, &saisie);
}

/// Pendant la saisie de la ville, le clavier est à elle : le jeu ne voit pas les touches.
fn saisir(
    mut saisie: ResMut<Saisie>,
    mut clavier: MessageReader<KeyboardInput>,
    mut touches: ResMut<ButtonInput<KeyCode>>,
    mut textes: Query<&mut Text, With<TexteVille>>,
    mut etats: Query<&mut Text, (With<TexteEtat>, Without<TexteVille>)>,
    recue: Res<MeteoRecue>,
) {
    if !saisie.active {
        clavier.clear();
        return;
    }
    for ev in clavier.read() {
        if !ev.state.is_pressed() {
            continue;
        }
        match &ev.logical_key {
            Key::Enter => {
                saisie.active = false;
                let ville = saisie.texte.trim().to_string();
                if !ville.is_empty() {
                    chercher_meteo(&recue, None, Some(ville), &mut saisie);
                }
            }
            Key::Escape => {
                saisie.active = false;
                saisie.etat.clear();
            }
            Key::Backspace => {
                saisie.texte.pop();
            }
            Key::Space => saisie.texte.push(' '),
            Key::Character(c) if saisie.texte.chars().count() < 40 => saisie.texte.push_str(c),
            _ => {}
        }
    }
    touches.reset_all();
    for mut t in &mut textes {
        t.0 = if saisie.active { format!("{}{}_", ville_titre(), saisie.texte) } else { format!("{}{}", ville_titre(), saisie.texte) };
    }
    for mut t in &mut etats {
        t.0 = saisie.etat.clone();
    }
}

/// Une réponse météo est arrivée : on l'enregistre et on prévient l'ambiance.
fn reponse_meteo(mut recue: ResMut<MeteoRecue>, mut saisie: ResMut<Saisie>, mut etats: Query<&mut Text, With<TexteEtat>>) {
    let reponse = recue.0.lock().ok().and_then(|mut r| r.take());
    let Some(reponse) = reponse else { return };
    saisie.etat = match reponse {
        Ok((ville, temps)) => {
            recue.set_changed();
            tx!("Météo de {} : {}.", "Weather in {}: {}.", "Wetter in {}: {}.", "Meteo di {}: {}.", "Tiempo en {}: {}.").avec(&[&ville, &nom_meteo(temps)])
        }
        Err(e) => e,
    };
    for mut t in &mut etats {
        t.0 = saisie.etat.clone();
    }
}

/// Le panneau ouvert suit les réglages (après une réponse météo, par exemple).
fn rafraichir(
    mut commands: Commands,
    recue: Res<MeteoRecue>,
    prefs: Res<Preferences>,
    saisie: Res<Saisie>,
    panneaux: Query<Entity, With<Panneau>>,
    amb: Res<Ambiance>,
) {
    let _ = amb;
    if recue.is_changed() && !recue.is_added() && !panneaux.is_empty() {
        panneaux.iter().for_each(|e| commands.entity(e).despawn());
        ouvrir(&mut commands, &prefs, &saisie);
    }
}

/// Au lancement, on rafraîchit la météo si elle a plus d'une demi-heure.
fn meteo_au_lancement(prefs: Res<Preferences>, recue: Res<MeteoRecue>, mut saisie: ResMut<Saisie>) {
    if !prefs.active || !prefs.meteo || cfg!(target_arch = "wasm32") {
        return;
    }
    let fraiche = meteo_enregistree().is_some_and(|(_, age)| age < 30.0 * 60.0);
    if !fraiche && let Some(lieu) = ville_enregistree() {
        chercher_meteo(&recue, Some(lieu), None, &mut saisie);
        saisie.etat.clear();
    }
}

// ---- Météo : enregistrement et recherche ------------------------------------

fn maintenant_s() -> f64 {
    web_time::SystemTime::now().duration_since(web_time::UNIX_EPOCH).map(|d| d.as_secs_f64()).unwrap_or(0.0)
}

/// La ville choisie : (nom, latitude, longitude).
fn ville_enregistree() -> Option<(String, String, String)> {
    let v = crate::save::load("studio", "ville")?;
    let mut it = v.split('|');
    Some((it.next()?.to_string(), it.next()?.to_string(), it.next()?.to_string()))
}

/// La dernière météo connue et son âge (secondes).
fn meteo_enregistree() -> Option<(Meteo, f64)> {
    let v = crate::save::load("studio", "meteo")?;
    let mut it = v.split_whitespace();
    let temps = it.next()?;
    let t: f64 = it.next()?.parse().ok()?;
    let m = [Meteo::Clair, Meteo::Nuageux, Meteo::Pluie, Meteo::Neige, Meteo::Orage, Meteo::Brouillard]
        .into_iter()
        .find(|m| code(*m) == temps)?;
    Some((m, maintenant_s() - t))
}

fn code(m: Meteo) -> &'static str {
    match m {
        Meteo::Clair => "clair",
        Meteo::Nuageux => "nuageux",
        Meteo::Pluie => "pluie",
        Meteo::Neige => "neige",
        Meteo::Orage => "orage",
        Meteo::Brouillard => "brouillard",
    }
}

/// La météo locale à utiliser : celle du site dans le navigateur, sinon la
/// dernière reçue (si elle a moins de trois heures).
pub fn meteo_locale() -> Option<Meteo> {
    if cfg!(target_arch = "wasm32") {
        return web::meteo_du_site();
    }
    meteo_enregistree().filter(|(_, age)| *age < 3.0 * 3600.0).map(|(m, _)| m)
}

/// Code météo WMO (Open-Meteo) → temps du jeu (mêmes règles que le site).
#[cfg(not(target_arch = "wasm32"))]
fn traduire(code: i32) -> Meteo {
    match code {
        0 | 1 => Meteo::Clair,
        2 | 3 => Meteo::Nuageux,
        45 | 48 => Meteo::Brouillard,
        71..=77 | 85 | 86 => Meteo::Neige,
        95.. => Meteo::Orage,
        _ => Meteo::Pluie,
    }
}

/// Lance la recherche en arrière-plan : d'une ville déjà connue, ou d'un nom tapé.
fn chercher_meteo(recue: &MeteoRecue, lieu: Option<(String, String, String)>, nom: Option<String>, saisie: &mut Saisie) {
    saisie.etat = tx!("Recherche de la météo…", "Getting the weather…", "Wetter wird abgefragt…", "Ricerca del meteo…", "Buscando el tiempo…").t().into();
    #[cfg(not(target_arch = "wasm32"))]
    {
        let sortie = recue.0.clone();
        std::thread::spawn(move || {
            let r = (|| -> Result<(String, Meteo), String> {
                let (ville, lat, lon) = match lieu {
                    Some(l) => l,
                    None => geocoder(nom.as_deref().unwrap_or(""))?,
                };
                let json = telecharger(&format!(
                    "https://api.open-meteo.com/v1/forecast?latitude={lat}&longitude={lon}&current=weather_code"
                ))?;
                let courant = &json[json.find("\"current\":").ok_or_else(indisponible)?..];
                let code: i32 = web::champ(courant, "weather_code").and_then(|c| c.parse().ok()).ok_or_else(indisponible)?;
                let temps = traduire(code);
                crate::save::store("studio", "ville", &format!("{ville}|{lat}|{lon}"));
                crate::save::store("studio", "meteo", &format!("{} {}", self::code(temps), maintenant_s()));
                Ok((ville, temps))
            })();
            if let Ok(mut s) = sortie.lock() {
                *s = Some(r);
            }
        });
    }
    #[cfg(target_arch = "wasm32")]
    {
        let _ = (recue, lieu, nom);
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn telecharger(url: &str) -> Result<String, String> {
    let sortie = std::process::Command::new("curl")
        .args(["-s", "-f", "--max-time", "12", url])
        .output()
        .map_err(|_| tx!("Météo indisponible : l'outil curl est introuvable.", "Weather unavailable: the curl tool was not found.", "Wetter nicht verfügbar: Das Programm curl fehlt.", "Meteo non disponibile: manca lo strumento curl.", "Tiempo no disponible: falta la herramienta curl.").t().to_string())?;
    if !sortie.status.success() {
        return Err(indisponible());
    }
    Ok(String::from_utf8_lossy(&sortie.stdout).into_owned())
}

/// Nom de ville → (nom affiché, latitude, longitude), arrondies au centième.
#[cfg(not(target_arch = "wasm32"))]
fn geocoder(nom: &str) -> Result<(String, String, String), String> {
    let encode: String = nom
        .bytes()
        .map(|b| if b.is_ascii_alphanumeric() { (b as char).to_string() } else { format!("%{b:02X}") })
        .collect();
    let json = telecharger(&format!("https://geocoding-api.open-meteo.com/v1/search?count=1&language={}&name={encode}", crate::langue::actuelle().code()))?;
    let introuvable = || tx!("Ville introuvable.", "City not found.", "Stadt nicht gefunden.", "Città non trovata.", "Ciudad no encontrada.").t().to_string();
    let lat: f64 = web::champ(&json, "latitude").and_then(|v| v.parse().ok()).ok_or_else(introuvable)?;
    let lon: f64 = web::champ(&json, "longitude").and_then(|v| v.parse().ok()).ok_or_else(introuvable)?;
    let ville = web::champ(&json, "name").ok_or_else(introuvable)?;
    let pays = web::champ(&json, "country_code").map(|p| format!(" ({p})")).unwrap_or_default();
    Ok((format!("{ville}{pays}"), format!("{lat:.2}"), format!("{lon:.2}")))
}

fn ville_titre() -> &'static str {
    tx!("Ville : ", "City: ", "Stadt: ", "Città: ", "Ciudad: ").t()
}

fn tape_ta_ville() -> String {
    tx!("Tapez votre ville, puis Entrée.", "Type your city, then Enter.", "Stadt eingeben, dann Eingabe.", "Scrivi la tua città, poi Invio.", "Escribe tu ciudad y pulsa Intro.").t().to_string()
}

#[cfg(not(target_arch = "wasm32"))]
fn indisponible() -> String {
    tx!("Météo indisponible pour le moment.", "Weather unavailable right now.", "Wetter zurzeit nicht verfügbar.", "Meteo non disponibile al momento.", "Tiempo no disponible por ahora.").t().to_string()
}
