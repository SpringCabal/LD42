# My Cube (WASM)

SpringCabal's Ludum Dare 42 game, running on the Recoil engine's Core-WASM
modules instead of Lua. The Lua original is the sibling `MyCube.sdd`.

- `wasm-src/`: the Rust sources of the two modules (see its README).
- `LuaRules/wasm/`, `LuaUI/wasm/`: the manifests that load them, and the
  built modules (`just copy` in `wasm-src/` installs them; not committed).
- `LuaRules/main.lua`, `LuaUI/main.lua`: small entry points that leave the
  gameplay, the unit scripts and the UI to the modules.
- `Units/`, `Weapons/`, `GameData/`, models, textures and sounds: the game
  data, as in the Lua original.

## Differences from the Lua version

- Restart (Ctrl+R) resets the round inside the rules module: `luarules reload`, which `ui_endgame.lua` uses, doesn't reload WASM rules.
- Restart and quit are the keys Ctrl+R and Ctrl+Q, and work at any time; Lua shows Restart and Exit buttons on the end screen only.
- The HUD (resources, key legend, end screen, story text) is drawn with plain rectangles and text instead of Chili.
- Health bars are drawn in screen space above each unit, not as billboards in the world: the drawing API has no display lists.
- Excavation subtracts from the height (floor -500) instead of setting a target from the corner height: the interpolated ground height made the Lua formula raise corners now and then.
- A drill ship searching for new ice grows its search square by one heightmap square at a time instead of Lua's one elmo (which samples nearly the same points eight times over and could exceed the work budget on this map), and skips points off the map, which Lua can pick at the map edge.
- The story gives up looking for room for an iglu after 1000 tries and places it anyway; Lua keeps trying.
- The sell cursor's harvest value is recomputed at most ten times a second, and once a second while the cursor rests; Lua recomputes its 676 height samples on every move.

[![Join the chat at https://gitter.im/SpringCabal/LD42](https://badges.gitter.im/SpringCabal/LD42.svg)](https://gitter.im/SpringCabal/LD42?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)
