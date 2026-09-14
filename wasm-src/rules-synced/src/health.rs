//! The gameplay health pool, ported from `game_health.lua`.
//!
//! Spring's engine health is deliberately not the authority for this game.

use crate::state::PUBLIC;

pub fn unit_pre_damaged(
    id: i32,
    _def: i32,
    _team: i32,
    damage: f32,
    _paralyzer: bool,
    _weapon: i32,
    _projectile: i32,
    _attacker: i32,
    attacker_def: i32,
    _attacker_team: i32,
) -> spring::DamageResult {
    // This is the original `game_health.lua` contract: use the attacker's
    // `customParams.damage`, update the script health pool, and suppress the
    // engine's ordinary weapon damage.  Falling/environmental damage has no
    // attacker and is likewise suppressed by the Lua implementation.
    let attacker_damage = gameplay_damage(attacker_def);
    if attacker_damage == 0.0 {
        return spring::DamageResult {
            new_damage: 0.0,
            impulse_mult: 1.0,
        };
    }
    let health = spring::get_unit_rules_param_f32(id, "health").unwrap_or(0.0);
    let remaining = health - attacker_damage;
    let _ = spring::set_unit_rules_param_f32(id, "health", remaining, PUBLIC);
    // Lua returns a lethal engine hit only after its custom pool reaches zero.
    // It otherwise returns zero so the engine cannot bypass that pool.
    let engine_damage = if remaining < 0.0 { 1.0e16 } else { 0.0 };
    let _ = damage; // The incoming engine damage is intentionally ignored.
    spring::DamageResult {
        new_damage: engine_damage,
        impulse_mult: 1.0,
    }
}

pub fn gameplay_health(def: i32) -> f32 {
    // The engine API does not expose merged inherited Lua customParams for
    // these content definitions.  Preserve the authored values explicitly:
    // Humanoid's Eskimo/Pirate subclasses declare health=100 and the
    // drillship declares health=500. Other units receive game_health.lua's
    // generic 100000-point pool.
    if spring::get_unit_def_id_by_name("drillship").ok() == Some(def) {
        return 500.0;
    }
    let humanoid = ["eskimo", "pirate"]
        .into_iter()
        .any(|name| spring::get_unit_def_id_by_name(name).ok() == Some(def));
    if humanoid { 100.0 } else { 100_000.0 }
}

pub fn gameplay_damage(def: i32) -> f32 {
    if ["eskimo", "pirate"]
        .into_iter()
        .any(|name| spring::get_unit_def_id_by_name(name).ok() == Some(def))
    {
        10.0
    } else {
        0.0
    }
}
