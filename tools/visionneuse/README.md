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
