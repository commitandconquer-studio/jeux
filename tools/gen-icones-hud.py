#!/usr/bin/env python3
# Commit & Conquer — licence GPL-3.0-or-later.
#
# Dessine les icônes du bandeau de jeu de Cross or Die (étoile, trophée, cadenas, flèche) :
#   python3 tools/gen-icones-hud.py      →  assets/branding/hud/*.png
import math, os
from PIL import Image, ImageDraw

RACINE = os.path.abspath(os.path.join(os.path.dirname(__file__), ".."))
SORTIE = os.path.join(RACINE, "assets/branding/hud")
N, S = 128, 4  # taille finale, sur-échantillonnage
os.makedirs(SORTIE, exist_ok=True)
OR = (255, 196, 36, 255)
OR_FONCE = (196, 120, 8, 255)
BLANC = (255, 255, 255, 255)
GRIS = (70, 72, 82, 255)


def toile():
    im = Image.new("RGBA", (N * S, N * S), (0, 0, 0, 0))
    return im, ImageDraw.Draw(im)


def sauver(im, nom):
    im.resize((N, N), Image.LANCZOS).save(os.path.join(SORTIE, nom + ".png"), optimize=True)


def p(x, y):
    return (x * S, y * S)


# Étoile à cinq branches, bord foncé
im, d = toile()
pts = []
for i in range(10):
    r = 56 if i % 2 == 0 else 25
    a = -math.pi / 2 + i * math.pi / 5
    pts.append(p(64 + r * math.cos(a), 68 + r * math.sin(a)))
d.polygon(pts, fill=OR, outline=OR_FONCE, width=7 * S // 2)
sauver(im, "etoile")

# Trophée : coupe, anses, pied
im, d = toile()
d.arc([p(14, 26), p(52, 70)], 90, 270, fill=OR_FONCE, width=8 * S)
d.arc([p(76, 26), p(114, 70)], 270, 90, fill=OR_FONCE, width=8 * S)
d.polygon([p(30, 14), p(98, 14), p(92, 62), p(76, 84), p(52, 84), p(36, 62)], fill=OR, outline=OR_FONCE, width=5 * S)
d.rectangle([p(56, 82), p(72, 100)], fill=OR_FONCE)
d.rounded_rectangle([p(36, 98), p(92, 116)], radius=6 * S, fill=OR, outline=OR_FONCE, width=4 * S)
d.ellipse([p(56, 30), p(72, 46)], fill=(255, 236, 160, 255))
sauver(im, "trophee")

# Cadenas
im, d = toile()
d.arc([p(36, 10), p(92, 76)], 180, 360, fill=GRIS, width=12 * S)
d.rounded_rectangle([p(24, 52), p(104, 114)], radius=12 * S, fill=GRIS)
d.ellipse([p(56, 70), p(72, 86)], fill=BLANC)
d.rectangle([p(61, 80), p(67, 98)], fill=BLANC)
sauver(im, "cadenas")

# Distance : une route en perspective avec ses pointillés, un fanion au bout
im, d = toile()
d.polygon([p(52, 30), p(76, 30), p(118, 118), p(10, 118)], fill=GRIS)
for y0, y1, w0, w1 in [(40, 50, 2, 3), (62, 78, 3, 4), (92, 114, 4, 6)]:
    d.polygon([p(64 - w0, y0), p(64 + w0, y0), p(64 + w1, y1), p(64 - w1, y1)], fill=BLANC)
d.rectangle([p(62, 4), p(66, 32)], fill=OR_FONCE)
d.polygon([p(66, 4), p(92, 12), p(66, 20)], fill=(236, 70, 60, 255))
sauver(im, "distance")

# Flèche (vers le haut) blanche
im, d = toile()
d.polygon([p(64, 10), p(112, 62), p(80, 62), p(80, 118), p(48, 118), p(48, 62), p(16, 62)], fill=BLANC)
sauver(im, "fleche")
