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

- Une grille de **7 × 7** cases, vide au départ.
- Un **paquet de 40 cartes**, déterminé par le **numéro de la partie**.
  Chaque carte montre une pièce de voie qui tient dans une case :
  **rail droit**, **rail en courbe**, **route droite**, **route en
  courbe**, ou **passage à niveau** (un rail et une route qui se
  croisent à angle droit).
- La **carte du dessus** est à poser tout de suite, où l'on veut, sur une
  case libre, et dans le sens qu'on veut (on la tourne). Les **deux
  suivantes** sont visibles, pour prévoir.
- On ne peut ni la garder ni la jeter : **toutes les cartes doivent être
  posées**. La partie s'arrête quand la 40e est posée (la grille a 49
  cases, il en reste donc au moins 9 de libres).
- On ne peut pas déplacer ni reprendre une carte posée ; **annuler la
  dernière carte** est permis (c'est un jeu pour s'amuser, pas pour
  punir).

### Le score (à équilibrer à l'essai)

Deux cartes **voisines** sont **raccordées** quand leurs deux côtés qui se
touchent sont ouverts et de la même voie (rail avec rail, route avec
route).

- **1 point par raccord.**
- **1 point par carte** de la plus longue **voie ferrée** d'un seul
  tenant, et **1 point par carte** de la plus longue **route** d'un seul
  tenant. Un passage à niveau compte dans les deux.
- **5 points par passage à niveau** dont les **quatre côtés** sont
  raccordés (le rail le traverse et la route aussi).
- **−1 point par chemin qui arrive dans le vide** : chaque **côté ouvert**
  d'une carte qui ne tombe pas sur un côté ouvert de la même voie (il
  donne sur une case vide, sur le bord de la grille, sur un côté fermé ou
  sur l'autre voie) coûte un point. Un passage à niveau a quatre côtés
  ouverts, donc il peut en coûter jusqu'à quatre. Les deux bouts d'une
  voie coûtent donc déjà deux points : il faut les rapprocher, les
  refermer l'un sur l'autre, ou les ramener contre une autre voie.

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
- Le paquet contient toujours **les mêmes quantités de pièces** (par
  exemple 8 de chaque sorte) : seul **l'ordre** change d'un numéro à
  l'autre.

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
  train** (locomotive rouge, deux wagons, en low-poly) parcourt **la plus
  longue voie ferrée** qu'on a posée, **la camionnette suit la plus longue
  route**, et les barrières des passages à niveau s'abaissent à leur
  passage. Fumée, sifflet. Plus la voie est longue, plus le voyage est
  beau. La 3D n'est chargée que pour cette séquence, de sorte que le jeu
  reste fluide pendant la partie.
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

1. **La taille de la grille et du paquet.** 7 × 7 et 40 cartes : d'accord ?
   (49 cases pour 40 cartes laisse peu de place libre, ce qui rend les
   choix serrés.)
2. **Les quantités.** 8 cartes de chaque sorte (5 sortes) : d'accord ?
3. **Le barème** est un point de départ, à ajuster à l'essai. Pour les
   bouts dans le vide, un côté ouvert contre l'**autre** voie ou contre un
   côté fermé compte-t-il aussi −1 (ce que j'ai écrit), ou seulement ceux
   qui donnent sur une case vide ou sur le bord ?
4. **Le puzzle à déduire.** On le laisse tomber ? Le noyau de règles déjà
   écrit pour lui (`crates/passage-core`, 12 tests) sera alors remplacé par
   celui de ce jeu.
