//! The gameplay health pool, ported from `game_health.lua`.
//!
//! Spring's engine health is deliberately not the authority for this game.

use mycube_shared::unit;
use spring::rules_params::los::PUBLIC;

/// Returned once the pool is exhausted, so the engine kills the unit.
const LETHAL_DAMAGE: f32 = 1.0e16;

/// A unit's health pool (its `health` rules param). The Health gadget sets
/// it when the unit is created, so a failed read is a bug: it is logged and
/// the caller leaves the unit alone.
pub fn pool(id: i32) -> Option<f32> {
    spring::get_unit_rules_param_f32(id, unit::HEALTH)
        .inspect_err(|error| {
            spring::log_warning!("mycube", "no health pool for unit {id}: {error:?}");
        })
        .ok()
}

pub fn set_pool(id: i32, health: f32) {
    if let Err(error) = spring::set_unit_rules_param_f32(id, unit::HEALTH, health, PUBLIC) {
        spring::log_warning!(
            "mycube",
            "setting the health of unit {id} failed: {error:?}"
        );
    }
}

/// `game_health.lua`'s UnitPreDamaged: subtract the attacker's gameplay
/// damage from the unit's health pool and suppress the engine damage, unless
/// the pool has run out. Damage without a known attacker (falling,
/// environment) is suppressed too, and so is damage to a unit without a pool.
pub fn unit_pre_damaged(id: i32, attacker_def: i32) -> spring::DamageResult {
    let attacker_damage = crate::defs::damage(attacker_def);
    let health = if attacker_damage == 0.0 {
        None
    } else {
        pool(id)
    };
    let Some(health) = health else {
        return spring::DamageResult {
            new_damage: 0.0,
            impulse_mult: 1.0,
        };
    };
    let remaining = health - attacker_damage;
    set_pool(id, remaining);
    let engine_damage = if remaining < 0.0 { LETHAL_DAMAGE } else { 0.0 };
    spring::DamageResult {
        new_damage: engine_damage,
        impulse_mult: 1.0,
    }
}
