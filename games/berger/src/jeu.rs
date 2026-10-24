//! La partie : la simulation de `berger-core` branchée sur Bevy (commandes du
//! joueur, mise à jour des animaux, effets, progression).
use std::f32::consts::{PI, TAU};

use berger_core::{Commande, Evenement, Fin, Monde, NIVEAUX, V2, generer, niveau, niveau_du_jour, robot, v2};
use bevy::prelude::*;
use game_common::calendar::Date;
use game_common::{Rng, iso_camera, iso_projection, save};

use crate::GameState;
use crate::vue::{Allure, DeScene, Kit, OndeVue, VueChien, VueLoup, VueMouton, eclat, spawn_scene};

pub const JEU: &str = "berger";

/// Quelle partie on joue.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Genre {
    /// Un niveau de la campagne (la carte est toujours la même).
    Campagne(usize),
    /// La carte du jour : la même pour tout le monde.
    Jour(i64),
    /// Une carte tirée au hasard (niveau de difficulté, graine).
    Hasard(usize, u64),
}

#[derive(Resource)]
pub struct Partie {
    pub monde: Monde,
    pub genre: Genre,
    /// Le décor du menu : un champ tranquille, pas une vraie partie.
    pub demo: bool,
    /// Le chien automatique joue (captures d'écran).
    pub robot: bool,
    pub intro: f32,
    pub alerte: f32,
    pub bouton_aboyer: bool,
    poussiere: f32,
    phase_chien: f32,
}

impl Partie {
    pub fn en_cours(&self) -> bool {
        !self.demo && self.monde.fin.is_none() && self.monde.temps > 1.0
    }
}

#[derive(Message)]
pub struct Commencer {
    pub genre: Genre,
    pub demo: bool,
    pub robot: bool,
}

/// Un événement de la simulation, pour le son et les effets.
#[derive(Message, Clone, Copy)]
pub struct Evt(pub Evenement);

#[derive(Component)]
pub struct MainCamera;

/// Les étoiles gagnées et la série des cartes du jour (sauvegardées).
#[derive(Resource)]
pub struct Progression {
    pub etoiles: [u8; 12],
    pub jour_fait: Option<i64>,
    pub serie: u32,
}

impl Progression {
    pub fn charger() -> Self {
        let mut etoiles = [0u8; 12];
        if let Some(s) = save::load(JEU, "etoiles") {
            for (e, c) in etoiles.iter_mut().zip(s.chars()) {
                *e = c.to_digit(10).unwrap_or(0).min(3) as u8;
            }
        }
        let jour_fait = save::load(JEU, "jour").and_then(|v| v.parse().ok());
        let serie = save::load(JEU, "serie").and_then(|v| v.parse().ok()).unwrap_or(0);
        Progression { etoiles, jour_fait, serie }
    }

    pub fn enregistrer(&self) {
        let s: String = self.etoiles.iter().map(|e| char::from_digit(*e as u32, 10).unwrap_or('0')).collect();
        save::store(JEU, "etoiles", &s);
        if let Some(j) = self.jour_fait {
            save::store(JEU, "jour", &j.to_string());
        }
        save::store(JEU, "serie", &self.serie.to_string());
    }

    /// Un niveau est ouvert quand le précédent est réussi.
    pub fn ouvert(&self, i: usize) -> bool {
        i == 0 || self.etoiles[i - 1] > 0
    }

    /// Le premier niveau pas encore réussi (ou le dernier).
    pub fn suivant(&self) -> usize {
        self.etoiles.iter().position(|&e| e == 0).unwrap_or(NIVEAUX.len() - 1)
    }

    pub fn total(&self) -> u32 {
        self.etoiles.iter().map(|&e| e as u32).sum()
    }

    pub fn jour_reussi(&self) -> bool {
        self.jour_fait == Some(Date::today().days())
    }

    fn victoire(&mut self, genre: Genre, etoiles: u32) {
        match genre {
            Genre::Campagne(i) => self.etoiles[i] = self.etoiles[i].max(etoiles as u8),
            Genre::Jour(j) => {
                if self.jour_fait != Some(j) {
                    self.serie = if self.jour_fait.is_some_and(|p| p + 1 == j) { self.serie + 1 } else { 1 };
                    self.jour_fait = Some(j);
                }
            }
            Genre::Hasard(..) => {}
        }
        self.enregistrer();
    }
}

pub struct JeuPlugin;

impl Plugin for JeuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Kit>()
            .insert_resource(Progression::charger())
            .add_message::<Commencer>()
            .add_message::<Evt>()
            .add_systems(Startup, setup)
            .add_systems(
                Update,
                (demarrer, avancer, synchroniser, effets, crate::vue::animer_allures, crate::vue::animer_particules).chain(),
            );
    }
}

fn setup(mut commands: Commands, mut depart: MessageWriter<Commencer>, progression: Res<Progression>) {
    commands.spawn((iso_camera(Vec3::ZERO, 40.0, 30.0), MainCamera));
    // Le menu s'affiche devant un champ tranquille.
    let robot = std::env::var("BERGER_ROBOT").is_ok();
    depart.write(Commencer { genre: Genre::Campagne(progression.suivant().min(3)), demo: true, robot });
}

/// Zone de champ à garder à l'écran.
fn cadrage(demi: V2) -> Projection {
    iso_projection(2.0 * (0.919 * demi.x + 0.394 * demi.y) + 9.0, 2.0 * (0.301 * demi.x + 0.701 * demi.y) + 8.0)
}

fn demarrer(
    mut commands: Commands,
    mut demandes: MessageReader<Commencer>,
    scene: Query<Entity, With<DeScene>>,
    kit: Res<Kit>,
    mut cam: Query<&mut Projection, With<MainCamera>>,
) {
    let Some(d) = demandes.read().last() else { return };
    for e in &scene {
        commands.entity(e).despawn();
    }
    let (nv, graine) = match d.genre {
        Genre::Campagne(i) => (niveau(i), 1000 + i as u64 * 77),
        Genre::Jour(j) => niveau_du_jour(j),
        Genre::Hasard(i, g) => (niveau(i), g),
    };
    let carte = generer(&nv, graine);
    spawn_scene(&mut commands, &kit, &carte, &nv);
    for mut p in &mut cam {
        *p = cadrage(carte.demi);
    }
    commands.insert_resource(Partie {
        monde: Monde::nouveau(carte, nv),
        genre: d.genre,
        demo: d.demo,
        robot: d.robot,
        intro: 3.5,
        alerte: 0.0,
        bouton_aboyer: false,
        poussiere: 0.0,
        phase_chien: 0.0,
    });
}

/// Le point du sol sous le pointeur (souris ou doigt).
fn point_vise(touches: &Touches, windows: &Query<&Window>, cam: &Query<(&Camera, &GlobalTransform), With<MainCamera>>) -> Option<V2> {
    let (camera, tf) = cam.single().ok()?;
    let pos = touches.iter().next().map(|t| t.position()).or_else(|| windows.single().ok().and_then(|w| w.cursor_position()))?;
    let ray = camera.viewport_to_world(tf, pos).ok()?;
    let t = ray.intersect_plane(Vec3::ZERO, InfinitePlane3d::new(Vec3::Y))?;
    let p = ray.get_point(t);
    Some(v2(p.x, p.z))
}

#[allow(clippy::too_many_arguments)]
fn avancer(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    touches: Res<Touches>,
    windows: Query<&Window>,
    cam: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    partie: Option<ResMut<Partie>>,
    state: Res<State<GameState>>,
    mut next: ResMut<NextState<GameState>>,
    mut progression: ResMut<Progression>,
    mut evts: MessageWriter<Evt>,
) {
    let Some(mut partie) = partie else { return };
    let dt = time.delta_secs();
    let jeu = *state.get() == GameState::Playing;
    let mut cmd = Commande::default();
    if partie.robot && (jeu || partie.demo) {
        cmd = robot::commande(&partie.monde);
    } else if jeu {
        let mut dir = V2::ZERO;
        if keys.any_pressed([KeyCode::ArrowUp, KeyCode::KeyW]) {
            dir.y -= 1.0;
        }
        if keys.any_pressed([KeyCode::ArrowDown, KeyCode::KeyS]) {
            dir.y += 1.0;
        }
        if keys.any_pressed([KeyCode::ArrowLeft, KeyCode::KeyA]) {
            dir.x -= 1.0;
        }
        if keys.any_pressed([KeyCode::ArrowRight, KeyCode::KeyD]) {
            dir.x += 1.0;
        }
        cmd.dir = dir.norm();
        cmd.course = keys.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]);
        cmd.aboyer = keys.any_just_pressed([KeyCode::Space, KeyCode::KeyB]) ||mouse.just_pressed(MouseButton::Right);
        // Souris ou doigt : le chien va vers le pointeur, et court quand il est loin.
        let pointe = mouse.pressed(MouseButton::Left) || touches.iter().next().is_some();
        if pointe && cmd.dir == V2::ZERO {
            if let Some(p) = point_vise(&touches, &windows, &cam) {
                let d = p - partie.monde.chien.pos;
                if d.len() > 0.7 {
                    cmd.dir = d.norm() * (d.len() / 2.5).min(1.0);
                    cmd.course = d.len() > 9.0;
                }
            }
        }
    }
    if jeu && std::mem::take(&mut partie.bouton_aboyer) {
        cmd.aboyer = true;
    }
    partie.intro = (partie.intro - dt).max(0.0);
    partie.alerte = (partie.alerte - dt).max(0.0);
    partie.monde.pas(dt, cmd);
    let evenements = std::mem::take(&mut partie.monde.evenements);
    for e in evenements {
        match e {
            Evenement::LoupArrive(_) => partie.alerte = 4.0,
            Evenement::Fin(f) if jeu => {
                if f == Fin::Victoire {
                    let (genre, etoiles) = (partie.genre, partie.monde.etoiles());
                    progression.victoire(genre, etoiles);
                }
                next.set(GameState::Fin);
            }
            _ => {}
        }
        evts.write(Evt(e));
    }
}

fn tourner(cap: f32, vers: f32, k: f32) -> f32 {
    let mut d = (vers - cap) % TAU;
    if d > PI {
        d -= TAU;
    }
    if d < -PI {
        d += TAU;
    }
    cap + d * k
}

#[allow(clippy::type_complexity)]
fn synchroniser(
    time: Res<Time>,
    partie: Option<ResMut<Partie>>,
    mut commands: Commands,
    kit: Res<Kit>,
    mut moutons: Query<(&VueMouton, &mut Transform, &mut Visibility, &mut Allure), (Without<VueLoup>, Without<VueChien>, Without<OndeVue>)>,
    mut loups: Query<(&VueLoup, &mut Transform, &mut Visibility, &mut Allure), (Without<VueMouton>, Without<VueChien>, Without<OndeVue>)>,
    mut chien: Query<(&mut Transform, &mut Allure), (With<VueChien>, Without<VueMouton>, Without<VueLoup>, Without<OndeVue>)>,
    mut onde: Query<(&mut Transform, &mut Visibility), (With<OndeVue>, Without<VueMouton>, Without<VueLoup>, Without<VueChien>)>,
) {
    let Some(mut partie) = partie else { return };
    let dt = time.delta_secs();
    let k = (dt * 12.0).min(1.0);
    for (v, mut t, mut vis, mut a) in &mut moutons {
        let Some(m) = partie.monde.moutons.get(v.0) else { continue };
        *vis = if m.perdu { Visibility::Hidden } else { Visibility::Inherited };
        t.translation = Vec3::new(m.pos.x, 0.0, m.pos.y);
        if m.vel.len() > 0.12 {
            a.cap = tourner(a.cap, m.vel.x.atan2(m.vel.y), k);
        }
        t.rotation = Quat::from_rotation_y(a.cap);
        a.phase = m.phase * 2.0;
        a.mouvement = (m.vel.len() / 2.2).min(1.0);
        a.agite = m.panique;
    }
    for (v, mut t, mut vis, mut a) in &mut loups {
        let Some(l) = partie.monde.loups.get(v.0) else { continue };
        *vis = if l.etat == berger_core::EtatLoup::Absent { Visibility::Hidden } else { Visibility::Inherited };
        t.translation = Vec3::new(l.pos.x, 0.0, l.pos.y);
        if l.vel.len() > 0.2 {
            a.cap = tourner(a.cap, l.vel.x.atan2(l.vel.y), k);
        }
        t.rotation = Quat::from_rotation_y(a.cap);
        a.phase = l.phase * 2.0;
        a.mouvement = (l.vel.len() / 3.5).min(1.0);
        a.agite = if l.etat == berger_core::EtatLoup::Rode { 1.0 } else { 0.0 };
    }
    let c = partie.monde.chien.clone();
    let vitesse = c.vel.len();
    partie.phase_chien += dt * (4.0 + vitesse * 1.6);
    let phase = partie.phase_chien;
    for (mut t, mut a) in &mut chien {
        t.translation = Vec3::new(c.pos.x, 0.0, c.pos.y);
        a.cap = tourner(a.cap, c.cap_vers(), k);
        t.rotation = Quat::from_rotation_y(a.cap);
        a.phase = phase;
        a.mouvement = (vitesse / 4.5).min(1.0);
        a.agite = if c.onde > 0.0 { 1.0 } else { (vitesse / 6.0).min(1.0) };
    }
    for (mut t, mut vis) in &mut onde {
        if c.onde > 0.0 {
            *vis = Visibility::Inherited;
            t.translation = Vec3::new(c.pos.x, 0.12, c.pos.y);
            t.scale = Vec3::splat(c.onde);
        } else {
            *vis = Visibility::Hidden;
        }
    }
    // Poussière sous les pattes quand le chien court.
    if c.court {
        partie.poussiere -= dt;
        if partie.poussiere <= 0.0 {
            partie.poussiere = 0.05;
            let derriere = Vec3::new(-c.vel.x, 0.0, -c.vel.y).normalize_or_zero();
            let p = Vec3::new(c.pos.x, 0.12, c.pos.y) + derriere * 0.4;
            eclat(&mut commands, &kit, &kit.poussiere, p, derriere * 0.8 + Vec3::Y * 0.6, 0.5, 0.0, 0.26);
        }
    }
}

trait CapChien {
    fn cap_vers(&self) -> f32;
}

impl CapChien for berger_core::monde::Chien {
    /// Angle de la carte converti en cap : le chien regarde dans le sens de la marche.
    fn cap_vers(&self) -> f32 {
        self.vel.x.atan2(self.vel.y)
    }
}

/// Étincelles, laine et confettis selon les événements.
fn effets(mut commands: Commands, kit: Res<Kit>, partie: Option<Res<Partie>>, mut evts: MessageReader<Evt>, mut rng: Local<Option<Rng>>) {
    let Some(partie) = partie else {
        evts.clear();
        return;
    };
    let rng = rng.get_or_insert_with(Rng::from_time);
    for Evt(e) in evts.read() {
        match *e {
            Evenement::Entre(i) => {
                let p = partie.monde.moutons[i].pos;
                for _ in 0..7 {
                    let v = Vec3::new(rng.next_f32() - 0.5, 1.5 + rng.next_f32() * 1.5, rng.next_f32() - 0.5);
                    eclat(&mut commands, &kit, &kit.etoile, Vec3::new(p.x, 0.8, p.y), v, 0.9, 4.0, 0.14);
                }
            }
            Evenement::Mange(i) => {
                let p = partie.monde.moutons[i].pos;
                for _ in 0..14 {
                    let v = Vec3::new(rng.next_f32() - 0.5, 1.0 + rng.next_f32() * 2.0, rng.next_f32() - 0.5) * 2.0;
                    eclat(&mut commands, &kit, &kit.laine[0], Vec3::new(p.x, 0.5, p.y), v, 1.0, 6.0, 0.3);
                }
            }
            Evenement::Fin(Fin::Victoire) => {
                let c = partie.monde.carte.enclos.centre;
                for j in 0..50 {
                    let v = Vec3::new((rng.next_f32() - 0.5) * 7.0, 4.0 + rng.next_f32() * 4.0, (rng.next_f32() - 0.5) * 7.0);
                    eclat(&mut commands, &kit, &kit.fleurs[j % 4], Vec3::new(c.x, 1.0, c.y), v, 1.8, 7.0, 0.2);
                }
            }
            _ => {}
        }
    }
}
