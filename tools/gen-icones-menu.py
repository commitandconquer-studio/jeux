#!/usr/bin/env python3
# Commit & Conquer — licence GPL-3.0-or-later.
#
# Dessine les icônes du menu de Cross or Die (blanches sur fond transparent, une couronne dorée) :
#   python3 tools/gen-icones-menu.py      →  assets/branding/menu/*.png
import math, os
from PIL import Image, ImageDraw, ImageFont

RACINE = os.path.abspath(os.path.join(os.path.dirname(__file__), ".."))
SORTIE = os.path.join(RACINE, "assets/branding/menu")
POLICE = os.path.join(RACINE, "assets/fonts/NotoSans-Bold.ttf")
N, S = 128, 4  # taille finale, sur-échantillonnage
os.makedirs(SORTIE, exist_ok=True)
BLANC = (255, 255, 255, 255)


def toile():
    im = Image.new("RGBA", (N * S, N * S), (0, 0, 0, 0))
    return im, ImageDraw.Draw(im)


def sauver(im, nom):
    im.resize((N, N), Image.LANCZOS).save(os.path.join(SORTIE, nom + ".png"), optimize=True)


def p(x, y):
    return (x * S, y * S)


# Jouer : triangle arrondi
im, d = toile()
pts = [(36, 22), (36, 106), (104, 64)]
d.polygon([p(*q) for q in pts], fill=BLANC)
for q in pts:
    d.ellipse([p(q[0] - 7, q[1] - 7), p(q[0] + 7, q[1] + 7)], fill=BLANC)
sauver(im, "jouer")

# Collection : deux cartes, la deuxième penchée derrière, avec un cadre découpé sur celle de devant
im, d = toile()
def carte(cx, cy, w, h, angle, couleur):
    couche = Image.new("RGBA", im.size, (0, 0, 0, 0))
    ImageDraw.Draw(couche).rounded_rectangle([p(cx - w, cy - h), p(cx + w, cy + h)], radius=9 * S, fill=couleur)
    im.alpha_composite(couche.rotate(angle, center=(cx * S, cy * S), resample=Image.BICUBIC))


carte(74, 62, 26, 36, 14, (255, 255, 255, 190))
carte(54, 66, 26, 36, -8, BLANC)
masque = Image.new("L", im.size, 0)
ImageDraw.Draw(masque).rounded_rectangle([p(42, 46), p(68, 70)], radius=5 * S, fill=255)
a = im.getchannel("A")
a.paste(0, mask=masque)
im.putalpha(a)
sauver(im, "collection")

# Options : engrenage
im, d = toile()
cx = cy = 64
dents = 8
for i in range(dents):
    a = i * 2 * math.pi / dents
    x, y = cx + 42 * math.cos(a), cy + 42 * math.sin(a)
    d.ellipse([p(x - 11, y - 11), p(x + 11, y + 11)], fill=BLANC)
d.ellipse([p(cx - 36, cy - 36), p(cx + 36, cy + 36)], fill=BLANC)
d.ellipse([p(cx - 15, cy - 15), p(cx + 15, cy + 15)], fill=(0, 0, 0, 0))
# on perce le centre (alpha 0)
masque = Image.new("L", im.size, 0)
ImageDraw.Draw(masque).ellipse([p(cx - 15, cy - 15), p(cx + 15, cy + 15)], fill=255)
im.putalpha(Image.eval(im.getchannel("A"), lambda v: v).point(lambda v: v))
a = im.getchannel("A")
a.paste(0, mask=masque)
im.putalpha(a)
sauver(im, "options")

# Aide : disque blanc avec un « ? » découpé
im, d = toile()
d.ellipse([p(16, 16), p(112, 112)], fill=BLANC)
police = ImageFont.truetype(POLICE, 84 * S)
masque = Image.new("L", im.size, 0)
md = ImageDraw.Draw(masque)
bb = md.textbbox((0, 0), "?", font=police)
md.text((64 * S - (bb[0] + bb[2]) / 2, 66 * S - (bb[1] + bb[3]) / 2), "?", font=police, fill=255)
a = im.getchannel("A")
a.paste(0, mask=masque)
im.putalpha(a)
sauver(im, "aide")

# Couronne dorée
im, d = toile()
OR, OR2, CONTOUR = (255, 205, 60, 255), (255, 160, 30, 255), (150, 85, 20, 255)
couronne = [(14, 98), (14, 42), (40, 68), (64, 30), (88, 68), (114, 42), (114, 98)]
d.polygon([p(*q) for q in couronne], fill=OR, outline=CONTOUR)
d.line([p(*q) for q in couronne + [couronne[0]]], fill=CONTOUR, width=5 * S, joint="curve")
d.rounded_rectangle([p(14, 98), p(114, 112)], radius=5 * S, fill=OR2, outline=CONTOUR, width=4 * S)
for x, y in ((14, 42), (64, 30), (114, 42)):
    d.ellipse([p(x - 8, y - 8), p(x + 8, y + 8)], fill=(255, 235, 140, 255), outline=CONTOUR, width=3 * S)
for x in (40, 64, 88):
    d.ellipse([p(x - 5, 84), p(x + 5, 94)], fill=(230, 80, 90, 255))
sauver(im, "couronne")
print("icônes écrites dans", SORTIE)
