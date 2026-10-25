#!/usr/bin/env python3
# Commit & Conquer — licence GPL-3.0-or-later.
#
# Le monde « Halloween Croâ » de Traversée : allée de cimetière, marais aux cercueils et
# citrouilles flottantes, train fantôme, créatures qui traversent, chauve-souris géante.
# Avant = +X, haut = +Y, largeur = Z. Écrit dans games/traversee/data/import/ :
#   halloween.json (monde, animaux, héros)   halloween_vehicules.json (véhicules)
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


# =============================================================== DÉCOR (obstacles : 1 case)
tombe = model("tombe", N("Tombe Tranquille", "Quiet Grave", "Stilles Grab", "Tumba Tranquila", "Tomba Tranquilla"), "#8d8f99", "#4f7a4a", [
    P("c", (0, 0.07, 0), (0.85, 0.14, 0.45), "couleur_2"), P("c", (0, 0.5, 0), (0.6, 0.8, 0.14), "couleur_2"), P("s", (0, 0.9, 0), (0.6, 0.36, 0.14), "couleur_2"),
    P("c", (0, 0.65, 0.075), (0.34, 0.04, 0.012), "noir"), P("c", (0, 0.55, 0.075), (0.26, 0.04, 0.012), "noir"), P("c", (0, 0.45, 0.075), (0.3, 0.04, 0.012), "noir"),
    P("s", (0.36, 0.1, 0.1), (0.3, 0.14, 0.26), "couleur_3"), P("s", (-0.3, 0.05, 0.15), (0.22, 0.1, 0.2), "couleur_3")], roll=1, taille_case=1)
citrouille = model("citrouille", N("Citrouille Rigolote", "Jolly Pumpkin", "Lustiger Kürbis", "Calabaza Graciosa", "Zucca Buffa"), "#ff8a1f", "#4a8a3a", [
    P("s", (0, 0.32, 0), (0.85, 0.62, 0.85), "couleur_2"), P("s", (0.26, 0.32, 0), (0.45, 0.62, 0.8), "couleur_2"), P("s", (-0.26, 0.32, 0), (0.45, 0.62, 0.8), "couleur_2"),
    P("y", (0.0, 0.68, 0), (0.12, 0.22, 0.12), "couleur_3", None, (0, 0, 12)),
    P("c", (0.16, 0.4, 0.4), (0.12, 0.12, 0.05), "phare", "feu"), P("c", (-0.16, 0.4, 0.4), (0.12, 0.12, 0.05), "phare", "feu"), P("c", (0.0, 0.22, 0.4), (0.34, 0.08, 0.05), "phare", "feu")],
    [dict(piece="feu", animation="pulse")], roll=2, taille_case=1)
arbre_mort = model("arbre_mort", N("Arbre Tordu", "Twisted Tree", "Krummer Baum", "Árbol Torcido", "Albero Contorto"), "#4a3a30", "#2e2420", [
    P("y", (0, 0.55, 0), (0.18, 1.1, 0.18), "couleur_2"), P("y", (0, 0.04, 0), (0.34, 0.08, 0.34), "couleur_3"),
    P("n", (0.25, 0.95, 0), (0.09, 0.6, 0.09), "couleur_2", None, (0, 0, -55)), P("n", (-0.22, 0.85, 0.1), (0.08, 0.55, 0.08), "couleur_2", None, (20, 0, 60)),
    P("n", (0.05, 1.2, 0.2), (0.07, 0.5, 0.07), "couleur_2", None, (-50, 0, 8)), P("n", (0.4, 1.2, 0), (0.05, 0.3, 0.05), "couleur_2", None, (0, 0, -20)),
    P("n", (-0.4, 1.05, 0.2), (0.05, 0.3, 0.05), "couleur_2", None, (0, 0, 25))], roll=3, taille_case=1)
chaudron = model("chaudron", N("Chaudron Qui Bout", "Bubbling Cauldron", "Brodelnder Kessel", "Caldero Burbujeante", "Calderone Gorgogliante"), "#2a2a30", "#7aff5a", [
    P("s", (0, 0.32, 0), (0.85, 0.62, 0.85), "couleur_2"), P("y", (0, 0.58, 0), (0.8, 0.08, 0.8), "couleur_2"), P("y", (0, 0.6, 0), (0.62, 0.03, 0.62), "couleur_3"),
    P("s", (0.1, 0.7, 0.05), (0.14, 0.14, 0.14), "couleur_3", "bulle"), P("s", (-0.12, 0.7, -0.08), (0.1, 0.1, 0.1), "couleur_3", "bulle"),
    P("y", (0.3, 0.06, 0.3), (0.1, 0.14, 0.1), "sombre"), P("y", (-0.3, 0.06, 0.3), (0.1, 0.14, 0.1), "sombre"), P("y", (0.0, 0.06, -0.38), (0.1, 0.14, 0.1), "sombre"),
    P("s", (0, 0.1, 0), (0.3, 0.15, 0.3), "lumiere_rouge", "feu")], [dict(piece="bulle", animation="derive_haut"), dict(piece="feu", animation="pulse")], roll=4, taille_case=1)
obstacles = [tombe, citrouille, arbre_mort, chaudron]
manoir = model("manoir", N("Manoir Pas Rassurant", "Not-So-Cosy Mansion", "Unheimliches Herrenhaus", "Mansión Poco Acogedora", "Villa Poco Rassicurante"), "#3d3550", "#ffcf5a", [
    P("c", (0, 0.9, 0), (2.0, 1.8, 1.4), "couleur_2"), P("n", (0, 2.4, 0), (2.5, 1.3, 1.8), "sombre", None, (0, 45, 0)), P("c", (-0.8, 2.0, 0.3), (0.6, 2.8, 0.6), "couleur_2"),
    P("n", (-0.8, 3.7, 0.3), (0.8, 0.9, 0.8), "sombre"),
    P("c", (0.5, 1.2, 0.71), (0.28, 0.42, 0.03), "couleur_3", "fenetre"), P("c", (-0.1, 1.2, 0.71), (0.28, 0.42, 0.03), "couleur_3"), P("c", (-0.8, 2.4, 0.61), (0.22, 0.4, 0.03), "couleur_3", "fenetre"),
    P("c", (0.25, 0.5, 0.71), (0.4, 0.9, 0.03), "sombre"), P("c", (0, 0.05, 0.3), (2.3, 0.1, 1.9), "sombre")], [dict(piece="fenetre", animation="clignote_lent")])
grand_arbre = model("grand_arbre_mort", N("Grand Arbre Fantomatique", "Great Ghostly Tree", "Großer Geisterbaum", "Gran Árbol Fantasmal", "Grande Albero Fantasma"), "#4a3a30", "#2e2420", [
    P("y", (0, 1.2, 0), (0.5, 2.4, 0.5), "couleur_2"), P("n", (0.7, 2.3, 0), (0.22, 1.6, 0.22), "couleur_2", None, (0, 0, -50)), P("n", (-0.7, 2.1, 0.2), (0.2, 1.5, 0.2), "couleur_2", None, (15, 0, 52)),
    P("n", (0.1, 2.6, 0.6), (0.18, 1.3, 0.18), "couleur_2", None, (-48, 0, 6)), P("n", (-0.1, 2.5, -0.6), (0.18, 1.3, 0.18), "couleur_2", None, (48, 0, -6)),
    P("n", (1.2, 2.9, 0), (0.1, 0.7, 0.1), "couleur_2", None, (0, 0, -25)), P("n", (-1.25, 2.7, 0.2), (0.1, 0.7, 0.1), "couleur_2", None, (0, 0, 28)), P("c", (0, 0.05, 0), (1.2, 0.1, 1.2), "couleur_3")])
bords = [manoir, grand_arbre]
deco = [
    model("bougie", N("Bougie", "Candle", "Kerze", "Vela", "Candela"), "#f4f0e0", "#ffcf5a", [P("y", (0, 0.1, 0), (0.08, 0.2, 0.08), "couleur_2"), P("s", (0, 0.24, 0), (0.06, 0.1, 0.06), "couleur_3", "flamme")], [dict(piece="flamme", animation="clignote_alterne")]),
    model("mini_citrouille", N("Mini Citrouille", "Mini Pumpkin", "Mini-Kürbis", "Mini Calabaza", "Mini Zucca"), "#ff8a1f", "#4a8a3a", [P("s", (0, 0.1, 0), (0.24, 0.2, 0.24), "couleur_2"), P("y", (0, 0.23, 0), (0.04, 0.08, 0.04), "couleur_3")]),
    model("os", N("Vieil Os", "Old Bone", "Alter Knochen", "Hueso Viejo", "Vecchio Osso"), "#f1ead8", "#d8d0bc", [P("c", (0, 0.03, 0), (0.3, 0.05, 0.07), "couleur_2", None, (0, 30, 0)), P("s", (0.15, 0.04, 0.05), (0.1, 0.07, 0.09), "couleur_2"), P("s", (-0.15, 0.04, -0.05), (0.1, 0.07, 0.09), "couleur_2")]),
    model("toile", N("Toile d'Araignée", "Cobweb", "Spinnennetz", "Telaraña", "Ragnatela"), "#c8c8d0", "#9a9aa4", [P("c", (0, 0.01, 0), (0.5, 0.01, 0.03), "couleur_2"), P("c", (0, 0.01, 0), (0.03, 0.01, 0.5), "couleur_2"), P("c", (0, 0.01, 0), (0.44, 0.01, 0.03), "couleur_3", None, (0, 45, 0)), P("c", (0, 0.01, 0), (0.44, 0.01, 0.03), "couleur_3", None, (0, -45, 0))]),
]

# =============================================================== CRÉATURES
def creature(id, classe, noms, longueur, largeur, comportement, vitesse, dangereux, pts, c2, c3, pieces, mobiles):
    return dict(id=id, univers="halloween", classe=classe, noms=noms, longueur=longueur, largeur=largeur, comportement=comportement, vitesse=vitesse,
                dangereux=dangereux, points_deblocage=pts, couleur_2=c2, couleur_3=c3, pieces=pieces, parties_mobiles=mobiles)


def eyes(x, y, dz, d, mat="noir"):
    return [P("s", (x, y, dz), (d, d, d), mat), P("s", (x, y, -dz), (d, d, d), mat)]


fantome = [P("s", (0, 0.6, 0), (0.55, 0.75, 0.55), "couleur_2", "corps"), P("s", (0, 0.32, 0), (0.62, 0.5, 0.62), "couleur_2", "corps")] + [
    P("s", (math.cos(a) * 0.26, 0.15, math.sin(a) * 0.26), (0.2, 0.22, 0.2), "couleur_2", "corps") for a in (0, 1.57, 3.14, 4.71)] + eyes(0.24, 0.68, 0.1, 0.09) + [
    P("s", (0.27, 0.5, 0), (0.1, 0.12, 0.08), "noir", "corps"), P("s", (0.0, 0.5, 0.34), (0.18, 0.2, 0.12), "couleur_2", "corps"), P("s", (0.0, 0.5, -0.34), (0.18, 0.2, 0.12), "couleur_2", "corps")]
citrouille_r = [P("s", (0, 0.32, 0), (0.7, 0.6, 0.7), "couleur_2", "boule"), P("y", (0, 0.66, 0), (0.1, 0.2, 0.1), "couleur_3", "boule"),
                P("c", (0.34, 0.4, 0.14), (0.05, 0.12, 0.12), "phare", "boule"), P("c", (0.34, 0.4, -0.14), (0.05, 0.12, 0.12), "phare", "boule"), P("c", (0.34, 0.22, 0), (0.05, 0.08, 0.3), "phare", "boule")]
chat = [P("s", (0, 0.4, 0), (0.62, 0.34, 0.3), "couleur_2", "corps"), P("s", (0.34, 0.5, 0), (0.3, 0.28, 0.28), "couleur_2", "corps"),
        P("n", (0.32, 0.72, 0.1), (0.1, 0.16, 0.06), "couleur_2", "corps"), P("n", (0.32, 0.72, -0.1), (0.1, 0.16, 0.06), "couleur_2", "corps"),
        P("s", (0.46, 0.55, 0.08), (0.07, 0.07, 0.05), "phare", "corps"), P("s", (0.46, 0.55, -0.08), (0.07, 0.07, 0.05), "phare", "corps"),
        P("y", (-0.45, 0.6, 0), (0.07, 0.5, 0.07), "couleur_2", "queue", (0, 0, 25))]
chat_m = [dict(piece="corps", animation="rebond", amplitude=0.03), dict(piece="queue", animation="balance", axe="y")]
for k, (x, z, ph) in enumerate(((0.2, 0.09, 0.0), (0.2, -0.09, 0.5), (-0.2, 0.09, 0.5), (-0.2, -0.09, 0.0))):
    chat += [P("y", (x, 0.2, z), (0.08, 0.4, 0.08), "couleur_2", f"patte{k}")]
    chat_m.append(swing(f"patte{k}", ph))
squelette = [P("s", (0.04, 1.5, 0), (0.34, 0.36, 0.32), "couleur_2", "corps"), P("c", (0.2, 1.4, 0), (0.12, 0.1, 0.22), "couleur_2", "corps")] + eyes(0.2, 1.54, 0.09, 0.09) + [
    P("c", (0, 1.05, 0), (0.1, 0.6, 0.1), "couleur_2", "corps"), P("c", (0, 1.22, 0), (0.16, 0.05, 0.4), "couleur_2", "corps"), P("c", (0, 1.1, 0), (0.14, 0.05, 0.36), "couleur_2", "corps"),
    P("c", (0, 0.98, 0), (0.12, 0.05, 0.3), "couleur_2", "corps"), P("c", (0, 0.72, 0), (0.18, 0.16, 0.34), "couleur_2", "corps")]
sq_m = [dict(piece="corps", animation="rebond", amplitude=0.03)]
for nom, z, ph in (("bras_g", 0.26, 0.5), ("bras_d", -0.26, 0.0)):
    squelette += [P("y", (0, 1.05, z), (0.07, 0.55, 0.07), "couleur_2", nom)]
    sq_m.append(swing(nom, ph))
for nom, z, ph in (("patte_g", 0.12, 0.0), ("patte_d", -0.12, 0.5)):
    squelette += [P("y", (0, 0.36, z), (0.09, 0.7, 0.09), "couleur_2", nom), P("c", (0.08, 0.03, z), (0.22, 0.06, 0.12), "couleur_2", nom)]
    sq_m.append(swing(nom, ph))
sorciere = [P("y", (-0.1, 0.62, 0), (0.07, 1.7, 0.07), "couleur_3", "balai", (0, 0, 90)), P("n", (-1.0, 0.62, 0), (0.28, 0.5, 0.28), "couleur_2", "balai", (0, 0, 90)),
            P("n", (0.0, 0.95, 0), (0.46, 0.55, 0.4), "couleur_3", "corps"), P("s", (0.04, 1.35, 0), (0.28, 0.3, 0.28), "couleur_2", "corps"),
            P("y", (0.04, 1.5, 0), (0.62, 0.04, 0.62), "sombre", "corps"), P("n", (0.04, 1.8, 0), (0.34, 0.6, 0.34), "sombre", "corps", (0, 0, 10)),
            P("c", (0.22, 1.32, 0), (0.12, 0.06, 0.1), "couleur_2", "corps"), P("c", (0.14, 1.4, 0.07), (0.05, 0.05, 0.05), "noir", "corps"), P("c", (0.14, 1.4, -0.07), (0.05, 0.05, 0.05), "noir", "corps"),
            P("c", (-0.55, 0.9, 0), (0.5, 0.03, 0.4), "couleur_3", "cape")]
sorc_m = [dict(piece="corps", animation="rebond", amplitude=0.04), dict(piece="cape", animation="balance", axe="y"), dict(piece="balai", animation="rebond", amplitude=0.02)]
araignee = [P("s", (0, 0.65, 0), (0.9, 0.6, 0.8), "couleur_2", "corps"), P("s", (0.52, 0.7, 0), (0.45, 0.4, 0.45), "couleur_2", "corps")] + eyes(0.72, 0.8, 0.12, 0.1, "lumiere_rouge") + [
    P("s", (-0.5, 0.75, 0), (0.7, 0.55, 0.65), "couleur_3", "corps")]
ar_m = [dict(piece="corps", animation="rebond", amplitude=0.03)]
k = 0
for z, sgn in ((0.45, 1), (-0.45, -1)):
    for x in (0.38, 0.12, -0.14, -0.4):
        nom = f"patte{k}"
        araignee += [P("y", (x, 0.5, z + sgn * 0.12), (0.07, 0.75, 0.07), "couleur_2", nom, (sgn * 35, 0, 0)), P("y", (x, 0.2, z + sgn * 0.45), (0.06, 0.5, 0.06), "couleur_2", nom, (-sgn * 25, 0, 0))]
        ar_m.append(swing(nom, 0.5 * (k % 2)))
        k += 1
zombie = [P("c", (0, 0.95, 0), (0.45, 0.8, 0.6), "couleur_3", "corps"), P("s", (0.1, 1.55, 0), (0.46, 0.46, 0.44), "couleur_2", "corps")] + eyes(0.31, 1.6, 0.12, 0.1, "blanc") + [
    P("c", (0.32, 1.45, 0), (0.05, 0.06, 0.2), "noir", "corps"), P("c", (0.0, 0.5, 0.0), (0.4, 0.3, 0.56), "sombre", "corps")]
zo_m = [dict(piece="corps", animation="rebond", amplitude=0.03)]
for nom, z, ph in (("bras_g", 0.38, 0.0), ("bras_d", -0.38, 0.5)):
    zombie += [P("c", (0.4, 1.12, z), (0.7, 0.16, 0.16), "couleur_2", nom)]
    zo_m.append(dict(piece=nom, animation="balance", axe="z"))
for nom, z, ph in (("patte_g", 0.16, 0.0), ("patte_d", -0.16, 0.5)):
    zombie += [P("y", (0, 0.3, z), (0.2, 0.6, 0.2), "sombre", nom), P("c", (0.1, 0.04, z), (0.34, 0.08, 0.2), "noir", nom)]
    zo_m.append(swing(nom, ph))
chauve = [P("s", (0, 0.9, 0), (0.55, 0.4, 0.4), "couleur_2", "corps"), P("s", (0.34, 0.95, 0), (0.34, 0.3, 0.3), "couleur_2", "corps"),
          P("n", (0.34, 1.18, 0.1), (0.1, 0.22, 0.06), "couleur_2", "corps"), P("n", (0.34, 1.18, -0.1), (0.1, 0.22, 0.06), "couleur_2", "corps"),
          P("s", (0.5, 1.0, 0.08), (0.07, 0.07, 0.05), "lumiere_rouge", "corps"), P("s", (0.5, 1.0, -0.08), (0.07, 0.07, 0.05), "lumiere_rouge", "corps"),
          P("n", (0.5, 0.88, 0.04), (0.04, 0.1, 0.03), "blanc", "corps", (180, 0, 0)), P("n", (0.5, 0.88, -0.04), (0.04, 0.1, 0.03), "blanc", "corps", (180, 0, 0))]
for nom, s in (("aile_g", 1), ("aile_d", -1)):
    chauve += [P("c", (0.0, 0.92, 0.6 * s), (0.5, 0.04, 0.9), "couleur_3", nom), P("c", (-0.1, 0.92, 1.1 * s), (0.45, 0.04, 0.5), "couleur_3", nom),
               P("c", (-0.3, 0.92, 1.5 * s), (0.3, 0.04, 0.35), "couleur_3", nom), P("c", (0.05, 0.93, 1.4 * s), (0.08, 0.05, 0.8), "couleur_2", nom)]
ch_m = [dict(piece="corps", animation="rebond", amplitude=0.04), dict(piece="aile_g", animation="pendule", phase=0.0, axe="x", pivot="base"), dict(piece="aile_d", animation="pendule", phase=0.0, axe="x", pivot="base")]

animaux = [
    creature("citrouille_roulante", "petit", N("Citrouille Roulante", "Rolling Pumpkin", "Rollender Kürbis", "Calabaza Rodante", "Zucca Rotolante"), 0.8, 0.7, "traverse", 2.0, False, 300, "#ff8a1f", "#4a8a3a", citrouille_r, [dict(piece="boule", animation="rotation_groupe")]),
    creature("fantome_petit", "petit", N("Petit Fantôme", "Little Ghost", "Kleines Gespenst", "Fantasmita", "Fantasmino"), 0.8, 0.7, "patrouille", 1.4, False, 450, "#f4f6ff", "#c8d0ff", fantome, [dict(piece="corps", animation="rebond", amplitude=0.08)]),
    creature("chat_noir", "petit", N("Chat Noir Pressé", "Hurried Black Cat", "Eiliger Kater", "Gato Negro con Prisa", "Gatto Nero Frettoloso"), 1.0, 0.4, "traverse", 3.4, False, 650, "#1c1c24", "#ffd23f", chat, chat_m),
    creature("squelette", "moyen", N("Squelette Danseur", "Dancing Skeleton", "Tanzendes Skelett", "Esqueleto Bailarín", "Scheletro Ballerino"), 0.9, 0.7, "traverse", 3.4, True, 1000, "#f1ead8", "#d8d0bc", squelette, sq_m),
    creature("sorciere_balai", "moyen", N("Sorcière sur Balai", "Witch on a Broom", "Hexe auf dem Besen", "Bruja en Escoba", "Strega sulla Scopa"), 2.2, 0.6, "traverse", 4.2, True, 1500, "#8fbf5a", "#5a3f8a", sorciere, sorc_m),
    creature("zombie", "grand", N("Zombie Pas Pressé", "Unhurried Zombie", "Gemütlicher Zombie", "Zombi Sin Prisa", "Zombie Senza Fretta"), 1.0, 0.9, "traverse", 1.4, True, 1900, "#7aa05a", "#5a4a7a", zombie, zo_m),
    creature("araignee", "grand", N("Araignée Géante", "Giant Spider", "Riesenspinne", "Araña Gigante", "Ragno Gigante"), 1.8, 1.8, "charge", 3.0, True, 2500, "#1c1c24", "#3a2a4a", araignee, ar_m),
    creature("chauve_souris", "volant", N("Chauve-Souris Colossale", "Colossal Bat", "Riesenfledermaus", "Murciélago Colosal", "Pipistrello Colossale"), 1.0, 3.0, "vole_en_ligne", 3.8, True, 3000, "#2a2433", "#4a3a5a", chauve, ch_m),
]

# =============================================================== HÉROS : grenouille sorcière
src = json.load(open(os.path.join(IMP, "jurassic_croa.json"), encoding="utf-8"))["heros"]
hero_pieces = [p for p in src["pieces"] if p.get("nom") != "canne"]
chapeau = [P("y", (0.12, 0.69, 0), (0.7, 0.03, 0.7), "sombre"), P("n", (0.1, 0.98, 0), (0.42, 0.6, 0.42), "sombre"), P("y", (0.12, 0.74, 0), (0.43, 0.06, 0.43), "couleur_3"),
           P("c", (0.34, 0.76, 0), (0.05, 0.1, 0.1), "phare"), P("n", (0.0, 1.32, 0), (0.12, 0.3, 0.12), "sombre", None, (0, 0, 40)), P("s", (-0.1, 1.45, 0), (0.1, 0.1, 0.1), "couleur_3")]
heros = dict(id="grenouille_sorciere", univers="halloween", noms=N("Grenouille Sorcière", "Witch Frog", "Hexen-Frosch", "Rana Bruja", "Rana Strega"), longueur=0.7, largeur=0.6,
             couleur_2="#6b3fa0", couleur_3="#ff8a1f", pieces=hero_pieces, parties_mobiles=[dict(piece="corps", animation="rebond", amplitude=0.02)],
             accessoire=dict(nom=N("Chapeau de sorcière", "Witch hat", "Hexenhut", "Sombrero de bruja", "Cappello da strega"), pieces=chapeau))


# =============================================================== TRAIN FANTÔME
def wheel_x(x, z, r, mat="couleur_3"):
    return [P("y", (x, r, z), (r * 2, 0.08, r * 2), "sombre", None, (90, 0, 0)), P("y", (x, r, z + (0.045 if z > 0 else -0.045)), (r * 1.1, 0.05, r * 1.1), mat, None, (90, 0, 0))]


def loco():
    p = [P("y", (0.35, 0.85, 0), (0.7, 1.8, 0.7), "sombre", "corps", (0, 0, 90)), P("y", (1.28, 0.85, 0), (0.76, 0.12, 0.76), "couleur_3", "corps", (0, 0, 90)),
         P("y", (1.0, 1.4, 0), (0.24, 0.5, 0.24), "sombre", "corps"), P("n", (1.0, 1.75, 0), (0.46, 0.3, 0.46), "sombre", "corps", (180, 0, 0)),
         P("c", (-0.65, 1.0, 0), (0.85, 1.0, 0.85), "couleur_2", "corps"), P("c", (-0.65, 1.55, 0), (1.0, 0.08, 1.0), "sombre", "corps"),
         P("c", (-0.65, 1.05, 0.43), (0.4, 0.4, 0.04), "phare", "corps"), P("c", (-0.65, 1.05, -0.43), (0.4, 0.4, 0.04), "phare", "corps"),
         P("c", (1.5, 0.3, 0), (0.5, 0.12, 0.8), "sombre", "corps", (0, 0, 28)), P("s", (1.32, 0.85, 0.0), (0.3, 0.3, 0.3), "blanc", "corps"),
         P("s", (1.45, 0.92, 0.1), (0.1, 0.1, 0.1), "noir", "corps"), P("s", (1.45, 0.92, -0.1), (0.1, 0.1, 0.1), "noir", "corps"),
         P("c", (0.35, 0.28, 0), (1.8, 0.14, 0.7), "sombre", "corps")]
    for x, r in ((-0.3, 0.38), (0.5, 0.38), (1.15, 0.22)):
        for z in (0.4, -0.4):
            p += [P("y", (x, r, z), (r * 2, 0.08, r * 2), "sombre", "corps", (90, 0, 0)), P("y", (x, r, z + (0.045 if z > 0 else -0.045)), (r * 1.1, 0.05, r * 1.1), "couleur_3", "corps", (90, 0, 0))]
    for i, y in enumerate((1.95, 2.2, 2.45)):
        p.append(P("s", (1.0 + i * 0.05, y, 0), (0.3 + i * 0.1, 0.3 + i * 0.1, 0.3 + i * 0.1), "couleur_3", "fumee"))
    return p, [dict(piece="fumee", animation="derive_haut"), dict(piece="corps", animation="rebond", amplitude=0.012)]


def tender():
    p = [P("c", (0, 0.6, 0), (1.4, 0.7, 0.8), "couleur_2"), P("c", (0, 1.0, 0), (1.3, 0.2, 0.7), "noir"), P("s", (0.2, 1.15, 0), (0.5, 0.3, 0.4), "couleur_3"), P("c", (0, 0.25, 0), (1.5, 0.12, 0.7), "sombre")]
    for x in (-0.4, 0.4):
        for z in (0.4, -0.4):
            p += wheel_x(x, z, 0.25)
    return p, []


def wagon(variante):
    p = [P("c", (0, 0.75, 0), (1.9, 0.9, 0.85), "couleur_2"), P("c", (0, 1.25, 0), (2.0, 0.08, 0.95), "sombre"), P("c", (0, 0.25, 0), (2.0, 0.12, 0.7), "sombre")]
    for x in ((-0.5, 0.5) if variante == 0 else (-0.7, 0.0, 0.7)):
        p += [P("c", (x, 0.85, 0.43), (0.3, 0.4, 0.03), "phare"), P("c", (x, 0.85, -0.43), (0.3, 0.4, 0.03), "phare")]
    if variante == 1:
        p += [P("c", (0, 1.4, 0), (0.5, 0.2, 0.4), "couleur_3")]
    for x in (-0.6, 0.6):
        for z in (0.4, -0.4):
            p += wheel_x(x, z, 0.25, "couleur_3")
    return p, []


lp, la = loco()
tp, _ = tender()
w1, _ = wagon(0)
w2, _ = wagon(1)
train = [
    dict(id="locomotive_fantome", longueur=2.9, couleur_2="#2a2540", couleur_3="#6bff7a", pieces=lp, parties_mobiles=la),
    dict(id="tender_fantome", longueur=1.6, couleur_2="#2a2540", couleur_3="#6bff7a", pieces=tp, parties_mobiles=[]),
    dict(id="wagon_hante", longueur=2.1, couleur_2="#3d3550", couleur_3="#ffcf5a", pieces=w1, parties_mobiles=[]),
    dict(id="wagon_cage", longueur=2.1, couleur_2="#3d3550", couleur_3="#ff8a1f", pieces=w2, parties_mobiles=[]),
]

monde = dict(id="halloween", nom=N("Halloween Croâ", "Croak-o-ween", "Quak-o-ween", "Croa-ween", "Gra-lloween"), points_deblocage=3600,
             ciel="#2a1a3a", brume="#3a2a4a", sol_voie_route="#6a6258", marquage="#8a8478", herbe_a="#4a4a62", herbe_b="#42425a", eau="#4f7a3a", rondin="#5a3a24",
             obstacles=obstacles, bords=bords, deco=deco, voie_speciale=dict(remplace="route", description="Allée de cimetière.", pieces=[]),
             vehicules_du_monde=["corbillard", "carrosse_citrouille", "camion_bonbons"], animaux_du_monde=[a["id"] for a in animaux], train=train,
             musique=dict(tempo_bpm=100, gamme="mineure harmonique", instruments=["orgue", "clavecin", "cloches", "violons"], ambiance="Nuit d'Halloween, un peu de frissons et beaucoup de bonbons."),
             **{"bannière": N("Bienvenue à Halloween Croâ ! Si un fantôme vous salue, ne répondez pas.", "Welcome to Croak-o-ween! If a ghost waves at you, don't wave back.",
                              "Willkommen bei Quak-o-ween! Wenn ein Geist winkt, nicht zurückwinken.", "¡Bienvenido a Croa-ween! Si un fantasma te saluda, no respondas.",
                              "Benvenuto a Gra-lloween! Se un fantasma ti saluta, non rispondere.")})
json.dump(dict(monde=monde, animaux=animaux, heros=heros), open(os.path.join(IMP, "halloween.json"), "w", encoding="utf-8"), ensure_ascii=False, indent=1)


# =============================================================== VÉHICULES
def V(id, classe, longueur, largeur, noms, pts, c2, c3, pieces, mobiles):
    return dict(id=id, univers="halloween", classe=classe, noms=noms, longueur=longueur, largeur=largeur, points_deblocage=pts, couleur_2=c2, couleur_3=c3, pieces=pieces, parties_mobiles=mobiles)


def wheels(xs, y, d, zt):
    out = []
    for x in xs:
        for z in (zt, -zt):
            out += [P("y", (x, y, z), (d, 0.11, d), "pneu", None, (90, 0, 0)), P("y", (x, y, z + (0.005 if z > 0 else -0.005)), (d * 0.55, 0.13, d * 0.55), "enjoliveur", None, (90, 0, 0))]
    return out


corbillard = [P("c", (0, 0.2, 0), (2.5, 0.1, 0.7), "sombre"), P("c", (0, 0.5, 0), (2.7, 0.45, 0.86), "couleur_2"), P("c", (0.95, 0.85, 0), (0.7, 0.3, 0.8), "couleur_2"),
              P("c", (0.95, 0.88, 0.41), (0.5, 0.2, 0.02), "vitre"), P("c", (0.95, 0.88, -0.41), (0.5, 0.2, 0.02), "vitre"), P("c", (1.31, 0.88, 0), (0.02, 0.2, 0.7), "vitre"),
              P("c", (-0.35, 0.95, 0), (1.5, 0.45, 0.78), "vitre"), P("c", (-0.35, 0.72, 0), (1.55, 0.04, 0.82), "couleur_3"), P("c", (-0.35, 1.2, 0), (1.55, 0.05, 0.82), "couleur_2"),
              P("c", (-0.35, 0.95, 0), (1.2, 0.3, 0.4), "couleur_3"), P("c", (1.35, 0.35, 0.28), (0.05, 0.08, 0.14), "phare"), P("c", (1.35, 0.35, -0.28), (0.05, 0.08, 0.14), "phare"),
              P("c", (-1.36, 0.35, 0.28), (0.05, 0.08, 0.14), "feu_arriere"), P("c", (-1.36, 0.35, -0.28), (0.05, 0.08, 0.14), "feu_arriere"),
              P("s", (1.1, 1.08, 0), (0.1, 0.1, 0.1), "lumiere_bleue", "gyro")] + wheels([0.9, -0.9], 0.18, 0.36, 0.42)
carrosse = [P("s", (0, 0.72, 0), (1.5, 1.05, 1.0), "couleur_2"), P("s", (0.3, 0.72, 0), (0.7, 1.05, 0.98), "couleur_2"), P("s", (-0.3, 0.72, 0), (0.7, 1.05, 0.98), "couleur_2"),
            P("y", (0, 1.28, 0), (0.12, 0.2, 0.12), "couleur_3"), P("c", (0.1, 0.8, 0.5), (0.45, 0.3, 0.03), "phare"), P("c", (0.1, 0.8, -0.5), (0.45, 0.3, 0.03), "phare"),
            P("c", (0.0, 0.3, 0), (1.6, 0.1, 0.7), "sombre"), P("c", (0.9, 0.34, 0), (0.15, 0.1, 0.5), "couleur_3"), P("s", (1.0, 0.5, 0.0), (0.12, 0.12, 0.12), "lumiere_rouge", "feu")] + wheels([0.55, -0.55], 0.28, 0.56, 0.5)
camion = [P("c", (0, 0.25, 0), (2.9, 0.1, 0.7), "sombre"), P("c", (0.95, 0.6, 0), (0.9, 0.6, 0.88), "couleur_2"), P("c", (1.15, 0.9, 0), (0.45, 0.2, 0.8), "vitre"),
          P("c", (-0.4, 0.95, 0), (1.9, 1.1, 0.95), "couleur_3"), P("c", (-0.4, 0.62, 0.48), (1.9, 0.14, 0.02), "couleur_2"), P("c", (-0.4, 0.95, 0.48), (1.9, 0.14, 0.02), "couleur_2"),
          P("c", (-0.4, 1.28, 0.48), (1.9, 0.14, 0.02), "couleur_2"), P("c", (-0.4, 0.62, -0.48), (1.9, 0.14, 0.02), "couleur_2"), P("c", (-0.4, 0.95, -0.48), (1.9, 0.14, 0.02), "couleur_2"),
          P("c", (-0.4, 1.28, -0.48), (1.9, 0.14, 0.02), "couleur_2"), P("s", (0.9, 1.15, 0), (0.3, 0.26, 0.3), "couleur_3", "clou"), P("c", (1.46, 0.5, 0.28), (0.04, 0.08, 0.14), "phare"),
          P("c", (1.46, 0.5, -0.28), (0.04, 0.08, 0.14), "phare"), P("s", (-1.0, 1.7, 0), (0.18, 0.18, 0.18), "lumiere_rouge", "gyro")] + wheels([1.0, -0.2, -1.1], 0.26, 0.52, 0.42)
vehicules = [
    V("corbillard", "voiture", 2.8, 0.9, N("Corbillard Pressé", "Hurried Hearse", "Eiliger Leichenwagen", "Coche Fúnebre con Prisa", "Carro Funebre Frettoloso"), 0, "#1e1e26", "#c8c8d0", corbillard, [dict(piece="gyro", animation="pulse")]),
    V("carrosse_citrouille", "voiture", 2.0, 1.0, N("Carrosse Citrouille", "Pumpkin Coach", "Kürbiskutsche", "Carroza Calabaza", "Carrozza Zucca"), 300, "#ff8a1f", "#ffd23f", carrosse, [dict(piece="feu", animation="clignote_alterne")]),
    V("camion_bonbons", "camion", 3.0, 1.0, N("Camion à Bonbons", "Candy Truck", "Süßigkeiten-Laster", "Camión de Caramelos", "Camion di Caramelle"), 700, "#7a3fa0", "#ff7ac8", camion, [dict(piece="gyro", animation="pulse"), dict(piece="clou", animation="flotte_verticale_0.04")]),
]
json.dump(dict(vehicules=vehicules), open(os.path.join(IMP, "halloween_vehicules.json"), "w", encoding="utf-8"), ensure_ascii=False, indent=1)
print(len(obstacles), "obstacles,", len(animaux), "créatures,", len(vehicules), "véhicules,", len(train), "voitures de train")
