# My Cube WASM modules

Run these commands from this directory:

```sh
just fmt     # format Rust sources
just clippy  # release WASM clippy, warnings denied
just build   # format, lint, and build both modules
just copy    # build and install both WASM artifacts
```

- `rules-synced/`: the synced rules module. Each gadget in `src/gadgets/`
  ports one Lua gadget (storyteller, Eskimo needs, drowning, drills, health,
  trade, ...) and owns its state; `src/state.rs` holds what several share.
  The same module is the unit-script (CUS) module: `src/unit_scripts/`
  implements `scripts/*.lua` of the Lua original, and the rules reach it
  directly through `with_cus_module`.
- `ui/`: the UI module, one widget per Lua widget (resources, trade,
  health bars, story text, end screen, music).
- `shared/`: what both modules must agree on: the messages between them,
  the unit rules params, the ice-trade yield and the round's constants.
  Its host tests run with
  `cargo test -p mycube-shared --target x86_64-unknown-linux-gnu`.

The `headless-integration` (rules and UI) and `e2e` (UI) features add the
tests the verification suites run. The verification gallery sets the mod option
`gallery`, which turns the story off so the photographed unit stands alone.
