# Traversée : idées notées (pas encore faites)

Notes du 26 octobre 2026, à reprendre une par une. Rien ici n'est développé.

## 1. Une autre vue dans la Collection

- Le jeu garde sa vue isométrique (elle est très bien pour jouer).
- Dans la **Collection** (ennemis, décors, personnages), la caméra ne doit plus être isométrique : plus **à l'horizontale**, avec une **petite inclinaison** (pas 100 % à plat), pour qu'on voie le profil des voitures et des personnages.
- Les modèles **tournent sur eux-mêmes** (c'est déjà le cas, à garder).
- Côté code : la caméra de la Collection est déjà à part (`garage.rs`, position autour de `ORIGIN`, réglée dans `main.rs`) ; il suffit d'une autre pose (hauteur plus basse, angle de 15 à 25°). À prévoir : les plateaux et les textes sous les modèles se recadrent avec le nouvel angle.

## 2. Les 12 personnages

- Le Mini Characters de Kenney en contient **12** (`character-male-a` à `-f`, `character-female-a` à `-f`). Le jeu n'en utilise que **8** (Léo, Lola, Gaston, Zoé, Hugo, Jade, Théo, Mila).
- Objectif : **les 12**. Il faut 4 personnages de plus (enum `Character`, seuils de déblocage dans `progress.rs`, noms).

## 3. Les chaises roulantes

- Le kit propose **4 modèles** de fauteuils : `wheelchair`, `wheelchair-deluxe`, `wheelchair-power`, `wheelchair-power-deluxe`. Les personnages ont déjà des animations « assis en fauteuil » (`wheelchair-sit`, `wheelchair-move-forward/back/left/right`, `wheelchair-look-left/right`).
- Souhait : pouvoir jouer **chacun des 12 personnages en fauteuil**, avec un fauteuil « différent » pour chacun. Remarque : il n'y a que 4 modèles ; pour 12 variantes différentes, il faudra varier la couleur (teinte) ou combiner modèle de fauteuil et personnage.

## 4. Mobilité qui évolue (idée à réfléchir)

L'idée : des personnages âgés qui **marchent difficilement** au début, puis on gagne une **canne** (un peu plus rapide), puis un **fauteuil roulant** (encore un peu plus rapide).

Mon avis (à discuter) :
- **Bonne idée pour l'inclusion**, et elle donne un vrai sens aux déblocages (on progresse en autonomie, pas seulement en points).
- **Attention au ton** : éviter que le handicap soit une pénalité. Mieux vaut que le fauteuil soit **un plus** (le plus rapide, le plus stable), et que les personnages ne soient pas « lents par nature ».
- **Choix du joueur** : ne pas forcer un parcours ; le joueur choisit son personnage et son équipement.
- **Côté jeu** : la vitesse se règle par la durée du saut (aujourd'hui 0,14 s) et par la reprise après l'atterrissage. Un fauteuil roule au lieu de sauter : animation `wheelchair-move-forward` et déplacement sans arc de saut.
- **Accessoires** : le kit contient aussi des aides (`aid-cane`, `aid-cane-blind`, `aid-cane-low-vision`, `aid-crutch`, `aid-glasses`, `aid-sunglasses`, `aid_hearing`, `aid-mask`, défibrillateurs). Elles peuvent redonner un sens à l'onglet « accessoires » supprimé de la Collection, à la place des chapeaux.
- **À équilibrer** : si la vitesse change réellement le jeu, les scores ne sont plus comparables d'un personnage à l'autre (classement, record). Une piste : la vitesse reste la même, et les aides donnent d'autres avantages (par exemple une tolérance de saut) ou sont purement visuelles.

## Autres restes (rappel)

- Mondes retirés à refaire : Jurassic, Nippon, Far West, ville futuriste, désert, neige, automne.
- Rochers et touffes d'herbe de la prairie encore en modèles du jeu (couleurs fausses avec Kenney).
- Les captures de la fiche du site sont à refaire quand le rendu change.
