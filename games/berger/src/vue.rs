//! Tout ce qu'on voit : le champ, les arbres, les rochers, les mares, les
//! clôtures, l'enclos, le chien, les moutons et les loups. Des formes simples
//! à couleurs unies (le style maison) assemblées à la main.
use std::f32::consts::{FRAC_PI_2, TAU};

use berger_core::{Carte, Genre, GenreLoup, Mur, Niveau, TypeMouton};
use bevy::light::NotShadowCaster;
use bevy::prelude::*;
use game_common::{Rng, flat_material, palette};

/// Tout ce qui dépend de la partie en cours (refait à chaque nouvelle partie).
#[derive(Component)]
pub struct DeScene;

#[derive(Component)]
pub struct VueMouton(pub usize);
#[derive(Component)]
pub struct VueLoup(pub usize);
#[derive(Component)]
pub struct VueBelier(pub usize);
#[derive(Component)]
pub struct VueChien;
#[derive(Component)]
pub struct OndeVue;

/// Un animal en mouvement : sa phase de marche et son allure (0 à 1).
#[derive(Component, Default)]
pub struct Allure {
    pub phase: f32,
    pub mouvement: f32,
    pub cap: f32,
    pub agite: f32,
    /// De 0 à 1 : petit saut de surprise.
    pub saut: f32,
    /// De 0 à 1 : la tête se baisse (le bélier prépare sa charge).
    pub tete: f32,
}

#[derive(Component)]
pub struct Patte {
    pub decalage: f32,
}
#[derive(Component)]
pub struct Corps {
    pub base: f32,
}
#[derive(Component)]
pub struct Queue {
    pub vitesse: f32,
}

/// Formes et couleurs partagées (créées une fois).
#[derive(Resource)]
pub struct Kit {
    pub cube: Handle<Mesh>,
    pub boule: Handle<Mesh>,
    pub cone: Handle<Mesh>,
    pub cyl: Handle<Mesh>,
    pub anneau: Handle<Mesh>,
    pub disque: Handle<Mesh>,
    pub laine: [Handle<StandardMaterial>; 3],
    pub laine_noire: Handle<StandardMaterial>,
    pub laine_belier: Handle<StandardMaterial>,
    pub peau: Handle<StandardMaterial>,
    pub fonce: Handle<StandardMaterial>,
    pub noir: Handle<StandardMaterial>,
    pub chien_pelage: Handle<StandardMaterial>,
    pub chien_accent: Handle<StandardMaterial>,
    pub blanc: Handle<StandardMaterial>,
    pub gris: Handle<StandardMaterial>,
    pub roux: Handle<StandardMaterial>,
    pub gris_fonce: Handle<StandardMaterial>,
    pub rouge: Handle<StandardMaterial>,
    pub bois: Handle<StandardMaterial>,
    pub bois_clair: Handle<StandardMaterial>,
    pub paille: Handle<StandardMaterial>,
    pub paille_fonce: Handle<StandardMaterial>,
    pub eau: Handle<StandardMaterial>,
    pub eau_claire: Handle<StandardMaterial>,
    pub herbe: Handle<StandardMaterial>,
    pub herbe_claire: Handle<StandardMaterial>,
    pub feuillage: Handle<StandardMaterial>,
    pub feuillage_fonce: Handle<StandardMaterial>,
    pub sol_foret: Handle<StandardMaterial>,
    pub fleurs: [Handle<StandardMaterial>; 4],
    pub onde: Handle<StandardMaterial>,
    pub onde_disque: Handle<StandardMaterial>,
    pub poussiere: Handle<StandardMaterial>,
    pub etoile: Handle<StandardMaterial>,
    pub alerte: Handle<StandardMaterial>,
}

impl FromWorld for Kit {
    fn from_world(world: &mut World) -> Self {
        let mut meshes = world.resource_mut::<Assets<Mesh>>();
        let cube = meshes.add(Cuboid::new(1.0, 1.0, 1.0));
        let boule = meshes.add(Sphere::new(0.5).mesh().uv(10, 7));
        let cone = meshes.add(Cone { radius: 0.5, height: 1.0 }.mesh().resolution(7));
        let cyl = meshes.add(Cylinder::new(0.5, 1.0).mesh().resolution(10));
        let anneau = meshes.add(Annulus::new(0.95, 1.0));
        let disque = meshes.add(Circle::new(1.0));
        let mut mats = world.resource_mut::<Assets<StandardMaterial>>();
        let mut m = |r: f32, g: f32, b: f32| mats.add(flat_material(Color::srgb(r, g, b)));
        let laine = [m(0.97, 0.95, 0.88), m(0.93, 0.91, 0.84), m(0.99, 0.98, 0.94)];
        let laine_noire = m(0.2, 0.19, 0.22);
        let laine_belier = m(0.62, 0.5, 0.38);
        let peau = m(0.28, 0.24, 0.24);
        let fonce = m(0.16, 0.14, 0.16);
        let noir = m(0.09, 0.09, 0.11);
        let chien_pelage = m(0.09, 0.09, 0.11);
        let chien_accent = m(0.96, 0.96, 0.95);
        let blanc = m(0.96, 0.96, 0.95);
        let gris = m(0.52, 0.54, 0.58);
        let roux = m(0.66, 0.46, 0.3);
        let gris_fonce = m(0.34, 0.36, 0.42);
        let rouge = m(0.86, 0.18, 0.16);
        let bois = m(0.55, 0.38, 0.24);
        let bois_clair = m(0.72, 0.54, 0.34);
        let paille = m(0.93, 0.81, 0.46);
        let paille_fonce = m(0.82, 0.66, 0.32);
        let eau = m(0.36, 0.64, 0.84);
        let eau_claire = m(0.55, 0.80, 0.93);
        let sol_foret = m(0.50, 0.70, 0.36);
        let fleurs = [m(0.98, 0.98, 0.95), m(0.98, 0.82, 0.18), m(0.94, 0.33, 0.55), m(0.78, 0.68, 0.93)];
        // Les herbes et feuillages gardent les couleurs de la palette : les saisons les repeignent.
        let herbe = mats.add(flat_material(palette::GRASS));
        let herbe_claire = mats.add(flat_material(palette::GRASS_LIGHT));
        let feuillage = mats.add(flat_material(palette::FOLIAGE));
        let feuillage_fonce = mats.add(flat_material(palette::FOLIAGE_DARK));
        let transparent = |c: Color| StandardMaterial { base_color: c, alpha_mode: AlphaMode::Blend, unlit: true, ..default() };
        let onde = mats.add(transparent(Color::srgba(1.0, 1.0, 1.0, 0.75)));
        let onde_disque = mats.add(transparent(Color::srgba(1.0, 1.0, 1.0, 0.2)));
        let poussiere = mats.add(transparent(Color::srgba(0.92, 0.88, 0.76, 0.8)));
        let etoile = mats.add(StandardMaterial { base_color: palette::YELLOW, unlit: true, ..default() });
        let alerte = mats.add(transparent(Color::srgba(0.95, 0.2, 0.15, 0.55)));
        Kit {
            cube,
            boule,
            cone,
            cyl,
            anneau,
            disque,
            laine,
            laine_noire,
            laine_belier,
            peau,
            fonce,
            noir,
            chien_pelage,
            chien_accent,
            blanc,
            gris,
            roux,
            gris_fonce,
            rouge,
            bois,
            bois_clair,
            paille,
            paille_fonce,
            eau,
            eau_claire,
            herbe,
            herbe_claire,
            feuillage,
            feuillage_fonce,
            sol_foret,
            fleurs,
            onde,
            onde_disque,
            poussiere,
            etoile,
            alerte,
        }
    }
}

type Mat = Handle<StandardMaterial>;

/// Une pièce : forme, matière, position et taille.
fn piece(c: &mut Commands, parent: Entity, mesh: &Handle<Mesh>, mat: &Mat, pos: Vec3, scale: Vec3) -> Entity {
    c.spawn((Mesh3d(mesh.clone()), MeshMaterial3d(mat.clone()), Transform::from_translation(pos).with_scale(scale), ChildOf(parent))).id()
}

fn piece_tournee(c: &mut Commands, parent: Entity, mesh: &Handle<Mesh>, mat: &Mat, pos: Vec3, scale: Vec3, rot: Quat) -> Entity {
    c.spawn((Mesh3d(mesh.clone()), MeshMaterial3d(mat.clone()), Transform { translation: pos, rotation: rot, scale }, ChildOf(parent))).id()
}

/// Une patte : un pivot à la hanche, qui balance une jambe (et un pied d'une autre couleur).
fn patte(c: &mut Commands, root: Entity, k: &Kit, x: f32, z: f32, haut: f32, epaisseur: f32, jambe: &Mat, pied: &Mat, decalage: f32) {
    let pivot = c.spawn((Transform::from_xyz(x, haut, z), Visibility::default(), Patte { decalage }, ChildOf(root))).id();
    piece(c, pivot, &k.cube, jambe, Vec3::new(0.0, -haut * 0.5, 0.0), Vec3::new(epaisseur, haut, epaisseur));
    piece(c, pivot, &k.cube, pied, Vec3::new(0.0, -haut + epaisseur * 0.4, 0.0), Vec3::new(epaisseur * 1.05, epaisseur * 0.8, epaisseur * 1.05));
}

/// Le corps (groupe qui rebondit en marchant).
fn corps(c: &mut Commands, root: Entity, base: f32) -> Entity {
    c.spawn((Transform::from_xyz(0.0, base, 0.0), Visibility::default(), Corps { base }, ChildOf(root))).id()
}

fn racine(c: &mut Commands, scale: f32, extra: impl Bundle) -> Entity {
    c.spawn((Transform::from_scale(Vec3::splat(scale)), Visibility::default(), Allure::default(), DeScene, extra)).id()
}

/// Un mouton, tourné vers +Z, posé en y = 0. `v` varie la laine et la taille.
pub fn spawn_mouton(c: &mut Commands, k: &Kit, i: usize, genre: TypeMouton) -> Entity {
    let v = (i * 7 + 3) % 5;
    let taille = (0.95 + v as f32 * 0.05) * if genre == TypeMouton::Agneau { 0.68 } else { 1.0 };
    let root = racine(c, taille, VueMouton(i));
    let laine = if genre == TypeMouton::Noir { &k.laine_noire } else { &k.laine[i % 3] };
    let corps = corps(c, root, 0.42);
    // Une laine bouclée : une grosse boule et des touffes tout autour.
    piece(c, corps, &k.boule, laine, Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.78, 0.66, 0.92));
    for (x, y, z, s) in [
        (-0.24, 0.12, 0.18, 0.46),
        (0.24, 0.12, 0.18, 0.46),
        (-0.22, 0.14, -0.22, 0.46),
        (0.22, 0.14, -0.22, 0.46),
        (0.0, 0.26, 0.0, 0.5),
        (0.0, 0.2, -0.4, 0.4),
        (-0.3, -0.02, -0.02, 0.44),
        (0.3, -0.02, -0.02, 0.44),
    ] {
        piece(c, corps, &k.boule, laine, Vec3::new(x, y, z), Vec3::splat(s));
    }
    // La tête sombre, avec une houppe et des oreilles.
    piece(c, corps, &k.boule, &k.peau, Vec3::new(0.0, 0.16, 0.5), Vec3::new(0.3, 0.3, 0.38));
    piece(c, corps, &k.boule, laine, Vec3::new(0.0, 0.34, 0.46), Vec3::new(0.26, 0.16, 0.2));
    for s in [-1.0f32, 1.0] {
        piece_tournee(c, corps, &k.cube, &k.peau, Vec3::new(0.19 * s, 0.2, 0.44), Vec3::new(0.2, 0.05, 0.1), Quat::from_rotation_z(-0.5 * s));
        piece(c, corps, &k.boule, &k.blanc, Vec3::new(0.08 * s, 0.22, 0.64), Vec3::splat(0.07));
        piece(c, corps, &k.boule, &k.noir, Vec3::new(0.08 * s, 0.22, 0.67), Vec3::splat(0.04));
    }
    // La queue.
    piece(c, corps, &k.boule, laine, Vec3::new(0.0, 0.1, -0.5), Vec3::splat(0.18));
    for (x, z, d) in [(-0.2, 0.3, 0.0), (0.2, 0.3, 3.14), (-0.2, -0.28, 3.14), (0.2, -0.28, 0.0)] {
        patte(c, root, k, x, z, 0.3, 0.1, &k.peau, &k.fonce, d);
    }
    root
}

/// Un bélier : plus gros que les moutons, laine brune, tête sombre, grandes cornes enroulées.
pub fn spawn_belier(c: &mut Commands, k: &Kit, i: usize) -> Entity {
    let root = racine(c, 1.35, VueBelier(i));
    let corps = corps(c, root, 0.46);
    let laine = &k.laine_belier;
    piece(c, corps, &k.boule, laine, Vec3::ZERO, Vec3::new(0.84, 0.7, 1.0));
    for (x, y, z, s) in [(-0.26, 0.14, 0.2, 0.46), (0.26, 0.14, 0.2, 0.46), (-0.24, 0.16, -0.24, 0.46), (0.24, 0.16, -0.24, 0.46), (0.0, 0.28, 0.0, 0.5), (0.0, 0.18, -0.42, 0.4)] {
        piece(c, corps, &k.boule, laine, Vec3::new(x, y, z), Vec3::splat(s));
    }
    // La tête, large, avec le museau.
    piece(c, corps, &k.boule, &k.fonce, Vec3::new(0.0, 0.12, 0.54), Vec3::new(0.34, 0.34, 0.42));
    piece(c, corps, &k.boule, &k.peau, Vec3::new(0.0, 0.06, 0.72), Vec3::new(0.2, 0.18, 0.2));
    for s in [-1.0f32, 1.0] {
        piece(c, corps, &k.boule, &k.blanc, Vec3::new(0.09 * s, 0.2, 0.68), Vec3::splat(0.07));
        piece(c, corps, &k.boule, &k.noir, Vec3::new(0.09 * s, 0.2, 0.71), Vec3::splat(0.04));
        // Les cornes : une spirale de boules qui s'écartent, montent puis reviennent devant.
        for (j, (x, y, z, r)) in [(0.2, 0.2, 0.46, 0.12), (0.3, 0.24, 0.42, 0.11), (0.38, 0.18, 0.34, 0.1), (0.4, 0.08, 0.3, 0.09), (0.34, 0.0, 0.36, 0.08)].into_iter().enumerate() {
            let _ = j;
            piece(c, corps, &k.boule, &k.bois_clair, Vec3::new(x * s, y, z), Vec3::splat(r * 2.0));
        }
    }
    for (x, z, d) in [(-0.24, 0.32, 0.0), (0.24, 0.32, 3.14), (-0.24, -0.3, 3.14), (0.24, -0.3, 0.0)] {
        patte(c, root, k, x, z, 0.32, 0.12, &k.fonce, &k.noir, d);
    }
    root
}

/// Le chien de berger (border collie) : noir et blanc, oreilles dressées.
pub fn spawn_chien(c: &mut Commands, k: &Kit) -> Entity {
    let root = racine(c, 1.0, VueChien);
    let corps = corps(c, root, 0.48);
    piece(c, corps, &k.boule, &k.chien_pelage, Vec3::new(0.0, 0.0, -0.02), Vec3::new(0.46, 0.42, 1.02));
    // Poitrail et collerette blancs.
    piece(c, corps, &k.boule, &k.chien_accent, Vec3::new(0.0, -0.06, 0.3), Vec3::new(0.36, 0.34, 0.42));
    piece(c, corps, &k.boule, &k.chien_accent, Vec3::new(0.0, 0.1, 0.42), Vec3::new(0.4, 0.3, 0.2));
    // Tête : museau blanc, liste blanche, truffe noire.
    piece(c, corps, &k.boule, &k.chien_pelage, Vec3::new(0.0, 0.2, 0.62), Vec3::new(0.34, 0.32, 0.36));
    piece(c, corps, &k.cube, &k.chien_accent, Vec3::new(0.0, 0.14, 0.82), Vec3::new(0.16, 0.14, 0.26));
    piece(c, corps, &k.cube, &k.chien_accent, Vec3::new(0.0, 0.32, 0.7), Vec3::new(0.06, 0.04, 0.3));
    piece(c, corps, &k.boule, &k.noir, Vec3::new(0.0, 0.18, 0.96), Vec3::splat(0.08));
    for s in [-1.0f32, 1.0] {
        // Oreilles dressées, pointe pliée.
        piece_tournee(c, corps, &k.cone, &k.chien_pelage, Vec3::new(0.12 * s, 0.44, 0.56), Vec3::new(0.14, 0.24, 0.1), Quat::from_rotation_z(-0.25 * s) * Quat::from_rotation_x(0.25));
        piece(c, corps, &k.boule, &k.chien_accent, Vec3::new(0.08 * s, 0.28, 0.76), Vec3::splat(0.07));
        piece(c, corps, &k.boule, &k.chien_pelage, Vec3::new(0.08 * s, 0.28, 0.79), Vec3::splat(0.04));
    }
    // La queue en panache, bout blanc : elle remue.
    let queue = c.spawn((Transform::from_xyz(0.0, 0.08, -0.5), Visibility::default(), Queue { vitesse: 9.0 }, ChildOf(corps))).id();
    piece_tournee(c, queue, &k.cube, &k.chien_pelage, Vec3::new(0.0, 0.1, -0.2), Vec3::new(0.1, 0.1, 0.46), Quat::from_rotation_x(0.55));
    piece(c, queue, &k.boule, &k.chien_accent, Vec3::new(0.0, 0.2, -0.42), Vec3::new(0.12, 0.12, 0.2));
    for (x, z, d) in [(-0.17, 0.32, 0.0), (0.17, 0.32, 3.14), (-0.17, -0.3, 3.14), (0.17, -0.3, 0.0)] {
        patte(c, root, k, x, z, 0.36, 0.11, &k.chien_pelage, &k.chien_accent, d);
    }
    root
}

/// Un loup : gris, plus grand, les yeux rouges et les oreilles pointues.
pub fn spawn_loup(c: &mut Commands, k: &Kit, i: usize, genre: GenreLoup) -> Entity {
    let (pelage, pelage_fonce) = match genre {
        GenreLoup::Solitaire => (&k.gris, &k.gris_fonce),
        GenreLoup::Prudent => (&k.gris_fonce, &k.noir),
        GenreLoup::Audacieux => (&k.roux, &k.bois),
    };
    let taille = if genre == GenreLoup::Audacieux { 1.2 } else { 1.1 };
    let root = racine(c, taille, VueLoup(i));
    let corps = corps(c, root, 0.54);
    piece(c, corps, &k.boule, pelage, Vec3::new(0.0, 0.0, -0.04), Vec3::new(0.5, 0.46, 1.1));
    piece(c, corps, &k.boule, pelage_fonce, Vec3::new(0.0, 0.12, 0.1), Vec3::new(0.46, 0.3, 0.6));
    piece(c, corps, &k.boule, &k.blanc, Vec3::new(0.0, -0.1, 0.36), Vec3::new(0.34, 0.26, 0.4));
    // Tête allongée, gueule, truffe.
    piece(c, corps, &k.boule, pelage, Vec3::new(0.0, 0.2, 0.7), Vec3::new(0.38, 0.34, 0.4));
    piece(c, corps, &k.cube, pelage_fonce, Vec3::new(0.0, 0.14, 0.94), Vec3::new(0.2, 0.16, 0.3));
    piece(c, corps, &k.boule, &k.noir, Vec3::new(0.0, 0.18, 1.1), Vec3::splat(0.09));
    for s in [-1.0f32, 1.0] {
        piece_tournee(c, corps, &k.cone, pelage_fonce, Vec3::new(0.13 * s, 0.46, 0.62), Vec3::new(0.15, 0.3, 0.1), Quat::from_rotation_z(-0.15 * s));
        piece(c, corps, &k.boule, &k.rouge, Vec3::new(0.1 * s, 0.3, 0.88), Vec3::splat(0.07));
    }
    // Un anneau rouge au sol : il se repère de loin.
    c.spawn((
        Mesh3d(k.anneau.clone()),
        MeshMaterial3d(k.alerte.clone()),
        Transform::from_xyz(0.0, 0.07, 0.0).with_rotation(Quat::from_rotation_x(-FRAC_PI_2)).with_scale(Vec3::splat(1.3)),
        NotShadowCaster,
        ChildOf(root),
    ));
    // La queue basse et touffue.
    let queue = c.spawn((Transform::from_xyz(0.0, 0.0, -0.55), Visibility::default(), Queue { vitesse: 3.0 }, ChildOf(corps))).id();
    piece_tournee(c, queue, &k.cube, pelage_fonce, Vec3::new(0.0, -0.1, -0.22), Vec3::new(0.14, 0.14, 0.55), Quat::from_rotation_x(-0.5));
    for (x, z, d) in [(-0.18, 0.34, 0.0), (0.18, 0.34, 3.14), (-0.18, -0.34, 3.14), (0.18, -0.34, 0.0)] {
        patte(c, root, k, x, z, 0.42, 0.12, &k.gris_fonce, &k.noir, d);
    }
    root
}

/// Un modèle de Kenney (CC0), posé au sol.
fn modele(c: &mut Commands, server: &AssetServer, fichier: &str, pos: Vec3, rot: f32, echelle: Vec3) -> Entity {
    let scene = server.load(GltfAssetLabel::Scene(0).from_asset(format!("kenney/{fichier}.glb")));
    c.spawn((WorldAssetRoot(scene), Transform::from_translation(pos).with_rotation(Quat::from_rotation_y(rot)).with_scale(echelle), DeScene)).id()
}

fn arbre(c: &mut Commands, server: &AssetServer, pos: Vec3, rayon: f32, variante: u8, pin: bool) {
    let fichier = if pin { "mini-forest/tree-high" } else { "mini-forest/tree" };
    modele(c, server, fichier, pos, variante as f32 * 1.3, Vec3::splat(rayon * 2.0 * (0.9 + (variante % 3) as f32 * 0.1)));
}

fn rocher(c: &mut Commands, server: &AssetServer, pos: Vec3, rayon: f32, variante: u8) {
    let fichier = ["survival-kit/rock-sand-a", "survival-kit/rock-sand-b", "survival-kit/rock-sand-c"][variante as usize % 3];
    modele(c, server, fichier, pos, variante as f32 * 1.9, Vec3::splat(rayon * 3.6));
}

fn mare(c: &mut Commands, k: &Kit, pos: Vec3, rayon: f32, variante: u8) {
    let racine = c.spawn((Transform::from_translation(pos), Visibility::default(), DeScene)).id();
    // Rive de terre, eau, reflet.
    let e = piece(c, racine, &k.cyl, &k.paille_fonce, Vec3::new(0.0, 0.015, 0.0), Vec3::new(rayon * 2.1, 0.03, rayon * 2.1));
    c.entity(e).insert(NotShadowCaster);
    let e = piece(c, racine, &k.cyl, &k.eau, Vec3::new(0.0, 0.035, 0.0), Vec3::new(rayon * 1.9, 0.03, rayon * 1.9));
    c.entity(e).insert(NotShadowCaster);
    let e = piece(c, racine, &k.cyl, &k.eau_claire, Vec3::new(rayon * 0.2, 0.05, -rayon * 0.15), Vec3::new(rayon * 0.9, 0.03, rayon * 0.7));
    c.entity(e).insert(NotShadowCaster);
    // Nénuphars et roseaux.
    for j in 0..3 {
        let a = variante as f32 + j as f32 * 2.1;
        let e = piece(c, racine, &k.cyl, &k.feuillage, Vec3::new(a.cos() * rayon * 0.5, 0.06, a.sin() * rayon * 0.5), Vec3::new(0.34, 0.02, 0.34));
        c.entity(e).insert(NotShadowCaster);
    }
    for j in 0..5 {
        let a = variante as f32 * 0.7 + j as f32 * 0.5 + 2.2;
        piece(c, racine, &k.cone, &k.feuillage_fonce, Vec3::new(a.cos() * rayon * 1.0, 0.35, a.sin() * rayon * 1.0), Vec3::new(0.08, 0.7, 0.08));
    }
}

/// Une clôture de bois : des éléments de clôture de Kenney bout à bout.
fn cloture(c: &mut Commands, server: &AssetServer, m: &Mur) {
    let a = Vec3::new(m.a.x, 0.0, m.a.y);
    let b = Vec3::new(m.b.x, 0.0, m.b.y);
    let long = a.distance(b);
    if long < 0.05 {
        return;
    }
    let dir = (b - a) / long;
    let yaw = (-dir.z).atan2(dir.x);
    // Un élément mesure 0,5 de long (hauteur 0,5) ; on en pose un tous les ~1,6.
    let n = (long / 1.6).round().max(1.0) as usize;
    let pas = long / n as f32;
    for i in 0..n {
        let milieu = a + dir * (pas * (i as f32 + 0.5));
        modele(c, server, "survival-kit/fence", milieu, yaw, Vec3::new(pas / 0.5, 1.5, 1.5));
    }
}

/// Construit le champ : sol, herbe, décor, obstacles, enclos, animaux.
pub fn spawn_scene(c: &mut Commands, server: &AssetServer, k: &Kit, carte: &Carte, niveau: &Niveau) {
    let (hx, hz) = (carte.demi.x, carte.demi.y);
    let mut rng = Rng::new(carte.graine ^ 0xDEC0);

    // Le champ, tondu en bandes, entouré d'un grand pré.
    let racine = c.spawn((Transform::default(), Visibility::default(), DeScene)).id();
    piece(c, racine, &k.cube, &k.sol_foret, Vec3::new(0.0, -0.6, 0.0), Vec3::new(hx * 2.0 + 90.0, 1.0, hz * 2.0 + 90.0));
    piece(c, racine, &k.cube, &k.herbe, Vec3::new(0.0, -0.1, 0.0), Vec3::new(hx * 2.0 + 0.4, 0.2, hz * 2.0 + 0.4));
    let bande = 3.0;
    let n = (hx * 2.0 / bande).ceil() as i32;
    for i in (0..n).step_by(2) {
        let x0 = -hx + i as f32 * bande;
        let l = bande.min(hx - x0);
        let e = piece(c, racine, &k.cube, &k.herbe_claire, Vec3::new(x0 + l * 0.5, 0.005, 0.0), Vec3::new(l, 0.02, hz * 2.0));
        c.entity(e).insert(NotShadowCaster);
    }

    // Touffes et fleurs semées au hasard, hors des obstacles.
    for _ in 0..(hx * hz * 0.45) as usize {
        let p = Vec3::new((rng.next_f32() * 2.0 - 1.0) * (hx - 0.4), 0.0, (rng.next_f32() * 2.0 - 1.0) * (hz - 0.4));
        let v2 = berger_core::v2(p.x, p.z);
        if carte.bloque(v2, 0.6) || carte.enclos.dedans(v2, -1.0) {
            continue;
        }
        if rng.next_f32() < 0.55 {
            let h = 0.18 + rng.next_f32() * 0.16;
            piece(c, racine, &k.cone, &k.feuillage, p + Vec3::Y * (h * 0.5), Vec3::new(0.16, h, 0.16));
            piece(c, racine, &k.cone, &k.feuillage, p + Vec3::new(0.1, h * 0.4, 0.05), Vec3::new(0.12, h * 0.8, 0.12));
        } else {
            let f = &k.fleurs[rng.below(4) as usize];
            piece(c, racine, &k.cube, &k.feuillage_fonce, p + Vec3::Y * 0.1, Vec3::new(0.025, 0.2, 0.025));
            piece(c, racine, &k.boule, f, p + Vec3::Y * 0.22, Vec3::splat(0.1));
        }
    }

    // La clôture du champ et la forêt tout autour.
    let coins = [berger_core::v2(-hx, -hz), berger_core::v2(hx, -hz), berger_core::v2(hx, hz), berger_core::v2(-hx, hz)];
    for i in 0..4 {
        cloture(c, server, &Mur { a: coins[i], b: coins[(i + 1) % 4] });
    }
    let pas = 3.4;
    for (cote, long) in [(0, hx * 2.0), (1, hx * 2.0), (2, hz * 2.0), (3, hz * 2.0)] {
        let mut t = -long * 0.5 - 2.0;
        while t < long * 0.5 + 2.0 {
            let ecart = 1.6 + rng.next_f32() * 3.2;
            let (x, z) = match cote {
                0 => (t, -hz - ecart),
                1 => (t, hz + ecart),
                2 => (-hx - ecart, t),
                _ => (hx + ecart, t),
            };
            let pin = rng.next_f32() < 0.55;
            let r = 0.9 + rng.next_f32() * 0.5;
            arbre(c, server, Vec3::new(x, 0.0, z), r, rng.below(4) as u8, pin);
            t += pas * (0.7 + rng.next_f32() * 0.7);
        }
    }

    // Les obstacles du champ.
    for o in &carte.obstacles {
        let p = Vec3::new(o.pos.x, 0.0, o.pos.y);
        match o.genre {
            Genre::Arbre => arbre(c, server, p, o.rayon, o.variante, false),
            Genre::Pin => arbre(c, server, p, o.rayon, o.variante, true),
            Genre::Rocher => rocher(c, server, p, o.rayon, o.variante),
            Genre::Mare => mare(c, k, p, o.rayon, o.variante),
        }
    }
    for m in &carte.clotures {
        cloture(c, server, m);
    }

    // L'enclos : sol de paille, clôtures, poteaux de la porte, bottes de foin.
    let e = carte.enclos;
    let sol = c.spawn((Transform::from_xyz(e.centre.x, 0.0, e.centre.y), Visibility::default(), DeScene)).id();
    let s = piece(c, sol, &k.cube, &k.paille, Vec3::new(0.0, 0.015, 0.0), Vec3::new(e.demi.x * 2.0, 0.03, e.demi.y * 2.0));
    c.entity(s).insert(NotShadowCaster);
    for j in 0..6 {
        let x = ((j * 5 + 1) % 7) as f32 / 7.0 - 0.5;
        let z = ((j * 3 + 2) % 5) as f32 / 5.0 - 0.5;
        let s = piece_tournee(c, sol, &k.cube, &k.paille_fonce, Vec3::new(x * e.demi.x * 1.6, 0.04, z * e.demi.y * 1.6), Vec3::new(0.7, 0.01, 0.18), Quat::from_rotation_y(j as f32 * 1.1));
        c.entity(s).insert(NotShadowCaster);
    }
    for m in &carte.murs_enclos {
        cloture(c, server, m);
    }
    // Les poteaux de la porte, plus hauts, avec une lanterne.
    let sortie = e.cote.sortie();
    let tangente = Vec3::new(-sortie.y, 0.0, sortie.x);
    let porte = Vec3::new(e.entree().x, 0.0, e.entree().y);
    for s in [-1.0f32, 1.0] {
        let p = porte + tangente * (e.porte * 0.5 * s);
        piece(c, racine, &k.cube, &k.bois, p + Vec3::Y * 0.7, Vec3::new(0.22, 1.4, 0.22));
        piece(c, racine, &k.cube, &k.bois_clair, p + Vec3::Y * 1.45, Vec3::new(0.3, 0.1, 0.3));
        piece(c, racine, &k.boule, &k.etoile, p + Vec3::Y * 1.65, Vec3::splat(0.22));
    }
    // Bottes de foin au fond de l'enclos.
    let fond = Vec3::new(e.centre.x, 0.0, e.centre.y) - Vec3::new(sortie.x * (e.demi.x - 0.6), 0.0, sortie.y * (e.demi.y - 0.6));
    for (d, h) in [(-1.0f32, 0.55), (1.0, 0.45)] {
        let p = fond + tangente * (d * 0.9);
        piece_tournee(c, racine, &k.cyl, &k.paille_fonce, p + Vec3::Y * h * 0.5, Vec3::new(h * 1.6, 0.8, h * 1.6), Quat::from_rotation_z(FRAC_PI_2) * Quat::from_rotation_x(0.3 * d));
    }

    // L'onde de l'aboiement (un anneau à plat, invisible tant qu'on n'aboie pas).
    c.spawn((
        Mesh3d(k.anneau.clone()),
        MeshMaterial3d(k.onde.clone()),
        Transform::from_xyz(0.0, 0.1, 0.0).with_rotation(Quat::from_rotation_x(-FRAC_PI_2)),
        Visibility::Hidden,
        NotShadowCaster,
        OndeVue,
        DeScene,
        children![(Mesh3d(k.disque.clone()), MeshMaterial3d(k.onde_disque.clone()), Transform::from_xyz(0.0, 0.0, -0.01), NotShadowCaster)],
    ));

    // Les animaux.
    for i in 0..carte.moutons.len() {
        spawn_mouton(c, k, i, carte.types[i]);
    }
    for i in 0..carte.beliers.len() {
        spawn_belier(c, k, i);
    }
    spawn_chien(c, k);
    for i in 0..carte.loups.len() {
        let l = spawn_loup(c, k, i, carte.loups[i].genre);
        c.entity(l).insert(Visibility::Hidden);
    }
    let _ = niveau;
    let _ = TAU;
}

/// Un éclat qui vole puis disparaît (poussière, laine, étincelles).
#[derive(Component)]
pub struct Particule {
    pub vel: Vec3,
    pub vie: f32,
    pub duree: f32,
    pub gravite: f32,
    pub taille: f32,
}

pub fn eclat(c: &mut Commands, k: &Kit, mat: &Mat, pos: Vec3, vel: Vec3, duree: f32, gravite: f32, taille: f32) {
    c.spawn((
        Mesh3d(k.boule.clone()),
        MeshMaterial3d(mat.clone()),
        Transform::from_translation(pos).with_scale(Vec3::splat(taille)),
        NotShadowCaster,
        Particule { vel, vie: duree, duree, gravite, taille },
    ));
}

pub fn animer_particules(time: Res<Time>, mut c: Commands, mut q: Query<(Entity, &mut Transform, &mut Particule)>) {
    let dt = time.delta_secs();
    for (e, mut t, mut p) in &mut q {
        p.vie -= dt;
        if p.vie <= 0.0 {
            c.entity(e).despawn();
            continue;
        }
        p.vel.y -= p.gravite * dt;
        t.translation += p.vel * dt;
        if t.translation.y < 0.05 && p.gravite > 0.0 {
            t.translation.y = 0.05;
            p.vel *= 0.5;
        }
        t.scale = Vec3::splat(p.taille * (p.vie / p.duree).clamp(0.0, 1.0));
    }
}

/// Marche des animaux : pattes qui balancent, corps qui rebondit, queue qui remue.
pub fn animer_allures(
    time: Res<Time>,
    racines: Query<(&Children, &Allure)>,
    mut parties: Query<(&mut Transform, Option<&Patte>, Option<&Corps>, Option<&Queue>)>,
) {
    let t = time.elapsed_secs();
    for (enfants, a) in &racines {
        for enfant in enfants.iter() {
            let Ok((mut tr, patte, corps, queue)) = parties.get_mut(enfant) else { continue };
            if let Some(p) = patte {
                tr.rotation = Quat::from_rotation_x((a.phase + p.decalage).sin() * 0.75 * a.mouvement);
            }
            if let Some(c) = corps {
                tr.translation.y = c.base + (a.phase * 2.0).sin().abs() * 0.07 * a.mouvement + (t * 2.0 + a.phase).sin() * 0.004 + (a.saut * std::f32::consts::PI).sin() * 0.28;
                tr.rotation = Quat::from_rotation_x((a.phase).sin() * 0.05 * a.mouvement + a.tete * 0.32) * Quat::from_rotation_z((a.phase).cos() * 0.04 * a.mouvement);
            }
            if let Some(q) = queue {
                tr.rotation = Quat::from_rotation_y((t * q.vitesse).sin() * (0.35 + 0.5 * a.agite));
            }
        }
    }
}
