//! L'écran du lanceur : l'illustration du studio en fond, un panneau à gauche
//! (titre, mises à jour, pied de menu commun) et, à droite, les jeux rangés
//! par univers. Chaque carte a un bouton « Installer » ou « Jouer », et
//! « Mettre à jour » quand une nouvelle version est publiée.
use bevy::asset::RenderAssetUsages;
use bevy::image::{CompressedImageFormats, ImageSampler, ImageType};
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use game_common::brand::{self, colors};
use game_common::langue::Texte;
use game_common::tx;
use game_common::ui::{ButtonColor, GameHelp, ThemedPanel, UiTheme, pied_de_menu, spawn_button, spawn_button_sized, spawn_text};
use game_common::universe::{UiStyle, Universe};

use crate::GameState;
use crate::catalogue::{JEUX, LANCEUR, UNIVERS};
use crate::installation::{self, Catalogue, Etat, Partage, Tache};

const AIDE: Texte = tx!(
    "Le lanceur installe les jeux du studio sur ce PC, les met à jour et les lance.\n\n\
     Installer : télécharge le jeu (une connexion Internet est nécessaire).\n\
     Jouer : ouvre le jeu. Il est aussi dans le menu d'applications, catégorie Jeux.\n\
     Mettre à jour : une nouvelle version est publiée.\n\n\
     Les jeux sont rangés dans ~/.local/share/commit-and-conquer.\n\
     Molette : faire défiler · M : son",
    "The launcher installs the studio's games on this PC, updates them and starts them.\n\n\
     Install: downloads the game (an Internet connection is needed).\n\
     Play: opens the game. It is also in the applications menu, under Games.\n\
     Update: a new version has been released.\n\n\
     The games are stored in ~/.local/share/commit-and-conquer.\n\
     Mouse wheel: scroll · M: sound",
    "Der Launcher installiert die Spiele des Studios auf diesem PC, aktualisiert und startet sie.\n\n\
     Installieren: lädt das Spiel herunter (Internetverbindung nötig).\n\
     Spielen: öffnet das Spiel. Es steht auch im Anwendungsmenü unter Spiele.\n\
     Aktualisieren: eine neue Version ist erschienen.\n\n\
     Die Spiele liegen in ~/.local/share/commit-and-conquer.\n\
     Mausrad: blättern · M: Ton",
    "Il launcher installa i giochi dello studio su questo PC, li aggiorna e li avvia.\n\n\
     Installa: scarica il gioco (serve una connessione Internet).\n\
     Gioca: apre il gioco. Si trova anche nel menu delle applicazioni, in Giochi.\n\
     Aggiorna: è uscita una nuova versione.\n\n\
     I giochi si trovano in ~/.local/share/commit-and-conquer.\n\
     Rotellina: scorrere · M: audio",
    "El lanzador instala los juegos del estudio en este PC, los actualiza y los abre.\n\n\
     Instalar: descarga el juego (hace falta conexión a Internet).\n\
     Jugar: abre el juego. También está en el menú de aplicaciones, en Juegos.\n\
     Actualizar: hay una versión nueva.\n\n\
     Los juegos se guardan en ~/.local/share/commit-and-conquer.\n\
     Rueda del ratón: desplazar · M: sonido",
);

/// L'interface aux couleurs du studio (charte), par-dessus le style de Nova.
pub const STYLE: UiStyle = UiStyle {
    font: game_common::sounds::FONT_NOVA,
    panel: Color::srgba(0.078, 0.172, 0.251, 0.90),
    panel_border: colors::BLUE,
    text: IVOIRE,
    button: Color::srgba(0.11, 0.23, 0.33, 0.96),
    button_hover: colors::BLUE,
    button_border: CIEL,
    accent: colors::GREEN,
};

const IVOIRE: Color = colors::IVORY;
const CIEL: Color = colors::SKY;
const NUIT: Color = colors::NIGHT;
const CARTE: Color = Color::srgba(0.078, 0.172, 0.251, 0.95);

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameHelp(AIDE))
            .insert_resource(UiTheme(STYLE))
            .insert_resource(ClearColor(NUIT))
            .init_resource::<Lances>()
            .add_systems(Startup, camera)
            .add_systems(OnEnter(GameState::Menu), spawn_ecran)
            .add_systems(Update, (cadrer_le_fond, defiler, maj_cartes, maj_panneau, relancer))
            .add_observer(au_clic);
    }
}

fn camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

/// Les icônes des jeux, intégrées au lanceur.
#[derive(Resource)]
struct Icones(Vec<Handle<Image>>);

/// Prêtes dès la construction de l'écran (le premier `OnEnter` passe avant `Startup`).
impl FromWorld for Icones {
    fn from_world(world: &mut World) -> Self {
        let mut images = world.resource_mut::<Assets<Image>>();
        let icones = JEUX
            .iter()
            .map(|j| {
                let image =
                    Image::from_buffer(j.icone, ImageType::Extension("png"), CompressedImageFormats::NONE, true, ImageSampler::linear(), RenderAssetUsages::RENDER_WORLD)
                        .unwrap_or_default();
                images.add(image)
            })
            .collect();
        Icones(icones)
    }
}

#[derive(Component)]
struct Fond;

#[derive(Component)]
struct Defilement;

/// Bouton principal d'une carte (Installer / Jouer), avec son texte.
#[derive(Component)]
struct BoutonJeu {
    jeu: usize,
    texte: Entity,
}

/// Bouton « Mettre à jour » d'une carte.
#[derive(Component)]
struct BoutonMaj {
    jeu: usize,
    texte: Entity,
}

/// Ligne d'état d'une carte (taille, version, progression…).
#[derive(Component)]
struct LigneEtat(usize);

#[derive(Component)]
struct ToutMettreAJour;

#[derive(Component)]
struct MajLanceur;

#[derive(Component)]
struct TexteCatalogue;

/// Moment du dernier clic sur « Jouer » (le jeu met un instant à s'ouvrir).
#[derive(Resource, Default)]
struct Lances(Vec<(usize, f32)>);

fn couleur_univers(u: Universe) -> Color {
    match u {
        Universe::Croa => colors::GREEN,
        Universe::Nova => colors::BLUE,
        Universe::Eureka => colors::CORAL,
    }
}

fn spawn_ecran(mut commands: Commands, server: Res<AssetServer>, icones: Local<Icones>) {
    let racine = commands
        // Bornée à la fenêtre : sinon la liste des jeux l'agrandirait au lieu de défiler.
        .spawn((
            Node { width: Val::Percent(100.0), height: Val::Percent(100.0), min_height: Val::Px(0.0), overflow: Overflow::clip(), ..default() },
            DespawnOnExit(GameState::Menu),
        ))
        .id();

    // L'illustration du studio couvre l'écran, voilée de bleu nuit.
    let cadre = commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                overflow: Overflow::clip(),
                ..default()
            },
            Pickable::IGNORE,
            ChildOf(racine),
        ))
        .id();
    commands.spawn((Node { flex_shrink: 0.0, ..default() }, ImageNode::new(server.load(brand::SPLASH_IMAGE)), Fond, Pickable::IGNORE, ChildOf(cadre)));
    commands.spawn((
        Node { position_type: PositionType::Absolute, width: Val::Percent(100.0), height: Val::Percent(100.0), ..default() },
        BackgroundColor(NUIT.with_alpha(0.45)),
        Pickable::IGNORE,
        ChildOf(racine),
    ));

    let page = commands
        .spawn((
            Node { width: Val::Percent(100.0), height: Val::Percent(100.0), padding: UiRect::all(Val::Px(24.0)), column_gap: Val::Px(24.0), ..default() },
            Pickable::IGNORE,
            ChildOf(racine),
        ))
        .id();

    // À gauche : le panneau du lanceur.
    let panneau = commands
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                align_self: AlignSelf::Center,
                flex_shrink: 0.0,
                row_gap: Val::Px(14.0),
                padding: UiRect::axes(Val::Px(28.0), Val::Px(28.0)),
                border: UiRect::all(Val::Px(2.0)),
                border_radius: BorderRadius::all(Val::Px(24.0)),
                ..default()
            },
            BackgroundColor(STYLE.panel),
            BorderColor::all(STYLE.panel_border),
            ThemedPanel,
            ChildOf(page),
        ))
        .id();
    spawn_text(&mut commands, panneau, tx!("Les jeux", "The games", "Die Spiele", "I giochi", "Los juegos").t(), 44.0);
    let t = spawn_text(&mut commands, panneau, "", 17.0);
    commands.entity(t).insert((TexteCatalogue, TextColor(CIEL), Node { max_width: Val::Px(320.0), ..default() }));
    let (b, _) = spawn_button(&mut commands, panneau, tx!("Tout mettre à jour", "Update all", "Alle aktualisieren", "Aggiorna tutto", "Actualizar todo").t());
    commands.entity(b).insert((ToutMettreAJour, Node { display: Display::None, ..bouton_standard() }));
    let (b, _) = spawn_button(&mut commands, panneau, tx!("Mettre à jour le lanceur", "Update the launcher", "Launcher aktualisieren", "Aggiorna il launcher", "Actualizar el lanzador").t());
    commands.entity(b).insert((MajLanceur, ButtonColor(colors::LEMON), Node { display: Display::None, ..bouton_standard() }));
    pied_de_menu(&mut commands, panneau);

    // À droite : les jeux, par univers, dans une zone qui défile.
    let zone = commands
        .spawn((
            Node {
                flex_grow: 1.0,
                min_height: Val::Px(0.0),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(10.0),
                padding: UiRect::axes(Val::Px(8.0), Val::Px(4.0)),
                overflow: Overflow::scroll_y(),
                ..default()
            },
            ScrollPosition::default(),
            Defilement,
            ChildOf(page),
        ))
        .id();
    for univers in UNIVERS {
        let titre = commands
            .spawn((
                Node { align_items: AlignItems::Center, column_gap: Val::Px(10.0), margin: UiRect::top(Val::Px(8.0)), ..default() },
                Pickable::IGNORE,
                ChildOf(zone),
            ))
            .id();
        commands.spawn((
            Node { width: Val::Px(14.0), height: Val::Px(14.0), border_radius: BorderRadius::MAX, ..default() },
            BackgroundColor(couleur_univers(univers)),
            ChildOf(titre),
        ));
        let t = spawn_text(&mut commands, titre, univers.name(), 28.0);
        commands.entity(t).insert(TextColor(IVOIRE));
        let grille = commands
            .spawn((Node { flex_wrap: FlexWrap::Wrap, column_gap: Val::Px(14.0), row_gap: Val::Px(14.0), ..default() }, ChildOf(zone)))
            .id();
        for (i, jeu) in JEUX.iter().enumerate().filter(|(_, j)| j.univers == univers) {
            spawn_carte(&mut commands, grille, i, icones.0[i].clone(), couleur_univers(univers), jeu.nom.t(), jeu.description.t());
        }
    }
}

fn bouton_standard() -> Node {
    Node {
        width: Val::Px(320.0),
        height: Val::Px(56.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        border: UiRect::all(Val::Px(3.0)),
        border_radius: BorderRadius::all(Val::Px(14.0)),
        ..default()
    }
}

#[allow(clippy::too_many_arguments)]
fn spawn_carte(commands: &mut Commands, grille: Entity, i: usize, icone: Handle<Image>, couleur: Color, nom: &str, description: &str) {
    let carte = commands
        .spawn((
            Node {
                width: Val::Px(290.0),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(8.0),
                padding: UiRect::all(Val::Px(14.0)),
                border: UiRect::all(Val::Px(2.0)),
                border_radius: BorderRadius::all(Val::Px(18.0)),
                ..default()
            },
            BackgroundColor(CARTE),
            BorderColor::all(couleur.with_alpha(0.8)),
            ChildOf(grille),
        ))
        .id();
    let tete = commands.spawn((Node { align_items: AlignItems::Center, column_gap: Val::Px(12.0), ..default() }, Pickable::IGNORE, ChildOf(carte))).id();
    commands.spawn((
        Node { width: Val::Px(72.0), height: Val::Px(72.0), flex_shrink: 0.0, border_radius: BorderRadius::all(Val::Px(14.0)), ..default() },
        ImageNode::new(icone),
        Pickable::IGNORE,
        ChildOf(tete),
    ));
    let t = spawn_text(commands, tete, nom, 21.0);
    commands.entity(t).insert((TextColor(IVOIRE), TextLayout::justify(Justify::Left), Node { flex_shrink: 1.0, ..default() }));
    let d = spawn_text(commands, carte, description, 15.0);
    commands.entity(d).insert((TextColor(CIEL), TextLayout::justify(Justify::Left), Node { min_height: Val::Px(40.0), ..default() }));
    let e = spawn_text(commands, carte, "", 14.0);
    commands.entity(e).insert((LigneEtat(i), TextColor(IVOIRE.with_alpha(0.75)), TextLayout::justify(Justify::Left)));
    let boutons = commands.spawn((Node { column_gap: Val::Px(8.0), ..default() }, ChildOf(carte))).id();
    let (b, texte) = spawn_button_sized(commands, boutons, "", 150.0, 44.0, 19.0);
    commands.entity(b).insert((BoutonJeu { jeu: i, texte }, ButtonColor(colors::BLUE)));
    let (b, texte) = spawn_button_sized(commands, boutons, tx!("Mettre à jour", "Update", "Aktualisieren", "Aggiorna", "Actualizar").t(), 104.0, 44.0, 14.0);
    commands.entity(b).insert((BoutonMaj { jeu: i, texte }, ButtonColor(colors::LEMON)));
}

/// L'illustration garde ses proportions et couvre toute la fenêtre.
fn cadrer_le_fond(window: Query<&Window, With<PrimaryWindow>>, mut q: Query<&mut Node, With<Fond>>) {
    let Ok(w) = window.single() else { return };
    let large = w.width() / w.height().max(1.0) > brand::SPLASH_RATIO;
    for mut node in &mut q {
        let (largeur, hauteur) = if large {
            (Val::Px(w.width()), Val::Px(w.width() / brand::SPLASH_RATIO))
        } else {
            (Val::Px(w.height() * brand::SPLASH_RATIO), Val::Px(w.height()))
        };
        if node.width != largeur || node.height != hauteur {
            node.width = largeur;
            node.height = hauteur;
        }
    }
}

fn defiler(mut molette: MessageReader<MouseWheel>, mut q: Query<&mut ScrollPosition, With<Defilement>>) {
    for m in molette.read() {
        let dy = match m.unit {
            MouseScrollUnit::Line => m.y * 40.0,
            MouseScrollUnit::Pixel => m.y,
        };
        for mut pos in &mut q {
            pos.y = (pos.y - dy).max(0.0);
        }
    }
}

fn mega_octets(octets: u64) -> String {
    format!("{} Mo", (octets as f64 / 1_000_000.0).round().max(1.0))
}

/// Ce que montre une carte, d'après l'état partagé.
struct Vue {
    bouton: String,
    couleur: Color,
    maj: bool,
    ligne: String,
}

fn vue(etat: &Etat, id: &str, lance: bool) -> Vue {
    let installee = etat.installes.get(id);
    let distant = etat.distant(id);
    let bleu = |bouton: String, ligne: String| Vue { bouton, couleur: colors::BLUE, maj: false, ligne };
    match etat.taches.get(id) {
        Some(Tache::Telechargement(p)) => {
            return bleu(format!("{} %", (p * 100.0).round()), tx!("Téléchargement…", "Downloading…", "Wird geladen…", "Download in corso…", "Descargando…").t().into());
        }
        Some(Tache::Installation) => {
            return bleu("…".into(), tx!("Installation…", "Installing…", "Wird installiert…", "Installazione…", "Instalando…").t().into());
        }
        _ => {}
    }
    let echec = match etat.taches.get(id) {
        Some(Tache::Echec(e)) => Some(e.texte().t().to_string()),
        _ => None,
    };
    match installee {
        Some(v) => {
            let maj = etat.mise_a_jour(id);
            let ligne = if lance {
                tx!("Ouverture du jeu…", "Opening the game…", "Spiel wird geöffnet…", "Apertura del gioco…", "Abriendo el juego…").t().into()
            } else if let Some(e) = echec {
                e
            } else if let Some(d) = maj {
                tx!("Nouvelle version : {} ({})", "New version: {} ({})", "Neue Version: {} ({})", "Nuova versione: {} ({})", "Nueva versión: {} ({})")
                    .avec(&[&d.version, &mega_octets(d.taille)])
            } else if v == "?" {
                tx!("Installé", "Installed", "Installiert", "Installato", "Instalado").t().into()
            } else {
                tx!("Version {}", "Version {}", "Version {}", "Versione {}", "Versión {}").avec(&[v])
            };
            Vue { bouton: tx!("Jouer", "Play", "Spielen", "Gioca", "Jugar").t().into(), couleur: colors::GREEN, maj: maj.is_some(), ligne }
        }
        None => {
            let ligne = match (echec, distant, &etat.catalogue) {
                (Some(e), _, _) => e,
                (None, Some(d), _) => mega_octets(d.taille),
                (None, None, Some(Catalogue::HorsLigne)) => tx!("Hors ligne", "Offline", "Offline", "Non in linea", "Sin conexión").t().into(),
                (None, None, Some(Catalogue::Pret(_))) => tx!("Bientôt disponible", "Coming soon", "Bald verfügbar", "Presto disponibile", "Próximamente").t().into(),
                (None, None, _) => "…".into(),
            };
            bleu(tx!("Installer", "Install", "Installieren", "Installa", "Instalar").t().into(), ligne)
        }
    }
}

#[allow(clippy::type_complexity)]
fn maj_cartes(
    partage: Res<Partage>,
    time: Res<Time>,
    mut lances: ResMut<Lances>,
    mut boutons: Query<(&BoutonJeu, &mut ButtonColor, &mut BackgroundColor), Without<BoutonMaj>>,
    mut maj: Query<(&BoutonMaj, &mut Node)>,
    mut lignes: Query<(&LigneEtat, &mut Text)>,
    mut textes: Query<(&mut Text, &mut TextColor), Without<LigneEtat>>,
) {
    let maintenant = time.elapsed_secs();
    lances.0.retain(|(_, t)| maintenant - t < 4.0);
    let etat = partage.lire();
    let vues: Vec<Vue> = JEUX.iter().enumerate().map(|(i, j)| vue(&etat, j.id, lances.0.iter().any(|(k, _)| *k == i))).collect();
    drop(etat);
    for (b, mut couleur, mut fond) in &mut boutons {
        let v = &vues[b.jeu];
        if couleur.0 != v.couleur {
            couleur.0 = v.couleur;
            fond.0 = v.couleur;
        }
        if let Ok((mut t, mut c)) = textes.get_mut(b.texte) {
            if t.0 != v.bouton {
                t.0.clone_from(&v.bouton);
            }
            c.0 = if v.couleur == colors::BLUE { IVOIRE } else { NUIT };
        }
    }
    for (b, mut node) in &mut maj {
        let display = if vues[b.jeu].maj { Display::Flex } else { Display::None };
        if node.display != display {
            node.display = display;
        }
        if let Ok((_, mut c)) = textes.get_mut(b.texte) {
            c.0 = NUIT;
        }
    }
    for (l, mut t) in &mut lignes {
        if t.0 != vues[l.0].ligne {
            t.0.clone_from(&vues[l.0].ligne);
        }
    }
}

#[allow(clippy::type_complexity)]
fn maj_panneau(
    partage: Res<Partage>,
    mut texte: Query<&mut Text, With<TexteCatalogue>>,
    mut tout: Query<&mut Node, (With<ToutMettreAJour>, Without<MajLanceur>)>,
    mut lanceur: Query<(&mut Node, &Children), (With<MajLanceur>, Without<ToutMettreAJour>)>,
    mut couleurs: Query<&mut TextColor>,
) {
    let etat = partage.lire();
    let a_mettre_a_jour = JEUX.iter().filter(|j| etat.mise_a_jour(j.id).is_some() && !etat.occupe(j.id)).count();
    let lanceur_maj = etat.mise_a_jour(LANCEUR).is_some() || etat.occupe(LANCEUR);
    let message = match &etat.catalogue {
        None | Some(Catalogue::Recherche) => tx!("Recherche des nouveautés…", "Checking for news…", "Suche nach Neuem…", "Ricerca di novità…", "Buscando novedades…").t().into(),
        Some(Catalogue::HorsLigne) => tx!(
            "Hors ligne : seuls les jeux installés sont disponibles.",
            "Offline: only installed games are available.",
            "Offline: nur installierte Spiele sind verfügbar.",
            "Non in linea: solo i giochi installati sono disponibili.",
            "Sin conexión: solo están disponibles los juegos instalados.",
        )
        .t()
        .to_string(),
        Some(Catalogue::Pret(c)) => {
            let version = c.values().next().map(|d| d.version.clone()).unwrap_or_default();
            if a_mettre_a_jour == 1 {
                tx!("Version {} : 1 mise à jour", "Version {}: 1 update", "Version {}: 1 Aktualisierung", "Versione {}: 1 aggiornamento", "Versión {}: 1 actualización").avec(&[&version])
            } else if a_mettre_a_jour > 1 {
                tx!("Version {} : {} mises à jour", "Version {}: {} updates", "Version {}: {} Aktualisierungen", "Versione {}: {} aggiornamenti", "Versión {}: {} actualizaciones")
                    .avec(&[&version, &a_mettre_a_jour])
            } else {
                tx!("Version {} : tout est à jour", "Version {}: all up to date", "Version {}: alles aktuell", "Versione {}: tutto aggiornato", "Versión {}: todo al día").avec(&[&version])
            }
        }
    };
    drop(etat);
    for mut t in &mut texte {
        if t.0 != message {
            t.0.clone_from(&message);
        }
    }
    for mut node in &mut tout {
        let d = if a_mettre_a_jour > 1 { Display::Flex } else { Display::None };
        if node.display != d {
            node.display = d;
        }
    }
    for (mut node, enfants) in &mut lanceur {
        let d = if lanceur_maj { Display::Flex } else { Display::None };
        if node.display != d {
            node.display = d;
        }
        for e in enfants {
            if let Ok(mut c) = couleurs.get_mut(*e) {
                c.0 = NUIT;
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn au_clic(
    clic: On<PointerClick>,
    partage: Res<Partage>,
    time: Res<Time>,
    mut lances: ResMut<Lances>,
    jeux: Query<&BoutonJeu>,
    maj: Query<&BoutonMaj>,
    tout: Query<(), With<ToutMettreAJour>>,
    lanceur: Query<(), With<MajLanceur>>,
) {
    if let Ok(b) = jeux.get(clic.entity) {
        let id = JEUX[b.jeu].id;
        let installe = partage.lire().installes.contains_key(id);
        if installe {
            if installation::lancer(id) {
                lances.0.push((b.jeu, time.elapsed_secs()));
            }
        } else {
            installation::installer(&partage, id);
        }
    } else if let Ok(b) = maj.get(clic.entity) {
        installation::installer(&partage, JEUX[b.jeu].id);
    } else if tout.contains(clic.entity) {
        let ids: Vec<&'static str> = {
            let etat = partage.lire();
            JEUX.iter().map(|j| j.id).filter(|id| etat.mise_a_jour(id).is_some()).collect()
        };
        for id in ids {
            installation::installer(&partage, id);
        }
    } else if lanceur.contains(clic.entity) {
        installation::installer(&partage, LANCEUR);
    }
}

/// Le lanceur s'est mis à jour : la nouvelle version prend le relais.
fn relancer(partage: Res<Partage>, mut exit: MessageWriter<AppExit>) {
    let mut etat = partage.lire();
    if etat.relancer {
        etat.relancer = false;
        installation::relancer_le_lanceur();
        exit.write(AppExit::Success);
    }
}

