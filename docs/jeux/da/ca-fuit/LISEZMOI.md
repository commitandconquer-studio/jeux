# Direction artistique de Ça fuit ! — « La plomberie de poche »

Références validées par Olivier le 2026-10-17, images produites avec ChatGPT à
partir d'un brief du studio. Ce sont des **références d'intention**, pas des
assets finaux : les pièces du jeu sont dessinées par nos outils à partir des
règles ci-dessous ; le décor et l'interface peuvent être découpés dans les
planches sur fond uni (`decor.png`, `interface.png`, `tuyaux.png`).

- `ensemble-1.png`, `ensemble-2.png` : l'écran de jeu complet.
- `tuyaux.png` : droit vide, droit plein, coude vide, coude plein, bouchon, fuite.
- `decor.png` : touffes, fleurs, cailloux, buissons, clôture, roseaux, flaques, éclaboussure.
- `interface.png` : carte, carte sélectionnée, pictogrammes (cartes posées, paquet, goutte, trophée), bandeau.

Planches sur **fond transparent**, demandées ensuite pour être découpées
telles quelles (`tools/decoupe-ca-fuit.py`) :

- `decor-2.png` : touffes, fleurs, cailloux, buissons, roseaux, nénuphars, flaques, éclaboussure.
- `crapaud.png` : le crapaud plombier (trois poses), la caisse à outils, la clé.
- `pictos.png` : les quatre pictogrammes du bandeau.

Les tuyaux, eux, restent tracés par `tools/sprites-ca-fuit.py` : 34 formes
à raccorder au pixel près. Décision d'Olivier le 17 octobre 2026 : le
bouchon est **plat** (une bride rouge), pas un dôme.

Principes : 2D cartoon vue de dessus, aplats, contours bleu nuit épais, ombres
décalées simples, très peu de textures. Tuyaux en cuivre orangé à colliers gris
rivetés, canal intérieur bleu vif avec reflets quand l'eau circule, bouchon
rouge bombé, fuite = tuyau fendu qui jaillit. Herbe en deux verts (damier doux)
avec touffes et fleurs, clôture sur les bords, une mare dans un coin, le
crapaud plombier à casquette et sa caisse à outils. Cartes ivoire à bord bleu
nuit, sélection en halo cyan ; bandeau ivoire à pictogrammes.
