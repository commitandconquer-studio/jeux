#!/usr/bin/env python3
# Commit & Conquer — licence GPL-3.0-or-later.
#
# Les décors d'un monde de Traversée (obstacles, bords, petits décors) décrits en
# pièces simples : lit games/traversee/data/import/<monde>.json et écrit
#   games/traversee/src/gen_worlds.rs
# Mêmes conventions que les véhicules (voir gen-vehicules.py) ; les matières
# couleur_2 / couleur_3 sont propres à chaque modèle.
import json, os

ROOT = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..")
WORLDS = [("JURA", "data/import/jurassic_croa.json")]
SHAPE_RS = {"cube": "Shape::Cube", "cylindre": "Shape::Cyl", "cone": "Shape::Cone", "sphere": "Shape::Sphere"}
MAT_RS = {"peinture": "Mat::C2", "vitre": "Mat::Vitre", "sombre": "Mat::Sombre", "pneu": "Mat::Pneu", "enjoliveur": "Mat::Enjoliveur",
          "phare": "Mat::Phare", "feu_arriere": "Mat::FeuArriere", "lumiere_rouge": "Mat::LumiereRouge", "lumiere_bleue": "Mat::LumiereBleue",
          "blanc": "Mat::Blanc", "noir": "Mat::Noir", "couleur_2": "Mat::C2", "couleur_3": "Mat::C3"}
ANIM_RS = {"clignote": ("Blink", 0.0), "clignote_alterne": ("BlinkAlt", 0.0), "pulse": ("Pulse", 0.0), "rebond": ("Bob", 0.03),
           "pendule": ("Pendulum", 0.0), "balance": ("Sway", 0.0), "derive_haut": ("DriftUp", 0.0), "flotte_verticale": ("Float", 0.03)}


def f(x):
    return repr(float(x))


def rgb(h):
    h = h.lstrip("#")
    return "[" + ", ".join(str(int(h[i:i + 2], 16)) for i in (0, 2, 4)) + "]"


def q(s):
    return '"' + s.replace('"', '\\"') + '"'


def tx(n):
    return f"tx!({q(n['fr'])}, {q(n['en'])}, {q(n['de'])}, {q(n['it'])}, {q(n['es'])})"


def anim(m):
    t = m.get("animation", m.get("type"))
    amp = None
    parts = t.rsplit("_", 1)
    if len(parts) == 2:
        try:
            amp = float(parts[1]); t = parts[0]
        except ValueError:
            pass
    kind, default = ANIM_RS[t]
    return kind, amp if amp is not None else m.get("amplitude", default)


rs = ["// Généré par tools/gen-mondes.py à partir de data/import/*.json : ne pas modifier à la main.",
      "//! Décors des mondes décrits en pièces simples (cube, cylindre, cône, sphère).",
      "#![allow(dead_code)]",
      "use game_common::tx;",
      "",
      "use crate::extra::{Anim, AnimKind, Mat, Model, Piece, Shape};",
      ""]
n = 0


def model(m):
    global n
    i = n
    n += 1
    rs.append(f"const PIECES_{i}: &[Piece] = &[")
    for p in m["pieces"]:
        pid = f'Some("{p["nom"]}")' if "nom" in p else "None"
        rs.append(f"    Piece {{ shape: {SHAPE_RS[p['forme']]}, pos: [{', '.join(f(x) for x in p['position'])}], size: [{', '.join(f(x) for x in p['taille'])}], rot: [{', '.join(f(x) for x in p['rotation_deg'])}], mat: {MAT_RS[p['matiere']]}, id: {pid} }},")
    rs.append("];")
    rs.append(f"const ANIMS_{i}: &[Anim] = &[")
    for a in m.get("parties_mobiles", []):
        kind, amp = anim(a)
        rs.append(f'    Anim {{ target: "{a["piece"]}", kind: AnimKind::{kind}, amp: {f(amp)} }},')
    rs.append("];")
    return f'Model {{ c2: {rgb(m.get("couleur_2", "#888888"))}, c3: {rgb(m.get("couleur_3", "#888888"))}, pieces: PIECES_{i}, anims: ANIMS_{i} }}'


for tag, fichier in WORLDS:
    d = json.load(open(os.path.join(ROOT, "games/traversee", fichier), encoding="utf-8"))
    w = d["monde"]
    for key, label in (("obstacles", "BLOCKS"), ("bords", "SIDES"), ("deco", "DECO")):
        items = [model(m) for m in w[key]]
        rs.append(f"pub const {tag}_{label}: [Model; {len(items)}] = [")
        rs += [f"    {x}," for x in items]
        rs.append("];")
    rs.append(f"pub const {tag}_COLORS: WorldColors = WorldColors {{")
    for k, field in (("herbe_a", "grass_a"), ("herbe_b", "grass_b"), ("eau", "water"), ("sol_voie_route", "road"), ("marquage", "marking"), ("rondin", "log"), ("ciel", "sky")):
        rs.append(f"    {field}: {rgb(w[k])},")
    rs.append("};")
    rs.append(f"pub fn {tag.lower()}_name() -> &'static str {{ {tx(w['nom'])}.t() }}")
    rs.append(f"pub fn {tag.lower()}_banner() -> &'static str {{ {tx(w['bannière'])}.t() }}")
    rs.append(f"pub const {tag}_POINTS: u32 = {w['points_deblocage']};")
    ids = [v["id"] for v in json.load(open(os.path.join(ROOT, "games/traversee/data/vehicules.json"), encoding="utf-8"))["vehicules"]]
    idx = [ids.index(v) for v in w["vehicules_du_monde"]]
    rs.append(f"/// Numéros (dans `gen_vehicles::GEN`) des véhicules qui roulent dans ce monde.")
    rs.append(f"pub const {tag}_VEHICLES: [usize; {len(idx)}] = [{', '.join(str(i) for i in idx)}];")
rs.append("""
pub struct WorldColors {
    pub grass_a: [u8; 3],
    pub grass_b: [u8; 3],
    pub water: [u8; 3],
    pub road: [u8; 3],
    pub marking: [u8; 3],
    pub log: [u8; 3],
    pub sky: [u8; 3],
}
""")
open(os.path.join(ROOT, "games/traversee/src/gen_worlds.rs"), "w", encoding="utf-8").write("\n".join(rs) + "\n")
print(n, "modèles de décor")
