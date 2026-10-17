# Ça fuit ! — cahier des charges

Univers **Croâ & Cie**. Un jeu de pose de tuyaux, calme et rigolo : une fuite
au milieu de nulle part, un crapaud plombier, et un paquet de tuyaux dépareillés
pour la colmater. Conçu à l'oral par Olivier le 17 octobre 2026.

> **État : développé le 17 octobre 2026** sur la branche `jeu/ca-fuit`,
> règles, décor et interface validés par Olivier. Reste à confirmer : la
> formule de score (ci-dessous, « 5 points par ouverture bouchée » est
> compté comme 5 points par bouchon posé). Linux et web. Non publié.

## Le décor : la pâture

Choisi par Olivier le 17 octobre 2026 (« un tuyau qui sort au milieu d'un
champ, d'une pâture, c'est largement suffisant »).

Une **pâture** vue de dessus, un peu en biais : l'herbe de Croâ & Cie, quelques
touffes plus hautes, des fleurs, une clôture de piquets au loin, un arbre, une
vache ou deux qui regardent (facultatif). Au centre, un bout de **canalisation
de zinc** qui sort de terre et a crevé, la terre retournée autour, l'eau qui
jaillit. Le **crapaud plombier** (`spawn_frog`, casquette et clé à molette)
regarde la fuite, les bras ballants.

Les tuyaux : des **cartes 2 × 1** low poly en **zinc** gris, un peu brillant,
joints et colliers plus sombres, posées sur une grille de cases carrées
dessinée en creux dans l'herbe. L'eau bleue avance **à vue** dans les tuyaux
(tuyaux ouverts sur le dessus, ou eau visible par transparence). Quand elle
sort sur du vide, elle **déborde** : une flaque bleue s'étale, assombrit
l'herbe, grandit un peu, un « splash », et c'est tout.

Ambiance `AmbiancePlugin::exterieur()` : lumière du moment, saisons, pluie et
neige ; musique `MUSIC_PRAIRIE`.

Autres décors envisagés et écartés : les toits du village (gouttière de zinc),
la mare (planches flottantes), le potager (robinet du jardin), la cave du
plombier (intérieur, lanternes).

## Les cartes

Une carte occupe deux cases. Debout (verticale), ses six côtés sont numérotés
dans le sens des aiguilles d'une montre :

```
        1
     ┌─────┐
   6 │     │ 2
     │     │
   5 │     │ 3
     └─────┘
        4
```

- Une carte = l'ensemble de ses **sorties** (côtés ouverts), toutes reliées
  entre elles. Les autres côtés sont des **murs**.
- Une seule sortie = un **bouchon** : l'eau entre et s'arrête là.
- On peut tourner une carte de 90°, 180° ou 270° (donc la poser à
  l'horizontale), jamais la retourner en miroir. À 180° : 1↔4, 2↔5, 3↔6.
- La **fuite** du centre est la carte toute ouverte (six sorties), posée à
  l'horizontale : deux sorties en haut, deux en bas, une à gauche, une à droite.
  Elle n'est pas dans le paquet.

### Les 34 formes

Toutes les formes possibles à rotation près, sauf la vide et la toute ouverte
(vérifié : 64 ensembles, 36 classes par rotation de 180°, moins 2).

| Sorties | Nombre | Formes |
|---|---|---|
| 1 (bouchons) | 3 | 1 · 2 · 3 |
| 2 | 9 | 1-4 · 2-5 · 3-6 · 1-2 · 1-6 · 1-3 · 1-5 · 2-3 · 2-6 |
| 3 | 10 | 1-2-3 · 1-2-4 · 1-2-5 · 1-2-6 · 1-3-4 · 1-3-5 · 1-3-6 · 1-5-6 · 2-3-5 · 2-3-6 |
| 4 | 9 | 1-2-3-4 · 1-2-3-5 · 1-2-3-6 · 1-2-4-5 · 1-2-4-6 · 1-2-5-6 · 1-3-4-6 · 1-3-5-6 · 2-3-5-6 |
| 5 | 3 | 1-2-3-4-5 · 1-2-3-4-6 · 1-2-3-5-6 |

### Le paquet

**70 cartes**, mélangées en début de partie (la « fuite du jour », à graine
tirée de la date, viendra dans une version suivante). Fréquences visées :

| Sorties | Part | ≈ cartes sur 70 |
|---|---|---|
| bouchons | 30 % | 21 (7 de chaque) |
| deux | 40 % | 28 |
| trois | 18 % | 13 |
| quatre et cinq | 12 % | 8 |

Les **prochains tuyaux** du paquet sont visibles dans un petit panneau
déroulant sur le côté, dans l'ordre où ils arriveront : on peut prévoir.

## La règle

1. L'eau sort de la fuite et **avance toute seule**, de proche en proche, dans
   tout ce qui est relié. Pas de limite de temps : on réfléchit autant qu'on
   veut. Une ouverture sur du vide **déborde** ; ce n'est pas une défaite.
2. **La main : 3 cartes** au départ. On en pose une, on en repioche une. On
   ne jette jamais une carte. Si **aucune** carte de la main n'a de place
   valide, on **pioche une carte de plus** : la main passe à 4, puis 5, 6…
   et n'en redescend jamais.
3. **Où poser :** une carte doit toucher **au moins une ouverture d'eau** du
   réseau (une sortie d'une carte posée, reliée à la fuite), ouverture contre
   ouverture.
4. **Jamais de fuite irréparable :** partout où la nouvelle carte touche une
   carte déjà posée, les bords doivent concorder : ouverture contre ouverture,
   mur contre mur. Sinon la pose est refusée. Le curseur de pose est vert
   quand c'est possible, rouge sinon.
5. **Fin de partie** quand il n'y a plus rien à poser **et plus rien à
   piocher** : le paquet est vide et aucune carte de la main n'a de place
   valide (ou la main est vide).

### Le score

Moins on pose de cartes, plus on marque. Un réseau **entièrement bouché**
(plus aucune ouverture sur du vide) vaut bien plus qu'un réseau qui fuit
encore. Meilleur score gardé avec `game_common::save`.

❓ Formule proposée, pas encore confirmée par Olivier :

- Réseau bouché : **1000 − 10 × cartes posées** (jamais moins de 100).
  Exemple : bouché en 12 cartes = 880 points.
- Réseau qui fuit encore : **100 − cartes posées** (jamais moins de 0),
  plus **5 points par ouverture bouchée**.
- Les cartes piochées en plus (main agrandie) ne comptent pas : seules les
  cartes posées comptent.

## Jouer

- Souris : survol de la grille = aperçu de la carte choisie, clic gauche =
  poser, clic droit ou **R** = tourner de 90°, molette = zoom.
- Clavier : flèches pour déplacer le curseur, 1/2/3… pour choisir une carte
  de la main, R pour tourner, Entrée pour poser, Échap pour le menu.
- La vue **dézoome toute seule** pour garder tout le réseau à l'écran ; la
  molette permet d'ajuster, et la vue revient à l'automatique à la pose
  suivante.
- Pas de bouton annuler : une carte posée est posée (l'eau y coule déjà).

### L'interface : la main en bas, la pioche à droite

Choisie par Olivier le 17 octobre 2026 (disposition « P2 »).

- **La grille 3D** occupe presque tout l'écran. Le curseur de pose suit la
  souris (ou les flèches) et montre la carte choisie en transparence, verte
  si la pose est possible, rouge sinon.
- **En haut à gauche**, un petit bandeau de compteurs : cartes posées,
  cartes restantes dans le paquet, ouvertures qui fuient, meilleur score.
- **En bas, au centre, la main** : les cartes à l'horizontale, comme on les
  tient, dessinées en vrais petits tuyaux de zinc vus de dessus (modèles 3D
  en vue orthographique, ou rendu à plat de la même forme). La carte choisie
  se soulève un peu. Quand la main grandit (4, 5, 6 cartes…), les cartes se
  resserrent. Touches 1 à 9 pour choisir, R pour tourner (la carte de la
  main tourne aussi).
- **À droite, une fine colonne « À venir »** : les prochains tuyaux du
  paquet empilés de haut en bas, en petit (les 6 premiers) ; un clic ou un
  survol la déroule pour en voir davantage, jusqu'à la fin du paquet.
- **Menu** : Échap, ou un petit bouton dans le bandeau. Pied commun du
  studio (Options, Aide, Quitter) dans le menu, comme partout.
- **Fin de partie** : panneau central avec le score, le détail (cartes
  posées, réseau bouché ou non, ouvertures bouchées), le meilleur score, et
  les boutons Rejouer et Menu.

Dispositions écartées : le panneau à gauche façon L'Observatoire (la grille
perdait un quart de largeur) ; tout dans le décor (main dans l'herbe,
brouette, pancarte), gardé comme idée pour une version suivante.

## Technique

- `crates/fuite-core` (sans Bevy) : cartes et sorties, rotations, raccords,
  grille infinie (`HashMap<(i32, i32), …>`), propagation de l'eau, règles de
  pose, paquet et mélange, score. Tests : les 34 formes exactement, les
  rotations, les raccords interdits, la propagation, la fin de partie.
- `games/ca-fuit` : le jeu sur le squelette commun (StudioPlugin, cinq
  langues, Options, ambiance, version web), mode capture `CA_FUIT_CAPTURE`.

## Décisions du 17 octobre 2026

1. Une ouverture qui a débordé reste une place valide : la flaque est
   cosmétique.
2. L'eau avance tuyau par tuyau, à vue (~0,4 s par case) ; on peut poser
   pendant qu'elle coule.
3. Première version : « Jouer » avec un paquet aléatoire. La fuite du jour
   viendra après.
4. Score : formule ci-dessus, **à confirmer**.
5. Paquet vide : on continue tant qu'une carte de la main est posable.
6. Tuyaux en **zinc**, gris.
7. Main qui grandit quand rien n'est posable ; paquet de 70 ; aperçu des
   prochains tuyaux.
8. Décor : la pâture (A). Interface : main en bas, pioche à droite (P2).
9. Icône proposée : le crapaud plombier, clé à molette en main, devant un
   tuyau de zinc qui gicle (à confirmer).
