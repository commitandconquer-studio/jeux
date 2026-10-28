# « Suivez le guide ! » — écrire une aventure (mode opératoire)

Ce document sert de consigne à une IA (ou à une personne) qui doit écrire une **aventure** pour le mode scénario :
des **scènes** (décor, personnages, dialogues) et des **niveaux** (cartes à jouer) enchaînés. Le résultat est **un seul
fichier texte `.txt`**, en UTF-8, que l'on charge dans l'éditeur en ligne (bouton « Ouvrir un fichier »), qui permet de le
vérifier et de le jouer en 3D.

## 1. Le jeu en bref

Le joueur est un **guide** qui mène une file de touristes, comme une chenille de bal populaire. Il se déplace case par case
(flèches), attrape les touristes isolés pour allonger la file, et perd s'il touche un mur, un obstacle, un trou ou sa propre
file. Un niveau est réussi quand il a recruté le nombre de touristes demandé (et, si le niveau a une sortie, quand toute la
file l'a franchie). Une aventure enchaîne les étapes dans l'ordre du fichier : après une scène vient l'étape suivante ; un
niveau réussi enchaîne tout de suite ; un niveau raté se rejoue.

## 2. Structure du fichier

```
# Une ligne qui commence par # est un commentaire.
titre = Titre de l'aventure
description = Une phrase de présentation

[Nom de l'étape]
type = scene          ← seulement pour une scène
...clés de l'étape...

[Autre étape]
lieu = jardin         ← pas de « type » : c'est un niveau
...clés...
carte:
...lignes de la carte...
```

- `titre` et `description` : avant la première étape (facultatifs mais recommandés).
- Chaque étape commence par `[Nom]` (le nom est libre, unique de préférence, sans crochets).
- Une ligne = une clé : `clé = valeur`. Une ligne vide termine une carte. Pas de ligne en dehors de ce format.
- Interdit dans les textes : le caractère `|` (séparateur) et les retours à la ligne dans une réplique.
- L'**ordre des étapes** est l'ordre de l'aventure. Alterner scènes et niveaux : une scène avant un niveau explique
  le but, une scène après le raconte, un tutoriel introduit **un seul élément nouveau** par niveau.
- Langue : tout le texte des aventures est en français pour l'instant.

## 3. Une scène (`type = scene`)

```
[Rencontre]
type = scene
decor = jardin
camera = moyen
personnage = Léa | 1 | 12 | centre | salut | guide
personnage = Max | 2 | 3.8.5.1.0 | droite | repos | - | lunettes-soleil+canne
replique = Léa | Bonjour ! Je m'appelle Léa, je serai votre guide.
replique = Max | Enchanté ! | gros
```

### 3.1 `decor`
Un lieu : `jardin`, `desert`, `ruines`, `cimetiere`, `pirates`, `parc`, `ville`, `gare`, `port`, `medieval`, `grottes`.
Chaque lieu est un paysage différent (prairie, désert, ruines antiques, cimetière de nuit, île des pirates, parc de loisirs,
parc au milieu d'une ville, gare, port, village médiéval avec remparts, grottes sombres). Une scène a **un seul décor**.

### 3.2 `camera`
Le cadrage de départ (perspective, jamais isométrique) :
`large` (tout le décor), `moyen` (plan standard pour dialoguer), `gros` (gros plan sur les personnages),
`plongee` (vue de dessus), `bas` (contre-plongée, les personnages paraissent grands).
La caméra se recale en douceur quand une réplique change de cadrage (voir 3.5). Défaut : `moyen`.

### 3.3 `personnage` (de 0 à 5 par scène)
`personnage = Nom | modèle | apparence | place | animation | rôle | accessoires`
Les quatre premiers champs sont obligatoires, les suivants facultatifs (dans l'ordre).

- **Nom** : affiché devant la réplique si c'est le même que « Qui » dans `replique`. Garder le même nom (et la même
  apparence) pour un personnage d'une scène à l'autre.
- **modèle** : un numéro de 0 à 18 :
  0 Léo, 1 Lola, 2 Max, 3 Inès, 4 Noé, 5 Zoé, 6 Hugo, 7 Jade, 8 Sacha, 9 Maya, 10 Théo, 11 Lou,
  12 Zombie, 13 Squelette, 14 Vampire, 15 Fantôme (4 monstres : couleurs non modifiables),
  16 Rémi, 17 Camille, 18 Sam (les trois en fauteuil roulant).
  Un même modèle peut servir plusieurs fois avec des couleurs différentes.
- **apparence** : soit un numéro de 0 à 95 (apparence prédéfinie au hasard), soit **cinq indices** séparés par des points
  `peau.cheveux.haut.bas.chaussures`, par exemple `3.8.5.1.0` :
  - peau 0–7 : du plus clair (0) au plus foncé (7) ;
  - cheveux 0–11 : 0 noir, 1 brun foncé, 2 brun, 3 châtain, 4 blond, 5 blond clair, 6 roux, 7 gris, 8 blanc, 9 bleu, 10 rose, 11 vert ;
  - haut 0–15 : 0 rouge, 1 orange, 2 jaune, 3 vert, 4 turquoise, 5 bleu, 6 bleu marine, 7 violet, 8 rose, 9 blanc, 10 gris, 11 noir, 12 marron, 13 vert d'eau, 14 lavande, 15 or ;
  - bas 0–7 : 0 bleu nuit, 1 noir, 2 gris, 3 beige-marron, 4 sable, 5 bleu, 6 vert foncé, 7 bordeaux ;
  - chaussures 0–3 : 0 blanc, 1 noir, 2 marron, 3 rouge.
  Éviter un haut et un bas de même couleur. Pour un monstre (12–15) l'apparence est ignorée.
- **place** : `gauche2`, `gauche`, `centre`, `droite`, `droite2` (de gauche à droite de l'écran), éventuellement suivi de
  `-fond` pour reculer d'un rang (`centre-fond`), ou deux nombres `x,z` (x de -4 à 4 : gauche/droite ; z de 0 à 3 : vers le
  fond). Ne pas mettre deux personnages au même endroit. Pour 3 personnages : `gauche`, `centre`, `droite`.
- **animation** (défaut `repos`) : `repos`, `salut` (salue, dit oui), `non`, `marche`, `court`, `saute`, `tombe`, `accroupi`, `assis`,
  `conduit`, `mort` (couché au sol), `ramasse`, `tient-droite`, `tient-gauche`, `tient-deux`, `vise-droite`, `vise-gauche`,
  `vise-deux`, `frappe-droite`, `frappe-gauche`, `coup-pied-droite`, `coup-pied-gauche`, `interagit-droite`,
  `interagit-gauche`, `fixe` (pose neutre), et pour les modèles en fauteuil (16–18) : `fauteuil-assis`,
  `fauteuil-regarde-gauche`, `fauteuil-regarde-droite`, `fauteuil-avance`, `fauteuil-recule`, `fauteuil-gauche`, `fauteuil-droite`.
  Les animations sont en boucle (sauf `mort`, qui reste couché). Un personnage qui parle peut saluer (`salut`) ; les
  autres sont au `repos`.
- **rôle** : `guide` (casquette rouge et drapeau) ou `-` pour aucun. Un seul guide par scène en général.
- **accessoires** (liste séparée par `+`) : `lunettes`, `lunettes-soleil`, `masque`, `appareil-auditif` (sur la tête) ;
  `canne`, `canne-blanche`, `canne-malvoyant`, `bequille`, `defibrillateur-vert`, `defibrillateur-rouge` (posés au sol à côté
  du personnage). Pour mettre des accessoires sans guide, écrire `-` dans le champ rôle.

### 3.4 `replique`
`replique = Qui | texte` ou `replique = Qui | texte | camera`
- **Qui** : le nom du personnage qui parle (comme dans `personnage`), ou un narrateur (`Narrateur`, `Voix`…).
- **texte** : une phrase ou deux, **140 caractères maximum** (c'est un sous-titre de deux lignes). Pas de `|`.
- Les répliques s'affichent dans l'ordre, en sous-titres en bas de l'écran, avec les boutons Précédent / Suivant.
  Une scène peut n'avoir aucune réplique (simple plan). Prévoir 2 à 8 répliques par scène.

### 3.5 Changer de caméra pendant la scène
Le troisième champ d'une réplique (`large`, `moyen`, `gros`, `plongee`, `bas`) change le cadrage **à partir de cette réplique**
et pour les suivantes, jusqu'au prochain changement. Exemple : plan large pour installer le décor, plan moyen pour
dialoguer, gros plan pour une confidence ou une blague.

### 3.6 De la fiche de personnage au code
Le scénariste décrit ses personnages en langage naturel (« Léa, 30 ans, guide, se déplace avec des béquilles »). L'IA doit
**traduire chaque trait en paramètres du jeu**, une fois pour toutes, puis recopier **exactement la même ligne
`personnage = …`** (sauf `place` et `animation`) dans toutes les scènes où le personnage apparaît.

Gabarit de fiche à fournir à l'IA :
```
PERSONNAGES
- Léa : guide, 30 ans, peau claire, cheveux roux, pull orange, jean, béquilles. Chaleureuse, un peu maladroite.
- Max : touriste, 45 ans, peau mate, cheveux noirs, lunettes de soleil, toujours de bonne humeur.
- Zoé : touriste, 70 ans, cheveux blancs, canne blanche (malvoyante), vêtements bleus.
- Hugo : touriste en fauteuil roulant, 25 ans, cheveux châtains, sportif.
```

Correspondances à appliquer :
| trait dans la fiche | paramètre du jeu |
|---|---|
| guide, chef de groupe | rôle `guide` (casquette rouge et drapeau). Tout personnage peut être guide |
| béquilles / jambe cassée / blessé | accessoire `bequille` (une seule béquille posée au sol à côté du personnage ; on ne peut pas en afficher deux) |
| canne, personne âgée | accessoire `canne` ; cheveux gris ou blancs (cheveux 7 ou 8) pour une personne âgée |
| malvoyant, aveugle | accessoire `canne-blanche` ; pour une basse vision `canne-malvoyant` ; éventuellement `lunettes-soleil` : `canne-blanche+lunettes-soleil` |
| lunettes | `lunettes` ; lunettes de soleil : `lunettes-soleil` |
| malentendant, sourd | `appareil-auditif` |
| masque, malade, soignant | `masque` ; soignant avec défibrillateur : `defibrillateur-vert` ou `defibrillateur-rouge` |
| fauteuil roulant | modèle 16 (Rémi, homme), 17 (Camille, femme) ou 18 (Sam), animation `fauteuil-assis` (au repos), `fauteuil-avance` (se déplace), `fauteuil-regarde-gauche` / `fauteuil-regarde-droite` |
| couleur de peau (très claire … très foncée) | peau 0 à 7 |
| couleur de cheveux (noir, brun, châtain, blond, roux, gris, blanc, bleu, rose, vert) | cheveux 0, 1, 3, 4, 6, 7, 8, 9, 10, 11 (voir 3.3) |
| vêtements, couleur du haut | haut 0 à 15 (voir 3.3) ; pantalon, jupe : bas 0 à 7 ; chaussures : 0 à 3 |
| homme / femme / enfant / ado | modèle : hommes 0 Léo, 2 Max, 4 Noé, 6 Hugo, 8 Sacha, 10 Théo ; femmes 1 Lola, 3 Inès, 5 Zoé, 7 Jade, 9 Maya, 11 Lou (tous de même taille : l'âge se montre par les cheveux et les accessoires) |
| monstre (zombie, squelette, vampire, fantôme) | modèles 12, 13, 14, 15 (couleurs fixes) |
| sportif, pressé | animation `court` ou `saute` ; fatigué : `accroupi` ; assis : `assis` ; en colère : `non` ; content : `salut` |

Règles :
- Un personnage = un nom, un modèle, des couleurs, des accessoires **fixes pour toute l'aventure**.
- Le prénom du modèle (Léo, Lola…) n'a aucune importance : le **Nom** de la ligne `personnage` est celui de la fiche.
- Si la fiche demande quelque chose d'impossible (deux béquilles, une couleur exacte, un chapeau), l'IA choisit
  l'approximation la plus proche et le signale dans une note à part (hors du fichier).

Exemple de traduction : « Léa, guide, cheveux roux, pull orange, jean, béquilles »
→ `personnage = Léa | 1 | 1.6.1.0.2 | centre | salut | guide | bequille`
(modèle 1 ; peau 1 claire ; cheveux 6 roux ; haut 1 orange ; bas 0 jean ; chaussures 2 marron ; guide ; béquille).

## 4. Un niveau (pas de `type`)

```
[Premier rassemblement]
lieu = jardin
touristes = 4
pas = 0.34
depart = est
sortie = oui
carte:
0011111111111100
0111111111111110
1111111111111111
111S111111111111
1111111111111111
0111111111111110
0011111111111100
```

### 4.1 Clés
| clé | valeur | rôle |
|---|---|---|
| `lieu` | `jardin`, `desert`, `ruines`, `cimetiere`, `pirates`, `parc`, `ville`, `gare`, `port`, `medieval`, `grottes`, ou `special` | décor et éléments disponibles (voir 4.3) |
| `paysage` | un nom de lieu | seulement avec `lieu = special` : le décor visuel |
| `touristes` | entier ≥ 1 | nombre de touristes à recruter |
| `pas` | 0.24 à 0.40 | secondes par case (plus petit = plus rapide). Débutant : 0.34 |
| `depart` | `est`, `ouest`, `nord`, `sud` | direction de départ du guide |
| `sortie` | `oui` | une sortie s'ouvre quand l'objectif est atteint : toute la file doit la traverser |
| `temps` | secondes | temps visé pour avoir 2 étoiles (facultatif ; ignoré dans les aventures) |
| `rivaux`, `trains`, `eboulements` | entiers | éléments **placés au hasard** par le jeu (voir 4.3) |

### 4.2 La carte (`carte:` puis une ligne par rangée, toutes de même longueur)
Un caractère par case ; la rangée du haut est le nord. Taille de 6×6 à 64×64 ; prévoir 12×12 à 24×24 pour un niveau normal.

| caractère | case |
|---|---|
| `0` | vide : rien, non jouable (permet de dessiner des formes) |
| `1` | case libre |
| `2` à `7` | **bloc** (obstacle mortel) : l'objet dépend du lieu (rocher, cactus, tombe, tonneau, arbre…) ; n'utiliser que `2`, `3`, `4` (les 4 premiers) en général |
| `o` | trou (mortel) |
| `p` | ponton fragile : on ne peut le traverser qu'une fois, ensuite il devient un trou |
| `z` | accélérateur : un coup de vitesse |
| `h` / `H` | herse (porte qui s'ouvre et se ferme) : `h` et `H` sont en opposition de phase |
| `>` `<` `^` `v` | tapis roulant vers l'est, l'ouest, le nord, le sud (on ne peut pas le prendre à contresens) |
| `A` `B` `C` `D` | téléporteurs : **exactement deux cases de la même lettre** ; entrer dans l'une fait sortir par l'autre |
| `S` | départ du guide (obligatoire, **un seul**) |
| `E` | sortie (si `sortie = oui` ; sinon la sortie est placée automatiquement loin du départ) |

### 4.3 Ce qui existe dans chaque lieu (les nouveautés arrivent dans cet ordre et se cumulent)
`jardin` : cases libres seulement → `desert` : + blocs (`2`…) → `ruines` : + trous (`o`) → `cimetiere` : + téléporteurs (`A`…`D`)
→ `pirates` : + tapis (`>` `<` `^` `v`) → `parc` : + accélérateurs (`z`) → `ville` : + `rivaux` (un guide rival traverse
le plateau et emmène les touristes qu'il touche ; son entrée clignote 3 s avant) → `gare` : + `trains` (un vrai train mortel
traverse le plateau) → `port` : + pontons (`p`) → `medieval` : + herses (`h`, `H`) → `grottes` : + `eboulements` (des pierres
tombent et restent).
`lieu = special` autorise **tout** (choisir `paysage`). Ne pas utiliser un élément avant le lieu qui l'introduit (sauf en
`special`). `rivaux`, `trains` et `eboulements` s'écrivent en clés (`rivaux = 2`), pas dans la carte.
Un niveau d'aventure peut changer de lieu à chaque fois (jardin puis désert puis ville).

### 4.4 Règles à respecter absolument (sinon le niveau est refusé ou injouable)
1. Une seule `S`. Les **deux cases derrière S** (dans le sens opposé à `depart`) et **la case devant S** doivent être libres.
2. Aucune case libre isolée : tout doit être atteignable depuis `S` à pied, **ou** via une paire de téléporteurs (des îlots
   séparés par du vide sont permis s'ils sont reliés par `A`…`D`).
3. **Pas de cul-de-sac** : une case libre doit avoir au moins deux voisines libres (haut, bas, gauche, droite). Les coins
   d'une forme arrondie (cases `0` aux angles) respectent déjà la règle ; éviter les couloirs d'une case qui finissent en bout.
4. Nombre de cases libres **supérieur à touristes + 13** (de la place pour la file).
5. Ne pas enfermer un bloc ou un trou : pas deux blocs/trous voisins, pas de bloc collé à un mur qui coupe un passage.
6. Un tapis roulant fait avancer la file ; ne pas en mettre un face à un mur.
7. Les téléporteurs vont **par paires**, éloignés d'au moins 4 cases.

### 4.5 Difficulté
- Tutoriel : 4 à 6 touristes, `pas = 0.34`, grand plateau (16×16), aucun piège ou un seul nouveau.
- Moyen : 8 à 14 touristes, `pas` 0.30–0.32, plateau 14×14 à 20×20, 3–6 pièges.
- Difficile : 16+ touristes, `pas` 0.26–0.28, plateau resserré, plusieurs pièges.
- Introduire **un seul élément nouveau** par niveau et l'expliquer dans la scène qui précède.

## 5. Gabarit d'écriture d'une aventure
(Entrée attendue de l'IA : la fiche des personnages au format 3.6, le thème de l'aventure, le nombre d'étapes souhaité et les lieux.)
1. Une scène d'ouverture (présentation du guide, du lieu, du but).
2. Une alternance « scène courte → niveau → scène courte → niveau », 3 à 8 étapes pour une aventure courte.
3. Une scène finale (conclusion, clin d'œil, annonce de la suite).
4. Les personnages importants gardent leur nom, leur modèle et leurs couleurs d'une scène à l'autre.
5. Les dialogues sont courts, vivants, adaptés à tous les âges ; l'humour passe par les personnages et les accessoires.

## 6. Exemple complet et valide

Voir `games/snake/scenarios/exemple.txt` : 4 étapes (scène, niveau jardin, scène désert avec accessoires, niveau désert avec
sortie). Ce fichier est vérifié par un test du jeu : il reste valide.

## 7. Liste de contrôle avant de rendre le fichier
- [ ] `titre` et `description` présents.
- [ ] Chaque étape commence par `[Nom]`, scène avec `type = scene`.
- [ ] Chaque scène : `decor`, au moins un `personnage` (sauf plan vide) et des `replique`.
- [ ] Au plus 5 personnages par scène, places différentes, modèle 0–18, apparence 0–95 ou `a.b.c.d.e` dans les bornes.
- [ ] Chaque personnage de la fiche est traduit une fois (3.6) et sa ligne `personnage` est identique d'une scène à l'autre.
- [ ] Les animations et accessoires sont écrits exactement comme dans les listes (minuscules, tirets).
- [ ] Aucun `|` dans les textes, répliques de 140 caractères maximum.
- [ ] Chaque niveau : `lieu`, `touristes`, `pas`, `depart`, une carte rectangulaire avec un seul `S`.
- [ ] Les règles du 4.4 sont respectées (accessibilité, pas de cul-de-sac, places libres, téléporteurs par paires).
- [ ] Les éléments utilisés existent dans le lieu choisi (4.3).
- [ ] Le fichier se charge dans l'éditeur sans message rouge, et se joue en « Aperçu 3D ».
