#!/usr/bin/env bash
# Télécharge les assets de Dernière Lueur (Poly Haven, CC0 ; Quaternius via Poly Pizza, CC-BY).
# Licences et auteurs : assets/lueur/CREDITS. Usage : tools/lueur-assets.sh
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT="$ROOT/assets/lueur"
mkdir -p "$OUT/textures" "$OUT/modeles"
api() { curl -sf "https://api.polyhaven.com/$1"; }
for t in asphalt_01 concrete_wall_001 rust_coarse_01 brown_mud_leaves_01; do
  for m in Diffuse nor_gl Rough; do
    url=$(api files/$t | python3 -c "import sys,json;print(json.load(sys.stdin)['$m']['1k']['jpg']['url'])")
    curl -sfL "$url" -o "$OUT/textures/${t}_$m.jpg"
  done
done
for p in metal_jerrycan old_tyre wooden_crate_02 covered_car power_box_01 propane_tank medical_box; do
  mkdir -p "$OUT/modeles/$p"
  api files/$p | python3 -c "
import sys,json
g=json.load(sys.stdin)['gltf']['1k']['gltf']
print(g['url'], '$OUT/modeles/$p/'+g['url'].split('/')[-1])
for rel,i in g['include'].items(): print(i['url'], '$OUT/modeles/$p/'+rel)
" | while read -r u f; do mkdir -p "$(dirname "$f")"; curl -sfL "$u" -o "$f"; done
done
curl -sfL "https://static.poly.pizza/972d277d-6fc4-46c7-85d1-e4d68cf46548.glb" -o "$OUT/modeles/zombie.glb"
