#!/usr/bin/env python3
# Commit & Conquer — licence GPL-3.0-or-later.
#
# Construit la liste des pays et les drapeaux de « À la carte ! » :
#   - Wikidata (CC0) : États souverains, noms français, capitales, notoriété
#     (nombre de pages dans les autres langues de Wikipédia) ;
#   - dépôt flag-icons 7.2.3 (licence MIT, © Panayiotis Lipiridis) : drapeaux
#     SVG au format 4:3, copiés avec leur licence.
#
#   python3 tools/a-la-carte-pays.py
import csv, io, json, time, urllib.parse, urllib.request
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
BASE = ROOT / "site/static/jeux-html/a-la-carte"
FI = "https://cdn.jsdelivr.net/npm/flag-icons@7.2.3/"

# Noms usuels (Wikidata donne parfois le nom officiel) et capitales à trancher.
NOMS = {
    "NL": "Pays-Bas", "CN": "Chine", "CG": "Congo", "PS": "Palestine",
    "FM": "Micronésie", "VA": "Vatican",
}
CAPITALES = {
    "NO": "Oslo", "ZA": "Pretoria", "BO": "Sucre", "LK": "Sri Jayawardenapura Kotte",
    "SZ": "Mbabane", "YE": "Sanaa", "PS": None,  # capitale contestée : non demandée
}

REQ = """SELECT ?code ?nom ?capNom ?sl WHERE {
  ?p wdt:P31 wd:Q3624078; wdt:P297 ?code; wikibase:sitelinks ?sl.
  FILTER NOT EXISTS { ?p wdt:P576 ?fin }
  ?p rdfs:label ?nom FILTER(LANG(?nom)="fr")
  OPTIONAL { ?p wdt:P36 ?cap. ?cap rdfs:label ?capNom FILTER(LANG(?capNom)="fr") }
}"""


def get(url, entetes=None):
    h = {"User-Agent": "a-la-carte-data/1.0 (jeu libre GPL-3.0, commitandconquer.fr)"}
    h.update(entetes or {})
    req = urllib.request.Request(url, headers=h)
    for essai in range(4):
        try:
            with urllib.request.urlopen(req, timeout=60) as r:
                return r.read()
        except urllib.error.HTTPError as e:
            if essai == 3:
                raise
            time.sleep(70 if e.code == 429 else 2 * (essai + 1))  # Wikidata limite le débit
        except Exception:
            if essai == 3:
                raise
            time.sleep(2 * (essai + 1))


wd = get("https://query.wikidata.org/sparql?" + urllib.parse.urlencode({"query": REQ}),
         {"Accept": "text/csv"}).decode("utf-8")
fi = {c["code"]: c for c in json.loads(get(FI + "country.json"))}

pays = {}
for r in csv.DictReader(io.StringIO(wd)):
    code = r["code"]
    if code.lower() not in fi:
        continue
    p = pays.setdefault(code, {"code": code.lower(), "nom": NOMS.get(code, r["nom"]),
                               "capitale": r["capNom"] or None, "sl": 0})
    p["sl"] = max(p["sl"], int(r["sl"]))
    if not p["capitale"] and r["capNom"]:
        p["capitale"] = r["capNom"]
for code, cap in CAPITALES.items():
    if code in pays:
        pays[code]["capitale"] = cap

pays["NL"]["sl"] = 330  # l'entité « Royaume des Pays-Bas » a peu de pages : on prend la notoriété des Pays-Bas
liste = sorted(pays.values(), key=lambda p: p["nom"])
for p in liste:
    p["continent"] = fi[p["code"]]["continent"]

dossier = BASE / "data/drapeaux"
dossier.mkdir(parents=True, exist_ok=True)
for p in liste:
    (dossier / f"{p['code']}.svg").write_bytes(get(FI + f"flags/4x3/{p['code']}.svg"))
(dossier / "LICENSE-flag-icons.txt").write_bytes(get(FI + "LICENSE"))

(BASE / "data/pays.json").write_text(
    json.dumps(liste, ensure_ascii=False, separators=(",", ":")), encoding="utf-8")
taille = sum(f.stat().st_size for f in dossier.glob("*.svg"))
print(len(liste), "pays,", round(taille / 1024), "Ko de drapeaux")
