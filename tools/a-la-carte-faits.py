#!/usr/bin/env python3
# Commit & Conquer — licence GPL-3.0-or-later.
#
# Faits sur les pays pour le mode « Pays caché » de « À la carte ! » : superficie
# (calculée sur la sphère à partir des contours), population estimée et PIB (millions
# de dollars) tirés de Natural Earth (ne_50m_admin_0_countries, domaine public, chiffres 2018-2019).
#
#   python3 tools/a-la-carte-faits.py [fichier.geojson]
import json, math, sys, urllib.request
from pathlib import Path

BASE = Path(__file__).resolve().parent.parent / "site/static/jeux-html/a-la-carte/data"
URL = "https://raw.githubusercontent.com/nvkelso/natural-earth-vector/master/geojson/ne_50m_admin_0_countries.geojson"
R = 6371.0088


def aire_anneau(anneau):
    """Aire (km²) d'un anneau longitude/latitude sur la sphère (formule de Chamberlain et Duquette)."""
    s = 0.0
    for (x1, y1), (x2, y2) in zip(anneau, anneau[1:] + anneau[:1]):
        s += math.radians(x2 - x1) * (2 + math.sin(math.radians(y1)) + math.sin(math.radians(y2)))
    return abs(s) * R * R / 2


def aire(geometrie):
    polys = geometrie["coordinates"] if geometrie["type"] == "MultiPolygon" else [geometrie["coordinates"]]
    return sum(aire_anneau(p[0]) - sum(aire_anneau(t) for t in p[1:]) for p in polys)


donnees = json.load(open(sys.argv[1], encoding="utf-8")) if len(sys.argv) > 1 else json.loads(urllib.request.urlopen(URL, timeout=120).read())
pays = {p["code"] for p in json.loads((BASE / "pays.json").read_text(encoding="utf-8"))}
faits = {}
for f in donnees["features"]:
    pr = f["properties"]
    code = (pr.get("ISO_A2_EH") or "").lower()
    if code not in pays:
        continue
    e = faits.setdefault(code, {"s": 0, "p": 0, "g": 0})
    # plusieurs entités peuvent partager un code (îles rattachées) : on garde les valeurs les plus grandes
    e["s"] = max(e["s"], round(aire(f["geometry"])))
    e["p"] = max(e["p"], int(pr.get("POP_EST") or 0))
    e["g"] = max(e["g"], int(pr.get("GDP_MD") or 0))
sans_pib = sorted(c for c, e in faits.items() if e["g"] <= 0)
(BASE / "faits.json").write_text(json.dumps(faits, separators=(",", ":")), encoding="utf-8")
print(len(faits), "pays ; sans PIB :", sans_pib)
for c in ("fr", "us", "ru", "mc", "au"):
    print(c, faits[c])
