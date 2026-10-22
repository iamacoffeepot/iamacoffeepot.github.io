# iamacoffeepot.github.io

Small games and demos that run in the browser, served at <https://iamacoffeepot.github.io>. They need a browser with
WebGPU.

Each one is a Rust crate built on [aethereal](https://github.com/iamacoffeepot/aethereal)'s browser chassis:

- `crates/pong/` — Pong, at `/games/pong/`;
- `crates/galaxy/` — the galaxy, at `/demos/galaxy/`.

`site/` holds the static pages. `scripts/build.sh` copies it to `_site/`, builds each crate for
`wasm32-unknown-unknown`, and writes its `wasm-bindgen` glue into the `pkg/` directory its page imports. On every push
to `main`, the Pages workflow (`.github/workflows/pages.yml`) runs the same script and deploys `_site/`.

## Building locally

You need the toolchain in `rust-toolchain.toml` (rustup installs it on first use) and `wasm-bindgen-cli` at the
version of the `wasm-bindgen` crate pinned in `Cargo.toml`:

```sh
cargo install wasm-bindgen-cli --version 0.2.129
scripts/build.sh
python3 -m http.server 8780 --bind 127.0.0.1 --directory _site
```

Then open <http://127.0.0.1:8780/>. WebGPU runs on localhost without https.

## Bumping aethereal

The crates take aethereal as git dependencies at one commit, the `rev` in `[workspace.dependencies]` in `Cargo.toml`.
To move to a newer commit:

1. Change the `rev` on the aethereal lines to the new commit's full SHA.
2. Run `cargo update`.
3. Build with `scripts/build.sh`, and fix what broke.
4. Push to `main`.

If the new aethereal pins a different `wasm-bindgen`, change it in `Cargo.toml` and the `wasm-bindgen@` version in the
workflow together: the CLI and the crate must match exactly.
