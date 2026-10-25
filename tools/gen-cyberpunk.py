#!/usr/bin/env python3
# Commit & Conquer — licence GPL-3.0-or-later.
#
# Le monde « Néo-Croâ » de Traversée (ville futuriste) : autoroute néon, canaux
# holographiques, monorail, robots, drone, grenouille à visière. Avant = +X, haut = +Y,
# largeur = Z. Écrit dans games/traversee/data/import/ :
#   cyber.json (monde, animaux, héros)   cyber_vehicules.json (véhicules)
import json, os, math

HERE = os.path.dirname(os.path.abspath(__file__))
IMP = os.path.join(HERE, "../games/traversee/data/import")
SH = {"c": "cube", "y": "cylindre", "n": "cone", "s": "sphere"}


def P(shape, pos, size, mat, nom=None, rot=(0, 0, 0)):
    d = {"forme": SH[shape], "position": [round(v, 3) for v in pos], "taille": [round(v, 3) for v in size], "rotation_deg": list(rot), "matiere": mat}
    if nom:
        d["nom"] = nom
    return d


def N(fr, en, de, es, it):
    return dict(fr=fr, en=en, de=de, es=es, it=it)


def model(id, nom, c2, c3, pieces, mobiles=(), **extra):
    return dict(id=id, nom=nom, couleur_2=c2, couleur_3=c3, pieces=pieces, parties_mobiles=list(mobiles), **extra)


def swing(nom, phase):
    return dict(piece=nom, animation="pendule", phase=phase, axe="z", pivot="haut")


# =============================================================== DÉCOR
lampadaire = model("lampadaire", N("Lampadaire Néon", "Neon Lamppost", "Neon-Laterne", "Farola de Neón", "Lampione al Neon"), "#2d3142", "#37e6ff", [
    P("c", (0, 0.04, 0), (0.45, 0.08, 0.45), "couleur_2"), P("y", (0, 0.6, 0), (0.1, 1.2, 0.1), "couleur_2"), P("y", (0, 1.25, 0), (0.5, 0.05, 0.5), "lumiere_bleue", "anneau"),
    P("s", (0, 1.32, 0), (0.18, 0.12, 0.18), "phare")], [dict(piece="anneau", animation="pulse")], roll=1, taille_case=1)
distributeur = model("distributeur_neon", N("Distributeur Néon", "Neon Vending Machine", "Neon-Automat", "Máquina de Neón", "Distributore al Neon"), "#3a3f5a", "#ff3fa8", [
    P("c", (0, 0.55, 0), (0.7, 1.1, 0.6), "couleur_2"), P("c", (0.0, 0.7, 0.31), (0.5, 0.5, 0.02), "lumiere_bleue", "ecran"), P("c", (0.0, 0.25, 0.31), (0.5, 0.12, 0.02), "couleur_3"),
    P("c", (0.0, 1.12, 0), (0.74, 0.04, 0.64), "sombre"), P("c", (0.36, 0.55, 0.0), (0.02, 1.0, 0.5), "couleur_3")], [dict(piece="ecran", animation="clignote_lent")], roll=2, taille_case=1)
conteneur = model("conteneur", N("Conteneur Louche", "Dodgy Container", "Verdächtiger Container", "Contenedor Sospechoso", "Container Sospetto"), "#2d3142", "#ff9a3f", [
    P("c", (0, 0.4, 0), (1.0, 0.8, 0.7), "couleur_2"), P("c", (0, 0.4, 0.36), (0.9, 0.6, 0.02), "couleur_3"), P("c", (0, 0.4, -0.36), (0.9, 0.6, 0.02), "couleur_3"),
    P("c", (0, 0.82, 0), (1.04, 0.04, 0.74), "sombre"), P("c", (0.2, 0.2, 0.36), (0.05, 0.05, 0.03), "lumiere_rouge", "voyant")], [dict(piece="voyant", animation="clignote")], roll=3, taille_case=1)
borne = model("borne_holo", N("Borne Holographique", "Holo Kiosk", "Holo-Säule", "Tótem Holográfico", "Colonna Olografica"), "#3a3f5a", "#37e6ff", [
    P("y", (0, 0.35, 0), (0.4, 0.7, 0.4), "couleur_2"), P("y", (0, 0.72, 0), (0.46, 0.05, 0.46), "couleur_3"), P("s", (0, 1.05, 0), (0.46, 0.46, 0.46), "bulle", "holo"),
    P("s", (0, 1.05, 0), (0.18, 0.18, 0.18), "lumiere_bleue", "holo")], [dict(piece="holo", animation="pulse")], roll=4, taille_case=1)
obstacles = [lampadaire, distributeur, conteneur, borne]


def tour(h, c2, c3, nom):
    p = [P("c", (0, h / 2, 0), (1.4, h, 1.2), "couleur_2"), P("c", (0, h + 0.05, 0), (1.5, 0.1, 1.3), "sombre")]
    k = 0
    for y in [0.7 + 0.55 * i for i in range(int((h - 0.8) / 0.55) + 1)]:
        for x in (-0.45, 0.0, 0.45):
            p.append(P("c", (x, y, 0.61), (0.28, 0.3, 0.03), "phare" if (k % 3 == 0) else "couleur_3"))
        k += 1
    p += [P("y", (0.4, h + 0.4, 0), (0.05, 0.6, 0.05), "sombre"), P("s", (0.4, h + 0.72, 0), (0.1, 0.1, 0.1), "lumiere_rouge", "balise"), P("c", (0, 0.05, 0), (1.7, 0.1, 1.5), "sombre")]
    return model(nom, N("Gratte-Ciel Néon", "Neon Skyscraper", "Neon-Wolkenkratzer", "Rascacielos de Neón", "Grattacielo al Neon"), c2, c3, p, [dict(piece="balise", animation="clignote")])


enseigne = model("enseigne_geante", N("Enseigne Géante", "Giant Sign", "Riesenschild", "Cartel Gigante", "Insegna Gigante"), "#1c1f2e", "#ff3fa8", [
    P("y", (0, 1.2, 0), (0.12, 2.4, 0.12), "sombre"), P("c", (0, 2.7, 0), (1.8, 1.0, 0.12), "couleur_2"), P("c", (0, 2.7, 0.07), (1.6, 0.8, 0.03), "couleur_3", "neon"),
    P("c", (0, 2.7, 0.09), (1.2, 0.1, 0.02), "lumiere_bleue", "neon"), P("c", (0, 0.05, 0), (0.8, 0.1, 0.8), "sombre")], [dict(piece="neon", animation="clignote_lent")])
bords = [tour(4.2, "#1c1f2e", "#37e6ff", "gratte_ciel_a"), tour(3.0, "#2a2438", "#ff3fa8", "gratte_ciel_b"), enseigne]
deco = [
    model("flaque_neon", N("Flaque Néon", "Neon Puddle", "Neonpfütze", "Charco de Neón", "Pozzanghera al Neon"), "#1a6fd0", "#37e6ff", [P("s", (0, 0.0, 0), (0.6, 0.02, 0.4), "couleur_2"), P("s", (0.05, 0.01, 0.02), (0.3, 0.02, 0.2), "couleur_3")]),
    model("cable", N("Câble", "Cable", "Kabel", "Cable", "Cavo"), "#2d3142", "#ff3fa8", [P("c", (0, 0.015, 0), (0.7, 0.03, 0.04), "couleur_2", None, (0, 20, 0)), P("c", (0.15, 0.015, 0.1), (0.4, 0.03, 0.04), "couleur_3", None, (0, -30, 0))]),
    model("canette", N("Canette", "Soda Can", "Dose", "Lata", "Lattina"), "#c8ccd8", "#ff3fa8", [P("y", (0, 0.07, 0), (0.08, 0.14, 0.08), "couleur_2"), P("y", (0, 0.07, 0), (0.085, 0.05, 0.085), "couleur_3")]),
    model("led_sol", N("Balise au Sol", "Floor Beacon", "Bodenlicht", "Baliza de Suelo", "Luce a Terra"), "#37e6ff", "#2d3142", [P("c", (0, 0.02, 0), (0.3, 0.04, 0.1), "couleur_2"), P("c", (0, 0.01, 0), (0.34, 0.02, 0.14), "couleur_3")]),
]


# =============================================================== CRÉATURES (robots, drones, mechas)
def creature(id, classe, noms, longueur, largeur, comportement, vitesse, dangereux, pts, c2, c3, pieces, mobiles):
    return dict(id=id, univers="cyberpunk", classe=classe, noms=noms, longueur=longueur, largeur=largeur, comportement=comportement, vitesse=vitesse,
                dangereux=dangereux, points_deblocage=pts, couleur_2=c2, couleur_3=c3, pieces=pieces, parties_mobiles=mobiles)


def eye_strip(x, y, w=0.3):
    return [P("c", (x, y, 0), (0.04, 0.07, w), "lumiere_bleue", "oeil")]


drone_l = [P("s", (0, 0.7, 0), (0.5, 0.26, 0.5), "couleur_2", "corps"), P("c", (0, 0.5, 0), (0.3, 0.22, 0.3), "couleur_3", "corps"), P("s", (0.22, 0.7, 0), (0.1, 0.1, 0.1), "lumiere_bleue", "oeil")]
for k, (x, z) in enumerate(((0.3, 0.3), (0.3, -0.3), (-0.3, 0.3), (-0.3, -0.3))):
    drone_l += [P("c", (x * 0.6, 0.72, z * 0.6), (0.4, 0.04, 0.06), "sombre", "corps", (0, 45 if x * z > 0 else -45, 0)), P("c", (x, 0.8, z), (0.4, 0.015, 0.06), "noir", f"helice{k}")]
drone_m = [dict(piece="corps", animation="rebond", amplitude=0.05), dict(piece="oeil", animation="clignote_alterne")] + [dict(piece=f"helice{k}", animation="rotation_groupe") for k in range(4)]
chat_r = [P("c", (0, 0.4, 0), (0.55, 0.28, 0.28), "couleur_2", "corps"), P("c", (0.34, 0.5, 0), (0.28, 0.26, 0.26), "couleur_2", "corps"),
          P("n", (0.3, 0.72, 0.09), (0.1, 0.16, 0.06), "couleur_3", "corps"), P("n", (0.3, 0.72, -0.09), (0.1, 0.16, 0.06), "couleur_3", "corps"),
          P("c", (0.48, 0.52, 0.0), (0.03, 0.07, 0.2), "lumiere_bleue", "oeil"), P("y", (-0.4, 0.62, 0), (0.05, 0.5, 0.05), "couleur_3", "queue", (0, 0, 25)), P("s", (-0.5, 0.88, 0), (0.08, 0.08, 0.08), "lumiere_rouge", "oeil")]
chat_m = [dict(piece="corps", animation="rebond", amplitude=0.03), dict(piece="queue", animation="balance", axe="y"), dict(piece="oeil", animation="clignote_alterne")]
for k, (x, z, ph) in enumerate(((0.2, 0.08, 0.0), (0.2, -0.08, 0.5), (-0.2, 0.08, 0.5), (-0.2, -0.08, 0.0))):
    chat_r += [P("y", (x, 0.2, z), (0.07, 0.4, 0.07), "couleur_3", f"patte{k}")]
    chat_m.append(swing(f"patte{k}", ph))
cyborg = [P("c", (0, 0.95, 0), (0.42, 0.75, 0.58), "couleur_2", "corps"), P("c", (0.2, 0.95, 0), (0.04, 0.5, 0.4), "couleur_3", "corps"), P("s", (0.08, 1.55, 0), (0.4, 0.4, 0.38), "couleur_2", "corps")] + eye_strip(0.28, 1.58, 0.3) + [
    P("y", (0.0, 1.9, 0.1), (0.03, 0.3, 0.03), "sombre", "corps"), P("s", (0.0, 2.08, 0.1), (0.07, 0.07, 0.07), "lumiere_rouge", "oeil"), P("c", (0, 0.5, 0), (0.4, 0.2, 0.52), "sombre", "corps")]
cy_m = [dict(piece="corps", animation="rebond", amplitude=0.03), dict(piece="oeil", animation="clignote_lent")]
for nom, z, ph in (("bras_g", 0.38, 0.5), ("bras_d", -0.38, 0.0)):
    cyborg += [P("y", (0, 1.0, z), (0.12, 0.7, 0.12), "couleur_3", nom), P("s", (0, 0.62, z), (0.18, 0.16, 0.18), "sombre", nom)]
    cy_m.append(swing(nom, ph))
for nom, z, ph in (("patte_g", 0.15, 0.0), ("patte_d", -0.15, 0.5)):
    cyborg += [P("y", (0, 0.32, z), (0.16, 0.64, 0.16), "couleur_3", nom), P("c", (0.09, 0.04, z), (0.3, 0.08, 0.18), "sombre", nom)]
    cy_m.append(swing(nom, ph))
chien = [P("c", (0, 0.62, 0), (0.95, 0.34, 0.4), "couleur_2", "corps"), P("c", (0.58, 0.8, 0), (0.4, 0.3, 0.32), "couleur_2", "corps"), P("c", (0.82, 0.75, 0), (0.28, 0.16, 0.24), "couleur_3", "corps"),
         P("c", (0.8, 0.86, 0.0), (0.2, 0.05, 0.3), "lumiere_bleue", "oeil"), P("y", (-0.55, 0.78, 0), (0.05, 0.4, 0.05), "sombre", "queue", (0, 0, 40)), P("c", (0.0, 0.8, 0.0), (0.6, 0.05, 0.3), "couleur_3", "corps"),
         P("n", (0.55, 1.05, 0.1), (0.07, 0.16, 0.05), "sombre", "corps"), P("n", (0.55, 1.05, -0.1), (0.07, 0.16, 0.05), "sombre", "corps")]
ch_m = [dict(piece="corps", animation="rebond", amplitude=0.04), dict(piece="oeil", animation="clignote_lent"), dict(piece="queue", animation="balance", axe="y")]
for k, (x, z, ph) in enumerate(((0.35, 0.16, 0.0), (0.35, -0.16, 0.5), (-0.35, 0.16, 0.5), (-0.35, -0.16, 0.0))):
    chien += [P("y", (x, 0.28, z), (0.1, 0.56, 0.1), "couleur_3", f"patte{k}"), P("c", (x + 0.04, 0.03, z), (0.16, 0.06, 0.12), "sombre", f"patte{k}")]
    ch_m.append(swing(f"patte{k}", ph))
hoverbike = [P("c", (0, 0.55, 0), (1.5, 0.2, 0.36), "couleur_2", "corps"), P("n", (0.85, 0.55, 0), (0.4, 0.3, 0.34), "couleur_2", "corps", (0, 0, -90)), P("c", (0.35, 0.78, 0), (0.4, 0.24, 0.3), "bulle", "corps"),
             P("c", (-0.5, 0.78, 0), (0.5, 0.04, 0.5), "couleur_3", "corps"), P("c", (-0.75, 0.55, 0), (0.1, 0.4, 0.8), "couleur_3", "corps"), P("c", (-0.78, 0.55, 0.0), (0.03, 0.08, 0.7), "lumiere_rouge", "feu"),
             P("c", (0.0, 0.38, 0.0), (1.3, 0.04, 0.5), "lumiere_bleue", "coussin"), P("s", (-0.2, 0.98, 0), (0.22, 0.26, 0.22), "couleur_3", "corps"), P("c", (0.0, 0.4, 0), (1.4, 0.08, 0.6), "sombre", "corps")]
hb_m = [dict(piece="corps", animation="flotte_verticale_0.04"), dict(piece="coussin", animation="pulse"), dict(piece="feu", animation="clignote")]
mecha = [P("c", (0, 1.7, 0), (0.9, 1.0, 1.2), "couleur_2", "corps"), P("c", (0.46, 1.8, 0), (0.04, 0.5, 0.8), "couleur_3", "corps"), P("c", (0.15, 2.4, 0), (0.55, 0.45, 0.55), "couleur_2", "corps"),
         P("c", (0.44, 2.45, 0), (0.04, 0.12, 0.48), "lumiere_rouge", "oeil"), P("y", (-0.1, 2.9, 0.2), (0.04, 0.4, 0.04), "sombre", "corps"), P("s", (-0.1, 3.12, 0.2), (0.1, 0.1, 0.1), "lumiere_bleue", "oeil"),
         P("c", (0.0, 1.05, 0), (0.8, 0.3, 1.0), "sombre", "corps"), P("c", (-0.55, 1.85, 0), (0.4, 0.7, 0.7), "couleur_3", "corps")]
me_m = [dict(piece="corps", animation="rebond", amplitude=0.02), dict(piece="oeil", animation="clignote_alterne")]
for nom, z, ph in (("bras_g", 0.78, 0.5), ("bras_d", -0.78, 0.0)):
    mecha += [P("y", (0, 1.7, z), (0.34, 1.3, 0.34), "couleur_3", nom), P("c", (0.2, 1.0, z), (0.5, 0.36, 0.4), "sombre", nom)]
    me_m.append(swing(nom, ph))
for nom, z, ph in (("patte_g", 0.36, 0.0), ("patte_d", -0.36, 0.5)):
    mecha += [P("y", (0, 0.5, z), (0.4, 1.0, 0.4), "couleur_3", nom), P("c", (0.2, 0.08, z), (0.9, 0.16, 0.56), "sombre", nom)]
    me_m.append(swing(nom, ph))
drone_g = [P("s", (0, 0.95, 0), (0.7, 0.3, 0.7), "couleur_2", "corps"), P("c", (0, 0.75, 0), (0.4, 0.2, 0.4), "couleur_3", "corps"), P("s", (0.32, 0.95, 0), (0.14, 0.14, 0.14), "lumiere_rouge", "oeil"),
           P("c", (0, 0.65, 0), (0.1, 0.1, 0.5), "lumiere_bleue", "oeil")]
for k, (x, z) in enumerate(((0.8, 0.8), (0.8, -0.8), (-0.8, 0.8), (-0.8, -0.8))):
    drone_g += [P("c", (x * 0.55, 0.97, z * 0.55), (1.1, 0.06, 0.1), "sombre", "corps", (0, 45 if x * z > 0 else -45, 0)), P("c", (x, 1.05, z), (0.8, 0.02, 0.1), "noir", f"helice{k}"), P("s", (x, 0.98, z), (0.12, 0.12, 0.12), "lumiere_bleue", "oeil")]
dg_m = [dict(piece="corps", animation="rebond", amplitude=0.05), dict(piece="oeil", animation="clignote_alterne")] + [dict(piece=f"helice{k}", animation="rotation_groupe") for k in range(4)]

animaux = [
    creature("drone_livreur", "petit", N("Drone Livreur", "Delivery Drone", "Lieferdrohne", "Dron Repartidor", "Drone Fattorino"), 0.8, 0.8, "traverse", 2.2, False, 300, "#c8ccd8", "#ff3fa8", drone_l, drone_m),
    creature("chat_robot", "petit", N("Chat Robot", "Robo Cat", "Roboterkatze", "Gato Robot", "Gatto Robot"), 1.0, 0.4, "traverse", 3.2, False, 500, "#8890a8", "#37e6ff", chat_r, chat_m),
    creature("cyborg", "moyen", N("Cyborg Pressé", "Hasty Cyborg", "Eiliger Cyborg", "Cíborg con Prisa", "Cyborg Frettoloso"), 0.9, 0.8, "traverse", 3.2, True, 900, "#3a3f5a", "#ff9a3f", cyborg, cy_m),
    creature("robot_chien", "moyen", N("Robot-Chien Pas Gentil", "Not-So-Friendly Robodog", "Unfreundlicher Roboterhund", "Perro Robot Antipático", "Cane Robot Scorbutico"), 1.6, 0.6, "traverse", 3.8, True, 1400, "#8890a8", "#ff3fa8", chien, ch_m),
    creature("hoverbike", "moyen", N("Moto Volante", "Hoverbike", "Schwebebike", "Moto Voladora", "Moto Volante"), 1.8, 0.7, "charge", 4.6, True, 1900, "#ff3fa8", "#2d3142", hoverbike, hb_m),
    creature("mecha", "grand", N("Mécha Colossal", "Colossal Mech", "Riesen-Mech", "Mecha Colosal", "Mecha Colossale"), 1.8, 2.0, "traverse", 1.6, True, 2600, "#3a3f5a", "#ff9a3f", mecha, me_m),
    creature("drone_geant", "volant", N("Drone Géant", "Giant Drone", "Riesendrohne", "Dron Gigante", "Drone Gigante"), 1.8, 1.8, "vole_en_ligne", 3.8, True, 3000, "#2d3142", "#37e6ff", drone_g, dg_m),
]

# =============================================================== HÉROS : grenouille à visière néon
src = json.load(open(os.path.join(IMP, "jurassic_croa.json"), encoding="utf-8"))["heros"]
hero_pieces = [p for p in src["pieces"] if p.get("nom") != "canne"]
visiere = [P("c", (0.3, 0.55, 0.0), (0.1, 0.1, 0.5), "lumiere_bleue", "visiere"), P("c", (0.2, 0.66, 0.0), (0.4, 0.05, 0.5), "couleur_3"),
           P("c", (0.0, 0.5, 0.27), (0.12, 0.2, 0.06), "couleur_3"), P("c", (0.0, 0.5, -0.27), (0.12, 0.2, 0.06), "couleur_3"),
           P("y", (0.0, 0.82, 0.15), (0.03, 0.3, 0.03), "sombre"), P("s", (0.0, 0.98, 0.15), (0.07, 0.07, 0.07), "lumiere_rouge", "balise")]
heros = dict(id="grenouille_cyber", univers="cyberpunk", noms=N("Grenouille Cyber", "Cyber Frog", "Cyber-Frosch", "Rana Cíber", "Rana Cyber"), longueur=0.7, largeur=0.6,
             couleur_2="#3a3f5a", couleur_3="#37e6ff", pieces=hero_pieces, parties_mobiles=[dict(piece="corps", animation="rebond", amplitude=0.02)],
             accessoire=dict(nom=N("Visière néon", "Neon visor", "Neon-Visier", "Visera de neón", "Visore al neon"), pieces=visiere, parties_mobiles=[dict(piece="balise", animation="clignote"), dict(piece="visiere", animation="pulse")]))


# =============================================================== MONORAIL
def wheel(x, z, r):
    return [P("y", (x, r, z), (r * 2, 0.06, r * 2), "sombre", None, (90, 0, 0))]


def tete():
    p = [P("c", (0, 0.65, 0), (1.9, 0.7, 0.8), "couleur_2"), P("n", (1.2, 0.65, 0), (0.7, 0.7, 0.8), "couleur_2", None, (0, 0, -90)), P("c", (0.7, 0.88, 0), (0.7, 0.3, 0.72), "bulle"),
         P("c", (0.0, 0.5, 0.405), (1.9, 0.06, 0.02), "lumiere_bleue", "bande"), P("c", (0.0, 0.5, -0.405), (1.9, 0.06, 0.02), "lumiere_bleue", "bande"),
         P("s", (1.5, 0.65, 0.0), (0.14, 0.14, 0.14), "phare"), P("c", (0.0, 0.25, 0), (2.4, 0.1, 0.6), "sombre")]
    return p, [dict(piece="bande", animation="pulse")]


def voiture(c):
    p = [P("c", (0, 0.65, 0), (1.9, 0.7, 0.8), "couleur_2"), P("c", (0, 0.9, 0.405), (1.5, 0.28, 0.02), "bulle"), P("c", (0, 0.9, -0.405), (1.5, 0.28, 0.02), "bulle"),
         P("c", (0.0, 0.5, 0.405), (1.9, 0.06, 0.02), "lumiere_bleue"), P("c", (0.0, 0.5, -0.405), (1.9, 0.06, 0.02), "lumiere_bleue"),
         P("c", (0, 1.02, 0), (1.95, 0.05, 0.85), "couleur_3"), P("c", (0.0, 0.25, 0), (2.1, 0.1, 0.6), "sombre")]
    if c:
        p.append(P("c", (0, 1.1, 0), (0.6, 0.08, 0.3), "lumiere_rouge", "voyant"))
    return p, [dict(piece="voyant", animation="clignote")] if c else []


tp, ta = tete()
v1, _ = voiture(False)
v2, va = voiture(True)
train = [
    dict(id="monorail_tete", longueur=2.9, couleur_2="#d8dce8", couleur_3="#ff3fa8", pieces=tp, parties_mobiles=ta),
    dict(id="monorail_queue", longueur=1.6, couleur_2="#d8dce8", couleur_3="#ff3fa8", pieces=voiture(False)[0][:5] + [P("c", (0.0, 0.25, 0), (1.5, 0.1, 0.6), "sombre")], parties_mobiles=[]),
    dict(id="monorail_a", longueur=2.1, couleur_2="#d8dce8", couleur_3="#37e6ff", pieces=v1, parties_mobiles=[]),
    dict(id="monorail_b", longueur=2.1, couleur_2="#aab0c4", couleur_3="#ff3fa8", pieces=v2, parties_mobiles=va),
]

monde = dict(id="cyberpunk", nom=N("Néo-Croâ", "Neo-Croak", "Neo-Quak", "Neo-Croa", "Neo-Gra"), points_deblocage=4200,
             ciel="#0b0d1a", brume="#1a1030", sol_voie_route="#171a26", marquage="#37e6ff", herbe_a="#363b52", herbe_b="#2f3348", eau="#1a6fd0", rondin="#2d3142",
             obstacles=obstacles, bords=bords, deco=deco, voie_speciale=dict(remplace="rail", description="Monorail magnétique.", pieces=[]),
             vehicules_du_monde=["aerotaxi", "camion_cyber", "bus_maglev", "muscle_neon"], animaux_du_monde=[a["id"] for a in animaux], train=train,
             musique=dict(tempo_bpm=118, gamme="phrygienne", instruments=["synthé", "basse pulsée", "boîte à rythmes", "arpège"], ambiance="Nuit néon, pluie sur le bitume."),
             **{"bannière": N("Bienvenue à Néo-Croâ ! Les passages piétons sont désactivés pour votre confort.", "Welcome to Neo-Croak! Crosswalks have been disabled for your comfort.",
                              "Willkommen in Neo-Quak! Fußgängerüberwege wurden zu Ihrem Komfort deaktiviert.", "¡Bienvenido a Neo-Croa! Los pasos de peatones están desactivados por tu comodidad.",
                              "Benvenuto a Neo-Gra! Le strisce pedonali sono disattivate per il tuo comfort.")})
json.dump(dict(monde=monde, animaux=animaux, heros=heros), open(os.path.join(IMP, "cyber.json"), "w", encoding="utf-8"), ensure_ascii=False, indent=1)


# =============================================================== VÉHICULES
def V(id, classe, longueur, largeur, noms, pts, c2, c3, pieces, mobiles):
    return dict(id=id, univers="cyberpunk", classe=classe, noms=noms, longueur=longueur, largeur=largeur, points_deblocage=pts, couleur_2=c2, couleur_3=c3, pieces=pieces, parties_mobiles=mobiles)


def coussins(xs, z):
    return [P("c", (x, 0.2, zz), (0.5, 0.05, 0.3), "lumiere_bleue", "coussin") for x in xs for zz in (z, -z)]


aerotaxi = [P("c", (0, 0.5, 0), (1.6, 0.28, 0.82), "couleur_2"), P("c", (-0.1, 0.78, 0), (0.9, 0.3, 0.74), "bulle"), P("c", (-0.1, 0.96, 0), (0.95, 0.05, 0.78), "couleur_2"),
            P("c", (0.4, 0.72, 0), (0.4, 0.24, 0.7), "couleur_2", None, (0, 0, -20)), P("c", (-0.1, 1.06, 0), (0.3, 0.1, 0.14), "phare", "enseigne"),
            P("c", (0.82, 0.5, 0.25), (0.04, 0.08, 0.14), "phare"), P("c", (0.82, 0.5, -0.25), (0.04, 0.08, 0.14), "phare"), P("c", (-0.82, 0.5, 0.25), (0.04, 0.08, 0.14), "lumiere_rouge"),
            P("c", (-0.82, 0.5, -0.25), (0.04, 0.08, 0.14), "lumiere_rouge"), P("c", (0.0, 0.33, 0), (1.6, 0.04, 0.6), "couleur_3")] + coussins([0.5, -0.5], 0.28)
camion_c = [P("c", (0, 0.3, 0), (2.8, 0.1, 0.7), "sombre"), P("c", (0.95, 0.65, 0), (0.9, 0.7, 0.86), "couleur_2"), P("c", (1.15, 0.9, 0), (0.45, 0.24, 0.78), "bulle"),
            P("c", (-0.4, 0.9, 0), (1.9, 1.1, 0.96), "couleur_3"), P("c", (-0.4, 0.9, 0.49), (1.7, 0.08, 0.02), "lumiere_bleue", "bande"), P("c", (-0.4, 0.9, -0.49), (1.7, 0.08, 0.02), "lumiere_bleue", "bande"),
            P("c", (1.46, 0.5, 0.28), (0.04, 0.08, 0.14), "phare"), P("c", (1.46, 0.5, -0.28), (0.04, 0.08, 0.14), "phare"), P("s", (-1.0, 1.55, 0), (0.12, 0.12, 0.12), "lumiere_rouge", "gyro")] + coussins([1.0, 0.0, -1.0], 0.3)
bus_m = [P("c", (0, 0.3, 0), (3.5, 0.1, 0.72), "sombre"), P("c", (0, 0.72, 0), (3.6, 0.8, 0.9), "couleur_2"), P("c", (0.0, 0.95, 0.455), (3.0, 0.3, 0.02), "bulle"), P("c", (0.0, 0.95, -0.455), (3.0, 0.3, 0.02), "bulle"),
         P("n", (1.9, 0.72, 0), (0.7, 0.8, 0.9), "couleur_2", None, (0, 0, -90)), P("c", (0.0, 1.15, 0), (3.5, 0.06, 0.84), "couleur_3"), P("c", (0.0, 0.55, 0.455), (3.6, 0.05, 0.02), "lumiere_bleue", "bande"),
         P("c", (0.0, 0.55, -0.455), (3.6, 0.05, 0.02), "lumiere_bleue", "bande"), P("c", (2.2, 0.8, 0.0), (0.04, 0.1, 0.5), "phare")] + coussins([1.2, -0.2, -1.4], 0.32)
muscle = [P("c", (0, 0.5, 0), (1.9, 0.3, 0.85), "couleur_2"), P("c", (-0.2, 0.8, 0), (0.9, 0.3, 0.74), "bulle"), P("c", (-0.2, 0.97, 0), (0.95, 0.05, 0.78), "couleur_2"),
          P("c", (0.7, 0.62, 0), (0.6, 0.12, 0.8), "couleur_2", None, (0, 0, -8)), P("c", (0.0, 0.62, 0.43), (1.9, 0.05, 0.02), "couleur_3", "bande"), P("c", (0.0, 0.62, -0.43), (1.9, 0.05, 0.02), "couleur_3", "bande"),
          P("c", (0.95, 0.5, 0.28), (0.04, 0.08, 0.16), "phare"), P("c", (0.95, 0.5, -0.28), (0.04, 0.08, 0.16), "phare"), P("c", (-0.95, 0.55, 0.0), (0.04, 0.08, 0.7), "lumiere_rouge"),
          P("c", (-0.8, 0.95, 0), (0.3, 0.05, 0.9), "couleur_3")] + coussins([0.6, -0.6], 0.3)
vehicules = [
    V("aerotaxi", "voiture", 1.8, 0.9, N("Aérotaxi", "Air Taxi", "Lufttaxi", "Aerotaxi", "Aerotaxi"), 0, "#ffd23f", "#2d3142", aerotaxi, [dict(piece="enseigne", animation="clignote"), dict(piece="coussin", animation="pulse"), dict(piece="*", animation="flotte_verticale_0.03")]),
    V("camion_cyber", "camion", 3.0, 1.0, N("Camion Autonome", "Self-Driving Truck", "Autonomer Laster", "Camión Autónomo", "Camion Autonomo"), 300, "#3a3f5a", "#d8dce8", camion_c, [dict(piece="gyro", animation="pulse"), dict(piece="bande", animation="pulse"), dict(piece="coussin", animation="pulse"), dict(piece="*", animation="flotte_verticale_0.03")]),
    V("bus_maglev", "bus", 3.7, 0.95, N("Bus Maglev", "Maglev Bus", "Maglev-Bus", "Autobús Maglev", "Bus Maglev"), 800, "#aab0c4", "#ff3fa8", bus_m, [dict(piece="bande", animation="pulse"), dict(piece="coussin", animation="pulse"), dict(piece="*", animation="flotte_verticale_0.03")]),
    V("muscle_neon", "voiture", 2.0, 0.9, N("Bolide Néon", "Neon Muscle Car", "Neon-Bolide", "Bólido de Neón", "Bolide al Neon"), 500, "#ff3fa8", "#37e6ff", muscle, [dict(piece="bande", animation="pulse"), dict(piece="coussin", animation="pulse"), dict(piece="*", animation="flotte_verticale_0.03")]),
]
json.dump(dict(vehicules=vehicules), open(os.path.join(IMP, "cyber_vehicules.json"), "w", encoding="utf-8"), ensure_ascii=False, indent=1)
print(len(obstacles), "obstacles,", len(animaux), "robots,", len(vehicules), "véhicules,", len(train), "voitures de monorail")
