# Commit & Conquer

Studio de jeux vidéo libres et gratuits. Ce dépôt réunit tous nos jeux, écrits en Rust avec le moteur [Bevy](https://bevy.org) et publiés sous licence **GPL-3.0-or-later** (voir [LICENSE](LICENSE)).

Style maison : 3D low-poly en vue plongeante, couleurs unies, matériaux mats et ombres douces.

Chaque jeu s'ouvre sur l'écran de démarrage du studio (illustration, barre de chargement, fondu ; un clic le passe). La charte (couleurs, logo, police Russo One) est dans [packaging/branding/CHARTE.md](packaging/branding/CHARTE.md).

## Les univers

Les jeux ne sont pas reliés par une histoire, mais chacun appartient à un **univers** qui lui donne son identité (police, couleurs de l'interface, ambiance). Dans le code : `UiKitPlugin { universe: Universe::… }` (`crates/game-common/src/universe.rs`).

| Univers | Ambiance | Interface | Jeux |
|---|---|---|---|
| 🐸 **Croâ & Cie** | Enfantin et sympa : jouets en bois, pastels, animaux (le crapaud en mascotte) | Panneaux crème, police Noto Sans | Snake, Traversée, Tour de blocs, Pousse-caisses |
| 🚀 **Nova** | Science-fiction : espace profond, néons, vaisseaux | Panneaux sombres à bordure cyan, police Space Grotesk, fond spatial commun (`game_common::space`) | NOVA — Le Dernier Rempart, 12 Parsecs |
| 🔬 **Eurêka** | Sciences et maths : cahier quadrillé, feutres, laboratoire | Fiches bristol à bordure bleue, police Gelasio | 2048, Le Concile des Évêques |

## Structure

```
Cargo.toml            workspace (version, licence, Bevy partagé)
assets/               bibliothèque commune à tous les jeux : sons, musiques, police
crates/game-common/   code commun : style, caméra, interface, son, noms des sons
games/snake/          Snake en 3D low-poly
games/traversee/      Traversée : un crapaud traverse routes, rivières et voies ferrées
games/tour/           Tour de blocs : empiler des blocs le plus haut possible
games/pousse-caisses/ Pousse-caisses : le crapaud range les caisses (12 niveaux)
games/nova/           NOVA — Le Dernier Rempart : blocs d'énergie contre l'Hégémonie
games/eureka-2048/    2048 (univers Eurêka) : fusionner les puissances de 2
games/concile/        Le Concile des Évêques (univers Eurêka) : un évêque par diocèse
games/parsecs/        12 Parsecs (univers Nova) : la traversée d'une ceinture d'astéroïdes
games/modele/         le jeu modèle : point de départ de tout nouveau jeu
crates/concile-core/  grilles, solveurs et générateur du Concile (sans Bevy)
tools/concile-gen/    génère le Livre des Conciles (games/concile/puzzles.txt)
tools/logo-gen/       rend l'emblème du studio en PNG (assets/branding/)
tools/sfx-gen/        fabrique les sons et musiques de la bibliothèque
packaging/            icônes des jeux et charte du studio
```

Chaque jeu est un crate indépendant dans `games/`.

## Prérequis (Linux)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
sudo apt install build-essential pkg-config libasound2-dev libudev-dev libwayland-dev libxkbcommon-dev
```

## Jouer

```bash
cargo run -p snake
```

La première compilation de Bevy prend plusieurs minutes. Les suivantes ne recompilent que le jeu.

### Snake

- Écran d'accueil : Jouer, choix de l'univers, Options, Quitter.
- Univers : Prairie, Désert, Banquise, Océan, Volcan.
- Options : univers, vitesse (lente/normale/rapide), taille du plateau (petit/moyen/grand), bords (murs ou traversables).
- Commandes : flèches ou ZQSD pour diriger, Échap pour revenir au menu, Entrée ou Espace pour valider.
- Son : une musique par univers, croque (de plus en plus aigu), tic de virage, départ, défaite. Touche M ou bouton du menu pour couper.

Captures d'écran automatiques de tous les univers (pour itch.io, les magasins d'applications…) :

```bash
SNAKE_CAPTURE=captures cargo run -p snake
```

### Traversée

```bash
cargo run -p traversee
```

- Aidez le crapaud à aller le plus loin possible : herbe (arbres et rochers bloquent), routes (voitures, camions), rivières (sautez sur les rondins et nénuphars), voies ferrées (le feu clignote avant le train).
- La caméra avance toute seule : ne traînez pas !
- Commandes : flèches ou ZQSD, Espace ou clic pour avancer, ou glisser du doigt (tactile).
- Son : musique, saut, coassement, plouf, crash, klaxons, cloche et passage du train. Touche M ou bouton du menu pour couper.
- Captures automatiques : `TRAVERSEE_CAPTURE=captures cargo run -p traversee`.

### Tour de blocs

```bash
cargo run -p tour
```

- Empilez les blocs le plus haut possible : chaque bloc glisse d'avant en arrière, posez-le au bon moment. Ce qui dépasse est coupé et tombe.
- « Parfait ! » quand le bloc tombe pile : trois d'affilée et le bloc regrandit.
- Commandes : Espace, clic ou toucher l'écran. Échap : menu. M : son.
- Captures automatiques : `TOUR_CAPTURE=captures cargo run -p tour`.

### Pousse-caisses

```bash
cargo run -p pousse-caisses
```

- Le crapaud pousse les caisses sur les cibles. 12 niveaux de difficulté croissante, à travers les 5 univers (et leurs musiques).
- Commandes : flèches ou ZQSD, glisser du doigt ; U ou Retour arrière : annuler ; R : recommencer ; Échap : menu. Boutons à l'écran pour le tactile.
- La progression est sauvegardée (`~/.local/share/commit-and-conquer/save/`).
- Les niveaux sont dans `games/pousse-caisses/levels.txt` (format texte : `#` mur, `$` caisse, `.` cible, `@` crapaud). Un test vérifie qu'ils ont tous une solution et affiche le nombre minimal de poussées :

  ```bash
  cargo test --release -p pousse-caisses -- --nocapture
  ```
- Captures automatiques : `CAISSES_CAPTURE=captures cargo run -p pousse-caisses`.

### NOVA — Le Dernier Rempart

```bash
cargo run -p nova
```

- **L'histoire** : la colonie libre de Nova résiste à l'Hégémonie. Isa Valen, ingénieure en chef, bâtit le Bouclier de Nova bloc par bloc ; Boulon, petit drone de maintenance, commente (et tremble). Univers original Commit & Conquer.
- **Le jeu** : les blocs d'énergie tombent dans le bouclier (12 × 20). Chaque rangée complète tire le canon à ions sur le croiseur ennemi (plus de rangées d'un coup = tir plus puissant). Le croiseur riposte en ajoutant des rangées endommagées par le bas.
- **Campagne** en 5 chapitres (Nova Prime, Kessa, Hélion, Thalassa, La Forge), avec dialogues ; **mode infini** pour le record. Progression sauvegardée.
- Commandes : flèches ou ZQSD (gauche/droite déplacer, haut tourner, bas accélérer), Espace poser, C réserve, M son, Échap menu. Tactile : glisser pour déplacer, toucher pour tourner, glisser vers le bas pour poser, vers le haut pour la réserve.
- Les règles (`games/nova/src/board.rs`) ont leurs tests : `cargo test -p nova`.
- Captures automatiques (le pilote automatique joue le chapitre 1) : `NOVA_CAPTURE=captures cargo run -p nova`.

### 2048 — Eurêka

```bash
cargo run -p eureka-2048
```

- Faites glisser les tuiles : deux nombres égaux fusionnent et doublent. Atteindrez-vous 2048 ?
- Sur un bureau de scientifique : papier quadrillé, fiole, atome, crayon, règle. Tuiles 3D aux couleurs de feutres, plus épaisses pour les grands nombres ; animations de glissement, de fusion et d'apparition.
- À chaque nouvelle puissance de 2 (8, 16… 4096), une fiche « Eurêka ! » donne un petit fait scientifique.
- Commandes : flèches ou ZQSD, glisser du doigt ; U ou Retour arrière : annuler le dernier coup ; Échap : menu. Record sauvegardé.
- Règles testées (`games/eureka-2048/src/grid.rs`) : `cargo test -p eureka-2048`.
- Captures automatiques (pilote automatique) : `EUREKA_CAPTURE=captures cargo run -p eureka-2048`.

### Le Concile des Évêques — Eurêka

```bash
cargo run -p concile
```

- **La règle** : placez un évêque par diocèse (région de couleur), par rangée et par colonne. Deux évêques ne se touchent jamais, même en diagonale.
- Sur une carte des diocèses en parchemin : les évêques sont des sceaux de cire, les croix sont tracées à la plume. Clic sur une case : croix, puis évêque, puis vide ; glisser pour barrer plusieurs cases.
- **Mode Novice** : poser un évêque hachure d'office son diocèse, sa rangée, sa colonne et les cases voisines. **Mode Normal** : rien d'automatique.
- **Livre des Conciles** : 120 grilles de 5 × 5 à 10 × 10, classées par difficulté. **Défi du jour** : une grille tirée de la date, la même pour tous ; la série de jours réussis est comptée.
- **Indice** : explique la prochaine déduction logique (case seule, diocèse confiné, groupe…).
- **Calendrier des fêtes** : décorations à Noël, pendant l'Avent, à l'Épiphanie, aux Rameaux, à Pâques, à Halloween et à la Saint-Nicolas ; pendant la semaine sainte, les cloches se taisent. Option pour masquer les décorations. Pour voir une autre date : `CC_DATE=2026-12-25 cargo run -p concile`.
- Commandes : souris ou doigt ; U ou Retour arrière : annuler ; H : indice ; Échap : menu. Progression sauvegardée.
- Les grilles sont générées à l'avance (solution unique, résolubles par la logique) : `cargo run --release -p concile-gen` (environ 2 minutes). Tests : `cargo test -p concile-core`.
- Captures automatiques : `CONCILE_CAPTURE=captures cargo run -p concile`.

## Le squelette commun (obligatoire pour chaque jeu)

Tous les jeux suivent le même squelette, décrit dans [docs/SQUELETTE.md](docs/SQUELETTE.md) : `StudioPlugin` (style, écran de démarrage, interface, son, langues, Options, ambiance), un menu terminé par `pied_de_menu` (Options, Aide, Quitter, logo), l'aide du jeu dans `GameHelp`, et tous les textes en **cinq langues** (français, anglais, allemand, italien, espagnol) avec `tx!`.

Les **Options**, communes à tous les jeux : langue, son, musique, volume de la musique, volume des effets, et l'ambiance (activée ou non, moment de la journée, saison au choix, fête du moment, météo locale avec la ville du joueur).

Pour commencer un nouveau jeu :

```bash
tools/nouveau-jeu.sh mon-jeu "Mon jeu"
```

Il copie le jeu modèle (`games/modele`) et l'ajoute au workspace. Pour essayer une langue : `CC_LANGUE=de cargo run -p snake`.

## L'ambiance : le moment, la saison, les fêtes, la météo

Les jeux et le site reflètent le moment où l'on joue, avec les mêmes règles partout (`game_common::ambiance`, et `site/static/js/ambiance.js` pour le site) :

- **moment** : matin (6–10 h), journée (10–18 h), soir (18–21 h), nuit. La lumière et le ciel changent.
- **saison** : printemps (20 mars), été (21 juin), automne (22 septembre), hiver (21 décembre). Les herbes et feuillages de la palette commune changent de couleur.
- **fêtes** : le calendrier commun (`game_common::calendar`). Un message salue la fête du jour, et le site et le Concile se décorent.
- **météo** : seulement si le joueur l'accepte, sur le site (bouton nuage : géolocalisation arrondie ou ville, service Open-Meteo). Les jeux en ligne reprennent ce choix. Pluie, neige, orage et brouillard s'affichent dans les jeux en plein air.

Chaque jeu choisit ce que l'ambiance peut changer : `AmbiancePlugin::exterieur()` (Snake, Traversée, Tour de blocs, Pousse-caisses), `::interieur()` (2048, Concile : lumière du moment seulement) ou `::espace()` (NOVA : les fêtes seulement).

Pour essayer une ambiance :

```bash
CC_MOMENT=nuit CC_SAISON=hiver CC_FETE=noel CC_METEO=neige cargo run -p snake
```

Sur le site de développement (`zola serve`), le bouton « 🌦 Ambiance » force ces mêmes réglages, pour le site comme pour les jeux en ligne.

## Jouer dans le navigateur

```bash
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli --version 0.2.129   # la version de Cargo.lock
tools/build-web.sh                                  # ou : tools/build-web.sh concile
zola --root site serve
```

Les jeux compilés (`site/static/jouer/`, non versionnés) se jouent sur `/jouer/<jeu>/`. La sauvegarde utilise alors le stockage du navigateur.

En ligne, c'est l'action GitHub `.github/workflows/site.yml` qui compile les jeux et publie le site sur https://commitandconquer.fr à chaque envoi sur `main`.

### 12 Parsecs — Nova

```bash
cargo run -p parsecs
```

- Traversez les **12 parsecs** d'une ceinture d'astéroïdes : un parsec toutes les 25 secondes, de plus en plus dense, un boss tous les trois parsecs, puis le jeu continue sans fin pour le record.
- Rochers glacés (ils ralentissent), métalliques (plus durs), explosifs (ils emportent leurs voisins) ; soucoupes et drones de l'Hégémonie ; sept bonus (bouclier, tir triple, laser, missiles, ralenti, aimant, vie).
- **Le hangar** : on commence avec la Faucheuse ; le **total des points cumulés** débloque le Bourdon, l'Aiguillon et le Cargo Libre, chacun avec sa façon de voler et de tirer, et trois livrées par vaisseau.
- **Le pilote et Boulon** commentent la traversée (cinq langues). **Défi du jour** : la même traversée pour tout le monde, atteindre le parsec 4.
- Réglages : pilotage direct ou classique (tourner et accélérer), tir automatique.
- Cahier des charges : [docs/jeux/12-parsecs.md](docs/jeux/12-parsecs.md). Tests : `cargo test -p parsecs`. Captures : `PARSECS_CAPTURE=captures cargo run -p parsecs`.

## Bibliothèque commune (`assets/`)

Tous les jeux partagent le même dossier `assets/` : un son créé pour un jeu est utilisable par tous les autres.

```
assets/branding/       écran de démarrage et emblème du studio
assets/fonts/          polices : Noto Sans, Space Grotesk, Gelasio, Russo One (logo)
                       (SIL Open Font License 1.1, voir les fichiers OFL*.txt)
assets/sounds/ui/      clic, départ, carillon, défaite
assets/sounds/sfx/     saut, coassement, plouf, crash, klaxons, train, croque, tic…
assets/sounds/music/   campagne, prairie, désert, banquise, océan, volcan, zen, nova, orgue
assets/sounds/CATALOGUE.md   liste de tous les sons avec durée et description
```

Tous les sons et musiques sont **fabriqués par synthèse** par `tools/sfx-gen` (aucun fichier téléchargé, donc sous GPL comme le code). Pour ajouter ou modifier un son : écrire sa recette dans `tools/sfx-gen/src/main.rs`, ajouter son nom dans `crates/game-common/src/sounds.rs`, puis régénérer :

```bash
cargo run -p sfx-gen
```

Dans un jeu : `asset_server.load(game_common::sounds::HOP)`. Les jeux trouvent la bibliothèque grâce à `.set(game_common::shared_assets())` sur `DefaultPlugins`. Pour distribuer un jeu, copier `assets/` à côté de l'exécutable.

## Code commun

`crates/game-common` fournit à tous les jeux : palette, lumière et ombres, caméra isométrique, score, aléatoire, l'interface (`ui` : panneaux, boutons, police, survol), le son (`audio` : activé/coupé, touche M), les noms des sons (`sounds`), le crapaud (`models`), une sauvegarde simple (`save`), le calendrier et les fêtes (`calendar`, date forcée par `CC_DATE=AAAA-MM-JJ`), l'identité du studio (`brand`) et l'écran de démarrage (`splash`, désactivé par `CC_NO_SPLASH=1`).

## Le lanceur et les versions publiées (Linux)

Le lanceur (`games/lanceur`) installe, met à jour et lance tous les jeux, sans Rust ni compilation pour le joueur. Il télécharge les archives de la dernière version publiée dans les « Releases » GitHub et les installe dans `~/.local/share/commit-and-conquer`, avec une entrée dans le menu d'applications.

Pour publier une version, poser une étiquette :

```bash
git tag v0.2.0 && git push origin v0.2.0
```

L'action `.github/workflows/versions.yml` fabrique alors une archive par jeu (`tools/archives-linux.sh` : exécutable, icône, `assets/`) et le `catalogue.json` que lit le lanceur, et les publie dans une Release. Pour essayer le lanceur avec des archives fabriquées sur le PC :

```bash
tools/archives-linux.sh 0.2.0 dist
(cd dist && python3 -m http.server 8000) &
CC_DEPOT=http://localhost:8000 cargo run -p lanceur
```

## Installer les jeux depuis les sources (Linux)

```bash
tools/install-linux.sh
```

Compile les jeux en version optimisée, les copie avec la bibliothèque `assets/` dans `~/.local/share/commit-and-conquer/` et ajoute chaque jeu (avec son icône, `packaging/icons/`) au menu d'applications, catégorie Jeux. Aucun droit administrateur n'est nécessaire. Relancer le script après une modification pour mettre à jour les jeux installés.

Pour tout retirer :

```bash
tools/install-linux.sh --desinstaller
```

## Plateformes visées

Linux, Windows, macOS, Android (et web). Les builds Windows/macOS/Android seront automatisés via GitHub Actions.

## Ajouter un jeu

```bash
cargo new games/mon-jeu
```

Ajouter `"games/mon-jeu"` dans `members` du `Cargo.toml` racine, puis dans le `Cargo.toml` du jeu :

```toml
version.workspace = true
edition.workspace = true
license.workspace = true
authors.workspace = true

[dependencies]
bevy.workspace = true
game-common.workspace = true
```

Dans le code : `DefaultPlugins.set(game_common::shared_assets())` pour la bibliothèque commune, `LowPolyStylePlugin` et `iso_camera(...)` pour le style maison, `splash::SplashPlugin` pour l'écran de démarrage, `UiKitPlugin` et `AudioKitPlugin` pour les menus et le son.

## Licence

GPL-3.0-or-later. Bevy (MIT/Apache-2.0) est compatible. Pour les assets (modèles 3D, sons, polices), n'utiliser que des ressources sous licence compatible : CC0 (ex. Kenney, Quaternius), CC-BY, GPL.
