#!/usr/bin/env python3
# Commit & Conquer — licence GPL-3.0-or-later.
#
# Construit les silhouettes de pays de « À la carte ! » à partir de Natural
# Earth (ne_50m_admin_0_countries, domaine public), simplifiées et mises à
# l'échelle dans un carré de 100 x 100 (projection équirectangulaire locale).
#
#   python3 tools/a-la-carte-formes.py [fichier.geojson]
# Sans argument, le fichier est téléchargé depuis le dépôt natural-earth-vector.
import json, math, sys, time, urllib.request
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
BASE = ROOT / "site/static/jeux-html/a-la-carte"
URL = "https://raw.githubusercontent.com/nvkelso/natural-earth-vector/master/geojson/ne_50m_admin_0_countries.geojson"

# Distance maximale (en degrés) entre une partie du pays et sa plus grande
# partie ; au-delà, on la laisse de côté (îles lointaines, outre-mer…).
DIST_MAX = {"default": 30, "no": 9, "ec": 6, "es": 9, "pt": 5, "dk": 12, "nl": 6, "cl": 30}
SURFACE_MIN_PARTIE = 0.005   # une partie doit faire au moins 0,5 % de l'ensemble
SURFACE_MIN_PAYS = 0.45      # sous ce seuil (degrés carrés réels), la silhouette est trop petite
EXCLURE = {"ps"}            # silhouette trop discutable pour un jeu
TOLERANCE = 0.45             # simplification de Douglas-Peucker, dans le carré de 100


def aire(anneau):
    s = 0.0
    for (x1, y1), (x2, y2) in zip(anneau, anneau[1:] + anneau[:1]):
        s += x1 * y2 - x2 * y1
    return s / 2


def centre(anneau):
    return (sum(p[0] for p in anneau) / len(anneau), sum(p[1] for p in anneau) / len(anneau))


def dp(points, eps):
    """Douglas-Peucker sur une ligne ouverte."""
    if len(points) < 3:
        return points
    (x1, y1), (x2, y2) = points[0], points[-1]
    dx, dy = x2 - x1, y2 - y1
    n = math.hypot(dx, dy) or 1e-12
    dmax, imax = 0.0, 0
    for i in range(1, len(points) - 1):
        d = abs(dy * points[i][0] - dx * points[i][1] + x2 * y1 - y2 * x1) / n
        if d > dmax:
            dmax, imax = d, i
    if dmax <= eps:
        return [points[0], points[-1]]
    return dp(points[: imax + 1], eps)[:-1] + dp(points[imax:], eps)


def simplifier(anneau, eps):
    pts = anneau[:-1] if anneau[0] == anneau[-1] else anneau[:]
    # on coupe l'anneau en deux pour que DP ait des extrémités stables
    m = len(pts) // 2
    haut = dp(pts[: m + 1], eps)
    bas = dp(pts[m:] + [pts[0]], eps)
    res = haut[:-1] + bas[:-1]
    return res if len(res) >= 3 else None


def construire(feature, code):
    g = feature["geometry"]
    polys = g["coordinates"] if g["type"] == "MultiPolygon" else [g["coordinates"]]
    parties = []
    for p in polys:
        ext = [(x, y) for x, y in p[0]]
        trous = [[(x, y) for x, y in t] for t in p[1:]]
        parties.append({"ext": ext, "trous": trous, "aire": abs(aire(ext))})
    parties.sort(key=lambda p: -p["aire"])
    principal = parties[0]
    c0 = centre(principal["ext"])
    total = sum(p["aire"] for p in parties)
    gardees = []
    for p in parties:
        c = centre(p["ext"])
        dlon = (c[0] - c0[0] + 180) % 360 - 180
        dist = math.hypot(dlon * math.cos(math.radians(c0[1])), c[1] - c0[1])
        if p is not principal and (p["aire"] / total < SURFACE_MIN_PARTIE
                                   or dist > DIST_MAX.get(code, DIST_MAX["default"])):
            continue
        decal = c0[0] + dlon - c[0]            # ramène la partie près de la principale
        gardees.append({
            "ext": [(x + decal, y) for x, y in p["ext"]],
            "trous": [[(x + decal, y) for x, y in t] for t in p["trous"] if abs(aire(t)) > 0.5],
        })
    return gardees


def en_chemin(gardees):
    lats = [y for p in gardees for x, y in p["ext"]]
    lat0 = (min(lats) + max(lats)) / 2
    k = math.cos(math.radians(lat0))
    projet = lambda a: [(x * k, -y) for x, y in a]
    anneaux = [(projet(p["ext"]), True) for p in gardees] + [(projet(t), False) for p in gardees for t in p["trous"]]
    surface = sum(abs(aire(a)) * (1 if ext else -1) for a, ext in anneaux)   # degrés carrés réels
    xs = [x for a, _ in anneaux if _ for x, y in a]
    ys = [y for a, _ in anneaux if _ for x, y in a]
    w, h = max(xs) - min(xs), max(ys) - min(ys)
    echelle = 100 / max(w, h)
    ox, oy = (100 - w * echelle) / 2 - min(xs) * echelle, (100 - h * echelle) / 2 - min(ys) * echelle
    d = []
    for a, _ext in anneaux:
        a = [(x * echelle + ox, y * echelle + oy) for x, y in a]
        s = simplifier(a, TOLERANCE)
        if not s:
            continue
        d.append("M" + "L".join(f"{x:.1f} {y:.1f}" for x, y in s) + "Z")
    return "".join(d), surface


def main():
    if len(sys.argv) > 1:
        donnees = json.load(open(sys.argv[1], encoding="utf-8"))
    else:
        donnees = json.loads(urllib.request.urlopen(URL, timeout=120).read())
    pays = {p["code"] for p in json.load(open(BASE / "data/pays.json", encoding="utf-8"))}
    formes, absents = {}, set(pays)
    for f in donnees["features"]:
        code = (f["properties"].get("ISO_A2_EH") or "").lower()
        if code not in pays or code in EXCLURE:
            continue
        absents.discard(code)
        gardees = construire(f, code)
        d, surface = en_chemin(gardees)
        formes[code] = {"d": d, "s": round(surface, 1)}
    sortie = BASE / "data/formes.json"
    sortie.write_text(json.dumps(formes, separators=(",", ":")), encoding="utf-8")
    petits = [c for c, v in formes.items() if v["s"] < SURFACE_MIN_PAYS]
    print(len(formes), "silhouettes,", sortie.stat().st_size // 1024, "Ko ; sans silhouette :", sorted(absents),
          "; trop petits :", len(petits))


main()
