#!/usr/bin/env bash
# Commit & Conquer — licence GPL-3.0-or-later.
#
# Installe les jeux sur ce PC (Linux), avec une entrée et une icône dans le
# menu d'applications. Aucun droit administrateur nécessaire : tout va dans
# ~/.local/share.
#
#   tools/install-linux.sh                 compile (release) et installe tout
#   tools/install-linux.sh --desinstaller  retire les jeux et leurs entrées
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
DATA="${XDG_DATA_HOME:-$HOME/.local/share}"
DEST="$DATA/commit-and-conquer"
APPS="$DATA/applications"

# identifiant | nom affiché | description
GAMES=(
  "snake|Snake|Le serpent classique en 3D low-poly, dans 5 univers"
  "traversee|Traversée|Aidez le crapaud à traverser routes, rivières et voies ferrées"
  "tour|Tour de blocs|Empilez les blocs le plus haut possible"
  "pousse-caisses|Pousse-caisses|Le crapaud range les caisses sur les cibles, en 12 niveaux"
  "nova|NOVA — Le Dernier Rempart|Bâtissez le bouclier de Nova et abattez le croiseur de l'Hégémonie"
)

if [[ "${1:-}" == "--desinstaller" ]]; then
  for entry in "${GAMES[@]}"; do
    rm -f "$APPS/commit-and-conquer-${entry%%|*}.desktop"
  done
  rm -rf "${DEST:?}"
  update-desktop-database "$APPS" 2>/dev/null || true
  echo "Jeux Commit & Conquer désinstallés."
  exit 0
fi

CARGO="$(command -v cargo || echo "$HOME/.cargo/bin/cargo")"
packages=()
for entry in "${GAMES[@]}"; do
  packages+=(-p "${entry%%|*}")
done

echo "Compilation optimisée (la première fois peut prendre plusieurs minutes)…"
(cd "$ROOT" && "$CARGO" build --release "${packages[@]}")

mkdir -p "$DEST" "$APPS"
# La bibliothèque commune (sons, musiques, police) est partagée par tous les jeux.
rm -rf "${DEST:?}/assets"
cp -r "$ROOT/assets" "$DEST/assets"

for entry in "${GAMES[@]}"; do
  IFS='|' read -r id name description <<< "$entry"
  install -m 755 "$ROOT/target/release/$id" "$DEST/$id"
  install -m 644 "$ROOT/packaging/icons/$id.png" "$DEST/$id.png"
  cat > "$APPS/commit-and-conquer-$id.desktop" <<EOF
[Desktop Entry]
Type=Application
Name=$name
Comment=$description — un jeu Commit & Conquer
Exec="$DEST/$id"
Path=$DEST
Icon=$DEST/$id.png
Terminal=false
Categories=Game;
EOF
  echo "  ✓ $name"
done

update-desktop-database "$APPS" 2>/dev/null || true
echo "Installé dans $DEST — les jeux sont dans le menu d'applications (catégorie Jeux)."
