# My Cube WASM

Rust/WASM port of SpringCabal’s Ludum Dare 42 game.

Use the [SpringCabal development repository](https://github.com/gajop/springcabal-dev) for engine setup, launching, builds and verification.

## Scoped SDK port

The port branch uses the adjacent `spring-sdk-unified` and `springcabal-rs`
worktrees. Engine calls receive the current Spring reference; queued commands
and their continuations receive a fresh reference when executed. Unit scripts
belong to the rules addon module and use the SDK's shared CUS scheduler.
This remains a Wasm game; native game-module conversion is not included.

The existing 32-check integration suite passes on Linux and the Windows engine
under Wine (20x simulation speed). All workspace features pass strict Clippy.
The five-shot visual scenario completes on NVIDIA; the parent repository stores
reviewed NVIDIA goldens. Health-bar membership refreshes on creation,
destruction, LOS and ownership events instead of scanning and sorting all units
every simulation frame.

## Differences from Lua

- The HUD and unit health bars use screen-space rendering instead of Chili and world-space billboards.
- Ctrl+R resets the current round inside the rules module; Ctrl+Q returns to the menu. Both work throughout the game.
- Excavation only lowers terrain, drill ships search for ice in heightmap steps within the map, and support-building placement stops searching after 1,000 attempts.
- The sell cursor refreshes its harvest estimate at most ten times a second while moving and once a second while stationary.

[![Join the chat at https://gitter.im/SpringCabal/LD42](https://badges.gitter.im/SpringCabal/LD42.svg)](https://gitter.im/SpringCabal/LD42?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)
