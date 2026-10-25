#!/usr/bin/env python3
# Commit & Conquer — licence GPL-3.0-or-later.
#
# Le monde « Royaume de Croâ » de Traversée (Moyen Âge et dragon) : chemin pavé, douves,
# convoi de charrettes, chevaliers, gobelins, ogre, catapulte, dragon, château.
# Avant = +X, haut = +Y, largeur = Z. Écrit games/traversee/data/import/moyenage.json
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


def shift(pieces, dx=0.0, dy=0.0, dz=0.0):
    return [dict(p, position=[round(p["position"][0] + dx, 3), round(p["position"][1] + dy, 3), round(p["position"][2] + dz, 3)]) for p in pieces]


# =============================================================== DÉCOR
chene = model("chene", N("Vieux Chêne", "Old Oak", "Alte Eiche", "Viejo Roble", "Vecchia Quercia"), "#4f8f3a", "#6b4a2b", [
    P("y", (0, 0.4, 0), (0.28, 0.8, 0.28), "couleur_3"), P("s", (0, 1.0, 0), (1.0, 0.85, 1.0), "couleur_2"), P("s", (0.3, 0.8, 0.15), (0.55, 0.5, 0.55), "couleur_2"),
    P("s", (-0.28, 0.85, -0.1), (0.5, 0.45, 0.5), "couleur_2")], roll=1, taille_case=1)
rocher = model("rocher_mousse", N("Rocher à Mousse", "Mossy Rock", "Moosfelsen", "Roca con Musgo", "Roccia Muschiosa"), "#8d8f99", "#4f7a4a", [
    P("s", (0, 0.3, 0), (0.95, 0.6, 0.8), "couleur_2"), P("s", (0.25, 0.55, 0.1), (0.55, 0.45, 0.5), "couleur_2"), P("s", (-0.05, 0.62, -0.05), (0.5, 0.18, 0.4), "couleur_3")], roll=2, taille_case=1)
puits = model("puits", N("Puits du Village", "Village Well", "Dorfbrunnen", "Pozo del Pueblo", "Pozzo del Villaggio"), "#9a9aa4", "#7a4d24", [
    P("y", (0, 0.3, 0), (0.7, 0.6, 0.7), "couleur_2"), P("y", (0, 0.62, 0), (0.55, 0.05, 0.55), "sombre"), P("c", (0.3, 0.8, 0), (0.06, 0.8, 0.06), "couleur_3"), P("c", (-0.3, 0.8, 0), (0.06, 0.8, 0.06), "couleur_3"),
    P("c", (0, 1.22, 0), (0.8, 0.06, 0.5), "couleur_3", None, (0, 0, 0)), P("n", (0, 1.38, 0), (0.9, 0.3, 0.6), "couleur_3")], roll=3, taille_case=1)
banniere = model("banniere", N("Bannière Royale", "Royal Banner", "Königliches Banner", "Estandarte Real", "Stendardo Reale"), "#c0392b", "#7a4d24", [
    P("y", (0, 0.7, 0), (0.07, 1.4, 0.07), "couleur_3"), P("c", (0.0, 1.0, 0.25), (0.03, 0.55, 0.45), "couleur_2", "fanion"), P("c", (0, 0.03, 0), (0.4, 0.06, 0.4), "sombre"),
    P("s", (0, 1.42, 0), (0.12, 0.12, 0.12), "phare")], [dict(piece="fanion", animation="balance", axe="y")], roll=4, taille_case=1)
foin = model("botte_foin", N("Botte de Foin", "Hay Bale", "Heuballen", "Fardo de Heno", "Balla di Fieno"), "#e6c566", "#a98737", [
    P("c", (0, 0.3, 0), (0.75, 0.6, 0.6), "couleur_2"), P("c", (0, 0.3, 0.0), (0.04, 0.62, 0.62), "couleur_3"), P("c", (0.2, 0.3, 0.0), (0.04, 0.62, 0.62), "couleur_3"), P("c", (-0.2, 0.3, 0.0), (0.04, 0.62, 0.62), "couleur_3")], roll=5, taille_case=1)
obstacles = [chene, rocher, puits, banniere, foin]


def tour(x, h, nom="t"):
    return [P("y", (x, h / 2, 0), (1.0, h, 1.0), "couleur_2", nom), P("y", (x, h + 0.08, 0), (1.15, 0.16, 1.15), "couleur_3", nom), P("n", (x, h + 0.55, 0), (1.2, 0.9, 1.2), "couleur_3", nom),
            P("c", (x, h * 0.6, 0.5), (0.18, 0.35, 0.03), "sombre", nom)]


chateau = model("chateau", N("Château du Roi Crapaud", "Toad King's Castle", "Schloss des Krötenkönigs", "Castillo del Rey Sapo", "Castello del Re Rospo"), "#b9b4aa", "#c0392b",
    tour(-1.2, 2.6) + tour(1.2, 2.6) + [P("c", (0, 1.0, 0), (2.4, 2.0, 1.0), "couleur_2"), P("c", (0, 2.1, 0), (2.5, 0.2, 1.1), "couleur_2"),
    P("c", (0, 0.5, 0.52), (0.7, 1.0, 0.04), "sombre"), P("n", (0, 0.95, 0.52), (0.7, 0.3, 0.04), "sombre"), P("c", (-0.6, 2.35, 0), (0.3, 0.3, 0.3), "couleur_2"), P("c", (0.0, 2.35, 0), (0.3, 0.3, 0.3), "couleur_2"),
    P("c", (0.6, 2.35, 0), (0.3, 0.3, 0.3), "couleur_2"), P("y", (0, 3.0, 0), (0.05, 1.0, 0.05), "sombre"), P("c", (0.0, 3.35, 0.25), (0.03, 0.3, 0.5), "couleur_3", "fanion")], [dict(piece="fanion", animation="balance", axe="y")])
moulin_p = [P("c", (0, 0.9, 0), (1.0, 1.8, 1.0), "couleur_2", None, (0, 0, 0)), P("n", (0, 2.2, 0), (1.3, 1.0, 1.3), "couleur_3"), P("c", (0, 0.4, 0.51), (0.4, 0.8, 0.03), "sombre"),
            P("y", (0.6, 1.8, 0), (0.12, 0.3, 0.12), "sombre", None, (0, 0, 90))]
for k in range(4):
    moulin_p += [P("c", (0.7, 1.8, 0), (0.06, 1.7, 0.3), "couleur_3", "ailes", (0, 0, 90 * k))]
moulin = model("moulin", N("Moulin à Vent", "Windmill", "Windmühle", "Molino de Viento", "Mulino a Vento"), "#e8dcc0", "#8a5a2b", moulin_p, [dict(piece="ailes", animation="rotation_groupe")])
veille = model("tour_de_guet", N("Tour de Guet", "Watchtower", "Wachturm", "Torre de Vigilancia", "Torre di Guardia"), "#a8a49a", "#7a4d24",
               tour(0, 2.8) + [P("c", (0, 3.0, 0), (1.3, 0.2, 1.3), "couleur_3"), P("c", (0, 0.05, 0), (1.3, 0.1, 1.3), "sombre")])
bords = [chateau, moulin, veille]
deco = [
    model("marguerites", N("Marguerites", "Daisies", "Gänseblümchen", "Margaritas", "Margherite"), "#f4f4f0", "#ffd23f", [P("y", (0, 0.05, 0), (0.02, 0.1, 0.02), "sombre"), P("s", (0, 0.11, 0), (0.1, 0.03, 0.1), "couleur_2"), P("s", (0, 0.12, 0), (0.04, 0.03, 0.04), "couleur_3")]),
    model("fleche", N("Flèche Plantée", "Stray Arrow", "Verirrter Pfeil", "Flecha Clavada", "Freccia Piantata"), "#c9a063", "#e8e8f0", [P("y", (0, 0.2, 0), (0.025, 0.4, 0.025), "couleur_2", None, (0, 0, 15)), P("n", (0.05, 0.38, 0), (0.06, 0.1, 0.06), "couleur_3", None, (0, 0, 15))]),
    model("piece_or", N("Pièce d'Or", "Gold Coin", "Goldmünze", "Moneda de Oro", "Moneta d'Oro"), "#ffd23f", "#e0a800", [P("y", (0, 0.015, 0), (0.14, 0.03, 0.14), "couleur_2"), P("y", (0, 0.03, 0), (0.09, 0.015, 0.09), "couleur_3")]),
    model("bouclier", N("Bouclier Oublié", "Forgotten Shield", "Vergessener Schild", "Escudo Olvidado", "Scudo Dimenticato"), "#c0392b", "#e8e8f0", [P("y", (0, 0.02, 0), (0.3, 0.03, 0.3), "couleur_2"), P("c", (0, 0.045, 0), (0.28, 0.02, 0.05), "couleur_3"), P("c", (0, 0.045, 0), (0.05, 0.02, 0.28), "couleur_3")]),
]


# =============================================================== CRÉATURES
def creature(id, classe, noms, longueur, largeur, comportement, vitesse, dangereux, pts, c2, c3, pieces, mobiles):
    return dict(id=id, univers="moyen_age", classe=classe, noms=noms, longueur=longueur, largeur=largeur, comportement=comportement, vitesse=vitesse,
                dangereux=dangereux, points_deblocage=pts, couleur_2=c2, couleur_3=c3, pieces=pieces, parties_mobiles=mobiles)


def eyes(x, y, dz, d, mat="noir"):
    return [P("s", (x, y, dz), (d, d, d), mat), P("s", (x, y, -dz), (d, d, d), mat)]


def horse(pre="", dx=0.0, dz=0.0, rider=False):
    """Cheval qui galope, +X. c2 = robe, c3 = crinière ; cavalier en armure si rider."""
    p = [P("s", (0.0, 0.78, 0), (1.05, 0.52, 0.46), "couleur_2", pre + "corps"), P("s", (0.3, 0.84, 0), (0.55, 0.58, 0.44), "couleur_2", pre + "corps"),
         P("s", (-0.32, 0.8, 0), (0.55, 0.55, 0.46), "couleur_2", pre + "corps"), P("s", (0.58, 1.05, 0), (0.3, 0.6, 0.26), "couleur_2", pre + "corps", (0, 0, -32)),
         P("s", (0.84, 1.22, 0), (0.42, 0.26, 0.22), "couleur_2", pre + "corps"), P("s", (1.02, 1.15, 0), (0.24, 0.2, 0.18), "couleur_3", pre + "corps"),
         P("n", (0.78, 1.42, 0.07), (0.07, 0.14, 0.05), "couleur_2", pre + "corps"), P("n", (0.78, 1.42, -0.07), (0.07, 0.14, 0.05), "couleur_2", pre + "corps"),
         P("s", (0.92, 1.27, 0.1), (0.05, 0.05, 0.05), "noir", pre + "corps"), P("s", (0.92, 1.27, -0.1), (0.05, 0.05, 0.05), "noir", pre + "corps"),
         P("n", (-0.62, 0.78, 0), (0.12, 0.55, 0.12), "couleur_3", pre + "queue", (0, 0, 110)), P("c", (0.0, 0.98, 0), (0.5, 0.06, 0.5), "rouge" if False else "sombre", pre + "corps")]
    anims = [dict(piece=pre + "corps", animation="rebond", amplitude=0.05), dict(piece=pre + "queue", animation="balance", axe="y")]
    for k, (x, z, ph) in enumerate(((0.4, 0.14, 0.0), (0.4, -0.14, 0.5), (-0.4, 0.14, 0.5), (-0.4, -0.14, 0.0))):
        nom = f"{pre}patte{k}"
        p += [P("y", (x, 0.34, z), (0.12, 0.7, 0.12), "couleur_2", nom), P("c", (x + 0.03, 0.03, z), (0.18, 0.06, 0.14), "noir", nom)]
        anims.append(swing(nom, ph))
    if rider:
        p += [P("c", (-0.02, 1.32, 0), (0.32, 0.46, 0.32), "blanc", pre + "corps"), P("s", (0.02, 1.7, 0), (0.26, 0.28, 0.26), "blanc", pre + "corps"), P("c", (0.15, 1.7, 0), (0.04, 0.05, 0.2), "noir", pre + "corps"),
              P("n", (0.0, 1.98, 0), (0.1, 0.3, 0.1), "lumiere_rouge", pre + "corps"), P("y", (0.5, 1.5, 0.28), (0.04, 1.6, 0.04), "sombre", pre + "corps", (0, 0, -75)),
              P("c", (0.1, 1.15, 0.3), (0.06, 0.55, 0.36), "couleur_3", pre + "corps")]
    return shift(p, dx, 0, dz), anims


poule = [P("s", (0, 0.32, 0), (0.45, 0.4, 0.34), "blanc", "corps"), P("s", (0.22, 0.5, 0), (0.22, 0.24, 0.2), "blanc", "corps"), P("n", (0.36, 0.5, 0), (0.1, 0.12, 0.08), "couleur_3", "corps", (0, 0, -90)),
         P("s", (0.24, 0.64, 0), (0.1, 0.1, 0.06), "couleur_2", "corps"), P("s", (0.3, 0.54, 0.07), (0.04, 0.04, 0.04), "noir", "corps"), P("s", (0.3, 0.54, -0.07), (0.04, 0.04, 0.04), "noir", "corps"),
         P("c", (-0.28, 0.45, 0), (0.2, 0.2, 0.1), "blanc", "corps", (0, 0, 30))]
poule_m = [dict(piece="corps", animation="rebond", amplitude=0.04)]
for k, (z, ph) in enumerate(((0.07, 0.0), (-0.07, 0.5))):
    poule += [P("y", (0.0, 0.12, z), (0.04, 0.24, 0.04), "couleur_3", f"patte{k}")]
    poule_m.append(swing(f"patte{k}", ph))
cochon = [P("s", (0, 0.4, 0), (0.7, 0.42, 0.42), "couleur_2", "corps"), P("s", (0.38, 0.45, 0), (0.3, 0.3, 0.3), "couleur_2", "corps"), P("c", (0.55, 0.42, 0), (0.1, 0.12, 0.16), "couleur_3", "corps"),
          P("n", (0.35, 0.65, 0.1), (0.08, 0.12, 0.06), "couleur_2", "corps"), P("n", (0.35, 0.65, -0.1), (0.08, 0.12, 0.06), "couleur_2", "corps"), P("s", (0.5, 0.52, 0.08), (0.05, 0.05, 0.05), "noir", "corps"),
          P("s", (0.5, 0.52, -0.08), (0.05, 0.05, 0.05), "noir", "corps"), P("y", (-0.42, 0.5, 0), (0.04, 0.2, 0.04), "couleur_3", "queue", (0, 0, 60))]
cochon_m = [dict(piece="corps", animation="rebond", amplitude=0.03), dict(piece="queue", animation="balance", axe="y")]
for k, (x, z, ph) in enumerate(((0.22, 0.12, 0.0), (0.22, -0.12, 0.5), (-0.22, 0.12, 0.5), (-0.22, -0.12, 0.0))):
    cochon += [P("y", (x, 0.14, z), (0.1, 0.28, 0.1), "couleur_3", f"patte{k}")]
    cochon_m.append(swing(f"patte{k}", ph))
gobelin = [P("c", (0, 0.5, 0), (0.28, 0.4, 0.3), "couleur_3", "corps"), P("s", (0.04, 0.82, 0), (0.3, 0.28, 0.28), "couleur_2", "corps")] + eyes(0.18, 0.86, 0.08, 0.07, "lumiere_rouge") + [
    P("n", (0.0, 0.88, 0.2), (0.1, 0.24, 0.05), "couleur_2", "corps", (60, 0, 0)), P("n", (0.0, 0.88, -0.2), (0.1, 0.24, 0.05), "couleur_2", "corps", (-60, 0, 0)),
    P("n", (0.2, 0.78, 0), (0.06, 0.1, 0.06), "couleur_2", "corps", (0, 0, -90)), P("c", (0.25, 0.5, 0.2), (0.04, 0.3, 0.04), "blanc", "corps", (0, 0, -70))]
gob_m = [dict(piece="corps", animation="rebond", amplitude=0.04)]
for nom, z, ph in (("patte_g", 0.08, 0.0), ("patte_d", -0.08, 0.5)):
    gobelin += [P("y", (0, 0.15, z), (0.08, 0.3, 0.08), "couleur_2", nom)]
    gob_m.append(swing(nom, ph))
chevalier = [P("c", (0, 0.95, 0), (0.46, 0.7, 0.6), "blanc", "corps"), P("s", (0.04, 1.45, 0), (0.4, 0.4, 0.4), "blanc", "corps"), P("c", (0.22, 1.45, 0), (0.04, 0.06, 0.26), "noir", "corps"),
             P("n", (0.0, 1.8, 0), (0.12, 0.34, 0.12), "lumiere_rouge", "corps"), P("c", (0.28, 0.95, 0.38), (0.06, 0.7, 0.5), "couleur_3", "bouclier"), P("c", (0.31, 0.95, 0.38), (0.02, 0.16, 0.36), "blanc", "bouclier"),
             P("c", (0.0, 0.5, 0), (0.44, 0.2, 0.58), "sombre", "corps")]
chev_m = [dict(piece="corps", animation="rebond", amplitude=0.03), dict(piece="bouclier", animation="balance", axe="y")]
chevalier += [P("y", (0.4, 1.2, -0.38), (0.05, 1.0, 0.05), "blanc", "epee", (0, 0, -35)), P("c", (0.25, 0.85, -0.38), (0.04, 0.04, 0.2), "couleur_3", "epee")]
chev_m.append(dict(piece="epee", animation="balance", axe="z"))
for nom, z, ph in (("patte_g", 0.14, 0.0), ("patte_d", -0.14, 0.5)):
    chevalier += [P("y", (0, 0.3, z), (0.16, 0.6, 0.16), "blanc", nom), P("c", (0.09, 0.04, z), (0.3, 0.08, 0.18), "sombre", nom)]
    chev_m.append(swing(nom, ph))
sanglier = [P("s", (0, 0.5, 0), (0.95, 0.6, 0.5), "couleur_2", "corps"), P("s", (0.5, 0.52, 0), (0.45, 0.48, 0.44), "couleur_2", "corps"), P("c", (0.76, 0.46, 0), (0.18, 0.2, 0.22), "couleur_3", "corps"),
            P("n", (0.78, 0.52, 0.14), (0.05, 0.2, 0.05), "blanc", "corps", (0, 0, -75)), P("n", (0.78, 0.52, -0.14), (0.05, 0.2, 0.05), "blanc", "corps", (0, 0, -75)),
            P("s", (0.62, 0.64, 0.14), (0.05, 0.05, 0.05), "noir", "corps"), P("s", (0.62, 0.64, -0.14), (0.05, 0.05, 0.05), "noir", "corps"),
            P("n", (0.42, 0.8, 0.12), (0.08, 0.14, 0.05), "couleur_2", "corps"), P("n", (0.42, 0.8, -0.12), (0.08, 0.14, 0.05), "couleur_2", "corps"), P("n", (0.0, 0.85, 0), (0.8, 0.2, 0.1), "couleur_3", "corps", (0, 0, -90)),
            P("y", (-0.52, 0.55, 0), (0.04, 0.2, 0.04), "couleur_3", "queue", (0, 0, 60))]
sang_m = [dict(piece="corps", animation="rebond", amplitude=0.04), dict(piece="queue", animation="balance", axe="y")]
for k, (x, z, ph) in enumerate(((0.3, 0.16, 0.0), (0.3, -0.16, 0.5), (-0.3, 0.16, 0.5), (-0.3, -0.16, 0.0))):
    sanglier += [P("y", (x, 0.2, z), (0.1, 0.4, 0.1), "couleur_3", f"patte{k}")]
    sang_m.append(swing(f"patte{k}", ph))
cav_p, cav_a = horse(rider=True)
atte_p, atte_a = horse("a", 0.0, 0.0)
charrette = [P("c", (-1.2, 0.55, 0), (1.4, 0.12, 0.8), "couleur_3"), P("c", (-1.2, 0.75, 0.4), (1.4, 0.3, 0.04), "couleur_3"), P("c", (-1.2, 0.75, -0.4), (1.4, 0.3, 0.04), "couleur_3"),
             P("c", (-1.9, 0.75, 0), (0.04, 0.3, 0.8), "couleur_3"), P("y", (-1.0, 0.95, 0.1), (0.4, 0.5, 0.4), "couleur_3"), P("y", (-1.5, 0.95, -0.1), (0.4, 0.5, 0.4), "couleur_3"),
             P("y", (-1.0, 1.05, 0.1), (0.42, 0.04, 0.42), "sombre"), P("c", (-0.4, 0.55, 0), (0.7, 0.04, 0.08), "sombre"), P("c", (-0.3, 1.0, 0), (0.1, 0.4, 0.5), "couleur_2")]
for x in (-1.5, -0.9):
    for z in (0.46, -0.46):
        charrette += [P("y", (x, 0.3, z), (0.6, 0.08, 0.6), "sombre", None, (90, 0, 0)), P("y", (x, 0.3, z + (0.045 if z > 0 else -0.045)), (0.3, 0.05, 0.3), "couleur_2", None, (90, 0, 0))]
ch_c = charrette + shift(atte_p, 0.2, 0, 0)
ch_ca = atte_a
ogre = [P("s", (0, 1.15, 0), (1.0, 1.3, 0.9), "couleur_2", "corps"), P("s", (0.1, 2.0, 0), (0.62, 0.62, 0.6), "couleur_2", "corps")] + eyes(0.38, 2.05, 0.15, 0.1, "blanc") + [
    P("s", (0.42, 2.05, 0.15), (0.05, 0.05, 0.05), "noir", "corps"), P("s", (0.42, 2.05, -0.15), (0.05, 0.05, 0.05), "noir", "corps"), P("c", (0.4, 1.85, 0), (0.06, 0.08, 0.36), "noir", "corps"),
    P("n", (0.35, 1.9, 0.12), (0.06, 0.12, 0.05), "blanc", "corps", (180, 0, 0)), P("n", (0.35, 1.9, -0.12), (0.06, 0.12, 0.05), "blanc", "corps", (180, 0, 0)),
    P("c", (0.0, 0.85, 0), (0.95, 0.3, 0.9), "couleur_3", "corps"), P("y", (0.55, 1.75, -0.85), (0.14, 1.8, 0.14), "couleur_3", "massue", (0, 0, -20)), P("s", (0.8, 2.6, -0.85), (0.4, 0.5, 0.4), "couleur_3", "massue")]
og_m = [dict(piece="corps", animation="rebond", amplitude=0.03), dict(piece="massue", animation="balance", axe="z")]
for nom, z, ph in (("bras_g", 0.62, 0.5),):
    ogre += [P("y", (0, 1.3, z), (0.3, 1.1, 0.3), "couleur_2", nom)]
    og_m.append(swing(nom, ph))
for nom, z, ph in (("patte_g", 0.28, 0.0), ("patte_d", -0.28, 0.5)):
    ogre += [P("y", (0, 0.4, z), (0.34, 0.8, 0.34), "couleur_2", nom), P("c", (0.15, 0.06, z), (0.56, 0.12, 0.4), "couleur_3", nom)]
    og_m.append(swing(nom, ph))
catapulte = [P("c", (0, 0.5, 0), (1.9, 0.18, 0.9), "couleur_3"), P("c", (-0.2, 1.0, 0.3), (0.14, 1.0, 0.14), "couleur_3"), P("c", (-0.2, 1.0, -0.3), (0.14, 1.0, 0.14), "couleur_3"),
             P("c", (0.8, 0.75, 0.3), (0.1, 0.5, 0.1), "couleur_3"), P("c", (0.8, 0.75, -0.3), (0.1, 0.5, 0.1), "couleur_3"), P("y", (-0.2, 1.5, 0), (0.12, 0.8, 0.12), "sombre", None, (90, 0, 0)),
             P("c", (0.4, 1.4, 0), (1.5, 0.1, 0.1), "couleur_2", "bras", (0, 0, 20)), P("s", (1.1, 1.65, 0), (0.4, 0.3, 0.4), "sombre", "bras"), P("c", (-0.4, 1.2, 0), (0.3, 0.3, 0.3), "couleur_3", "bras")]
for x in (-0.6, 0.6):
    for z in (0.5, -0.5):
        catapulte += [P("y", (x, 0.34, z), (0.68, 0.1, 0.68), "sombre", None, (90, 0, 0)), P("y", (x, 0.34, z + (0.055 if z > 0 else -0.055)), (0.3, 0.06, 0.3), "couleur_2", None, (90, 0, 0))]
cat_m = [dict(piece="bras", animation="balance", axe="z"), dict(piece="*", animation="vibre")] if False else [dict(piece="bras", animation="balance", axe="z")]
dragon = [P("s", (0, 1.0, 0), (1.1, 0.5, 0.5), "couleur_2", "corps"), P("s", (0.55, 1.1, 0), (0.45, 0.42, 0.4), "couleur_2", "corps"), P("n", (0.82, 1.1, 0), (0.28, 0.4, 0.28), "couleur_2", "corps", (0, 0, -90)),
          P("n", (0.4, 1.38, 0.1), (0.06, 0.3, 0.06), "couleur_3", "corps", (0, 0, 10)), P("n", (0.4, 1.38, -0.1), (0.06, 0.3, 0.06), "couleur_3", "corps", (0, 0, 10)),
          P("s", (0.62, 1.2, 0.12), (0.07, 0.07, 0.05), "lumiere_rouge", "corps"), P("s", (0.62, 1.2, -0.12), (0.07, 0.07, 0.05), "lumiere_rouge", "corps"),
          P("n", (1.0, 1.1, 0), (0.18, 0.3, 0.18), "lumiere_rouge", "flamme", (0, 0, -90)), P("n", (-0.7, 0.95, 0), (0.3, 1.0, 0.3), "couleur_2", "corps", (0, 0, 105)), P("n", (-1.2, 0.9, 0), (0.12, 0.3, 0.12), "couleur_3", "corps", (0, 0, 105))]
for i, x in enumerate((0.3, 0.0, -0.3, -0.6)):
    dragon.append(P("n", (x, 1.32, 0), (0.1, 0.22, 0.08), "couleur_3", "corps"))
for nom, s in (("aile_g", 1), ("aile_d", -1)):
    dragon += [P("c", (0.0, 1.1, 0.7 * s), (0.7, 0.04, 1.0), "couleur_3", nom), P("c", (-0.2, 1.1, 1.3 * s), (0.6, 0.04, 0.6), "couleur_3", nom), P("c", (0.1, 1.11, 1.6 * s), (0.08, 0.05, 1.0), "couleur_2", nom),
               P("c", (-0.25, 1.11, 1.5 * s), (0.08, 0.05, 0.8), "couleur_2", nom)]
dr_m = [dict(piece="corps", animation="rebond", amplitude=0.04), dict(piece="flamme", animation="pulse"), dict(piece="aile_g", animation="pendule", phase=0.0, axe="x", pivot="base"), dict(piece="aile_d", animation="pendule", phase=0.0, axe="x", pivot="base")]

animaux = [
    creature("poule", "petit", N("Poule Pressée", "Hurried Hen", "Eilige Henne", "Gallina con Prisa", "Gallina Frettolosa"), 0.6, 0.4, "traverse", 2.0, False, 300, "#f4f0e0", "#e8a02a", poule, poule_m),
    creature("cochon", "petit", N("Cochon Gourmand", "Greedy Pig", "Gieriges Schwein", "Cerdo Glotón", "Maiale Goloso"), 0.9, 0.5, "traverse", 1.6, False, 450, "#f2b2b8", "#d88a94", cochon, cochon_m),
    creature("gobelin", "petit", N("Gobelin Voleur", "Thieving Goblin", "Diebischer Kobold", "Duende Ladrón", "Goblin Ladro"), 0.6, 0.5, "traverse", 3.4, True, 700, "#6fb04a", "#5a3f2a", gobelin, gob_m),
    creature("chevalier_pied", "moyen", N("Chevalier Bruyant", "Clanking Knight", "Scheppernder Ritter", "Caballero Ruidoso", "Cavaliere Rumoroso"), 0.9, 0.9, "traverse", 3.0, True, 1000, "#d8dce8", "#c0392b", chevalier, chev_m),
    creature("sanglier", "moyen", N("Sanglier Furieux", "Furious Boar", "Wütender Eber", "Jabalí Furioso", "Cinghiale Furioso"), 1.3, 0.6, "charge", 4.0, True, 1300, "#6b4a30", "#3a2a1a", sanglier, sang_m),
    creature("cavalier", "moyen", N("Chevalier au Galop", "Galloping Knight", "Galoppierender Ritter", "Caballero al Galope", "Cavaliere al Galoppo"), 1.9, 0.6, "traverse", 4.2, True, 1700, "#8b5a2b", "#c0392b", cav_p, cav_a),
    creature("charrette", "grand", N("Charrette de Marchand", "Merchant Cart", "Händlerkarren", "Carreta de Mercader", "Carro del Mercante"), 3.3, 1.0, "traverse", 2.8, True, 2000, "#6b4a30", "#8a5a2b", ch_c, ch_ca),
    creature("ogre", "grand", N("Ogre Pas Content", "Cranky Ogre", "Mürrischer Oger", "Ogro Gruñón", "Orco Scontroso"), 1.3, 1.9, "traverse", 1.8, True, 2500, "#7aa05a", "#7a4d24", ogre, og_m),
    creature("catapulte", "grand", N("Catapulte Roulante", "Rolling Catapult", "Rollende Katapult", "Catapulta Rodante", "Catapulta Rotolante"), 2.0, 1.0, "traverse", 2.0, True, 2900, "#c9a063", "#6b4a30", catapulte, cat_m),
    creature("dragon", "volant", N("Dragon Cracheur", "Fire-Breathing Dragon", "Feuerspeiender Drache", "Dragón Escupefuego", "Drago Sputafuoco"), 2.2, 3.4, "vole_en_ligne", 3.6, True, 3200, "#4f9a3a", "#c0392b", dragon, dr_m),
]

# =============================================================== HÉROS : grenouille chevalier
src = json.load(open(os.path.join(IMP, "jurassic_croa.json"), encoding="utf-8"))["heros"]
hero_pieces = [p for p in src["pieces"] if p.get("nom") != "canne"]
casque = [P("s", (0.12, 0.78, 0), (0.5, 0.34, 0.5), "couleur_3"), P("y", (0.12, 0.67, 0), (0.56, 0.05, 0.56), "couleur_3"), P("c", (0.34, 0.74, 0), (0.05, 0.05, 0.3), "noir"),
          P("n", (0.1, 1.08, 0), (0.12, 0.34, 0.12), "couleur_2"), P("s", (0.1, 1.28, 0), (0.1, 0.1, 0.1), "couleur_2")]
heros = dict(id="grenouille_chevalier", univers="moyen_age", noms=N("Grenouille Chevalier", "Knight Frog", "Ritter-Frosch", "Rana Caballero", "Rana Cavaliere"), longueur=0.7, largeur=0.6,
             couleur_2="#c0392b", couleur_3="#c8ccd8", pieces=hero_pieces, parties_mobiles=[dict(piece="corps", animation="rebond", amplitude=0.02)],
             accessoire=dict(nom=N("Casque de chevalier", "Knight helmet", "Ritterhelm", "Casco de caballero", "Elmo da cavaliere"), pieces=casque))


# =============================================================== CONVOI (à la place du train)
def attelage():
    ph, ah = horse("a", 0.5, 0.24)
    pb, ab = horse("b", 0.5, -0.24)
    p = [P("c", (-0.9, 0.55, 0), (1.2, 0.1, 0.7), "couleur_3"), P("c", (-1.3, 0.85, 0), (0.5, 0.5, 0.6), "couleur_3"), P("c", (-0.9, 0.3, 0), (0.1, 0.1, 0.1), "sombre"),
         P("c", (-0.6, 0.62, 0.0), (1.0, 0.04, 0.1), "sombre"), P("s", (-1.0, 1.25, 0), (0.22, 0.24, 0.22), "couleur_2"), P("c", (-1.0, 1.0, 0), (0.3, 0.4, 0.3), "couleur_2"),
         P("c", (-1.0, 1.42, 0), (0.46, 0.04, 0.46), "sombre"), P("y", (-1.0, 1.5, 0), (0.24, 0.14, 0.24), "sombre")]
    for x in (-1.2, -0.5):
        for z in (0.42, -0.42):
            p += [P("y", (x, 0.28, z), (0.56, 0.08, 0.56), "sombre", None, (90, 0, 0)), P("y", (x, 0.28, z + (0.045 if z > 0 else -0.045)), (0.28, 0.05, 0.28), "couleur_3", None, (90, 0, 0))]
    return p + ph + pb, ah + ab


def chariot(kind):
    p = [P("c", (0, 0.55, 0), (1.9, 0.12, 0.85), "couleur_3"), P("c", (0, 0.75, 0.42), (1.9, 0.3, 0.04), "couleur_3"), P("c", (0, 0.75, -0.42), (1.9, 0.3, 0.04), "couleur_3"),
         P("c", (0.94, 0.75, 0), (0.04, 0.3, 0.85), "couleur_3"), P("c", (-0.94, 0.75, 0), (0.04, 0.3, 0.85), "couleur_3")]
    if kind == 0:
        p += [P("y", (-0.4, 1.0, 0.0), (0.5, 0.55, 0.5), "couleur_3"), P("y", (0.35, 1.0, 0.1), (0.5, 0.55, 0.5), "couleur_3"), P("y", (-0.4, 1.12, 0), (0.52, 0.04, 0.52), "sombre"), P("y", (0.35, 1.12, 0.1), (0.52, 0.04, 0.52), "sombre")]
    elif kind == 1:
        p += [P("c", (0, 1.0, 0), (1.7, 0.5, 0.7), "couleur_2"), P("c", (0, 1.28, 0), (1.5, 0.1, 0.55), "couleur_2")]
    else:
        for z in (-0.3, -0.1, 0.1, 0.3):
            p.append(P("c", (0, 1.05, z), (1.7, 0.7, 0.04), "sombre"))
        p.append(P("c", (0, 1.4, 0), (1.8, 0.05, 0.8), "sombre"))
    for x in (-0.55, 0.55):
        for z in (0.46, -0.46):
            p += [P("y", (x, 0.28, z), (0.56, 0.08, 0.56), "sombre", None, (90, 0, 0)), P("y", (x, 0.28, z + (0.045 if z > 0 else -0.045)), (0.28, 0.05, 0.28), "couleur_3", None, (90, 0, 0))]
    return p


ap, aa = attelage()
train = [
    dict(id="attelage_roi", longueur=3.6, couleur_2="#8b5a2b", couleur_3="#7a4d24", pieces=ap, parties_mobiles=aa),
    dict(id="chariot_tonneaux", longueur=2.1, couleur_2="#c0392b", couleur_3="#7a4d24", pieces=chariot(0), parties_mobiles=[]),
    dict(id="chariot_foin", longueur=2.1, couleur_2="#e6c566", couleur_3="#7a4d24", pieces=chariot(1), parties_mobiles=[]),
    dict(id="chariot_cage", longueur=2.1, couleur_2="#c0392b", couleur_3="#7a4d24", pieces=chariot(2), parties_mobiles=[]),
]
# Le premier véhicule (l'attelage) doit être en tête : spawn_old_train met la locomotive (0) puis le tender (1) puis des wagons (2, 3) : on glisse donc un chariot en « tender ».

monde = dict(id="moyen_age", nom=N("Royaume de Croâ", "Kingdom of Croak", "Quak-Königreich", "Reino de Croa", "Regno di Gra"), points_deblocage=4800,
             ciel="#9ad0f0", brume="#d6ecf6", sol_voie_route="#8d8171", marquage="#a89a86", herbe_a="#74a84c", herbe_b="#6a9a46", eau="#3f86a0", rondin="#7a4d24",
             obstacles=obstacles, bords=bords, deco=deco, voie_speciale=dict(remplace="rail", description="Convoi royal : attelage et chariots.", pieces=[]),
             vehicules_du_monde=[], animaux_du_monde=[a["id"] for a in animaux], train=train,
             musique=dict(tempo_bpm=104, gamme="dorienne", instruments=["luth", "flûte", "tambourin", "vièle"], ambiance="Ménestrel dans la plaine, un dragon plane au loin."),
             **{"bannière": N("Bienvenue au Royaume de Croâ ! Le dragon est déjà au courant.", "Welcome to the Kingdom of Croak! The dragon already knows.",
                              "Willkommen im Quak-Königreich! Der Drache weiß es schon.", "¡Bienvenido al Reino de Croa! El dragón ya lo sabe.", "Benvenuto nel Regno di Gra! Il drago lo sa già.")})
json.dump(dict(monde=monde, animaux=animaux, heros=heros), open(os.path.join(IMP, "moyenage.json"), "w", encoding="utf-8"), ensure_ascii=False, indent=1)
print(len(obstacles), "obstacles,", len(animaux), "créatures,", len(train), "voitures de convoi")
