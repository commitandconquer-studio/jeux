#!/usr/bin/env python3
# Commit & Conquer — licence GPL-3.0-or-later.
#
# Relève les couleurs « de couleur » (ni blanc, ni noir, ni gris) de chaque
# drapeau SVG de « À la carte ! », regroupées quand elles sont presque
# identiques, de la plus fréquente à la moins fréquente. Aucun accès réseau.
#
#   python3 tools/a-la-carte-couleurs.py
import colorsys, collections, json, re
from pathlib import Path

BASE = Path(__file__).resolve().parent.parent / "site/static/jeux-html/a-la-carte/data"
NOMMEES = {"red": "#ff0000", "gold": "#ffd700", "gray": "#808080", "olive": "#808000", "purple": "#800080"}
HEX = re.compile(r"#[0-9a-fA-F]{6}\b|#[0-9a-fA-F]{3}\b")
SEUIL_GROUPE = 28   # distance RVB en dessous de laquelle deux couleurs sont regroupées


def normaliser(h):
    h = h.lower()
    return "#" + "".join(c * 2 for c in h[1:]) if len(h) == 4 else h


def rvb(h):
    return tuple(int(h[i:i + 2], 16) for i in (1, 3, 5))


def coloree(h):
    r, g, b = (v / 255 for v in rvb(h))
    _, l, s = colorsys.rgb_to_hls(r, g, b)
    return s > 0.18 and 0.08 < l < 0.93


def dist(a, b):
    return sum((x - y) ** 2 for x, y in zip(rvb(a), rvb(b))) ** 0.5


resultat = {}
for f in sorted((BASE / "drapeaux").glob("*.svg")):
    t = f.read_text(encoding="utf-8")
    t = re.sub(r'(?<=[="\s:])(%s)(?=["\s;])' % "|".join(NOMMEES), lambda m: NOMMEES[m.group(1)], t)
    n = collections.Counter(normaliser(h) for h in HEX.findall(t))
    groupes = []
    for h, c in n.most_common():
        if not coloree(h):
            continue
        for g in groupes:
            if dist(g["rep"], h) < SEUIL_GROUPE:
                g["couleurs"].append(h); g["n"] += c
                break
        else:
            groupes.append({"rep": h, "couleurs": [h], "n": c})
    groupes.sort(key=lambda g: -g["n"])
    resultat[f.stem] = [g["couleurs"] for g in groupes]

(BASE / "couleurs.json").write_text(json.dumps(resultat, separators=(",", ":")), encoding="utf-8")
c = collections.Counter(len(v) for v in resultat.values())
print(len(resultat), "drapeaux ; groupes de couleurs :", sorted(c.items()))
