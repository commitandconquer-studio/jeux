# Cross or Die : progression et nouveautés (proposition, pas développée)

Notes du 28 octobre 2026. Rien n'est codé. Complète `cross-or-die-livraison.md`.

## 1. Deux axes

| | Dans la partie (série) | Cumulatif (toutes parties) |
|---|---|---|
| Question | Qu'est-ce qui apparaît maintenant ? | Qu'est-ce qui existe dans mon jeu ? |
| Contrôle | Les **types** de danger et leur intensité | Les **variantes** (modèles, personnages, mondes) |
| Remise à zéro | À chaque partie | Jamais |

Règle de liaison : un modèle débloqué en cumulatif n'apparaît que sur un type de route déjà introduit dans la partie.

## 2. Dans la partie : l'ordre d'apparition (valeurs d'Olivier)

| Rangées / score de la série | Ce qui arrive |
|---|---|
| 0 à 12 | Herbe, arbres, rochers. Aucun danger. |
| 13 | **Première route** : une voie, un seul modèle (Pépette), lente. |
| 20 | **Premiers nénuphars** (rivière facile). |
| 30 | **Premiers troncs**. |
| 50 | Les autres modèles de voitures (selon le cumul) peuvent apparaître. |

À placer (proposé, à confirmer) :
- Première voie ferrée : à la **100e ligne** (valeur d'Olivier), après la route à deux voies, avec l'annonce « Ce train ne fait pas de livraison ».
- Colis : vers la ligne **25** (valeur d'Olivier ; aujourd'hui dès la rangée 6).
- Densité, vitesse, rivières mixtes : montée douce de 75 à 300.
- Premier changement de monde : **double condition** (Olivier). 1) avoir débloqué pas mal de petites choses (permission, en cumulé) ; 2) atteindre **200** dans une seule série (déclencheur). Sans la 1re, la série reste en Prairie même à 200.

Un joueur expérimenté rejoue la même montée : elle est courte, c'est le rythme du jeu.

## 3. Cumulatif

- Petite récompense tous les **250** points cumulés (modèle de voiture, accessoire, cadeau doré).
- Grosse récompense tous les **1000** (monde, personnage rare, mécanique).
- **Aigle : 2000 à 3000 points cumulés (2000 proposé), et seulement si d'autres nouveautés sont déjà débloquées** (proposé : le premier monde). Valeur d'Olivier : « mi-parcours, après une dizaine de parties ». Une fois débloqué, il apparaît dans la série à partir du score **250** (valeur d'Olivier : assez d'habitude, mais le personnage traîne parfois).
- Dès le départ : Pépette, un personnage, la Prairie, les nénuphars.
- Ordre proposé ensuite : camion (≈ 250), lapin, taxi, moto, bus, premier monde (≈ 1000).

## 4. Annonces des nouveautés

1. **Pop-up « Nouveau modèle gagné ! »** quand le seuil est franchi, en pleine partie : carte avec miniature et nom, environ 3 s, au centre en haut de l'écran (≈ 8 % de la hauteur, au-dessus des bandeaux d'annonce qui sont à 22 %), une à la fois. Décidé : en pleine partie (Olivier : plus de dynamisme). Récapitulatif sur l'écran Fin.
2. **Bandeau « Tiens, je ne l'avais jamais vue »** la première fois que le modèle apparaît à l'écran (variantes de phrase tirées au hasard). Nécessite une liste « vus » sauvegardée, distincte des déblocages. Pour les véhicules, les mondes et les dangers (train, aigle) ; pas pour les personnages.
3. Possible : compteur « 7 / 24 vus » dans la Collection.

## 5. À décider

- Score de la série ou distance pour les paliers ? (recommandé : score)
- Premier monde : permission au bout de combien de petites récompenses (proposé : 4, soit ≈ 1000 cumulés, en même temps que l'aigle ; ou 5 pour les décaler) ? Les mondes suivants : même principe, un par tranche de 1000 + score de série plus haut ?
- Pop-up de déblocage : en pleine partie (décidé). Cliquable ou non (recommandé : non) ?

## 6. Bonus de points : de la nourriture à la place des « cadeaux »

Décision d'Olivier (28 octobre) : les cadeaux ramassés pour gagner des points font doublon avec les colis. On garde les **colis** (livraison) et on remplace les cadeaux par de la **nourriture** : fruits, légumes, pizzas, burgers, etc.

- Halo lumineux autour de chaque bonus, pour qu'on comprenne qu'il rapporte des points.
- Rotation lente de tout ce qui rapporte des points (nourriture, et sans doute les colis).
- Les paliers « Flies » / « GoldFlies » actuels (cadeaux, cadeaux dorés) deviennent : nourriture ordinaire, nourriture dorée (plus de points).
- Modèles : aucun dans les kits Kenney du dépôt (seulement `coaster-kit/stall-food.glb`, un stand). Piste : le Food Kit de Kenney (CC0) à importer, ou des formes simples faites à la main. À décider.
- Choisir : fruit/légume par monde (thème) ou mélange partout.

Food Kit de Kenney (CC0) trouvé dans `~/Téléchargements/kenney_food-kit.zip` : 200 modèles .glb (pomme, carotte, burger, pizza, banane, pastèque, maki, taco, donut, citrouille…). Pas encore importé dans `assets/kenney/`.

## 7. État du code (28 octobre, à tester)

Dans la partie (`lanes.rs`, constantes `PREMIERE_ROUTE`, etc.) : herbe jusqu'à la ligne 12, première route en 13 (une voie, toujours suivie d'herbe, Pépette seule), rivière à nénuphars en 20, troncs en 30, route à deux voies en 50 (les modèles du cumul apparaissent), voie ferrée en 100, premier monde en 200, aigle possible en 250 (`PREMIER_MONDE`, `PREMIER_AIGLE`). Un test (`cargo test -p traversee lanes::tests`) vérifie les paliers sur 200 graines.

Cumulatif (`progress.rs`, valeurs finales, sans étirement) : nouveauté tous les 250 points : camion 250, lapin 500, taxi 750, moto 1000, **Désert 1250 (premier monde)**, bus 1500, blaireau 1750, **aigle 2000**, cadeaux dorés 2250, pick-up 2500, raton 2750, neige 3000, etc. Les trains ne sont plus bloqués par le cumul (seulement par la ligne 100).

Pas encore fait : pop-ups « Nouveau modèle gagné » et « jamais vu », liste des modèles vus, compteur.

## 8. Plafond de rangées d'un même type d'affilée (29 octobre, valeur d'Olivier)

Un MAXIMUM, pas un nombre fixe : chaque série (routes, rivières, voies ferrées) tire un nombre au hasard entre 1 et le maximum, et deux séries du même type bout à bout ne dépassent pas ce maximum. Route puis rivière (types différents) reste permis.

| Ligne | Maximum d'affilée |
|---|---|
| avant 100 | 1 (une seule route, une seule rivière, une seule voie ferrée à la fois) |
| 100 à 199 | 2 |
| 200 à 299 | 3 |
| 300 et plus | 4 (jamais plus) |

Règle simplifiée par Olivier (29 octobre) : un de plus tous les 100 points. Remplace la route à deux voies à 50. Tests : `cargo test -p traversee lanes::tests`.
