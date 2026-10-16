//! Identité du studio Commit & Conquer : couleurs de la charte
//! (`packaging/branding/04_palette_couleurs.json`) et emblème « CC » en 3D
//! low poly, avec son drapeau.
use bevy::asset::RenderAssetUsages;
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::prelude::*;

use crate::Rng;

/// Couleurs officielles du studio.
pub mod colors {
    use bevy::prelude::Color;

    /// Premier C, technologie, créativité.
    pub const BLUE: Color = Color::srgb_u8(0x16, 0x8F, 0xE8);
    /// Deuxième C, univers ludiques.
    pub const GREEN: Color = Color::srgb_u8(0x80, 0xC9, 0x2B);
    /// Facettes éclairées du low poly.
    pub const LEMON: Color = Color::srgb_u8(0xD8, 0xF4, 0x3F);
    /// Drapeau, détails emblématiques.
    pub const CORAL: Color = Color::srgb_u8(0xFF, 0x49, 0x3E);
    /// Typographie et contraste.
    pub const NIGHT: Color = Color::srgb_u8(0x14, 0x2C, 0x40);
    /// Décors, ciel, interfaces.
    pub const SKY: Color = Color::srgb_u8(0x8B, 0xD9, 0xFF);
    /// Arrière-plans clairs.
    pub const IVORY: Color = Color::srgb_u8(0xFF, 0xF9, 0xED);
}

/// Police du logo : Russo One.
pub const FONT_LOGO: &str = "fonts/RussoOne-Regular.ttf";
/// Emblème rendu en PNG transparent par `tools/logo-gen` (icônes).
pub const EMBLEM_IMAGE: &str = "branding/embleme.png";
/// Emblème officiel du kit de marque, pour le logo des menus.
pub const EMBLEM_MENU: &str = "branding/embleme-officiel.png";
/// Illustration de l'écran de démarrage (16:9).
pub const SPLASH_IMAGE: &str = "branding/splash.png";
/// Proportions de l'illustration (largeur / hauteur).
pub const SPLASH_RATIO: f32 = 1672.0 / 941.0;

/// Teintes d'une lettre : face avant, dessus (éclairé), côtés (à l'ombre).
struct Shades {
    front: Color,
    top: Color,
    side: Color,
}

/// Construit le maillage d'une lettre C extrudée, centrée sur l'origine,
/// face avant vers +Z. Les couleurs sont dans les sommets, avec de petites
/// variations par facette pour l'effet low poly.
fn letter_c(shades: &Shades, rng: &mut Rng) -> Mesh {
    // La lettre est faite de trois blocs convexes (bas, gauche, haut),
    // avec le coin extérieur en haut à gauche coupé.
    const W: f32 = 4.2;
    const H: f32 = 4.8;
    const T: f32 = 1.65;
    const K: f32 = 1.3;
    let pieces: [&[Vec2]; 3] = [
        &[vec2(0.0, 0.0), vec2(W, 0.0), vec2(W, T), vec2(0.0, T)],
        &[vec2(0.0, T), vec2(T, T), vec2(T, H - T), vec2(0.0, H - T)],
        &[vec2(0.0, H - T), vec2(W, H - T), vec2(W, H), vec2(K, H), vec2(0.0, H - K)],
    ];
    let center = vec2(W / 2.0, H / 2.0);
    let depth = 2.1;
    let mut b = Builder::default();
    for piece in pieces {
        let pts: Vec<Vec2> = piece.iter().map(|p| *p - center).collect();
        prism(&mut b, &pts, depth, shades, rng);
    }
    b.mesh()
}

#[derive(Default)]
struct Builder {
    positions: Vec<[f32; 3]>,
    normals: Vec<[f32; 3]>,
    colors: Vec<[f32; 4]>,
}

impl Builder {
    fn tri(&mut self, a: Vec3, b: Vec3, c: Vec3, color: Color) {
        let n = (b - a).cross(c - a).normalize_or_zero();
        let lin = color.to_linear();
        for p in [a, b, c] {
            self.positions.push(p.to_array());
            self.normals.push(n.to_array());
            self.colors.push([lin.red, lin.green, lin.blue, 1.0]);
        }
    }

    fn mesh(self) -> Mesh {
        let count = self.positions.len() as u32;
        Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default())
            .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, self.positions)
            .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, self.normals)
            .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, self.colors)
            .with_inserted_indices(Indices::U32((0..count).collect()))
    }
}

/// Nuance une couleur de ±`amount` pour différencier les facettes.
fn jitter(color: Color, rng: &mut Rng, amount: f32) -> Color {
    let k = 1.0 + (rng.next_f32() * 2.0 - 1.0) * amount;
    let c = color.to_srgba();
    Color::srgb((c.red * k).min(1.0), (c.green * k).min(1.0), (c.blue * k).min(1.0))
}

/// Extrude un polygone convexe (sens trigonométrique) sur `depth` selon Z.
fn prism(b: &mut Builder, pts: &[Vec2], depth: f32, shades: &Shades, rng: &mut Rng) {
    let front = depth / 2.0;
    let back = -depth / 2.0;
    // Face avant : éventail depuis le centre, chaque triangle légèrement nuancé.
    let mid = pts.iter().copied().sum::<Vec2>() / pts.len() as f32;
    for i in 0..pts.len() {
        let (p, q) = (pts[i], pts[(i + 1) % pts.len()]);
        let color = jitter(shades.front, rng, 0.05);
        b.tri(mid.extend(front), p.extend(front), q.extend(front), color);
        b.tri(mid.extend(back), q.extend(back), p.extend(back), shades.side);
    }
    // Côtés : la teinte dépend de l'orientation (dessus éclairé, reste à l'ombre).
    for i in 0..pts.len() {
        let (p, q) = (pts[i], pts[(i + 1) % pts.len()]);
        let edge = q - p;
        let outward = vec2(edge.y, -edge.x).normalize_or_zero();
        let base = if outward.y > 0.5 { shades.top } else { shades.side };
        let (pf, qf, pb, qb) = (p.extend(front), q.extend(front), p.extend(back), q.extend(back));
        b.tri(pf, pb, qb, jitter(base, rng, 0.06));
        b.tri(pf, qb, qf, jitter(base, rng, 0.06));
    }
}

/// Drapeau corail en trois facettes, attaché au mât en x = 0, de `top` à
/// `top - height`.
fn flag_mesh(rng: &mut Rng) -> Mesh {
    let mut b = Builder::default();
    let (top, height, length) = (0.0, 1.3, 2.1);
    let a = vec3(0.0, top, 0.0);
    let c = vec3(0.0, top - height, 0.0);
    let fold = vec3(length * 0.45, top - height * 0.5, 0.25);
    let tip = vec3(length, top - height * 0.5, 0.0);
    let m = vec3(0.0, top - height * 0.5, 0.0);
    b.tri(a, m, fold, jitter(colors::CORAL, rng, 0.04));
    b.tri(m, c, fold, jitter(colors::CORAL, rng, 0.08));
    b.tri(a, fold, tip, jitter(colors::CORAL, rng, 0.03));
    b.tri(fold, c, tip, jitter(colors::CORAL, rng, 0.1));
    b.mesh()
}

/// Mélange deux couleurs (t = 0 : a, t = 1 : b).
fn mix(a: Color, b: Color, t: f32) -> Color {
    let (a, b) = (a.to_srgba(), b.to_srgba());
    Color::srgb(a.red + (b.red - a.red) * t, a.green + (b.green - a.green) * t, a.blue + (b.blue - a.blue) * t)
}

/// Fait apparaître l'emblème « CC » (centré sur `at`) ; renvoie son parent.
/// Les matériaux sont non éclairés : les teintes de la charte sont rendues
/// telles quelles, l'ombrage est peint dans les sommets.
pub fn spawn_emblem(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    at: Transform,
) -> Entity {
    let mut rng = Rng::new(0xC0FF_EE11);
    let painted = materials.add(StandardMaterial { base_color: Color::WHITE, unlit: true, cull_mode: None, ..default() });
    let blue = Shades {
        front: colors::BLUE,
        top: mix(colors::BLUE, colors::SKY, 0.55),
        side: mix(colors::BLUE, colors::NIGHT, 0.35),
    };
    let green = Shades {
        front: colors::GREEN,
        top: mix(colors::GREEN, colors::LEMON, 0.7),
        side: mix(colors::GREEN, colors::NIGHT, 0.3),
    };
    let root = commands.spawn((at, Visibility::default())).id();
    let c1 = meshes.add(letter_c(&blue, &mut rng));
    let c2 = meshes.add(letter_c(&green, &mut rng));
    commands.spawn((Mesh3d(c1), MeshMaterial3d(painted.clone()), Transform::from_xyz(-1.15, 1.0, -1.1), ChildOf(root)));
    commands.spawn((Mesh3d(c2), MeshMaterial3d(painted.clone()), Transform::from_xyz(1.15, -1.0, 1.1), ChildOf(root)));
    // Mât planté au bout de la barre du haut du premier C, et son drapeau.
    let pole_base = vec3(-1.15 + 1.4, 1.0 + 2.4, -1.1 - 0.4);
    let pole_h = 2.9;
    let pole = materials.add(StandardMaterial { base_color: colors::NIGHT, unlit: true, ..default() });
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(0.34, pole_h, 0.34))),
        MeshMaterial3d(pole),
        Transform::from_translation(pole_base + Vec3::Y * pole_h / 2.0),
        ChildOf(root),
    ));
    commands.spawn((
        Mesh3d(meshes.add(flag_mesh(&mut rng))),
        MeshMaterial3d(painted),
        Transform::from_translation(pole_base + vec3(0.17, pole_h - 0.05, 0.0)),
        ChildOf(root),
    ));
    root
}
