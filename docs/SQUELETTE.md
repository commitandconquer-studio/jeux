# Le squelette d'un jeu Commit & Conquer

Tous les jeux du studio partagent le même squelette. **Un nouveau jeu le
respecte obligatoirement** : c'est ce qui garantit que chaque jeu a les mêmes
menus, les mêmes Options, les cinq langues, l'ambiance, le son réglable et une
version web. Point de départ : copier le jeu modèle (`tools/nouveau-jeu.sh`).

## 1. `main.rs` : une ligne pour tout le commun

```rust
App::new()
    .add_plugins(
        DefaultPlugins
            .set(WindowPlugin { primary_window: Some(game_common::fenetre("Mon jeu — Commit & Conquer")), ..default() })
            .set(game_common::shared_assets()),
    )
    .init_state::<GameState>()
    .add_plugins(StudioPlugin::new(Universe::Croa, AmbiancePlugin::exterieur()))
    .add_plugins((play::PlayPlugin, ui::UiPlugin, audio::AudioPlugin, capture::CapturePlugin))
    .add_systems(Update, game_common::langue::rebatir_menu(GameState::Menu))
    .run();
```

`StudioPlugin` installe : langues, style low poly, écran de démarrage,
interface de l'univers, son, ambiance et Options. **Ne jamais ajouter à part**
`LowPolyStylePlugin`, `SplashPlugin`, `UiKitPlugin`, `AudioKitPlugin`,
`AmbiancePlugin` ou `LanguePlugin` (Bevy refuse les doublons).

L'ambiance selon le lieu du jeu : `AmbiancePlugin::exterieur()` (plein air :
lumière, saisons, pluie, neige), `::interieur()` (lumière du moment), `::espace()`
(seulement les fêtes).

## 2. Le menu : les boutons du jeu, puis le pied commun

```rust
fn spawn_menu(mut commands: Commands) {
    let panel = spawn_panel(&mut commands, DespawnOnExit(GameState::Menu));
    spawn_text(&mut commands, panel, tx!("MON JEU", "MY GAME", "MEIN SPIEL", "IL MIO GIOCO", "MI JUEGO").t(), 60.0);
    button(&mut commands, panel, tx!("Jouer", "Play", "Spielen", "Gioca", "Jugar").t(), UiAction::Play);
    pied_de_menu(&mut commands, panel); // Options, Aide, Quitter (sauf web/Android), logo
}
```

- Pas de bouton « Son » ni « Quitter » à la main : ils sont dans le pied et dans
  les Options (langue, son, musique et son volume, volume des effets, ambiance).
- Pas de texte d'aide ni de rappel des commandes dans le menu : tout va dans
  l'aide du jeu, `GameHelp(tx!(…))`, insérée par le plugin d'interface.
- Les réglages propres au jeu (vitesse, taille du plateau…) restent dans un
  bouton du jeu, nommé « Réglages » (`tx!("Réglages", "Settings", "Einstellungen", "Impostazioni", "Ajustes")`).

## 3. Les textes : toujours dans les cinq langues

Tout texte visible passe par `tx!(français, anglais, allemand, italien, espagnol)` :

```rust
let t = tx!("Jouer", "Play", "Spielen", "Gioca", "Jugar").t();                       // &'static str
let n = tx!("Niveau {}", "Level {}", "Level {}", "Livello {}", "Nivel {}").avec(&[&3]); // String
```

Aucune chaîne française en dur dans l'interface. Les données longues (dialogues,
anecdotes…) sont des tableaux de `Texte`. La langue change dans les Options :
le menu est rebâti (`rebatir_menu`), donc les textes sont lus au moment où ils
sont créés.

## 4. Le son

- La musique : `(AudioPlayer, PlaybackSettings::LOOP…, Musique, Mutable)` ;
  son volume suit « Volume de la musique ».
- Les effets : ne pas les jouer si `SoundOn` est faux ; leur volume suit
  « Volume des effets » tout seul. Un son long en boucle qui n'est pas une
  musique est seulement `Mutable`.
- La touche M coupe le son dans tous les jeux.

## 5. Ce qui doit marcher partout (PC, navigateur, Android)

- Pas de `std::time::SystemTime`/`Instant` (prendre `web_time`, `Time` de Bevy,
  ou `game_common::Rng::from_time`), pas de `std::env::vars()`.
- Sauvegardes : `game_common::save::{load, store}` (fichiers sur PC, stockage
  du navigateur sur le web).
- Herbes et feuillages en couleurs de `game_common::palette` (`GRASS`,
  `GRASS_LIGHT`, `FOLIAGE`, `FOLIAGE_DARK`) : les saisons les repeignent.
- Mode capture : `<JEU>_CAPTURE=dossier` enregistre les écrans clés puis quitte
  (voir les jeux existants). Les variables `CC_LANGUE`, `CC_MOMENT`,
  `CC_SAISON`, `CC_FETE`, `CC_METEO`, `CC_OPEN_HELP`, `CC_OPEN_OPTIONS`,
  `CC_OPEN_AMBIANCE` aident à vérifier.

## 6. Pour publier

- `tools/install-linux.sh` : une ligne dans `GAMES` et une icône
  `packaging/icons/<jeu>.png`.
- Le lanceur et les versions publiées : une entrée dans `JEUX` de
  `games/lanceur/src/catalogue.rs` et le nom du paquet dans `JEUX` de
  `tools/archives-linux.sh`.
- `tools/build-web.sh` : une entrée `page|paquet` dans `JEUX`.
- Le site : `site/content/jeux/<page>.md` et `.en.md` (fiche, `web = true`),
  `site/content/jouer/<page>.md` et `.en.md`.
- Le README : une section du jeu.
