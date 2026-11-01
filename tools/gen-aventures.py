#!/usr/bin/env python3
"""Génère les aventures de « Suivez le guide ! » (games/snake/scenarios/*.txt) : le tutoriel du jardin, puis une aventure
de six niveaux par lieu. Chaque carte est vérifiée (accessibilité à travers les téléporteurs, pas de cul-de-sac,
place pour la file) avant d'être écrite ; le jeu refait ces contrôles dans un test.

    python3 tools/gen-aventures.py

Un seul fichier, `aventure.txt` : une aventure continue (tutoriel puis dix lieux), à ouvrir dans l'éditeur du site.
"""
import os
import random
import sys

ICI = os.path.dirname(os.path.abspath(__file__))
SORTIE = os.path.join(ICI, "..", "games", "snake", "scenarios")

BLOCKED = set("234567oHh")  # cases où l'on ne marche pas (herses comptées comme fermées par prudence)


# ---------------------------------------------------------------- formes
def cut_corners(rows, k=2):
    h, w = len(rows), len(rows[0])
    for i in range(k):
        for j in range(k - i):
            for (y, x) in [(i, j), (i, w - 1 - j), (h - 1 - i, j), (h - 1 - i, w - 1 - j)]:
                rows[y][x] = "0"
    return rows


def square(w, h=None):
    h = h or w
    return cut_corners([["1"] * w for _ in range(h)])


def shape_l(n):
    rows = [["1"] * n for _ in range(n)]
    for y in range(int(n * 0.42)):
        for x in range(int(n * 0.55), n):
            rows[y][x] = "0"
    return cut_corners(rows, 1)


def shape_u(w, h):
    rows = [["1"] * w for _ in range(h)]
    x0, x1 = w // 3, w - w // 3
    for y in range(0, h * 55 // 100):
        for x in range(x0, x1):
            rows[y][x] = "0"
    return rows


def ring(n, t=5):
    rows = [["1"] * n for _ in range(n)]
    for y in range(t, n - t):
        for x in range(t, n - t):
            rows[y][x] = "0"
    return cut_corners(rows, 2)


def eight(t=4, m=5):
    w, h = 3 * t + 2 * m, 2 * t + m
    rows = [["1"] * w for _ in range(h)]
    for y in range(t, t + m):
        for x in list(range(t, t + m)) + list(range(2 * t + m, 2 * t + 2 * m)):
            rows[y][x] = "0"
    return cut_corners(rows, 2)


def ring_with_island(n=24, t=5, gap=2):
    rows = ring(n, t)
    m = n - 2 * t
    for y in range(t + gap, n - t - gap):
        for x in range(t + gap, n - t - gap):
            rows[y][x] = "1"
    return rows, (t + gap, n - t - gap)  # bornes de l'îlot (x et y)


def two_islands(w=20, h=16, side=7):
    rows = [["0"] * (w + 2 + side) for _ in range(h)]
    for y in range(h):
        for x in range(w):
            rows[y][x] = "1"
    cut_corners_area = square(w, h)
    for y in range(h):
        for x in range(w):
            rows[y][x] = cut_corners_area[y][x]
    y0 = (h - side) // 2
    for y in range(y0, y0 + side):
        for x in range(w + 2, w + 2 + side):
            rows[y][x] = "1"
    return rows, (w + 2, w + 2 + side, y0, y0 + side)


# ---------------------------------------------------------------- contrôles
def nbrs(y, x):
    return [(y - 1, x), (y + 1, x), (y, x - 1), (y, x + 1)]


def problems(rows, tourists):
    h, w = len(rows), len(rows[0])
    s = [(y, x) for y in range(h) for x in range(w) if rows[y][x] == "S"]
    if len(s) != 1:
        return ["pas un seul départ"]
    sy, sx = s[0]

    def ok(y, x):
        return 0 <= y < h and 0 <= x < w and rows[y][x] != "0" and rows[y][x] not in BLOCKED

    out = []
    if not all(ok(sy, sx - i) for i in (0, 1, 2)) or not ok(sy, sx + 1):
        out.append("départ sans place")
    opened = [(y, x) for y in range(h) for x in range(w) if ok(y, x)]
    if len(opened) <= tourists + 13:
        out.append("trop étroit")
    tele = {}
    for y in range(h):
        for x in range(w):
            if rows[y][x] in "ABCD":
                tele.setdefault(rows[y][x], []).append((y, x))
    for k, v in tele.items():
        if len(v) != 2:
            out.append(f"téléporteur {k} : {len(v)} cases")
    seen, st = set(), [(sy, sx)]
    while st:
        c = st.pop()
        if c in seen or not ok(*c):
            continue
        seen.add(c)
        st.extend(nbrs(*c))
        if rows[c[0]][c[1]] in "ABCD":
            st.extend(tele.get(rows[c[0]][c[1]], []))
    if len(seen) != len(opened):
        out.append(f"{len(opened) - len(seen)} cases inaccessibles")
    dead = sum(1 for (y, x) in opened if sum(1 for n in nbrs(y, x) if ok(*n)) < 2)
    if dead:
        out.append(f"{dead} culs-de-sac")
    return out


# ---------------------------------------------------------------- placement
def find_start(rows):
    h, w = len(rows), len(rows[0])
    best = None
    for y in range(h):
        for x in range(3, w - 8):
            if all(rows[y][x + i] == "1" for i in range(-3, 8)):
                score = abs(x - w // 4) + abs(y - h // 2) * 1.2
                if best is None or score < best[0]:
                    best = (score, y, x)
    if best is None:
        raise ValueError("pas de place pour le départ")
    rows[best[1]][best[2]] = "S"
    return best[1], best[2]


def far_from_start(start, y, x, d=6):
    return abs(y - start[0]) + abs(x - start[1]) >= d and not (y == start[0] and x > start[1] and x - start[1] < 9)


def near_feature(rows, y, x):
    h, w = len(rows), len(rows[0])
    for dy in (-1, 0, 1):
        for dx in (-1, 0, 1):
            yy, xx = y + dy, x + dx
            if 0 <= yy < h and 0 <= xx < w and rows[yy][xx] not in "01":
                return True
    return False


def place_single(rows, rng, chars, n, start, tourists):
    h, w = len(rows), len(rows[0])
    placed = 0
    for _ in range(2000):
        if placed >= n:
            break
        y, x = rng.randrange(h), rng.randrange(w)
        if rows[y][x] != "1" or not far_from_start(start, y, x) or near_feature(rows, y, x):
            continue
        rows[y][x] = rng.choice(chars)
        if problems(rows, tourists):
            rows[y][x] = "1"
            continue
        placed += 1
    return placed


def place_line(rows, rng, ch, length, n, start, tourists):
    h, w = len(rows), len(rows[0])
    placed = 0
    for _ in range(3000):
        if placed >= n:
            break
        y, x = rng.randrange(h), rng.randrange(w)
        dy, dx = rng.choice([(0, 1), (0, -1), (1, 0), (-1, 0)]) if ch in "<>^v" else rng.choice([(0, 1), (1, 0)])
        if ch in "<>^v":
            dy, dx = {">": (0, 1), "<": (0, -1), "^": (-1, 0), "v": (1, 0)}[ch]
        cells = [(y + dy * i, x + dx * i) for i in range(length)]
        beyond = (y + dy * length, x + dx * length)
        if not all(0 <= cy < h and 0 <= cx < w and rows[cy][cx] == "1" and far_from_start(start, cy, cx) and not near_feature(rows, cy, cx) for cy, cx in cells):
            continue
        if not (0 <= beyond[0] < h and 0 <= beyond[1] < w and rows[beyond[0]][beyond[1]] == "1"):
            continue
        for cy, cx in cells:
            rows[cy][cx] = ch
        if problems(rows, tourists):
            for cy, cx in cells:
                rows[cy][cx] = "1"
            continue
        placed += 1
    return placed


def place_pair(rows, rng, letter, start, tourists, regions=None):
    """Deux cases jumelles ; `regions` = deux listes de cases autorisées (une par extrémité)."""
    h, w = len(rows), len(rows[0])
    for _ in range(4000):
        if regions:
            (y1, x1), (y2, x2) = rng.choice(regions[0]), rng.choice(regions[1])
        else:
            y1, x1, y2, x2 = rng.randrange(h), rng.randrange(w), rng.randrange(h), rng.randrange(w)
        if (y1, x1) == (y2, x2) or abs(y1 - y2) + abs(x1 - x2) < 5:
            continue
        if rows[y1][x1] != "1" or rows[y2][x2] != "1" or near_feature(rows, y1, x1) or near_feature(rows, y2, x2):
            continue
        if not far_from_start(start, y1, x1) or not far_from_start(start, y2, x2):
            continue
        rows[y1][x1] = rows[y2][x2] = letter
        if problems(rows, tourists):
            rows[y1][x1] = rows[y2][x2] = "1"
            continue
        return True
    return False


def place_exit(rows, start):
    h, w = len(rows), len(rows[0])
    best = None
    for y in range(h):
        for x in range(w):
            if rows[y][x] == "1" and not near_feature(rows, y, x):
                nb = sum(1 for n in nbrs(y, x) if 0 <= n[0] < h and 0 <= n[1] < w and rows[n[0]][n[1]] == "1")
                if nb >= 3:
                    d = abs(y - start[0]) + abs(x - start[1])
                    if best is None or d > best[0]:
                        best = (d, y, x)
    rows[best[1]][best[2]] = "E"


# ---------------------------------------------------------------- niveaux
def level(name, place, mask, tourists, pace, seed, exit_=False, blocs=0, trous=0, tapis=0, accel=0, pontons=0, herses=0,
          paires=0, regions=None, rivaux=0, trains=0, eboulements=0):
    """Construit une carte valide ; renvoie le texte de la section."""
    for attempt in range(40):
        rng = random.Random(seed * 100 + attempt)
        rows = [r[:] for r in mask]
        try:
            start = find_start(rows)
        except ValueError:
            continue
        # Au départ, les îlots ne sont pas encore reliés : seule l'accessibilité peut manquer.
        if [p for p in problems(rows, tourists) if "inaccessibles" not in p]:
            continue
        ok = True
        for i, letter in enumerate("ABCD"[:paires]):
            reg = regions[i % len(regions)] if regions else None
            ok &= place_pair(rows, rng, letter, start, tourists, reg)
        ok &= place_single(rows, rng, "234"[:3], blocs, start, tourists) >= blocs
        ok &= place_single(rows, rng, "o", trous, start, tourists) >= trous
        for ch in "><^v"[:0]:
            pass
        if tapis:
            per = [">", "<", "^", "v"]
            done = 0
            for k in range(tapis):
                done += place_line(rows, rng, per[k % 4], 4, 1, start, tourists)
            ok &= done >= tapis
        ok &= place_single(rows, rng, "z", accel, start, tourists) >= accel
        if pontons:
            ok &= place_line(rows, rng, "p", 3, pontons, start, tourists) >= pontons
        ok &= place_single(rows, rng, "hH", herses, start, tourists) >= herses
        if not ok:
            continue
        if exit_:
            place_exit(rows, start)
        if problems(rows, tourists):
            continue
        txt = f"[{name}]\nlieu = {place}\ntouristes = {tourists}\npas = {pace}\ndepart = est\n"
        if exit_:
            txt += "sortie = oui\n"
        for k, v in (("rivaux", rivaux), ("trains", trains), ("eboulements", eboulements)):
            if v:
                txt += f"{k} = {v}\n"
        return txt + "carte:\n" + "\n".join("".join(r) for r in rows) + "\n"
    raise SystemExit(f"Impossible de construire le niveau « {name} » ({place})")


# ---------------------------------------------------------------- scènes
LEA = ("Léa", 1, "1.6.1.0.2", "guide", "bequille")
MAX = ("Max", 2, "5.0.3.2.1", "-", "lunettes-soleil")
ZOE = ("Zoé", 5, "0.8.5.0.2", "-", "canne-blanche")
REMI = ("Rémi", 16, "2.3.4.1.1", "-", "")


def scene(name, decor, camera, cast, lines):
    """cast : liste de (personnage, place, animation) ; lines : (qui, texte[, caméra])."""
    t = f"[{name}]\ntype = scene\ndecor = {decor}\ncamera = {camera}\n"
    for (nom, kind, look, role, acc), place, anim in cast:
        t += f"personnage = {nom} | {kind} | {look} | {place} | {anim} | {role}" + (f" | {acc}" if acc else "") + "\n"
    for line in lines:
        t += "replique = " + " | ".join(line) + "\n"
    return t


def groupe(decor, camera, lines, name, leader_anim="salut", pose=("repos", "repos", "fauteuil-assis")):
    return scene(name, decor, camera, [(LEA, "centre", leader_anim), (MAX, "gauche", pose[0]), (ZOE, "droite", pose[1]), (REMI, "gauche2", pose[2])], lines)


def ecrire(nom, titre, description, sections):
    with open(os.path.join(SORTIE, nom), "w") as f:
        f.write(f"# Généré par tools/gen-aventures.py — ne pas modifier à la main (relancer le script).\ntitre = {titre}\ndescription = {description}\n\n")
        f.write("\n".join(sections))
    print("écrit", nom)


# ---------------------------------------------------------------- tutoriel
def tutoriel():
    s = []
    s.append(scene("Bienvenue", "jardin", "moyen", [(LEA, "centre", "salut")], [
        ("Léa", "Bonjour et bienvenue ! Je suis Léa, ton guide dans le jeu."),
        ("Léa", "Ça va être très simple : ton but, c'est d'attraper les touristes. Déplace-toi avec les flèches du clavier.", "gros"),
        ("Léa", "Essayons avec un seul touriste. Il t'attend quelque part dans le parc !", "moyen")]))
    s.append(level("Mon premier touriste", "jardin", square(18), 1, 0.36, 1))
    s.append(groupe("jardin", "moyen", [
        ("Léa", "Bravo, il te suit ! Tu vois, la file s'allonge derrière toi."),
        ("Léa", "Maintenant, essayons d'en attraper deux. Dès que le premier a rejoint la file, le deuxième apparaît.", "gros"),
        ("Léa", "Attention : ne touche ni les bords, ni ta propre file.", "moyen")], "Léa : deux touristes"))
    s.append(level("Deux touristes", "jardin", square(18), 2, 0.36, 2))
    s.append(groupe("jardin", "moyen", [
        ("Léa", "Très bien ! Tu as compris le principe."),
        ("Max", "Alors, tu arriveras à en attraper dix ?"),
        ("Léa", "On voit ça tout de suite. Courage !", "gros")], "Léa : dix touristes"))
    s.append(level("Dix touristes", "jardin", square(18), 10, 0.34, 3))
    s.append(groupe("jardin", "large", [
        ("Léa", "Dix touristes, bravo ! Le groupe est déjà bien long."),
        ("Léa", "Et si on essayait d'autres formes de terrain ? On commence par un grand L."),
        ("Zoé", "J'entends que le chemin tourne. Fais attention aux virages.", "moyen")], "Les formes de terrain"))
    s.append(level("Le grand L", "jardin", shape_l(20), 10, 0.34, 4))
    s.append(groupe("jardin", "moyen", [
        ("Léa", "Pas mal ! Maintenant, un carré creux : on tourne autour d'un massif de fleurs."),
        ("Léa", "Et cette fois, quinze touristes !", "gros")], "Léa : le carré creux"))
    s.append(level("Le carré creux", "jardin", ring(20, 5), 15, 0.33, 5))
    s.append(groupe("jardin", "large", [
        ("Léa", "Dernière forme : le 8. Deux massifs, une seule file, et vingt touristes."),
        ("Max", "Ça ressemble à un rond-point. Ne te perds pas !", "moyen")], "Léa : le huit"))
    s.append(level("Le huit", "jardin", eight(4, 5), 20, 0.33, 6))
    s.append(groupe("jardin", "moyen", [
        ("Léa", "Tu es prêt pour la grande récupération."),
        ("Léa", "Trente touristes dispersés sur tout le terrain, puis la sortie. Tu les rassembles tous, et ils sortent derrière toi.", "gros"),
        ("Rémi", "Les allées sont larges, on te suit !", "moyen")], "Léa : la grande récupération"))
    s.append(level("La grande récupération", "jardin", square(22), 30, 0.32, 7, exit_=True))
    s.append(groupe("jardin", "large", [
        ("Léa", "Félicitations, tu connais toutes les bases !"),
        ("Léa", "Maintenant, on change de lieu. Il y aura des obstacles, des trous, des pièges... et toujours plus de touristes."),
        ("Max", "En route pour l'aventure !", "moyen")], "En route", leader_anim="salut", pose=("salut", "repos", "fauteuil-assis")))
    return s


# ---------------------------------------------------------------- les lieux
def scenes_lieu(decor, camera_intro, intro, milieu, fin, anim_fin="salut"):
    return (groupe(decor, camera_intro, intro, "Arrivée"), groupe(decor, "moyen", milieu, "En chemin"),
            groupe(decor, "large", fin, "Fin de la visite", leader_anim=anim_fin, pose=("salut", "repos", "fauteuil-assis")))


def lieu(fichier, titre, description, decor, place, intro, milieu, fin, niveaux):
    """niveaux : liste de 6 dicts d'arguments pour `level`."""
    a, m, f = scenes_lieu(decor, "large", intro, milieu, fin)
    # Noms uniques dans le fichier : on préfixe par le lieu.
    a, m, f = (x.replace("[Arrivée]", f"[{titre} : arrivée]").replace("[En chemin]", f"[{titre} : en chemin]").replace("[Fin de la visite]", f"[{titre} : fin de la visite]") for x in (a, m, f))
    secs = [a]
    for i, spec in enumerate(niveaux):
        secs.append(level(spec.pop("nom"), place, spec.pop("mask"), spec.pop("t"), spec.pop("pas"), spec.pop("seed"), **spec))
        if i == 2:
            secs.append(m)
    secs.append(f)
    return secs


def six(prefix, seed0, t0, **extra):
    """Les six niveaux d'un lieu : découverte, rectangle, L, anneau, huit, grande récupération."""
    def e(k, n):
        return {key: max(0, int(v[0] + (v[1] - v[0]) * n / 5)) for key, v in extra.items() if key not in ("paires", "regions")}
    return [
        dict(nom=f"{prefix} 1", mask=square(18), t=t0[0], pas=0.34, seed=seed0 + 1, **e("a", 0)),
        dict(nom=f"{prefix} 2", mask=square(22, 12), t=t0[1], pas=0.33, seed=seed0 + 2, **e("a", 1)),
        dict(nom=f"{prefix} 3", mask=shape_l(20), t=t0[2], pas=0.32, seed=seed0 + 3, **e("a", 2)),
        dict(nom=f"{prefix} 4", mask=ring(20, 5), t=t0[3], pas=0.31, seed=seed0 + 4, **e("a", 3)),
        dict(nom=f"{prefix} 5", mask=eight(4, 5), t=t0[4], pas=0.30, seed=seed0 + 5, **e("a", 4)),
        dict(nom=f"{prefix} 6", mask=square(24), t=30, pas=0.30, seed=seed0 + 6, exit_=True, **e("a", 5)),
    ]


T = (6, 10, 14, 18, 22)


def main():
    os.makedirs(SORTIE, exist_ok=True)
    tout = tutoriel()

    tout += lieu("lieu_desert.txt", "Le désert", "Cactus et rochers : on ne les touche pas", "desert", "desert",
         [("Léa", "Voilà le désert ! Il fait chaud, mais la visite est magnifique."),
          ("Léa", "Attention aux cactus et aux gros rochers : si on les touche, c'est raté. Contournez-les !", "gros"),
          ("Max", "J'ai mis mes lunettes de soleil. Je suis prêt.", "moyen")],
         [("Zoé", "Le vent siffle entre les rochers. Je sens qu'ils sont de plus en plus nombreux."),
          ("Léa", "On avance, les dunes ne sont plus très loin.", "moyen")],
         [("Léa", "Le désert est derrière nous ! Place à de vieilles ruines."),
          ("Rémi", "Mon fauteuil a ramassé du sable partout, mais quelle belle balade.", "moyen")],
         six("Dans le désert", 100, T, blocs=(4, 12)))

    tout += lieu("lieu_ruines.txt", "Les ruines", "Le sol est fissuré : attention aux trous", "ruines", "ruines",
         [("Léa", "Nous voici dans les ruines antiques. Admirez les colonnes !"),
          ("Léa", "Le sol est fissuré : les trous sont mortels. Regardez où vous mettez les pieds.", "gros"),
          ("Rémi", "Je roule doucement, ces dalles sont inégales.", "moyen")],
         [("Zoé", "J'entends des pierres tomber dans le vide. Les failles sont nombreuses."),
          ("Léa", "Gardons le rythme, la fin du site est proche.", "moyen")],
         [("Léa", "Les ruines n'ont plus de secrets pour nous. Direction le cimetière !"),
          ("Max", "Un cimetière ? Avec ma chance, je vais croiser un fantôme.", "moyen")],
         six("Dans les ruines", 200, T, blocs=(2, 6), trous=(3, 10)))

    def regions(mask, box):
        x0, x1, y0, y1 = box
        main, isle = [], []
        for y, r in enumerate(mask):
            for x, c in enumerate(r):
                if c == "1":
                    (isle if x0 <= x < x1 and y0 <= y < y1 else main).append((y, x))
        return [(main, isle)]

    m_two, b_two = two_islands(20, 16, 7)
    m_r20, (a20, b20) = ring_with_island(20, 5, 2)
    m_r24, (a24, b24) = ring_with_island(24, 5, 2)
    tout += lieu("lieu_cimetiere.txt", "Le cimetière", "Passages mystérieux : entrez par un côté, sortez par l'autre", "cimetiere", "cimetiere",
         [("Léa", "Bienvenue au cimetière. Ne vous inquiétez pas, les fantômes sont plutôt timides."),
          ("Léa", "Vous voyez ces lueurs ? Ce sont des passages : entrez dans l'une, vous ressortez par l'autre. Toute la file suit !", "gros"),
          ("Zoé", "Je sens un courant d'air frais, comme une porte invisible.", "moyen")],
         [("Max", "Certaines zones ne se rejoignent que par les passages, non ?"),
          ("Léa", "Exactement. Il faut parfois les prendre pour rejoindre les touristes.", "moyen")],
         [("Léa", "Le cimetière est derrière nous. Cap sur la mer !"),
          ("Rémi", "Je n'ai croisé aucun fantôme. Dommage, j'avais un appareil photo.", "moyen")],
         [dict(nom="Dans le cimetière 1", mask=square(18), t=6, pas=0.34, seed=301, paires=1, blocs=2, trous=1),
          dict(nom="Dans le cimetière 2", mask=square(22, 12), t=10, pas=0.33, seed=302, paires=2, blocs=3, trous=2),
          dict(nom="Dans le cimetière 3", mask=m_two, t=12, pas=0.32, seed=303, paires=1, regions=regions(m_two, b_two)),
          dict(nom="Dans le cimetière 4", mask=m_r20, t=16, pas=0.31, seed=304, paires=2, regions=regions(m_r20, (a20, b20, a20, b20))),
          dict(nom="Dans le cimetière 5", mask=m_r24, t=22, pas=0.30, seed=305, paires=3, blocs=4, regions=regions(m_r24, (a24, b24, a24, b24))),
          dict(nom="Dans le cimetière 6", mask=square(24), t=30, pas=0.30, seed=306, exit_=True, paires=4, blocs=7, trous=6)])

    tout += lieu("lieu_pirates.txt", "L'île des pirates", "Des courants vous poussent : impossible de les remonter", "pirates", "pirates",
         [("Léa", "Ohé ! Nous voici sur l'île des pirates."),
          ("Léa", "Ces planches sont des courants qui vous poussent. Vous ne pouvez pas les prendre à contresens !", "gros"),
          ("Max", "Je prends le courant, c'est comme un tapis roulant.", "moyen")],
         [("Zoé", "Je sens les planches bouger sous mes pas."),
          ("Léa", "Suivez le courant, ou évitez-le. À vous de choisir !", "moyen")],
         [("Léa", "Pas de trésor, mais de beaux souvenirs. Prochaine étape : le parc de loisirs !"),
          ("Rémi", "Enfin quelque chose de plus calme... j'espère.", "moyen")],
         six("Chez les pirates", 400, T, blocs=(2, 6), trous=(0, 3), tapis=(2, 7)))

    tout += lieu("lieu_parc.txt", "Le parc de loisirs", "Des plaques qui donnent un coup de vitesse", "parc", "parc",
         [("Léa", "Bienvenue au parc de loisirs : manèges, barbe à papa et sensations fortes !"),
          ("Léa", "Les plaques jaunes donnent un coup de vitesse à toute la file. Utilisez-les, mais ne vous laissez pas dépasser par elles.", "gros"),
          ("Max", "Je veux faire le grand huit !", "moyen")],
         [("Zoé", "J'entends les rires des manèges tout près."),
          ("Léa", "On accélère, mais pas trop vite : il faut garder tout le monde.", "moyen")],
         [("Léa", "Le parc est fini. On part en ville !"),
          ("Rémi", "Quelle journée ! Mon fauteuil a le tournis.", "moyen")],
         six("Au parc", 500, T, blocs=(2, 5), trous=(0, 3), tapis=(0, 4), accel=(3, 10)))

    tout += lieu("lieu_ville.txt", "La ville", "Un guide rival traverse le parc", "ville", "ville",
         [("Léa", "Nous voilà en ville, au milieu d'un grand parc entouré d'immeubles."),
          ("Léa", "Attention : un guide rival traverse parfois le terrain. Son entrée clignote juste avant. S'il touche votre file, il emmène vos touristes !", "gros"),
          ("Max", "Un rival ? Il ne nous volera personne.", "moyen")],
         [("Zoé", "J'entends des pas rapides : quelqu'un arrive de l'autre côté."),
          ("Léa", "Écartez la file à temps. Surveillez le clignotement !", "moyen")],
         [("Léa", "La ville est à nous. Direction la gare !"),
          ("Rémi", "Les routes étaient larges, c'est agréable.", "moyen")],
         six("En ville", 600, T, blocs=(2, 5), trous=(0, 2), accel=(0, 4), rivaux=(1, 3)))

    tout += lieu("lieu_gare.txt", "La gare", "Des trains traversent le terrain", "gare", "gare",
         [("Léa", "Bienvenue à la gare. Prochain départ dans quelques minutes."),
          ("Léa", "Des trains traversent le terrain : la voie clignote avant leur passage. Un train, ça ne s'arrête pas : n'y touchez pas !", "gros"),
          ("Zoé", "J'entends le sifflement au loin.", "moyen")],
         [("Max", "Le train est passé de justesse. Je vais surveiller les voies !"),
          ("Léa", "Gardez bien la file serrée entre deux trains.", "moyen")],
         [("Léa", "Tout le monde est à bord de l'aventure. Cap sur le port !"),
          ("Rémi", "Enfin un peu de calme sur les quais.", "moyen")],
         six("À la gare", 700, T, blocs=(2, 4), trous=(0, 2), accel=(0, 3), trains=(1, 3)))

    tout += lieu("lieu_port.txt", "Le port", "Pontons pourris : une seule traversée", "port", "port",
         [("Léa", "Voici le port, avec ses bateaux et ses conteneurs."),
          ("Léa", "Ces pontons sont pourris : on ne peut les traverser qu'une fois, ensuite ils s'effondrent dans l'eau.", "gros"),
          ("Max", "Ça craque sous mes pieds !", "moyen")],
         [("Zoé", "Je sens l'odeur du sel et du bois mouillé."),
          ("Léa", "Réfléchissez à votre chemin avant de vous engager.", "moyen")],
         [("Léa", "Le port est derrière nous. Prochaine escale : un village médiéval !"),
          ("Rémi", "J'ai évité les pontons grâce à mes roues.", "moyen")],
         six("Au port", 800, T, blocs=(2, 4), trous=(0, 2), tapis=(0, 3), pontons=(2, 6)))

    tout += lieu("lieu_medieval.txt", "Le village médiéval", "Des herses qui s'ouvrent et se ferment", "medieval", "medieval",
         [("Léa", "Nous entrons dans le village médiéval, derrière ses remparts."),
          ("Léa", "Ces herses se ferment au rythme de vos pas : vertes, elles sont ouvertes ; oranges, elles vont se fermer ; rouges, elles sont fermées.", "gros"),
          ("Max", "Je ne veux pas me retrouver coincé !", "moyen")],
         [("Zoé", "J'entends le bruit des chaînes. Les herses bougent."),
          ("Léa", "Attendez l'ouverture, passez avec toute la file.", "moyen")],
         [("Léa", "Le village est derrière nous. Dernière étape : les grottes !"),
          ("Rémi", "Les pavés, ce n'est pas pour mon fauteuil... mais quel bel endroit.", "moyen")],
         six("Au village", 900, T, blocs=(2, 4), trous=(0, 2), herses=(4, 10)))

    tout += lieu("lieu_grottes.txt", "Les grottes", "Des pierres tombent du plafond", "grottes", "grottes",
         [("Léa", "Bienvenue dans les grottes. Il fait sombre, prenez garde."),
          ("Léa", "De temps en temps, une case clignote : une pierre va tomber et elle restera là. Ne restez pas dessous !", "gros"),
          ("Zoé", "J'entends les cailloux rouler. Ça gronde.", "moyen")],
         [("Max", "Une pierre vient de tomber derrière nous. C'était juste !"),
          ("Léa", "Gardez le rythme, et regardez bien où ça clignote.", "moyen")],
         [("Léa", "Nous voilà de retour à la lumière. Quelle aventure !"),
          ("Rémi", "Je n'en reviens pas. On a traversé tous ces lieux ensemble.", "moyen")],
         six("Dans les grottes", 1000, T, blocs=(2, 4), trous=(0, 2), eboulements=(6, 16)))
    ecrire("aventure.txt", "Le grand voyage", "Du tutoriel aux grottes : une visite guidée en onze lieux, avec Léa, Max, Zoé et Rémi", tout)


if __name__ == "__main__":
    sys.exit(main())
