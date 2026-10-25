#!/usr/bin/env python3
# Commit & Conquer — licence GPL-3.0-or-later.
#
# Le monde « Far West Croâ » de Traversée : chevaux, diligences, bisons, vautour,
# vieux train à vapeur et décor sobre, décrits en pièces simples. Avant = +X, haut = +Y,
# largeur = Z. Écrit games/traversee/data/import/farwest.json et farwest_vehicules.json
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


def shift(pieces, dx=0.0, dy=0.0, dz=0.0):
    out = []
    for p in pieces:
        q = dict(p, position=[round(p["position"][0] + dx, 3), round(p["position"][1] + dy, 3), round(p["position"][2] + dz, 3)])
        out.append(q)
    return out


def swing(nom, phase):
    return dict(piece=nom, animation="pendule", phase=phase, axe="z", pivot="haut")


def model(id, nom, c2, c3, pieces, mobiles=(), **extra):
    return dict(id=id, nom=nom, couleur_2=c2, couleur_3=c3, pieces=pieces, parties_mobiles=list(mobiles), **extra)


# ------------------------------------------------------------------ Le cheval (c2 = robe, c3 = crinière, queue)
def horse(pre="", dx=0.0, dz=0.0, rider=False):
    """Un cheval qui galope, tourné vers +X. Renvoie (pièces, animations)."""
    p = [
        P("s", (0.0, 0.78, 0), (1.05, 0.52, 0.46), "couleur_2", pre + "corps"), P("s", (0.3, 0.84, 0), (0.55, 0.58, 0.44), "couleur_2", pre + "corps"),
        P("s", (-0.32, 0.8, 0), (0.55, 0.55, 0.46), "couleur_2", pre + "corps"),
        P("s", (0.58, 1.05, 0), (0.3, 0.6, 0.26), "couleur_2", pre + "corps", (0, 0, -32)),
        P("s", (0.84, 1.22, 0), (0.42, 0.26, 0.22), "couleur_2", pre + "corps"), P("s", (1.02, 1.15, 0), (0.24, 0.2, 0.18), "couleur_3", pre + "corps"),
        P("n", (0.78, 1.42, 0.07), (0.07, 0.14, 0.05), "couleur_2", pre + "corps"), P("n", (0.78, 1.42, -0.07), (0.07, 0.14, 0.05), "couleur_2", pre + "corps"),
        P("s", (0.92, 1.27, 0.1), (0.05, 0.05, 0.05), "noir", pre + "corps"), P("s", (0.92, 1.27, -0.1), (0.05, 0.05, 0.05), "noir", pre + "corps"),
    ]
    for x in (0.45, 0.6, 0.72):
        p.append(P("c", (x - 0.1, 1.15 + (x - 0.45) * 0.4, 0), (0.1, 0.22, 0.06), "couleur_3", pre + "corps", (0, 0, -30)))
    p += [P("n", (-0.62, 0.78, 0), (0.12, 0.55, 0.12), "couleur_3", pre + "queue", (0, 0, 110))]
    anims = [dict(piece=pre + "corps", animation="rebond", amplitude=0.05), dict(piece=pre + "queue", animation="balance", axe="y")]
    for k, (x, z, ph) in enumerate(((0.4, 0.14, 0.0), (0.4, -0.14, 0.5), (-0.4, 0.14, 0.5), (-0.4, -0.14, 0.0))):
        nom = f"{pre}patte{k}"
        p += [P("y", (x, 0.34, z), (0.12, 0.7, 0.12), "couleur_2", nom), P("c", (x + 0.03, 0.03, z), (0.18, 0.06, 0.14), "noir", nom)]
        anims.append(swing(nom, ph))
    if rider:
        p += [P("c", (-0.02, 1.32, 0), (0.3, 0.42, 0.3), "couleur_3", pre + "corps"), P("s", (0.02, 1.68, 0), (0.22, 0.24, 0.22), "couleur_2", pre + "corps"),
              P("c", (0.02, 1.84, 0), (0.46, 0.04, 0.46), "sombre", pre + "corps"), P("y", (0.02, 1.92, 0), (0.24, 0.16, 0.24), "sombre", pre + "corps"),
              P("c", (0.08, 1.12, 0.22), (0.1, 0.4, 0.1), "sombre", pre + "corps"), P("c", (0.08, 1.12, -0.22), (0.1, 0.4, 0.1), "sombre", pre + "corps")]
    return shift(p, dx, 0, dz), anims


def wheel(x, z, r, spoke="couleur_3"):
    return [P("y", (x, r, z), (r * 2, 0.08, r * 2), "pneu", None, (90, 0, 0)), P("y", (x, r, z + (0.045 if z > 0 else -0.045)), (r * 1.2, 0.05, r * 1.2), spoke, None, (90, 0, 0))]


# ------------------------------------------------------------------ Décor (obstacles)
cactus = model("cactus", N("Cactus Piquant", "Prickly Cactus", "Stachelkaktus", "Cactus Espinoso", "Cactus Spinoso"), "#3f8f4a", "#f06fa8", [
    P("y", (0, 0.55, 0), (0.28, 1.1, 0.28), "couleur_2"), P("s", (0, 1.12, 0), (0.28, 0.2, 0.28), "couleur_2"),
    P("c", (0.2, 0.62, 0), (0.28, 0.14, 0.14), "couleur_2"), P("y", (0.32, 0.88, 0), (0.14, 0.5, 0.14), "couleur_2"),
    P("c", (-0.2, 0.45, 0), (0.28, 0.14, 0.14), "couleur_2"), P("y", (-0.32, 0.68, 0), (0.14, 0.42, 0.14), "couleur_2"),
    P("s", (0, 1.24, 0), (0.1, 0.1, 0.1), "couleur_3")], roll=1, taille_case=1)
rocher = model("rocher_rouge", N("Rocher Rouge", "Red Rock", "Roter Fels", "Roca Roja", "Roccia Rossa"), "#b5653a", "#8a4a2a", [
    P("s", (0, 0.35, 0), (0.95, 0.7, 0.8), "couleur_2"), P("s", (0.25, 0.62, 0.1), (0.55, 0.5, 0.5), "couleur_3"), P("s", (-0.3, 0.2, -0.2), (0.5, 0.4, 0.45), "couleur_3")], roll=2, taille_case=1)
tonneau = model("tonneau", N("Tonneau Louche", "Dodgy Barrel", "Verdächtiges Fass", "Barril Sospechoso", "Botte Sospetta"), "#c9a063", "#7a4d24", [
    P("y", (-0.1, 0.3, 0), (0.55, 0.6, 0.55), "couleur_3"), P("y", (-0.1, 0.18, 0), (0.58, 0.05, 0.58), "sombre"), P("y", (-0.1, 0.44, 0), (0.58, 0.05, 0.58), "sombre"),
    P("c", (0.3, 0.2, 0.2), (0.4, 0.4, 0.4), "couleur_2"), P("c", (0.3, 0.2, 0.2), (0.42, 0.05, 0.42), "couleur_3")], roll=3, taille_case=1)
crane = model("crane_vache", N("Crâne de Vache", "Cow Skull", "Kuhschädel", "Cráneo de Vaca", "Teschio di Mucca"), "#f1ead8", "#7a4d24", [
    P("y", (0, 0.35, 0), (0.1, 0.7, 0.1), "couleur_3"), P("c", (0, 0.04, 0), (0.4, 0.08, 0.4), "sombre"), P("s", (0.04, 0.74, 0), (0.36, 0.28, 0.3), "couleur_2"),
    P("s", (0.2, 0.68, 0), (0.18, 0.16, 0.2), "couleur_2"), P("n", (0.0, 0.86, 0.22), (0.08, 0.3, 0.08), "couleur_2", None, (50, 0, 0)), P("n", (0.0, 0.86, -0.22), (0.08, 0.3, 0.08), "couleur_2", None, (-50, 0, 0)),
    P("s", (0.12, 0.78, 0.09), (0.07, 0.07, 0.04), "noir"), P("s", (0.12, 0.78, -0.09), (0.07, 0.07, 0.04), "noir")], roll=4, taille_case=1)
foin = model("botte_foin", N("Botte de Foin", "Hay Bale", "Heuballen", "Fardo de Heno", "Balla di Fieno"), "#e6c566", "#a98737", [
    P("y", (0, 0.32, 0), (0.7, 0.7, 0.7), "couleur_2", None, (90, 0, 0)), P("y", (0, 0.32, 0.12), (0.72, 0.04, 0.72), "couleur_3", None, (90, 0, 0)),
    P("y", (0, 0.32, -0.12), (0.72, 0.04, 0.72), "couleur_3", None, (90, 0, 0))], roll=5, taille_case=1)
obstacles = [cactus, rocher, tonneau, crane, foin]
mesa = model("mesa", N("Grande Mesa", "Great Mesa", "Große Mesa", "Gran Mesa", "Grande Mesa"), "#c4673a", "#a24f2a", [
    P("c", (0, 1.0, 0), (2.4, 2.0, 1.8), "couleur_2"), P("c", (0.1, 2.3, 0), (1.8, 0.6, 1.4), "couleur_3"), P("c", (0.2, 2.8, 0), (1.2, 0.4, 1.0), "couleur_2"),
    P("c", (-0.4, 0.4, 1.0), (1.2, 0.8, 0.5), "couleur_3")])
saloon = model("saloon", N("Saloon du Coin", "Corner Saloon", "Saloon an der Ecke", "Salón de la Esquina", "Saloon all'Angolo"), "#a0693a", "#e8d9a8", [
    P("c", (0, 0.7, 0), (1.8, 1.4, 1.2), "couleur_2"), P("c", (0, 1.65, 0.0), (1.8, 0.7, 1.2), "couleur_3"), P("c", (0, 1.4, 0.64), (1.9, 0.08, 0.2), "couleur_2"),
    P("c", (0.5, 0.8, 0.62), (0.4, 0.6, 0.04), "vitre"), P("c", (-0.5, 0.8, 0.62), (0.4, 0.6, 0.04), "vitre"), P("c", (0, 0.5, 0.62), (0.3, 0.9, 0.04), "sombre"),
    P("c", (0, 1.95, 0.4), (1.2, 0.4, 0.06), "couleur_3"), P("c", (-0.85, 0.35, 0.7), (0.08, 0.7, 0.08), "sombre"), P("c", (0.85, 0.35, 0.7), (0.08, 0.7, 0.08), "sombre")])
chateau = model("chateau_eau", N("Château d'Eau", "Water Tower", "Wasserturm", "Torre de Agua", "Torre dell'Acqua"), "#8a5a2b", "#c9a063", [
    P("y", (0.5, 1.0, 0.5), (0.1, 2.0, 0.1), "couleur_2", None, (0, 0, 6)), P("y", (-0.5, 1.0, 0.5), (0.1, 2.0, 0.1), "couleur_2", None, (0, 0, -6)),
    P("y", (0.5, 1.0, -0.5), (0.1, 2.0, 0.1), "couleur_2", None, (0, 0, 6)), P("y", (-0.5, 1.0, -0.5), (0.1, 2.0, 0.1), "couleur_2", None, (0, 0, -6)),
    P("y", (0, 2.5, 0), (1.3, 1.1, 1.3), "couleur_3"), P("n", (0, 3.3, 0), (1.4, 0.6, 1.4), "couleur_2"), P("c", (0, 0.05, 0), (1.4, 0.1, 1.4), "sombre")])
poteau = model("poteau_telegraphe", N("Poteau du Télégraphe", "Telegraph Pole", "Telegrafenmast", "Poste de Telégrafo", "Palo del Telegrafo"), "#7a4d24", "#4a3018", [
    P("y", (0, 1.4, 0), (0.12, 2.8, 0.12), "couleur_2"), P("c", (0, 2.6, 0), (0.1, 0.1, 1.0), "couleur_3"), P("c", (0, 2.2, 0), (0.1, 0.1, 0.8), "couleur_3"),
    P("s", (0, 2.68, 0.45), (0.08, 0.08, 0.08), "blanc"), P("s", (0, 2.68, -0.45), (0.08, 0.08, 0.08), "blanc")])
bords = [mesa, saloon, chateau, poteau]
deco = [
    model("petit_cactus", N("Petit Cactus", "Little Cactus", "Kleiner Kaktus", "Cactus Pequeño", "Piccolo Cactus"), "#3f8f4a", "#f06fa8", [P("y", (0, 0.14, 0), (0.12, 0.28, 0.12), "couleur_2"), P("s", (0, 0.3, 0), (0.07, 0.07, 0.07), "couleur_3")]),
    model("os", N("Vieil Os", "Old Bone", "Alter Knochen", "Hueso Viejo", "Vecchio Osso"), "#f1ead8", "#d8d0bc", [P("c", (0, 0.03, 0), (0.3, 0.05, 0.07), "couleur_2", None, (0, 30, 0)), P("s", (0.15, 0.04, 0.05), (0.1, 0.07, 0.09), "couleur_2"), P("s", (-0.15, 0.04, -0.05), (0.1, 0.07, 0.09), "couleur_2")]),
    model("fer_cheval", N("Fer à Cheval", "Horseshoe", "Hufeisen", "Herradura", "Ferro di Cavallo"), "#8890a0", "#5a6070", [P("c", (-0.06, 0.02, 0.07), (0.05, 0.03, 0.14), "couleur_2"), P("c", (-0.06, 0.02, -0.07), (0.05, 0.03, 0.14), "couleur_2"), P("c", (0.02, 0.02, 0.0), (0.05, 0.03, 0.2), "couleur_2")]),
    model("caillou", N("Caillou", "Pebble", "Kiesel", "Guijarro", "Sassolino"), "#b59a78", "#8a7358", [P("s", (0, 0.06, 0), (0.2, 0.12, 0.18), "couleur_2"), P("s", (0.14, 0.04, 0.08), (0.1, 0.08, 0.1), "couleur_3")]),
]

# ------------------------------------------------------------------ Animaux
def alien(id, classe, noms, longueur, largeur, comportement, vitesse, dangereux, pts, c2, c3, pieces, mobiles):
    return dict(id=id, univers="far_west", classe=classe, noms=noms, longueur=longueur, largeur=largeur, comportement=comportement, vitesse=vitesse,
                dangereux=dangereux, points_deblocage=pts, couleur_2=c2, couleur_3=c3, pieces=pieces, parties_mobiles=mobiles)


tumble = [P("s", (0, 0.3, 0), (0.55, 0.5, 0.5), "couleur_2", "boule"), P("s", (0.1, 0.35, 0.05), (0.4, 0.38, 0.4), "couleur_3", "boule"), P("s", (-0.1, 0.25, -0.05), (0.38, 0.36, 0.36), "couleur_3", "boule"),
          P("c", (0.0, 0.3, 0.0), (0.7, 0.04, 0.04), "couleur_3", "boule"), P("c", (0.0, 0.3, 0.0), (0.04, 0.04, 0.7), "couleur_3", "boule")]
coyote = [P("s", (0, 0.42, 0), (0.62, 0.3, 0.26), "couleur_2", "corps"), P("s", (0.32, 0.5, 0), (0.28, 0.24, 0.22), "couleur_2", "corps"), P("n", (0.52, 0.48, 0), (0.12, 0.26, 0.12), "couleur_3", "corps", (0, 0, -90)),
          P("n", (0.3, 0.68, 0.07), (0.07, 0.14, 0.05), "couleur_2", "corps"), P("n", (0.3, 0.68, -0.07), (0.07, 0.14, 0.05), "couleur_2", "corps"),
          P("s", (0.42, 0.55, 0.08), (0.05, 0.05, 0.05), "noir", "corps"), P("s", (0.42, 0.55, -0.08), (0.05, 0.05, 0.05), "noir", "corps"),
          P("s", (-0.4, 0.42, 0), (0.4, 0.18, 0.18), "couleur_3", "queue")]
coyote_mobile = [dict(piece="corps", animation="rebond", amplitude=0.03), dict(piece="queue", animation="balance", axe="y")]
for k, (x, z, ph) in enumerate(((0.2, 0.08, 0.0), (0.2, -0.08, 0.5), (-0.2, 0.08, 0.5), (-0.2, -0.08, 0.0))):
    nom = f"patte{k}"
    coyote += [P("y", (x, 0.18, z), (0.07, 0.36, 0.07), "couleur_2", nom)]
    coyote_mobile.append(swing(nom, ph))
cheval_p, cheval_a = horse()
cavalier_p, cavalier_a = horse(rider=True)


def bison(pre="", dx=0.0, dz=0.0):
    p = [P("s", (0.0, 0.85, 0), (1.3, 0.85, 0.85), "couleur_2", pre + "corps"), P("s", (0.38, 1.18, 0), (0.85, 0.75, 0.82), "couleur_2", pre + "corps"),
         P("s", (0.78, 1.0, 0), (0.55, 0.7, 0.6), "couleur_3", pre + "corps"), P("s", (1.0, 0.72, 0), (0.46, 0.52, 0.46), "couleur_3", pre + "corps"),
         P("n", (0.98, 0.42, 0), (0.12, 0.3, 0.12), "couleur_3", pre + "corps", (0, 0, 180)),
         P("n", (0.98, 0.95, 0.26), (0.08, 0.3, 0.08), "blanc", pre + "corps", (50, 0, 0)), P("n", (0.98, 0.95, -0.26), (0.08, 0.3, 0.08), "blanc", pre + "corps", (-50, 0, 0)),
         P("s", (1.14, 0.8, 0.14), (0.06, 0.06, 0.06), "noir", pre + "corps"), P("s", (1.14, 0.8, -0.14), (0.06, 0.06, 0.06), "noir", pre + "corps"),
         P("n", (-0.62, 0.7, 0), (0.1, 0.5, 0.1), "couleur_3", pre + "queue", (0, 0, 160))]
    anims = [dict(piece=pre + "corps", animation="rebond", amplitude=0.04), dict(piece=pre + "queue", animation="balance", axe="y")]
    for k, (x, z, ph) in enumerate(((0.4, 0.22, 0.0), (0.4, -0.22, 0.5), (-0.4, 0.22, 0.5), (-0.4, -0.22, 0.0))):
        nom = f"{pre}patte{k}"
        p += [P("y", (x, 0.3, z), (0.2, 0.6, 0.2), "couleur_2", nom), P("c", (x + 0.03, 0.03, z), (0.24, 0.06, 0.2), "noir", nom)]
        anims.append(swing(nom, ph))
    return shift(p, dx, 0, dz), anims


bison_p, bison_a = bison()
troupeau_p, troupeau_a = [], []
for i, (dx, dz) in enumerate(((1.9, 0.25), (0.0, -0.3), (-1.9, 0.2))):
    pp, aa = bison(f"b{i}", dx, dz)
    troupeau_p += pp
    troupeau_a += aa

vautour = [P("s", (0, 0.9, 0), (0.7, 0.3, 0.34), "couleur_2", "corps"), P("s", (0.4, 0.95, 0), (0.22, 0.2, 0.2), "couleur_2", "corps"),
           P("s", (0.52, 1.0, 0), (0.18, 0.16, 0.16), "couleur_3", "corps"), P("n", (0.65, 0.97, 0), (0.07, 0.16, 0.07), "blanc", "corps", (0, 0, -90)),
           P("s", (0.34, 0.86, 0), (0.3, 0.12, 0.3), "blanc", "corps"), P("s", (0.56, 1.04, 0.07), (0.05, 0.05, 0.05), "noir", "corps"), P("s", (0.56, 1.04, -0.07), (0.05, 0.05, 0.05), "noir", "corps"),
           P("c", (-0.45, 0.88, 0), (0.4, 0.04, 0.22), "couleur_2", "corps", (0, 0, 8))]
for nom, s in (("aile_g", 1), ("aile_d", -1)):
    vautour += [P("c", (0.0, 0.92, 0.65 * s), (0.7, 0.04, 1.0), "couleur_2", nom), P("c", (-0.15, 0.92, 1.2 * s), (0.6, 0.04, 0.5), "couleur_2", nom),
                P("c", (0.1, 0.92, 1.55 * s), (0.2, 0.04, 0.4), "sombre", nom), P("c", (-0.3, 0.92, 1.5 * s), (0.2, 0.04, 0.4), "sombre", nom)]
vautour_mobile = [dict(piece="corps", animation="rebond", amplitude=0.04), dict(piece="aile_g", animation="pendule", phase=0.0, axe="x", pivot="base"), dict(piece="aile_d", animation="pendule", phase=0.0, axe="x", pivot="base")]

animaux = [
    alien("boule_herbe", "petit", N("Boule d'Herbe Sèche", "Tumbleweed", "Steppenhexe", "Planta Rodadora", "Erba Rotolante"), 0.7, 0.6, "traverse", 2.0, False, 300, "#c9a85a", "#8a6a2a", tumble, [dict(piece="boule", animation="rotation_groupe")]),
    alien("coyote", "petit", N("Coyote Rigolo", "Funny Coyote", "Lustiger Kojote", "Coyote Gracioso", "Coyote Buffo"), 1.0, 0.4, "traverse", 3.0, False, 450, "#b59a78", "#8a7358", coyote, coyote_mobile),
    alien("cheval_galop", "moyen", N("Mustang au Galop", "Galloping Mustang", "Galoppierender Mustang", "Mustang al Galope", "Mustang al Galoppo"), 1.9, 0.5, "traverse", 4.2, True, 800, "#d8c8a8", "#4a3020", cheval_p, cheval_a),
    alien("cavalier", "moyen", N("Cavalier Pressé", "Hasty Rider", "Eiliger Reiter", "Jinete con Prisa", "Cavaliere Frettoloso"), 1.9, 0.5, "traverse", 4.0, True, 1300, "#8b5a2b", "#c0392b", cavalier_p, cavalier_a),
    alien("bison", "grand", N("Bison Pas Content", "Cranky Bison", "Mürrischer Bison", "Bisonte Gruñón", "Bisonte Scontroso"), 2.0, 1.0, "traverse", 2.4, True, 1800, "#4a3020", "#7a5a40", bison_p, bison_a),
    alien("troupeau_bisons", "geant", N("Troupeau de Bisons", "Bison Herd", "Bisonherde", "Manada de Bisontes", "Mandria di Bisonti"), 5.4, 1.4, "charge", 2.0, True, 2600, "#4a3020", "#7a5a40", troupeau_p, troupeau_a),
    alien("vautour", "volant", N("Vautour Affamé", "Hungry Vulture", "Hungriger Geier", "Buitre Hambriento", "Avvoltoio Affamato"), 1.2, 3.2, "vole_en_ligne", 3.5, True, 3000, "#4a3a2a", "#e8786a", vautour, vautour_mobile),
]

# ------------------------------------------------------------------ Héros : grenouille cowboy
src = json.load(open(os.path.join(IMP, "jurassic_croa.json"), encoding="utf-8"))["heros"]
hero_pieces = [p for p in src["pieces"] if p.get("nom") != "canne"]
chapeau = [P("y", (0.12, 0.78, 0), (0.3, 0.2, 0.3), "couleur_3"), P("y", (0.12, 0.69, 0), (0.64, 0.03, 0.64), "couleur_3"), P("y", (0.12, 0.72, 0), (0.31, 0.04, 0.31), "noir"),
           P("c", (0.34, 0.28, 0), (0.06, 0.14, 0.34), "lumiere_rouge"), P("s", (0.34, 0.2, 0), (0.08, 0.08, 0.08), "lumiere_rouge")]
heros = dict(id="grenouille_cowboy", univers="far_west", noms=N("Grenouille Cowboy", "Cowboy Frog", "Cowboy-Frosch", "Rana Vaquera", "Rana Cowboy"), longueur=0.7, largeur=0.6,
             couleur_2="#e8c27a", couleur_3="#6b3f1d", pieces=hero_pieces, parties_mobiles=[dict(piece="corps", animation="rebond", amplitude=0.02)],
             accessoire=dict(nom=N("Chapeau de cow-boy", "Cowboy hat", "Cowboyhut", "Sombrero vaquero", "Cappello da cowboy"), pieces=chapeau))

# ------------------------------------------------------------------ Le vieux train à vapeur : locomotive, tender, wagons
def loco():
    p = [P("y", (0.35, 0.85, 0), (0.7, 1.8, 0.7), "sombre", "corps", (0, 0, 90)), P("y", (1.28, 0.85, 0), (0.76, 0.12, 0.76), "couleur_3", "corps", (0, 0, 90)),
         P("y", (1.0, 1.4, 0), (0.24, 0.5, 0.24), "sombre", "corps"), P("n", (1.0, 1.75, 0), (0.46, 0.3, 0.46), "sombre", "corps", (180, 0, 0)),
         P("s", (0.35, 1.28, 0), (0.28, 0.24, 0.28), "couleur_3", "corps"), P("c", (-0.65, 1.0, 0), (0.85, 1.0, 0.85), "couleur_2", "corps"),
         P("c", (-0.65, 1.55, 0), (1.0, 0.08, 1.0), "sombre", "corps"), P("c", (-0.65, 1.05, 0.43), (0.4, 0.4, 0.04), "vitre", "corps"), P("c", (-0.65, 1.05, -0.43), (0.4, 0.4, 0.04), "vitre", "corps"),
         P("c", (1.5, 0.3, 0), (0.5, 0.12, 0.8), "sombre", "corps", (0, 0, 28)), P("c", (1.3, 0.3, 0.0), (0.1, 0.08, 0.8), "sombre", "corps"),
         P("s", (1.3, 0.95, 0), (0.18, 0.18, 0.18), "phare", "corps"), P("c", (0.35, 0.28, 0), (1.8, 0.14, 0.7), "sombre", "corps")]
    for x, r in ((-0.3, 0.38), (0.5, 0.38), (1.15, 0.22)):
        for z in (0.4, -0.4):
            p += [P("y", (x, r, z), (r * 2, 0.08, r * 2), "sombre", "corps", (90, 0, 0)), P("y", (x, r, z + (0.045 if z > 0 else -0.045)), (r * 1.1, 0.05, r * 1.1), "couleur_3", "corps", (90, 0, 0))]
    for i, y in enumerate((1.95, 2.2, 2.45)):
        p.append(P("s", (1.0 + i * 0.05, y, 0), (0.3 + i * 0.1, 0.3 + i * 0.1, 0.3 + i * 0.1), "blanc", "fumee"))
    return p, [dict(piece="fumee", animation="derive_haut"), dict(piece="corps", animation="rebond", amplitude=0.012)]


def tender():
    p = [P("c", (0, 0.6, 0), (1.4, 0.7, 0.8), "couleur_2"), P("c", (0, 1.0, 0), (1.3, 0.2, 0.7), "noir"), P("c", (0.2, 1.15, 0), (0.5, 0.15, 0.4), "noir"), P("c", (0, 0.25, 0), (1.5, 0.12, 0.7), "sombre")]
    for x in (-0.4, 0.4):
        for z in (0.4, -0.4):
            p += [P("y", (x, 0.25, z), (0.5, 0.08, 0.5), "sombre", None, (90, 0, 0)), P("y", (x, 0.25, z + (0.045 if z > 0 else -0.045)), (0.3, 0.05, 0.3), "couleur_3", None, (90, 0, 0))]
    return p, []


def wagon(betail):
    p = [P("c", (0, 0.75, 0), (1.9, 0.9, 0.85), "couleur_3"), P("c", (0, 1.25, 0), (2.0, 0.08, 0.95), "sombre"), P("c", (0, 0.25, 0), (2.0, 0.12, 0.7), "sombre")]
    if betail:
        for x in (-0.8, -0.4, 0.0, 0.4, 0.8):
            p.append(P("c", (x, 0.78, 0.43), (0.06, 0.7, 0.03), "sombre"))
            p.append(P("c", (x, 0.78, -0.43), (0.06, 0.7, 0.03), "sombre"))
    else:
        p += [P("c", (0, 0.75, 0.43), (0.5, 0.7, 0.03), "couleur_2"), P("c", (0, 0.75, -0.43), (0.5, 0.7, 0.03), "couleur_2")]
    for x in (-0.6, 0.6):
        for z in (0.4, -0.4):
            p += [P("y", (x, 0.25, z), (0.5, 0.08, 0.5), "sombre", None, (90, 0, 0)), P("y", (x, 0.25, z + (0.045 if z > 0 else -0.045)), (0.3, 0.05, 0.3), "couleur_2", None, (90, 0, 0))]
    return p, []


lp, la = loco()
tp, ta = tender()
w1p, _ = wagon(False)
w2p, _ = wagon(True)
train = [
    dict(id="locomotive", longueur=2.9, couleur_2="#2f5d3a", couleur_3="#e8b830", pieces=lp, parties_mobiles=la),
    dict(id="tender", longueur=1.6, couleur_2="#2f5d3a", couleur_3="#e8b830", pieces=tp, parties_mobiles=[]),
    dict(id="wagon_bois", longueur=2.1, couleur_2="#8a5a2b", couleur_3="#7a4d24", pieces=w1p, parties_mobiles=[]),
    dict(id="wagon_betail", longueur=2.1, couleur_2="#8a5a2b", couleur_3="#6b3f1d", pieces=w2p, parties_mobiles=[]),
]

# ------------------------------------------------------------------ Véhicules : diligence, chariot bâché, buggy
# Chevaux attelés : deux de front, devant la voiture
def attelage(dx):
    ph, ah = horse("a", dx, 0.24)
    pb, ab = horse("b", dx, -0.24)
    return ph + pb, ah + ab


dil = [P("c", (-0.9, 0.8, 0), (1.5, 0.75, 0.85), "couleur_2"), P("c", (-0.9, 1.22, 0), (1.6, 0.1, 0.95), "sombre"), P("c", (-0.9, 1.4, 0), (1.3, 0.25, 0.7), "couleur_3"),
       P("c", (-0.9, 0.85, 0.44), (0.5, 0.35, 0.03), "vitre"), P("c", (-0.9, 0.85, -0.44), (0.5, 0.35, 0.03), "vitre"), P("c", (-0.2, 0.9, 0), (0.2, 0.5, 0.7), "couleur_3"),
       P("c", (-0.15, 1.15, 0), (0.45, 0.14, 0.7), "sombre"), P("c", (-0.15, 1.45, 0), (0.1, 0.5, 0.6), "couleur_2"),
       P("c", (-0.9, 0.42, 0), (1.7, 0.1, 0.7), "sombre")]
dil += wheel(-1.4, 0.5, 0.4) + wheel(-1.4, -0.5, 0.4) + wheel(-0.35, 0.5, 0.32) + wheel(-0.35, -0.5, 0.32)
dil += [P("s", (-0.1, 1.82, 0), (0.22, 0.24, 0.22), "couleur_2"), P("c", (-0.1, 1.64, 0), (0.28, 0.34, 0.3), "couleur_3"), P("c", (-0.1, 1.97, 0), (0.46, 0.04, 0.46), "sombre"), P("y", (-0.1, 2.05, 0), (0.24, 0.16, 0.24), "sombre")]
hp, ha = attelage(0.7)
dil += hp
chariot = [P("c", (-0.7, 0.55, 0), (1.9, 0.18, 0.85), "couleur_3"), P("y", (-0.7, 0.98, 0), (0.95, 1.8, 0.95), "blanc", None, (0, 0, 90)), P("c", (0.25, 0.9, 0), (0.06, 0.7, 0.8), "blanc"),
           P("c", (-1.65, 0.9, 0), (0.06, 0.7, 0.8), "blanc"), P("c", (0.2, 0.62, 0), (0.5, 0.1, 0.3), "couleur_3")]
chariot += wheel(-1.3, 0.5, 0.42) + wheel(-1.3, -0.5, 0.42) + wheel(-0.1, 0.5, 0.36) + wheel(-0.1, -0.5, 0.36)
cp, ca = attelage(0.85)
chariot += cp
buggy = [P("c", (-0.35, 0.55, 0), (0.9, 0.12, 0.65), "couleur_2"), P("c", (-0.6, 0.78, 0), (0.15, 0.4, 0.62), "couleur_2"), P("c", (-0.2, 0.7, 0), (0.45, 0.14, 0.55), "couleur_3"),
         P("c", (0.2, 0.62, 0), (0.55, 0.08, 0.15), "sombre")]
buggy += wheel(-0.6, 0.38, 0.34) + wheel(-0.6, -0.38, 0.34) + wheel(0.05, 0.38, 0.26) + wheel(0.05, -0.38, 0.26)
buggy += [P("s", (-0.25, 1.05, 0), (0.2, 0.22, 0.2), "couleur_2"), P("c", (-0.25, 0.92, 0), (0.24, 0.28, 0.26), "couleur_3"), P("c", (-0.25, 1.2, 0), (0.4, 0.04, 0.4), "sombre"), P("y", (-0.25, 1.28, 0), (0.22, 0.14, 0.22), "sombre")]
bp, ba = horse("h", 0.85, 0.0)
buggy += bp
# Les attelages sont des animaux : leurs chevaux pivotent (pattes qui galopent).
animaux += [
    alien("buggy_docteur", "moyen", N("Buggy du Docteur", "Doctor's Buggy", "Doktors Buggy", "Buggy del Doctor", "Buggy del Dottore"), 2.6, 0.9, "traverse", 3.4, True, 600, "#4a3020", "#c0392b", buggy, ba),
    alien("diligence", "grand", N("Diligence Fonceuse", "Speeding Stagecoach", "Rasende Postkutsche", "Diligencia Veloz", "Diligenza Veloce"), 4.0, 1.0, "traverse", 3.2, True, 1100, "#a0522d", "#e8b830", dil, ha),
    alien("chariot_bache", "grand", N("Chariot Bâché", "Covered Wagon", "Planwagen", "Carro Cubierto", "Carro Coperto"), 3.9, 1.0, "traverse", 2.8, True, 1500, "#8b5a2b", "#7a4d24", chariot, ca),
]

monde = dict(id="far_west", nom=N("Far West Croâ", "Wild West Croak", "Wilder Westen", "Lejano Oeste Croa", "Far West Gra"), points_deblocage=3000,
             ciel="#f2c98a", brume="#f6dcb0", sol_voie_route="#b8925f", marquage="#e8d9a8", herbe_a="#dcb66e", herbe_b="#cfa85e", eau="#4d8fb0", rondin="#8a5a2b",
             obstacles=obstacles, bords=bords, deco=deco, voie_speciale=dict(remplace="rail", description="Vieux chemin de fer avec locomotive à vapeur.", pieces=[]),
             vehicules_du_monde=[], animaux_du_monde=[a["id"] for a in animaux], train=train,
             musique=dict(tempo_bpm=112, gamme="mixolydienne", instruments=["guitare", "harmonica", "banjo", "tambour"], ambiance="Cavalcade dans la plaine, soleil de plomb."),
             **{"bannière": N("Bienvenue dans le Far West Croâ ! Ici, la loi, c'est le plus rapide à sauter.", "Welcome to the Wild West Croak! Here the law is whoever hops fastest.",
                              "Willkommen im Wilden Westen! Hier gilt das Gesetz des schnellsten Hüpfers.", "¡Bienvenido al Lejano Oeste Croa! Aquí manda el que salta más rápido.",
                              "Benvenuto nel Far West Gra! Qui la legge è chi salta più veloce.")})
json.dump(dict(monde=monde, animaux=animaux, heros=heros), open(os.path.join(IMP, "farwest.json"), "w", encoding="utf-8"), ensure_ascii=False, indent=1)


print(len(obstacles), "obstacles,", len(animaux), "animaux,", len(train), "voitures de train")
