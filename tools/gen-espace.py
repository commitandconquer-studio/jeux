#!/usr/bin/env python3
# Commit & Conquer — licence GPL-3.0-or-later.
#
# L'univers « Cosmos Croâ » de Traversée : station sur la Lune, vaisseaux, extraterrestres
# et grenouille astronaute à globe, décrits en pièces simples (cube, cylindre, cône, sphère).
# Avant = +X, haut = +Y, largeur = Z. Écrit dans games/traversee/data/import/ :
#   espace.json              (monde, animaux, héros)       espace_vehicules.json (véhicules)
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


def legs(zs, xs, y, h=0.5, w=0.14, foot="noir", base_id="patte"):
    """Pattes : un groupe par côté (pivot en haut), pieds noirs."""
    out = []
    for k, z in enumerate(zs):
        nom = f"{base_id}_{'g' if z > 0 else 'd'}{k // 2 if len(zs) > 2 else ''}"
        for x in xs:
            out += [P("y", (x, y, z), (w, h, w), "couleur_3", nom), P("c", (x + 0.05, 0.04, z), (0.2, 0.08, w * 1.4), foot, nom)]
    return out


def leg_anims(zs, n_x=1, base_id="patte"):
    out = []
    for k, z in enumerate(zs):
        nom = f"{base_id}_{'g' if z > 0 else 'd'}{k // 2 if len(zs) > 2 else ''}"
        out.append(dict(piece=nom, animation="pendule", phase=0.0 if (k % 2 == 0) else 0.5, axe="z", pivot="haut"))
    return out


# =============================================================== DÉCORS (obstacles : 1 case, hauteur ≤ 1,3)
obstacles = [
    model("rocher_lunaire", N("Astéroïde Posé", "Parked Asteroid", "Geparkter Asteroid", "Asteroide Aparcado", "Asteroide Parcheggiato"), "#9c9ea8", "#6a6c76", [
        P("s", (0, 0.3, 0), (0.95, 0.62, 0.8), "couleur_2"), P("s", (0.25, 0.55, 0.1), (0.55, 0.45, 0.5), "couleur_2"),
        P("s", (-0.28, 0.2, -0.2), (0.5, 0.4, 0.45), "couleur_3"), P("s", (0.3, 0.15, -0.3), (0.35, 0.3, 0.3), "couleur_3"),
        P("s", (-0.1, 0.5, -0.1), (0.3, 0.1, 0.3), "couleur_3")], roll=1, taille_case=1),
    model("cristal_cosmique", N("Cristal Qui Brille", "Shiny Crystal", "Leuchtkristall", "Cristal Brillante", "Cristallo Splendente"), "#a46bff", "#5ff3ff", [
        P("n", (0, 0.5, 0), (0.34, 1.0, 0.34), "couleur_2"), P("n", (0.25, 0.35, 0.1), (0.24, 0.7, 0.24), "couleur_3", None, (0, 0, -18)),
        P("n", (-0.25, 0.3, -0.05), (0.22, 0.6, 0.22), "couleur_2", None, (0, 0, 20)), P("n", (0.05, 0.28, 0.3), (0.2, 0.55, 0.2), "couleur_3", None, (18, 0, 0)),
        P("s", (0, 0.06, 0), (0.7, 0.14, 0.7), "sombre"), P("s", (0, 1.04, 0), (0.08, 0.08, 0.08), "lumiere_bleue", "eclat")],
          [dict(piece="eclat", animation="clignote")], roll=2, taille_case=1),
    model("parabole", N("Parabole Qui Écoute", "Listening Dish", "Lauschschüssel", "Parabólica Curiosa", "Parabola Curiosa"), "#e8ecf4", "#8890a8", [
        P("y", (0, 0.35, 0), (0.12, 0.7, 0.12), "couleur_3"), P("c", (0, 0.04, 0), (0.6, 0.08, 0.6), "sombre"),
        P("y", (0.12, 0.85, 0), (0.7, 0.1, 0.7), "couleur_2", None, (0, 0, 35)), P("n", (0.25, 1.03, 0), (0.06, 0.4, 0.06), "couleur_3", None, (0, 0, -55)),
        P("s", (0.4, 1.14, 0), (0.1, 0.1, 0.1), "lumiere_rouge", "voyant")], [dict(piece="voyant", animation="clignote")], roll=3, taille_case=1),
    model("module_habitat", N("Module Douillet", "Cosy Module", "Gemütliches Modul", "Módulo Acogedor", "Modulo Accogliente"), "#f4f6fa", "#ff8a2a", [
        P("c", (0, 0.12, 0), (0.95, 0.24, 0.95), "sombre"), P("c", (0, 0.4, 0), (0.8, 0.4, 0.8), "couleur_2"),
        P("s", (0, 0.62, 0), (0.78, 0.5, 0.78), "bulle"), P("c", (0.405, 0.38, 0), (0.02, 0.3, 0.26), "couleur_3"),
        P("y", (0, 1.0, 0), (0.03, 0.3, 0.03), "sombre"), P("s", (0, 1.18, 0), (0.08, 0.08, 0.08), "lumiere_rouge", "balise")],
          [dict(piece="balise", animation="clignote")], roll=4, taille_case=1),
    model("panneau_solaire", N("Panneau Sans Soleil", "Sunless Panel", "Sonnenloses Paneel", "Panel Sin Sol", "Pannello Senza Sole"), "#2a4aa8", "#d0d6e4", [
        P("c", (0, 0.04, 0), (0.5, 0.08, 0.5), "sombre"), P("y", (0, 0.4, 0), (0.08, 0.8, 0.08), "couleur_3"),
        P("c", (0, 0.88, 0), (0.95, 0.05, 0.62), "couleur_2", None, (0, 0, 14)), P("c", (0, 0.905, 0), (0.9, 0.012, 0.03), "couleur_3", None, (0, 0, 14)),
        P("c", (0, 0.905, 0.16), (0.9, 0.012, 0.03), "couleur_3", None, (0, 0, 14)), P("c", (0, 0.905, -0.16), (0.9, 0.012, 0.03), "couleur_3", None, (0, 0, 14))],
          roll=5, taille_case=1),
    model("mini_fusee", N("Fusée de Poche", "Pocket Rocket", "Taschenrakete", "Cohete de Bolsillo", "Razzo Tascabile"), "#f4f6fa", "#e8433a", [
        P("y", (0, 0.55, 0), (0.42, 0.8, 0.42), "couleur_2"), P("n", (0, 1.15, 0), (0.42, 0.45, 0.42), "couleur_3"), P("s", (0.2, 0.7, 0), (0.12, 0.12, 0.06), "bulle"),
        P("c", (0.27, 0.35, 0), (0.22, 0.4, 0.04), "couleur_3"), P("c", (-0.27, 0.35, 0), (0.22, 0.4, 0.04), "couleur_3"),
        P("c", (0, 0.35, 0.27), (0.04, 0.4, 0.22), "couleur_3"), P("c", (0, 0.35, -0.27), (0.04, 0.4, 0.22), "couleur_3"),
        P("c", (0, 0.04, 0), (0.7, 0.08, 0.7), "sombre"), P("n", (0, 0.12, 0), (0.22, 0.3, 0.22), "lumiere_rouge", "flamme", (180, 0, 0))],
          [dict(piece="flamme", animation="pulse")], roll=6, taille_case=1),
]
# =============================================================== BORDS (grands décors, hors zone jouable)
bords = [
    model("grande_fusee", N("Fusée Pas Encore Partie", "Rocket Not Yet Gone", "Noch-nicht-gestartete Rakete", "Cohete Que No Despega", "Razzo Non Ancora Partito"), "#f4f6fa", "#e8433a", [
        P("y", (0, 1.6, 0), (0.9, 3.2, 0.9), "couleur_2"), P("n", (0, 3.6, 0), (0.9, 1.0, 0.9), "couleur_3"), P("y", (0, 2.4, 0), (0.92, 0.12, 0.92), "couleur_3"),
        P("s", (0.45, 2.7, 0), (0.3, 0.3, 0.15), "bulle"), P("c", (0.7, 0.7, 0), (0.7, 1.4, 0.08), "couleur_3"), P("c", (-0.7, 0.7, 0), (0.7, 1.4, 0.08), "couleur_3"),
        P("c", (0, 0.7, 0.7), (0.08, 1.4, 0.7), "couleur_3"), P("c", (0, 0.7, -0.7), (0.08, 1.4, 0.7), "couleur_3"),
        P("c", (0, 0.06, 0), (1.8, 0.12, 1.8), "sombre"), P("n", (0, 0.35, 0), (0.55, 0.6, 0.55), "lumiere_rouge", "flamme", (180, 0, 0))],
          [dict(piece="flamme", animation="pulse")]),
    model("planete_anneau", N("Planète à Anneau Fâchée", "Cross Ringed Planet", "Verärgerter Ringplanet", "Planeta Anillado Enfadado", "Pianeta Anellato Arrabbiato"), "#ff9a4a", "#ffe0a0", [
        P("s", (0, 3.0, 0), (2.4, 2.4, 2.4), "couleur_2"), P("y", (0, 3.0, 0), (4.0, 0.06, 4.0), "couleur_3", None, (0, 0, 22)),
        P("y", (0, 3.0, 0), (3.2, 0.08, 3.2), "couleur_2", None, (0, 0, 22)), P("c", (0, 3.35, 0.3), (2.3, 0.1, 0.1), "couleur_3", None, (0, 0, 0)),
        P("y", (0, 0.9, 0), (0.1, 1.8, 0.1), "sombre"), P("c", (0, 0.05, 0), (1.0, 0.1, 1.0), "sombre")]),
    model("tour_radar", N("Tour de Contrôle Distraite", "Absent-Minded Control Tower", "Zerstreuter Kontrollturm", "Torre de Control Despistada", "Torre di Controllo Distratta"), "#e8ecf4", "#37e6ff", [
        P("y", (0, 1.2, 0), (0.5, 2.4, 0.5), "couleur_2"), P("c", (0, 2.5, 0), (1.1, 0.5, 1.1), "couleur_2"), P("c", (0, 2.5, 0.56), (1.0, 0.3, 0.02), "couleur_3"),
        P("c", (0, 2.5, -0.56), (1.0, 0.3, 0.02), "couleur_3"), P("c", (0.56, 2.5, 0), (0.02, 0.3, 1.0), "couleur_3"), P("c", (-0.56, 2.5, 0), (0.02, 0.3, 1.0), "couleur_3"),
        P("c", (0, 2.82, 0), (1.3, 0.1, 1.3), "sombre"), P("y", (0, 3.1, 0), (0.06, 0.5, 0.06), "sombre"),
        P("c", (0.3, 3.4, 0), (0.7, 0.08, 0.3), "couleur_3", "radar"), P("s", (0, 3.4, 0), (0.12, 0.12, 0.12), "lumiere_rouge", "voyant"),
        P("c", (0, 0.06, 0), (1.3, 0.12, 1.3), "sombre")],
          [dict(piece="voyant", animation="clignote"), dict(piece="radar", animation="balance", axe="y")]),
    model("grand_cristal", N("Cristal Géant Timide", "Shy Giant Crystal", "Schüchterner Riesenkristall", "Cristal Gigante Tímido", "Cristallo Gigante Timido"), "#a46bff", "#5ff3ff", [
        P("n", (0, 1.5, 0), (1.0, 3.0, 1.0), "couleur_2"), P("n", (0.8, 1.0, 0.3), (0.7, 2.0, 0.7), "couleur_3", None, (0, 0, -16)),
        P("n", (-0.8, 0.9, -0.2), (0.6, 1.8, 0.6), "couleur_2", None, (0, 0, 18)), P("n", (0.2, 0.8, -0.8), (0.55, 1.6, 0.55), "couleur_3", None, (-16, 0, 0)),
        P("n", (-0.1, 0.7, 0.8), (0.5, 1.4, 0.5), "couleur_2", None, (16, 0, 0)), P("s", (0, 3.05, 0), (0.18, 0.18, 0.18), "lumiere_bleue", "eclat"),
        P("s", (0, 0.1, 0), (2.2, 0.25, 2.2), "sombre")],
          [dict(piece="eclat", animation="pulse")]),
]
deco = [
    model("cratere", N("Cratère", "Crater", "Krater", "Cráter", "Cratere"), "#505266", "#7a7d92", [P("s", (0, 0.0, 0), (0.7, 0.06, 0.6), "couleur_2"), P("s", (0.2, 0.02, 0.05), (0.3, 0.05, 0.25), "couleur_3")]),
    model("petit_caillou", N("Petit Caillou", "Little Pebble", "Kieselchen", "Guijarrito", "Sassolino"), "#9a9db0", "#6c6f82", [P("s", (0, 0.07, 0), (0.22, 0.14, 0.2), "couleur_2"), P("s", (0.18, 0.04, 0.1), (0.12, 0.08, 0.1), "couleur_3")]),
    model("drapeau", N("Drapeau Planté", "Planted Flag", "Gepflanzte Flagge", "Bandera Clavada", "Bandiera Piantata"), "#e8433a", "#e8ecf4", [
        P("y", (0, 0.3, 0), (0.025, 0.6, 0.025), "couleur_3"), P("c", (0.0, 0.5, 0.14), (0.02, 0.2, 0.28), "couleur_2", "fanion"), P("s", (0, 0.04, 0), (0.15, 0.06, 0.15), "sombre")],
          [dict(piece="fanion", animation="balance", axe="y")]),
    model("mini_cristal", N("Mini Cristal", "Mini Crystal", "Minikristall", "Minicristal", "Minicristallo"), "#a46bff", "#5ff3ff", [P("n", (0, 0.15, 0), (0.12, 0.3, 0.12), "couleur_2"), P("n", (0.1, 0.1, 0.03), (0.08, 0.2, 0.08), "couleur_3"), P("n", (-0.08, 0.08, -0.04), (0.07, 0.16, 0.07), "couleur_3")]),
    model("balise_lumineuse", N("Balise Qui Clignote", "Blinking Beacon", "Blinkbake", "Baliza Parpadeante", "Segnale Lampeggiante"), "#37e6ff", "#e8ecf4", [P("y", (0, 0.1, 0), (0.1, 0.2, 0.1), "couleur_3"), P("s", (0, 0.24, 0), (0.1, 0.1, 0.1), "couleur_2", "lampe")], [dict(piece="lampe", animation="clignote_alterne")]),
    model("caisse_espace", N("Caisse du Matériel", "Gear Crate", "Materialkiste", "Caja de Material", "Cassa del Materiale"), "#f4f6fa", "#ff8a2a", [P("c", (0, 0.12, 0), (0.3, 0.24, 0.3), "couleur_2"), P("c", (0, 0.12, 0), (0.32, 0.05, 0.32), "couleur_3")]),
    model("empreinte_botte", N("Empreinte de Botte", "Boot Print", "Stiefelabdruck", "Huella de Bota", "Impronta di Stivale"), "#505266", "#7a7d92", [P("c", (0, 0.0, 0), (0.22, 0.01, 0.1), "couleur_2"), P("c", (0.2, 0.0, 0.0), (0.12, 0.01, 0.08), "couleur_2")]),
]
# Dans l'espace, il n'y a que des vaisseaux et des astéroïdes.
obstacles = [obstacles[0]]
obstacles += [
    model("asteroide_cristal", N("Astéroïde Scintillant", "Sparkly Asteroid", "Funkelnder Asteroid", "Asteroide Brillante", "Asteroide Scintillante"), "#8d8f99", "#a46bff", [
        P("s", (0, 0.35, 0), (1.0, 0.7, 0.9), "couleur_2"), P("s", (-0.25, 0.2, 0.2), (0.5, 0.4, 0.5), "couleur_2"),
        P("n", (0.15, 0.85, 0), (0.22, 0.6, 0.22), "couleur_3", None, (0, 0, -14)), P("n", (-0.1, 0.75, 0.15), (0.16, 0.45, 0.16), "couleur_3", None, (10, 0, 14)),
        P("s", (0.15, 1.15, 0), (0.07, 0.07, 0.07), "lumiere_bleue", "eclat")], [dict(piece="eclat", animation="clignote")], roll=2, taille_case=1),
    model("asteroide_haut", N("Astéroïde en Hauteur", "Tall Asteroid", "Hoher Asteroid", "Asteroide Alto", "Asteroide Alto"), "#7f818b", "#5a5c66", [
        P("s", (0, 0.45, 0), (0.8, 0.9, 0.75), "couleur_2"), P("s", (0.1, 0.95, 0.05), (0.55, 0.7, 0.5), "couleur_2"), P("s", (-0.2, 0.2, -0.2), (0.5, 0.4, 0.5), "couleur_3"),
        P("s", (0.25, 0.3, 0.25), (0.4, 0.35, 0.4), "couleur_3"), P("s", (0.0, 1.25, 0.0), (0.3, 0.3, 0.3), "couleur_3")], roll=3, taille_case=1),
]
deco = [d for d in deco if d["id"] in ("petit_caillou", "mini_cristal")]
voie = dict(remplace="rail", description="Rails magnétiques de la base lunaire (non utilisé pour l'instant).", pieces=[])

# =============================================================== ANIMAUX : extraterrestres
def eyes(x, y, dz, d):
    return [P("s", (x, y, dz), (d, d, d), "blanc"), P("s", (x + d * 0.4, y, dz), (d * 0.5, d * 0.5, d * 0.5), "noir"),
            P("s", (x, y, -dz), (d, d, d), "blanc"), P("s", (x + d * 0.4, y, -dz), (d * 0.5, d * 0.5, d * 0.5), "noir")]


martien = [P("s", (0, 0.3, 0), (0.38, 0.34, 0.34), "couleur_2", "corps"), P("s", (0.15, 0.55, 0), (0.34, 0.3, 0.32), "couleur_2", "corps")] + eyes(0.3, 0.6, 0.1, 0.1) + [
    P("y", (0.1, 0.78, 0.07), (0.025, 0.2, 0.025), "couleur_3", None, (0, 0, -10)), P("s", (0.08, 0.9, 0.07), (0.06, 0.06, 0.06), "couleur_3"),
    P("y", (0.1, 0.78, -0.07), (0.025, 0.2, 0.025), "couleur_3", None, (0, 0, -10)), P("s", (0.08, 0.9, -0.07), (0.06, 0.06, 0.06), "couleur_3")] + legs([0.1, -0.1], [0.0], 0.12, 0.22, 0.07, "couleur_3")
blob = [P("s", (0, 0.2, 0), (0.7, 0.4, 0.5), "couleur_2", "corps"), P("s", (0.1, 0.32, 0), (0.5, 0.34, 0.4), "couleur_3", "corps")] + eyes(0.3, 0.32, 0.1, 0.1) + [
    P("c", (0.4, 0.2, 0), (0.02, 0.03, 0.14), "noir", "corps"), P("s", (-0.25, 0.08, 0.2), (0.18, 0.12, 0.18), "couleur_2", "corps"), P("s", (-0.3, 0.08, -0.15), (0.14, 0.1, 0.14), "couleur_2", "corps")]
cyclope = [P("s", (0, 0.65, 0), (0.8, 0.8, 0.6), "couleur_2", "corps"), P("s", (0.15, 1.2, 0), (0.6, 0.55, 0.55), "couleur_2", "corps"),
           P("s", (0.38, 1.25, 0), (0.34, 0.34, 0.34), "blanc"), P("s", (0.52, 1.25, 0), (0.17, 0.17, 0.17), "couleur_3"), P("s", (0.58, 1.25, 0), (0.08, 0.08, 0.08), "noir"),
           P("y", (0.1, 1.62, 0.12), (0.03, 0.3, 0.03), "couleur_3", None, (0, 0, -12)), P("s", (0.07, 1.8, 0.12), (0.08, 0.08, 0.08), "lumiere_rouge", "voyant"),
           P("y", (0.1, 1.62, -0.12), (0.03, 0.3, 0.03), "couleur_3", None, (0, 0, -12)), P("s", (0.07, 1.8, -0.12), (0.08, 0.08, 0.08), "lumiere_rouge", "voyant"),
           P("c", (0.5, 1.02, 0), (0.04, 0.05, 0.3), "noir"), P("s", (-0.25, 0.5, 0.0), (0.3, 0.5, 0.4), "couleur_3", "queue")] + legs([0.22, -0.22], [0.0], 0.35, 0.55, 0.2, "noir") + [
    P("y", (0.1, 0.75, 0.5), (0.14, 0.6, 0.14), "couleur_2", "bras_g"), P("y", (0.1, 0.75, -0.5), (0.14, 0.6, 0.14), "couleur_2", "bras_d")]
robot = [P("c", (0, 1.2, 0), (0.9, 0.9, 0.8), "couleur_2", "corps"), P("c", (0.2, 1.85, 0), (0.55, 0.45, 0.55), "couleur_2", "corps"),
         P("c", (0.48, 1.88, 0.14), (0.04, 0.12, 0.14), "lumiere_rouge", "oeil"), P("c", (0.48, 1.88, -0.14), (0.04, 0.12, 0.14), "lumiere_rouge", "oeil"),
         P("c", (0.48, 1.7, 0), (0.04, 0.05, 0.3), "noir"), P("y", (0.2, 2.25, 0), (0.03, 0.3, 0.03), "sombre"), P("s", (0.2, 2.42, 0), (0.09, 0.09, 0.09), "lumiere_bleue", "voyant"),
         P("c", (0.46, 1.3, 0), (0.02, 0.4, 0.55), "couleur_3"), P("s", (0.47, 1.3, 0.12), (0.12, 0.12, 0.04), "lumiere_rouge", "oeil"),
         P("y", (0.1, 1.3, 0.62), (0.2, 0.8, 0.2), "couleur_3", "bras_g"), P("c", (0.1, 0.85, 0.62), (0.3, 0.2, 0.3), "sombre", "bras_g"),
         P("y", (0.1, 1.3, -0.62), (0.2, 0.8, 0.2), "couleur_3", "bras_d"), P("c", (0.1, 0.85, -0.62), (0.3, 0.2, 0.3), "sombre", "bras_d")] + legs([0.28, -0.28], [0.0], 0.45, 0.8, 0.3, "sombre")
pieuvre = [P("s", (0, 0.9, 0), (1.1, 0.95, 1.0), "couleur_2", "corps"), P("s", (0.1, 1.45, 0), (0.85, 0.8, 0.8), "couleur_2", "corps")] + eyes(0.45, 1.5, 0.22, 0.2) + [
    P("c", (0.52, 1.28, 0), (0.04, 0.06, 0.34), "noir")] + [P("s", (-0.1, 1.7, 0), (0.2, 0.2, 0.2), "couleur_3", "corps")]
for k, z in enumerate((0.45, 0.15, -0.15, -0.45)):
    pieuvre += [P("y", (0.0, 0.5, z), (0.18, 0.9, 0.18), "couleur_3", f"tentacule{k}"), P("s", (0.0, 0.1, z), (0.24, 0.2, 0.24), "couleur_3", f"tentacule{k}")]
soucoupe = [P("s", (0, 0.9, 0), (1.9, 0.38, 1.9), "couleur_2", "corps"), P("s", (0, 1.12, 0), (0.95, 0.65, 0.95), "bulle"), P("s", (0, 1.12, 0), (0.3, 0.32, 0.3), "couleur_3"),
            P("s", (0.1, 1.16, 0.1), (0.1, 0.1, 0.1), "noir"), P("s", (0.1, 1.16, -0.1), (0.1, 0.1, 0.1), "noir"), P("y", (0, 0.68, 0), (0.7, 0.14, 0.7), "sombre")]
for i in range(8):
    a = i * math.pi / 4
    soucoupe.append(P("s", (math.cos(a) * 0.78, 0.86, math.sin(a) * 0.78), (0.16, 0.12, 0.16), "lumiere_bleue" if i % 2 else "lumiere_rouge", "feu_a" if i % 2 else "feu_b"))
soucoupe += [P("c", (0, 0.42, 0), (0.4, 0.5, 0.4), "lumiere_bleue", "rayon")]


ROCHES = [P("s", (0, 0.5, 0), (1.0, 0.8, 0.9), "couleur_2", "roche"), P("s", (0.3, 0.7, 0.2), (0.55, 0.5, 0.5), "couleur_3", "roche"),
          P("s", (-0.3, 0.35, -0.25), (0.5, 0.45, 0.5), "couleur_3", "roche"), P("s", (0.1, 0.25, 0.35), (0.4, 0.35, 0.4), "couleur_2", "roche"),
          P("s", (-0.1, 0.75, -0.3), (0.35, 0.3, 0.35), "couleur_2", "roche"), P("s", (0.4, 0.4, -0.35), (0.3, 0.25, 0.3), "couleur_3", "roche")]
GROSSES_ROCHES = [P("s", (0, 0.8, 0), (1.7, 1.4, 1.5), "couleur_2", "roche"), P("s", (0.5, 1.2, 0.3), (0.9, 0.8, 0.8), "couleur_3", "roche"),
                  P("s", (-0.5, 0.6, -0.5), (0.9, 0.8, 0.8), "couleur_3", "roche"), P("s", (0.2, 0.4, 0.6), (0.7, 0.6, 0.7), "couleur_2", "roche"),
                  P("s", (-0.3, 1.2, -0.3), (0.6, 0.5, 0.6), "couleur_2", "roche"), P("s", (0.8, 0.6, -0.5), (0.5, 0.4, 0.5), "couleur_3", "roche"),
                  P("s", (-0.9, 0.9, 0.3), (0.5, 0.4, 0.5), "couleur_3", "roche")]


CAILLOU = [P("s", (0, 0.3, 0), (0.5, 0.4, 0.45), "couleur_2", "roche"), P("s", (0.12, 0.42, 0.1), (0.28, 0.25, 0.26), "couleur_3", "roche"), P("s", (-0.14, 0.2, -0.1), (0.24, 0.2, 0.22), "couleur_3", "roche")]


def alien(id, classe, noms, longueur, largeur, comportement, vitesse, dangereux, pts, c2, c3, pieces, mobiles, **x):
    return dict(id=id, univers="cosmos_croa", classe=classe, noms=noms, longueur=longueur, largeur=largeur, comportement=comportement, vitesse=vitesse,
                dangereux=dangereux, points_deblocage=pts, couleur_2=c2, couleur_3=c3, pieces=pieces, parties_mobiles=mobiles, **x)


_aliens = [
    alien("mini_martien", "petit", N("Mini-Martien", "Tiny Martian", "Mini-Marsianer", "Mini-Marciano", "Mini-Marziano"), 0.6, 0.4, "traverse", 2.0, False, 300, "#7be37b", "#ff7ad9", martien,
          [dict(piece="corps", animation="rebond", amplitude=0.03)] + leg_anims([0.1, -0.1])),
    alien("blob_gelatine", "petit", N("Blob Gélatine", "Jelly Blob", "Gelee-Blob", "Blob Gelatina", "Blob Gelatina"), 0.8, 0.5, "patrouille", 1.0, False, 450, "#5fe0d0", "#a7fff2", blob,
          [dict(piece="corps", animation="rebond", amplitude=0.05)]),
    alien("alien_cyclope", "moyen", N("Cyclope Pressé", "Hasty Cyclops", "Eiliger Zyklop", "Cíclope con Prisa", "Ciclope Frettoloso"), 1.1, 0.6, "traverse", 3.6, True, 800, "#8be34a", "#2c6e2c", cyclope,
          [dict(piece="corps", animation="rebond", amplitude=0.03), dict(piece="queue", animation="balance", axe="y"), dict(piece="voyant", animation="clignote_alterne"),
           dict(piece="bras_g", animation="pendule", phase=0.5, axe="z", pivot="haut"), dict(piece="bras_d", animation="pendule", phase=0.0, axe="z", pivot="haut")] + leg_anims([0.22, -0.22])),
    alien("robot_pas_content", "grand", N("Robot Pas Content", "Cranky Robot", "Mürrischer Roboter", "Robot Gruñón", "Robot Scontroso"), 1.4, 1.3, "patrouille", 1.6, True, 1500, "#b8c0d8", "#ff5a3c", robot,
          [dict(piece="corps", animation="rebond", amplitude=0.02), dict(piece="oeil", animation="clignote_alterne"), dict(piece="voyant", animation="clignote"),
           dict(piece="bras_g", animation="pendule", phase=0.5, axe="z", pivot="haut"), dict(piece="bras_d", animation="pendule", phase=0.0, axe="z", pivot="haut")] + leg_anims([0.28, -0.28])),
    alien("pieuvre_cosmique", "grand", N("Pieuvre Cosmique", "Cosmic Octopus", "Kosmischer Oktopus", "Pulpo Cósmico", "Polpo Cosmico"), 1.3, 1.4, "traverse", 2.4, True, 2300, "#c06bff", "#ff9ae0", pieuvre,
          [dict(piece="corps", animation="rebond", amplitude=0.04)] + [dict(piece=f"tentacule{k}", animation="pendule", phase=0.25 * k, axe="z", pivot="haut") for k in range(4)]),
] 
animaux = [
    alien("caillou_spatial", "petit", N("Caillou Spatial", "Space Pebble", "Weltraumkiesel", "Guijarro Espacial", "Sassolino Spaziale"), 0.6, 0.5, "traverse", 2.0, False, 300, "#9c9ea8", "#6a6c76", CAILLOU,
          [dict(piece="roche", animation="rotation_groupe")]),
    alien("asteroide", "moyen", N("Astéroïde Qui Roule", "Rolling Asteroid", "Rollender Asteroid", "Asteroide Rodante", "Asteroide Rotolante"), 1.1, 1.0, "traverse", 2.6, True, 600, "#9c9ea8", "#6a6c76", ROCHES,
          [dict(piece="roche", animation="rotation_groupe")]),
    alien("gros_asteroide", "grand", N("Gros Caillou Pas Aimable", "Big Unfriendly Rock", "Großer Unfreundlicher Brocken", "Roca Grande Antipática", "Grosso Masso Antipatico"), 1.9, 1.7, "traverse", 1.8, True, 1900, "#868893", "#5a5c66", GROSSES_ROCHES,
          [dict(piece="roche", animation="rotation_groupe")]),
    alien("soucoupe_volante", "volant", N("Soucoupe Curieuse", "Curious Saucer", "Neugierige Untertasse", "Platillo Curioso", "Disco Curioso"), 1.9, 1.9, "vole_en_ligne", 3.5, True, 3000, "#c8d0e0", "#37e6ff", soucoupe,
          [dict(piece="corps", animation="rebond", amplitude=0.05), dict(piece="feu_a", animation="clignote"), dict(piece="feu_b", animation="clignote_alterne"), dict(piece="rayon", animation="clignote_lent")]),
]

# =============================================================== HÉROS : grenouille astronaute à globe
src = json.load(open(os.path.join(IMP, "jurassic_croa.json"), encoding="utf-8"))["heros"]
hero_pieces = [p for p in src["pieces"] if p.get("nom") != "canne"]
helmet = [
    P("s", (0.2, 0.46, 0), (0.7, 0.66, 0.7), "bulle", "globe"),
    P("y", (0.1, 0.2, 0), (0.5, 0.07, 0.5), "blanc"),
    P("c", (-0.28, 0.34, 0), (0.22, 0.34, 0.36), "blanc"), P("c", (-0.28, 0.34, 0), (0.24, 0.1, 0.38), "couleur_3"),
    P("y", (0.0, 0.8, 0), (0.025, 0.2, 0.025), "sombre"), P("s", (0.0, 0.92, 0), (0.07, 0.07, 0.07), "lumiere_rouge", "balise"),
]
heros = dict(id="grenouille_astronaute", univers="cosmos_croa", noms=N("Grenouille Astronaute", "Astronaut Frog", "Astronauten-Frosch", "Rana Astronauta", "Rana Astronauta"),
             longueur=0.7, largeur=0.6, couleur_2="#f4f6fa", couleur_3="#ff8a2a", pieces=hero_pieces,
             parties_mobiles=[dict(piece="corps", animation="rebond", amplitude=0.02)],
             accessoire=dict(nom=N("Globe d'astronaute", "Astronaut globe", "Astronautenhelm", "Casco de astronauta", "Casco da astronauta"),
                             pieces=helmet, parties_mobiles=[dict(piece="balise", animation="clignote")]))

monde = dict(id="cosmos_croa", nom=N("Cosmos Croâ", "Croak Cosmos", "Quak-Kosmos", "Cosmos Croa", "Cosmo Gra"), points_deblocage=2400,
             ciel="#0b1030", brume="#1a2050", sol_voie_route="#2a2c3a", marquage="#37e6ff", herbe_a="#8d8f99", herbe_b="#7f818b", eau="#05060f", rondin="#9aa3b5",
             obstacles=obstacles, bords=bords, deco=deco, voie_speciale=voie,
             vehicules_du_monde=["navette_pomme", "chasseur_etoile", "cargo_vagabond", "taxi_soucoupe", "fusee_bidon"],
             animaux_du_monde=[a["id"] for a in animaux],
             musique=dict(tempo_bpm=96, gamme="mineure", instruments=["synthé planant", "arpège", "basse sourde", "clochettes"], ambiance="Voyage calme dans le vide, un peu de mystère."),
             **{"bannière": N("Bienvenue dans le Cosmos Croâ ! Gardez votre casque, il n'y a pas d'air, mais il y a des Martiens.", "Welcome to the Croak Cosmos! Keep your helmet on: no air, but plenty of Martians.",
                              "Willkommen im Quak-Kosmos! Helm aufbehalten: keine Luft, aber jede Menge Marsianer.", "¡Bienvenido al Cosmos Croa! No te quites el casco: no hay aire, pero sí marcianos.",
                              "Benvenuto nel Cosmo Gra! Tieni il casco: niente aria, ma tanti marziani.")})
json.dump(dict(monde=monde, animaux=animaux, heros=heros), open(os.path.join(IMP, "espace.json"), "w", encoding="utf-8"), ensure_ascii=False, indent=1)


# =============================================================== VÉHICULES : vaisseaux et rover
def V(id, classe, longueur, largeur, noms, pts, c2, c3, pieces, mobiles=()):
    return dict(id=id, univers="cosmos_croa", classe=classe, noms=noms, longueur=longueur, largeur=largeur, points_deblocage=pts, couleur_2=c2, couleur_3=c3,
                pieces=pieces, parties_mobiles=list(mobiles))


def flame(x, y, z, s=1.0, nom="flamme"):
    return [P("n", (x, y, z), (0.2 * s, 0.5 * s, 0.2 * s), "lumiere_rouge", nom, (0, 0, 90))]


navette = [P("c", (0, 0.5, 0), (1.5, 0.28, 0.5), "couleur_2"), P("n", (0.95, 0.5, 0), (0.5, 0.5, 0.5), "couleur_2", None, (0, 0, -90)), P("c", (0.2, 0.7, 0), (0.7, 0.12, 0.44), "couleur_3"),
           P("c", (0.55, 0.62, 0), (0.3, 0.1, 0.4), "vitre"), P("c", (-0.2, 0.46, 0.5), (0.9, 0.05, 0.5), "couleur_2", None, (0, 0, 0)), P("c", (-0.2, 0.46, -0.5), (0.9, 0.05, 0.5), "couleur_2"),
           P("c", (-0.55, 0.78, 0), (0.4, 0.3, 0.05), "couleur_3"), P("c", (-0.78, 0.5, 0), (0.1, 0.24, 0.4), "sombre")] + flame(-0.95, 0.5, 0.12, 1.0, "flamme") + flame(-0.95, 0.5, -0.12, 1.0, "flamme")
chasseur = [P("c", (0, 0.45, 0), (1.0, 0.18, 0.3), "couleur_2"), P("n", (0.65, 0.45, 0), (0.3, 0.45, 0.3), "couleur_2", None, (0, 0, -90)), P("s", (0.15, 0.58, 0), (0.4, 0.2, 0.24), "bulle"),
            P("c", (-0.1, 0.45, 0.4), (0.6, 0.04, 0.5), "couleur_3", None, (0, 20, 0)), P("c", (-0.1, 0.45, -0.4), (0.6, 0.04, 0.5), "couleur_3", None, (0, -20, 0)),
            P("c", (-0.3, 0.62, 0.0), (0.3, 0.2, 0.04), "couleur_3"), P("c", (0.2, 0.45, 0.42), (0.1, 0.1, 0.1), "sombre"), P("c", (0.2, 0.45, -0.42), (0.1, 0.1, 0.1), "sombre")] + flame(-0.6, 0.45, 0.0, 0.9)
rover = [P("c", (0, 0.32, 0), (1.3, 0.2, 0.7), "couleur_2"), P("c", (0.15, 0.55, 0), (0.6, 0.28, 0.56), "bulle"), P("c", (0.15, 0.7, 0), (0.64, 0.05, 0.6), "couleur_2"),
         P("c", (-0.5, 0.5, 0), (0.3, 0.3, 0.6), "couleur_3"), P("y", (-0.2, 0.9, 0), (0.03, 0.4, 0.03), "sombre"), P("s", (-0.2, 1.12, 0), (0.08, 0.08, 0.08), "lumiere_rouge", "voyant"),
         P("c", (0.68, 0.34, 0.2), (0.04, 0.08, 0.14), "phare"), P("c", (0.68, 0.34, -0.2), (0.04, 0.08, 0.14), "phare")]
for x in (0.45, -0.45):
    for z in (0.4, -0.4):
        rover += [P("y", (x, 0.2, z), (0.4, 0.16, 0.4), "pneu", None, (90, 0, 0)), P("y", (x, 0.2, z + (0.01 if z > 0 else -0.01)), (0.2, 0.18, 0.2), "enjoliveur", None, (90, 0, 0))]
cargo = [P("c", (0, 0.6, 0), (2.6, 0.5, 0.8), "couleur_2"), P("c", (1.5, 0.6, 0), (0.7, 0.4, 0.6), "couleur_3"), P("c", (1.7, 0.68, 0), (0.3, 0.14, 0.5), "vitre"),
         P("c", (-0.2, 0.95, 0), (1.8, 0.3, 0.7), "couleur_3"), P("c", (-0.2, 1.12, 0), (1.6, 0.04, 0.6), "sombre"), P("n", (1.95, 0.6, 0), (0.5, 0.6, 0.5), "couleur_3", None, (0, 0, -90))]
for x in (-0.9, -0.3, 0.3):
    cargo += [P("c", (x, 0.6, 0.41), (0.4, 0.3, 0.02), "couleur_3"), P("c", (x, 0.6, -0.41), (0.4, 0.3, 0.02), "couleur_3")]
cargo += flame(-1.45, 0.45, 0.25, 1.2) + flame(-1.45, 0.45, -0.25, 1.2) + flame(-1.45, 0.8, 0.0, 1.2)
cargo += [P("s", (1.0, 1.05, 0.0), (0.1, 0.1, 0.1), "lumiere_bleue", "gyro")]
taxi = [P("s", (0, 0.5, 0), (1.5, 0.28, 1.1), "couleur_2"), P("s", (0.05, 0.7, 0), (0.7, 0.4, 0.55), "bulle"), P("y", (0, 0.36, 0), (0.7, 0.1, 0.7), "sombre"),
        P("c", (0.05, 0.98, 0), (0.3, 0.1, 0.14), "blanc", "enseigne"), P("s", (0.62, 0.52, 0.3), (0.12, 0.1, 0.12), "phare"), P("s", (0.62, 0.52, -0.3), (0.12, 0.1, 0.12), "phare"),
        P("c", (0.0, 0.5, 0.0), (1.3, 0.04, 1.0), "couleur_3")] + [P("s", (math.cos(a) * 0.62, 0.45, math.sin(a) * 0.45), (0.1, 0.08, 0.1), "lumiere_bleue", "halo") for a in (2.0, 3.14, 4.3)]
fusee = [P("y", (0, 0.5, 0), (1.9, 0.5, 0.5), "couleur_2", None, (0, 0, 90)), P("n", (1.2, 0.5, 0), (0.5, 0.6, 0.5), "couleur_3", None, (0, 0, -90)),
         P("s", (0.5, 0.62, 0.22), (0.14, 0.14, 0.08), "bulle"), P("s", (0.5, 0.62, -0.22), (0.14, 0.14, 0.08), "bulle"),
         P("c", (-0.6, 0.5, 0.0), (0.4, 0.04, 1.1), "couleur_3"), P("c", (-0.6, 0.5, 0.0), (0.4, 0.9, 0.04), "couleur_3"), P("c", (0.1, 0.5, 0.0), (0.08, 0.52, 0.52), "couleur_3")] + flame(-1.2, 0.5, 0.0, 1.5)
vehicules = [
    V("navette_pomme", "voiture", 1.9, 0.9, N("Navette Pomme", "Apple Shuttle", "Apfel-Shuttle", "Lanzadera Manzana", "Navetta Mela"), 0, "#f4f6fa", "#2a2f4a", navette, [dict(piece="flamme", animation="pulse")]),
    V("chasseur_etoile", "voiture", 1.5, 0.9, N("Chasseur Étoilé", "Starry Fighter", "Sternjäger", "Caza Estelar", "Caccia Stellare"), 300, "#e8433a", "#f4f6fa", chasseur, [dict(piece="flamme", animation="pulse")]),
    V("cargo_vagabond", "camion", 3.4, 0.95, N("Cargo Vagabond", "Wandering Freighter", "Wanderfrachter", "Carguero Errante", "Cargo Vagabondo"), 1000, "#9aa3b5", "#ff8a2a", cargo, [dict(piece="flamme", animation="pulse"), dict(piece="gyro", animation="pulse")]),
    V("taxi_soucoupe", "voiture", 1.7, 1.1, N("Taxi Soucoupe", "Saucer Cab", "Untertassen-Taxi", "Taxi Platillo", "Taxi Disco"), 500, "#ffd23f", "#2a2f4a", taxi, [dict(piece="enseigne", animation="clignote"), dict(piece="halo", animation="clignote_alterne"), dict(piece="*", animation="flotte_verticale_0.04")]),
    V("fusee_bidon", "camion", 2.4, 0.9, N("Fusée Bidon", "Rubbish Rocket", "Schrottrakete", "Cohete Chatarra", "Razzo Rottame"), 1600, "#e8ecf4", "#e8433a", fusee, [dict(piece="flamme", animation="pulse")]),
]
json.dump(dict(vehicules=vehicules), open(os.path.join(IMP, "espace_vehicules.json"), "w", encoding="utf-8"), ensure_ascii=False, indent=1)
print(len(obstacles), "obstacles,", len(bords), "bords,", len(animaux), "aliens,", len(vehicules), "vaisseaux")
