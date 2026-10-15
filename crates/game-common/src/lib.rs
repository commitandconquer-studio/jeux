//! Style visuel et code partagés entre les jeux Commit & Conquer.
//!
//! Le « style maison » : 3D low-poly en vue isométrique, couleurs unies,
//! matériaux mats et ombres douces.

pub mod audio;
pub mod sounds;
pub mod ui;

/// Fait pointer le jeu vers la bibliothèque d'assets commune (`assets/` à la
/// racine du projet) : sons, musiques, polices partagés par tous les jeux.
///
/// Avec `cargo run`, Bevy part du dossier du jeu (`games/<jeu>/`) ; une fois
/// le jeu distribué, `assets/` est copié à côté de l'exécutable.
pub fn shared_assets() -> bevy::asset::AssetPlugin {
    let file_path = if std::env::var_os("CARGO_MANIFEST_DIR").is_some() { "../../assets" } else { "assets" };
    bevy::asset::AssetPlugin { file_path: file_path.into(), ..Default::default() }
}

use bevy::camera::ScalingMode;
use bevy::light::{CascadeShadowConfigBuilder, DirectionalLightShadowMap};
use bevy::prelude::*;

/// Palette de couleurs commune à tous les jeux.
pub mod palette {
    use bevy::prelude::Color;

    pub const SKY: Color = Color::srgb(0.93, 0.91, 0.86);
    pub const GRASS_LIGHT: Color = Color::srgb(0.71, 0.88, 0.47);
    pub const GRASS: Color = Color::srgb(0.64, 0.83, 0.42);
    pub const PATH: Color = Color::srgb(0.55, 0.59, 0.72);
    pub const FOLIAGE: Color = Color::srgb(0.38, 0.66, 0.38);
    pub const FOLIAGE_DARK: Color = Color::srgb(0.29, 0.55, 0.31);
    pub const ROOF: Color = Color::srgb(0.91, 0.52, 0.35);
    pub const WALL: Color = Color::srgb(0.90, 0.88, 0.86);
    pub const YELLOW: Color = Color::srgb(0.98, 0.82, 0.18);
    pub const CYAN: Color = Color::srgb(0.20, 0.70, 0.86);
    pub const LILAC: Color = Color::srgb(0.78, 0.68, 0.93);
    pub const WHITE: Color = Color::srgb(0.95, 0.95, 0.95);
    pub const NAVY: Color = Color::srgb(0.20, 0.18, 0.28);
    pub const PINK: Color = Color::srgb(0.94, 0.33, 0.55);
    pub const TEXT_DARK: Color = Color::srgb(0.16, 0.16, 0.22);
}

/// Ajoute le fond, la lumière ambiante et le soleil avec ombres.
pub struct LowPolyStylePlugin;

impl Plugin for LowPolyStylePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(palette::SKY))
            .insert_resource(GlobalAmbientLight {
                color: Color::WHITE,
                brightness: 900.0,
                ..default()
            })
            .insert_resource(DirectionalLightShadowMap { size: 4096 })
            .add_systems(Startup, spawn_sun);
    }
}

fn spawn_sun(mut commands: Commands) {
    commands.spawn((
        DirectionalLight {
            illuminance: 6_000.0,
            shadow_maps_enabled: true,
            ..default()
        },
        // Une seule cascade resserrée sur la scène (quelques dizaines de mètres) :
        // ombres nettes, sans rayures. Les réglages par défaut visent 1 km.
        CascadeShadowConfigBuilder {
            num_cascades: 1,
            minimum_distance: 1.0,
            maximum_distance: 80.0,
            first_cascade_far_bound: 80.0,
            ..default()
        }
        .build(),
        Transform::from_xyz(-6.0, 14.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

/// Caméra orthographique en vue plongeante, centrée sur `target`.
///
/// Elle regarde depuis le sud (+Z), légèrement décalée vers l'est, ce qui
/// montre les faces des cubes tout en gardant « haut écran » ≈ -Z.
///
/// `min_width` / `min_height` : zone du monde toujours visible, quel que soit
/// le format de l'écran (paysage sur PC, portrait sur téléphone).
pub fn iso_camera(target: Vec3, min_width: f32, min_height: f32) -> impl Bundle {
    (
        Camera3d::default(),
        iso_projection(min_width, min_height),
        Transform::from_translation(target + ISO_OFFSET).looking_at(target, Vec3::Y),
    )
}

/// Décalage de la caméra par rapport au point visé.
pub const ISO_OFFSET: Vec3 = Vec3::new(6.0, 18.0, 14.0);

/// Projection orthographique garantissant une zone visible minimale.
pub fn iso_projection(min_width: f32, min_height: f32) -> Projection {
    Projection::from(OrthographicProjection {
        scaling_mode: ScalingMode::AutoMin { min_width, min_height },
        ..OrthographicProjection::default_3d()
    })
}

/// Matériau mat à couleur unie, pour le rendu « jouet ».
pub fn flat_material(color: Color) -> StandardMaterial {
    StandardMaterial {
        base_color: color,
        perceptual_roughness: 0.9,
        reflectance: 0.2,
        ..default()
    }
}

/// Générateur pseudo-aléatoire minimal (xorshift), sans dépendance.
#[derive(Resource)]
pub struct Rng(u64);

impl Rng {
    pub fn new(seed: u64) -> Self {
        Self(seed.max(1))
    }

    pub fn from_time() -> Self {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos() as u64)
            .unwrap_or(0x9E37_79B9_7F4A_7C15);
        Self::new(nanos)
    }

    pub fn next_u64(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }

    /// Entier dans `0..max`.
    pub fn below(&mut self, max: u32) -> u32 {
        (self.next_u64() % max as u64) as u32
    }
}

/// Score courant et meilleur score.
#[derive(Resource, Debug, Default, Clone, Copy)]
pub struct Score {
    pub current: u32,
    pub best: u32,
}

impl Score {
    pub fn add(&mut self, points: u32) {
        self.current += points;
        self.best = self.best.max(self.current);
    }

    pub fn reset(&mut self) {
        self.current = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn best_score_survives_reset() {
        let mut s = Score::default();
        s.add(5);
        s.reset();
        s.add(2);
        assert_eq!((s.current, s.best), (2, 5));
    }

    #[test]
    fn rng_stays_in_range() {
        let mut r = Rng::new(42);
        assert!((0..1000).all(|_| r.below(7) < 7));
    }
}
