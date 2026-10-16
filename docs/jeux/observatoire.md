# L'Observatoire — cahier des charges

Univers **Eurêka** (sciences et logique). Un jeu de logique sur le principe
du « Tango » de LinkedIn (lui-même de la famille du Takuzu), sous un nom et
dans un décor originaux. Conçu en autonomie à la demande d'Olivier
(« faut tout tout seul, je veux voir ce que ça va donner »).

## Le décor

Le bureau d'un astronome, la nuit : une **carte du ciel** sur papier bleu
nuit, cerclée de laiton, posée sur une table en bois ; autour, une lunette de
cuivre, un globe céleste, un compas et une chandelle. Les **soleils** sont des
médaillons dorés à rayons, les **lunes** des croissants argentés. Ambiance
`AmbiancePlugin::interieur()` : la lumière suit l'heure, la chandelle brille
davantage la nuit, et les fêtes saluent le joueur.

## La règle

Une grille carrée (6 × 6 pour l'essentiel) à remplir de soleils et de lunes :

1. **Autant de soleils que de lunes** dans chaque rangée et chaque colonne.
2. **Jamais plus de deux** soleils (ou deux lunes) côte à côte, en rangée comme
   en colonne.
3. Entre deux cases, un **« = »** veut dire qu'elles sont pareilles, un
   **« × »** qu'elles sont différentes.

Quelques cases sont données au départ. Chaque grille a **une seule solution**,
trouvable par la logique seule.

## Jouer

- Clic (ou toucher) sur une case : vide → soleil → lune → vide. Clic droit :
  dans l'autre sens. Les cases données ne bougent pas.
- **Mode Novice** (par défaut) : les erreurs de règle sont signalées tout de
  suite (trois à la suite, rangée trop pleine, signe non respecté).
  **Mode Normal** : rien n'est signalé avant la fin.
- **Indice** : la prochaine déduction logique, expliquée en une phrase
  (« ces deux soleils côte à côte : leurs voisins sont des lunes »).
- Annuler, effacer, sablier ; victoire avec le temps et le meilleur temps.

## Le contenu

- **Le Livre des Constellations** : 100 grilles générées à l'avance
  (`tools/observatoire-gen`), vérifiées (solution unique, résolubles par la
  logique), classées par difficulté : 4 × 4 pour apprendre, 6 × 6 pour
  l'essentiel, 8 × 8 pour les experts.
- **La grille du jour** : une 6 × 6 tirée de la date, la même pour tout le
  monde ; série de jours réussis.

## Technique

- `crates/observatoire-core` (sans Bevy) : règles, solveur exhaustif, solveur
  logique (techniques et difficulté), générateur, grille du jour.
- `games/observatoire` : le jeu, sur le squelette commun (StudioPlugin, cinq
  langues, Options, ambiance, version web).
