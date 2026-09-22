# MyCube Core-WASM port

Run these commands from this directory:

```sh
just fmt     # format Rust sources
just clippy  # release WASM clippy, warnings denied
just build   # format, lint, and build both modules
just copy    # build and install both WASM artifacts
```

The original Lua game is retained beside this port; authored unit scripts are
implemented by the Rust CUS module in `rules-synced/src/unit_scripts`.
