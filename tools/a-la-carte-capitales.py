#!/usr/bin/env python3
# Commit & Conquer — licence GPL-3.0-or-later.
#
# Capitales du monde de « À la carte ! » : nom français (celui de pays.json),
# coordonnées et notoriété (nombre de pages Wikipédia) depuis Wikidata (CC0).
#
#   python3 tools/a-la-carte-capitales.py
import csv, io, json, re, time, urllib.error, urllib.parse, urllib.request
from pathlib import Path

BASE = Path(__file__).resolve().parent.parent / "site/static/jeux-html/a-la-carte/data"
REQ = """SELECT ?code ?capNom ?coord ?sl WHERE {
  ?p wdt:P31 wd:Q3624078; wdt:P297 ?code; wdt:P36 ?cap.
  FILTER NOT EXISTS { ?p wdt:P576 ?fin }
  ?cap wdt:P625 ?coord; wikibase:sitelinks ?sl.
  ?cap rdfs:label ?capNom FILTER(LANG(?capNom)="fr")
}"""


def get(url):
    req = urllib.request.Request(url, headers={"Accept": "text/csv", "User-Agent": "a-la-carte-data/1.0 (jeu libre GPL-3.0, commitandconquer.fr)"})
    for essai in range(4):
        try:
            with urllib.request.urlopen(req, timeout=90) as r:
                return r.read().decode("utf-8")
        except urllib.error.HTTPError as e:
            if essai == 3:
                raise
            time.sleep(70 if e.code == 429 else 3)  # Wikidata limite le débit
        except Exception:
            if essai == 3:
                raise
            time.sleep(3)


pays = {p["code"]: p for p in json.loads((BASE / "pays.json").read_text(encoding="utf-8"))}
lignes = list(csv.DictReader(io.StringIO(get("https://query.wikidata.org/sparql?" + urllib.parse.urlencode({"query": REQ})))))
trouvees = {}
for r in lignes:
    code = r["code"].lower()
    p = pays.get(code)
    if not p or not p["capitale"] or r["capNom"] != p["capitale"]:
        continue
    m = re.match(r"Point\(([-\d.]+) ([-\d.]+)\)", r["coord"])
    if not m:
        continue
    trouvees[code] = {"n": p["capitale"], "c": code, "t": "c", "d": p["nom"],
                      "lat": round(float(m.group(2)), 3), "lon": round(float(m.group(1)), 3),
                      "pop": max(int(r["sl"]), p["sl"])}  # notoriété : la plus haute du pays ou de la capitale
# Capitales sans coordonnées exploitables dans Wikidata (étiquette différente, ou pas de capitale déclarée).
MANUELLES = {"no": (59.913, 10.752, 300), "lk": (6.9, 79.95, 70)}
for code, (lat, lon, sl) in MANUELLES.items():
    p = pays.get(code)
    if p and p["capitale"] and code not in trouvees:
        trouvees[code] = {"n": p["capitale"], "c": code, "t": "c", "d": p["nom"], "lat": lat, "lon": lon, "pop": max(sl, p["sl"])}
manquantes = sorted(c for c, p in pays.items() if p["capitale"] and c not in trouvees)
liste = sorted(trouvees.values(), key=lambda v: v["n"])
(BASE / "capitales.json").write_text(json.dumps(liste, ensure_ascii=False, separators=(",", ":")), encoding="utf-8")
print(len(liste), "capitales ; sans coordonnées :", manquantes)
