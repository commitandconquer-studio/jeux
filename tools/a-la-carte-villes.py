#!/usr/bin/env python3
# Commit & Conquer — licence GPL-3.0-or-later.
#
# Construit la liste des préfectures et sous-préfectures de France pour
# « À la carte ! » :
#   - INSEE, code officiel géographique 2025 (départements, arrondissements) :
#     licence ouverte Etalab ;
#   - geo.api.gouv.fr (nom et centre de la commune) : licence ouverte Etalab.
#
#   python3 tools/a-la-carte-villes.py
import csv, io, json, time, urllib.request
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
SORTIE = ROOT / "site/static/jeux-html/a-la-carte/data/villes-france.json"
INSEE = "https://www.insee.fr/fr/statistiques/fichier/8377162/"


def get(url):
    req = urllib.request.Request(url, headers={"User-Agent": "Mozilla/5.0 (a-la-carte data build)"})
    for essai in range(4):
        try:
            with urllib.request.urlopen(req, timeout=60) as r:
                return r.read()
        except Exception:
            if essai == 3:
                raise
            time.sleep(2 * (essai + 1))


def csv_insee(nom):
    return list(csv.DictReader(io.StringIO(get(INSEE + nom).decode("utf-8"))))


deps = csv_insee("v_departement_2025.csv")
arrs = csv_insee("v_arrondissement_2025.csv")
nom_dep = {d["DEP"]: d["LIBELLE"] for d in deps}
prefs = {d["CHEFLIEU"]: d["DEP"] for d in deps}
subs = {}
for a in arrs:
    if a["CHEFLIEU"] not in prefs:
        subs[a["CHEFLIEU"]] = a["DEP"]

villes = []
for codes, type_ in ((prefs, "p"), (subs, "s")):
    for code, dep in sorted(codes.items()):
        j = json.loads(get(f"https://geo.api.gouv.fr/communes/{code}?fields=nom,centre,population"))
        lon, lat = j["centre"]["coordinates"]
        villes.append({
            "n": j["nom"], "c": code, "t": type_,
            "d": nom_dep.get(dep, dep),
            "lat": round(lat, 4), "lon": round(lon, 4),
            "pop": j.get("population", 0),
        })
        time.sleep(0.05)

SORTIE.parent.mkdir(parents=True, exist_ok=True)
SORTIE.write_text(json.dumps(villes, ensure_ascii=False, separators=(",", ":")), encoding="utf-8")
print(len(villes), "villes,",
      sum(v["t"] == "p" for v in villes), "préfectures,",
      sum(v["t"] == "s" for v in villes), "sous-préfectures →", SORTIE)
