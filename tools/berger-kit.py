#!/usr/bin/env python3
"""Le Chien de berger : transforme les rendus du dossier de presse et du kit en fichiers du site.

  BERGER_KIT=/tmp/kit    cargo run -p berger     # chaque animal sur fond magenta
  BERGER_PRESSE=/tmp/pr  cargo run -p berger     # captures 1920x1080 sans interface
  python3 tools/berger-kit.py /tmp/kit /tmp/pr

Écrit dans site/static/img/jeux/berger/ : les .jpg de presse et kit/*.png transparents
(plus la ligne des six chiens, 09-six-chiens.jpg).
"""
import glob
import os
import sys

from PIL import Image

RACINE = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "site", "static", "img", "jeux", "berger")


def detourer(chemin, sortie, haut=560):
    im = Image.open(chemin).convert("RGB")
    px = im.load()
    w, h = im.size
    out = Image.new("RGBA", (w, h))
    po = out.load()
    for y in range(h):
        for x in range(w):
            r, g, b = px[x, y]
            m = max(0.0, (min(r, b) - g) / 255.0)  # « magenta-ité » du pixel
            a = 1.0 - min(1.0, m * 1.15)
            if a < 0.02:
                po[x, y] = (0, 0, 0, 0)
                continue
            # On retire le magenta qui déborde sur les bords adoucis.
            d = lambda c, k: max(0, min(255, int(round((c - k * (1 - a)) / a))))
            po[x, y] = (d(r, 255), d(g, 0), d(b, 255), int(a * 255))
    boite = out.getbbox()
    out = out.crop((max(0, boite[0] - 6), max(0, boite[1] - 6), min(w, boite[2] + 6), min(h, boite[3] + 6)))
    if out.height > haut:
        out = out.resize((round(out.width * haut / out.height), haut), Image.LANCZOS)
    out.save(sortie)
    return out


def main(kit, presse):
    os.makedirs(os.path.join(RACINE, "kit"), exist_ok=True)
    chiens = []
    if kit:
        for f in sorted(glob.glob(os.path.join(kit, "*.png"))):
            nom = os.path.basename(f)
            im = detourer(f, os.path.join(RACINE, "kit", nom))
            if nom.startswith("chien-"):
                chiens.append((nom, im))
    if presse:
        for f in sorted(glob.glob(os.path.join(presse, "*.png"))):
            nom = os.path.splitext(os.path.basename(f))[0] + ".jpg"
            Image.open(f).convert("RGB").save(os.path.join(RACINE, nom), quality=88, optimize=True)
    if chiens:
        # Les six chiens côte à côte, sur le vert du pré.
        ordre = ["chien-border", "chien-caramel", "chien-neige", "chien-ardoise", "chien-braise", "chien-dore"]
        ims = [im for o in ordre for n, im in chiens if n == o + ".png"]
        haut = max(i.height for i in ims)
        larg = sum(i.width for i in ims) + 40 * (len(ims) + 1)
        toile = Image.new("RGB", (larg, haut + 120), (159, 196, 114))
        x = 40
        for i in ims:
            toile.paste(i, (x, 60 + haut - i.height), i)
            x += i.width + 40
        toile.save(os.path.join(RACINE, "09-six-chiens.jpg"), quality=90)


if __name__ == "__main__":
    main(sys.argv[1] if len(sys.argv) > 1 else None, sys.argv[2] if len(sys.argv) > 2 else None)
