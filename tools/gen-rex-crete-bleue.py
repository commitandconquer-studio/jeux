#!/usr/bin/env python3
# Commit & Conquer — licence GPL-3.0-or-later.
#
# Le tyrannosaure gris à crête bleue de Nippon Kaboom, plus détaillé que le modèle
# d'origine (tête avec mâchoire et dents, bras minuscules à griffes, pattes en trois
# morceaux, queue en tronçons, crête en lames). Avant = +X, haut = +Y, largeur = Z.
# Écrit games/traversee/data/import/rex_crete_bleue_detaille.json
import json, os

HERE = os.path.dirname(os.path.abspath(__file__))
SRC = os.path.join(HERE, "../games/traversee/data/import/rex_crete_bleue.json")
OUT = os.path.join(HERE, "../games/traversee/data/import/rex_crete_bleue_detaille.json")
SH = {"c": "cube", "y": "cylindre", "n": "cone", "s": "sphere"}
K = 0.8  # échelle d'ensemble : 4,7 de long, un peu plus de 2 de haut


def P(shape, pos, size, mat, nom=None, rot=(0, 0, 0)):
    d = {"forme": SH[shape], "position": [round(v * K, 3) for v in pos], "taille": [round(v * K, 3) for v in size], "rotation_deg": list(rot), "matiere": mat}
    if nom:
        d["nom"] = nom
    return d


r = []
# Corps : torse, ventre clair, hanches, cou
r += [P("s", (0.2, 1.45, 0), (1.7, 1.2, 1.1), "couleur_2", "corps"), P("s", (0.3, 1.2, 0), (1.3, 0.75, 0.96), "blanc", "corps"),
      P("s", (-0.6, 1.5, 0), (1.0, 0.95, 1.0), "couleur_2", "corps"), P("s", (1.05, 1.9, 0), (0.8, 0.8, 0.7), "couleur_2", "corps")]
# Tête : crâne, museau, mâchoire, dents, œil, narines
r += [P("s", (1.55, 2.15, 0), (1.0, 0.68, 0.78), "couleur_2", "corps"), P("s", (2.0, 2.1, 0), (0.95, 0.44, 0.58), "couleur_2", "corps"),
      P("s", (2.38, 2.1, 0), (0.3, 0.38, 0.5), "couleur_2", "corps")]
r += [P("c", (1.95, 1.84, 0), (0.85, 0.16, 0.46), "couleur_2", "machoire"), P("c", (2.32, 1.86, 0), (0.1, 0.18, 0.4), "couleur_2", "machoire")]
for x in (1.7, 1.9, 2.1, 2.28):
    for z in (0.19, -0.19):
        r.append(P("n", (x, 1.9, z), (0.07, 0.15, 0.07), "blanc", "corps", (180, 0, 0)))
for x in (1.78, 2.0, 2.2):
    for z in (0.17, -0.17):
        r.append(P("n", (x, 1.95, z), (0.06, 0.12, 0.06), "blanc", "machoire"))
for z in (0.3, -0.3):
    r += [P("s", (1.78, 2.34, z * 1.2), (0.22, 0.22, 0.12), "blanc", "corps"), P("s", (1.86, 2.35, z * 1.26), (0.1, 0.1, 0.07), "noir", "corps")]
for z in (0.12, -0.12):
    r.append(P("s", (2.4, 2.22, z), (0.06, 0.06, 0.06), "noir", "corps"))
# Crête bleue : lames de la tête jusqu'au bout de la queue
spine = [(1.65, 2.62, 0.5), (1.4, 2.6, 0.45), (1.12, 2.5, 0.42), (0.8, 2.25, 0.4), (0.45, 2.15, 0.36), (0.1, 2.12, 0.34),
         (-0.3, 2.12, 0.32), (-0.7, 2.1, 0.3), (-1.1, 1.92, 0.27), (-1.55, 1.82, 0.24), (-2.0, 1.72, 0.2), (-2.5, 1.62, 0.16)]
for i, (x, y, h) in enumerate(spine):
    r.append(P("n", (x, y + h * 0.9, 0), (0.34, h * 2.0, 0.12), "couleur_3", "crete_pointe" if i == 0 else None, (0, 0, 16)))
# Bras minuscules et griffes
for nom, z, ph in (("bras_g", 0.5, 0.0), ("bras_d", -0.5, 0.5)):
    r += [P("c", (0.95, 1.3, z), (0.4, 0.16, 0.15), "couleur_2", nom), P("n", (1.2, 1.24, z + 0.04), (0.05, 0.15, 0.05), "blanc", nom, (0, 0, -90)),
          P("n", (1.2, 1.24, z - 0.04), (0.05, 0.15, 0.05), "blanc", nom, (0, 0, -90))]
# Pattes : cuisse, jambe, pied, orteils
for nom, z in (("patte_g", 0.5), ("patte_d", -0.5)):
    r += [P("s", (-0.4, 1.0, z), (0.85, 1.05, 0.5), "couleur_2", nom), P("y", (-0.3, 0.45, z), (0.3, 0.8, 0.3), "couleur_2", nom),
          P("c", (-0.12, 0.07, z), (0.8, 0.14, 0.42), "sombre", nom)]
    for dz in (-0.14, 0.0, 0.14):
        r.append(P("n", (0.36, 0.08, z + dz), (0.08, 0.22, 0.08), "blanc", nom, (0, 0, -90)))
# Queue en tronçons
for x, y, w in ((-1.3, 1.45, 0.72), (-1.95, 1.4, 0.58), (-2.55, 1.34, 0.45), (-3.05, 1.3, 0.34), (-3.45, 1.28, 0.22)):
    r.append(P("s", (x, y, 0), (w * 1.35, w, w), "couleur_2", "queue"))

src = json.load(open(SRC, encoding="utf-8"))["animaux"][0]
src.update(pieces=r, longueur=4.7, largeur=1.5,
           parties_mobiles=[dict(piece="corps", animation="rebond", amplitude=0.02),
                            dict(piece="patte_g", animation="pendule", phase=0.0, axe="z", pivot="haut"),
                            dict(piece="patte_d", animation="pendule", phase=0.5, axe="z", pivot="haut"),
                            dict(piece="bras_g", animation="pendule", phase=0.0, axe="z", pivot="haut"),
                            dict(piece="bras_d", animation="pendule", phase=0.5, axe="z", pivot="haut"),
                            dict(piece="machoire", animation="balance", axe="z"),
                            dict(piece="queue", animation="balance", axe="y"),
                            dict(piece="crete_pointe", animation="pulse")])
json.dump(dict(animaux=[src]), open(OUT, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
print(len(r), "pièces")
