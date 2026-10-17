#!/usr/bin/env python3
# Commit & Conquer — licence GPL-3.0-or-later.
#
# Fabrique les sprites de « Ça fuit ! » à partir des photos du générateur :
#
#   CA_FUIT_SPRITES=photos cargo run -p ca-fuit      (chaque sujet sur fond noir puis blanc)
#   tools/sprites-ca-fuit.py photos                   (-> assets/sprites/ca-fuit/*.png)
#
# La transparence exacte se déduit des deux fonds : alpha = 1 − (blanc − noir),
# couleur = noir / alpha. Les sprites sont réduits à 128 pixels par case.
import os
import sys

from PIL import Image, ImageMath

PX_SPRITE = 128  # pixels par case dans le sprite

dossier = sys.argv[1] if len(sys.argv) > 1 else "photos"
racine = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
sortie = os.path.join(racine, "assets", "sprites", "ca-fuit")
os.makedirs(sortie, exist_ok=True)

noms = sorted(f[: -len("-noir.png")] for f in os.listdir(dossier) if f.endswith("-noir.png") and not f.startswith("chauffe"))
for nom in noms:
    noir = Image.open(os.path.join(dossier, f"{nom}-noir.png")).convert("RGB")
    blanc = Image.open(os.path.join(dossier, f"{nom}-blanc.png")).convert("RGB")
    bandes = []
    alphas = []
    for n, b in zip(noir.split(), blanc.split()):
        n, b = n.convert("F"), b.convert("F")
        alphas.append(ImageMath.eval("255 - (b - n)", n=n, b=b))
    alpha = ImageMath.eval("min(min(a, b), c)", a=alphas[0], b=alphas[1], c=alphas[2])
    un = Image.new("F", noir.size, 1.0)
    for n in noir.split():
        n = n.convert("F")
        # Couleur dé-multipliée par l'alpha (sans diviser par zéro) ; la
        # conversion en « L » ramène dans 0..255.
        bandes.append(ImageMath.eval("n * 255 / max(a, un)", n=n, a=alpha, un=un).convert("L"))
    rgba = Image.merge("RGBA", bandes + [alpha.convert("L")])
    # La photo est en pixels physiques (facteur d'écran inconnu) : on retrouve
    # le nombre de cases par les proportions (1×2, 4×3, 1×1 ou 3×3), puis on
    # ramène à PX_SPRITE pixels par case.
    formes = {(1, 2): (1, 2), (4, 3): (4, 3), (1, 1): (1, 1), (3, 3): (3, 3)}
    ratio = rgba.width / rgba.height
    cases = min(formes, key=lambda c: abs(c[0] / c[1] - ratio))
    taille = (cases[0] * PX_SPRITE, cases[1] * PX_SPRITE)
    rgba = rgba.resize(taille, Image.LANCZOS)
    rgba.save(os.path.join(sortie, f"{nom}.png"), optimize=True)
    print(f"{nom}.png {taille[0]}×{taille[1]}")
print(f"{len(noms)} sprites dans {sortie}")
