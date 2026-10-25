//! Fusionne le décor fixe de chaque voie : au lieu de centaines de petites
//! entités (arbres, traverses, pointillés…), une seule par matériau. Beaucoup
//! moins de travail pour le processeur et la carte graphique, surtout sur les
//! petites cartes et dans le navigateur.
use std::collections::HashMap;

use bevy::prelude::*;

use crate::lanes::{Lane, RailLane};

pub fn merge_lanes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    new: Query<(Entity, &Children, Option<&RailLane>), Added<Lane>>,
    parts: Query<(&Mesh3d, &MeshMaterial3d<StandardMaterial>, &Transform), Without<crate::extra::GenAnim>>,
) {
    for (lane, children, rail) in &new {
        let keep = rail.map(|r| r.light());
        let mut groups: HashMap<AssetId<StandardMaterial>, (Handle<StandardMaterial>, Mesh)> = HashMap::new();
        for child in children.iter() {
            if Some(child) == keep {
                continue; // le feu change de matériau : on le laisse tel quel
            }
            let Ok((mesh, mat, tf)) = parts.get(child) else {
                continue;
            };
            let Some(source) = meshes.get(&mesh.0) else {
                continue;
            };
            // Tout en sommets séparés, pour que les maillages se combinent.
            let part = source.clone().with_duplicated_vertices().transformed_by(*tf);
            match groups.get_mut(&mat.0.id()) {
                Some((_, merged)) => {
                    if merged.merge(&part).is_err() {
                        continue; // formes incompatibles : la pièce reste seule
                    }
                }
                None => {
                    groups.insert(mat.0.id(), (mat.0.clone(), part));
                }
            }
            commands.entity(child).despawn();
        }
        for (_, (mat, mesh)) in groups {
            commands.spawn((Mesh3d(meshes.add(mesh)), MeshMaterial3d(mat), Transform::default(), ChildOf(lane)));
        }
    }
}
