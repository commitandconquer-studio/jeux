#!/usr/bin/env bash
# Commit & Conquer — licence GPL-3.0-or-later.
#
# Fabrique une archive Linux prête à jouer par jeu (et pour le lanceur), et le
# catalogue que lit le lanceur. Utilisé par .github/workflows/versions.yml pour
# les « Releases » GitHub ; marche aussi sur un PC pour essayer.
#
#   tools/archives-linux.sh 0.2.0 dist
#
# Chaque archive <jeu>-linux-x86_64.tar.gz contient, à sa racine :
#   <jeu>            l'exécutable
#   <jeu>.png        l'icône (menu d'applications)
#   <jeu>.version    la version (le lanceur s'en sert pour les mises à jour)
#   assets/          sons, musiques, polices et images communs
# On la décompresse dans ~/.local/share/commit-and-conquer, comme
# tools/install-linux.sh.
set -euo pipefail

VERSION="${1:?usage : $0 <version> <dossier>}"
VERSION="${VERSION#v}"
OUT="${2:?usage : $0 <version> <dossier>}"
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
CARGO="$(command -v cargo || echo "$HOME/.cargo/bin/cargo")"

# Les jeux publiés (paquets Cargo), puis le lanceur.
JEUX=(snake traversee tour eureka-2048 berger lanceur)

packages=()
for jeu in "${JEUX[@]}"; do
  packages+=(-p "$jeu")
done
echo "Compilation optimisée…"
(cd "$ROOT" && "$CARGO" build --release "${packages[@]}")

mkdir -p "$OUT"
OUT="$(cd "$OUT" && pwd)"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

catalogue="$OUT/catalogue.json"
{
  echo "{"
  echo "  \"version\": \"$VERSION\","
  echo "  \"jeux\": {"
} > "$catalogue"

n=0
for jeu in "${JEUX[@]}"; do
  dossier="$TMP/$jeu"
  mkdir -p "$dossier"
  install -m 755 "$ROOT/target/release/$jeu" "$dossier/$jeu"
  install -m 644 "$ROOT/packaging/icons/$jeu.png" "$dossier/$jeu.png"
  echo "$VERSION" > "$dossier/$jeu.version"
  cp -r "$ROOT/assets" "$dossier/assets"
  archive="$jeu-linux-x86_64.tar.gz"
  # Archive reproductible : même date et même propriétaire pour tous les fichiers.
  tar --sort=name --owner=0 --group=0 --numeric-owner --mtime="@0" \
    -C "$dossier" -czf "$OUT/$archive" .
  taille=$(stat -c %s "$OUT/$archive")
  sha=$(sha256sum "$OUT/$archive" | cut -d' ' -f1)
  n=$((n + 1))
  virgule=","
  [[ $n -eq ${#JEUX[@]} ]] && virgule=""
  echo "    \"$jeu\": { \"archive\": \"$archive\", \"taille\": $taille, \"sha256\": \"$sha\" }$virgule" >> "$catalogue"
  echo "  ✓ $archive ($((taille / 1024 / 1024)) Mo)"
done

{
  echo "  }"
  echo "}"
} >> "$catalogue"
echo "Archives et catalogue dans $OUT"
