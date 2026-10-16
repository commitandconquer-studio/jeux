//! Ambiance commune à tous les jeux (et au site) : le moment de la journée,
//! la saison, la fête du jour et la météo. Les règles sont les mêmes que sur
//! le site (`site/static/js/ambiance.js`) : matin 6–10 h, journée 10–18 h,
//! soir 18–21 h, nuit ; printemps au 20 mars, été au 21 juin, automne au
//! 22 septembre, hiver au 21 décembre.
//!
//! `AmbiancePlugin` s'applique par-dessus les réglages de chaque jeu :
//! - la lumière (soleil, lumière ambiante, couleur du ciel) suit le moment
//!   et la météo ; si le jeu change sa lumière, l'ambiance repart de là ;
//! - les herbes et feuillages de la palette commune prennent la couleur de
//!   la saison ;
//! - pluie, neige, orage et brouillard tombent devant la caméra ;
//! - un message salue la fête du jour.
//!
//! Pour essayer : `CC_MOMENT=nuit CC_SAISON=hiver CC_FETE=noel CC_METEO=neige`
//! (et `CC_DATE=AAAA-MM-JJ`). Dans le navigateur, les réglages du bouton
//! « Ambiance » du site et la météo acceptée sur le site sont repris.
use bevy::light::NotShadowCaster;
use bevy::pbr::{DistanceFog, FogFalloff};
use bevy::prelude::*;

use crate::Rng;
use crate::calendar::{Date, Feast, feast_on};
use crate::palette;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Moment {
    Matin,
    Jour,
    Soir,
    Nuit,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Saison {
    Printemps,
    Ete,
    Automne,
    Hiver,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Meteo {
    Clair,
    Nuageux,
    Pluie,
    Neige,
    Orage,
    Brouillard,
}

/// L'ambiance du moment (ressource insérée par `AmbiancePlugin`).
#[derive(Resource, Clone, Copy, Debug)]
pub struct Ambiance {
    pub date: Date,
    pub moment: Moment,
    pub saison: Saison,
    pub fete: Option<Feast>,
    pub meteo: Option<Meteo>,
}

impl Moment {
    pub fn de_l_heure(h: u32) -> Moment {
        match h {
            6..=9 => Moment::Matin,
            10..=17 => Moment::Jour,
            18..=20 => Moment::Soir,
            _ => Moment::Nuit,
        }
    }
    fn nom(s: &str) -> Option<Moment> {
        Some(match s {
            "matin" => Moment::Matin,
            "jour" => Moment::Jour,
            "soir" => Moment::Soir,
            "nuit" => Moment::Nuit,
            _ => return None,
        })
    }
}

impl Saison {
    pub fn de_la_date(d: Date) -> Saison {
        match d.month * 100 + d.day {
            320..=620 => Saison::Printemps,
            621..=921 => Saison::Ete,
            922..=1220 => Saison::Automne,
            _ => Saison::Hiver,
        }
    }
    fn nom(s: &str) -> Option<Saison> {
        Some(match s {
            "printemps" => Saison::Printemps,
            "ete" => Saison::Ete,
            "automne" => Saison::Automne,
            "hiver" => Saison::Hiver,
            _ => return None,
        })
    }
}

impl Meteo {
    fn nom(s: &str) -> Option<Meteo> {
        Some(match s {
            "clair" => Meteo::Clair,
            "nuageux" => Meteo::Nuageux,
            "pluie" => Meteo::Pluie,
            "neige" => Meteo::Neige,
            "orage" => Meteo::Orage,
            "brouillard" => Meteo::Brouillard,
            _ => return None,
        })
    }
    /// Le ciel est-il couvert ?
    pub fn gris(self) -> bool {
        !matches!(self, Meteo::Clair)
    }
}

/// Une fête désignée par son nom sur le site (`halloween`, `noel`…).
fn fete_nom(s: &str) -> Option<Option<Feast>> {
    Some(Some(match s {
        "aucune" | "" => return Some(None),
        "halloween" => Feast::Halloween,
        "avent" => Feast::Advent(2),
        "noel" => Feast::Christmas,
        "saint-nicolas" => Feast::SaintNicholas,
        "epiphanie" => Feast::Epiphany,
        "rameaux" => Feast::PalmSunday,
        "paques" => Feast::Easter,
        "semaine-sainte" => Feast::HolyTriduum,
        "chandeleur" => Feast::Candlemas,
        "mardi-gras" => Feast::MardiGras,
        "toussaint" => Feast::AllSaints,
        "ascension" => Feast::Ascension,
        "pentecote" => Feast::Pentecost,
        _ => return None,
    }))
}

/// Un réglage forcé : variable `CC_…` sur PC, bouton « Ambiance » du site
/// dans le navigateur (« auto » = pas de forçage).
fn force(cle: &str) -> Option<String> {
    let v = std::env::var(format!("CC_{}", cle.to_uppercase())).ok().or_else(|| web::force(cle))?;
    (v != "auto").then_some(v)
}

/// Le choix de saison du joueur.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChoixSaison {
    /// La vraie saison du jour.
    Auto,
    /// Toujours la même saison.
    Fixe(Saison),
    /// Pas de saison : les couleurs d'origine du jeu.
    Aucune,
}

/// Les réglages d'ambiance du joueur, communs à tous les jeux du studio
/// (sauvegardés sous « studio-ambiance »).
#[derive(Resource, Clone, Copy, Debug, PartialEq, Eq)]
pub struct Preferences {
    /// Interrupteur général : désactivée, le jeu garde son décor d'origine.
    pub active: bool,
    pub moment: bool,
    pub saison: ChoixSaison,
    /// La fête du moment (le joueur ne choisit pas laquelle).
    pub fete: bool,
    /// La météo locale (il faut aussi une ville, voir `ambiance_panneau`).
    pub meteo: bool,
}

impl Default for Preferences {
    fn default() -> Self {
        Preferences { active: true, moment: true, saison: ChoixSaison::Auto, fete: true, meteo: false }
    }
}

impl Preferences {
    const SAISONS: [(&'static str, ChoixSaison); 6] = [
        ("auto", ChoixSaison::Auto),
        ("printemps", ChoixSaison::Fixe(Saison::Printemps)),
        ("ete", ChoixSaison::Fixe(Saison::Ete)),
        ("automne", ChoixSaison::Fixe(Saison::Automne)),
        ("hiver", ChoixSaison::Fixe(Saison::Hiver)),
        ("aucune", ChoixSaison::Aucune),
    ];

    pub fn charger() -> Preferences {
        let Some(texte) = crate::save::load("studio", "ambiance") else { return Preferences::default() };
        let mots: Vec<&str> = texte.split_whitespace().collect();
        let oui = |i: usize, defaut: bool| mots.get(i).map_or(defaut, |m| *m == "1");
        let d = Preferences::default();
        Preferences {
            active: oui(0, d.active),
            moment: oui(1, d.moment),
            saison: mots.get(2).and_then(|m| Self::SAISONS.iter().find(|(n, _)| n == m)).map_or(d.saison, |(_, c)| *c),
            fete: oui(3, d.fete),
            meteo: oui(4, d.meteo),
        }
    }

    pub fn enregistrer(&self) {
        let b = |v: bool| if v { "1" } else { "0" };
        let saison = Self::SAISONS.iter().find(|(_, c)| *c == self.saison).map_or("auto", |(n, _)| n);
        crate::save::store("studio", "ambiance", &format!("{} {} {saison} {} {}", b(self.active), b(self.moment), b(self.fete), b(self.meteo)));
    }

    /// Saison suivante dans le choix du panneau.
    pub fn saison_suivante(&mut self) {
        let i = Self::SAISONS.iter().position(|(_, c)| *c == self.saison).unwrap_or(0);
        self.saison = Self::SAISONS[(i + 1) % Self::SAISONS.len()].1;
    }
}

impl Ambiance {
    /// L'ambiance de maintenant, selon les réglages enregistrés du joueur.
    pub fn maintenant() -> Ambiance {
        Ambiance::avec(&Preferences::charger())
    }

    /// L'ambiance de maintenant selon des réglages donnés. Les réglages forcés
    /// (variables `CC_…`, bouton « Ambiance » du site de développement)
    /// passent toujours en dernier.
    pub fn avec(p: &Preferences) -> Ambiance {
        use chrono::Timelike;
        let date = Date::today();
        let mut a = Ambiance {
            date,
            moment: Moment::de_l_heure(chrono::Local::now().hour()),
            saison: Saison::de_la_date(date),
            fete: feast_on(date),
            meteo: crate::ambiance_panneau::meteo_locale(),
        };
        // Ce que le joueur a désactivé reprend l'allure d'origine du jeu
        // (journée d'été, ciel clair, pas de fête).
        if !p.active || !p.moment {
            a.moment = Moment::Jour;
        }
        match p.saison {
            _ if !p.active => a.saison = Saison::Ete,
            ChoixSaison::Auto => {}
            ChoixSaison::Fixe(s) => a.saison = s,
            ChoixSaison::Aucune => a.saison = Saison::Ete,
        }
        if !p.active || !p.fete {
            a.fete = None;
        }
        if !p.active || !p.meteo {
            a.meteo = None;
        }
        if let Some(m) = force("moment").as_deref().and_then(Moment::nom) {
            a.moment = m;
        }
        if let Some(s) = force("saison").as_deref().and_then(Saison::nom) {
            a.saison = s;
        }
        if let Some(f) = force("fete").as_deref().and_then(fete_nom) {
            a.fete = f;
        }
        if let Some(m) = force("meteo") {
            a.meteo = Meteo::nom(&m);
        }
        a
    }

    pub fn nuit(&self) -> bool {
        self.moment == Moment::Nuit
    }
}

/// La fête du jour, réglages forcés compris (à utiliser plutôt que `feast_on`).
pub fn fete_du_jour() -> Option<Feast> {
    Ambiance::maintenant().fete
}

/// Message de fête, pour tous les jeux (le Concile a ses propres vœux).
pub fn voeux(f: Feast) -> Option<&'static str> {
    Some(
        match f {
            Feast::Halloween => tx!("Joyeux Halloween ! Même nos jeux ont peur.", "Happy Halloween! Even our games are scared.", "Fröhliches Halloween! Sogar unsere Spiele fürchten sich.", "Buon Halloween! Anche i nostri giochi hanno paura.", "¡Feliz Halloween! Hasta nuestros juegos tienen miedo."),
            Feast::Advent(_) => tx!("Bon temps de l'Avent !", "Happy Advent!", "Schöne Adventszeit!", "Buon Avvento!", "¡Feliz Adviento!"),
            Feast::Christmas => tx!("Joyeux Noël !", "Merry Christmas!", "Frohe Weihnachten!", "Buon Natale!", "¡Feliz Navidad!"),
            Feast::SaintNicholas => tx!("Bonne Saint-Nicolas !", "Happy Saint Nicholas Day!", "Frohen Nikolaustag!", "Buon San Nicola!", "¡Feliz San Nicolás!"),
            Feast::Epiphany => tx!("Bonne Épiphanie ! Qui aura la fève ?", "Happy Epiphany! Who will find the bean?", "Frohen Dreikönigstag! Wer findet die Bohne?", "Buona Epifania! Chi troverà la fava?", "¡Feliz Día de Reyes! ¿Quién encontrará el haba?"),
            Feast::PalmSunday => tx!("Bon dimanche des Rameaux !", "Happy Palm Sunday!", "Frohen Palmsonntag!", "Buona Domenica delle Palme!", "¡Feliz Domingo de Ramos!"),
            Feast::Easter => tx!("Joyeuses Pâques !", "Happy Easter!", "Frohe Ostern!", "Buona Pasqua!", "¡Felices Pascuas!"),
            Feast::HolyTriduum => tx!("Les cloches sont parties pour Rome…", "The bells have flown to Rome…", "Die Glocken sind nach Rom geflogen…", "Le campane sono volate a Roma…", "Las campanas se han ido a Roma…"),
            Feast::MardiGras => tx!("Joyeux Mardi gras !", "Happy Mardi Gras!", "Fröhlichen Faschingsdienstag!", "Buon Martedì grasso!", "¡Feliz Martes de Carnaval!"),
            Feast::Candlemas => tx!("Bonne Chandeleur !", "Happy Candlemas!", "Frohe Lichtmess!", "Buona Candelora!", "¡Feliz Candelaria!"),
            Feast::Ascension => tx!("Bonne fête de l'Ascension !", "Happy Ascension Day!", "Frohe Christi Himmelfahrt!", "Buona Ascensione!", "¡Feliz Ascensión!"),
            Feast::Pentecost => tx!("Bonne Pentecôte !", "Happy Pentecost!", "Frohe Pfingsten!", "Buona Pentecoste!", "¡Feliz Pentecostés!"),
            _ => return None,
        }
        .t(),
    )
}

/// Lecture des réglages du site, dans le navigateur.
pub(crate) mod web {
    use super::Meteo;

    #[cfg(target_arch = "wasm32")]
    pub(crate) fn lire(cle: &str) -> Option<String> {
        web_sys::window()?.local_storage().ok()??.get_item(cle).ok()?
    }

    /// Valeur d'un champ texte dans un petit objet JSON (`{"moment":"nuit"}`).
    pub(crate) fn champ(json: &str, cle: &str) -> Option<String> {
        let debut = json.find(&format!("\"{cle}\""))? + cle.len() + 2;
        let reste = json[debut..].trim_start().strip_prefix(':')?.trim_start();
        if let Some(texte) = reste.strip_prefix('"') {
            Some(texte[..texte.find('"')?].to_string())
        } else {
            Some(reste.split([',', '}']).next()?.trim().to_string())
        }
    }

    pub fn force(cle: &str) -> Option<String> {
        #[cfg(target_arch = "wasm32")]
        {
            let v = champ(&lire("cc-ambiance-force")?, cle)?;
            return Some(v);
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = cle;
            None
        }
    }

    /// La météo acceptée sur le site (moins de trois heures).
    pub fn meteo_du_site() -> Option<Meteo> {
        #[cfg(target_arch = "wasm32")]
        {
            let json = lire("cc-meteo")?;
            let t: f64 = champ(&json, "t")?.parse().ok()?;
            if js_sys_now() - t > 3.0 * 3600.0 * 1000.0 {
                return None;
            }
            return Meteo::nom(&champ(&json, "temps")?);
        }
        #[cfg(not(target_arch = "wasm32"))]
        None
    }

    #[cfg(target_arch = "wasm32")]
    fn js_sys_now() -> f64 {
        web_time::SystemTime::now().duration_since(web_time::UNIX_EPOCH).map(|d| d.as_millis() as f64).unwrap_or(0.0)
    }

    #[cfg(test)]
    mod tests {
        #[test]
        fn lit_un_champ() {
            let j = r#"{"moment":"nuit","saison":"hiver","t":1792135574421}"#;
            assert_eq!(super::champ(j, "saison").as_deref(), Some("hiver"));
            assert_eq!(super::champ(j, "t").as_deref(), Some("1792135574421"));
            assert_eq!(super::champ(j, "fete"), None);
        }
    }
}

// ---- Plugin -----------------------------------------------------------------

/// Ce que l'ambiance a le droit de changer dans un jeu.
#[derive(Clone, Copy)]
pub struct AmbiancePlugin {
    /// Lumière et ciel selon le moment et la météo.
    pub lumiere: bool,
    /// Le jeu se passe dehors : couleur du ciel, saisons, pluie et neige.
    pub exterieur: bool,
    /// Message de fête au lancement.
    pub voeux: bool,
}

impl AmbiancePlugin {
    /// Jeu en plein air (Snake, Traversée…).
    pub fn exterieur() -> Self {
        AmbiancePlugin { lumiere: true, exterieur: true, voeux: true }
    }
    /// Jeu en intérieur (un bureau, une table) : lumière du moment seulement.
    pub fn interieur() -> Self {
        AmbiancePlugin { lumiere: true, exterieur: false, voeux: true }
    }
    /// Jeu dans l'espace : seulement les fêtes.
    pub fn espace() -> Self {
        AmbiancePlugin { lumiere: false, exterieur: false, voeux: true }
    }
}

#[derive(Resource, Clone, Copy)]
struct Reglages(AmbiancePlugin);

impl Plugin for AmbiancePlugin {
    fn build(&self, app: &mut App) {
        let amb = Ambiance::maintenant();
        // `CC_AMBIANCE=non` : l'ambiance n'est que calculée, rien n'est changé (débogage).
        if std::env::var("CC_AMBIANCE").is_ok_and(|v| v == "non") {
            app.insert_resource(amb);
            return;
        }
        app.insert_resource(amb)
            .insert_resource(Reglages(*self))
            .insert_resource(Preferences::charger())
            .init_resource::<Originaux>()
            .add_plugins(crate::ambiance_panneau::PanneauPlugin)
            .add_systems(PreUpdate, recalculer);
        if self.lumiere {
            app.add_systems(PostUpdate, (lumiere_soleils, lumiere_globale));
        }
        if self.exterieur {
            app.add_systems(Update, (couleurs_de_saison, brouillard, precipitations));
        }
        if self.voeux && amb.fete.and_then(voeux).is_some() {
            app.add_systems(Update, bandeau_de_fete);
        }
    }
}

/// Teinte et force de la lumière : (couleur du soleil, × soleil, couleur
/// ambiante, × ambiante, couleur vers laquelle tirer le ciel, part du mélange).
fn reglage(a: &Ambiance, exterieur: bool) -> (Color, f32, Color, f32, Color, f32) {
    let (soleil, fs, amb, fa, ciel, part) = match a.moment {
        Moment::Matin => (Color::srgb(1.0, 0.86, 0.74), 0.85, Color::srgb(1.0, 0.92, 0.86), 0.9, Color::srgb(1.0, 0.82, 0.72), 0.35),
        Moment::Jour => (Color::WHITE, 1.0, Color::WHITE, 1.0, Color::WHITE, 0.0),
        Moment::Soir => (Color::srgb(1.0, 0.70, 0.48), 0.7, Color::srgb(1.0, 0.82, 0.72), 0.75, Color::srgb(0.96, 0.58, 0.42), 0.4),
        Moment::Nuit => (Color::srgb(0.62, 0.72, 1.0), 0.28, Color::srgb(0.62, 0.70, 1.0), 0.5, Color::srgb(0.05, 0.09, 0.18), if exterieur { 0.82 } else { 0.45 }),
    };
    let (fs, part, ciel) = match a.meteo {
        Some(m) if exterieur && m.gris() => {
            let f = match m {
                Meteo::Pluie | Meteo::Orage => 0.55,
                Meteo::Brouillard => 0.7,
                _ => 0.8,
            };
            let gris = Color::srgb(0.60, 0.66, 0.71);
            (fs * f, (part + 0.45_f32).min(0.9), if part > 0.0 { ciel.mix(&gris, 0.5) } else { gris })
        }
        _ => (fs, part, ciel),
    };
    (soleil, fs, amb, fa, ciel, part)
}

fn teinter(c: Color, t: Color) -> Color {
    let (c, t) = (c.to_linear(), t.to_linear());
    LinearRgba::new(c.red * t.red, c.green * t.green, c.blue * t.blue, c.alpha).into()
}

/// Valeurs d'origine d'une lumière, et ce que l'ambiance y a mis.
#[derive(Component)]
struct Origine {
    couleur: Color,
    force: f32,
    mise: (Color, f32),
}

fn lumiere_soleils(
    mut commands: Commands,
    amb: Res<Ambiance>,
    reg: Res<Reglages>,
    time: Res<Time>,
    mut soleils: Query<(Entity, &mut DirectionalLight, Option<&mut Origine>)>,
    mut lampes: Query<(Entity, &mut PointLight, Option<&mut Origine>), Without<DirectionalLight>>,
) {
    let (teinte, f, ..) = reglage(&amb, reg.0.exterieur);
    let eclair = eclair(&amb, &time);
    for (e, mut l, origine) in &mut soleils {
        match origine {
            Some(mut o) => {
                // Le jeu a changé sa lumière (changement d'univers…) : on repart
                // de sa nouvelle valeur, réglage par réglage.
                if l.color != o.mise.0 {
                    o.couleur = l.color;
                }
                if l.illuminance != o.mise.1 {
                    o.force = l.illuminance;
                }
                appliquer(&mut l, &mut o, teinte, f * eclair);
            }
            None => {
                let mut o = Origine { couleur: l.color, force: l.illuminance, mise: (l.color, l.illuminance) };
                appliquer(&mut l, &mut o, teinte, f);
                commands.entity(e).insert(o);
            }
        }
    }
    // En intérieur la nuit, chandelles et lampes brillent davantage.
    let gain = if amb.nuit() && !reg.0.exterieur { 1.6 } else { 1.0 };
    for (e, mut l, origine) in &mut lampes {
        match origine {
            Some(mut o) => {
                if l.intensity != o.mise.1 {
                    o.force = l.intensity;
                }
                l.intensity = o.force * gain;
                o.mise.1 = l.intensity;
            }
            None => {
                let force = l.intensity;
                l.intensity = force * gain;
                commands.entity(e).insert(Origine { couleur: l.color, force, mise: (l.color, l.intensity) });
            }
        }
    }
}

fn appliquer(l: &mut DirectionalLight, o: &mut Origine, teinte: Color, f: f32) {
    l.color = teinter(o.couleur, teinte);
    l.illuminance = o.force * f;
    o.mise = (l.color, l.illuminance);
}

/// Un éclair de temps en temps, les jours d'orage.
fn eclair(amb: &Ambiance, time: &Time) -> f32 {
    if amb.meteo != Some(Meteo::Orage) {
        return 1.0;
    }
    let t = time.elapsed_secs();
    let s = (t * 0.7).sin();
    if s > 0.995 || (s > 0.99 && (t * 40.0).sin() > 0.0) { 4.0 } else { 1.0 }
}

/// Lumière ambiante et couleur du ciel (ressources du jeu).
fn lumiere_globale(
    amb: Res<Ambiance>,
    reg: Res<Reglages>,
    time: Res<Time>,
    mut ambiante: ResMut<GlobalAmbientLight>,
    mut ciel: ResMut<ClearColor>,
    mut origine: Local<Option<(Color, f32, Color)>>,
    mut mise: Local<(Color, f32, Color)>,
) {
    let (_, _, teinte, fa, couleur_ciel, part) = reglage(&amb, reg.0.exterieur);
    // Première fois : les réglages du jeu. Ensuite, seul ce que le jeu a
    // changé lui-même (un ciel qui change de couleur…) devient la nouvelle
    // origine : sinon la teinte s'appliquerait sur elle-même à chaque image.
    let o = origine.get_or_insert((ambiante.color, ambiante.brightness, ciel.0));
    if ambiante.color != mise.0 && *mise != Default::default() {
        o.0 = ambiante.color;
    }
    if ambiante.brightness != mise.1 && *mise != Default::default() {
        o.1 = ambiante.brightness;
    }
    if ciel.0 != mise.2 && *mise != Default::default() {
        o.2 = ciel.0;
    }
    let (c, b, fond) = *o;
    ambiante.color = teinter(c, teinte);
    ambiante.brightness = b * fa * if eclair(&amb, &time) > 1.0 { 2.5 } else { 1.0 };
    ciel.0 = if part > 0.0 { fond.mix(&couleur_ciel, part) } else { fond };
    *mise = (ambiante.color, ambiante.brightness, ciel.0);
}

/// Recalcule l'ambiance quand le joueur change un réglage (ou qu'une météo arrive).
fn recalculer(prefs: Res<Preferences>, mut amb: ResMut<Ambiance>, recue: Option<Res<crate::ambiance_panneau::MeteoRecue>>) {
    if prefs.is_changed() || recue.is_some_and(|r| r.is_changed()) {
        let neuve = Ambiance::avec(&prefs);
        // Le moment et la date ne bougent pas en cours de partie : seuls les
        // réglages comptent ici.
        *amb = Ambiance { moment: neuve.moment, saison: neuve.saison, fete: neuve.fete, meteo: neuve.meteo, ..*amb };
    }
}

/// Matériaux de la palette commune repérés, avec leur couleur d'origine.
#[derive(Resource, Default)]
struct Originaux(Vec<(AssetId<StandardMaterial>, Color)>);

fn teinte_de_saison(saison: Saison, c: Color) -> Color {
    let remplacements: &[(Color, Color)] = match saison {
        Saison::Ete => return c,
        Saison::Printemps => &[
            (palette::GRASS, Color::srgb(0.62, 0.87, 0.44)),
            (palette::GRASS_LIGHT, Color::srgb(0.70, 0.92, 0.50)),
            (palette::FOLIAGE, Color::srgb(0.44, 0.75, 0.40)),
        ],
        Saison::Automne => &[
            (palette::GRASS, Color::srgb(0.72, 0.70, 0.38)),
            (palette::GRASS_LIGHT, Color::srgb(0.80, 0.74, 0.42)),
            (palette::FOLIAGE, Color::srgb(0.86, 0.52, 0.22)),
            (palette::FOLIAGE_DARK, Color::srgb(0.72, 0.38, 0.17)),
        ],
        Saison::Hiver => &[
            (palette::GRASS, Color::srgb(0.92, 0.95, 0.97)),
            (palette::GRASS_LIGHT, Color::srgb(0.97, 0.98, 1.0)),
            (palette::FOLIAGE, Color::srgb(0.36, 0.56, 0.42)),
            (palette::FOLIAGE_DARK, Color::srgb(0.27, 0.45, 0.34)),
        ],
    };
    remplacements.iter().find(|(o, _)| proche(c, *o)).map_or(c, |(_, n)| *n)
}

/// Herbes et feuillages de la palette commune, aux couleurs de la saison.
fn couleurs_de_saison(
    amb: Res<Ambiance>,
    mut events: MessageReader<AssetEvent<StandardMaterial>>,
    mut mats: ResMut<Assets<StandardMaterial>>,
    mut originaux: ResMut<Originaux>,
    mut derniere: Local<Option<Saison>>,
) {
    const PALETTE: [Color; 4] = [palette::GRASS, palette::GRASS_LIGHT, palette::FOLIAGE, palette::FOLIAGE_DARK];
    for ev in events.read() {
        if let AssetEvent::Added { id } = ev
            && let Some(mut m) = mats.get_mut(*id)
            && PALETTE.iter().any(|c| proche(m.base_color, *c))
        {
            originaux.0.push((*id, m.base_color));
            m.base_color = teinte_de_saison(amb.saison, m.base_color);
        }
    }
    // Le joueur a changé de saison : on repeint tout ce qui a été repéré.
    if *derniere != Some(amb.saison) {
        if derniere.is_some() {
            originaux.0.retain(|(id, _)| mats.contains(*id));
            for (id, c) in &originaux.0 {
                if let Some(mut m) = mats.get_mut(*id) {
                    m.base_color = teinte_de_saison(amb.saison, *c);
                }
            }
        }
        *derniere = Some(amb.saison);
    }
}

fn proche(a: Color, b: Color) -> bool {
    let (a, b) = (a.to_srgba(), b.to_srgba());
    (a.red - b.red).abs() < 0.01 && (a.green - b.green).abs() < 0.01 && (a.blue - b.blue).abs() < 0.01
}

/// Brouillard sur les caméras, les jours de brume.
fn brouillard(mut commands: Commands, amb: Res<Ambiance>, cams: Query<(Entity, Has<DistanceFog>), With<Camera3d>>) {
    let voulu = amb.meteo == Some(Meteo::Brouillard);
    let couleur = if amb.nuit() { Color::srgb(0.16, 0.22, 0.31) } else { Color::srgb(0.86, 0.89, 0.91) };
    for (e, a) in &cams {
        if voulu && !a {
            commands.entity(e).insert(DistanceFog { color: couleur, falloff: FogFalloff::Linear { start: 18.0, end: 55.0 }, ..default() });
        } else if !voulu && a {
            commands.entity(e).remove::<DistanceFog>();
        }
    }
}

/// Une goutte ou un flocon, repéré par rapport au point que regarde la caméra.
#[derive(Component)]
struct Particule(Vec3);

const BOITE: Vec3 = Vec3::new(16.0, 18.0, 16.0);

fn precipitations(
    mut commands: Commands,
    amb: Res<Ambiance>,
    time: Res<Time>,
    cams: Query<&GlobalTransform, With<Camera3d>>,
    mut gouttes: Query<(Entity, &mut Particule, &mut Transform)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<StandardMaterial>>,
    mut en_place: Local<Option<Meteo>>,
) {
    let voulu = amb.meteo.filter(|m| matches!(m, Meteo::Pluie | Meteo::Neige | Meteo::Orage));
    let neige = voulu == Some(Meteo::Neige);
    if *en_place != voulu {
        for (e, ..) in &gouttes {
            commands.entity(e).despawn();
        }
        *en_place = voulu;
        if voulu.is_none() {
            return;
        }
        let (mesh, couleur, n) = if neige {
            (meshes.add(Sphere::new(0.07).mesh().ico(0).unwrap()), Color::WHITE, 260)
        } else {
            (meshes.add(Cuboid::new(0.03, 0.55, 0.03)), Color::srgba(0.66, 0.80, 0.92, 0.75), 380)
        };
        let mat = mats.add(StandardMaterial { base_color: couleur, unlit: true, alpha_mode: AlphaMode::Blend, ..default() });
        let mut rng = Rng::new(42);
        let mut au_hasard = |m: f32| (rng.below(10_000) as f32 / 10_000.0 * 2.0 - 1.0) * m;
        for _ in 0..n {
            let p = Vec3::new(au_hasard(BOITE.x), au_hasard(BOITE.y), au_hasard(BOITE.z));
            commands.spawn((Mesh3d(mesh.clone()), MeshMaterial3d(mat.clone()), Transform::from_translation(p), Particule(p), NotShadowCaster));
        }
        return;
    }
    let Some(cam) = cams.iter().next() else { return };
    // Le centre de la boîte : un peu devant la caméra, là où se passe le jeu.
    let centre = cam.translation() + cam.forward() * 20.0;
    let dt = time.delta_secs();
    let t = time.elapsed_secs();
    let vitesse = if neige { 1.6 } else { 16.0 };
    for (i, (_, mut p, mut tr)) in gouttes.iter_mut().enumerate() {
        p.0.y -= vitesse * dt;
        if neige {
            p.0.x += (t * 0.8 + i as f32).sin() * 0.4 * dt;
        }
        if p.0.y < -BOITE.y {
            p.0.y += 2.0 * BOITE.y;
        }
        tr.translation = centre + p.0;
    }
}

/// Le message de fête : son heure d'apparition, sa pastille et son texte.
#[derive(Component)]
struct Bandeau(f32, Entity, Entity);

/// Le message de fête, quelques secondes après le lancement (après l'écran
/// de démarrage), puis il s'efface.
fn bandeau_de_fete(
    mut commands: Commands,
    amb: Res<Ambiance>,
    time: Res<Time>,
    bandeaux: Query<(Entity, &Bandeau)>,
    mut fonds: Query<&mut BackgroundColor>,
    mut textes: Query<&mut TextColor>,
    mut fait: Local<bool>,
) {
    let t = time.elapsed_secs();
    if !*fait && t > 3.0 {
        *fait = true;
        let Some(texte) = amb.fete.and_then(voeux) else { return };
        let racine = commands
            .spawn((
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(14.0),
                    width: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                Pickable::IGNORE,
                GlobalZIndex(i32::MAX - 2),
            ))
            .id();
        let pastille = commands
            .spawn((
                Node { padding: UiRect::axes(Val::Px(20.0), Val::Px(8.0)), border_radius: BorderRadius::MAX, ..default() },
                BackgroundColor(crate::brand::colors::LEMON),
                Pickable::IGNORE,
                ChildOf(racine),
            ))
            .id();
        let mot = commands
            .spawn((
                Text::new(texte),
                TextFont::from_font_size(20.0),
                TextColor(crate::brand::colors::NIGHT),
                Pickable::IGNORE,
                ChildOf(pastille),
            ))
            .id();
        commands.entity(racine).insert(Bandeau(t, pastille, mot));
        return;
    }
    for (e, b) in &bandeaux {
        let age = t - b.0;
        let alpha = (1.0 - (age - 6.0) / 1.0).clamp(0.0, 1.0);
        if let Ok(mut f) = fonds.get_mut(b.1) {
            f.0.set_alpha(alpha);
        }
        if let Ok(mut tc) = textes.get_mut(b.2) {
            tc.0.set_alpha(alpha);
        }
        if age > 7.0 {
            commands.entity(e).despawn();
        }
    }
}
