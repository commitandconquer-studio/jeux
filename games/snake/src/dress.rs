//! Habille les personnages Mini Characters de Kenney : peau, cheveux, haut, bas et chaussures
//! changent de couleur selon le numéro d'apparence (`game_common::look`).
//!
//! Tous les modèles partagent une palette de 512×512 px faite de bandes de couleur ; peau, cheveux
//! et vêtements y partagent souvent la même bande. On ne peut donc pas recolorer la palette : on
//! réécrit les coordonnées de texture de chaque sommet selon son rôle (peau, cheveux, haut, bas,
//! chaussures) pour qu'il pointe vers une petite zone de couleur à lui. Les maillages réécrits sont
//! partagés par tous les personnages d'un même modèle ; il n'y a qu'une texture (128×128) et un
//! matériau par apparence.
use std::collections::HashMap;

use bevy::asset::RenderAssetUsages;
use bevy::mesh::VertexAttributeValues;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use game_common::flat_material;
use game_common::look::{Look, Rgb};

use crate::people::Person;

/// Bande de peau du visage de chaque modèle (a, b, c… alternés homme/femme), en (colonne de 32 px, ligne de 128 px).
const SKIN_STRIP: [(i32, i32); 12] = [
    (13, 3),
    (13, 3),
    (15, 3),
    (15, 3),
    (15, 3),
    (11, 3),
    (15, 3),
    (15, 3),
    (15, 3),
    (15, 3),
    (11, 3),
    (15, 3),
];

#[derive(Clone, Copy)]
enum Role {
    Skin = 0,
    Hair = 1,
    Top = 2,
    Bottom = 3,
    Shoes = 4,
}

/// Nombre d'apparences différentes (une texture chacune) : au-delà, on réutilise.
pub const LOOKS: u32 = 96;

#[derive(Resource)]
pub struct Atlas {
    base: Handle<Image>,
    /// La palette de Kenney réduite à 128×128 (RGBA).
    small: Option<Vec<u8>>,
    materials: HashMap<u32, Handle<StandardMaterial>>,
    meshes: HashMap<(usize, AssetId<Mesh>), Handle<Mesh>>,
}

pub fn setup(mut commands: Commands, server: Res<AssetServer>) {
    commands.insert_resource(Atlas {
        base: server.load("kenney/mini-characters/Textures/colormap.png"),
        small: None,
        materials: HashMap::new(),
        meshes: HashMap::new(),
    });
}

/// Réduit la palette : chaque bande est unie ou en dégradé vertical, un échantillon par bloc de 4×4 suffit.
fn shrink(base: &Image) -> Option<Vec<u8>> {
    let data = base.data.as_ref()?;
    if base.width() != 512 || base.height() != 512 || data.len() < 512 * 512 * 4 {
        return None;
    }
    let mut out = vec![255u8; 128 * 128 * 4];
    for y in 0..128 {
        for x in 0..128 {
            let s = ((y * 4 + 2) * 512 + (x * 4 + 2)) * 4;
            let d = (y * 128 + x) * 4;
            out[d..d + 4].copy_from_slice(&data[s..s + 4]);
        }
    }
    Some(out)
}

/// Écrit les cinq zones de couleur dans le haut de la palette (inutilisé chez Kenney) :
/// pour chaque rôle, une bande unie puis une bande en dégradé (clair en haut, foncé en bas).
fn paint(small: &[u8], look: &Look) -> Vec<u8> {
    let mut data = small.to_vec();
    let colors: [Rgb; 5] = [look.skin, look.hair, look.top, look.bottom, look.shoes];
    for (role, c) in colors.iter().enumerate() {
        for y in 0..32usize {
            let f = 1.10 - 0.30 * y as f32 / 31.0;
            for x in 0..16usize {
                let k = if x < 8 { 1.0 } else { f };
                let d = (y * 128 + role * 16 + x) * 4;
                for ch in 0..3 {
                    data[d + ch] = (c[ch] as f32 * k).clamp(0.0, 255.0) as u8;
                }
                data[d + 3] = 255;
            }
        }
    }
    data
}

fn strip(uv: [f32; 2]) -> (i32, i32) {
    (((uv[0] * 16.0).floor() as i32).clamp(0, 15), ((uv[1] * 4.0).floor() as i32).clamp(0, 3))
}

/// Réécrit les coordonnées de texture de `mesh` pour qu'elles pointent vers les zones de rôle.
fn remap(mesh: &Mesh, skin: (i32, i32)) -> Option<Mesh> {
    let Some(VertexAttributeValues::Float32x2(uv)) = mesh.attribute(Mesh::ATTRIBUTE_UV_0) else { return None };
    let Some(VertexAttributeValues::Float32x3(pos)) = mesh.attribute(Mesh::ATTRIBUTE_POSITION) else { return None };
    let Some(VertexAttributeValues::Uint16x4(joints)) = mesh.attribute(Mesh::ATTRIBUTE_JOINT_INDEX) else { return None };
    let Some(VertexAttributeValues::Float32x4(weights)) = mesh.attribute(Mesh::ATTRIBUTE_JOINT_WEIGHT) else { return None };

    // Articulation dominante de chaque sommet : 1-2 jambes, 3-5 buste et bras, 6 tête.
    let dominant: Vec<u16> = joints
        .iter()
        .zip(weights)
        .map(|(j, w)| {
            let k = (0..4).max_by(|a, b| w[*a].total_cmp(&w[*b])).unwrap_or(0);
            j[k]
        })
        .collect();
    let head = dominant.iter().filter(|j| **j == 6).count() * 2 > dominant.len();
    let mut count: HashMap<(i32, i32), usize> = HashMap::new();
    for u in uv {
        *count.entry(strip(*u)).or_default() += 1;
    }

    let mut out = uv.clone();
    for (i, u) in uv.iter().enumerate() {
        let s = strip(*u);
        let n = count[&s];
        let role = if s == skin {
            Some(Role::Skin)
        } else if head {
            // Les petites bandes de la tête (yeux, bouche, barrette) gardent leur couleur.
            (n >= 100).then_some(Role::Hair)
        } else if n < 10 {
            None
        } else {
            Some(match dominant[i] {
                1 | 2 | 0 => {
                    if pos[i][1] < 0.045 { Role::Shoes } else { Role::Bottom }
                }
                _ => Role::Top,
            })
        };
        let Some(role) = role else { continue };
        let (px, py) = (u[0] * 512.0, u[1] * 512.0);
        let (du, dv) = (px - s.0 as f32 * 32.0, py - s.1 as f32 * 128.0);
        out[i] = [(role as i32 as f32 * 64.0 + (s.0 % 2) as f32 * 32.0 + du) / 512.0, dv / 512.0];
    }
    let mut m = mesh.clone();
    m.insert_attribute(Mesh::ATTRIBUTE_UV_0, VertexAttributeValues::Float32x2(out));
    Some(m)
}

/// Habille les personnages dès que leurs maillages apparaissent.
#[allow(clippy::too_many_arguments)]
pub fn dress(
    mut commands: Commands,
    mut atlas: ResMut<Atlas>,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    fresh: Query<Entity, Added<Mesh3d>>,
    mesh_of: Query<&Mesh3d>,
    parents: Query<&ChildOf>,
    persons: Query<&Person>,
    mut pending: Local<Vec<Entity>>,
) {
    if atlas.small.is_none() {
        if let Some(img) = images.get(&atlas.base) {
            atlas.small = shrink(img);
        }
    }
    let todo: Vec<Entity> = pending.drain(..).chain(fresh.iter()).collect();
    for e in todo {
        // Le personnage dont ce maillage fait partie (il peut ne pas en avoir : décor, obstacle…).
        let mut up = e;
        let person = loop {
            if let Ok(p) = persons.get(up) {
                break Some(p);
            }
            match parents.get(up) {
                Ok(p) => up = p.parent(),
                Err(_) => break None,
            }
        };
        let Some((model, id)) = person.and_then(|p| p.look()) else { continue };
        let Some(small) = atlas.small.clone() else {
            pending.push(e);
            continue;
        };
        let Ok(handle) = mesh_of.get(e) else { continue };
        let key = (model, handle.0.id());
        if !atlas.meshes.contains_key(&key) {
            let Some(src) = meshes.get(&handle.0) else {
                pending.push(e);
                continue;
            };
            let Some(new) = remap(src, SKIN_STRIP[model]) else { continue };
            let h = meshes.add(new);
            atlas.meshes.insert(key, h);
        }
        let mesh = atlas.meshes[&key].clone();
        let mat = atlas
            .materials
            .entry(id)
            .or_insert_with(|| {
                let img = Image::new(
                    Extent3d { width: 128, height: 128, depth_or_array_layers: 1 },
                    TextureDimension::D2,
                    paint(&small, &Look::from_id(id as u64)),
                    TextureFormat::Rgba8UnormSrgb,
                    RenderAssetUsages::RENDER_WORLD,
                );
                materials.add(StandardMaterial { base_color_texture: Some(images.add(img)), double_sided: true, cull_mode: None, ..flat_material(Color::WHITE) })
            })
            .clone();
        commands.entity(e).insert((Mesh3d(mesh), MeshMaterial3d(mat)));
    }
}
