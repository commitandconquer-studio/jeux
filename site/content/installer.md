+++
title = "Installer les jeux"
template = "page.html"
+++

Tous nos jeux sont **gratuits et libres**. Choisissez la façon de jouer qui vous convient.

## 1. Jouer dans le navigateur, sans rien installer

C'est le plus simple : ouvrez la fiche d'un jeu et cliquez sur **▶ Jouer maintenant**. Le jeu se charge en quelques secondes la première fois (une dizaine de mégaoctets), puis il démarre instantanément.

- Il faut un navigateur récent (Firefox, Chrome, Edge, Safari, Brave…) et une carte graphique compatible WebGL2, ce qui est le cas de presque tous les ordinateurs et téléphones récents.
- Vos parties et vos réglages sont enregistrés dans votre navigateur.

## 2. Installer les jeux sur Linux

Les jeux s'installent dans votre menu d'applications (catégorie **Jeux**), sans droits administrateur. Pour l'instant, on les compile depuis le code source : comptez une dizaine de minutes la première fois.

**Une seule fois** : installez Rust et les bibliothèques nécessaires (Ubuntu, Debian, Linux Mint…).

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
sudo apt install git build-essential pkg-config libasound2-dev libudev-dev libwayland-dev libxkbcommon-dev
```

**Puis** récupérez les jeux et installez-les :

```
git clone https://github.com/commitandconquer-studio/jeux.git commit-and-conquer
cd commit-and-conquer
tools/install-linux.sh
```

C'est tout : les jeux apparaissent dans votre menu d'applications. Pour les mettre à jour plus tard :

```
cd commit-and-conquer
git pull
tools/install-linux.sh
```

Et pour tout retirer : `tools/install-linux.sh --desinstaller`.

## 3. Windows, macOS et Android

Les versions **Windows**, **macOS** et **Android** sont en préparation. En attendant, vous pouvez jouer dans le navigateur sur ces systèmes.

## Une question, un problème ?

Écrivez-nous sur le [dépôt GitHub du studio](https://github.com/commitandconquer-studio/jeux/issues) (onglet « Issues ») : toutes les remarques sont bienvenues.
