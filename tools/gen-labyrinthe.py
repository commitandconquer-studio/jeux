#!/usr/bin/env python3
"""Génère la campagne « Le labyrinthe du Minotaure » (games/snake/scenarios/labyrinthe.txt) : huit niveaux dans les ruines de
Cnossos. Chaque niveau est un labyrinthe de salles (3 lignes × 3 ou 4 colonnes) reliées par des couloirs : 1 case de large
(passages obligés) ou 2 cases. Toutes les salles ont au moins deux issues (pas de salle sans issue : la file ne pourrait pas
faire demi-tour). L'entrée est à gauche au milieu, la sortie à droite au milieu, sur la même rangée dans tous les niveaux :
la sortie d'un niveau se raccorde à l'entrée du suivant. Chaque carte est vérifiée avant d'être écrite ; le jeu refait ces
contrôles dans un test.

    python3 tools/gen-labyrinthe.py
"""
import importlib.util
import os
import random
import sys

ICI = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("aventures", os.path.join(ICI, "gen-aventures.py"))
av = importlib.util.module_from_spec(spec)
spec.loader.exec_module(av)

H = 18          # hauteur commune à tous les niveaux : l'entrée et la sortie sont sur la même rangée
M = 9           # rangée de l'entrée et de la sortie (le milieu)
SW, SH = 6, 6   # taille d'un emplacement de salle
X0 = 4          # largeur du vestibule d'entrée


class Room:
    def __init__(self, r, c, rng, big):
        self.r, self.c = r, c
        edge = (r == 1 and c in (0, None))  # remplacé juste après pour la salle d'entrée et de sortie
        self.w = rng.choice([3, 4, 4] if not big else [4, 4])
        self.h = rng.choice([3, 4, 4] if not big else [4, 4])
        self.ox = rng.randint(1, 5 - self.w)
        self.oy = rng.randint(1, 5 - self.h)

    @property
    def x0(self):
        return X0 + SW * self.c + self.ox

    @property
    def y0(self):
        return SH * self.r + self.oy

    @property
    def x1(self):  # exclu
        return self.x0 + self.w

    @property
    def y1(self):
        return self.y0 + self.h


def build(cols, p_narrow, seed, min_route):
    """Un labyrinthe : (rangées de la carte, salles, liens). Réessaie avec d'autres tirages tant que ce n'est pas bon."""
    for attempt in range(500):
        rng = random.Random(seed * 1000 + attempt)
        rooms = {}
        for r in range(3):
            for c in range(cols):
                rooms[(r, c)] = Room(r, c, rng, big=False)
        # Les salles d'entrée et de sortie : quatre cases de haut, sur les rangées M-2..M+1, pour un couloir de deux cases.
        for key in ((1, 0), (1, cols - 1)):
            rm = rooms[key]
            rm.h, rm.oy = 4, 1
            rm.w = 4
            rm.ox = 1
        # Arbre couvrant en parcours en profondeur (chemin sinueux), puis une issue de plus aux salles qui n'en ont qu'une.
        edges, seen, stack = set(), {(1, 0)}, [(1, 0)]
        while stack:
            r, c = stack[-1]
            nb = [(r + dr, c + dc) for dr, dc in ((0, 1), (1, 0), (0, -1), (-1, 0)) if (r + dr, c + dc) in rooms and (r + dr, c + dc) not in seen]
            if not nb:
                stack.pop()
                continue
            n = rng.choice(nb)
            edges.add(frozenset([(r, c), n]))
            seen.add(n)
            stack.append(n)

        def degree(k):
            d = sum(1 for e in edges if k in e)
            return d + (1 if k in ((1, 0), (1, cols - 1)) else 0)

        for k in rooms:
            while degree(k) < 2:
                r, c = k
                cand = [(r + dr, c + dc) for dr, dc in ((0, 1), (1, 0), (0, -1), (-1, 0)) if (r + dr, c + dc) in rooms and frozenset([k, (r + dr, c + dc)]) not in edges]
                if not cand:
                    break
                edges.add(frozenset([k, rng.choice(cand)]))
        if any(degree(k) < 2 for k in rooms):
            continue
        # La route la plus courte de l'entrée à la sortie doit traverser assez de salles.
        route = shortest(edges, (1, 0), (1, cols - 1))
        if route is None or len(route) < min_route:
            continue
        rows = carve(rooms, edges, cols, rng, p_narrow)
        if rows:
            return rows, rooms, edges
    raise SystemExit(f"labyrinthe introuvable (graine {seed})")


def shortest(edges, a, b):
    prev, queue = {a: None}, [a]
    for k in queue:
        if k == b:
            break
        for e in edges:
            if k in e:
                (n,) = e - {k}
                if n not in prev:
                    prev[n] = k
                    queue.append(n)
    if b not in prev:
        return None
    out, k = [], b
    while k is not None:
        out.append(k)
        k = prev[k]
    return out[::-1]


def carve(rooms, edges, cols, rng, p_narrow):
    W = X0 + SW * cols + 3
    rows = [["0"] * W for _ in range(H)]

    def fill(y0, y1, x0, x1):
        for y in range(y0, y1):
            for x in range(x0, x1):
                rows[y][x] = "1"

    for rm in rooms.values():
        fill(rm.y0, rm.y1, rm.x0, rm.x1)
    # Entrée et sortie : un vestibule de deux cases de large, rangées M et M+1.
    first, last = rooms[(1, 0)], rooms[(1, cols - 1)]
    fill(M, M + 2, 0, first.x0)
    fill(M, M + 2, last.x1, W)
    # Les couloirs entre salles voisines.
    for e in edges:
        a, b = sorted(e)
        ra, rb = rooms[a], rooms[b]
        width = 1 if rng.random() < p_narrow else 2
        if a[0] == b[0]:  # côte à côte : couloir horizontal
            lo, hi = max(ra.y0, rb.y0), min(ra.y1, rb.y1)
            if hi - lo < width:
                return None
            y = rng.randint(lo, hi - width)
            fill(y, y + width, ra.x1, rb.x0)
        else:  # l'un au-dessus de l'autre : couloir vertical
            lo, hi = max(ra.x0, rb.x0), min(ra.x1, rb.x1)
            if hi - lo < width:
                return None
            x = rng.randint(lo, hi - width)
            fill(ra.y1, rb.y0, x, x + width)
    rows[M][2] = "S"
    rows[M][W - 1] = "E"
    return rows


def hazards(rows, rooms, rng, blocs, trous, paires, tourists):
    """Blocs et trous au centre des salles (jamais sur un passage), téléporteurs aux bords de salles éloignées."""
    cores = []
    for k, rm in rooms.items():
        if k in ((1, 0),) or rm.w < 4 or rm.h < 4:
            continue
        cores.append([(y, x) for y in range(rm.y0 + 1, rm.y1 - 1) for x in range(rm.x0 + 1, rm.x1 - 1)])
    rng.shuffle(cores)
    placed = []
    for ch in "2" * blocs + "o" * trous:
        for core in cores:
            if core in [c for c, _ in placed]:
                continue
            y, x = rng.choice(core)
            if all(rows[y + dy][x + dx] == "1" for dy in (-1, 0, 1) for dx in (-1, 0, 1)):
                rows[y][x] = ch
                placed.append((core, ch))
                break
        else:
            return False
    ring = []
    for rm in rooms.values():
        cells = [(y, x) for y in range(rm.y0, rm.y1) for x in range(rm.x0, rm.x1) if (y in (rm.y0, rm.y1 - 1)) != (x in (rm.x0, rm.x1 - 1))]
        ring.append((rm, cells))
    for i, letter in enumerate("ABCD"[:paires]):
        for _ in range(2000):
            (ra, ca), (rb, cb) = rng.sample(ring, 2)
            (y1, x1), (y2, x2) = rng.choice(ca), rng.choice(cb)
            if abs(y1 - y2) + abs(x1 - x2) < 8:
                continue
            if (ra.r, ra.c) == (1, 0) or (rb.r, rb.c) == (1, 0):
                continue
            if rows[y1][x1] != "1" or rows[y2][x2] != "1" or av.near_feature(rows, y1, x1) or av.near_feature(rows, y2, x2):
                continue
            rows[y1][x1] = rows[y2][x2] = letter
            break
        else:
            return False
    return not av.problems(rows, tourists)


def niveau(nom, cols, narrow, tourists, pace, seed, min_route, blocs=0, trous=0, paires=0):
    for attempt in range(60):
        rows, rooms, _ = build(cols, narrow, seed + attempt * 7919, min_route)
        rng = random.Random(seed + attempt)
        if not hazards(rows, rooms, rng, blocs, trous, paires, tourists):
            continue
        if av.problems(rows, tourists):
            continue
        # Entrée et sortie sur la même rangée dans tous les niveaux.
        assert rows[M][2] == "S" and rows[M][-1] == "E"
        txt = f"[{nom}]\nlieu = ruines\ntouristes = {tourists}\npas = {pace}\ndepart = est\nsortie = oui\ncarte:\n"
        return txt + "\n".join("".join(r) for r in rows) + "\n"
    raise SystemExit(f"niveau « {nom} » introuvable")


# ---------------------------------------------------------------- scènes
GARDIEN = ("Le gardien", 6, "2.7.9.3.2", "-", "lunettes")


def groupe(camera, lines, name, leader_anim="salut", pose=("repos", "repos", "fauteuil-assis"), extra=None):
    cast = [(av.LEA, "centre", leader_anim), (av.MAX, "gauche", pose[0]), (av.ZOE, "droite", pose[1]), (av.REMI, "gauche2", pose[2])]
    if extra:
        cast.append(extra)
    return av.scene(name, "ruines", camera, cast, lines)


def main():
    s = []
    s.append(groupe("large", [
        ("Léa", "Voici Cnossos, et son fameux labyrinthe ! Mais attention : une partie du groupe s'est égarée dans les couloirs."),
        ("Léa", "On entre par la gauche, on cherche tout le monde, et on ressort par la droite. Suivez-moi de près !", "gros"),
        ("Max", "Des salles reliées par des passages d'une case seulement ? Ça va être serré.", "moyen")], "Le labyrinthe : entrée"))
    s.append(niveau("Les premières salles", 3, 0.3, 8, 0.33, 11, 5))
    s.append(niveau("La salle des amphores", 3, 0.45, 10, 0.33, 12, 5))
    s.append(niveau("Les passages étroits", 3, 0.85, 12, 0.32, 13, 5))
    s.append(groupe("moyen", [
        ("Max", "Je crois avoir entendu un mugissement, de l'autre côté du mur..."),
        ("Léa", "Ce n'est que le vent dans les couloirs. Continuons, la sortie est de l'autre côté.", "moyen")], "Le labyrinthe : le mugissement"))
    s.append(niveau("Le fil d'Ariane", 4, 0.6, 14, 0.32, 14, 6, paires=1))
    s.append(niveau("Les dalles effondrées", 4, 0.6, 16, 0.31, 15, 6, trous=3, paires=1))
    s.append(niveau("La salle des colonnes", 4, 0.55, 18, 0.31, 16, 6, blocs=3, trous=2, paires=1))
    s.append(groupe("moyen", [
        ("Zoé", "J'entends des sabots, quelque part devant nous. Quelqu'un nous attend."),
        ("Léa", "Plus que deux salles avant le cœur du labyrinthe. Gardons le groupe serré.", "moyen")], "Le labyrinthe : les sabots"))
    s.append(niveau("Le cœur du labyrinthe", 4, 0.75, 20, 0.30, 17, 6, blocs=2, trous=3, paires=2))
    s.append(groupe("moyen", [
        ("Rémi", "Ces murs sont plus hauts que mon fauteuil... et ce couloir est plus étroit que mes roues !"),
        ("Léa", "Courage, la grille de sortie est tout près.", "moyen")], "Le labyrinthe : le fauteuil"))
    s.append(niveau("La sortie du Minotaure", 4, 0.7, 24, 0.30, 18, 6, blocs=3, trous=4, paires=2))
    s.append(groupe("moyen", [
        ("Léa", "Nous voilà dehors, tout le monde est là ! Et le Minotaure alors ?"),
        ("Le gardien", "Le Minotaure, c'est moi : je garde le site depuis trente ans. Désolé pour le mugissement, j'avais un rhume."),
        ("Max", "Un Minotaure enrhumé... Je ne m'en remettrai pas.", "moyen")], "Le labyrinthe : la sortie",
        leader_anim="salut", extra=(GARDIEN, "droite2", "salut")))
    chemin = os.path.join(ICI, "..", "games", "snake", "scenarios", "labyrinthe.txt")
    with open(chemin, "w") as f:
        f.write("# Généré par tools/gen-labyrinthe.py — ne pas modifier à la main (relancer le script).\n")
        f.write("titre = Le labyrinthe du Minotaure\n")
        f.write("description = Un labyrinthe de salles dans les ruines de Cnossos : retrouvez les touristes égarés, puis ressortez\n\n")
        f.write("\n".join(s))
    print("écrit labyrinthe.txt")
    return 0


if __name__ == "__main__":
    sys.exit(main())
