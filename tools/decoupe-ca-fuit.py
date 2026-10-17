#!/usr/bin/env python3
# Commit & Conquer — licence GPL-3.0-or-later.
#
# Découpe les planches dessinées pour « Ça fuit ! » (docs/jeux/da/ca-fuit/,
# éléments isolés sur fond blanc) en sprites à fond transparent :
#
#   tools/decoupe-ca-fuit.py reperer <planche.png> <aperçu.png>
#       repère les éléments et écrit un aperçu numéroté, pour les nommer ;
#   tools/decoupe-ca-fuit.py                                         (tout)
#       découpe selon la table NOMS ci-dessous -> assets/sprites/ca-fuit/.
#
# Une planche à fond transparent est prise telle quelle ; une planche à fond
# blanc est détourée par remplissage depuis les bords de l'image (les blancs
# intérieurs, pétales et yeux, restent opaques, et l'ombre grise posée au sol
# devient une ombre à demi transparente).
import os
import sys
from collections import deque

from PIL import Image, ImageDraw

RACINE = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
DA = os.path.join(RACINE, "docs", "jeux", "da", "ca-fuit")
SORTIE = os.path.join(RACINE, "assets", "sprites", "ca-fuit")

# planche -> [(nom du sprite, largeur en cases, élément : son numéro dans
# l'aperçu, ou sa boîte (x0, y0, x1, y1) quand il est en plusieurs morceaux)]
NOMS = {
    "decor-2.png": [
        ("touffe-1", 0.9, 1), ("touffe-2", 0.9, 2), ("touffe-3", 0.9, 3), ("touffe-4", 0.9, 4),
        ("fleur-1", 0.8, 5), ("fleur-2", 0.8, 6), ("fleur-3", 0.9, 7), ("fleur-4", 0.8, 8),
        ("caillou-1", 1.0, 9), ("caillou-2", 1.0, 10), ("caillou-3", 0.9, 11),
        ("buisson-1", 1.8, 12), ("buisson-2", 1.8, 13),
        ("roseaux", 1.3, 14),
        ("nenuphar-1", 1.0, 15), ("nenuphar-2", 1.0, 16),
        ("flaque-1", 1.0, 20), ("flaque-2", 1.0, 21), ("flaque-3", 1.0, 22),
        ("eclaboussure", 1.5, (1176, 740, 1444, 1060)),
    ],
    "crapaud.png": [("crapaud", 1.5, 2), ("caisse", 1.0, 4)],
    "pictos.png": [("picto-cle", 0.5, 1), ("picto-tuyau", 0.5, 2), ("picto-goutte", 0.5, 3), ("picto-trophee", 0.5, 4)],
}


def detourer(im):
    """Alpha du fond : 0 pour le blanc relié aux bords, un voile pour le gris
    de l'ombre au sol, 255 ailleurs. Une planche déjà transparente est
    prise telle quelle."""
    if im.mode == "RGBA" and im.getextrema()[3][0] == 0:
        w, h = im.size
        a = im.getchannel("A").load()
        fond = bytearray(w * h)
        for y in range(h):
            for x in range(w):
                if a[x, y] < 16:
                    fond[y * w + x] = 1
        return im, fond
    im = im.convert("RGB")
    w, h = im.size
    px = im.load()
    fond = bytearray(w * h)  # 1 = relié au fond
    def clair(x, y):
        r, g, b = px[x, y]
        m = min(r, g, b)
        return m > 150 and max(r, g, b) - m < 40
    file = deque()
    for x in range(w):
        for y in (0, h - 1):
            if clair(x, y) and not fond[y * w + x]:
                fond[y * w + x] = 1
                file.append((x, y))
    for y in range(h):
        for x in (0, w - 1):
            if clair(x, y) and not fond[y * w + x]:
                fond[y * w + x] = 1
                file.append((x, y))
    while file:
        x, y = file.popleft()
        for nx, ny in ((x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)):
            if 0 <= nx < w and 0 <= ny < h and not fond[ny * w + nx] and clair(nx, ny):
                fond[ny * w + nx] = 1
                file.append((nx, ny))
    alpha = Image.new("L", (w, h), 255)
    pa = alpha.load()
    for y in range(h):
        for x in range(w):
            if fond[y * w + x]:
                m = min(px[x, y])
                # Blanc pur : transparent ; gris de l'ombre : un voile.
                pa[x, y] = max(0, min(255, int((255 - m) * 1.6)))
    rgba = im.convert("RGBA")
    rgba.putalpha(alpha)
    return rgba, fond


def composantes(fond, w, h, pas=4, marge=28):
    """Les boîtes des éléments (pixels non fond), à l'échelle 1/pas, les
    petites boîtes (gouttes) fondues dans la grande boîte voisine."""
    gw, gh = w // pas, h // pas
    plein = bytearray(gw * gh)
    for gy in range(gh):
        for gx in range(gw):
            if not fond[(gy * pas) * w + gx * pas]:
                plein[gy * gw + gx] = 1
    vu = bytearray(gw * gh)
    boites = []
    for gy in range(gh):
        for gx in range(gw):
            if plein[gy * gw + gx] and not vu[gy * gw + gx]:
                vu[gy * gw + gx] = 1
                file = deque([(gx, gy)])
                x0 = x1 = gx
                y0 = y1 = gy
                n = 0
                while file:
                    x, y = file.popleft()
                    n += 1
                    x0, x1, y0, y1 = min(x0, x), max(x1, x), min(y0, y), max(y1, y)
                    for nx, ny in ((x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)):
                        if 0 <= nx < gw and 0 <= ny < gh and plein[ny * gw + nx] and not vu[ny * gw + nx]:
                            vu[ny * gw + nx] = 1
                            file.append((nx, ny))
                boites.append([x0 * pas, y0 * pas, (x1 + 1) * pas, (y1 + 1) * pas, n])
    # Les petites boîtes rejoignent la grande la plus proche (gouttes, pétales détachés).
    boites.sort(key=lambda b: -b[4])
    grandes = []
    for b in boites:
        if b[4] * pas * pas >= 2500:
            grandes.append(b)
            continue
        cx, cy = (b[0] + b[2]) / 2, (b[1] + b[3]) / 2
        def dist(g):
            dx = max(g[0] - cx, 0, cx - g[2])
            dy = max(g[1] - cy, 0, cy - g[3])
            return (dx * dx + dy * dy) ** 0.5
        proche = min(grandes, key=dist, default=None)
        if proche and dist(proche) < 70:
            proche[0], proche[1] = min(proche[0], b[0]), min(proche[1], b[1])
            proche[2], proche[3] = max(proche[2], b[2]), max(proche[3], b[3])
        else:
            grandes.append(b)
    # Numérotées par rangées (haut en bas), puis de gauche à droite.
    grandes.sort(key=lambda b: ((b[1] + b[3]) // 2 // 180, b[0]))
    return [(max(0, b[0] - marge), max(0, b[1] - marge), min(w, b[2] + marge), min(h, b[3] + marge)) for b in grandes]


def reperer(planche, apercu):
    im = Image.open(planche)
    rgba, fond = detourer(im)
    boites = composantes(fond, *im.size)
    d = ImageDraw.Draw(rgba)
    for i, b in enumerate(boites, start=1):
        d.rectangle(b, outline=(255, 0, 0, 255), width=3)
        d.text((b[0] + 6, b[1] + 6), str(i), fill=(255, 0, 0, 255))
        print(i, b)
    fondvert = Image.new("RGBA", rgba.size, (163, 212, 107, 255))
    fondvert.alpha_composite(rgba)
    fondvert.save(apercu)


def decouper():
    os.makedirs(SORTIE, exist_ok=True)
    for planche, elements in NOMS.items():
        im = Image.open(os.path.join(DA, planche))
        rgba, fond = detourer(im)
        boites = composantes(fond, *im.size)
        for nom, cases, element in elements:
            b = element if isinstance(element, tuple) else boites[element - 1]
            morceau = rgba.crop(b)
            # Rogné au plus près du dessin, mis à l'échelle (`cases` cases de
            # large, 256 px la case) et centré dans un canevas carré : le jeu
            # l'affiche en `custom_size` carré sans le déformer.
            morceau = morceau.crop(morceau.getbbox())
            ech = cases * 256 / morceau.width
            morceau = morceau.resize((max(1, round(morceau.width * ech)), max(1, round(morceau.height * ech))), Image.LANCZOS)
            cote = max(morceau.size)
            carre = Image.new("RGBA", (cote, cote), (0, 0, 0, 0))
            carre.alpha_composite(morceau, ((cote - morceau.width) // 2, (cote - morceau.height) // 2))
            carre.save(os.path.join(SORTIE, f"{nom}.png"), optimize=True)
            print(f"{nom}.png {cote}×{cote}")


if __name__ == "__main__":
    if len(sys.argv) >= 4 and sys.argv[1] == "reperer":
        reperer(sys.argv[2], sys.argv[3])
    else:
        decouper()
