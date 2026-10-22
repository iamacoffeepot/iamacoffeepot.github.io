#!/usr/bin/env bash
# Builds the site into _site/: a copy of site/ with each demo's crate built for wasm32 and its bindings written into
# the pkg/ directory its page imports from. The Pages workflow runs this same script. Serve _site/ over http on
# localhost, where WebGPU is let run.
set -euo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$root"

# Each crate, and the directory under the site its page lives in.
demos=(
  "pong games/pong"
  "galaxy demos/galaxy"
  "blackhole demos/blackhole"
)

rm -rf _site
cp -R site _site

target="$(cargo metadata --locked --format-version 1 --no-deps | python3 -c 'import json, sys; print(json.load(sys.stdin)["target_directory"])')"

for demo in "${demos[@]}"; do
  read -r name page <<< "$demo"
  cargo build --locked --release --target wasm32-unknown-unknown -p "$name"
  wasm-bindgen --target web --no-typescript --out-dir "_site/$page/pkg" "$target/wasm32-unknown-unknown/release/$name.wasm"
  echo "built demo=$name bytes=$(wc -c < "_site/$page/pkg/${name}_bg.wasm")"
done
