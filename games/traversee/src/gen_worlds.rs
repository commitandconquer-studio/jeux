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
const PIECES_133: &[Piece] = &[
    Piece { shape: Shape::Cube, pos: [0.0, 0.04, 0.0], size: [0.45, 0.08, 0.45], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cyl, pos: [0.0, 0.6, 0.0], size: [0.1, 1.2, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cyl, pos: [0.0, 1.25, 0.0], size: [0.5, 0.05, 0.5], rot: [0.0, 0.0, 0.0], mat: Mat::LumiereBleue, id: Some("anneau") },
    Piece { shape: Shape::Sphere, pos: [0.0, 1.32, 0.0], size: [0.18, 0.12, 0.18], rot: [0.0, 0.0, 0.0], mat: Mat::Phare, id: None },
];
const ANIMS_133: &[Anim] = &[
    Anim { target: "anneau", kind: AnimKind::Pulse, amp: 0.0 },
];
const PIECES_134: &[Piece] = &[
    Piece { shape: Shape::Cube, pos: [0.0, 0.55, 0.0], size: [0.7, 1.1, 0.6], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.7, 0.31], size: [0.5, 0.5, 0.02], rot: [0.0, 0.0, 0.0], mat: Mat::LumiereBleue, id: Some("ecran") },
    Piece { shape: Shape::Cube, pos: [0.0, 0.25, 0.31], size: [0.5, 0.12, 0.02], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 1.12, 0.0], size: [0.74, 0.04, 0.64], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cube, pos: [0.36, 0.55, 0.0], size: [0.02, 1.0, 0.5], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
];
const ANIMS_134: &[Anim] = &[
    Anim { target: "ecran", kind: AnimKind::BlinkSlow, amp: 0.0 },
];
const PIECES_135: &[Piece] = &[
    Piece { shape: Shape::Cube, pos: [0.0, 0.4, 0.0], size: [1.0, 0.8, 0.7], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.4, 0.36], size: [0.9, 0.6, 0.02], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.4, -0.36], size: [0.9, 0.6, 0.02], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.82, 0.0], size: [1.04, 0.04, 0.74], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cube, pos: [0.2, 0.2, 0.36], size: [0.05, 0.05, 0.03], rot: [0.0, 0.0, 0.0], mat: Mat::LumiereRouge, id: Some("voyant") },
];
const ANIMS_135: &[Anim] = &[
    Anim { target: "voyant", kind: AnimKind::Blink, amp: 0.0 },
];
const PIECES_136: &[Piece] = &[
    Piece { shape: Shape::Cyl, pos: [0.0, 0.35, 0.0], size: [0.4, 0.7, 0.4], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cyl, pos: [0.0, 0.72, 0.0], size: [0.46, 0.05, 0.46], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Sphere, pos: [0.0, 1.05, 0.0], size: [0.46, 0.46, 0.46], rot: [0.0, 0.0, 0.0], mat: Mat::Bulle, id: Some("holo") },
    Piece { shape: Shape::Sphere, pos: [0.0, 1.05, 0.0], size: [0.18, 0.18, 0.18], rot: [0.0, 0.0, 0.0], mat: Mat::LumiereBleue, id: Some("holo") },
];
const ANIMS_136: &[Anim] = &[
    Anim { target: "holo", kind: AnimKind::Pulse, amp: 0.0 },
];
const CYBER_BLOCKS: [Model; 4] = [
    Model { c2: [45, 49, 66], c3: [55, 230, 255], pieces: PIECES_133, anims: ANIMS_133 },
    Model { c2: [58, 63, 90], c3: [255, 63, 168], pieces: PIECES_134, anims: ANIMS_134 },
    Model { c2: [45, 49, 66], c3: [255, 154, 63], pieces: PIECES_135, anims: ANIMS_135 },
    Model { c2: [58, 63, 90], c3: [55, 230, 255], pieces: PIECES_136, anims: ANIMS_136 },
];
const PIECES_137: &[Piece] = &[
    Piece { shape: Shape::Cube, pos: [0.0, 2.1, 0.0], size: [1.4, 4.2, 1.2], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 4.25, 0.0], size: [1.5, 0.1, 1.3], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cube, pos: [-0.45, 0.7, 0.61], size: [0.28, 0.3, 0.03], rot: [0.0, 0.0, 0.0], mat: Mat::Phare, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.7, 0.61], size: [0.28, 0.3, 0.03], rot: [0.0, 0.0, 0.0], mat: Mat::Phare, id: None },
    Piece { shape: Shape::Cube, pos: [0.45, 0.7, 0.61], size: [0.28, 0.3, 0.03], rot: [0.0, 0.0, 0.0], mat: Mat::Phare, id: None },
    Piece { shape: Shape::Cube, pos: [-0.45, 1.25, 0.61], size: [0.28, 0.3, 0.03], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 1.25, 0.61], size: [0.28, 0.3, 0.03], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [0.45, 1.25, 0.61], size: [0.28, 0.3, 0.03], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [-0.45, 1.8, 0.61], size: [0.28, 0.3, 0.03], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 1.8, 0.61], size: [0.28, 0.3, 0.03], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [0.45, 1.8, 0.61], size: [0.28, 0.3, 0.03], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [-0.45, 2.35, 0.61], size: [0.28, 0.3, 0.03], rot: [0.0, 0.0, 0.0], mat: Mat::Phare, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 2.35, 0.61], size: [0.28, 0.3, 0.03], rot: [0.0, 0.0, 0.0], mat: Mat::Phare, id: None },
    Piece { shape: Shape::Cube, pos: [0.45, 2.35, 0.61], size: [0.28, 0.3, 0.03], rot: [0.0, 0.0, 0.0], mat: Mat::Phare, id: None },
    Piece { shape: Shape::Cube, pos: [-0.45, 2.9, 0.61], size: [0.28, 0.3, 0.03], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 2.9, 0.61], size: [0.28, 0.3, 0.03], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [0.45, 2.9, 0.61], size: [0.28, 0.3, 0.03], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [-0.45, 3.45, 0.61], size: [0.28, 0.3, 0.03], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 3.45, 0.61], size: [0.28, 0.3, 0.03], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [0.45, 3.45, 0.61], size: [0.28, 0.3, 0.03], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [-0.45, 4.0, 0.61], size: [0.28, 0.3, 0.03], rot: [0.0, 0.0, 0.0], mat: Mat::Phare, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 4.0, 0.61], size: [0.28, 0.3, 0.03], rot: [0.0, 0.0, 0.0], mat: Mat::Phare, id: None },
    Piece { shape: Shape::Cube, pos: [0.45, 4.0, 0.61], size: [0.28, 0.3, 0.03], rot: [0.0, 0.0, 0.0], mat: Mat::Phare, id: None },
    Piece { shape: Shape::Cyl, pos: [0.4, 4.6, 0.0], size: [0.05, 0.6, 0.05], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Sphere, pos: [0.4, 4.92, 0.0], size: [0.1, 0.1, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::LumiereRouge, id: Some("balise") },
    Piece { shape: Shape::Cube, pos: [0.0, 0.05, 0.0], size: [1.7, 0.1, 1.5], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
];
const ANIMS_137: &[Anim] = &[
    Anim { target: "balise", kind: AnimKind::Blink, amp: 0.0 },
];
const PIECES_138: &[Piece] = &[
    Piece { shape: Shape::Cube, pos: [0.0, 1.5, 0.0], size: [1.4, 3.0, 1.2], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 3.05, 0.0], size: [1.5, 0.1, 1.3], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cube, pos: [-0.45, 0.7, 0.61], size: [0.28, 0.3, 0.03], rot: [0.0, 0.0, 0.0], mat: Mat::Phare, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.7, 0.61], size: [0.28, 0.3, 0.03], rot: [0.0, 0.0, 0.0], mat: Mat::Phare, id: None },
    Piece { shape: Shape::Cube, pos: [0.45, 0.7, 0.61], size: [0.28, 0.3, 0.03], rot: [0.0, 0.0, 0.0], mat: Mat::Phare, id: None },
    Piece { shape: Shape::Cube, pos: [-0.45, 1.25, 0.61], size: [0.28, 0.3, 0.03], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 1.25, 0.61], size: [0.28, 0.3, 0.03], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [0.45, 1.25, 0.61], size: [0.28, 0.3, 0.03], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [-0.45, 1.8, 0.61], size: [0.28, 0.3, 0.03], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 1.8, 0.61], size: [0.28, 0.3, 0.03], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [0.45, 1.8, 0.61], size: [0.28, 0.3, 0.03], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [-0.45, 2.35, 0.61], size: [0.28, 0.3, 0.03], rot: [0.0, 0.0, 0.0], mat: Mat::Phare, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 2.35, 0.61], size: [0.28, 0.3, 0.03], rot: [0.0, 0.0, 0.0], mat: Mat::Phare, id: None },
    Piece { shape: Shape::Cube, pos: [0.45, 2.35, 0.61], size: [0.28, 0.3, 0.03], rot: [0.0, 0.0, 0.0], mat: Mat::Phare, id: None },
    Piece { shape: Shape::Cube, pos: [-0.45, 2.9, 0.61], size: [0.28, 0.3, 0.03], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 2.9, 0.61], size: [0.28, 0.3, 0.03], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [0.45, 2.9, 0.61], size: [0.28, 0.3, 0.03], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [0.4, 3.4, 0.0], size: [0.05, 0.6, 0.05], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Sphere, pos: [0.4, 3.72, 0.0], size: [0.1, 0.1, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::LumiereRouge, id: Some("balise") },
    Piece { shape: Shape::Cube, pos: [0.0, 0.05, 0.0], size: [1.7, 0.1, 1.5], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
];
const ANIMS_138: &[Anim] = &[
    Anim { target: "balise", kind: AnimKind::Blink, amp: 0.0 },
];
const PIECES_139: &[Piece] = &[
    Piece { shape: Shape::Cyl, pos: [0.0, 1.2, 0.0], size: [0.12, 2.4, 0.12], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 2.7, 0.0], size: [1.8, 1.0, 0.12], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 2.7, 0.07], size: [1.6, 0.8, 0.03], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("neon") },
    Piece { shape: Shape::Cube, pos: [0.0, 2.7, 0.09], size: [1.2, 0.1, 0.02], rot: [0.0, 0.0, 0.0], mat: Mat::LumiereBleue, id: Some("neon") },
    Piece { shape: Shape::Cube, pos: [0.0, 0.05, 0.0], size: [0.8, 0.1, 0.8], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
];
const ANIMS_139: &[Anim] = &[
    Anim { target: "neon", kind: AnimKind::BlinkSlow, amp: 0.0 },
];
const CYBER_SIDES: [Model; 3] = [
    Model { c2: [28, 31, 46], c3: [55, 230, 255], pieces: PIECES_137, anims: ANIMS_137 },
    Model { c2: [42, 36, 56], c3: [255, 63, 168], pieces: PIECES_138, anims: ANIMS_138 },
    Model { c2: [28, 31, 46], c3: [255, 63, 168], pieces: PIECES_139, anims: ANIMS_139 },
];
const PIECES_140: &[Piece] = &[
    Piece { shape: Shape::Sphere, pos: [0.0, 0.0, 0.0], size: [0.6, 0.02, 0.4], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Sphere, pos: [0.05, 0.01, 0.02], size: [0.3, 0.02, 0.2], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
];
const ANIMS_140: &[Anim] = &[
];
const PIECES_141: &[Piece] = &[
    Piece { shape: Shape::Cube, pos: [0.0, 0.015, 0.0], size: [0.7, 0.03, 0.04], rot: [0.0, 20.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [0.15, 0.015, 0.1], size: [0.4, 0.03, 0.04], rot: [0.0, -30.0, 0.0], mat: Mat::C3, id: None },
];
const ANIMS_141: &[Anim] = &[
];
const PIECES_142: &[Piece] = &[
    Piece { shape: Shape::Cyl, pos: [0.0, 0.07, 0.0], size: [0.08, 0.14, 0.08], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cyl, pos: [0.0, 0.07, 0.0], size: [0.085, 0.05, 0.085], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
];
const ANIMS_142: &[Anim] = &[
];
const PIECES_143: &[Piece] = &[
    Piece { shape: Shape::Cube, pos: [0.0, 0.02, 0.0], size: [0.3, 0.04, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.01, 0.0], size: [0.34, 0.02, 0.14], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
];
const ANIMS_143: &[Anim] = &[
];
const CYBER_DECO: [Model; 4] = [
    Model { c2: [26, 111, 208], c3: [55, 230, 255], pieces: PIECES_140, anims: ANIMS_140 },
    Model { c2: [45, 49, 66], c3: [255, 63, 168], pieces: PIECES_141, anims: ANIMS_141 },
    Model { c2: [200, 204, 216], c3: [255, 63, 168], pieces: PIECES_142, anims: ANIMS_142 },
    Model { c2: [55, 230, 255], c3: [45, 49, 66], pieces: PIECES_143, anims: ANIMS_143 },
];
const PIECES_144: &[Piece] = &[
    Piece { shape: Shape::Sphere, pos: [0.0, 0.7, 0.0], size: [0.5, 0.26, 0.5], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Cube, pos: [0.0, 0.5, 0.0], size: [0.3, 0.22, 0.3], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [0.22, 0.7, 0.0], size: [0.1, 0.1, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::LumiereBleue, id: Some("oeil") },
    Piece { shape: Shape::Cube, pos: [0.18, 0.72, 0.18], size: [0.4, 0.04, 0.06], rot: [0.0, 45.0, 0.0], mat: Mat::Sombre, id: Some("corps") },
    Piece { shape: Shape::Cube, pos: [0.3, 0.8, 0.3], size: [0.4, 0.015, 0.06], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("helice0") },
    Piece { shape: Shape::Cube, pos: [0.18, 0.72, -0.18], size: [0.4, 0.04, 0.06], rot: [0.0, -45.0, 0.0], mat: Mat::Sombre, id: Some("corps") },
    Piece { shape: Shape::Cube, pos: [0.3, 0.8, -0.3], size: [0.4, 0.015, 0.06], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("helice1") },
    Piece { shape: Shape::Cube, pos: [-0.18, 0.72, 0.18], size: [0.4, 0.04, 0.06], rot: [0.0, -45.0, 0.0], mat: Mat::Sombre, id: Some("corps") },
    Piece { shape: Shape::Cube, pos: [-0.3, 0.8, 0.3], size: [0.4, 0.015, 0.06], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("helice2") },
    Piece { shape: Shape::Cube, pos: [-0.18, 0.72, -0.18], size: [0.4, 0.04, 0.06], rot: [0.0, 45.0, 0.0], mat: Mat::Sombre, id: Some("corps") },
    Piece { shape: Shape::Cube, pos: [-0.3, 0.8, -0.3], size: [0.4, 0.015, 0.06], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("helice3") },
];
const ANIMS_144: &[Anim] = &[
    Anim { target: "corps", kind: AnimKind::Bob, amp: 0.05 },
    Anim { target: "oeil", kind: AnimKind::BlinkAlt, amp: 0.0 },
    Anim { target: "helice0", kind: AnimKind::SpinGroup, amp: 0.0 },
    Anim { target: "helice1", kind: AnimKind::SpinGroup, amp: 0.0 },
    Anim { target: "helice2", kind: AnimKind::SpinGroup, amp: 0.0 },
    Anim { target: "helice3", kind: AnimKind::SpinGroup, amp: 0.0 },
];
const PIECES_145: &[Piece] = &[
    Piece { shape: Shape::Cube, pos: [0.0, 0.4, 0.0], size: [0.55, 0.28, 0.28], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Cube, pos: [0.34, 0.5, 0.0], size: [0.28, 0.26, 0.26], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Cone, pos: [0.3, 0.72, 0.09], size: [0.1, 0.16, 0.06], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("corps") },
    Piece { shape: Shape::Cone, pos: [0.3, 0.72, -0.09], size: [0.1, 0.16, 0.06], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("corps") },
    Piece { shape: Shape::Cube, pos: [0.48, 0.52, 0.0], size: [0.03, 0.07, 0.2], rot: [0.0, 0.0, 0.0], mat: Mat::LumiereBleue, id: Some("oeil") },
    Piece { shape: Shape::Cyl, pos: [-0.4, 0.62, 0.0], size: [0.05, 0.5, 0.05], rot: [0.0, 0.0, 25.0], mat: Mat::C3, id: Some("queue") },
    Piece { shape: Shape::Sphere, pos: [-0.5, 0.88, 0.0], size: [0.08, 0.08, 0.08], rot: [0.0, 0.0, 0.0], mat: Mat::LumiereRouge, id: Some("oeil") },
    Piece { shape: Shape::Cyl, pos: [0.2, 0.2, 0.08], size: [0.07, 0.4, 0.07], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("patte0") },
    Piece { shape: Shape::Cyl, pos: [0.2, 0.2, -0.08], size: [0.07, 0.4, 0.07], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("patte1") },
    Piece { shape: Shape::Cyl, pos: [-0.2, 0.2, 0.08], size: [0.07, 0.4, 0.07], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("patte2") },
    Piece { shape: Shape::Cyl, pos: [-0.2, 0.2, -0.08], size: [0.07, 0.4, 0.07], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("patte3") },
];
const ANIMS_145: &[Anim] = &[
    Anim { target: "corps", kind: AnimKind::Bob, amp: 0.03 },
    Anim { target: "queue", kind: AnimKind::TailY, amp: 0.0 },
    Anim { target: "oeil", kind: AnimKind::BlinkAlt, amp: 0.0 },
    Anim { target: "patte0", kind: AnimKind::SwingZ, amp: 0.0 },
    Anim { target: "patte1", kind: AnimKind::SwingZ, amp: 0.5 },
    Anim { target: "patte2", kind: AnimKind::SwingZ, amp: 0.5 },
    Anim { target: "patte3", kind: AnimKind::SwingZ, amp: 0.0 },
];
const PIECES_146: &[Piece] = &[
    Piece { shape: Shape::Cube, pos: [0.0, 0.95, 0.0], size: [0.42, 0.75, 0.58], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Cube, pos: [0.2, 0.95, 0.0], size: [0.04, 0.5, 0.4], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [0.08, 1.55, 0.0], size: [0.4, 0.4, 0.38], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Cube, pos: [0.28, 1.58, 0.0], size: [0.04, 0.07, 0.3], rot: [0.0, 0.0, 0.0], mat: Mat::LumiereBleue, id: Some("oeil") },
    Piece { shape: Shape::Cyl, pos: [0.0, 1.9, 0.1], size: [0.03, 0.3, 0.03], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [0.0, 2.08, 0.1], size: [0.07, 0.07, 0.07], rot: [0.0, 0.0, 0.0], mat: Mat::LumiereRouge, id: Some("oeil") },
    Piece { shape: Shape::Cube, pos: [0.0, 0.5, 0.0], size: [0.4, 0.2, 0.52], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: Some("corps") },
    Piece { shape: Shape::Cyl, pos: [0.0, 1.0, 0.38], size: [0.12, 0.7, 0.12], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("bras_g") },
    Piece { shape: Shape::Sphere, pos: [0.0, 0.62, 0.38], size: [0.18, 0.16, 0.18], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: Some("bras_g") },
    Piece { shape: Shape::Cyl, pos: [0.0, 1.0, -0.38], size: [0.12, 0.7, 0.12], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("bras_d") },
    Piece { shape: Shape::Sphere, pos: [0.0, 0.62, -0.38], size: [0.18, 0.16, 0.18], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: Some("bras_d") },
    Piece { shape: Shape::Cyl, pos: [0.0, 0.32, 0.15], size: [0.16, 0.64, 0.16], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("patte_g") },
    Piece { shape: Shape::Cube, pos: [0.09, 0.04, 0.15], size: [0.3, 0.08, 0.18], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: Some("patte_g") },
    Piece { shape: Shape::Cyl, pos: [0.0, 0.32, -0.15], size: [0.16, 0.64, 0.16], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("patte_d") },
    Piece { shape: Shape::Cube, pos: [0.09, 0.04, -0.15], size: [0.3, 0.08, 0.18], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: Some("patte_d") },
];
const ANIMS_146: &[Anim] = &[
    Anim { target: "corps", kind: AnimKind::Bob, amp: 0.03 },
    Anim { target: "oeil", kind: AnimKind::BlinkSlow, amp: 0.0 },
    Anim { target: "bras_g", kind: AnimKind::SwingZ, amp: 0.5 },
    Anim { target: "bras_d", kind: AnimKind::SwingZ, amp: 0.0 },
    Anim { target: "patte_g", kind: AnimKind::SwingZ, amp: 0.0 },
    Anim { target: "patte_d", kind: AnimKind::SwingZ, amp: 0.5 },
];
const PIECES_147: &[Piece] = &[
    Piece { shape: Shape::Cube, pos: [0.0, 0.62, 0.0], size: [0.95, 0.34, 0.4], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Cube, pos: [0.58, 0.8, 0.0], size: [0.4, 0.3, 0.32], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Cube, pos: [0.82, 0.75, 0.0], size: [0.28, 0.16, 0.24], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("corps") },
    Piece { shape: Shape::Cube, pos: [0.8, 0.86, 0.0], size: [0.2, 0.05, 0.3], rot: [0.0, 0.0, 0.0], mat: Mat::LumiereBleue, id: Some("oeil") },
    Piece { shape: Shape::Cyl, pos: [-0.55, 0.78, 0.0], size: [0.05, 0.4, 0.05], rot: [0.0, 0.0, 40.0], mat: Mat::Sombre, id: Some("queue") },
    Piece { shape: Shape::Cube, pos: [0.0, 0.8, 0.0], size: [0.6, 0.05, 0.3], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("corps") },
    Piece { shape: Shape::Cone, pos: [0.55, 1.05, 0.1], size: [0.07, 0.16, 0.05], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: Some("corps") },
    Piece { shape: Shape::Cone, pos: [0.55, 1.05, -0.1], size: [0.07, 0.16, 0.05], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: Some("corps") },
    Piece { shape: Shape::Cyl, pos: [0.35, 0.28, 0.16], size: [0.1, 0.56, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("patte0") },
    Piece { shape: Shape::Cube, pos: [0.39, 0.03, 0.16], size: [0.16, 0.06, 0.12], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: Some("patte0") },
    Piece { shape: Shape::Cyl, pos: [0.35, 0.28, -0.16], size: [0.1, 0.56, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("patte1") },
    Piece { shape: Shape::Cube, pos: [0.39, 0.03, -0.16], size: [0.16, 0.06, 0.12], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: Some("patte1") },
    Piece { shape: Shape::Cyl, pos: [-0.35, 0.28, 0.16], size: [0.1, 0.56, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("patte2") },
    Piece { shape: Shape::Cube, pos: [-0.31, 0.03, 0.16], size: [0.16, 0.06, 0.12], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: Some("patte2") },
    Piece { shape: Shape::Cyl, pos: [-0.35, 0.28, -0.16], size: [0.1, 0.56, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("patte3") },
    Piece { shape: Shape::Cube, pos: [-0.31, 0.03, -0.16], size: [0.16, 0.06, 0.12], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: Some("patte3") },
];
const ANIMS_147: &[Anim] = &[
    Anim { target: "corps", kind: AnimKind::Bob, amp: 0.04 },
    Anim { target: "oeil", kind: AnimKind::BlinkSlow, amp: 0.0 },
    Anim { target: "queue", kind: AnimKind::TailY, amp: 0.0 },
    Anim { target: "patte0", kind: AnimKind::SwingZ, amp: 0.0 },
    Anim { target: "patte1", kind: AnimKind::SwingZ, amp: 0.5 },
    Anim { target: "patte2", kind: AnimKind::SwingZ, amp: 0.5 },
    Anim { target: "patte3", kind: AnimKind::SwingZ, amp: 0.0 },
];
const PIECES_148: &[Piece] = &[
    Piece { shape: Shape::Cube, pos: [0.0, 0.55, 0.0], size: [1.5, 0.2, 0.36], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Cone, pos: [0.85, 0.55, 0.0], size: [0.4, 0.3, 0.34], rot: [0.0, 0.0, -90.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Cube, pos: [0.35, 0.78, 0.0], size: [0.4, 0.24, 0.3], rot: [0.0, 0.0, 0.0], mat: Mat::Bulle, id: Some("corps") },
    Piece { shape: Shape::Cube, pos: [-0.5, 0.78, 0.0], size: [0.5, 0.04, 0.5], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("corps") },
    Piece { shape: Shape::Cube, pos: [-0.75, 0.55, 0.0], size: [0.1, 0.4, 0.8], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("corps") },
    Piece { shape: Shape::Cube, pos: [-0.78, 0.55, 0.0], size: [0.03, 0.08, 0.7], rot: [0.0, 0.0, 0.0], mat: Mat::LumiereRouge, id: Some("feu") },
    Piece { shape: Shape::Cube, pos: [0.0, 0.38, 0.0], size: [1.3, 0.04, 0.5], rot: [0.0, 0.0, 0.0], mat: Mat::LumiereBleue, id: Some("coussin") },
    Piece { shape: Shape::Sphere, pos: [-0.2, 0.98, 0.0], size: [0.22, 0.26, 0.22], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("corps") },
    Piece { shape: Shape::Cube, pos: [0.0, 0.4, 0.0], size: [1.4, 0.08, 0.6], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: Some("corps") },
];
const ANIMS_148: &[Anim] = &[
    Anim { target: "corps", kind: AnimKind::Float, amp: 0.04 },
    Anim { target: "coussin", kind: AnimKind::Pulse, amp: 0.0 },
    Anim { target: "feu", kind: AnimKind::Blink, amp: 0.0 },
];
const PIECES_149: &[Piece] = &[
    Piece { shape: Shape::Cube, pos: [0.0, 1.7, 0.0], size: [0.9, 1.0, 1.2], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Cube, pos: [0.46, 1.8, 0.0], size: [0.04, 0.5, 0.8], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("corps") },
    Piece { shape: Shape::Cube, pos: [0.15, 2.4, 0.0], size: [0.55, 0.45, 0.55], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Cube, pos: [0.44, 2.45, 0.0], size: [0.04, 0.12, 0.48], rot: [0.0, 0.0, 0.0], mat: Mat::LumiereRouge, id: Some("oeil") },
    Piece { shape: Shape::Cyl, pos: [-0.1, 2.9, 0.2], size: [0.04, 0.4, 0.04], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [-0.1, 3.12, 0.2], size: [0.1, 0.1, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::LumiereBleue, id: Some("oeil") },
    Piece { shape: Shape::Cube, pos: [0.0, 1.05, 0.0], size: [0.8, 0.3, 1.0], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: Some("corps") },
    Piece { shape: Shape::Cube, pos: [-0.55, 1.85, 0.0], size: [0.4, 0.7, 0.7], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("corps") },
    Piece { shape: Shape::Cyl, pos: [0.0, 1.7, 0.78], size: [0.34, 1.3, 0.34], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("bras_g") },
    Piece { shape: Shape::Cube, pos: [0.2, 1.0, 0.78], size: [0.5, 0.36, 0.4], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: Some("bras_g") },
    Piece { shape: Shape::Cyl, pos: [0.0, 1.7, -0.78], size: [0.34, 1.3, 0.34], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("bras_d") },
    Piece { shape: Shape::Cube, pos: [0.2, 1.0, -0.78], size: [0.5, 0.36, 0.4], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: Some("bras_d") },
    Piece { shape: Shape::Cyl, pos: [0.0, 0.5, 0.36], size: [0.4, 1.0, 0.4], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("patte_g") },
    Piece { shape: Shape::Cube, pos: [0.2, 0.08, 0.36], size: [0.9, 0.16, 0.56], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: Some("patte_g") },
    Piece { shape: Shape::Cyl, pos: [0.0, 0.5, -0.36], size: [0.4, 1.0, 0.4], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("patte_d") },
    Piece { shape: Shape::Cube, pos: [0.2, 0.08, -0.36], size: [0.9, 0.16, 0.56], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: Some("patte_d") },
];
const ANIMS_149: &[Anim] = &[
    Anim { target: "corps", kind: AnimKind::Bob, amp: 0.02 },
    Anim { target: "oeil", kind: AnimKind::BlinkAlt, amp: 0.0 },
    Anim { target: "bras_g", kind: AnimKind::SwingZ, amp: 0.5 },
    Anim { target: "bras_d", kind: AnimKind::SwingZ, amp: 0.0 },
    Anim { target: "patte_g", kind: AnimKind::SwingZ, amp: 0.0 },
    Anim { target: "patte_d", kind: AnimKind::SwingZ, amp: 0.5 },
];
const PIECES_150: &[Piece] = &[
    Piece { shape: Shape::Sphere, pos: [0.0, 0.95, 0.0], size: [0.7, 0.3, 0.7], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Cube, pos: [0.0, 0.75, 0.0], size: [0.4, 0.2, 0.4], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [0.32, 0.95, 0.0], size: [0.14, 0.14, 0.14], rot: [0.0, 0.0, 0.0], mat: Mat::LumiereRouge, id: Some("oeil") },
    Piece { shape: Shape::Cube, pos: [0.0, 0.65, 0.0], size: [0.1, 0.1, 0.5], rot: [0.0, 0.0, 0.0], mat: Mat::LumiereBleue, id: Some("oeil") },
    Piece { shape: Shape::Cube, pos: [0.44, 0.97, 0.44], size: [1.1, 0.06, 0.1], rot: [0.0, 45.0, 0.0], mat: Mat::Sombre, id: Some("corps") },
    Piece { shape: Shape::Cube, pos: [0.8, 1.05, 0.8], size: [0.8, 0.02, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("helice0") },
    Piece { shape: Shape::Sphere, pos: [0.8, 0.98, 0.8], size: [0.12, 0.12, 0.12], rot: [0.0, 0.0, 0.0], mat: Mat::LumiereBleue, id: Some("oeil") },
    Piece { shape: Shape::Cube, pos: [0.44, 0.97, -0.44], size: [1.1, 0.06, 0.1], rot: [0.0, -45.0, 0.0], mat: Mat::Sombre, id: Some("corps") },
    Piece { shape: Shape::Cube, pos: [0.8, 1.05, -0.8], size: [0.8, 0.02, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("helice1") },
    Piece { shape: Shape::Sphere, pos: [0.8, 0.98, -0.8], size: [0.12, 0.12, 0.12], rot: [0.0, 0.0, 0.0], mat: Mat::LumiereBleue, id: Some("oeil") },
    Piece { shape: Shape::Cube, pos: [-0.44, 0.97, 0.44], size: [1.1, 0.06, 0.1], rot: [0.0, -45.0, 0.0], mat: Mat::Sombre, id: Some("corps") },
    Piece { shape: Shape::Cube, pos: [-0.8, 1.05, 0.8], size: [0.8, 0.02, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("helice2") },
    Piece { shape: Shape::Sphere, pos: [-0.8, 0.98, 0.8], size: [0.12, 0.12, 0.12], rot: [0.0, 0.0, 0.0], mat: Mat::LumiereBleue, id: Some("oeil") },
    Piece { shape: Shape::Cube, pos: [-0.44, 0.97, -0.44], size: [1.1, 0.06, 0.1], rot: [0.0, 45.0, 0.0], mat: Mat::Sombre, id: Some("corps") },
    Piece { shape: Shape::Cube, pos: [-0.8, 1.05, -0.8], size: [0.8, 0.02, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("helice3") },
    Piece { shape: Shape::Sphere, pos: [-0.8, 0.98, -0.8], size: [0.12, 0.12, 0.12], rot: [0.0, 0.0, 0.0], mat: Mat::LumiereBleue, id: Some("oeil") },
];
const ANIMS_150: &[Anim] = &[
    Anim { target: "corps", kind: AnimKind::Bob, amp: 0.05 },
    Anim { target: "oeil", kind: AnimKind::BlinkAlt, amp: 0.0 },
    Anim { target: "helice0", kind: AnimKind::SpinGroup, amp: 0.0 },
    Anim { target: "helice1", kind: AnimKind::SpinGroup, amp: 0.0 },
    Anim { target: "helice2", kind: AnimKind::SpinGroup, amp: 0.0 },
    Anim { target: "helice3", kind: AnimKind::SpinGroup, amp: 0.0 },
];
const CYBER_ANIMALS: [Animal; 7] = [
    Animal { model: Model { c2: [200, 204, 216], c3: [255, 63, 168], pieces: PIECES_144, anims: ANIMS_144 }, len: 0.8, width: 0.8, points: 300, dangerous: false, flying: false },
    Animal { model: Model { c2: [136, 144, 168], c3: [55, 230, 255], pieces: PIECES_145, anims: ANIMS_145 }, len: 1.0, width: 0.4, points: 500, dangerous: false, flying: false },
    Animal { model: Model { c2: [58, 63, 90], c3: [255, 154, 63], pieces: PIECES_146, anims: ANIMS_146 }, len: 0.9, width: 0.8, points: 900, dangerous: true, flying: false },
    Animal { model: Model { c2: [136, 144, 168], c3: [255, 63, 168], pieces: PIECES_147, anims: ANIMS_147 }, len: 1.6, width: 0.6, points: 1400, dangerous: true, flying: false },
    Animal { model: Model { c2: [255, 63, 168], c3: [45, 49, 66], pieces: PIECES_148, anims: ANIMS_148 }, len: 1.8, width: 0.7, points: 1900, dangerous: true, flying: false },
    Animal { model: Model { c2: [58, 63, 90], c3: [255, 154, 63], pieces: PIECES_149, anims: ANIMS_149 }, len: 1.8, width: 2.0, points: 2600, dangerous: true, flying: false },
    Animal { model: Model { c2: [45, 49, 66], c3: [55, 230, 255], pieces: PIECES_150, anims: ANIMS_150 }, len: 1.8, width: 1.8, points: 3000, dangerous: true, flying: true },
];
fn cyber_animal_name(i: usize) -> &'static str {
    match i {
        0 => tx!("Drone Livreur", "Delivery Drone", "Lieferdrohne", "Drone Fattorino", "Dron Repartidor").t(),
        1 => tx!("Chat Robot", "Robo Cat", "Roboterkatze", "Gatto Robot", "Gato Robot").t(),
        2 => tx!("Cyborg Pressé", "Hasty Cyborg", "Eiliger Cyborg", "Cyborg Frettoloso", "Cíborg con Prisa").t(),
        3 => tx!("Robot-Chien Pas Gentil", "Not-So-Friendly Robodog", "Unfreundlicher Roboterhund", "Cane Robot Scorbutico", "Perro Robot Antipático").t(),
        4 => tx!("Moto Volante", "Hoverbike", "Schwebebike", "Moto Volante", "Moto Voladora").t(),
        5 => tx!("Mécha Colossal", "Colossal Mech", "Riesen-Mech", "Mecha Colossale", "Mecha Colosal").t(),
        6 => tx!("Drone Géant", "Giant Drone", "Riesendrohne", "Drone Gigante", "Dron Gigante").t(),
        _ => "?",
    }
}
const PIECES_151: &[Piece] = &[
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
const ANIMS_151: &[Anim] = &[
    Anim { target: "corps", kind: AnimKind::Bob, amp: 0.02 },
];
const PIECES_152: &[Piece] = &[
    Piece { shape: Shape::Cube, pos: [0.3, 0.55, 0.0], size: [0.1, 0.1, 0.5], rot: [0.0, 0.0, 0.0], mat: Mat::LumiereBleue, id: Some("visiere") },
    Piece { shape: Shape::Cube, pos: [0.2, 0.66, 0.0], size: [0.4, 0.05, 0.5], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.5, 0.27], size: [0.12, 0.2, 0.06], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.5, -0.27], size: [0.12, 0.2, 0.06], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [0.0, 0.82, 0.15], size: [0.03, 0.3, 0.03], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Sphere, pos: [0.0, 0.98, 0.15], size: [0.07, 0.07, 0.07], rot: [0.0, 0.0, 0.0], mat: Mat::LumiereRouge, id: Some("balise") },
];
const ANIMS_152: &[Anim] = &[
    Anim { target: "balise", kind: AnimKind::Blink, amp: 0.0 },
    Anim { target: "visiere", kind: AnimKind::Pulse, amp: 0.0 },
];
const PIECES_153: &[Piece] = &[
    Piece { shape: Shape::Cube, pos: [0.0, 0.65, 0.0], size: [1.9, 0.7, 0.8], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cone, pos: [1.2, 0.65, 0.0], size: [0.7, 0.7, 0.8], rot: [0.0, 0.0, -90.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [0.7, 0.88, 0.0], size: [0.7, 0.3, 0.72], rot: [0.0, 0.0, 0.0], mat: Mat::Bulle, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.5, 0.405], size: [1.9, 0.06, 0.02], rot: [0.0, 0.0, 0.0], mat: Mat::LumiereBleue, id: Some("bande") },
    Piece { shape: Shape::Cube, pos: [0.0, 0.5, -0.405], size: [1.9, 0.06, 0.02], rot: [0.0, 0.0, 0.0], mat: Mat::LumiereBleue, id: Some("bande") },
    Piece { shape: Shape::Sphere, pos: [1.5, 0.65, 0.0], size: [0.14, 0.14, 0.14], rot: [0.0, 0.0, 0.0], mat: Mat::Phare, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.25, 0.0], size: [2.4, 0.1, 0.6], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
];
const ANIMS_153: &[Anim] = &[
    Anim { target: "bande", kind: AnimKind::Pulse, amp: 0.0 },
];
const PIECES_154: &[Piece] = &[
    Piece { shape: Shape::Cube, pos: [0.0, 0.65, 0.0], size: [1.9, 0.7, 0.8], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.9, 0.405], size: [1.5, 0.28, 0.02], rot: [0.0, 0.0, 0.0], mat: Mat::Bulle, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.9, -0.405], size: [1.5, 0.28, 0.02], rot: [0.0, 0.0, 0.0], mat: Mat::Bulle, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.5, 0.405], size: [1.9, 0.06, 0.02], rot: [0.0, 0.0, 0.0], mat: Mat::LumiereBleue, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.5, -0.405], size: [1.9, 0.06, 0.02], rot: [0.0, 0.0, 0.0], mat: Mat::LumiereBleue, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.25, 0.0], size: [1.5, 0.1, 0.6], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
];
const ANIMS_154: &[Anim] = &[
];
const PIECES_155: &[Piece] = &[
    Piece { shape: Shape::Cube, pos: [0.0, 0.65, 0.0], size: [1.9, 0.7, 0.8], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.9, 0.405], size: [1.5, 0.28, 0.02], rot: [0.0, 0.0, 0.0], mat: Mat::Bulle, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.9, -0.405], size: [1.5, 0.28, 0.02], rot: [0.0, 0.0, 0.0], mat: Mat::Bulle, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.5, 0.405], size: [1.9, 0.06, 0.02], rot: [0.0, 0.0, 0.0], mat: Mat::LumiereBleue, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.5, -0.405], size: [1.9, 0.06, 0.02], rot: [0.0, 0.0, 0.0], mat: Mat::LumiereBleue, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 1.02, 0.0], size: [1.95, 0.05, 0.85], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.25, 0.0], size: [2.1, 0.1, 0.6], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
];
const ANIMS_155: &[Anim] = &[
];
const PIECES_156: &[Piece] = &[
    Piece { shape: Shape::Cube, pos: [0.0, 0.65, 0.0], size: [1.9, 0.7, 0.8], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.9, 0.405], size: [1.5, 0.28, 0.02], rot: [0.0, 0.0, 0.0], mat: Mat::Bulle, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.9, -0.405], size: [1.5, 0.28, 0.02], rot: [0.0, 0.0, 0.0], mat: Mat::Bulle, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.5, 0.405], size: [1.9, 0.06, 0.02], rot: [0.0, 0.0, 0.0], mat: Mat::LumiereBleue, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.5, -0.405], size: [1.9, 0.06, 0.02], rot: [0.0, 0.0, 0.0], mat: Mat::LumiereBleue, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 1.02, 0.0], size: [1.95, 0.05, 0.85], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.25, 0.0], size: [2.1, 0.1, 0.6], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 1.1, 0.0], size: [0.6, 0.08, 0.3], rot: [0.0, 0.0, 0.0], mat: Mat::LumiereRouge, id: Some("voyant") },
];
const ANIMS_156: &[Anim] = &[
    Anim { target: "voyant", kind: AnimKind::Blink, amp: 0.0 },
];
const CYBER_TRAIN: [TrainCar; 4] = [
    TrainCar { model: Model { c2: [216, 220, 232], c3: [255, 63, 168], pieces: PIECES_153, anims: ANIMS_153 }, len: 2.9 },
    TrainCar { model: Model { c2: [216, 220, 232], c3: [255, 63, 168], pieces: PIECES_154, anims: ANIMS_154 }, len: 1.6 },
    TrainCar { model: Model { c2: [216, 220, 232], c3: [55, 230, 255], pieces: PIECES_155, anims: ANIMS_155 }, len: 2.1 },
    TrainCar { model: Model { c2: [170, 176, 196], c3: [255, 63, 168], pieces: PIECES_156, anims: ANIMS_156 }, len: 2.1 },
];
const CYBER_COLORS: WorldColors = WorldColors {
    grass_a: [54, 59, 82],
    grass_b: [47, 51, 72],
    water: [26, 111, 208],
    road: [23, 26, 38],
    marking: [55, 230, 255],
    log: [45, 49, 66],
    sky: [11, 13, 26],
};
fn cyber_name() -> &'static str { tx!("Néo-Croâ", "Neo-Croak", "Neo-Quak", "Neo-Gra", "Neo-Croa").t() }
fn cyber_banner() -> &'static str { tx!("Bienvenue à Néo-Croâ ! Les passages piétons sont désactivés pour votre confort.", "Welcome to Neo-Croak! Crosswalks have been disabled for your comfort.", "Willkommen in Neo-Quak! Fußgängerüberwege wurden zu Ihrem Komfort deaktiviert.", "Benvenuto a Neo-Gra! Le strisce pedonali sono disattivate per il tuo comfort.", "¡Bienvenido a Neo-Croa! Los pasos de peatones están desactivados por tu comodidad.").t() }
fn cyber_hero_name() -> &'static str { tx!("Grenouille Cyber", "Cyber Frog", "Cyber-Frosch", "Rana Cyber", "Rana Cíber").t() }
pub static CYBER_THEME: Theme = Theme {
    train: &CYBER_TRAIN, only_own: true,
    blocks: &CYBER_BLOCKS, sides: &CYBER_SIDES, deco: &CYBER_DECO, animals: &CYBER_ANIMALS,
    block_mix: &[0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 3, 3, 3, 3], side_mix: &[0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 2, 2, 2, 100, 100, 100, 100, 103, 103, 103],
    side_pct: 9, vehicles: &[50, 51, 52, 53], colors: CYBER_COLORS, points: 4200,
    name: cyber_name, banner: cyber_banner, animal_name: cyber_animal_name, hero_name: cyber_hero_name,
    hero: Model { c2: [58, 63, 90], c3: [55, 230, 255], pieces: PIECES_151, anims: ANIMS_151 }, hat: Model { c2: [58, 63, 90], c3: [55, 230, 255], pieces: PIECES_152, anims: ANIMS_152 },
};
const PIECES_157: &[Piece] = &[
    Piece { shape: Shape::Cyl, pos: [0.0, 0.4, 0.0], size: [0.28, 0.8, 0.28], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Sphere, pos: [0.0, 1.0, 0.0], size: [1.0, 0.85, 1.0], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Sphere, pos: [0.3, 0.8, 0.15], size: [0.55, 0.5, 0.55], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Sphere, pos: [-0.28, 0.85, -0.1], size: [0.5, 0.45, 0.5], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
];
const ANIMS_157: &[Anim] = &[
];
const PIECES_158: &[Piece] = &[
    Piece { shape: Shape::Sphere, pos: [0.0, 0.3, 0.0], size: [0.95, 0.6, 0.8], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Sphere, pos: [0.25, 0.55, 0.1], size: [0.55, 0.45, 0.5], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Sphere, pos: [-0.05, 0.62, -0.05], size: [0.5, 0.18, 0.4], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
];
const ANIMS_158: &[Anim] = &[
];
const PIECES_159: &[Piece] = &[
    Piece { shape: Shape::Cyl, pos: [0.0, 0.3, 0.0], size: [0.7, 0.6, 0.7], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cyl, pos: [0.0, 0.62, 0.0], size: [0.55, 0.05, 0.55], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cube, pos: [0.3, 0.8, 0.0], size: [0.06, 0.8, 0.06], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [-0.3, 0.8, 0.0], size: [0.06, 0.8, 0.06], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 1.22, 0.0], size: [0.8, 0.06, 0.5], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cone, pos: [0.0, 1.38, 0.0], size: [0.9, 0.3, 0.6], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
];
const ANIMS_159: &[Anim] = &[
];
const PIECES_160: &[Piece] = &[
    Piece { shape: Shape::Cyl, pos: [0.0, 0.7, 0.0], size: [0.07, 1.4, 0.07], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 1.0, 0.25], size: [0.03, 0.55, 0.45], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("fanion") },
    Piece { shape: Shape::Cube, pos: [0.0, 0.03, 0.0], size: [0.4, 0.06, 0.4], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Sphere, pos: [0.0, 1.42, 0.0], size: [0.12, 0.12, 0.12], rot: [0.0, 0.0, 0.0], mat: Mat::Phare, id: None },
];
const ANIMS_160: &[Anim] = &[
    Anim { target: "fanion", kind: AnimKind::TailY, amp: 0.0 },
];
const PIECES_161: &[Piece] = &[
    Piece { shape: Shape::Cube, pos: [0.0, 0.3, 0.0], size: [0.75, 0.6, 0.6], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.3, 0.0], size: [0.04, 0.62, 0.62], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [0.2, 0.3, 0.0], size: [0.04, 0.62, 0.62], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [-0.2, 0.3, 0.0], size: [0.04, 0.62, 0.62], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
];
const ANIMS_161: &[Anim] = &[
];
const MOYENAGE_BLOCKS: [Model; 5] = [
    Model { c2: [79, 143, 58], c3: [107, 74, 43], pieces: PIECES_157, anims: ANIMS_157 },
    Model { c2: [141, 143, 153], c3: [79, 122, 74], pieces: PIECES_158, anims: ANIMS_158 },
    Model { c2: [154, 154, 164], c3: [122, 77, 36], pieces: PIECES_159, anims: ANIMS_159 },
    Model { c2: [192, 57, 43], c3: [122, 77, 36], pieces: PIECES_160, anims: ANIMS_160 },
    Model { c2: [230, 197, 102], c3: [169, 135, 55], pieces: PIECES_161, anims: ANIMS_161 },
];
const PIECES_162: &[Piece] = &[
    Piece { shape: Shape::Cyl, pos: [-1.2, 1.3, 0.0], size: [1.0, 2.6, 1.0], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("t") },
    Piece { shape: Shape::Cyl, pos: [-1.2, 2.68, 0.0], size: [1.15, 0.16, 1.15], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("t") },
    Piece { shape: Shape::Cone, pos: [-1.2, 3.15, 0.0], size: [1.2, 0.9, 1.2], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("t") },
    Piece { shape: Shape::Cube, pos: [-1.2, 1.56, 0.5], size: [0.18, 0.35, 0.03], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: Some("t") },
    Piece { shape: Shape::Cyl, pos: [1.2, 1.3, 0.0], size: [1.0, 2.6, 1.0], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("t") },
    Piece { shape: Shape::Cyl, pos: [1.2, 2.68, 0.0], size: [1.15, 0.16, 1.15], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("t") },
    Piece { shape: Shape::Cone, pos: [1.2, 3.15, 0.0], size: [1.2, 0.9, 1.2], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("t") },
    Piece { shape: Shape::Cube, pos: [1.2, 1.56, 0.5], size: [0.18, 0.35, 0.03], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: Some("t") },
    Piece { shape: Shape::Cube, pos: [0.0, 1.0, 0.0], size: [2.4, 2.0, 1.0], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 2.1, 0.0], size: [2.5, 0.2, 1.1], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.5, 0.52], size: [0.7, 1.0, 0.04], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cone, pos: [0.0, 0.95, 0.52], size: [0.7, 0.3, 0.04], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cube, pos: [-0.6, 2.35, 0.0], size: [0.3, 0.3, 0.3], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 2.35, 0.0], size: [0.3, 0.3, 0.3], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [0.6, 2.35, 0.0], size: [0.3, 0.3, 0.3], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cyl, pos: [0.0, 3.0, 0.0], size: [0.05, 1.0, 0.05], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 3.35, 0.25], size: [0.03, 0.3, 0.5], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("fanion") },
];
const ANIMS_162: &[Anim] = &[
    Anim { target: "fanion", kind: AnimKind::TailY, amp: 0.0 },
];
const PIECES_163: &[Piece] = &[
    Piece { shape: Shape::Cube, pos: [0.0, 0.9, 0.0], size: [1.0, 1.8, 1.0], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cone, pos: [0.0, 2.2, 0.0], size: [1.3, 1.0, 1.3], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.4, 0.51], size: [0.4, 0.8, 0.03], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cyl, pos: [0.6, 1.8, 0.0], size: [0.12, 0.3, 0.12], rot: [0.0, 0.0, 90.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cube, pos: [0.7, 1.8, 0.0], size: [0.06, 1.7, 0.3], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("ailes") },
    Piece { shape: Shape::Cube, pos: [0.7, 1.8, 0.0], size: [0.06, 1.7, 0.3], rot: [0.0, 0.0, 90.0], mat: Mat::C3, id: Some("ailes") },
    Piece { shape: Shape::Cube, pos: [0.7, 1.8, 0.0], size: [0.06, 1.7, 0.3], rot: [0.0, 0.0, 180.0], mat: Mat::C3, id: Some("ailes") },
    Piece { shape: Shape::Cube, pos: [0.7, 1.8, 0.0], size: [0.06, 1.7, 0.3], rot: [0.0, 0.0, 270.0], mat: Mat::C3, id: Some("ailes") },
];
const ANIMS_163: &[Anim] = &[
    Anim { target: "ailes", kind: AnimKind::SpinGroup, amp: 0.0 },
];
const PIECES_164: &[Piece] = &[
    Piece { shape: Shape::Cyl, pos: [0.0, 1.4, 0.0], size: [1.0, 2.8, 1.0], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("t") },
    Piece { shape: Shape::Cyl, pos: [0.0, 2.88, 0.0], size: [1.15, 0.16, 1.15], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("t") },
    Piece { shape: Shape::Cone, pos: [0.0, 3.35, 0.0], size: [1.2, 0.9, 1.2], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("t") },
    Piece { shape: Shape::Cube, pos: [0.0, 1.68, 0.5], size: [0.18, 0.35, 0.03], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: Some("t") },
    Piece { shape: Shape::Cube, pos: [0.0, 3.0, 0.0], size: [1.3, 0.2, 1.3], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.05, 0.0], size: [1.3, 0.1, 1.3], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
];
const ANIMS_164: &[Anim] = &[
];
const MOYENAGE_SIDES: [Model; 3] = [
    Model { c2: [185, 180, 170], c3: [192, 57, 43], pieces: PIECES_162, anims: ANIMS_162 },
    Model { c2: [232, 220, 192], c3: [138, 90, 43], pieces: PIECES_163, anims: ANIMS_163 },
    Model { c2: [168, 164, 154], c3: [122, 77, 36], pieces: PIECES_164, anims: ANIMS_164 },
];
const PIECES_165: &[Piece] = &[
    Piece { shape: Shape::Cyl, pos: [0.0, 0.05, 0.0], size: [0.02, 0.1, 0.02], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Sphere, pos: [0.0, 0.11, 0.0], size: [0.1, 0.03, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Sphere, pos: [0.0, 0.12, 0.0], size: [0.04, 0.03, 0.04], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
];
const ANIMS_165: &[Anim] = &[
];
const PIECES_166: &[Piece] = &[
    Piece { shape: Shape::Cyl, pos: [0.0, 0.2, 0.0], size: [0.025, 0.4, 0.025], rot: [0.0, 0.0, 15.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cone, pos: [0.05, 0.38, 0.0], size: [0.06, 0.1, 0.06], rot: [0.0, 0.0, 15.0], mat: Mat::C3, id: None },
];
const ANIMS_166: &[Anim] = &[
];
const PIECES_167: &[Piece] = &[
    Piece { shape: Shape::Cyl, pos: [0.0, 0.015, 0.0], size: [0.14, 0.03, 0.14], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cyl, pos: [0.0, 0.03, 0.0], size: [0.09, 0.015, 0.09], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
];
const ANIMS_167: &[Anim] = &[
];
const PIECES_168: &[Piece] = &[
    Piece { shape: Shape::Cyl, pos: [0.0, 0.02, 0.0], size: [0.3, 0.03, 0.3], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.045, 0.0], size: [0.28, 0.02, 0.05], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.045, 0.0], size: [0.05, 0.02, 0.28], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
];
const ANIMS_168: &[Anim] = &[
];
const MOYENAGE_DECO: [Model; 4] = [
    Model { c2: [244, 244, 240], c3: [255, 210, 63], pieces: PIECES_165, anims: ANIMS_165 },
    Model { c2: [201, 160, 99], c3: [232, 232, 240], pieces: PIECES_166, anims: ANIMS_166 },
    Model { c2: [255, 210, 63], c3: [224, 168, 0], pieces: PIECES_167, anims: ANIMS_167 },
    Model { c2: [192, 57, 43], c3: [232, 232, 240], pieces: PIECES_168, anims: ANIMS_168 },
];
const PIECES_169: &[Piece] = &[
    Piece { shape: Shape::Sphere, pos: [0.0, 0.32, 0.0], size: [0.45, 0.4, 0.34], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [0.22, 0.5, 0.0], size: [0.22, 0.24, 0.2], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: Some("corps") },
    Piece { shape: Shape::Cone, pos: [0.36, 0.5, 0.0], size: [0.1, 0.12, 0.08], rot: [0.0, 0.0, -90.0], mat: Mat::C3, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [0.24, 0.64, 0.0], size: [0.1, 0.1, 0.06], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [0.3, 0.54, 0.07], size: [0.04, 0.04, 0.04], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [0.3, 0.54, -0.07], size: [0.04, 0.04, 0.04], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("corps") },
    Piece { shape: Shape::Cube, pos: [-0.28, 0.45, 0.0], size: [0.2, 0.2, 0.1], rot: [0.0, 0.0, 30.0], mat: Mat::Blanc, id: Some("corps") },
    Piece { shape: Shape::Cyl, pos: [0.0, 0.12, 0.07], size: [0.04, 0.24, 0.04], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("patte0") },
    Piece { shape: Shape::Cyl, pos: [0.0, 0.12, -0.07], size: [0.04, 0.24, 0.04], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("patte1") },
];
const ANIMS_169: &[Anim] = &[
    Anim { target: "corps", kind: AnimKind::Bob, amp: 0.04 },
    Anim { target: "patte0", kind: AnimKind::SwingZ, amp: 0.0 },
    Anim { target: "patte1", kind: AnimKind::SwingZ, amp: 0.5 },
];
const PIECES_170: &[Piece] = &[
    Piece { shape: Shape::Sphere, pos: [0.0, 0.4, 0.0], size: [0.7, 0.42, 0.42], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [0.38, 0.45, 0.0], size: [0.3, 0.3, 0.3], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Cube, pos: [0.55, 0.42, 0.0], size: [0.1, 0.12, 0.16], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("corps") },
    Piece { shape: Shape::Cone, pos: [0.35, 0.65, 0.1], size: [0.08, 0.12, 0.06], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Cone, pos: [0.35, 0.65, -0.1], size: [0.08, 0.12, 0.06], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [0.5, 0.52, 0.08], size: [0.05, 0.05, 0.05], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [0.5, 0.52, -0.08], size: [0.05, 0.05, 0.05], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("corps") },
    Piece { shape: Shape::Cyl, pos: [-0.42, 0.5, 0.0], size: [0.04, 0.2, 0.04], rot: [0.0, 0.0, 60.0], mat: Mat::C3, id: Some("queue") },
    Piece { shape: Shape::Cyl, pos: [0.22, 0.14, 0.12], size: [0.1, 0.28, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("patte0") },
    Piece { shape: Shape::Cyl, pos: [0.22, 0.14, -0.12], size: [0.1, 0.28, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("patte1") },
    Piece { shape: Shape::Cyl, pos: [-0.22, 0.14, 0.12], size: [0.1, 0.28, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("patte2") },
    Piece { shape: Shape::Cyl, pos: [-0.22, 0.14, -0.12], size: [0.1, 0.28, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("patte3") },
];
const ANIMS_170: &[Anim] = &[
    Anim { target: "corps", kind: AnimKind::Bob, amp: 0.03 },
    Anim { target: "queue", kind: AnimKind::TailY, amp: 0.0 },
    Anim { target: "patte0", kind: AnimKind::SwingZ, amp: 0.0 },
    Anim { target: "patte1", kind: AnimKind::SwingZ, amp: 0.5 },
    Anim { target: "patte2", kind: AnimKind::SwingZ, amp: 0.5 },
    Anim { target: "patte3", kind: AnimKind::SwingZ, amp: 0.0 },
];
const PIECES_171: &[Piece] = &[
    Piece { shape: Shape::Cube, pos: [0.0, 0.5, 0.0], size: [0.28, 0.4, 0.3], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [0.04, 0.82, 0.0], size: [0.3, 0.28, 0.28], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [0.18, 0.86, 0.08], size: [0.07, 0.07, 0.07], rot: [0.0, 0.0, 0.0], mat: Mat::LumiereRouge, id: None },
    Piece { shape: Shape::Sphere, pos: [0.18, 0.86, -0.08], size: [0.07, 0.07, 0.07], rot: [0.0, 0.0, 0.0], mat: Mat::LumiereRouge, id: None },
    Piece { shape: Shape::Cone, pos: [0.0, 0.88, 0.2], size: [0.1, 0.24, 0.05], rot: [60.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Cone, pos: [0.0, 0.88, -0.2], size: [0.1, 0.24, 0.05], rot: [-60.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Cone, pos: [0.2, 0.78, 0.0], size: [0.06, 0.1, 0.06], rot: [0.0, 0.0, -90.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Cube, pos: [0.25, 0.5, 0.2], size: [0.04, 0.3, 0.04], rot: [0.0, 0.0, -70.0], mat: Mat::Blanc, id: Some("corps") },
    Piece { shape: Shape::Cyl, pos: [0.0, 0.15, 0.08], size: [0.08, 0.3, 0.08], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("patte_g") },
    Piece { shape: Shape::Cyl, pos: [0.0, 0.15, -0.08], size: [0.08, 0.3, 0.08], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("patte_d") },
];
const ANIMS_171: &[Anim] = &[
    Anim { target: "corps", kind: AnimKind::Bob, amp: 0.04 },
    Anim { target: "patte_g", kind: AnimKind::SwingZ, amp: 0.0 },
    Anim { target: "patte_d", kind: AnimKind::SwingZ, amp: 0.5 },
];
const PIECES_172: &[Piece] = &[
    Piece { shape: Shape::Cube, pos: [0.0, 0.95, 0.0], size: [0.46, 0.7, 0.6], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [0.04, 1.45, 0.0], size: [0.4, 0.4, 0.4], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: Some("corps") },
    Piece { shape: Shape::Cube, pos: [0.22, 1.45, 0.0], size: [0.04, 0.06, 0.26], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("corps") },
    Piece { shape: Shape::Cone, pos: [0.0, 1.8, 0.0], size: [0.12, 0.34, 0.12], rot: [0.0, 0.0, 0.0], mat: Mat::LumiereRouge, id: Some("corps") },
    Piece { shape: Shape::Cube, pos: [0.28, 0.95, 0.38], size: [0.06, 0.7, 0.5], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("bouclier") },
    Piece { shape: Shape::Cube, pos: [0.31, 0.95, 0.38], size: [0.02, 0.16, 0.36], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: Some("bouclier") },
    Piece { shape: Shape::Cube, pos: [0.0, 0.5, 0.0], size: [0.44, 0.2, 0.58], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: Some("corps") },
    Piece { shape: Shape::Cyl, pos: [0.4, 1.2, -0.38], size: [0.05, 1.0, 0.05], rot: [0.0, 0.0, -35.0], mat: Mat::Blanc, id: Some("epee") },
    Piece { shape: Shape::Cube, pos: [0.25, 0.85, -0.38], size: [0.04, 0.04, 0.2], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("epee") },
    Piece { shape: Shape::Cyl, pos: [0.0, 0.3, 0.14], size: [0.16, 0.6, 0.16], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: Some("patte_g") },
    Piece { shape: Shape::Cube, pos: [0.09, 0.04, 0.14], size: [0.3, 0.08, 0.18], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: Some("patte_g") },
    Piece { shape: Shape::Cyl, pos: [0.0, 0.3, -0.14], size: [0.16, 0.6, 0.16], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: Some("patte_d") },
    Piece { shape: Shape::Cube, pos: [0.09, 0.04, -0.14], size: [0.3, 0.08, 0.18], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: Some("patte_d") },
];
const ANIMS_172: &[Anim] = &[
    Anim { target: "corps", kind: AnimKind::Bob, amp: 0.03 },
    Anim { target: "bouclier", kind: AnimKind::TailY, amp: 0.0 },
    Anim { target: "epee", kind: AnimKind::TailZ, amp: 0.0 },
    Anim { target: "patte_g", kind: AnimKind::SwingZ, amp: 0.0 },
    Anim { target: "patte_d", kind: AnimKind::SwingZ, amp: 0.5 },
];
const PIECES_173: &[Piece] = &[
    Piece { shape: Shape::Sphere, pos: [0.0, 0.5, 0.0], size: [0.95, 0.6, 0.5], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [0.5, 0.52, 0.0], size: [0.45, 0.48, 0.44], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Cube, pos: [0.76, 0.46, 0.0], size: [0.18, 0.2, 0.22], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("corps") },
    Piece { shape: Shape::Cone, pos: [0.78, 0.52, 0.14], size: [0.05, 0.2, 0.05], rot: [0.0, 0.0, -75.0], mat: Mat::Blanc, id: Some("corps") },
    Piece { shape: Shape::Cone, pos: [0.78, 0.52, -0.14], size: [0.05, 0.2, 0.05], rot: [0.0, 0.0, -75.0], mat: Mat::Blanc, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [0.62, 0.64, 0.14], size: [0.05, 0.05, 0.05], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [0.62, 0.64, -0.14], size: [0.05, 0.05, 0.05], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("corps") },
    Piece { shape: Shape::Cone, pos: [0.42, 0.8, 0.12], size: [0.08, 0.14, 0.05], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Cone, pos: [0.42, 0.8, -0.12], size: [0.08, 0.14, 0.05], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Cone, pos: [0.0, 0.85, 0.0], size: [0.8, 0.2, 0.1], rot: [0.0, 0.0, -90.0], mat: Mat::C3, id: Some("corps") },
    Piece { shape: Shape::Cyl, pos: [-0.52, 0.55, 0.0], size: [0.04, 0.2, 0.04], rot: [0.0, 0.0, 60.0], mat: Mat::C3, id: Some("queue") },
    Piece { shape: Shape::Cyl, pos: [0.3, 0.2, 0.16], size: [0.1, 0.4, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("patte0") },
    Piece { shape: Shape::Cyl, pos: [0.3, 0.2, -0.16], size: [0.1, 0.4, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("patte1") },
    Piece { shape: Shape::Cyl, pos: [-0.3, 0.2, 0.16], size: [0.1, 0.4, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("patte2") },
    Piece { shape: Shape::Cyl, pos: [-0.3, 0.2, -0.16], size: [0.1, 0.4, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("patte3") },
];
const ANIMS_173: &[Anim] = &[
    Anim { target: "corps", kind: AnimKind::Bob, amp: 0.04 },
    Anim { target: "queue", kind: AnimKind::TailY, amp: 0.0 },
    Anim { target: "patte0", kind: AnimKind::SwingZ, amp: 0.0 },
    Anim { target: "patte1", kind: AnimKind::SwingZ, amp: 0.5 },
    Anim { target: "patte2", kind: AnimKind::SwingZ, amp: 0.5 },
    Anim { target: "patte3", kind: AnimKind::SwingZ, amp: 0.0 },
];
const PIECES_174: &[Piece] = &[
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
    Piece { shape: Shape::Cone, pos: [-0.62, 0.78, 0.0], size: [0.12, 0.55, 0.12], rot: [0.0, 0.0, 110.0], mat: Mat::C3, id: Some("queue") },
    Piece { shape: Shape::Cube, pos: [0.0, 0.98, 0.0], size: [0.5, 0.06, 0.5], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: Some("corps") },
    Piece { shape: Shape::Cyl, pos: [0.4, 0.34, 0.14], size: [0.12, 0.7, 0.12], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("patte0") },
    Piece { shape: Shape::Cube, pos: [0.43, 0.03, 0.14], size: [0.18, 0.06, 0.14], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("patte0") },
    Piece { shape: Shape::Cyl, pos: [0.4, 0.34, -0.14], size: [0.12, 0.7, 0.12], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("patte1") },
    Piece { shape: Shape::Cube, pos: [0.43, 0.03, -0.14], size: [0.18, 0.06, 0.14], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("patte1") },
    Piece { shape: Shape::Cyl, pos: [-0.4, 0.34, 0.14], size: [0.12, 0.7, 0.12], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("patte2") },
    Piece { shape: Shape::Cube, pos: [-0.37, 0.03, 0.14], size: [0.18, 0.06, 0.14], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("patte2") },
    Piece { shape: Shape::Cyl, pos: [-0.4, 0.34, -0.14], size: [0.12, 0.7, 0.12], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("patte3") },
    Piece { shape: Shape::Cube, pos: [-0.37, 0.03, -0.14], size: [0.18, 0.06, 0.14], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("patte3") },
    Piece { shape: Shape::Cube, pos: [-0.02, 1.32, 0.0], size: [0.32, 0.46, 0.32], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [0.02, 1.7, 0.0], size: [0.26, 0.28, 0.26], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: Some("corps") },
    Piece { shape: Shape::Cube, pos: [0.15, 1.7, 0.0], size: [0.04, 0.05, 0.2], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("corps") },
    Piece { shape: Shape::Cone, pos: [0.0, 1.98, 0.0], size: [0.1, 0.3, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::LumiereRouge, id: Some("corps") },
    Piece { shape: Shape::Cyl, pos: [0.5, 1.5, 0.28], size: [0.04, 1.6, 0.04], rot: [0.0, 0.0, -75.0], mat: Mat::Sombre, id: Some("corps") },
    Piece { shape: Shape::Cube, pos: [0.1, 1.15, 0.3], size: [0.06, 0.55, 0.36], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("corps") },
];
const ANIMS_174: &[Anim] = &[
    Anim { target: "corps", kind: AnimKind::Bob, amp: 0.05 },
    Anim { target: "queue", kind: AnimKind::TailY, amp: 0.0 },
    Anim { target: "patte0", kind: AnimKind::SwingZ, amp: 0.0 },
    Anim { target: "patte1", kind: AnimKind::SwingZ, amp: 0.5 },
    Anim { target: "patte2", kind: AnimKind::SwingZ, amp: 0.5 },
    Anim { target: "patte3", kind: AnimKind::SwingZ, amp: 0.0 },
];
const PIECES_175: &[Piece] = &[
    Piece { shape: Shape::Cube, pos: [-1.2, 0.55, 0.0], size: [1.4, 0.12, 0.8], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [-1.2, 0.75, 0.4], size: [1.4, 0.3, 0.04], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [-1.2, 0.75, -0.4], size: [1.4, 0.3, 0.04], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [-1.9, 0.75, 0.0], size: [0.04, 0.3, 0.8], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [-1.0, 0.95, 0.1], size: [0.4, 0.5, 0.4], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [-1.5, 0.95, -0.1], size: [0.4, 0.5, 0.4], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [-1.0, 1.05, 0.1], size: [0.42, 0.04, 0.42], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cube, pos: [-0.4, 0.55, 0.0], size: [0.7, 0.04, 0.08], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cube, pos: [-0.3, 1.0, 0.0], size: [0.1, 0.4, 0.5], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cyl, pos: [-1.5, 0.3, 0.46], size: [0.6, 0.08, 0.6], rot: [90.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cyl, pos: [-1.5, 0.3, 0.505], size: [0.3, 0.05, 0.3], rot: [90.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cyl, pos: [-1.5, 0.3, -0.46], size: [0.6, 0.08, 0.6], rot: [90.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cyl, pos: [-1.5, 0.3, -0.505], size: [0.3, 0.05, 0.3], rot: [90.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.9, 0.3, 0.46], size: [0.6, 0.08, 0.6], rot: [90.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.9, 0.3, 0.505], size: [0.3, 0.05, 0.3], rot: [90.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.9, 0.3, -0.46], size: [0.6, 0.08, 0.6], rot: [90.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.9, 0.3, -0.505], size: [0.3, 0.05, 0.3], rot: [90.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Sphere, pos: [0.2, 0.78, 0.0], size: [1.05, 0.52, 0.46], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("acorps") },
    Piece { shape: Shape::Sphere, pos: [0.5, 0.84, 0.0], size: [0.55, 0.58, 0.44], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("acorps") },
    Piece { shape: Shape::Sphere, pos: [-0.12, 0.8, 0.0], size: [0.55, 0.55, 0.46], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("acorps") },
    Piece { shape: Shape::Sphere, pos: [0.78, 1.05, 0.0], size: [0.3, 0.6, 0.26], rot: [0.0, 0.0, -32.0], mat: Mat::C2, id: Some("acorps") },
    Piece { shape: Shape::Sphere, pos: [1.04, 1.22, 0.0], size: [0.42, 0.26, 0.22], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("acorps") },
    Piece { shape: Shape::Sphere, pos: [1.22, 1.15, 0.0], size: [0.24, 0.2, 0.18], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("acorps") },
    Piece { shape: Shape::Cone, pos: [0.98, 1.42, 0.07], size: [0.07, 0.14, 0.05], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("acorps") },
    Piece { shape: Shape::Cone, pos: [0.98, 1.42, -0.07], size: [0.07, 0.14, 0.05], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("acorps") },
    Piece { shape: Shape::Sphere, pos: [1.12, 1.27, 0.1], size: [0.05, 0.05, 0.05], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("acorps") },
    Piece { shape: Shape::Sphere, pos: [1.12, 1.27, -0.1], size: [0.05, 0.05, 0.05], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("acorps") },
    Piece { shape: Shape::Cone, pos: [-0.42, 0.78, 0.0], size: [0.12, 0.55, 0.12], rot: [0.0, 0.0, 110.0], mat: Mat::C3, id: Some("aqueue") },
    Piece { shape: Shape::Cube, pos: [0.2, 0.98, 0.0], size: [0.5, 0.06, 0.5], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: Some("acorps") },
    Piece { shape: Shape::Cyl, pos: [0.6, 0.34, 0.14], size: [0.12, 0.7, 0.12], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("apatte0") },
    Piece { shape: Shape::Cube, pos: [0.63, 0.03, 0.14], size: [0.18, 0.06, 0.14], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("apatte0") },
    Piece { shape: Shape::Cyl, pos: [0.6, 0.34, -0.14], size: [0.12, 0.7, 0.12], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("apatte1") },
    Piece { shape: Shape::Cube, pos: [0.63, 0.03, -0.14], size: [0.18, 0.06, 0.14], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("apatte1") },
    Piece { shape: Shape::Cyl, pos: [-0.2, 0.34, 0.14], size: [0.12, 0.7, 0.12], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("apatte2") },
    Piece { shape: Shape::Cube, pos: [-0.17, 0.03, 0.14], size: [0.18, 0.06, 0.14], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("apatte2") },
    Piece { shape: Shape::Cyl, pos: [-0.2, 0.34, -0.14], size: [0.12, 0.7, 0.12], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("apatte3") },
    Piece { shape: Shape::Cube, pos: [-0.17, 0.03, -0.14], size: [0.18, 0.06, 0.14], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("apatte3") },
];
const ANIMS_175: &[Anim] = &[
    Anim { target: "acorps", kind: AnimKind::Bob, amp: 0.05 },
    Anim { target: "aqueue", kind: AnimKind::TailY, amp: 0.0 },
    Anim { target: "apatte0", kind: AnimKind::SwingZ, amp: 0.0 },
    Anim { target: "apatte1", kind: AnimKind::SwingZ, amp: 0.5 },
    Anim { target: "apatte2", kind: AnimKind::SwingZ, amp: 0.5 },
    Anim { target: "apatte3", kind: AnimKind::SwingZ, amp: 0.0 },
];
const PIECES_176: &[Piece] = &[
    Piece { shape: Shape::Sphere, pos: [0.0, 1.15, 0.0], size: [1.0, 1.3, 0.9], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [0.1, 2.0, 0.0], size: [0.62, 0.62, 0.6], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [0.38, 2.05, 0.15], size: [0.1, 0.1, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: None },
    Piece { shape: Shape::Sphere, pos: [0.38, 2.05, -0.15], size: [0.1, 0.1, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: None },
    Piece { shape: Shape::Sphere, pos: [0.42, 2.05, 0.15], size: [0.05, 0.05, 0.05], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [0.42, 2.05, -0.15], size: [0.05, 0.05, 0.05], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("corps") },
    Piece { shape: Shape::Cube, pos: [0.4, 1.85, 0.0], size: [0.06, 0.08, 0.36], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("corps") },
    Piece { shape: Shape::Cone, pos: [0.35, 1.9, 0.12], size: [0.06, 0.12, 0.05], rot: [180.0, 0.0, 0.0], mat: Mat::Blanc, id: Some("corps") },
    Piece { shape: Shape::Cone, pos: [0.35, 1.9, -0.12], size: [0.06, 0.12, 0.05], rot: [180.0, 0.0, 0.0], mat: Mat::Blanc, id: Some("corps") },
    Piece { shape: Shape::Cube, pos: [0.0, 0.85, 0.0], size: [0.95, 0.3, 0.9], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("corps") },
    Piece { shape: Shape::Cyl, pos: [0.55, 1.75, -0.85], size: [0.14, 1.8, 0.14], rot: [0.0, 0.0, -20.0], mat: Mat::C3, id: Some("massue") },
    Piece { shape: Shape::Sphere, pos: [0.8, 2.6, -0.85], size: [0.4, 0.5, 0.4], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("massue") },
    Piece { shape: Shape::Cyl, pos: [0.0, 1.3, 0.62], size: [0.3, 1.1, 0.3], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("bras_g") },
    Piece { shape: Shape::Cyl, pos: [0.0, 0.4, 0.28], size: [0.34, 0.8, 0.34], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("patte_g") },
    Piece { shape: Shape::Cube, pos: [0.15, 0.06, 0.28], size: [0.56, 0.12, 0.4], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("patte_g") },
    Piece { shape: Shape::Cyl, pos: [0.0, 0.4, -0.28], size: [0.34, 0.8, 0.34], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("patte_d") },
    Piece { shape: Shape::Cube, pos: [0.15, 0.06, -0.28], size: [0.56, 0.12, 0.4], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("patte_d") },
];
const ANIMS_176: &[Anim] = &[
    Anim { target: "corps", kind: AnimKind::Bob, amp: 0.03 },
    Anim { target: "massue", kind: AnimKind::TailZ, amp: 0.0 },
    Anim { target: "bras_g", kind: AnimKind::SwingZ, amp: 0.5 },
    Anim { target: "patte_g", kind: AnimKind::SwingZ, amp: 0.0 },
    Anim { target: "patte_d", kind: AnimKind::SwingZ, amp: 0.5 },
];
const PIECES_177: &[Piece] = &[
    Piece { shape: Shape::Cube, pos: [0.0, 0.5, 0.0], size: [1.9, 0.18, 0.9], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [-0.2, 1.0, 0.3], size: [0.14, 1.0, 0.14], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [-0.2, 1.0, -0.3], size: [0.14, 1.0, 0.14], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [0.8, 0.75, 0.3], size: [0.1, 0.5, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [0.8, 0.75, -0.3], size: [0.1, 0.5, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.2, 1.5, 0.0], size: [0.12, 0.8, 0.12], rot: [90.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cube, pos: [0.4, 1.4, 0.0], size: [1.5, 0.1, 0.1], rot: [0.0, 0.0, 20.0], mat: Mat::C2, id: Some("bras") },
    Piece { shape: Shape::Sphere, pos: [1.1, 1.65, 0.0], size: [0.4, 0.3, 0.4], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: Some("bras") },
    Piece { shape: Shape::Cube, pos: [-0.4, 1.2, 0.0], size: [0.3, 0.3, 0.3], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("bras") },
    Piece { shape: Shape::Cyl, pos: [-0.6, 0.34, 0.5], size: [0.68, 0.1, 0.68], rot: [90.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.6, 0.34, 0.555], size: [0.3, 0.06, 0.3], rot: [90.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.6, 0.34, -0.5], size: [0.68, 0.1, 0.68], rot: [90.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.6, 0.34, -0.555], size: [0.3, 0.06, 0.3], rot: [90.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cyl, pos: [0.6, 0.34, 0.5], size: [0.68, 0.1, 0.68], rot: [90.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cyl, pos: [0.6, 0.34, 0.555], size: [0.3, 0.06, 0.3], rot: [90.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cyl, pos: [0.6, 0.34, -0.5], size: [0.68, 0.1, 0.68], rot: [90.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cyl, pos: [0.6, 0.34, -0.555], size: [0.3, 0.06, 0.3], rot: [90.0, 0.0, 0.0], mat: Mat::C2, id: None },
];
const ANIMS_177: &[Anim] = &[
    Anim { target: "bras", kind: AnimKind::TailZ, amp: 0.0 },
];
const PIECES_178: &[Piece] = &[
    Piece { shape: Shape::Sphere, pos: [0.0, 1.0, 0.0], size: [1.1, 0.5, 0.5], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [0.55, 1.1, 0.0], size: [0.45, 0.42, 0.4], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Cone, pos: [0.82, 1.1, 0.0], size: [0.28, 0.4, 0.28], rot: [0.0, 0.0, -90.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Cone, pos: [0.4, 1.38, 0.1], size: [0.06, 0.3, 0.06], rot: [0.0, 0.0, 10.0], mat: Mat::C3, id: Some("corps") },
    Piece { shape: Shape::Cone, pos: [0.4, 1.38, -0.1], size: [0.06, 0.3, 0.06], rot: [0.0, 0.0, 10.0], mat: Mat::C3, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [0.62, 1.2, 0.12], size: [0.07, 0.07, 0.05], rot: [0.0, 0.0, 0.0], mat: Mat::LumiereRouge, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [0.62, 1.2, -0.12], size: [0.07, 0.07, 0.05], rot: [0.0, 0.0, 0.0], mat: Mat::LumiereRouge, id: Some("corps") },
    Piece { shape: Shape::Cone, pos: [1.0, 1.1, 0.0], size: [0.18, 0.3, 0.18], rot: [0.0, 0.0, -90.0], mat: Mat::LumiereRouge, id: Some("flamme") },
    Piece { shape: Shape::Cone, pos: [-0.7, 0.95, 0.0], size: [0.3, 1.0, 0.3], rot: [0.0, 0.0, 105.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Cone, pos: [-1.2, 0.9, 0.0], size: [0.12, 0.3, 0.12], rot: [0.0, 0.0, 105.0], mat: Mat::C3, id: Some("corps") },
    Piece { shape: Shape::Cone, pos: [0.3, 1.32, 0.0], size: [0.1, 0.22, 0.08], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("corps") },
    Piece { shape: Shape::Cone, pos: [0.0, 1.32, 0.0], size: [0.1, 0.22, 0.08], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("corps") },
    Piece { shape: Shape::Cone, pos: [-0.3, 1.32, 0.0], size: [0.1, 0.22, 0.08], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("corps") },
    Piece { shape: Shape::Cone, pos: [-0.6, 1.32, 0.0], size: [0.1, 0.22, 0.08], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("corps") },
    Piece { shape: Shape::Cube, pos: [0.0, 1.1, 0.7], size: [0.7, 0.04, 1.0], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("aile_g") },
    Piece { shape: Shape::Cube, pos: [-0.2, 1.1, 1.3], size: [0.6, 0.04, 0.6], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("aile_g") },
    Piece { shape: Shape::Cube, pos: [0.1, 1.11, 1.6], size: [0.08, 0.05, 1.0], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("aile_g") },
    Piece { shape: Shape::Cube, pos: [-0.25, 1.11, 1.5], size: [0.08, 0.05, 0.8], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("aile_g") },
    Piece { shape: Shape::Cube, pos: [0.0, 1.1, -0.7], size: [0.7, 0.04, 1.0], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("aile_d") },
    Piece { shape: Shape::Cube, pos: [-0.2, 1.1, -1.3], size: [0.6, 0.04, 0.6], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("aile_d") },
    Piece { shape: Shape::Cube, pos: [0.1, 1.11, -1.6], size: [0.08, 0.05, 1.0], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("aile_d") },
    Piece { shape: Shape::Cube, pos: [-0.25, 1.11, -1.5], size: [0.08, 0.05, 0.8], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("aile_d") },
];
const ANIMS_178: &[Anim] = &[
    Anim { target: "corps", kind: AnimKind::Bob, amp: 0.04 },
    Anim { target: "flamme", kind: AnimKind::Pulse, amp: 0.0 },
    Anim { target: "aile_g", kind: AnimKind::SwingXBase, amp: 0.0 },
    Anim { target: "aile_d", kind: AnimKind::SwingXBase, amp: 0.0 },
];
const MOYENAGE_ANIMALS: [Animal; 10] = [
    Animal { model: Model { c2: [244, 240, 224], c3: [232, 160, 42], pieces: PIECES_169, anims: ANIMS_169 }, len: 0.6, width: 0.4, points: 300, dangerous: false, flying: false },
    Animal { model: Model { c2: [242, 178, 184], c3: [216, 138, 148], pieces: PIECES_170, anims: ANIMS_170 }, len: 0.9, width: 0.5, points: 450, dangerous: false, flying: false },
    Animal { model: Model { c2: [111, 176, 74], c3: [90, 63, 42], pieces: PIECES_171, anims: ANIMS_171 }, len: 0.6, width: 0.5, points: 700, dangerous: true, flying: false },
    Animal { model: Model { c2: [216, 220, 232], c3: [192, 57, 43], pieces: PIECES_172, anims: ANIMS_172 }, len: 0.9, width: 0.9, points: 1000, dangerous: true, flying: false },
    Animal { model: Model { c2: [107, 74, 48], c3: [58, 42, 26], pieces: PIECES_173, anims: ANIMS_173 }, len: 1.3, width: 0.6, points: 1300, dangerous: true, flying: false },
    Animal { model: Model { c2: [139, 90, 43], c3: [192, 57, 43], pieces: PIECES_174, anims: ANIMS_174 }, len: 1.9, width: 0.6, points: 1700, dangerous: true, flying: false },
    Animal { model: Model { c2: [107, 74, 48], c3: [138, 90, 43], pieces: PIECES_175, anims: ANIMS_175 }, len: 3.3, width: 1.0, points: 2000, dangerous: true, flying: false },
    Animal { model: Model { c2: [122, 160, 90], c3: [122, 77, 36], pieces: PIECES_176, anims: ANIMS_176 }, len: 1.3, width: 1.9, points: 2500, dangerous: true, flying: false },
    Animal { model: Model { c2: [201, 160, 99], c3: [107, 74, 48], pieces: PIECES_177, anims: ANIMS_177 }, len: 2.0, width: 1.0, points: 2900, dangerous: true, flying: false },
    Animal { model: Model { c2: [79, 154, 58], c3: [192, 57, 43], pieces: PIECES_178, anims: ANIMS_178 }, len: 2.2, width: 3.4, points: 3200, dangerous: true, flying: true },
];
fn moyenage_animal_name(i: usize) -> &'static str {
    match i {
        0 => tx!("Poule Pressée", "Hurried Hen", "Eilige Henne", "Gallina Frettolosa", "Gallina con Prisa").t(),
        1 => tx!("Cochon Gourmand", "Greedy Pig", "Gieriges Schwein", "Maiale Goloso", "Cerdo Glotón").t(),
        2 => tx!("Gobelin Voleur", "Thieving Goblin", "Diebischer Kobold", "Goblin Ladro", "Duende Ladrón").t(),
        3 => tx!("Chevalier Bruyant", "Clanking Knight", "Scheppernder Ritter", "Cavaliere Rumoroso", "Caballero Ruidoso").t(),
        4 => tx!("Sanglier Furieux", "Furious Boar", "Wütender Eber", "Cinghiale Furioso", "Jabalí Furioso").t(),
        5 => tx!("Chevalier au Galop", "Galloping Knight", "Galoppierender Ritter", "Cavaliere al Galoppo", "Caballero al Galope").t(),
        6 => tx!("Charrette de Marchand", "Merchant Cart", "Händlerkarren", "Carro del Mercante", "Carreta de Mercader").t(),
        7 => tx!("Ogre Pas Content", "Cranky Ogre", "Mürrischer Oger", "Orco Scontroso", "Ogro Gruñón").t(),
        8 => tx!("Catapulte Roulante", "Rolling Catapult", "Rollende Katapult", "Catapulta Rotolante", "Catapulta Rodante").t(),
        9 => tx!("Dragon Cracheur", "Fire-Breathing Dragon", "Feuerspeiender Drache", "Drago Sputafuoco", "Dragón Escupefuego").t(),
        _ => "?",
    }
}
const PIECES_179: &[Piece] = &[
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
const ANIMS_179: &[Anim] = &[
    Anim { target: "corps", kind: AnimKind::Bob, amp: 0.02 },
];
const PIECES_180: &[Piece] = &[
    Piece { shape: Shape::Sphere, pos: [0.12, 0.78, 0.0], size: [0.5, 0.34, 0.5], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [0.12, 0.67, 0.0], size: [0.56, 0.05, 0.56], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [0.34, 0.74, 0.0], size: [0.05, 0.05, 0.3], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: None },
    Piece { shape: Shape::Cone, pos: [0.1, 1.08, 0.0], size: [0.12, 0.34, 0.12], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Sphere, pos: [0.1, 1.28, 0.0], size: [0.1, 0.1, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
];
const ANIMS_180: &[Anim] = &[
];
const PIECES_181: &[Piece] = &[
    Piece { shape: Shape::Cube, pos: [-0.9, 0.55, 0.0], size: [1.2, 0.1, 0.7], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [-1.3, 0.85, 0.0], size: [0.5, 0.5, 0.6], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [-0.9, 0.3, 0.0], size: [0.1, 0.1, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cube, pos: [-0.6, 0.62, 0.0], size: [1.0, 0.04, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Sphere, pos: [-1.0, 1.25, 0.0], size: [0.22, 0.24, 0.22], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [-1.0, 1.0, 0.0], size: [0.3, 0.4, 0.3], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [-1.0, 1.42, 0.0], size: [0.46, 0.04, 0.46], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cyl, pos: [-1.0, 1.5, 0.0], size: [0.24, 0.14, 0.24], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cyl, pos: [-1.2, 0.28, 0.42], size: [0.56, 0.08, 0.56], rot: [90.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cyl, pos: [-1.2, 0.28, 0.465], size: [0.28, 0.05, 0.28], rot: [90.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [-1.2, 0.28, -0.42], size: [0.56, 0.08, 0.56], rot: [90.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cyl, pos: [-1.2, 0.28, -0.465], size: [0.28, 0.05, 0.28], rot: [90.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.5, 0.28, 0.42], size: [0.56, 0.08, 0.56], rot: [90.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.5, 0.28, 0.465], size: [0.28, 0.05, 0.28], rot: [90.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.5, 0.28, -0.42], size: [0.56, 0.08, 0.56], rot: [90.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.5, 0.28, -0.465], size: [0.28, 0.05, 0.28], rot: [90.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Sphere, pos: [0.5, 0.78, 0.24], size: [1.05, 0.52, 0.46], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("acorps") },
    Piece { shape: Shape::Sphere, pos: [0.8, 0.84, 0.24], size: [0.55, 0.58, 0.44], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("acorps") },
    Piece { shape: Shape::Sphere, pos: [0.18, 0.8, 0.24], size: [0.55, 0.55, 0.46], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("acorps") },
    Piece { shape: Shape::Sphere, pos: [1.08, 1.05, 0.24], size: [0.3, 0.6, 0.26], rot: [0.0, 0.0, -32.0], mat: Mat::C2, id: Some("acorps") },
    Piece { shape: Shape::Sphere, pos: [1.34, 1.22, 0.24], size: [0.42, 0.26, 0.22], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("acorps") },
    Piece { shape: Shape::Sphere, pos: [1.52, 1.15, 0.24], size: [0.24, 0.2, 0.18], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("acorps") },
    Piece { shape: Shape::Cone, pos: [1.28, 1.42, 0.31], size: [0.07, 0.14, 0.05], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("acorps") },
    Piece { shape: Shape::Cone, pos: [1.28, 1.42, 0.17], size: [0.07, 0.14, 0.05], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("acorps") },
    Piece { shape: Shape::Sphere, pos: [1.42, 1.27, 0.34], size: [0.05, 0.05, 0.05], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("acorps") },
    Piece { shape: Shape::Sphere, pos: [1.42, 1.27, 0.14], size: [0.05, 0.05, 0.05], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("acorps") },
    Piece { shape: Shape::Cone, pos: [-0.12, 0.78, 0.24], size: [0.12, 0.55, 0.12], rot: [0.0, 0.0, 110.0], mat: Mat::C3, id: Some("aqueue") },
    Piece { shape: Shape::Cube, pos: [0.5, 0.98, 0.24], size: [0.5, 0.06, 0.5], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: Some("acorps") },
    Piece { shape: Shape::Cyl, pos: [0.9, 0.34, 0.38], size: [0.12, 0.7, 0.12], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("apatte0") },
    Piece { shape: Shape::Cube, pos: [0.93, 0.03, 0.38], size: [0.18, 0.06, 0.14], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("apatte0") },
    Piece { shape: Shape::Cyl, pos: [0.9, 0.34, 0.1], size: [0.12, 0.7, 0.12], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("apatte1") },
    Piece { shape: Shape::Cube, pos: [0.93, 0.03, 0.1], size: [0.18, 0.06, 0.14], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("apatte1") },
    Piece { shape: Shape::Cyl, pos: [0.1, 0.34, 0.38], size: [0.12, 0.7, 0.12], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("apatte2") },
    Piece { shape: Shape::Cube, pos: [0.13, 0.03, 0.38], size: [0.18, 0.06, 0.14], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("apatte2") },
    Piece { shape: Shape::Cyl, pos: [0.1, 0.34, 0.1], size: [0.12, 0.7, 0.12], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("apatte3") },
    Piece { shape: Shape::Cube, pos: [0.13, 0.03, 0.1], size: [0.18, 0.06, 0.14], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("apatte3") },
    Piece { shape: Shape::Sphere, pos: [0.5, 0.78, -0.24], size: [1.05, 0.52, 0.46], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("bcorps") },
    Piece { shape: Shape::Sphere, pos: [0.8, 0.84, -0.24], size: [0.55, 0.58, 0.44], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("bcorps") },
    Piece { shape: Shape::Sphere, pos: [0.18, 0.8, -0.24], size: [0.55, 0.55, 0.46], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("bcorps") },
    Piece { shape: Shape::Sphere, pos: [1.08, 1.05, -0.24], size: [0.3, 0.6, 0.26], rot: [0.0, 0.0, -32.0], mat: Mat::C2, id: Some("bcorps") },
    Piece { shape: Shape::Sphere, pos: [1.34, 1.22, -0.24], size: [0.42, 0.26, 0.22], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("bcorps") },
    Piece { shape: Shape::Sphere, pos: [1.52, 1.15, -0.24], size: [0.24, 0.2, 0.18], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("bcorps") },
    Piece { shape: Shape::Cone, pos: [1.28, 1.42, -0.17], size: [0.07, 0.14, 0.05], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("bcorps") },
    Piece { shape: Shape::Cone, pos: [1.28, 1.42, -0.31], size: [0.07, 0.14, 0.05], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("bcorps") },
    Piece { shape: Shape::Sphere, pos: [1.42, 1.27, -0.14], size: [0.05, 0.05, 0.05], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("bcorps") },
    Piece { shape: Shape::Sphere, pos: [1.42, 1.27, -0.34], size: [0.05, 0.05, 0.05], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("bcorps") },
    Piece { shape: Shape::Cone, pos: [-0.12, 0.78, -0.24], size: [0.12, 0.55, 0.12], rot: [0.0, 0.0, 110.0], mat: Mat::C3, id: Some("bqueue") },
    Piece { shape: Shape::Cube, pos: [0.5, 0.98, -0.24], size: [0.5, 0.06, 0.5], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: Some("bcorps") },
    Piece { shape: Shape::Cyl, pos: [0.9, 0.34, -0.1], size: [0.12, 0.7, 0.12], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("bpatte0") },
    Piece { shape: Shape::Cube, pos: [0.93, 0.03, -0.1], size: [0.18, 0.06, 0.14], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("bpatte0") },
    Piece { shape: Shape::Cyl, pos: [0.9, 0.34, -0.38], size: [0.12, 0.7, 0.12], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("bpatte1") },
    Piece { shape: Shape::Cube, pos: [0.93, 0.03, -0.38], size: [0.18, 0.06, 0.14], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("bpatte1") },
    Piece { shape: Shape::Cyl, pos: [0.1, 0.34, -0.1], size: [0.12, 0.7, 0.12], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("bpatte2") },
    Piece { shape: Shape::Cube, pos: [0.13, 0.03, -0.1], size: [0.18, 0.06, 0.14], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("bpatte2") },
    Piece { shape: Shape::Cyl, pos: [0.1, 0.34, -0.38], size: [0.12, 0.7, 0.12], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("bpatte3") },
    Piece { shape: Shape::Cube, pos: [0.13, 0.03, -0.38], size: [0.18, 0.06, 0.14], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("bpatte3") },
];
const ANIMS_181: &[Anim] = &[
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
const PIECES_182: &[Piece] = &[
    Piece { shape: Shape::Cube, pos: [0.0, 0.55, 0.0], size: [1.9, 0.12, 0.85], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.75, 0.42], size: [1.9, 0.3, 0.04], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.75, -0.42], size: [1.9, 0.3, 0.04], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [0.94, 0.75, 0.0], size: [0.04, 0.3, 0.85], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [-0.94, 0.75, 0.0], size: [0.04, 0.3, 0.85], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.4, 1.0, 0.0], size: [0.5, 0.55, 0.5], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [0.35, 1.0, 0.1], size: [0.5, 0.55, 0.5], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.4, 1.12, 0.0], size: [0.52, 0.04, 0.52], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cyl, pos: [0.35, 1.12, 0.1], size: [0.52, 0.04, 0.52], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.55, 0.28, 0.46], size: [0.56, 0.08, 0.56], rot: [90.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.55, 0.28, 0.505], size: [0.28, 0.05, 0.28], rot: [90.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.55, 0.28, -0.46], size: [0.56, 0.08, 0.56], rot: [90.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.55, 0.28, -0.505], size: [0.28, 0.05, 0.28], rot: [90.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [0.55, 0.28, 0.46], size: [0.56, 0.08, 0.56], rot: [90.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cyl, pos: [0.55, 0.28, 0.505], size: [0.28, 0.05, 0.28], rot: [90.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [0.55, 0.28, -0.46], size: [0.56, 0.08, 0.56], rot: [90.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cyl, pos: [0.55, 0.28, -0.505], size: [0.28, 0.05, 0.28], rot: [90.0, 0.0, 0.0], mat: Mat::C3, id: None },
];
const ANIMS_182: &[Anim] = &[
];
const PIECES_183: &[Piece] = &[
    Piece { shape: Shape::Cube, pos: [0.0, 0.55, 0.0], size: [1.9, 0.12, 0.85], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.75, 0.42], size: [1.9, 0.3, 0.04], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.75, -0.42], size: [1.9, 0.3, 0.04], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [0.94, 0.75, 0.0], size: [0.04, 0.3, 0.85], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [-0.94, 0.75, 0.0], size: [0.04, 0.3, 0.85], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 1.0, 0.0], size: [1.7, 0.5, 0.7], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 1.28, 0.0], size: [1.5, 0.1, 0.55], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.55, 0.28, 0.46], size: [0.56, 0.08, 0.56], rot: [90.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.55, 0.28, 0.505], size: [0.28, 0.05, 0.28], rot: [90.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.55, 0.28, -0.46], size: [0.56, 0.08, 0.56], rot: [90.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.55, 0.28, -0.505], size: [0.28, 0.05, 0.28], rot: [90.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [0.55, 0.28, 0.46], size: [0.56, 0.08, 0.56], rot: [90.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cyl, pos: [0.55, 0.28, 0.505], size: [0.28, 0.05, 0.28], rot: [90.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [0.55, 0.28, -0.46], size: [0.56, 0.08, 0.56], rot: [90.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cyl, pos: [0.55, 0.28, -0.505], size: [0.28, 0.05, 0.28], rot: [90.0, 0.0, 0.0], mat: Mat::C3, id: None },
];
const ANIMS_183: &[Anim] = &[
];
const PIECES_184: &[Piece] = &[
    Piece { shape: Shape::Cube, pos: [0.0, 0.55, 0.0], size: [1.9, 0.12, 0.85], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.75, 0.42], size: [1.9, 0.3, 0.04], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.75, -0.42], size: [1.9, 0.3, 0.04], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [0.94, 0.75, 0.0], size: [0.04, 0.3, 0.85], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [-0.94, 0.75, 0.0], size: [0.04, 0.3, 0.85], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 1.05, -0.3], size: [1.7, 0.7, 0.04], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 1.05, -0.1], size: [1.7, 0.7, 0.04], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 1.05, 0.1], size: [1.7, 0.7, 0.04], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 1.05, 0.3], size: [1.7, 0.7, 0.04], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 1.4, 0.0], size: [1.8, 0.05, 0.8], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.55, 0.28, 0.46], size: [0.56, 0.08, 0.56], rot: [90.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.55, 0.28, 0.505], size: [0.28, 0.05, 0.28], rot: [90.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.55, 0.28, -0.46], size: [0.56, 0.08, 0.56], rot: [90.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.55, 0.28, -0.505], size: [0.28, 0.05, 0.28], rot: [90.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [0.55, 0.28, 0.46], size: [0.56, 0.08, 0.56], rot: [90.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cyl, pos: [0.55, 0.28, 0.505], size: [0.28, 0.05, 0.28], rot: [90.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [0.55, 0.28, -0.46], size: [0.56, 0.08, 0.56], rot: [90.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cyl, pos: [0.55, 0.28, -0.505], size: [0.28, 0.05, 0.28], rot: [90.0, 0.0, 0.0], mat: Mat::C3, id: None },
];
const ANIMS_184: &[Anim] = &[
];
const MOYENAGE_TRAIN: [TrainCar; 4] = [
    TrainCar { model: Model { c2: [139, 90, 43], c3: [122, 77, 36], pieces: PIECES_181, anims: ANIMS_181 }, len: 3.6 },
    TrainCar { model: Model { c2: [192, 57, 43], c3: [122, 77, 36], pieces: PIECES_182, anims: ANIMS_182 }, len: 2.1 },
    TrainCar { model: Model { c2: [230, 197, 102], c3: [122, 77, 36], pieces: PIECES_183, anims: ANIMS_183 }, len: 2.1 },
    TrainCar { model: Model { c2: [192, 57, 43], c3: [122, 77, 36], pieces: PIECES_184, anims: ANIMS_184 }, len: 2.1 },
];
const MOYENAGE_COLORS: WorldColors = WorldColors {
    grass_a: [116, 168, 76],
    grass_b: [106, 154, 70],
    water: [63, 134, 160],
    road: [141, 129, 113],
    marking: [168, 154, 134],
    log: [122, 77, 36],
    sky: [154, 208, 240],
};
fn moyenage_name() -> &'static str { tx!("Royaume de Croâ", "Kingdom of Croak", "Quak-Königreich", "Regno di Gra", "Reino de Croa").t() }
fn moyenage_banner() -> &'static str { tx!("Bienvenue au Royaume de Croâ ! Le dragon est déjà au courant.", "Welcome to the Kingdom of Croak! The dragon already knows.", "Willkommen im Quak-Königreich! Der Drache weiß es schon.", "Benvenuto nel Regno di Gra! Il drago lo sa già.", "¡Bienvenido al Reino de Croa! El dragón ya lo sabe.").t() }
fn moyenage_hero_name() -> &'static str { tx!("Grenouille Chevalier", "Knight Frog", "Ritter-Frosch", "Rana Cavaliere", "Rana Caballero").t() }
pub static MOYENAGE_THEME: Theme = Theme {
    train: &MOYENAGE_TRAIN, only_own: true,
    blocks: &MOYENAGE_BLOCKS, sides: &MOYENAGE_SIDES, deco: &MOYENAGE_DECO, animals: &MOYENAGE_ANIMALS,
    block_mix: &[0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 3, 3, 3, 4, 4, 4], side_mix: &[100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 1, 1, 1, 2, 2, 2, 0, 0],
    side_pct: 7, vehicles: &[], colors: MOYENAGE_COLORS, points: 4800,
    name: moyenage_name, banner: moyenage_banner, animal_name: moyenage_animal_name, hero_name: moyenage_hero_name,
    hero: Model { c2: [192, 57, 43], c3: [200, 204, 216], pieces: PIECES_179, anims: ANIMS_179 }, hat: Model { c2: [192, 57, 43], c3: [200, 204, 216], pieces: PIECES_180, anims: ANIMS_180 },
};
const PIECES_185: &[Piece] = &[
    Piece { shape: Shape::Cyl, pos: [0.0, 0.3, 0.0], size: [0.2, 0.6, 0.2], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [0.08, 0.8, 0.0], size: [0.18, 0.5, 0.18], rot: [0.0, 0.0, -10.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [0.2, 1.2, 0.0], size: [0.16, 0.5, 0.16], rot: [0.0, 0.0, -18.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cone, pos: [0.3, 1.5, 0.0], size: [0.14, 0.8, 0.14], rot: [0.0, 0.0, 70.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cone, pos: [0.3, 1.5, 0.0], size: [0.14, 0.8, 0.14], rot: [0.0, 0.0, -70.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cone, pos: [0.3, 1.5, 0.0], size: [0.14, 0.8, 0.14], rot: [70.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cone, pos: [0.3, 1.5, 0.0], size: [0.14, 0.8, 0.14], rot: [-70.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Sphere, pos: [0.32, 1.4, 0.08], size: [0.12, 0.12, 0.12], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Sphere, pos: [0.32, 1.4, -0.08], size: [0.12, 0.12, 0.12], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
];
const ANIMS_185: &[Anim] = &[
];
const PIECES_186: &[Piece] = &[
    Piece { shape: Shape::Cube, pos: [0.0, 0.2, 0.0], size: [0.8, 0.4, 0.55], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cyl, pos: [0.0, 0.42, 0.0], size: [0.55, 0.8, 0.55], rot: [0.0, 0.0, 90.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.2, 0.28], size: [0.82, 0.06, 0.02], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.4, 0.0], size: [0.1, 0.44, 0.58], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.3, 0.29], size: [0.1, 0.12, 0.03], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Sphere, pos: [0.0, 0.62, 0.0], size: [0.18, 0.1, 0.18], rot: [0.0, 0.0, 0.0], mat: Mat::Phare, id: Some("or") },
];
const ANIMS_186: &[Anim] = &[
    Anim { target: "or", kind: AnimKind::BlinkSlow, amp: 0.0 },
];
const PIECES_187: &[Piece] = &[
    Piece { shape: Shape::Cyl, pos: [-0.15, 0.3, -0.1], size: [0.5, 0.6, 0.5], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.15, 0.18, -0.1], size: [0.53, 0.05, 0.53], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.15, 0.42, -0.1], size: [0.53, 0.05, 0.53], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [0.25, 0.28, 0.2], size: [0.46, 0.56, 0.46], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cyl, pos: [0.25, 0.16, 0.2], size: [0.49, 0.05, 0.49], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [0.25, 0.4, 0.2], size: [0.49, 0.05, 0.49], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [0.0, 0.8, 0.05], size: [0.42, 0.5, 0.42], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cyl, pos: [0.0, 0.72, 0.05], size: [0.45, 0.05, 0.45], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
];
const ANIMS_187: &[Anim] = &[
];
const PIECES_188: &[Piece] = &[
    Piece { shape: Shape::Cyl, pos: [0.0, 0.5, 0.0], size: [0.1, 1.0, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.85, 0.0], size: [0.5, 0.08, 0.08], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Sphere, pos: [0.0, 1.05, 0.0], size: [0.18, 0.18, 0.18], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.1, 0.0], size: [0.7, 0.1, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cone, pos: [0.36, 0.2, 0.0], size: [0.12, 0.28, 0.1], rot: [0.0, 0.0, 20.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cone, pos: [-0.36, 0.2, 0.0], size: [0.12, 0.28, 0.1], rot: [0.0, 0.0, -20.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.04, 0.0], size: [0.7, 0.08, 0.7], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
];
const ANIMS_188: &[Anim] = &[
];
const PIECES_189: &[Piece] = &[
    Piece { shape: Shape::Cyl, pos: [0.0, 0.7, 0.0], size: [0.07, 1.4, 0.07], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 1.15, 0.28], size: [0.03, 0.4, 0.5], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("fanion") },
    Piece { shape: Shape::Sphere, pos: [0.0, 1.18, 0.28], size: [0.14, 0.14, 0.14], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: Some("fanion") },
    Piece { shape: Shape::Cube, pos: [0.0, 1.07, 0.28], size: [0.05, 0.04, 0.3], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: Some("fanion") },
    Piece { shape: Shape::Cube, pos: [0.0, 0.04, 0.0], size: [0.4, 0.08, 0.4], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
];
const ANIMS_189: &[Anim] = &[
    Anim { target: "fanion", kind: AnimKind::TailY, amp: 0.0 },
];
const PIRATES_BLOCKS: [Model; 5] = [
    Model { c2: [63, 154, 74], c3: [138, 90, 43], pieces: PIECES_185, anims: ANIMS_185 },
    Model { c2: [138, 90, 43], c3: [255, 210, 63], pieces: PIECES_186, anims: ANIMS_186 },
    Model { c2: [160, 105, 58], c3: [58, 42, 26], pieces: PIECES_187, anims: ANIMS_187 },
    Model { c2: [90, 96, 112], c3: [138, 90, 43], pieces: PIECES_188, anims: ANIMS_188 },
    Model { c2: [28, 28, 36], c3: [244, 240, 224], pieces: PIECES_189, anims: ANIMS_189 },
];
const PIECES_190: &[Piece] = &[
    Piece { shape: Shape::Cube, pos: [0.0, 0.5, 0.0], size: [3.0, 1.0, 1.2], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [1.6, 0.8, 0.0], size: [0.6, 1.0, 0.9], rot: [0.0, 0.0, -15.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [-1.4, 1.0, 0.0], size: [0.6, 1.6, 1.1], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 1.05, 0.0], size: [3.1, 0.1, 1.25], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cyl, pos: [0.2, 2.2, 0.0], size: [0.12, 2.4, 0.12], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cube, pos: [0.2, 2.3, 0.0], size: [0.04, 1.6, 1.4], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("voile") },
    Piece { shape: Shape::Cyl, pos: [-0.9, 1.9, 0.0], size: [0.1, 1.6, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cube, pos: [-0.9, 2.0, 0.0], size: [0.04, 1.0, 0.9], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("voile") },
    Piece { shape: Shape::Cube, pos: [0.2, 3.5, 0.2], size: [0.03, 0.3, 0.5], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: Some("fanion") },
];
const ANIMS_190: &[Anim] = &[
    Anim { target: "voile", kind: AnimKind::TailY, amp: 0.0 },
    Anim { target: "fanion", kind: AnimKind::TailY, amp: 0.0 },
];
const PIECES_191: &[Piece] = &[
    Piece { shape: Shape::Cyl, pos: [0.0, 1.2, 0.0], size: [0.9, 2.4, 0.9], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cyl, pos: [0.0, 0.6, 0.0], size: [0.95, 0.5, 0.95], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [0.0, 1.6, 0.0], size: [0.92, 0.45, 0.92], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [0.0, 2.5, 0.0], size: [1.1, 0.12, 1.1], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cyl, pos: [0.0, 2.8, 0.0], size: [0.6, 0.5, 0.6], rot: [0.0, 0.0, 0.0], mat: Mat::Bulle, id: None },
    Piece { shape: Shape::Sphere, pos: [0.0, 2.8, 0.0], size: [0.3, 0.3, 0.3], rot: [0.0, 0.0, 0.0], mat: Mat::Phare, id: Some("feu") },
    Piece { shape: Shape::Cone, pos: [0.0, 3.35, 0.0], size: [0.7, 0.6, 0.7], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
];
const ANIMS_191: &[Anim] = &[
    Anim { target: "feu", kind: AnimKind::Pulse, amp: 0.0 },
];
const PIECES_192: &[Piece] = &[
    Piece { shape: Shape::Cube, pos: [0.0, 0.6, 0.0], size: [1.4, 1.2, 1.1], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cone, pos: [0.0, 1.5, 0.0], size: [1.9, 0.9, 1.5], rot: [0.0, 45.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.4, 0.56], size: [0.4, 0.8, 0.03], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cube, pos: [0.45, 0.7, 0.56], size: [0.3, 0.3, 0.03], rot: [0.0, 0.0, 0.0], mat: Mat::Vitre, id: None },
];
const ANIMS_192: &[Anim] = &[
];
const PIRATES_SIDES: [Model; 3] = [
    Model { c2: [138, 90, 43], c3: [244, 240, 224], pieces: PIECES_190, anims: ANIMS_190 },
    Model { c2: [244, 240, 224], c3: [192, 57, 43], pieces: PIECES_191, anims: ANIMS_191 },
    Model { c2: [201, 160, 99], c3: [63, 154, 74], pieces: PIECES_192, anims: ANIMS_192 },
];
const PIECES_193: &[Piece] = &[
    Piece { shape: Shape::Sphere, pos: [0.0, 0.04, 0.0], size: [0.16, 0.08, 0.14], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cone, pos: [0.0, 0.1, 0.0], size: [0.1, 0.1, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
];
const ANIMS_193: &[Anim] = &[
];
const PIECES_194: &[Piece] = &[
    Piece { shape: Shape::Cube, pos: [0.0, 0.015, 0.0], size: [0.28, 0.03, 0.06], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.015, 0.0], size: [0.28, 0.03, 0.06], rot: [0.0, 72.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.015, 0.0], size: [0.28, 0.03, 0.06], rot: [0.0, 144.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Sphere, pos: [0.0, 0.03, 0.0], size: [0.08, 0.04, 0.08], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
];
const ANIMS_194: &[Anim] = &[
];
const PIECES_195: &[Piece] = &[
    Piece { shape: Shape::Cyl, pos: [0.0, 0.015, 0.0], size: [0.14, 0.03, 0.14], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cyl, pos: [0.0, 0.03, 0.0], size: [0.09, 0.015, 0.09], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
];
const ANIMS_195: &[Anim] = &[
];
const PIECES_196: &[Piece] = &[
    Piece { shape: Shape::Cyl, pos: [0.0, 0.06, 0.0], size: [0.08, 0.16, 0.08], rot: [0.0, 0.0, 80.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [0.04, 0.06, 0.0], size: [0.06, 0.04, 0.03], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
];
const ANIMS_196: &[Anim] = &[
];
const PIRATES_DECO: [Model; 4] = [
    Model { c2: [246, 200, 208], c3: [232, 160, 176], pieces: PIECES_193, anims: ANIMS_193 },
    Model { c2: [255, 138, 63], c3: [224, 112, 42], pieces: PIECES_194, anims: ANIMS_194 },
    Model { c2: [255, 210, 63], c3: [224, 168, 0], pieces: PIECES_195, anims: ANIMS_195 },
    Model { c2: [95, 184, 154], c3: [244, 240, 224], pieces: PIECES_196, anims: ANIMS_196 },
];
const PIECES_197: &[Piece] = &[
    Piece { shape: Shape::Sphere, pos: [0.0, 0.25, 0.0], size: [0.55, 0.28, 0.5], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [0.22, 0.42, 0.1], size: [0.07, 0.07, 0.07], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: None },
    Piece { shape: Shape::Sphere, pos: [0.22, 0.42, -0.1], size: [0.07, 0.07, 0.07], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: None },
    Piece { shape: Shape::Sphere, pos: [0.26, 0.42, 0.1], size: [0.035, 0.035, 0.035], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: None },
    Piece { shape: Shape::Sphere, pos: [0.26, 0.42, -0.1], size: [0.035, 0.035, 0.035], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: None },
    Piece { shape: Shape::Cyl, pos: [0.22, 0.38, 0.1], size: [0.025, 0.12, 0.025], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Cyl, pos: [0.22, 0.38, -0.1], size: [0.025, 0.12, 0.025], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Cyl, pos: [0.15, 0.3, 0.32], size: [0.07, 0.3, 0.07], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("pince_g") },
    Piece { shape: Shape::Sphere, pos: [0.3, 0.34, 0.32], size: [0.18, 0.14, 0.14], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("pince_g") },
    Piece { shape: Shape::Cone, pos: [0.4, 0.34, 0.32], size: [0.08, 0.14, 0.05], rot: [0.0, 0.0, -90.0], mat: Mat::C3, id: Some("pince_g") },
    Piece { shape: Shape::Cyl, pos: [0.15, 0.3, -0.32], size: [0.07, 0.3, 0.07], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("pince_d") },
    Piece { shape: Shape::Sphere, pos: [0.3, 0.34, -0.32], size: [0.18, 0.14, 0.14], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("pince_d") },
    Piece { shape: Shape::Cone, pos: [0.4, 0.34, -0.32], size: [0.08, 0.14, 0.05], rot: [0.0, 0.0, -90.0], mat: Mat::C3, id: Some("pince_d") },
    Piece { shape: Shape::Cyl, pos: [0.1, 0.12, 0.28], size: [0.04, 0.25, 0.04], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("patte0") },
    Piece { shape: Shape::Cyl, pos: [-0.05, 0.12, 0.3], size: [0.04, 0.25, 0.04], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("patte1") },
    Piece { shape: Shape::Cyl, pos: [-0.2, 0.12, 0.28], size: [0.04, 0.25, 0.04], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("patte2") },
    Piece { shape: Shape::Cyl, pos: [0.1, 0.12, -0.28], size: [0.04, 0.25, 0.04], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("patte3") },
    Piece { shape: Shape::Cyl, pos: [-0.05, 0.12, -0.3], size: [0.04, 0.25, 0.04], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("patte4") },
    Piece { shape: Shape::Cyl, pos: [-0.2, 0.12, -0.28], size: [0.04, 0.25, 0.04], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("patte5") },
];
const ANIMS_197: &[Anim] = &[
    Anim { target: "corps", kind: AnimKind::Bob, amp: 0.03 },
    Anim { target: "pince_g", kind: AnimKind::SwingZ, amp: 0.0 },
    Anim { target: "pince_d", kind: AnimKind::SwingZ, amp: 0.5 },
    Anim { target: "patte0", kind: AnimKind::SwingZ, amp: 0.0 },
    Anim { target: "patte1", kind: AnimKind::SwingZ, amp: 0.5 },
    Anim { target: "patte2", kind: AnimKind::SwingZ, amp: 0.0 },
    Anim { target: "patte3", kind: AnimKind::SwingZ, amp: 0.5 },
    Anim { target: "patte4", kind: AnimKind::SwingZ, amp: 0.0 },
    Anim { target: "patte5", kind: AnimKind::SwingZ, amp: 0.5 },
];
const PIECES_198: &[Piece] = &[
    Piece { shape: Shape::Sphere, pos: [0.0, 0.2, 0.0], size: [0.5, 0.26, 0.24], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [0.28, 0.24, 0.0], size: [0.22, 0.2, 0.2], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Cone, pos: [0.42, 0.22, 0.0], size: [0.08, 0.16, 0.08], rot: [0.0, 0.0, -90.0], mat: Mat::C3, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [0.22, 0.36, 0.08], size: [0.08, 0.1, 0.04], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [0.22, 0.36, -0.08], size: [0.08, 0.1, 0.04], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [0.36, 0.28, 0.07], size: [0.04, 0.04, 0.04], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [0.36, 0.28, -0.07], size: [0.04, 0.04, 0.04], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("corps") },
    Piece { shape: Shape::Cyl, pos: [-0.4, 0.2, 0.0], size: [0.03, 0.4, 0.03], rot: [0.0, 0.0, 80.0], mat: Mat::C3, id: Some("queue") },
    Piece { shape: Shape::Cyl, pos: [0.15, 0.07, 0.08], size: [0.05, 0.14, 0.05], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("patte0") },
    Piece { shape: Shape::Cyl, pos: [0.15, 0.07, -0.08], size: [0.05, 0.14, 0.05], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("patte1") },
    Piece { shape: Shape::Cyl, pos: [-0.15, 0.07, 0.08], size: [0.05, 0.14, 0.05], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("patte2") },
    Piece { shape: Shape::Cyl, pos: [-0.15, 0.07, -0.08], size: [0.05, 0.14, 0.05], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("patte3") },
];
const ANIMS_198: &[Anim] = &[
    Anim { target: "corps", kind: AnimKind::Bob, amp: 0.02 },
    Anim { target: "queue", kind: AnimKind::TailY, amp: 0.0 },
    Anim { target: "patte0", kind: AnimKind::SwingZ, amp: 0.0 },
    Anim { target: "patte1", kind: AnimKind::SwingZ, amp: 0.5 },
    Anim { target: "patte2", kind: AnimKind::SwingZ, amp: 0.5 },
    Anim { target: "patte3", kind: AnimKind::SwingZ, amp: 0.0 },
];
const PIECES_199: &[Piece] = &[
    Piece { shape: Shape::Cube, pos: [0.0, 0.95, 0.0], size: [0.42, 0.7, 0.56], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [0.04, 1.5, 0.0], size: [0.38, 0.38, 0.36], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [0.22, 1.54, 0.08], size: [0.07, 0.07, 0.07], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: None },
    Piece { shape: Shape::Sphere, pos: [0.22, 1.54, -0.08], size: [0.07, 0.07, 0.07], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: None },
    Piece { shape: Shape::Cube, pos: [0.2, 1.54, 0.0], size: [0.04, 0.06, 0.2], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("corps") },
    Piece { shape: Shape::Cube, pos: [0.2, 1.58, 0.07], size: [0.04, 0.12, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("corps") },
    Piece { shape: Shape::Cyl, pos: [0.04, 1.72, 0.0], size: [0.62, 0.05, 0.62], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: Some("corps") },
    Piece { shape: Shape::Cone, pos: [0.04, 1.9, 0.0], size: [0.46, 0.3, 0.4], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: Some("corps") },
    Piece { shape: Shape::Cube, pos: [0.3, 1.78, 0.0], size: [0.04, 0.14, 0.14], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: Some("corps") },
    Piece { shape: Shape::Cyl, pos: [0.35, 1.2, -0.38], size: [0.04, 0.9, 0.04], rot: [0.0, 0.0, -40.0], mat: Mat::Blanc, id: Some("sabre") },
    Piece { shape: Shape::Cube, pos: [0.2, 0.9, -0.38], size: [0.03, 0.03, 0.2], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("sabre") },
    Piece { shape: Shape::Cyl, pos: [0.0, 1.0, 0.3], size: [0.1, 0.55, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("bras_g") },
    Piece { shape: Shape::Cone, pos: [0.0, 0.7, 0.3], size: [0.08, 0.16, 0.06], rot: [180.0, 0.0, 0.0], mat: Mat::Sombre, id: Some("bras_g") },
    Piece { shape: Shape::Cyl, pos: [0.0, 0.3, 0.13], size: [0.14, 0.6, 0.14], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: Some("patte_g") },
    Piece { shape: Shape::Cube, pos: [0.08, 0.04, 0.13], size: [0.26, 0.08, 0.16], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("patte_g") },
    Piece { shape: Shape::Cyl, pos: [0.0, 0.3, -0.13], size: [0.14, 0.6, 0.14], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: Some("patte_d") },
    Piece { shape: Shape::Cube, pos: [0.08, 0.04, -0.13], size: [0.26, 0.08, 0.16], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("patte_d") },
];
const ANIMS_199: &[Anim] = &[
    Anim { target: "corps", kind: AnimKind::Bob, amp: 0.03 },
    Anim { target: "sabre", kind: AnimKind::TailZ, amp: 0.0 },
    Anim { target: "bras_g", kind: AnimKind::SwingZ, amp: 0.5 },
    Anim { target: "patte_g", kind: AnimKind::SwingZ, amp: 0.0 },
    Anim { target: "patte_d", kind: AnimKind::SwingZ, amp: 0.5 },
];
const PIECES_200: &[Piece] = &[
    Piece { shape: Shape::Cube, pos: [0.0, 0.95, 0.0], size: [0.42, 0.7, 0.56], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [0.04, 1.5, 0.0], size: [0.38, 0.38, 0.36], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [0.22, 1.54, 0.08], size: [0.07, 0.07, 0.07], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: None },
    Piece { shape: Shape::Sphere, pos: [0.22, 1.54, -0.08], size: [0.07, 0.07, 0.07], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: None },
    Piece { shape: Shape::Cube, pos: [0.2, 1.54, 0.0], size: [0.04, 0.06, 0.2], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("corps") },
    Piece { shape: Shape::Cube, pos: [0.2, 1.58, 0.07], size: [0.04, 0.12, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("corps") },
    Piece { shape: Shape::Cyl, pos: [0.04, 1.72, 0.0], size: [0.62, 0.05, 0.62], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: Some("corps") },
    Piece { shape: Shape::Cone, pos: [0.04, 1.9, 0.0], size: [0.46, 0.3, 0.4], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: Some("corps") },
    Piece { shape: Shape::Cube, pos: [0.3, 1.78, 0.0], size: [0.04, 0.14, 0.14], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: Some("corps") },
    Piece { shape: Shape::Cyl, pos: [0.35, 1.2, -0.38], size: [0.04, 0.9, 0.04], rot: [0.0, 0.0, -40.0], mat: Mat::Blanc, id: Some("sabre") },
    Piece { shape: Shape::Cube, pos: [0.2, 0.9, -0.38], size: [0.03, 0.03, 0.2], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("sabre") },
    Piece { shape: Shape::Cyl, pos: [0.0, 1.0, 0.3], size: [0.1, 0.55, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: Some("bras_g") },
    Piece { shape: Shape::Cone, pos: [0.0, 0.7, 0.3], size: [0.08, 0.16, 0.06], rot: [180.0, 0.0, 0.0], mat: Mat::Sombre, id: Some("bras_g") },
    Piece { shape: Shape::Cyl, pos: [0.0, 0.3, 0.13], size: [0.14, 0.6, 0.14], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: Some("patte_g") },
    Piece { shape: Shape::Cube, pos: [0.08, 0.04, 0.13], size: [0.26, 0.08, 0.16], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("patte_g") },
    Piece { shape: Shape::Cyl, pos: [0.0, 0.3, -0.13], size: [0.14, 0.6, 0.14], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: Some("patte_d") },
    Piece { shape: Shape::Cube, pos: [0.08, 0.04, -0.13], size: [0.26, 0.08, 0.16], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("patte_d") },
];
const ANIMS_200: &[Anim] = &[
    Anim { target: "corps", kind: AnimKind::Bob, amp: 0.03 },
    Anim { target: "sabre", kind: AnimKind::TailZ, amp: 0.0 },
    Anim { target: "bras_g", kind: AnimKind::SwingZ, amp: 0.5 },
    Anim { target: "patte_g", kind: AnimKind::SwingZ, amp: 0.0 },
    Anim { target: "patte_d", kind: AnimKind::SwingZ, amp: 0.5 },
];
const PIECES_201: &[Piece] = &[
    Piece { shape: Shape::Cube, pos: [0.0, 1.188, 0.0], size: [0.504, 0.84, 0.672], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [0.04, 1.875, 0.0], size: [0.456, 0.456, 0.432], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [0.22, 1.925, 0.08], size: [0.084, 0.084, 0.084], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: None },
    Piece { shape: Shape::Sphere, pos: [0.22, 1.925, -0.08], size: [0.084, 0.084, 0.084], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: None },
    Piece { shape: Shape::Cube, pos: [0.2, 1.925, 0.0], size: [0.048, 0.072, 0.24], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("corps") },
    Piece { shape: Shape::Cube, pos: [0.2, 1.975, 0.07], size: [0.048, 0.144, 0.12], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("corps") },
    Piece { shape: Shape::Cyl, pos: [0.04, 2.15, 0.0], size: [0.744, 0.06, 0.744], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: Some("corps") },
    Piece { shape: Shape::Cone, pos: [0.04, 2.375, 0.0], size: [0.552, 0.36, 0.48], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: Some("corps") },
    Piece { shape: Shape::Cube, pos: [0.3, 2.225, 0.0], size: [0.048, 0.168, 0.168], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: Some("corps") },
    Piece { shape: Shape::Cube, pos: [0.04, 2.438, -0.3], size: [0.048, 0.36, 0.12], rot: [0.0, 0.0, 15.0], mat: Mat::LumiereRouge, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [0.25, 1.688, -0.38], size: [0.144, 0.24, 0.192], rot: [0.0, 0.0, 0.0], mat: Mat::LumiereRouge, id: Some("corps") },
    Piece { shape: Shape::Cyl, pos: [0.35, 1.5, -0.38], size: [0.048, 1.08, 0.048], rot: [0.0, 0.0, -40.0], mat: Mat::Blanc, id: Some("sabre") },
    Piece { shape: Shape::Cube, pos: [0.2, 1.125, -0.38], size: [0.036, 0.036, 0.24], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("sabre") },
    Piece { shape: Shape::Cyl, pos: [0.0, 1.25, 0.3], size: [0.12, 0.66, 0.12], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("bras_g") },
    Piece { shape: Shape::Cone, pos: [0.0, 0.875, 0.3], size: [0.096, 0.192, 0.072], rot: [180.0, 0.0, 0.0], mat: Mat::Sombre, id: Some("bras_g") },
    Piece { shape: Shape::Cyl, pos: [0.0, 0.375, 0.13], size: [0.168, 0.72, 0.168], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: Some("patte_g") },
    Piece { shape: Shape::Cube, pos: [0.08, 0.05, 0.13], size: [0.312, 0.096, 0.192], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("patte_g") },
    Piece { shape: Shape::Cyl, pos: [0.0, 0.375, -0.13], size: [0.168, 0.72, 0.168], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: Some("patte_d") },
    Piece { shape: Shape::Cube, pos: [0.08, 0.05, -0.13], size: [0.312, 0.096, 0.192], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("patte_d") },
    Piece { shape: Shape::Sphere, pos: [0.1, 1.95, 0.4], size: [0.22, 0.26, 0.2], rot: [0.0, 0.0, 0.0], mat: Mat::LumiereRouge, id: Some("corps") },
    Piece { shape: Shape::Cone, pos: [0.35, 1.9, 0.4], size: [0.06, 0.1, 0.05], rot: [0.0, 0.0, -90.0], mat: Mat::Phare, id: Some("corps") },
];
const ANIMS_201: &[Anim] = &[
    Anim { target: "corps", kind: AnimKind::Bob, amp: 0.03 },
    Anim { target: "sabre", kind: AnimKind::TailZ, amp: 0.0 },
    Anim { target: "bras_g", kind: AnimKind::SwingZ, amp: 0.5 },
    Anim { target: "patte_g", kind: AnimKind::SwingZ, amp: 0.0 },
    Anim { target: "patte_d", kind: AnimKind::SwingZ, amp: 0.5 },
];
const PIECES_202: &[Piece] = &[
    Piece { shape: Shape::Cyl, pos: [0.2, 0.7, 0.0], size: [0.4, 1.6, 0.4], rot: [0.0, 0.0, 90.0], mat: Mat::Sombre, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [-0.6, 0.7, 0.0], size: [0.45, 0.45, 0.45], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: Some("corps") },
    Piece { shape: Shape::Cyl, pos: [1.0, 0.7, 0.0], size: [0.5, 0.12, 0.5], rot: [0.0, 0.0, 90.0], mat: Mat::Sombre, id: Some("corps") },
    Piece { shape: Shape::Cube, pos: [0.0, 0.5, 0.0], size: [1.1, 0.2, 0.7], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [-0.78, 0.82, 0.0], size: [0.1, 0.1, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::LumiereRouge, id: Some("meche") },
    Piece { shape: Shape::Cyl, pos: [-0.3, 0.32, 0.4], size: [0.64, 0.1, 0.64], rot: [90.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.3, 0.32, 0.455], size: [0.3, 0.06, 0.3], rot: [90.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.3, 0.32, -0.4], size: [0.64, 0.1, 0.64], rot: [90.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.3, 0.32, -0.455], size: [0.3, 0.06, 0.3], rot: [90.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [0.5, 0.32, 0.4], size: [0.64, 0.1, 0.64], rot: [90.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cyl, pos: [0.5, 0.32, 0.455], size: [0.3, 0.06, 0.3], rot: [90.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [0.5, 0.32, -0.4], size: [0.64, 0.1, 0.64], rot: [90.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cyl, pos: [0.5, 0.32, -0.455], size: [0.3, 0.06, 0.3], rot: [90.0, 0.0, 0.0], mat: Mat::C3, id: None },
];
const ANIMS_202: &[Anim] = &[
    Anim { target: "corps", kind: AnimKind::Vibrate, amp: 0.012 },
    Anim { target: "meche", kind: AnimKind::Blink, amp: 0.0 },
];
const PIECES_203: &[Piece] = &[
    Piece { shape: Shape::Sphere, pos: [0.0, 1.4, 0.0], size: [1.5, 1.4, 1.3], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [0.1, 2.1, 0.0], size: [1.0, 0.9, 0.9], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [0.5, 1.9, 0.3], size: [0.3, 0.3, 0.3], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: None },
    Piece { shape: Shape::Sphere, pos: [0.5, 1.9, -0.3], size: [0.3, 0.3, 0.3], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: None },
    Piece { shape: Shape::Sphere, pos: [0.62, 1.9, 0.3], size: [0.14, 0.14, 0.14], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: None },
    Piece { shape: Shape::Sphere, pos: [0.62, 1.9, -0.3], size: [0.14, 0.14, 0.14], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: None },
    Piece { shape: Shape::Cube, pos: [0.65, 1.55, 0.0], size: [0.06, 0.1, 0.5], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [-0.2, 2.5, 0.0], size: [0.4, 0.3, 0.4], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("corps") },
    Piece { shape: Shape::Cyl, pos: [0.0, 0.7, 0.9], size: [0.26, 1.3, 0.26], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("tentacule0") },
    Piece { shape: Shape::Sphere, pos: [0.0, 0.1, 0.9], size: [0.34, 0.24, 0.34], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("tentacule0") },
    Piece { shape: Shape::Cyl, pos: [0.0, 0.7, 0.55], size: [0.26, 1.3, 0.26], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("tentacule1") },
    Piece { shape: Shape::Sphere, pos: [0.0, 0.1, 0.55], size: [0.34, 0.24, 0.34], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("tentacule1") },
    Piece { shape: Shape::Cyl, pos: [0.0, 0.7, 0.2], size: [0.26, 1.3, 0.26], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("tentacule2") },
    Piece { shape: Shape::Sphere, pos: [0.0, 0.1, 0.2], size: [0.34, 0.24, 0.34], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("tentacule2") },
    Piece { shape: Shape::Cyl, pos: [0.0, 0.7, -0.2], size: [0.26, 1.3, 0.26], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("tentacule3") },
    Piece { shape: Shape::Sphere, pos: [0.0, 0.1, -0.2], size: [0.34, 0.24, 0.34], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("tentacule3") },
    Piece { shape: Shape::Cyl, pos: [0.0, 0.7, -0.55], size: [0.26, 1.3, 0.26], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("tentacule4") },
    Piece { shape: Shape::Sphere, pos: [0.0, 0.1, -0.55], size: [0.34, 0.24, 0.34], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("tentacule4") },
    Piece { shape: Shape::Cyl, pos: [0.0, 0.7, -0.9], size: [0.26, 1.3, 0.26], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("tentacule5") },
    Piece { shape: Shape::Sphere, pos: [0.0, 0.1, -0.9], size: [0.34, 0.24, 0.34], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("tentacule5") },
];
const ANIMS_203: &[Anim] = &[
    Anim { target: "corps", kind: AnimKind::Bob, amp: 0.05 },
    Anim { target: "tentacule0", kind: AnimKind::SwingZ, amp: 0.0 },
    Anim { target: "tentacule1", kind: AnimKind::SwingZ, amp: 0.2 },
    Anim { target: "tentacule2", kind: AnimKind::SwingZ, amp: 0.4 },
    Anim { target: "tentacule3", kind: AnimKind::SwingZ, amp: 0.6000000000000001 },
    Anim { target: "tentacule4", kind: AnimKind::SwingZ, amp: 0.8 },
    Anim { target: "tentacule5", kind: AnimKind::SwingZ, amp: 1.0 },
];
const PIECES_204: &[Piece] = &[
    Piece { shape: Shape::Sphere, pos: [0.0, 0.95, 0.0], size: [0.6, 0.4, 0.34], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [0.35, 1.05, 0.0], size: [0.3, 0.3, 0.28], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [0.52, 1.02, 0.0], size: [0.2, 0.2, 0.2], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [0.48, 1.14, 0.1], size: [0.07, 0.07, 0.05], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [0.48, 1.14, -0.1], size: [0.07, 0.07, 0.05], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [0.65, 0.98, 0.0], size: [0.18, 0.2, 0.14], rot: [0.0, 0.0, 0.0], mat: Mat::Phare, id: Some("corps") },
    Piece { shape: Shape::Cone, pos: [0.7, 0.9, 0.0], size: [0.1, 0.14, 0.08], rot: [0.0, 0.0, 180.0], mat: Mat::Sombre, id: Some("corps") },
    Piece { shape: Shape::Cube, pos: [-0.55, 0.85, 0.0], size: [0.7, 0.04, 0.2], rot: [0.0, 0.0, 8.0], mat: Mat::C3, id: Some("corps") },
    Piece { shape: Shape::Cube, pos: [-0.6, 0.82, 0.1], size: [0.6, 0.04, 0.1], rot: [0.0, 0.0, 8.0], mat: Mat::LumiereBleue, id: Some("corps") },
    Piece { shape: Shape::Cube, pos: [-0.6, 0.82, -0.1], size: [0.6, 0.04, 0.1], rot: [0.0, 0.0, 8.0], mat: Mat::Phare, id: Some("corps") },
    Piece { shape: Shape::Cube, pos: [0.0, 0.97, 0.55], size: [0.5, 0.04, 0.8], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("aile_g") },
    Piece { shape: Shape::Cube, pos: [-0.1, 0.97, 1.0], size: [0.5, 0.04, 0.5], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("aile_g") },
    Piece { shape: Shape::Cube, pos: [-0.2, 0.98, 1.4], size: [0.4, 0.04, 0.4], rot: [0.0, 0.0, 0.0], mat: Mat::LumiereBleue, id: Some("aile_g") },
    Piece { shape: Shape::Cube, pos: [0.1, 0.97, 1.2], size: [0.1, 0.05, 0.9], rot: [0.0, 0.0, 0.0], mat: Mat::Phare, id: Some("aile_g") },
    Piece { shape: Shape::Cube, pos: [0.0, 0.97, -0.55], size: [0.5, 0.04, 0.8], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: Some("aile_d") },
    Piece { shape: Shape::Cube, pos: [-0.1, 0.97, -1.0], size: [0.5, 0.04, 0.5], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("aile_d") },
    Piece { shape: Shape::Cube, pos: [-0.2, 0.98, -1.4], size: [0.4, 0.04, 0.4], rot: [0.0, 0.0, 0.0], mat: Mat::LumiereBleue, id: Some("aile_d") },
    Piece { shape: Shape::Cube, pos: [0.1, 0.97, -1.2], size: [0.1, 0.05, 0.9], rot: [0.0, 0.0, 0.0], mat: Mat::Phare, id: Some("aile_d") },
];
const ANIMS_204: &[Anim] = &[
    Anim { target: "corps", kind: AnimKind::Bob, amp: 0.04 },
    Anim { target: "aile_g", kind: AnimKind::SwingXBase, amp: 0.0 },
    Anim { target: "aile_d", kind: AnimKind::SwingXBase, amp: 0.0 },
];
const PIRATES_ANIMALS: [Animal; 8] = [
    Animal { model: Model { c2: [232, 80, 58], c3: [192, 57, 43], pieces: PIECES_197, anims: ANIMS_197 }, len: 0.8, width: 0.8, points: 300, dangerous: false, flying: false },
    Animal { model: Model { c2: [138, 132, 120], c3: [216, 160, 168], pieces: PIECES_198, anims: ANIMS_198 }, len: 0.9, width: 0.4, points: 450, dangerous: false, flying: false },
    Animal { model: Model { c2: [232, 184, 138], c3: [192, 57, 43], pieces: PIECES_199, anims: ANIMS_199 }, len: 0.8, width: 0.8, points: 900, dangerous: true, flying: false },
    Animal { model: Model { c2: [241, 234, 216], c3: [106, 106, 122], pieces: PIECES_200, anims: ANIMS_200 }, len: 0.8, width: 0.8, points: 1300, dangerous: true, flying: false },
    Animal { model: Model { c2: [232, 184, 138], c3: [42, 74, 138], pieces: PIECES_201, anims: ANIMS_201 }, len: 1.0, width: 1.0, points: 1800, dangerous: true, flying: false },
    Animal { model: Model { c2: [138, 90, 43], c3: [201, 160, 99], pieces: PIECES_202, anims: ANIMS_202 }, len: 2.0, width: 1.0, points: 2300, dangerous: true, flying: false },
    Animal { model: Model { c2: [122, 74, 154], c3: [216, 144, 200], pieces: PIECES_203, anims: ANIMS_203 }, len: 1.8, width: 2.0, points: 3000, dangerous: true, flying: false },
    Animal { model: Model { c2: [216, 58, 58], c3: [255, 210, 63], pieces: PIECES_204, anims: ANIMS_204 }, len: 1.6, width: 3.2, points: 3400, dangerous: true, flying: true },
];
fn pirates_animal_name(i: usize) -> &'static str {
    match i {
        0 => tx!("Crabe Pas Content", "Cranky Crab", "Mürrische Krabbe", "Granchio Scontroso", "Cangrejo Gruñón").t(),
        1 => tx!("Rat de Cale", "Bilge Rat", "Schiffsratte", "Ratto di Stiva", "Rata de Sentina").t(),
        2 => tx!("Pirate Bagarreur", "Brawling Pirate", "Raufender Pirat", "Pirata Attaccabrighe", "Pirata Pendenciero").t(),
        3 => tx!("Pirate Squelette", "Skeleton Pirate", "Skelett-Pirat", "Pirata Scheletro", "Pirata Esqueleto").t(),
        4 => tx!("Capitaine Crochet-Crapaud", "Captain Hook-Toad", "Kapitän Hakenkröte", "Capitan Uncino-Rospo", "Capitán Garfio-Sapo").t(),
        5 => tx!("Canon Qui Roule", "Rolling Cannon", "Rollende Kanone", "Cannone Rotolante", "Cañón Rodante").t(),
        6 => tx!("Kraken des Plages", "Beach Kraken", "Strand-Kraken", "Kraken da Spiaggia", "Kraken de Playa").t(),
        7 => tx!("Perroquet Pirate", "Pirate Parrot", "Piraten-Papagei", "Pappagallo Pirata", "Loro Pirata").t(),
        _ => "?",
    }
}
const PIECES_205: &[Piece] = &[
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
const ANIMS_205: &[Anim] = &[
    Anim { target: "corps", kind: AnimKind::Bob, amp: 0.02 },
];
const PIECES_206: &[Piece] = &[
    Piece { shape: Shape::Cyl, pos: [0.12, 0.7, 0.0], size: [0.64, 0.04, 0.64], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [0.12, 0.8, 0.0], size: [0.46, 0.18, 0.5], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [0.38, 0.78, 0.0], size: [0.1, 0.2, 0.62], rot: [0.0, 0.0, -20.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [-0.14, 0.78, 0.0], size: [0.1, 0.2, 0.62], rot: [0.0, 0.0, 20.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Sphere, pos: [0.3, 0.8, 0.0], size: [0.12, 0.12, 0.04], rot: [0.0, 0.0, 0.0], mat: Mat::Blanc, id: None },
    Piece { shape: Shape::Cube, pos: [0.3, 0.74, 0.0], size: [0.05, 0.03, 0.04], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: None },
    Piece { shape: Shape::Cube, pos: [0.3, 0.57, 0.14], size: [0.05, 0.12, 0.14], rot: [0.0, 0.0, 0.0], mat: Mat::Noir, id: None },
    Piece { shape: Shape::Sphere, pos: [0.1, 0.34, 0.38], size: [0.07, 0.07, 0.07], rot: [0.0, 0.0, 0.0], mat: Mat::Phare, id: None },
];
const ANIMS_206: &[Anim] = &[
];
const PIECES_207: &[Piece] = &[
    Piece { shape: Shape::Cube, pos: [0.0, 0.45, 0.0], size: [1.9, 0.14, 0.8], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.65, 0.4], size: [1.9, 0.3, 0.04], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.65, -0.4], size: [1.9, 0.3, 0.04], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cyl, pos: [0.2, 0.85, 0.0], size: [0.36, 1.5, 0.36], rot: [0.0, 0.0, 80.0], mat: Mat::Sombre, id: Some("corps") },
    Piece { shape: Shape::Sphere, pos: [-0.5, 0.82, 0.0], size: [0.4, 0.4, 0.4], rot: [0.0, 0.0, 0.0], mat: Mat::Sombre, id: Some("corps") },
    Piece { shape: Shape::Cyl, pos: [0.95, 0.9, 0.0], size: [0.46, 0.1, 0.46], rot: [0.0, 0.0, 80.0], mat: Mat::Sombre, id: Some("corps") },
    Piece { shape: Shape::Cube, pos: [-0.1, 0.62, 0.0], size: [0.7, 0.2, 0.5], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Sphere, pos: [-0.68, 0.95, 0.0], size: [0.1, 0.1, 0.1], rot: [0.0, 0.0, 0.0], mat: Mat::LumiereRouge, id: Some("meche") },
    Piece { shape: Shape::Cyl, pos: [-0.6, 0.26, 0.42], size: [0.52, 0.07, 0.52], rot: [90.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.6, 0.26, 0.46], size: [0.286, 0.05, 0.286], rot: [90.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.6, 0.26, -0.42], size: [0.52, 0.07, 0.52], rot: [90.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.6, 0.26, -0.46], size: [0.286, 0.05, 0.286], rot: [90.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [0.6, 0.26, 0.42], size: [0.52, 0.07, 0.52], rot: [90.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cyl, pos: [0.6, 0.26, 0.46], size: [0.286, 0.05, 0.286], rot: [90.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [0.6, 0.26, -0.42], size: [0.52, 0.07, 0.52], rot: [90.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cyl, pos: [0.6, 0.26, -0.46], size: [0.286, 0.05, 0.286], rot: [90.0, 0.0, 0.0], mat: Mat::C3, id: None },
];
const ANIMS_207: &[Anim] = &[
    Anim { target: "corps", kind: AnimKind::Vibrate, amp: 0.012 },
    Anim { target: "meche", kind: AnimKind::Blink, amp: 0.0 },
];
const PIECES_208: &[Piece] = &[
    Piece { shape: Shape::Cube, pos: [0.0, 0.45, 0.0], size: [1.9, 0.14, 0.8], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.7, 0.4], size: [1.9, 0.4, 0.04], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.7, -0.4], size: [1.9, 0.4, 0.04], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [0.94, 0.7, 0.0], size: [0.04, 0.4, 0.8], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [-0.94, 0.7, 0.0], size: [0.04, 0.4, 0.8], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [-0.4, 0.95, 0.0], size: [0.7, 0.5, 0.55], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.4, 1.2, 0.0], size: [0.55, 0.7, 0.55], rot: [0.0, 0.0, 90.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [-0.4, 1.0, 0.0], size: [0.72, 0.1, 0.6], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [0.4, 0.95, 0.0], size: [0.7, 0.5, 0.55], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [0.4, 0.95, 0.0], size: [0.1, 0.5, 0.58], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.6, 0.26, 0.42], size: [0.52, 0.07, 0.52], rot: [90.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.6, 0.26, 0.46], size: [0.286, 0.05, 0.286], rot: [90.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.6, 0.26, -0.42], size: [0.52, 0.07, 0.52], rot: [90.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.6, 0.26, -0.46], size: [0.286, 0.05, 0.286], rot: [90.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [0.6, 0.26, 0.42], size: [0.52, 0.07, 0.52], rot: [90.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cyl, pos: [0.6, 0.26, 0.46], size: [0.286, 0.05, 0.286], rot: [90.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [0.6, 0.26, -0.42], size: [0.52, 0.07, 0.52], rot: [90.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cyl, pos: [0.6, 0.26, -0.46], size: [0.286, 0.05, 0.286], rot: [90.0, 0.0, 0.0], mat: Mat::C3, id: None },
];
const ANIMS_208: &[Anim] = &[
];
const PIECES_209: &[Piece] = &[
    Piece { shape: Shape::Cube, pos: [0.0, 0.45, 0.0], size: [1.9, 0.14, 0.8], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.7, 0.4], size: [1.9, 0.4, 0.04], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.7, -0.4], size: [1.9, 0.4, 0.04], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [0.94, 0.7, 0.0], size: [0.04, 0.4, 0.8], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [-0.94, 0.7, 0.0], size: [0.04, 0.4, 0.8], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Sphere, pos: [0.0, 0.92, 0.0], size: [1.7, 0.5, 0.72], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("or") },
    Piece { shape: Shape::Sphere, pos: [-0.3, 1.1, 0.1], size: [0.5, 0.3, 0.4], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("or") },
    Piece { shape: Shape::Sphere, pos: [0.4, 1.12, -0.1], size: [0.5, 0.3, 0.4], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: Some("or") },
    Piece { shape: Shape::Cyl, pos: [-0.6, 0.26, 0.42], size: [0.52, 0.07, 0.52], rot: [90.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.6, 0.26, 0.46], size: [0.286, 0.05, 0.286], rot: [90.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.6, 0.26, -0.42], size: [0.52, 0.07, 0.52], rot: [90.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.6, 0.26, -0.46], size: [0.286, 0.05, 0.286], rot: [90.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [0.6, 0.26, 0.42], size: [0.52, 0.07, 0.52], rot: [90.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cyl, pos: [0.6, 0.26, 0.46], size: [0.286, 0.05, 0.286], rot: [90.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [0.6, 0.26, -0.42], size: [0.52, 0.07, 0.52], rot: [90.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cyl, pos: [0.6, 0.26, -0.46], size: [0.286, 0.05, 0.286], rot: [90.0, 0.0, 0.0], mat: Mat::C3, id: None },
];
const ANIMS_209: &[Anim] = &[
    Anim { target: "or", kind: AnimKind::BlinkSlow, amp: 0.0 },
];
const PIECES_210: &[Piece] = &[
    Piece { shape: Shape::Cube, pos: [0.0, 0.45, 0.0], size: [1.9, 0.14, 0.8], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.7, 0.4], size: [1.9, 0.4, 0.04], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [0.0, 0.7, -0.4], size: [1.9, 0.4, 0.04], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [0.94, 0.7, 0.0], size: [0.04, 0.4, 0.8], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [-0.94, 0.7, 0.0], size: [0.04, 0.4, 0.8], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [-0.4, 0.95, 0.0], size: [0.7, 0.5, 0.55], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.4, 1.2, 0.0], size: [0.55, 0.7, 0.55], rot: [0.0, 0.0, 90.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [-0.4, 1.0, 0.0], size: [0.72, 0.1, 0.6], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cube, pos: [0.4, 0.95, 0.0], size: [0.7, 0.5, 0.55], rot: [0.0, 0.0, 0.0], mat: Mat::C2, id: None },
    Piece { shape: Shape::Cube, pos: [0.4, 0.95, 0.0], size: [0.1, 0.5, 0.58], rot: [0.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.6, 0.26, 0.42], size: [0.52, 0.07, 0.52], rot: [90.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.6, 0.26, 0.46], size: [0.286, 0.05, 0.286], rot: [90.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.6, 0.26, -0.42], size: [0.52, 0.07, 0.52], rot: [90.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cyl, pos: [-0.6, 0.26, -0.46], size: [0.286, 0.05, 0.286], rot: [90.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [0.6, 0.26, 0.42], size: [0.52, 0.07, 0.52], rot: [90.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cyl, pos: [0.6, 0.26, 0.46], size: [0.286, 0.05, 0.286], rot: [90.0, 0.0, 0.0], mat: Mat::C3, id: None },
    Piece { shape: Shape::Cyl, pos: [0.6, 0.26, -0.42], size: [0.52, 0.07, 0.52], rot: [90.0, 0.0, 0.0], mat: Mat::Sombre, id: None },
    Piece { shape: Shape::Cyl, pos: [0.6, 0.26, -0.46], size: [0.286, 0.05, 0.286], rot: [90.0, 0.0, 0.0], mat: Mat::C3, id: None },
];
const ANIMS_210: &[Anim] = &[
];
const PIRATES_TRAIN: [TrainCar; 4] = [
    TrainCar { model: Model { c2: [138, 90, 43], c3: [255, 210, 63], pieces: PIECES_207, anims: ANIMS_207 }, len: 2.5 },
    TrainCar { model: Model { c2: [138, 90, 43], c3: [255, 210, 63], pieces: PIECES_208, anims: ANIMS_208 }, len: 2.1 },
    TrainCar { model: Model { c2: [138, 90, 43], c3: [255, 210, 63], pieces: PIECES_209, anims: ANIMS_209 }, len: 2.1 },
    TrainCar { model: Model { c2: [138, 90, 43], c3: [255, 210, 63], pieces: PIECES_210, anims: ANIMS_210 }, len: 2.1 },
];
const PIRATES_COLORS: WorldColors = WorldColors {
    grass_a: [236, 220, 164],
    grass_b: [226, 208, 148],
    water: [42, 143, 208],
    road: [138, 90, 43],
    marking: [90, 58, 28],
    log: [138, 90, 43],
    sky: [143, 211, 240],
};
fn pirates_name() -> &'static str { tx!("Baie des Pirates", "Pirate Cove", "Piratenbucht", "Baia dei Pirati", "Bahía Pirata").t() }
fn pirates_banner() -> &'static str { tx!("Bienvenue à la Baie des Pirates ! Les trésors sont à gauche, les ennuis aussi.", "Welcome to Pirate Cove! Treasure's on the left, so is trouble.", "Willkommen in der Piratenbucht! Schätze links, Ärger auch.", "Benvenuto nella Baia dei Pirati! I tesori a sinistra, i guai anche.", "¡Bienvenido a la Bahía Pirata! Los tesoros a la izquierda, los problemas también.").t() }
fn pirates_hero_name() -> &'static str { tx!("Grenouille Pirate", "Pirate Frog", "Piraten-Frosch", "Rana Pirata", "Rana Pirata").t() }
pub static PIRATES_THEME: Theme = Theme {
    train: &PIRATES_TRAIN, only_own: true,
    blocks: &PIRATES_BLOCKS, sides: &PIRATES_SIDES, deco: &PIRATES_DECO, animals: &PIRATES_ANIMALS,
    block_mix: &[0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 4, 4], side_mix: &[100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 1, 1, 1, 2, 2, 2, 0],
    side_pct: 7, vehicles: &[], colors: PIRATES_COLORS, points: 5400,
    name: pirates_name, banner: pirates_banner, animal_name: pirates_animal_name, hero_name: pirates_hero_name,
    hero: Model { c2: [192, 57, 43], c3: [42, 42, 53], pieces: PIECES_205, anims: ANIMS_205 }, hat: Model { c2: [192, 57, 43], c3: [42, 42, 53], pieces: PIECES_206, anims: ANIMS_206 },
};
pub static THEMES: [&Theme; 8] = [&JURA_THEME, &NIPPON_THEME, &FARWEST_THEME, &ESPACE_THEME, &HALLOWEEN_THEME, &CYBER_THEME, &MOYENAGE_THEME, &PIRATES_THEME];
