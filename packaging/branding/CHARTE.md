# Charte du studio Commit & Conquer

Kit de marque conceptuel (voir `LIRE_MOI.txt`).

- **Écran de démarrage** de tous les jeux (`game_common::splash`) :
  l'illustration 16:9 `assets/branding/splash.png`, recadrée à la fenêtre, avec
  une vraie barre de chargement. `CC_NO_SPLASH=1` le désactive ;
  `SPLASH_CAPTURE=fichier.png` en fait une capture.
- **Emblème « CC »** modélisé en 3D low poly (`game_common::brand`) et rendu en
  PNG transparent 1024 px par `cargo run -p logo-gen`
  (`assets/branding/embleme.png`), pour des icônes nettes.

## Couleurs (source de vérité : `04_palette_couleurs.json`)

| Nom | HEX | Usage |
|---|---|---|
| Bleu principal | `#168FE8` | premier C, technologie |
| Vert principal | `#80C92B` | deuxième C, univers ludiques |
| Jaune citron | `#D8F43F` | facettes éclairées du low poly |
| Rouge corail | `#FF493E` | drapeau, détails |
| Bleu nuit | `#142C40` | typographie, contraste, mât |
| Bleu ciel | `#8BD9FF` | décors, ciel, interfaces |
| Ivoire | `#FFF9ED` | fonds clairs |

Les codes écrits sur les planches générées sont indicatifs.

## Typographie

- Titre et logo : **Russo One** (SIL OFL 1.1), `assets/fonts/RussoOne-Regular.ttf`.
- Sous-titre « GAME STUDIO » : capitales espacées.

Chaque univers (Croâ & Cie, Nova, Eurêka, Légendes) garde sa propre palette et
sa police ; la charte s'applique à ce qui porte la marque du studio : écran de
démarrage, icônes, site web.
