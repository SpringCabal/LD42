//! The gameplay health pool, ported from `game_health.lua`.
//!
//! Spring's engine health is deliberately not the authority for this game.

use spring::rules_params::los::PUBLIC;

/// Returned once the pool is exhausted, so the engine kills the unit.
const LETHAL_DAMAGE: f32 = 1.0e16;

/// `game_health.lua`'s UnitPreDamaged: subtract the attacker's gameplay
/// damage from the unit's health pool and suppress the engine damage, unless
/// the pool has run out. Damage without a known attacker (falling,
/// environment) is suppressed too.
pub fn unit_pre_damaged(id: i32, attacker_def: i32) -> spring::DamageResult {
    let attacker_damage = crate::defs::damage(attacker_def);
    if attacker_damage == 0.0 {
        return spring::DamageResult {
            new_damage: 0.0,
            impulse_mult: 1.0,
        };
    }
    let health = spring::get_unit_rules_param_f32(id, "health").unwrap_or(0.0);
    let remaining = health - attacker_damage;
    let _ = spring::set_unit_rules_param_f32(id, "health", remaining, PUBLIC);
    let engine_damage = if remaining < 0.0 { LETHAL_DAMAGE } else { 0.0 };
    spring::DamageResult {
        new_damage: engine_damage,
        impulse_mult: 1.0,
    }
}
