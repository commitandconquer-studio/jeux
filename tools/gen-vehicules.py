#!/usr/bin/env python3
# Commit & Conquer — licence GPL-3.0-or-later.
#
# Les véhicules de Traversée décrits en pièces simples (cube, cylindre, cône,
# sphère) : ce script écrit
#   games/traversee/data/vehicules.json    (la description, lisible et rééditable)
#   games/traversee/src/gen_vehicles.rs    (les tableaux Rust compilés dans le jeu)
# Repère : avant = +X, haut = +Y, largeur = Z, roues sur y = 0.
import json, os

ROOT = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..")
SHAPES = {"c": "cube", "y": "cylindre", "n": "cone", "s": "sphere"}


def P(shape, pos, size, mat, rot=(0, 0, 0), id=None):
    d = {"forme": SHAPES[shape], "position": list(pos), "taille": list(size), "rotation_deg": list(rot), "matiere": mat}
    if id:
        d["id"] = id
    return d


def wheels(xs, y, d, zt, h=0.11):
    """Pneu + enjoliveur de chaque côté, couchés sur l'axe Z."""
    out = []
    for x in xs:
        for z in (zt, -zt):
            out.append(P("y", (x, y, z), (d, h, d), "pneu", (90, 0, 0)))
            out.append(P("y", (x, y, z + (0.005 if z > 0 else -0.005)), (round(d * 0.55, 3), 0.13, round(d * 0.55, 3)), "enjoliveur", (90, 0, 0)))
    return out


def lights(xf, xr, y, z, size=(0.04, 0.08, 0.14), yr=None):
    return [
        P("c", (xf, y, -z), size, "phare"), P("c", (-xr, yr if yr else y, -z), size, "feu_arriere"),
        P("c", (xf, y, z), size, "phare"), P("c", (-xr, yr if yr else y, z), size, "feu_arriere"),
    ]


def car_base(paint, hub_mat=None):
    return []


V = []

# --- 1. Taxi Multipass
p = [
    P("c", (0, 0.17, 0), (1.48, 0.1, 0.66), "sombre"),
    P("c", (0, 0.32, 0), (1.6, 0.2, 0.72), "peinture"),
    P("c", (-0.08, 0.55, 0), (0.7, 0.26, 0.66), "peinture"),
    P("c", (-0.08, 0.7, 0), (0.72, 0.04, 0.68), "peinture"),
    P("c", (-0.08, 0.56, -0.341), (0.56, 0.156, 0.02), "vitre"),
    P("c", (-0.08, 0.56, 0.341), (0.56, 0.156, 0.02), "vitre"),
    P("c", (-0.435, 0.56, 0), (0.02, 0.156, 0.52), "vitre"),
    P("c", (0.275, 0.56, 0), (0.02, 0.156, 0.52), "vitre"),
]
p += lights(0.8, 0.8, 0.29, 0.25)
p += [P("c", (0.81, 0.2, 0), (0.06, 0.07, 0.76), "sombre"), P("c", (-0.81, 0.2, 0), (0.06, 0.07, 0.76), "sombre")]
p += wheels([0.44, -0.44], 0.14, 0.28, 0.375)
p += [P("c", (-0.08, 0.77, 0), (0.3, 0.1, 0.14), "blanc", id="enseigne"), P("c", (-0.08, 0.84, 0), (0.2, 0.04, 0.1), "noir")]
for x, m in [(-0.6, "noir"), (-0.36, "blanc"), (-0.12, "noir"), (0.12, "blanc"), (0.36, "noir"), (0.6, "blanc")]:
    for z in (0.365, -0.365):
        p.append(P("c", (x, 0.34, z), (0.12, 0.07, 0.01), m))
p += [
    P("c", (0.3, 0.1, 0), (0.5, 0.03, 0.5), "lumiere_bleue", id="halo_av"),
    P("c", (-0.3, 0.1, 0), (0.5, 0.03, 0.5), "lumiere_bleue", id="halo_ar"),
    P("c", (0.35, 0.52, 0), (0.5, 0.02, 0.6), "noir"),
]
V.append(dict(id="taxi_multipass", classe="voiture", longueur=1.6, largeur=0.85,
    noms=dict(fr="Taxi Multipass", en="Multipass Cab", de="Multipass-Taxi", es="Taxi Multipás", it="Taxi Multipass"),
    points_deblocage=25,
    parties_mobiles=[dict(piece="enseigne", type="clignote"), dict(piece="halo_av", type="pulse"), dict(piece="halo_ar", type="pulse"), dict(piece="*", type="flotte_verticale_0.03")],
    pieces=p))

# --- 2. DeLoréenne 88
p = [
    P("c", (0, 0.17, 0), (1.48, 0.1, 0.66), "sombre"),
    P("c", (0, 0.3, 0), (1.6, 0.16, 0.72), "enjoliveur"),
    P("c", (-0.05, 0.49, 0), (0.6, 0.22, 0.66), "enjoliveur"),
    P("c", (-0.05, 0.62, 0), (0.62, 0.04, 0.68), "enjoliveur"),
    P("c", (-0.05, 0.5, -0.341), (0.46, 0.132, 0.02), "vitre"),
    P("c", (-0.05, 0.5, 0.341), (0.46, 0.132, 0.02), "vitre"),
    P("c", (-0.355, 0.5, 0), (0.02, 0.132, 0.52), "vitre"),
    P("c", (0.255, 0.5, 0), (0.02, 0.132, 0.52), "vitre"),
]
p += lights(0.8, 0.8, 0.29, 0.25)
p += [P("c", (0.81, 0.2, 0), (0.06, 0.07, 0.76), "sombre"), P("c", (-0.81, 0.2, 0), (0.06, 0.07, 0.76), "sombre")]
p += wheels([0.44, -0.44], 0.14, 0.28, 0.375)
p += [
    P("c", (0, 0.52, -0.28), (0.5, 0.025, 0.25), "enjoliveur", (-35, 0, 0)),
    P("c", (0.15, 0.34, -0.365), (0.9, 0.02, 0.01), "lumiere_bleue", id="filet_d"),
    P("c", (0, 0.52, 0.28), (0.5, 0.025, 0.25), "enjoliveur", (35, 0, 0)),
    P("c", (0.15, 0.34, 0.365), (0.9, 0.02, 0.01), "lumiere_bleue", id="filet_g"),
    P("c", (0.55, 0.4, 0), (0.2, 0.04, 0.3), "noir"),
    P("c", (0.7, 0.4, 0), (0.08, 0.03, 0.08), "lumiere_rouge", id="compteur"),
    P("n", (-1.0, 0.3, 0), (0.3, 0.4, 0.3), "lumiere_rouge", (0, 0, 90), id="flamme"),
    P("y", (-0.55, 0.45, 0), (0.12, 0.12, 0.12), "lumiere_bleue"),
    P("c", (-0.85, 0.3, -0.2), (0.15, 0.05, 0.1), "sombre"),
    P("c", (-0.85, 0.3, 0.2), (0.15, 0.05, 0.1), "sombre"),
    P("c", (0.0, 0.42, 0.0), (0.06, 0.03, 0.64), "noir"),
]
V.append(dict(id="deloreenne_88", classe="voiture", longueur=1.6, largeur=0.85,
    noms=dict(fr="DeLoréenne 88", en="DeLorrain 88", de="DeLorean-88", es="La DeLoreana 88", it="DeLoreana 88"),
    points_deblocage=50,
    parties_mobiles=[dict(piece="flamme", type="clignote"), dict(piece="filet_g", type="pulse"), dict(piece="filet_d", type="pulse"), dict(piece="compteur", type="clignote")],
    pieces=p))

# --- 3. K.I.T.CROA
p = [
    P("c", (0, 0.17, 0), (1.48, 0.1, 0.66), "sombre"),
    P("c", (0, 0.32, 0), (1.6, 0.2, 0.72), "noir"),
    P("c", (-0.1, 0.52, 0), (0.7, 0.2, 0.66), "noir"),
    P("c", (-0.1, 0.64, 0), (0.72, 0.04, 0.68), "noir"),
    P("c", (-0.1, 0.53, -0.341), (0.56, 0.12, 0.02), "vitre"),
    P("c", (-0.1, 0.53, 0.341), (0.56, 0.12, 0.02), "vitre"),
    P("c", (-0.455, 0.53, 0), (0.02, 0.12, 0.52), "vitre"),
    P("c", (0.255, 0.53, 0), (0.02, 0.12, 0.52), "vitre"),
]
p += lights(0.8, 0.8, 0.29, 0.25)
p += [P("c", (0.81, 0.2, 0), (0.06, 0.07, 0.76), "sombre"), P("c", (-0.81, 0.2, 0), (0.06, 0.07, 0.76), "sombre")]
p += wheels([0.44, -0.44], 0.14, 0.28, 0.375)
p += [
    P("c", (0.76, 0.37, 0), (0.04, 0.05, 0.6), "sombre"),
    P("c", (0.78, 0.37, 0), (0.04, 0.04, 0.12), "lumiere_rouge", id="scanner"),
    P("c", (0.3, 0.43, 0), (0.45, 0.02, 0.6), "sombre"),
    P("n", (0.5, 0.4, 0), (0.15, 0.12, 0.15), "noir", (0, 0, -90)),
    P("c", (-0.7, 0.5, 0), (0.3, 0.04, 0.6), "sombre"),
    P("s", (0.2, 0.6, 0.1), (0.06, 0.06, 0.06), "lumiere_rouge", id="oeil_tableau"),
    P("c", (0.3, 0.3, -0.362), (0.4, 0.05, 0.01), "lumiere_rouge"),
    P("c", (0.3, 0.3, 0.362), (0.4, 0.05, 0.01), "lumiere_rouge"),
]
V.append(dict(id="kit_croa", classe="voiture", longueur=1.6, largeur=0.85,
    noms=dict(fr="K.I.T.CROA", en="K.I.T.T.-Ribbit", de="K.I.T.T.-Quak", es="K.I.T.R.A.", it="K.I.T.-Gra"),
    points_deblocage=100,
    parties_mobiles=[dict(piece="scanner", type="translation_z_ping_pong_0.22"), dict(piece="oeil_tableau", type="clignote")],
    pieces=p))

# --- 4. Intercepteur Fou
p = [
    P("c", (0, 0.17, 0), (1.48, 0.1, 0.66), "sombre"),
    P("c", (0, 0.32, 0), (1.6, 0.2, 0.72), "noir"),
    P("c", (-0.2, 0.53, 0), (0.55, 0.22, 0.66), "noir"),
    P("c", (-0.2, 0.66, 0), (0.57, 0.04, 0.68), "noir"),
    P("c", (-0.2, 0.54, -0.341), (0.41, 0.132, 0.02), "vitre"),
    P("c", (-0.2, 0.54, 0.341), (0.41, 0.132, 0.02), "vitre"),
    P("c", (-0.48, 0.54, 0), (0.02, 0.132, 0.52), "vitre"),
    P("c", (0.08, 0.54, 0), (0.02, 0.132, 0.52), "vitre"),
]
p += lights(0.8, 0.8, 0.29, 0.25)
p += [P("c", (0.81, 0.2, 0), (0.06, 0.07, 0.76), "sombre"), P("c", (-0.81, 0.2, 0), (0.06, 0.07, 0.76), "sombre")]
p += wheels([0.44, -0.44], 0.16, 0.32, 0.375)
p += [
    P("c", (0.35, 0.52, 0), (0.4, 0.14, 0.2), "sombre", id="compresseur"),
    P("y", (0.35, 0.64, 0), (0.12, 0.1, 0.12), "enjoliveur"),
    P("y", (-0.1, 0.28, -0.4), (0.07, 0.7, 0.07), "enjoliveur", (0, 0, 90)),
    P("c", (0.0, 0.43, -0.362), (0.4, 0.1, 0.01), "sombre"),
    P("y", (-0.1, 0.28, 0.4), (0.07, 0.7, 0.07), "enjoliveur", (0, 0, 90)),
    P("c", (0.0, 0.43, 0.362), (0.4, 0.1, 0.01), "sombre"),
    P("c", (-0.82, 0.5, 0.0), (0.12, 0.2, 0.3), "sombre"),
    P("c", (-0.5, 0.62, 0.0), (0.14, 0.12, 0.28), "peinture"),
    P("c", (0.8, 0.38, 0), (0.04, 0.06, 0.3), "blanc"),
    P("c", (-0.82, 0.25, 0), (0.04, 0.1, 0.2), "blanc"),
    P("c", (-0.1, 0.64, 0.0), (0.2, 0.02, 0.4), "sombre"),
    P("s", (-0.95, 0.4, 0.15), (0.1, 0.1, 0.1), "sombre", id="fumee"),
]
V.append(dict(id="intercepteur_fou", classe="voiture", longueur=1.6, largeur=0.85,
    noms=dict(fr="Intercepteur Fou", en="Mad Interceptor", de="Irrer Abfangjäger", es="Interceptor Loco", it="Intercettore Pazzo"),
    points_deblocage=150,
    parties_mobiles=[dict(piece="compresseur", type="vibre"), dict(piece="fumee", type="derive_arriere")],
    pieces=p))

# --- 5. La Chauve-Mobile
p = [
    P("c", (0, 0.17, 0), (1.48, 0.1, 0.66), "sombre"),
    P("c", (0, 0.29, 0), (1.6, 0.14, 0.72), "noir"),
    P("c", (0.0, 0.43, 0), (0.55, 0.14, 0.66), "noir"),
    P("c", (0.0, 0.52, 0), (0.57, 0.04, 0.68), "noir"),
    P("c", (0.0, 0.44, -0.341), (0.41, 0.084, 0.02), "vitre"),
    P("c", (0.0, 0.44, 0.341), (0.41, 0.084, 0.02), "vitre"),
    P("c", (-0.28, 0.44, 0), (0.02, 0.084, 0.52), "vitre"),
    P("c", (0.28, 0.44, 0), (0.02, 0.084, 0.52), "vitre"),
]
p += lights(0.8, 0.8, 0.29, 0.25)
p += [P("c", (0.81, 0.2, 0), (0.06, 0.07, 0.76), "sombre"), P("c", (-0.81, 0.2, 0), (0.06, 0.07, 0.76), "sombre")]
p += wheels([0.44, -0.44], 0.14, 0.28, 0.375)
p += [
    P("s", (0.05, 0.52, 0), (0.6, 0.22, 0.5), "vitre"),
    P("c", (-0.65, 0.5, -0.3), (0.4, 0.3, 0.03), "noir", (-10, 0, 0)),
    P("n", (-0.62, 0.75, -0.3), (0.3, 0.4, 0.03), "noir"),
    P("n", (0.05, 0.7, -0.15), (0.1, 0.18, 0.1), "noir", id="oreille_d"),
    P("c", (-0.65, 0.5, 0.3), (0.4, 0.3, 0.03), "noir", (10, 0, 0)),
    P("n", (-0.62, 0.75, 0.3), (0.3, 0.4, 0.03), "noir"),
    P("n", (0.05, 0.7, 0.15), (0.1, 0.18, 0.1), "noir", id="oreille_g"),
    P("n", (0.88, 0.34, 0), (0.2, 0.5, 0.34), "noir", (0, 0, -90)),
    P("n", (-1.0, 0.35, 0), (0.22, 0.4, 0.22), "lumiere_bleue", (0, 0, 90), id="flamme"),
    P("c", (0.3, 0.38, 0), (0.3, 0.02, 0.2), "sombre"),
]
V.append(dict(id="chauve_mobile", classe="voiture", longueur=1.6, largeur=0.85,
    noms=dict(fr="La Chauve-Mobile", en="The Bat-Mobile-ish", de="Die Fledermaus-Karre", es="El Murciélago-Móvil", it="La Pipistrello-Mobile"),
    points_deblocage=250,
    parties_mobiles=[dict(piece="flamme", type="clignote"), dict(piece="oreille_g", type="oscille"), dict(piece="oreille_d", type="oscille")],
    pieces=p))

# --- 6. Cycle Néon
p = [
    P("y", (0.38, 0.2, 0), (0.4, 0.1, 0.4), "pneu", (90, 0, 0)),
    P("y", (0.38, 0.2, 0), (0.24, 0.12, 0.24), "lumiere_bleue", (90, 0, 0), id="neon_roue"),
    P("y", (-0.38, 0.2, 0), (0.4, 0.1, 0.4), "pneu", (90, 0, 0)),
    P("y", (-0.38, 0.2, 0), (0.24, 0.12, 0.24), "lumiere_bleue", (90, 0, 0), id="neon_roue"),
    P("c", (0, 0.3, 0), (0.9, 0.18, 0.18), "noir"),
    P("c", (0.1, 0.42, 0), (0.5, 0.12, 0.2), "noir"),
    P("c", (0, 0.34, 0.095), (0.8, 0.03, 0.01), "lumiere_bleue", id="neon_g"),
    P("c", (0, 0.34, -0.095), (0.8, 0.03, 0.01), "lumiere_bleue", id="neon_d"),
    P("n", (0.52, 0.34, 0), (0.18, 0.2, 0.18), "noir", (0, 0, -90)),
    P("c", (0.42, 0.52, 0), (0.1, 0.1, 0.2), "vitre", (0, 0, -30)),
    P("c", (-0.35, 0.45, 0), (0.3, 0.1, 0.2), "noir"),
    P("c", (-0.5, 0.4, 0), (0.04, 0.06, 0.12), "feu_arriere"),
    P("s", (0.0, 0.52, 0), (0.2, 0.2, 0.2), "noir"),
    P("c", (-0.1, 0.6, 0), (0.35, 0.14, 0.22), "noir", (0, 0, -70)),
    P("s", (0.1, 0.75, 0), (0.18, 0.18, 0.18), "noir"),
    P("c", (0.1, 0.76, 0.0), (0.14, 0.04, 0.2), "lumiere_bleue", id="neon_casque"),
    P("y", (0.5, 0.3, -0.06), (0.03, 0.4, 0.03), "sombre", (0, 0, -15)),
    P("y", (0.5, 0.3, 0.06), (0.03, 0.4, 0.03), "sombre", (0, 0, -15)),
    P("c", (0.45, 0.58, 0), (0.04, 0.04, 0.36), "noir"),
    P("c", (-0.45, 0.4, 0), (0.3, 0.04, 0.3), "noir"),
    P("c", (0.55, 0.27, 0), (0.06, 0.06, 0.18), "phare"),
    P("c", (-0.2, 0.2, 0), (0.3, 0.06, 0.1), "lumiere_bleue", id="neon_ventre"),
    P("c", (-1.0, 0.2, 0), (1.0, 0.35, 0.03), "lumiere_bleue", id="traine"),
]
V.append(dict(id="cycle_neon", classe="moto", longueur=1.1, largeur=0.4,
    noms=dict(fr="Cycle Néon", en="Neon Cycle", de="Neon-Zyklus", es="Ciclo de Neón", it="Ciclo al Neon"),
    points_deblocage=25,
    parties_mobiles=[dict(piece="traine", type="longueur_pulse"), dict(piece="*neon", type="pulse")],
    pieces=p))

# --- 7. Easy Ribbit
p = [
    P("y", (0.55, 0.2, 0), (0.4, 0.08, 0.4), "pneu", (90, 0, 0)),
    P("y", (0.55, 0.2, 0), (0.24, 0.1, 0.24), "enjoliveur", (90, 0, 0)),
    P("y", (-0.5, 0.2, 0), (0.4, 0.08, 0.4), "pneu", (90, 0, 0)),
    P("y", (-0.5, 0.2, 0), (0.24, 0.1, 0.24), "enjoliveur", (90, 0, 0)),
    P("c", (-0.1, 0.28, 0), (0.8, 0.12, 0.14), "sombre"),
    P("c", (-0.1, 0.4, 0), (0.2, 0.2, 0.2), "enjoliveur"),
    P("s", (0.1, 0.52, 0), (0.4, 0.22, 0.24), "peinture"),
    P("c", (0.1, 0.6, 0), (0.3, 0.02, 0.04), "blanc"),
    P("c", (0.1, 0.55, 0), (0.3, 0.02, 0.08), "noir"),
    P("y", (0.65, 0.55, -0.07), (0.03, 0.6, 0.03), "enjoliveur", (0, 0, 35)),
    P("y", (0.65, 0.55, 0.07), (0.03, 0.6, 0.03), "enjoliveur", (0, 0, 35)),
    P("y", (0.65, 0.85, 0), (0.03, 0.5, 0.03), "enjoliveur", (90, 0, 0)),
    P("s", (0.8, 0.6, 0), (0.1, 0.1, 0.1), "phare"),
    P("c", (-0.35, 0.5, 0), (0.3, 0.08, 0.2), "noir"),
    P("c", (-0.62, 0.46, 0), (0.04, 0.06, 0.1), "feu_arriere"),
    P("y", (-0.5, 0.2, 0.12), (0.06, 0.5, 0.06), "enjoliveur", (0, 0, 90)),
    P("y", (-0.5, 0.2, -0.12), (0.06, 0.5, 0.06), "enjoliveur", (0, 0, 90)),
    P("c", (-0.15, 0.78, 0), (0.18, 0.3, 0.26), "noir"),
    P("s", (0.0, 0.98, 0), (0.18, 0.16, 0.18), "noir"),
    P("c", (0.09, 0.97, 0), (0.02, 0.06, 0.2), "vitre"),
]
for z in (-0.07, 0.07):
    fz = -0.15 if z < 0 else 0.15
    p += [
        P("s", (0.0, 1.08, z), (0.07, 0.07, 0.07), "blanc", id="pilote_yeux"),
        P("s", (0.04, 1.08, z), (0.03, 0.03, 0.03), "noir"),
        P("c", (-0.15, 0.85, fz), (0.05, 0.2, 0.02), "blanc", id="franges"),
        P("c", (0.05, 0.85, fz), (0.25, 0.04, 0.05), "noir", (0, 0, -20)),
    ]
V.append(dict(id="easy_ribbit", classe="moto", longueur=1.3, largeur=0.5,
    noms=dict(fr="Easy Ribbit", en="Easy Ribbit", de="Easy Quak", es="Easy Ribbit", it="Easy Ribbit"),
    points_deblocage=100,
    parties_mobiles=[dict(piece="pilote_yeux", type="oscille_vertical"), dict(piece="franges", type="oscille")],
    pieces=p))

# --- 8. Capsule Écarlate
p = [
    P("y", (0.45, 0.22, 0), (0.44, 0.1, 0.44), "pneu", (90, 0, 0)),
    P("y", (0.45, 0.22, 0), (0.242, 0.12, 0.242), "enjoliveur", (90, 0, 0)),
    P("y", (-0.45, 0.22, 0), (0.44, 0.1, 0.44), "pneu", (90, 0, 0)),
    P("y", (-0.45, 0.22, 0), (0.242, 0.12, 0.242), "enjoliveur", (90, 0, 0)),
    P("s", (0, 0.42, 0), (1.1, 0.34, 0.28), "peinture"),
    P("s", (0.3, 0.55, 0), (0.3, 0.2, 0.22), "vitre"),
    P("n", (0.65, 0.4, 0), (0.18, 0.2, 0.18), "peinture", (0, 0, -90)),
    P("s", (0.66, 0.45, 0), (0.1, 0.1, 0.1), "phare", id="phare"),
    P("c", (-0.5, 0.46, 0), (0.04, 0.08, 0.14), "feu_arriere"),
    P("c", (0.0, 0.62, 0), (0.35, 0.06, 0.2), "sombre"),
    P("c", (0.0, 0.25, 0), (0.8, 0.12, 0.2), "sombre"),
    P("c", (0.05, 0.08, 0), (0.9, 0.02, 0.2), "lumiere_rouge", id="halo"),
    P("c", (0.0, 0.45, 0.15), (0.35, 0.06, 0.01), "blanc"),
    P("c", (0.0, 0.45, -0.15), (0.35, 0.06, 0.01), "blanc"),
    P("c", (-0.2, 0.8, 0), (0.18, 0.3, 0.28), "peinture"),
    P("s", (-0.2, 1.05, 0), (0.18, 0.18, 0.18), "noir"),
    P("c", (-0.1, 0.9, 0), (0.3, 0.08, 0.1), "noir", (0, 0, -30)),
    P("c", (0.3, 0.74, 0), (0.1, 0.04, 0.45), "sombre"),
    P("y", (0.55, 0.25, -0.07), (0.03, 0.4, 0.03), "enjoliveur", (0, 0, -15)),
    P("y", (-0.35, 0.2, -0.14), (0.05, 0.6, 0.05), "enjoliveur", (0, 0, 90)),
    P("y", (0.55, 0.25, 0.07), (0.03, 0.4, 0.03), "enjoliveur", (0, 0, -15)),
    P("y", (-0.35, 0.2, 0.14), (0.05, 0.6, 0.05), "enjoliveur", (0, 0, 90)),
    P("c", (-0.35, 0.5, 0), (0.3, 0.03, 0.24), "sombre"),
    P("c", (0.0, 0.1, 0.0), (0.2, 0.04, 0.12), "sombre"),
]
V.append(dict(id="capsule_ecarlate", classe="moto", longueur=1.2, largeur=0.5,
    noms=dict(fr="Capsule Écarlate", en="Scarlet Capsule", de="Scharlachkapsel", es="Cápsula Escarlata", it="Capsula Scarlatta"),
    points_deblocage=350,
    parties_mobiles=[dict(piece="halo", type="pulse"), dict(piece="phare", type="clignote")],
    pieces=p))

# --- 9. Speed-Bus 50
p = [
    P("c", (0, 0.22, 0), (3.4, 0.1, 0.66), "sombre"),
    P("c", (0, 0.62, 0), (3.6, 0.7, 0.76), "peinture"),
    P("c", (0.05, 0.7, 0), (3.3, 0.25, 0.78), "blanc"),
]
for x in (-1.3, -0.92, -0.54, -0.16, 0.22, 0.6, 0.98, 1.36):
    for z in (-0.4, 0.4):
        p.append(P("c", (x, 0.7, z), (0.28, 0.2, 0.02), "vitre"))
p += [
    P("c", (1.75, 0.7, -0.2), (0.04, 0.22, 0.3), "vitre"),
    P("c", (1.75, 0.7, 0.2), (0.04, 0.22, 0.3), "vitre"),
    P("c", (-1.0, 0.99, 0), (1.4, 0.04, 0.7), "blanc"),
    P("c", (1.78, 0.5, 0), (0.04, 0.1, 0.78), "noir"),
]
p += lights(1.8, 1.8, 0.38, 0.28, size=(0.04, 0.1, 0.14))
p += [
    P("c", (-1.82, 0.85, 0), (0.04, 0.34, 0.5), "blanc"),
    P("y", (-1.84, 0.86, 0), (0.3, 0.04, 0.3), "lumiere_rouge", (0, 0, 90), id="panneau_rouge"),
    P("c", (-1.86, 0.86, 0.08), (0.02, 0.22, 0.04), "blanc"),
    P("c", (-1.86, 0.86, -0.08), (0.02, 0.22, 0.04), "blanc"),
    P("c", (0.6, 1.02, 0), (0.5, 0.06, 0.5), "noir", (0, 0, 12)),
]
p += wheels([1.15, -1.15, -1.45], 0.17, 0.34, 0.4)
V.append(dict(id="speed_bus_50", classe="bus", longueur=3.6, largeur=0.85,
    noms=dict(fr="Speed-Bus 50", en="Fifty-Mile Bus", de="Tempo-50-Bus", es="Autobús 50", it="Bus Cinquanta"),
    points_deblocage=500,
    parties_mobiles=[dict(piece="panneau_rouge", type="clignote"), dict(piece="*", type="oscille_vertical_0.02")],
    pieces=p))

# --- 10. Le Noctambulobus
p = [
    P("c", (0, 0.2, 0), (3.0, 0.08, 0.66), "sombre"),
    P("c", (0, 0.34, 0), (3.2, 0.24, 0.78), "peinture"),
    P("c", (0.1, 0.58, 0), (2.9, 0.24, 0.78), "peinture", id="etage_2"),
    P("c", (-0.1, 0.82, 0), (2.5, 0.24, 0.78), "peinture", id="etage_3"),
]
for y, w, xs in [(0.34, 0.24, (-1.2, -0.84, -0.48, -0.12, 0.24, 0.6, 0.96)), (0.58, 0.26, (-1.0, -0.62, -0.24, 0.14, 0.52, 0.9)), (0.82, 0.3, (-1.0, -0.58, -0.16, 0.26, 0.68))]:
    for x in xs:
        for z in (-0.4, 0.4):
            p.append(P("c", (x, y, z), (w, 0.1, 0.02), "vitre"))
p += [
    P("c", (1.6, 0.34, 0), (0.04, 0.14, 0.5), "vitre"),
    P("c", (1.7, 0.58, 0), (0.04, 0.14, 0.5), "vitre"),
    P("n", (1.6, 1.0, 0), (0.6, 0.2, 0.5), "sombre", (0, 0, -90)),
    P("n", (-0.1, 1.05, 0), (0.7, 0.2, 0.7), "sombre"),
]
p += lights(1.6, 1.6, 0.25, 0.3, size=(0.04, 0.08, 0.12))
p += [
    P("s", (0.4, 0.95, 0.42), (0.08, 0.08, 0.08), "phare", id="lanterne_0"),
    P("s", (-0.5, 0.95, 0.42), (0.08, 0.08, 0.08), "phare", id="lanterne_1"),
    P("s", (0.4, 0.95, -0.42), (0.08, 0.08, 0.08), "phare", id="lanterne_2"),
    P("s", (-0.5, 0.95, -0.42), (0.08, 0.08, 0.08), "phare", id="lanterne_3"),
]
p += wheels([1.0, -0.9], 0.12, 0.24, 0.4)
V.append(dict(id="noctambulobus", classe="bus", longueur=3.2, largeur=0.85,
    noms=dict(fr="Le Noctambulobus", en="The Night-Hopper Bus", de="Der Nachtschwärmer-Bus", es="El Autobús Trasnochador", it="Il Bus Nottambulo"),
    points_deblocage=700,
    parties_mobiles=[dict(piece="lanterne_*", type="balance"), dict(piece="etage_2", type="decalage_x_ping_pong_0.08"), dict(piece="etage_3", type="decalage_x_ping_pong_-0.08")],
    pieces=p))

# --- 11. La Machine à Mystères
p = [
    P("c", (0, 0.2, 0), (2.0, 0.08, 0.66), "sombre"),
    P("c", (0, 0.58, 0), (2.2, 0.7, 0.78), "peinture"),
    P("c", (0.3, 0.95, 0), (1.4, 0.04, 0.78), "blanc"),
    P("c", (0.95, 0.65, 0), (0.3, 0.35, 0.7), "vitre"),
    P("c", (1.08, 0.35, 0), (0.06, 0.2, 0.7), "sombre"),
    P("c", (1.12, 0.45, -0.25), (0.04, 0.1, 0.14), "phare"),
    P("c", (-1.12, 0.4, -0.25), (0.04, 0.1, 0.14), "feu_arriere"),
]
for z in (-0.4, 0.4):
    s = -1 if z < 0 else 1
    p += [
        P("c", (0.0, 0.7, z), (0.6, 0.25, 0.02), "vitre"),
        P("c", (-0.6, 0.62, z), (0.35, 0.4, 0.02), "blanc"),
        P("y", (0.2, 0.45, z), (0.18, 0.02, 0.18), "blanc", (90, 0, 0)),
        P("y", (-0.4, 0.3, z), (0.14, 0.02, 0.14), "blanc", (90, 0, 0)),
        P("s", (-0.2, 0.4, z + 0.01 * s), (0.1, 0.1, 0.02), "noir"),
        P("n", (0.6, 0.35, z), (0.18, 0.12, 0.02), "blanc"),
    ]
    if z < 0:
        p += [P("c", (1.12, 0.45, 0.25), (0.04, 0.1, 0.14), "phare"), P("c", (-1.12, 0.4, 0.25), (0.04, 0.1, 0.14), "feu_arriere")]
p += [
    P("c", (0.85, 0.98, 0), (0.1, 0.1, 0.3), "noir"),
    P("s", (0.3, 0.8, -0.05), (0.12, 0.12, 0.12), "blanc", id="chien"),
    P("n", (0.3, 0.9, -0.05), (0.08, 0.1, 0.08), "noir", id="chien"),
    P("c", (1.12, 0.3, 0), (0.04, 0.06, 0.5), "blanc"),
]
p += wheels([0.7, -0.7], 0.15, 0.3, 0.375)
V.append(dict(id="machine_mysteres", classe="bus", longueur=2.3, largeur=0.85,
    noms=dict(fr="La Machine à Mystères", en="The Mystery Wagon", de="Der Geheimnis-Wagen", es="La Máquina de Misterios", it="La Macchina dei Misteri"),
    points_deblocage=1000,
    parties_mobiles=[dict(piece="chien", type="tremble"), dict(piece="*", type="oscille_vertical_0.02")],
    pieces=p))

# --- 12. Ecto-Croa 1
p = [
    P("c", (0, 0.2, 0), (2.3, 0.1, 0.66), "sombre"),
    P("c", (0, 0.62, 0), (2.4, 0.6, 0.76), "blanc"),
    P("c", (0.85, 0.68, 0), (0.5, 0.3, 0.78), "vitre"),
    P("c", (0.85, 0.95, 0), (0.5, 0.04, 0.78), "blanc"),
    P("c", (0.0, 0.5, 0.385), (2.2, 0.1, 0.01), "peinture"),
    P("c", (0.0, 0.5, -0.385), (2.2, 0.1, 0.01), "peinture"),
    P("c", (1.2, 0.45, 0), (0.06, 0.18, 0.78), "sombre"),
    P("c", (1.2, 0.52, -0.28), (0.04, 0.1, 0.14), "phare"),
    P("c", (-1.2, 0.45, -0.28), (0.04, 0.1, 0.14), "feu_arriere"),
    P("c", (1.2, 0.52, 0.28), (0.04, 0.1, 0.14), "phare"),
    P("c", (-1.2, 0.45, 0.28), (0.04, 0.1, 0.14), "feu_arriere"),
]
for z in (-1, 1):
    p += [
        P("y", (0.0, 0.72, 0.385 * z), (0.22, 0.02, 0.22), "blanc", (90, 0, 0)),
        P("y", (0.0, 0.72, 0.395 * z), (0.18, 0.02, 0.18), "noir", (90, 0, 0)),
        P("c", (0.0, 0.72, 0.4 * z), (0.22, 0.04, 0.02), "blanc", (0, 0, 45)),
    ]
p += [
    P("c", (-0.3, 0.96, 0), (1.2, 0.04, 0.6), "sombre"),
    P("c", (-0.4, 1.0, -0.25), (0.04, 0.1, 0.04), "sombre"),
    P("c", (0.2, 1.0, -0.25), (0.04, 0.1, 0.04), "sombre"),
    P("c", (-0.4, 1.0, 0.25), (0.04, 0.1, 0.04), "sombre"),
    P("c", (0.2, 1.0, 0.25), (0.04, 0.1, 0.04), "sombre"),
    P("y", (-0.6, 1.04, 0.15), (0.1, 0.2, 0.1), "sombre"),
    P("y", (-0.2, 1.04, -0.15), (0.1, 0.2, 0.1), "sombre"),
    P("n", (-0.9, 1.1, 0), (0.15, 0.2, 0.15), "noir"),
    P("c", (0.7, 1.01, 0.15), (0.1, 0.08, 0.1), "lumiere_rouge", id="gyro_rouge"),
    P("c", (0.7, 1.01, -0.15), (0.1, 0.08, 0.1), "lumiere_bleue", id="gyro_bleu"),
    P("c", (0.0, 1.04, 0), (0.3, 0.04, 0.04), "noir", id="radar"),
    P("s", (-0.2, 0.98, 0), (0.12, 0.1, 0.12), "enjoliveur"),
]
p += wheels([0.85, -0.8], 0.16, 0.32, 0.375)
V.append(dict(id="ecto_croa_1", classe="camion", longueur=2.4, largeur=0.85,
    noms=dict(fr="Ecto-Croa 1", en="Ecto-Ribbit 1", de="Ekto-Quak 1", es="Ecto-Croa 1", it="Ecto-Gra 1"),
    points_deblocage=1500,
    parties_mobiles=[dict(piece="gyro_rouge", type="clignote"), dict(piece="gyro_bleu", type="clignote_alterne"), dict(piece="radar", type="rotation_y")],
    pieces=p))

# --- 13. Le Pétrolier du Duel
p = [
    P("c", (0, 0.22, 0), (2.8, 0.1, 0.62), "sombre"),
    P("c", (1.0, 0.62, 0), (0.7, 0.5, 0.76), "peinture"),
    P("c", (1.15, 0.8, 0), (0.4, 0.22, 0.78), "vitre"),
    P("c", (1.35, 0.55, 0), (0.08, 0.35, 0.7), "sombre"),
    P("c", (1.4, 0.3, 0), (0.1, 0.1, 0.8), "noir"),
    P("c", (1.4, 0.55, -0.28), (0.04, 0.1, 0.14), "phare"),
    P("c", (-1.4, 0.38, -0.25), (0.04, 0.08, 0.12), "feu_arriere"),
    P("y", (0.65, 0.82, -0.3), (0.06, 0.4, 0.06), "noir"),
    P("c", (1.4, 0.55, 0.28), (0.04, 0.1, 0.14), "phare"),
    P("c", (-1.4, 0.38, 0.25), (0.04, 0.08, 0.12), "feu_arriere"),
    P("y", (0.65, 0.82, 0.3), (0.06, 0.4, 0.06), "noir"),
    P("y", (-0.35, 0.6, 0), (0.7, 1.9, 0.7), "sombre", (0, 0, 90)),
    P("s", (0.62, 0.6, 0), (0.7, 0.7, 0.7), "sombre"),
    P("s", (-1.32, 0.6, 0), (0.7, 0.7, 0.7), "sombre"),
    P("y", (0.3, 0.6, 0), (0.74, 0.08, 0.74), "noir", (0, 0, 90)),
    P("y", (-0.35, 0.6, 0), (0.74, 0.08, 0.74), "noir", (0, 0, 90)),
    P("y", (-1.0, 0.6, 0), (0.74, 0.08, 0.74), "noir", (0, 0, 90)),
    P("y", (-0.35, 0.98, 0), (0.2, 0.08, 0.2), "noir"),
    P("c", (-0.35, 0.22, 0.39), (1.9, 0.08, 0.02), "noir"),
    P("c", (-0.35, 0.22, -0.39), (1.9, 0.08, 0.02), "noir"),
]
p += wheels([1.05, -0.7, -1.0, -1.3], 0.17, 0.34, 0.4)
p += [
    P("s", (0.65, 1.08, 0.3), (0.09, 0.09, 0.09), "noir", id="fumee_0"),
    P("s", (0.65, 1.14, 0.3), (0.11, 0.11, 0.11), "noir", id="fumee_1"),
    P("s", (0.65, 1.2, 0.3), (0.13, 0.13, 0.13), "noir", id="fumee_2"),
]
V.append(dict(id="petrolier_duel", classe="camion", longueur=2.9, largeur=0.85,
    noms=dict(fr="Le Pétrolier du Duel", en="The Dueling Tanker", de="Der Duell-Tanker", es="El Petrolero del Duelo", it="L'Autobotte del Duello"),
    points_deblocage=1500,
    parties_mobiles=[dict(piece="fumee_*", type="derive_vertical_boucle"), dict(piece="*", type="vibre_0.01")],
    pieces=p))

# --- 14. Optimus Primeur
p = [
    P("c", (0, 0.22, 0), (2.8, 0.1, 0.64), "sombre"),
    P("c", (0.95, 0.55, 0), (0.8, 0.35, 0.76), "peinture"),
    P("c", (1.0, 0.88, 0), (0.65, 0.3, 0.74), "peinture"),
    P("c", (1.2, 0.9, 0), (0.3, 0.24, 0.76), "vitre"),
    P("c", (1.37, 0.5, 0), (0.06, 0.3, 0.7), "enjoliveur"),
]
for y in (0.4, 0.47, 0.54, 0.61):
    p.append(P("c", (1.4, y, 0), (0.03, 0.02, 0.6), "sombre"))
p += [
    P("c", (1.4, 0.2, 0), (0.08, 0.1, 0.8), "enjoliveur"),
    P("c", (1.0, 1.06, 0), (0.55, 0.04, 0.7), "enjoliveur"),
    P("s", (1.38, 0.62, 0.24), (0.1, 0.1, 0.1), "phare", id="phare_g"),
    P("s", (1.38, 0.62, -0.24), (0.1, 0.1, 0.1), "phare"),
]
for z in (-1, 1):
    p += [
        P("y", (1.0, 1.2, 0.3 * z), (0.04, 0.3, 0.04), "enjoliveur"),
        P("n", (0.9, 0.45, 0.39 * z), (0.14, 0.4, 0.03), "blanc", (0, 0, -90)),
        P("n", (0.75, 0.58, 0.39 * z), (0.1, 0.3, 0.03), "blanc", (0, 0, -90)),
    ]
p += [
    P("c", (-0.5, 0.5, 0), (1.8, 0.55, 0.8), "sombre"),
    P("c", (-0.5, 0.8, 0), (1.9, 0.04, 0.84), "enjoliveur"),
    P("c", (-1.45, 0.3, -0.28), (0.04, 0.08, 0.12), "feu_arriere"),
    P("c", (-1.45, 0.3, 0.28), (0.04, 0.08, 0.12), "feu_arriere"),
]
for x, z in [(-1.1, -0.2), (-1.1, 0.2), (-0.65, -0.2), (-0.65, 0.2), (-0.2, -0.2), (-0.2, 0.2), (0.25, -0.2), (0.25, 0.2)]:
    p.append(P("s", (x, 0.9, z), (0.28, 0.22, 0.28), "peinture", id="cageots"))
p += [P("s", (-0.4, 0.92, 0), (0.3, 0.25, 0.3), "blanc", id="cageots"), P("n", (-0.9, 0.92, 0), (0.2, 0.3, 0.2), "noir", id="cageots")]
p += wheels([1.0, -0.8, -1.1], 0.17, 0.34, 0.4)
V.append(dict(id="optimus_primeur", classe="camion", longueur=2.9, largeur=0.85,
    noms=dict(fr="Optimus Primeur", en="Optimus Produce", de="Optimus Gemüsehändler", es="Optimus Verdulero", it="Optimus Fruttivendolo"),
    points_deblocage=700,
    parties_mobiles=[dict(piece="phare_g", type="clignote_clin_doeil"), dict(piece="cageots", type="oscille")],
    pieces=p))


def bumpers(y=0.2, mat="sombre", x=0.81):
    return [P("c", (x, y, 0), (0.06, 0.07, 0.76), mat), P("c", (-x, y, 0), (0.06, 0.07, 0.76), mat)]


def car_glass(x, y, w, h, side_z=0.341, front_x=None, rear_x=None, d=0.52):
    return [
        P("c", (x, y, -side_z), (w, h, 0.02), "vitre"), P("c", (x, y, side_z), (w, h, 0.02), "vitre"),
        P("c", (rear_x, y, 0), (0.02, h, d), "vitre"), P("c", (front_x, y, 0), (0.02, h, d), "vitre"),
    ]


def vehicle(id, classe, longueur, largeur, noms, points, mobiles, pieces):
    fr, en, de, es, it = noms
    V.append(dict(id=id, classe=classe, longueur=longueur, largeur=largeur,
        noms=dict(fr=fr, en=en, de=de, es=es, it=it), points_deblocage=points,
        parties_mobiles=[dict(piece=a, type=b) for a, b in mobiles], pieces=pieces))


# --- 15. Hobbie 53
p = [
    P("c", (0, 0.17, 0), (1.48, 0.1, 0.66), "sombre"),
    P("c", (0, 0.32, 0), (1.6, 0.2, 0.72), "blanc"),
    P("c", (-0.1, 0.53, 0), (0.6, 0.22, 0.66), "blanc"),
    P("c", (-0.1, 0.66, 0), (0.62, 0.04, 0.68), "blanc"),
] + car_glass(-0.1, 0.54, 0.46, 0.132, front_x=0.205, rear_x=-0.405)
p += lights(0.8, 0.8, 0.29, 0.25) + bumpers() + wheels([0.44, -0.44], 0.14, 0.28, 0.375)
p += [P("s", (x, 0.3, z), (0.3, 0.24, 0.14), "blanc") for x, z in [(0.6, -0.33), (0.6, 0.33), (-0.6, -0.33), (-0.6, 0.33)]]
p += [
    P("c", (0.3, 0.445, 0.12), (0.5, 0.01, 0.07), "peinture"), P("c", (0.3, 0.445, -0.12), (0.5, 0.01, 0.07), "peinture"),
    P("c", (-0.1, 0.7, 0.12), (0.6, 0.01, 0.07), "peinture"), P("c", (-0.1, 0.7, -0.12), (0.6, 0.01, 0.07), "peinture"),
    P("y", (0.5, 0.45, 0), (0.2, 0.02, 0.2), "noir"), P("c", (0.5, 0.465, 0), (0.08, 0.01, 0.1), "blanc"),
]
vehicle("hobbie_53", "voiture", 1.6, 0.85, ("Hobbie 53", "Hobby Bug 53", "Hobbie-Käfer 53", "Escarabajo Hobbie 53", "Maggiolino Hobbie 53"), 125, [("*", "sautille")], p)

# --- 16. La Mobile des Frères Blues
p = [
    P("c", (0, 0.17, 0), (1.48, 0.1, 0.66), "sombre"),
    P("c", (0, 0.32, 0), (1.6, 0.2, 0.72), "noir"),
    P("c", (-0.1, 0.53, 0), (0.7, 0.22, 0.66), "noir"),
    P("c", (-0.1, 0.66, 0), (0.72, 0.04, 0.68), "noir"),
] + car_glass(-0.1, 0.54, 0.56, 0.132, front_x=0.255, rear_x=-0.455)
p += lights(0.8, 0.8, 0.29, 0.25) + bumpers() + wheels([0.44, -0.44], 0.15, 0.3, 0.375)
p += [
    P("c", (0, 0.34, -0.362), (0.9, 0.14, 0.01), "blanc"), P("c", (0, 0.34, 0.362), (0.9, 0.14, 0.01), "blanc"),
    P("c", (0, 0.82, 0), (0.25, 0.04, 0.5), "sombre"),
    P("c", (0, 0.86, 0.12), (0.15, 0.06, 0.12), "lumiere_rouge", id="gyro_rouge"),
    P("c", (0, 0.86, -0.12), (0.15, 0.06, 0.12), "lumiere_bleue", id="gyro_bleu"),
    P("y", (-0.5, 0.85, 0.25), (0.02, 0.3, 0.02), "sombre", (0, 0, 10)),
    P("c", (0.45, 0.46, 0), (0.5, 0.02, 0.4), "sombre"),
]
vehicle("mobile_freres_blues", "voiture", 1.6, 0.85, ("La Mobile des Frères Blues", "The Blues Brothers' Cruiser", "Der Blues-Brüder-Wagen", "El Coche de los Hermanos Blues", "L'Auto dei Fratelli Blues"), 225,
        [("gyro_rouge", "clignote"), ("gyro_bleu", "clignote_alterne")], p)

# --- 17. Le Taxi Marseillais
p = [
    P("c", (0, 0.17, 0), (1.48, 0.1, 0.66), "sombre"),
    P("c", (0, 0.32, 0), (1.6, 0.2, 0.72), "blanc"),
    P("c", (-0.15, 0.53, 0), (0.65, 0.22, 0.66), "blanc"),
    P("c", (-0.15, 0.66, 0), (0.67, 0.04, 0.68), "blanc"),
] + car_glass(-0.15, 0.54, 0.51, 0.132, front_x=0.18, rear_x=-0.48)
p += lights(0.8, 0.8, 0.29, 0.25) + bumpers() + wheels([0.44, -0.44], 0.14, 0.28, 0.375)
p += [
    P("c", (-0.15, 0.73, 0), (0.3, 0.1, 0.14), "blanc", id="enseigne"),
    P("c", (-0.15, 0.73, 0.075), (0.2, 0.05, 0.01), "noir"),
    P("c", (-0.05, 0.34, -0.362), (1.0, 0.05, 0.01), "noir"), P("c", (0.25, 0.5, -0.4), (0.06, 0.05, 0.04), "sombre"),
    P("c", (-0.05, 0.34, 0.362), (1.0, 0.05, 0.01), "noir"), P("c", (0.25, 0.5, 0.4), (0.06, 0.05, 0.04), "sombre"),
    P("y", (-0.7, 0.8, 0.28), (0.02, 0.3, 0.02), "sombre", (0, 0, -10)),
    P("c", (0.55, 0.46, 0), (0.3, 0.02, 0.4), "blanc"), P("c", (-0.7, 0.46, 0), (0.2, 0.02, 0.4), "blanc"),
]
vehicle("taxi_marseillais", "voiture", 1.6, 0.85, ("Le Taxi Marseillais", "The Marseille Cab", "Das Marseille-Taxi", "El Taxi Marsellés", "Il Taxi Marsigliese"), 175,
        [("*", "embardee_laterale_0.05"), ("enseigne", "clignote")], p)

# --- 18. La Deudeuche du Corniaud
p = [
    P("c", (0, 0.17, 0), (1.48, 0.1, 0.66), "sombre"),
    P("c", (0, 0.33, 0), (1.6, 0.22, 0.72), "peinture"),
    P("c", (-0.05, 0.59, 0), (0.9, 0.3, 0.66), "peinture"),
    P("c", (-0.05, 0.76, 0), (0.92, 0.04, 0.68), "peinture"),
] + car_glass(-0.05, 0.6, 0.76, 0.18, front_x=0.405, rear_x=-0.505)
p += lights(0.8, 0.8, 0.29, 0.25) + bumpers() + wheels([0.44, -0.44], 0.14, 0.28, 0.375)
p += [P("s", (x, 0.28, z), (0.3, 0.22, 0.14), "peinture") for x, z in [(0.6, -0.33), (0.6, 0.33), (-0.6, -0.33), (-0.6, 0.33)]]
p += [
    P("s", (0.82, 0.4, -0.27), (0.16, 0.16, 0.16), "phare"), P("s", (0.82, 0.4, 0.27), (0.16, 0.16, 0.16), "phare"),
    P("c", (-0.5, 0.8, 0), (0.4, 0.05, 0.7), "noir", (0, 0, -12)),
    P("c", (-0.1, 0.46, 0), (0.6, 0.03, 0.34), "sombre"),
]
vehicle("deudeuche_corniaud", "voiture", 1.6, 0.85, ("La Deudeuche du Corniaud", "The Sucker's Deux-Chevaux", "Die Ente vom Gimpel", "El Dos Caballos del Primo", "La Due Cavalli del Pollo"), 300,
        [("*", "penche_dans_les_virages")], p)

# --- 19. Jeep-Rassic Park
p = [
    P("c", (0, 0.17, 0), (1.48, 0.1, 0.66), "sombre"),
    P("c", (0, 0.32, 0), (1.6, 0.2, 0.72), "peinture"),
    P("c", (-0.15, 0.52, 0), (0.55, 0.2, 0.66), "peinture"),
    P("c", (-0.15, 0.64, 0), (0.57, 0.04, 0.68), "peinture"),
] + car_glass(-0.15, 0.53, 0.41, 0.12, front_x=0.13, rear_x=-0.43)
p += lights(0.8, 0.8, 0.29, 0.25) + bumpers() + wheels([0.44, -0.44], 0.16, 0.32, 0.375)
for z in (-0.28, 0.28):
    p += [
        P("c", (0.4, 0.45, z / 0.28 * 0.15), (0.4, 0.01, 0.07), "blanc"),
        P("y", (-0.45, 0.66, z), (0.03, 0.28, 0.03), "sombre"),
        P("y", (0.15, 0.66, z), (0.03, 0.28, 0.03), "sombre"),
        P("y", (-0.15, 0.8, z), (0.03, 0.6, 0.03), "sombre", (0, 0, 90)),
    ]
p += [
    P("y", (-0.15, 0.8, 0), (0.03, 0.56, 0.03), "sombre", (90, 0, 0)),
    P("y", (-0.86, 0.45, 0), (0.28, 0.08, 0.28), "pneu", (0, 0, 90)),
    P("n", (0.88, 0.25, 0.2), (0.06, 0.12, 0.06), "blanc", (0, 0, -90), id="dent"),
    P("c", (0.35, 0.42, 0), (0.04, 0.1, 0.24), "blanc"),
]
vehicle("jeep_rassic", "voiture", 1.8, 0.85, ("Jeep-Rassic Park", "Jeep-Rassic Park", "Jeep-Rassic Park", "Jeep-Rásico", "Jeep-Rassic Park"), 550, [("dent", "oscille")], p)

# --- 20. Mini Braquage (trois Mini qui serpentent)
def mini(cx, body, roof, mid):
    q = [
        P("c", (cx, 0.25, 0), (1.0, 0.22, 0.7), body, id=mid),
        P("c", (cx - 0.05, 0.46, 0), (0.5, 0.2, 0.64), body, id=mid),
        P("c", (cx - 0.05, 0.58, 0), (0.52, 0.03, 0.66), roof, id=mid),
        P("c", (cx - 0.05, 0.47, -0.325), (0.4, 0.12, 0.01), "vitre", id=mid),
        P("c", (cx + 0.5, 0.28, -0.22), (0.03, 0.07, 0.12), "phare", id=mid),
        P("c", (cx - 0.5, 0.28, -0.22), (0.03, 0.07, 0.12), "feu_arriere", id=mid),
        P("c", (cx - 0.05, 0.47, 0.325), (0.4, 0.12, 0.01), "vitre", id=mid),
        P("c", (cx + 0.5, 0.28, 0.22), (0.03, 0.07, 0.12), "phare", id=mid),
        P("c", (cx - 0.5, 0.28, 0.22), (0.03, 0.07, 0.12), "feu_arriere", id=mid),
    ]
    for x in (cx + 0.3, cx - 0.3):
        for z in (0.375, -0.375):
            q.append(P("y", (x, 0.12, z), (0.24, 0.11, 0.24), "pneu", (90, 0, 0), id=mid))
            q.append(P("y", (x, 0.12, z + (0.005 if z > 0 else -0.005)), (0.13, 0.13, 0.13), "enjoliveur", (90, 0, 0), id=mid))
    return q

p = mini(1.1, "peinture", "blanc", "mini_a") + mini(0.0, "blanc", "noir", "mini_b") + mini(-1.1, "sombre", "blanc", "mini_c")
vehicle("mini_braquage", "voiture", 3.2, 0.85, ("Mini Braquage", "Mini Heist", "Mini-Coup", "Mini Atraco", "Mini Colpo"), 400,
        [("mini_a", "serpente_phase_0"), ("mini_b", "serpente_phase_1"), ("mini_c", "serpente_phase_2")], p)

# --- 21. Brillantine Éclair
p = [
    P("c", (0, 0.17, 0), (1.48, 0.1, 0.66), "sombre"),
    P("c", (0, 0.34, 0), (1.6, 0.24, 0.72), "blanc"),
    P("c", (0.15, 0.58, 0), (0.04, 0.24, 0.62), "vitre", (0, 0, -20)),
    P("c", (-0.15, 0.52, 0), (0.3, 0.14, 0.5), "sombre"),
    P("c", (-0.3, 0.62, 0), (0.06, 0.2, 0.5), "sombre"),
]
for z in (-1, 1):
    p += [
        P("c", (-0.75, 0.58, 0.3 * z), (0.2, 0.22, 0.04), "blanc", (0, 0, 15)),
        P("n", (0.1, 0.34, 0.365 * z), (0.12, 0.6, 0.02), "lumiere_rouge", (0, 0, -90), id="flammes"),
        P("n", (-0.3, 0.36, 0.365 * z), (0.1, 0.4, 0.02), "lumiere_rouge", (0, 0, -90), id="flammes"),
        P("c", (0.8, 0.3, 0.25 * z), (0.04, 0.08, 0.14), "phare"),
        P("c", (-0.8, 0.3, 0.25 * z), (0.04, 0.08, 0.14), "feu_arriere"),
    ]
p += bumpers(mat="enjoliveur", x=0.82) + [P("y", (0.3, 0.5, 0), (0.1, 0.02, 0.1), "noir")] + wheels([0.45, -0.45], 0.14, 0.28, 0.375)
p += [
    P("s", (-1.0, 0.45, -0.05), (0.05, 0.05, 0.05), "blanc", id="paillettes"),
    P("s", (-1.12, 0.49, 0.0), (0.05, 0.05, 0.05), "blanc", id="paillettes"),
    P("s", (-1.24, 0.53, 0.05), (0.05, 0.05, 0.05), "blanc", id="paillettes"),
]
vehicle("brillantine_eclair", "voiture", 1.6, 0.85, ("Brillantine Éclair", "Brilliantine Lightning", "Brillantine-Blitz", "Brillantina Relámpago", "Brillantina Lampo"), 750,
        [("paillettes", "derive_arriere_boucle"), ("flammes", "pulse")], p)

# --- 22. Scooter de Vacances Romaines
p = [
    P("y", (0.38, 0.14, 0), (0.28, 0.08, 0.28), "pneu", (90, 0, 0)), P("y", (0.38, 0.14, 0), (0.168, 0.1, 0.168), "enjoliveur", (90, 0, 0)),
    P("y", (-0.38, 0.14, 0), (0.28, 0.08, 0.28), "pneu", (90, 0, 0)), P("y", (-0.38, 0.14, 0), (0.168, 0.1, 0.168), "enjoliveur", (90, 0, 0)),
    P("c", (0.0, 0.22, 0), (0.6, 0.07, 0.26), "peinture"),
    P("c", (0.38, 0.45, 0), (0.1, 0.45, 0.28), "peinture"),
    P("c", (0.3, 0.45, 0), (0.04, 0.4, 0.3), "peinture", (0, 0, 12)),
    P("c", (-0.3, 0.34, 0), (0.45, 0.3, 0.28), "peinture"),
    P("c", (-0.25, 0.52, 0), (0.4, 0.07, 0.24), "sombre"),
    P("c", (0.38, 0.7, 0), (0.06, 0.04, 0.44), "sombre"),
    P("s", (0.44, 0.58, 0), (0.12, 0.12, 0.12), "phare"),
    P("y", (0.38, 0.78, -0.25), (0.02, 0.12, 0.02), "enjoliveur"), P("y", (0.38, 0.86, -0.25), (0.1, 0.02, 0.1), "enjoliveur"),
    P("y", (0.38, 0.78, 0.25), (0.02, 0.12, 0.02), "enjoliveur"), P("y", (0.38, 0.86, 0.25), (0.1, 0.02, 0.1), "enjoliveur"),
    P("c", (-0.52, 0.4, 0), (0.04, 0.07, 0.12), "feu_arriere"),
    P("c", (-0.6, 0.55, 0), (0.2, 0.2, 0.3), "sombre"),
    P("c", (-0.15, 0.8, 0), (0.2, 0.32, 0.26), "blanc"),
    P("s", (-0.15, 1.08, 0), (0.2, 0.2, 0.2), "noir"),
    P("c", (0.2, 0.55, 0.12), (0.35, 0.06, 0.08), "blanc", (0, 0, 15)), P("c", (0.2, 0.55, -0.12), (0.35, 0.06, 0.08), "blanc", (0, 0, 15)),
    P("c", (-0.35, 1.02, 0), (0.3, 0.04, 0.12), "peinture", id="foulard"),
]
vehicle("scooter_vacances_romaines", "moto", 1.1, 0.45, ("Scooter de Vacances Romaines", "Roman Holiday Scooter", "Römische-Ferien-Roller", "Scooter de Vacaciones Romanas", "Scooter delle Vacanze Romane"), 75,
        [("foulard", "flotte_vent")], p)

# --- 23. Hasta la Vista Bike
p = [
    P("y", (0.52, 0.21, 0), (0.42, 0.1, 0.42), "pneu", (90, 0, 0)), P("y", (0.52, 0.21, 0), (0.231, 0.12, 0.231), "enjoliveur", (90, 0, 0)),
    P("y", (-0.5, 0.21, 0), (0.42, 0.1, 0.42), "pneu", (90, 0, 0)), P("y", (-0.5, 0.21, 0), (0.231, 0.12, 0.231), "enjoliveur", (90, 0, 0)),
    P("c", (0, 0.32, 0), (0.9, 0.1, 0.14), "sombre"),
    P("c", (0, 0.38, 0), (0.4, 0.28, 0.3), "enjoliveur"),
    P("c", (0.18, 0.6, 0), (0.36, 0.14, 0.24), "noir"),
    P("c", (-0.2, 0.5, 0), (0.4, 0.08, 0.22), "noir"),
]
for z in (-1, 1):
    p += [
        P("y", (0.62, 0.4, 0.08 * z), (0.04, 0.5, 0.04), "enjoliveur", (0, 0, -20)),
        P("y", (-0.1, 0.18, 0.2 * z), (0.07, 0.8, 0.07), "enjoliveur", (0, 0, 90), id="echappement"),
        P("c", (-0.55, 0.35, 0.22 * z), (0.3, 0.2, 0.14), "noir"),
    ]
p += [
    P("c", (0.6, 0.72, 0), (0.06, 0.04, 0.46), "sombre"),
    P("s", (0.72, 0.58, 0), (0.12, 0.12, 0.12), "lumiere_rouge", id="oeil"),
    P("c", (-0.7, 0.42, 0), (0.04, 0.06, 0.12), "feu_arriere"),
    P("c", (-0.2, 0.8, 0), (0.2, 0.38, 0.3), "noir"),
    P("s", (-0.1, 1.06, 0), (0.2, 0.2, 0.2), "noir"),
    P("c", (0.0, 1.07, 0), (0.04, 0.05, 0.2), "noir"),
    P("c", (0.25, 0.85, 0.18), (0.4, 0.06, 0.07), "noir", (0, 0, -25)), P("c", (0.25, 0.85, -0.18), (0.4, 0.06, 0.07), "noir", (0, 0, -25)),
]
vehicle("hasta_la_vista_bike", "moto", 1.3, 0.5, ("Hasta la Vista Bike", "Hasta la Vista Bike", "Hasta-la-Vista-Bike", "Moto Hasta la Vista", "Moto Hasta la Vista"), 600,
        [("oeil", "clignote_lent"), ("echappement", "vibre")], p)

# --- 24. Le Side-car des Inventeurs
p = [
    P("y", (0.48, 0.18, 0.18), (0.36, 0.09, 0.36), "pneu", (90, 0, 0)), P("y", (0.48, 0.18, 0.18), (0.198, 0.11, 0.198), "enjoliveur", (90, 0, 0)),
    P("y", (-0.45, 0.18, 0.18), (0.36, 0.09, 0.36), "pneu", (90, 0, 0)), P("y", (-0.45, 0.18, 0.18), (0.198, 0.11, 0.198), "enjoliveur", (90, 0, 0)),
    P("y", (-0.2, 0.18, -0.3), (0.36, 0.09, 0.36), "pneu", (90, 0, 0)), P("y", (-0.2, 0.18, -0.3), (0.198, 0.11, 0.198), "enjoliveur", (90, 0, 0)),
    P("c", (0, 0.32, 0.18), (0.85, 0.1, 0.14), "sombre"),
    P("c", (0, 0.45, 0.18), (0.5, 0.22, 0.24), "peinture"),
    P("c", (0.12, 0.64, 0.18), (0.3, 0.12, 0.22), "noir"),
    P("s", (0.52, 0.58, 0.18), (0.12, 0.12, 0.12), "phare"),
    P("c", (0.5, 0.74, 0.18), (0.06, 0.04, 0.4), "sombre"),
    P("c", (-0.55, 0.45, 0.18), (0.04, 0.07, 0.12), "feu_arriere"),
    P("c", (-0.15, 0.62, 0.18), (0.3, 0.07, 0.22), "sombre"),
    P("c", (-0.15, 0.85, 0.18), (0.2, 0.35, 0.24), "blanc"),
    P("s", (-0.12, 1.1, 0.18), (0.2, 0.2, 0.2), "blanc"),
    P("c", (-0.05, 0.32, -0.05), (0.1, 0.05, 0.5), "sombre"),
    P("c", (0.3, 0.32, -0.1), (0.1, 0.05, 0.5), "sombre"),
    P("s", (-0.2, 0.45, -0.3), (0.9, 0.35, 0.4), "peinture"),
    P("s", (-0.1, 0.55, -0.3), (0.5, 0.2, 0.3), "vitre"),
    P("s", (-0.1, 0.7, -0.3), (0.2, 0.2, 0.2), "blanc", id="chien"),
    P("n", (-0.1, 0.82, -0.38), (0.07, 0.12, 0.07), "noir", id="chien"),
    P("n", (-0.1, 0.82, -0.22), (0.07, 0.12, 0.07), "noir", id="chien"),
    P("c", (0.3, 0.7, -0.3), (0.1, 0.1, 0.1), "phare"),
    P("c", (-0.65, 0.35, -0.3), (0.04, 0.07, 0.12), "feu_arriere"),
]
vehicle("sidecar_inventeurs", "moto", 1.5, 0.8, ("Le Side-car des Inventeurs", "The Inventors' Sidecar", "Das Erfinder-Beiwagengespann", "El Sidecar de los Inventores", "Il Sidecar degli Inventori"), 900,
        [("chien", "oscille_tete"), ("*", "oscille_vertical_0.01")], p)

# --- 25. La Camionnette Cabot
p = [
    P("c", (-0.45, 0.2, 0), (1.4, 0.08, 0.66), "sombre"),
    P("c", (-0.45, 0.58, 0), (1.4, 0.66, 0.78), "peinture"),
    P("c", (-0.45, 0.95, 0), (1.4, 0.04, 0.74), "peinture"),
    P("c", (0.55, 0.55, 0), (0.6, 0.6, 0.74), "peinture"),
    P("c", (0.95, 0.45, 0), (0.3, 0.25, 0.4), "blanc"),
    P("s", (1.12, 0.55, 0), (0.14, 0.14, 0.14), "noir"),
    P("c", (0.8, 0.82, 0), (0.3, 0.04, 0.7), "peinture"),
]
for z, oid in ((-1, "oreille_d"), (1, "oreille_g")):
    p += [
        P("s", (0.7, 0.72, 0.2 * z), (0.16, 0.16, 0.16), "blanc"),
        P("s", (0.78, 0.72, 0.2 * z), (0.07, 0.07, 0.07), "noir"),
        P("c", (0.5, 0.6, 0.4 * z), (0.15, 0.4, 0.05), "sombre", (0, 0, 6), id=oid),
        P("c", (-0.9, 0.4, 0.25 * z), (0.04, 0.1, 0.14), "feu_arriere"),
        P("c", (-0.45, 0.7, 0.395 * z), (0.7, 0.25, 0.01), "vitre"),
    ]
p += [
    P("c", (1.05, 0.3, 0), (0.2, 0.03, 0.2), "lumiere_rouge", id="langue"),
    P("c", (0.3, 0.8, 0), (0.04, 0.28, 0.4), "vitre", (0, 0, -15)),
    P("c", (1.08, 0.2, 0), (0.04, 0.07, 0.4), "sombre"),
] + wheels([0.6, -0.7], 0.15, 0.3, 0.375)
vehicle("camionnette_cabot", "bus", 2.3, 0.85, ("La Camionnette Cabot", "The Doggy Van", "Der Hundewagen", "La Furgoneta Perruna", "Il Furgone Cagnolone"), 1200,
        [("langue", "bat_au_vent"), ("oreille_*", "oscille"), ("*", "oscille_vertical_0.02")], p)

# --- 26. Petite Miss Combi
p = [
    P("c", (0, 0.2, 0), (2.1, 0.08, 0.66), "sombre"),
    P("c", (0, 0.4, 0), (2.3, 0.34, 0.8), "peinture"),
    P("c", (0, 0.7, 0), (2.2, 0.3, 0.78), "peinture"),
    P("c", (0, 0.9, 0), (2.2, 0.1, 0.8), "blanc"),
    P("c", (1.1, 0.55, 0), (0.04, 0.34, 0.7), "vitre"),
    P("c", (1.12, 0.4, 0), (0.04, 0.08, 0.3), "blanc"),
]
for z in (-1, 1):
    p += [P("c", (x, 0.74, 0.395 * z), (0.45, 0.2, 0.01), "vitre") for x in (-0.7, 0.0, 0.7)]
    p += [P("s", (1.12, 0.38, 0.25 * z), (0.14, 0.14, 0.14), "phare"), P("c", (-1.12, 0.4, 0.25 * z), (0.04, 0.08, 0.14), "feu_arriere")]
p += [
    P("c", (-0.3, 0.5, 0.405), (0.5, 0.3, 0.02), "sombre", (0, 6, 0), id="portiere"),
    P("c", (0.0, 0.98, 0), (1.4, 0.03, 0.6), "sombre"),
    P("c", (-0.3, 1.08, 0.1), (0.4, 0.18, 0.25), "sombre", id="valises"),
    P("c", (0.3, 1.06, -0.1), (0.4, 0.14, 0.25), "blanc", id="valises"),
    P("c", (0.8, 1.08, 0.1), (0.3, 0.16, 0.22), "noir", id="valises"),
    P("c", (1.15, 0.2, 0), (0.06, 0.07, 0.8), "enjoliveur"), P("c", (-1.15, 0.2, 0), (0.06, 0.07, 0.8), "enjoliveur"),
] + wheels([0.7, -0.7], 0.15, 0.3, 0.375)
vehicle("petite_miss_combi", "bus", 2.3, 0.85, ("Petite Miss Combi", "Little Miss Van", "Kleine Miss Bulli", "Pequeña Miss Furgo", "Piccola Miss Furgone"), 850,
        [("portiere", "tombe_periodiquement"), ("valises", "oscille")], p)

# --- 27. Le Bus de Tournée Magique
p = [
    P("c", (0, 0.2, 0), (3.2, 0.08, 0.66), "sombre"),
    P("c", (0, 0.4, 0), (3.4, 0.34, 0.78), "peinture"),
    P("c", (0, 0.73, 0), (3.0, 0.3, 0.78), "blanc"),
    P("c", (0, 0.9, 0), (3.0, 0.03, 0.8), "peinture"),
]
for x in (-1.2, -0.72, -0.24, 0.24, 0.72, 1.2):
    for y in (0.42, 0.74):
        for z in (-0.395, 0.395):
            p.append(P("c", (x, y, z), (0.32, 0.14, 0.01), "vitre"))
for z in (-1, 1):
    p += [P("c", (1.7, 0.42, 0.2 * z), (0.04, 0.16, 0.3), "vitre"), P("c", (1.5, 0.74, 0.2 * z), (0.04, 0.16, 0.3), "vitre"),
          P("c", (1.7, 0.3, 0.28 * z), (0.04, 0.08, 0.14), "phare"), P("c", (-1.7, 0.3, 0.28 * z), (0.04, 0.08, 0.14), "feu_arriere"),
          P("y", (0.5, 0.58, 0.4 * z), (0.16, 0.02, 0.16), "lumiere_rouge", (90, 0, 0), id="fleurs"),
          P("y", (-0.5, 0.58, 0.4 * z), (0.16, 0.02, 0.16), "lumiere_bleue", (90, 0, 0), id="fleurs")]
p += [P("y", (0.0, 0.58, 0.4), (0.12, 0.02, 0.12), "blanc", (90, 0, 0)), P("c", (1.7, 0.2, 0), (0.06, 0.07, 0.8), "sombre")]
p += wheels([1.1, -1.1], 0.15, 0.3, 0.4)
vehicle("bus_tournee_magique", "bus", 3.4, 0.85, ("Le Bus de Tournée Magique", "The Magical Tour Bus", "Der Magische Tourbus", "El Autobús de la Gira Mágica", "L'Autobus del Tour Magico"), 1000,
        [("fleurs", "rotation_z")], p)

# --- 28. L'Équipe Tous Risques
p = [
    P("c", (0, 0.2, 0), (2.2, 0.08, 0.66), "sombre"),
    P("c", (0, 0.58, 0), (2.4, 0.66, 0.78), "noir"),
    P("c", (0.9, 0.5, 0), (0.6, 0.35, 0.76), "noir"),
    P("c", (0.9, 0.75, 0), (0.04, 0.3, 0.7), "vitre", (0, 0, -18)),
]
for z in (-1, 1):
    p += [
        P("c", (0, 0.55, 0.395 * z), (2.4, 0.1, 0.01), "peinture"),
        P("c", (0.1, 0.78, 0.395 * z), (0.6, 0.2, 0.01), "vitre"),
        P("c", (0.9, 0.6, 0.395 * z), (0.45, 0.2, 0.01), "vitre"),
        P("c", (1.2, 0.4, 0.25 * z), (0.04, 0.1, 0.14), "phare"),
        P("c", (-1.2, 0.4, 0.25 * z), (0.04, 0.1, 0.14), "feu_arriere"),
        P("c", (-1.1, 0.88, 0.3 * z), (0.04, 0.1, 0.04), "sombre"),
    ]
p += [
    P("c", (0, 0.55, 0), (0.04, 0.1, 0.78), "peinture"),
    P("c", (1.2, 0.28, 0), (0.06, 0.1, 0.78), "enjoliveur"),
    P("c", (-1.2, 0.28, 0), (0.06, 0.1, 0.78), "sombre"),
    P("c", (-1.2, 0.95, 0), (0.3, 0.04, 0.7), "noir"),
    P("c", (0.3, 0.97, 0), (0.4, 0.08, 0.3), "noir"),
    P("y", (1.22, 0.5, 0), (0.04, 0.4, 0.04), "sombre", (90, 0, 0)),
] + wheels([0.75, -0.75], 0.15, 0.3, 0.375)
vehicle("equipe_tous_risques", "bus", 2.4, 0.85, ("L'Équipe Tous Risques", "The All-Risk Crew", "Das Allrisiko-Team", "El Equipo a Todo Riesgo", "La Squadra Tutto Rischio"), 1100, [], p)

# --- 29. Le Canard de Convoi
p = [
    P("c", (0, 0.2, 0), (2.8, 0.08, 0.62), "sombre"),
    P("c", (0.55, 0.45, 0), (0.7, 0.35, 0.74), "peinture"),
    P("c", (0.8, 0.45, 0), (0.7, 0.28, 0.7), "peinture"),
    P("c", (0.3, 0.85, 0), (0.5, 0.45, 0.76), "peinture"),
    P("c", (0.56, 0.9, 0), (0.04, 0.3, 0.7), "vitre", (0, 0, -10)),
    P("c", (0.3, 0.75, 0), (0.04, 0.1, 0.78), "vitre"),
    P("c", (1.18, 0.5, 0), (0.04, 0.3, 0.6), "enjoliveur"),
    P("c", (1.22, 0.3, 0), (0.06, 0.1, 0.76), "enjoliveur"),
    P("y", (0.2, 1.02, 0.3), (0.05, 0.4, 0.05), "enjoliveur", id="cheminee_g"),
    P("y", (0.2, 1.02, -0.3), (0.05, 0.4, 0.05), "enjoliveur", id="cheminee_d"),
    P("s", (1.0, 0.72, 0), (0.18, 0.14, 0.14), "blanc", id="canard"),
    P("s", (1.06, 0.82, 0), (0.1, 0.1, 0.1), "blanc", id="canard"),
    P("n", (1.12, 0.82, 0), (0.04, 0.06, 0.04), "peinture", (0, 0, -90), id="canard"),
]
for z in (-1, 1):
    p += [P("c", (1.2, 0.55, 0.28 * z), (0.04, 0.1, 0.14), "phare"), P("c", (-1.4, 0.35, 0.25 * z), (0.04, 0.08, 0.12), "feu_arriere"),
          P("c", (-0.5, 0.35, 0.4 * z), (1.8, 0.06, 0.02), "peinture")]
p += [P("c", (-0.5, 0.65, 0), (1.7, 0.7, 0.78), "blanc"), P("c", (-0.5, 0.65, 0.395), (1.5, 0.1, 0.01), "peinture"), P("c", (-0.5, 0.65, -0.395), (1.5, 0.1, 0.01), "peinture")]
p += wheels([0.8, 0.35, -0.8, -1.15], 0.16, 0.32, 0.4)
vehicle("canard_convoi", "camion", 2.9, 0.85, ("Le Canard de Convoi", "The Convoy Duck", "Die Konvoi-Ente", "El Pato del Convoy", "L'Anatra del Convoglio"), 900,
        [("canard", "oscille"), ("cheminee_*", "fumee")], p)

# --- 30. Planète Pizza
p = [
    P("c", (0, 0.2, 0), (2.7, 0.08, 0.62), "sombre"),
    P("c", (0.95, 0.5, 0), (0.8, 0.5, 0.76), "peinture"),
    P("c", (1.15, 0.72, 0), (0.04, 0.3, 0.7), "vitre", (0, 0, -12)),
    P("c", (1.37, 0.35, 0), (0.06, 0.1, 0.78), "sombre"),
    P("c", (-0.3, 0.65, 0), (2.1, 0.7, 0.82), "blanc"),
]
for z in (-1, 1):
    p += [
        P("c", (1.38, 0.5, 0.28 * z), (0.04, 0.1, 0.14), "phare"),
        P("c", (-1.36, 0.4, 0.25 * z), (0.04, 0.08, 0.12), "feu_arriere"),
        P("y", (0, 0.7, 0.415 * z), (0.3, 0.02, 0.3), "lumiere_bleue", (90, 0, 0)),
        P("y", (0, 0.7, 0.425 * z), (0.1, 0.02, 0.1), "lumiere_rouge", (90, 0, 0)),
        P("y", (-0.7, 0.7, 0.415 * z), (0.4, 0.02, 0.4), "peinture", (90, 0, 0)),
        P("y", (-0.62, 0.74, 0.425 * z), (0.06, 0.02, 0.06), "lumiere_rouge", (90, 0, 0)),
        P("y", (-0.78, 0.66, 0.425 * z), (0.06, 0.02, 0.06), "lumiere_rouge", (90, 0, 0)),
    ]
p += [
    P("y", (-0.3, 1.08, 0), (0.12, 0.16, 0.12), "blanc", id="fusee"),
    P("n", (-0.3, 1.19, 0), (0.12, 0.08, 0.12), "peinture", id="fusee"),
    P("n", (-0.3, 1.06, 0.08), (0.08, 0.12, 0.03), "noir", id="fusee"),
    P("n", (-0.3, 1.06, -0.08), (0.08, 0.12, 0.03), "noir", id="fusee"),
    P("n", (-0.3, 1.0, 0), (0.08, 0.06, 0.08), "lumiere_rouge", (180, 0, 0), id="flamme"),
    P("c", (-1.36, 0.65, 0), (0.04, 0.5, 0.6), "blanc"),
    P("c", (-0.3, 1.0, 0), (0.5, 0.06, 0.5), "sombre"),
] + wheels([0.95, -0.7, -1.0], 0.16, 0.32, 0.4)
vehicle("planete_pizza", "camion", 2.9, 0.85, ("Planète Pizza", "Planet Pizza", "Planet Pizza", "Planeta Pizza", "Pianeta Pizza"), 1300,
        [("flamme", "clignote"), ("fusee", "vibre")], p)

# --- 31. Le Camion Furie
p = [
    P("c", (0, 0.25, 0), (2.8, 0.1, 0.62), "sombre"),
    P("c", (0.8, 0.55, 0), (0.9, 0.5, 0.76), "sombre"),
    P("c", (0.5, 0.9, 0), (0.5, 0.35, 0.74), "sombre"),
    P("c", (0.76, 0.9, 0), (0.04, 0.2, 0.6), "vitre"),
    P("c", (1.28, 0.5, 0), (0.06, 0.4, 0.7), "enjoliveur"),
]
for z in (-1, 1):
    p += [
        P("c", (0.7, 0.6, 0.392 * z), (0.9, 0.3, 0.03), "noir", (0, 0, 5)),
        P("c", (1.3, 0.5, 0.25 * z), (0.04, 0.12, 0.14), "phare"),
        P("c", (-1.38, 0.4, 0.25 * z), (0.04, 0.1, 0.14), "feu_arriere"),
        P("n", (1.3, 0.8, 0.3 * z), (0.1, 0.25, 0.1), "noir", (0, 0, -90)),
        P("n", (0.2, 1.1, 0.3 * z), (0.1, 0.14, 0.1), "noir"),
        P("n", (1.0, 0.9, 0.43 * z), (0.08, 0.08, 0.08), "noir", (90 * (-z), 0, 0)),
        P("n", (0.3, 0.7, 0.43 * z), (0.08, 0.08, 0.08), "noir", (90 * (-z), 0, 0)),
    ]
p += [P("y", (-0.5, 0.7, 0), (0.6, 1.4, 0.6), "sombre", (0, 0, 90))]
p += [P("y", (x, 0.7, 0), (0.64, 0.06, 0.64), "noir", (0, 0, 90)) for x in (0.0, -0.5, -1.0)]
p += [
    P("y", (-0.5, 1.05, 0), (0.3, 0.1, 0.3), "enjoliveur", id="tourelle"),
    P("c", (-0.5, 1.12, 0), (0.3, 0.12, 0.2), "sombre", id="tourelle"),
    P("y", (-0.35, 1.12, 0), (0.04, 0.4, 0.04), "noir", (0, 0, 90), id="tourelle"),
    P("y", (0.5, 1.0, 0.2), (0.05, 0.2, 0.05), "sombre", id="fumee"),
    P("s", (0.5, 1.17, 0.2), (0.1, 0.1, 0.1), "noir", id="fumee"),
] + wheels([0.9, -0.5, -1.0], 0.2, 0.4, 0.4)
vehicle("camion_furie", "camion", 2.9, 0.85, ("Le Camion Furie", "The Fury Rig", "Der Furien-Truck", "El Camión Furia", "Il Camion Furia"), 1800,
        [("tourelle", "rotation_y"), ("fumee", "derive_vertical_boucle")], p)

# --- 32. Le Marchand de Glaces Fantôme
p = [
    P("c", (0, 0.2, 0), (2.2, 0.08, 0.64), "sombre"),
    P("c", (-0.2, 0.58, 0), (1.8, 0.66, 0.78), "blanc"),
    P("c", (0.9, 0.5, 0), (0.6, 0.5, 0.76), "blanc"),
    P("c", (1.1, 0.65, 0), (0.04, 0.25, 0.7), "vitre", (0, 0, -10)),
]
for z in (-1, 1):
    p += [
        P("c", (0, 0.65, 0.395 * z), (0.7, 0.28, 0.01), "vitre"),
        P("c", (0, 0.82, 0.41 * z), (0.8, 0.04, 0.06), "peinture"),
        P("c", (-0.9, 0.42, 0.25 * z), (0.04, 0.08, 0.14), "feu_arriere"),
        P("c", (1.2, 0.4, 0.25 * z), (0.04, 0.1, 0.14), "phare"),
        P("c", (0.9, 0.55, 0.395 * z), (0.4, 0.18, 0.01), "vitre"),
        P("y", (0.5, 0.96, -0.2 * z), (0.12, 0.08, 0.12), "enjoliveur", id="haut_parleur"),
        P("n", (0.6, 0.96, -0.2 * z), (0.2, 0.16, 0.2), "sombre", (0, 0, -90), id="haut_parleur"),
    ]
p += [
    P("c", (0, 0.45, 0.41), (0.7, 0.04, 0.12), "sombre"),
    P("n", (-0.2, 0.99, 0), (0.2, 0.18, 0.2), "peinture", (180, 0, 0), id="glace"),
    P("s", (-0.2, 1.1, 0), (0.24, 0.12, 0.24), "blanc", id="glace"),
    P("s", (-0.2, 1.18, 0), (0.05, 0.05, 0.05), "lumiere_rouge", id="glace"),
    P("c", (1.2, 0.2, 0), (0.06, 0.07, 0.8), "enjoliveur"), P("c", (-1.0, 0.2, 0), (0.06, 0.07, 0.8), "enjoliveur"),
] + wheels([0.8, -0.7], 0.15, 0.3, 0.375)
vehicle("marchand_glaces_fantome", "camion", 2.4, 0.85, ("Le Marchand de Glaces Fantôme", "The Ghost Ice Cream Van", "Der Geister-Eiswagen", "El Heladero Fantasma", "Il Gelataio Fantasma"), 1500,
        [("glace", "rotation_y"), ("haut_parleur", "pulse")], p)

# Le numéro de ligne « 2552 » en afficheur à 7 segments, au-dessus du pare-brise du Speed-Bus.
SEGS = {"2": "abged", "5": "afgcd"}
def digits(text, x, y, cz0, step, thick=0.015):
    out = []
    for i, ch in enumerate(text):
        cz = cz0 - i * step  # de gauche à droite pour qui regarde le bus de face (la droite est -Z)
        for sg in SEGS[ch]:
            if sg in "adg":
                dy = {"a": 0.065, "g": 0.0, "d": -0.065}[sg]
                out.append(P("c", (x, y + dy, cz), (thick, 0.02, 0.09), "lumiere_rouge"))
            else:
                dz = -0.045 if sg in "bc" else 0.045
                dy = 0.0325 if sg in "bf" else -0.0325
                out.append(P("c", (x, y + dy, cz + dz), (thick, 0.05, 0.02), "lumiere_rouge"))
    return out

def roof_digits(text, x0, step, y=1.014):
    """Le numéro en grand sur le toit : se lit de l'avant vers l'arrière, vu d'en haut (haut = -Z)."""
    out = []
    for i, ch in enumerate(text):
        cx = x0 + i * step
        for sg in SEGS[ch]:
            if sg in "adg":
                dz = {"a": -0.14, "g": 0.0, "d": 0.14}[sg]
                out.append(P("c", (cx, y, dz), (0.14, 0.012, 0.04), "lumiere_rouge"))
            else:
                dx = 0.07 if sg in "bc" else -0.07
                dz = -0.07 if sg in "bf" else 0.07
                out.append(P("c", (cx + dx, y, dz), (0.04, 0.012, 0.14), "lumiere_rouge"))
    return out

for v in V:
    if v["id"] == "speed_bus_50":
        v["pieces"] += roof_digits("2552", -1.5, 0.32)
for v in V:
    if v["id"] == "speed_bus_50":
        v["pieces"] += [P("c", (1.803, 0.89, 0), (0.01, 0.17, 0.7), "noir")] + digits("2552", 1.812, 0.89, 0.27, 0.18)

# --- Véhicules importés d'un univers (fichiers JSON de data/import)
IMPORTS = [("data/import/jurassic_croa_vehicules.json", 1500), ("data/import/nippon_kaboom_vehicules.json", 1800), ("data/import/espace_vehicules.json", 2400), ("data/import/halloween_vehicules.json", 3600)]  # (fichier, points ajoutés au déblocage)
for fichier, decalage in IMPORTS:
    with open(os.path.join(ROOT, "games/traversee", fichier), encoding="utf-8") as fh:
        for v in json.load(fh)["vehicules"]:
            v["points_deblocage"] += decalage
            for q in v["pieces"]:
                if "nom" in q:
                    q["id"] = q.pop("nom")
            v["parties_mobiles"] = [dict(piece=m["piece"], type=m.get("type", m.get("animation"))) for m in v.get("parties_mobiles", [])]
            V.append(v)

# ---------------------------------------------------------------------------
doc = {
    "convention": {
        "axes": "avant=+X, haut=+Y, largeur=Z; origine au centre, roues sur y=0",
        "formes": "cube/cylindre/cone/sphere; taille=[x,y,z] dans le repère local de la pièce avant rotation",
        "cylindre": "axe par défaut = Y; rotation [90,0,0] le couche sur Z (roues)",
        "cone": "pointe vers +Y par défaut; rotation [0,0,-90] -> pointe vers +X",
        "rotation_deg": "[x,y,z] en degrés, ordre X puis Y puis Z autour du centre de la pièce",
    },
    "vehicules": V,
}
with open(os.path.join(ROOT, "games/traversee/data/vehicules.json"), "w", encoding="utf-8") as f:
    json.dump(doc, f, ensure_ascii=False, indent=1)

SHAPE_RS = {"cube": "Shape::Cube", "cylindre": "Shape::Cyl", "cone": "Shape::Cone", "sphere": "Shape::Sphere"}
MAT_RS = {"peinture": "Mat::Peinture", "vitre": "Mat::Vitre", "sombre": "Mat::Sombre", "pneu": "Mat::Pneu", "enjoliveur": "Mat::Enjoliveur",
          "phare": "Mat::Phare", "feu_arriere": "Mat::FeuArriere", "lumiere_rouge": "Mat::LumiereRouge", "lumiere_bleue": "Mat::LumiereBleue",
          "blanc": "Mat::Blanc", "noir": "Mat::Noir", "couleur_2": "Mat::C2", "couleur_3": "Mat::C3", "bulle": "Mat::Bulle"}
ANIM_RS = {"clignote": ("Blink", 0.0), "clignote_alterne": ("BlinkAlt", 0.0), "clignote_clin_doeil": ("Wink", 0.0), "pulse": ("Pulse", 0.0),
           "longueur_pulse": ("LengthPulse", 0.0), "vibre": ("Vibrate", 0.012), "tremble": ("Tremble", 0.0), "oscille": ("Sway", 0.0),
           "oscille_vertical": ("Bob", 0.04), "flotte_verticale": ("Float", 0.03), "translation_z_ping_pong": ("PingPongZ", 0.22),
           "derive_arriere": ("DriftBack", 0.0), "derive_vertical_boucle": ("DriftUp", 0.0), "balance": ("Pendulum", 0.0),
           "decalage_x_ping_pong": ("PingPongX", 0.08), "rotation_y": ("SpinY", 0.0),
           "sautille": ("Hop", 0.04), "embardee_laterale": ("Weave", 0.05), "penche_dans_les_virages": ("Lean", 0.0),
           "serpente_phase": ("Snake", 0.0), "clignote_lent": ("BlinkSlow", 0.0), "flotte_vent": ("Flutter", 0.0),
           "oscille_tete": ("Sway", 0.0), "bat_au_vent": ("Flap", 0.0), "tombe_periodiquement": ("Fall", 0.0),
           "rotation_z": ("SpinLocal", 0.0), "derive_haut": ("DriftUp", 0.0), "rebond": ("Bob", 0.03), "fumee": ("DriftUp", 0.0), "derive_arriere_boucle": ("DriftBack", 0.0)}


def parse_anim(t):
    amp = None
    base = t
    parts = t.rsplit("_", 1)
    if len(parts) == 2:
        try:
            amp = float(parts[1])
            base = parts[0]
        except ValueError:
            pass
    kind, default = ANIM_RS[base]
    return kind, (amp if amp is not None else default)


def f(x):
    s = repr(float(x))
    return s


def rgb(h):
    h = h.lstrip("#")
    return "[" + ", ".join(str(int(h[i:i + 2], 16)) for i in (0, 2, 4)) + "]"


rs = ["// Généré par tools/gen-vehicules.py à partir de data/vehicules.json : ne pas modifier à la main.",
      "//! Véhicules décrits en pièces simples (cube, cylindre, cône, sphère).",
      "use game_common::tx;",
      "",
      "use crate::extra::{Anim, AnimKind, Gen, Mat, Piece, Shape};",
      ""]
for i, v in enumerate(V):
    rs.append(f"const PIECES_{i}: &[Piece] = &[")
    for q in v["pieces"]:
        pid = f'Some("{q["id"]}")' if "id" in q else "None"
        rs.append(f"    Piece {{ shape: {SHAPE_RS[q['forme']]}, pos: [{', '.join(f(x) for x in q['position'])}], size: [{', '.join(f(x) for x in q['taille'])}], rot: [{', '.join(f(x) for x in q['rotation_deg'])}], mat: {MAT_RS[q['matiere']]}, id: {pid} }},")
    rs.append("];")
    rs.append(f"const ANIMS_{i}: &[Anim] = &[")
    for m in v["parties_mobiles"]:
        kind, amp = parse_anim(m["type"])
        rs.append(f'    Anim {{ target: "{m["piece"]}", kind: AnimKind::{kind}, amp: {f(amp)} }},')
    rs.append("];")
    rs.append("")
rs.append(f"pub const GEN: [Gen; {len(V)}] = [")
for i, v in enumerate(V):
    rs.append(f'    Gen {{ len: {f(v["longueur"])}, points: {v["points_deblocage"]}, class: "{v["classe"]}", c2: {rgb(v.get("couleur_2", "#888888"))}, c3: {rgb(v.get("couleur_3", "#888888"))}, pieces: PIECES_{i}, anims: ANIMS_{i} }},')
rs.append("];")
rs.append("")
rs.append("/// Le nom du véhicule dans la langue en cours.")
rs.append("pub fn name(i: usize) -> &'static str {")
rs.append("    match i {")
for i, v in enumerate(V):
    n = v["noms"]
    q = lambda s: '"' + s.replace('"', '\\"') + '"'
    rs.append(f"        {i} => tx!({q(n['fr'])}, {q(n['en'])}, {q(n['de'])}, {q(n['it'])}, {q(n['es'])}).t(),")
rs.append('        _ => "?",')
rs.append("    }")
rs.append("}")
with open(os.path.join(ROOT, "games/traversee/src/gen_vehicles.rs"), "w", encoding="utf-8") as fh:
    fh.write("\n".join(rs) + "\n")
print(len(V), "véhicules,", sum(len(v["pieces"]) for v in V), "pièces")
