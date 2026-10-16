#!/usr/bin/env bash
# Commit & Conquer — licence GPL-3.0-or-later.
#
# Crée un nouveau jeu à partir du jeu modèle (games/modele), qui respecte le
# squelette commun du studio (docs/SQUELETTE.md).
#
#   tools/nouveau-jeu.sh mon-jeu "Mon jeu"
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
id="${1:?usage : tools/nouveau-jeu.sh <identifiant> \"Nom affiché\"}"
nom="${2:-$id}"
[[ "$id" =~ ^[a-z][a-z0-9-]*$ ]] || { echo "Identifiant : minuscules, chiffres et tirets."; exit 1; }
dest="$ROOT/games/$id"
[ -e "$dest" ] && { echo "games/$id existe déjà."; exit 1; }

cp -r "$ROOT/games/modele" "$dest"
VAR="$(echo "$id" | tr 'a-z-' 'A-Z_')_CAPTURE"
sed -i "s/^name = \"modele\"/name = \"$id\"/; s/^description = .*/description = \"$nom\"/" "$dest/Cargo.toml"
sed -i "s/Modèle — Commit \& Conquer/$nom — Commit \& Conquer/" "$dest/src/main.rs"
sed -i "s/MODELE_CAPTURE/$VAR/g; s/-p modele/-p $id/g" "$dest/src/capture.rs"
sed -i "s/\"modele\", \"record\"/\"$id\", \"record\"/g" "$dest/src/play.rs"
# Le nouveau jeu rejoint le workspace.
sed -i "s|    \"games/modele\",|    \"games/modele\",\n    \"games/$id\",|" "$ROOT/Cargo.toml"

echo "Jeu créé dans games/$id."
echo "  cargo run -p $id                (essayer)"
echo "  $VAR=captures cargo run -p $id  (captures)"
echo "Ensuite : docs/SQUELETTE.md, section « Pour publier »."
