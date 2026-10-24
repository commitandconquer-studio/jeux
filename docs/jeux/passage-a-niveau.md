# Passage à niveau — cahier des charges

Univers **Croâ & Cie**. Un jeu de cartes solo, tout simple : des cartes de
**rails** et de **routes** arrivent l'une après l'autre, on les pose sur une
grille, et on essaie de faire le **meilleur score**. Chaque partie dépend
d'un **numéro** : le même numéro donne toujours les mêmes cartes, dans le
même ordre. Idée d'Olivier le 24 octobre 2026, d'après un puzzle de rails
et un jeu de plateau où l'on trace des voies de chemin de fer et des routes.

> **État : cahier des charges, à valider par Olivier.** Rien n'est codé.
> Version simplifiée du 24 octobre 2026 : plus de puzzle à déduire ; le
> jeu est la pioche numérotée et la chasse au score. Le noyau de règles du
> puzzle écrit avant (`crates/passage-core`) ne sert plus à ce jeu.

## L'originalité

Le principe (poser des pièces de voie sur une grille, un score à battre)
est celui de nombreux jeux de cartes et de plateau. Le nom, les décors,
les icônes, les pièces, les couleurs, le train et les règles de score sont
**les nôtres**.

## La règle

- Une grille de **7 × 7** cases. **Quatre pièces sont déjà posées**, sur
  les bords, et imposées : **deux gares de départ** (voie ferrée) et **deux
  parkings de départ** (route). Il reste 45 cases libres.
- Un **paquet de 40 cartes**, déterminé par le **numéro de la partie**.
  Chaque carte montre une pièce qui tient dans une case (voir « Les
  cartes »).
- La **carte du dessus** est à poser tout de suite, où l'on veut, sur une
  case libre, et dans le sens qu'on veut (on la tourne). Les **deux
  suivantes** sont visibles, pour prévoir.
- On ne peut ni la garder ni la jeter : la carte du dessus doit être
  posée avant de voir la suivante. Mais **on peut s'arrêter quand on
  veut** (« c'est bon, j'ai assez de cartes ») avec le bouton
  **Terminer** (touche Entrée), qu'il faut presser deux fois pour
  confirmer. Sinon la partie s'arrête quand la 40e carte est posée.
- On ne peut pas déplacer ni reprendre une carte posée ; **annuler la
  dernière carte** est permis (c'est un jeu pour s'amuser, pas pour
  punir).

### Les cartes (40 par partie)

Chaque côté d'une carte est **ouvert** (rail ou route qui sort) ou
fermé. Deux côtés ouverts de la **même voie** qui se touchent sont
**raccordés**.

| Pièce | Entrées | Voie | Par partie |
|---|---|---|---|
| Rail droit, rail en courbe | 2 | rail | 6 + 4 |
| **Rail en T**, **rail en croix** | 3, 4 | rail | 2 + 1 |
| Route droite, route en courbe | 2 | route | 6 + 4 |
| **Route en T**, **route en croix** | 3, 4 | route | 2 + 1 |
| **Passage à niveau** | 4 | un rail et une route qui se croisent, **sans se raccorder** | 2 |
| **Gare routière à 2 entrées** (un rail, une route : en face ou en coude) | 2 | rail **et** route raccordés | 2 + 1 |
| **Gare routière à 3 entrées** (rail qui passe et route qui s'y raccorde, ou l'inverse) | 3 | rail **et** route raccordés | 1 + 1 |
| **Gare routière à 4 entrées** (rail et route qui se croisent **en se raccordant**) | 4 | rail **et** route raccordés | 1 |
| **Impasse** (rail) et **rond-point** (route) : un **cul-de-sac** | 1 | rail ou route | 3 + 3 |

- Une **gare routière** est le seul endroit où l'on **passe d'une route
  à une voie ferrée** : ses entrées de rail se raccordent aux rails
  voisins, ses entrées de route aux routes voisines, et les deux
  réseaux ne forment plus qu'un.
- Un **cul-de-sac** (impasse ou rond-point) n'a **qu'une entrée** : il
  **termine** proprement une voie, sans bout dans le vide au-delà.
- Les quatre pièces de départ sont des culs-de-sac particuliers : la
  **gare de départ** (une entrée de rail, un petit bâtiment) et le
  **parking de départ** (une entrée de route, un P sur fond bleu).

### Les côtés qui posent problème

Quand on vise une case, la carte s'affiche en fantôme **avec ses
conséquences** : chaque côté ouvert qui n'est pas raccordé est marqué,
sur la carte posée **et** sur ses voisines.

- un **petit rond rouge** : le côté donne sur une case vide ou sur le bord
  de la grille. Rien n'est perdu, une carte à venir peut encore le
  raccorder (sinon il coûte un point) ;
- une **grande croix rouge** : le côté bute contre une carte qui ne
  l'accepte pas (un rail qui arrive sur une route, ou contre un côté
  fermé). **Plus rien ne pourra le raccorder**, il coûtera un point
  jusqu'à la fin. La barre du haut prévient aussi : « côté(s)
  bloqué(s) ».

### Les départs

Les quatre pièces de départ sont toujours **aux mêmes places**, quel que
soit le numéro : une gare sur le bord ouest (rangée 2) et une au
bord est (rangée 6), un parking au bord nord (colonne 6) et un au bord sud
(colonne 2). Leur seule entrée regarde vers l'intérieur de la grille.

### Le score (à équilibrer à l'essai)

- **1 point par raccord** (rail avec rail, route avec route).
- **1 point par carte** du plus grand **groupe de voie ferrée** relié, et
  **1 point par carte** du plus grand **groupe de route** relié. Les
  passages à niveau et les gares routières comptent dans les deux.
- **5 points par passage à niveau** dont les quatre côtés sont raccordés.
- **4 points par gare routière** dont **toutes les entrées** sont
  raccordées.
- **6 points** si les **deux gares de départ** sont reliées entre elles
  par la voie ferrée ; **6 points** si les **deux parkings de départ**
  sont reliés par la route ; **6 points** si **les quatre départs** font
  partie d'un même réseau (en passant par des gares routières).
- **−1 point par chemin qui arrive dans le vide** : chaque **côté ouvert**
  d'une carte qui ne tombe pas sur un côté ouvert de la même voie (case
  vide, bord de la grille, côté fermé, ou l'autre voie) coûte un point.
  Les bouts de voie coûtent donc : il faut les **raccorder**, les
  **refermer par un cul-de-sac**, ou les ramener contre une autre voie.

Le total donne une **note** de une à trois étoiles ; un second repère
compte pour la fierté : le **meilleur score** de ce numéro.

## Le numéro

- On **choisit un numéro** de partie (n'importe quel nombre entier), ou
  on prend **« le numéro du jour »** (tiré de la date), ou **« au
  hasard »**.
- Le **paquet** en est déduit : le même numéro redonne exactement les
  mêmes 40 cartes dans le même ordre, sur n'importe quel ordinateur.
- Le jeu retient le **meilleur score de chaque numéro** déjà joué, dans la
  sauvegarde, et le montre quand on le choisit. On peut ainsi se mesurer à
  soi-même, ou comparer son score avec quelqu'un qui joue le même numéro.
- Le paquet contient toujours **les mêmes quantités de pièces** (le
  tableau ci-dessus) : seul **l'ordre** change d'un numéro à l'autre.

## Le décor : la gare du crapaud

**Le plateau** est une table de jeu en bois clair, vue de dessus, posée
dans une prairie : cases en pastel (herbe, sable, prés), quelques arbres
ronds, un étang, une barrière. Les **rails** sont ceux d'un jouet en bois :
traverses brunes, deux rails brun clair, courbes d'un quart de cercle de
rayon une demi-case. La **route** est un ruban gris doux à pointillés
blancs. Les **passages à niveau** ont des barrières à bandes rouges et
blanches. Les raccords se voient : quand deux cartes se raccordent, une
petite étincelle y passe.

Le **chef de gare**, c'est le crapaud de la maison, casquette et sifflet :
il apparaît dans le menu et sur l'écran de fin. La **camionnette** du
facteur, petite et ronde, fait la route. Ambiance
`AmbiancePlugin::exterieur()` pour les fêtes et le moment ; musique
`MUSIC_PRAIRIE`.

## Deux dimensions pour jouer, trois pour fêter

- **Pendant la partie, tout est en 2D** : la grille est dessinée à plat.
  C'est léger (on vise 55 images par seconde sur la puce Intel d'Olivier,
  comme pour les autres jeux) et lisible.
- **À la fin, la scène passe en 3D** : la caméra s'incline, le **petit
  train** (locomotive rouge, deux wagons, en low-poly, avec sa fumée)
  parcourt **la plus longue voie ferrée** posée, et **la camionnette**
  suit **la plus longue route**. Ils repartent du début quand ils sont
  arrivés. Plus la voie est longue, plus le voyage est beau. La 3D ne
  sert qu'à cet écran, le jeu reste léger pendant la partie. *(Les
  barrières des passages à niveau qui s'abaissent au passage du train
  sont prévues, pas encore faites.)*
- Un train **de bois** (une locomotive de jouet) : enfantin et sympa,
  comme le reste de l'univers.

## Jouer

- **Toucher ou cliquer une case libre** pose la carte du dessus, dans le
  sens affiché. **Un second toucher (ou la molette, ou une touche) la
  tourne** avant la pose ; une zone de prévisualisation montre où elle
  se raccordera, et combien de points cela rapporte.
- **Annuler** la dernière carte, **recommencer** le même numéro.
- Les parties sont **sauvegardées**, avec le meilleur score de chaque
  numéro.
- **Pas de défaite** : la partie finit simplement quand le paquet est
  épuisé. Le score peut être **négatif** si l'on dissémine des bouts
  ouverts partout ; les étoiles commencent à 0.

## Technique

Comme pour L'Observatoire et Le Concile : **les règles sont séparées du
moteur**, pour qu'on puisse les tester sans écran.

```
crates/passage-core/    cartes, paquet numéroté, grille, raccords et score (sans Bevy)
games/passage-a-niveau/ le jeu : plateau 2D, interface, séquence 3D
```

- **`passage-core`** : une carte (pièce et sens), le **tirage du paquet**
  à partir du numéro (même résultat partout, indépendant de la machine et
  de la version de Rust), le plateau, les **raccords**, les **plus longues
  voies** et le **score**.
- **Tests** : un numéro donne toujours le même paquet, et il contient les
  bonnes quantités de cartes ; les raccords et les plus longues voies sont
  calculés comme prévu sur des plateaux dessinés à la main ; le score d'un
  plateau ne dépend pas de l'ordre de pose ; les bouts dans le vide sont
  comptés côté par côté.
- Sprites 2D dessinés par un script, sur des règles chiffrées ; modèles
  3D du train, de la camionnette et du décor faits en code, dans le style
  low-poly des autres jeux.
- Version Linux et web, comme les autres.

## Questions pour Olivier

1. **Les places des départs.** Gares en (rangée 2, bord ouest) et
   (rangée 6, bord est), parkings en (colonne 6, bord nord) et
   (colonne 2, bord sud), toujours les mêmes : d'accord ? (Ou veux-tu
   qu'elles changent avec le numéro ?)
2. **La taille.** 7 × 7 donne 45 cases libres pour 40 cartes, donc 5 cases
   vides à la fin. Trop serré ? Un 8 × 8 laisserait plus de choix.
3. **Les quantités** de chaque pièce sont des premières propositions.
4. **Le barème** est un point de départ, à ajuster à l'essai. Un côté
   ouvert contre l'**autre** voie ou contre un côté fermé compte-t-il aussi
   −1 (ce que j'ai écrit) ?
5. **Les vies.** Pas de défaite : on garde le choix calme ?
