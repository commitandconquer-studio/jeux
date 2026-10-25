#!/usr/bin/env python3
# Commit & Conquer — licence GPL-3.0-or-later.
#
# Le monde « Baie des Pirates » de Traversée : plage, ponton de bois, mer avec tonneaux et
# radeaux, wagonnets de trésor, pirates, canon, kraken, perroquet. Avant = +X, haut = +Y,
# largeur = Z. Écrit games/traversee/data/import/pirates.json
import json, os

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


def eyes(x, y, dz, d, mat="noir"):
    return [P("s", (x, y, dz), (d, d, d), mat), P("s", (x, y, -dz), (d, d, d), mat)]


# =============================================================== DÉCOR
palmier = model("palmier", N("Palmier Penché", "Leaning Palm", "Schiefe Palme", "Palmera Torcida", "Palma Pendente"), "#3f9a4a", "#8a5a2b", [
    P("y", (0, 0.3, 0), (0.2, 0.6, 0.2), "couleur_3"), P("y", (0.08, 0.8, 0), (0.18, 0.5, 0.18), "couleur_3", None, (0, 0, -10)), P("y", (0.2, 1.2, 0), (0.16, 0.5, 0.16), "couleur_3", None, (0, 0, -18)),
    P("n", (0.3, 1.5, 0), (0.14, 0.8, 0.14), "couleur_2", None, (0, 0, 70)), P("n", (0.3, 1.5, 0), (0.14, 0.8, 0.14), "couleur_2", None, (0, 0, -70)),
    P("n", (0.3, 1.5, 0), (0.14, 0.8, 0.14), "couleur_2", None, (70, 0, 0)), P("n", (0.3, 1.5, 0), (0.14, 0.8, 0.14), "couleur_2", None, (-70, 0, 0)),
    P("s", (0.32, 1.4, 0.08), (0.12, 0.12, 0.12), "couleur_3"), P("s", (0.32, 1.4, -0.08), (0.12, 0.12, 0.12), "couleur_3")], roll=1, taille_case=1)
coffre = model("coffre", N("Coffre au Trésor", "Treasure Chest", "Schatztruhe", "Cofre del Tesoro", "Forziere del Tesoro"), "#8a5a2b", "#ffd23f", [
    P("c", (0, 0.2, 0), (0.8, 0.4, 0.55), "couleur_2"), P("y", (0, 0.42, 0), (0.55, 0.8, 0.55), "couleur_2", None, (0, 0, 90)), P("c", (0, 0.2, 0.28), (0.82, 0.06, 0.02), "couleur_3"),
    P("c", (0, 0.4, 0.0), (0.1, 0.44, 0.58), "couleur_3"), P("c", (0.0, 0.3, 0.29), (0.1, 0.12, 0.03), "sombre"), P("s", (0.0, 0.62, 0.0), (0.18, 0.1, 0.18), "phare", "or")], [dict(piece="or", animation="clignote_lent")], roll=2, taille_case=1)
tonneaux = model("tonneaux", N("Tonneaux de Rhum", "Rum Barrels", "Rumfässer", "Barriles de Ron", "Botti di Rum"), "#a0693a", "#3a2a1a", [
    P("y", (-0.15, 0.3, -0.1), (0.5, 0.6, 0.5), "couleur_2"), P("y", (-0.15, 0.18, -0.1), (0.53, 0.05, 0.53), "couleur_3"), P("y", (-0.15, 0.42, -0.1), (0.53, 0.05, 0.53), "couleur_3"),
    P("y", (0.25, 0.28, 0.2), (0.46, 0.56, 0.46), "couleur_2"), P("y", (0.25, 0.16, 0.2), (0.49, 0.05, 0.49), "couleur_3"), P("y", (0.25, 0.4, 0.2), (0.49, 0.05, 0.49), "couleur_3"),
    P("y", (0.0, 0.8, 0.05), (0.42, 0.5, 0.42), "couleur_2"), P("y", (0.0, 0.72, 0.05), (0.45, 0.05, 0.45), "couleur_3")], roll=3, taille_case=1)
ancre = model("ancre", N("Vieille Ancre", "Old Anchor", "Alter Anker", "Ancla Vieja", "Vecchia Ancora"), "#5a6070", "#8a5a2b", [
    P("y", (0, 0.5, 0), (0.1, 1.0, 0.1), "couleur_2"), P("c", (0, 0.85, 0), (0.5, 0.08, 0.08), "couleur_2"), P("s", (0, 1.05, 0), (0.18, 0.18, 0.18), "couleur_2"),
    P("c", (0, 0.1, 0), (0.7, 0.1, 0.1), "couleur_2", None, (0, 0, 0)), P("n", (0.36, 0.2, 0), (0.12, 0.28, 0.1), "couleur_2", None, (0, 0, 20)), P("n", (-0.36, 0.2, 0), (0.12, 0.28, 0.1), "couleur_2", None, (0, 0, -20)),
    P("c", (0, 0.04, 0), (0.7, 0.08, 0.7), "couleur_3")], roll=4, taille_case=1)
drapeau = model("drapeau_pirate", N("Drapeau Noir", "Jolly Roger", "Piratenflagge", "Bandera Pirata", "Bandiera Pirata"), "#1c1c24", "#f4f0e0", [
    P("y", (0, 0.7, 0), (0.07, 1.4, 0.07), "couleur_3"), P("c", (0.0, 1.15, 0.28), (0.03, 0.4, 0.5), "couleur_2", "fanion"), P("s", (0.0, 1.18, 0.28), (0.14, 0.14, 0.14), "blanc", "fanion"),
    P("c", (0.0, 1.07, 0.28), (0.05, 0.04, 0.3), "blanc", "fanion"), P("c", (0, 0.04, 0), (0.4, 0.08, 0.4), "sombre")], [dict(piece="fanion", animation="balance", axe="y")], roll=5, taille_case=1)
obstacles = [palmier, coffre, tonneaux, ancre, drapeau]
galion = model("galion", N("Galion Échoué", "Beached Galleon", "Gestrandete Galeone", "Galeón Varado", "Galeone Arenato"), "#8a5a2b", "#f4f0e0", [
    P("c", (0, 0.5, 0), (3.0, 1.0, 1.2), "couleur_2"), P("c", (1.6, 0.8, 0), (0.6, 1.0, 0.9), "couleur_2", None, (0, 0, -15)), P("c", (-1.4, 1.0, 0), (0.6, 1.6, 1.1), "couleur_2"),
    P("c", (0, 1.05, 0), (3.1, 0.1, 1.25), "sombre"), P("y", (0.2, 2.2, 0), (0.12, 2.4, 0.12), "sombre"), P("c", (0.2, 2.3, 0.0), (0.04, 1.6, 1.4), "couleur_3", "voile"),
    P("y", (-0.9, 1.9, 0), (0.1, 1.6, 0.1), "sombre"), P("c", (-0.9, 2.0, 0.0), (0.04, 1.0, 0.9), "couleur_3", "voile"), P("c", (0.2, 3.5, 0.2), (0.03, 0.3, 0.5), "sombre", "fanion")],
    [dict(piece="voile", animation="balance", axe="y"), dict(piece="fanion", animation="balance", axe="y")])
phare = model("phare", N("Vieux Phare", "Old Lighthouse", "Alter Leuchtturm", "Faro Viejo", "Vecchio Faro"), "#f4f0e0", "#c0392b", [
    P("y", (0, 1.2, 0), (0.9, 2.4, 0.9), "couleur_2"), P("y", (0, 0.6, 0), (0.95, 0.5, 0.95), "couleur_3"), P("y", (0, 1.6, 0), (0.92, 0.45, 0.92), "couleur_3"), P("y", (0, 2.5, 0), (1.1, 0.12, 1.1), "sombre"),
    P("y", (0, 2.8, 0), (0.6, 0.5, 0.6), "bulle"), P("s", (0, 2.8, 0), (0.3, 0.3, 0.3), "phare", "feu"), P("n", (0, 3.35, 0), (0.7, 0.6, 0.7), "couleur_3")], [dict(piece="feu", animation="pulse")])
cabane = model("cabane_plage", N("Cabane de Plage", "Beach Shack", "Strandhütte", "Cabaña de Playa", "Capanna di Spiaggia"), "#c9a063", "#3f9a4a", [
    P("c", (0, 0.6, 0), (1.4, 1.2, 1.1), "couleur_2"), P("n", (0, 1.5, 0), (1.9, 0.9, 1.5), "couleur_3", None, (0, 45, 0)), P("c", (0, 0.4, 0.56), (0.4, 0.8, 0.03), "sombre"), P("c", (0.45, 0.7, 0.56), (0.3, 0.3, 0.03), "vitre")])
bords = [galion, phare, cabane]
deco = [
    model("coquillage", N("Coquillage", "Seashell", "Muschel", "Concha", "Conchiglia"), "#f6c8d0", "#e8a0b0", [P("s", (0, 0.04, 0), (0.16, 0.08, 0.14), "couleur_2"), P("n", (0.0, 0.1, 0), (0.1, 0.1, 0.1), "couleur_3")]),
    model("etoile_mer", N("Étoile de Mer", "Starfish", "Seestern", "Estrella de Mar", "Stella Marina"), "#ff8a3f", "#e0702a", [P("c", (0, 0.015, 0), (0.28, 0.03, 0.06), "couleur_2", None, (0, 0, 0)), P("c", (0, 0.015, 0), (0.28, 0.03, 0.06), "couleur_2", None, (0, 72, 0)), P("c", (0, 0.015, 0), (0.28, 0.03, 0.06), "couleur_2", None, (0, 144, 0)), P("s", (0, 0.03, 0), (0.08, 0.04, 0.08), "couleur_3")]),
    model("piece_or", N("Pièce d'Or", "Gold Coin", "Goldmünze", "Moneda de Oro", "Moneta d'Oro"), "#ffd23f", "#e0a800", [P("y", (0, 0.015, 0), (0.14, 0.03, 0.14), "couleur_2"), P("y", (0, 0.03, 0), (0.09, 0.015, 0.09), "couleur_3")]),
    model("bouteille", N("Bouteille à la Mer", "Message in a Bottle", "Flaschenpost", "Botella con Mensaje", "Messaggio in Bottiglia"), "#5fb89a", "#f4f0e0", [P("y", (0, 0.06, 0), (0.08, 0.16, 0.08), "couleur_2", None, (0, 0, 80)), P("c", (0.04, 0.06, 0), (0.06, 0.04, 0.03), "couleur_3")]),
]


# =============================================================== CRÉATURES
def creature(id, classe, noms, longueur, largeur, comportement, vitesse, dangereux, pts, c2, c3, pieces, mobiles):
    return dict(id=id, univers="pirates", classe=classe, noms=noms, longueur=longueur, largeur=largeur, comportement=comportement, vitesse=vitesse,
                dangereux=dangereux, points_deblocage=pts, couleur_2=c2, couleur_3=c3, pieces=pieces, parties_mobiles=mobiles)


crabe = [P("s", (0, 0.25, 0), (0.55, 0.28, 0.5), "couleur_2", "corps")] + eyes(0.22, 0.42, 0.1, 0.07, "blanc") + eyes(0.26, 0.42, 0.1, 0.035) + [
    P("y", (0.22, 0.38, 0.1), (0.025, 0.12, 0.025), "couleur_2", "corps"), P("y", (0.22, 0.38, -0.1), (0.025, 0.12, 0.025), "couleur_2", "corps")]
crabe_m = [dict(piece="corps", animation="rebond", amplitude=0.03)]
for nom, z, ph in (("pince_g", 0.32, 0.0), ("pince_d", -0.32, 0.5)):
    crabe += [P("y", (0.15, 0.3, z), (0.07, 0.3, 0.07), "couleur_2", nom), P("s", (0.3, 0.34, z), (0.18, 0.14, 0.14), "couleur_3", nom), P("n", (0.4, 0.34, z), (0.08, 0.14, 0.05), "couleur_3", nom, (0, 0, -90))]
    crabe_m.append(swing(nom, ph))
for k, (x, z, ph) in enumerate(((0.1, 0.28, 0.0), (-0.05, 0.3, 0.5), (-0.2, 0.28, 0.0), (0.1, -0.28, 0.5), (-0.05, -0.3, 0.0), (-0.2, -0.28, 0.5))):
    crabe += [P("y", (x, 0.12, z), (0.04, 0.25, 0.04), "couleur_2", f"patte{k}")]
    crabe_m.append(swing(f"patte{k}", ph))
rat = [P("s", (0, 0.2, 0), (0.5, 0.26, 0.24), "couleur_2", "corps"), P("s", (0.28, 0.24, 0), (0.22, 0.2, 0.2), "couleur_2", "corps"), P("n", (0.42, 0.22, 0), (0.08, 0.16, 0.08), "couleur_3", "corps", (0, 0, -90)),
       P("s", (0.22, 0.36, 0.08), (0.08, 0.1, 0.04), "couleur_3", "corps"), P("s", (0.22, 0.36, -0.08), (0.08, 0.1, 0.04), "couleur_3", "corps"), P("s", (0.36, 0.28, 0.07), (0.04, 0.04, 0.04), "noir", "corps"),
       P("s", (0.36, 0.28, -0.07), (0.04, 0.04, 0.04), "noir", "corps"), P("y", (-0.4, 0.2, 0), (0.03, 0.4, 0.03), "couleur_3", "queue", (0, 0, 80))]
rat_m = [dict(piece="corps", animation="rebond", amplitude=0.02), dict(piece="queue", animation="balance", axe="y")]
for k, (x, z, ph) in enumerate(((0.15, 0.08, 0.0), (0.15, -0.08, 0.5), (-0.15, 0.08, 0.5), (-0.15, -0.08, 0.0))):
    rat += [P("y", (x, 0.07, z), (0.05, 0.14, 0.05), "couleur_3", f"patte{k}")]
    rat_m.append(swing(f"patte{k}", ph))


def pirate(couleur_peau="couleur_2", cap=False, squelette=False):
    body = "blanc" if squelette else "couleur_3"
    p = [P("c", (0, 0.95, 0), (0.42, 0.7, 0.56), body, "corps"), P("s", (0.04, 1.5, 0), (0.38, 0.38, 0.36), "blanc" if squelette else couleur_peau, "corps")]
    p += eyes(0.22, 1.54, 0.08, 0.07, "noir")
    p += [P("c", (0.2, 1.54, 0.0), (0.04, 0.06, 0.2), "noir", "corps"), P("c", (0.2, 1.58, 0.07), (0.04, 0.12, 0.1), "noir", "corps")]   # bandeau sur l'œil
    p += [P("y", (0.04, 1.72, 0), (0.62, 0.05, 0.62), "sombre", "corps"), P("n", (0.04, 1.9, 0), (0.46, 0.3, 0.4), "sombre", "corps"), P("c", (0.3, 1.78, 0), (0.04, 0.14, 0.14), "blanc", "corps")]
    if cap:
        p += [P("c", (0.04, 1.95, -0.3), (0.04, 0.3, 0.1), "lumiere_rouge", "corps", (0, 0, 15)), P("s", (0.25, 1.35, -0.38), (0.12, 0.2, 0.16), "lumiere_rouge", "corps")]
    p += [P("y", (0.35, 1.2, -0.38), (0.04, 0.9, 0.04), "blanc", "sabre", (0, 0, -40)), P("c", (0.2, 0.9, -0.38), (0.03, 0.03, 0.2), "couleur_3", "sabre")]
    m = [dict(piece="corps", animation="rebond", amplitude=0.03), dict(piece="sabre", animation="balance", axe="z")]
    for nom, z, ph in (("bras_g", 0.3, 0.5),):
        p += [P("y", (0, 1.0, z), (0.1, 0.55, 0.1), body, nom), P("n", (0.0, 0.7, z), (0.08, 0.16, 0.06), "sombre", nom, (180, 0, 0))]
        m.append(swing(nom, ph))
    for nom, z, ph in (("patte_g", 0.13, 0.0), ("patte_d", -0.13, 0.5)):
        p += [P("y", (0, 0.3, z), (0.14, 0.6, 0.14), "sombre", nom), P("c", (0.08, 0.04, z), (0.26, 0.08, 0.16), "noir", nom)]
        m.append(swing(nom, ph))
    return p, m


pir_p, pir_m = pirate()
squ_p, squ_m = pirate(squelette=True)
cap_p, cap_m = pirate(cap=True)
cap_p = [dict(p, position=[p["position"][0], round(p["position"][1] * 1.25, 3), p["position"][2]], taille=[round(v * 1.2, 3) for v in p["taille"]]) for p in cap_p]
cap_p += [P("s", (0.1, 1.95, 0.4), (0.22, 0.26, 0.2), "lumiere_rouge", "corps"), P("n", (0.35, 1.9, 0.4), (0.06, 0.1, 0.05), "phare", "corps", (0, 0, -90))]   # perroquet sur l'épaule
canon = [P("y", (0.2, 0.7, 0), (0.4, 1.6, 0.4), "sombre", "corps", (0, 0, 90)), P("s", (-0.6, 0.7, 0), (0.45, 0.45, 0.45), "sombre", "corps"), P("y", (1.0, 0.7, 0), (0.5, 0.12, 0.5), "sombre", "corps", (0, 0, 90)),
         P("c", (0.0, 0.5, 0), (1.1, 0.2, 0.7), "couleur_2", "corps"), P("s", (-0.78, 0.82, 0), (0.1, 0.1, 0.1), "lumiere_rouge", "meche")]
for x in (-0.3, 0.5):
    for z in (0.4, -0.4):
        canon += [P("y", (x, 0.32, z), (0.64, 0.1, 0.64), "couleur_2", None, (90, 0, 0)), P("y", (x, 0.32, z + (0.055 if z > 0 else -0.055)), (0.3, 0.06, 0.3), "couleur_3", None, (90, 0, 0))]
canon_m = [dict(piece="corps", animation="vibre"), dict(piece="meche", animation="clignote")]
kraken = [P("s", (0, 1.4, 0), (1.5, 1.4, 1.3), "couleur_2", "corps"), P("s", (0.1, 2.1, 0), (1.0, 0.9, 0.9), "couleur_2", "corps")] + eyes(0.5, 1.9, 0.3, 0.3, "blanc") + eyes(0.62, 1.9, 0.3, 0.14, "noir") + [
    P("c", (0.65, 1.55, 0), (0.06, 0.1, 0.5), "noir", "corps"), P("s", (-0.2, 2.5, 0), (0.4, 0.3, 0.4), "couleur_3", "corps")]
kr_m = [dict(piece="corps", animation="rebond", amplitude=0.05)]
for k, z in enumerate((0.9, 0.55, 0.2, -0.2, -0.55, -0.9)):
    kraken += [P("y", (0, 0.7, z), (0.26, 1.3, 0.26), "couleur_2", f"tentacule{k}"), P("s", (0, 0.1, z), (0.34, 0.24, 0.34), "couleur_3", f"tentacule{k}")]
    kr_m.append(swing(f"tentacule{k}", 0.2 * k))
perroquet = [P("s", (0, 0.95, 0), (0.6, 0.4, 0.34), "couleur_2", "corps"), P("s", (0.35, 1.05, 0), (0.3, 0.3, 0.28), "couleur_2", "corps"), P("s", (0.52, 1.02, 0), (0.2, 0.2, 0.2), "blanc", "corps"),
             P("s", (0.48, 1.14, 0.1), (0.07, 0.07, 0.05), "noir", "corps"), P("s", (0.48, 1.14, -0.1), (0.07, 0.07, 0.05), "noir", "corps"),
             P("s", (0.65, 0.98, 0), (0.18, 0.2, 0.14), "phare", "corps"), P("n", (0.7, 0.9, 0), (0.1, 0.14, 0.08), "sombre", "corps", (0, 0, 180)),
             P("c", (-0.55, 0.85, 0), (0.7, 0.04, 0.2), "couleur_3", "corps", (0, 0, 8)), P("c", (-0.6, 0.82, 0.1), (0.6, 0.04, 0.1), "lumiere_bleue", "corps", (0, 0, 8)), P("c", (-0.6, 0.82, -0.1), (0.6, 0.04, 0.1), "phare", "corps", (0, 0, 8))]
for nom, s in (("aile_g", 1), ("aile_d", -1)):
    perroquet += [P("c", (0.0, 0.97, 0.55 * s), (0.5, 0.04, 0.8), "couleur_2", nom), P("c", (-0.1, 0.97, 1.0 * s), (0.5, 0.04, 0.5), "couleur_3", nom), P("c", (-0.2, 0.98, 1.4 * s), (0.4, 0.04, 0.4), "lumiere_bleue", nom),
                  P("c", (0.1, 0.97, 1.2 * s), (0.1, 0.05, 0.9), "phare", nom)]
pe_m = [dict(piece="corps", animation="rebond", amplitude=0.04), dict(piece="aile_g", animation="pendule", phase=0.0, axe="x", pivot="base"), dict(piece="aile_d", animation="pendule", phase=0.0, axe="x", pivot="base")]

animaux = [
    creature("crabe", "petit", N("Crabe Pas Content", "Cranky Crab", "Mürrische Krabbe", "Cangrejo Gruñón", "Granchio Scontroso"), 0.8, 0.8, "traverse", 2.2, False, 300, "#e8503a", "#c0392b", crabe, crabe_m),
    creature("rat_cale", "petit", N("Rat de Cale", "Bilge Rat", "Schiffsratte", "Rata de Sentina", "Ratto di Stiva"), 0.9, 0.4, "traverse", 3.0, False, 450, "#8a8478", "#d8a0a8", rat, rat_m),
    creature("pirate", "moyen", N("Pirate Bagarreur", "Brawling Pirate", "Raufender Pirat", "Pirata Pendenciero", "Pirata Attaccabrighe"), 0.8, 0.8, "traverse", 3.2, True, 900, "#e8b88a", "#c0392b", pir_p, pir_m),
    creature("squelette_pirate", "moyen", N("Pirate Squelette", "Skeleton Pirate", "Skelett-Pirat", "Pirata Esqueleto", "Pirata Scheletro"), 0.8, 0.8, "traverse", 3.4, True, 1300, "#f1ead8", "#6a6a7a", squ_p, squ_m),
    creature("capitaine", "grand", N("Capitaine Crochet-Crapaud", "Captain Hook-Toad", "Kapitän Hakenkröte", "Capitán Garfio-Sapo", "Capitan Uncino-Rospo"), 1.0, 1.0, "traverse", 2.6, True, 1800, "#e8b88a", "#2a4a8a", cap_p, cap_m),
    creature("canon", "grand", N("Canon Qui Roule", "Rolling Cannon", "Rollende Kanone", "Cañón Rodante", "Cannone Rotolante"), 2.0, 1.0, "traverse", 2.4, True, 2300, "#8a5a2b", "#c9a063", canon, canon_m),
    creature("kraken", "geant", N("Kraken des Plages", "Beach Kraken", "Strand-Kraken", "Kraken de Playa", "Kraken da Spiaggia"), 1.8, 2.0, "charge", 1.8, True, 3000, "#7a4a9a", "#d890c8", kraken, kr_m),
    creature("perroquet", "volant", N("Perroquet Pirate", "Pirate Parrot", "Piraten-Papagei", "Loro Pirata", "Pappagallo Pirata"), 1.6, 3.2, "vole_en_ligne", 3.6, True, 3400, "#d83a3a", "#ffd23f", perroquet, pe_m),
]

# =============================================================== HÉROS : grenouille pirate
src = json.load(open(os.path.join(IMP, "jurassic_croa.json"), encoding="utf-8"))["heros"]
hero_pieces = [p for p in src["pieces"] if p.get("nom") != "canne"]
tricorne = [P("y", (0.12, 0.7, 0), (0.64, 0.04, 0.64), "couleur_3"), P("c", (0.12, 0.8, 0), (0.46, 0.18, 0.5), "couleur_3"), P("c", (0.38, 0.78, 0), (0.1, 0.2, 0.62), "couleur_3", None, (0, 0, -20)),
            P("c", (-0.14, 0.78, 0), (0.1, 0.2, 0.62), "couleur_3", None, (0, 0, 20)), P("s", (0.3, 0.8, 0.0), (0.12, 0.12, 0.04), "blanc"), P("c", (0.3, 0.74, 0.0), (0.05, 0.03, 0.04), "noir"),
            P("c", (0.3, 0.57, 0.14), (0.05, 0.12, 0.14), "noir"), P("s", (0.1, 0.34, 0.38), (0.07, 0.07, 0.07), "phare")]
heros = dict(id="grenouille_pirate", univers="pirates", noms=N("Grenouille Pirate", "Pirate Frog", "Piraten-Frosch", "Rana Pirata", "Rana Pirata"), longueur=0.7, largeur=0.6,
             couleur_2="#c0392b", couleur_3="#2a2a35", pieces=hero_pieces, parties_mobiles=[dict(piece="corps", animation="rebond", amplitude=0.02)],
             accessoire=dict(nom=N("Tricorne de pirate", "Pirate tricorn", "Piratendreispitz", "Tricornio pirata", "Tricorno da pirata"), pieces=tricorne))


# =============================================================== CONVOI : wagonnets de trésor
def roue(x, z, r):
    return [P("y", (x, r, z), (r * 2, 0.07, r * 2), "sombre", None, (90, 0, 0)), P("y", (x, r, z + (0.04 if z > 0 else -0.04)), (r * 1.1, 0.05, r * 1.1), "couleur_3", None, (90, 0, 0))]


def wagon_canon():
    p = [P("c", (0, 0.45, 0), (1.9, 0.14, 0.8), "couleur_2"), P("c", (0, 0.65, 0.4), (1.9, 0.3, 0.04), "couleur_2"), P("c", (0, 0.65, -0.4), (1.9, 0.3, 0.04), "couleur_2"),
         P("y", (0.2, 0.85, 0), (0.36, 1.5, 0.36), "sombre", "corps", (0, 0, 80)), P("s", (-0.5, 0.82, 0), (0.4, 0.4, 0.4), "sombre", "corps"), P("y", (0.95, 0.9, 0), (0.46, 0.1, 0.46), "sombre", "corps", (0, 0, 80)),
         P("c", (-0.1, 0.62, 0), (0.7, 0.2, 0.5), "couleur_3"), P("s", (-0.68, 0.95, 0), (0.1, 0.1, 0.1), "lumiere_rouge", "meche")]
    for x in (-0.6, 0.6):
        for z in (0.42, -0.42):
            p += roue(x, z, 0.26)
    return p, [dict(piece="corps", animation="vibre"), dict(piece="meche", animation="clignote")]


def wagon_tresor(kind):
    p = [P("c", (0, 0.45, 0), (1.9, 0.14, 0.8), "couleur_2"), P("c", (0, 0.7, 0.4), (1.9, 0.4, 0.04), "couleur_2"), P("c", (0, 0.7, -0.4), (1.9, 0.4, 0.04), "couleur_2"),
         P("c", (0.94, 0.7, 0), (0.04, 0.4, 0.8), "couleur_2"), P("c", (-0.94, 0.7, 0), (0.04, 0.4, 0.8), "couleur_2")]
    if kind == 0:
        p += [P("s", (0, 0.92, 0), (1.7, 0.5, 0.72), "couleur_3", "or"), P("s", (-0.3, 1.1, 0.1), (0.5, 0.3, 0.4), "couleur_3", "or"), P("s", (0.4, 1.12, -0.1), (0.5, 0.3, 0.4), "couleur_3", "or")]
    else:
        p += [P("c", (-0.4, 0.95, 0), (0.7, 0.5, 0.55), "couleur_2"), P("y", (-0.4, 1.2, 0), (0.55, 0.7, 0.55), "couleur_2", None, (0, 0, 90)), P("c", (-0.4, 1.0, 0), (0.72, 0.1, 0.6), "couleur_3"),
              P("c", (0.4, 0.95, 0), (0.7, 0.5, 0.55), "couleur_2"), P("c", (0.4, 0.95, 0.0), (0.1, 0.5, 0.58), "couleur_3")]
    for x in (-0.6, 0.6):
        for z in (0.42, -0.42):
            p += roue(x, z, 0.26)
    return p, [dict(piece="or", animation="clignote_lent")] if kind == 0 else []


cp, cm = wagon_canon()
t0, _ = wagon_tresor(1)
t1, m1 = wagon_tresor(0)
t2, _ = wagon_tresor(1)
train = [
    dict(id="wagon_canon", longueur=2.5, couleur_2="#8a5a2b", couleur_3="#ffd23f", pieces=cp, parties_mobiles=cm),
    dict(id="wagon_coffres", longueur=2.1, couleur_2="#8a5a2b", couleur_3="#ffd23f", pieces=t0, parties_mobiles=[]),
    dict(id="wagon_or", longueur=2.1, couleur_2="#8a5a2b", couleur_3="#ffd23f", pieces=t1, parties_mobiles=m1),
    dict(id="wagon_coffres_b", longueur=2.1, couleur_2="#8a5a2b", couleur_3="#ffd23f", pieces=t2, parties_mobiles=[]),
]

monde = dict(id="pirates", nom=N("Baie des Pirates", "Pirate Cove", "Piratenbucht", "Bahía Pirata", "Baia dei Pirati"), points_deblocage=5400,
             ciel="#8fd3f0", brume="#d6eef6", sol_voie_route="#8a5a2b", marquage="#5a3a1c", herbe_a="#ecdca4", herbe_b="#e2d094", eau="#2a8fd0", rondin="#8a5a2b",
             obstacles=obstacles, bords=bords, deco=deco, voie_speciale=dict(remplace="route", description="Ponton de bois.", pieces=[]),
             vehicules_du_monde=[], animaux_du_monde=[a["id"] for a in animaux], train=train,
             musique=dict(tempo_bpm=112, gamme="dorienne", instruments=["accordéon", "violon", "tambour", "flûte"], ambiance="Chanson de marins sur la plage."),
             **{"bannière": N("Bienvenue à la Baie des Pirates ! Les trésors sont à gauche, les ennuis aussi.", "Welcome to Pirate Cove! Treasure's on the left, so is trouble.",
                              "Willkommen in der Piratenbucht! Schätze links, Ärger auch.", "¡Bienvenido a la Bahía Pirata! Los tesoros a la izquierda, los problemas también.",
                              "Benvenuto nella Baia dei Pirati! I tesori a sinistra, i guai anche.")})
json.dump(dict(monde=monde, animaux=animaux, heros=heros), open(os.path.join(IMP, "pirates.json"), "w", encoding="utf-8"), ensure_ascii=False, indent=1)
print(len(obstacles), "obstacles,", len(animaux), "créatures,", len(train), "voitures de train")
