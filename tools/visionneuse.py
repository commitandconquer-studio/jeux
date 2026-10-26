#!/usr/bin/env python3
# Commit & Conquer — licence GPL-3.0-or-later.
#
# Visionneuse / éditeur 3D des modèles de Traversée (véhicules, animaux, décors, héros,
# trains), sans recompiler le jeu :
#
#   python3 tools/visionneuse.py            puis ouvrir http://localhost:8800
#   python3 tools/visionneuse.py --ouvrir   (ouvre le navigateur)
#
# Il lit games/traversee/data/import/*.json et data/vehicules.json, montre chaque modèle en
# 3D (avec ses animations), permet de modifier les pièces et d'enregistrer dans le JSON.
# La page se recharge d'elle-même quand un fichier change sur le disque.
#
# Attention : les fichiers produits par un script (gen-espace.py, gen-farwest.py…) sont
# réécrits quand on relance ce script ; la visionneuse édite le JSON, qui devient alors la
# référence (ne relancez plus le script de ce monde). data/vehicules.json, lui, est produit
# par gen-vehicules.py : il est en lecture seule ici.
import argparse, json, os, sys, tempfile, threading, webbrowser
from http.server import SimpleHTTPRequestHandler, ThreadingHTTPServer
from urllib.parse import urlparse

RACINE = os.path.abspath(os.path.join(os.path.dirname(os.path.abspath(__file__)), ".."))
DATA = os.path.join(RACINE, "games/traversee/data")
PAGE = os.path.join(os.path.dirname(os.path.abspath(__file__)), "visionneuse")
LECTURE_SEULE = {"vehicules.json"}


def fichiers():
    sortie = []
    imp = os.path.join(DATA, "import")
    for nom in sorted(os.listdir(imp)):
        if nom.endswith(".json"):
            sortie.append(os.path.join("import", nom))
    sortie.append("vehicules.json")
    return sortie


def nom_fr(m):
    n = m.get("noms") or m.get("nom") or {}
    return n.get("fr") if isinstance(n, dict) else str(n)


def modeles_du_fichier(rel):
    chemin = os.path.join(DATA, rel)
    with open(chemin, encoding="utf-8") as f:
        d = json.load(f)
    out = []

    def ajouter(m, type_, chemin_json):
        if not isinstance(m, dict) or "pieces" not in m:
            return
        out.append(dict(
            cle=f"{rel}::" + "/".join(str(c) for c in chemin_json), fichier=rel, chemin=chemin_json, type=type_, id=m.get("id", chemin_json[-1]),
            nom=nom_fr(m) or m.get("id", ""), c2=m.get("couleur_2", "#888888"), c3=m.get("couleur_3", "#888888"),
            pieces=m["pieces"], mobiles=m.get("parties_mobiles", []), longueur=m.get("longueur"), largeur=m.get("largeur"),
            editable=os.path.basename(rel) not in LECTURE_SEULE,
        ))

    for i, v in enumerate(d.get("vehicules", [])):
        ajouter(v, "véhicule", ["vehicules", i])
    for i, a in enumerate(d.get("animaux", [])):
        ajouter(a, "animal", ["animaux", i])
    m = d.get("monde")
    if isinstance(m, dict):
        for cle, type_ in (("obstacles", "obstacle"), ("bords", "bord"), ("deco", "décor")):
            for i, x in enumerate(m.get(cle, [])):
                ajouter(x, type_, ["monde", cle, i])
        for i, x in enumerate(m.get("train", [])):
            ajouter(x, "train", ["monde", "train", i])
    h = d.get("heros")
    if isinstance(h, dict):
        ajouter(h, "héros", ["heros"])
        acc = h.get("accessoire")
        if isinstance(acc, dict) and "pieces" in acc:
            ajouter(dict(acc, id=h.get("id", "heros") + "_accessoire", couleur_2=h.get("couleur_2"), couleur_3=h.get("couleur_3")), "accessoire", ["heros", "accessoire"])
    return out


def tous_les_modeles():
    sortie = []
    for rel in fichiers():
        try:
            sortie += modeles_du_fichier(rel)
        except Exception as e:  # un JSON cassé ne doit pas bloquer les autres
            print("Lecture impossible :", rel, e, file=sys.stderr)
    return sortie


def sauver(corps):
    rel = corps["fichier"]
    if os.path.basename(rel) in LECTURE_SEULE or rel not in fichiers():
        raise ValueError("fichier non modifiable")
    chemin = os.path.join(DATA, rel)
    with open(chemin, encoding="utf-8") as f:
        d = json.load(f)
    cible = d
    for c in corps["chemin"]:
        cible = cible[c]
    m = corps["modele"]
    cible["pieces"] = m["pieces"]
    if "mobiles" in m:
        if cible.get("parties_mobiles") is not None or m["mobiles"]:
            cible["parties_mobiles"] = m["mobiles"]
    for src, dst in (("c2", "couleur_2"), ("c3", "couleur_3")):
        if m.get(src):
            cible[dst] = m[src]
    tmp = tempfile.NamedTemporaryFile("w", delete=False, dir=os.path.dirname(chemin), encoding="utf-8", suffix=".tmp")
    json.dump(d, tmp, ensure_ascii=False, indent=1)
    tmp.close()
    os.replace(tmp.name, chemin)


class Gestionnaire(SimpleHTTPRequestHandler):
    def __init__(self, *a, **k):
        super().__init__(*a, directory=PAGE, **k)

    def log_message(self, fmt, *args):
        if "/api/mtimes" not in (args[0] if args else ""):
            super().log_message(fmt, *args)

    def reponse_json(self, objet, code=200):
        corps = json.dumps(objet, ensure_ascii=False).encode("utf-8")
        self.send_response(code)
        self.send_header("Content-Type", "application/json; charset=utf-8")
        self.send_header("Content-Length", str(len(corps)))
        self.send_header("Cache-Control", "no-store")
        self.end_headers()
        self.wfile.write(corps)

    def do_GET(self):
        chemin = urlparse(self.path).path
        if chemin == "/api/modeles":
            return self.reponse_json(tous_les_modeles())
        if chemin == "/api/mtimes":
            return self.reponse_json({rel: os.path.getmtime(os.path.join(DATA, rel)) for rel in fichiers()})
        self.send_header_no_cache = True
        return super().do_GET()

    def end_headers(self):
        self.send_header("Cache-Control", "no-store")
        super().end_headers()

    def do_POST(self):
        if urlparse(self.path).path != "/api/sauver":
            return self.reponse_json({"erreur": "inconnu"}, 404)
        try:
            n = int(self.headers.get("Content-Length", "0"))
            sauver(json.loads(self.rfile.read(n).decode("utf-8")))
            return self.reponse_json({"ok": True})
        except Exception as e:
            return self.reponse_json({"erreur": str(e)}, 400)


def main():
    p = argparse.ArgumentParser(description="Visionneuse 3D des modèles de Traversée")
    p.add_argument("--port", type=int, default=8800)
    p.add_argument("--ouvrir", action="store_true", help="ouvre le navigateur")
    a = p.parse_args()
    serveur = None
    for port in range(a.port, a.port + 10):  # si le port est pris (une autre visionneuse ?), on essaie le suivant
        try:
            serveur = ThreadingHTTPServer(("127.0.0.1", port), Gestionnaire)
            break
        except OSError:
            continue
    if serveur is None:
        sys.exit(f"Aucun port libre entre {a.port} et {a.port + 9}.")
    url = f"http://localhost:{serveur.server_address[1]}/"
    print(f"Visionneuse : {url}  (Ctrl+C pour arrêter)")
    if a.ouvrir:
        threading.Timer(0.5, lambda: webbrowser.open(url)).start()
    try:
        serveur.serve_forever()
    except KeyboardInterrupt:
        pass


if __name__ == "__main__":
    main()
