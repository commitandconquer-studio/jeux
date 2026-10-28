# Le Chien de berger — cahier des charges

Univers **Croâ & Cie** (jouets en bois, pastels, animaux). Un jeu d'adresse et
de réflexion : on joue un chien de berger qui rassemble un troupeau et le rentre
dans l'enclos, sur des prés qui changent. Demandé par Olivier à partir d'une
vidéo de jeu indépendant (« il faudrait que le jeu soit plus soigné que celui-là »).

## Le décor

Un pré tondu en bandes claires et foncées, une clôture de bois tout autour, la
forêt derrière (arbres ronds et sapins). Dans le champ : **arbres, rochers,
mares** (nénuphars et roseaux) et **clôtures libres** qui font des couloirs.
L'**enclos** a un sol de paille, des bottes de foin, des poteaux de porte plus
hauts avec une lanterne dorée, et un **entonnoir** de clôtures aux premiers
niveaux pour guider les moutons vers la porte.

Les animaux sont faits de formes simples : moutons à la laine bouclée, tête
sombre et houppe ; chien de berger noir et blanc, oreilles dressées, queue qui
remue ; loups gris aux yeux rouges, cerclés d'un anneau rouge au sol pour les
repérer de loin. Les pattes balancent, le corps rebondit, la poussière vole
quand le chien sprinte. L'ambiance (`AmbiancePlugin::exterieur()`) change la
lumière, les saisons et la météo ; l'herbe et les feuillages suivent la palette
commune.

## Le jeu

- **Le but** : rentrer tous les moutons dans l'enclos (et qu'ils y restent
  1,2 s) avant la fin du temps (2,4 × le temps de référence).
- **Le chien** : flèches ou ZQSD pour courir, **Maj** pour sprinter (le souffle
  s'épuise, puis revient), **Espace** ou clic droit pour **aboyer** (une onde qui
  affole les moutons proches et fait fuir les loups ; recharge de 1,8 s). À la
  souris ou au doigt, le chien va vers le pointeur et sprinte quand il est loin.
- **Les moutons** sont des **boids** : séparation (ne pas se marcher dessus),
  alignement (aller dans le même sens que les voisins), cohésion (rester près
  du groupe), plus la **peur du chien** (rayon qui grandit avec sa vitesse) et
  des loups, l'évitement des obstacles et des clôtures. Tranquilles, ils
  broutent et font de petits pas ; affolés, ils courent presque aussi vite que
  le chien. Un mouton isolé rejoint le troupeau.
- **Les loups** (dès le niveau 5) arrivent par un bord du champ après un
  délai, visent le mouton libre le plus proche et le dévorent s'ils restent
  collés à lui 0,7 s. Les moutons de l'enclos sont à l'abri. Le chien les met
  en fuite en s'approchant à moins de 2,8 ou en aboyant ; ils reviennent plus
  tard. Au niveau 12, trois loups se relaient.
- **Les étoiles** : 1 pour réussir, 1 de plus sous le temps de référence,
  1 de plus sans avoir perdu un seul mouton.

## Le contenu

- **12 niveaux** (`berger_core::NIVEAUX`) : de 5 moutons sur un pré dégagé à
  24 moutons, obstacles, clôtures et trois loups. Les cartes de la campagne
  sont fixes (une graine par niveau) ; chaque niveau s'ouvre quand le
  précédent est réussi.
- **Le pré du jour** : une carte tirée de la date, la même pour tout le monde,
  avec la série de jours réussis.
- **Le pré au hasard** : une nouvelle carte à chaque fois, au niveau de
  difficulté atteint.

## Technique

- `crates/berger-core` (sans Bevy) : vecteur plat, générateur de cartes
  (obstacles, clôtures, enclos et entonnoir ; vérifie qu'un chemin mène à la
  porte), simulation (chien, boids, loups), et un **chien automatique**
  (`robot`) qui sert aux tests et aux captures d'écran. Tests : `cargo test -p
  berger-core` (cartes toujours accessibles, déterminisme, troupeau calme sans
  chien, fuite devant le chien, loup qui attaque, le robot gagne les premiers
  niveaux et protège le troupeau).
- `games/berger` : le jeu, sur le squelette commun (StudioPlugin, cinq langues,
  Options, ambiance, version web). `vue.rs` fabrique les modèles, `jeu.rs`
  branche la simulation sur Bevy, `ui.rs` les écrans, `audio.rs` le son.
- Sons ajoutés à la bibliothèque (`tools/sfx-gen`) : aboiement, bêlement,
  hurlement de loup.
- Captures automatiques (le robot joue) : `BERGER_CAPTURE=captures cargo run -p berger`.
  Menu animé par le robot : `BERGER_ROBOT=1 cargo run -p berger`.

## Refonte (phase A, B et C)

- **Réglages** : `berger_core::Reglages` regroupe tous les chiffres du gameplay.
  Simulation à **pas fixe de 60 Hz** (`PAS_FIXE`), indépendante de l'affichage.
- **Aboiement** : une vraie onde qui grandit ; chaque animal n'est touché qu'une
  fois par onde (`Sursaut`). Le chien se tasse et ralentit en aboyant.
- **Moutons** : états (broute, flâne, troupeau, alerte, fuit, calme, abrité) ;
  un mouton coincé est dégagé ; l'enclos ne sécurise qu'après la porte, et le
  mouton y reste. Trois sortes : **blanc**, **noir** (indépendant) et
  **agneau** (petit, rapide, peureux, collé aux adultes).
- **Bélier** (cinq phases) : vigilance, avertissement (il s'arrête, baisse la
  tête et gratte), charge, récupération. Un aboiement avant la charge l'annule ;
  s'il touche le chien, celui-ci recule et est sonné 0,7 s.
- **Loups** : trois caractères (solitaire, prudent qui contourne le chien,
  audacieux plus rapide de loin). Un loup **emporte** le mouton vers le bord :
  le chien le lui reprend en s'approchant ou en aboyant ; sorti du champ, le
  mouton est perdu.
- **Caméra** : suit le chien et le troupeau (zoom doux) ; touche C pour la fixer.
- **Niveau 4 = vitrine**, dessiné à la main : enclos à l'ouest, mare, passage
  entre deux rochers, un bélier, un loup.
