//! La partie : le numéro, le paquet, la grille, la carte à poser et son sens,
//! la case visée, les records par numéro.
use bevy::prelude::*;
use game_common::iso_projection;
use passage_core::{Card, DECK_SIZE, Game, N, Network};

use crate::GameState;
use crate::plateau::{self, Carte, DESSUS, Look};

const SAVE: &str = "passage-a-niveau";
/// La caméra regarde la table d'en haut, un peu de biais.
const CAMERA: Vec3 = Vec3::new(0.0, 15.0, 4.5);
/// Les plus grands numéros qu'on peut taper (neuf chiffres).
pub const NUMERO_MAX: u64 = 999_999_999;

pub struct JeuPlugin;

impl Plugin for JeuPlugin {
    fn build(&self, app: &mut App) {
        let progres = Progres::charger();
        app.insert_resource(Partie::nouvelle(progres.numero))
            .insert_resource(progres)
            .init_resource::<Look>()
            .add_message::<JeuEvenement>()
            .add_systems(Startup, preparer)
            .add_systems(Update, (viser, tourner, poser, fin).chain().run_if(in_state(GameState::Playing)))
            // La table reste visible derrière le menu et l'écran de fin.
            .add_systems(Update, (redessiner, cadrer).chain().after(fin));
    }
}

#[derive(Message, Clone, Copy, PartialEq, Eq)]
pub enum JeuEvenement {
    Pose,
    Tourne,
    Fin,
}

/// Le numéro choisi au menu et les meilleurs scores de chaque numéro joué.
#[derive(Resource)]
pub struct Progres {
    pub numero: u64,
    records: Vec<(u64, i32)>,
}

impl Progres {
    fn charger() -> Progres {
        let l = |cle: &str| game_common::save::load(SAVE, cle);
        let records = l("records")
            .map(|s| {
                s.split_whitespace()
                    .filter_map(|p| {
                        let (n, v) = p.split_once(':')?;
                        Some((n.parse().ok()?, v.parse().ok()?))
                    })
                    .collect()
            })
            .unwrap_or_default();
        Progres { numero: l("numero").and_then(|v| v.parse().ok()).unwrap_or(1), records }
    }

    fn enregistrer(&self) {
        let r: Vec<String> = self.records.iter().map(|(n, v)| format!("{n}:{v}")).collect();
        game_common::save::store(SAVE, "records", &r.join(" "));
        game_common::save::store(SAVE, "numero", &self.numero.to_string());
    }

    pub fn choisir(&mut self, numero: u64) {
        self.numero = numero.min(NUMERO_MAX);
        self.enregistrer();
    }

    pub fn record(&self, numero: u64) -> Option<i32> {
        self.records.iter().find(|(n, _)| *n == numero).map(|(_, v)| *v)
    }

    /// Retient le score s'il bat le record du numéro. Vrai si c'est un nouveau record.
    pub fn proposer(&mut self, numero: u64, score: i32) -> bool {
        match self.records.iter_mut().find(|(n, _)| *n == numero) {
            Some((_, v)) if *v >= score => return false,
            Some((_, v)) => *v = score,
            None => self.records.push((numero, score)),
        }
        self.enregistrer();
        true
    }
}

/// La partie en cours.
#[derive(Resource)]
pub struct Partie {
    pub jeu: Game,
    /// Le sens de la carte à poser (quarts de tour).
    pub rot: u8,
    /// La case sous la souris.
    pub survol: Option<usize>,
    /// Ce que rapporterait la carte posée sur la case visée.
    pub gain: Option<i32>,
    /// Faux dès que quelque chose change : la table est redessinée.
    pub dessinee: bool,
    pub nouveau_record: bool,
}

impl Partie {
    pub fn nouvelle(numero: u64) -> Partie {
        Partie { jeu: Game::new(numero), rot: 0, survol: None, gain: None, dessinee: false, nouveau_record: false }
    }

    pub fn carte(&self) -> Option<Card> {
        self.jeu.current().map(|piece| Card { piece, rot: self.rot })
    }

    pub fn annuler(&mut self) {
        if self.jeu.undo() {
            self.dessinee = false;
        }
    }

    pub fn tourner(&mut self) {
        self.rot = (self.rot + 1) % 4;
        self.dessinee = false;
    }

    fn calculer_gain(&mut self) {
        self.gain = self.survol.filter(|&i| self.jeu.board.cells[i].is_none()).and_then(|i| {
            let avant = self.jeu.score().total();
            let mut essai = self.jeu.clone();
            essai.play(i, self.rot).then(|| essai.score().total() - avant)
        });
    }
}

#[derive(Component)]
pub struct Camera;

fn preparer(mut commands: Commands, look: Res<Look>, mut meshes: ResMut<Assets<Mesh>>, mut mats: ResMut<Assets<StandardMaterial>>) {
    // Sans tonemapping : les couleurs unies restent franches.
    commands.spawn((Camera3d::default(), bevy::core_pipeline::tonemapping::Tonemapping::None, iso_projection(17.0, 11.0), Transform::from_translation(CAMERA).looking_at(Vec3::ZERO, Vec3::Y), Camera));
    commands.spawn((
        DirectionalLight { illuminance: 4_500.0, shadow_maps_enabled: true, ..default() },
        Transform::from_xyz(-4.0, 12.0, 6.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    plateau::decor(&mut commands, &look, &mut meshes, &mut mats);
}

/// La case sous la souris (ou le doigt) : on vise le plan de la table.
fn case_visee(camera: &bevy::camera::Camera, cam: &GlobalTransform, ecran: Vec2) -> Option<usize> {
    let rayon = camera.viewport_to_world(cam, ecran).ok()?;
    let d = rayon.intersect_plane(Vec3::Y * DESSUS, InfinitePlane3d::new(Vec3::Y))?;
    let p = rayon.get_point(d);
    let m = N as f32 / 2.0;
    let (c, r) = ((p.x + m).floor(), (p.z + m).floor());
    (c >= 0.0 && r >= 0.0 && c < N as f32 && r < N as f32).then(|| r as usize * N + c as usize)
}

fn viser(windows: Query<&Window>, cam: Query<(&bevy::camera::Camera, &GlobalTransform), With<Camera>>, mut partie: ResMut<Partie>) {
    let Ok((camera, cam_tf)) = cam.single() else { return };
    let survol = windows.single().ok().and_then(|w| w.cursor_position()).and_then(|p| case_visee(camera, cam_tf, p));
    if survol != partie.survol {
        partie.survol = survol;
        partie.calculer_gain();
        partie.dessinee = false;
    }
}

fn tourner(
    mouse: Res<ButtonInput<MouseButton>>,
    keys: Res<ButtonInput<KeyCode>>,
    mut molette: MessageReader<bevy::input::mouse::MouseWheel>,
    mut partie: ResMut<Partie>,
    mut evenements: MessageWriter<JeuEvenement>,
) {
    let roulee = molette.read().any(|m| m.y != 0.0);
    if mouse.just_pressed(MouseButton::Right) || keys.any_just_pressed([KeyCode::KeyR, KeyCode::Space]) || roulee {
        partie.tourner();
        partie.calculer_gain();
        evenements.write(JeuEvenement::Tourne);
    }
}

fn poser(
    mouse: Res<ButtonInput<MouseButton>>,
    touches: Res<Touches>,
    windows: Query<&Window>,
    cam: Query<(&bevy::camera::Camera, &GlobalTransform), With<Camera>>,
    mut partie: ResMut<Partie>,
    mut evenements: MessageWriter<JeuEvenement>,
) {
    let Ok((camera, cam_tf)) = cam.single() else { return };
    let pos = if let Some(t) = touches.iter_just_pressed().next() {
        Some(t.position())
    } else if mouse.just_pressed(MouseButton::Left) {
        windows.single().ok().and_then(|w| w.cursor_position())
    } else {
        return;
    };
    let Some(i) = pos.and_then(|p| case_visee(camera, cam_tf, p)) else { return };
    let rot = partie.rot;
    if partie.jeu.play(i, rot) {
        partie.survol = Some(i);
        partie.calculer_gain();
        partie.dessinee = false;
        evenements.write(JeuEvenement::Pose);
    }
}

/// Le paquet est épuisé : on enregistre le score et on passe à l'écran de fin.
fn fin(mut partie: ResMut<Partie>, mut progres: ResMut<Progres>, mut evenements: MessageWriter<JeuEvenement>, mut next: ResMut<NextState<GameState>>) {
    if !partie.jeu.is_finished() {
        return;
    }
    let (numero, total) = (partie.jeu.number, partie.jeu.score().total());
    partie.nouveau_record = progres.proposer(numero, total);
    partie.survol = None;
    partie.gain = None;
    partie.dessinee = false;
    evenements.write(JeuEvenement::Fin);
    next.set(GameState::Fini);
}

/// Redessine les cartes, les bouts dans le vide, le fantôme et la file quand quelque chose change.
fn redessiner(mut commands: Commands, mut partie: ResMut<Partie>, look: Res<Look>, anciennes: Query<Entity, With<Carte>>) {
    if partie.dessinee {
        return;
    }
    partie.dessinee = true;
    anciennes.iter().for_each(|e| commands.entity(e).despawn());
    let board = &partie.jeu.board;
    for (i, card) in board.cells.iter().enumerate() {
        let Some(card) = *card else { continue };
        plateau::carte(&mut commands, &look, card, plateau::pos_case(i), 1.0, i / N + i % N, ());
        for net in Network::ALL {
            for side in 0..4 {
                if card.sides(net) & (1 << side) != 0 && !board.is_linked(i, side, net) {
                    plateau::bout_dans_le_vide(&mut commands, &look, i, side);
                }
            }
        }
    }
    // La carte à poser, en fantôme sur la case visée.
    if let (Some(i), Some(card)) = (partie.survol.filter(|&i| board.cells[i].is_none()), partie.carte()) {
        if partie.jeu.current().is_some() {
            plateau::carte(&mut commands, &look, card, plateau::pos_case(i) + Vec3::Y * 0.12, 1.0, i / N + i % N, ());
            plateau::cadre(&mut commands, &look, plateau::pos_case(i) + Vec3::Y * 0.12);
        }
    }
    // La file : la carte à poser en grand, puis les deux suivantes.
    if let Some(card) = partie.carte() {
        let x = N as f32 / 2.0 + 2.6;
        plateau::carte(&mut commands, &look, card, Vec3::new(x, DESSUS, -2.0), 1.5, 0, ());
        for (k, piece) in partie.jeu.upcoming().iter().enumerate() {
            plateau::carte(&mut commands, &look, Card::new(*piece), Vec3::new(x, DESSUS, 0.5 + k as f32 * 1.3), 0.95, 1, ());
        }
    }
}

/// Au menu et à la fin, la table glisse à droite pour laisser la place au panneau.
fn cadrer(time: Res<Time>, state: Res<State<GameState>>, mut decalage: Local<f32>, mut q: Query<&mut Transform, With<Camera>>) {
    let voulu = match state.get() {
        GameState::Menu | GameState::Fini => -3.4,
        GameState::Playing => 2.2,
    };
    *decalage += (voulu - *decalage) * (4.0 * time.delta_secs()).min(1.0);
    let regard = Vec3::new(*decalage, 0.0, 0.0);
    for mut tr in &mut q {
        *tr = Transform::from_translation(regard + CAMERA).looking_at(regard, Vec3::Y);
    }
}

/// Commence une nouvelle partie du numéro donné.
pub fn commencer(commands: &mut Commands, progres: &mut Progres, numero: u64) {
    progres.choisir(numero);
    commands.insert_resource(Partie::nouvelle(numero.min(NUMERO_MAX)));
}

/// Le nombre de cartes d'une partie (pour l'affichage).
pub const CARTES: usize = DECK_SIZE;
