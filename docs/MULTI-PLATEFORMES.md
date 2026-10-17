# Linux, Windows, macOS, Android : une seule version — le plan

> **État : mis de côté le 17 octobre 2026, rien de développé.** Olivier a
> décidé de s'en tenir pour l'instant à **Linux et au web** ; Android, Windows
> et macOS viendront après. Ce plan reste la référence le jour où l'on s'y
> met. Décision déjà prise : les jeux deviendront des bibliothèques, pour
> permettre une appli tout-en-un. Trois points restent ouverts (§ 8).

## 1. Le principe : une version, un tag, un workflow, un catalogue

- **Un numéro de version** : celui de l'espace de travail (`Cargo.toml`,
  `version.workspace`). Tous les jeux, le lanceur et l'appli Android portent
  le même.
- **Un tag** `vX.Y.Z` déclenche **un workflow** (`versions.yml`) qui fabrique
  tout : archives Linux, Windows, macOS, APK Android, et le `catalogue.json`.
- **Un catalogue** lu par le lanceur, qui y choisit l'entrée de sa plateforme.
- **Un seul code** par jeu : les différences de plateforme vivent dans
  `game-common` et dans un module `plateforme.rs` du lanceur, jamais dans les
  jeux.

Ce qui existe déjà et reste : le lanceur Bevy, `archives-linux.sh` (qui
devient `archives.sh`), `versions.yml`, la version web (`build-web.sh`).

## 2. Pourquoi des bibliothèques : l'appli tout-en-un

Sur Linux, Windows et macOS, le lanceur télécharge et lance des exécutables
séparés. Sur Android, c'est impossible (une appli ne lance pas d'autres
exécutables, et un APK par jeu est rédhibitoire). Il faut **une appli unique
qui embarque tous les jeux**, avec le menu du lanceur en écran d'accueil.

Dans une appli Bevy, on ne peut pas non plus enchaîner deux `App` (une seule
boucle d'événements par activité Android). Les jeux doivent donc tourner
**dans la même `App`**, à tour de rôle. D'où la structure suivante.

### 2.1 Dans `game-common`

- **`JeuActif`**, un état global (`States`) : `Lanceur`, `Snake`, `Traversee`,
  `Tour`, `PousseCaisses`, `Nova`, `Eureka2048`, `Concile`, `Parsecs`,
  `Observatoire`, (`CaFuit`…). Un jeu seul démarre directement dans son
  propre état ; le tout-en-un démarre dans `Lanceur`.
- **`UniversActif`** et **l'ambiance active** deviennent des ressources
  changées à l'entrée d'un jeu, au lieu d'être figées dans
  `StudioPlugin::new(univers, ambiance)`. `UiKitPlugin` (couleurs, pied de
  menu, aide) et `AmbiancePlugin` (extérieur, intérieur, espace) lisent ces
  ressources. `StudioPlugin::new(univers, ambiance)` reste, pour un jeu seul,
  et ne fait plus que poser les valeurs initiales.
- **`lancer_seul(jeu, titre, plugin)`** : les quinze lignes de `main.rs`
  d'aujourd'hui (DefaultPlugins, fenêtre, assets, StudioPlugin,
  `rebatir_menu`), en une fonction. Chaque `main.rs` devient trois lignes.
- **Un `Jeu` décrit par ses métadonnées** (`id`, nom, description, univers,
  icône) : la table `JEUX` de `games/lanceur/src/catalogue.rs` migre dans
  `game-common`, pour que le jeu seul, le lanceur et le tout-en-un partagent
  la même liste. `tools/install-linux.sh` et `archives.sh` gardent la leur
  (bash), vérifiée égale par un test.
- Le pied de menu : « Quitter » devient « Retour au lanceur » quand
  `JeuActif` n'est pas celui du démarrage (tout-en-un), et reste caché sur
  web et Android.

### 2.2 Dans chaque jeu (`games/<jeu>`)

- `src/lib.rs` expose **`pub struct JeuPlugin`** qui ajoute `play`, `ui`,
  `audio`, `capture`, et `pub const JEU: JeuActif`.
- `GameState` (Menu, Playing…) devient un **`SubStates`** de
  `JeuActif::<Jeu>` : il n'existe que quand le jeu est actif, et Bevy le remet
  à `Menu` à chaque entrée.
- Tout ce qui est en `Startup` (caméra, musique, décor fixe) passe en
  `OnEnter(JeuActif::<Jeu>)`, avec `DespawnOnExit(JeuActif::<Jeu>)` sur les
  entités. Les systèmes `Update` sont conditionnés par
  `in_state(JeuActif::<Jeu>)` (une seule ligne : `configure_sets`).
- La musique : lancée à l'entrée, arrêtée à la sortie (`Musique` est déjà un
  marqueur commun, `AudioKitPlugin` peut la couper pour tous).
- `src/main.rs` : `fn main() { game_common::lancer_seul(JEU, "Titre — Commit & Conquer", JeuPlugin) }`.
- Le mode capture (`<JEU>_CAPTURE`) ne change pas.

Chaque jeu seul doit se comporter **exactement comme avant** : c'est le
critère de validation de cette étape, jeu par jeu, avec une capture.

### 2.3 Le lanceur à deux modes (`games/lanceur`)

- **Mode catalogue** (bureau, par défaut) : télécharge, installe, met à jour,
  lance des exécutables séparés. C'est l'existant.
- **Mode tout-en-un** (feature Cargo `tout-en-un`, obligatoire sur Android,
  possible sur bureau) : dépend de tous les jeux comme bibliothèques, ajoute
  tous leurs `JeuPlugin`, et « Jouer » change `JeuActif` au lieu de lancer un
  processus. Pas de téléchargement, pas de mises à jour par jeu : la mise à
  jour, c'est l'appli entière.
- L'écran est le même dans les deux modes (grille par univers) ; seuls les
  boutons diffèrent (Installer / Mettre à jour n'existent qu'en catalogue).

## 3. `plateforme.rs` : ce qui diffère par système

Un seul module dans le lanceur, derrière `cfg(target_os)` :

| | Linux | Windows | macOS |
|---|---|---|---|
| identifiant du catalogue | `linux-x86_64` | `windows-x86_64` | `macos-universal` (ou `macos-aarch64`, § 8) |
| archive | `.tar.gz` | `.zip` | `.tar.gz` contenant `<Jeu>.app` |
| dossier d'installation | `$XDG_DATA_HOME/commit-and-conquer` | `%LOCALAPPDATA%\Commit & Conquer` | `~/Library/Application Support/Commit & Conquer` |
| raccourci | `.desktop` + `update-desktop-database` | `.lnk` dans le menu Démarrer (crate `mslnk`) | lien du `.app` dans `~/Applications` |
| exécutable | `<jeu>` | `<jeu>.exe` | `<Jeu>.app/Contents/MacOS/<jeu>` |
| première ouverture | rien | avertissement SmartScreen (§ 8) | Gatekeeper : clic droit → Ouvrir (§ 8) |

Nouvelles dépendances du lanceur : `zip` (Windows), `mslnk` (Windows),
`dirs` (les trois dossiers). `ureq`, `flate2`, `tar`, `sha2` restent.

### Catalogue, version 2

```json
{
  "version": "0.2.0",
  "jeux": {
    "snake": {
      "linux-x86_64":   { "fichier": "snake-linux-x86_64.tar.gz",   "taille": 12345678, "sha256": "…" },
      "windows-x86_64": { "fichier": "snake-windows-x86_64.zip",    "taille": 13456789, "sha256": "…" },
      "macos-universal":{ "fichier": "snake-macos-universal.tar.gz","taille": 23456789, "sha256": "…" }
    }
  },
  "android": { "fichier": "commit-and-conquer.apk", "taille": 98765432, "sha256": "…" }
}
```

Le lanceur d'aujourd'hui (version 1 du catalogue) lit `jeux.<id>.{fichier,…}`
à plat ; il faudra garder cette forme **en plus** le temps d'une version,
sinon les lanceurs déjà installés ne sauront plus se mettre à jour.

## 4. Fabrication : `archives.sh` et le workflow

- `tools/archives-linux.sh` devient `tools/archives.sh <version> <dossier> <plateforme>`,
  même logique : compile, une archive par jeu + lanceur, fragment de
  catalogue. Sur Windows il tourne sous Git Bash (fourni par le runner).
  macOS : fabrique le `.app` (Info.plist dans `packaging/macos/`, icône
  `.icns` générée par `logo-gen` ou `iconutil`). Universel : deux
  compilations (`x86_64-apple-darwin`, `aarch64-apple-darwin`) et `lipo`.
- `tools/catalogue.sh` assemble les fragments des quatre jobs en un
  `catalogue.json`.
- `versions.yml` : une **matrice** `ubuntu-22.04`, `windows-latest`,
  `macos-latest`, plus un job `android` (voir § 5), puis un job
  `publication` qui attend les quatre, assemble le catalogue et crée la
  Release. Les artefacts transitent par `upload-artifact`.
- Temps de CI : ~15 min par OS avec le cache ; Android ~20 min.

## 5. Android

- `packaging/android/` : un projet Gradle minimal (celui de l'exemple
  `mobile` de Bevy), qui charge la bibliothèque `liblanceur.so`.
- Le lanceur en `crate-type = ["cdylib", "rlib"]`, point d'entrée
  `#[bevy_main]`, feature `tout-en-un` activée.
- Compilation : `cargo ndk -t arm64-v8a build --release` (crate `cargo-ndk`,
  NDK installé par `android-actions/setup-android`). Un seul ABI (arm64)
  suffit en 2026.
- Signature : clé de **débogage** générée dans le job pour un APK GitHub ;
  clé **de publication** (secret GitHub) et format AAB si Play Store (§ 8).
- Sauvegardes : `game_common::save` a déjà deux voies (fichier, web) ; sur
  Android le dossier vient de `dirs::data_dir()` (ou du `ndk-context`).
- **Le vrai travail Android est le tactile.** Les jeux sont pensés clavier et
  souris. Il faut, dans `game-common`, des commandes tactiles communes
  (glisser = direction, toucher = action, deux doigts = zoom) et vérifier jeu
  par jeu : Snake et Traversée (directions), Tour de blocs (un toucher),
  Pousse-caisses (directions), Nova et 12 Parsecs (viser, tirer), les jeux de
  grille (toucher = clic, déjà le cas sur le web). Cette vérification est
  une étape à part entière (§ 7, étape 7).
- Aussi : le bouton « Retour » Android = retour au lanceur ; mise en pause
  quand l'appli passe en arrière-plan (musique coupée) ; orientation
  paysage imposée dans le manifeste.

## 6. Site, README, squelette

- `site/content/installer.md` / `.en.md` : un bouton par plateforme (Linux,
  Windows, macOS, Android), avec la phrase sur SmartScreen et Gatekeeper.
- `README.md` : la section Installation, et la liste des plateformes.
- `docs/SQUELETTE.md` et `tools/nouveau-jeu.sh` : `lib.rs` + `main.rs` de
  trois lignes, `JeuPlugin`, `SubStates`, `OnEnter` au lieu de `Startup`.
- `tools/install-linux.sh` et `build-web.sh` : inchangés (le web compile le
  binaire du jeu seul, comme avant).

## 7. L'ordre des étapes

Chaque étape est fusionnable seule, sans rien casser pour le joueur.

| # | Étape | Fichiers | Validation |
|---|---|---|---|
| 1 | `game-common` : `JeuActif`, `UniversActif`, ambiance dynamique, `lancer_seul`, liste `JEUX` | `crates/game-common/src/{lib,ui,ambiance,universe,jeux}.rs` | tous les jeux compilent et se comportent comme avant |
| 2 | Le jeu modèle en bibliothèque, squelette et script mis à jour | `games/modele/src/{lib,main}.rs`, `docs/SQUELETTE.md`, `tools/nouveau-jeu.sh` | capture du modèle identique |
| 3 | Les neuf jeux publiés convertis, un commit par jeu | `games/*/src/{lib,main,play,ui,audio}.rs` | une capture par jeu, version web rebâtie |
| 4 | Le lanceur tout-en-un sur Linux (feature `tout-en-un`) | `games/lanceur/{Cargo.toml,src/main.rs,src/ui.rs}` | on enchaîne trois jeux sans quitter, musique et caméra propres |
| 5 | `plateforme.rs`, `archives.sh`, catalogue v2, matrice Windows + macOS | `games/lanceur/src/{plateforme,installation}.rs`, `tools/archives.sh`, `tools/catalogue.sh`, `.github/workflows/versions.yml`, `packaging/macos/` | `workflow_dispatch` : les trois archives s'installent et se lancent (Windows et macOS testés à la main par Olivier ou une machine virtuelle) |
| 6 | Android : projet Gradle, `cdylib`, job CI, APK de débogage | `packaging/android/`, `games/lanceur/Cargo.toml`, `versions.yml` | l'APK s'installe et affiche le lanceur sur un téléphone |
| 7 | Tactile commun et passage de chaque jeu au doigt | `crates/game-common/src/tactile.rs`, `games/*/src/play.rs` | chaque jeu jouable au doigt |
| 8 | Site, README, mise à jour | `site/content/installer*.md`, `site/content/mises-a-jour/`, `README.md` | pages construites avec zola |
| 9 | Tag `v0.2.0` | — | Release avec les quatre plateformes, lanceur v1 toujours capable de se mettre à jour |

Les étapes 1 à 4 ne changent rien pour le joueur Linux et se font d'abord ;
5 et 6 sont indépendantes l'une de l'autre ; 7 est la plus longue.

## 8. Points ouverts

| Point | Choix | Ce que ça change dans le plan |
|---|---|---|
| **Android : GitHub seul, ou Play Store ?** | *Recommandé : GitHub seul d'abord.* | Play Store ajoute : compte développeur (25 $ une fois), AAB signé avec une clé gardée en secret GitHub, page de politique de confidentialité sur le site, fiche avec captures, revue de quelques jours, `targetSdk` récent et pages mémoire 16 Ko. Une étape 10 à part, qui ne bloque pas le reste. |
| **macOS : compte développeur Apple ?** | *Recommandé : sans, pour commencer ; Apple Silicon seul.* | Sans compte (99 $/an) : pas de signature ni notarisation, le site explique « clic droit → Ouvrir ». Avec : étape 5 gagne la signature et la notarisation dans le job macOS (`codesign`, `notarytool`, secrets GitHub). Universel = deux fois le temps de compilation et des archives deux fois plus grosses ; les Mac Intel ont dix ans. |
| **Windows : certificat de signature ?** | *Recommandé : sans.* | Un certificat (300 à 500 €/an, ou gratuit via SignPath pour les projets libres, sur dossier) supprime l'avertissement SmartScreen. Sans, le site explique « Informations complémentaires → Exécuter quand même ». Ajoutable plus tard sans rien changer d'autre. |

Dès que ces trois points sont tranchés, le plan est complet et l'étape 1 peut
commencer.
