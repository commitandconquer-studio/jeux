#!/usr/bin/env python3
# Commit & Conquer — licence GPL-3.0-or-later.
#
# Fonds de carte de « À la carte ! » (France et monde) à partir de Natural Earth
# (ne_50m_admin_0_countries, domaine public), simplifiés. Les coordonnées sont
# gardées en degrés (longitude, latitude) ; le jeu les projette (équirectangulaire).
#
#   python3 tools/a-la-carte-cartes.py [fichier.geojson]
import json, math, sys, urllib.request
from pathlib import Path

BASE = Path(__file__).resolve().parent.parent / "site/static/jeux-html/a-la-carte/data"
URL = "https://raw.githubusercontent.com/nvkelso/natural-earth-vector/master/geojson/ne_50m_admin_0_countries.geojson"
FRANCE = (-9.0, 40.0, 12.0, 52.5)   # lon min, lat min, lon max, lat max


def dp(points, eps):
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
    m = len(pts) // 2
    res = dp(pts[: m + 1], eps)[:-1] + dp(pts[m:] + [pts[0]], eps)[:-1]
    return res if len(res) >= 3 else None


def polygones(feature):
    g = feature["geometry"]
    return g["coordinates"] if g["type"] == "MultiPolygon" else [g["coordinates"]]


def bbox(anneau):
    xs = [p[0] for p in anneau]; ys = [p[1] for p in anneau]
    return min(xs), min(ys), max(xs), max(ys)


def dans(b, region):
    return not (b[2] < region[0] or b[0] > region[2] or b[3] < region[1] or b[1] > region[3])


def chemin(anneaux, eps, dec):
    d = []
    for a in anneaux:
        s = simplifier([(x, y) for x, y in a], eps)
        if s:
            d.append("M" + "L".join(f"{x:.{dec}f} {y:.{dec}f}" for x, y in s) + "Z")
    return "".join(d)


def main():
    donnees = json.load(open(sys.argv[1], encoding="utf-8")) if len(sys.argv) > 1 else json.loads(urllib.request.urlopen(URL, timeout=120).read())
    monde, france = [], []
    for f in donnees["features"]:
        code = (f["properties"].get("ISO_A2_EH") or "").lower()
        for p in polygones(f):
            ext = p[0]
            b = bbox(ext)
            if b[3] > -58:                         # pas l'Antarctique
                monde.append(ext)
            if dans(b, FRANCE):
                cx, cy = (b[0] + b[2]) / 2, (b[1] + b[3]) / 2
                if code == "fr" and not (FRANCE[0] < cx < FRANCE[2] and FRANCE[1] < cy < FRANCE[3]):
                    continue                       # l'outre-mer n'est pas sur cette carte
                france.append(ext)
    for nom, anneaux, eps, dec, region in (("monde", monde, 0.12, 1, None), ("france", france, 0.012, 2, FRANCE)):
        d = chemin(anneaux, eps, dec)
        (BASE / f"carte-{nom}.json").write_text(json.dumps({"d": d, "zone": region}, separators=(",", ":")), encoding="utf-8")
        print(nom, len(anneaux), "contours,", len(d) // 1024, "Ko")


main()
