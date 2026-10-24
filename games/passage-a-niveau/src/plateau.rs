//! Le décor et les cartes : une table de jeu en bois clair dans la prairie,
//! des rails de jouet en bois (traverses brunes, deux rails clairs), des
//! routes grises à pointillés blancs, des passages à niveau à barrières
//! rouges et blanches. Tout est dessiné à plat, vu de dessus, en couleurs
//! unies.
use std::f32::consts::FRAC_PI_2;

use bevy::light::NotShadowCaster;
use bevy::prelude::*;
use game_common::{flat_material, palette};
use passage_core::{Card, EAST, N, NORTH, Network, Piece, SOUTH, SideStatus, WEST};

/// Hauteur du dessus des cases.
pub const DESSUS: f32 = 0.08;

/// Un élément de la grille ou de la file qui est redessiné quand la partie change.
#[derive(Component)]
pub struct Carte;

#[derive(Resource)]
pub struct Look {
    pub cube: Handle<Mesh>,
    pub disque: Handle<Mesh>,
    pub secteur: Handle<Mesh>,
    pub tuile_a: Handle<StandardMaterial>,
    pub tuile_b: Handle<StandardMaterial>,
    pub bois: Handle<StandardMaterial>,
    pub traverse: Handle<StandardMaterial>,
    pub rail: Handle<StandardMaterial>,
    pub route: Handle<StandardMaterial>,
    pub ligne: Handle<StandardMaterial>,
    pub rouge: Handle<StandardMaterial>,
    pub blanc: Handle<StandardMaterial>,
    pub cadre: Handle<StandardMaterial>,
    pub mur: Handle<StandardMaterial>,
    pub vert: Handle<StandardMaterial>,
    pub bleu: Handle<StandardMaterial>,
    pub vide: Handle<StandardMaterial>,
    pub bloque: Handle<StandardMaterial>,
    pub noir: Handle<StandardMaterial>,
    pub jaune: Handle<StandardMaterial>,
}

impl FromWorld for Look {
    fn from_world(world: &mut World) -> Self {
        let mut meshes = world.resource_mut::<Assets<Mesh>>();
        let cube = meshes.add(Cuboid::new(1.0, 1.0, 1.0));
        let disque = meshes.add(Cylinder::new(0.5, 1.0).mesh().resolution(16));
        let secteur = meshes.add(secteur(0.5 - 0.28, 0.5 + 0.28, 16));
        let mut mats = world.resource_mut::<Assets<StandardMaterial>>();
        let mut uni = |r: u8, g: u8, b: u8| mats.add(StandardMaterial { base_color: Color::srgb_u8(r, g, b), unlit: true, ..default() });
        Look {
            cube,
            disque,
            secteur,
            tuile_a: uni(0xE6, 0xEF, 0xC8),
            tuile_b: uni(0xDA, 0xE8, 0xB8),
            bois: uni(0xB8, 0x8C, 0x58),
            traverse: uni(0x8A, 0x5C, 0x34),
            rail: uni(0xC9, 0xA9, 0x7C),
            route: uni(0x8F, 0x93, 0x9B),
            ligne: uni(0xF4, 0xF1, 0xE6),
            rouge: uni(0xD8, 0x3A, 0x2E),
            blanc: uni(0xF8, 0xF4, 0xEC),
            cadre: uni(0xFF, 0xC8, 0x2E),
            mur: uni(0xF1, 0xE6, 0xC8),
            vert: uni(0x3F, 0xA3, 0x4D),
            bleu: uni(0x2F, 0x6F, 0xD6),
            vide: uni(0xE0, 0x4A, 0x3C),
            bloque: uni(0xC4, 0x12, 0x12),
            noir: uni(0x2E, 0x2E, 0x38),
            jaune: uni(0xF2, 0xC2, 0x30),
        }
    }
}

/// Centre d'une case (la grille est dans le plan XZ, centrée sur l'origine ;
/// le nord est du côté des z négatifs, l'est du côté des x positifs).
pub fn pos_case(i: usize) -> Vec3 {
    let d = (N as f32 - 1.0) / 2.0;
    Vec3::new((i % N) as f32 - d, DESSUS, (i / N) as f32 - d)
}

/// Le milieu du côté `side` (0 nord, 1 est, 2 sud, 3 ouest), un peu en retrait.
pub fn bord(side: usize, retrait: f32) -> Vec3 {
    let r = 0.5 - retrait;
    match side {
        0 => Vec3::new(0.0, 0.0, -r),
        1 => Vec3::new(r, 0.0, 0.0),
        2 => Vec3::new(0.0, 0.0, r),
        _ => Vec3::new(-r, 0.0, 0.0),
    }
}

fn pave(c: &mut Commands, parent: Entity, look: &Look, mat: &Handle<StandardMaterial>, centre: Vec3, echelle: Vec3, angle: f32) {
    c.spawn((
        Mesh3d(look.cube.clone()),
        MeshMaterial3d(mat.clone()),
        Transform::from_translation(centre).with_rotation(Quat::from_rotation_y(angle)).with_scale(echelle),
        NotShadowCaster,
        ChildOf(parent),
    ));
}

/// Un segment de bande entre deux points du plan (x, z), de largeur et de
/// hauteur données, posé à la hauteur `y`.
fn segment(c: &mut Commands, parent: Entity, look: &Look, mat: &Handle<StandardMaterial>, a: Vec2, b: Vec2, largeur: f32, haut: f32, y: f32) {
    let d = b - a;
    let milieu = (a + b) / 2.0;
    let angle = d.x.atan2(d.y);
    pave(c, parent, look, mat, Vec3::new(milieu.x, y + haut / 2.0, milieu.y), Vec3::new(largeur, haut, d.length() + 0.012), angle);
}

/// Un point du quart de cercle nord–est (centre au coin nord-est de la case).
fn arc(r: f32, t: f32) -> Vec2 {
    Vec2::new(0.5 - r * t.cos(), -0.5 + r * t.sin())
}

const SEGMENTS: usize = 8;

fn rail_droit(c: &mut Commands, parent: Entity, look: &Look, axe_x: bool, y: f32) {
    let (a, b) = if axe_x { (Vec2::new(-0.5, 0.0), Vec2::new(0.5, 0.0)) } else { (Vec2::new(0.0, -0.5), Vec2::new(0.0, 0.5)) };
    let perp = if axe_x { Vec2::Y } else { Vec2::X };
    for k in 0..5 {
        let t = -0.4 + 0.2 * k as f32;
        let centre = (a + b) / 2.0 + (b - a) * t;
        segment(c, parent, look, &look.traverse, centre - perp * 0.26, centre + perp * 0.26, 0.07, 0.025, y);
    }
    for s in [-0.15, 0.15] {
        segment(c, parent, look, &look.rail, a + perp * s, b + perp * s, 0.06, 0.04, y + 0.02);
    }
}

fn rail_courbe(c: &mut Commands, parent: Entity, look: &Look, y: f32) {
    for k in 1..=5 {
        let t = k as f32 / 6.0 * FRAC_PI_2;
        segment(c, parent, look, &look.traverse, arc(0.24, t), arc(0.76, t), 0.07, 0.025, y);
    }
    for r in [0.35, 0.65] {
        for k in 0..SEGMENTS {
            let (t0, t1) = (k as f32 / SEGMENTS as f32 * FRAC_PI_2, (k + 1) as f32 / SEGMENTS as f32 * FRAC_PI_2);
            segment(c, parent, look, &look.rail, arc(r, t0), arc(r, t1), 0.06, 0.04, y + 0.02);
        }
    }
}

fn route_droite(c: &mut Commands, parent: Entity, look: &Look, axe_x: bool) {
    let (a, b) = if axe_x { (Vec2::new(-0.5, 0.0), Vec2::new(0.5, 0.0)) } else { (Vec2::new(0.0, -0.5), Vec2::new(0.0, 0.5)) };
    segment(c, parent, look, &look.route, a, b, 0.56, 0.02, 0.0);
    for t in [-0.28, 0.0, 0.28] {
        let centre = (a + b) / 2.0 + (b - a) * t;
        let d = (b - a) * 0.07;
        segment(c, parent, look, &look.ligne, centre - d, centre + d, 0.05, 0.01, 0.02);
    }
}

/// Un secteur d'anneau plat (le ruban d'une route en courbe), autour du coin
/// nord-est de la case, entre les rayons `r0` et `r1`, à la hauteur de la route.
fn secteur(r0: f32, r1: f32, pas: usize) -> Mesh {
    use bevy::asset::RenderAssetUsages;
    use bevy::mesh::{Indices, PrimitiveTopology};
    let mut pos = Vec::new();
    for k in 0..=pas {
        let t = k as f32 / pas as f32 * FRAC_PI_2;
        for r in [r0, r1] {
            let p = arc(r, t);
            pos.push([p.x, 0.02, p.y]);
        }
    }
    let mut idx = Vec::new();
    for k in 0..pas as u32 {
        let (a, b, c, d) = (2 * k, 2 * k + 1, 2 * k + 2, 2 * k + 3);
        // Le côté visible est le dessus : on ordonne les sommets pour que la normale monte.
        let monte = |i: u32, j: u32, l: u32| {
            let (p, q, r) = (Vec3::from(pos[i as usize]), Vec3::from(pos[j as usize]), Vec3::from(pos[l as usize]));
            (q - p).cross(r - p).y > 0.0
        };
        for tri in [[a, b, c], [b, d, c]] {
            if monte(tri[0], tri[1], tri[2]) {
                idx.extend(tri);
            } else {
                idx.extend([tri[0], tri[2], tri[1]]);
            }
        }
    }
    let normales = vec![[0.0, 1.0, 0.0]; pos.len()];
    Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default())
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, pos)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normales)
        .with_inserted_indices(Indices::U32(idx))
}

fn route_courbe(c: &mut Commands, parent: Entity, look: &Look) {
    c.spawn((Mesh3d(look.secteur.clone()), MeshMaterial3d(look.route.clone()), Transform::default(), NotShadowCaster, ChildOf(parent)));
    for t in [0.18, 0.5, 0.82] {
        let (a, b) = (arc(0.5, (t - 0.04) * FRAC_PI_2), arc(0.5, (t + 0.04) * FRAC_PI_2));
        segment(c, parent, look, &look.ligne, a, b, 0.05, 0.01, 0.02);
    }
}

fn dir(side: usize) -> Vec2 {
    match side {
        0 => Vec2::new(0.0, -1.0),
        1 => Vec2::new(1.0, 0.0),
        2 => Vec2::new(0.0, 1.0),
        _ => Vec2::new(-1.0, 0.0),
    }
}

fn cotes(masque: u8) -> impl Iterator<Item = usize> {
    (0..4).filter(move |s| masque & (1 << s) != 0)
}

/// Un bras de rail : du bord de la case jusqu'au centre.
fn rail_bras(c: &mut Commands, parent: Entity, look: &Look, side: usize, y: f32) {
    let d = dir(side);
    let perp = Vec2::new(-d.y, d.x);
    for t in [0.42, 0.27, 0.12] {
        let centre = d * t;
        segment(c, parent, look, &look.traverse, centre - perp * 0.26, centre + perp * 0.26, 0.07, 0.025, y);
    }
    for s in [-0.15, 0.15] {
        segment(c, parent, look, &look.rail, d * 0.5 + perp * s, d * 0.0 + perp * s, 0.06, 0.04, y + 0.02);
    }
}

/// Un bras de route : du bord de la case jusqu'au centre.
fn route_bras(c: &mut Commands, parent: Entity, look: &Look, side: usize) {
    let d = dir(side);
    segment(c, parent, look, &look.route, d * 0.5, d * 0.0, 0.56, 0.02, 0.0);
    segment(c, parent, look, &look.ligne, d * 0.34, d * 0.2, 0.05, 0.01, 0.02);
}

fn dessiner_rail(c: &mut Commands, parent: Entity, look: &Look, masque: u8, y: f32) {
    match masque {
        0 => {}
        m if m == NORTH | EAST => rail_courbe(c, parent, look, y),
        m if m == NORTH | SOUTH => rail_droit(c, parent, look, false, y),
        m if m == EAST | WEST => rail_droit(c, parent, look, true, y),
        m => cotes(m).for_each(|s| rail_bras(c, parent, look, s, y)),
    }
}

fn dessiner_route(c: &mut Commands, parent: Entity, look: &Look, masque: u8) {
    match masque {
        0 => {}
        m if m == NORTH | EAST => route_courbe(c, parent, look),
        m if m == NORTH | SOUTH => route_droite(c, parent, look, false),
        m if m == EAST | WEST => route_droite(c, parent, look, true),
        m => {
            cotes(m).for_each(|s| route_bras(c, parent, look, s));
            pave(c, parent, look, &look.route, Vec3::new(0.0, 0.01, 0.0), Vec3::new(0.56, 0.02, 0.56), 0.0);
        }
    }
}

/// Le bâtiment d'une gare, au centre de la case : des murs crème et un toit de couleur.
fn batiment(c: &mut Commands, parent: Entity, look: &Look, toit: &Handle<StandardMaterial>) {
    pave(c, parent, look, &look.mur, Vec3::new(0.0, 0.1, 0.0), Vec3::new(0.34, 0.2, 0.34), 0.0);
    pave(c, parent, look, toit, Vec3::new(0.0, 0.23, 0.0), Vec3::new(0.44, 0.07, 0.44), 0.0);
}

/// Le parking de départ : un carré bleu marqué d'un P blanc (toujours à l'endroit).
fn parking(c: &mut Commands, parent: Entity, look: &Look) {
    pave(c, parent, look, &look.bleu, Vec3::new(0.0, 0.02, 0.0), Vec3::new(0.5, 0.04, 0.5), 0.0);
    for (x, z, w, d) in [(-0.07, 0.0, 0.07, 0.3), (0.0, -0.11, 0.2, 0.07), (0.0, 0.0, 0.2, 0.07), (0.1, -0.055, 0.07, 0.18)] {
        pave(c, parent, look, &look.blanc, Vec3::new(x, 0.045, z), Vec3::new(w, 0.02, d), 0.0);
    }
}

/// Dessine une carte : la tuile, la pièce, tournée comme la carte. `echelle`
/// la grossit (la file des cartes à venir).
pub fn carte(c: &mut Commands, look: &Look, card: Card, centre: Vec3, echelle: f32, parite: usize, extra: impl Bundle) -> Entity {
    let racine = c.spawn((Transform::from_translation(centre).with_scale(Vec3::splat(echelle)), Visibility::default(), Carte, extra)).id();
    let tuile = if parite.is_multiple_of(2) { &look.tuile_a } else { &look.tuile_b };
    pave(c, racine, look, tuile, Vec3::new(0.0, -0.05, 0.0), Vec3::new(0.94, 0.1, 0.94), 0.0);
    let pose = c.spawn((Transform::from_rotation(Quat::from_rotation_y(-(card.rot as f32) * FRAC_PI_2)), Visibility::default(), ChildOf(racine))).id();
    let masque = |net: Network| Card::new(card.piece).sides(net);
    let (rail, route) = (masque(Network::Rail), masque(Network::Route));
    dessiner_route(c, pose, look, route);
    dessiner_rail(c, pose, look, rail, if route != 0 { 0.02 } else { 0.0 });
    match card.piece {
        Piece::Croisement => {
            // Les quatre barrières du passage à niveau, aux coins.
            for (sx, sz) in [(-1.0, -1.0), (1.0, -1.0), (-1.0, 1.0), (1.0, 1.0)] {
                let (x, z) = (sx * 0.4, sz * 0.4);
                pave(c, pose, look, &look.rouge, Vec3::new(x, 0.1, z), Vec3::new(0.07, 0.2, 0.07), 0.0);
                pave(c, pose, look, &look.blanc, Vec3::new(x, 0.17, z), Vec3::new(0.075, 0.05, 0.075), 0.0);
            }
        }
        Piece::Gare2Droite | Piece::Gare2Courbe | Piece::Gare3Rail | Piece::Gare3Route | Piece::Gare4 => batiment(c, racine, look, &look.rouge),
        Piece::GareDepart => batiment(c, racine, look, &look.vert),
        Piece::Parking => parking(c, racine, look),
        Piece::Impasse => {
            // Le heurtoir : une barre rouge en travers de la voie, à son extrémité.
            pave(c, pose, look, &look.rouge, Vec3::new(0.0, 0.07, 0.04), Vec3::new(0.36, 0.1, 0.08), 0.0);
            pave(c, pose, look, &look.blanc, Vec3::new(0.0, 0.13, 0.04), Vec3::new(0.3, 0.03, 0.04), 0.0);
        }
        Piece::RondPoint => {
            // L'anneau du rond-point, avec son îlot vert.
            c.spawn((Mesh3d(look.disque.clone()), MeshMaterial3d(look.route.clone()), Transform::from_xyz(0.0, 0.01, 0.0).with_scale(Vec3::new(0.66, 0.03, 0.66)), NotShadowCaster, ChildOf(pose)));
            c.spawn((Mesh3d(look.disque.clone()), MeshMaterial3d(look.vert.clone()), Transform::from_xyz(0.0, 0.03, 0.0).with_scale(Vec3::new(0.28, 0.03, 0.28)), NotShadowCaster, ChildOf(pose)));
        }
        _ => {}
    }
    racine
}

/// Un cadre jaune autour d'une case (la case visée).
pub fn cadre(c: &mut Commands, look: &Look, centre: Vec3) {
    let racine = c.spawn((Transform::from_translation(centre + Vec3::Y * 0.02), Visibility::default(), Carte)).id();
    for (d, e) in [
        (Vec3::new(0.0, 0.0, 0.47), Vec3::new(0.96, 0.03, 0.05)),
        (Vec3::new(0.0, 0.0, -0.47), Vec3::new(0.96, 0.03, 0.05)),
        (Vec3::new(0.47, 0.0, 0.0), Vec3::new(0.05, 0.03, 0.96)),
        (Vec3::new(-0.47, 0.0, 0.0), Vec3::new(0.05, 0.03, 0.96)),
    ] {
        pave(c, racine, look, &look.cadre, d, e, 0.0);
    }
}

/// Marque un côté ouvert qui n'est pas raccordé : un petit rond rouge s'il
/// donne sur du vide (une carte à venir peut encore le raccorder), une
/// grande croix rouge s'il bute contre une carte qui ne l'accepte pas.
pub fn marque_cote(c: &mut Commands, look: &Look, case: usize, side: usize, status: SideStatus) {
    let centre = pos_case(case) + bord(side, 0.07) + Vec3::Y * 0.2;
    match status {
        SideStatus::Open => {
            c.spawn((
                Mesh3d(look.disque.clone()),
                MeshMaterial3d(look.vide.clone()),
                Transform::from_translation(centre).with_scale(Vec3::new(0.15, 0.04, 0.15)),
                NotShadowCaster,
                Carte,
            ));
        }
        SideStatus::Blocked => {
            let racine = c.spawn((Transform::from_translation(centre + bord(side, 0.0) * -0.0), Visibility::default(), Carte)).id();
            for angle in [std::f32::consts::FRAC_PI_4, -std::f32::consts::FRAC_PI_4] {
                pave(c, racine, look, &look.bloque, Vec3::ZERO, Vec3::new(0.34, 0.05, 0.08), angle);
            }
        }
        SideStatus::Closed | SideStatus::Linked => {}
    }
}

/// La prairie, la table de bois, les cases vides et le décor (une fois pour toutes).
pub fn decor(c: &mut Commands, look: &Look, meshes: &mut Assets<Mesh>, mats: &mut Assets<StandardMaterial>) {
    let mut sans_ombre = |c: Color| mats.add(StandardMaterial { unlit: true, ..flat_material(c) });
    let herbe = sans_ombre(palette::GRASS);
    let clair = sans_ombre(palette::GRASS_LIGHT);
    let feuillage = sans_ombre(palette::FOLIAGE);
    let tronc = sans_ombre(Color::srgb(0.42, 0.28, 0.16));
    let eau = sans_ombre(Color::srgb_u8(0x58, 0xB4, 0xE6));
    let cone = meshes.add(Cone { radius: 0.5, height: 1.0 }.mesh().resolution(6));
    let cube = look.cube.clone();
    let mut p = |mesh: &Handle<Mesh>, mat: &Handle<StandardMaterial>, t: Transform| {
        c.spawn((Mesh3d(mesh.clone()), MeshMaterial3d(mat.clone()), t));
    };
    // La prairie.
    p(&cube, &herbe, Transform::from_xyz(2.0, -0.35, 0.0).with_scale(Vec3::new(40.0, 0.5, 30.0)));
    for (x, z, w, d) in [(-6.5, -5.5, 4.0, 3.0), (9.0, 6.0, 5.0, 3.0), (-9.0, 4.5, 3.0, 4.0), (10.5, -6.0, 3.0, 2.5)] {
        p(&cube, &clair, Transform::from_xyz(x, -0.09, z).with_scale(Vec3::new(w, 0.02, d)));
    }
    // La table de jeu : un cadre de bois clair autour des 7 × 7 cases.
    let t = N as f32 + 0.7;
    p(&cube, &look.bois, Transform::from_xyz(0.0, -0.06, 0.0).with_scale(Vec3::new(t, 0.12, t)));
    // L'étang et quelques arbres autour.
    p(&look.disque, &eau, Transform::from_xyz(-8.0, -0.07, -2.0).with_scale(Vec3::new(3.2, 0.05, 2.2)));
    for (x, z, h) in [(-6.5, 5.5, 1.8), (-9.5, 1.5, 1.4), (7.5, -5.5, 2.0), (11.5, 4.5, 1.6), (-5.0, -6.5, 1.5), (6.0, 6.5, 1.3)] {
        p(&cube, &tronc, Transform::from_xyz(x, h * 0.2, z).with_scale(Vec3::new(0.25, h * 0.4, 0.25)));
        p(&cone, &feuillage, Transform::from_xyz(x, h * 0.55 + 0.4, z).with_scale(Vec3::new(1.2, h, 1.2)));
    }
    // Les cases vides.
    for i in 0..N * N {
        let tuile = if (i / N + i % N).is_multiple_of(2) { &look.tuile_a } else { &look.tuile_b };
        let pos = pos_case(i);
        p(&cube, tuile, Transform::from_translation(pos - Vec3::Y * 0.05).with_scale(Vec3::new(0.94, 0.1, 0.94)));
    }
}
