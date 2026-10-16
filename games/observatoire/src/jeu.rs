//! La partie : la grille en cours, les symboles posés, l'historique, le
//! sablier, les erreurs (mode Novice), les indices et la victoire ; plus le
//! Livre des Constellations et la progression enregistrée.
use bevy::prelude::*;
use game_common::calendar::Date;
use game_common::langue::{Langue, actuelle};
use game_common::{iso_projection, tx};
use observatoire_core::{Grille, LUNE, SOLEIL, VIDE, erreurs, next_deduction};

use crate::GameState;
use crate::plateau::{self, DeLaGrille, Look, Marque, pos_case};

const SAVE: &str = "observatoire";
/// La caméra regarde la carte d'en haut, un peu de biais.
const CAMERA: Vec3 = Vec3::new(0.0, 15.0, 4.5);

pub struct JeuPlugin;

impl Plugin for JeuPlugin {
    fn build(&self, app: &mut App) {
        let livre = Livre::charger();
        let progression = Progression::charger(livre.0.len());
        app.insert_resource(livre)
            .insert_resource(progression)
            .init_resource::<Look>()
            .add_message::<JeuEvenement>()
            .add_systems(Startup, preparer)
            .add_systems(Update, (poser, sablier, victoire).chain().run_if(in_state(GameState::Playing)))
            // La carte reste visible derrière le menu et l'écran de victoire.
            .add_systems(Update, (rebatir, symboles).chain().after(victoire).run_if(resource_exists::<Partie>))
            .add_systems(Update, (plateau::vaciller, cadrer));
    }
}

// ---- Le livre et la progression ---------------------------------------------

#[derive(Resource)]
pub struct Livre(pub Vec<Grille>);

impl Livre {
    fn charger() -> Livre {
        Livre(include_str!("../grilles.txt").lines().filter(|l| !l.starts_with('#')).filter_map(Grille::from_line).collect())
    }
}

#[derive(Resource)]
pub struct Progression {
    pub resolues: Vec<bool>,
    pub meilleurs: Vec<Option<u32>>,
    pub novice: bool,
    pub jour_fait: Option<Date>,
    pub serie: u32,
}

impl Progression {
    fn charger(n: usize) -> Progression {
        let l = |cle: &str| game_common::save::load(SAVE, cle);
        let mut p = Progression { resolues: vec![false; n], meilleurs: vec![None; n], novice: l("novice").as_deref() != Some("0"), jour_fait: l("jour").and_then(|d| Date::parse(&d)), serie: l("serie").and_then(|s| s.parse().ok()).unwrap_or(0) };
        if let Some(v) = l("resolues") {
            for (i, x) in v.split_whitespace().enumerate().take(n) {
                if let Ok(t) = x.parse::<u32>() {
                    p.resolues[i] = true;
                    p.meilleurs[i] = (t > 0).then_some(t);
                }
                // « - » : pas encore résolue.
            }
        }
        p
    }

    pub fn enregistrer(&self) {
        let s = |cle: &str, v: String| game_common::save::store(SAVE, cle, &v);
        let r: Vec<String> = self.resolues.iter().zip(&self.meilleurs).map(|(ok, t)| if *ok { t.unwrap_or(0).to_string() } else { "-".into() }).collect();
        s("resolues", r.join(" "));
        s("novice", if self.novice { "1" } else { "0" }.into());
        if let Some(d) = self.jour_fait {
            s("jour", format!("{:04}-{:02}-{:02}", d.year, d.month, d.day));
        }
        s("serie", self.serie.to_string());
    }

    /// La première grille pas encore résolue.
    pub fn suivante(&self) -> Option<usize> {
        self.resolues.iter().position(|r| !r)
    }

    pub fn jour_reussi(&self) -> bool {
        self.jour_fait == Some(Date::today())
    }
}

// ---- La grille en cours -------------------------------------------------------

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Genre {
    Livre(usize),
    Jour(Date),
}

#[derive(Message, Clone, Copy, PartialEq, Eq)]
pub enum JeuEvenement {
    Pose,
    Efface,
    Erreur,
    Indice,
    Victoire,
}

/// L'indice affiché : la case à remplir, les cases qui l'expliquent, le texte.
#[derive(Clone)]
pub struct Indice {
    pub case: usize,
    pub raison: Vec<usize>,
    pub texte: String,
}

#[derive(Resource)]
pub struct Partie {
    pub grille: Grille,
    pub genre: Genre,
    pub cases: Vec<u8>,
    historique: Vec<Vec<u8>>,
    pub temps: f32,
    pub gagne: bool,
    construite: bool,
    dessinee: bool,
    pub indice: Option<Indice>,
}

impl Partie {
    pub fn n(&self) -> usize {
        self.grille.n
    }

    fn retenir(&mut self) {
        self.historique.push(self.cases.clone());
        if self.historique.len() > 300 {
            self.historique.remove(0);
        }
    }

    pub fn redessiner(&mut self) {
        self.dessinee = false;
    }

    pub fn annuler(&mut self) {
        if let Some(c) = self.historique.pop() {
            self.cases = c;
            self.indice = None;
            self.dessinee = false;
        }
    }

    /// Efface tout ce que le joueur a posé (les cases données restent).
    pub fn effacer(&mut self) {
        self.retenir();
        self.cases = self.grille.donnees.clone();
        self.indice = None;
        self.dessinee = false;
    }

    pub fn erreurs(&self) -> Vec<usize> {
        erreurs(self.n(), &self.grille.signes, &self.cases)
    }

    /// Calcule l'indice : un symbole faux d'abord, sinon la prochaine déduction.
    pub fn calculer_indice(&mut self) {
        let n = self.n();
        let faux = (0..n * n).find(|&i| self.cases[i] != VIDE && self.cases[i] != self.grille.solution[i]);
        self.indice = if let Some(i) = faux {
            Some(Indice {
                case: i,
                raison: vec![],
                texte: tx!(
                    "Ce symbole n'est pas à sa place : changez-le ou effacez-le.",
                    "This symbol isn't in the right place: change or clear it.",
                    "Dieses Symbol steht falsch: Ändere oder lösche es.",
                    "Questo simbolo non è al suo posto: cambialo o cancellalo.",
                    "Este símbolo no va aquí: cámbialo o bórralo."
                )
                .t()
                .to_string(),
            })
        } else {
            next_deduction(n, &self.grille.signes, &self.cases).map(|d| {
                let langue = Langue::TOUTES.iter().position(|l| *l == actuelle()).unwrap_or(0);
                Indice { case: d.case, raison: d.raison, texte: d.technique.explain(langue).to_string() }
            })
        };
        self.dessinee = false;
    }
}

/// Commence une grille.
pub fn commencer(commands: &mut Commands, grille: Grille, genre: Genre) {
    let cases = grille.donnees.clone();
    commands.insert_resource(Partie { grille, genre, cases, historique: Vec::new(), temps: 0.0, gagne: false, construite: false, dessinee: false, indice: None });
}

#[derive(Component)]
pub struct Camera;

fn preparer(
    mut commands: Commands,
    look: Res<Look>,
    livre: Res<Livre>,
    progression: Res<Progression>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<StandardMaterial>>,
) {
    // Derrière le menu : la prochaine grille du livre.
    let i = progression.suivante().unwrap_or(0);
    commencer(&mut commands, livre.0[i].clone(), Genre::Livre(i));
    commands.spawn((Camera3d::default(), iso_projection(12.0, 10.0), Transform::from_translation(CAMERA).looking_at(Vec3::ZERO, Vec3::Y), Camera));
    commands.spawn((
        DirectionalLight { illuminance: 4_500.0, shadow_maps_enabled: true, ..default() },
        Transform::from_xyz(-4.0, 12.0, 6.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    plateau::bureau(&mut commands, &look, &mut meshes, &mut mats);
}

/// Une nouvelle grille : on refait la carte et on cadre la caméra.
fn rebatir(
    mut commands: Commands,
    mut partie: ResMut<Partie>,
    look: Res<Look>,
    mut meshes: ResMut<Assets<Mesh>>,
    anciens: Query<Entity, With<DeLaGrille>>,
    mut cam: Query<&mut Projection, With<Camera>>,
) {
    if partie.construite {
        return;
    }
    partie.construite = true;
    partie.dessinee = false;
    anciens.iter().for_each(|e| commands.entity(e).despawn());
    let n = partie.n();
    plateau::carte(&mut commands, &look, &mut meshes, n, &partie.grille.donnees, &partie.grille.signes);
    for mut p in &mut cam {
        // Assez large pour voir le bureau : lunette, globe et chandelle.
        *p = iso_projection(n as f32 + 16.0, n as f32 + 4.2);
    }
}

/// Redessine les symboles, les erreurs et l'indice quand quelque chose change.
fn symboles(mut commands: Commands, mut partie: ResMut<Partie>, progression: Res<Progression>, look: Res<Look>, marques: Query<Entity, With<Marque>>) {
    if partie.dessinee {
        return;
    }
    partie.dessinee = true;
    marques.iter().for_each(|e| commands.entity(e).despawn());
    let n = partie.n();
    for i in 0..n * n {
        if partie.cases[i] != VIDE {
            plateau::symbole(&mut commands, &look, n, i, partie.cases[i], partie.grille.donnees[i] != VIDE);
        }
    }
    let disque = |c: &mut Commands, i: usize, mat: &Handle<StandardMaterial>, taille: f32| {
        c.spawn((
            Mesh3d(look.disque.clone()),
            MeshMaterial3d(mat.clone()),
            Transform::from_translation(pos_case(n, i) + Vec3::Y * 0.005).with_scale(Vec3::new(taille, 0.01, taille)),
            Marque,
            DeLaGrille,
        ));
    };
    if progression.novice && !partie.gagne {
        for i in partie.erreurs() {
            disque(&mut commands, i, &look.erreur, 0.86);
        }
    }
    if let Some(ind) = partie.indice.clone() {
        for i in ind.raison {
            disque(&mut commands, i, &look.indice_pale, 0.8);
        }
        // Un cadre lumineux autour de la case à remplir.
        let p = pos_case(n, ind.case) + Vec3::Y * 0.03;
        for (d, e) in [(Vec3::new(0.0, 0.0, 0.44), Vec3::new(0.92, 0.03, 0.05)), (Vec3::new(0.0, 0.0, -0.44), Vec3::new(0.92, 0.03, 0.05)), (Vec3::new(0.44, 0.0, 0.0), Vec3::new(0.05, 0.03, 0.92)), (Vec3::new(-0.44, 0.0, 0.0), Vec3::new(0.05, 0.03, 0.92))] {
            commands.spawn((Mesh3d(look.cube.clone()), MeshMaterial3d(look.indice.clone()), Transform::from_translation(p + d).with_scale(e), Marque, DeLaGrille));
        }
    }
}

/// La case sous la souris (ou le doigt) : on vise le plan de la carte.
fn case_visee(n: usize, camera: &bevy::camera::Camera, cam: &GlobalTransform, ecran: Vec2) -> Option<usize> {
    let rayon = camera.viewport_to_world(cam, ecran).ok()?;
    let d = rayon.intersect_plane(Vec3::Y * plateau::DESSUS, InfinitePlane3d::new(Vec3::Y))?;
    let p = rayon.get_point(d);
    let m = n as f32 / 2.0;
    let (c, r) = ((p.x + m).floor(), (p.z + m).floor());
    (c >= 0.0 && r >= 0.0 && c < n as f32 && r < n as f32).then(|| r as usize * n + c as usize)
}

fn poser(
    mouse: Res<ButtonInput<MouseButton>>,
    touches: Res<Touches>,
    windows: Query<&Window>,
    cam: Query<(&bevy::camera::Camera, &GlobalTransform), With<Camera>>,
    mut partie: ResMut<Partie>,
    progression: Res<Progression>,
    mut evenements: MessageWriter<JeuEvenement>,
) {
    if partie.gagne {
        return;
    }
    let Ok((camera, cam_tf)) = cam.single() else { return };
    let (sens, pos) = if let Some(t) = touches.iter_just_pressed().next() {
        (1, Some(t.position()))
    } else if mouse.just_pressed(MouseButton::Left) {
        (1, windows.single().ok().and_then(|w| w.cursor_position()))
    } else if mouse.just_pressed(MouseButton::Right) {
        (-1, windows.single().ok().and_then(|w| w.cursor_position()))
    } else {
        return;
    };
    let n = partie.n();
    let Some(i) = pos.and_then(|p| case_visee(n, camera, cam_tf, p)) else { return };
    if partie.grille.donnees[i] != VIDE {
        return;
    }
    partie.retenir();
    // Vide → soleil → lune → vide (et l'inverse au clic droit).
    let ordre = [VIDE, SOLEIL, LUNE];
    let k = ordre.iter().position(|v| *v == partie.cases[i]).unwrap_or(0);
    let neuf = ordre[(k as i32 + sens).rem_euclid(3) as usize];
    partie.cases[i] = neuf;
    partie.indice = None;
    partie.dessinee = false;
    evenements.write(if neuf == VIDE { JeuEvenement::Efface } else { JeuEvenement::Pose });
    if progression.novice && partie.erreurs().contains(&i) {
        evenements.write(JeuEvenement::Erreur);
    }
}

fn sablier(time: Res<Time>, mut partie: ResMut<Partie>) {
    if !partie.gagne {
        partie.temps += time.delta_secs();
    }
}

/// La grille est remplie et juste : victoire.
fn victoire(mut partie: ResMut<Partie>, mut progression: ResMut<Progression>, mut evenements: MessageWriter<JeuEvenement>, mut next: ResMut<NextState<GameState>>) {
    if partie.gagne || !partie.grille.est_resolue_par(&partie.cases) {
        return;
    }
    partie.gagne = true;
    partie.indice = None;
    partie.dessinee = false;
    let t = partie.temps as u32;
    match partie.genre {
        Genre::Livre(i) => {
            progression.resolues[i] = true;
            progression.meilleurs[i] = Some(progression.meilleurs[i].map_or(t, |b| b.min(t)));
        }
        Genre::Jour(d) => {
            if progression.jour_fait != Some(d) {
                progression.serie = if progression.jour_fait == Some(d.plus(-1)) { progression.serie + 1 } else { 1 };
                progression.jour_fait = Some(d);
            }
        }
    }
    progression.enregistrer();
    evenements.write(JeuEvenement::Victoire);
    next.set(GameState::Gagne);
}

/// Au menu, la carte glisse à droite pour laisser la place au panneau.
fn cadrer(time: Res<Time>, state: Res<State<GameState>>, mut decalage: Local<f32>, mut q: Query<&mut Transform, With<Camera>>) {
    let voulu = match state.get() {
        GameState::Menu | GameState::Gagne => -3.2,
        GameState::Livre => -6.0,
        GameState::Playing => 0.0,
    };
    *decalage += (voulu - *decalage) * (4.0 * time.delta_secs()).min(1.0);
    let regard = Vec3::new(*decalage, 0.0, 0.0);
    for mut tr in &mut q {
        *tr = Transform::from_translation(regard + CAMERA).looking_at(regard, Vec3::Y);
    }
}
