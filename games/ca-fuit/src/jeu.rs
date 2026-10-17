//! La partie, en 2D : les sprites des cartes (photographiés depuis les
//! modèles 3D, voir `sprites.rs`), la main, la pose, l'eau qui avance, les
//! flaques, le curseur et la caméra qui dézoome. Les règles viennent de
//! `fuite_core`.
//!
//! Une case fait 1 × 1 ; la case (x, y) de la grille est au monde en (x, y) :
//! le nord de la grille est le haut de l'écran.
use std::collections::HashMap;

use bevy::camera::ScalingMode;
use bevy::input::mouse::MouseWheel;
use bevy::picking::hover::Hovered;
use bevy::prelude::*;
use fuite_core::{Carte, Case, Direction, Forme, Partie, Refus, Rotation};
use game_common::ambiance::{Ambiance, Saison};
use game_common::{Rng, palette};

use crate::GameState;

pub struct JeuPlugin;

impl Plugin for JeuPlugin {
    fn build(&self, app: &mut App) {
        let record = game_common::save::load("ca-fuit", "record").and_then(|v| v.parse().ok()).unwrap_or(0);
        app.insert_resource(Record(record))
            .insert_resource(Jeu::nouveau(Rng::from_time().next_u64()))
            .insert_resource(Cadrage::default())
            .add_message::<Evenement>()
            .add_systems(Startup, setup)
            // On range la scène en revenant au menu ou en recommençant : au
            // bilan, le réseau reste visible.
            .add_systems(OnEnter(GameState::Playing), (ranger, commencer).chain())
            .add_systems(OnEnter(GameState::Menu), ranger)
            .add_systems(
                Update,
                (
                    (viser, agir, debloquer, fin_de_partie, apercu).chain().run_if(in_state(GameState::Playing)),
                    (rebatir, couler, deborder, cadrer, saison).run_if(en_jeu),
                    zoomer,
                ),
            );
    }
}

/// En partie ou au bilan : la scène vit.
fn en_jeu(state: Res<State<GameState>>) -> bool {
    matches!(state.get(), GameState::Playing | GameState::Fin)
}

/// Ce qui vient de se passer (le son et l'interface l'écoutent).
#[derive(Message, Clone, Copy, PartialEq, Eq, Debug)]
pub enum Evenement {
    Posee,
    Tournee,
    Refusee,
    /// Rien n'était posable : une carte de plus en main.
    Piochee,
    Deborde,
    Bouchee,
    Finie,
}

/// Le meilleur score, gardé par `game_common::save`.
#[derive(Resource)]
pub struct Record(pub u32);

/// Les sprites, chargés une fois.
#[derive(Resource)]
pub struct Images {
    /// Pour chaque forme (de référence) : la carte vide, puis pleine d'eau.
    cartes: HashMap<Forme, (Handle<Image>, Handle<Image>)>,
    fuite: Handle<Image>,
    flaque: Handle<Image>,
    arbre: Handle<Image>,
    touffe: Handle<Image>,
    touffe_fleur: Handle<Image>,
}

impl Images {
    /// L'image d'une forme, vide ; une forme non canonique est la même carte
    /// retournée : elle s'affiche avec un demi-tour de plus.
    pub fn carte(&self, forme: Forme) -> (Handle<Image>, Handle<Image>, Rotation) {
        let canonique = forme.canonique();
        let (vide, eau) = self.cartes[&canonique].clone();
        (vide, eau, if canonique == forme { Rotation::R0 } else { Rotation::R180 })
    }
}

/// L'état de la partie en cours, et ce que le joueur est en train de faire.
#[derive(Resource)]
pub struct Jeu {
    pub partie: Partie,
    /// La carte de la main choisie, et sa rotation.
    pub choix: usize,
    pub rotation: Rotation,
    /// La case visée (souris ou flèches).
    pub curseur: Option<Case>,
    /// Le moment (temps du jeu) où l'eau a fini de remplir chaque carte.
    pub plein: Vec<f32>,
    /// La partie est finie : on attend que l'eau finisse de couler.
    pub finie_depuis: Option<f32>,
    /// Les cartes ont changé : à redessiner.
    pub a_redessiner: bool,
    /// Le paquet ou la main ont changé : l'interface se met à jour.
    pub main_changee: bool,
}

impl Jeu {
    pub fn nouveau(graine: u64) -> Jeu {
        Jeu {
            partie: Partie::nouvelle(graine),
            choix: 0,
            rotation: Rotation::R0,
            curseur: None,
            plein: vec![0.0],
            finie_depuis: None,
            a_redessiner: true,
            main_changee: true,
        }
    }

    /// La carte qu'on s'apprête à poser au curseur, et si c'est possible.
    pub fn apercu(&self) -> Option<(Carte, Result<(), Refus>)> {
        let forme = *self.partie.main.get(self.choix)?;
        let carte = Carte::new(forme, self.rotation, self.curseur?);
        Some((carte, self.partie.grille.verifier(&carte)))
    }

    pub fn choisir(&mut self, i: usize) {
        if i < self.partie.main.len() {
            self.choix = i;
            self.main_changee = true;
        }
    }

    /// Après une pose : la main a changé, on garde une carte choisie valable.
    fn posees_apres(&mut self, choix: usize) {
        let n = self.partie.main.len();
        self.choix = if n == 0 { 0 } else { choix.min(n - 1) };
        self.a_redessiner = true;
        self.main_changee = true;
    }
}

/// Le cadrage voulu de la caméra, vers lequel elle glisse.
#[derive(Resource)]
pub struct Cadrage {
    pub cible: Vec2,
    pub largeur: f32,
    pub hauteur: f32,
    /// Zoom manuel à la molette (1 = automatique) ; revient à 1 à la pose suivante.
    pub zoom: f32,
    courant: (Vec2, f32, f32),
}

impl Default for Cadrage {
    fn default() -> Self {
        Cadrage { cible: Vec2::new(0.5, 0.0), largeur: 14.0, hauteur: 11.0, zoom: 1.0, courant: (Vec2::new(0.5, 0.0), 14.0, 11.0) }
    }
}

#[derive(Component)]
struct CameraJeu;

/// Tout ce qui est dessiné pour une carte posée (indice dans la grille).
#[derive(Component)]
struct DeLaCarte(usize);

/// L'eau d'une carte : le sprite plein, qui apparaît par fondu.
#[derive(Component)]
struct EauDeLaCarte(usize);

#[derive(Component, Clone)]
struct DeLaGrille;

/// L'aperçu de la carte au curseur : une seule entité, déplacée.
#[derive(Component)]
struct Apercu;

#[derive(Component)]
struct Sol;

/// Un élément du décor, avec la case qu'il occupe (il s'efface si on y pose un tuyau).
#[derive(Component)]
struct Decor(Case);

/// La flaque qui déborde d'une ouverture (case vide, direction d'où vient l'eau).
#[derive(Component)]
struct Debordement(Case, Direction, f32);

pub fn monde(case: Case) -> Vec2 {
    Vec2::new(case.0 as f32, case.1 as f32)
}

pub fn case_sous(p: Vec2) -> Case {
    (p.x.round() as i32, p.y.round() as i32)
}

/// La rotation d'un sprite pour une carte : quarts de tour dans le sens des
/// aiguilles d'une montre.
fn quat(rotation: Rotation) -> Quat {
    Quat::from_rotation_z(-(rotation.quarts() as f32) * std::f32::consts::FRAC_PI_2)
}

fn projection(largeur: f32, hauteur: f32) -> Projection {
    Projection::from(OrthographicProjection { scaling_mode: ScalingMode::AutoMin { min_width: largeur, min_height: hauteur }, ..OrthographicProjection::default_2d() })
}

fn setup(mut commands: Commands, server: Res<AssetServer>) {
    commands.spawn((Camera2d, projection(14.0, 11.0), Transform::from_xyz(0.5, 0.0, 0.0), CameraJeu, IsDefaultUiCamera));
    let charge = |nom: &str| server.load(format!("sprites/ca-fuit/{nom}.png"));
    let mut cartes = HashMap::new();
    for f in Forme::toutes() {
        cartes.insert(f, (charge(&format!("carte-{}", f.nom())), charge(&format!("carte-{}-eau", f.nom()))));
    }
    let images = Images { cartes, fuite: charge("fuite"), flaque: charge("flaque"), arbre: charge("arbre"), touffe: charge("touffe"), touffe_fleur: charge("touffe-fleur") };
    // La pâture : l'herbe, des arbres, des touffes.
    commands.spawn((Sprite { color: palette::GRASS, custom_size: Some(Vec2::splat(400.0)), ..default() }, Transform::from_xyz(0.0, 0.0, -2.0), Sol));
    for (x, y, t) in [(-9, 7, 1.2), (11, 6, 1.0), (-10, -6, 0.9), (12, -7, 1.3), (-14, 1, 1.1), (15, 0, 0.9), (2, 11, 1.0), (-3, -11, 1.1)] {
        commands.spawn((
            Sprite { image: images.arbre.clone(), custom_size: Some(Vec2::splat(3.0 * t)), ..default() },
            Transform::from_translation(monde((x, y)).extend(0.5)),
            Decor((x, y)),
        ));
    }
    for (i, (x, y)) in [(-6, 4), (7, 3), (-4, -5), (5, -6), (-8, -2), (9, -3), (3, 7), (-2, 8), (6, 8), (-7, -8), (10, 2), (-11, 4), (0, -9), (13, 4), (-12, -4)].into_iter().enumerate() {
        let image = if i % 3 == 0 { images.touffe_fleur.clone() } else { images.touffe.clone() };
        commands.spawn((
            Sprite { image, custom_size: Some(Vec2::splat(1.0)), ..default() },
            Transform::from_translation(monde((x, y)).extend(0.3)).with_rotation(Quat::from_rotation_z(i as f32 * 1.3)),
            Decor((x, y)),
        ));
    }
    // L'aperçu, invisible tant qu'il n'y a rien à montrer.
    commands.spawn((Sprite { custom_size: Some(Vec2::new(1.0, 2.0)), ..default() }, Transform::from_xyz(0.0, 0.0, 3.0), Visibility::Hidden, Apercu));
    commands.insert_resource(images);
}

/// L'herbe prend la couleur de la saison.
fn saison(ambiance: Option<Res<Ambiance>>, mut sol: Query<&mut Sprite, With<Sol>>) {
    let Some(a) = ambiance else { return };
    let couleur = match a.saison {
        Saison::Printemps => palette::GRASS_LIGHT,
        Saison::Ete => palette::GRASS,
        Saison::Automne => Color::srgb(0.76, 0.72, 0.42),
        Saison::Hiver => Color::srgb(0.90, 0.92, 0.94),
    };
    for mut s in &mut sol {
        if s.color != couleur {
            s.color = couleur;
        }
    }
}

/// Une nouvelle partie à chaque entrée dans le jeu.
fn commencer(mut jeu: ResMut<Jeu>, mut cadrage: ResMut<Cadrage>) {
    *jeu = Jeu::nouveau(Rng::from_time().next_u64());
    *cadrage = Cadrage::default();
}

fn ranger(mut commands: Commands, q: Query<Entity, Or<(With<DeLaCarte>, With<DeLaGrille>, With<Debordement>)>>, mut apercu: Query<&mut Visibility, With<Apercu>>) {
    for e in &q {
        commands.entity(e).despawn();
    }
    for mut v in &mut apercu {
        *v = Visibility::Hidden;
    }
}

/// La case sous la souris.
fn case_sous_la_souris(camera: &Camera, cam_tf: &GlobalTransform, ecran: Vec2) -> Option<Case> {
    camera.viewport_to_world_2d(cam_tf, ecran).ok().map(case_sous)
}

/// Le pointeur est-il sur un élément d'interface (la main, les panneaux…) ?
fn sur_l_interface(survols: &Query<&Hovered>) -> bool {
    survols.iter().any(|h| h.get())
}

fn viser(
    windows: Query<&Window>,
    cam: Query<(&Camera, &GlobalTransform), With<CameraJeu>>,
    keys: Res<ButtonInput<KeyCode>>,
    survols: Query<&Hovered>,
    mut jeu: ResMut<Jeu>,
) {
    if !sur_l_interface(&survols) {
        let Ok((camera, cam_tf)) = cam.single() else { return };
        if let Some(p) = windows.single().ok().and_then(|w| w.cursor_position()) {
            let case = case_sous_la_souris(camera, cam_tf, p);
            if case.is_some() && case != jeu.curseur {
                jeu.curseur = case;
            }
        }
    }
    let mut d = (0, 0);
    if keys.just_pressed(KeyCode::ArrowUp) { d.1 += 1; }
    if keys.just_pressed(KeyCode::ArrowDown) { d.1 -= 1; }
    if keys.just_pressed(KeyCode::ArrowLeft) { d.0 -= 1; }
    if keys.just_pressed(KeyCode::ArrowRight) { d.0 += 1; }
    if d != (0, 0) {
        let c = jeu.curseur.unwrap_or((0, 2));
        jeu.curseur = Some((c.0 + d.0, c.1 + d.1));
    }
}

/// Choisir, tourner, poser.
#[allow(clippy::too_many_arguments)]
fn agir(
    keys: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    survols: Query<&Hovered>,
    time: Res<Time>,
    mut jeu: ResMut<Jeu>,
    mut cadrage: ResMut<Cadrage>,
    mut evenements: MessageWriter<Evenement>,
) {
    if jeu.finie_depuis.is_some() {
        return;
    }
    for (i, k) in [KeyCode::Digit1, KeyCode::Digit2, KeyCode::Digit3, KeyCode::Digit4, KeyCode::Digit5, KeyCode::Digit6, KeyCode::Digit7, KeyCode::Digit8, KeyCode::Digit9].into_iter().enumerate() {
        if keys.just_pressed(k) {
            jeu.choisir(i);
        }
    }
    if keys.just_pressed(KeyCode::Tab) {
        let n = jeu.partie.main.len();
        let i = (jeu.choix + 1) % n.max(1);
        jeu.choisir(i);
    }
    let sur_ui = sur_l_interface(&survols);
    if keys.just_pressed(KeyCode::KeyR) || (mouse.just_pressed(MouseButton::Right) && !sur_ui) {
        jeu.rotation = jeu.rotation.suivante();
        jeu.main_changee = true;
        evenements.write(Evenement::Tournee);
    }
    let poser = keys.any_just_pressed([KeyCode::Enter, KeyCode::Space]) || (mouse.just_pressed(MouseButton::Left) && !sur_ui);
    if !poser {
        return;
    }
    let Some((carte, verdict)) = jeu.apercu() else { return };
    if verdict.is_err() {
        evenements.write(Evenement::Refusee);
        return;
    }
    let (rotation, ancre, choix) = (carte.rotation, carte.ancre, jeu.choix);
    let avant_bouchee = jeu.partie.grille.bouchee();
    if jeu.partie.poser(choix, rotation, ancre).is_err() {
        evenements.write(Evenement::Refusee);
        return;
    }
    // L'eau arrive quand la carte voisine qui l'alimente est pleine.
    let i = jeu.partie.grille.cartes().len() - 1;
    let alimentation = jeu
        .partie
        .grille
        .cartes()[i]
        .ouvertures()
        .filter_map(|(case, dir)| {
            let v = dir.voisine(case);
            let j = jeu.partie.grille.cartes().iter().position(|c| c.cases().contains(&v))?;
            (jeu.partie.grille.cote(v, dir.opposee()) == Some(true)).then_some(jeu.plein[j])
        })
        .fold(f32::INFINITY, f32::min);
    let debut = time.elapsed_secs().max(alimentation);
    jeu.plein.push(debut + DUREE_REMPLISSAGE);
    jeu.posees_apres(choix);
    cadrage.zoom = 1.0;
    evenements.write(Evenement::Posee);
    if !avant_bouchee && jeu.partie.grille.bouchee() {
        evenements.write(Evenement::Bouchee);
    }
}

/// Le temps pour que l'eau remplisse une carte.
const DUREE_REMPLISSAGE: f32 = 0.8;

/// Rien n'est posable : la main grandit d'une carte (une par image, pour
/// qu'on la voie arriver), jusqu'à ce qu'une carte soit posable. Le calcul
/// n'est refait que quand la main ou la grille ont changé.
fn debloquer(mut jeu: ResMut<Jeu>, mut evenements: MessageWriter<Evenement>, mut verifiee: Local<bool>) {
    if jeu.finie_depuis.is_some() {
        return;
    }
    if jeu.main_changee {
        *verifiee = false;
    }
    if *verifiee {
        return;
    }
    *verifiee = true;
    if !jeu.partie.bloquee() {
        return;
    }
    if jeu.partie.piocher() {
        jeu.main_changee = true;
        *verifiee = false;
        evenements.write(Evenement::Piochee);
    }
}

fn fin_de_partie(time: Res<Time>, mut jeu: ResMut<Jeu>, mut record: ResMut<Record>, mut next: ResMut<NextState<GameState>>, mut evenements: MessageWriter<Evenement>) {
    let t = time.elapsed_secs();
    if jeu.finie_depuis.is_none() && jeu.main_changee && jeu.partie.terminee() {
        jeu.finie_depuis = Some(t);
        let score = jeu.partie.score();
        if score.points > record.0 {
            record.0 = score.points;
            game_common::save::store("ca-fuit", "record", &record.0.to_string());
        }
        evenements.write(Evenement::Finie);
    }
    // On laisse l'eau finir de couler avant le bilan.
    let fin_eau = jeu.plein.iter().copied().fold(0.0, f32::max);
    if let Some(depuis) = jeu.finie_depuis
        && t >= (depuis + 1.2).max(fin_eau + 0.4)
    {
        next.set(GameState::Fin);
    }
}

/// Le sprite d'une carte posée : l'image vide, et par-dessus l'image pleine
/// d'eau, transparente tant que l'eau n'est pas arrivée.
fn dessiner_carte(commands: &mut Commands, images: &Images, i: usize, carte: &Carte) {
    let (vide, eau, demi_tour) = images.carte(carte.forme);
    let rotation = Rotation::de_quarts(carte.rotation.quarts() + demi_tour.quarts());
    let cases = carte.cases();
    let centre = (monde(cases[0]) + monde(cases[1])) / 2.0;
    let taille = Some(Vec2::new(1.0, 2.0));
    commands.spawn((Sprite { image: vide, custom_size: taille, ..default() }, Transform::from_translation(centre.extend(1.0)).with_rotation(quat(rotation)), DeLaCarte(i))).with_child((
        Sprite { image: eau, custom_size: taille, color: Color::WHITE.with_alpha(0.0), ..default() },
        Transform::from_xyz(0.0, 0.0, 0.1),
        EauDeLaCarte(i),
    ));
}

/// Dessine les cartes qui ne le sont pas encore, et la grille autour.
#[allow(clippy::too_many_arguments)]
fn rebatir(
    mut commands: Commands,
    mut jeu: ResMut<Jeu>,
    images: Res<Images>,
    dessinees: Query<&DeLaCarte>,
    grille: Query<Entity, With<DeLaGrille>>,
    decor: Query<(Entity, &Decor)>,
    debordements: Query<(Entity, &Debordement)>,
    mut cadrage: ResMut<Cadrage>,
) {
    if !jeu.a_redessiner {
        return;
    }
    jeu.a_redessiner = false;
    let deja: Vec<usize> = dessinees.iter().map(|d| d.0).collect();
    for (i, carte) in jeu.partie.grille.cartes().iter().enumerate() {
        if deja.contains(&i) {
            continue;
        }
        if i == 0 {
            // La fuite : la terre retournée dessous, puis la carte toute ouverte.
            let centre = (monde(carte.cases()[0]) + monde(carte.cases()[1])) / 2.0;
            commands.spawn((Sprite { image: images.fuite.clone(), custom_size: Some(Vec2::new(4.0, 3.0)), ..default() }, Transform::from_translation(centre.extend(0.6)), DeLaCarte(0)));
        } else {
            dessiner_carte(&mut commands, &images, i, carte);
        }
        // Le décor et les flaques qui étaient là s'effacent.
        for (e, d) in &decor {
            if carte.cases().contains(&d.0) {
                commands.entity(e).despawn();
            }
        }
        for (e, d) in &debordements {
            if carte.cases().contains(&d.0) {
                commands.entity(e).despawn();
            }
        }
    }
    for e in &grille {
        commands.entity(e).despawn();
    }
    let (min, max) = jeu.partie.grille.bornes();
    let marge = 2;
    let (min, max) = ((min.0 - marge, min.1 - marge), (max.0 + marge, max.1 + marge));
    let ligne = Color::srgba(1.0, 1.0, 1.0, 0.18);
    let (x0, x1) = (min.0 as f32 - 0.5, max.0 as f32 + 0.5);
    let (y0, y1) = (min.1 as f32 - 0.5, max.1 as f32 + 0.5);
    for x in min.0..=max.0 + 1 {
        commands.spawn((Sprite { color: ligne, custom_size: Some(Vec2::new(0.03, y1 - y0)), ..default() }, Transform::from_xyz(x as f32 - 0.5, (y0 + y1) / 2.0, -1.0), DeLaGrille));
    }
    for y in min.1..=max.1 + 1 {
        commands.spawn((Sprite { color: ligne, custom_size: Some(Vec2::new(x1 - x0, 0.03)), ..default() }, Transform::from_xyz((x0 + x1) / 2.0, y as f32 - 0.5, -1.0), DeLaGrille));
    }
    // La caméra cadre tout le réseau, avec de la place pour la main en bas.
    let (a, b) = (monde(jeu.partie.grille.bornes().0), monde(jeu.partie.grille.bornes().1));
    let centre = (a + b) / 2.0;
    cadrage.cible = centre - Vec2::Y * 0.8;
    cadrage.largeur = ((b.x - a.x).abs() + 7.0).max(14.0);
    cadrage.hauteur = ((b.y - a.y).abs() + 7.0).max(11.0);
}

/// L'aperçu de la carte au curseur : vert si la pose est possible, rouge
/// sinon. Une seule entité, mise à jour sans être recréée.
fn apercu(jeu: Res<Jeu>, images: Res<Images>, mut q: Query<(&mut Sprite, &mut Transform, &mut Visibility), With<Apercu>>) {
    let Ok((mut sprite, mut tf, mut vis)) = q.single_mut() else { return };
    let voulu = if jeu.finie_depuis.is_none() { jeu.apercu() } else { None };
    let Some((carte, verdict)) = voulu else {
        *vis = Visibility::Hidden;
        return;
    };
    *vis = Visibility::Inherited;
    let (vide, _, demi_tour) = images.carte(carte.forme);
    if sprite.image != vide {
        sprite.image = vide;
    }
    sprite.color = if verdict.is_ok() { Color::srgba(0.45, 1.0, 0.5, 0.75) } else { Color::srgba(1.0, 0.35, 0.3, 0.75) };
    let cases = carte.cases();
    let centre = (monde(cases[0]) + monde(cases[1])) / 2.0;
    tf.translation = centre.extend(3.0);
    tf.rotation = quat(Rotation::de_quarts(carte.rotation.quarts() + demi_tour.quarts()));
}

/// L'eau apparaît dans chaque tuyau, tuyau par tuyau.
fn couler(time: Res<Time>, jeu: Res<Jeu>, mut eaux: Query<(&EauDeLaCarte, &mut Sprite)>) {
    let t = time.elapsed_secs();
    for (e, mut sprite) in &mut eaux {
        let Some(&plein) = jeu.plein.get(e.0) else { continue };
        let niveau = ((t - (plein - DUREE_REMPLISSAGE)) / DUREE_REMPLISSAGE).clamp(0.0, 1.0);
        if sprite.color.alpha() != niveau {
            sprite.color = Color::WHITE.with_alpha(niveau);
        }
    }
}

/// Les ouvertures pleines qui donnent sur du vide débordent : une flaque
/// apparaît et s'étale.
fn deborder(
    mut commands: Commands,
    time: Res<Time>,
    jeu: Res<Jeu>,
    images: Res<Images>,
    mut flaques: Query<(&mut Debordement, &mut Sprite, &mut Transform)>,
    mut evenements: MessageWriter<Evenement>,
) {
    let t = time.elapsed_secs();
    let existantes: Vec<(Case, Direction)> = flaques.iter().map(|(d, _, _)| (d.0, d.1)).collect();
    for (i, carte) in jeu.partie.grille.cartes().iter().enumerate() {
        if jeu.plein.get(i).is_none_or(|&p| t < p) {
            continue;
        }
        for (case, dir) in carte.ouvertures() {
            let vide = dir.voisine(case);
            if jeu.partie.grille.libre(vide) && !existantes.contains(&(vide, dir)) {
                // Au bord de la case vide, côté tuyau, puis elle s'étale.
                let (dx, dy) = dir.delta();
                let pos = monde(vide) - Vec2::new(dx as f32, dy as f32) * 0.2;
                commands.spawn((
                    Sprite { image: images.flaque.clone(), custom_size: Some(Vec2::splat(0.05)), ..default() },
                    Transform::from_translation(pos.extend(0.4)),
                    Debordement(vide, dir, 0.05),
                ));
                evenements.write(Evenement::Deborde);
            }
        }
    }
    for (mut d, mut sprite, _) in &mut flaques {
        if d.2 < 0.95 {
            d.2 = (d.2 + time.delta_secs() * 0.25).min(0.95);
            sprite.custom_size = Some(Vec2::splat(d.2));
        }
    }
}

/// La caméra glisse vers le cadrage voulu.
fn cadrer(time: Res<Time>, mut cadrage: ResMut<Cadrage>, mut cam: Query<(&mut Transform, &mut Projection), With<CameraJeu>>) {
    let k = (time.delta_secs() * 4.0).min(1.0);
    let (cible, largeur, hauteur) = (cadrage.cible, cadrage.largeur * cadrage.zoom, cadrage.hauteur * cadrage.zoom);
    let c = &mut cadrage.courant;
    c.0 = c.0.lerp(cible, k);
    c.1 += (largeur - c.1) * k;
    c.2 += (hauteur - c.2) * k;
    for (mut tf, mut proj) in &mut cam {
        tf.translation = c.0.extend(0.0);
        *proj = projection(c.1, c.2);
    }
}

fn zoomer(mut molette: MessageReader<MouseWheel>, mut cadrage: ResMut<Cadrage>) {
    for m in molette.read() {
        let pas = if m.y > 0.0 { 0.85 } else { 1.0 / 0.85 };
        cadrage.zoom = (cadrage.zoom * pas).clamp(0.4, 2.5);
    }
}

/// Pour la capture : joue des coups au hasard. Avec `ouvert`, préfère les
/// cartes à plusieurs sorties, pour que le réseau grandisse au lieu de se boucher.
pub fn jouer_au_hasard(jeu: &mut Jeu, coups: usize, rng: &mut fuite_core::Rng, temps: f32, ouvert: bool) {
    for _ in 0..coups {
        jeu.partie.debloquer();
        let mut posables: Vec<usize> = (0..jeu.partie.main.len()).filter(|&i| jeu.partie.posable(i)).collect();
        if posables.is_empty() {
            break;
        }
        if ouvert {
            let max = posables.iter().map(|&i| jeu.partie.main[i].nb_sorties()).max().unwrap_or(1);
            posables.retain(|&i| jeu.partie.main[i].nb_sorties() == max);
        }
        let i = posables[rng.below(posables.len())];
        let poses = jeu.partie.grille.poses_possibles(jeu.partie.main[i]);
        let pose = poses[rng.below(poses.len())];
        if jeu.partie.poser(i, pose.rotation, pose.ancre).is_ok() {
            jeu.plein.push(temps);
            jeu.posees_apres(i);
        }
    }
}
