#!/usr/bin/env python3
# Commit & Conquer — licence GPL-3.0-or-later.
#
# Le singe géant et le papillon géant de Nippon Kaboom, décrits en pièces
# (mêmes conventions que les véhicules : avant = +X, haut = +Y, largeur = Z).
# Écrit games/traversee/data/import/nippon_extra_animaux.json
import json, os

OUT = os.path.join(os.path.dirname(os.path.abspath(__file__)), "../games/traversee/data/import/nippon_extra_animaux.json")
SH = {"c": "cube", "y": "cylindre", "n": "cone", "s": "sphere"}


def P(shape, pos, size, mat, nom=None, rot=(0, 0, 0)):
    d = {"forme": SH[shape], "position": list(pos), "taille": list(size), "rotation_deg": list(rot), "matiere": mat}
    if nom:
        d["nom"] = nom
    return d


def N(fr, en, de, es, it):
    return dict(fr=fr, en=en, de=de, es=es, it=it)


# --- Le singe géant : fourrure sombre (couleur_2), visage et poitrail clairs (couleur_3)
ape = [
    P("c", (0, 1.0, 0), (0.9, 1.0, 0.9), "couleur_2", "corps"),
    P("c", (0.455, 0.95, 0), (0.03, 0.6, 0.6), "couleur_3"),
    P("c", (0.15, 0.5, 0), (0.6, 0.3, 0.7), "couleur_2"),
    P("s", (0.45, 1.68, 0), (0.64, 0.6, 0.66), "couleur_2"),
    P("s", (0.68, 1.62, 0), (0.34, 0.36, 0.44), "couleur_3"),
    P("c", (0.73, 1.83, 0), (0.12, 0.07, 0.56), "couleur_2"),
    P("s", (0.8, 1.72, 0.13), (0.1, 0.1, 0.1), "blanc"), P("s", (0.84, 1.72, 0.13), (0.05, 0.05, 0.05), "noir"),
    P("s", (0.8, 1.72, -0.13), (0.1, 0.1, 0.1), "blanc"), P("s", (0.84, 1.72, -0.13), (0.05, 0.05, 0.05), "noir"),
    P("c", (0.87, 1.5, 0), (0.02, 0.05, 0.28), "noir"),
    P("s", (0.88, 1.62, 0.06), (0.04, 0.05, 0.05), "noir"), P("s", (0.88, 1.62, -0.06), (0.04, 0.05, 0.05), "noir"),
    P("s", (0.38, 1.72, 0.36), (0.16, 0.16, 0.12), "couleur_3"), P("s", (0.38, 1.72, -0.36), (0.16, 0.16, 0.12), "couleur_3"),
    P("s", (0, 1.42, 0.58), (0.5, 0.42, 0.42), "couleur_2"), P("s", (0, 1.42, -0.58), (0.5, 0.42, 0.42), "couleur_2"),
]
for nom, z, in (("bras_g", 0.7), ("bras_d", -0.7)):
    ape += [P("y", (0, 0.98, z), (0.3, 1.0, 0.3), "couleur_2", nom), P("s", (0.04, 0.46, z), (0.38, 0.34, 0.38), "couleur_2", nom)]
for nom, z in (("patte_g", 0.3), ("patte_d", -0.3)):
    ape += [P("y", (-0.05, 0.3, z), (0.36, 0.6, 0.36), "couleur_2", nom), P("c", (0.1, 0.05, z), (0.42, 0.1, 0.32), "noir", nom)]
ape_mobile = [
    dict(piece="corps", animation="rebond", amplitude=0.03),
    dict(piece="patte_g", animation="pendule", phase=0.0, axe="z", pivot="haut"),
    dict(piece="patte_d", animation="pendule", phase=0.5, axe="z", pivot="haut"),
    dict(piece="bras_g", animation="pendule", phase=0.5, axe="z", pivot="haut"),
    dict(piece="bras_d", animation="pendule", phase=0.0, axe="z", pivot="haut"),
]

# --- Le papillon géant : ailes bleues à pois, qui battent (comme un oiseau, il survole la carte)
bf = [
    P("s", (0, 0.9, 0), (0.78, 0.24, 0.24), "couleur_3", "corps"),
    P("s", (0.42, 0.93, 0), (0.24, 0.24, 0.24), "couleur_3"),
    P("s", (0.52, 0.98, 0.07), (0.07, 0.07, 0.07), "blanc"), P("s", (0.52, 0.98, -0.07), (0.07, 0.07, 0.07), "blanc"),
    P("y", (0.6, 1.12, 0.1), (0.03, 0.4, 0.03), "couleur_3", None, (0, 0, -50)), P("y", (0.6, 1.12, -0.1), (0.03, 0.4, 0.03), "couleur_3", None, (0, 0, -50)),
]
for nom, s in (("aile_g", 1), ("aile_d", -1)):
    bf += [
        P("c", (0.1, 0.9, 0.7 * s), (0.95, 0.04, 1.1), "couleur_2", nom),
        P("c", (-0.35, 0.9, 0.62 * s), (0.65, 0.04, 0.8), "couleur_2", nom),
        P("s", (0.1, 0.93, 0.95 * s), (0.3, 0.03, 0.3), "couleur_3", nom),
        P("s", (-0.35, 0.93, 0.85 * s), (0.22, 0.03, 0.22), "couleur_3", nom),
        P("c", (0.62, 0.9, 0.62 * s), (0.12, 0.045, 0.9), "couleur_3", nom),
    ]
bf_mobile = [
    dict(piece="corps", animation="rebond", amplitude=0.04),
    dict(piece="aile_g", animation="pendule", phase=0.0, axe="x", pivot="base"),
    dict(piece="aile_d", animation="pendule", phase=0.0, axe="x", pivot="base"),
]

animaux = [
    dict(id="singe_geant", univers="nippon_kaboom", classe="grand", noms=N("Kong-Kong le Costaud", "Big Bongo", "Riesen-Bongo", "Gran Bongo", "Gran Bongo"),
         longueur=1.9, largeur=1.8, comportement="traverse", vitesse=2.0, dangereux=True, points_deblocage=2200,
         couleur_2="#5a3d2b", couleur_3="#d2a679", pieces=ape, parties_mobiles=ape_mobile),
    dict(id="papillon_geant", univers="nippon_kaboom", classe="volant", noms=N("Papillon Colossal", "Colossal Butterfly", "Riesenfalter", "Mariposa Colosal", "Farfalla Colossale"),
         longueur=1.9, largeur=2.4, comportement="vole_en_ligne", vitesse=3.5, dangereux=True, points_deblocage=3000,
         couleur_2="#3b8cf0", couleur_3="#1f2a4a", pieces=bf, parties_mobiles=bf_mobile),
]
with open(OUT, "w", encoding="utf-8") as f:
    json.dump(dict(animaux=animaux), f, ensure_ascii=False, indent=1)
print(len(ape), "pièces (singe),", len(bf), "pièces (papillon)")
