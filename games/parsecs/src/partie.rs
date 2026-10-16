//! La partie : la traversée de la ceinture d'astéroïdes.
//!
//! Le jeu se passe dans le plan XY (la caméra regarde depuis +Z, un peu de
//! biais). Le vaisseau remonte la ceinture : les étoiles et les rochers
//! défilent vers le bas. Un parsec toutes les `DUREE_PARSEC` secondes ;
//! chaque parsec est plus dense, un boss arrive tous les trois parsecs.
use bevy::light::NotShadowCaster;
use bevy::mesh::VertexAttributeValues;
use bevy::prelude::*;
use game_common::calendar::Date;
use game_common::{Rng, flat_material};

use crate::GameState;
use crate::reglages::{Pilotage, Reglages};
use crate::vaisseaux::{Progression, Tir, Vaisseau, construire};

/// Moitié de la zone de jeu (largeur, hauteur).
pub const DEMI: Vec2 = Vec2::new(20.0, 12.5);
pub const DUREE_PARSEC: f32 = 25.0;
pub const PARSECS: u32 = 12;
/// Parsec à atteindre pour réussir le défi du jour.
pub const DEFI_PARSEC: u32 = 4;

pub struct PartiePlugin;

impl Plugin for PartiePlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<Evenement>()
            .add_message::<Son>()
            .init_resource::<Partie>()
            .init_resource::<Secousse>()
            .add_systems(Startup, preparer)
            .add_systems(OnEnter(GameState::Playing), commencer)
            .add_systems(OnEnter(GameState::Menu), nettoyer)
            .add_systems(OnEnter(GameState::Hangar), nettoyer)
            .add_systems(
                Update,
                (
                    (piloter, tirer, missiles_auto, avancer_traversee).run_if(en_jeu),
                    (faire_apparaitre, deplacer, guider, ennemis).run_if(en_jeu_ou_menu),
                    (collisions_tirs, collisions_vaisseau, ramasser, durees).run_if(en_jeu),
                    (eclats, secouer, clignoter),
                )
                    .chain(),
            );
    }
}

/// Les rochers défilent aussi derrière l'accueil.
fn en_jeu_ou_menu(state: Res<State<GameState>>, partie: Res<Partie>) -> bool {
    *state.get() == GameState::Menu || (*state.get() == GameState::Playing && !partie.pause)
}

/// Le jeu tourne (en partie et pas en pause).
pub fn en_jeu(state: Res<State<GameState>>, partie: Res<Partie>) -> bool {
    *state.get() == GameState::Playing && !partie.pause
}

// ---- Ce qui se passe, pour le duo et le son -------------------------------

#[derive(Message, Clone, Copy, Debug, PartialEq)]
pub enum Evenement {
    Debut,
    /// Un palier de points (tous les `PALIER_COMMENTAIRE`).
    Palier,
    Boss,
    BossDetruit,
    Frole,
    BouclierFaible,
    Bonus,
    Parsec(u32),
    Traversee,
    Record,
    Debloque,
    Touche,
}

#[derive(Message, Clone, Copy, PartialEq, Eq)]
pub enum Son {
    Tir,
    Rocher,
    Explosion,
    Bonus,
    Bouclier,
    Alarme,
}

pub const PALIER_COMMENTAIRE: u64 = 2_500;

// ---- L'état de la partie --------------------------------------------------

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum Mode {
    #[default]
    Arcade,
    Defi,
}

#[derive(Resource)]
pub struct Partie {
    pub mode: Mode,
    pub score: u64,
    pub vies: u32,
    pub bouclier: f32,
    pub bouclier_max: f32,
    pub temps: f32,
    pub parsec: u32,
    pub pause: bool,
    pub fin: bool,
    pub rng: Rng,
    prochain_rocher: f32,
    prochaine_soucoupe: f32,
    prochains_drones: f32,
    pub paliers_dits: u64,
    pub record_annonce: bool,
    pub frole_attente: f32,
    pub faible_annonce: bool,
    /// Bonus en cours (secondes restantes).
    pub triple: f32,
    pub laser: f32,
    pub missiles: f32,
    pub ralenti: f32,
    pub aimant: f32,
    prochain_missile: f32,
    pub vaisseau: Vaisseau,
}

impl Default for Partie {
    fn default() -> Self {
        Partie {
            mode: Mode::Arcade,
            score: 0,
            vies: 3,
            bouclier: 100.0,
            bouclier_max: 100.0,
            temps: 0.0,
            parsec: 1,
            pause: false,
            fin: false,
            rng: Rng::from_time(),
            prochain_rocher: 0.0,
            prochaine_soucoupe: 30.0,
            prochains_drones: 0.0,
            paliers_dits: 0,
            record_annonce: false,
            frole_attente: 0.0,
            faible_annonce: false,
            triple: 0.0,
            laser: 0.0,
            missiles: 0.0,
            ralenti: 0.0,
            aimant: 0.0,
            prochain_missile: 0.0,
            vaisseau: Vaisseau::Faucheuse,
        }
    }
}

impl Partie {
    fn hasard(&mut self, a: f32, b: f32) -> f32 {
        a + (b - a) * self.rng.below(10_000) as f32 / 10_000.0
    }
    /// Avancée dans le parsec en cours (0 à 1).
    pub fn avancee(&self) -> f32 {
        (self.temps / DUREE_PARSEC).fract()
    }
    fn lent(&self) -> f32 {
        if self.ralenti > 0.0 { 0.4 } else { 1.0 }
    }
}

/// Graine du défi du jour : la même pour tout le monde ce jour-là.
pub fn graine_du_jour(d: Date) -> u64 {
    (d.year as u64 * 10_000 + d.month as u64 * 100 + d.day as u64) * 2_654_435_761 + 12
}

// ---- Les objets du jeu ----------------------------------------------------

#[derive(Component)]
pub struct Joueur {
    pub vitesse: Vec2,
    pub angle: f32,
    recharge: f32,
    pub invulnerable: f32,
    gele: f32,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Taille {
    Petit,
    Moyen,
    Grand,
    Boss,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Genre {
    Normal,
    Glace,
    Metal,
    Explosif,
}

#[derive(Component)]
pub struct Rocher {
    pub taille: Taille,
    pub genre: Genre,
    pub rayon: f32,
    vitesse: Vec2,
    tourne: Vec3,
    pv: i32,
    frole: bool,
}

#[derive(Component)]
struct Projectile {
    vitesse: Vec2,
    vie: f32,
    degats: i32,
    perce: bool,
    guide: bool,
    ennemi: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Bonus {
    Bouclier,
    Triple,
    Laser,
    Missiles,
    Ralenti,
    Aimant,
    Vie,
}

#[derive(Component)]
struct Recompense {
    genre: Bonus,
    vie: f32,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum GenreEnnemi {
    Soucoupe,
    Drone,
}

#[derive(Component)]
struct Ennemi {
    genre: GenreEnnemi,
    pv: i32,
    recharge: f32,
    vitesse: Vec2,
}

#[derive(Component)]
struct Eclat {
    vitesse: Vec3,
    tourne: Vec3,
    vie: f32,
}

/// Tout ce qui appartient à une partie (effacé en revenant au menu).
#[derive(Component)]
pub struct DeLaPartie;

#[derive(Resource, Default)]
pub struct Secousse(pub f32);

#[derive(Component)]
pub struct CameraJeu;

/// Formes et couleurs préparées une fois.
#[derive(Resource)]
struct Look {
    rochers: Vec<Handle<Mesh>>,
    mat_rocher: [Handle<StandardMaterial>; 4],
    mat_boss: Handle<StandardMaterial>,
    tir: Handle<Mesh>,
    mat_tir: Handle<StandardMaterial>,
    mat_tir_laser: Handle<StandardMaterial>,
    mat_tir_ennemi: Handle<StandardMaterial>,
    missile: Handle<Mesh>,
    bonus: Handle<Mesh>,
    mat_bonus: [Handle<StandardMaterial>; 7],
    eclat: Handle<Mesh>,
    soucoupe: Handle<Mesh>,
    mat_soucoupe: Handle<StandardMaterial>,
    drone: Handle<Mesh>,
    mat_drone: Handle<StandardMaterial>,
}

/// Un rocher low poly : une sphère à facettes dont on bosselle les sommets.
fn rocher_mesh(rng: &mut Rng) -> Mesh {
    let mut m = Sphere::new(1.0).mesh().ico(1).unwrap();
    if let Some(VertexAttributeValues::Float32x3(pos)) = m.attribute_mut(Mesh::ATTRIBUTE_POSITION) {
        for p in pos.iter_mut() {
            let k = 0.72 + rng.below(1_000) as f32 / 1_000.0 * 0.5;
            *p = [p[0] * k, p[1] * k * 0.9, p[2] * k];
        }
    }
    m.with_duplicated_vertices().with_computed_flat_normals()
}

fn preparer(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut mats: ResMut<Assets<StandardMaterial>>) {
    // La caméra regarde le plan de jeu depuis +Z, un peu de biais pour le relief.
    commands.spawn((
        Camera3d::default(),
        game_common::iso_projection(DEMI.x * 2.0, DEMI.y * 2.0 + 1.0),
        Transform::from_xyz(0.0, -9.0, 40.0).looking_at(Vec3::new(0.0, 0.5, 0.0), Vec3::Y),
        CameraJeu,
    ));
    commands.spawn((
        DirectionalLight { illuminance: 9_000.0, shadow_maps_enabled: false, ..default() },
        Transform::from_xyz(-10.0, 12.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    let mut rng = Rng::new(1_212);
    let rochers = (0..8).map(|_| meshes.add(rocher_mesh(&mut rng))).collect();
    let emissif = |c: Color, e: LinearRgba| StandardMaterial { emissive: e, ..flat_material(c) };
    let lumiere = |c: Color, f: f32| StandardMaterial { base_color: c, emissive: c.to_linear() * f, unlit: true, ..default() };
    commands.insert_resource(Look {
        rochers,
        mat_rocher: [
            mats.add(flat_material(Color::srgb(0.46, 0.42, 0.40))),
            mats.add(emissif(Color::srgb(0.72, 0.88, 1.0), LinearRgba::rgb(0.05, 0.12, 0.2))),
            mats.add(StandardMaterial { metallic: 0.7, perceptual_roughness: 0.45, ..flat_material(Color::srgb(0.58, 0.60, 0.66)) }),
            mats.add(emissif(Color::srgb(0.50, 0.22, 0.16), LinearRgba::rgb(0.9, 0.25, 0.02))),
        ],
        mat_boss: mats.add(emissif(Color::srgb(0.36, 0.31, 0.34), LinearRgba::rgb(0.25, 0.04, 0.08))),
        tir: meshes.add(Cuboid::new(0.12, 0.7, 0.12)),
        mat_tir: mats.add(lumiere(Color::srgb(0.45, 0.95, 1.0), 3.0)),
        mat_tir_laser: mats.add(lumiere(Color::srgb(1.0, 0.35, 0.85), 4.0)),
        mat_tir_ennemi: mats.add(lumiere(Color::srgb(1.0, 0.35, 0.2), 3.0)),
        missile: meshes.add(Cone { radius: 0.14, height: 0.6 }.mesh().resolution(5)),
        bonus: meshes.add(Mesh::from(Sphere::new(0.45).mesh().ico(0).unwrap()).with_duplicated_vertices().with_computed_flat_normals()),
        mat_bonus: [
            mats.add(lumiere(Color::srgb(0.3, 0.75, 1.0), 1.5)),
            mats.add(lumiere(Color::srgb(1.0, 0.85, 0.2), 1.5)),
            mats.add(lumiere(Color::srgb(1.0, 0.35, 0.85), 1.5)),
            mats.add(lumiere(Color::srgb(1.0, 0.55, 0.15), 1.5)),
            mats.add(lumiere(Color::srgb(0.55, 1.0, 0.6), 1.5)),
            mats.add(lumiere(Color::srgb(0.75, 0.55, 1.0), 1.5)),
            mats.add(lumiere(Color::srgb(1.0, 0.3, 0.35), 1.5)),
        ],
        eclat: meshes.add(Tetrahedron::default()),
        soucoupe: meshes.add(Cylinder::new(1.0, 0.3).mesh().resolution(8)),
        mat_soucoupe: mats.add(emissif(Color::srgb(0.65, 0.68, 0.75), LinearRgba::rgb(0.3, 0.9, 0.4))),
        drone: meshes.add(Mesh::from(Sphere::new(0.5).mesh().ico(0).unwrap()).with_duplicated_vertices().with_computed_flat_normals()),
        mat_drone: mats.add(emissif(Color::srgb(0.25, 0.18, 0.30), LinearRgba::rgb(1.2, 0.1, 0.15))),
    });
}

/// Une nouvelle partie : le vaisseau choisi, au centre en bas.
fn commencer(
    mut commands: Commands,
    mut partie: ResMut<Partie>,
    progression: Res<Progression>,
    anciens: Query<Entity, With<DeLaPartie>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<StandardMaterial>>,
    mut evenements: MessageWriter<Evenement>,
) {
    // On repart d'un champ vide (les rochers du menu s'en vont).
    for e in &anciens {
        commands.entity(e).despawn();
    }
    let mode = partie.mode;
    let v = progression.choisi;
    let fiche = v.fiche();
    *partie = Partie {
        mode,
        vaisseau: v,
        bouclier: fiche.bouclier,
        bouclier_max: fiche.bouclier,
        rng: if mode == Mode::Defi { Rng::new(graine_du_jour(Date::today())) } else { Rng::from_time() },
        ..default()
    };
    let racine = commands
        .spawn((
            Transform::from_xyz(0.0, -DEMI.y + 3.0, 0.0).with_scale(Vec3::splat(1.5)),
            Visibility::default(),
            Joueur { vitesse: Vec2::ZERO, angle: 0.0, recharge: 0.0, invulnerable: 2.0, gele: 0.0 },
            DeLaPartie,
        ))
        .id();
    construire(&mut commands, racine, v, progression.livree[v.index()], false, &mut meshes, &mut mats);
    evenements.write(Evenement::Debut);
}

/// Retour au menu : on vide le champ de jeu.
fn nettoyer(mut commands: Commands, q: Query<Entity, With<DeLaPartie>>, mut partie: ResMut<Partie>, mut temps: ResMut<Time<Virtual>>) {
    for e in &q {
        commands.entity(e).despawn();
    }
    // On a pu quitter une partie en pause : le temps repart.
    temps.unpause();
    partie.pause = false;
    partie.fin = false;
}

// ---- Le vaisseau ----------------------------------------------------------

/// Direction voulue au tactile : le doigt par rapport à son point de départ.
fn tactile(touches: &Touches) -> Option<Vec2> {
    let t = touches.iter().next()?;
    let d = t.position() - t.start_position();
    (d.length() > 12.0).then(|| Vec2::new(d.x, -d.y).normalize())
}

fn piloter(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    touches: Res<Touches>,
    reglages: Res<Reglages>,
    mut partie: ResMut<Partie>,
    mut q: Query<(&mut Joueur, &mut Transform)>,
) {
    let dt = time.delta_secs();
    let fiche = partie.vaisseau.fiche();
    let Ok((mut j, mut tr)) = q.single_mut() else { return };
    let haut = keys.any_pressed([KeyCode::ArrowUp, KeyCode::KeyW]);
    let bas = keys.any_pressed([KeyCode::ArrowDown, KeyCode::KeyS]);
    let gauche = keys.any_pressed([KeyCode::ArrowLeft, KeyCode::KeyA]);
    let droite = keys.any_pressed([KeyCode::ArrowRight, KeyCode::KeyD]);
    let lent = if j.gele > 0.0 { 0.45 } else { 1.0 };
    let doigt = tactile(&touches);
    if reglages.pilotage == Pilotage::Classique && doigt.is_none() {
        // Tourner et accélérer, avec de l'inertie, comme le jeu d'origine.
        let sens = (gauche as i32 - droite as i32) as f32;
        j.angle += sens * fiche.rotation * dt;
        let devant = Vec2::new(-j.angle.sin(), j.angle.cos());
        if haut {
            j.vitesse += devant * fiche.poussee * lent * dt;
        }
        if bas {
            j.vitesse *= 1.0 - 2.5 * dt;
        }
        j.vitesse *= 1.0 - 0.35 * dt;
    } else {
        // Direct : le vaisseau va où l'on pousse, pointe toujours vers le haut.
        let mut d = Vec2::new((droite as i32 - gauche as i32) as f32, (haut as i32 - bas as i32) as f32);
        if let Some(t) = doigt {
            d = t;
        }
        let voulu = d.normalize_or_zero() * fiche.vitesse * lent;
        j.vitesse = j.vitesse.lerp(voulu, (8.0 * dt).min(1.0));
        j.angle += (0.0 - j.angle) * (8.0 * dt).min(1.0);
    }
    j.vitesse = j.vitesse.clamp_length_max(fiche.vitesse * lent);
    let mut p = tr.translation.truncate() + j.vitesse * dt;
    // On passe d'un bord à l'autre sur les côtés ; en haut et en bas, on bute.
    if p.x > DEMI.x {
        p.x -= 2.0 * DEMI.x;
    } else if p.x < -DEMI.x {
        p.x += 2.0 * DEMI.x;
    }
    p.y = p.y.clamp(-DEMI.y + 1.0, DEMI.y - 1.5);
    tr.translation = p.extend(0.0);
    // Un peu d'inclinaison dans les virages, pour le relief.
    let roulis = (-j.vitesse.x / fiche.vitesse * 0.5).clamp(-0.5, 0.5);
    tr.rotation = Quat::from_rotation_z(j.angle) * Quat::from_rotation_y(roulis);
    tr.scale = Vec3::splat(1.5);
    j.invulnerable = (j.invulnerable - dt).max(0.0);
    j.gele = (j.gele - dt).max(0.0);
    partie.frole_attente = (partie.frole_attente - dt).max(0.0);
}

#[allow(clippy::too_many_arguments)]
fn tirer(
    mut commands: Commands,
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    touches: Res<Touches>,
    reglages: Res<Reglages>,
    partie: Res<Partie>,
    look: Res<Look>,
    mut q: Query<(&mut Joueur, &Transform)>,
    mut sons: MessageWriter<Son>,
) {
    let Ok((mut j, tr)) = q.single_mut() else { return };
    j.recharge -= time.delta_secs();
    let veut = keys.pressed(KeyCode::Space) || reglages.tir_auto || touches.iter().next().is_some();
    if !veut || j.recharge > 0.0 {
        return;
    }
    let fiche = partie.vaisseau.fiche();
    j.recharge = fiche.cadence;
    let devant = Vec2::new(-j.angle.sin(), j.angle.cos());
    let cote = Vec2::new(devant.y, -devant.x);
    let base = tr.translation.truncate() + devant * 1.2;
    let mut tirs: Vec<(Vec2, Vec2, bool)> = match fiche.tir {
        Tir::Double => vec![(base + cote * 0.45, devant, false), (base - cote * 0.45, devant, false)],
        Tir::Aimante => vec![(base, devant, true)],
        Tir::Rapide => vec![(base, devant, false)],
        Tir::Eventail => [-0.22_f32, 0.0, 0.22].iter().map(|a| (base, Vec2::from_angle(*a).rotate(devant), false)).collect(),
    };
    if partie.triple > 0.0 {
        tirs.push((base, Vec2::from_angle(0.35).rotate(devant), false));
        tirs.push((base, Vec2::from_angle(-0.35).rotate(devant), false));
    }
    let laser = partie.laser > 0.0;
    for (p, d, guide) in tirs {
        commands.spawn((
            Mesh3d(look.tir.clone()),
            MeshMaterial3d(if laser { look.mat_tir_laser.clone() } else { look.mat_tir.clone() }),
            Transform::from_translation(p.extend(0.0)).with_rotation(Quat::from_rotation_arc_2d(Vec2::Y, d)).with_scale(if laser { Vec3::new(1.4, 1.8, 1.4) } else { Vec3::ONE }),
            Projectile { vitesse: d * 32.0, vie: 1.0, degats: if laser { 2 } else { 1 }, perce: laser, guide, ennemi: false },
            NotShadowCaster,
            DeLaPartie,
        ));
    }
    sons.write(Son::Tir);
}

/// Le bonus « Missiles » : un missile à tête chercheuse de temps en temps.
fn missiles_auto(mut commands: Commands, time: Res<Time>, mut partie: ResMut<Partie>, look: Res<Look>, q: Query<&Transform, With<Joueur>>) {
    if partie.missiles <= 0.0 {
        return;
    }
    partie.prochain_missile -= time.delta_secs();
    let Ok(tr) = q.single() else { return };
    if partie.prochain_missile <= 0.0 {
        partie.prochain_missile = 0.8;
        for s in [-1.0, 1.0] {
            commands.spawn((
                Mesh3d(look.missile.clone()),
                MeshMaterial3d(look.mat_bonus[3].clone()),
                Transform::from_translation(tr.translation + Vec3::new(s * 0.8, 0.0, 0.0)),
                Projectile { vitesse: Vec2::new(s * 6.0, 14.0), vie: 2.5, degats: 2, perce: false, guide: true, ennemi: false },
                NotShadowCaster,
                DeLaPartie,
            ));
        }
    }
}

// ---- La traversée -----------------------------------------------------------

fn avancer_traversee(
    time: Res<Time>,
    mut partie: ResMut<Partie>,
    progression: Res<Progression>,
    mut evenements: MessageWriter<Evenement>,
    mut sons: MessageWriter<Son>,
    mut commands: Commands,
    look: Res<Look>,
) {
    partie.temps += time.delta_secs();
    let parsec = 1 + (partie.temps / DUREE_PARSEC) as u32;
    if parsec != partie.parsec {
        partie.parsec = parsec;
        partie.score += 500;
        if parsec == PARSECS + 1 {
            evenements.write(Evenement::Traversee);
        } else {
            evenements.write(Evenement::Parsec(parsec));
        }
        // Un boss tous les trois parsecs.
        if parsec % 3 == 0 {
            let x = partie.hasard(-8.0, 8.0);
            apparaitre_rocher(&mut commands, &look, &mut partie, Taille::Boss, Genre::Normal, Vec2::new(x, DEMI.y + 5.0), Vec2::new(0.0, -1.6));
            evenements.write(Evenement::Boss);
            sons.write(Son::Alarme);
        }
    }
    // Le record tombe en pleine partie : le duo le fête tout de suite.
    if !partie.record_annonce && progression.record > 0 && partie.score > progression.record {
        partie.record_annonce = true;
        evenements.write(Evenement::Record);
    }
    let d = partie.score / PALIER_COMMENTAIRE;
    if d > partie.paliers_dits {
        partie.paliers_dits = d;
        evenements.write(Evenement::Palier);
    }
    let bas = partie.bouclier < partie.bouclier_max * 0.3 && partie.vies == 1;
    if bas && !partie.faible_annonce {
        partie.faible_annonce = true;
        evenements.write(Evenement::BouclierFaible);
    } else if !bas {
        partie.faible_annonce = false;
    }
}

fn rayon(t: Taille) -> f32 {
    match t {
        Taille::Petit => 0.7,
        Taille::Moyen => 1.25,
        Taille::Grand => 2.1,
        Taille::Boss => 4.2,
    }
}

#[allow(clippy::too_many_arguments)]
fn apparaitre_rocher(commands: &mut Commands, look: &Look, partie: &mut Partie, taille: Taille, genre: Genre, p: Vec2, vitesse: Vec2) {
    let r = rayon(taille);
    let pv = match (taille, genre) {
        (Taille::Boss, _) => 24 + partie.parsec as i32 * 2,
        (t, Genre::Metal) => 2 + t as i32,
        (Taille::Grand, _) => 2,
        _ => 1,
    };
    let i = partie.rng.below(look.rochers.len() as u32) as usize;
    let mat = if taille == Taille::Boss { look.mat_boss.clone() } else { look.mat_rocher[genre as usize].clone() };
    let tourne = Vec3::new(partie.hasard(-1.5, 1.5), partie.hasard(-1.5, 1.5), partie.hasard(-1.0, 1.0));
    commands.spawn((
        Mesh3d(look.rochers[i].clone()),
        MeshMaterial3d(mat),
        Transform::from_translation(p.extend(0.0)).with_scale(Vec3::splat(r)),
        Rocher { taille, genre, rayon: r, vitesse, tourne, pv, frole: false },
        DeLaPartie,
    ));
}

/// De nouveaux rochers arrivent par le haut, de plus en plus nombreux.
fn faire_apparaitre(
    mut commands: Commands,
    time: Res<Time>,
    state: Res<State<GameState>>,
    mut partie: ResMut<Partie>,
    look: Res<Look>,
    rochers: Query<&Rocher>,
) {
    let dt = time.delta_secs();
    let menu = *state.get() == GameState::Menu;
    partie.prochain_rocher -= dt;
    let parsec = if menu { 2 } else { partie.parsec } as f32;
    let max = if menu { 9 } else { 8 + (parsec as usize) * 2 };
    if partie.prochain_rocher <= 0.0 && rochers.iter().count() < max {
        partie.prochain_rocher = (2.4 - parsec * 0.14).max(0.6) * partie.hasard(0.7, 1.3);
        let taille = match partie.rng.below(10) {
            0..=4 => Taille::Grand,
            5..=7 => Taille::Moyen,
            _ => Taille::Petit,
        };
        // Les rochers spéciaux apparaissent avec la profondeur de la ceinture.
        let genre = match partie.rng.below(20) {
            0..=1 if parsec >= 2.0 => Genre::Glace,
            2..=3 if parsec >= 3.0 => Genre::Metal,
            4 if parsec >= 4.0 => Genre::Explosif,
            _ => Genre::Normal,
        };
        let x = partie.hasard(-DEMI.x, DEMI.x);
        let v = Vec2::new(partie.hasard(-2.0, 2.0), -partie.hasard(2.0, 3.5 + parsec * 0.25));
        apparaitre_rocher(&mut commands, &look, &mut partie, taille, genre, Vec2::new(x, DEMI.y + 3.0), v);
    }
    if menu {
        return;
    }
    // Une soucoupe de temps en temps à partir du parsec 2.
    partie.prochaine_soucoupe -= dt;
    if partie.parsec >= 2 && partie.prochaine_soucoupe <= 0.0 {
        partie.prochaine_soucoupe = partie.hasard(28.0, 40.0);
        let s = if partie.rng.below(2) == 0 { -1.0 } else { 1.0 };
        commands.spawn((
            Mesh3d(look.soucoupe.clone()),
            MeshMaterial3d(look.mat_soucoupe.clone()),
            Transform::from_xyz(-s * (DEMI.x + 2.0), DEMI.y * 0.55, 0.5).with_rotation(Quat::from_rotation_x(1.2)),
            Ennemi { genre: GenreEnnemi::Soucoupe, pv: 4, recharge: 1.5, vitesse: Vec2::new(s * 4.5, 0.0) },
            DeLaPartie,
        ));
    }
    // Des drones de l'Hégémonie, par deux, à partir du parsec 5.
    partie.prochains_drones -= dt;
    if partie.parsec >= 5 && partie.prochains_drones <= 0.0 {
        partie.prochains_drones = partie.hasard(18.0, 26.0);
        for _ in 0..2 {
            let x = partie.hasard(-DEMI.x + 2.0, DEMI.x - 2.0);
            commands.spawn((
                Mesh3d(look.drone.clone()),
                MeshMaterial3d(look.mat_drone.clone()),
                Transform::from_xyz(x, DEMI.y + 2.0, 0.0),
                Ennemi { genre: GenreEnnemi::Drone, pv: 2, recharge: 0.0, vitesse: Vec2::ZERO },
                DeLaPartie,
            ));
        }
    }
}

fn deplacer(
    mut commands: Commands,
    time: Res<Time>,
    partie: Res<Partie>,
    mut rochers: Query<(Entity, &Rocher, &mut Transform), Without<Projectile>>,
    mut tirs: Query<(Entity, &mut Projectile, &mut Transform), Without<Rocher>>,
    mut bonus: Query<(Entity, &mut Recompense, &mut Transform), (Without<Rocher>, Without<Projectile>)>,
) {
    let dt = time.delta_secs();
    let lent = partie.lent();
    for (e, r, mut tr) in &mut rochers {
        tr.translation += (r.vitesse * dt * lent).extend(0.0);
        tr.rotate(Quat::from_euler(EulerRot::XYZ, r.tourne.x * dt, r.tourne.y * dt, r.tourne.z * dt));
        let p = &mut tr.translation;
        if p.x > DEMI.x + r.rayon {
            p.x -= 2.0 * DEMI.x + 2.0 * r.rayon;
        } else if p.x < -DEMI.x - r.rayon {
            p.x += 2.0 * DEMI.x + 2.0 * r.rayon;
        }
        if p.y < -DEMI.y - r.rayon - 2.0 {
            commands.entity(e).despawn();
        }
    }
    for (e, mut t, mut tr) in &mut tirs {
        tr.translation += (t.vitesse * dt).extend(0.0);
        t.vie -= dt;
        let p = tr.translation;
        if t.vie <= 0.0 || p.y.abs() > DEMI.y + 3.0 || p.x.abs() > DEMI.x + 3.0 {
            commands.entity(e).despawn();
        }
    }
    for (e, mut b, mut tr) in &mut bonus {
        tr.translation.y -= 1.5 * dt;
        tr.rotate_y(2.0 * dt);
        b.vie -= dt;
        if b.vie <= 0.0 {
            commands.entity(e).despawn();
        }
    }
}

/// Les tirs guidés tournent vers le rocher le plus proche.
fn guider(time: Res<Time>, mut tirs: Query<(&mut Projectile, &mut Transform)>, rochers: Query<&Transform, (With<Rocher>, Without<Projectile>)>) {
    let dt = time.delta_secs();
    for (mut t, mut tr) in &mut tirs {
        if !t.guide || t.ennemi {
            continue;
        }
        let p = tr.translation.truncate();
        let Some(cible) = rochers.iter().map(|r| r.translation.truncate()).min_by(|a, b| a.distance_squared(p).total_cmp(&b.distance_squared(p))) else { continue };
        let voulu = (cible - p).normalize_or_zero() * t.vitesse.length();
        t.vitesse = t.vitesse.lerp(voulu, (3.5 * dt).min(1.0));
        tr.rotation = Quat::from_rotation_arc_2d(Vec2::Y, t.vitesse.normalize_or(Vec2::Y));
    }
}

/// Soucoupes (traversent et tirent) et drones (poursuivent le vaisseau).
fn ennemis(
    mut commands: Commands,
    time: Res<Time>,
    partie: Res<Partie>,
    look: Res<Look>,
    joueur: Query<&Transform, With<Joueur>>,
    mut q: Query<(Entity, &mut Ennemi, &mut Transform), Without<Joueur>>,
) {
    let dt = time.delta_secs() * partie.lent();
    let cible = joueur.single().map(|t| t.translation.truncate()).unwrap_or(Vec2::ZERO);
    for (e, mut en, mut tr) in &mut q {
        let p = tr.translation.truncate();
        match en.genre {
            GenreEnnemi::Soucoupe => {
                tr.translation += (en.vitesse * dt).extend(0.0);
                tr.rotate_local_y(3.0 * dt);
                en.recharge -= dt;
                if en.recharge <= 0.0 && joueur.single().is_ok() {
                    en.recharge = 1.6;
                    let d = (cible - p).normalize_or(Vec2::NEG_Y);
                    commands.spawn((
                        Mesh3d(look.tir.clone()),
                        MeshMaterial3d(look.mat_tir_ennemi.clone()),
                        Transform::from_translation(tr.translation).with_rotation(Quat::from_rotation_arc_2d(Vec2::Y, d)),
                        Projectile { vitesse: d * 11.0, vie: 3.0, degats: 15, perce: false, guide: false, ennemi: true },
                        NotShadowCaster,
                        DeLaPartie,
                    ));
                }
                if p.x.abs() > DEMI.x + 4.0 {
                    commands.entity(e).despawn();
                }
            }
            GenreEnnemi::Drone => {
                let voulu = (cible - p).normalize_or_zero() * 5.5;
                en.vitesse = en.vitesse.lerp(voulu, (1.5 * dt).min(1.0));
                tr.translation += (en.vitesse * dt).extend(0.0);
                tr.rotate_z(4.0 * dt);
            }
        }
    }
}

// ---- Les collisions ---------------------------------------------------------

/// Points selon la taille (les petits, plus durs à toucher, rapportent plus).
fn points(t: Taille) -> u64 {
    match t {
        Taille::Grand => 20,
        Taille::Moyen => 50,
        Taille::Petit => 100,
        Taille::Boss => 1_000,
    }
}

fn eclats_de(commands: &mut Commands, look: &Look, mat: Handle<StandardMaterial>, p: Vec3, n: usize, taille: f32, rng: &mut Rng) {
    let mut h = || rng.below(10_000) as f32 / 10_000.0 * 2.0 - 1.0;
    for _ in 0..n {
        let v = Vec3::new(h(), h(), h() * 0.5) * 7.0;
        let s = taille * (0.15 + 0.15 * h().abs());
        commands.spawn((
            Mesh3d(look.eclat.clone()),
            MeshMaterial3d(mat.clone()),
            Transform::from_translation(p).with_scale(Vec3::splat(s)),
            Eclat { vitesse: v, tourne: Vec3::new(h(), h(), h()) * 6.0, vie: 0.9 },
            NotShadowCaster,
            DeLaPartie,
        ));
    }
}

/// Brise un rocher : morceaux plus petits, éclats, points, et peut-être un bonus.
#[allow(clippy::too_many_arguments)]
fn briser(commands: &mut Commands, look: &Look, partie: &mut Partie, e: Entity, r: &Rocher, p: Vec2, secousse: &mut Secousse, sons: &mut MessageWriter<Son>, evenements: &mut MessageWriter<Evenement>) -> Vec<(Vec2, f32)> {
    commands.entity(e).despawn();
    partie.score += points(r.taille);
    let mat = if r.taille == Taille::Boss { look.mat_boss.clone() } else { look.mat_rocher[r.genre as usize].clone() };
    let n = match r.taille {
        Taille::Boss => 40,
        Taille::Grand => 14,
        _ => 8,
    };
    eclats_de(commands, look, mat, p.extend(0.0), n, r.rayon, &mut partie.rng);
    let morceaux = match r.taille {
        Taille::Boss => Some((Taille::Grand, 4)),
        Taille::Grand => Some((Taille::Moyen, 2)),
        Taille::Moyen => Some((Taille::Petit, 2)),
        Taille::Petit => None,
    };
    if let Some((t, k)) = morceaux {
        for i in 0..k {
            let a = std::f32::consts::TAU * i as f32 / k as f32 + partie.hasard(-0.4, 0.4);
            let v = r.vitesse * 0.5 + Vec2::from_angle(a) * partie.hasard(2.5, 4.5);
            let genre = if r.genre == Genre::Explosif { Genre::Normal } else { r.genre };
            apparaitre_rocher(commands, look, partie, t, genre, p + Vec2::from_angle(a) * r.rayon * 0.4, v);
        }
    }
    // Un bonus de temps en temps (toujours deux sur un boss).
    let tirages = if r.taille == Taille::Boss { 2 } else { (partie.rng.below(100) < 9) as u32 };
    for k in 0..tirages {
        let genre = match partie.rng.below(100) {
            0..=19 => Bonus::Bouclier,
            20..=36 => Bonus::Triple,
            37..=50 => Bonus::Laser,
            51..=64 => Bonus::Missiles,
            65..=77 => Bonus::Ralenti,
            78..=91 => Bonus::Aimant,
            _ => Bonus::Vie,
        };
        commands.spawn((
            Mesh3d(look.bonus.clone()),
            MeshMaterial3d(look.mat_bonus[genre as usize].clone()),
            Transform::from_translation((p + Vec2::new(k as f32 * 1.5, 0.0)).extend(0.0)),
            Recompense { genre, vie: 10.0 },
            NotShadowCaster,
            DeLaPartie,
        ));
    }
    secousse.0 = secousse.0.max(match r.taille {
        Taille::Boss => 1.2,
        Taille::Grand => 0.35,
        _ => 0.15,
    });
    if r.taille == Taille::Boss {
        sons.write(Son::Explosion);
        evenements.write(Evenement::BossDetruit);
    } else {
        sons.write(Son::Rocher);
    }
    // Un rocher explosif emporte ses voisins.
    if r.genre == Genre::Explosif { vec![(p, 5.0)] } else { vec![] }
}

#[allow(clippy::too_many_arguments)]
fn collisions_tirs(
    mut commands: Commands,
    mut partie: ResMut<Partie>,
    look: Res<Look>,
    mut secousse: ResMut<Secousse>,
    tirs: Query<(Entity, &Projectile, &Transform)>,
    mut rochers: Query<(Entity, &mut Rocher, &Transform)>,
    mut ennemis: Query<(Entity, &mut Ennemi, &Transform)>,
    mut sons: MessageWriter<Son>,
    mut evenements: MessageWriter<Evenement>,
) {
    let mut detruits: Vec<Entity> = Vec::new();
    let mut explosions: Vec<(Vec2, f32)> = Vec::new();
    for (te, t, ttr) in &tirs {
        if t.ennemi {
            continue;
        }
        let tp = ttr.translation.truncate();
        let mut touche = false;
        for (re, mut r, rtr) in &mut rochers {
            if detruits.contains(&re) {
                continue;
            }
            let rp = rtr.translation.truncate();
            if tp.distance(rp) < r.rayon + 0.2 {
                touche = true;
                r.pv -= t.degats;
                if r.pv <= 0 {
                    detruits.push(re);
                    let r = Rocher { vitesse: r.vitesse, tourne: r.tourne, ..*r };
                    explosions.extend(briser(&mut commands, &look, &mut partie, re, &r, rp, &mut secousse, &mut sons, &mut evenements));
                } else {
                    // Touché sans être brisé : quelques étincelles.
                    eclats_de(&mut commands, &look, look.mat_rocher[r.genre as usize].clone(), tp.extend(0.0), 2, 0.5, &mut partie.rng);
                }
                break;
            }
        }
        for (ee, mut en, etr) in &mut ennemis {
            if !touche && tp.distance(etr.translation.truncate()) < 1.1 {
                touche = true;
                en.pv -= t.degats;
                if en.pv <= 0 {
                    commands.entity(ee).despawn();
                    partie.score += if en.genre == GenreEnnemi::Soucoupe { 500 } else { 250 };
                    eclats_de(&mut commands, &look, look.mat_soucoupe.clone(), etr.translation, 16, 1.0, &mut partie.rng);
                    sons.write(Son::Explosion);
                    secousse.0 = secousse.0.max(0.4);
                }
            }
        }
        if touche && !t.perce {
            commands.entity(te).despawn();
        }
    }
    // Les explosions en chaîne des rochers explosifs.
    for (centre, portee) in explosions {
        for (re, r, rtr) in &rochers {
            let rp = rtr.translation.truncate();
            if !detruits.contains(&re) && r.taille != Taille::Boss && rp.distance(centre) < portee + r.rayon {
                detruits.push(re);
                let copie = Rocher { vitesse: r.vitesse, tourne: r.tourne, ..*r };
                briser(&mut commands, &look, &mut partie, re, &copie, rp, &mut secousse, &mut sons, &mut evenements);
            }
        }
        secousse.0 = secousse.0.max(0.8);
        sons.write(Son::Explosion);
    }
}

#[allow(clippy::too_many_arguments)]
fn collisions_vaisseau(
    mut commands: Commands,
    mut partie: ResMut<Partie>,
    look: Res<Look>,
    mut secousse: ResMut<Secousse>,
    mut joueur: Query<(&mut Joueur, &Transform)>,
    mut rochers: Query<(Entity, &mut Rocher, &Transform)>,
    tirs: Query<(Entity, &Projectile, &Transform)>,
    ennemis: Query<(Entity, &Ennemi, &Transform)>,
    mut sons: MessageWriter<Son>,
    mut evenements: MessageWriter<Evenement>,
    mut next: ResMut<NextState<GameState>>,
) {
    let Ok((mut j, jtr)) = joueur.single_mut() else { return };
    let rayon_j = partie.vaisseau.fiche().rayon;
    let jp = jtr.translation.truncate();
    let mut degats = 0.0;
    let mut gele = false;
    for (re, mut r, rtr) in &mut rochers {
        let rp = rtr.translation.truncate();
        let d = jp.distance(rp);
        if d < r.rayon + rayon_j * 0.85 {
            if j.invulnerable > 0.0 {
                continue;
            }
            degats += match r.taille {
                Taille::Petit => 15.0,
                Taille::Moyen => 25.0,
                Taille::Grand => 40.0,
                Taille::Boss => 70.0,
            };
            gele |= r.genre == Genre::Glace;
            if r.taille != Taille::Boss {
                let copie = Rocher { vitesse: r.vitesse, tourne: r.tourne, ..*r };
                briser(&mut commands, &look, &mut partie, re, &copie, rp, &mut secousse, &mut sons, &mut evenements);
            }
        } else if d < r.rayon + rayon_j + 1.1 && !r.frole {
            // Frôlé de près : le duo le remarque (pas trop souvent).
            r.frole = true;
            if partie.frole_attente <= 0.0 && r.taille != Taille::Petit {
                partie.frole_attente = 12.0;
                evenements.write(Evenement::Frole);
            }
        }
    }
    for (te, t, ttr) in &tirs {
        if t.ennemi && j.invulnerable <= 0.0 && jp.distance(ttr.translation.truncate()) < rayon_j {
            degats += t.degats as f32;
            commands.entity(te).despawn();
        }
    }
    for (ee, en, etr) in &ennemis {
        if j.invulnerable <= 0.0 && jp.distance(etr.translation.truncate()) < rayon_j + 0.8 {
            degats += 20.0;
            if en.genre == GenreEnnemi::Drone {
                commands.entity(ee).despawn();
                eclats_de(&mut commands, &look, look.mat_drone.clone(), etr.translation, 10, 0.8, &mut partie.rng);
            }
        }
    }
    if gele {
        j.gele = 2.0;
    }
    if degats <= 0.0 {
        return;
    }
    sons.write(Son::Bouclier);
    secousse.0 = secousse.0.max(0.6);
    partie.bouclier -= degats;
    if partie.bouclier > 0.0 {
        evenements.write(Evenement::Touche);
        return;
    }
    // Bouclier épuisé : une vie en moins.
    eclats_de(&mut commands, &look, look.mat_bonus[3].clone(), jtr.translation, 30, 1.2, &mut partie.rng);
    sons.write(Son::Explosion);
    if partie.vies > 1 {
        partie.vies -= 1;
        partie.bouclier = partie.bouclier_max;
        j.invulnerable = 2.5;
        j.vitesse = Vec2::ZERO;
    } else {
        partie.vies = 0;
        partie.bouclier = 0.0;
        partie.fin = true;
        next.set(GameState::Over);
    }
}

fn ramasser(
    mut commands: Commands,
    time: Res<Time>,
    mut partie: ResMut<Partie>,
    joueur: Query<&Transform, With<Joueur>>,
    mut bonus: Query<(Entity, &Recompense, &mut Transform), Without<Joueur>>,
    mut sons: MessageWriter<Son>,
    mut evenements: MessageWriter<Evenement>,
) {
    let Ok(jtr) = joueur.single() else { return };
    let jp = jtr.translation.truncate();
    let dt = time.delta_secs();
    for (e, b, mut tr) in &mut bonus {
        let p = tr.translation.truncate();
        // L'aimant attire les bonus de loin.
        if partie.aimant > 0.0 && p.distance(jp) < 12.0 {
            let d = (jp - p).normalize_or_zero() * 12.0 * dt;
            tr.translation += d.extend(0.0);
        }
        if p.distance(jp) < 1.4 {
            commands.entity(e).despawn();
            match b.genre {
                Bonus::Bouclier => partie.bouclier = partie.bouclier_max,
                Bonus::Triple => partie.triple = 10.0,
                Bonus::Laser => partie.laser = 8.0,
                Bonus::Missiles => partie.missiles = 10.0,
                Bonus::Ralenti => partie.ralenti = 6.0,
                Bonus::Aimant => partie.aimant = 14.0,
                Bonus::Vie => partie.vies = (partie.vies + 1).min(5),
            }
            partie.score += 150;
            sons.write(Son::Bonus);
            evenements.write(Evenement::Bonus);
        }
    }
}

fn durees(time: Res<Time>, mut partie: ResMut<Partie>) {
    let dt = time.delta_secs();
    let p = &mut *partie;
    for t in [&mut p.triple, &mut p.laser, &mut p.missiles, &mut p.ralenti, &mut p.aimant] {
        *t = (*t - dt).max(0.0);
    }
}

// ---- Effets -----------------------------------------------------------------

fn eclats(mut commands: Commands, time: Res<Time>, mut q: Query<(Entity, &mut Eclat, &mut Transform)>) {
    let dt = time.delta_secs();
    for (e, mut ec, mut tr) in &mut q {
        tr.translation += ec.vitesse * dt;
        ec.vitesse *= 1.0 - 1.5 * dt;
        tr.rotate(Quat::from_euler(EulerRot::XYZ, ec.tourne.x * dt, ec.tourne.y * dt, ec.tourne.z * dt));
        ec.vie -= dt;
        tr.scale *= 1.0 - 1.2 * dt;
        if ec.vie <= 0.0 {
            commands.entity(e).despawn();
        }
    }
}

/// La caméra tremble un instant après un choc ou une explosion.
fn secouer(time: Res<Time>, mut secousse: ResMut<Secousse>, mut cam: Query<&mut Transform, With<CameraJeu>>) {
    let t = time.elapsed_secs();
    let s = secousse.0;
    secousse.0 = (s - time.delta_secs() * 2.5).max(0.0);
    for mut tr in &mut cam {
        let d = Vec3::new((t * 53.0).sin(), (t * 47.0).cos(), 0.0) * s * 0.35;
        *tr = Transform::from_xyz(d.x, -9.0 + d.y, 40.0).looking_at(Vec3::new(d.x, 0.5 + d.y, 0.0), Vec3::Y);
    }
}

/// Le vaisseau clignote quand il est invulnérable.
fn clignoter(time: Res<Time>, mut q: Query<(&Joueur, &mut Visibility)>) {
    for (j, mut v) in &mut q {
        *v = if j.invulnerable > 0.0 && (time.elapsed_secs() * 12.0).sin() > 0.0 { Visibility::Hidden } else { Visibility::Inherited };
    }
}
