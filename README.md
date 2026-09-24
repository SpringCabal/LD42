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
- `port-reference/`: the Lua original's gadgets and widgets, kept for
  comparison while porting. The game does not load them and releases leave
  them out.

[![Join the chat at https://gitter.im/SpringCabal/LD42](https://badges.gitter.im/SpringCabal/LD42.svg)](https://gitter.im/SpringCabal/LD42?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)
