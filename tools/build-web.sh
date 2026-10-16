#!/usr/bin/env bash
# Commit & Conquer — licence GPL-3.0-or-later.
#
# Compile les jeux pour le navigateur (WebAssembly) et les dépose dans le site :
#   site/static/jouer/<jeu>/<jeu>.js + <jeu>_bg.wasm   (un dossier par jeu)
#   site/static/jouer/assets/                          (bibliothèque commune)
#
#   tools/build-web.sh                 tous les jeux
#   tools/build-web.sh concile 2048    seulement ceux-là (noms de la page /jouer/…)
#
# Prérequis : rustup target add wasm32-unknown-unknown
#             cargo install wasm-bindgen-cli --version <celle de Cargo.lock>
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT="$ROOT/site/static/jouer"
CARGO="$(command -v cargo || echo "$HOME/.cargo/bin/cargo")"
BINDGEN="$(command -v wasm-bindgen || echo "$HOME/.cargo/bin/wasm-bindgen")"

# nom de la page | paquet cargo
JEUX=(snake\|snake traversee\|traversee tour\|tour pousse-caisses\|pousse-caisses nova\|nova 2048\|eureka-2048 concile\|concile)
voulus=("$@")

mkdir -p "$OUT"
for entree in "${JEUX[@]}"; do
  page="${entree%%|*}"; paquet="${entree##*|}"
  if [ ${#voulus[@]} -gt 0 ] && [[ ! " ${voulus[*]} " == *" $page "* ]]; then continue; fi
  echo "→ $page"
  (cd "$ROOT" && "$CARGO" build -q --profile web --target wasm32-unknown-unknown -p "$paquet")
  rm -rf "$OUT/$page"
  "$BINDGEN" --target web --no-typescript --out-dir "$OUT/$page" --out-name "$page" \
    "$ROOT/target/wasm32-unknown-unknown/web/$paquet.wasm"
  if command -v wasm-opt >/dev/null; then
    wasm-opt -Os --enable-bulk-memory --enable-nontrapping-float-to-int -o "$OUT/$page/${page}_bg.wasm" "$OUT/$page/${page}_bg.wasm"
  fi
  du -h "$OUT/$page/${page}_bg.wasm" | cut -f1 | sed "s/^/  wasm : /"
done
# La bibliothèque commune, une seule fois pour tous les jeux.
rm -rf "$OUT/assets"
cp -r "$ROOT/assets" "$OUT/assets"
echo "Jeux prêts dans site/static/jouer/ ($(du -sh "$OUT" | cut -f1))."
