// Généré par tools/gen-mondes.py à partir de data/import/*.json : ne pas modifier à la main.
//! Décors des mondes décrits en pièces simples (cube, cylindre, cône, sphère).
#![allow(dead_code)]
use game_common::tx;

use crate::extra::{Anim, AnimKind, Mat, Model, Piece, Shape};

const PIECES_0: &[Piece] = &[
    Piece { shape: Shape::Cyl, pos: [0.0, 0.2, 0.0], size: [0.2, 0.4, 0.2], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [0.03, 0.58, 0.0], size: [0.18, 0.4, 0.18], rot: [0.0, 0.0, -4.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [0.07, 0.95, 0.0], size: [0.16, 0.4, 0.16], rot: [0.0, 0.0, -8.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [0.12, 1.3, 0.0], size: [0.14, 0.4, 0.14], rot: [0.0, 0.0, -12.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cone, pos: [0.15, 1.58, 0.0], size: [0.14, 0.7, 0.14], rot: [0.0, 0.0, 70.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cone, pos: [0.15, 1.58, 0.0], size: [0.14, 0.7, 0.14], rot: [0.0, 0.0, -70.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cone, pos: [0.12, 1.6, 0.0], size: [0.14, 0.7, 0.14], rot: [0.0, 0.0, 70.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cone, pos: [0.12, 1.6, 0.0], size: [0.14, 0.7, 0.14], rot: [0.0, 0.0, -70.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cone, pos: [0.12, 1.6, 0.0], size: [0.14, 0.7, 0.14], rot: [70.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cone, pos: [0.12, 1.6, 0.0], size: [0.14, 0.7, 0.14], rot: [-70.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cone, pos: [0.12, 1.62, 0.0], size: [0.14, 0.5, 0.14], rot: [45.0, 0.0, 45.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cone, pos: [0.12, 1.62, 0.0], size: [0.14, 0.5, 0.14], rot: [-45.0, 0.0, -45.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Sphere, pos: [0.15, 1.42, 0.08], size: [0.1, 0.1, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Sphere, pos: [0.15, 1.42, -0.08], size: [0.1, 0.1, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
];
const ANIMS_0: &[Anim] = &[
];
const PIECES_1: &[Piece] = &[
    Piece { shape: Shape::Cyl, pos: [0.0, 0.25, 0.0], size: [0.1, 0.5, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cone, pos: [0.1, 0.65, 0.2], size: [0.12, 0.75, 0.12], rot: [60.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cone, pos: [-0.1, 0.7, 0.12], size: [0.12, 0.6, 0.12], rot: [50.0, 0.0, 25.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cone, pos: [0.1, 0.65, -0.2], size: [0.12, 0.75, 0.12], rot: [-60.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cone, pos: [-0.1, 0.7, -0.12], size: [0.12, 0.6, 0.12], rot: [-50.0, 0.0, 25.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cone, pos: [0.2, 0.65, 0.0], size: [0.12, 0.75, 0.12], rot: [0.0, 0.0, -60.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cone, pos: [0.14, 0.72, 0.1], size: [0.12, 0.6, 0.12], rot: [-40.0, 0.0, -45.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cone, pos: [-0.2, 0.65, 0.0], size: [0.12, 0.75, 0.12], rot: [0.0, 0.0, 60.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cone, pos: [-0.14, 0.72, 0.1], size: [0.12, 0.6, 0.12], rot: [-40.0, 0.0, 45.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cone, pos: [0.0, 0.8, 0.0], size: [0.12, 0.7, 0.12], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cone, pos: [0.0, 0.72, 0.0], size: [0.1, 0.5, 0.1], rot: [30.0, 0.0, 30.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cone, pos: [0.0, 0.72, 0.0], size: [0.1, 0.5, 0.1], rot: [-30.0, 0.0, -30.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Sphere, pos: [0.0, 0.12, 0.0], size: [0.3, 0.2, 0.3], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Sphere, pos: [0.18, 0.1, 0.1], size: [0.2, 0.14, 0.2], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
];
const ANIMS_1: &[Anim] = &[
];
const PIECES_2: &[Piece] = &[
    Piece { shape: Shape::Sphere, pos: [0.0, 0.3, 0.0], size: [0.95, 0.6, 0.8], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Sphere, pos: [0.2, 0.55, 0.1], size: [0.55, 0.45, 0.5], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: None },
    Piece { shape: Shape::Sphere, pos: [-0.25, 0.2, -0.2], size: [0.5, 0.4, 0.45], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cone, pos: [0.0, 0.75, 0.0], size: [0.3, 0.35, 0.3], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.4, 0.41], size: [0.4, 0.03, 0.02], rot: [0.0, 0.0, 20.0], mat: Mat::LumiereRouge, id: Some("lave") },
    Piece { shape: Shape::Cube, pos: [0.1, 0.25, 0.38], size: [0.3, 0.03, 0.02], rot: [0.0, 0.0, -30.0], mat: Mat::LumiereRouge, id: Some("lave") },
    Piece { shape: Shape::Sphere, pos: [0.35, 0.2, -0.3], size: [0.3, 0.25, 0.3], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: None },
    Piece { shape: Shape::Cube, pos: [-0.2, 0.45, -0.38], size: [0.3, 0.03, 0.02], rot: [0.0, 0.0, 10.0], mat: Mat::LumiereRouge, id: Some("lave") },
    Piece { shape: Shape::Sphere, pos: [-0.3, 0.15, 0.3], size: [0.3, 0.25, 0.3], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Sphere, pos: [0.3, 0.5, -0.2], size: [0.3, 0.3, 0.3], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cone, pos: [-0.2, 0.6, 0.1], size: [0.2, 0.25, 0.2], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Sphere, pos: [0.0, 0.1, 0.0], size: [1.0, 0.2, 0.9], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: None },
];
const ANIMS_2: &[Anim] = &[
    Anim { target: "lave", kind: AnimKind::Pulse, amp: 0.0 },
];
const PIECES_3: &[Piece] = &[
    Piece { shape: Shape::Sphere, pos: [0.0, 0.3, 0.0], size: [0.6, 0.54, 0.6], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Sphere, pos: [0.25, 0.225, 0.1], size: [0.45, 0.405, 0.45], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Sphere, pos: [-0.25, 0.225, 0.1], size: [0.45, 0.405, 0.45], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Sphere, pos: [0.1, 0.25, -0.25], size: [0.5, 0.45, 0.5], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Sphere, pos: [-0.1, 0.2, -0.25], size: [0.4, 0.36, 0.4], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Sphere, pos: [0.1, 0.55, 0.25], size: [0.08, 0.08, 0.08], rot: [0.0, 0.0, 0.0], mat: Mat::LumiereRouge, id: None },
    Piece { shape: Shape::Sphere, pos: [-0.2, 0.5, 0.28], size: [0.08, 0.08, 0.08], rot: [0.0, 0.0, 0.0], mat: Mat::LumiereRouge, id: None },
    Piece { shape: Shape::Sphere, pos: [0.3, 0.35, 0.3], size: [0.08, 0.08, 0.08], rot: [0.0, 0.0, 0.0], mat: Mat::LumiereRouge, id: None },
    Piece { shape: Shape::Sphere, pos: [0.0, 0.7, 0.0], size: [0.08, 0.08, 0.08], rot: [0.0, 0.0, 0.0], mat: Mat::LumiereRouge, id: None },
    Piece { shape: Shape::Sphere, pos: [-0.3, 0.4, -0.2], size: [0.08, 0.08, 0.08], rot: [0.0, 0.0, 0.0], mat: Mat::LumiereRouge, id: None },
    Piece { shape: Shape::Sphere, pos: [0.2, 0.45, -0.3], size: [0.08, 0.08, 0.08], rot: [0.0, 0.0, 0.0], mat: Mat::LumiereRouge, id: None },
    Piece { shape: Shape::Sphere, pos: [0.3, 0.3, -0.05], size: [0.07, 0.07, 0.07], rot: [0.0, 0.0, 0.0], mat: Mat::LumiereRouge, id: None },
    Piece { shape: Shape::Sphere, pos: [-0.1, 0.3, 0.3], size: [0.07, 0.07, 0.07], rot: [0.0, 0.0, 0.0], mat: Mat::LumiereRouge, id: None },
];
const ANIMS_3: &[Anim] = &[
];
const PIECES_4: &[Piece] = &[
    Piece { shape: Shape::Cube, pos: [0.0, 0.35, 0.0], size: [0.9, 0.7, 0.9], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.74, 0.0], size: [1.0, 0.08, 1.0], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cube, pos: [0.451, 0.3, 0.0], size: [0.02, 0.5, 0.3], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [0.451, 0.45, 0.3], size: [0.02, 0.2, 0.2], rot: [0.0, 0.0, 0.0], mat: Mat::Vitre, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.45, 0.451], size: [0.3, 0.2, 0.02], rot: [0.0, 0.0, 0.0], mat: Mat::Vitre, id: None },
    Piece { shape: Shape::Cube, pos: [0.451, 0.45, -0.3], size: [0.02, 0.2, 0.2], rot: [0.0, 0.0, 0.0], mat: Mat::Vitre, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.2, -0.451], size: [0.9, 0.04, 0.02], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [-0.2, 0.85, 0.2], size: [0.3, 0.12, 0.2], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cyl, pos: [0.2, 0.95, -0.2], size: [0.03, 0.4, 0.03], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Sphere, pos: [0.2, 1.17, -0.2], size: [0.08, 0.08, 0.08], rot: [0.0, 0.0, 0.0], mat: Mat::LumiereRouge, id: Some("balise") },
    Piece { shape: Shape::Cube, pos: [0.451, 0.62, 0.0], size: [0.02, 0.1, 0.5], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.1, 0.0], size: [0.95, 0.2, 0.95], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.4, 0.9, -0.35], size: [0.1, 0.1, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
];
const ANIMS_4: &[Anim] = &[
    Anim { target: "balise", kind: AnimKind::Blink, amp: 0.0 },
];
const PIECES_5: &[Piece] = &[
    Piece { shape: Shape::Cyl, pos: [0.0, 0.45, 0.3], size: [0.08, 0.9, 0.08], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.95, 0.3], size: [0.14, 0.1, 0.14], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [0.0, 0.45, -0.3], size: [0.08, 0.9, 0.08], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.95, -0.3], size: [0.14, 0.1, 0.14], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.8, 0.0], size: [0.02, 0.02, 0.6], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.6, 0.0], size: [0.02, 0.02, 0.6], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.4, 0.0], size: [0.02, 0.02, 0.6], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.25, 0.0], size: [0.02, 0.02, 0.6], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: None },
    Piece { shape: Shape::Sphere, pos: [0.0, 1.05, 0.3], size: [0.1, 0.1, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::LumiereRouge, id: Some("voyant") },
    Piece { shape: Shape::Sphere, pos: [0.0, 1.05, -0.3], size: [0.1, 0.1, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::LumiereRouge, id: Some("voyant") },
    Piece { shape: Shape::Cube, pos: [0.0, 0.04, 0.0], size: [0.3, 0.08, 0.8], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.55, 0.35], size: [0.02, 0.3, 0.02], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
];
const ANIMS_5: &[Anim] = &[
    Anim { target: "voyant", kind: AnimKind::BlinkAlt, amp: 0.0 },
];
const PIECES_6: &[Piece] = &[
    Piece { shape: Shape::Cyl, pos: [0.4, 0.45, 0.4], size: [0.08, 0.9, 0.08], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cyl, pos: [0.4, 0.45, -0.4], size: [0.08, 0.9, 0.08], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.4, 0.45, 0.4], size: [0.08, 0.9, 0.08], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.4, 0.45, -0.4], size: [0.08, 0.9, 0.08], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cyl, pos: [0.4, 0.45, -0.13], size: [0.05, 0.9, 0.05], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cyl, pos: [0.4, 0.45, 0.13], size: [0.05, 0.9, 0.05], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.4, 0.45, -0.13], size: [0.05, 0.9, 0.05], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.4, 0.45, 0.13], size: [0.05, 0.9, 0.05], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.13, 0.45, 0.4], size: [0.05, 0.9, 0.05], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cyl, pos: [0.13, 0.45, 0.4], size: [0.05, 0.9, 0.05], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.13, 0.45, -0.4], size: [0.05, 0.9, 0.05], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cyl, pos: [0.13, 0.45, -0.4], size: [0.05, 0.9, 0.05], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.92, 0.0], size: [0.9, 0.06, 0.9], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.04, 0.0], size: [0.95, 0.08, 0.95], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.5, 0.44], size: [0.8, 0.1, 0.02], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
];
const ANIMS_6: &[Anim] = &[
];
pub const JURA_BLOCKS: [Model; 7] = [
    Model { c2: [47, 143, 78], c3: [122, 77, 36], pieces: PIECES_0, anims: ANIMS_0 },
    Model { c2: [47, 143, 78], c3: [122, 77, 36], pieces: PIECES_1, anims: ANIMS_1 },
    Model { c2: [255, 90, 31], c3: [58, 48, 48], pieces: PIECES_2, anims: ANIMS_2 },
    Model { c2: [47, 143, 78], c3: [122, 77, 36], pieces: PIECES_3, anims: ANIMS_3 },
    Model { c2: [47, 125, 58], c3: [242, 194, 48], pieces: PIECES_4, anims: ANIMS_4 },
    Model { c2: [255, 210, 63], c3: [58, 58, 58], pieces: PIECES_5, anims: ANIMS_5 },
    Model { c2: [242, 194, 48], c3: [58, 58, 58], pieces: PIECES_6, anims: ANIMS_6 },
];
const PIECES_7: &[Piece] = &[
    Piece { shape: Shape::Cyl, pos: [0.0, 0.4, 0.0], size: [0.4, 0.8, 0.4], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [0.05, 1.1, 0.0], size: [0.36, 0.8, 0.36], rot: [0.0, 0.0, -3.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [0.12, 1.8, 0.0], size: [0.32, 0.8, 0.32], rot: [0.0, 0.0, -6.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [0.22, 2.5, 0.0], size: [0.28, 0.8, 0.28], rot: [0.0, 0.0, -9.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [0.35, 3.1, 0.0], size: [0.24, 0.8, 0.24], rot: [0.0, 0.0, -12.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cone, pos: [0.4, 3.5, 0.0], size: [0.3, 1.4, 0.3], rot: [0.0, 0.0, 60.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cone, pos: [0.4, 3.5, 0.0], size: [0.3, 1.4, 0.3], rot: [0.0, 0.0, -60.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cone, pos: [0.4, 3.5, 0.0], size: [0.3, 1.4, 0.3], rot: [60.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cone, pos: [0.4, 3.5, 0.0], size: [0.3, 1.4, 0.3], rot: [-60.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cone, pos: [0.4, 3.6, 0.0], size: [0.3, 1.0, 0.3], rot: [40.0, 0.0, 40.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cone, pos: [0.4, 3.6, 0.0], size: [0.3, 1.0, 0.3], rot: [-40.0, 0.0, -40.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cone, pos: [0.4, 3.6, 0.0], size: [0.3, 1.0, 0.3], rot: [40.0, 0.0, -40.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cone, pos: [0.4, 3.6, 0.0], size: [0.3, 1.0, 0.3], rot: [-40.0, 0.0, 40.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cone, pos: [0.4, 3.7, 0.0], size: [0.28, 0.8, 0.28], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Sphere, pos: [0.4, 3.3, 0.12], size: [0.18, 0.18, 0.18], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Sphere, pos: [0.4, 3.3, -0.12], size: [0.18, 0.18, 0.18], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
];
const ANIMS_7: &[Anim] = &[
];
const PIECES_8: &[Piece] = &[
    Piece { shape: Shape::Cone, pos: [0.0, 1.2, 0.0], size: [3.0, 2.4, 3.0], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cone, pos: [0.0, 2.55, 0.0], size: [1.4, 0.6, 1.4], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: None },
    Piece { shape: Shape::Cyl, pos: [0.0, 2.78, 0.0], size: [0.8, 0.1, 0.8], rot: [0.0, 0.0, 0.0], mat: Mat::LumiereRouge, id: Some("lave") },
    Piece { shape: Shape::Cube, pos: [0.6, 1.2, 0.5], size: [0.12, 1.0, 0.06], rot: [0.0, 0.0, 15.0], mat: Mat::LumiereRouge, id: Some("lave") },
    Piece { shape: Shape::Cube, pos: [-0.7, 1.0, 0.3], size: [0.12, 1.0, 0.06], rot: [0.0, 0.0, 15.0], mat: Mat::LumiereRouge, id: Some("lave") },
    Piece { shape: Shape::Cube, pos: [0.2, 0.8, -0.8], size: [0.12, 1.0, 0.06], rot: [0.0, 0.0, 15.0], mat: Mat::LumiereRouge, id: Some("lave") },
    Piece { shape: Shape::Sphere, pos: [0.0, 3.0, 0.0], size: [0.5, 0.5, 0.5], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("fumee") },
    Piece { shape: Shape::Sphere, pos: [0.2, 3.4, 0.0], size: [0.7, 0.6, 0.7], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("fumee") },
    Piece { shape: Shape::Sphere, pos: [0.4, 3.9, 0.0], size: [0.9, 0.7, 0.9], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("fumee") },
    Piece { shape: Shape::Cone, pos: [1.0, 0.35, 0.9], size: [0.5, 0.7, 0.5], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: None },
    Piece { shape: Shape::Cone, pos: [-1.0, 0.3, 0.8], size: [0.45, 0.6, 0.45], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Sphere, pos: [0.0, 0.15, 1.2], size: [0.9, 0.3, 0.7], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: None },
];
const ANIMS_8: &[Anim] = &[
    Anim { target: "lave", kind: AnimKind::Pulse, amp: 0.0 },
    Anim { target: "fumee", kind: AnimKind::DriftUp, amp: 0.0 },
];
const PIECES_9: &[Piece] = &[
    Piece { shape: Shape::Cube, pos: [0.0, 0.5, 0.0], size: [2.6, 1.0, 1.8], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 1.25, 0.0], size: [1.8, 0.6, 1.3], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 1.58, 0.0], size: [2.0, 0.08, 1.5], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 1.03, 0.0], size: [2.8, 0.08, 2.0], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cube, pos: [-1.0, 0.6, 0.91], size: [0.35, 0.4, 0.02], rot: [0.0, 0.0, 0.0], mat: Mat::Vitre, id: None },
    Piece { shape: Shape::Cube, pos: [-0.5, 0.6, 0.91], size: [0.35, 0.4, 0.02], rot: [0.0, 0.0, 0.0], mat: Mat::Vitre, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.6, 0.91], size: [0.35, 0.4, 0.02], rot: [0.0, 0.0, 0.0], mat: Mat::Vitre, id: None },
    Piece { shape: Shape::Cube, pos: [0.5, 0.6, 0.91], size: [0.35, 0.4, 0.02], rot: [0.0, 0.0, 0.0], mat: Mat::Vitre, id: None },
    Piece { shape: Shape::Cube, pos: [1.0, 0.6, 0.91], size: [0.35, 0.4, 0.02], rot: [0.0, 0.0, 0.0], mat: Mat::Vitre, id: None },
    Piece { shape: Shape::Cube, pos: [-0.55, 1.25, 0.66], size: [0.4, 0.3, 0.02], rot: [0.0, 0.0, 0.0], mat: Mat::Vitre, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 1.25, 0.66], size: [0.4, 0.3, 0.02], rot: [0.0, 0.0, 0.0], mat: Mat::Vitre, id: None },
    Piece { shape: Shape::Cube, pos: [0.55, 1.25, 0.66], size: [0.4, 0.3, 0.02], rot: [0.0, 0.0, 0.0], mat: Mat::Vitre, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.9, 0.92], size: [2.6, 0.12, 0.02], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.5, 0.45, 1.1], size: [0.2, 0.9, 0.2], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [0.5, 0.45, 1.1], size: [0.2, 0.9, 0.2], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.95, 1.15], size: [1.4, 0.1, 0.6], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Sphere, pos: [0.7, 1.9, 0.0], size: [0.9, 0.7, 0.9], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: None },
    Piece { shape: Shape::Sphere, pos: [0.7, 1.95, 0.0], size: [0.5, 0.4, 0.5], rot: [0.0, 0.0, 0.0], mat: Mat::Vitre, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.8, 1.9, -0.3], size: [0.06, 0.8, 0.06], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cone, pos: [-0.8, 2.35, -0.3], size: [0.4, 0.1, 0.4], rot: [0.0, 0.0, -70.0], mat: Mat::Blanc, id: None },
    Piece { shape: Shape::Sphere, pos: [-0.8, 2.4, -0.3], size: [0.1, 0.1, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::LumiereBleue, id: Some("balise") },
    Piece { shape: Shape::Cube, pos: [0.0, 0.28, 0.91], size: [0.5, 0.55, 0.02], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.1, 0.0], size: [2.8, 0.2, 2.0], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 1.0, 0.5], size: [2.6, 0.02, 0.02], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
];
const ANIMS_9: &[Anim] = &[
    Anim { target: "balise", kind: AnimKind::Pulse, amp: 0.0 },
];
const PIECES_10: &[Piece] = &[
    Piece { shape: Shape::Cyl, pos: [0.0, 1.2, 1.3], size: [0.45, 2.4, 0.45], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.1, 1.3], size: [0.8, 0.2, 0.8], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cone, pos: [0.1, 2.55, 1.3], size: [0.3, 0.4, 0.3], rot: [0.0, 0.0, 0.0], mat: Mat::LumiereRouge, id: Some("torche") },
    Piece { shape: Shape::Cyl, pos: [0.0, 1.2, -1.3], size: [0.45, 2.4, 0.45], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.1, -1.3], size: [0.8, 0.2, 0.8], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cone, pos: [0.1, 2.55, -1.3], size: [0.3, 0.4, 0.3], rot: [0.0, 0.0, 0.0], mat: Mat::LumiereRouge, id: Some("torche") },
    Piece { shape: Shape::Cube, pos: [0.0, 2.35, 0.0], size: [0.3, 0.35, 3.1], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 2.8, 0.0], size: [0.25, 0.5, 2.2], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [0.14, 2.8, 0.0], size: [0.04, 0.12, 1.9], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: None },
    Piece { shape: Shape::Cube, pos: [0.14, 2.8, 0.5], size: [0.04, 0.3, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [0.14, 2.8, -0.5], size: [0.04, 0.3, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 1.4, 0.0], size: [0.1, 0.1, 2.3], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
];
const ANIMS_10: &[Anim] = &[
    Anim { target: "torche", kind: AnimKind::Pulse, amp: 0.0 },
];
const PIECES_11: &[Piece] = &[
    Piece { shape: Shape::Sphere, pos: [0.0, 0.8, 0.0], size: [1.6, 1.44, 1.6], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Sphere, pos: [0.5, 1.5, 0.2], size: [1.2, 1.08, 1.2], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Sphere, pos: [-0.5, 0.5, 0.3], size: [1.0, 0.9, 1.0], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Sphere, pos: [0.2, 0.4, -0.5], size: [0.9, 0.81, 0.9], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Sphere, pos: [-0.3, 1.1, -0.2], size: [1.0, 0.9, 1.0], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cube, pos: [0.9, 1.0, 0.0], size: [0.1, 1.6, 0.6], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("eau") },
    Piece { shape: Shape::Cube, pos: [0.95, 0.6, 0.0], size: [0.1, 1.0, 0.4], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("eau") },
    Piece { shape: Shape::Cube, pos: [0.9, 1.5, 0.3], size: [0.1, 0.5, 0.3], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("eau") },
    Piece { shape: Shape::Cyl, pos: [1.3, 0.05, 0.0], size: [1.2, 0.1, 1.2], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cone, pos: [0.0, 1.9, 0.0], size: [0.6, 0.5, 0.6], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Sphere, pos: [-0.2, 0.15, -0.9], size: [0.5, 0.3, 0.5], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cube, pos: [0.95, 0.2, 0.0], size: [0.2, 0.04, 0.7], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Sphere, pos: [0.4, 0.1, 0.8], size: [0.3, 0.2, 0.3], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
];
const ANIMS_11: &[Anim] = &[
    Anim { target: "eau", kind: AnimKind::Float, amp: 0.05 },
];
const PIECES_12: &[Piece] = &[
    Piece { shape: Shape::Cyl, pos: [0.18, 1.0, 0.18], size: [0.06, 2.0, 0.06], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cyl, pos: [0.18, 1.0, -0.18], size: [0.06, 2.0, 0.06], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.18, 1.0, 0.18], size: [0.06, 2.0, 0.06], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.18, 1.0, -0.18], size: [0.06, 2.0, 0.06], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.4, 0.0], size: [0.45, 0.04, 0.45], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.4, 0.0], size: [0.04, 0.04, 0.45], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 1.0, 0.0], size: [0.45, 0.04, 0.45], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 1.0, 0.0], size: [0.04, 0.04, 0.45], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 1.6, 0.0], size: [0.45, 0.04, 0.45], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 1.6, 0.0], size: [0.04, 0.04, 0.45], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [0.0, 2.3, 0.0], size: [0.05, 0.6, 0.05], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Sphere, pos: [0.0, 2.65, 0.0], size: [0.12, 0.12, 0.12], rot: [0.0, 0.0, 0.0], mat: Mat::LumiereRouge, id: Some("balise") },
    Piece { shape: Shape::Cone, pos: [0.0, 2.0, 0.3], size: [0.3, 0.12, 0.3], rot: [0.0, 0.0, -70.0], mat: Mat::Blanc, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.04, 0.0], size: [0.8, 0.08, 0.8], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
];
const ANIMS_12: &[Anim] = &[
    Anim { target: "balise", kind: AnimKind::Blink, amp: 0.0 },
];
pub const JURA_SIDES: [Model; 6] = [
    Model { c2: [47, 143, 78], c3: [122, 77, 36], pieces: PIECES_7, anims: ANIMS_7 },
    Model { c2: [255, 90, 31], c3: [138, 143, 153], pieces: PIECES_8, anims: ANIMS_8 },
    Model { c2: [47, 125, 58], c3: [242, 194, 48], pieces: PIECES_9, anims: ANIMS_9 },
    Model { c2: [242, 194, 48], c3: [122, 77, 36], pieces: PIECES_10, anims: ANIMS_10 },
    Model { c2: [111, 199, 232], c3: [138, 143, 153], pieces: PIECES_11, anims: ANIMS_11 },
    Model { c2: [232, 232, 232], c3: [192, 57, 43], pieces: PIECES_12, anims: ANIMS_12 },
];
const PIECES_13: &[Piece] = &[
    Piece { shape: Shape::Cyl, pos: [0.0, 0.1, 0.0], size: [0.03, 0.2, 0.03], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Sphere, pos: [0.07, 0.22, 0.0], size: [0.1, 0.04, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Sphere, pos: [0.022, 0.22, 0.067], size: [0.1, 0.04, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Sphere, pos: [-0.057, 0.22, 0.041], size: [0.1, 0.04, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Sphere, pos: [-0.057, 0.22, -0.041], size: [0.1, 0.04, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Sphere, pos: [0.022, 0.22, -0.067], size: [0.1, 0.04, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Sphere, pos: [0.0, 0.24, 0.0], size: [0.05, 0.05, 0.05], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
];
const ANIMS_13: &[Anim] = &[
];
const PIECES_14: &[Piece] = &[
    Piece { shape: Shape::Cone, pos: [0.06, 0.1, 0.0], size: [0.05, 0.25, 0.05], rot: [-0.0, 0.0, -50.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cone, pos: [0.019, 0.1, 0.057], size: [0.05, 0.25, 0.05], rot: [-47.552825814757675, 0.0, -15.450849718747373], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cone, pos: [-0.049, 0.1, 0.035], size: [0.05, 0.25, 0.05], rot: [-29.38926261462366, 0.0, 40.450849718747364], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cone, pos: [-0.049, 0.1, -0.035], size: [0.05, 0.25, 0.05], rot: [29.38926261462365, 0.0, 40.45084971874738], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cone, pos: [0.019, 0.1, -0.057], size: [0.05, 0.25, 0.05], rot: [47.55282581475768, 0.0, -15.450849718747362], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cone, pos: [0.0, 0.12, 0.0], size: [0.05, 0.25, 0.05], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
];
const ANIMS_14: &[Anim] = &[
];
const PIECES_15: &[Piece] = &[
    Piece { shape: Shape::Cyl, pos: [0.0, 0.04, 0.0], size: [0.05, 0.5, 0.05], rot: [0.0, 0.0, 90.0], mat: Mat::Blanc, id: None },
    Piece { shape: Shape::Sphere, pos: [0.26, 0.05, 0.03], size: [0.09, 0.09, 0.09], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: None },
    Piece { shape: Shape::Sphere, pos: [0.26, 0.05, -0.03], size: [0.09, 0.09, 0.09], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: None },
    Piece { shape: Shape::Sphere, pos: [-0.26, 0.05, 0.03], size: [0.09, 0.09, 0.09], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: None },
    Piece { shape: Shape::Sphere, pos: [-0.26, 0.05, -0.03], size: [0.09, 0.09, 0.09], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: None },
];
const ANIMS_15: &[Anim] = &[
];
const PIECES_16: &[Piece] = &[
    Piece { shape: Shape::Cyl, pos: [0.0, 0.01, 0.0], size: [0.3, 0.02, 0.34], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cone, pos: [0.2, 0.02, -0.12], size: [0.06, 0.12, 0.06], rot: [0.0, 0.0, -90.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cone, pos: [0.2, 0.02, 0.0], size: [0.06, 0.12, 0.06], rot: [0.0, 0.0, -90.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cone, pos: [0.2, 0.02, 0.12], size: [0.06, 0.12, 0.06], rot: [0.0, 0.0, -90.0], mat: Mat::C2, id: None },
];
const ANIMS_16: &[Anim] = &[
];
const PIECES_17: &[Piece] = &[
    Piece { shape: Shape::Sphere, pos: [0.0, 0.15, 0.0], size: [0.22, 0.3, 0.22], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: None },
    Piece { shape: Shape::Sphere, pos: [0.08, 0.15, 0.04], size: [0.06, 0.06, 0.04], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Sphere, pos: [-0.05, 0.22, 0.08], size: [0.06, 0.06, 0.04], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Sphere, pos: [0.0, 0.08, -0.09], size: [0.06, 0.06, 0.04], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Sphere, pos: [0.0, 0.02, 0.0], size: [0.28, 0.06, 0.28], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
];
const ANIMS_17: &[Anim] = &[
];
const PIECES_18: &[Piece] = &[
    Piece { shape: Shape::Cyl, pos: [0.0, 0.25, 0.0], size: [0.04, 0.5, 0.04], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.55, 0.0], size: [0.04, 0.3, 0.3], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [0.03, 0.55, 0.0], size: [0.02, 0.2, 0.04], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: None },
    Piece { shape: Shape::Cube, pos: [0.03, 0.47, 0.0], size: [0.02, 0.04, 0.04], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: None },
];
const ANIMS_18: &[Anim] = &[
];
const PIECES_19: &[Piece] = &[
    Piece { shape: Shape::Cyl, pos: [0.0, 0.08, 0.0], size: [0.2, 0.16, 0.2], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cyl, pos: [0.0, 0.2, 0.0], size: [0.16, 0.12, 0.16], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Sphere, pos: [0.1, 0.22, 0.0], size: [0.1, 0.1, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::Phare, id: Some("lampe") },
];
const ANIMS_19: &[Anim] = &[
    Anim { target: "lampe", kind: AnimKind::Pulse, amp: 0.0 },
];
const PIECES_20: &[Piece] = &[
    Piece { shape: Shape::Cube, pos: [0.0, 0.15, 0.0], size: [0.35, 0.3, 0.35], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.31, 0.0], size: [0.37, 0.03, 0.37], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cube, pos: [0.18, 0.15, 0.0], size: [0.01, 0.1, 0.2], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
];
const ANIMS_20: &[Anim] = &[
];
const PIECES_21: &[Piece] = &[
    Piece { shape: Shape::Sphere, pos: [0.0, 0.12, 0.0], size: [0.18, 0.24, 0.18], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Sphere, pos: [0.0, 0.12, 0.0], size: [0.03, 0.03, 0.03], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: None },
    Piece { shape: Shape::Cyl, pos: [0.0, 0.02, 0.0], size: [0.1, 0.04, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
];
const ANIMS_21: &[Anim] = &[
];
const PIECES_22: &[Piece] = &[
    Piece { shape: Shape::Cone, pos: [0.0, 0.15, 0.0], size: [0.25, 0.5, 0.1], rot: [0.0, 0.0, -70.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cone, pos: [0.0, 0.15, 0.0], size: [0.25, 0.5, 0.1], rot: [0.0, 0.0, 70.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cyl, pos: [0.0, 0.06, 0.0], size: [0.03, 0.12, 0.03], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
];
const ANIMS_22: &[Anim] = &[
];
pub const JURA_DECO: [Model; 10] = [
    Model { c2: [232, 51, 79], c3: [255, 210, 63], pieces: PIECES_13, anims: ANIMS_13 },
    Model { c2: [47, 143, 78], c3: [136, 136, 136], pieces: PIECES_14, anims: ANIMS_14 },
    Model { c2: [244, 239, 224], c3: [136, 136, 136], pieces: PIECES_15, anims: ANIMS_15 },
    Model { c2: [42, 58, 42], c3: [136, 136, 136], pieces: PIECES_16, anims: ANIMS_16 },
    Model { c2: [244, 239, 224], c3: [63, 122, 70], pieces: PIECES_17, anims: ANIMS_17 },
    Model { c2: [255, 210, 63], c3: [136, 136, 136], pieces: PIECES_18, anims: ANIMS_18 },
    Model { c2: [242, 194, 48], c3: [136, 136, 136], pieces: PIECES_19, anims: ANIMS_19 },
    Model { c2: [47, 125, 58], c3: [242, 194, 48], pieces: PIECES_20, anims: ANIMS_20 },
    Model { c2: [255, 176, 46], c3: [136, 136, 136], pieces: PIECES_21, anims: ANIMS_21 },
    Model { c2: [47, 143, 78], c3: [136, 136, 136], pieces: PIECES_22, anims: ANIMS_22 },
];
pub const JURA_COLORS: WorldColors = WorldColors {
    grass_a: [47, 107, 79],
    grass_b: [58, 122, 90],
    water: [47, 159, 191],
    road: [52, 54, 59],
    marking: [242, 194, 48],
    log: [138, 90, 43],
    sky: [143, 211, 232],
};
pub fn jura_name() -> &'static str { tx!("Parc Jurassic'Croa", "Jura-Croak Park", "Jura-Quak-Park", "Parco Giura-Gra", "Parque Jura-Croa").t() }
pub fn jura_banner() -> &'static str { tx!("Bienvenue au Parc Jurassic'Croa ! Merci de ne pas nourrir les dinosaures, ni de passer devant.", "Welcome to Jura-Croak Park! Please do not feed the dinosaurs, or walk in front of them.", "Willkommen im Jura-Quak-Park! Bitte die Dinosaurier nicht füttern und nicht vor ihnen herlaufen.", "Benvenuto al Parco Giura-Gra! Non dare da mangiare ai dinosauri e non passare davanti a loro.", "¡Bienvenido al Parque Jura-Croa! Por favor, no alimentes a los dinosaurios ni cruces delante de ellos.").t() }
pub const JURA_POINTS: u32 = 1500;
/// Numéros (dans `gen_vehicles::GEN`) des véhicules qui roulent dans ce monde.
pub const JURA_VEHICLES: [usize; 4] = [32, 33, 34, 35];

pub struct WorldColors {
    pub grass_a: [u8; 3],
    pub grass_b: [u8; 3],
    pub water: [u8; 3],
    pub road: [u8; 3],
    pub marking: [u8; 3],
    pub log: [u8; 3],
    pub sky: [u8; 3],
}

