# À la carte ! — cahier des charges

Mini-jeux de géographie en français, en **HTML** (mobile et web). Idée
d'Olivier, novembre 2026.

> **État (15 novembre 2026) : les cinq modes codés, à tester par Olivier.**
> Dans `site/static/jeux-html/a-la-carte/` : accueil, **ville mystère**
> jouable sur les préfectures et sous-préfectures de France (4 niveaux,
> étoiles, déblocage, hors ligne) **Devine le pays** (163 silhouettes Natural Earth simplifiées par
> `tools/a-la-carte-formes.py` : normale, tournée, retournée) et **Devine le drapeau** (196 pays, 4
> niveaux : célèbres, tous, couleurs modifiées, proportions ; drapeaux
> flag-icons sous licence MIT via `tools/a-la-carte-pays.py`, noms et
> capitales Wikidata CC0), données INSEE + geo.api.gouv.fr
> (`tools/a-la-carte-villes.py`), polices Baloo 2 et Nunito embarquées,
> logo et icônes. **Capitales du monde pour la ville mystère, fiche du site : pas encore.**
> Rien n'est commité ni poussé. Les points marqués *à vérifier* le sont
> avant toute reprise de données (drapeaux, silhouettes) dans le dépôt.

## Le descriptif d'Olivier (conservé tel quel)

- **Nom** : À la carte ! **Slogan** : « Le jeu qui vous fait découvrir le
  monde. » Univers coloré, moderne, ludique, inspiré des cartes et du
  voyage.
- **Couleurs** : bleu foncé `#0B3D91` (principale), bleu vif `#1E90FF`
  (navigation, interactions), vert `#34C759` (bonnes réponses), orange
  `#FF9800` (accents, défis), crème `#FFF3D6` (fonds).
- **Typographies** : Baloo 2 (titres), Nunito (textes et interface).
- **Logo** : globe, carte dépliée, drapeau, repère de localisation.
- **Mode 1, Devine le drapeau** : drapeau classique ; QCM à quatre
  propositions ; couleurs modifiées ; proportions de couleurs (retrouver
  un drapeau uniquement par la proportion de ses couleurs, sans voir les
  formes).
- **Mode 2, Devine le pays** : silhouette normale ; silhouette tournée
  d'un angle aléatoire ; silhouette retournée ou inversée.
- **Mode 3, La ville mystère** : trois villes de référence avec leur
  distance à la ville cherchée, pour la déduire. Parties sur les
  capitales du monde, les préfectures françaises, les sous-préfectures
  françaises.
- **Niveaux de difficulté** progressifs, pour débutants et passionnés.

## Décisions prises avec Olivier

- **Forme** : un jeu **HTML / JavaScript simple, sans framework**, léger,
  pour **mobile et web**, installable comme application (PWA) et
  utilisable **hors ligne**. Pas de Bevy, pas de `game-common`.
- **Langue** : **français uniquement**, noms de villes et de pays compris.
- **Licence et lieu** : GPL-3.0 comme le reste du dépôt, dans le dépôt,
  publié sur commitandconquer.fr avec une fiche sur le site.
- **Public** : grand public, usage en classe possible ; **sans compte,
  sans publicité, sans donnée personnelle**.
- **Pas de classement en ligne** (il demanderait un serveur et des
  comptes). Les défis (par exemple « le défi du jour », calculé sans
  serveur) sont pour plus tard.
- **Profil local** : étoiles et records gardés dans le navigateur.

## Le déroulé d'une partie

- **10 questions** par partie, **sans chrono** (mode calme). Pour la ville
  mystère, une partie compte **3 villes**.
- Un mode **défi chrono** facultatif (par exemple 60 secondes) pour les
  passionnés, à décider plus tard.
- **1 à 3 étoiles** par partie : part de bonnes réponses ; pour la ville
  mystère, nombre d'essais.
- Chaque mode a **4 niveaux** : Facile, Moyen, Difficile, Expert. Un niveau
  se débloque avec **2 étoiles** au précédent.
- Séries de bonnes réponses : une petite récompense visuelle, sans
  pénalité en cas d'erreur.

### Mode 1 : Devine le drapeau

| Niveau | Épreuve |
|---|---|
| Facile | drapeau normal, QCM à 4 propositions, pays très connus |
| Moyen | drapeau normal, tous les pays, QCM |
| Difficile | couleurs modifiées (teintes décalées, ordre mélangé), QCM |
| Expert | **proportions de couleurs** seulement (une barre ou un camembert), réponse tapée ou choisie dans une liste |

À traiter : les drapeaux presque identiques (Irlande / Côte d'Ivoire,
Roumanie / Tchad, Indonésie / Monaco…). Les proportions se calculent à
partir des SVG, sans donnée supplémentaire ; les pièges sont acceptés
comme réponses équivalentes ou évités dans les propositions.

### Mode 2 : Devine le pays

| Niveau | Épreuve |
|---|---|
| Facile | silhouette normale, QCM, grands pays |
| Moyen | silhouette normale, tous les pays |
| Difficile | silhouette tournée d'un angle aléatoire |
| Expert | silhouette tournée ou retournée (miroir), réponse tapée |

Les silhouettes sont simplifiées pour rester légères (quelques centaines
d'octets par pays).

### Mode 3 : La ville mystère (« Distances »)

1. Le jeu choisit une ville (par exemple Mâcon).
2. Le joueur voit **trois villes de départ** et leur **distance** à la
   ville cherchée.
3. Il **tape le nom d'une ville** ; une **liste de suggestions** se filtre
   pendant la frappe. Accents, majuscules et tirets sont tolérés
   (« macon » = « Mâcon »).
4. S'il trouve, il gagne. Sinon, le jeu affiche la **distance entre sa
   ville et la ville cherchée** : un indice de plus, à recouper avec les
   autres.
5. **Autant d'essais que nécessaire**, avec un bouton **« Donne-moi la
   réponse »**. Les étoiles dépendent du nombre d'essais.

- **Listes** : préfectures (101), sous-préfectures (environ 230), puis
  capitales du monde. La difficulté dépend de la liste et de la notoriété
  de la ville (une ville connue est plus facile).
- **Distance** : à vol d'oiseau, sur la sphère (formule de haversine),
  arrondie au kilomètre.
- **Indices facultatifs** (niveaux faciles) : flèche de direction.
  Aucune au niveau Expert.
- Le **tracé des trois cercles de distance** sur une carte est une
  option d'affichage à étudier après la première version.

### Mode 4 : Les bonnes couleurs (ajout d'Olivier)

On donne un pays et quatre drapeaux dont les couleurs sont fausses, sauf un :
le joueur choisit celui qui a les **vraies couleurs**. L'écart de teinte des
faux drapeaux diminue avec le niveau (facile : 100 à 170°, expert : 10 à 16°,
avec un léger écart de luminosité). Le blanc, le noir et les gris ne bougent pas.

### Mode 5 : Recolorier (ajout d'Olivier)

Le drapeau d'un pays est colorié à tort ; le joueur redonne leurs couleurs à
chaque couleur du drapeau avec une palette (8 couleurs aux niveaux facile et
moyen, 13 nuances ensuite). Facile : drapeaux à 2 couleurs ; moyen : 3 ;
difficile : 4 ; expert : 3 ou 4. La bonne réponse est la couleur de la palette
la plus proche (distance CIE Lab) de la vraie couleur.

## Les données et leurs licences (*à vérifier avant reprise*)

| Donnée | Source envisagée | Conditions à noter |
|---|---|---|
| Silhouettes des pays | Natural Earth (domaine public) | aucune obligation ; citer la source par courtoisie |
| Liste des pays, noms français, capitales | Wikidata (CC0) ou une liste ISO 3166 faite à la main | vérifier les noms français usuels (« Côte d'Ivoire », « Birmanie »…) |
| Drapeaux | dépôt flag-icons (MIT) ou Wikimedia Commons (fichiers du domaine public) | **licence fichier par fichier** ; les formes sont des symboles officiels mais certains écussons détaillés peuvent être protégés ; noter la source de chacun |
| Préfectures et sous-préfectures | INSEE, code officiel géographique | licence ouverte Etalab |
| Coordonnées des villes | base officielle des communes (Géo / data.gouv.fr, Licence ouverte) | vérifier la précision (centre de la commune) |
| Capitales du monde | Natural Earth (lieux peuplés) ou Wikidata | cas particuliers : capitales multiples (Afrique du Sud, Bolivie…) |
| Polices Baloo 2 et Nunito | Google Fonts, licence **OFL** | compatible GPL ; vérifier les accents français et les symboles ; **embarquer les fichiers** (pas de lien vers Google, pour le hors ligne et la vie privée) |

## Le nom

« À la carte ! » : une recherche rapide d'applications de quiz de
géographie n'a pas trouvé d'application portant ce nom (il existe de
nombreuses applications de drapeaux et de capitales, avec d'autres noms).
L'expression est courante (restauration), donc **à vérifier plus
sérieusement** avant toute publication : recherche INPI, Google Play,
App Store, itch.io. La fiche sur commitandconquer.fr donne déjà une
adresse propre.

## Technique

- **Dossier** : `site/static/jeux-html/a-la-carte/` (versionné), avec
  `index.html`, `manifest`, `sw.js`, un fichier JS par mode, les données
  en JSON et les drapeaux en SVG. **Pas dans `site/static/jouer/`** :
  il est dans `.gitignore` (sortie de `tools/build-web.sh`, jeux WASM).
  Tous les chemins sont relatifs et le scope du service worker est limité
  à ce dossier. Confirmé par la session du site.
- **Fiche** : `site/content/jeux/a-la-carte.md` et `.en.md` (front matter
  comme `snake.md`) ; le bouton Jouer pointe vers `/jeux-html/a-la-carte/`.
  La session du site adapte `jeu.html` (on ne touche pas aux templates).
- **Images** : `site/static/img/jeux/a-la-carte.png` (icône 480²),
  `a-la-carte/*.jpg` (1920×1080), `a-la-carte-logo.png`.
- **Publication** : on ne pousse pas ; on envoie les chemins à la session
  du site, qui construit et commite.
- **Hors ligne** : service worker et manifeste PWA ; icônes.
- **Sauvegarde** : `localStorage` (étoiles, records, niveaux débloqués),
  enveloppé dans un try/catch.
- **Mobile d'abord** : gros boutons, clavier de saisie sans gêner la
  liste de suggestions, aucune interaction au survol.
- **Accessibilité** : contrastes de la charte à contrôler (le vert
  `#34C759` et l'orange `#FF9800` sur le crème sont limites pour du
  texte), texte jamais seul porteur du bon/mauvais (icône aussi).
- `tools/nouveau-jeu.sh` crée un jeu Rust/Bevy à partir de `games/modele` :
  il **ne convient pas** ici. Il faut à la place une fiche dans
  `site/content/jeux/` et `site/content/jouer/` (comme Snake), et une ligne
  dans le README.
- **Poids visé** : moins de 2 Mo hors polices.
- **Tests** : un petit jeu de tests JS sur les calculs (distance, tolérance
  de saisie, proportions de couleurs, étoiles).

## Visuels

- Le mockup d'Olivier (écran d'accueil, icône, palette, écran de jeu) sert
  de **référence graphique**. Il vient d'un générateur d'images : il
  **n'entre pas dans le dépôt** tel quel. Le logo final sera un SVG
  propre, validé par Olivier.
- Le mockup montre un bouton « Drapeaux » violet, hors charte : à garder
  ou à remplacer. Il montre aussi un onglet « Classement » : **non retenu**
  (voir les décisions).
- Le **globe qui tourne** en décor est possible en 2D (SVG ou canvas), sans
  3D.

## Ordre de réalisation proposé

1. Validation de ce cahier par Olivier ; vérification du nom.
2. Squelette de la page (charte, polices, menu, profil local, PWA).
3. **La ville mystère** (le mode le plus original) avec les préfectures.
4. **Devine le drapeau** : classique, puis couleurs modifiées, puis
   proportions.
5. **Devine le pays** : normal, tourné, retourné.
6. Sous-préfectures et capitales du monde ; niveaux et étoiles ; fiche sur
   le site.

## Décisions complémentaires

- **Défi chrono** et **défi du jour** : oui, mais après les trois modes,
  pas dans la première version.
- **Couleurs du mockup** : il ne servait qu'à tester ; la charte du
  descriptif fait foi, une couleur de la charte par mode (le vert reste
  réservé aux bonnes réponses).
- **Logo** : Olivier fournira une nouvelle version ; rien n'est dessiné
  avant.

## Question encore ouverte

Aucune sur le cahier. Reste l'accord d'Olivier pour télécharger les
données et vérifier les licences, et son nouveau logo.
