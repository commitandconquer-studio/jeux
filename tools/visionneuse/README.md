# Visionneuse 3D des modèles de Traversée

Voir et modifier les modèles du jeu (véhicules, animaux, décors, héros, trains, chapeaux) **sans
recompiler** : la page lit directement les fichiers JSON.

```bash
python3 tools/visionneuse.py --ouvrir      # puis http://localhost:8800
```

- **Liste** à gauche (recherche, filtre par type) ; chaque modèle vient de `games/traversee/data/`.
- **Vue 3D** : glisser = tourner, molette = zoom, clic droit = déplacer. Boutons Iso / Profil / Face /
  Dessus, ortho, grille, crapaud de référence (échelle), animations (pattes, ailes, clignotants…).
- **Édition** : cliquer une pièce (vue ou liste) puis modifier forme, position, taille, rotation,
  matière, nom ; flèches du clavier pour la déplacer (`Maj` = plus fin) ; dupliquer, miroir Z,
  supprimer ; animations en JSON. `Ctrl+Z` annule, `Ctrl+S` ou « Enregistrer » écrit dans le JSON.
- **Rechargement automatique** quand un fichier change sur le disque (par exemple après un script).
- Adresse utile : `?modele=<fichier>::<chemin>&vue=profil&anim=0` (utilisée pour les captures).

Après avoir enregistré, régénérez le code du jeu :

```bash
python3 tools/gen-mondes.py      # décors, créatures, héros, trains des mondes à thème
python3 tools/gen-vehicules.py   # véhicules
```

## Attention

Les JSON produits par un script (`gen-espace.py`, `gen-farwest.py`, `gen-halloween.py`,
`gen-cyberpunk.py`, `gen-moyenage.py`, `gen-pirates.py`, `gen-monstres-nippon.py`,
`gen-rex-crete-bleue.py`) sont réécrits si on relance ce script. Dès qu'on retouche un monde dans la
visionneuse, **le JSON devient la référence : ne relancez plus le script de ce monde**.
`data/vehicules.json` est produit par `gen-vehicules.py` : il est en lecture seule dans la visionneuse.

Three.js (r160, licence MIT) est fourni dans `vendor/` : aucun accès réseau nécessaire.

## Personnages (couleurs et objets)

Page à part pour dessiner des personnages à partir des 12 humains de Kenney (Mini Characters, CC0) :

```bash
python3 tools/visionneuse.py            # puis http://localhost:8800/personnages.html (le port peut changer : voir la console)
```

- **Parties** (peau, cheveux, yeux, costume…) : une couleur par partie. La palette de Kenney est découpée en
  cases de 16 px ; chaque colonne est une matière (peau, cheveux, costume…) avec des nuances en lignes.
  Les cases sont rangées dans une partie par personnage de base (liste « Cases de la palette » : survoler une
  case l'allume en rose sur le modèle, puis choisir sa partie). Le rangement est gardé dans `personnages-cartes.json`.
  Pour l'instant seul `character-male-d` est rangé ; les autres se rangent de la même façon.
- **Pièces** : cubes, sphères, cônes et cylindres accrochés à un os (`head`, `torso`, `arm-left`, `arm-right`,
  `leg-left`, `leg-right`), qui suivent l'animation (cheveux dressés, kimono, chapeau…).
- **Enregistrer** écrit `games/traversee/data/personnages/<identifiant>.json` (parties, couleurs finales de chaque
  case, pièces). Le jeu ne les lit pas encore.
- Adresse utile : `?perso=<id>&vue=face|profil|dos|haut&rot=<degrés>&anim=<nom>`.
- Le panneau du navigateur doit être visible : une page cachée ne charge pas ses images.
