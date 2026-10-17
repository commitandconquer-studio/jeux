//! La partie : la grille 3D, la main, la pose, l'eau qui avance, les flaques,
//! le curseur et la caméra qui dézoome. Les règles viennent de `fuite_core`.
use bevy::input::mouse::MouseWheel;
use bevy::picking::hover::Hovered;
use bevy::prelude::*;
use fuite_core::{Carte, Case, Direction, Partie, Refus, Rotation};
use game_common::models::{FrogKit, spawn_frog};
use game_common::{ISO_OFFSET, Rng, iso_camera, iso_projection};

use crate::GameState;
use crate::modeles::{self, Decor, Eau, Flaque, Jet, Look, HAUTEUR};

pub struct JeuPlugin;

impl Plugin for JeuPlugin {
    fn build(&self, app: &mut App) {
        let record = game_common::save::load("ca-fuit", "record").and_then(|v| v.parse().ok()).unwrap_or(0);
        app.init_resource::<Look>()
            .init_resource::<FrogKit>()
            .insert_resource(Record(record))
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
                    (viser, agir, debloquer, fin_de_partie).chain().run_if(in_state(GameState::Playing)),
                    (rebatir, apercu, couler, deborder, cadrer, frissonner).run_if(en_jeu),
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
}

/// Le cadrage voulu de la caméra, vers lequel elle glisse.
#[derive(Resource)]
pub struct Cadrage {
    pub cible: Vec3,
    pub largeur: f32,
    pub hauteur: f32,
    /// Zoom manuel à la molette (1 = automatique) ; revient à 1 à la pose suivante.
    pub zoom: f32,
    courant: (Vec3, f32, f32),
}

impl Default for Cadrage {
    fn default() -> Self {
        Cadrage { cible: Vec3::new(0.5, 0.0, 0.0), largeur: 14.0, hauteur: 11.0, zoom: 1.0, courant: (Vec3::new(0.5, 0.0, 0.0), 14.0, 11.0) }
    }
}

#[derive(Component)]
struct CameraJeu;

/// Tout ce qui est dessiné pour une carte posée (indice dans la grille).
#[derive(Component)]
struct DeLaCarte(usize);

#[derive(Component, Clone)]
struct DeLaGrille;

#[derive(Component)]
struct DeLApercu;

#[derive(Component)]
struct Plombier;

/// La flaque qui déborde d'une ouverture (case vide, direction d'où vient l'eau).
#[derive(Component)]
struct Debordement(Case, Direction);

fn setup(mut commands: Commands, look: Res<Look>, kit: Res<FrogKit>, mut meshes: ResMut<Assets<Mesh>>) {
    commands.spawn((iso_camera(Vec3::new(0.5, 0.0, 0.0), 14.0, 11.0), CameraJeu));
    modeles::dessiner_pature(&mut commands, &look, &mut meshes);
    let crapaud = spawn_frog(&mut commands, &kit);
    commands.entity(crapaud).insert((Plombier, Transform::from_xyz(-1.6, 0.0, 1.5).with_scale(Vec3::splat(1.3)).looking_at(Vec3::new(0.5, 0.0, 0.0), Vec3::Y)));
    modeles::equiper_le_plombier(&mut commands, crapaud, &look);
}

/// Une nouvelle partie à chaque entrée dans le jeu.
fn commencer(mut jeu: ResMut<Jeu>, mut cadrage: ResMut<Cadrage>) {
    *jeu = Jeu::nouveau(Rng::from_time().next_u64());
    *cadrage = Cadrage::default();
}

fn ranger(mut commands: Commands, q: Query<Entity, Or<(With<DeLaCarte>, With<DeLaGrille>, With<DeLApercu>, With<Debordement>)>>) {
    for e in &q {
        commands.entity(e).despawn();
    }
}

/// La case sous la souris (plan du sol).
fn case_sous_la_souris(camera: &Camera, cam_tf: &GlobalTransform, ecran: Vec2) -> Option<Case> {
    let rayon = camera.viewport_to_world(cam_tf, ecran).ok()?;
    let d = rayon.intersect_plane(Vec3::Y * HAUTEUR * 0.5, InfinitePlane3d::new(Vec3::Y))?;
    Some(modeles::case_sous(rayon.get_point(d)))
}

/// Le pointeur est-il sur un élément d'interface (la main, les panneaux…) ?
fn sur_l_interface(survols: &Query<&Hovered>) -> bool {
    survols.iter().any(|h| h.get())
}

fn viser(
    windows: Query<&Window>,
    cam: Query<(&Camera, &GlobalTransform), With<CameraJeu>>,
    mut souris: MessageReader<bevy::input::mouse::MouseMotion>,
    keys: Res<ButtonInput<KeyCode>>,
    survols: Query<&Hovered>,
    mut jeu: ResMut<Jeu>,
) {
    let bouge = souris.read().count() > 0;
    if bouge && !sur_l_interface(&survols) {
        let Ok((camera, cam_tf)) = cam.single() else { return };
        if let Some(p) = windows.single().ok().and_then(|w| w.cursor_position()) {
            jeu.curseur = case_sous_la_souris(camera, cam_tf, p);
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

impl Jeu {
    /// Après une pose : la main a changé, on garde une carte choisie valable.
    fn posees_apres(&mut self, choix: usize) {
        let n = self.partie.main.len();
        self.choix = if n == 0 { 0 } else { choix.min(n - 1) };
        self.a_redessiner = true;
        self.main_changee = true;
    }
}

/// Rien n'est posable : la main grandit d'une carte (une par image, pour
/// qu'on la voie arriver), jusqu'à ce qu'une carte soit posable.
fn debloquer(mut jeu: ResMut<Jeu>, mut evenements: MessageWriter<Evenement>) {
    if jeu.finie_depuis.is_some() || !jeu.partie.bloquee() {
        return;
    }
    if jeu.partie.piocher() {
        jeu.main_changee = true;
        evenements.write(Evenement::Piochee);
    }
}

fn fin_de_partie(time: Res<Time>, mut jeu: ResMut<Jeu>, mut record: ResMut<Record>, mut next: ResMut<NextState<GameState>>, mut evenements: MessageWriter<Evenement>) {
    let t = time.elapsed_secs();
    if jeu.finie_depuis.is_none() && jeu.partie.terminee() {
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

/// Redessine les cartes qui ne le sont pas encore, et la grille autour.
fn rebatir(
    mut commands: Commands,
    mut jeu: ResMut<Jeu>,
    look: Res<Look>,
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
        let racine = commands.spawn((Transform::default(), Visibility::default(), DeLaCarte(i))).id();
        if i == 0 {
            modeles::dessiner_fuite(&mut commands, racine, &look, carte);
        } else {
            modeles::dessiner_carte(&mut commands, racine, &look, carte, None);
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
    modeles::dessiner_grille(&mut commands, &look, (min.0 - marge, min.1 - marge), (max.0 + marge, max.1 + marge), DeLaGrille);
    // La caméra cadre tout le réseau, avec de la place pour la main en bas.
    let a = modeles::monde(min);
    let b = modeles::monde(max);
    let centre = (a + b) / 2.0;
    let largeur = (b.x - a.x).abs() + 7.0;
    let hauteur = (b.z - a.z).abs() + 7.0;
    cadrage.cible = centre + Vec3::Z * 0.8;
    cadrage.largeur = largeur.max(14.0);
    cadrage.hauteur = hauteur.max(11.0);
}

/// L'aperçu de la carte au curseur : vert si la pose est possible, rouge sinon.
fn apercu(mut commands: Commands, jeu: Res<Jeu>, look: Res<Look>, anciens: Query<Entity, With<DeLApercu>>, mut dernier: Local<Option<(Carte, bool)>>, state: Res<State<GameState>>) {
    let voulu = if *state.get() == GameState::Playing && jeu.finie_depuis.is_none() { jeu.apercu().map(|(c, v)| (c, v.is_ok())) } else { None };
    if voulu == *dernier {
        return;
    }
    *dernier = voulu;
    for e in &anciens {
        commands.entity(e).despawn();
    }
    if let Some((carte, ok)) = voulu {
        let racine = commands.spawn((Transform::from_xyz(0.0, 0.02, 0.0), Visibility::default(), DeLApercu)).id();
        let mat = if ok { &look.valide } else { &look.refuse };
        modeles::dessiner_carte(&mut commands, racine, &look, &carte, Some(mat));
    }
}

/// L'eau monte dans chaque tuyau, tuyau par tuyau.
fn couler(time: Res<Time>, jeu: Res<Jeu>, cartes: Query<(&DeLaCarte, &Children)>, mut eaux: Query<(&Eau, &mut Transform, &mut Visibility)>) {
    let t = time.elapsed_secs();
    for (carte, enfants) in &cartes {
        let Some(&plein) = jeu.plein.get(carte.0) else { continue };
        let niveau = ((t - (plein - DUREE_REMPLISSAGE)) / DUREE_REMPLISSAGE).clamp(0.0, 1.0);
        for &e in enfants {
            if let Ok((eau, mut tf, mut vis)) = eaux.get_mut(e) {
                *vis = if niveau > 0.0 { Visibility::Inherited } else { Visibility::Hidden };
                let h = eau.taille.y * niveau.max(0.001);
                tf.translation = eau.base + Vec3::Y * h / 2.0;
                tf.scale = Vec3::new(eau.taille.x, h, eau.taille.z);
            }
        }
    }
}

/// Les ouvertures pleines qui donnent sur du vide débordent : une flaque
/// apparaît et s'étale.
fn deborder(
    mut commands: Commands,
    time: Res<Time>,
    jeu: Res<Jeu>,
    look: Res<Look>,
    debordements: Query<&Debordement>,
    mut toutes: Query<(&mut Flaque, &mut Transform)>,
    mut evenements: MessageWriter<Evenement>,
) {
    let t = time.elapsed_secs();
    let existantes: Vec<(Case, Direction)> = debordements.iter().map(|d| (d.0, d.1)).collect();
    for (i, carte) in jeu.partie.grille.cartes().iter().enumerate() {
        if jeu.plein.get(i).is_none_or(|&p| t < p) {
            continue;
        }
        for (case, dir) in carte.ouvertures() {
            let vide = dir.voisine(case);
            if jeu.partie.grille.libre(vide) && !existantes.contains(&(vide, dir)) {
                modeles::dessiner_flaque(&mut commands, &look, vide, dir, Debordement(vide, dir));
                evenements.write(Evenement::Deborde);
            }
        }
    }
    for (mut f, mut tf) in &mut toutes {
        if f.taille < f.max {
            f.taille = (f.taille + time.delta_secs() * 0.25).min(f.max);
            tf.scale = Vec3::new(f.taille, 0.02, f.taille * 0.8);
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
        *tf = Transform::from_translation(c.0 + ISO_OFFSET).looking_at(c.0, Vec3::Y);
        *proj = iso_projection(c.1, c.2);
    }
}

fn zoomer(mut molette: MessageReader<MouseWheel>, mut cadrage: ResMut<Cadrage>) {
    for m in molette.read() {
        let pas = if m.y > 0.0 { 0.85 } else { 1.0 / 0.85 };
        cadrage.zoom = (cadrage.zoom * pas).clamp(0.4, 2.5);
    }
}

/// Le jet de la fuite frémit.
fn frissonner(time: Res<Time>, mut jets: Query<(&Jet, &mut Transform)>) {
    let t = time.elapsed_secs();
    for (j, mut tf) in &mut jets {
        let s = 1.0 + 0.12 * (t * 9.0 + j.0 * 2.0).sin();
        tf.scale.y = (0.9 - j.0 * 0.17) * s;
        tf.translation.y = 0.62 + tf.scale.y / 2.0;
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
