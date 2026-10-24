# Passage à niveau — cahier des charges

Univers **Croâ & Cie**. Un jeu de logique où l'on trace, sur une même
grille, **une voie ferrée et une route** : le crapaud chef de gare doit
faire passer son petit train de bois d'une gare à l'autre, et la camionnette
du facteur d'un village à l'autre, sans qu'elles se gênent, sauf aux
passages à niveau. Idée d'Olivier le 24 octobre 2026, qui mélange deux
sources : un puzzle de rails vu sur un store d'applications (la déduction
par les chiffres des bords) et un jeu de plateau (les voies de chemin
de fer et les routes sur une même grille).

> **État : cahier des charges, à valider par Olivier.** Rien n'est codé.
> Nom choisi par Olivier le 24 octobre 2026. Jeu **solo**.

## L'originalité

On reprend des **principes** de jeu, pas leur habillage : la voie unique
avec des totaux sur les bords (genre classique « Tracks »), et l'idée de
tracer deux sortes de voies sur une grille à partir de pièces simples.
Le nom, les décors, les icônes, les pièces, les couleurs, le train et les
règles de score sont **les nôtres**. En particulier, on n'emploie ni le nom
ni le jeu de pièces ni le tableau de score du jeu de plateau qui a servi
d'inspiration.

## Le mode principal : la ligne à déduire

Une grille carrée (de 6 × 6 à 9 × 9) avec, sur ses bords, une **gare de
départ** et une **gare d'arrivée** (pour le train) et un **village de
départ** et un **village d'arrivée** (pour la route). Il faut tracer
**deux voies** :

1. La **voie ferrée** relie les deux gares, la **route** relie les deux
   villages. Chacune est **continue**, d'une seule pièce, **sans
   bifurcation** et **ne repasse jamais dans une de ses propres cases**.
2. Une case porte **au plus une voie**, droite ou en courbe, sauf aux
   **passages à niveau**.
3. Les **passages à niveau** sont des cases marquées sur la grille : la
   voie ferrée et la route s'y **croisent** en ligne droite, l'une à
   l'horizontale et l'autre à la verticale. Elles doivent y passer **toutes
   les deux**, et ne se croisent **nulle part ailleurs** (ni se touchent,
   ni se rejoignent).
4. Sur les bords, **deux chiffres par rangée et par colonne** : en bois,
   le nombre de cases de **rail** ; en gris, le nombre de cases de
   **route**. Un passage à niveau compte pour les deux.
5. Quelques **pièces sont déjà posées** au départ (cases grisées).

Chaque grille a **une solution unique**, qu'on trouve par raisonnement,
sans jamais deviner.

Les deux voies se **gênent** : une case prise par l'une ne sert plus à
l'autre, et elles ne se croisent qu'aux endroits prévus. C'est ce qui rend
les déductions plus riches que dans un puzzle à une seule voie.

## Le mode pioche du jour : les cartes du crapaud

Même grille, mêmes pièces, mais on **ne choisit plus** ce qu'on pose : les
pièces **arrivent au hasard**, sur des cartes, et il faut **toutes les
utiliser**. Ça se joue **seul**.

- Un paquet de **40 cartes** est mélangé au début. Chaque carte montre une
  pièce : rail droit, rail en courbe, route droite, route en courbe, ou
  passage à niveau.
- La **carte du dessus** est à poser tout de suite. Les **deux suivantes**
  sont visibles, pour prévoir.
- On **tourne** la carte à sa guise, puis on la **pose** sur la grille, à
  condition qu'elle **touche une voie déjà posée ou un bord de départ**.
- On **ne peut pas la garder** ni la jeter : elle doit être posée. S'il n'y
  a plus aucune case où elle puisse aller, elle est **perdue** (−2 points)
  et la partie continue.
- La partie finit quand le paquet est épuisé.
- Les cartes sont tirées de **la date** : tout le monde joue le même
  paquet le même jour, comme la grille du jour des autres jeux.

**Le score** (à équilibrer à l'essai) :

- **1 point par case** de la plus longue voie ferrée et **1 point par
  case** de la plus longue route ;
- **3 points par passage à niveau** formé, mais **−2 points** par voie qui
  se termine dans le vide ;
- **+5 points** si le train rejoint une gare et la route un village avec
  un trajet continu ;
- des **étoiles** (une à trois) selon le total.

Les barèmes ci-dessus sont une première proposition pour ce cahier des
charges : ils seront ajustés quand on pourra jouer.

## Le décor : la gare du crapaud

**Le plateau** est une table de jeu en bois clair, vue de dessus, posée
dans une prairie : cases en pastel (herbe, sable, prés), quelques arbres
ronds, un étang, une barrière. Les **rails** sont ceux d'un jouet en bois :
traverses brunes, deux rails brun clair, courbes d'un quart de cercle de
rayon une demi-case. La **route** est un ruban gris doux à pointillés
blancs. Les **passages à niveau** ont des barrières à bandes rouges et
blanches. Les cases données sont un cran plus foncées ; les cases marquées
« sans voie » portent une petite croix de crayon.

Le **chef de gare**, c'est le crapaud de la maison, casquette et sifflet :
il apparaît dans le menu et sur l'écran de victoire. La **camionnette** du
facteur, petite et ronde, fait la route. Ambiance
`AmbiancePlugin::exterieur()` pour les fêtes et le moment ; musique
`MUSIC_PRAIRIE`.

## Deux dimensions pour jouer, trois pour fêter

- **Pendant la partie, tout est en 2D** : la grille est dessinée à plat.
  C'est léger (on vise 55 images par seconde sur la puce Intel d'Olivier,
  comme pour les autres jeux) et c'est la meilleure vue pour raisonner.
- **À la victoire, la scène passe en 3D** : la caméra s'incline, le
  **petit train** (locomotive rouge, deux wagons, en low-poly) parcourt la
  voie qu'on vient de poser, **la camionnette suit la route**, et les
  barrières des passages à niveau s'abaissent à leur passage. Fumée,
  sifflet, entrée en gare. C'est la récompense. La 3D n'est chargée que
  pour cette séquence, de sorte que le jeu reste fluide pendant la
  réflexion.
- Un train **de bois** (une locomotive de jouet) : enfantin et sympa,
  comme le reste de l'univers.

## Jouer

- **Toucher ou cliquer une case** : elle passe de vide à **rail**, puis à
  **route**, puis à **sans voie** (croix), puis revient à vide. La forme
  de la pièce (droite ou courbe, et son sens) est **déduite de ses
  voisines** : le joueur ne tourne rien. Dans le mode de la pioche, on
  tourne la carte avant de la poser.
- **Glisser** d'une case à l'autre trace d'un coup une portion de voie,
  de la couleur choisie.
- **Les chiffres** des bords se grisent quand leur total est atteint, et
  rougissent s'il est dépassé.
- **Annuler**, **effacer** et **recommencer** ; les parties sont
  sauvegardées.
- **Indice** : met en évidence une case que l'on peut déduire, avec **une
  phrase** qui explique le raisonnement (« la colonne 3 demande deux rails
  et en a déjà deux : le reste est vide »).
- **Deux niveaux d'aide** pour le puzzle, comme dans L'Observatoire :
  **Novice** (les erreurs sont signalées tout de suite) et **Normal**
  (on ne sait qu'à la fin, ou en demandant à vérifier).
- Pas de défaite : se tromper ne fait que retarder le train.

## Le contenu

- **Le Livre des Lignes** : 100 grilles de difficulté croissante (de 6 × 6
  à 9 × 9), générées et vérifiées par un outil. Les premières ne
  contiennent qu'une voie à tracer en plus de la route déjà posée, pour
  apprendre ; les dernières demandent de combiner les deux réseaux.
  Chaque grille a une solution unique, trouvable par déduction.
- **La ligne du jour** : une grille du livre tirée de la date.
- **La pioche du jour** : le paquet de cartes de la date, pour le mode à
  cartes.
- Les **étoiles** récompensent le temps et l'absence d'indice ; la
  progression se débloque ligne après ligne, comme les autres jeux de la
  maison.

## Technique

Comme pour L'Observatoire et Le Concile : **les règles sont séparées du
moteur**, pour qu'on puisse les tester sans écran.

```
crates/passage-core/   grille, règles, solveur et générateur (sans Bevy)
tools/passage-gen/     génère le Livre des Lignes (games/passage-a-niveau/grilles.txt)
games/passage-a-niveau/         le jeu : plateau 2D, interface, séquence 3D
```

- **`passage-core`** : la grille avec ses deux voies, les passages à
  niveau, la vérification des règles, un **solveur par déduction** (qui
  sert aux indices et à mesurer la difficulté) et un **générateur** qui ne
  garde que les grilles à solution unique ; le tirage des cartes et le score
  du mode pioche du jour.
- **Tests** : toute grille du livre a une solution unique ; le solveur par
  déduction résout les grilles faciles sans chercher au hasard ; les deux
  voies d'une solution sont continues, sans bifurcation et ne se croisent
  qu'aux passages à niveau ; le paquet de cartes et le score sont les mêmes pour une même graine.
- Sprites 2D dessinés par un script, sur des règles chiffrées ; modèles
  3D du train, de la camionnette et du décor faits en code, dans le style
  low-poly des autres jeux.
- Version Linux et web, comme les autres.

## Questions pour Olivier

1. **Le paquet.** 40 cartes, avec les proportions (rails, routes, passages
   à niveau) à régler à l'essai. D'accord ?
2. **Une carte impossible à poser.** Je la perds avec −2 points. Préfères-tu
   pouvoir la **défausser** contre un petit malus, ou garder une **main**
   de deux cartes ?
3. **Les barèmes** sont des chiffres de départ, à ajuster à l'essai.
4. **Les vies.** Pas de défaite dans mon cahier des charges ; la capture
   qui a donné l'idée montrait trois cœurs. On garde le choix calme ?
5. **Ponts et tunnels.** À garder pour une version suivante, pour des
   grilles encore plus difficiles ?
