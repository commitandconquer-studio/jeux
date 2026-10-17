//! Le générateur de sprites : `CA_FUIT_SPRITES=dossier cargo run -p ca-fuit`
//! photographie chaque modèle 3D vu du dessus (les 34 cartes, vides et
//! pleines d'eau, la fuite, la flaque, le décor), sur fond noir puis blanc.
//! `tools/sprites-ca-fuit.py` en déduit la transparence exacte et dépose les
//! PNG dans `assets/sprites/ca-fuit/`. Le jeu lui-même est en 2D.
use bevy::camera::ScalingMode;
use bevy::prelude::*;
use bevy::render::view::screenshot::{Screenshot, save_to_disk};
use bevy::window::WindowResolution;
use fuite_core::{Carte, Forme, Rotation};

use crate::modeles::{self, Eau, Look};

/// Pixels (logiques) par case dans la photo.
const PX: f32 = 300.0;

#[derive(Clone)]
enum Sujet {
    /// Le premier sujet photographié sort toujours vide : celui-ci est jeté.
    Chauffe,
    Carte(Forme, bool),
    Fuite,
    Flaque,
    Arbre,
    Touffe(bool),
}

impl Sujet {
    fn nom(&self) -> String {
        match self {
            Sujet::Chauffe => "chauffe".into(),
            Sujet::Carte(f, eau) => format!("carte-{}{}", f.nom(), if *eau { "-eau" } else { "" }),
            Sujet::Fuite => "fuite".into(),
            Sujet::Flaque => "flaque".into(),
            Sujet::Arbre => "arbre".into(),
            Sujet::Touffe(false) => "touffe".into(),
            Sujet::Touffe(true) => "touffe-fleur".into(),
        }
    }

    /// Taille de la photo en cases (largeur, hauteur) et centre au sol (x, z).
    fn cadre(&self) -> ((f32, f32), (f32, f32)) {
        match self {
            Sujet::Chauffe | Sujet::Carte(..) => ((1.0, 2.0), (0.0, -0.5)),
            Sujet::Fuite => ((4.0, 3.0), (0.5, 0.0)),
            Sujet::Flaque => ((1.0, 1.0), (0.0, 0.0)),
            Sujet::Arbre => ((3.0, 3.0), (0.0, 0.0)),
            Sujet::Touffe(_) => ((1.0, 1.0), (0.0, 0.0)),
        }
    }
}

#[derive(Resource)]
struct Generation {
    dossier: String,
    sujets: Vec<Sujet>,
    i: usize,
    /// 0 = noir, 1 = blanc.
    fond: usize,
    image: u32,
    attente: u32,
}

#[derive(Component)]
struct DuSujet;

#[derive(Component)]
struct CameraPhoto;

pub fn generer(dossier: String) {
    let mut sujets = vec![Sujet::Chauffe];
    for f in Forme::toutes() {
        sujets.push(Sujet::Carte(f, false));
        sujets.push(Sujet::Carte(f, true));
    }
    sujets.extend([Sujet::Fuite, Sujet::Flaque, Sujet::Arbre, Sujet::Touffe(false), Sujet::Touffe(true)]);
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window { title: "Sprites".into(), resolution: WindowResolution::new(PX as u32, 2 * PX as u32), ..default() }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(GlobalAmbientLight { color: Color::WHITE, brightness: 900.0, ..default() })
        .insert_resource(Generation { dossier, sujets, i: 0, fond: 0, image: 0, attente: 0 })
        .init_resource::<Look>()
        .add_systems(Startup, lumiere)
        .add_systems(Update, photographier)
        .run();
}

fn lumiere(mut commands: Commands) {
    commands.spawn((DirectionalLight { illuminance: 6_000.0, shadow_maps_enabled: false, ..default() }, Transform::from_xyz(-6.0, 14.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y)));
    commands.spawn((Camera3d::default(), Projection::from(OrthographicProjection { scaling_mode: ScalingMode::FixedVertical { viewport_height: 2.0 }, ..OrthographicProjection::default_3d() }), Transform::from_xyz(0.0, 30.0, -0.5).looking_at(Vec3::new(0.0, 0.0, -0.5), Vec3::NEG_Z), CameraPhoto));
}

#[allow(clippy::too_many_arguments)]
fn photographier(
    mut commands: Commands,
    mut generation: ResMut<Generation>,
    look: Res<Look>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut fond: ResMut<ClearColor>,
    mut windows: Query<&mut Window>,
    mut camera: Query<(&mut Transform, &mut Projection), With<CameraPhoto>>,
    du_sujet: Query<Entity, With<DuSujet>>,
    mut eaux: Query<(&Eau, &mut Transform, &mut Visibility), Without<CameraPhoto>>,
    mut exit: MessageWriter<AppExit>,
) {
    let Some(sujet) = generation.sujets.get(generation.i).cloned() else {
        exit.write(AppExit::Success);
        return;
    };
    let ((l, h), (cx, cz)) = sujet.cadre();
    // Les toutes premières images de l'application ne rendent rien : on attend.
    if generation.attente < 40 {
        generation.attente += 1;
        return;
    }
    generation.image += 1;
    match generation.image {
        1 => {
            // La fenêtre à la taille du sujet, la caméra au-dessus, et le modèle.
            for mut w in &mut windows {
                w.resolution.set(l * PX, h * PX);
            }
            for (mut tf, mut proj) in &mut camera {
                *tf = Transform::from_xyz(cx, 30.0, cz).looking_at(Vec3::new(cx, 0.0, cz), Vec3::NEG_Z);
                *proj = Projection::from(OrthographicProjection { scaling_mode: ScalingMode::FixedVertical { viewport_height: h }, ..OrthographicProjection::default_3d() });
            }
            let racine = commands.spawn((Transform::default(), Visibility::default(), DuSujet)).id();
            match sujet {
                Sujet::Chauffe => modeles::dessiner_carte(&mut commands, racine, &look, &Carte::new(Forme::de(&[1, 4]), Rotation::R0, (0, 0)), None),
                Sujet::Carte(f, _) => modeles::dessiner_carte(&mut commands, racine, &look, &Carte::new(f, Rotation::R0, (0, 0)), None),
                Sujet::Fuite => modeles::dessiner_fuite(&mut commands, racine, &look, &Carte::new(Forme::FUITE, Rotation::R90, (0, 0))),
                Sujet::Flaque => {
                    commands.spawn((Mesh3d(look.disque.clone()), MeshMaterial3d(look.flaque.clone()), Transform::from_xyz(0.0, 0.012, 0.0).with_scale(Vec3::new(0.9, 0.02, 0.72)), ChildOf(racine)));
                }
                Sujet::Arbre => modeles::dessiner_arbre(&mut commands, racine, &look, &mut meshes, 1.0),
                Sujet::Touffe(fleur) => modeles::dessiner_touffe(&mut commands, racine, &look, &mut meshes, 0, fleur),
            }
            *fond = ClearColor(if generation.fond == 0 { Color::BLACK } else { Color::WHITE });
        }
        12 => {
            // L'eau à ras bord pour la version pleine (et la fuite est toujours pleine).
            if matches!(sujet, Sujet::Carte(_, true) | Sujet::Fuite) {
                for (eau, mut tf, mut vis) in &mut eaux {
                    *vis = Visibility::Inherited;
                    tf.translation = eau.base + Vec3::Y * eau.taille.y / 2.0;
                    tf.scale = Vec3::new(eau.taille.x, eau.taille.y, eau.taille.z);
                }
            }
        }
        20 => {
            let nom = format!("{}/{}-{}.png", generation.dossier, sujet.nom(), if generation.fond == 0 { "noir" } else { "blanc" });
            commands.spawn(Screenshot::primary_window()).observe(save_to_disk(nom));
        }
        28 => {
            for e in &du_sujet {
                commands.entity(e).despawn();
            }
            generation.image = 0;
            if generation.fond == 0 {
                generation.fond = 1;
            } else {
                generation.fond = 0;
                generation.i += 1;
            }
        }
        _ => {}
    }
}
