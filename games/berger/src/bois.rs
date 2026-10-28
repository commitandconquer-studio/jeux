//! Les écrans « en bois » : niveaux (cartes), « Mon chien » et fin de partie, avec le kit UI
//! (panneaux de bois, cartes, étoiles, cadenas). Le menu principal est dans `ui.rs`.
use berger_core::{Fin, NIVEAUX};
use bevy::prelude::*;
use game_common::langue::Texte;
use game_common::tx;

use crate::GameState;
use crate::chiens::{CHIENS, ChienVu};
use crate::jeu::{Genre, Partie, Progression};
use crate::ui::{Action, Survol, temps, titre_partie};

const CREME: Color = Color::srgb(1.0, 0.96, 0.84);
const OR: Color = Color::srgb(1.0, 0.9, 0.55);
const BRUN: Color = Color::srgb(0.30, 0.17, 0.05);
/// Les niveaux de la prairie (1 à 12), puis ceux du désert.
const PRAIRIE: usize = 12;

/// Les boutons de l'écran « Mon chien ».
#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub enum ActionChien {
    Ouvrir,
    Precedent,
    Suivant,
    Choisir,
}

/// Les onglets de l'écran Niveaux : 0 = prairie, 1 = désert.
#[derive(Component, Clone, Copy)]
struct Onglet(usize);

#[derive(Resource, Default)]
struct OngletVu(usize);

#[derive(Component)]
struct EcranRefait;

pub struct BoisPlugin;

impl Plugin for BoisPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<OngletVu>()
            .add_systems(OnEnter(GameState::Niveaux), ouvrir_niveaux)
            .add_systems(OnEnter(GameState::Fin), spawn_fin)
            .add_systems(OnEnter(GameState::Chiens), spawn_chiens)
            .add_systems(Update, (maj_niveaux, maj_chiens))
            .add_observer(au_clic_chien)
            .add_observer(au_clic_onglet);
    }
}

// --- Les pièces communes ---------------------------------------------------------------

fn image(server: &AssetServer, nom: &str) -> Handle<Image> {
    server.load(format!("ui/suivez/{nom}.png"))
}

fn texte(c: &mut Commands, parent: Entity, s: impl Into<String>, taille: f32, couleur: Color) -> Entity {
    c.spawn((
        Text::new(s),
        TextFont::from_font_size(taille),
        TextColor(couleur),
        TextShadow { offset: Vec2::new(0.0, 2.0), color: Color::srgba(0.12, 0.07, 0.04, 0.8) },
        TextLayout::justify(Justify::Center),
        Pickable::IGNORE,
        ChildOf(parent),
    ))
    .id()
}

/// L'écran : plein cadre, avec le tableau de bois au centre (`gauche` : contre le bord gauche).
fn ecran(c: &mut Commands, server: &AssetServer, scope: impl Bundle, gauche: bool) -> Entity {
    let racine = c
        .spawn((
            Node {
                width: percent(100),
                height: percent(100),
                justify_content: if gauche { JustifyContent::FlexStart } else { JustifyContent::Center },
                align_items: AlignItems::Center,
                padding: UiRect::all(px(16)),
                ..default()
            },
            scope,
        ))
        .id();
    c.spawn((Node { position_type: PositionType::Absolute, left: px(0), bottom: px(0), width: px(260), height: px(124), ..default() }, ImageNode::new(image(server, "decor_bas_gauche")), Pickable::IGNORE, ChildOf(racine)));
    c.spawn((
        Node { position_type: PositionType::Absolute, right: px(0), bottom: px(0), width: px(260), height: px(124), ..default() },
        ImageNode { flip_x: true, ..ImageNode::new(image(server, "decor_bas_gauche")) },
        Pickable::IGNORE,
        ChildOf(racine),
    ));
    c.spawn((
        Node { flex_direction: FlexDirection::Column, align_items: AlignItems::Center, row_gap: px(8), padding: UiRect::all(px(14)), border: UiRect::all(px(4)), border_radius: BorderRadius::all(px(18)), ..default() },
        BackgroundColor(Color::srgb(0.66, 0.43, 0.24)),
        BorderColor::all(Color::srgb(0.43, 0.27, 0.14)),
        ChildOf(racine),
    ))
    .id()
}

/// Une planche de bois avec son texte : le titre (dorée) ou un bouton (foncée, qui grossit au survol).
fn planche(c: &mut Commands, server: &AssetServer, parent: Entity, s: &str, largeur: f32, doree: bool) -> Entity {
    let b = c
        .spawn((
            Node { width: px(largeur), height: px(largeur * 0.28), justify_content: JustifyContent::Center, align_items: AlignItems::Center, padding: UiRect::axes(px(largeur * 0.08), px(0)), ..default() },
            ImageNode::new(image(server, if doree { "panneau_bois_dore_large" } else { "panneau_bois_fonce_large" })),
            ChildOf(parent),
        ))
        .id();
    if !doree {
        c.entity(b).insert((UiTransform::default(), Survol));
    }
    let taille = (largeur * 0.085).clamp(14.0, 26.0) * if s.chars().count() > 13 { 0.85 } else { 1.0 };
    let t = texte(c, b, s, taille, if doree { BRUN } else { CREME });
    if doree {
        c.entity(t).remove::<TextShadow>();
    }
    b
}

fn bouton(c: &mut Commands, server: &AssetServer, parent: Entity, s: &str, action: Action) -> Entity {
    let b = planche(c, server, parent, s, 240.0, false);
    c.entity(b).insert(action);
    b
}

/// Trois étoiles : dorées pour celles gagnées, grises pour les autres.
fn etoiles(c: &mut Commands, server: &AssetServer, parent: Entity, gagnees: u8, taille: f32) {
    let rangee = c.spawn((Node { column_gap: px(taille * 0.1), ..default() }, Pickable::IGNORE, ChildOf(parent))).id();
    for i in 0..3 {
        c.spawn((
            Node { width: px(taille), height: px(taille), ..default() },
            ImageNode::new(image(server, if i < gagnees { "icone_etoile_doree" } else { "icone_etoile_grise" })),
            Pickable::IGNORE,
            ChildOf(rangee),
        ));
    }
}

fn nom_monde(i: usize) -> Texte {
    if i == 0 { tx!("Prairie", "Meadow", "Wiese", "Prato", "Pradera") } else { tx!("Désert", "Desert", "Wüste", "Deserto", "Desierto") }
}

// --- Niveaux ---------------------------------------------------------------------------

fn ouvrir_niveaux(mut vu: ResMut<OngletVu>, progression: Res<Progression>) {
    // On ouvre le monde où l'on en est.
    vu.0 = (progression.suivant() >= PRAIRIE) as usize;
}

fn maj_niveaux(mut c: Commands, server: Res<AssetServer>, progression: Res<Progression>, vu: Res<OngletVu>, state: Res<State<GameState>>, ecran: Query<Entity, With<EcranRefait>>, mut deja: Local<bool>) {
    let dedans = *state.get() == GameState::Niveaux;
    if !dedans {
        *deja = false;
        return;
    }
    if *deja && !vu.is_changed() && !progression.is_changed() {
        return;
    }
    *deja = true;
    for e in &ecran {
        c.entity(e).despawn();
    }
    construire_niveaux(&mut c, &server, &progression, vu.0);
}

fn construire_niveaux(c: &mut Commands, server: &AssetServer, progression: &Progression, monde: usize) {
    let tableau = ecran(c, server, (DespawnOnExit(GameState::Niveaux), EcranRefait), false);
    let titre = planche(c, server, tableau, tx!("Niveaux", "Levels", "Level", "Livelli", "Niveles").t(), 300.0, true);
    let _ = titre;
    // Total d'étoiles.
    let ligne = c.spawn((Node { align_items: AlignItems::Center, column_gap: px(8), ..default() }, Pickable::IGNORE, ChildOf(tableau))).id();
    c.spawn((Node { width: px(24), height: px(24), ..default() }, ImageNode::new(image(server, "icone_etoile_doree")), Pickable::IGNORE, ChildOf(ligne)));
    texte(c, ligne, format!("{} / {}", progression.total(), NIVEAUX.len() * 3), 20.0, OR);
    // Un onglet par monde.
    let onglets = c.spawn((Node { column_gap: px(8), ..default() }, ChildOf(tableau))).id();
    for m in 0..2 {
        let debut = if m == 0 { 0 } else { PRAIRIE };
        let ouvert = progression.ouvert(debut);
        let o = c
            .spawn((
                Node { width: px(150), height: px(42), justify_content: JustifyContent::Center, align_items: AlignItems::Center, ..default() },
                ImageNode::new(image(server, if m == monde { "panneau_bois_dore_large" } else { "panneau_bois_fonce_large" })).with_color(if ouvert { Color::WHITE } else { Color::srgb(0.5, 0.5, 0.5) }),
                ChildOf(onglets),
            ))
            .id();
        if ouvert {
            c.entity(o).insert(Onglet(m));
        }
        texte(c, o, nom_monde(m).t(), 17.0, if m == monde { BRUN } else { CREME });
    }
    // Les cartes du monde.
    let (debut, fin) = if monde == 0 { (0, PRAIRIE) } else { (PRAIRIE, NIVEAUX.len()) };
    let grille = c.spawn((Node { width: px(4.0 * 136.0 + 3.0 * 8.0), min_height: px(3.0 * 118.0 + 2.0 * 8.0), flex_wrap: FlexWrap::Wrap, align_content: AlignContent::FlexStart, column_gap: px(8), row_gap: px(8), ..default() }, ChildOf(tableau))).id();
    for i in debut..fin {
        let ouvert = progression.ouvert(i);
        let carte = c
            .spawn((
                Node { width: px(136), height: px(118), flex_direction: FlexDirection::Column, align_items: AlignItems::Center, justify_content: JustifyContent::Center, row_gap: px(2), padding: UiRect::top(px(4)), ..default() },
                ImageNode::new(image(server, if !ouvert { "carte_vierge_verrouille" } else { "carte_vierge_disponible" })),
                ChildOf(grille),
            ))
            .id();
        if ouvert {
            c.entity(carte).insert((Action::Choisir(i), UiTransform::default(), Survol));
            let t = texte(c, carte, format!("{}", i + 1), 30.0, BRUN);
            c.entity(t).remove::<TextShadow>();
            etoiles(c, server, carte, progression.etoiles[i], 24.0);
            if NIVEAUX[i].loups > 0 {
                // Un point rouge : des loups rôdent sur ce niveau.
                c.spawn((
                    Node { position_type: PositionType::Absolute, top: px(12), right: px(14), width: px(12), height: px(12), border_radius: BorderRadius::all(percent(50)), ..default() },
                    BackgroundColor(Color::srgb(0.86, 0.18, 0.16)),
                    Pickable::IGNORE,
                    ChildOf(carte),
                ));
            }
        } else {
            let t = texte(c, carte, format!("{}", i + 1), 24.0, Color::srgb(0.62, 0.55, 0.48));
            c.entity(t).remove::<TextShadow>();
            c.spawn((Node { width: px(30), height: px(30), ..default() }, ImageNode::new(image(server, "icone_cadenas")), Pickable::IGNORE, ChildOf(carte)));
        }
    }
    texte(c, tableau, tx!("Point rouge : des loups rôdent", "Red dot: wolves about", "Roter Punkt: Wölfe unterwegs", "Punto rosso: ci sono lupi", "Punto rojo: hay lobos").t(), 14.0, CREME);
    bouton(c, server, tableau, tx!("Retour", "Back", "Zurück", "Indietro", "Volver").t(), Action::Menu);
}

fn au_clic_onglet(clic: On<PointerClick>, onglets: Query<&Onglet>, mut vu: ResMut<OngletVu>) {
    if let Ok(&Onglet(m)) = onglets.get(clic.entity) {
        vu.0 = m;
    }
}

// --- Mon chien -------------------------------------------------------------------------

fn spawn_chiens(mut c: Commands, server: Res<AssetServer>, progression: Res<Progression>, vu: Res<ChienVu>) {
    construire_chiens(&mut c, &server, &progression, vu.0);
}

/// L'écran se refait à chaque flèche ou choix.
fn maj_chiens(mut c: Commands, server: Res<AssetServer>, progression: Res<Progression>, vu: Res<ChienVu>, state: Res<State<GameState>>, ecran: Query<Entity, With<EcranRefait>>) {
    if *state.get() != GameState::Chiens || !(vu.is_changed() || progression.is_changed()) {
        return;
    }
    for e in &ecran {
        c.entity(e).despawn();
    }
    construire_chiens(&mut c, &server, &progression, vu.0);
}

fn construire_chiens(c: &mut Commands, server: &AssetServer, progression: &Progression, vu: usize) {
    let chien = &CHIENS[vu];
    let tableau = ecran(c, server, (DespawnOnExit(GameState::Chiens), EcranRefait), true);
    planche(c, server, tableau, tx!("Mon chien", "My dog", "Mein Hund", "Il mio cane", "Mi perro").t(), 300.0, true);
    // Flèche, nom du chien et ses deux couleurs, flèche.
    let rangee = c.spawn((Node { align_items: AlignItems::Center, column_gap: px(14), ..default() }, ChildOf(tableau))).id();
    let fleche = |c: &mut Commands, action: ActionChien, miroir: bool| {
        c.spawn((
            Node { width: px(54), height: px(54), ..default() },
            ImageNode::new(image(server, "fleche_precedent")),
            UiTransform { scale: Vec2::new(if miroir { -1.0 } else { 1.0 }, 1.0), ..default() },
            action,
            ChildOf(rangee),
        ));
    };
    fleche(c, ActionChien::Precedent, false);
    let centre = c.spawn((Node { flex_direction: FlexDirection::Column, align_items: AlignItems::Center, row_gap: px(6), width: px(170), ..default() }, Pickable::IGNORE, ChildOf(rangee))).id();
    texte(c, centre, chien.nom.t(), 30.0, CREME);
    let pastilles = c.spawn((Node { column_gap: px(8), ..default() }, Pickable::IGNORE, ChildOf(centre))).id();
    for couleur in [chien.pelage, chien.accent] {
        c.spawn((
            Node { width: px(26), height: px(26), border: UiRect::all(px(2)), border_radius: BorderRadius::all(percent(50)), ..default() },
            BackgroundColor(couleur),
            BorderColor::all(Color::srgb(0.35, 0.25, 0.15)),
            Pickable::IGNORE,
            ChildOf(pastilles),
        ));
    }
    texte(c, centre, format!("{} / {}", vu + 1, CHIENS.len()), 16.0, CREME);
    fleche(c, ActionChien::Suivant, true);
    if progression.chien_ouvert(vu) {
        if progression.chien == vu {
            texte(c, tableau, tx!("✓ C'est ton chien", "✓ This is your dog", "✓ Das ist dein Hund", "✓ È il tuo cane", "✓ Es tu perro").t(), 20.0, OR);
        } else {
            let b = planche(c, server, tableau, tx!("Choisir ce chien", "Pick this dog", "Diesen Hund wählen", "Scegli questo cane", "Elegir este perro").t(), 260.0, false);
            c.entity(b).insert(ActionChien::Choisir);
        }
    } else {
        let ligne = c.spawn((Node { align_items: AlignItems::Center, column_gap: px(8), ..default() }, Pickable::IGNORE, ChildOf(tableau))).id();
        c.spawn((ImageNode::new(image(server, "icone_cadenas")), Node { width: px(26), height: px(26), ..default() }, Pickable::IGNORE, ChildOf(ligne)));
        let reste = chien.seuil - progression.total();
        texte(c, ligne, tx!("Encore {} étoiles", "{} more stars", "Noch {} Sterne", "Ancora {} stelle", "{} estrellas más").avec(&[&reste]), 20.0, OR);
    }
    texte(c, tableau, tx!("Étoiles gagnées : {}", "Stars earned: {}", "Gewonnene Sterne: {}", "Stelle guadagnate: {}", "Estrellas ganadas: {}").avec(&[&progression.total()]), 16.0, CREME);
    bouton(c, server, tableau, tx!("Menu", "Menu", "Menü", "Menu", "Menú").t(), Action::Menu);
}

fn au_clic_chien(
    clic: On<PointerClick>,
    actions: Query<&ActionChien>,
    mut vu: ResMut<ChienVu>,
    mut progression: ResMut<Progression>,
    mut next: ResMut<NextState<GameState>>,
) {
    let Ok(&action) = actions.get(clic.entity) else { return };
    let n = CHIENS.len();
    match action {
        ActionChien::Ouvrir => {
            vu.0 = progression.chien;
            next.set(GameState::Chiens);
        }
        ActionChien::Precedent => vu.0 = (vu.0 + n - 1) % n,
        ActionChien::Suivant => vu.0 = (vu.0 + 1) % n,
        ActionChien::Choisir => {
            if progression.chien_ouvert(vu.0) {
                progression.chien = vu.0;
                progression.enregistrer();
            }
        }
    }
}

// --- Fin de partie ---------------------------------------------------------------------

fn spawn_fin(mut c: Commands, server: Res<AssetServer>, partie: Res<Partie>, progression: Res<Progression>) {
    let m = &partie.monde;
    let tableau = ecran(&mut c, &server, DespawnOnExit(GameState::Fin), true);
    let gagne = m.fin == Some(Fin::Victoire);
    let titre = match m.fin {
        Some(Fin::Victoire) => tx!("Bravo !", "Well done!", "Bravo!", "Bravo!", "¡Bravo!"),
        Some(Fin::Pertes) => tx!("Trop de pertes", "Too many lost", "Zu viele verloren", "Troppe perse", "Demasiadas perdidas"),
        _ => tx!("Temps écoulé", "Time's up", "Zeit abgelaufen", "Tempo scaduto", "Se acabó el tiempo"),
    };
    planche(&mut c, &server, tableau, titre.t(), 320.0, true);
    texte(&mut c, tableau, titre_partie(partie.genre), 20.0, CREME);
    if gagne {
        etoiles(&mut c, &server, tableau, m.etoiles() as u8, 56.0);
        texte(&mut c, tableau, tx!("Troupeau rentré en {}", "Flock home in {}", "Herde in {} eingebracht", "Gregge dentro in {}", "Rebaño dentro en {}").avec(&[&temps(m.temps)]), 20.0, CREME);
        let ligne = if m.temps <= m.niveau.temps_ref {
            tx!("Plus rapide que {}", "Faster than {}", "Schneller als {}", "Più veloce di {}", "Más rápido que {}").avec(&[&temps(m.niveau.temps_ref)])
        } else {
            tx!("Pour la 2ᵉ étoile : moins de {}", "For the 2nd star: under {}", "Für den 2. Stern: unter {}", "Per la 2ª stella: meno di {}", "Para la 2ª estrella: menos de {}").avec(&[&temps(m.niveau.temps_ref)])
        };
        texte(&mut c, tableau, ligne, 16.0, CREME);
        let perte = if m.perdus() == 0 {
            tx!("Aucun mouton perdu", "No sheep lost", "Kein Schaf verloren", "Nessuna pecora persa", "Ninguna oveja perdida").t().to_string()
        } else {
            tx!("Pour la 3ᵉ étoile : aucun mouton perdu", "For the 3rd star: lose no sheep", "Für den 3. Stern: kein Schaf verlieren", "Per la 3ª stella: nessuna pecora persa", "Para la 3ª estrella: ninguna oveja perdida").t().to_string()
        };
        texte(&mut c, tableau, perte, 16.0, CREME);
        if let Some(i) = progression.nouveau {
            texte(&mut c, tableau, tx!("Nouveau chien : {} !", "New dog: {}!", "Neuer Hund: {}!", "Nuovo cane: {}!", "¡Nuevo perro: {}!").avec(&[&CHIENS[i].nom.t()]), 22.0, OR);
        }
        if let Genre::Jour(_) = partie.genre {
            texte(&mut c, tableau, tx!("Série : {} jours", "Streak: {} days", "Serie: {} Tage", "Serie: {} giorni", "Racha: {} días").avec(&[&progression.serie]), 18.0, OR);
        }
    } else {
        texte(&mut c, tableau, tx!("Essayez encore : il faut garder le troupeau groupé.", "Try again: keep the flock together.", "Versuch's nochmal: Halte die Herde zusammen.", "Riprova: tieni unito il gregge.", "Inténtalo otra vez: mantén unido el rebaño.").t(), 17.0, CREME);
    }
    if gagne && matches!(partie.genre, Genre::Campagne(i) if i + 1 < NIVEAUX.len()) {
        bouton(&mut c, &server, tableau, tx!("Niveau suivant", "Next level", "Nächstes Level", "Livello successivo", "Siguiente nivel").t(), Action::Suivant);
    }
    bouton(&mut c, &server, tableau, tx!("Rejouer", "Play again", "Nochmal", "Rigioca", "Repetir").t(), Action::Rejouer);
    bouton(&mut c, &server, tableau, tx!("Niveaux", "Levels", "Level", "Livelli", "Niveles").t(), Action::Niveaux);
    bouton(&mut c, &server, tableau, tx!("Menu", "Menu", "Menü", "Menu", "Menú").t(), Action::Menu);
}
