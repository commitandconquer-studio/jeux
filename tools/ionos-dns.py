#!/usr/bin/env python3
# Commit & Conquer — licence GPL-3.0-or-later.
"""Relie commitandconquer.fr à GitHub Pages dans le DNS d'IONOS.

La clé d'API est lue dans ~/.config/commit-and-conquer/ionos.env (à remplir
soi-même, jamais commitée) :

    IONOS_PREFIXE=...
    IONOS_SECRET=...

    tools/ionos-dns.py                       montre ce qui va changer (rien n'est modifié)
    tools/ionos-dns.py --appliquer           applique les changements
    tools/ionos-dns.py --txt NOM VALEUR      ajoute aussi l'enregistrement TXT
                                             de vérification de GitHub

Seuls les enregistrements A, AAAA et CNAME de commitandconquer.fr et de
www.commitandconquer.fr sont touchés : l'e-mail (MX, TXT…) reste intact.
"""
import json
import os
import sys
import urllib.error
import urllib.request

DOMAINE = "commitandconquer.fr"
WWW = "www." + DOMAINE
GITHUB = "commitandconquer-studio.github.io"
TTL = 3600
API = "https://api.hosting.ionos.com/dns/v1"
ENV = os.environ.get("IONOS_ENV", os.path.expanduser("~/.config/commit-and-conquer/ionos.env"))

# Adresses officielles de GitHub Pages (docs.github.com, « Managing a custom domain »).
VOULUS = (
    [(DOMAINE, "A", f"185.199.{n}.153") for n in (108, 109, 110, 111)]
    + [(DOMAINE, "AAAA", f"2606:50c0:{n}::153") for n in ("8000", "8001", "8002", "8003")]
    + [(WWW, "CNAME", GITHUB)]
)
GERES = {"A", "AAAA", "CNAME"}


def lire_cle():
    if not os.path.exists(ENV):
        sys.exit(f"Fichier introuvable : {ENV}\nCréez-le avec IONOS_PREFIXE=… et IONOS_SECRET=…")
    valeurs = {}
    for ligne in open(ENV, encoding="utf-8"):
        ligne = ligne.strip()
        if ligne and not ligne.startswith("#") and "=" in ligne:
            k, v = ligne.split("=", 1)
            valeurs[k.strip()] = v.strip().strip('"').strip("'")
    prefixe, secret = valeurs.get("IONOS_PREFIXE", ""), valeurs.get("IONOS_SECRET", "")
    if not prefixe or not secret:
        sys.exit(f"IONOS_PREFIXE et IONOS_SECRET doivent être remplis dans {ENV}")
    return f"{prefixe}.{secret}"


def appel(cle, methode, chemin, corps=None):
    data = None if corps is None else json.dumps(corps).encode()
    req = urllib.request.Request(API + chemin, data=data, method=methode)
    req.add_header("X-API-Key", cle)
    req.add_header("Accept", "application/json")
    if data is not None:
        req.add_header("Content-Type", "application/json")
    try:
        with urllib.request.urlopen(req, timeout=30) as r:
            texte = r.read().decode()
            return json.loads(texte) if texte else None
    except urllib.error.HTTPError as e:
        sys.exit(f"Erreur IONOS {e.code} sur {methode} {chemin} : {e.read().decode()[:300]}")


def normal(nom, typ, contenu):
    return (nom.rstrip(".").lower(), typ.upper(), contenu.rstrip(".").lower())


def main():
    args = sys.argv[1:]
    appliquer = "--appliquer" in args
    voulus = list(VOULUS)
    if "--txt" in args:
        i = args.index("--txt")
        nom, valeur = args[i + 1], args[i + 2]
        if not nom.endswith(DOMAINE):
            nom = f"{nom}.{DOMAINE}"
        voulus.append((nom, "TXT", valeur))

    cle = lire_cle()
    zones = appel(cle, "GET", "/zones")
    zone = next((z for z in zones if z["name"] == DOMAINE), None)
    if zone is None:
        sys.exit(f"Zone {DOMAINE} introuvable dans ce compte IONOS.")
    actuels = appel(cle, "GET", f"/zones/{zone['id']}")["records"]

    cibles = {normal(*v) for v in voulus}
    noms_txt = {normal(*v)[0] for v in voulus if v[1] == "TXT"}
    a_supprimer, deja = [], set()
    for r in actuels:
        cle_r = normal(r["name"], r["type"], r["content"])
        if cle_r in cibles:
            deja.add(cle_r)
        elif cle_r[0] in (DOMAINE, WWW) and cle_r[1] in GERES:
            a_supprimer.append(r)
        elif cle_r[1] == "TXT" and cle_r[0] in noms_txt:
            a_supprimer.append(r)
    a_ajouter = [v for v in voulus if normal(*v) not in deja]

    print(f"Zone {DOMAINE}")
    for r in a_supprimer:
        print(f"  - supprimer  {r['type']:5} {r['name']:28} {r['content']}")
    for nom, typ, contenu in a_ajouter:
        print(f"  + ajouter    {typ:5} {nom:28} {contenu}")
    for nom, typ, contenu in sorted(deja):
        print(f"  = déjà bon   {typ:5} {nom:28} {contenu}")
    if not a_supprimer and not a_ajouter:
        print("Rien à changer : le DNS est déjà prêt pour GitHub Pages.")
        return
    if not appliquer:
        print("\nAucune modification faite. Relancez avec --appliquer pour appliquer.")
        return

    for r in a_supprimer:
        appel(cle, "DELETE", f"/zones/{zone['id']}/records/{r['id']}")
    if a_ajouter:
        appel(cle, "POST", f"/zones/{zone['id']}/records", [
            {"name": n, "type": t, "content": c, "ttl": TTL, "prio": 0, "disabled": False}
            for n, t, c in a_ajouter
        ])
    print("\nC'est fait. La propagation prend de quelques minutes à quelques heures.")


if __name__ == "__main__":
    main()
