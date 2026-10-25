#!/usr/bin/env python3
# Commit & Conquer — licence GPL-3.0-or-later.
#
# Les mondes à thème de Traversée (obstacles, bords, petits décors, animaux, héros
# et accessoire) décrits en pièces simples : lit les fichiers de
# games/traversee/data/import/ et écrit games/traversee/src/gen_worlds.rs.
# À lancer après gen-vehicules.py (les véhicules du monde sont retrouvés par leur id).
# Mêmes conventions que les véhicules ; couleur_2 / couleur_3 sont propres à chaque modèle.
import json, os

ROOT = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..")
T = os.path.join(ROOT, "games/traversee")
# tag, fichier du monde, animaux en plus, accessoire de départ en plus (pièces ajoutées au chapeau du héros),
# répartition des gros obstacles (indices de `obstacles`, 20 cases) et du décor des bords (0.. = bords, 100+i = obstacle i)
WORLDS = [
    dict(tag="JURA", file="data/import/jurassic_croa.json", extra_animals=[],
         side_pct=22,
         hat=[dict(forme="cylindre", position=[0.12, 0.76, 0], taille=[0.3, 0.16, 0.3], rotation_deg=[0, 0, 0], matiere="couleur_3"),
              dict(forme="cylindre", position=[0.12, 0.69, 0], taille=[0.56, 0.03, 0.56], rotation_deg=[0, 0, 0], matiere="couleur_3"),
              dict(forme="cylindre", position=[0.12, 0.725, 0], taille=[0.32, 0.03, 0.32], rotation_deg=[0, 0, 0], matiere="couleur_2")],
         blocks=[0] * 6 + [1] * 5 + [3] * 5 + [4, 5, 6, 2],
         sides=[0] * 11 + [100, 101, 103, 2, 3, 4, 5, 1, 0]),
    dict(tag="NIPPON", file="data/import/nippon_kaboom.json", keep_animals=["lapin_mochi", "tanuki_gourmand"],
         extra_animals=["data/import/rex_crete_bleue_detaille.json", "data/import/nippon_extra_animaux.json"], hat=[], side_pct=8,
         # Décor simple : cerisiers, bambous et lanternes ; un torii ou une enseigne de temps en temps.
         blocks=[0] * 6 + [3] * 5 + [1] * 3 + [5] * 3 + [6, 2, 4],
         sides=[100] * 8 + [103] * 5 + [101] * 3 + [2] * 2 + [5, 0]),
    dict(tag="FARWEST", file="data/import/farwest.json", extra_animals=[], hat=[], side_pct=8, only_own=True,
         blocks=[0] * 8 + [1] * 5 + [2] * 3 + [3] * 2 + [4] * 2,
         sides=[100] * 10 + [0] + [3] * 3 + [1, 2, 101]),
    dict(tag="ESPACE", file="data/import/espace.json", extra_animals=[], hat=[], side_pct=0, only_own=True,
         blocks=[0] * 8 + [1] * 6 + [2] * 6,
         sides=[100] * 10 + [101] * 5 + [102] * 5),
    dict(tag="HALLOWEEN", file="data/import/halloween.json", extra_animals=[], hat=[], side_pct=6, only_own=True,
         blocks=[0] * 7 + [1] * 6 + [2] * 4 + [3] * 3,
         sides=[1] * 8 + [0] * 2 + [102] * 6 + [100] * 4),
]
SHAPE_RS = {"cube": "Shape::Cube", "cylindre": "Shape::Cyl", "cone": "Shape::Cone", "sphere": "Shape::Sphere"}
MAT_RS = {"vitre": "Mat::Vitre", "sombre": "Mat::Sombre", "pneu": "Mat::Pneu", "enjoliveur": "Mat::Enjoliveur",
          "phare": "Mat::Phare", "feu_arriere": "Mat::FeuArriere", "lumiere_rouge": "Mat::LumiereRouge", "lumiere_bleue": "Mat::LumiereBleue",
          "blanc": "Mat::Blanc", "noir": "Mat::Noir", "couleur_2": "Mat::C2", "couleur_3": "Mat::C3", "bulle": "Mat::Bulle"}
ANIM_RS = {"clignote": ("Blink", 0.0), "clignote_alterne": ("BlinkAlt", 0.0), "pulse": ("Pulse", 0.0), "rebond": ("Bob", 0.03),
           "pendule": ("Pendulum", 0.0), "balance": ("Sway", 0.0), "derive_haut": ("DriftUp", 0.0), "flotte_verticale": ("Float", 0.03), "clignote_lent": ("BlinkSlow", 0.0), "rotation_groupe": ("SpinGroup", 0.0)}


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
    ax, pv = m.get("axe"), m.get("pivot")
    # Pattes, ailes et queues pivotent autour d'un point (voir AnimKind::Swing* / Tail*).
    if t == "pendule" and ax == "z":
        return ("SwingZBase" if pv == "base" else "SwingZ"), m.get("phase", 0.0)
    if t == "pendule" and ax == "x":
        return "SwingXBase", m.get("phase", 0.0)
    if t == "pendule" and ax == "y":
        return "SwingYBase", m.get("phase", 0.0)
    if t == "balance" and ax == "y":
        return "TailY", 0.0
    if t == "balance" and ax == "z":
        return "TailZ", 0.0
    if t == "balance" and ax == "x":
        return "TailX", 0.0
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
      "//! Mondes à thème décrits en pièces simples (cube, cylindre, cône, sphère).",
      "#![allow(dead_code)]",
      "use game_common::tx;",
      "",
      "use crate::extra::{Anim, AnimKind, Animal, Mat, Model, Piece, Shape, Theme, TrainCar, WorldColors};",
      ""]
n = 0


def model(m, hero=False, extra_pieces=()):
    global n
    i = n
    n += 1
    # `peinture` : la couleur principale (couleur_2) pour les décors et animaux, la peau verte pour les héros.
    mats = dict(MAT_RS, peinture="Mat::Peinture" if hero else "Mat::C2")
    rs.append(f"const PIECES_{i}: &[Piece] = &[")
    for p in list(m["pieces"]) + list(extra_pieces):
        pid = f'Some("{p["nom"]}")' if "nom" in p else "None"
        rs.append(f"    Piece {{ shape: {SHAPE_RS[p['forme']]}, pos: [{', '.join(f(x) for x in p['position'])}], size: [{', '.join(f(x) for x in p['taille'])}], rot: [{', '.join(f(x) for x in p['rotation_deg'])}], mat: {mats[p['matiere']]}, id: {pid} }},")
    rs.append("];")
    rs.append(f"const ANIMS_{i}: &[Anim] = &[")
    for a in m.get("parties_mobiles", []):
        kind, amp = anim(a)
        rs.append(f'    Anim {{ target: "{a["piece"]}", kind: AnimKind::{kind}, amp: {f(amp)} }},')
    rs.append("];")
    return f'Model {{ c2: {rgb(m.get("couleur_2", "#888888"))}, c3: {rgb(m.get("couleur_3", "#888888"))}, pieces: PIECES_{i}, anims: ANIMS_{i} }}'


vids = [v["id"] for v in json.load(open(os.path.join(T, "data/vehicules.json"), encoding="utf-8"))["vehicules"]]
for W in WORLDS:
    tag = W["tag"]
    d = json.load(open(os.path.join(T, W["file"]), encoding="utf-8"))
    w = d["monde"]
    animals = [a for a in d["animaux"] if "keep_animals" not in W or a["id"] in W["keep_animals"]]
    for extra in W["extra_animals"]:
        animals += json.load(open(os.path.join(T, extra), encoding="utf-8"))["animaux"]
    for key, label in (("obstacles", "BLOCKS"), ("bords", "SIDES"), ("deco", "DECO")):
        items = [model(m) for m in w[key]]
        rs.append(f"const {tag}_{label}: [Model; {len(items)}] = [")
        rs += [f"    {x}," for x in items]
        rs.append("];")
    items = [model(a) for a in animals]
    rs.append(f"const {tag}_ANIMALS: [Animal; {len(items)}] = [")
    for a, m in zip(animals, items):
        rs.append(f"    Animal {{ model: {m}, len: {f(a['longueur'])}, width: {f(a['largeur'])}, points: {a['points_deblocage']}, dangerous: {str(a['dangereux']).lower()}, flying: {str(a['classe'] == 'volant').lower()} }},")
    rs.append("];")
    rs.append(f"fn {tag.lower()}_animal_name(i: usize) -> &'static str {{")
    rs.append("    match i {")
    for i, a in enumerate(animals):
        rs.append(f"        {i} => {tx(a['noms'])}.t(),")
    rs.append('        _ => "?",')
    rs.append("    }")
    rs.append("}")
    h = d["heros"]
    hero = model(h, hero=True)
    acc = model(dict(h["accessoire"], couleur_2=h["couleur_2"], couleur_3=h["couleur_3"]), hero=True, extra_pieces=W["hat"])
    cars = [(model(t), t["longueur"]) for t in w.get("train", [])]
    rs.append(f"const {tag}_TRAIN: [TrainCar; {len(cars)}] = [")
    rs += [f"    TrainCar {{ model: {m}, len: {f(l)} }}," for m, l in cars]
    rs.append("];")
    rs.append(f"const {tag}_COLORS: WorldColors = WorldColors {{")
    for k, field in (("herbe_a", "grass_a"), ("herbe_b", "grass_b"), ("eau", "water"), ("sol_voie_route", "road"), ("marquage", "marking"), ("rondin", "log"), ("ciel", "sky")):
        rs.append(f"    {field}: {rgb(w[k])},")
    rs.append("};")
    rs.append(f"fn {tag.lower()}_name() -> &'static str {{ {tx(w['nom'])}.t() }}")
    rs.append(f"fn {tag.lower()}_banner() -> &'static str {{ {tx(w['bannière'])}.t() }}")
    rs.append(f"fn {tag.lower()}_hero_name() -> &'static str {{ {tx(h['noms'])}.t() }}")
    idx = [vids.index(v) for v in w["vehicules_du_monde"]]
    rs.append(f"pub static {tag}_THEME: Theme = Theme {{")
    rs.append(f"    train: &{tag}_TRAIN, only_own: {str(W.get('only_own', False)).lower()},")
    rs.append(f"    blocks: &{tag}_BLOCKS, sides: &{tag}_SIDES, deco: &{tag}_DECO, animals: &{tag}_ANIMALS,")
    rs.append(f"    block_mix: &{W['blocks']}, side_mix: &{W['sides']},")
    rs.append(f"    side_pct: {W['side_pct']}, vehicles: &{idx}, colors: {tag}_COLORS, points: {w['points_deblocage']},")
    rs.append(f"    name: {tag.lower()}_name, banner: {tag.lower()}_banner, animal_name: {tag.lower()}_animal_name, hero_name: {tag.lower()}_hero_name,")
    rs.append(f"    hero: {hero}, hat: {acc},")
    rs.append("};")
rs.append(f"pub static THEMES: [&Theme; {len(WORLDS)}] = [{', '.join('&' + W['tag'] + '_THEME' for W in WORLDS)}];")
open(os.path.join(T, "src/gen_worlds.rs"), "w", encoding="utf-8").write("\n".join(rs) + "\n")
print(n, "modèles")
