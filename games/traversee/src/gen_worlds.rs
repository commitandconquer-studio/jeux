// Généré par tools/gen-mondes.py à partir de data/import/*.json : ne pas modifier à la main.
//! Mondes à thème décrits en pièces simples (cube, cylindre, cône, sphère).
#![allow(dead_code)]
use game_common::tx;

use crate::extra::{Anim, AnimKind, Animal, Mat, Model, Piece, Shape, Theme, TrainCar, WorldColors};

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
const JURA_BLOCKS: [Model; 7] = [
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
const JURA_SIDES: [Model; 6] = [
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
const JURA_DECO: [Model; 10] = [
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
const PIECES_23: &[Piece] = &[
    Piece { shape: Shape::Cube, pos: [0.049, 0.3, 0.0], size: [0.322, 0.2, 0.28], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Cube, pos: [0.049, 0.24, 0.0], size: [0.29, 0.072, 0.288], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [0.182, 0.44, 0.0], size: [0.063, 0.136, 0.112], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [0.28, 0.48, 0.0], size: [0.14, 0.14, 0.16], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Sphere, pos: [0.308, 0.508, 0.08], size: [0.053, 0.053, 0.053], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: None },
    Piece { shape: Shape::Sphere, pos: [0.32, 0.508, 0.096], size: [0.027, 0.027, 0.027], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: None },
    Piece { shape: Shape::Sphere, pos: [0.308, 0.508, -0.08], size: [0.053, 0.053, 0.053], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: None },
    Piece { shape: Shape::Sphere, pos: [0.32, 0.508, -0.096], size: [0.027, 0.027, 0.027], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: None },
    Piece { shape: Shape::Cyl, pos: [0.075, 0.115, 0.078], size: [0.06, 0.17, 0.06], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("patte_g") },
    Piece { shape: Shape::Cube, pos: [0.105, 0.015, 0.078], size: [0.12, 0.03, 0.07], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: Some("patte_g") },
    Piece { shape: Shape::Cyl, pos: [0.075, 0.115, -0.078], size: [0.06, 0.17, 0.06], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("patte_d") },
    Piece { shape: Shape::Cube, pos: [0.105, 0.015, -0.078], size: [0.12, 0.03, 0.07], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: Some("patte_d") },
    Piece { shape: Shape::Cone, pos: [-0.231, 0.33, 0.0], size: [0.14, 0.238, 0.14], rot: [0.0, 0.0, 90.0], mat: Mat::C2, id: Some("queue") },
    Piece { shape: Shape::Cube, pos: [0.029, 0.41, 0.06], size: [0.18, 0.02, 0.04], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [0.149, 0.32, 0.15], size: [0.08, 0.03, 0.03], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("bras") },
    Piece { shape: Shape::Cube, pos: [0.029, 0.41, -0.06], size: [0.18, 0.02, 0.04], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [0.149, 0.32, -0.15], size: [0.08, 0.03, 0.03], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("bras") },
];
const ANIMS_23: &[Anim] = &[
    Anim { target: "corps", kind: AnimKind::Bob, amp: 0.02 },
    Anim { target: "patte_g", kind: AnimKind::SwingZ, amp: 0.0 },
    Anim { target: "patte_d", kind: AnimKind::SwingZ, amp: 0.5 },
    Anim { target: "queue", kind: AnimKind::TailY, amp: 0.0 },
];
const PIECES_24: &[Piece] = &[
    Piece { shape: Shape::Cube, pos: [-0.018, 0.26, 0.0], size: [0.504, 0.26, 0.45], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Cube, pos: [-0.018, 0.174, 0.0], size: [0.454, 0.088, 0.464], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [0.234, 0.333, 0.0], size: [0.13, 0.156, 0.221], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [0.342, 0.338, 0.0], size: [0.216, 0.2, 0.26], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Sphere, pos: [0.385, 0.378, 0.13], size: [0.076, 0.076, 0.076], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: None },
    Piece { shape: Shape::Sphere, pos: [0.402, 0.378, 0.153], size: [0.038, 0.038, 0.038], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: None },
    Piece { shape: Shape::Sphere, pos: [0.385, 0.378, -0.13], size: [0.076, 0.076, 0.076], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: None },
    Piece { shape: Shape::Sphere, pos: [0.402, 0.378, -0.153], size: [0.038, 0.038, 0.038], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: None },
    Piece { shape: Shape::Cyl, pos: [0.143, 0.065, 0.18], size: [0.09, 0.13, 0.09], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("patte_av_g") },
    Piece { shape: Shape::Cyl, pos: [0.143, 0.065, -0.18], size: [0.09, 0.13, 0.09], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("patte_av_d") },
    Piece { shape: Shape::Cyl, pos: [-0.179, 0.065, 0.18], size: [0.09, 0.13, 0.09], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("patte_ar_g") },
    Piece { shape: Shape::Cyl, pos: [-0.179, 0.065, -0.18], size: [0.09, 0.13, 0.09], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("patte_ar_d") },
    Piece { shape: Shape::Cone, pos: [-0.36, 0.312, 0.0], size: [0.225, 0.18, 0.225], rot: [0.0, 0.0, 90.0], mat: Mat::C2, id: Some("queue") },
    Piece { shape: Shape::Cube, pos: [0.212, 0.398, 0.0], size: [0.04, 0.3, 0.44], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("collerette") },
    Piece { shape: Shape::Cone, pos: [0.362, 0.478, 0.07], size: [0.04, 0.09, 0.04], rot: [0.0, 0.0, -20.0], mat: Mat::Blanc, id: None },
    Piece { shape: Shape::Cone, pos: [0.362, 0.478, -0.07], size: [0.04, 0.09, 0.04], rot: [0.0, 0.0, -20.0], mat: Mat::Blanc, id: None },
    Piece { shape: Shape::Cone, pos: [0.45, 0.338, 0.0], size: [0.04, 0.07, 0.04], rot: [0.0, 0.0, -50.0], mat: Mat::Blanc, id: None },
    Piece { shape: Shape::Cube, pos: [0.43, 0.268, 0.0], size: [0.05, 0.04, 0.12], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
];
const ANIMS_24: &[Anim] = &[
    Anim { target: "corps", kind: AnimKind::Bob, amp: 0.02 },
    Anim { target: "patte_av_g", kind: AnimKind::SwingZ, amp: 0.0 },
    Anim { target: "patte_av_d", kind: AnimKind::SwingZ, amp: 0.5 },
    Anim { target: "patte_ar_g", kind: AnimKind::SwingZ, amp: 0.5 },
    Anim { target: "patte_ar_d", kind: AnimKind::SwingZ, amp: 0.0 },
    Anim { target: "queue", kind: AnimKind::TailY, amp: 0.0 },
    Anim { target: "collerette", kind: AnimKind::Pulse, amp: 0.0 },
];
const PIECES_25: &[Piece] = &[
    Piece { shape: Shape::Cube, pos: [0.238, 1.05, 0.0], size: [0.748, 0.4, 0.5], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Cube, pos: [0.238, 0.93, 0.0], size: [0.673, 0.144, 0.515], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [0.544, 1.525, 0.0], size: [0.107, 0.606, 0.14], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [0.731, 1.8, 0.0], size: [0.238, 0.14, 0.2], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Sphere, pos: [0.779, 1.828, 0.1], size: [0.053, 0.053, 0.053], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: None },
    Piece { shape: Shape::Sphere, pos: [0.79, 1.828, 0.116], size: [0.027, 0.027, 0.027], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: None },
    Piece { shape: Shape::Sphere, pos: [0.779, 1.828, -0.1], size: [0.053, 0.053, 0.053], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: None },
    Piece { shape: Shape::Sphere, pos: [0.79, 1.828, -0.116], size: [0.027, 0.027, 0.027], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: None },
    Piece { shape: Shape::Cyl, pos: [0.298, 0.449, 0.14], size: [0.096, 0.802, 0.096], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("patte_g") },
    Piece { shape: Shape::Cube, pos: [0.346, 0.024, 0.14], size: [0.192, 0.048, 0.112], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: Some("patte_g") },
    Piece { shape: Shape::Cyl, pos: [0.298, 0.449, -0.14], size: [0.096, 0.802, 0.096], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("patte_d") },
    Piece { shape: Shape::Cube, pos: [0.346, 0.024, -0.14], size: [0.192, 0.048, 0.112], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: Some("patte_d") },
    Piece { shape: Shape::Cone, pos: [-0.493, 1.11, 0.0], size: [0.25, 0.714, 0.25], rot: [0.0, 0.0, 90.0], mat: Mat::C2, id: Some("queue") },
    Piece { shape: Shape::Cone, pos: [-0.9, 1.11, -0.1], size: [0.06, 0.25, 0.06], rot: [0.0, 0.0, 90.0], mat: Mat::C3, id: Some("queue") },
    Piece { shape: Shape::Cone, pos: [-0.9, 1.16, 0.0], size: [0.06, 0.25, 0.06], rot: [0.0, 0.0, 90.0], mat: Mat::C3, id: Some("queue") },
    Piece { shape: Shape::Cone, pos: [-0.9, 1.06, 0.1], size: [0.06, 0.25, 0.06], rot: [0.0, 0.0, 90.0], mat: Mat::C3, id: Some("queue") },
    Piece { shape: Shape::Cube, pos: [0.238, 1.27, 0.0], size: [0.648, 0.02, 0.2], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cone, pos: [0.85, 1.78, 0.0], size: [0.06, 0.1, 0.06], rot: [0.0, 0.0, -90.0], mat: Mat::C3, id: None },
];
const ANIMS_25: &[Anim] = &[
    Anim { target: "corps", kind: AnimKind::Bob, amp: 0.02 },
    Anim { target: "patte_g", kind: AnimKind::SwingZ, amp: 0.0 },
    Anim { target: "patte_d", kind: AnimKind::SwingZ, amp: 0.5 },
    Anim { target: "queue", kind: AnimKind::TailY, amp: 0.0 },
];
const PIECES_26: &[Piece] = &[
    Piece { shape: Shape::Cube, pos: [0.15, 0.67, 0.0], size: [0.6, 0.34, 0.4], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Cube, pos: [0.15, 0.568, 0.0], size: [0.54, 0.122, 0.412], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [0.39, 0.9, 0.0], size: [0.135, 0.2, 0.154], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [0.6, 0.96, 0.0], size: [0.3, 0.2, 0.22], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Sphere, pos: [0.66, 1.0, 0.11], size: [0.076, 0.076, 0.076], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: None },
    Piece { shape: Shape::Sphere, pos: [0.677, 1.0, 0.133], size: [0.038, 0.038, 0.038], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: None },
    Piece { shape: Shape::Sphere, pos: [0.66, 1.0, -0.11], size: [0.076, 0.076, 0.076], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: None },
    Piece { shape: Shape::Sphere, pos: [0.677, 1.0, -0.133], size: [0.038, 0.038, 0.038], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: None },
    Piece { shape: Shape::Cyl, pos: [0.198, 0.271, 0.112], size: [0.084, 0.458, 0.084], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("patte_g") },
    Piece { shape: Shape::Cube, pos: [0.24, 0.021, 0.112], size: [0.168, 0.042, 0.098], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: Some("patte_g") },
    Piece { shape: Shape::Cyl, pos: [0.198, 0.271, -0.112], size: [0.084, 0.458, 0.084], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("patte_d") },
    Piece { shape: Shape::Cube, pos: [0.24, 0.021, -0.112], size: [0.168, 0.042, 0.098], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: Some("patte_d") },
    Piece { shape: Shape::Cone, pos: [-0.45, 0.721, 0.0], size: [0.2, 0.6, 0.2], rot: [0.0, 0.0, 90.0], mat: Mat::C2, id: Some("queue") },
    Piece { shape: Shape::Cube, pos: [0.1, 0.852, 0.1], size: [0.5, 0.02, 0.07], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cone, pos: [0.35, 0.1, 0.11], size: [0.04, 0.1, 0.04], rot: [0.0, 0.0, -90.0], mat: Mat::Blanc, id: Some("patte_g") },
    Piece { shape: Shape::Cube, pos: [0.4, 0.7, 0.22], size: [0.14, 0.04, 0.04], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cone, pos: [0.49, 0.7, 0.22], size: [0.03, 0.05, 0.03], rot: [0.0, 0.0, -90.0], mat: Mat::Blanc, id: None },
    Piece { shape: Shape::Cube, pos: [0.1, 0.852, -0.1], size: [0.5, 0.02, 0.07], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cone, pos: [0.35, 0.1, -0.11], size: [0.04, 0.1, 0.04], rot: [0.0, 0.0, -90.0], mat: Mat::Blanc, id: Some("patte_d") },
    Piece { shape: Shape::Cube, pos: [0.4, 0.7, -0.22], size: [0.14, 0.04, 0.04], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cone, pos: [0.49, 0.7, -0.22], size: [0.03, 0.05, 0.03], rot: [0.0, 0.0, -90.0], mat: Mat::Blanc, id: None },
    Piece { shape: Shape::Cube, pos: [0.735, 0.89, 0.0], size: [0.04, 0.03, 0.16], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: None },
];
const ANIMS_26: &[Anim] = &[
    Anim { target: "corps", kind: AnimKind::Bob, amp: 0.02 },
    Anim { target: "patte_g", kind: AnimKind::SwingZ, amp: 0.0 },
    Anim { target: "patte_d", kind: AnimKind::SwingZ, amp: 0.5 },
    Anim { target: "queue", kind: AnimKind::TailY, amp: 0.0 },
];
const PIECES_27: &[Piece] = &[
    Piece { shape: Shape::Cube, pos: [0.15, 0.73, 0.0], size: [0.6, 0.36, 0.5], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Cube, pos: [0.15, 0.622, 0.0], size: [0.54, 0.13, 0.515], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [0.39, 1.01, 0.0], size: [0.135, 0.28, 0.154], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [0.6, 1.11, 0.0], size: [0.3, 0.2, 0.22], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Sphere, pos: [0.66, 1.15, 0.11], size: [0.076, 0.076, 0.076], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: None },
    Piece { shape: Shape::Sphere, pos: [0.677, 1.15, 0.133], size: [0.038, 0.038, 0.038], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: None },
    Piece { shape: Shape::Sphere, pos: [0.66, 1.15, -0.11], size: [0.076, 0.076, 0.076], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: None },
    Piece { shape: Shape::Sphere, pos: [0.677, 1.15, -0.133], size: [0.038, 0.038, 0.038], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: None },
    Piece { shape: Shape::Cyl, pos: [0.198, 0.299, 0.14], size: [0.096, 0.502, 0.096], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("patte_g") },
    Piece { shape: Shape::Cube, pos: [0.246, 0.024, 0.14], size: [0.192, 0.048, 0.112], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: Some("patte_g") },
    Piece { shape: Shape::Cyl, pos: [0.198, 0.299, -0.14], size: [0.096, 0.502, 0.096], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("patte_d") },
    Piece { shape: Shape::Cube, pos: [0.246, 0.024, -0.14], size: [0.192, 0.048, 0.112], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: Some("patte_d") },
    Piece { shape: Shape::Cone, pos: [-0.45, 0.784, 0.0], size: [0.25, 0.6, 0.25], rot: [0.0, 0.0, 90.0], mat: Mat::C2, id: Some("queue") },
    Piece { shape: Shape::Cone, pos: [0.48, 1.19, -0.2], size: [0.07, 0.2, 0.03], rot: [0.0, 0.0, 10.0], mat: Mat::C2, id: Some("collerette") },
    Piece { shape: Shape::Cone, pos: [0.48, 1.19, -0.1], size: [0.07, 0.25, 0.03], rot: [0.0, 0.0, 10.0], mat: Mat::C2, id: Some("collerette") },
    Piece { shape: Shape::Cone, pos: [0.48, 1.19, 0.0], size: [0.07, 0.3, 0.03], rot: [0.0, 0.0, 10.0], mat: Mat::C2, id: Some("collerette") },
    Piece { shape: Shape::Cone, pos: [0.48, 1.19, 0.1], size: [0.07, 0.25, 0.03], rot: [0.0, 0.0, 10.0], mat: Mat::C2, id: Some("collerette") },
    Piece { shape: Shape::Cone, pos: [0.48, 1.19, 0.2], size: [0.07, 0.2, 0.03], rot: [0.0, 0.0, 10.0], mat: Mat::C2, id: Some("collerette") },
    Piece { shape: Shape::Cone, pos: [0.6, 1.26, 0.05], size: [0.04, 0.12, 0.04], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [0.4, 0.77, 0.24], size: [0.14, 0.04, 0.04], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cone, pos: [0.6, 1.26, -0.05], size: [0.04, 0.12, 0.04], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [0.4, 0.77, -0.24], size: [0.14, 0.04, 0.04], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Sphere, pos: [0.77, 1.07, 0.0], size: [0.06, 0.06, 0.06], rot: [0.0, 0.0, 0.0], mat: Mat::LumiereBleue, id: Some("crachat") },
];
const ANIMS_27: &[Anim] = &[
    Anim { target: "corps", kind: AnimKind::Bob, amp: 0.02 },
    Anim { target: "patte_g", kind: AnimKind::SwingZ, amp: 0.0 },
    Anim { target: "patte_d", kind: AnimKind::SwingZ, amp: 0.5 },
    Anim { target: "queue", kind: AnimKind::TailY, amp: 0.0 },
    Anim { target: "collerette", kind: AnimKind::Pulse, amp: 0.0 },
    Anim { target: "crachat", kind: AnimKind::Blink, amp: 0.0 },
];
const PIECES_28: &[Piece] = &[
    Piece { shape: Shape::Cube, pos: [0.072, 0.47, 0.0], size: [1.488, 0.5, 1.3], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Cube, pos: [0.072, 0.305, 0.0], size: [1.339, 0.17, 1.339], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [0.816, 0.61, 0.0], size: [0.23, 0.3, 0.51], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [1.008, 0.62, 0.0], size: [0.384, 0.3, 0.6], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Sphere, pos: [1.085, 0.68, 0.3], size: [0.114, 0.114, 0.114], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: None },
    Piece { shape: Shape::Sphere, pos: [1.11, 0.68, 0.334], size: [0.057, 0.057, 0.057], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: None },
    Piece { shape: Shape::Sphere, pos: [1.085, 0.68, -0.3], size: [0.114, 0.114, 0.114], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: None },
    Piece { shape: Shape::Sphere, pos: [1.11, 0.68, -0.334], size: [0.057, 0.057, 0.057], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: None },
    Piece { shape: Shape::Cyl, pos: [0.548, 0.11, 0.55], size: [0.2, 0.22, 0.2], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("patte_av_g") },
    Piece { shape: Shape::Cyl, pos: [0.548, 0.11, -0.55], size: [0.2, 0.22, 0.2], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("patte_av_d") },
    Piece { shape: Shape::Cyl, pos: [-0.404, 0.11, 0.55], size: [0.2, 0.22, 0.2], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("patte_ar_g") },
    Piece { shape: Shape::Cyl, pos: [-0.404, 0.11, -0.55], size: [0.2, 0.22, 0.2], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("patte_ar_d") },
    Piece { shape: Shape::Cone, pos: [-0.936, 0.57, 0.0], size: [0.65, 0.528, 0.65], rot: [0.0, 0.0, 90.0], mat: Mat::C2, id: Some("queue") },
    Piece { shape: Shape::Cone, pos: [-0.428, 0.78, 0.4], size: [0.12, 0.18, 0.12], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cone, pos: [-0.428, 0.78, -0.4], size: [0.12, 0.18, 0.12], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cone, pos: [-0.098, 0.78, 0.4], size: [0.12, 0.18, 0.12], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cone, pos: [-0.098, 0.78, -0.4], size: [0.12, 0.18, 0.12], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cone, pos: [0.232, 0.78, 0.4], size: [0.12, 0.18, 0.12], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cone, pos: [0.232, 0.78, -0.4], size: [0.12, 0.18, 0.12], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cone, pos: [0.562, 0.78, 0.4], size: [0.12, 0.18, 0.12], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cone, pos: [0.562, 0.78, -0.4], size: [0.12, 0.18, 0.12], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [0.072, 0.735, 0.0], size: [1.288, 0.03, 0.55], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Sphere, pos: [-1.15, 0.57, 0.0], size: [0.4, 0.3, 0.4], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("queue") },
    Piece { shape: Shape::Cone, pos: [0.958, 0.8, 0.28], size: [0.07, 0.14, 0.07], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cone, pos: [0.958, 0.8, -0.28], size: [0.07, 0.14, 0.07], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
];
const ANIMS_28: &[Anim] = &[
    Anim { target: "corps", kind: AnimKind::Bob, amp: 0.02 },
    Anim { target: "patte_av_g", kind: AnimKind::SwingZ, amp: 0.0 },
    Anim { target: "patte_av_d", kind: AnimKind::SwingZ, amp: 0.5 },
    Anim { target: "patte_ar_g", kind: AnimKind::SwingZ, amp: 0.5 },
    Anim { target: "patte_ar_d", kind: AnimKind::SwingZ, amp: 0.0 },
    Anim { target: "queue", kind: AnimKind::TailY, amp: 0.0 },
];
const PIECES_29: &[Piece] = &[
    Piece { shape: Shape::Cube, pos: [0.224, 0.7, 0.0], size: [1.568, 0.7, 0.8], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Cube, pos: [0.224, 0.469, 0.0], size: [1.411, 0.238, 0.824], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [1.008, 0.896, 0.0], size: [0.235, 0.42, 0.289], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [1.204, 0.91, 0.0], size: [0.392, 0.28, 0.34], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Sphere, pos: [1.282, 0.966, 0.17], size: [0.106, 0.106, 0.106], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: None },
    Piece { shape: Shape::Sphere, pos: [1.306, 0.966, 0.202], size: [0.053, 0.053, 0.053], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: None },
    Piece { shape: Shape::Sphere, pos: [1.282, 0.966, -0.17], size: [0.106, 0.106, 0.106], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: None },
    Piece { shape: Shape::Sphere, pos: [1.306, 0.966, -0.202], size: [0.053, 0.053, 0.053], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: None },
    Piece { shape: Shape::Cyl, pos: [0.726, 0.175, 0.3], size: [0.2, 0.35, 0.2], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("patte_av_g") },
    Piece { shape: Shape::Cyl, pos: [0.726, 0.175, -0.3], size: [0.2, 0.35, 0.2], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("patte_av_d") },
    Piece { shape: Shape::Cyl, pos: [-0.278, 0.175, 0.3], size: [0.2, 0.35, 0.2], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("patte_ar_g") },
    Piece { shape: Shape::Cyl, pos: [-0.278, 0.175, -0.3], size: [0.2, 0.35, 0.2], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("patte_ar_d") },
    Piece { shape: Shape::Cone, pos: [-0.98, 0.84, 0.0], size: [0.4, 0.84, 0.4], rot: [0.0, 0.0, 90.0], mat: Mat::C2, id: Some("queue") },
    Piece { shape: Shape::Cube, pos: [-0.226, 1.176, 0.0], size: [0.28, 0.28, 0.06], rot: [0.0, 0.0, 45.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [0.024, 1.221, 0.0], size: [0.38, 0.38, 0.06], rot: [0.0, 0.0, 45.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [0.274, 1.248, 0.0], size: [0.44, 0.44, 0.06], rot: [0.0, 0.0, 45.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [0.524, 1.221, 0.0], size: [0.38, 0.38, 0.06], rot: [0.0, 0.0, 45.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [0.774, 1.185, 0.0], size: [0.3, 0.3, 0.06], rot: [0.0, 0.0, 45.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cone, pos: [-1.3, 0.7, -0.12], size: [0.07, 0.3, 0.07], rot: [0.0, 0.0, 100.0], mat: Mat::Noir, id: Some("queue") },
    Piece { shape: Shape::Cone, pos: [-1.15, 0.75, -0.12], size: [0.06, 0.22, 0.06], rot: [0.0, 0.0, 110.0], mat: Mat::Noir, id: Some("queue") },
    Piece { shape: Shape::Cone, pos: [-1.3, 0.7, 0.12], size: [0.07, 0.3, 0.07], rot: [0.0, 0.0, 100.0], mat: Mat::Noir, id: Some("queue") },
    Piece { shape: Shape::Cone, pos: [-1.15, 0.75, 0.12], size: [0.06, 0.22, 0.06], rot: [0.0, 0.0, 110.0], mat: Mat::Noir, id: Some("queue") },
];
const ANIMS_29: &[Anim] = &[
    Anim { target: "corps", kind: AnimKind::Bob, amp: 0.02 },
    Anim { target: "patte_av_g", kind: AnimKind::SwingZ, amp: 0.0 },
    Anim { target: "patte_av_d", kind: AnimKind::SwingZ, amp: 0.5 },
    Anim { target: "patte_ar_g", kind: AnimKind::SwingZ, amp: 0.5 },
    Anim { target: "patte_ar_d", kind: AnimKind::SwingZ, amp: 0.0 },
    Anim { target: "queue", kind: AnimKind::TailY, amp: 0.0 },
];
const PIECES_30: &[Piece] = &[
    Piece { shape: Shape::Cube, pos: [-0.13, 0.8, 0.0], size: [1.508, 0.8, 1.0], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Cube, pos: [-0.13, 0.536, 0.0], size: [1.357, 0.272, 1.03], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [0.624, 1.024, 0.0], size: [0.406, 0.48, 0.51], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [0.962, 1.04, 0.0], size: [0.676, 0.6, 0.6], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Sphere, pos: [1.097, 1.16, 0.3], size: [0.14, 0.14, 0.14], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: None },
    Piece { shape: Shape::Sphere, pos: [1.128, 1.16, 0.342], size: [0.07, 0.07, 0.07], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: None },
    Piece { shape: Shape::Sphere, pos: [1.097, 1.16, -0.3], size: [0.14, 0.14, 0.14], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: None },
    Piece { shape: Shape::Sphere, pos: [1.128, 1.16, -0.342], size: [0.07, 0.07, 0.07], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: None },
    Piece { shape: Shape::Cyl, pos: [0.353, 0.2, 0.38], size: [0.24, 0.4, 0.24], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("patte_av_g") },
    Piece { shape: Shape::Cyl, pos: [0.353, 0.2, -0.38], size: [0.24, 0.4, 0.24], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("patte_av_d") },
    Piece { shape: Shape::Cyl, pos: [-0.613, 0.2, 0.38], size: [0.24, 0.4, 0.24], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("patte_ar_g") },
    Piece { shape: Shape::Cyl, pos: [-0.613, 0.2, -0.38], size: [0.24, 0.4, 0.24], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("patte_ar_d") },
    Piece { shape: Shape::Cone, pos: [-1.092, 0.96, 0.0], size: [0.5, 0.416, 0.5], rot: [0.0, 0.0, 90.0], mat: Mat::C2, id: Some("queue") },
    Piece { shape: Shape::Cube, pos: [0.658, 1.39, 0.0], size: [0.1, 0.8, 0.95], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("collerette") },
    Piece { shape: Shape::Cone, pos: [1.012, 1.44, 0.2], size: [0.08, 0.4, 0.08], rot: [0.0, 0.0, -25.0], mat: Mat::Blanc, id: None },
    Piece { shape: Shape::Cone, pos: [1.012, 1.44, -0.2], size: [0.08, 0.4, 0.08], rot: [0.0, 0.0, -25.0], mat: Mat::Blanc, id: None },
    Piece { shape: Shape::Cone, pos: [1.3, 1.14, 0.0], size: [0.09, 0.2, 0.09], rot: [0.0, 0.0, -55.0], mat: Mat::Blanc, id: None },
    Piece { shape: Shape::Cube, pos: [1.27, 0.86, 0.0], size: [0.14, 0.1, 0.3], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
];
const ANIMS_30: &[Anim] = &[
    Anim { target: "corps", kind: AnimKind::Bob, amp: 0.02 },
    Anim { target: "patte_av_g", kind: AnimKind::SwingZ, amp: 0.0 },
    Anim { target: "patte_av_d", kind: AnimKind::SwingZ, amp: 0.5 },
    Anim { target: "patte_ar_g", kind: AnimKind::SwingZ, amp: 0.5 },
    Anim { target: "patte_ar_d", kind: AnimKind::SwingZ, amp: 0.0 },
    Anim { target: "queue", kind: AnimKind::TailY, amp: 0.0 },
    Anim { target: "collerette", kind: AnimKind::Pulse, amp: 0.0 },
];
const PIECES_31: &[Piece] = &[
    Piece { shape: Shape::Cube, pos: [0.0, 0.7, 0.0], size: [0.7, 0.3, 0.4], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Cube, pos: [0.0, 0.6, 0.0], size: [0.55, 0.12, 0.36], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [0.5, 0.82, 0.0], size: [0.3, 0.2, 0.22], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cone, pos: [0.9, 0.82, 0.0], size: [0.1, 0.7, 0.1], rot: [0.0, 0.0, -90.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cone, pos: [0.3, 0.95, 0.0], size: [0.1, 0.6, 0.1], rot: [0.0, 0.0, 90.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Sphere, pos: [0.58, 0.87, 0.11], size: [0.09, 0.09, 0.09], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: None },
    Piece { shape: Shape::Sphere, pos: [0.62, 0.87, 0.14], size: [0.045, 0.045, 0.045], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.78, 0.75], size: [0.9, 0.04, 1.1], rot: [5.0, 0.0, 0.0], mat: Mat::C2, id: Some("aile_g") },
    Piece { shape: Shape::Cube, pos: [-0.2, 0.77, 1.0], size: [0.6, 0.03, 0.9], rot: [5.0, 0.0, 0.0], mat: Mat::C2, id: Some("aile_g") },
    Piece { shape: Shape::Cone, pos: [0.2, 0.78, 1.35], size: [0.12, 0.45, 0.1], rot: [-90.0, 0.0, 0.0], mat: Mat::C2, id: Some("aile_g") },
    Piece { shape: Shape::Cyl, pos: [-0.05, 0.3, 0.1], size: [0.05, 0.5, 0.05], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: Some("patte_g") },
    Piece { shape: Shape::Sphere, pos: [0.58, 0.87, -0.11], size: [0.09, 0.09, 0.09], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: None },
    Piece { shape: Shape::Sphere, pos: [0.62, 0.87, -0.14], size: [0.045, 0.045, 0.045], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.78, -0.75], size: [0.9, 0.04, 1.1], rot: [-5.0, 0.0, 0.0], mat: Mat::C2, id: Some("aile_d") },
    Piece { shape: Shape::Cube, pos: [-0.2, 0.77, -1.0], size: [0.6, 0.03, 0.9], rot: [-5.0, 0.0, 0.0], mat: Mat::C2, id: Some("aile_d") },
    Piece { shape: Shape::Cone, pos: [0.2, 0.78, -1.35], size: [0.12, 0.45, 0.1], rot: [90.0, 0.0, 0.0], mat: Mat::C2, id: Some("aile_d") },
    Piece { shape: Shape::Cyl, pos: [-0.05, 0.3, -0.1], size: [0.05, 0.5, 0.05], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: Some("patte_d") },
    Piece { shape: Shape::Cone, pos: [-0.55, 0.68, 0.0], size: [0.18, 0.4, 0.1], rot: [0.0, 0.0, 90.0], mat: Mat::C2, id: Some("queue") },
];
const ANIMS_31: &[Anim] = &[
    Anim { target: "corps", kind: AnimKind::Bob, amp: 0.04 },
    Anim { target: "aile_g", kind: AnimKind::SwingXBase, amp: 0.0 },
    Anim { target: "patte_g", kind: AnimKind::SwingZ, amp: 0.0 },
    Anim { target: "aile_d", kind: AnimKind::SwingXBase, amp: 0.0 },
    Anim { target: "patte_d", kind: AnimKind::SwingZ, amp: 0.5 },
    Anim { target: "queue", kind: AnimKind::TailY, amp: 0.0 },
];
const PIECES_32: &[Piece] = &[
    Piece { shape: Shape::Cube, pos: [0.368, 1.65, 0.0], size: [1.656, 1.1, 1.5], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Cube, pos: [0.368, 1.32, 0.0], size: [1.49, 0.396, 1.545], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [1.012, 2.29, 0.0], size: [0.497, 0.5, 0.595], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [1.748, 2.38, 0.0], size: [1.104, 0.8, 0.85], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Sphere, pos: [1.969, 2.54, 0.425], size: [0.14, 0.14, 0.14], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: None },
    Piece { shape: Shape::Sphere, pos: [2.0, 2.54, 0.467], size: [0.07, 0.07, 0.07], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: None },
    Piece { shape: Shape::Sphere, pos: [1.969, 2.54, -0.425], size: [0.14, 0.14, 0.14], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: None },
    Piece { shape: Shape::Sphere, pos: [2.0, 2.54, -0.467], size: [0.07, 0.07, 0.07], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: None },
    Piece { shape: Shape::Cyl, pos: [0.5, 0.64, 0.42], size: [0.36, 0.92, 0.36], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("patte_g") },
    Piece { shape: Shape::Cube, pos: [0.68, 0.09, 0.42], size: [0.72, 0.18, 0.42], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: Some("patte_g") },
    Piece { shape: Shape::Cyl, pos: [0.5, 0.64, -0.42], size: [0.36, 0.92, 0.36], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("patte_d") },
    Piece { shape: Shape::Cube, pos: [0.68, 0.09, -0.42], size: [0.72, 0.18, 0.42], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: Some("patte_d") },
    Piece { shape: Shape::Cone, pos: [-1.38, 1.815, 0.0], size: [0.75, 1.84, 0.75], rot: [0.0, 0.0, 90.0], mat: Mat::C2, id: Some("queue") },
    Piece { shape: Shape::Cube, pos: [1.698, 1.86, 0.0], size: [1.049, 0.22, 0.8], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: None },
    Piece { shape: Shape::Cone, pos: [2.2, 2.0, -0.28], size: [0.07, 0.14, 0.07], rot: [180.0, 0.0, 0.0], mat: Mat::Blanc, id: None },
    Piece { shape: Shape::Cone, pos: [2.2, 2.0, -0.1], size: [0.07, 0.14, 0.07], rot: [180.0, 0.0, 0.0], mat: Mat::Blanc, id: None },
    Piece { shape: Shape::Cone, pos: [2.2, 2.0, 0.1], size: [0.07, 0.14, 0.07], rot: [180.0, 0.0, 0.0], mat: Mat::Blanc, id: None },
    Piece { shape: Shape::Cone, pos: [2.2, 2.0, 0.28], size: [0.07, 0.14, 0.07], rot: [180.0, 0.0, 0.0], mat: Mat::Blanc, id: None },
    Piece { shape: Shape::Cube, pos: [2.28, 2.43, 0.0], size: [0.04, 0.05, 0.4], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: None },
    Piece { shape: Shape::Cube, pos: [1.096, 1.65, 0.5], size: [0.3, 0.08, 0.08], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cone, pos: [1.266, 1.65, 0.5], size: [0.05, 0.1, 0.05], rot: [0.0, 0.0, -90.0], mat: Mat::Blanc, id: None },
    Piece { shape: Shape::Cube, pos: [1.096, 1.65, -0.5], size: [0.3, 0.08, 0.08], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cone, pos: [1.266, 1.65, -0.5], size: [0.05, 0.1, 0.05], rot: [0.0, 0.0, -90.0], mat: Mat::Blanc, id: None },
    Piece { shape: Shape::Cube, pos: [0.368, 2.22, 0.0], size: [1.6, 0.04, 0.8], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [1.548, 2.8, 0.0], size: [0.7, 0.05, 0.5], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
];
const ANIMS_32: &[Anim] = &[
    Anim { target: "corps", kind: AnimKind::Bob, amp: 0.02 },
    Anim { target: "patte_g", kind: AnimKind::SwingZ, amp: 0.0 },
    Anim { target: "patte_d", kind: AnimKind::SwingZ, amp: 0.5 },
    Anim { target: "queue", kind: AnimKind::TailY, amp: 0.0 },
];
const PIECES_33: &[Piece] = &[
    Piece { shape: Shape::Sphere, pos: [0.0, 0.38, 0.0], size: [0.55, 0.5, 0.5], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [0.0, 0.28, 0.0], size: [0.4, 0.3, 0.46], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Sphere, pos: [0.28, 0.62, 0.0], size: [0.5, 0.42, 0.52], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [0.5, 0.5, 0.0], size: [0.1, 0.1, 0.34], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Sphere, pos: [0.4, 0.7, 0.2], size: [0.2, 0.2, 0.2], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: None },
    Piece { shape: Shape::Sphere, pos: [0.46, 0.7, 0.26], size: [0.1, 0.1, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: None },
    Piece { shape: Shape::Cube, pos: [0.14, 0.35, 0.27], size: [0.08, 0.04, 0.04], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cyl, pos: [0.0, 0.12, 0.14], size: [0.14, 0.14, 0.14], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("patte_g") },
    Piece { shape: Shape::Cube, pos: [0.06, 0.03, 0.14], size: [0.2, 0.06, 0.14], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: Some("patte_g") },
    Piece { shape: Shape::Sphere, pos: [0.4, 0.7, -0.2], size: [0.2, 0.2, 0.2], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: None },
    Piece { shape: Shape::Sphere, pos: [0.46, 0.7, -0.26], size: [0.1, 0.1, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: None },
    Piece { shape: Shape::Cube, pos: [0.14, 0.35, -0.27], size: [0.08, 0.04, 0.04], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cyl, pos: [0.0, 0.12, -0.14], size: [0.14, 0.14, 0.14], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("patte_d") },
    Piece { shape: Shape::Cube, pos: [0.06, 0.03, -0.14], size: [0.2, 0.06, 0.14], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: Some("patte_d") },
    Piece { shape: Shape::Cone, pos: [-0.38, 0.32, 0.0], size: [0.22, 0.4, 0.22], rot: [0.0, 0.0, 90.0], mat: Mat::C2, id: Some("queue") },
    Piece { shape: Shape::Cone, pos: [-0.05, 0.64, -0.12], size: [0.08, 0.12, 0.08], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cone, pos: [-0.05, 0.64, 0.0], size: [0.08, 0.12, 0.08], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cone, pos: [-0.05, 0.64, 0.12], size: [0.08, 0.12, 0.08], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Sphere, pos: [0.18, 0.4, 0.0], size: [0.14, 0.14, 0.04], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
];
const ANIMS_33: &[Anim] = &[
    Anim { target: "corps", kind: AnimKind::Bob, amp: 0.025 },
    Anim { target: "patte_g", kind: AnimKind::SwingZ, amp: 0.0 },
    Anim { target: "patte_d", kind: AnimKind::SwingZ, amp: 0.5 },
    Anim { target: "queue", kind: AnimKind::TailY, amp: 0.0 },
];
const JURA_ANIMALS: [Animal; 11] = [
    Animal { model: Model { c2: [242, 210, 122], c3: [63, 122, 70], pieces: PIECES_23, anims: ANIMS_23 }, len: 0.7, width: 0.3, points: 300, dangerous: false, flying: false },
    Animal { model: Model { c2: [242, 166, 90], c3: [255, 243, 214], pieces: PIECES_24, anims: ANIMS_24 }, len: 0.9, width: 0.5, points: 450, dangerous: false, flying: false },
    Animal { model: Model { c2: [244, 227, 176], c3: [193, 85, 58], pieces: PIECES_25, anims: ANIMS_25 }, len: 1.7, width: 0.5, points: 700, dangerous: true, flying: false },
    Animal { model: Model { c2: [232, 162, 60], c3: [42, 42, 42], pieces: PIECES_26, anims: ANIMS_26 }, len: 1.5, width: 0.4, points: 1000, dangerous: true, flying: false },
    Animal { model: Model { c2: [109, 74, 168], c3: [242, 194, 48], pieces: PIECES_27, anims: ANIMS_27 }, len: 1.5, width: 0.5, points: 1400, dangerous: true, flying: false },
    Animal { model: Model { c2: [122, 106, 88], c3: [207, 138, 46], pieces: PIECES_28, anims: ANIMS_28 }, len: 2.4, width: 1.3, points: 1800, dangerous: true, flying: false },
    Animal { model: Model { c2: [138, 176, 160], c3: [232, 104, 58], pieces: PIECES_29, anims: ANIMS_29 }, len: 2.8, width: 0.8, points: 2200, dangerous: true, flying: false },
    Animal { model: Model { c2: [154, 123, 90], c3: [217, 83, 79], pieces: PIECES_30, anims: ANIMS_30 }, len: 2.6, width: 1.0, points: 2700, dangerous: true, flying: false },
    Animal { model: Model { c2: [168, 118, 78], c3: [242, 194, 48], pieces: PIECES_31, anims: ANIMS_31 }, len: 1.9, width: 2.4, points: 3300, dangerous: true, flying: true },
    Animal { model: Model { c2: [90, 125, 58], c3: [232, 217, 160], pieces: PIECES_32, anims: ANIMS_32 }, len: 4.6, width: 1.5, points: 4200, dangerous: true, flying: false },
    Animal { model: Model { c2: [242, 166, 90], c3: [255, 255, 255], pieces: PIECES_33, anims: ANIMS_33 }, len: 0.9, width: 0.55, points: 5000, dangerous: false, flying: false },
];
fn jura_animal_name(i: usize) -> &'static str {
    match i {
        0 => tx!("Mini-Pique-Miettes", "Crumb Nibbler", "Krümelpicker", "Beccamiche", "Picamigas").t(),
        1 => tx!("Bébé Triple-Corne", "Baby Triple-Horn", "Dreihorn-Baby", "Cucciolo Tre-Corna", "Bebé Tres Cuernos").t(),
        2 => tx!("Autruche Folle", "Mad Ostrich-Saur", "Irrer Laufsaurier", "Struzzo Pazzo", "Avestruz Loca").t(),
        3 => tx!("Vélo-Rapide", "Quick-Claw", "Flinkkralle", "Artiglio Lesto", "Garra Veloz").t(),
        4 => tx!("Crache-Collerette", "Frill Spitter", "Kragenspucker", "Sputa-Gorgiera", "Escupe-Gorguera").t(),
        5 => tx!("Tank à Piques", "Spiky Tank", "Stachelpanzer", "Carro Spinoso", "Tanque Espinoso").t(),
        6 => tx!("Dos à Plaques", "Plate-Back", "Plattenrücken", "Dorso a Placche", "Lomo de Placas").t(),
        7 => tx!("Triple-Corne", "Three-Horn Bruiser", "Dreihorn-Rüpel", "Tre-Corna Bruto", "Tres Cuernos Bruto").t(),
        8 => tx!("Ptéro Pique-Vite", "Swoop-O-Dactyl", "Sturzflug-Ptero", "Pterodattilo Picchiatello", "Ptero Picado").t(),
        9 => tx!("Rex Pas Content", "Cranky Rex", "Grummel-Rex", "Rex Brontolone", "Rex Gruñón").t(),
        10 => tx!("Mini-Rex Mignon", "Tiny Rex", "Mini-Rex Putzig", "Mini-Rex Tenerino", "Mini-Rex Monín").t(),
        _ => "?",
    }
}
const PIECES_34: &[Piece] = &[
    Piece { shape: Shape::Sphere, pos: [-0.05, 0.26, 0.0], size: [0.5, 0.34, 0.46], rot: [0.0, 0.0, 0.0], mat: Mat::Peinture, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [0.0, 0.17, 0.0], size: [0.38, 0.2, 0.4], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Sphere, pos: [0.2, 0.36, 0.0], size: [0.36, 0.26, 0.4], rot: [0.0, 0.0, 0.0], mat: Mat::Peinture, id: None },
    Piece { shape: Shape::Cube, pos: [0.37, 0.31, 0.0], size: [0.02, 0.02, 0.3], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: None },
    Piece { shape: Shape::Sphere, pos: [0.2, 0.54, 0.14], size: [0.15, 0.15, 0.15], rot: [0.0, 0.0, 0.0], mat: Mat::Peinture, id: None },
    Piece { shape: Shape::Sphere, pos: [0.25, 0.56, 0.14], size: [0.11, 0.11, 0.11], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: None },
    Piece { shape: Shape::Sphere, pos: [0.3, 0.57, 0.14], size: [0.06, 0.06, 0.06], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: None },
    Piece { shape: Shape::Sphere, pos: [-0.2, 0.2, 0.26], size: [0.22, 0.2, 0.14], rot: [0.0, 0.0, 0.0], mat: Mat::Peinture, id: None },
    Piece { shape: Shape::Cube, pos: [-0.1, 0.07, 0.27], size: [0.14, 0.1, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::Peinture, id: None },
    Piece { shape: Shape::Cube, pos: [-0.04, 0.025, 0.27], size: [0.26, 0.05, 0.12], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cyl, pos: [0.18, 0.1, 0.2], size: [0.07, 0.2, 0.07], rot: [0.0, 0.0, 0.0], mat: Mat::Peinture, id: None },
    Piece { shape: Shape::Cube, pos: [0.22, 0.015, 0.2], size: [0.1, 0.03, 0.08], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Sphere, pos: [0.2, 0.54, -0.14], size: [0.15, 0.15, 0.15], rot: [0.0, 0.0, 0.0], mat: Mat::Peinture, id: None },
    Piece { shape: Shape::Sphere, pos: [0.25, 0.56, -0.14], size: [0.11, 0.11, 0.11], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: None },
    Piece { shape: Shape::Sphere, pos: [0.3, 0.57, -0.14], size: [0.06, 0.06, 0.06], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: None },
    Piece { shape: Shape::Sphere, pos: [-0.2, 0.2, -0.26], size: [0.22, 0.2, 0.14], rot: [0.0, 0.0, 0.0], mat: Mat::Peinture, id: None },
    Piece { shape: Shape::Cube, pos: [-0.1, 0.07, -0.27], size: [0.14, 0.1, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::Peinture, id: None },
    Piece { shape: Shape::Cube, pos: [-0.04, 0.025, -0.27], size: [0.26, 0.05, 0.12], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cyl, pos: [0.18, 0.1, -0.2], size: [0.07, 0.2, 0.07], rot: [0.0, 0.0, 0.0], mat: Mat::Peinture, id: None },
    Piece { shape: Shape::Cube, pos: [0.22, 0.015, -0.2], size: [0.1, 0.03, 0.08], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Sphere, pos: [-0.12, 0.42, 0.1], size: [0.08, 0.05, 0.08], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Sphere, pos: [-0.18, 0.4, -0.08], size: [0.06, 0.04, 0.06], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [0.35, 0.28, 0.32], size: [0.04, 0.56, 0.04], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: Some("canne") },
    Piece { shape: Shape::Sphere, pos: [0.35, 0.6, 0.32], size: [0.12, 0.12, 0.12], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("canne") },
    Piece { shape: Shape::Sphere, pos: [0.35, 0.6, 0.32], size: [0.03, 0.03, 0.03], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("canne") },
];
const ANIMS_34: &[Anim] = &[
    Anim { target: "corps", kind: AnimKind::Bob, amp: 0.02 },
    Anim { target: "canne", kind: AnimKind::TailZ, amp: 0.0 },
];
const PIECES_35: &[Piece] = &[
    Piece { shape: Shape::Cube, pos: [0.3, 0.57, 0.14], size: [0.1, 0.1, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: None },
    Piece { shape: Shape::Cube, pos: [0.33, 0.57, 0.14], size: [0.02, 0.08, 0.08], rot: [0.0, 0.0, 0.0], mat: Mat::Vitre, id: None },
    Piece { shape: Shape::Cube, pos: [0.3, 0.57, -0.14], size: [0.1, 0.1, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: None },
    Piece { shape: Shape::Cube, pos: [0.33, 0.57, -0.14], size: [0.02, 0.08, 0.08], rot: [0.0, 0.0, 0.0], mat: Mat::Vitre, id: None },
    Piece { shape: Shape::Cube, pos: [0.3, 0.57, 0.0], size: [0.04, 0.03, 0.18], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: None },
    Piece { shape: Shape::Cube, pos: [0.2, 0.58, 0.25], size: [0.2, 0.02, 0.02], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: None },
    Piece { shape: Shape::Cube, pos: [0.2, 0.58, -0.25], size: [0.2, 0.02, 0.02], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: None },
    Piece { shape: Shape::Cyl, pos: [0.12, 0.76, 0.0], size: [0.3, 0.16, 0.3], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [0.12, 0.69, 0.0], size: [0.56, 0.03, 0.56], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [0.12, 0.725, 0.0], size: [0.32, 0.03, 0.32], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
];
const ANIMS_35: &[Anim] = &[
];
const JURA_TRAIN: [TrainCar; 0] = [
];
const JURA_COLORS: WorldColors = WorldColors {
    grass_a: [47, 107, 79],
    grass_b: [58, 122, 90],
    water: [47, 159, 191],
    road: [52, 54, 59],
    marking: [242, 194, 48],
    log: [138, 90, 43],
    sky: [143, 211, 232],
};
fn jura_name() -> &'static str { tx!("Parc Jurassic'Croa", "Jura-Croak Park", "Jura-Quak-Park", "Parco Giura-Gra", "Parque Jura-Croa").t() }
fn jura_banner() -> &'static str { tx!("Bienvenue au Parc Jurassic'Croa ! Merci de ne pas nourrir les dinosaures, ni de passer devant.", "Welcome to Jura-Croak Park! Please do not feed the dinosaurs, or walk in front of them.", "Willkommen im Jura-Quak-Park! Bitte die Dinosaurier nicht füttern und nicht vor ihnen herlaufen.", "Benvenuto al Parco Giura-Gra! Non dare da mangiare ai dinosauri e non passare davanti a loro.", "¡Bienvenido al Parque Jura-Croa! Por favor, no alimentes a los dinosaurios ni cruces delante de ellos.").t() }
fn jura_hero_name() -> &'static str { tx!("Grenouille Ranger", "Ranger Frog", "Ranger-Frosch", "Rana Guardiaparco", "Rana Guardabosques").t() }
pub static JURA_THEME: Theme = Theme {
    train: &JURA_TRAIN, only_own: false,
    blocks: &JURA_BLOCKS, sides: &JURA_SIDES, deco: &JURA_DECO, animals: &JURA_ANIMALS,
    block_mix: &[0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 3, 3, 3, 3, 3, 4, 5, 6, 2], side_mix: &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 100, 101, 103, 2, 3, 4, 5, 1, 0],
    side_pct: 22, vehicles: &[32, 33, 34, 35], colors: JURA_COLORS, points: 1500,
    name: jura_name, banner: jura_banner, animal_name: jura_animal_name, hero_name: jura_hero_name,
    hero: Model { c2: [242, 140, 40], c3: [43, 111, 62], pieces: PIECES_34, anims: ANIMS_34 }, hat: Model { c2: [242, 140, 40], c3: [43, 111, 62], pieces: PIECES_35, anims: ANIMS_35 },
};
const PIECES_36: &[Piece] = &[
    Piece { shape: Shape::Cyl, pos: [0.0, 0.25, 0.0], size: [0.22, 0.5, 0.22], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [0.05, 0.65, 0.0], size: [0.22, 0.5, 0.22], rot: [0.0, 0.0, -6.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [0.1, 1.0, 0.0], size: [0.22, 0.5, 0.22], rot: [0.0, 0.0, -10.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Sphere, pos: [0.15, 1.35, 0.0], size: [0.8, 0.64, 0.8], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Sphere, pos: [0.45, 1.15, 0.2], size: [0.5, 0.4, 0.5], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Sphere, pos: [-0.2, 1.2, 0.1], size: [0.5, 0.4, 0.5], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Sphere, pos: [0.2, 1.1, -0.35], size: [0.5, 0.4, 0.5], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Sphere, pos: [0.0, 1.55, 0.0], size: [0.45, 0.36, 0.45], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Sphere, pos: [0.35, 1.5, -0.1], size: [0.4, 0.32, 0.4], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Sphere, pos: [0.4, 0.04, 0.4], size: [0.1, 0.04, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("petale") },
    Piece { shape: Shape::Sphere, pos: [-0.3, 0.04, -0.4], size: [0.1, 0.04, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("petale") },
    Piece { shape: Shape::Sphere, pos: [0.1, 0.04, 0.5], size: [0.1, 0.04, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("petale") },
    Piece { shape: Shape::Cyl, pos: [0.0, 0.1, 0.0], size: [0.5, 0.2, 0.5], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
];
const ANIMS_36: &[Anim] = &[
];
const PIECES_37: &[Piece] = &[
    Piece { shape: Shape::Cube, pos: [0.0, 0.08, 0.0], size: [0.6, 0.16, 0.6], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cyl, pos: [0.0, 0.35, 0.0], size: [0.2, 0.4, 0.2], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.6, 0.0], size: [0.5, 0.14, 0.5], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.78, 0.0], size: [0.36, 0.2, 0.36], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [0.19, 0.78, 0.0], size: [0.02, 0.12, 0.14], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("flamme") },
    Piece { shape: Shape::Cube, pos: [0.0, 0.82, 0.19], size: [0.14, 0.12, 0.02], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("flamme") },
    Piece { shape: Shape::Cone, pos: [0.0, 1.0, 0.0], size: [0.6, 0.25, 0.6], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Sphere, pos: [0.0, 1.17, 0.0], size: [0.12, 0.12, 0.12], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.2, 0.0], size: [0.4, 0.04, 0.4], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cyl, pos: [0.0, 0.45, 0.0], size: [0.26, 0.04, 0.26], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.7, 0.0], size: [0.3, 0.06, 0.3], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Sphere, pos: [0.3, 0.06, 0.3], size: [0.14, 0.12, 0.14], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Sphere, pos: [-0.3, 0.05, -0.3], size: [0.12, 0.1, 0.12], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
];
const ANIMS_37: &[Anim] = &[
    Anim { target: "flamme", kind: AnimKind::Pulse, amp: 0.0 },
];
const PIECES_38: &[Piece] = &[
    Piece { shape: Shape::Cube, pos: [0.0, 0.55, 0.0], size: [0.7, 1.1, 0.6], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [0.36, 0.65, 0.0], size: [0.02, 0.7, 0.46], rot: [0.0, 0.0, 0.0], mat: Mat::Vitre, id: None },
    Piece { shape: Shape::Cyl, pos: [0.33, 0.85, -0.14], size: [0.1, 0.16, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::LumiereRouge, id: None },
    Piece { shape: Shape::Cyl, pos: [0.33, 0.85, 0.0], size: [0.1, 0.16, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::LumiereRouge, id: None },
    Piece { shape: Shape::Cyl, pos: [0.33, 0.85, 0.14], size: [0.1, 0.16, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::LumiereRouge, id: None },
    Piece { shape: Shape::Cyl, pos: [0.33, 0.62, -0.14], size: [0.1, 0.16, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::LumiereBleue, id: None },
    Piece { shape: Shape::Cyl, pos: [0.33, 0.62, 0.0], size: [0.1, 0.16, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::LumiereBleue, id: None },
    Piece { shape: Shape::Cyl, pos: [0.33, 0.62, 0.14], size: [0.1, 0.16, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::LumiereBleue, id: None },
    Piece { shape: Shape::Cyl, pos: [0.33, 0.4, -0.14], size: [0.1, 0.16, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: None },
    Piece { shape: Shape::Cyl, pos: [0.33, 0.4, 0.0], size: [0.1, 0.16, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: None },
    Piece { shape: Shape::Cyl, pos: [0.33, 0.4, 0.14], size: [0.1, 0.16, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: None },
    Piece { shape: Shape::Cube, pos: [0.36, 0.25, 0.18], size: [0.02, 0.2, 0.12], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: None },
    Piece { shape: Shape::Cube, pos: [0.36, 0.2, -0.12], size: [0.02, 0.08, 0.2], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("voyant") },
    Piece { shape: Shape::Cube, pos: [0.0, 1.12, 0.0], size: [0.72, 0.05, 0.62], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.04, 0.0], size: [0.76, 0.08, 0.66], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 1.0, 0.0], size: [0.6, 0.1, 0.5], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Sphere, pos: [0.36, 1.0, 0.0], size: [0.06, 0.06, 0.2], rot: [0.0, 0.0, 0.0], mat: Mat::LumiereBleue, id: Some("voyant") },
];
const ANIMS_38: &[Anim] = &[
    Anim { target: "voyant", kind: AnimKind::Pulse, amp: 0.0 },
];
const PIECES_39: &[Piece] = &[
    Piece { shape: Shape::Cyl, pos: [0.0, 0.75, 0.0], size: [0.1, 1.5, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cyl, pos: [0.0, 0.4, 0.0], size: [0.13, 0.04, 0.13], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [0.0, 0.8, 0.0], size: [0.13, 0.04, 0.13], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [0.2, 0.6, 0.15], size: [0.1, 1.2, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cyl, pos: [0.2, 0.4, 0.15], size: [0.13, 0.04, 0.13], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [0.2, 0.8, 0.15], size: [0.13, 0.04, 0.13], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.2, 0.65, 0.1], size: [0.1, 1.3, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.2, 0.4, 0.1], size: [0.13, 0.04, 0.13], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.2, 0.8, 0.1], size: [0.13, 0.04, 0.13], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [0.1, 0.7, -0.2], size: [0.1, 1.4, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cyl, pos: [0.1, 0.4, -0.2], size: [0.13, 0.04, 0.13], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [0.1, 0.8, -0.2], size: [0.13, 0.04, 0.13], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.15, 0.55, -0.2], size: [0.1, 1.1, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.15, 0.4, -0.2], size: [0.13, 0.04, 0.13], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.15, 0.8, -0.2], size: [0.13, 0.04, 0.13], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cone, pos: [0.0, 1.6, 0.0], size: [0.4, 0.3, 0.3], rot: [0.0, 0.0, 60.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cone, pos: [0.1, 1.4, 0.1], size: [0.4, 0.3, 0.3], rot: [0.0, 0.0, -60.0], mat: Mat::C3, id: None },
];
const ANIMS_39: &[Anim] = &[
];
const PIECES_40: &[Piece] = &[
    Piece { shape: Shape::Sphere, pos: [0.0, 0.35, 0.0], size: [0.8, 0.7, 0.65], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Sphere, pos: [0.05, 0.85, 0.0], size: [0.6, 0.5, 0.55], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cone, pos: [0.0, 1.15, 0.2], size: [0.16, 0.2, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Sphere, pos: [0.25, 0.92, 0.13], size: [0.1, 0.1, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: None },
    Piece { shape: Shape::Sphere, pos: [0.3, 0.92, 0.14], size: [0.05, 0.05, 0.05], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: None },
    Piece { shape: Shape::Cone, pos: [0.0, 1.15, -0.2], size: [0.16, 0.2, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Sphere, pos: [0.25, 0.92, -0.13], size: [0.1, 0.1, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: None },
    Piece { shape: Shape::Sphere, pos: [0.3, 0.92, -0.14], size: [0.05, 0.05, 0.05], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: None },
    Piece { shape: Shape::Cube, pos: [0.3, 0.8, 0.0], size: [0.04, 0.04, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: None },
    Piece { shape: Shape::Cyl, pos: [0.3, 0.6, 0.0], size: [0.2, 0.04, 0.2], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.6, 0.0], size: [0.45, 0.06, 0.4], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [0.3, 0.65, -0.4], size: [0.14, 0.4, 0.14], rot: [0.0, 0.0, -10.0], mat: Mat::C2, id: Some("patte_salut") },
    Piece { shape: Shape::Sphere, pos: [0.3, 0.88, -0.4], size: [0.18, 0.18, 0.18], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("patte_salut") },
    Piece { shape: Shape::Cyl, pos: [0.0, 0.08, 0.0], size: [0.7, 0.16, 0.7], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cyl, pos: [0.3, 0.6, 0.35], size: [0.2, 0.4, 0.2], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
];
const ANIMS_40: &[Anim] = &[
    Anim { target: "patte_salut", kind: AnimKind::SwingZBase, amp: 0.0 },
];
const PIECES_41: &[Piece] = &[
    Piece { shape: Shape::Cube, pos: [0.0, 0.04, 0.0], size: [0.95, 0.08, 0.95], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.09, -0.36], size: [0.9, 0.02, 0.04], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.09, -0.18], size: [0.9, 0.02, 0.04], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.09, 0.0], size: [0.9, 0.02, 0.04], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.09, 0.18], size: [0.9, 0.02, 0.04], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.09, 0.36], size: [0.9, 0.02, 0.04], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: None },
    Piece { shape: Shape::Sphere, pos: [0.0, 0.3, 0.0], size: [0.6, 0.48, 0.6], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Sphere, pos: [0.25, 0.18, 0.2], size: [0.35, 0.28, 0.35], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Sphere, pos: [-0.25, 0.15, -0.25], size: [0.3, 0.24, 0.3], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cone, pos: [0.0, 0.65, 0.0], size: [0.2, 0.2, 0.2], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Sphere, pos: [0.3, 0.1, -0.3], size: [0.15, 0.1, 0.15], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Sphere, pos: [-0.3, 0.1, 0.3], size: [0.15, 0.1, 0.15], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
];
const ANIMS_41: &[Anim] = &[
];
const PIECES_42: &[Piece] = &[
    Piece { shape: Shape::Cube, pos: [0.0, 0.2, 0.0], size: [0.8, 0.3, 0.8], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cone, pos: [0.0, 0.4, 0.0], size: [1.2, 0.16, 1.2], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.6, 0.0], size: [0.65, 0.3, 0.65], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cone, pos: [0.0, 0.8, 0.0], size: [0.975, 0.16, 0.975], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 1.0, 0.0], size: [0.5, 0.3, 0.5], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cone, pos: [0.0, 1.2, 0.0], size: [0.75, 0.16, 0.75], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [0.0, 1.45, 0.0], size: [0.03, 0.3, 0.03], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Sphere, pos: [0.0, 1.62, 0.0], size: [0.08, 0.08, 0.08], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.04, 0.0], size: [0.9, 0.08, 0.9], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cube, pos: [0.336, 0.2, 0.15], size: [0.02, 0.16, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::Phare, id: None },
    Piece { shape: Shape::Cube, pos: [0.336, 0.2, -0.15], size: [0.02, 0.16, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::Phare, id: None },
    Piece { shape: Shape::Cube, pos: [0.5, 0.04, 0.0], size: [0.2, 0.08, 0.4], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cube, pos: [0.6, 0.02, 0.0], size: [0.2, 0.04, 0.3], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
];
const ANIMS_42: &[Anim] = &[
];
const NIPPON_BLOCKS: [Model; 7] = [
    Model { c2: [255, 183, 208], c3: [122, 77, 36], pieces: PIECES_36, anims: ANIMS_36 },
    Model { c2: [154, 154, 163], c3: [255, 217, 138], pieces: PIECES_37, anims: ANIMS_37 },
    Model { c2: [255, 90, 90], c3: [53, 208, 255], pieces: PIECES_38, anims: ANIMS_38 },
    Model { c2: [74, 154, 86], c3: [143, 207, 122], pieces: PIECES_39, anims: ANIMS_39 },
    Model { c2: [255, 255, 255], c3: [255, 210, 63], pieces: PIECES_40, anims: ANIMS_40 },
    Model { c2: [217, 205, 176], c3: [143, 143, 149], pieces: PIECES_41, anims: ANIMS_41 },
    Model { c2: [200, 40, 60], c3: [42, 47, 74], pieces: PIECES_42, anims: ANIMS_42 },
];
const PIECES_43: &[Piece] = &[
    Piece { shape: Shape::Cube, pos: [0.0, 0.5, 0.0], size: [2.0, 0.5, 2.0], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cone, pos: [0.0, 0.9, 0.0], size: [3.0, 0.3, 3.0], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 1.25, 0.0], size: [1.72, 0.5, 1.72], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cone, pos: [0.0, 1.65, 0.0], size: [2.58, 0.3, 2.58], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 2.0, 0.0], size: [1.44, 0.5, 1.44], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cone, pos: [0.0, 2.4, 0.0], size: [2.16, 0.3, 2.16], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 2.75, 0.0], size: [1.16, 0.5, 1.16], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cone, pos: [0.0, 3.15, 0.0], size: [1.74, 0.3, 1.74], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 3.5, 0.0], size: [0.88, 0.5, 0.88], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cone, pos: [0.0, 3.9, 0.0], size: [1.32, 0.3, 1.32], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [0.0, 4.7, 0.0], size: [0.06, 1.0, 0.06], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Sphere, pos: [0.0, 5.3, 0.0], size: [0.2, 0.2, 0.2], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.1, 0.0], size: [2.4, 0.2, 2.4], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cube, pos: [1.01, 0.5, 0.0], size: [0.04, 0.6, 0.5], rot: [0.0, 0.0, 0.0], mat: Mat::Phare, id: None },
];
const ANIMS_43: &[Anim] = &[
];
const PIECES_44: &[Piece] = &[
    Piece { shape: Shape::Cone, pos: [0.0, 1.6, 0.0], size: [4.0, 3.2, 4.0], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cone, pos: [0.0, 3.0, 0.0], size: [1.7, 1.0, 1.7], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cone, pos: [1.2, 0.7, 0.5], size: [1.6, 1.4, 1.6], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cone, pos: [-1.4, 0.6, -0.4], size: [1.4, 1.2, 1.4], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Sphere, pos: [0.3, 3.7, 0.0], size: [0.7, 0.35, 0.6], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("nuage") },
    Piece { shape: Shape::Sphere, pos: [-0.3, 3.9, 0.2], size: [0.8, 0.4, 0.6], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("nuage") },
    Piece { shape: Shape::Sphere, pos: [0.7, 0.2, 0.8], size: [0.6, 0.4, 0.6], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Sphere, pos: [-0.8, 0.2, 0.9], size: [0.6, 0.4, 0.6], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Sphere, pos: [0.0, 0.2, 1.2], size: [0.6, 0.4, 0.6], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cone, pos: [0.5, 2.0, 0.7], size: [0.7, 0.8, 0.7], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cone, pos: [-0.6, 1.7, 0.8], size: [0.6, 0.7, 0.6], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cone, pos: [-1.0, 2.2, 0.3], size: [0.5, 0.6, 0.5], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Sphere, pos: [1.6, 0.15, 0.9], size: [0.5, 0.3, 0.5], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
];
const ANIMS_44: &[Anim] = &[
    Anim { target: "nuage", kind: AnimKind::DriftUp, amp: 0.0 },
];
const PIECES_45: &[Piece] = &[
    Piece { shape: Shape::Cyl, pos: [0.0, 1.2, -1.1], size: [0.4, 2.4, 0.4], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cyl, pos: [0.0, 1.2, 1.1], size: [0.4, 2.4, 0.4], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 2.35, 0.0], size: [0.4, 0.3, 3.2], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 2.7, 0.0], size: [0.5, 0.2, 3.8], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 1.9, 0.0], size: [0.3, 0.2, 2.4], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.1, -1.1], size: [0.7, 0.2, 0.7], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.1, 1.1], size: [0.7, 0.2, 0.7], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 2.1, 0.0], size: [0.1, 0.4, 0.4], rot: [0.0, 0.0, 0.0], mat: Mat::Phare, id: None },
    Piece { shape: Shape::Cone, pos: [0.0, 2.9, -1.9], size: [0.4, 0.3, 0.3], rot: [0.0, 0.0, -90.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cone, pos: [0.0, 2.9, 1.9], size: [0.4, 0.3, 0.3], rot: [0.0, 0.0, 90.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [0.0, 0.3, -1.1], size: [0.5, 0.2, 0.5], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [0.0, 0.3, 1.1], size: [0.5, 0.2, 0.5], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 1.4, 0.0], size: [0.2, 0.2, 2.2], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [0.3, 2.1, 0.0], size: [0.04, 0.5, 0.3], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
];
const ANIMS_45: &[Anim] = &[
];
const PIECES_46: &[Piece] = &[
    Piece { shape: Shape::Cube, pos: [0.0, 2.0, 0.0], size: [1.6, 4.0, 1.4], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cube, pos: [0.81, 0.8, -0.4], size: [0.02, 0.3, 0.25], rot: [0.0, 0.0, 0.0], mat: Mat::Vitre, id: None },
    Piece { shape: Shape::Cube, pos: [0.81, 0.8, 0.0], size: [0.02, 0.3, 0.25], rot: [0.0, 0.0, 0.0], mat: Mat::Vitre, id: None },
    Piece { shape: Shape::Cube, pos: [0.81, 0.8, 0.4], size: [0.02, 0.3, 0.25], rot: [0.0, 0.0, 0.0], mat: Mat::Vitre, id: None },
    Piece { shape: Shape::Cube, pos: [0.81, 1.6, -0.4], size: [0.02, 0.3, 0.25], rot: [0.0, 0.0, 0.0], mat: Mat::Vitre, id: None },
    Piece { shape: Shape::Cube, pos: [0.81, 1.6, 0.0], size: [0.02, 0.3, 0.25], rot: [0.0, 0.0, 0.0], mat: Mat::Vitre, id: None },
    Piece { shape: Shape::Cube, pos: [0.81, 1.6, 0.4], size: [0.02, 0.3, 0.25], rot: [0.0, 0.0, 0.0], mat: Mat::Vitre, id: None },
    Piece { shape: Shape::Cube, pos: [0.81, 2.4, -0.4], size: [0.02, 0.3, 0.25], rot: [0.0, 0.0, 0.0], mat: Mat::Vitre, id: None },
    Piece { shape: Shape::Cube, pos: [0.81, 2.4, 0.0], size: [0.02, 0.3, 0.25], rot: [0.0, 0.0, 0.0], mat: Mat::Vitre, id: None },
    Piece { shape: Shape::Cube, pos: [0.81, 2.4, 0.4], size: [0.02, 0.3, 0.25], rot: [0.0, 0.0, 0.0], mat: Mat::Vitre, id: None },
    Piece { shape: Shape::Cube, pos: [0.81, 3.2, -0.4], size: [0.02, 0.3, 0.25], rot: [0.0, 0.0, 0.0], mat: Mat::Vitre, id: None },
    Piece { shape: Shape::Cube, pos: [0.81, 3.2, 0.0], size: [0.02, 0.3, 0.25], rot: [0.0, 0.0, 0.0], mat: Mat::Vitre, id: None },
    Piece { shape: Shape::Cube, pos: [0.81, 3.2, 0.4], size: [0.02, 0.3, 0.25], rot: [0.0, 0.0, 0.0], mat: Mat::Vitre, id: None },
    Piece { shape: Shape::Cube, pos: [0.82, 3.8, 0.0], size: [0.04, 0.4, 1.2], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("enseigne") },
    Piece { shape: Shape::Cube, pos: [0.84, 3.8, 0.0], size: [0.02, 0.12, 0.8], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: Some("enseigne") },
    Piece { shape: Shape::Cube, pos: [0.82, 2.0, 0.75], size: [0.04, 2.4, 0.2], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("neon") },
    Piece { shape: Shape::Cyl, pos: [0.0, 4.3, 0.0], size: [0.05, 0.6, 0.05], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Sphere, pos: [0.0, 4.65, 0.0], size: [0.12, 0.12, 0.12], rot: [0.0, 0.0, 0.0], mat: Mat::LumiereRouge, id: Some("balise") },
    Piece { shape: Shape::Cube, pos: [0.0, 0.1, 0.0], size: [1.8, 0.2, 1.6], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: None },
];
const ANIMS_46: &[Anim] = &[
    Anim { target: "enseigne", kind: AnimKind::Blink, amp: 0.0 },
    Anim { target: "neon", kind: AnimKind::Pulse, amp: 0.0 },
    Anim { target: "balise", kind: AnimKind::Blink, amp: 0.0 },
];
const PIECES_47: &[Piece] = &[
    Piece { shape: Shape::Cube, pos: [0.0, 0.6, 0.0], size: [3.0, 0.8, 2.4], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cone, pos: [0.0, 1.15, 0.0], size: [3.9, 0.4, 3.12], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 1.5, 0.0], size: [2.4, 0.8, 1.8], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cone, pos: [0.0, 2.05, 0.0], size: [3.12, 0.4, 2.34], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 2.3, 0.0], size: [1.8, 0.8, 1.3], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cone, pos: [0.0, 2.85, 0.0], size: [2.34, 0.4, 1.69], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.15, 0.0], size: [3.4, 0.3, 2.8], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Sphere, pos: [0.0, 3.3, 0.0], size: [0.3, 0.3, 0.3], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [1.53, 0.6, -0.5], size: [0.04, 0.4, 0.3], rot: [0.0, 0.0, 0.0], mat: Mat::Vitre, id: None },
    Piece { shape: Shape::Cube, pos: [1.53, 0.6, 0.0], size: [0.04, 0.4, 0.3], rot: [0.0, 0.0, 0.0], mat: Mat::Vitre, id: None },
    Piece { shape: Shape::Cube, pos: [1.53, 0.6, 0.5], size: [0.04, 0.4, 0.3], rot: [0.0, 0.0, 0.0], mat: Mat::Vitre, id: None },
    Piece { shape: Shape::Cube, pos: [1.6, 0.2, 0.0], size: [0.3, 0.3, 3.0], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cube, pos: [1.55, 0.45, 0.0], size: [0.3, 0.6, 0.7], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
];
const ANIMS_47: &[Anim] = &[
];
const PIECES_48: &[Piece] = &[
    Piece { shape: Shape::Cyl, pos: [0.0, 1.5, 0.0], size: [2.2, 1.0, 2.2], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cyl, pos: [0.0, 2.02, 0.0], size: [1.9, 0.06, 1.9], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [0.6, 2.08, 0.0], size: [0.3, 0.06, 0.3], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: None },
    Piece { shape: Shape::Cyl, pos: [0.424, 2.08, 0.424], size: [0.3, 0.06, 0.3], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: None },
    Piece { shape: Shape::Cyl, pos: [0.0, 2.08, 0.6], size: [0.3, 0.06, 0.3], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.424, 2.08, 0.424], size: [0.3, 0.06, 0.3], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.6, 2.08, 0.0], size: [0.3, 0.06, 0.3], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.424, 2.08, -0.424], size: [0.3, 0.06, 0.3], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.0, 2.08, -0.6], size: [0.3, 0.06, 0.3], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: None },
    Piece { shape: Shape::Cyl, pos: [0.424, 2.08, -0.424], size: [0.3, 0.06, 0.3], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: None },
    Piece { shape: Shape::Sphere, pos: [0.0, 2.15, 0.0], size: [0.4, 0.2, 0.4], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [0.6, 2.5, 0.2], size: [0.2, 0.5, 0.2], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: Some("vapeur") },
    Piece { shape: Shape::Cyl, pos: [-0.4, 2.7, -0.3], size: [0.2, 0.6, 0.2], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: Some("vapeur") },
    Piece { shape: Shape::Cyl, pos: [0.0, 0.4, 0.0], size: [0.6, 0.8, 0.6], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cyl, pos: [0.0, 0.1, 0.0], size: [1.8, 0.2, 1.8], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cube, pos: [1.12, 1.3, 0.0], size: [0.04, 0.4, 1.6], rot: [0.0, 0.0, 0.0], mat: Mat::LumiereRouge, id: Some("neon") },
];
const ANIMS_48: &[Anim] = &[
    Anim { target: "vapeur", kind: AnimKind::DriftUp, amp: 0.0 },
    Anim { target: "neon", kind: AnimKind::BlinkAlt, amp: 0.0 },
];
const NIPPON_SIDES: [Model; 6] = [
    Model { c2: [200, 40, 60], c3: [42, 47, 74], pieces: PIECES_43, anims: ANIMS_43 },
    Model { c2: [255, 255, 255], c3: [106, 122, 138], pieces: PIECES_44, anims: ANIMS_44 },
    Model { c2: [200, 40, 60], c3: [42, 47, 74], pieces: PIECES_45, anims: ANIMS_45 },
    Model { c2: [255, 95, 168], c3: [53, 208, 255], pieces: PIECES_46, anims: ANIMS_46 },
    Model { c2: [255, 255, 255], c3: [42, 47, 74], pieces: PIECES_47, anims: ANIMS_47 },
    Model { c2: [255, 210, 63], c3: [255, 90, 60], pieces: PIECES_48, anims: ANIMS_48 },
];
const PIECES_49: &[Piece] = &[
    Piece { shape: Shape::Cyl, pos: [0.0, 0.3, 0.0], size: [0.04, 0.6, 0.04], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Sphere, pos: [0.0, 0.55, 0.0], size: [0.2, 0.26, 0.2], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("lampe") },
];
const ANIMS_49: &[Anim] = &[
    Anim { target: "lampe", kind: AnimKind::Pulse, amp: 0.0 },
];
const PIECES_50: &[Piece] = &[
    Piece { shape: Shape::Cone, pos: [0.0, 0.12, 0.0], size: [0.4, 0.3, 0.04], rot: [0.0, 0.0, 180.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cyl, pos: [0.0, 0.02, 0.0], size: [0.04, 0.1, 0.04], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.14, 0.0], size: [0.4, 0.01, 0.03], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
];
const ANIMS_50: &[Anim] = &[
];
const PIECES_51: &[Piece] = &[
    Piece { shape: Shape::Sphere, pos: [0.0, 0.1, 0.0], size: [0.3, 0.14, 0.14], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cone, pos: [-0.2, 0.1, 0.0], size: [0.1, 0.14, 0.04], rot: [0.0, 0.0, 90.0], mat: Mat::C2, id: Some("queue") },
    Piece { shape: Shape::Sphere, pos: [0.1, 0.12, 0.04], size: [0.04, 0.04, 0.04], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: None },
];
const ANIMS_51: &[Anim] = &[
    Anim { target: "queue", kind: AnimKind::Sway, amp: 0.0 },
];
const PIECES_52: &[Piece] = &[
    Piece { shape: Shape::Sphere, pos: [0.0, 0.02, 0.0], size: [0.08, 0.02, 0.06], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Sphere, pos: [0.1, 0.02, 0.05], size: [0.08, 0.02, 0.06], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Sphere, pos: [-0.08, 0.02, 0.08], size: [0.08, 0.02, 0.06], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
];
const ANIMS_52: &[Anim] = &[
];
const PIECES_53: &[Piece] = &[
    Piece { shape: Shape::Cube, pos: [0.0, 0.12, 0.0], size: [0.2, 0.2, 0.2], rot: [0.0, 0.0, 45.0], mat: Mat::Blanc, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.04, 0.0], size: [0.24, 0.08, 0.2], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
];
const ANIMS_53: &[Anim] = &[
];
const PIECES_54: &[Piece] = &[
    Piece { shape: Shape::Cyl, pos: [0.0, 0.25, 0.0], size: [0.03, 0.5, 0.03], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cone, pos: [0.0, 0.45, 0.0], size: [0.5, 0.2, 0.5], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
];
const ANIMS_54: &[Anim] = &[
];
const PIECES_55: &[Piece] = &[
    Piece { shape: Shape::Cyl, pos: [0.0, 0.08, 0.0], size: [0.28, 0.16, 0.28], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cyl, pos: [0.0, 0.17, 0.0], size: [0.24, 0.02, 0.24], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Sphere, pos: [0.05, 0.2, 0.0], size: [0.08, 0.04, 0.08], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: None },
    Piece { shape: Shape::Cyl, pos: [0.0, 0.3, 0.0], size: [0.06, 0.18, 0.06], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: Some("vapeur") },
];
const ANIMS_55: &[Anim] = &[
    Anim { target: "vapeur", kind: AnimKind::DriftUp, amp: 0.0 },
];
const PIECES_56: &[Piece] = &[
    Piece { shape: Shape::Cube, pos: [0.0, 0.4, 0.0], size: [0.3, 0.3, 0.02], rot: [0.0, 0.0, 45.0], mat: Mat::C2, id: Some("cv") },
    Piece { shape: Shape::Cyl, pos: [0.0, 0.15, 0.0], size: [0.01, 0.3, 0.01], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
];
const ANIMS_56: &[Anim] = &[
    Anim { target: "cv", kind: AnimKind::Sway, amp: 0.0 },
];
const PIECES_57: &[Piece] = &[
    Piece { shape: Shape::Cyl, pos: [0.0, 0.25, 0.0], size: [0.04, 0.5, 0.04], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.52, 0.0], size: [0.04, 0.24, 0.3], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: None },
    Piece { shape: Shape::Cube, pos: [0.03, 0.52, 0.0], size: [0.02, 0.14, 0.08], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: None },
];
const ANIMS_57: &[Anim] = &[
];
const NIPPON_DECO: [Model; 9] = [
    Model { c2: [216, 40, 60], c3: [255, 210, 63], pieces: PIECES_49, anims: ANIMS_49 },
    Model { c2: [255, 95, 168], c3: [255, 210, 63], pieces: PIECES_50, anims: ANIMS_50 },
    Model { c2: [255, 122, 69], c3: [255, 255, 255], pieces: PIECES_51, anims: ANIMS_51 },
    Model { c2: [255, 183, 208], c3: [136, 136, 136], pieces: PIECES_52, anims: ANIMS_52 },
    Model { c2: [255, 255, 255], c3: [42, 47, 74], pieces: PIECES_53, anims: ANIMS_53 },
    Model { c2: [255, 95, 168], c3: [138, 90, 43], pieces: PIECES_54, anims: ANIMS_54 },
    Model { c2: [255, 210, 63], c3: [255, 90, 60], pieces: PIECES_55, anims: ANIMS_55 },
    Model { c2: [53, 208, 255], c3: [255, 90, 90], pieces: PIECES_56, anims: ANIMS_56 },
    Model { c2: [255, 255, 255], c3: [42, 47, 74], pieces: PIECES_57, anims: ANIMS_57 },
];
const PIECES_58: &[Piece] = &[
    Piece { shape: Shape::Cube, pos: [-0.035, 0.25, 0.0], size: [0.42, 0.3, 0.4], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Cube, pos: [-0.035, 0.151, 0.0], size: [0.378, 0.102, 0.412], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [0.175, 0.334, 0.0], size: [0.105, 0.18, 0.238], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [0.262, 0.34, 0.0], size: [0.175, 0.26, 0.28], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Sphere, pos: [0.297, 0.392, 0.14], size: [0.099, 0.099, 0.099], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: None },
    Piece { shape: Shape::Sphere, pos: [0.319, 0.392, 0.17], size: [0.049, 0.049, 0.049], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: None },
    Piece { shape: Shape::Sphere, pos: [0.297, 0.392, -0.14], size: [0.099, 0.099, 0.099], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: None },
    Piece { shape: Shape::Sphere, pos: [0.319, 0.392, -0.17], size: [0.049, 0.049, 0.049], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: None },
    Piece { shape: Shape::Cyl, pos: [0.099, 0.05, 0.165], size: [0.07, 0.1, 0.07], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("patte_av_g") },
    Piece { shape: Shape::Cyl, pos: [0.099, 0.05, -0.165], size: [0.07, 0.1, 0.07], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("patte_av_d") },
    Piece { shape: Shape::Cyl, pos: [-0.169, 0.05, 0.165], size: [0.07, 0.1, 0.07], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("patte_ar_g") },
    Piece { shape: Shape::Cyl, pos: [-0.169, 0.05, -0.165], size: [0.07, 0.1, 0.07], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("patte_ar_d") },
    Piece { shape: Shape::Cone, pos: [-0.297, 0.31, 0.0], size: [0.2, 0.105, 0.2], rot: [0.0, 0.0, 90.0], mat: Mat::C2, id: Some("queue") },
    Piece { shape: Shape::Cube, pos: [0.15, 0.62, 0.08], size: [0.07, 0.3, 0.05], rot: [0.0, 0.0, -10.0], mat: Mat::C2, id: Some("oreille") },
    Piece { shape: Shape::Cube, pos: [0.15, 0.62, 0.08], size: [0.04, 0.24, 0.02], rot: [0.0, 0.0, -10.0], mat: Mat::C2, id: Some("oreille") },
    Piece { shape: Shape::Cube, pos: [0.15, 0.62, -0.08], size: [0.07, 0.3, 0.05], rot: [0.0, 0.0, -10.0], mat: Mat::C2, id: Some("oreille") },
    Piece { shape: Shape::Cube, pos: [0.15, 0.62, -0.08], size: [0.04, 0.24, 0.02], rot: [0.0, 0.0, -10.0], mat: Mat::C2, id: Some("oreille") },
    Piece { shape: Shape::Sphere, pos: [-0.3, 0.3, 0.0], size: [0.14, 0.14, 0.14], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Sphere, pos: [0.0, 0.46, 0.0], size: [0.2, 0.1, 0.2], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: None },
];
const ANIMS_58: &[Anim] = &[
    Anim { target: "corps", kind: AnimKind::Bob, amp: 0.02 },
    Anim { target: "patte_av_g", kind: AnimKind::SwingZ, amp: 0.0 },
    Anim { target: "patte_av_d", kind: AnimKind::SwingZ, amp: 0.5 },
    Anim { target: "patte_ar_g", kind: AnimKind::SwingZ, amp: 0.5 },
    Anim { target: "patte_ar_d", kind: AnimKind::SwingZ, amp: 0.0 },
    Anim { target: "queue", kind: AnimKind::TailY, amp: 0.0 },
    Anim { target: "oreille", kind: AnimKind::TailX, amp: 0.0 },
];
const PIECES_59: &[Piece] = &[
    Piece { shape: Shape::Cube, pos: [0.0, 0.35, 0.0], size: [0.45, 0.4, 0.5], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Cube, pos: [0.0, 0.218, 0.0], size: [0.405, 0.136, 0.515], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [0.225, 0.462, 0.0], size: [0.135, 0.24, 0.289], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [0.338, 0.47, 0.0], size: [0.225, 0.3, 0.34], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Sphere, pos: [0.383, 0.53, 0.17], size: [0.114, 0.114, 0.114], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: None },
    Piece { shape: Shape::Sphere, pos: [0.408, 0.53, 0.204], size: [0.057, 0.057, 0.057], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: None },
    Piece { shape: Shape::Sphere, pos: [0.383, 0.53, -0.17], size: [0.114, 0.114, 0.114], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: None },
    Piece { shape: Shape::Sphere, pos: [0.408, 0.53, -0.204], size: [0.057, 0.057, 0.057], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: None },
    Piece { shape: Shape::Cyl, pos: [0.144, 0.075, 0.2], size: [0.1, 0.15, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("patte_av_g") },
    Piece { shape: Shape::Cyl, pos: [0.144, 0.075, -0.2], size: [0.1, 0.15, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("patte_av_d") },
    Piece { shape: Shape::Cyl, pos: [-0.144, 0.075, 0.2], size: [0.1, 0.15, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("patte_ar_g") },
    Piece { shape: Shape::Cyl, pos: [-0.144, 0.075, -0.2], size: [0.1, 0.15, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("patte_ar_d") },
    Piece { shape: Shape::Cone, pos: [-0.338, 0.43, 0.0], size: [0.25, 0.225, 0.25], rot: [0.0, 0.0, 90.0], mat: Mat::C2, id: Some("queue") },
    Piece { shape: Shape::Sphere, pos: [0.0, 0.38, 0.0], size: [0.46, 0.42, 0.52], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cone, pos: [0.288, 0.67, 0.12], size: [0.08, 0.1, 0.08], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: None },
    Piece { shape: Shape::Cone, pos: [0.288, 0.67, -0.12], size: [0.08, 0.1, 0.08], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: None },
    Piece { shape: Shape::Cone, pos: [0.338, 0.75, 0.0], size: [0.2, 0.16, 0.04], rot: [0.0, 0.0, 10.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Sphere, pos: [0.45, 0.44, 0.0], size: [0.07, 0.06, 0.07], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: None },
];
const ANIMS_59: &[Anim] = &[
    Anim { target: "corps", kind: AnimKind::Bob, amp: 0.02 },
    Anim { target: "patte_av_g", kind: AnimKind::SwingZ, amp: 0.0 },
    Anim { target: "patte_av_d", kind: AnimKind::SwingZ, amp: 0.5 },
    Anim { target: "patte_ar_g", kind: AnimKind::SwingZ, amp: 0.5 },
    Anim { target: "patte_ar_d", kind: AnimKind::SwingZ, amp: 0.0 },
    Anim { target: "queue", kind: AnimKind::TailY, amp: 0.0 },
];
const PIECES_60: &[Piece] = &[
    Piece { shape: Shape::Sphere, pos: [0.16, 1.16, 0.0], size: [1.36, 0.96, 0.88], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [0.24, 0.96, 0.0], size: [1.04, 0.6, 0.768], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [-0.48, 1.2, 0.0], size: [0.8, 0.76, 0.8], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [0.84, 1.52, 0.0], size: [0.64, 0.64, 0.56], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [1.24, 1.72, 0.0], size: [0.8, 0.544, 0.624], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [1.6, 1.68, 0.0], size: [0.76, 0.352, 0.464], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [1.904, 1.68, 0.0], size: [0.24, 0.304, 0.4], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Cube, pos: [1.56, 1.472, 0.0], size: [0.68, 0.128, 0.368], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("machoire") },
    Piece { shape: Shape::Cube, pos: [1.856, 1.488, 0.0], size: [0.08, 0.144, 0.32], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("machoire") },
    Piece { shape: Shape::Cone, pos: [1.36, 1.52, 0.152], size: [0.056, 0.12, 0.056], rot: [180.0, 0.0, 0.0], mat: Mat::Blanc, id: Some("corps") },
    Piece { shape: Shape::Cone, pos: [1.36, 1.52, -0.152], size: [0.056, 0.12, 0.056], rot: [180.0, 0.0, 0.0], mat: Mat::Blanc, id: Some("corps") },
    Piece { shape: Shape::Cone, pos: [1.52, 1.52, 0.152], size: [0.056, 0.12, 0.056], rot: [180.0, 0.0, 0.0], mat: Mat::Blanc, id: Some("corps") },
    Piece { shape: Shape::Cone, pos: [1.52, 1.52, -0.152], size: [0.056, 0.12, 0.056], rot: [180.0, 0.0, 0.0], mat: Mat::Blanc, id: Some("corps") },
    Piece { shape: Shape::Cone, pos: [1.68, 1.52, 0.152], size: [0.056, 0.12, 0.056], rot: [180.0, 0.0, 0.0], mat: Mat::Blanc, id: Some("corps") },
    Piece { shape: Shape::Cone, pos: [1.68, 1.52, -0.152], size: [0.056, 0.12, 0.056], rot: [180.0, 0.0, 0.0], mat: Mat::Blanc, id: Some("corps") },
    Piece { shape: Shape::Cone, pos: [1.824, 1.52, 0.152], size: [0.056, 0.12, 0.056], rot: [180.0, 0.0, 0.0], mat: Mat::Blanc, id: Some("corps") },
    Piece { shape: Shape::Cone, pos: [1.824, 1.52, -0.152], size: [0.056, 0.12, 0.056], rot: [180.0, 0.0, 0.0], mat: Mat::Blanc, id: Some("corps") },
    Piece { shape: Shape::Cone, pos: [1.424, 1.56, 0.136], size: [0.048, 0.096, 0.048], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: Some("machoire") },
    Piece { shape: Shape::Cone, pos: [1.424, 1.56, -0.136], size: [0.048, 0.096, 0.048], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: Some("machoire") },
    Piece { shape: Shape::Cone, pos: [1.6, 1.56, 0.136], size: [0.048, 0.096, 0.048], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: Some("machoire") },
    Piece { shape: Shape::Cone, pos: [1.6, 1.56, -0.136], size: [0.048, 0.096, 0.048], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: Some("machoire") },
    Piece { shape: Shape::Cone, pos: [1.76, 1.56, 0.136], size: [0.048, 0.096, 0.048], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: Some("machoire") },
    Piece { shape: Shape::Cone, pos: [1.76, 1.56, -0.136], size: [0.048, 0.096, 0.048], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: Some("machoire") },
    Piece { shape: Shape::Sphere, pos: [1.424, 1.872, 0.288], size: [0.176, 0.176, 0.096], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [1.488, 1.88, 0.302], size: [0.08, 0.08, 0.056], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [1.424, 1.872, -0.288], size: [0.176, 0.176, 0.096], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [1.488, 1.88, -0.302], size: [0.08, 0.08, 0.056], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [1.92, 1.776, 0.096], size: [0.048, 0.048, 0.048], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [1.92, 1.776, -0.096], size: [0.048, 0.048, 0.048], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("corps") },
    Piece { shape: Shape::Cone, pos: [1.32, 2.456, 0.0], size: [0.272, 0.8, 0.096], rot: [0.0, 0.0, 16.0], mat: Mat::C3, id: Some("crete_pointe") },
    Piece { shape: Shape::Cone, pos: [1.12, 2.404, 0.0], size: [0.272, 0.72, 0.096], rot: [0.0, 0.0, 16.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cone, pos: [0.896, 2.302, 0.0], size: [0.272, 0.672, 0.096], rot: [0.0, 0.0, 16.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cone, pos: [0.64, 2.088, 0.0], size: [0.272, 0.64, 0.096], rot: [0.0, 0.0, 16.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cone, pos: [0.36, 1.979, 0.0], size: [0.272, 0.576, 0.096], rot: [0.0, 0.0, 16.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cone, pos: [0.08, 1.941, 0.0], size: [0.272, 0.544, 0.096], rot: [0.0, 0.0, 16.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cone, pos: [-0.24, 1.926, 0.0], size: [0.272, 0.512, 0.096], rot: [0.0, 0.0, 16.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cone, pos: [-0.56, 1.896, 0.0], size: [0.272, 0.48, 0.096], rot: [0.0, 0.0, 16.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cone, pos: [-0.88, 1.73, 0.0], size: [0.272, 0.432, 0.096], rot: [0.0, 0.0, 16.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cone, pos: [-1.24, 1.629, 0.0], size: [0.272, 0.384, 0.096], rot: [0.0, 0.0, 16.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cone, pos: [-1.6, 1.52, 0.0], size: [0.272, 0.32, 0.096], rot: [0.0, 0.0, 16.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cone, pos: [-2.0, 1.411, 0.0], size: [0.272, 0.256, 0.096], rot: [0.0, 0.0, 16.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [0.76, 1.04, 0.4], size: [0.32, 0.128, 0.12], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("bras_g") },
    Piece { shape: Shape::Cone, pos: [0.96, 0.992, 0.432], size: [0.04, 0.12, 0.04], rot: [0.0, 0.0, -90.0], mat: Mat::Blanc, id: Some("bras_g") },
    Piece { shape: Shape::Cone, pos: [0.96, 0.992, 0.368], size: [0.04, 0.12, 0.04], rot: [0.0, 0.0, -90.0], mat: Mat::Blanc, id: Some("bras_g") },
    Piece { shape: Shape::Cube, pos: [0.76, 1.04, -0.4], size: [0.32, 0.128, 0.12], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("bras_d") },
    Piece { shape: Shape::Cone, pos: [0.96, 0.992, -0.368], size: [0.04, 0.12, 0.04], rot: [0.0, 0.0, -90.0], mat: Mat::Blanc, id: Some("bras_d") },
    Piece { shape: Shape::Cone, pos: [0.96, 0.992, -0.432], size: [0.04, 0.12, 0.04], rot: [0.0, 0.0, -90.0], mat: Mat::Blanc, id: Some("bras_d") },
    Piece { shape: Shape::Sphere, pos: [-0.32, 0.8, 0.4], size: [0.68, 0.84, 0.4], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("patte_g") },
    Piece { shape: Shape::Cyl, pos: [-0.24, 0.36, 0.4], size: [0.24, 0.64, 0.24], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("patte_g") },
    Piece { shape: Shape::Cube, pos: [-0.096, 0.056, 0.4], size: [0.64, 0.112, 0.336], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: Some("patte_g") },
    Piece { shape: Shape::Cone, pos: [0.288, 0.064, 0.288], size: [0.064, 0.176, 0.064], rot: [0.0, 0.0, -90.0], mat: Mat::Blanc, id: Some("patte_g") },
    Piece { shape: Shape::Cone, pos: [0.288, 0.064, 0.4], size: [0.064, 0.176, 0.064], rot: [0.0, 0.0, -90.0], mat: Mat::Blanc, id: Some("patte_g") },
    Piece { shape: Shape::Cone, pos: [0.288, 0.064, 0.512], size: [0.064, 0.176, 0.064], rot: [0.0, 0.0, -90.0], mat: Mat::Blanc, id: Some("patte_g") },
    Piece { shape: Shape::Sphere, pos: [-0.32, 0.8, -0.4], size: [0.68, 0.84, 0.4], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("patte_d") },
    Piece { shape: Shape::Cyl, pos: [-0.24, 0.36, -0.4], size: [0.24, 0.64, 0.24], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("patte_d") },
    Piece { shape: Shape::Cube, pos: [-0.096, 0.056, -0.4], size: [0.64, 0.112, 0.336], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: Some("patte_d") },
    Piece { shape: Shape::Cone, pos: [0.288, 0.064, -0.512], size: [0.064, 0.176, 0.064], rot: [0.0, 0.0, -90.0], mat: Mat::Blanc, id: Some("patte_d") },
    Piece { shape: Shape::Cone, pos: [0.288, 0.064, -0.4], size: [0.064, 0.176, 0.064], rot: [0.0, 0.0, -90.0], mat: Mat::Blanc, id: Some("patte_d") },
    Piece { shape: Shape::Cone, pos: [0.288, 0.064, -0.288], size: [0.064, 0.176, 0.064], rot: [0.0, 0.0, -90.0], mat: Mat::Blanc, id: Some("patte_d") },
    Piece { shape: Shape::Sphere, pos: [-1.04, 1.16, 0.0], size: [0.778, 0.576, 0.576], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("queue") },
    Piece { shape: Shape::Sphere, pos: [-1.56, 1.12, 0.0], size: [0.626, 0.464, 0.464], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("queue") },
    Piece { shape: Shape::Sphere, pos: [-2.04, 1.072, 0.0], size: [0.486, 0.36, 0.36], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("queue") },
    Piece { shape: Shape::Sphere, pos: [-2.44, 1.04, 0.0], size: [0.367, 0.272, 0.272], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("queue") },
    Piece { shape: Shape::Sphere, pos: [-2.76, 1.024, 0.0], size: [0.238, 0.176, 0.176], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("queue") },
];
const ANIMS_60: &[Anim] = &[
    Anim { target: "corps", kind: AnimKind::Bob, amp: 0.02 },
    Anim { target: "patte_g", kind: AnimKind::SwingZ, amp: 0.0 },
    Anim { target: "patte_d", kind: AnimKind::SwingZ, amp: 0.5 },
    Anim { target: "bras_g", kind: AnimKind::SwingZ, amp: 0.0 },
    Anim { target: "bras_d", kind: AnimKind::SwingZ, amp: 0.5 },
    Anim { target: "machoire", kind: AnimKind::TailZ, amp: 0.0 },
    Anim { target: "queue", kind: AnimKind::TailY, amp: 0.0 },
    Anim { target: "crete_pointe", kind: AnimKind::Pulse, amp: 0.0 },
];
const PIECES_61: &[Piece] = &[
    Piece { shape: Shape::Cube, pos: [0.0, 1.0, 0.0], size: [0.9, 1.0, 0.9], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Cube, pos: [0.455, 0.95, 0.0], size: [0.03, 0.6, 0.6], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [0.15, 0.5, 0.0], size: [0.6, 0.3, 0.7], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Sphere, pos: [0.45, 1.68, 0.0], size: [0.64, 0.6, 0.66], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Sphere, pos: [0.68, 1.62, 0.0], size: [0.34, 0.36, 0.44], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [0.73, 1.83, 0.0], size: [0.12, 0.07, 0.56], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Sphere, pos: [0.8, 1.72, 0.13], size: [0.1, 0.1, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: None },
    Piece { shape: Shape::Sphere, pos: [0.84, 1.72, 0.13], size: [0.05, 0.05, 0.05], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: None },
    Piece { shape: Shape::Sphere, pos: [0.8, 1.72, -0.13], size: [0.1, 0.1, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: None },
    Piece { shape: Shape::Sphere, pos: [0.84, 1.72, -0.13], size: [0.05, 0.05, 0.05], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: None },
    Piece { shape: Shape::Cube, pos: [0.87, 1.5, 0.0], size: [0.02, 0.05, 0.28], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: None },
    Piece { shape: Shape::Sphere, pos: [0.88, 1.62, 0.06], size: [0.04, 0.05, 0.05], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: None },
    Piece { shape: Shape::Sphere, pos: [0.88, 1.62, -0.06], size: [0.04, 0.05, 0.05], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: None },
    Piece { shape: Shape::Sphere, pos: [0.38, 1.72, 0.36], size: [0.16, 0.16, 0.12], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Sphere, pos: [0.38, 1.72, -0.36], size: [0.16, 0.16, 0.12], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Sphere, pos: [0.0, 1.42, 0.58], size: [0.5, 0.42, 0.42], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Sphere, pos: [0.0, 1.42, -0.58], size: [0.5, 0.42, 0.42], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cyl, pos: [0.0, 0.98, 0.7], size: [0.3, 1.0, 0.3], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("bras_g") },
    Piece { shape: Shape::Sphere, pos: [0.04, 0.46, 0.7], size: [0.38, 0.34, 0.38], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("bras_g") },
    Piece { shape: Shape::Cyl, pos: [0.0, 0.98, -0.7], size: [0.3, 1.0, 0.3], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("bras_d") },
    Piece { shape: Shape::Sphere, pos: [0.04, 0.46, -0.7], size: [0.38, 0.34, 0.38], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("bras_d") },
    Piece { shape: Shape::Cyl, pos: [-0.05, 0.3, 0.3], size: [0.36, 0.6, 0.36], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("patte_g") },
    Piece { shape: Shape::Cube, pos: [0.1, 0.05, 0.3], size: [0.42, 0.1, 0.32], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("patte_g") },
    Piece { shape: Shape::Cyl, pos: [-0.05, 0.3, -0.3], size: [0.36, 0.6, 0.36], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("patte_d") },
    Piece { shape: Shape::Cube, pos: [0.1, 0.05, -0.3], size: [0.42, 0.1, 0.32], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("patte_d") },
];
const ANIMS_61: &[Anim] = &[
    Anim { target: "corps", kind: AnimKind::Bob, amp: 0.03 },
    Anim { target: "patte_g", kind: AnimKind::SwingZ, amp: 0.0 },
    Anim { target: "patte_d", kind: AnimKind::SwingZ, amp: 0.5 },
    Anim { target: "bras_g", kind: AnimKind::SwingZ, amp: 0.5 },
    Anim { target: "bras_d", kind: AnimKind::SwingZ, amp: 0.0 },
];
const PIECES_62: &[Piece] = &[
    Piece { shape: Shape::Sphere, pos: [0.0, 0.9, 0.0], size: [0.78, 0.24, 0.24], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [0.42, 0.93, 0.0], size: [0.24, 0.24, 0.24], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Sphere, pos: [0.52, 0.98, 0.07], size: [0.07, 0.07, 0.07], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: None },
    Piece { shape: Shape::Sphere, pos: [0.52, 0.98, -0.07], size: [0.07, 0.07, 0.07], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: None },
    Piece { shape: Shape::Cyl, pos: [0.6, 1.12, 0.1], size: [0.03, 0.4, 0.03], rot: [0.0, 0.0, -50.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [0.6, 1.12, -0.1], size: [0.03, 0.4, 0.03], rot: [0.0, 0.0, -50.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [0.1, 0.9, 0.7], size: [0.95, 0.04, 1.1], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("aile_g") },
    Piece { shape: Shape::Cube, pos: [-0.35, 0.9, 0.62], size: [0.65, 0.04, 0.8], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("aile_g") },
    Piece { shape: Shape::Sphere, pos: [0.1, 0.93, 0.95], size: [0.3, 0.03, 0.3], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("aile_g") },
    Piece { shape: Shape::Sphere, pos: [-0.35, 0.93, 0.85], size: [0.22, 0.03, 0.22], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("aile_g") },
    Piece { shape: Shape::Cube, pos: [0.62, 0.9, 0.62], size: [0.12, 0.045, 0.9], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("aile_g") },
    Piece { shape: Shape::Cube, pos: [0.1, 0.9, -0.7], size: [0.95, 0.04, 1.1], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("aile_d") },
    Piece { shape: Shape::Cube, pos: [-0.35, 0.9, -0.62], size: [0.65, 0.04, 0.8], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("aile_d") },
    Piece { shape: Shape::Sphere, pos: [0.1, 0.93, -0.95], size: [0.3, 0.03, 0.3], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("aile_d") },
    Piece { shape: Shape::Sphere, pos: [-0.35, 0.93, -0.85], size: [0.22, 0.03, 0.22], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("aile_d") },
    Piece { shape: Shape::Cube, pos: [0.62, 0.9, -0.62], size: [0.12, 0.045, 0.9], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("aile_d") },
];
const ANIMS_62: &[Anim] = &[
    Anim { target: "corps", kind: AnimKind::Bob, amp: 0.04 },
    Anim { target: "aile_g", kind: AnimKind::SwingXBase, amp: 0.0 },
    Anim { target: "aile_d", kind: AnimKind::SwingXBase, amp: 0.0 },
];
const NIPPON_ANIMALS: [Animal; 5] = [
    Animal { model: Model { c2: [255, 194, 212], c3: [255, 255, 255], pieces: PIECES_58, anims: ANIMS_58 }, len: 0.7, width: 0.4, points: 300, dangerous: false, flying: false },
    Animal { model: Model { c2: [154, 123, 90], c3: [244, 227, 176], pieces: PIECES_59, anims: ANIMS_59 }, len: 0.9, width: 0.5, points: 450, dangerous: false, flying: false },
    Animal { model: Model { c2: [141, 147, 156], c3: [47, 123, 224], pieces: PIECES_60, anims: ANIMS_60 }, len: 4.7, width: 1.5, points: 4200, dangerous: true, flying: false },
    Animal { model: Model { c2: [90, 61, 43], c3: [210, 166, 121], pieces: PIECES_61, anims: ANIMS_61 }, len: 1.9, width: 1.8, points: 2200, dangerous: true, flying: false },
    Animal { model: Model { c2: [59, 140, 240], c3: [31, 42, 74], pieces: PIECES_62, anims: ANIMS_62 }, len: 1.9, width: 2.4, points: 3000, dangerous: true, flying: true },
];
fn nippon_animal_name(i: usize) -> &'static str {
    match i {
        0 => tx!("Lapin Mochi", "Mochi Bunny", "Mochi-Häschen", "Coniglio Mochi", "Conejo Mochi").t(),
        1 => tx!("Tanuki Gourmand", "Greedy Tanuki", "Gieriger Tanuki", "Tanuki Goloso", "Tanuki Glotón").t(),
        2 => tx!("Rex Crête-Bleue", "Blue-Crest Rex", "Blaukamm-Rex", "Rex Cresta Blu", "Rex Cresta Azul").t(),
        3 => tx!("Kong-Kong le Costaud", "Big Bongo", "Riesen-Bongo", "Gran Bongo", "Gran Bongo").t(),
        4 => tx!("Papillon Colossal", "Colossal Butterfly", "Riesenfalter", "Farfalla Colossale", "Mariposa Colosal").t(),
        _ => "?",
    }
}
const PIECES_63: &[Piece] = &[
    Piece { shape: Shape::Sphere, pos: [-0.05, 0.26, 0.0], size: [0.5, 0.34, 0.46], rot: [0.0, 0.0, 0.0], mat: Mat::Peinture, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [0.0, 0.17, 0.0], size: [0.38, 0.2, 0.4], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Sphere, pos: [0.2, 0.36, 0.0], size: [0.36, 0.26, 0.4], rot: [0.0, 0.0, 0.0], mat: Mat::Peinture, id: None },
    Piece { shape: Shape::Cube, pos: [0.37, 0.31, 0.0], size: [0.02, 0.02, 0.3], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: None },
    Piece { shape: Shape::Sphere, pos: [0.2, 0.54, 0.14], size: [0.15, 0.15, 0.15], rot: [0.0, 0.0, 0.0], mat: Mat::Peinture, id: None },
    Piece { shape: Shape::Sphere, pos: [0.25, 0.56, 0.14], size: [0.11, 0.11, 0.11], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: None },
    Piece { shape: Shape::Sphere, pos: [0.3, 0.57, 0.14], size: [0.06, 0.06, 0.06], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: None },
    Piece { shape: Shape::Sphere, pos: [-0.2, 0.2, 0.26], size: [0.22, 0.2, 0.14], rot: [0.0, 0.0, 0.0], mat: Mat::Peinture, id: None },
    Piece { shape: Shape::Cube, pos: [-0.1, 0.07, 0.27], size: [0.14, 0.1, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::Peinture, id: None },
    Piece { shape: Shape::Cube, pos: [-0.04, 0.025, 0.27], size: [0.26, 0.05, 0.12], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cyl, pos: [0.18, 0.1, 0.2], size: [0.07, 0.2, 0.07], rot: [0.0, 0.0, 0.0], mat: Mat::Peinture, id: None },
    Piece { shape: Shape::Cube, pos: [0.22, 0.015, 0.2], size: [0.1, 0.03, 0.08], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Sphere, pos: [0.2, 0.54, -0.14], size: [0.15, 0.15, 0.15], rot: [0.0, 0.0, 0.0], mat: Mat::Peinture, id: None },
    Piece { shape: Shape::Sphere, pos: [0.25, 0.56, -0.14], size: [0.11, 0.11, 0.11], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: None },
    Piece { shape: Shape::Sphere, pos: [0.3, 0.57, -0.14], size: [0.06, 0.06, 0.06], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: None },
    Piece { shape: Shape::Sphere, pos: [-0.2, 0.2, -0.26], size: [0.22, 0.2, 0.14], rot: [0.0, 0.0, 0.0], mat: Mat::Peinture, id: None },
    Piece { shape: Shape::Cube, pos: [-0.1, 0.07, -0.27], size: [0.14, 0.1, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::Peinture, id: None },
    Piece { shape: Shape::Cube, pos: [-0.04, 0.025, -0.27], size: [0.26, 0.05, 0.12], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cyl, pos: [0.18, 0.1, -0.2], size: [0.07, 0.2, 0.07], rot: [0.0, 0.0, 0.0], mat: Mat::Peinture, id: None },
    Piece { shape: Shape::Cube, pos: [0.22, 0.015, -0.2], size: [0.1, 0.03, 0.08], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [0.2, 0.45, 0.0], size: [0.4, 0.07, 0.43], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("bandeau") },
    Piece { shape: Shape::Cube, pos: [-0.1, 0.42, 0.1], size: [0.3, 0.04, 0.05], rot: [0.0, 0.0, -10.0], mat: Mat::C3, id: Some("bandeau") },
    Piece { shape: Shape::Cube, pos: [-0.12, 0.42, -0.1], size: [0.3, 0.04, 0.05], rot: [0.0, 0.0, -18.0], mat: Mat::C3, id: Some("bandeau") },
    Piece { shape: Shape::Cube, pos: [0.28, 0.46, 0.0], size: [0.02, 0.06, 0.12], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: None },
];
const ANIMS_63: &[Anim] = &[
    Anim { target: "corps", kind: AnimKind::Bob, amp: 0.02 },
    Anim { target: "bandeau", kind: AnimKind::TailZ, amp: 0.0 },
];
const PIECES_64: &[Piece] = &[
    Piece { shape: Shape::Cone, pos: [0.0, 0.72, 0.0], size: [0.7, 0.22, 0.7], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cyl, pos: [0.0, 0.62, 0.0], size: [0.3, 0.02, 0.3], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Sphere, pos: [0.0, 0.84, 0.0], size: [0.07, 0.07, 0.07], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
];
const ANIMS_64: &[Anim] = &[
];
const NIPPON_TRAIN: [TrainCar; 0] = [
];
const NIPPON_COLORS: WorldColors = WorldColors {
    grass_a: [217, 205, 176],
    grass_b: [207, 194, 162],
    water: [63, 127, 191],
    road: [35, 36, 42],
    marking: [255, 255, 255],
    log: [138, 90, 43],
    sky: [246, 209, 220],
};
fn nippon_name() -> &'static str { tx!("Nippon Kaboom", "Nippon Kaboom", "Nippon Bumm", "Nippon Bum", "Nipón Bum").t() }
fn nippon_banner() -> &'static str { tx!("Bienvenue à Nippon Kaboom ! Merci d'enlever vos chaussures, et de ne pas réveiller le crabe.", "Welcome to Nippon Kaboom! Please take off your shoes, and do not wake the crab.", "Willkommen in Nippon Bumm! Bitte Schuhe ausziehen und den Krebs nicht wecken.", "Benvenuto a Nippon Bum! Per favore, togliti le scarpe e non svegliare il granchio.", "¡Bienvenido a Nipón Bum! Por favor, quítate los zapatos y no despiertes al cangrejo.").t() }
fn nippon_hero_name() -> &'static str { tx!("Grenouille Ninja", "Ninja Frog", "Ninja-Frosch", "Rana Ninja", "Rana Ninja").t() }
pub static NIPPON_THEME: Theme = Theme {
    train: &NIPPON_TRAIN, only_own: false,
    blocks: &NIPPON_BLOCKS, sides: &NIPPON_SIDES, deco: &NIPPON_DECO, animals: &NIPPON_ANIMALS,
    block_mix: &[0, 0, 0, 0, 0, 0, 3, 3, 3, 3, 3, 1, 1, 1, 5, 5, 5, 6, 2, 4], side_mix: &[100, 100, 100, 100, 100, 100, 100, 100, 103, 103, 103, 103, 103, 101, 101, 101, 2, 2, 5, 0],
    side_pct: 8, vehicles: &[36, 37, 38, 39, 40, 41], colors: NIPPON_COLORS, points: 1800,
    name: nippon_name, banner: nippon_banner, animal_name: nippon_animal_name, hero_name: nippon_hero_name,
    hero: Model { c2: [242, 140, 40], c3: [200, 40, 60], pieces: PIECES_63, anims: ANIMS_63 }, hat: Model { c2: [242, 140, 40], c3: [200, 40, 60], pieces: PIECES_64, anims: ANIMS_64 },
};
const PIECES_65: &[Piece] = &[
    Piece { shape: Shape::Cyl, pos: [0.0, 0.55, 0.0], size: [0.28, 1.1, 0.28], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Sphere, pos: [0.0, 1.12, 0.0], size: [0.28, 0.2, 0.28], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [0.2, 0.62, 0.0], size: [0.28, 0.14, 0.14], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cyl, pos: [0.32, 0.88, 0.0], size: [0.14, 0.5, 0.14], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [-0.2, 0.45, 0.0], size: [0.28, 0.14, 0.14], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.32, 0.68, 0.0], size: [0.14, 0.42, 0.14], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Sphere, pos: [0.0, 1.24, 0.0], size: [0.1, 0.1, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
];
const ANIMS_65: &[Anim] = &[
];
const PIECES_66: &[Piece] = &[
    Piece { shape: Shape::Sphere, pos: [0.0, 0.35, 0.0], size: [0.95, 0.7, 0.8], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Sphere, pos: [0.25, 0.62, 0.1], size: [0.55, 0.5, 0.5], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Sphere, pos: [-0.3, 0.2, -0.2], size: [0.5, 0.4, 0.45], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
];
const ANIMS_66: &[Anim] = &[
];
const PIECES_67: &[Piece] = &[
    Piece { shape: Shape::Cyl, pos: [-0.1, 0.3, 0.0], size: [0.55, 0.6, 0.55], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.1, 0.18, 0.0], size: [0.58, 0.05, 0.58], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.1, 0.44, 0.0], size: [0.58, 0.05, 0.58], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cube, pos: [0.3, 0.2, 0.2], size: [0.4, 0.4, 0.4], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [0.3, 0.2, 0.2], size: [0.42, 0.05, 0.42], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
];
const ANIMS_67: &[Anim] = &[
];
const PIECES_68: &[Piece] = &[
    Piece { shape: Shape::Cyl, pos: [0.0, 0.35, 0.0], size: [0.1, 0.7, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.04, 0.0], size: [0.4, 0.08, 0.4], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Sphere, pos: [0.04, 0.74, 0.0], size: [0.36, 0.28, 0.3], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Sphere, pos: [0.2, 0.68, 0.0], size: [0.18, 0.16, 0.2], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cone, pos: [0.0, 0.86, 0.22], size: [0.08, 0.3, 0.08], rot: [50.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cone, pos: [0.0, 0.86, -0.22], size: [0.08, 0.3, 0.08], rot: [-50.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Sphere, pos: [0.12, 0.78, 0.09], size: [0.07, 0.07, 0.04], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: None },
    Piece { shape: Shape::Sphere, pos: [0.12, 0.78, -0.09], size: [0.07, 0.07, 0.04], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: None },
];
const ANIMS_68: &[Anim] = &[
];
const PIECES_69: &[Piece] = &[
    Piece { shape: Shape::Cyl, pos: [0.0, 0.32, 0.0], size: [0.7, 0.7, 0.7], rot: [90.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cyl, pos: [0.0, 0.32, 0.12], size: [0.72, 0.04, 0.72], rot: [90.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [0.0, 0.32, -0.12], size: [0.72, 0.04, 0.72], rot: [90.0, 0.0, 0.0], mat: Mat::C3, id: None },
];
const ANIMS_69: &[Anim] = &[
];
const FARWEST_BLOCKS: [Model; 5] = [
    Model { c2: [63, 143, 74], c3: [240, 111, 168], pieces: PIECES_65, anims: ANIMS_65 },
    Model { c2: [181, 101, 58], c3: [138, 74, 42], pieces: PIECES_66, anims: ANIMS_66 },
    Model { c2: [201, 160, 99], c3: [122, 77, 36], pieces: PIECES_67, anims: ANIMS_67 },
    Model { c2: [241, 234, 216], c3: [122, 77, 36], pieces: PIECES_68, anims: ANIMS_68 },
    Model { c2: [230, 197, 102], c3: [169, 135, 55], pieces: PIECES_69, anims: ANIMS_69 },
];
const PIECES_70: &[Piece] = &[
    Piece { shape: Shape::Cube, pos: [0.0, 1.0, 0.0], size: [2.4, 2.0, 1.8], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [0.1, 2.3, 0.0], size: [1.8, 0.6, 1.4], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [0.2, 2.8, 0.0], size: [1.2, 0.4, 1.0], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [-0.4, 0.4, 1.0], size: [1.2, 0.8, 0.5], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
];
const ANIMS_70: &[Anim] = &[
];
const PIECES_71: &[Piece] = &[
    Piece { shape: Shape::Cube, pos: [0.0, 0.7, 0.0], size: [1.8, 1.4, 1.2], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 1.65, 0.0], size: [1.8, 0.7, 1.2], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 1.4, 0.64], size: [1.9, 0.08, 0.2], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [0.5, 0.8, 0.62], size: [0.4, 0.6, 0.04], rot: [0.0, 0.0, 0.0], mat: Mat::Vitre, id: None },
    Piece { shape: Shape::Cube, pos: [-0.5, 0.8, 0.62], size: [0.4, 0.6, 0.04], rot: [0.0, 0.0, 0.0], mat: Mat::Vitre, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.5, 0.62], size: [0.3, 0.9, 0.04], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 1.95, 0.4], size: [1.2, 0.4, 0.06], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [-0.85, 0.35, 0.7], size: [0.08, 0.7, 0.08], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cube, pos: [0.85, 0.35, 0.7], size: [0.08, 0.7, 0.08], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
];
const ANIMS_71: &[Anim] = &[
];
const PIECES_72: &[Piece] = &[
    Piece { shape: Shape::Cyl, pos: [0.5, 1.0, 0.5], size: [0.1, 2.0, 0.1], rot: [0.0, 0.0, 6.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.5, 1.0, 0.5], size: [0.1, 2.0, 0.1], rot: [0.0, 0.0, -6.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cyl, pos: [0.5, 1.0, -0.5], size: [0.1, 2.0, 0.1], rot: [0.0, 0.0, 6.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.5, 1.0, -0.5], size: [0.1, 2.0, 0.1], rot: [0.0, 0.0, -6.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cyl, pos: [0.0, 2.5, 0.0], size: [1.3, 1.1, 1.3], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cone, pos: [0.0, 3.3, 0.0], size: [1.4, 0.6, 1.4], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.05, 0.0], size: [1.4, 0.1, 1.4], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
];
const ANIMS_72: &[Anim] = &[
];
const PIECES_73: &[Piece] = &[
    Piece { shape: Shape::Cyl, pos: [0.0, 1.4, 0.0], size: [0.12, 2.8, 0.12], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 2.6, 0.0], size: [0.1, 0.1, 1.0], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 2.2, 0.0], size: [0.1, 0.1, 0.8], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Sphere, pos: [0.0, 2.68, 0.45], size: [0.08, 0.08, 0.08], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: None },
    Piece { shape: Shape::Sphere, pos: [0.0, 2.68, -0.45], size: [0.08, 0.08, 0.08], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: None },
];
const ANIMS_73: &[Anim] = &[
];
const FARWEST_SIDES: [Model; 4] = [
    Model { c2: [196, 103, 58], c3: [162, 79, 42], pieces: PIECES_70, anims: ANIMS_70 },
    Model { c2: [160, 105, 58], c3: [232, 217, 168], pieces: PIECES_71, anims: ANIMS_71 },
    Model { c2: [138, 90, 43], c3: [201, 160, 99], pieces: PIECES_72, anims: ANIMS_72 },
    Model { c2: [122, 77, 36], c3: [74, 48, 24], pieces: PIECES_73, anims: ANIMS_73 },
];
const PIECES_74: &[Piece] = &[
    Piece { shape: Shape::Cyl, pos: [0.0, 0.14, 0.0], size: [0.12, 0.28, 0.12], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Sphere, pos: [0.0, 0.3, 0.0], size: [0.07, 0.07, 0.07], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
];
const ANIMS_74: &[Anim] = &[
];
const PIECES_75: &[Piece] = &[
    Piece { shape: Shape::Cube, pos: [0.0, 0.03, 0.0], size: [0.3, 0.05, 0.07], rot: [0.0, 30.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Sphere, pos: [0.15, 0.04, 0.05], size: [0.1, 0.07, 0.09], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Sphere, pos: [-0.15, 0.04, -0.05], size: [0.1, 0.07, 0.09], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
];
const ANIMS_75: &[Anim] = &[
];
const PIECES_76: &[Piece] = &[
    Piece { shape: Shape::Cube, pos: [-0.06, 0.02, 0.07], size: [0.05, 0.03, 0.14], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [-0.06, 0.02, -0.07], size: [0.05, 0.03, 0.14], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [0.02, 0.02, 0.0], size: [0.05, 0.03, 0.2], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
];
const ANIMS_76: &[Anim] = &[
];
const PIECES_77: &[Piece] = &[
    Piece { shape: Shape::Sphere, pos: [0.0, 0.06, 0.0], size: [0.2, 0.12, 0.18], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Sphere, pos: [0.14, 0.04, 0.08], size: [0.1, 0.08, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
];
const ANIMS_77: &[Anim] = &[
];
const FARWEST_DECO: [Model; 4] = [
    Model { c2: [63, 143, 74], c3: [240, 111, 168], pieces: PIECES_74, anims: ANIMS_74 },
    Model { c2: [241, 234, 216], c3: [216, 208, 188], pieces: PIECES_75, anims: ANIMS_75 },
    Model { c2: [136, 144, 160], c3: [90, 96, 112], pieces: PIECES_76, anims: ANIMS_76 },
    Model { c2: [181, 154, 120], c3: [138, 115, 88], pieces: PIECES_77, anims: ANIMS_77 },
];
const PIECES_78: &[Piece] = &[
    Piece { shape: Shape::Sphere, pos: [0.0, 0.3, 0.0], size: [0.55, 0.5, 0.5], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("boule") },
    Piece { shape: Shape::Sphere, pos: [0.1, 0.35, 0.05], size: [0.4, 0.38, 0.4], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("boule") },
    Piece { shape: Shape::Sphere, pos: [-0.1, 0.25, -0.05], size: [0.38, 0.36, 0.36], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("boule") },
    Piece { shape: Shape::Cube, pos: [0.0, 0.3, 0.0], size: [0.7, 0.04, 0.04], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("boule") },
    Piece { shape: Shape::Cube, pos: [0.0, 0.3, 0.0], size: [0.04, 0.04, 0.7], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("boule") },
];
const ANIMS_78: &[Anim] = &[
    Anim { target: "boule", kind: AnimKind::SpinGroup, amp: 0.0 },
];
const PIECES_79: &[Piece] = &[
    Piece { shape: Shape::Sphere, pos: [0.0, 0.42, 0.0], size: [0.62, 0.3, 0.26], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [0.32, 0.5, 0.0], size: [0.28, 0.24, 0.22], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Cone, pos: [0.52, 0.48, 0.0], size: [0.12, 0.26, 0.12], rot: [0.0, 0.0, -90.0], mat: Mat::C3, id: Some("corps") },
    Piece { shape: Shape::Cone, pos: [0.3, 0.68, 0.07], size: [0.07, 0.14, 0.05], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Cone, pos: [0.3, 0.68, -0.07], size: [0.07, 0.14, 0.05], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [0.42, 0.55, 0.08], size: [0.05, 0.05, 0.05], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [0.42, 0.55, -0.08], size: [0.05, 0.05, 0.05], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [-0.4, 0.42, 0.0], size: [0.4, 0.18, 0.18], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("queue") },
    Piece { shape: Shape::Cyl, pos: [0.2, 0.18, 0.08], size: [0.07, 0.36, 0.07], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("patte0") },
    Piece { shape: Shape::Cyl, pos: [0.2, 0.18, -0.08], size: [0.07, 0.36, 0.07], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("patte1") },
    Piece { shape: Shape::Cyl, pos: [-0.2, 0.18, 0.08], size: [0.07, 0.36, 0.07], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("patte2") },
    Piece { shape: Shape::Cyl, pos: [-0.2, 0.18, -0.08], size: [0.07, 0.36, 0.07], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("patte3") },
];
const ANIMS_79: &[Anim] = &[
    Anim { target: "corps", kind: AnimKind::Bob, amp: 0.03 },
    Anim { target: "queue", kind: AnimKind::TailY, amp: 0.0 },
    Anim { target: "patte0", kind: AnimKind::SwingZ, amp: 0.0 },
    Anim { target: "patte1", kind: AnimKind::SwingZ, amp: 0.5 },
    Anim { target: "patte2", kind: AnimKind::SwingZ, amp: 0.5 },
    Anim { target: "patte3", kind: AnimKind::SwingZ, amp: 0.0 },
];
const PIECES_80: &[Piece] = &[
    Piece { shape: Shape::Sphere, pos: [0.0, 0.78, 0.0], size: [1.05, 0.52, 0.46], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [0.3, 0.84, 0.0], size: [0.55, 0.58, 0.44], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [-0.32, 0.8, 0.0], size: [0.55, 0.55, 0.46], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [0.58, 1.05, 0.0], size: [0.3, 0.6, 0.26], rot: [0.0, 0.0, -32.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [0.84, 1.22, 0.0], size: [0.42, 0.26, 0.22], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [1.02, 1.15, 0.0], size: [0.24, 0.2, 0.18], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("corps") },
    Piece { shape: Shape::Cone, pos: [0.78, 1.42, 0.07], size: [0.07, 0.14, 0.05], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Cone, pos: [0.78, 1.42, -0.07], size: [0.07, 0.14, 0.05], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [0.92, 1.27, 0.1], size: [0.05, 0.05, 0.05], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [0.92, 1.27, -0.1], size: [0.05, 0.05, 0.05], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("corps") },
    Piece { shape: Shape::Cube, pos: [0.35, 1.15, 0.0], size: [0.1, 0.22, 0.06], rot: [0.0, 0.0, -30.0], mat: Mat::C3, id: Some("corps") },
    Piece { shape: Shape::Cube, pos: [0.5, 1.21, 0.0], size: [0.1, 0.22, 0.06], rot: [0.0, 0.0, -30.0], mat: Mat::C3, id: Some("corps") },
    Piece { shape: Shape::Cube, pos: [0.62, 1.258, 0.0], size: [0.1, 0.22, 0.06], rot: [0.0, 0.0, -30.0], mat: Mat::C3, id: Some("corps") },
    Piece { shape: Shape::Cone, pos: [-0.62, 0.78, 0.0], size: [0.12, 0.55, 0.12], rot: [0.0, 0.0, 110.0], mat: Mat::C3, id: Some("queue") },
    Piece { shape: Shape::Cyl, pos: [0.4, 0.34, 0.14], size: [0.12, 0.7, 0.12], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("patte0") },
    Piece { shape: Shape::Cube, pos: [0.43, 0.03, 0.14], size: [0.18, 0.06, 0.14], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("patte0") },
    Piece { shape: Shape::Cyl, pos: [0.4, 0.34, -0.14], size: [0.12, 0.7, 0.12], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("patte1") },
    Piece { shape: Shape::Cube, pos: [0.43, 0.03, -0.14], size: [0.18, 0.06, 0.14], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("patte1") },
    Piece { shape: Shape::Cyl, pos: [-0.4, 0.34, 0.14], size: [0.12, 0.7, 0.12], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("patte2") },
    Piece { shape: Shape::Cube, pos: [-0.37, 0.03, 0.14], size: [0.18, 0.06, 0.14], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("patte2") },
    Piece { shape: Shape::Cyl, pos: [-0.4, 0.34, -0.14], size: [0.12, 0.7, 0.12], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("patte3") },
    Piece { shape: Shape::Cube, pos: [-0.37, 0.03, -0.14], size: [0.18, 0.06, 0.14], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("patte3") },
];
const ANIMS_80: &[Anim] = &[
    Anim { target: "corps", kind: AnimKind::Bob, amp: 0.05 },
    Anim { target: "queue", kind: AnimKind::TailY, amp: 0.0 },
    Anim { target: "patte0", kind: AnimKind::SwingZ, amp: 0.0 },
    Anim { target: "patte1", kind: AnimKind::SwingZ, amp: 0.5 },
    Anim { target: "patte2", kind: AnimKind::SwingZ, amp: 0.5 },
    Anim { target: "patte3", kind: AnimKind::SwingZ, amp: 0.0 },
];
const PIECES_81: &[Piece] = &[
    Piece { shape: Shape::Sphere, pos: [0.0, 0.78, 0.0], size: [1.05, 0.52, 0.46], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [0.3, 0.84, 0.0], size: [0.55, 0.58, 0.44], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [-0.32, 0.8, 0.0], size: [0.55, 0.55, 0.46], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [0.58, 1.05, 0.0], size: [0.3, 0.6, 0.26], rot: [0.0, 0.0, -32.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [0.84, 1.22, 0.0], size: [0.42, 0.26, 0.22], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [1.02, 1.15, 0.0], size: [0.24, 0.2, 0.18], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("corps") },
    Piece { shape: Shape::Cone, pos: [0.78, 1.42, 0.07], size: [0.07, 0.14, 0.05], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Cone, pos: [0.78, 1.42, -0.07], size: [0.07, 0.14, 0.05], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [0.92, 1.27, 0.1], size: [0.05, 0.05, 0.05], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [0.92, 1.27, -0.1], size: [0.05, 0.05, 0.05], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("corps") },
    Piece { shape: Shape::Cube, pos: [0.35, 1.15, 0.0], size: [0.1, 0.22, 0.06], rot: [0.0, 0.0, -30.0], mat: Mat::C3, id: Some("corps") },
    Piece { shape: Shape::Cube, pos: [0.5, 1.21, 0.0], size: [0.1, 0.22, 0.06], rot: [0.0, 0.0, -30.0], mat: Mat::C3, id: Some("corps") },
    Piece { shape: Shape::Cube, pos: [0.62, 1.258, 0.0], size: [0.1, 0.22, 0.06], rot: [0.0, 0.0, -30.0], mat: Mat::C3, id: Some("corps") },
    Piece { shape: Shape::Cone, pos: [-0.62, 0.78, 0.0], size: [0.12, 0.55, 0.12], rot: [0.0, 0.0, 110.0], mat: Mat::C3, id: Some("queue") },
    Piece { shape: Shape::Cyl, pos: [0.4, 0.34, 0.14], size: [0.12, 0.7, 0.12], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("patte0") },
    Piece { shape: Shape::Cube, pos: [0.43, 0.03, 0.14], size: [0.18, 0.06, 0.14], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("patte0") },
    Piece { shape: Shape::Cyl, pos: [0.4, 0.34, -0.14], size: [0.12, 0.7, 0.12], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("patte1") },
    Piece { shape: Shape::Cube, pos: [0.43, 0.03, -0.14], size: [0.18, 0.06, 0.14], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("patte1") },
    Piece { shape: Shape::Cyl, pos: [-0.4, 0.34, 0.14], size: [0.12, 0.7, 0.12], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("patte2") },
    Piece { shape: Shape::Cube, pos: [-0.37, 0.03, 0.14], size: [0.18, 0.06, 0.14], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("patte2") },
    Piece { shape: Shape::Cyl, pos: [-0.4, 0.34, -0.14], size: [0.12, 0.7, 0.12], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("patte3") },
    Piece { shape: Shape::Cube, pos: [-0.37, 0.03, -0.14], size: [0.18, 0.06, 0.14], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("patte3") },
    Piece { shape: Shape::Cube, pos: [-0.02, 1.32, 0.0], size: [0.3, 0.42, 0.3], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [0.02, 1.68, 0.0], size: [0.22, 0.24, 0.22], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Cube, pos: [0.02, 1.84, 0.0], size: [0.46, 0.04, 0.46], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: Some("corps") },
    Piece { shape: Shape::Cyl, pos: [0.02, 1.92, 0.0], size: [0.24, 0.16, 0.24], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: Some("corps") },
    Piece { shape: Shape::Cube, pos: [0.08, 1.12, 0.22], size: [0.1, 0.4, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: Some("corps") },
    Piece { shape: Shape::Cube, pos: [0.08, 1.12, -0.22], size: [0.1, 0.4, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: Some("corps") },
];
const ANIMS_81: &[Anim] = &[
    Anim { target: "corps", kind: AnimKind::Bob, amp: 0.05 },
    Anim { target: "queue", kind: AnimKind::TailY, amp: 0.0 },
    Anim { target: "patte0", kind: AnimKind::SwingZ, amp: 0.0 },
    Anim { target: "patte1", kind: AnimKind::SwingZ, amp: 0.5 },
    Anim { target: "patte2", kind: AnimKind::SwingZ, amp: 0.5 },
    Anim { target: "patte3", kind: AnimKind::SwingZ, amp: 0.0 },
];
const PIECES_82: &[Piece] = &[
    Piece { shape: Shape::Sphere, pos: [0.0, 0.85, 0.0], size: [1.3, 0.85, 0.85], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [0.38, 1.18, 0.0], size: [0.85, 0.75, 0.82], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [0.78, 1.0, 0.0], size: [0.55, 0.7, 0.6], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [1.0, 0.72, 0.0], size: [0.46, 0.52, 0.46], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("corps") },
    Piece { shape: Shape::Cone, pos: [0.98, 0.42, 0.0], size: [0.12, 0.3, 0.12], rot: [0.0, 0.0, 180.0], mat: Mat::C3, id: Some("corps") },
    Piece { shape: Shape::Cone, pos: [0.98, 0.95, 0.26], size: [0.08, 0.3, 0.08], rot: [50.0, 0.0, 0.0], mat: Mat::Blanc, id: Some("corps") },
    Piece { shape: Shape::Cone, pos: [0.98, 0.95, -0.26], size: [0.08, 0.3, 0.08], rot: [-50.0, 0.0, 0.0], mat: Mat::Blanc, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [1.14, 0.8, 0.14], size: [0.06, 0.06, 0.06], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [1.14, 0.8, -0.14], size: [0.06, 0.06, 0.06], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("corps") },
    Piece { shape: Shape::Cone, pos: [-0.62, 0.7, 0.0], size: [0.1, 0.5, 0.1], rot: [0.0, 0.0, 160.0], mat: Mat::C3, id: Some("queue") },
    Piece { shape: Shape::Cyl, pos: [0.4, 0.3, 0.22], size: [0.2, 0.6, 0.2], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("patte0") },
    Piece { shape: Shape::Cube, pos: [0.43, 0.03, 0.22], size: [0.24, 0.06, 0.2], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("patte0") },
    Piece { shape: Shape::Cyl, pos: [0.4, 0.3, -0.22], size: [0.2, 0.6, 0.2], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("patte1") },
    Piece { shape: Shape::Cube, pos: [0.43, 0.03, -0.22], size: [0.24, 0.06, 0.2], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("patte1") },
    Piece { shape: Shape::Cyl, pos: [-0.4, 0.3, 0.22], size: [0.2, 0.6, 0.2], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("patte2") },
    Piece { shape: Shape::Cube, pos: [-0.37, 0.03, 0.22], size: [0.24, 0.06, 0.2], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("patte2") },
    Piece { shape: Shape::Cyl, pos: [-0.4, 0.3, -0.22], size: [0.2, 0.6, 0.2], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("patte3") },
    Piece { shape: Shape::Cube, pos: [-0.37, 0.03, -0.22], size: [0.24, 0.06, 0.2], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("patte3") },
];
const ANIMS_82: &[Anim] = &[
    Anim { target: "corps", kind: AnimKind::Bob, amp: 0.04 },
    Anim { target: "queue", kind: AnimKind::TailY, amp: 0.0 },
    Anim { target: "patte0", kind: AnimKind::SwingZ, amp: 0.0 },
    Anim { target: "patte1", kind: AnimKind::SwingZ, amp: 0.5 },
    Anim { target: "patte2", kind: AnimKind::SwingZ, amp: 0.5 },
    Anim { target: "patte3", kind: AnimKind::SwingZ, amp: 0.0 },
];
const PIECES_83: &[Piece] = &[
    Piece { shape: Shape::Sphere, pos: [1.9, 0.85, 0.25], size: [1.3, 0.85, 0.85], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("b0corps") },
    Piece { shape: Shape::Sphere, pos: [2.28, 1.18, 0.25], size: [0.85, 0.75, 0.82], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("b0corps") },
    Piece { shape: Shape::Sphere, pos: [2.68, 1.0, 0.25], size: [0.55, 0.7, 0.6], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("b0corps") },
    Piece { shape: Shape::Sphere, pos: [2.9, 0.72, 0.25], size: [0.46, 0.52, 0.46], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("b0corps") },
    Piece { shape: Shape::Cone, pos: [2.88, 0.42, 0.25], size: [0.12, 0.3, 0.12], rot: [0.0, 0.0, 180.0], mat: Mat::C3, id: Some("b0corps") },
    Piece { shape: Shape::Cone, pos: [2.88, 0.95, 0.51], size: [0.08, 0.3, 0.08], rot: [50.0, 0.0, 0.0], mat: Mat::Blanc, id: Some("b0corps") },
    Piece { shape: Shape::Cone, pos: [2.88, 0.95, -0.01], size: [0.08, 0.3, 0.08], rot: [-50.0, 0.0, 0.0], mat: Mat::Blanc, id: Some("b0corps") },
    Piece { shape: Shape::Sphere, pos: [3.04, 0.8, 0.39], size: [0.06, 0.06, 0.06], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("b0corps") },
    Piece { shape: Shape::Sphere, pos: [3.04, 0.8, 0.11], size: [0.06, 0.06, 0.06], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("b0corps") },
    Piece { shape: Shape::Cone, pos: [1.28, 0.7, 0.25], size: [0.1, 0.5, 0.1], rot: [0.0, 0.0, 160.0], mat: Mat::C3, id: Some("b0queue") },
    Piece { shape: Shape::Cyl, pos: [2.3, 0.3, 0.47], size: [0.2, 0.6, 0.2], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("b0patte0") },
    Piece { shape: Shape::Cube, pos: [2.33, 0.03, 0.47], size: [0.24, 0.06, 0.2], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("b0patte0") },
    Piece { shape: Shape::Cyl, pos: [2.3, 0.3, 0.03], size: [0.2, 0.6, 0.2], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("b0patte1") },
    Piece { shape: Shape::Cube, pos: [2.33, 0.03, 0.03], size: [0.24, 0.06, 0.2], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("b0patte1") },
    Piece { shape: Shape::Cyl, pos: [1.5, 0.3, 0.47], size: [0.2, 0.6, 0.2], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("b0patte2") },
    Piece { shape: Shape::Cube, pos: [1.53, 0.03, 0.47], size: [0.24, 0.06, 0.2], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("b0patte2") },
    Piece { shape: Shape::Cyl, pos: [1.5, 0.3, 0.03], size: [0.2, 0.6, 0.2], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("b0patte3") },
    Piece { shape: Shape::Cube, pos: [1.53, 0.03, 0.03], size: [0.24, 0.06, 0.2], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("b0patte3") },
    Piece { shape: Shape::Sphere, pos: [0.0, 0.85, -0.3], size: [1.3, 0.85, 0.85], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("b1corps") },
    Piece { shape: Shape::Sphere, pos: [0.38, 1.18, -0.3], size: [0.85, 0.75, 0.82], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("b1corps") },
    Piece { shape: Shape::Sphere, pos: [0.78, 1.0, -0.3], size: [0.55, 0.7, 0.6], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("b1corps") },
    Piece { shape: Shape::Sphere, pos: [1.0, 0.72, -0.3], size: [0.46, 0.52, 0.46], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("b1corps") },
    Piece { shape: Shape::Cone, pos: [0.98, 0.42, -0.3], size: [0.12, 0.3, 0.12], rot: [0.0, 0.0, 180.0], mat: Mat::C3, id: Some("b1corps") },
    Piece { shape: Shape::Cone, pos: [0.98, 0.95, -0.04], size: [0.08, 0.3, 0.08], rot: [50.0, 0.0, 0.0], mat: Mat::Blanc, id: Some("b1corps") },
    Piece { shape: Shape::Cone, pos: [0.98, 0.95, -0.56], size: [0.08, 0.3, 0.08], rot: [-50.0, 0.0, 0.0], mat: Mat::Blanc, id: Some("b1corps") },
    Piece { shape: Shape::Sphere, pos: [1.14, 0.8, -0.16], size: [0.06, 0.06, 0.06], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("b1corps") },
    Piece { shape: Shape::Sphere, pos: [1.14, 0.8, -0.44], size: [0.06, 0.06, 0.06], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("b1corps") },
    Piece { shape: Shape::Cone, pos: [-0.62, 0.7, -0.3], size: [0.1, 0.5, 0.1], rot: [0.0, 0.0, 160.0], mat: Mat::C3, id: Some("b1queue") },
    Piece { shape: Shape::Cyl, pos: [0.4, 0.3, -0.08], size: [0.2, 0.6, 0.2], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("b1patte0") },
    Piece { shape: Shape::Cube, pos: [0.43, 0.03, -0.08], size: [0.24, 0.06, 0.2], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("b1patte0") },
    Piece { shape: Shape::Cyl, pos: [0.4, 0.3, -0.52], size: [0.2, 0.6, 0.2], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("b1patte1") },
    Piece { shape: Shape::Cube, pos: [0.43, 0.03, -0.52], size: [0.24, 0.06, 0.2], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("b1patte1") },
    Piece { shape: Shape::Cyl, pos: [-0.4, 0.3, -0.08], size: [0.2, 0.6, 0.2], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("b1patte2") },
    Piece { shape: Shape::Cube, pos: [-0.37, 0.03, -0.08], size: [0.24, 0.06, 0.2], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("b1patte2") },
    Piece { shape: Shape::Cyl, pos: [-0.4, 0.3, -0.52], size: [0.2, 0.6, 0.2], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("b1patte3") },
    Piece { shape: Shape::Cube, pos: [-0.37, 0.03, -0.52], size: [0.24, 0.06, 0.2], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("b1patte3") },
    Piece { shape: Shape::Sphere, pos: [-1.9, 0.85, 0.2], size: [1.3, 0.85, 0.85], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("b2corps") },
    Piece { shape: Shape::Sphere, pos: [-1.52, 1.18, 0.2], size: [0.85, 0.75, 0.82], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("b2corps") },
    Piece { shape: Shape::Sphere, pos: [-1.12, 1.0, 0.2], size: [0.55, 0.7, 0.6], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("b2corps") },
    Piece { shape: Shape::Sphere, pos: [-0.9, 0.72, 0.2], size: [0.46, 0.52, 0.46], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("b2corps") },
    Piece { shape: Shape::Cone, pos: [-0.92, 0.42, 0.2], size: [0.12, 0.3, 0.12], rot: [0.0, 0.0, 180.0], mat: Mat::C3, id: Some("b2corps") },
    Piece { shape: Shape::Cone, pos: [-0.92, 0.95, 0.46], size: [0.08, 0.3, 0.08], rot: [50.0, 0.0, 0.0], mat: Mat::Blanc, id: Some("b2corps") },
    Piece { shape: Shape::Cone, pos: [-0.92, 0.95, -0.06], size: [0.08, 0.3, 0.08], rot: [-50.0, 0.0, 0.0], mat: Mat::Blanc, id: Some("b2corps") },
    Piece { shape: Shape::Sphere, pos: [-0.76, 0.8, 0.34], size: [0.06, 0.06, 0.06], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("b2corps") },
    Piece { shape: Shape::Sphere, pos: [-0.76, 0.8, 0.06], size: [0.06, 0.06, 0.06], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("b2corps") },
    Piece { shape: Shape::Cone, pos: [-2.52, 0.7, 0.2], size: [0.1, 0.5, 0.1], rot: [0.0, 0.0, 160.0], mat: Mat::C3, id: Some("b2queue") },
    Piece { shape: Shape::Cyl, pos: [-1.5, 0.3, 0.42], size: [0.2, 0.6, 0.2], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("b2patte0") },
    Piece { shape: Shape::Cube, pos: [-1.47, 0.03, 0.42], size: [0.24, 0.06, 0.2], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("b2patte0") },
    Piece { shape: Shape::Cyl, pos: [-1.5, 0.3, -0.02], size: [0.2, 0.6, 0.2], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("b2patte1") },
    Piece { shape: Shape::Cube, pos: [-1.47, 0.03, -0.02], size: [0.24, 0.06, 0.2], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("b2patte1") },
    Piece { shape: Shape::Cyl, pos: [-2.3, 0.3, 0.42], size: [0.2, 0.6, 0.2], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("b2patte2") },
    Piece { shape: Shape::Cube, pos: [-2.27, 0.03, 0.42], size: [0.24, 0.06, 0.2], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("b2patte2") },
    Piece { shape: Shape::Cyl, pos: [-2.3, 0.3, -0.02], size: [0.2, 0.6, 0.2], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("b2patte3") },
    Piece { shape: Shape::Cube, pos: [-2.27, 0.03, -0.02], size: [0.24, 0.06, 0.2], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("b2patte3") },
];
const ANIMS_83: &[Anim] = &[
    Anim { target: "b0corps", kind: AnimKind::Bob, amp: 0.04 },
    Anim { target: "b0queue", kind: AnimKind::TailY, amp: 0.0 },
    Anim { target: "b0patte0", kind: AnimKind::SwingZ, amp: 0.0 },
    Anim { target: "b0patte1", kind: AnimKind::SwingZ, amp: 0.5 },
    Anim { target: "b0patte2", kind: AnimKind::SwingZ, amp: 0.5 },
    Anim { target: "b0patte3", kind: AnimKind::SwingZ, amp: 0.0 },
    Anim { target: "b1corps", kind: AnimKind::Bob, amp: 0.04 },
    Anim { target: "b1queue", kind: AnimKind::TailY, amp: 0.0 },
    Anim { target: "b1patte0", kind: AnimKind::SwingZ, amp: 0.0 },
    Anim { target: "b1patte1", kind: AnimKind::SwingZ, amp: 0.5 },
    Anim { target: "b1patte2", kind: AnimKind::SwingZ, amp: 0.5 },
    Anim { target: "b1patte3", kind: AnimKind::SwingZ, amp: 0.0 },
    Anim { target: "b2corps", kind: AnimKind::Bob, amp: 0.04 },
    Anim { target: "b2queue", kind: AnimKind::TailY, amp: 0.0 },
    Anim { target: "b2patte0", kind: AnimKind::SwingZ, amp: 0.0 },
    Anim { target: "b2patte1", kind: AnimKind::SwingZ, amp: 0.5 },
    Anim { target: "b2patte2", kind: AnimKind::SwingZ, amp: 0.5 },
    Anim { target: "b2patte3", kind: AnimKind::SwingZ, amp: 0.0 },
];
const PIECES_84: &[Piece] = &[
    Piece { shape: Shape::Sphere, pos: [0.0, 0.9, 0.0], size: [0.7, 0.3, 0.34], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [0.4, 0.95, 0.0], size: [0.22, 0.2, 0.2], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [0.52, 1.0, 0.0], size: [0.18, 0.16, 0.16], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("corps") },
    Piece { shape: Shape::Cone, pos: [0.65, 0.97, 0.0], size: [0.07, 0.16, 0.07], rot: [0.0, 0.0, -90.0], mat: Mat::Blanc, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [0.34, 0.86, 0.0], size: [0.3, 0.12, 0.3], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [0.56, 1.04, 0.07], size: [0.05, 0.05, 0.05], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [0.56, 1.04, -0.07], size: [0.05, 0.05, 0.05], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("corps") },
    Piece { shape: Shape::Cube, pos: [-0.45, 0.88, 0.0], size: [0.4, 0.04, 0.22], rot: [0.0, 0.0, 8.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Cube, pos: [0.0, 0.92, 0.65], size: [0.7, 0.04, 1.0], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("aile_g") },
    Piece { shape: Shape::Cube, pos: [-0.15, 0.92, 1.2], size: [0.6, 0.04, 0.5], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("aile_g") },
    Piece { shape: Shape::Cube, pos: [0.1, 0.92, 1.55], size: [0.2, 0.04, 0.4], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: Some("aile_g") },
    Piece { shape: Shape::Cube, pos: [-0.3, 0.92, 1.5], size: [0.2, 0.04, 0.4], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: Some("aile_g") },
    Piece { shape: Shape::Cube, pos: [0.0, 0.92, -0.65], size: [0.7, 0.04, 1.0], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("aile_d") },
    Piece { shape: Shape::Cube, pos: [-0.15, 0.92, -1.2], size: [0.6, 0.04, 0.5], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("aile_d") },
    Piece { shape: Shape::Cube, pos: [0.1, 0.92, -1.55], size: [0.2, 0.04, 0.4], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: Some("aile_d") },
    Piece { shape: Shape::Cube, pos: [-0.3, 0.92, -1.5], size: [0.2, 0.04, 0.4], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: Some("aile_d") },
];
const ANIMS_84: &[Anim] = &[
    Anim { target: "corps", kind: AnimKind::Bob, amp: 0.04 },
    Anim { target: "aile_g", kind: AnimKind::SwingXBase, amp: 0.0 },
    Anim { target: "aile_d", kind: AnimKind::SwingXBase, amp: 0.0 },
];
const PIECES_85: &[Piece] = &[
    Piece { shape: Shape::Cube, pos: [-0.35, 0.55, 0.0], size: [0.9, 0.12, 0.65], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [-0.6, 0.78, 0.0], size: [0.15, 0.4, 0.62], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [-0.2, 0.7, 0.0], size: [0.45, 0.14, 0.55], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [0.2, 0.62, 0.0], size: [0.55, 0.08, 0.15], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.6, 0.34, 0.38], size: [0.68, 0.08, 0.68], rot: [90.0, 0.0, 0.0], mat: Mat::Pneu, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.6, 0.34, 0.425], size: [0.408, 0.05, 0.408], rot: [90.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.6, 0.34, -0.38], size: [0.68, 0.08, 0.68], rot: [90.0, 0.0, 0.0], mat: Mat::Pneu, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.6, 0.34, -0.425], size: [0.408, 0.05, 0.408], rot: [90.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [0.05, 0.26, 0.38], size: [0.52, 0.08, 0.52], rot: [90.0, 0.0, 0.0], mat: Mat::Pneu, id: None },
    Piece { shape: Shape::Cyl, pos: [0.05, 0.26, 0.425], size: [0.312, 0.05, 0.312], rot: [90.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [0.05, 0.26, -0.38], size: [0.52, 0.08, 0.52], rot: [90.0, 0.0, 0.0], mat: Mat::Pneu, id: None },
    Piece { shape: Shape::Cyl, pos: [0.05, 0.26, -0.425], size: [0.312, 0.05, 0.312], rot: [90.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Sphere, pos: [-0.25, 1.05, 0.0], size: [0.2, 0.22, 0.2], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [-0.25, 0.92, 0.0], size: [0.24, 0.28, 0.26], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [-0.25, 1.2, 0.0], size: [0.4, 0.04, 0.4], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.25, 1.28, 0.0], size: [0.22, 0.14, 0.22], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Sphere, pos: [0.85, 0.78, 0.0], size: [1.05, 0.52, 0.46], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("hcorps") },
    Piece { shape: Shape::Sphere, pos: [1.15, 0.84, 0.0], size: [0.55, 0.58, 0.44], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("hcorps") },
    Piece { shape: Shape::Sphere, pos: [0.53, 0.8, 0.0], size: [0.55, 0.55, 0.46], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("hcorps") },
    Piece { shape: Shape::Sphere, pos: [1.43, 1.05, 0.0], size: [0.3, 0.6, 0.26], rot: [0.0, 0.0, -32.0], mat: Mat::C2, id: Some("hcorps") },
    Piece { shape: Shape::Sphere, pos: [1.69, 1.22, 0.0], size: [0.42, 0.26, 0.22], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("hcorps") },
    Piece { shape: Shape::Sphere, pos: [1.87, 1.15, 0.0], size: [0.24, 0.2, 0.18], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("hcorps") },
    Piece { shape: Shape::Cone, pos: [1.63, 1.42, 0.07], size: [0.07, 0.14, 0.05], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("hcorps") },
    Piece { shape: Shape::Cone, pos: [1.63, 1.42, -0.07], size: [0.07, 0.14, 0.05], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("hcorps") },
    Piece { shape: Shape::Sphere, pos: [1.77, 1.27, 0.1], size: [0.05, 0.05, 0.05], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("hcorps") },
    Piece { shape: Shape::Sphere, pos: [1.77, 1.27, -0.1], size: [0.05, 0.05, 0.05], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("hcorps") },
    Piece { shape: Shape::Cube, pos: [1.2, 1.15, 0.0], size: [0.1, 0.22, 0.06], rot: [0.0, 0.0, -30.0], mat: Mat::C3, id: Some("hcorps") },
    Piece { shape: Shape::Cube, pos: [1.35, 1.21, 0.0], size: [0.1, 0.22, 0.06], rot: [0.0, 0.0, -30.0], mat: Mat::C3, id: Some("hcorps") },
    Piece { shape: Shape::Cube, pos: [1.47, 1.258, 0.0], size: [0.1, 0.22, 0.06], rot: [0.0, 0.0, -30.0], mat: Mat::C3, id: Some("hcorps") },
    Piece { shape: Shape::Cone, pos: [0.23, 0.78, 0.0], size: [0.12, 0.55, 0.12], rot: [0.0, 0.0, 110.0], mat: Mat::C3, id: Some("hqueue") },
    Piece { shape: Shape::Cyl, pos: [1.25, 0.34, 0.14], size: [0.12, 0.7, 0.12], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("hpatte0") },
    Piece { shape: Shape::Cube, pos: [1.28, 0.03, 0.14], size: [0.18, 0.06, 0.14], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("hpatte0") },
    Piece { shape: Shape::Cyl, pos: [1.25, 0.34, -0.14], size: [0.12, 0.7, 0.12], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("hpatte1") },
    Piece { shape: Shape::Cube, pos: [1.28, 0.03, -0.14], size: [0.18, 0.06, 0.14], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("hpatte1") },
    Piece { shape: Shape::Cyl, pos: [0.45, 0.34, 0.14], size: [0.12, 0.7, 0.12], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("hpatte2") },
    Piece { shape: Shape::Cube, pos: [0.48, 0.03, 0.14], size: [0.18, 0.06, 0.14], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("hpatte2") },
    Piece { shape: Shape::Cyl, pos: [0.45, 0.34, -0.14], size: [0.12, 0.7, 0.12], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("hpatte3") },
    Piece { shape: Shape::Cube, pos: [0.48, 0.03, -0.14], size: [0.18, 0.06, 0.14], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("hpatte3") },
];
const ANIMS_85: &[Anim] = &[
    Anim { target: "hcorps", kind: AnimKind::Bob, amp: 0.05 },
    Anim { target: "hqueue", kind: AnimKind::TailY, amp: 0.0 },
    Anim { target: "hpatte0", kind: AnimKind::SwingZ, amp: 0.0 },
    Anim { target: "hpatte1", kind: AnimKind::SwingZ, amp: 0.5 },
    Anim { target: "hpatte2", kind: AnimKind::SwingZ, amp: 0.5 },
    Anim { target: "hpatte3", kind: AnimKind::SwingZ, amp: 0.0 },
];
const PIECES_86: &[Piece] = &[
    Piece { shape: Shape::Cube, pos: [-0.9, 0.8, 0.0], size: [1.5, 0.75, 0.85], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [-0.9, 1.22, 0.0], size: [1.6, 0.1, 0.95], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cube, pos: [-0.9, 1.4, 0.0], size: [1.3, 0.25, 0.7], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [-0.9, 0.85, 0.44], size: [0.5, 0.35, 0.03], rot: [0.0, 0.0, 0.0], mat: Mat::Vitre, id: None },
    Piece { shape: Shape::Cube, pos: [-0.9, 0.85, -0.44], size: [0.5, 0.35, 0.03], rot: [0.0, 0.0, 0.0], mat: Mat::Vitre, id: None },
    Piece { shape: Shape::Cube, pos: [-0.2, 0.9, 0.0], size: [0.2, 0.5, 0.7], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [-0.15, 1.15, 0.0], size: [0.45, 0.14, 0.7], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cube, pos: [-0.15, 1.45, 0.0], size: [0.1, 0.5, 0.6], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [-0.9, 0.42, 0.0], size: [1.7, 0.1, 0.7], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cyl, pos: [-1.4, 0.4, 0.5], size: [0.8, 0.08, 0.8], rot: [90.0, 0.0, 0.0], mat: Mat::Pneu, id: None },
    Piece { shape: Shape::Cyl, pos: [-1.4, 0.4, 0.545], size: [0.48, 0.05, 0.48], rot: [90.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [-1.4, 0.4, -0.5], size: [0.8, 0.08, 0.8], rot: [90.0, 0.0, 0.0], mat: Mat::Pneu, id: None },
    Piece { shape: Shape::Cyl, pos: [-1.4, 0.4, -0.545], size: [0.48, 0.05, 0.48], rot: [90.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.35, 0.32, 0.5], size: [0.64, 0.08, 0.64], rot: [90.0, 0.0, 0.0], mat: Mat::Pneu, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.35, 0.32, 0.545], size: [0.384, 0.05, 0.384], rot: [90.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.35, 0.32, -0.5], size: [0.64, 0.08, 0.64], rot: [90.0, 0.0, 0.0], mat: Mat::Pneu, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.35, 0.32, -0.545], size: [0.384, 0.05, 0.384], rot: [90.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Sphere, pos: [-0.1, 1.82, 0.0], size: [0.22, 0.24, 0.22], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [-0.1, 1.64, 0.0], size: [0.28, 0.34, 0.3], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [-0.1, 1.97, 0.0], size: [0.46, 0.04, 0.46], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.1, 2.05, 0.0], size: [0.24, 0.16, 0.24], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Sphere, pos: [0.7, 0.78, 0.24], size: [1.05, 0.52, 0.46], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("acorps") },
    Piece { shape: Shape::Sphere, pos: [1.0, 0.84, 0.24], size: [0.55, 0.58, 0.44], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("acorps") },
    Piece { shape: Shape::Sphere, pos: [0.38, 0.8, 0.24], size: [0.55, 0.55, 0.46], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("acorps") },
    Piece { shape: Shape::Sphere, pos: [1.28, 1.05, 0.24], size: [0.3, 0.6, 0.26], rot: [0.0, 0.0, -32.0], mat: Mat::C2, id: Some("acorps") },
    Piece { shape: Shape::Sphere, pos: [1.54, 1.22, 0.24], size: [0.42, 0.26, 0.22], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("acorps") },
    Piece { shape: Shape::Sphere, pos: [1.72, 1.15, 0.24], size: [0.24, 0.2, 0.18], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("acorps") },
    Piece { shape: Shape::Cone, pos: [1.48, 1.42, 0.31], size: [0.07, 0.14, 0.05], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("acorps") },
    Piece { shape: Shape::Cone, pos: [1.48, 1.42, 0.17], size: [0.07, 0.14, 0.05], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("acorps") },
    Piece { shape: Shape::Sphere, pos: [1.62, 1.27, 0.34], size: [0.05, 0.05, 0.05], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("acorps") },
    Piece { shape: Shape::Sphere, pos: [1.62, 1.27, 0.14], size: [0.05, 0.05, 0.05], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("acorps") },
    Piece { shape: Shape::Cube, pos: [1.05, 1.15, 0.24], size: [0.1, 0.22, 0.06], rot: [0.0, 0.0, -30.0], mat: Mat::C3, id: Some("acorps") },
    Piece { shape: Shape::Cube, pos: [1.2, 1.21, 0.24], size: [0.1, 0.22, 0.06], rot: [0.0, 0.0, -30.0], mat: Mat::C3, id: Some("acorps") },
    Piece { shape: Shape::Cube, pos: [1.32, 1.258, 0.24], size: [0.1, 0.22, 0.06], rot: [0.0, 0.0, -30.0], mat: Mat::C3, id: Some("acorps") },
    Piece { shape: Shape::Cone, pos: [0.08, 0.78, 0.24], size: [0.12, 0.55, 0.12], rot: [0.0, 0.0, 110.0], mat: Mat::C3, id: Some("aqueue") },
    Piece { shape: Shape::Cyl, pos: [1.1, 0.34, 0.38], size: [0.12, 0.7, 0.12], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("apatte0") },
    Piece { shape: Shape::Cube, pos: [1.13, 0.03, 0.38], size: [0.18, 0.06, 0.14], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("apatte0") },
    Piece { shape: Shape::Cyl, pos: [1.1, 0.34, 0.1], size: [0.12, 0.7, 0.12], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("apatte1") },
    Piece { shape: Shape::Cube, pos: [1.13, 0.03, 0.1], size: [0.18, 0.06, 0.14], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("apatte1") },
    Piece { shape: Shape::Cyl, pos: [0.3, 0.34, 0.38], size: [0.12, 0.7, 0.12], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("apatte2") },
    Piece { shape: Shape::Cube, pos: [0.33, 0.03, 0.38], size: [0.18, 0.06, 0.14], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("apatte2") },
    Piece { shape: Shape::Cyl, pos: [0.3, 0.34, 0.1], size: [0.12, 0.7, 0.12], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("apatte3") },
    Piece { shape: Shape::Cube, pos: [0.33, 0.03, 0.1], size: [0.18, 0.06, 0.14], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("apatte3") },
    Piece { shape: Shape::Sphere, pos: [0.7, 0.78, -0.24], size: [1.05, 0.52, 0.46], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("bcorps") },
    Piece { shape: Shape::Sphere, pos: [1.0, 0.84, -0.24], size: [0.55, 0.58, 0.44], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("bcorps") },
    Piece { shape: Shape::Sphere, pos: [0.38, 0.8, -0.24], size: [0.55, 0.55, 0.46], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("bcorps") },
    Piece { shape: Shape::Sphere, pos: [1.28, 1.05, -0.24], size: [0.3, 0.6, 0.26], rot: [0.0, 0.0, -32.0], mat: Mat::C2, id: Some("bcorps") },
    Piece { shape: Shape::Sphere, pos: [1.54, 1.22, -0.24], size: [0.42, 0.26, 0.22], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("bcorps") },
    Piece { shape: Shape::Sphere, pos: [1.72, 1.15, -0.24], size: [0.24, 0.2, 0.18], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("bcorps") },
    Piece { shape: Shape::Cone, pos: [1.48, 1.42, -0.17], size: [0.07, 0.14, 0.05], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("bcorps") },
    Piece { shape: Shape::Cone, pos: [1.48, 1.42, -0.31], size: [0.07, 0.14, 0.05], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("bcorps") },
    Piece { shape: Shape::Sphere, pos: [1.62, 1.27, -0.14], size: [0.05, 0.05, 0.05], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("bcorps") },
    Piece { shape: Shape::Sphere, pos: [1.62, 1.27, -0.34], size: [0.05, 0.05, 0.05], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("bcorps") },
    Piece { shape: Shape::Cube, pos: [1.05, 1.15, -0.24], size: [0.1, 0.22, 0.06], rot: [0.0, 0.0, -30.0], mat: Mat::C3, id: Some("bcorps") },
    Piece { shape: Shape::Cube, pos: [1.2, 1.21, -0.24], size: [0.1, 0.22, 0.06], rot: [0.0, 0.0, -30.0], mat: Mat::C3, id: Some("bcorps") },
    Piece { shape: Shape::Cube, pos: [1.32, 1.258, -0.24], size: [0.1, 0.22, 0.06], rot: [0.0, 0.0, -30.0], mat: Mat::C3, id: Some("bcorps") },
    Piece { shape: Shape::Cone, pos: [0.08, 0.78, -0.24], size: [0.12, 0.55, 0.12], rot: [0.0, 0.0, 110.0], mat: Mat::C3, id: Some("bqueue") },
    Piece { shape: Shape::Cyl, pos: [1.1, 0.34, -0.1], size: [0.12, 0.7, 0.12], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("bpatte0") },
    Piece { shape: Shape::Cube, pos: [1.13, 0.03, -0.1], size: [0.18, 0.06, 0.14], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("bpatte0") },
    Piece { shape: Shape::Cyl, pos: [1.1, 0.34, -0.38], size: [0.12, 0.7, 0.12], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("bpatte1") },
    Piece { shape: Shape::Cube, pos: [1.13, 0.03, -0.38], size: [0.18, 0.06, 0.14], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("bpatte1") },
    Piece { shape: Shape::Cyl, pos: [0.3, 0.34, -0.1], size: [0.12, 0.7, 0.12], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("bpatte2") },
    Piece { shape: Shape::Cube, pos: [0.33, 0.03, -0.1], size: [0.18, 0.06, 0.14], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("bpatte2") },
    Piece { shape: Shape::Cyl, pos: [0.3, 0.34, -0.38], size: [0.12, 0.7, 0.12], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("bpatte3") },
    Piece { shape: Shape::Cube, pos: [0.33, 0.03, -0.38], size: [0.18, 0.06, 0.14], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("bpatte3") },
];
const ANIMS_86: &[Anim] = &[
    Anim { target: "acorps", kind: AnimKind::Bob, amp: 0.05 },
    Anim { target: "aqueue", kind: AnimKind::TailY, amp: 0.0 },
    Anim { target: "apatte0", kind: AnimKind::SwingZ, amp: 0.0 },
    Anim { target: "apatte1", kind: AnimKind::SwingZ, amp: 0.5 },
    Anim { target: "apatte2", kind: AnimKind::SwingZ, amp: 0.5 },
    Anim { target: "apatte3", kind: AnimKind::SwingZ, amp: 0.0 },
    Anim { target: "bcorps", kind: AnimKind::Bob, amp: 0.05 },
    Anim { target: "bqueue", kind: AnimKind::TailY, amp: 0.0 },
    Anim { target: "bpatte0", kind: AnimKind::SwingZ, amp: 0.0 },
    Anim { target: "bpatte1", kind: AnimKind::SwingZ, amp: 0.5 },
    Anim { target: "bpatte2", kind: AnimKind::SwingZ, amp: 0.5 },
    Anim { target: "bpatte3", kind: AnimKind::SwingZ, amp: 0.0 },
];
const PIECES_87: &[Piece] = &[
    Piece { shape: Shape::Cube, pos: [-0.7, 0.55, 0.0], size: [1.9, 0.18, 0.85], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.7, 0.98, 0.0], size: [0.95, 1.8, 0.95], rot: [0.0, 0.0, 90.0], mat: Mat::Blanc, id: None },
    Piece { shape: Shape::Cube, pos: [0.25, 0.9, 0.0], size: [0.06, 0.7, 0.8], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: None },
    Piece { shape: Shape::Cube, pos: [-1.65, 0.9, 0.0], size: [0.06, 0.7, 0.8], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: None },
    Piece { shape: Shape::Cube, pos: [0.2, 0.62, 0.0], size: [0.5, 0.1, 0.3], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [-1.3, 0.42, 0.5], size: [0.84, 0.08, 0.84], rot: [90.0, 0.0, 0.0], mat: Mat::Pneu, id: None },
    Piece { shape: Shape::Cyl, pos: [-1.3, 0.42, 0.545], size: [0.504, 0.05, 0.504], rot: [90.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [-1.3, 0.42, -0.5], size: [0.84, 0.08, 0.84], rot: [90.0, 0.0, 0.0], mat: Mat::Pneu, id: None },
    Piece { shape: Shape::Cyl, pos: [-1.3, 0.42, -0.545], size: [0.504, 0.05, 0.504], rot: [90.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.1, 0.36, 0.5], size: [0.72, 0.08, 0.72], rot: [90.0, 0.0, 0.0], mat: Mat::Pneu, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.1, 0.36, 0.545], size: [0.432, 0.05, 0.432], rot: [90.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.1, 0.36, -0.5], size: [0.72, 0.08, 0.72], rot: [90.0, 0.0, 0.0], mat: Mat::Pneu, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.1, 0.36, -0.545], size: [0.432, 0.05, 0.432], rot: [90.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Sphere, pos: [0.85, 0.78, 0.24], size: [1.05, 0.52, 0.46], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("acorps") },
    Piece { shape: Shape::Sphere, pos: [1.15, 0.84, 0.24], size: [0.55, 0.58, 0.44], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("acorps") },
    Piece { shape: Shape::Sphere, pos: [0.53, 0.8, 0.24], size: [0.55, 0.55, 0.46], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("acorps") },
    Piece { shape: Shape::Sphere, pos: [1.43, 1.05, 0.24], size: [0.3, 0.6, 0.26], rot: [0.0, 0.0, -32.0], mat: Mat::C2, id: Some("acorps") },
    Piece { shape: Shape::Sphere, pos: [1.69, 1.22, 0.24], size: [0.42, 0.26, 0.22], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("acorps") },
    Piece { shape: Shape::Sphere, pos: [1.87, 1.15, 0.24], size: [0.24, 0.2, 0.18], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("acorps") },
    Piece { shape: Shape::Cone, pos: [1.63, 1.42, 0.31], size: [0.07, 0.14, 0.05], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("acorps") },
    Piece { shape: Shape::Cone, pos: [1.63, 1.42, 0.17], size: [0.07, 0.14, 0.05], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("acorps") },
    Piece { shape: Shape::Sphere, pos: [1.77, 1.27, 0.34], size: [0.05, 0.05, 0.05], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("acorps") },
    Piece { shape: Shape::Sphere, pos: [1.77, 1.27, 0.14], size: [0.05, 0.05, 0.05], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("acorps") },
    Piece { shape: Shape::Cube, pos: [1.2, 1.15, 0.24], size: [0.1, 0.22, 0.06], rot: [0.0, 0.0, -30.0], mat: Mat::C3, id: Some("acorps") },
    Piece { shape: Shape::Cube, pos: [1.35, 1.21, 0.24], size: [0.1, 0.22, 0.06], rot: [0.0, 0.0, -30.0], mat: Mat::C3, id: Some("acorps") },
    Piece { shape: Shape::Cube, pos: [1.47, 1.258, 0.24], size: [0.1, 0.22, 0.06], rot: [0.0, 0.0, -30.0], mat: Mat::C3, id: Some("acorps") },
    Piece { shape: Shape::Cone, pos: [0.23, 0.78, 0.24], size: [0.12, 0.55, 0.12], rot: [0.0, 0.0, 110.0], mat: Mat::C3, id: Some("aqueue") },
    Piece { shape: Shape::Cyl, pos: [1.25, 0.34, 0.38], size: [0.12, 0.7, 0.12], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("apatte0") },
    Piece { shape: Shape::Cube, pos: [1.28, 0.03, 0.38], size: [0.18, 0.06, 0.14], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("apatte0") },
    Piece { shape: Shape::Cyl, pos: [1.25, 0.34, 0.1], size: [0.12, 0.7, 0.12], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("apatte1") },
    Piece { shape: Shape::Cube, pos: [1.28, 0.03, 0.1], size: [0.18, 0.06, 0.14], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("apatte1") },
    Piece { shape: Shape::Cyl, pos: [0.45, 0.34, 0.38], size: [0.12, 0.7, 0.12], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("apatte2") },
    Piece { shape: Shape::Cube, pos: [0.48, 0.03, 0.38], size: [0.18, 0.06, 0.14], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("apatte2") },
    Piece { shape: Shape::Cyl, pos: [0.45, 0.34, 0.1], size: [0.12, 0.7, 0.12], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("apatte3") },
    Piece { shape: Shape::Cube, pos: [0.48, 0.03, 0.1], size: [0.18, 0.06, 0.14], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("apatte3") },
    Piece { shape: Shape::Sphere, pos: [0.85, 0.78, -0.24], size: [1.05, 0.52, 0.46], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("bcorps") },
    Piece { shape: Shape::Sphere, pos: [1.15, 0.84, -0.24], size: [0.55, 0.58, 0.44], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("bcorps") },
    Piece { shape: Shape::Sphere, pos: [0.53, 0.8, -0.24], size: [0.55, 0.55, 0.46], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("bcorps") },
    Piece { shape: Shape::Sphere, pos: [1.43, 1.05, -0.24], size: [0.3, 0.6, 0.26], rot: [0.0, 0.0, -32.0], mat: Mat::C2, id: Some("bcorps") },
    Piece { shape: Shape::Sphere, pos: [1.69, 1.22, -0.24], size: [0.42, 0.26, 0.22], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("bcorps") },
    Piece { shape: Shape::Sphere, pos: [1.87, 1.15, -0.24], size: [0.24, 0.2, 0.18], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("bcorps") },
    Piece { shape: Shape::Cone, pos: [1.63, 1.42, -0.17], size: [0.07, 0.14, 0.05], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("bcorps") },
    Piece { shape: Shape::Cone, pos: [1.63, 1.42, -0.31], size: [0.07, 0.14, 0.05], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("bcorps") },
    Piece { shape: Shape::Sphere, pos: [1.77, 1.27, -0.14], size: [0.05, 0.05, 0.05], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("bcorps") },
    Piece { shape: Shape::Sphere, pos: [1.77, 1.27, -0.34], size: [0.05, 0.05, 0.05], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("bcorps") },
    Piece { shape: Shape::Cube, pos: [1.2, 1.15, -0.24], size: [0.1, 0.22, 0.06], rot: [0.0, 0.0, -30.0], mat: Mat::C3, id: Some("bcorps") },
    Piece { shape: Shape::Cube, pos: [1.35, 1.21, -0.24], size: [0.1, 0.22, 0.06], rot: [0.0, 0.0, -30.0], mat: Mat::C3, id: Some("bcorps") },
    Piece { shape: Shape::Cube, pos: [1.47, 1.258, -0.24], size: [0.1, 0.22, 0.06], rot: [0.0, 0.0, -30.0], mat: Mat::C3, id: Some("bcorps") },
    Piece { shape: Shape::Cone, pos: [0.23, 0.78, -0.24], size: [0.12, 0.55, 0.12], rot: [0.0, 0.0, 110.0], mat: Mat::C3, id: Some("bqueue") },
    Piece { shape: Shape::Cyl, pos: [1.25, 0.34, -0.1], size: [0.12, 0.7, 0.12], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("bpatte0") },
    Piece { shape: Shape::Cube, pos: [1.28, 0.03, -0.1], size: [0.18, 0.06, 0.14], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("bpatte0") },
    Piece { shape: Shape::Cyl, pos: [1.25, 0.34, -0.38], size: [0.12, 0.7, 0.12], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("bpatte1") },
    Piece { shape: Shape::Cube, pos: [1.28, 0.03, -0.38], size: [0.18, 0.06, 0.14], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("bpatte1") },
    Piece { shape: Shape::Cyl, pos: [0.45, 0.34, -0.1], size: [0.12, 0.7, 0.12], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("bpatte2") },
    Piece { shape: Shape::Cube, pos: [0.48, 0.03, -0.1], size: [0.18, 0.06, 0.14], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("bpatte2") },
    Piece { shape: Shape::Cyl, pos: [0.45, 0.34, -0.38], size: [0.12, 0.7, 0.12], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("bpatte3") },
    Piece { shape: Shape::Cube, pos: [0.48, 0.03, -0.38], size: [0.18, 0.06, 0.14], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("bpatte3") },
];
const ANIMS_87: &[Anim] = &[
    Anim { target: "acorps", kind: AnimKind::Bob, amp: 0.05 },
    Anim { target: "aqueue", kind: AnimKind::TailY, amp: 0.0 },
    Anim { target: "apatte0", kind: AnimKind::SwingZ, amp: 0.0 },
    Anim { target: "apatte1", kind: AnimKind::SwingZ, amp: 0.5 },
    Anim { target: "apatte2", kind: AnimKind::SwingZ, amp: 0.5 },
    Anim { target: "apatte3", kind: AnimKind::SwingZ, amp: 0.0 },
    Anim { target: "bcorps", kind: AnimKind::Bob, amp: 0.05 },
    Anim { target: "bqueue", kind: AnimKind::TailY, amp: 0.0 },
    Anim { target: "bpatte0", kind: AnimKind::SwingZ, amp: 0.0 },
    Anim { target: "bpatte1", kind: AnimKind::SwingZ, amp: 0.5 },
    Anim { target: "bpatte2", kind: AnimKind::SwingZ, amp: 0.5 },
    Anim { target: "bpatte3", kind: AnimKind::SwingZ, amp: 0.0 },
];
const FARWEST_ANIMALS: [Animal; 10] = [
    Animal { model: Model { c2: [201, 168, 90], c3: [138, 106, 42], pieces: PIECES_78, anims: ANIMS_78 }, len: 0.7, width: 0.6, points: 300, dangerous: false, flying: false },
    Animal { model: Model { c2: [181, 154, 120], c3: [138, 115, 88], pieces: PIECES_79, anims: ANIMS_79 }, len: 1.0, width: 0.4, points: 450, dangerous: false, flying: false },
    Animal { model: Model { c2: [216, 200, 168], c3: [74, 48, 32], pieces: PIECES_80, anims: ANIMS_80 }, len: 1.9, width: 0.5, points: 800, dangerous: true, flying: false },
    Animal { model: Model { c2: [139, 90, 43], c3: [192, 57, 43], pieces: PIECES_81, anims: ANIMS_81 }, len: 1.9, width: 0.5, points: 1300, dangerous: true, flying: false },
    Animal { model: Model { c2: [74, 48, 32], c3: [122, 90, 64], pieces: PIECES_82, anims: ANIMS_82 }, len: 2.0, width: 1.0, points: 1800, dangerous: true, flying: false },
    Animal { model: Model { c2: [74, 48, 32], c3: [122, 90, 64], pieces: PIECES_83, anims: ANIMS_83 }, len: 5.4, width: 1.4, points: 2600, dangerous: true, flying: false },
    Animal { model: Model { c2: [74, 58, 42], c3: [232, 120, 106], pieces: PIECES_84, anims: ANIMS_84 }, len: 1.2, width: 3.2, points: 3000, dangerous: true, flying: true },
    Animal { model: Model { c2: [74, 48, 32], c3: [192, 57, 43], pieces: PIECES_85, anims: ANIMS_85 }, len: 2.6, width: 0.9, points: 600, dangerous: true, flying: false },
    Animal { model: Model { c2: [160, 82, 45], c3: [232, 184, 48], pieces: PIECES_86, anims: ANIMS_86 }, len: 4.0, width: 1.0, points: 1100, dangerous: true, flying: false },
    Animal { model: Model { c2: [139, 90, 43], c3: [122, 77, 36], pieces: PIECES_87, anims: ANIMS_87 }, len: 3.9, width: 1.0, points: 1500, dangerous: true, flying: false },
];
fn farwest_animal_name(i: usize) -> &'static str {
    match i {
        0 => tx!("Boule d'Herbe Sèche", "Tumbleweed", "Steppenhexe", "Erba Rotolante", "Planta Rodadora").t(),
        1 => tx!("Coyote Rigolo", "Funny Coyote", "Lustiger Kojote", "Coyote Buffo", "Coyote Gracioso").t(),
        2 => tx!("Mustang au Galop", "Galloping Mustang", "Galoppierender Mustang", "Mustang al Galoppo", "Mustang al Galope").t(),
        3 => tx!("Cavalier Pressé", "Hasty Rider", "Eiliger Reiter", "Cavaliere Frettoloso", "Jinete con Prisa").t(),
        4 => tx!("Bison Pas Content", "Cranky Bison", "Mürrischer Bison", "Bisonte Scontroso", "Bisonte Gruñón").t(),
        5 => tx!("Troupeau de Bisons", "Bison Herd", "Bisonherde", "Mandria di Bisonti", "Manada de Bisontes").t(),
        6 => tx!("Vautour Affamé", "Hungry Vulture", "Hungriger Geier", "Avvoltoio Affamato", "Buitre Hambriento").t(),
        7 => tx!("Buggy du Docteur", "Doctor's Buggy", "Doktors Buggy", "Buggy del Dottore", "Buggy del Doctor").t(),
        8 => tx!("Diligence Fonceuse", "Speeding Stagecoach", "Rasende Postkutsche", "Diligenza Veloce", "Diligencia Veloz").t(),
        9 => tx!("Chariot Bâché", "Covered Wagon", "Planwagen", "Carro Coperto", "Carro Cubierto").t(),
        _ => "?",
    }
}
const PIECES_88: &[Piece] = &[
    Piece { shape: Shape::Sphere, pos: [-0.05, 0.26, 0.0], size: [0.5, 0.34, 0.46], rot: [0.0, 0.0, 0.0], mat: Mat::Peinture, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [0.0, 0.17, 0.0], size: [0.38, 0.2, 0.4], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Sphere, pos: [0.2, 0.36, 0.0], size: [0.36, 0.26, 0.4], rot: [0.0, 0.0, 0.0], mat: Mat::Peinture, id: None },
    Piece { shape: Shape::Cube, pos: [0.37, 0.31, 0.0], size: [0.02, 0.02, 0.3], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: None },
    Piece { shape: Shape::Sphere, pos: [0.2, 0.54, 0.14], size: [0.15, 0.15, 0.15], rot: [0.0, 0.0, 0.0], mat: Mat::Peinture, id: None },
    Piece { shape: Shape::Sphere, pos: [0.25, 0.56, 0.14], size: [0.11, 0.11, 0.11], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: None },
    Piece { shape: Shape::Sphere, pos: [0.3, 0.57, 0.14], size: [0.06, 0.06, 0.06], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: None },
    Piece { shape: Shape::Sphere, pos: [-0.2, 0.2, 0.26], size: [0.22, 0.2, 0.14], rot: [0.0, 0.0, 0.0], mat: Mat::Peinture, id: None },
    Piece { shape: Shape::Cube, pos: [-0.1, 0.07, 0.27], size: [0.14, 0.1, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::Peinture, id: None },
    Piece { shape: Shape::Cube, pos: [-0.04, 0.025, 0.27], size: [0.26, 0.05, 0.12], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cyl, pos: [0.18, 0.1, 0.2], size: [0.07, 0.2, 0.07], rot: [0.0, 0.0, 0.0], mat: Mat::Peinture, id: None },
    Piece { shape: Shape::Cube, pos: [0.22, 0.015, 0.2], size: [0.1, 0.03, 0.08], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Sphere, pos: [0.2, 0.54, -0.14], size: [0.15, 0.15, 0.15], rot: [0.0, 0.0, 0.0], mat: Mat::Peinture, id: None },
    Piece { shape: Shape::Sphere, pos: [0.25, 0.56, -0.14], size: [0.11, 0.11, 0.11], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: None },
    Piece { shape: Shape::Sphere, pos: [0.3, 0.57, -0.14], size: [0.06, 0.06, 0.06], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: None },
    Piece { shape: Shape::Sphere, pos: [-0.2, 0.2, -0.26], size: [0.22, 0.2, 0.14], rot: [0.0, 0.0, 0.0], mat: Mat::Peinture, id: None },
    Piece { shape: Shape::Cube, pos: [-0.1, 0.07, -0.27], size: [0.14, 0.1, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::Peinture, id: None },
    Piece { shape: Shape::Cube, pos: [-0.04, 0.025, -0.27], size: [0.26, 0.05, 0.12], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cyl, pos: [0.18, 0.1, -0.2], size: [0.07, 0.2, 0.07], rot: [0.0, 0.0, 0.0], mat: Mat::Peinture, id: None },
    Piece { shape: Shape::Cube, pos: [0.22, 0.015, -0.2], size: [0.1, 0.03, 0.08], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Sphere, pos: [-0.12, 0.42, 0.1], size: [0.08, 0.05, 0.08], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Sphere, pos: [-0.18, 0.4, -0.08], size: [0.06, 0.04, 0.06], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
];
const ANIMS_88: &[Anim] = &[
    Anim { target: "corps", kind: AnimKind::Bob, amp: 0.02 },
];
const PIECES_89: &[Piece] = &[
    Piece { shape: Shape::Cyl, pos: [0.12, 0.78, 0.0], size: [0.3, 0.2, 0.3], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [0.12, 0.69, 0.0], size: [0.64, 0.03, 0.64], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [0.12, 0.72, 0.0], size: [0.31, 0.04, 0.31], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: None },
    Piece { shape: Shape::Cube, pos: [0.34, 0.28, 0.0], size: [0.06, 0.14, 0.34], rot: [0.0, 0.0, 0.0], mat: Mat::LumiereRouge, id: None },
    Piece { shape: Shape::Sphere, pos: [0.34, 0.2, 0.0], size: [0.08, 0.08, 0.08], rot: [0.0, 0.0, 0.0], mat: Mat::LumiereRouge, id: None },
];
const ANIMS_89: &[Anim] = &[
];
const PIECES_90: &[Piece] = &[
    Piece { shape: Shape::Cyl, pos: [0.35, 0.85, 0.0], size: [0.7, 1.8, 0.7], rot: [0.0, 0.0, 90.0], mat: Mat::Sombre, id: Some("corps") },
    Piece { shape: Shape::Cyl, pos: [1.28, 0.85, 0.0], size: [0.76, 0.12, 0.76], rot: [0.0, 0.0, 90.0], mat: Mat::C3, id: Some("corps") },
    Piece { shape: Shape::Cyl, pos: [1.0, 1.4, 0.0], size: [0.24, 0.5, 0.24], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: Some("corps") },
    Piece { shape: Shape::Cone, pos: [1.0, 1.75, 0.0], size: [0.46, 0.3, 0.46], rot: [180.0, 0.0, 0.0], mat: Mat::Sombre, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [0.35, 1.28, 0.0], size: [0.28, 0.24, 0.28], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("corps") },
    Piece { shape: Shape::Cube, pos: [-0.65, 1.0, 0.0], size: [0.85, 1.0, 0.85], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Cube, pos: [-0.65, 1.55, 0.0], size: [1.0, 0.08, 1.0], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: Some("corps") },
    Piece { shape: Shape::Cube, pos: [-0.65, 1.05, 0.43], size: [0.4, 0.4, 0.04], rot: [0.0, 0.0, 0.0], mat: Mat::Vitre, id: Some("corps") },
    Piece { shape: Shape::Cube, pos: [-0.65, 1.05, -0.43], size: [0.4, 0.4, 0.04], rot: [0.0, 0.0, 0.0], mat: Mat::Vitre, id: Some("corps") },
    Piece { shape: Shape::Cube, pos: [1.5, 0.3, 0.0], size: [0.5, 0.12, 0.8], rot: [0.0, 0.0, 28.0], mat: Mat::Sombre, id: Some("corps") },
    Piece { shape: Shape::Cube, pos: [1.3, 0.3, 0.0], size: [0.1, 0.08, 0.8], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [1.3, 0.95, 0.0], size: [0.18, 0.18, 0.18], rot: [0.0, 0.0, 0.0], mat: Mat::Phare, id: Some("corps") },
    Piece { shape: Shape::Cube, pos: [0.35, 0.28, 0.0], size: [1.8, 0.14, 0.7], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: Some("corps") },
    Piece { shape: Shape::Cyl, pos: [-0.3, 0.38, 0.4], size: [0.76, 0.08, 0.76], rot: [90.0, 0.0, 0.0], mat: Mat::Sombre, id: Some("corps") },
    Piece { shape: Shape::Cyl, pos: [-0.3, 0.38, 0.445], size: [0.418, 0.05, 0.418], rot: [90.0, 0.0, 0.0], mat: Mat::C3, id: Some("corps") },
    Piece { shape: Shape::Cyl, pos: [-0.3, 0.38, -0.4], size: [0.76, 0.08, 0.76], rot: [90.0, 0.0, 0.0], mat: Mat::Sombre, id: Some("corps") },
    Piece { shape: Shape::Cyl, pos: [-0.3, 0.38, -0.445], size: [0.418, 0.05, 0.418], rot: [90.0, 0.0, 0.0], mat: Mat::C3, id: Some("corps") },
    Piece { shape: Shape::Cyl, pos: [0.5, 0.38, 0.4], size: [0.76, 0.08, 0.76], rot: [90.0, 0.0, 0.0], mat: Mat::Sombre, id: Some("corps") },
    Piece { shape: Shape::Cyl, pos: [0.5, 0.38, 0.445], size: [0.418, 0.05, 0.418], rot: [90.0, 0.0, 0.0], mat: Mat::C3, id: Some("corps") },
    Piece { shape: Shape::Cyl, pos: [0.5, 0.38, -0.4], size: [0.76, 0.08, 0.76], rot: [90.0, 0.0, 0.0], mat: Mat::Sombre, id: Some("corps") },
    Piece { shape: Shape::Cyl, pos: [0.5, 0.38, -0.445], size: [0.418, 0.05, 0.418], rot: [90.0, 0.0, 0.0], mat: Mat::C3, id: Some("corps") },
    Piece { shape: Shape::Cyl, pos: [1.15, 0.22, 0.4], size: [0.44, 0.08, 0.44], rot: [90.0, 0.0, 0.0], mat: Mat::Sombre, id: Some("corps") },
    Piece { shape: Shape::Cyl, pos: [1.15, 0.22, 0.445], size: [0.242, 0.05, 0.242], rot: [90.0, 0.0, 0.0], mat: Mat::C3, id: Some("corps") },
    Piece { shape: Shape::Cyl, pos: [1.15, 0.22, -0.4], size: [0.44, 0.08, 0.44], rot: [90.0, 0.0, 0.0], mat: Mat::Sombre, id: Some("corps") },
    Piece { shape: Shape::Cyl, pos: [1.15, 0.22, -0.445], size: [0.242, 0.05, 0.242], rot: [90.0, 0.0, 0.0], mat: Mat::C3, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [1.0, 1.95, 0.0], size: [0.3, 0.3, 0.3], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: Some("fumee") },
    Piece { shape: Shape::Sphere, pos: [1.05, 2.2, 0.0], size: [0.4, 0.4, 0.4], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: Some("fumee") },
    Piece { shape: Shape::Sphere, pos: [1.1, 2.45, 0.0], size: [0.5, 0.5, 0.5], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: Some("fumee") },
];
const ANIMS_90: &[Anim] = &[
    Anim { target: "fumee", kind: AnimKind::DriftUp, amp: 0.0 },
    Anim { target: "corps", kind: AnimKind::Bob, amp: 0.012 },
];
const PIECES_91: &[Piece] = &[
    Piece { shape: Shape::Cube, pos: [0.0, 0.6, 0.0], size: [1.4, 0.7, 0.8], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 1.0, 0.0], size: [1.3, 0.2, 0.7], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: None },
    Piece { shape: Shape::Cube, pos: [0.2, 1.15, 0.0], size: [0.5, 0.15, 0.4], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.25, 0.0], size: [1.5, 0.12, 0.7], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.4, 0.25, 0.4], size: [0.5, 0.08, 0.5], rot: [90.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.4, 0.25, 0.445], size: [0.3, 0.05, 0.3], rot: [90.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.4, 0.25, -0.4], size: [0.5, 0.08, 0.5], rot: [90.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.4, 0.25, -0.445], size: [0.3, 0.05, 0.3], rot: [90.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [0.4, 0.25, 0.4], size: [0.5, 0.08, 0.5], rot: [90.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cyl, pos: [0.4, 0.25, 0.445], size: [0.3, 0.05, 0.3], rot: [90.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [0.4, 0.25, -0.4], size: [0.5, 0.08, 0.5], rot: [90.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cyl, pos: [0.4, 0.25, -0.445], size: [0.3, 0.05, 0.3], rot: [90.0, 0.0, 0.0], mat: Mat::C3, id: None },
];
const ANIMS_91: &[Anim] = &[
];
const PIECES_92: &[Piece] = &[
    Piece { shape: Shape::Cube, pos: [0.0, 0.75, 0.0], size: [1.9, 0.9, 0.85], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 1.25, 0.0], size: [2.0, 0.08, 0.95], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.25, 0.0], size: [2.0, 0.12, 0.7], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.75, 0.43], size: [0.5, 0.7, 0.03], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.75, -0.43], size: [0.5, 0.7, 0.03], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.6, 0.25, 0.4], size: [0.5, 0.08, 0.5], rot: [90.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.6, 0.25, 0.445], size: [0.3, 0.05, 0.3], rot: [90.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.6, 0.25, -0.4], size: [0.5, 0.08, 0.5], rot: [90.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.6, 0.25, -0.445], size: [0.3, 0.05, 0.3], rot: [90.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cyl, pos: [0.6, 0.25, 0.4], size: [0.5, 0.08, 0.5], rot: [90.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cyl, pos: [0.6, 0.25, 0.445], size: [0.3, 0.05, 0.3], rot: [90.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cyl, pos: [0.6, 0.25, -0.4], size: [0.5, 0.08, 0.5], rot: [90.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cyl, pos: [0.6, 0.25, -0.445], size: [0.3, 0.05, 0.3], rot: [90.0, 0.0, 0.0], mat: Mat::C2, id: None },
];
const ANIMS_92: &[Anim] = &[
];
const PIECES_93: &[Piece] = &[
    Piece { shape: Shape::Cube, pos: [0.0, 0.75, 0.0], size: [1.9, 0.9, 0.85], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 1.25, 0.0], size: [2.0, 0.08, 0.95], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.25, 0.0], size: [2.0, 0.12, 0.7], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cube, pos: [-0.8, 0.78, 0.43], size: [0.06, 0.7, 0.03], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cube, pos: [-0.8, 0.78, -0.43], size: [0.06, 0.7, 0.03], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cube, pos: [-0.4, 0.78, 0.43], size: [0.06, 0.7, 0.03], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cube, pos: [-0.4, 0.78, -0.43], size: [0.06, 0.7, 0.03], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.78, 0.43], size: [0.06, 0.7, 0.03], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.78, -0.43], size: [0.06, 0.7, 0.03], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cube, pos: [0.4, 0.78, 0.43], size: [0.06, 0.7, 0.03], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cube, pos: [0.4, 0.78, -0.43], size: [0.06, 0.7, 0.03], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cube, pos: [0.8, 0.78, 0.43], size: [0.06, 0.7, 0.03], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cube, pos: [0.8, 0.78, -0.43], size: [0.06, 0.7, 0.03], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.6, 0.25, 0.4], size: [0.5, 0.08, 0.5], rot: [90.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.6, 0.25, 0.445], size: [0.3, 0.05, 0.3], rot: [90.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.6, 0.25, -0.4], size: [0.5, 0.08, 0.5], rot: [90.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.6, 0.25, -0.445], size: [0.3, 0.05, 0.3], rot: [90.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cyl, pos: [0.6, 0.25, 0.4], size: [0.5, 0.08, 0.5], rot: [90.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cyl, pos: [0.6, 0.25, 0.445], size: [0.3, 0.05, 0.3], rot: [90.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cyl, pos: [0.6, 0.25, -0.4], size: [0.5, 0.08, 0.5], rot: [90.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cyl, pos: [0.6, 0.25, -0.445], size: [0.3, 0.05, 0.3], rot: [90.0, 0.0, 0.0], mat: Mat::C2, id: None },
];
const ANIMS_93: &[Anim] = &[
];
const FARWEST_TRAIN: [TrainCar; 4] = [
    TrainCar { model: Model { c2: [47, 93, 58], c3: [232, 184, 48], pieces: PIECES_90, anims: ANIMS_90 }, len: 2.9 },
    TrainCar { model: Model { c2: [47, 93, 58], c3: [232, 184, 48], pieces: PIECES_91, anims: ANIMS_91 }, len: 1.6 },
    TrainCar { model: Model { c2: [138, 90, 43], c3: [122, 77, 36], pieces: PIECES_92, anims: ANIMS_92 }, len: 2.1 },
    TrainCar { model: Model { c2: [138, 90, 43], c3: [107, 63, 29], pieces: PIECES_93, anims: ANIMS_93 }, len: 2.1 },
];
const FARWEST_COLORS: WorldColors = WorldColors {
    grass_a: [220, 182, 110],
    grass_b: [207, 168, 94],
    water: [77, 143, 176],
    road: [184, 146, 95],
    marking: [232, 217, 168],
    log: [138, 90, 43],
    sky: [242, 201, 138],
};
fn farwest_name() -> &'static str { tx!("Far West Croâ", "Wild West Croak", "Wilder Westen", "Far West Gra", "Lejano Oeste Croa").t() }
fn farwest_banner() -> &'static str { tx!("Bienvenue dans le Far West Croâ ! Ici, la loi, c'est le plus rapide à sauter.", "Welcome to the Wild West Croak! Here the law is whoever hops fastest.", "Willkommen im Wilden Westen! Hier gilt das Gesetz des schnellsten Hüpfers.", "Benvenuto nel Far West Gra! Qui la legge è chi salta più veloce.", "¡Bienvenido al Lejano Oeste Croa! Aquí manda el que salta más rápido.").t() }
fn farwest_hero_name() -> &'static str { tx!("Grenouille Cowboy", "Cowboy Frog", "Cowboy-Frosch", "Rana Cowboy", "Rana Vaquera").t() }
pub static FARWEST_THEME: Theme = Theme {
    train: &FARWEST_TRAIN, only_own: true,
    blocks: &FARWEST_BLOCKS, sides: &FARWEST_SIDES, deco: &FARWEST_DECO, animals: &FARWEST_ANIMALS,
    block_mix: &[0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 2, 2, 2, 3, 3, 4, 4], side_mix: &[100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 0, 3, 3, 3, 1, 2, 101],
    side_pct: 8, vehicles: &[], colors: FARWEST_COLORS, points: 3000,
    name: farwest_name, banner: farwest_banner, animal_name: farwest_animal_name, hero_name: farwest_hero_name,
    hero: Model { c2: [232, 194, 122], c3: [107, 63, 29], pieces: PIECES_88, anims: ANIMS_88 }, hat: Model { c2: [232, 194, 122], c3: [107, 63, 29], pieces: PIECES_89, anims: ANIMS_89 },
};
const PIECES_94: &[Piece] = &[
    Piece { shape: Shape::Sphere, pos: [0.0, 0.3, 0.0], size: [0.95, 0.62, 0.8], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Sphere, pos: [0.25, 0.55, 0.1], size: [0.55, 0.45, 0.5], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Sphere, pos: [-0.28, 0.2, -0.2], size: [0.5, 0.4, 0.45], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Sphere, pos: [0.3, 0.15, -0.3], size: [0.35, 0.3, 0.3], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Sphere, pos: [-0.1, 0.5, -0.1], size: [0.3, 0.1, 0.3], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
];
const ANIMS_94: &[Anim] = &[
];
const PIECES_95: &[Piece] = &[
    Piece { shape: Shape::Sphere, pos: [0.0, 0.35, 0.0], size: [1.0, 0.7, 0.9], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Sphere, pos: [-0.25, 0.2, 0.2], size: [0.5, 0.4, 0.5], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cone, pos: [0.15, 0.85, 0.0], size: [0.22, 0.6, 0.22], rot: [0.0, 0.0, -14.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cone, pos: [-0.1, 0.75, 0.15], size: [0.16, 0.45, 0.16], rot: [10.0, 0.0, 14.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Sphere, pos: [0.15, 1.15, 0.0], size: [0.07, 0.07, 0.07], rot: [0.0, 0.0, 0.0], mat: Mat::LumiereBleue, id: Some("eclat") },
];
const ANIMS_95: &[Anim] = &[
    Anim { target: "eclat", kind: AnimKind::Blink, amp: 0.0 },
];
const PIECES_96: &[Piece] = &[
    Piece { shape: Shape::Sphere, pos: [0.0, 0.45, 0.0], size: [0.8, 0.9, 0.75], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Sphere, pos: [0.1, 0.95, 0.05], size: [0.55, 0.7, 0.5], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Sphere, pos: [-0.2, 0.2, -0.2], size: [0.5, 0.4, 0.5], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Sphere, pos: [0.25, 0.3, 0.25], size: [0.4, 0.35, 0.4], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Sphere, pos: [0.0, 1.25, 0.0], size: [0.3, 0.3, 0.3], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
];
const ANIMS_96: &[Anim] = &[
];
const ESPACE_BLOCKS: [Model; 3] = [
    Model { c2: [156, 158, 168], c3: [106, 108, 118], pieces: PIECES_94, anims: ANIMS_94 },
    Model { c2: [141, 143, 153], c3: [164, 107, 255], pieces: PIECES_95, anims: ANIMS_95 },
    Model { c2: [127, 129, 139], c3: [90, 92, 102], pieces: PIECES_96, anims: ANIMS_96 },
];
const PIECES_97: &[Piece] = &[
    Piece { shape: Shape::Cyl, pos: [0.0, 1.6, 0.0], size: [0.9, 3.2, 0.9], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cone, pos: [0.0, 3.6, 0.0], size: [0.9, 1.0, 0.9], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [0.0, 2.4, 0.0], size: [0.92, 0.12, 0.92], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Sphere, pos: [0.45, 2.7, 0.0], size: [0.3, 0.3, 0.15], rot: [0.0, 0.0, 0.0], mat: Mat::Bulle, id: None },
    Piece { shape: Shape::Cube, pos: [0.7, 0.7, 0.0], size: [0.7, 1.4, 0.08], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [-0.7, 0.7, 0.0], size: [0.7, 1.4, 0.08], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.7, 0.7], size: [0.08, 1.4, 0.7], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.7, -0.7], size: [0.08, 1.4, 0.7], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.06, 0.0], size: [1.8, 0.12, 1.8], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cone, pos: [0.0, 0.35, 0.0], size: [0.55, 0.6, 0.55], rot: [180.0, 0.0, 0.0], mat: Mat::LumiereRouge, id: Some("flamme") },
];
const ANIMS_97: &[Anim] = &[
    Anim { target: "flamme", kind: AnimKind::Pulse, amp: 0.0 },
];
const PIECES_98: &[Piece] = &[
    Piece { shape: Shape::Sphere, pos: [0.0, 3.0, 0.0], size: [2.4, 2.4, 2.4], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cyl, pos: [0.0, 3.0, 0.0], size: [4.0, 0.06, 4.0], rot: [0.0, 0.0, 22.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [0.0, 3.0, 0.0], size: [3.2, 0.08, 3.2], rot: [0.0, 0.0, 22.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 3.35, 0.3], size: [2.3, 0.1, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [0.0, 0.9, 0.0], size: [0.1, 1.8, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.05, 0.0], size: [1.0, 0.1, 1.0], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
];
const ANIMS_98: &[Anim] = &[
];
const PIECES_99: &[Piece] = &[
    Piece { shape: Shape::Cyl, pos: [0.0, 1.2, 0.0], size: [0.5, 2.4, 0.5], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 2.5, 0.0], size: [1.1, 0.5, 1.1], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 2.5, 0.56], size: [1.0, 0.3, 0.02], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 2.5, -0.56], size: [1.0, 0.3, 0.02], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [0.56, 2.5, 0.0], size: [0.02, 0.3, 1.0], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [-0.56, 2.5, 0.0], size: [0.02, 0.3, 1.0], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 2.82, 0.0], size: [1.3, 0.1, 1.3], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cyl, pos: [0.0, 3.1, 0.0], size: [0.06, 0.5, 0.06], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cube, pos: [0.3, 3.4, 0.0], size: [0.7, 0.08, 0.3], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("radar") },
    Piece { shape: Shape::Sphere, pos: [0.0, 3.4, 0.0], size: [0.12, 0.12, 0.12], rot: [0.0, 0.0, 0.0], mat: Mat::LumiereRouge, id: Some("voyant") },
    Piece { shape: Shape::Cube, pos: [0.0, 0.06, 0.0], size: [1.3, 0.12, 1.3], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
];
const ANIMS_99: &[Anim] = &[
    Anim { target: "voyant", kind: AnimKind::Blink, amp: 0.0 },
    Anim { target: "radar", kind: AnimKind::TailY, amp: 0.0 },
];
const PIECES_100: &[Piece] = &[
    Piece { shape: Shape::Cone, pos: [0.0, 1.5, 0.0], size: [1.0, 3.0, 1.0], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cone, pos: [0.8, 1.0, 0.3], size: [0.7, 2.0, 0.7], rot: [0.0, 0.0, -16.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cone, pos: [-0.8, 0.9, -0.2], size: [0.6, 1.8, 0.6], rot: [0.0, 0.0, 18.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cone, pos: [0.2, 0.8, -0.8], size: [0.55, 1.6, 0.55], rot: [-16.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cone, pos: [-0.1, 0.7, 0.8], size: [0.5, 1.4, 0.5], rot: [16.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Sphere, pos: [0.0, 3.05, 0.0], size: [0.18, 0.18, 0.18], rot: [0.0, 0.0, 0.0], mat: Mat::LumiereBleue, id: Some("eclat") },
    Piece { shape: Shape::Sphere, pos: [0.0, 0.1, 0.0], size: [2.2, 0.25, 2.2], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
];
const ANIMS_100: &[Anim] = &[
    Anim { target: "eclat", kind: AnimKind::Pulse, amp: 0.0 },
];
const ESPACE_SIDES: [Model; 4] = [
    Model { c2: [244, 246, 250], c3: [232, 67, 58], pieces: PIECES_97, anims: ANIMS_97 },
    Model { c2: [255, 154, 74], c3: [255, 224, 160], pieces: PIECES_98, anims: ANIMS_98 },
    Model { c2: [232, 236, 244], c3: [55, 230, 255], pieces: PIECES_99, anims: ANIMS_99 },
    Model { c2: [164, 107, 255], c3: [95, 243, 255], pieces: PIECES_100, anims: ANIMS_100 },
];
const PIECES_101: &[Piece] = &[
    Piece { shape: Shape::Sphere, pos: [0.0, 0.07, 0.0], size: [0.22, 0.14, 0.2], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Sphere, pos: [0.18, 0.04, 0.1], size: [0.12, 0.08, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
];
const ANIMS_101: &[Anim] = &[
];
const PIECES_102: &[Piece] = &[
    Piece { shape: Shape::Cone, pos: [0.0, 0.15, 0.0], size: [0.12, 0.3, 0.12], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cone, pos: [0.1, 0.1, 0.03], size: [0.08, 0.2, 0.08], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cone, pos: [-0.08, 0.08, -0.04], size: [0.07, 0.16, 0.07], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
];
const ANIMS_102: &[Anim] = &[
];
const ESPACE_DECO: [Model; 2] = [
    Model { c2: [154, 157, 176], c3: [108, 111, 130], pieces: PIECES_101, anims: ANIMS_101 },
    Model { c2: [164, 107, 255], c3: [95, 243, 255], pieces: PIECES_102, anims: ANIMS_102 },
];
const PIECES_103: &[Piece] = &[
    Piece { shape: Shape::Sphere, pos: [0.0, 0.3, 0.0], size: [0.5, 0.4, 0.45], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("roche") },
    Piece { shape: Shape::Sphere, pos: [0.12, 0.42, 0.1], size: [0.28, 0.25, 0.26], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("roche") },
    Piece { shape: Shape::Sphere, pos: [-0.14, 0.2, -0.1], size: [0.24, 0.2, 0.22], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("roche") },
];
const ANIMS_103: &[Anim] = &[
    Anim { target: "roche", kind: AnimKind::SpinGroup, amp: 0.0 },
];
const PIECES_104: &[Piece] = &[
    Piece { shape: Shape::Sphere, pos: [0.0, 0.5, 0.0], size: [1.0, 0.8, 0.9], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("roche") },
    Piece { shape: Shape::Sphere, pos: [0.3, 0.7, 0.2], size: [0.55, 0.5, 0.5], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("roche") },
    Piece { shape: Shape::Sphere, pos: [-0.3, 0.35, -0.25], size: [0.5, 0.45, 0.5], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("roche") },
    Piece { shape: Shape::Sphere, pos: [0.1, 0.25, 0.35], size: [0.4, 0.35, 0.4], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("roche") },
    Piece { shape: Shape::Sphere, pos: [-0.1, 0.75, -0.3], size: [0.35, 0.3, 0.35], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("roche") },
    Piece { shape: Shape::Sphere, pos: [0.4, 0.4, -0.35], size: [0.3, 0.25, 0.3], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("roche") },
];
const ANIMS_104: &[Anim] = &[
    Anim { target: "roche", kind: AnimKind::SpinGroup, amp: 0.0 },
];
const PIECES_105: &[Piece] = &[
    Piece { shape: Shape::Sphere, pos: [0.0, 0.8, 0.0], size: [1.7, 1.4, 1.5], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("roche") },
    Piece { shape: Shape::Sphere, pos: [0.5, 1.2, 0.3], size: [0.9, 0.8, 0.8], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("roche") },
    Piece { shape: Shape::Sphere, pos: [-0.5, 0.6, -0.5], size: [0.9, 0.8, 0.8], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("roche") },
    Piece { shape: Shape::Sphere, pos: [0.2, 0.4, 0.6], size: [0.7, 0.6, 0.7], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("roche") },
    Piece { shape: Shape::Sphere, pos: [-0.3, 1.2, -0.3], size: [0.6, 0.5, 0.6], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("roche") },
    Piece { shape: Shape::Sphere, pos: [0.8, 0.6, -0.5], size: [0.5, 0.4, 0.5], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("roche") },
    Piece { shape: Shape::Sphere, pos: [-0.9, 0.9, 0.3], size: [0.5, 0.4, 0.5], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("roche") },
];
const ANIMS_105: &[Anim] = &[
    Anim { target: "roche", kind: AnimKind::SpinGroup, amp: 0.0 },
];
const PIECES_106: &[Piece] = &[
    Piece { shape: Shape::Sphere, pos: [0.0, 0.9, 0.0], size: [1.9, 0.38, 1.9], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [0.0, 1.12, 0.0], size: [0.95, 0.65, 0.95], rot: [0.0, 0.0, 0.0], mat: Mat::Bulle, id: None },
    Piece { shape: Shape::Sphere, pos: [0.0, 1.12, 0.0], size: [0.3, 0.32, 0.3], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Sphere, pos: [0.1, 1.16, 0.1], size: [0.1, 0.1, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: None },
    Piece { shape: Shape::Sphere, pos: [0.1, 1.16, -0.1], size: [0.1, 0.1, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: None },
    Piece { shape: Shape::Cyl, pos: [0.0, 0.68, 0.0], size: [0.7, 0.14, 0.7], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Sphere, pos: [0.78, 0.86, 0.0], size: [0.16, 0.12, 0.16], rot: [0.0, 0.0, 0.0], mat: Mat::LumiereRouge, id: Some("feu_b") },
    Piece { shape: Shape::Sphere, pos: [0.552, 0.86, 0.552], size: [0.16, 0.12, 0.16], rot: [0.0, 0.0, 0.0], mat: Mat::LumiereBleue, id: Some("feu_a") },
    Piece { shape: Shape::Sphere, pos: [0.0, 0.86, 0.78], size: [0.16, 0.12, 0.16], rot: [0.0, 0.0, 0.0], mat: Mat::LumiereRouge, id: Some("feu_b") },
    Piece { shape: Shape::Sphere, pos: [-0.552, 0.86, 0.552], size: [0.16, 0.12, 0.16], rot: [0.0, 0.0, 0.0], mat: Mat::LumiereBleue, id: Some("feu_a") },
    Piece { shape: Shape::Sphere, pos: [-0.78, 0.86, 0.0], size: [0.16, 0.12, 0.16], rot: [0.0, 0.0, 0.0], mat: Mat::LumiereRouge, id: Some("feu_b") },
    Piece { shape: Shape::Sphere, pos: [-0.552, 0.86, -0.552], size: [0.16, 0.12, 0.16], rot: [0.0, 0.0, 0.0], mat: Mat::LumiereBleue, id: Some("feu_a") },
    Piece { shape: Shape::Sphere, pos: [-0.0, 0.86, -0.78], size: [0.16, 0.12, 0.16], rot: [0.0, 0.0, 0.0], mat: Mat::LumiereRouge, id: Some("feu_b") },
    Piece { shape: Shape::Sphere, pos: [0.552, 0.86, -0.552], size: [0.16, 0.12, 0.16], rot: [0.0, 0.0, 0.0], mat: Mat::LumiereBleue, id: Some("feu_a") },
    Piece { shape: Shape::Cube, pos: [0.0, 0.42, 0.0], size: [0.4, 0.5, 0.4], rot: [0.0, 0.0, 0.0], mat: Mat::LumiereBleue, id: Some("rayon") },
];
const ANIMS_106: &[Anim] = &[
    Anim { target: "corps", kind: AnimKind::Bob, amp: 0.05 },
    Anim { target: "feu_a", kind: AnimKind::Blink, amp: 0.0 },
    Anim { target: "feu_b", kind: AnimKind::BlinkAlt, amp: 0.0 },
    Anim { target: "rayon", kind: AnimKind::BlinkSlow, amp: 0.0 },
];
const ESPACE_ANIMALS: [Animal; 4] = [
    Animal { model: Model { c2: [156, 158, 168], c3: [106, 108, 118], pieces: PIECES_103, anims: ANIMS_103 }, len: 0.6, width: 0.5, points: 300, dangerous: false, flying: false },
    Animal { model: Model { c2: [156, 158, 168], c3: [106, 108, 118], pieces: PIECES_104, anims: ANIMS_104 }, len: 1.1, width: 1.0, points: 600, dangerous: true, flying: false },
    Animal { model: Model { c2: [134, 136, 147], c3: [90, 92, 102], pieces: PIECES_105, anims: ANIMS_105 }, len: 1.9, width: 1.7, points: 1900, dangerous: true, flying: false },
    Animal { model: Model { c2: [200, 208, 224], c3: [55, 230, 255], pieces: PIECES_106, anims: ANIMS_106 }, len: 1.9, width: 1.9, points: 3000, dangerous: true, flying: true },
];
fn espace_animal_name(i: usize) -> &'static str {
    match i {
        0 => tx!("Caillou Spatial", "Space Pebble", "Weltraumkiesel", "Sassolino Spaziale", "Guijarro Espacial").t(),
        1 => tx!("Astéroïde Qui Roule", "Rolling Asteroid", "Rollender Asteroid", "Asteroide Rotolante", "Asteroide Rodante").t(),
        2 => tx!("Gros Caillou Pas Aimable", "Big Unfriendly Rock", "Großer Unfreundlicher Brocken", "Grosso Masso Antipatico", "Roca Grande Antipática").t(),
        3 => tx!("Soucoupe Curieuse", "Curious Saucer", "Neugierige Untertasse", "Disco Curioso", "Platillo Curioso").t(),
        _ => "?",
    }
}
const PIECES_107: &[Piece] = &[
    Piece { shape: Shape::Sphere, pos: [-0.05, 0.26, 0.0], size: [0.5, 0.34, 0.46], rot: [0.0, 0.0, 0.0], mat: Mat::Peinture, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [0.0, 0.17, 0.0], size: [0.38, 0.2, 0.4], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Sphere, pos: [0.2, 0.36, 0.0], size: [0.36, 0.26, 0.4], rot: [0.0, 0.0, 0.0], mat: Mat::Peinture, id: None },
    Piece { shape: Shape::Cube, pos: [0.37, 0.31, 0.0], size: [0.02, 0.02, 0.3], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: None },
    Piece { shape: Shape::Sphere, pos: [0.2, 0.54, 0.14], size: [0.15, 0.15, 0.15], rot: [0.0, 0.0, 0.0], mat: Mat::Peinture, id: None },
    Piece { shape: Shape::Sphere, pos: [0.25, 0.56, 0.14], size: [0.11, 0.11, 0.11], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: None },
    Piece { shape: Shape::Sphere, pos: [0.3, 0.57, 0.14], size: [0.06, 0.06, 0.06], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: None },
    Piece { shape: Shape::Sphere, pos: [-0.2, 0.2, 0.26], size: [0.22, 0.2, 0.14], rot: [0.0, 0.0, 0.0], mat: Mat::Peinture, id: None },
    Piece { shape: Shape::Cube, pos: [-0.1, 0.07, 0.27], size: [0.14, 0.1, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::Peinture, id: None },
    Piece { shape: Shape::Cube, pos: [-0.04, 0.025, 0.27], size: [0.26, 0.05, 0.12], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cyl, pos: [0.18, 0.1, 0.2], size: [0.07, 0.2, 0.07], rot: [0.0, 0.0, 0.0], mat: Mat::Peinture, id: None },
    Piece { shape: Shape::Cube, pos: [0.22, 0.015, 0.2], size: [0.1, 0.03, 0.08], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Sphere, pos: [0.2, 0.54, -0.14], size: [0.15, 0.15, 0.15], rot: [0.0, 0.0, 0.0], mat: Mat::Peinture, id: None },
    Piece { shape: Shape::Sphere, pos: [0.25, 0.56, -0.14], size: [0.11, 0.11, 0.11], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: None },
    Piece { shape: Shape::Sphere, pos: [0.3, 0.57, -0.14], size: [0.06, 0.06, 0.06], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: None },
    Piece { shape: Shape::Sphere, pos: [-0.2, 0.2, -0.26], size: [0.22, 0.2, 0.14], rot: [0.0, 0.0, 0.0], mat: Mat::Peinture, id: None },
    Piece { shape: Shape::Cube, pos: [-0.1, 0.07, -0.27], size: [0.14, 0.1, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::Peinture, id: None },
    Piece { shape: Shape::Cube, pos: [-0.04, 0.025, -0.27], size: [0.26, 0.05, 0.12], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cyl, pos: [0.18, 0.1, -0.2], size: [0.07, 0.2, 0.07], rot: [0.0, 0.0, 0.0], mat: Mat::Peinture, id: None },
    Piece { shape: Shape::Cube, pos: [0.22, 0.015, -0.2], size: [0.1, 0.03, 0.08], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Sphere, pos: [-0.12, 0.42, 0.1], size: [0.08, 0.05, 0.08], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Sphere, pos: [-0.18, 0.4, -0.08], size: [0.06, 0.04, 0.06], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
];
const ANIMS_107: &[Anim] = &[
    Anim { target: "corps", kind: AnimKind::Bob, amp: 0.02 },
];
const PIECES_108: &[Piece] = &[
    Piece { shape: Shape::Sphere, pos: [0.2, 0.46, 0.0], size: [0.7, 0.66, 0.7], rot: [0.0, 0.0, 0.0], mat: Mat::Bulle, id: Some("globe") },
    Piece { shape: Shape::Cyl, pos: [0.1, 0.2, 0.0], size: [0.5, 0.07, 0.5], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: None },
    Piece { shape: Shape::Cube, pos: [-0.28, 0.34, 0.0], size: [0.22, 0.34, 0.36], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: None },
    Piece { shape: Shape::Cube, pos: [-0.28, 0.34, 0.0], size: [0.24, 0.1, 0.38], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [0.0, 0.8, 0.0], size: [0.025, 0.2, 0.025], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Sphere, pos: [0.0, 0.92, 0.0], size: [0.07, 0.07, 0.07], rot: [0.0, 0.0, 0.0], mat: Mat::LumiereRouge, id: Some("balise") },
];
const ANIMS_108: &[Anim] = &[
    Anim { target: "balise", kind: AnimKind::Blink, amp: 0.0 },
];
const ESPACE_TRAIN: [TrainCar; 0] = [
];
const ESPACE_COLORS: WorldColors = WorldColors {
    grass_a: [141, 143, 153],
    grass_b: [127, 129, 139],
    water: [5, 6, 15],
    road: [42, 44, 58],
    marking: [55, 230, 255],
    log: [154, 163, 181],
    sky: [11, 16, 48],
};
fn espace_name() -> &'static str { tx!("Cosmos Croâ", "Croak Cosmos", "Quak-Kosmos", "Cosmo Gra", "Cosmos Croa").t() }
fn espace_banner() -> &'static str { tx!("Bienvenue dans le Cosmos Croâ ! Gardez votre casque, il n'y a pas d'air, mais il y a des Martiens.", "Welcome to the Croak Cosmos! Keep your helmet on: no air, but plenty of Martians.", "Willkommen im Quak-Kosmos! Helm aufbehalten: keine Luft, aber jede Menge Marsianer.", "Benvenuto nel Cosmo Gra! Tieni il casco: niente aria, ma tanti marziani.", "¡Bienvenido al Cosmos Croa! No te quites el casco: no hay aire, pero sí marcianos.").t() }
fn espace_hero_name() -> &'static str { tx!("Grenouille Astronaute", "Astronaut Frog", "Astronauten-Frosch", "Rana Astronauta", "Rana Astronauta").t() }
pub static ESPACE_THEME: Theme = Theme {
    train: &ESPACE_TRAIN, only_own: true,
    blocks: &ESPACE_BLOCKS, sides: &ESPACE_SIDES, deco: &ESPACE_DECO, animals: &ESPACE_ANIMALS,
    block_mix: &[0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2], side_mix: &[100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 101, 101, 101, 101, 101, 102, 102, 102, 102, 102],
    side_pct: 0, vehicles: &[42, 43, 44, 45, 46], colors: ESPACE_COLORS, points: 2400,
    name: espace_name, banner: espace_banner, animal_name: espace_animal_name, hero_name: espace_hero_name,
    hero: Model { c2: [244, 246, 250], c3: [255, 138, 42], pieces: PIECES_107, anims: ANIMS_107 }, hat: Model { c2: [244, 246, 250], c3: [255, 138, 42], pieces: PIECES_108, anims: ANIMS_108 },
};
const PIECES_109: &[Piece] = &[
    Piece { shape: Shape::Cube, pos: [0.0, 0.07, 0.0], size: [0.85, 0.14, 0.45], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.5, 0.0], size: [0.6, 0.8, 0.14], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Sphere, pos: [0.0, 0.9, 0.0], size: [0.6, 0.36, 0.14], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.65, 0.075], size: [0.34, 0.04, 0.012], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.55, 0.075], size: [0.26, 0.04, 0.012], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.45, 0.075], size: [0.3, 0.04, 0.012], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: None },
    Piece { shape: Shape::Sphere, pos: [0.36, 0.1, 0.1], size: [0.3, 0.14, 0.26], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Sphere, pos: [-0.3, 0.05, 0.15], size: [0.22, 0.1, 0.2], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
];
const ANIMS_109: &[Anim] = &[
];
const PIECES_110: &[Piece] = &[
    Piece { shape: Shape::Sphere, pos: [0.0, 0.32, 0.0], size: [0.85, 0.62, 0.85], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Sphere, pos: [0.26, 0.32, 0.0], size: [0.45, 0.62, 0.8], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Sphere, pos: [-0.26, 0.32, 0.0], size: [0.45, 0.62, 0.8], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cyl, pos: [0.0, 0.68, 0.0], size: [0.12, 0.22, 0.12], rot: [0.0, 0.0, 12.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [0.16, 0.4, 0.4], size: [0.12, 0.12, 0.05], rot: [0.0, 0.0, 0.0], mat: Mat::Phare, id: Some("feu") },
    Piece { shape: Shape::Cube, pos: [-0.16, 0.4, 0.4], size: [0.12, 0.12, 0.05], rot: [0.0, 0.0, 0.0], mat: Mat::Phare, id: Some("feu") },
    Piece { shape: Shape::Cube, pos: [0.0, 0.22, 0.4], size: [0.34, 0.08, 0.05], rot: [0.0, 0.0, 0.0], mat: Mat::Phare, id: Some("feu") },
];
const ANIMS_110: &[Anim] = &[
    Anim { target: "feu", kind: AnimKind::Pulse, amp: 0.0 },
];
const PIECES_111: &[Piece] = &[
    Piece { shape: Shape::Cyl, pos: [0.0, 0.55, 0.0], size: [0.18, 1.1, 0.18], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cyl, pos: [0.0, 0.04, 0.0], size: [0.34, 0.08, 0.34], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cone, pos: [0.25, 0.95, 0.0], size: [0.09, 0.6, 0.09], rot: [0.0, 0.0, -55.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cone, pos: [-0.22, 0.85, 0.1], size: [0.08, 0.55, 0.08], rot: [20.0, 0.0, 60.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cone, pos: [0.05, 1.2, 0.2], size: [0.07, 0.5, 0.07], rot: [-50.0, 0.0, 8.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cone, pos: [0.4, 1.2, 0.0], size: [0.05, 0.3, 0.05], rot: [0.0, 0.0, -20.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cone, pos: [-0.4, 1.05, 0.2], size: [0.05, 0.3, 0.05], rot: [0.0, 0.0, 25.0], mat: Mat::C2, id: None },
];
const ANIMS_111: &[Anim] = &[
];
const PIECES_112: &[Piece] = &[
    Piece { shape: Shape::Sphere, pos: [0.0, 0.32, 0.0], size: [0.85, 0.62, 0.85], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cyl, pos: [0.0, 0.58, 0.0], size: [0.8, 0.08, 0.8], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cyl, pos: [0.0, 0.6, 0.0], size: [0.62, 0.03, 0.62], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Sphere, pos: [0.1, 0.7, 0.05], size: [0.14, 0.14, 0.14], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("bulle") },
    Piece { shape: Shape::Sphere, pos: [-0.12, 0.7, -0.08], size: [0.1, 0.1, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("bulle") },
    Piece { shape: Shape::Cyl, pos: [0.3, 0.06, 0.3], size: [0.1, 0.14, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.3, 0.06, 0.3], size: [0.1, 0.14, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cyl, pos: [0.0, 0.06, -0.38], size: [0.1, 0.14, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Sphere, pos: [0.0, 0.1, 0.0], size: [0.3, 0.15, 0.3], rot: [0.0, 0.0, 0.0], mat: Mat::LumiereRouge, id: Some("feu") },
];
const ANIMS_112: &[Anim] = &[
    Anim { target: "bulle", kind: AnimKind::DriftUp, amp: 0.0 },
    Anim { target: "feu", kind: AnimKind::Pulse, amp: 0.0 },
];
const HALLOWEEN_BLOCKS: [Model; 4] = [
    Model { c2: [141, 143, 153], c3: [79, 122, 74], pieces: PIECES_109, anims: ANIMS_109 },
    Model { c2: [255, 138, 31], c3: [74, 138, 58], pieces: PIECES_110, anims: ANIMS_110 },
    Model { c2: [74, 58, 48], c3: [46, 36, 32], pieces: PIECES_111, anims: ANIMS_111 },
    Model { c2: [42, 42, 48], c3: [122, 255, 90], pieces: PIECES_112, anims: ANIMS_112 },
];
const PIECES_113: &[Piece] = &[
    Piece { shape: Shape::Cube, pos: [0.0, 0.9, 0.0], size: [2.0, 1.8, 1.4], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cone, pos: [0.0, 2.4, 0.0], size: [2.5, 1.3, 1.8], rot: [0.0, 45.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cube, pos: [-0.8, 2.0, 0.3], size: [0.6, 2.8, 0.6], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cone, pos: [-0.8, 3.7, 0.3], size: [0.8, 0.9, 0.8], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cube, pos: [0.5, 1.2, 0.71], size: [0.28, 0.42, 0.03], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("fenetre") },
    Piece { shape: Shape::Cube, pos: [-0.1, 1.2, 0.71], size: [0.28, 0.42, 0.03], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [-0.8, 2.4, 0.61], size: [0.22, 0.4, 0.03], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("fenetre") },
    Piece { shape: Shape::Cube, pos: [0.25, 0.5, 0.71], size: [0.4, 0.9, 0.03], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.05, 0.3], size: [2.3, 0.1, 1.9], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
];
const ANIMS_113: &[Anim] = &[
    Anim { target: "fenetre", kind: AnimKind::BlinkSlow, amp: 0.0 },
];
const PIECES_114: &[Piece] = &[
    Piece { shape: Shape::Cyl, pos: [0.0, 1.2, 0.0], size: [0.5, 2.4, 0.5], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cone, pos: [0.7, 2.3, 0.0], size: [0.22, 1.6, 0.22], rot: [0.0, 0.0, -50.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cone, pos: [-0.7, 2.1, 0.2], size: [0.2, 1.5, 0.2], rot: [15.0, 0.0, 52.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cone, pos: [0.1, 2.6, 0.6], size: [0.18, 1.3, 0.18], rot: [-48.0, 0.0, 6.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cone, pos: [-0.1, 2.5, -0.6], size: [0.18, 1.3, 0.18], rot: [48.0, 0.0, -6.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cone, pos: [1.2, 2.9, 0.0], size: [0.1, 0.7, 0.1], rot: [0.0, 0.0, -25.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cone, pos: [-1.25, 2.7, 0.2], size: [0.1, 0.7, 0.1], rot: [0.0, 0.0, 28.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.05, 0.0], size: [1.2, 0.1, 1.2], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
];
const ANIMS_114: &[Anim] = &[
];
const HALLOWEEN_SIDES: [Model; 2] = [
    Model { c2: [61, 53, 80], c3: [255, 207, 90], pieces: PIECES_113, anims: ANIMS_113 },
    Model { c2: [74, 58, 48], c3: [46, 36, 32], pieces: PIECES_114, anims: ANIMS_114 },
];
const PIECES_115: &[Piece] = &[
    Piece { shape: Shape::Cyl, pos: [0.0, 0.1, 0.0], size: [0.08, 0.2, 0.08], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Sphere, pos: [0.0, 0.24, 0.0], size: [0.06, 0.1, 0.06], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("flamme") },
];
const ANIMS_115: &[Anim] = &[
    Anim { target: "flamme", kind: AnimKind::BlinkAlt, amp: 0.0 },
];
const PIECES_116: &[Piece] = &[
    Piece { shape: Shape::Sphere, pos: [0.0, 0.1, 0.0], size: [0.24, 0.2, 0.24], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cyl, pos: [0.0, 0.23, 0.0], size: [0.04, 0.08, 0.04], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
];
const ANIMS_116: &[Anim] = &[
];
const PIECES_117: &[Piece] = &[
    Piece { shape: Shape::Cube, pos: [0.0, 0.03, 0.0], size: [0.3, 0.05, 0.07], rot: [0.0, 30.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Sphere, pos: [0.15, 0.04, 0.05], size: [0.1, 0.07, 0.09], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Sphere, pos: [-0.15, 0.04, -0.05], size: [0.1, 0.07, 0.09], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
];
const ANIMS_117: &[Anim] = &[
];
const PIECES_118: &[Piece] = &[
    Piece { shape: Shape::Cube, pos: [0.0, 0.01, 0.0], size: [0.5, 0.01, 0.03], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.01, 0.0], size: [0.03, 0.01, 0.5], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.01, 0.0], size: [0.44, 0.01, 0.03], rot: [0.0, 45.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.01, 0.0], size: [0.44, 0.01, 0.03], rot: [0.0, -45.0, 0.0], mat: Mat::C3, id: None },
];
const ANIMS_118: &[Anim] = &[
];
const HALLOWEEN_DECO: [Model; 4] = [
    Model { c2: [244, 240, 224], c3: [255, 207, 90], pieces: PIECES_115, anims: ANIMS_115 },
    Model { c2: [255, 138, 31], c3: [74, 138, 58], pieces: PIECES_116, anims: ANIMS_116 },
    Model { c2: [241, 234, 216], c3: [216, 208, 188], pieces: PIECES_117, anims: ANIMS_117 },
    Model { c2: [200, 200, 208], c3: [154, 154, 164], pieces: PIECES_118, anims: ANIMS_118 },
];
const PIECES_119: &[Piece] = &[
    Piece { shape: Shape::Sphere, pos: [0.0, 0.32, 0.0], size: [0.7, 0.6, 0.7], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("boule") },
    Piece { shape: Shape::Cyl, pos: [0.0, 0.66, 0.0], size: [0.1, 0.2, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("boule") },
    Piece { shape: Shape::Cube, pos: [0.34, 0.4, 0.14], size: [0.05, 0.12, 0.12], rot: [0.0, 0.0, 0.0], mat: Mat::Phare, id: Some("boule") },
    Piece { shape: Shape::Cube, pos: [0.34, 0.4, -0.14], size: [0.05, 0.12, 0.12], rot: [0.0, 0.0, 0.0], mat: Mat::Phare, id: Some("boule") },
    Piece { shape: Shape::Cube, pos: [0.34, 0.22, 0.0], size: [0.05, 0.08, 0.3], rot: [0.0, 0.0, 0.0], mat: Mat::Phare, id: Some("boule") },
];
const ANIMS_119: &[Anim] = &[
    Anim { target: "boule", kind: AnimKind::SpinGroup, amp: 0.0 },
];
const PIECES_120: &[Piece] = &[
    Piece { shape: Shape::Sphere, pos: [0.0, 0.6, 0.0], size: [0.55, 0.75, 0.55], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [0.0, 0.32, 0.0], size: [0.62, 0.5, 0.62], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [0.26, 0.15, 0.0], size: [0.2, 0.22, 0.2], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [0.0, 0.15, 0.26], size: [0.2, 0.22, 0.2], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [-0.26, 0.15, 0.0], size: [0.2, 0.22, 0.2], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [-0.001, 0.15, -0.26], size: [0.2, 0.22, 0.2], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [0.24, 0.68, 0.1], size: [0.09, 0.09, 0.09], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: None },
    Piece { shape: Shape::Sphere, pos: [0.24, 0.68, -0.1], size: [0.09, 0.09, 0.09], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: None },
    Piece { shape: Shape::Sphere, pos: [0.27, 0.5, 0.0], size: [0.1, 0.12, 0.08], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [0.0, 0.5, 0.34], size: [0.18, 0.2, 0.12], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [0.0, 0.5, -0.34], size: [0.18, 0.2, 0.12], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
];
const ANIMS_120: &[Anim] = &[
    Anim { target: "corps", kind: AnimKind::Bob, amp: 0.08 },
];
const PIECES_121: &[Piece] = &[
    Piece { shape: Shape::Sphere, pos: [0.0, 0.4, 0.0], size: [0.62, 0.34, 0.3], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [0.34, 0.5, 0.0], size: [0.3, 0.28, 0.28], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Cone, pos: [0.32, 0.72, 0.1], size: [0.1, 0.16, 0.06], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Cone, pos: [0.32, 0.72, -0.1], size: [0.1, 0.16, 0.06], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [0.46, 0.55, 0.08], size: [0.07, 0.07, 0.05], rot: [0.0, 0.0, 0.0], mat: Mat::Phare, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [0.46, 0.55, -0.08], size: [0.07, 0.07, 0.05], rot: [0.0, 0.0, 0.0], mat: Mat::Phare, id: Some("corps") },
    Piece { shape: Shape::Cyl, pos: [-0.45, 0.6, 0.0], size: [0.07, 0.5, 0.07], rot: [0.0, 0.0, 25.0], mat: Mat::C2, id: Some("queue") },
    Piece { shape: Shape::Cyl, pos: [0.2, 0.2, 0.09], size: [0.08, 0.4, 0.08], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("patte0") },
    Piece { shape: Shape::Cyl, pos: [0.2, 0.2, -0.09], size: [0.08, 0.4, 0.08], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("patte1") },
    Piece { shape: Shape::Cyl, pos: [-0.2, 0.2, 0.09], size: [0.08, 0.4, 0.08], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("patte2") },
    Piece { shape: Shape::Cyl, pos: [-0.2, 0.2, -0.09], size: [0.08, 0.4, 0.08], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("patte3") },
];
const ANIMS_121: &[Anim] = &[
    Anim { target: "corps", kind: AnimKind::Bob, amp: 0.03 },
    Anim { target: "queue", kind: AnimKind::TailY, amp: 0.0 },
    Anim { target: "patte0", kind: AnimKind::SwingZ, amp: 0.0 },
    Anim { target: "patte1", kind: AnimKind::SwingZ, amp: 0.5 },
    Anim { target: "patte2", kind: AnimKind::SwingZ, amp: 0.5 },
    Anim { target: "patte3", kind: AnimKind::SwingZ, amp: 0.0 },
];
const PIECES_122: &[Piece] = &[
    Piece { shape: Shape::Sphere, pos: [0.04, 1.5, 0.0], size: [0.34, 0.36, 0.32], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Cube, pos: [0.2, 1.4, 0.0], size: [0.12, 0.1, 0.22], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [0.2, 1.54, 0.09], size: [0.09, 0.09, 0.09], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: None },
    Piece { shape: Shape::Sphere, pos: [0.2, 1.54, -0.09], size: [0.09, 0.09, 0.09], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 1.05, 0.0], size: [0.1, 0.6, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Cube, pos: [0.0, 1.22, 0.0], size: [0.16, 0.05, 0.4], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Cube, pos: [0.0, 1.1, 0.0], size: [0.14, 0.05, 0.36], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Cube, pos: [0.0, 0.98, 0.0], size: [0.12, 0.05, 0.3], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Cube, pos: [0.0, 0.72, 0.0], size: [0.18, 0.16, 0.34], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Cyl, pos: [0.0, 1.05, 0.26], size: [0.07, 0.55, 0.07], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("bras_g") },
    Piece { shape: Shape::Cyl, pos: [0.0, 1.05, -0.26], size: [0.07, 0.55, 0.07], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("bras_d") },
    Piece { shape: Shape::Cyl, pos: [0.0, 0.36, 0.12], size: [0.09, 0.7, 0.09], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("patte_g") },
    Piece { shape: Shape::Cube, pos: [0.08, 0.03, 0.12], size: [0.22, 0.06, 0.12], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("patte_g") },
    Piece { shape: Shape::Cyl, pos: [0.0, 0.36, -0.12], size: [0.09, 0.7, 0.09], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("patte_d") },
    Piece { shape: Shape::Cube, pos: [0.08, 0.03, -0.12], size: [0.22, 0.06, 0.12], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("patte_d") },
];
const ANIMS_122: &[Anim] = &[
    Anim { target: "corps", kind: AnimKind::Bob, amp: 0.03 },
    Anim { target: "bras_g", kind: AnimKind::SwingZ, amp: 0.5 },
    Anim { target: "bras_d", kind: AnimKind::SwingZ, amp: 0.0 },
    Anim { target: "patte_g", kind: AnimKind::SwingZ, amp: 0.0 },
    Anim { target: "patte_d", kind: AnimKind::SwingZ, amp: 0.5 },
];
const PIECES_123: &[Piece] = &[
    Piece { shape: Shape::Cyl, pos: [-0.1, 0.62, 0.0], size: [0.07, 1.7, 0.07], rot: [0.0, 0.0, 90.0], mat: Mat::C3, id: Some("balai") },
    Piece { shape: Shape::Cone, pos: [-1.0, 0.62, 0.0], size: [0.28, 0.5, 0.28], rot: [0.0, 0.0, 90.0], mat: Mat::C2, id: Some("balai") },
    Piece { shape: Shape::Cone, pos: [0.0, 0.95, 0.0], size: [0.46, 0.55, 0.4], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [0.04, 1.35, 0.0], size: [0.28, 0.3, 0.28], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Cyl, pos: [0.04, 1.5, 0.0], size: [0.62, 0.04, 0.62], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: Some("corps") },
    Piece { shape: Shape::Cone, pos: [0.04, 1.8, 0.0], size: [0.34, 0.6, 0.34], rot: [0.0, 0.0, 10.0], mat: Mat::Sombre, id: Some("corps") },
    Piece { shape: Shape::Cube, pos: [0.22, 1.32, 0.0], size: [0.12, 0.06, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Cube, pos: [0.14, 1.4, 0.07], size: [0.05, 0.05, 0.05], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("corps") },
    Piece { shape: Shape::Cube, pos: [0.14, 1.4, -0.07], size: [0.05, 0.05, 0.05], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("corps") },
    Piece { shape: Shape::Cube, pos: [-0.55, 0.9, 0.0], size: [0.5, 0.03, 0.4], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("cape") },
];
const ANIMS_123: &[Anim] = &[
    Anim { target: "corps", kind: AnimKind::Bob, amp: 0.04 },
    Anim { target: "cape", kind: AnimKind::TailY, amp: 0.0 },
    Anim { target: "balai", kind: AnimKind::Bob, amp: 0.02 },
];
const PIECES_124: &[Piece] = &[
    Piece { shape: Shape::Cube, pos: [0.0, 0.95, 0.0], size: [0.45, 0.8, 0.6], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [0.1, 1.55, 0.0], size: [0.46, 0.46, 0.44], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [0.31, 1.6, 0.12], size: [0.1, 0.1, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: None },
    Piece { shape: Shape::Sphere, pos: [0.31, 1.6, -0.12], size: [0.1, 0.1, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: None },
    Piece { shape: Shape::Cube, pos: [0.32, 1.45, 0.0], size: [0.05, 0.06, 0.2], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("corps") },
    Piece { shape: Shape::Cube, pos: [0.0, 0.5, 0.0], size: [0.4, 0.3, 0.56], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: Some("corps") },
    Piece { shape: Shape::Cube, pos: [0.4, 1.12, 0.38], size: [0.7, 0.16, 0.16], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("bras_g") },
    Piece { shape: Shape::Cube, pos: [0.4, 1.12, -0.38], size: [0.7, 0.16, 0.16], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("bras_d") },
    Piece { shape: Shape::Cyl, pos: [0.0, 0.3, 0.16], size: [0.2, 0.6, 0.2], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: Some("patte_g") },
    Piece { shape: Shape::Cube, pos: [0.1, 0.04, 0.16], size: [0.34, 0.08, 0.2], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("patte_g") },
    Piece { shape: Shape::Cyl, pos: [0.0, 0.3, -0.16], size: [0.2, 0.6, 0.2], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: Some("patte_d") },
    Piece { shape: Shape::Cube, pos: [0.1, 0.04, -0.16], size: [0.34, 0.08, 0.2], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("patte_d") },
];
const ANIMS_124: &[Anim] = &[
    Anim { target: "corps", kind: AnimKind::Bob, amp: 0.03 },
    Anim { target: "bras_g", kind: AnimKind::TailZ, amp: 0.0 },
    Anim { target: "bras_d", kind: AnimKind::TailZ, amp: 0.0 },
    Anim { target: "patte_g", kind: AnimKind::SwingZ, amp: 0.0 },
    Anim { target: "patte_d", kind: AnimKind::SwingZ, amp: 0.5 },
];
const PIECES_125: &[Piece] = &[
    Piece { shape: Shape::Sphere, pos: [0.0, 0.65, 0.0], size: [0.9, 0.6, 0.8], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [0.52, 0.7, 0.0], size: [0.45, 0.4, 0.45], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [0.72, 0.8, 0.12], size: [0.1, 0.1, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::LumiereRouge, id: None },
    Piece { shape: Shape::Sphere, pos: [0.72, 0.8, -0.12], size: [0.1, 0.1, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::LumiereRouge, id: None },
    Piece { shape: Shape::Sphere, pos: [-0.5, 0.75, 0.0], size: [0.7, 0.55, 0.65], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("corps") },
    Piece { shape: Shape::Cyl, pos: [0.38, 0.5, 0.57], size: [0.07, 0.75, 0.07], rot: [35.0, 0.0, 0.0], mat: Mat::C2, id: Some("patte0") },
    Piece { shape: Shape::Cyl, pos: [0.38, 0.2, 0.9], size: [0.06, 0.5, 0.06], rot: [-25.0, 0.0, 0.0], mat: Mat::C2, id: Some("patte0") },
    Piece { shape: Shape::Cyl, pos: [0.12, 0.5, 0.57], size: [0.07, 0.75, 0.07], rot: [35.0, 0.0, 0.0], mat: Mat::C2, id: Some("patte1") },
    Piece { shape: Shape::Cyl, pos: [0.12, 0.2, 0.9], size: [0.06, 0.5, 0.06], rot: [-25.0, 0.0, 0.0], mat: Mat::C2, id: Some("patte1") },
    Piece { shape: Shape::Cyl, pos: [-0.14, 0.5, 0.57], size: [0.07, 0.75, 0.07], rot: [35.0, 0.0, 0.0], mat: Mat::C2, id: Some("patte2") },
    Piece { shape: Shape::Cyl, pos: [-0.14, 0.2, 0.9], size: [0.06, 0.5, 0.06], rot: [-25.0, 0.0, 0.0], mat: Mat::C2, id: Some("patte2") },
    Piece { shape: Shape::Cyl, pos: [-0.4, 0.5, 0.57], size: [0.07, 0.75, 0.07], rot: [35.0, 0.0, 0.0], mat: Mat::C2, id: Some("patte3") },
    Piece { shape: Shape::Cyl, pos: [-0.4, 0.2, 0.9], size: [0.06, 0.5, 0.06], rot: [-25.0, 0.0, 0.0], mat: Mat::C2, id: Some("patte3") },
    Piece { shape: Shape::Cyl, pos: [0.38, 0.5, -0.57], size: [0.07, 0.75, 0.07], rot: [-35.0, 0.0, 0.0], mat: Mat::C2, id: Some("patte4") },
    Piece { shape: Shape::Cyl, pos: [0.38, 0.2, -0.9], size: [0.06, 0.5, 0.06], rot: [25.0, 0.0, 0.0], mat: Mat::C2, id: Some("patte4") },
    Piece { shape: Shape::Cyl, pos: [0.12, 0.5, -0.57], size: [0.07, 0.75, 0.07], rot: [-35.0, 0.0, 0.0], mat: Mat::C2, id: Some("patte5") },
    Piece { shape: Shape::Cyl, pos: [0.12, 0.2, -0.9], size: [0.06, 0.5, 0.06], rot: [25.0, 0.0, 0.0], mat: Mat::C2, id: Some("patte5") },
    Piece { shape: Shape::Cyl, pos: [-0.14, 0.5, -0.57], size: [0.07, 0.75, 0.07], rot: [-35.0, 0.0, 0.0], mat: Mat::C2, id: Some("patte6") },
    Piece { shape: Shape::Cyl, pos: [-0.14, 0.2, -0.9], size: [0.06, 0.5, 0.06], rot: [25.0, 0.0, 0.0], mat: Mat::C2, id: Some("patte6") },
    Piece { shape: Shape::Cyl, pos: [-0.4, 0.5, -0.57], size: [0.07, 0.75, 0.07], rot: [-35.0, 0.0, 0.0], mat: Mat::C2, id: Some("patte7") },
    Piece { shape: Shape::Cyl, pos: [-0.4, 0.2, -0.9], size: [0.06, 0.5, 0.06], rot: [25.0, 0.0, 0.0], mat: Mat::C2, id: Some("patte7") },
];
const ANIMS_125: &[Anim] = &[
    Anim { target: "corps", kind: AnimKind::Bob, amp: 0.03 },
    Anim { target: "patte0", kind: AnimKind::SwingZ, amp: 0.0 },
    Anim { target: "patte1", kind: AnimKind::SwingZ, amp: 0.5 },
    Anim { target: "patte2", kind: AnimKind::SwingZ, amp: 0.0 },
    Anim { target: "patte3", kind: AnimKind::SwingZ, amp: 0.5 },
    Anim { target: "patte4", kind: AnimKind::SwingZ, amp: 0.0 },
    Anim { target: "patte5", kind: AnimKind::SwingZ, amp: 0.5 },
    Anim { target: "patte6", kind: AnimKind::SwingZ, amp: 0.0 },
    Anim { target: "patte7", kind: AnimKind::SwingZ, amp: 0.5 },
];
const PIECES_126: &[Piece] = &[
    Piece { shape: Shape::Sphere, pos: [0.0, 0.9, 0.0], size: [0.55, 0.4, 0.4], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [0.34, 0.95, 0.0], size: [0.34, 0.3, 0.3], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Cone, pos: [0.34, 1.18, 0.1], size: [0.1, 0.22, 0.06], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Cone, pos: [0.34, 1.18, -0.1], size: [0.1, 0.22, 0.06], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [0.5, 1.0, 0.08], size: [0.07, 0.07, 0.05], rot: [0.0, 0.0, 0.0], mat: Mat::LumiereRouge, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [0.5, 1.0, -0.08], size: [0.07, 0.07, 0.05], rot: [0.0, 0.0, 0.0], mat: Mat::LumiereRouge, id: Some("corps") },
    Piece { shape: Shape::Cone, pos: [0.5, 0.88, 0.04], size: [0.04, 0.1, 0.03], rot: [180.0, 0.0, 0.0], mat: Mat::Blanc, id: Some("corps") },
    Piece { shape: Shape::Cone, pos: [0.5, 0.88, -0.04], size: [0.04, 0.1, 0.03], rot: [180.0, 0.0, 0.0], mat: Mat::Blanc, id: Some("corps") },
    Piece { shape: Shape::Cube, pos: [0.0, 0.92, 0.6], size: [0.5, 0.04, 0.9], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("aile_g") },
    Piece { shape: Shape::Cube, pos: [-0.1, 0.92, 1.1], size: [0.45, 0.04, 0.5], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("aile_g") },
    Piece { shape: Shape::Cube, pos: [-0.3, 0.92, 1.5], size: [0.3, 0.04, 0.35], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("aile_g") },
    Piece { shape: Shape::Cube, pos: [0.05, 0.93, 1.4], size: [0.08, 0.05, 0.8], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("aile_g") },
    Piece { shape: Shape::Cube, pos: [0.0, 0.92, -0.6], size: [0.5, 0.04, 0.9], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("aile_d") },
    Piece { shape: Shape::Cube, pos: [-0.1, 0.92, -1.1], size: [0.45, 0.04, 0.5], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("aile_d") },
    Piece { shape: Shape::Cube, pos: [-0.3, 0.92, -1.5], size: [0.3, 0.04, 0.35], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("aile_d") },
    Piece { shape: Shape::Cube, pos: [0.05, 0.93, -1.4], size: [0.08, 0.05, 0.8], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("aile_d") },
];
const ANIMS_126: &[Anim] = &[
    Anim { target: "corps", kind: AnimKind::Bob, amp: 0.04 },
    Anim { target: "aile_g", kind: AnimKind::SwingXBase, amp: 0.0 },
    Anim { target: "aile_d", kind: AnimKind::SwingXBase, amp: 0.0 },
];
const HALLOWEEN_ANIMALS: [Animal; 8] = [
    Animal { model: Model { c2: [255, 138, 31], c3: [74, 138, 58], pieces: PIECES_119, anims: ANIMS_119 }, len: 0.8, width: 0.7, points: 300, dangerous: false, flying: false },
    Animal { model: Model { c2: [244, 246, 255], c3: [200, 208, 255], pieces: PIECES_120, anims: ANIMS_120 }, len: 0.8, width: 0.7, points: 450, dangerous: false, flying: false },
    Animal { model: Model { c2: [28, 28, 36], c3: [255, 210, 63], pieces: PIECES_121, anims: ANIMS_121 }, len: 1.0, width: 0.4, points: 650, dangerous: false, flying: false },
    Animal { model: Model { c2: [241, 234, 216], c3: [216, 208, 188], pieces: PIECES_122, anims: ANIMS_122 }, len: 0.9, width: 0.7, points: 1000, dangerous: true, flying: false },
    Animal { model: Model { c2: [143, 191, 90], c3: [90, 63, 138], pieces: PIECES_123, anims: ANIMS_123 }, len: 2.2, width: 0.6, points: 1500, dangerous: true, flying: false },
    Animal { model: Model { c2: [122, 160, 90], c3: [90, 74, 122], pieces: PIECES_124, anims: ANIMS_124 }, len: 1.0, width: 0.9, points: 1900, dangerous: true, flying: false },
    Animal { model: Model { c2: [28, 28, 36], c3: [58, 42, 74], pieces: PIECES_125, anims: ANIMS_125 }, len: 1.8, width: 1.8, points: 2500, dangerous: true, flying: false },
    Animal { model: Model { c2: [42, 36, 51], c3: [74, 58, 90], pieces: PIECES_126, anims: ANIMS_126 }, len: 1.0, width: 3.0, points: 3000, dangerous: true, flying: true },
];
fn halloween_animal_name(i: usize) -> &'static str {
    match i {
        0 => tx!("Citrouille Roulante", "Rolling Pumpkin", "Rollender Kürbis", "Zucca Rotolante", "Calabaza Rodante").t(),
        1 => tx!("Petit Fantôme", "Little Ghost", "Kleines Gespenst", "Fantasmino", "Fantasmita").t(),
        2 => tx!("Chat Noir Pressé", "Hurried Black Cat", "Eiliger Kater", "Gatto Nero Frettoloso", "Gato Negro con Prisa").t(),
        3 => tx!("Squelette Danseur", "Dancing Skeleton", "Tanzendes Skelett", "Scheletro Ballerino", "Esqueleto Bailarín").t(),
        4 => tx!("Sorcière sur Balai", "Witch on a Broom", "Hexe auf dem Besen", "Strega sulla Scopa", "Bruja en Escoba").t(),
        5 => tx!("Zombie Pas Pressé", "Unhurried Zombie", "Gemütlicher Zombie", "Zombie Senza Fretta", "Zombi Sin Prisa").t(),
        6 => tx!("Araignée Géante", "Giant Spider", "Riesenspinne", "Ragno Gigante", "Araña Gigante").t(),
        7 => tx!("Chauve-Souris Colossale", "Colossal Bat", "Riesenfledermaus", "Pipistrello Colossale", "Murciélago Colosal").t(),
        _ => "?",
    }
}
const PIECES_127: &[Piece] = &[
    Piece { shape: Shape::Sphere, pos: [-0.05, 0.26, 0.0], size: [0.5, 0.34, 0.46], rot: [0.0, 0.0, 0.0], mat: Mat::Peinture, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [0.0, 0.17, 0.0], size: [0.38, 0.2, 0.4], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Sphere, pos: [0.2, 0.36, 0.0], size: [0.36, 0.26, 0.4], rot: [0.0, 0.0, 0.0], mat: Mat::Peinture, id: None },
    Piece { shape: Shape::Cube, pos: [0.37, 0.31, 0.0], size: [0.02, 0.02, 0.3], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: None },
    Piece { shape: Shape::Sphere, pos: [0.2, 0.54, 0.14], size: [0.15, 0.15, 0.15], rot: [0.0, 0.0, 0.0], mat: Mat::Peinture, id: None },
    Piece { shape: Shape::Sphere, pos: [0.25, 0.56, 0.14], size: [0.11, 0.11, 0.11], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: None },
    Piece { shape: Shape::Sphere, pos: [0.3, 0.57, 0.14], size: [0.06, 0.06, 0.06], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: None },
    Piece { shape: Shape::Sphere, pos: [-0.2, 0.2, 0.26], size: [0.22, 0.2, 0.14], rot: [0.0, 0.0, 0.0], mat: Mat::Peinture, id: None },
    Piece { shape: Shape::Cube, pos: [-0.1, 0.07, 0.27], size: [0.14, 0.1, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::Peinture, id: None },
    Piece { shape: Shape::Cube, pos: [-0.04, 0.025, 0.27], size: [0.26, 0.05, 0.12], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cyl, pos: [0.18, 0.1, 0.2], size: [0.07, 0.2, 0.07], rot: [0.0, 0.0, 0.0], mat: Mat::Peinture, id: None },
    Piece { shape: Shape::Cube, pos: [0.22, 0.015, 0.2], size: [0.1, 0.03, 0.08], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Sphere, pos: [0.2, 0.54, -0.14], size: [0.15, 0.15, 0.15], rot: [0.0, 0.0, 0.0], mat: Mat::Peinture, id: None },
    Piece { shape: Shape::Sphere, pos: [0.25, 0.56, -0.14], size: [0.11, 0.11, 0.11], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: None },
    Piece { shape: Shape::Sphere, pos: [0.3, 0.57, -0.14], size: [0.06, 0.06, 0.06], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: None },
    Piece { shape: Shape::Sphere, pos: [-0.2, 0.2, -0.26], size: [0.22, 0.2, 0.14], rot: [0.0, 0.0, 0.0], mat: Mat::Peinture, id: None },
    Piece { shape: Shape::Cube, pos: [-0.1, 0.07, -0.27], size: [0.14, 0.1, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::Peinture, id: None },
    Piece { shape: Shape::Cube, pos: [-0.04, 0.025, -0.27], size: [0.26, 0.05, 0.12], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cyl, pos: [0.18, 0.1, -0.2], size: [0.07, 0.2, 0.07], rot: [0.0, 0.0, 0.0], mat: Mat::Peinture, id: None },
    Piece { shape: Shape::Cube, pos: [0.22, 0.015, -0.2], size: [0.1, 0.03, 0.08], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Sphere, pos: [-0.12, 0.42, 0.1], size: [0.08, 0.05, 0.08], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Sphere, pos: [-0.18, 0.4, -0.08], size: [0.06, 0.04, 0.06], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
];
const ANIMS_127: &[Anim] = &[
    Anim { target: "corps", kind: AnimKind::Bob, amp: 0.02 },
];
const PIECES_128: &[Piece] = &[
    Piece { shape: Shape::Cyl, pos: [0.12, 0.69, 0.0], size: [0.7, 0.03, 0.7], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cone, pos: [0.1, 0.98, 0.0], size: [0.42, 0.6, 0.42], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cyl, pos: [0.12, 0.74, 0.0], size: [0.43, 0.06, 0.43], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [0.34, 0.76, 0.0], size: [0.05, 0.1, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::Phare, id: None },
    Piece { shape: Shape::Cone, pos: [0.0, 1.32, 0.0], size: [0.12, 0.3, 0.12], rot: [0.0, 0.0, 40.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Sphere, pos: [-0.1, 1.45, 0.0], size: [0.1, 0.1, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
];
const ANIMS_128: &[Anim] = &[
];
const PIECES_129: &[Piece] = &[
    Piece { shape: Shape::Cyl, pos: [0.35, 0.85, 0.0], size: [0.7, 1.8, 0.7], rot: [0.0, 0.0, 90.0], mat: Mat::Sombre, id: Some("corps") },
    Piece { shape: Shape::Cyl, pos: [1.28, 0.85, 0.0], size: [0.76, 0.12, 0.76], rot: [0.0, 0.0, 90.0], mat: Mat::C3, id: Some("corps") },
    Piece { shape: Shape::Cyl, pos: [1.0, 1.4, 0.0], size: [0.24, 0.5, 0.24], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: Some("corps") },
    Piece { shape: Shape::Cone, pos: [1.0, 1.75, 0.0], size: [0.46, 0.3, 0.46], rot: [180.0, 0.0, 0.0], mat: Mat::Sombre, id: Some("corps") },
    Piece { shape: Shape::Cube, pos: [-0.65, 1.0, 0.0], size: [0.85, 1.0, 0.85], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Cube, pos: [-0.65, 1.55, 0.0], size: [1.0, 0.08, 1.0], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: Some("corps") },
    Piece { shape: Shape::Cube, pos: [-0.65, 1.05, 0.43], size: [0.4, 0.4, 0.04], rot: [0.0, 0.0, 0.0], mat: Mat::Phare, id: Some("corps") },
    Piece { shape: Shape::Cube, pos: [-0.65, 1.05, -0.43], size: [0.4, 0.4, 0.04], rot: [0.0, 0.0, 0.0], mat: Mat::Phare, id: Some("corps") },
    Piece { shape: Shape::Cube, pos: [1.5, 0.3, 0.0], size: [0.5, 0.12, 0.8], rot: [0.0, 0.0, 28.0], mat: Mat::Sombre, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [1.32, 0.85, 0.0], size: [0.3, 0.3, 0.3], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [1.45, 0.92, 0.1], size: [0.1, 0.1, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [1.45, 0.92, -0.1], size: [0.1, 0.1, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("corps") },
    Piece { shape: Shape::Cube, pos: [0.35, 0.28, 0.0], size: [1.8, 0.14, 0.7], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: Some("corps") },
    Piece { shape: Shape::Cyl, pos: [-0.3, 0.38, 0.4], size: [0.76, 0.08, 0.76], rot: [90.0, 0.0, 0.0], mat: Mat::Sombre, id: Some("corps") },
    Piece { shape: Shape::Cyl, pos: [-0.3, 0.38, 0.445], size: [0.418, 0.05, 0.418], rot: [90.0, 0.0, 0.0], mat: Mat::C3, id: Some("corps") },
    Piece { shape: Shape::Cyl, pos: [-0.3, 0.38, -0.4], size: [0.76, 0.08, 0.76], rot: [90.0, 0.0, 0.0], mat: Mat::Sombre, id: Some("corps") },
    Piece { shape: Shape::Cyl, pos: [-0.3, 0.38, -0.445], size: [0.418, 0.05, 0.418], rot: [90.0, 0.0, 0.0], mat: Mat::C3, id: Some("corps") },
    Piece { shape: Shape::Cyl, pos: [0.5, 0.38, 0.4], size: [0.76, 0.08, 0.76], rot: [90.0, 0.0, 0.0], mat: Mat::Sombre, id: Some("corps") },
    Piece { shape: Shape::Cyl, pos: [0.5, 0.38, 0.445], size: [0.418, 0.05, 0.418], rot: [90.0, 0.0, 0.0], mat: Mat::C3, id: Some("corps") },
    Piece { shape: Shape::Cyl, pos: [0.5, 0.38, -0.4], size: [0.76, 0.08, 0.76], rot: [90.0, 0.0, 0.0], mat: Mat::Sombre, id: Some("corps") },
    Piece { shape: Shape::Cyl, pos: [0.5, 0.38, -0.445], size: [0.418, 0.05, 0.418], rot: [90.0, 0.0, 0.0], mat: Mat::C3, id: Some("corps") },
    Piece { shape: Shape::Cyl, pos: [1.15, 0.22, 0.4], size: [0.44, 0.08, 0.44], rot: [90.0, 0.0, 0.0], mat: Mat::Sombre, id: Some("corps") },
    Piece { shape: Shape::Cyl, pos: [1.15, 0.22, 0.445], size: [0.242, 0.05, 0.242], rot: [90.0, 0.0, 0.0], mat: Mat::C3, id: Some("corps") },
    Piece { shape: Shape::Cyl, pos: [1.15, 0.22, -0.4], size: [0.44, 0.08, 0.44], rot: [90.0, 0.0, 0.0], mat: Mat::Sombre, id: Some("corps") },
    Piece { shape: Shape::Cyl, pos: [1.15, 0.22, -0.445], size: [0.242, 0.05, 0.242], rot: [90.0, 0.0, 0.0], mat: Mat::C3, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [1.0, 1.95, 0.0], size: [0.3, 0.3, 0.3], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("fumee") },
    Piece { shape: Shape::Sphere, pos: [1.05, 2.2, 0.0], size: [0.4, 0.4, 0.4], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("fumee") },
    Piece { shape: Shape::Sphere, pos: [1.1, 2.45, 0.0], size: [0.5, 0.5, 0.5], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("fumee") },
];
const ANIMS_129: &[Anim] = &[
    Anim { target: "fumee", kind: AnimKind::DriftUp, amp: 0.0 },
    Anim { target: "corps", kind: AnimKind::Bob, amp: 0.012 },
];
const PIECES_130: &[Piece] = &[
    Piece { shape: Shape::Cube, pos: [0.0, 0.6, 0.0], size: [1.4, 0.7, 0.8], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 1.0, 0.0], size: [1.3, 0.2, 0.7], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: None },
    Piece { shape: Shape::Sphere, pos: [0.2, 1.15, 0.0], size: [0.5, 0.3, 0.4], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.25, 0.0], size: [1.5, 0.12, 0.7], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.4, 0.25, 0.4], size: [0.5, 0.08, 0.5], rot: [90.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.4, 0.25, 0.445], size: [0.275, 0.05, 0.275], rot: [90.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.4, 0.25, -0.4], size: [0.5, 0.08, 0.5], rot: [90.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.4, 0.25, -0.445], size: [0.275, 0.05, 0.275], rot: [90.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [0.4, 0.25, 0.4], size: [0.5, 0.08, 0.5], rot: [90.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cyl, pos: [0.4, 0.25, 0.445], size: [0.275, 0.05, 0.275], rot: [90.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [0.4, 0.25, -0.4], size: [0.5, 0.08, 0.5], rot: [90.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cyl, pos: [0.4, 0.25, -0.445], size: [0.275, 0.05, 0.275], rot: [90.0, 0.0, 0.0], mat: Mat::C3, id: None },
];
const ANIMS_130: &[Anim] = &[
];
const PIECES_131: &[Piece] = &[
    Piece { shape: Shape::Cube, pos: [0.0, 0.75, 0.0], size: [1.9, 0.9, 0.85], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 1.25, 0.0], size: [2.0, 0.08, 0.95], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.25, 0.0], size: [2.0, 0.12, 0.7], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cube, pos: [-0.5, 0.85, 0.43], size: [0.3, 0.4, 0.03], rot: [0.0, 0.0, 0.0], mat: Mat::Phare, id: None },
    Piece { shape: Shape::Cube, pos: [-0.5, 0.85, -0.43], size: [0.3, 0.4, 0.03], rot: [0.0, 0.0, 0.0], mat: Mat::Phare, id: None },
    Piece { shape: Shape::Cube, pos: [0.5, 0.85, 0.43], size: [0.3, 0.4, 0.03], rot: [0.0, 0.0, 0.0], mat: Mat::Phare, id: None },
    Piece { shape: Shape::Cube, pos: [0.5, 0.85, -0.43], size: [0.3, 0.4, 0.03], rot: [0.0, 0.0, 0.0], mat: Mat::Phare, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.6, 0.25, 0.4], size: [0.5, 0.08, 0.5], rot: [90.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.6, 0.25, 0.445], size: [0.275, 0.05, 0.275], rot: [90.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.6, 0.25, -0.4], size: [0.5, 0.08, 0.5], rot: [90.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.6, 0.25, -0.445], size: [0.275, 0.05, 0.275], rot: [90.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [0.6, 0.25, 0.4], size: [0.5, 0.08, 0.5], rot: [90.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cyl, pos: [0.6, 0.25, 0.445], size: [0.275, 0.05, 0.275], rot: [90.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [0.6, 0.25, -0.4], size: [0.5, 0.08, 0.5], rot: [90.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cyl, pos: [0.6, 0.25, -0.445], size: [0.275, 0.05, 0.275], rot: [90.0, 0.0, 0.0], mat: Mat::C3, id: None },
];
const ANIMS_131: &[Anim] = &[
];
const PIECES_132: &[Piece] = &[
    Piece { shape: Shape::Cube, pos: [0.0, 0.75, 0.0], size: [1.9, 0.9, 0.85], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 1.25, 0.0], size: [2.0, 0.08, 0.95], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.25, 0.0], size: [2.0, 0.12, 0.7], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cube, pos: [-0.7, 0.85, 0.43], size: [0.3, 0.4, 0.03], rot: [0.0, 0.0, 0.0], mat: Mat::Phare, id: None },
    Piece { shape: Shape::Cube, pos: [-0.7, 0.85, -0.43], size: [0.3, 0.4, 0.03], rot: [0.0, 0.0, 0.0], mat: Mat::Phare, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.85, 0.43], size: [0.3, 0.4, 0.03], rot: [0.0, 0.0, 0.0], mat: Mat::Phare, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.85, -0.43], size: [0.3, 0.4, 0.03], rot: [0.0, 0.0, 0.0], mat: Mat::Phare, id: None },
    Piece { shape: Shape::Cube, pos: [0.7, 0.85, 0.43], size: [0.3, 0.4, 0.03], rot: [0.0, 0.0, 0.0], mat: Mat::Phare, id: None },
    Piece { shape: Shape::Cube, pos: [0.7, 0.85, -0.43], size: [0.3, 0.4, 0.03], rot: [0.0, 0.0, 0.0], mat: Mat::Phare, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 1.4, 0.0], size: [0.5, 0.2, 0.4], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.6, 0.25, 0.4], size: [0.5, 0.08, 0.5], rot: [90.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.6, 0.25, 0.445], size: [0.275, 0.05, 0.275], rot: [90.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.6, 0.25, -0.4], size: [0.5, 0.08, 0.5], rot: [90.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.6, 0.25, -0.445], size: [0.275, 0.05, 0.275], rot: [90.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [0.6, 0.25, 0.4], size: [0.5, 0.08, 0.5], rot: [90.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cyl, pos: [0.6, 0.25, 0.445], size: [0.275, 0.05, 0.275], rot: [90.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [0.6, 0.25, -0.4], size: [0.5, 0.08, 0.5], rot: [90.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cyl, pos: [0.6, 0.25, -0.445], size: [0.275, 0.05, 0.275], rot: [90.0, 0.0, 0.0], mat: Mat::C3, id: None },
];
const ANIMS_132: &[Anim] = &[
];
const HALLOWEEN_TRAIN: [TrainCar; 4] = [
    TrainCar { model: Model { c2: [42, 37, 64], c3: [107, 255, 122], pieces: PIECES_129, anims: ANIMS_129 }, len: 2.9 },
    TrainCar { model: Model { c2: [42, 37, 64], c3: [107, 255, 122], pieces: PIECES_130, anims: ANIMS_130 }, len: 1.6 },
    TrainCar { model: Model { c2: [61, 53, 80], c3: [255, 207, 90], pieces: PIECES_131, anims: ANIMS_131 }, len: 2.1 },
    TrainCar { model: Model { c2: [61, 53, 80], c3: [255, 138, 31], pieces: PIECES_132, anims: ANIMS_132 }, len: 2.1 },
];
const HALLOWEEN_COLORS: WorldColors = WorldColors {
    grass_a: [74, 74, 98],
    grass_b: [66, 66, 90],
    water: [79, 122, 58],
    road: [106, 98, 88],
    marking: [138, 132, 120],
    log: [90, 58, 36],
    sky: [42, 26, 58],
};
fn halloween_name() -> &'static str { tx!("Halloween Croâ", "Croak-o-ween", "Quak-o-ween", "Gra-lloween", "Croa-ween").t() }
fn halloween_banner() -> &'static str { tx!("Bienvenue à Halloween Croâ ! Si un fantôme vous salue, ne répondez pas.", "Welcome to Croak-o-ween! If a ghost waves at you, don't wave back.", "Willkommen bei Quak-o-ween! Wenn ein Geist winkt, nicht zurückwinken.", "Benvenuto a Gra-lloween! Se un fantasma ti saluta, non rispondere.", "¡Bienvenido a Croa-ween! Si un fantasma te saluda, no respondas.").t() }
fn halloween_hero_name() -> &'static str { tx!("Grenouille Sorcière", "Witch Frog", "Hexen-Frosch", "Rana Strega", "Rana Bruja").t() }
pub static HALLOWEEN_THEME: Theme = Theme {
    train: &HALLOWEEN_TRAIN, only_own: true,
    blocks: &HALLOWEEN_BLOCKS, sides: &HALLOWEEN_SIDES, deco: &HALLOWEEN_DECO, animals: &HALLOWEEN_ANIMALS,
    block_mix: &[0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3], side_mix: &[1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 102, 102, 102, 102, 102, 102, 100, 100, 100, 100],
    side_pct: 6, vehicles: &[47, 48, 49], colors: HALLOWEEN_COLORS, points: 3600,
    name: halloween_name, banner: halloween_banner, animal_name: halloween_animal_name, hero_name: halloween_hero_name,
    hero: Model { c2: [107, 63, 160], c3: [255, 138, 31], pieces: PIECES_127, anims: ANIMS_127 }, hat: Model { c2: [107, 63, 160], c3: [255, 138, 31], pieces: PIECES_128, anims: ANIMS_128 },
};
pub static THEMES: [&Theme; 5] = [&JURA_THEME, &NIPPON_THEME, &FARWEST_THEME, &ESPACE_THEME, &HALLOWEEN_THEME];
