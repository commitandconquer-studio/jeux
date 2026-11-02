#!/usr/bin/env python3
"""Génère la campagne « Le labyrinthe du Minotaure » (games/snake/scenarios/labyrinthe.txt) : vingt niveaux dans les ruines de
Cnossos. Chaque niveau est un vrai labyrinthe : des couloirs de deux cases de large, séparés par des murs, qui tournent,
bifurquent et se recoupent. Il n'y a aucune impasse (la file ne peut pas faire demi-tour) : chaque cul-de-sac est rouvert sur un
autre couloir. Les niveaux grandissent peu à peu et ajoutent des éléments : trous, planches (dalles qui s'effondrent après un
passage, toujours contournables), blocs, grandes salles ouvertes, et salles secrètes qu'on ne rejoint que par un téléporteur
(un pour entrer, un autre dans la même salle pour ressortir).

L'entrée et la sortie sont sur des côtés quelconques du plateau ; la sortie d'un niveau se raccorde à l'entrée du suivant :
sortir en haut, c'est recommencer en bas, à la même hauteur relative le long du bord. Chaque carte est vérifiée avant d'être
écrite ; le jeu refait ces contrôles dans un test.

    python3 tools/gen-labyrinthe.py
"""
import os
import random
import sys

ICI = os.path.dirname(os.path.abspath(__file__))
SORTIE = os.path.join(ICI, "..", "games", "snake", "scenarios", "labyrinthe.txt")

BLOCKED = set("234567oHh")
FEATURES = set("2345oApBCDzSE")
N4 = [(-1, 0), (1, 0), (0, -1), (0, 1)]

# côté de l'entrée -> (nom de la direction du départ, vecteur y/x) ; la sortie du niveau précédent est du côté opposé
OPPOSITE = {"G": "D", "D": "G", "H": "B", "B": "H"}
DEPART = {"G": ("est", (0, 1)), "D": ("ouest", (0, -1)), "H": ("sud", (1, 0)), "B": ("nord", (-1, 0))}


# ---------------------------------------------------------------- contrôles
def problems(rows, tourists, depart):
    """Mêmes contrôles que le jeu : départ avec de la place, cases atteignables (téléporteurs compris), pas de cul-de-sac."""
    h, w = len(rows), len(rows[0])

    def ok(y, x):
        return 0 <= y < h and 0 <= x < w and rows[y][x] != "0" and rows[y][x] not in BLOCKED

    s = [(y, x) for y in range(h) for x in range(w) if rows[y][x] == "S"]
    if len(s) != 1:
        return ["pas un seul départ"]
    sy, sx = s[0]
    dy, dx = depart
    out = []
    if not (ok(sy - dy, sx - dx) and ok(sy - 2 * dy, sx - 2 * dx) and ok(sy + dy, sx + dx)):
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
        elif abs(v[0][0] - v[1][0]) + abs(v[0][1] - v[1][1]) < 4:
            out.append(f"téléporteur {k} trop proche de son jumeau")
    seen, st = set(), [(sy, sx)]
    while st:
        c = st.pop()
        if c in seen or not ok(*c):
            continue
        seen.add(c)
        st.extend((c[0] + a, c[1] + b) for a, b in N4)
        if rows[c[0]][c[1]] in "ABCD":
            st.extend(tele.get(rows[c[0]][c[1]], []))
    if len(seen) != len(opened):
        out.append(f"{len(opened) - len(seen)} cases inaccessibles")
    dead = sum(1 for (y, x) in opened if sum(1 for a, b in N4 if ok(y + a, x + b)) < 2)
    if dead:
        out.append(f"{dead} culs-de-sac")
    return out


def find(rows, ch):
    for y, r in enumerate(rows):
        for x, c in enumerate(r):
            if c == ch:
                return (y, x)


def walk_distance(rows):
    """Distance à pied (sans téléporteur) de S à E ; None si la sortie n'est pas atteignable à pied."""
    h, w = len(rows), len(rows[0])
    start, goal = find(rows, "S"), find(rows, "E")
    dist, queue = {start: 0}, [start]
    for c in queue:
        if c == goal:
            return dist[c]
        for a, b in N4:
            n = (c[0] + a, c[1] + b)
            if 0 <= n[0] < h and 0 <= n[1] < w and n not in dist and rows[n[0]][n[1]] != "0" and rows[n[0]][n[1]] not in BLOCKED:
                dist[n] = dist[c] + 1
                queue.append(n)
    return None


def crowded(rows, y, x, r):
    h, w = len(rows), len(rows[0])
    return any(0 <= y + a < h and 0 <= x + b < w and rows[y + a][x + b] in FEATURES for a in range(-r, r + 1) for b in range(-r, r + 1) if (a, b) != (0, 0))


def free(rows, y, x):
    return 0 <= y < len(rows) and 0 <= x < len(rows[0]) and rows[y][x] == "1"


# ---------------------------------------------------------------- le labyrinthe
def door_block(side, t, cw, ch):
    if side in "GD":
        return (0 if side == "G" else cw - 1, round(t * (ch - 1)))
    return (round(t * (cw - 1)), 0 if side == "H" else ch - 1)


def rect_overlap(a, b):
    return not (a[0] + a[2] <= b[0] or b[0] + b[2] <= a[0] or a[1] + a[3] <= b[1] or b[1] + b[3] <= a[1])


def maze(cw, ch, rng, entry, exit_, islands, halls, loops):
    """Les blocs de 2×2 cases, reliés par des ouvertures dans les murs. Renvoie (arêtes ouvertes, salles secrètes, grandes
    salles) ou None si le tirage n'est pas bon."""
    blocked, rects_i, rects_h = set(), [], []
    doors = {door_block(entry[0], entry[1], cw, ch), door_block(exit_[0], exit_[1], cw, ch)}

    def place(w, h, kind):
        for _ in range(300):
            i, j = rng.randint(1, cw - 1 - w), rng.randint(1, ch - 1 - h)
            cells = {(i + a, j + b) for a in range(w) for b in range(h)}
            ring = {(i + a, j + b) for a in range(-1, w + 1) for b in range(-1, h + 1)}
            if kind == "i" and (ring & blocked or ring & doors or any(rect_overlap((i - 1, j - 1, w + 2, h + 2), r) for r in rects_h)):
                continue
            if kind == "h" and (cells & blocked or cells & doors or any(rect_overlap((i, j, w, h), r) for r in rects_h)):
                continue
            return (i, j, w, h), cells
        return None, None

    for w, h in islands:
        r, cells = place(w, h, "i")
        if r is None:
            return None
        rects_i.append(r)
        blocked |= cells
    for w, h in halls:
        r, cells = place(w, h, "h")
        if r is None:
            return None
        rects_h.append(r)

    nodes = {(i, j) for i in range(cw) for j in range(ch) if (i, j) not in blocked}
    edges, seen = set(), set()
    start = rng.choice(sorted(nodes))
    stack = [start]
    seen.add(start)
    while stack:
        c = stack[-1]
        nb = [(c[0] + a, c[1] + b) for a, b in ((1, 0), (-1, 0), (0, 1), (0, -1)) if (c[0] + a, c[1] + b) in nodes and (c[0] + a, c[1] + b) not in seen]
        if not nb:
            stack.pop()
            continue
        n = rng.choice(nb)
        edges.add(frozenset([c, n]))
        seen.add(n)
        stack.append(n)
    if seen != nodes:
        return None
    # Les grandes salles : toutes leurs cloisons s'ouvrent.
    for (i, j, w, h) in rects_h:
        for a in range(w):
            for b in range(h):
                if a + 1 < w:
                    edges.add(frozenset([(i + a, j + b), (i + a + 1, j + b)]))
                if b + 1 < h:
                    edges.add(frozenset([(i + a, j + b), (i + a, j + b + 1)]))

    def degree(k):
        return sum(1 for e in edges if k in e) + (1 if k in doors else 0)

    # Aucune impasse : un bloc qui n'a qu'une issue s'ouvre sur un voisin de plus.
    for k in sorted(nodes):
        while degree(k) < 2:
            cand = [(k[0] + a, k[1] + b) for a, b in ((1, 0), (-1, 0), (0, 1), (0, -1)) if (k[0] + a, k[1] + b) in nodes and frozenset([k, (k[0] + a, k[1] + b)]) not in edges]
            if not cand:
                return None
            edges.add(frozenset([k, rng.choice(cand)]))
    # Quelques boucles de plus.
    for k in sorted(nodes):
        for a, b in ((1, 0), (0, 1)):
            n = (k[0] + a, k[1] + b)
            if n in nodes and frozenset([k, n]) not in edges and rng.random() < loops:
                edges.add(frozenset([k, n]))
    return edges, rects_i, rects_h


def render(cw, ch, edges, rects_i, rects_h, entry, exit_):
    sides = (entry[0], exit_[0])
    ox, oy = 3 if "G" in sides else 0, 3 if "H" in sides else 0
    rx, by = 3 if "D" in sides else 0, 3 if "B" in sides else 0
    wm, hm = 3 * cw - 1, 3 * ch - 1
    w, h = ox + wm + rx, oy + hm + by
    rows = [["0"] * w for _ in range(h)]
    in_island = {(i + p, j + q) for (i, j, a, b) in rects_i for p in range(a) for q in range(b)}

    def fill(y0, y1, x0, x1):
        for y in range(y0, y1 + 1):
            for x in range(x0, x1 + 1):
                rows[y][x] = "1"

    def box(r):
        i, j, a, b = r
        return (oy + 3 * j, oy + 3 * (j + b) - 2, ox + 3 * i, ox + 3 * (i + a) - 2)

    for i in range(cw):
        for j in range(ch):
            if (i, j) not in in_island:
                fill(oy + 3 * j, oy + 3 * j + 1, ox + 3 * i, ox + 3 * i + 1)
    for e in edges:
        a, b = sorted(e)
        if a[1] == b[1]:  # côte à côte
            fill(oy + 3 * a[1], oy + 3 * a[1] + 1, ox + 3 * a[0] + 2, ox + 3 * a[0] + 2)
        else:
            fill(oy + 3 * a[1] + 2, oy + 3 * a[1] + 2, ox + 3 * a[0], ox + 3 * a[0] + 1)
    for r in rects_h + rects_i:  # grande salle ou salle secrète : le rectangle entier, piliers compris
        fill(*box(r))
    marks = {}
    for kind, (side, t) in (("S", entry), ("E", exit_)):
        bi, bj = door_block(side, t, cw, ch)
        if side == "G":
            y = oy + 3 * bj
            fill(y, y + 1, 0, ox - 1)
            pos = (y, ox - 1 if kind == "S" else 0)
        elif side == "D":
            y = oy + 3 * bj
            fill(y, y + 1, ox + wm, w - 1)
            pos = (y, w - 3 if kind == "S" else w - 1)
        elif side == "H":
            x = ox + 3 * bi
            fill(0, oy - 1, x, x + 1)
            pos = (oy - 1 if kind == "S" else 0, x)
        else:
            x = ox + 3 * bi
            fill(oy + hm, h - 1, x, x + 1)
            pos = (h - 3 if kind == "S" else h - 1, x)
        marks[kind] = pos
    for kind, (y, x) in marks.items():
        rows[y][x] = kind
    return rows, [box(r) for r in rects_i], [box(r) for r in rects_h]


# ---------------------------------------------------------------- les éléments
def place_blocks_and_holes(rows, rng, blocs, trous, halls, tourists, depart):
    """Blocs au centre des grandes salles ; trous dans les couloirs et les salles, jamais près d'un autre élément."""
    for _ in range(blocs):
        for _ in range(300):
            if not halls:
                return False
            y0, y1, x0, x1 = rng.choice(halls)
            if y1 - y0 < 3 or x1 - x0 < 3:
                continue
            y, x = rng.randint(y0 + 1, y1 - 1), rng.randint(x0 + 1, x1 - 1)
            if all(free(rows, y + a, x + b) for a in (-1, 0, 1) for b in (-1, 0, 1)) and not crowded(rows, y, x, 2):
                rows[y][x] = "2"
                break
        else:
            return False
    h, w = len(rows), len(rows[0])
    for _ in range(trous):
        for _ in range(800):
            y, x = rng.randrange(h), rng.randrange(w)
            if not free(rows, y, x) or crowded(rows, y, x, 2):
                continue
            if sum(1 for a, b in N4 if free(rows, y + a, x + b)) < 3:
                continue
            rows[y][x] = "o"
            if problems(rows, tourists, depart):
                rows[y][x] = "1"
                continue
            break
        else:
            return False
    return True


def place_planks(rows, rng, runs, tourists, depart):
    """Des planches par séries de trois, sur une seule file d'un couloir de deux cases : l'autre file reste libre."""
    h, w = len(rows), len(rows[0])
    for _ in range(runs):
        for _ in range(1500):
            y, x = rng.randrange(h), rng.randrange(w)
            a, b = rng.choice(N4)
            perp = (b, a) if rng.random() < 0.5 else (-b, -a)
            cells = [(y + a * k, x + b * k) for k in range(3)]
            side = [(c[0] + perp[0], c[1] + perp[1]) for c in cells]
            before, after = (y - a, x - b), (y + 3 * a, x + 3 * b)
            if not all(free(rows, *c) for c in cells + side + [before, after]):
                continue
            if any(crowded(rows, c[0], c[1], 1) for c in cells):
                continue
            for c in cells:
                rows[c[0]][c[1]] = "p"
            if problems(rows, tourists, depart):
                for c in cells:
                    rows[c[0]][c[1]] = "1"
                continue
            break
        else:
            return False
    return True


def place_pads(rows, rng, boxes, pairs, tourists, depart):
    """Pour chaque salle secrète : deux téléporteurs dedans, chacun relié à une case du labyrinthe, loin de la salle ;
    puis les paires simples (deux cases du labyrinthe)."""
    h, w = len(rows), len(rows[0])
    letters = iter("ABCD")

    def spot(avoid):
        for _ in range(3000):
            y, x = rng.randrange(h), rng.randrange(w)
            if not free(rows, y, x) or crowded(rows, y, x, 2) or sum(1 for a, b in N4 if free(rows, y + a, x + b)) < 3:
                continue
            if all(abs(y - p[0]) + abs(x - p[1]) >= 10 for p in avoid):
                return (y, x)
        return None

    for (y0, y1, x0, x1) in boxes:
        corners = [(y0, x0), (y1, x1)]
        for inner in corners:
            letter = next(letters)
            rows[inner[0]][inner[1]] = letter
            for _ in range(3000):
                p = spot([])
                if p is None:
                    return False
                if y0 - 6 <= p[0] <= y1 + 6 and x0 - 6 <= p[1] <= x1 + 6:
                    continue
                rows[p[0]][p[1]] = letter
                break
            else:
                return False
    for _ in range(pairs):
        letter = next(letters)
        a = spot([])
        if a is None:
            return False
        rows[a[0]][a[1]] = letter
        b = spot([a])
        if b is None:
            return False
        rows[b[0]][b[1]] = letter
    return not problems(rows, tourists, depart)


# ---------------------------------------------------------------- niveaux
def niveau(nom, spec, entry, exit_, seed):
    cw, ch = spec["taille"]
    tourists = spec["touristes"]
    depart_name, depart = DEPART[entry[0]]
    for attempt in range(600):
        rng = random.Random(seed * 10007 + attempt)
        built = maze(cw, ch, rng, entry, exit_, spec.get("secretes", []), spec.get("salles", []), spec.get("boucles", 0.03))
        if built is None:
            continue
        edges, rects_i, rects_h = built
        rows, island_boxes, hall_boxes = render(cw, ch, edges, rects_i, rects_h, entry, exit_)
        walk = walk_distance(rows)
        if walk is None:
            continue
        s, e = find(rows, "S"), find(rows, "E")
        if walk < 1.35 * (abs(s[0] - e[0]) + abs(s[1] - e[1])) + 6:
            continue
        if not place_pads(rows, rng, island_boxes, spec.get("paires", 0), tourists, depart):
            continue
        if not place_blocks_and_holes(rows, rng, spec.get("blocs", 0), spec.get("trous", 0), hall_boxes, tourists, depart):
            continue
        if not place_planks(rows, rng, spec.get("planches", 0), tourists, depart):
            continue
        if problems(rows, tourists, depart):
            continue
        txt = f"[{nom}]\nlieu = ruines\ntouristes = {tourists}\npas = {spec['pas']}\ndepart = {depart_name}\nsortie = oui\ncarte:\n"
        return txt + "\n".join("".join(r) for r in rows) + "\n"
    raise SystemExit(f"niveau « {nom} » introuvable")


NIVEAUX = [
    ("L'entrée du labyrinthe", dict(taille=(7, 5), touristes=8, pas=0.34, trous=2, planches=1)),
    ("Premier carrefour", dict(taille=(7, 5), touristes=9, pas=0.34, trous=3, planches=1)),
    ("Les couloirs sombres", dict(taille=(8, 6), touristes=10, pas=0.33, trous=3, planches=2)),
    ("La galerie des fresques", dict(taille=(9, 6), touristes=12, pas=0.33, trous=4, planches=2)),
    ("Les dalles fragiles", dict(taille=(9, 6), touristes=12, pas=0.33, trous=3, planches=3, paires=1)),
    ("Le pont effondré", dict(taille=(9, 7), touristes=14, pas=0.32, trous=4, planches=3, paires=1)),
    ("Les voûtes", dict(taille=(10, 7), touristes=16, pas=0.32, trous=4, planches=3, paires=1, salles=[(2, 2)])),
    ("La cour brisée", dict(taille=(11, 7), touristes=18, pas=0.32, trous=5, planches=3, paires=1, salles=[(2, 2)])),
    ("La salle des tourbillons", dict(taille=(11, 7), touristes=18, pas=0.31, trous=3, planches=3, secretes=[(2, 2)])),
    ("Le passage caché", dict(taille=(11, 7), touristes=20, pas=0.31, trous=4, planches=3, secretes=[(2, 2)], salles=[(2, 2)])),
    ("Les escaliers perdus", dict(taille=(11, 8), touristes=20, pas=0.31, trous=4, planches=4, secretes=[(2, 2)], salles=[(2, 2)])),
    ("La chambre secrète", dict(taille=(11, 8), touristes=22, pas=0.30, trous=5, planches=4, secretes=[(3, 2)], salles=[(2, 2), (2, 2)])),
    ("La grande salle", dict(taille=(12, 8), touristes=22, pas=0.30, trous=4, planches=3, blocs=2, secretes=[(3, 2)], salles=[(3, 2), (2, 2)])),
    ("Le dédale des colonnes", dict(taille=(12, 8), touristes=24, pas=0.30, trous=5, planches=4, blocs=3, secretes=[(3, 2)], salles=[(3, 2), (2, 2)])),
    ("La salle du trône", dict(taille=(12, 8), touristes=24, pas=0.30, trous=4, planches=4, blocs=2, secretes=[(2, 2), (2, 2)], salles=[(3, 2), (2, 2)])),
    ("Les caves du palais", dict(taille=(12, 8), touristes=26, pas=0.29, trous=5, planches=4, blocs=3, secretes=[(2, 2), (2, 2)], salles=[(3, 2), (2, 2), (2, 2)])),
    ("La galerie des amphores", dict(taille=(12, 8), touristes=26, pas=0.29, trous=6, planches=5, blocs=3, secretes=[(2, 2), (2, 2)], salles=[(3, 2), (2, 2), (2, 2)])),
    ("Le labyrinthe des labyrinthes", dict(taille=(12, 8), touristes=28, pas=0.29, trous=6, planches=5, blocs=3, secretes=[(2, 2), (3, 2)], salles=[(3, 2), (2, 2), (2, 2)])),
    ("Le cœur du dédale", dict(taille=(12, 8), touristes=28, pas=0.28, trous=6, planches=5, blocs=4, secretes=[(2, 2), (3, 2)], salles=[(3, 2), (3, 2), (2, 2)])),
    ("La sortie du Minotaure", dict(taille=(12, 8), touristes=30, pas=0.28, trous=7, planches=5, blocs=4, secretes=[(3, 2), (3, 2)], salles=[(3, 2), (3, 2), (2, 2)])),
]


def portes():
    """Les portes : celle d'entrée du premier niveau, puis la sortie de chaque niveau (côté et position le long du bord).
    L'entrée du niveau suivant est du côté opposé, au même endroit relatif."""
    rng = random.Random(2026)
    out = [("G", 0.5)]
    for k in range(len(NIVEAUX)):
        entry_side = OPPOSITE[out[-1][0]] if k else "G"
        side = rng.choice([s for s in "GDHB" if s != entry_side])
        out.append((side, rng.choice([0.2, 0.35, 0.5, 0.65, 0.8])))
    return out


# ---------------------------------------------------------------- scènes
LEA = ("Léa", 1, "1.6.1.0.2", "guide", "bequille")
MAX = ("Max", 2, "5.0.3.2.1", "-", "lunettes-soleil")
ZOE = ("Zoé", 5, "0.8.5.0.2", "-", "canne-blanche")
REMI = ("Rémi", 16, "2.3.4.1.1", "-", "")
GARDIEN = ("Le gardien", 6, "2.7.9.3.2", "-", "lunettes")


def scene(name, camera, cast, lines):
    t = f"[{name}]\ntype = scene\ndecor = ruines\ncamera = {camera}\n"
    for (nom, kind, look, role, acc), place, anim in cast:
        t += f"personnage = {nom} | {kind} | {look} | {place} | {anim} | {role}" + (f" | {acc}" if acc else "") + "\n"
    for line in lines:
        t += "replique = " + " | ".join(line) + "\n"
    return t


def main():
    doors = portes()
    sections = [scene("Perdue dans le labyrinthe", "large", [(LEA, "centre", "salut")], [
        ("Léa", "Oh non... j'ai perdu mon groupe dans le labyrinthe de Cnossos !"),
        ("Léa", "Ce dédale est immense. Aide-moi à retrouver tous mes touristes avant de sortir d'ici.", "gros"),
        ("Léa", "Suis-moi, et ne te perds pas à ton tour !", "moyen")])]
    after = {
        3: scene("Max est retrouvé", "moyen", [(LEA, "centre", "salut"), (MAX, "gauche", "salut")], [
            ("Max", "Léa ! Ça fait une heure que je tourne en rond entre les murs !"),
            ("Léa", "Reste près de moi. Il en manque encore beaucoup.", "moyen")]),
        7: scene("Zoé est retrouvée", "moyen", [(LEA, "centre", "salut"), (MAX, "gauche", "repos"), (ZOE, "droite", "salut")], [
            ("Zoé", "J'entendais ta voix de l'autre côté du mur, mais impossible de te rejoindre."),
            ("Léa", "Courage, on est de plus en plus nombreux. Continuons.", "moyen")]),
        11: scene("Rémi est retrouvé", "moyen", [(LEA, "centre", "salut"), (MAX, "gauche", "repos"), (ZOE, "droite", "repos"), (REMI, "gauche2", "fauteuil-assis")], [
            ("Rémi", "Ces couloirs sont à peine plus larges que mon fauteuil. Je vous cherchais partout !"),
            ("Léa", "Plus personne ne se sépare. On avance tous ensemble.", "moyen")]),
        15: scene("Le gardien", "moyen", [(LEA, "centre", "salut"), (MAX, "gauche", "repos"), (ZOE, "droite", "repos"), (REMI, "gauche2", "fauteuil-assis"), (GARDIEN, "droite2", "salut")], [
            ("Le gardien", "Encore des visiteurs dans le dédale ? Je ferme bientôt, dépêchez-vous !"),
            ("Léa", "Il reste des touristes égarés. Nous allons les chercher.", "moyen")]),
    }
    for k, (nom, spec) in enumerate(NIVEAUX):
        entry = (OPPOSITE[doors[k][0]], doors[k][1]) if k else doors[0]
        sections.append(niveau(nom, spec, entry, doors[k + 1], k + 1))
        if k in after:
            sections.append(after[k])
    sections.append(scene("Dehors", "moyen", [(LEA, "centre", "salut"), (MAX, "gauche", "salut"), (ZOE, "droite", "repos"), (REMI, "gauche2", "fauteuil-assis"), (GARDIEN, "droite2", "salut")], [
        ("Léa", "Nous voilà dehors, et tout le monde est là ! Merci de m'avoir aidée."),
        ("Le gardien", "Le Minotaure, c'est moi : je garde ce site depuis trente ans. Désolé pour le mugissement, j'avais un rhume."),
        ("Max", "Un Minotaure enrhumé... Je ne m'en remettrai pas.", "moyen")]))
    with open(SORTIE, "w") as f:
        f.write("# Généré par tools/gen-labyrinthe.py — ne pas modifier à la main (relancer le script).\n")
        f.write("titre = Le labyrinthe du Minotaure\n")
        f.write("description = Léa a perdu son groupe dans le labyrinthe de Cnossos : retrouvez les touristes, puis ressortez\n\n")
        f.write("\n".join(sections))
    print("écrit labyrinthe.txt")
    return 0


if __name__ == "__main__":
    sys.exit(main())
