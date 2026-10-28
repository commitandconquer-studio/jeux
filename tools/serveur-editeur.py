#!/usr/bin/env python3
"""Sert l'éditeur de niveaux avec l'aperçu 3D, sans Zola, pour travailler hors ligne.

    tools/build-web.sh snake        # une fois (et après chaque changement du jeu)
    tools/serveur-editeur.py        # puis ouvrir http://127.0.0.1:8765/editeur/

Pour que d'autres ordinateurs du réseau local y accèdent aussi : tools/serveur-editeur.py 8765 --reseau
(il faut peut-être autoriser le port dans le pare-feu ; n'importe qui sur le réseau peut alors ouvrir la page).

Le site met le jeu dans une page Zola (`/jouer/snake/`) ; ici une page minimale la remplace.
"""
import http.server
import os
import socketserver
import sys

RACINE = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "site", "static")
PAGE = b"""<!doctype html><html><head><meta charset="utf-8"><style>html,body{margin:0;height:100%;background:#000}canvas{width:100%;height:100%;display:block}</style></head>
<body><canvas id="jeu" tabindex="0"></canvas>
<script type="module">import init from "/jouer/snake/snake.js"; init().catch(e => { if (!String(e).includes("Using exceptions for control flow")) console.error(e); });</script></body></html>"""


class Gestionnaire(http.server.SimpleHTTPRequestHandler):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, directory=RACINE, **kwargs)

    def do_GET(self):
        chemin = self.path.split("?")[0]
        if chemin.startswith("/jouer/snake/") and not chemin.endswith((".js", ".wasm")):
            self.send_response(200)
            self.send_header("Content-Type", "text/html; charset=utf-8")
            self.send_header("Content-Length", str(len(PAGE)))
            self.end_headers()
            self.wfile.write(PAGE)
            return
        return super().do_GET()

    def end_headers(self):
        self.send_header("Cache-Control", "no-store")
        super().end_headers()


if __name__ == "__main__":
    args = [a for a in sys.argv[1:] if not a.startswith("--")]
    reseau = "--reseau" in sys.argv
    port = int(args[0]) if args else 8765
    if not os.path.exists(os.path.join(RACINE, "jouer", "snake", "snake_bg.wasm")):
        sys.exit("Le jeu n'est pas compilé pour le navigateur : lancez d'abord tools/build-web.sh snake")
    socketserver.TCPServer.allow_reuse_address = True
    try:
        serveur = socketserver.TCPServer(("0.0.0.0" if reseau else "127.0.0.1", port), Gestionnaire)
    except OSError:
        sys.exit(f"Le port {port} est déjà utilisé (le serveur tourne peut-être déjà : ouvrez http://127.0.0.1:{port}/editeur/, ou essayez un autre port : tools/serveur-editeur.py 8800)")
    with serveur:
        print(f"Éditeur : http://127.0.0.1:{port}/editeur/  (Ctrl+C pour arrêter)")
        if reseau:
            print("Ouvert au réseau local : les autres ordinateurs utilisent l'adresse IP de celui-ci, par exemple http://<adresse-ip>:%d/editeur/" % port)
        serveur.serve_forever()
