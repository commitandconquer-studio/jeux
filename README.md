# Commit & Conquer

Studio de jeux vidéo libres et gratuits. Ce dépôt réunit tous nos jeux, écrits en Rust avec le moteur [Bevy](https://bevy.org) et publiés sous licence **GPL-3.0-or-later** (voir [LICENSE](LICENSE)).

Style maison : 3D low-poly en vue plongeante, couleurs unies, matériaux mats et ombres douces.

## Les univers

Les jeux ne sont pas reliés par une histoire, mais chacun appartient à un **univers** qui lui donne son identité (police, couleurs de l'interface, ambiance). Dans le code : `UiKitPlugin { universe: Universe::… }` (`crates/game-common/src/universe.rs`).

| Univers | Ambiance | Interface | Jeux |
|---|---|---|---|
| 🐸 **Croâ & Cie** | Enfantin et sympa : jouets en bois, pastels, animaux (le crapaud en mascotte) | Panneaux crème, police Noto Sans | Snake, Traversée, Tour de blocs, Pousse-caisses |
| 🚀 **Nova** | Science-fiction : espace profond, néons, vaisseaux | Panneaux sombres à bordure cyan, police Space Grotesk, fond spatial commun (`game_common::space`) | NOVA — Le Dernier Rempart |
| 🔬 **Eurêka** | Sciences et maths : cahier quadrillé, feutres, laboratoire | Fiches bristol à bordure bleue, police Gelasio | 2048 |

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
tools/sfx-gen/        fabrique les sons et musiques de la bibliothèque
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

## Bibliothèque commune (`assets/`)

Tous les jeux partagent le même dossier `assets/` : un son créé pour un jeu est utilisable par tous les autres.

```
assets/fonts/          polices des univers : Noto Sans, Space Grotesk, Gelasio
                       (SIL Open Font License 1.1, voir les fichiers OFL*.txt)
assets/sounds/ui/      clic, départ, carillon, défaite
assets/sounds/sfx/     saut, coassement, plouf, crash, klaxons, train, croque, tic…
assets/sounds/music/   campagne, prairie, désert, banquise, océan, volcan
assets/sounds/CATALOGUE.md   liste de tous les sons avec durée et description
```

Tous les sons et musiques sont **fabriqués par synthèse** par `tools/sfx-gen` (aucun fichier téléchargé, donc sous GPL comme le code). Pour ajouter ou modifier un son : écrire sa recette dans `tools/sfx-gen/src/main.rs`, ajouter son nom dans `crates/game-common/src/sounds.rs`, puis régénérer :

```bash
cargo run -p sfx-gen
```

Dans un jeu : `asset_server.load(game_common::sounds::HOP)`. Les jeux trouvent la bibliothèque grâce à `.set(game_common::shared_assets())` sur `DefaultPlugins`. Pour distribuer un jeu, copier `assets/` à côté de l'exécutable.

## Code commun

`crates/game-common` fournit à tous les jeux : palette, lumière et ombres, caméra isométrique, score, aléatoire, l'interface (`ui` : panneaux, boutons, police, survol), le son (`audio` : activé/coupé, touche M), les noms des sons (`sounds`), le crapaud (`models`) et une sauvegarde simple (`save`).

## Installer les jeux sur son PC (Linux)

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

Dans le code : `DefaultPlugins.set(game_common::shared_assets())` pour la bibliothèque commune, `LowPolyStylePlugin` et `iso_camera(...)` pour le style maison, `UiKitPlugin` et `AudioKitPlugin` pour les menus et le son.

## Licence

GPL-3.0-or-later. Bevy (MIT/Apache-2.0) est compatible. Pour les assets (modèles 3D, sons, polices), n'utiliser que des ressources sous licence compatible : CC0 (ex. Kenney, Quaternius), CC-BY, GPL.
