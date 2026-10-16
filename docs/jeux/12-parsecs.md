# 12 Parsecs — cahier des charges

Univers **Nova** (science-fiction). Un jeu d'astéroïdes : traverser une
ceinture de rochers de 12 parsecs, aux commandes d'un petit vaisseau, avec un
duo de commentateurs. Il respecte le squelette commun (`docs/SQUELETTE.md`) :
menus, Options, cinq langues, ambiance (`AmbiancePlugin::espace()` : les
fêtes seulement), version web.

> Statut : **validé par Olivier** le 2026-10-16 (noms des vaisseaux, livrées, paliers).

## Le principe

- Vue du dessus, en 3D low poly, sur le fond spatial commun de Nova (étoiles
  sur trois plans, nébuleuses, vaisseaux lointains).
- Le vaisseau tire sur les astéroïdes, qui se brisent en morceaux plus petits.
- **La traversée** : une jauge de distance avance pendant la partie. Chaque
  parsec franchi est une étape (plus de rochers, un boss, une réplique du duo).
  Arriver au bout des 12 parsecs est l'exploit ; ensuite le jeu continue sans
  fin pour le record.
- Un seul joueur (pas de mode à deux).

## Les vaisseaux et le hangar

- On commence avec **un seul vaisseau**. Dès le menu, le **hangar** montre les
  autres **sous cadenas** (silhouette dans l'ombre et « Débloqué à N points »).
- Les vaisseaux se gagnent avec le **total des points cumulés** sur toutes les
  parties (chaque partie rapproche du suivant). Annonce en cours de partie au
  passage d'un palier : « Nouveau vaisseau débloqué ! ».
- Vaisseaux **originaux du studio** : ils évoquent les grandes familles de la
  science-fiction sans en copier ni les noms ni les silhouettes.

| Vaisseau (noms provisoires) | Style | En jeu | Débloqué à |
|---|---|---|---|
| Faucheuse | chasseur à ailes en croix | équilibré, tir double | dès le départ |
| Bourdon | petit drone de maintenance | minuscule et agile, tir aimanté | 5 000 points cumulés |
| Aiguillon | intercepteur à cockpit rond, ailes plates | très rapide, fragile, tir rapide | 20 000 |
| Cargo Libre | vieux cargo rapiécé | lent, très solide, tir en éventail | 50 000 |

(Paliers à ajuster en jouant.)

**Livrées** : chaque vaisseau a **3 livrées** (couleurs). La première est là
dès que le vaisseau est gagné ; les deux autres se gagnent avec les points
cumulés **avec ce vaisseau-là** (2 000, puis 10 000). On les choisit au hangar.

## Le duo de commentateurs

Le **pilote** et **Boulon** (le drone de NOVA) se répondent dans une bulle, avec
une petite voix synthétisée. Plusieurs répliques par situation, tirées au
hasard. Ton drôle et familial, adapté dans chaque langue (pas traduit mot à mot).

| Situation | Exemples |
|---|---|
| tous les N points | « Ouah… on s'en sort bien ! » — « Continue comme ça, pilote ! » |
| un gros astéroïde (le « boss ») | Pilote : « Ouch, un big boss arrive… » — Boulon : « Il est moins large que mes fesses… mais il ne va pas être facile à détruire ! » |
| un rocher frôlé | « C'était moins une ! » |
| bouclier presque vide | « On prend l'eau, là ! » |
| un bonus ramassé | « Ça, c'est un cadeau ! » |
| un parsec franchi | « Parsec 5 ! Plus que 7. » |
| nouveau record | « Record battu ! » |
| nouveau vaisseau débloqué | « Regarde ce qui nous attend au hangar… » |

## Ce qui rend la partie vivante

- **Astéroïdes** : trois tailles qui se brisent ; glacés (ralentissent le
  vaisseau qui les touche), métalliques (plus durs), explosifs (détruisent
  leurs voisins), et le **boss** : un très gros rocher en plusieurs morceaux.
- **Ennemis**, de temps en temps : une soucoupe qui tire, des drones de
  l'Hégémonie.
- **Bonus** : bouclier, tir triple, laser, missiles à tête chercheuse, ralenti,
  aimant, vie supplémentaire.
- **Effets** : débris low poly qui s'éparpillent, petite secousse d'écran, sons
  synthétisés (`tools/sfx-gen`), une musique spatiale.

## Les modes

- **Première version** : Arcade (la traversée des 12 parsecs, puis sans fin) et
  **Défi du jour** (la même partie pour tous ce jour-là, tirée de la date).
- **Plus tard** : Campagne (secteurs avec objectifs : escorte, survie, station
  à détruire) et Zen (sans mort, pour se détendre ou pour les plus jeunes).

## Jouabilité

- Clavier : deux pilotages au choix dans les **Réglages** du jeu — « classique »
  (tourner et accélérer) ou « direct » (le vaisseau va où l'on pointe).
- Tactile (web, Android) : joystick virtuel et bouton de tir, ou tir automatique.
- Pause avec Échap ; le son se coupe avec M (commun).

## Sauvegarde

Total des points cumulés, vaisseaux débloqués, vaisseau choisi, record, défi du
jour réussi et série : avec `game_common::save` (fichier sur PC, navigateur sur
le web). Pas de classement en ligne (il faudrait un serveur) : plus tard.

## Décisions

- Noms des vaisseaux : Faucheuse, Bourdon, Aiguillon, Cargo Libre (gardés).
- Livrées : oui, 3 par vaisseau, gagnées avec les points faits avec ce vaisseau.
- Paliers de points : réglés par nos soins en jouant.
- Identifiant du jeu dans le dépôt : `parsecs` (`games/parsecs`).
