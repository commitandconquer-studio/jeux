+++
title = "Installer les jeux"
template = "page.html"
+++

Tous nos jeux sont **gratuits et libres**. Choisissez la façon de jouer qui vous convient.

## 1. Jouer dans le navigateur, sans rien installer

C'est le plus simple : ouvrez la fiche d'un jeu et cliquez sur **▶ Jouer maintenant**. Le jeu se charge en quelques secondes la première fois (une dizaine de mégaoctets), puis il démarre instantanément.

- Il faut un navigateur récent (Firefox, Chrome, Edge, Safari, Brave…) et une carte graphique compatible WebGL2, ce qui est le cas de presque tous les ordinateurs et téléphones récents.
- Vos parties et vos réglages sont enregistrés dans votre navigateur.

## 2. Installer le lanceur sur Linux

Le **lanceur Commit & Conquer** installe tous nos jeux, les met à jour et les lance. Il s'ajoute à votre menu d'applications (catégorie **Jeux**), sans droits administrateur et sans rien compiler.

Le plus rapide : copiez cette ligne dans un terminal.

```
mkdir -p /tmp/lanceur-cc && curl -sL https://github.com/commitandconquer-studio/jeux/releases/latest/download/lanceur-linux-x86_64.tar.gz | tar xz -C /tmp/lanceur-cc && /tmp/lanceur-cc/lanceur
```

Ou à la main : [téléchargez le lanceur](https://github.com/commitandconquer-studio/jeux/releases/latest/download/lanceur-linux-x86_64.tar.gz), décompressez l'archive, puis ouvrez le fichier `lanceur` (clic droit, « Exécuter comme un programme »).

Au premier lancement, il s'installe dans `~/.local/share/commit-and-conquer` et apparaît dans le menu d'applications sous le nom **Commit & Conquer**. Ensuite, un clic sur **Installer** télécharge un jeu, **Jouer** l'ouvre, et **Mettre à jour** apparaît quand une nouvelle version sort.

Pour tout retirer, supprimez le dossier `~/.local/share/commit-and-conquer` et les fichiers `commit-and-conquer*.desktop` de `~/.local/share/applications`.

**Vous préférez compiler vous-même ?** Le code source et le script `tools/install-linux.sh` sont sur le [dépôt GitHub du studio](https://github.com/commitandconquer-studio/jeux).

## 3. Windows, macOS et Android

Les versions **Windows**, **macOS** et **Android** sont en préparation. En attendant, vous pouvez jouer dans le navigateur sur ces systèmes.

## Une question, un problème ?

Écrivez-nous à [contact@commitandconquer.fr](mailto:contact@commitandconquer.fr), ou sur le [dépôt GitHub du studio](https://github.com/commitandconquer-studio/jeux/issues) (onglet « Issues ») : toutes les remarques sont bienvenues.
