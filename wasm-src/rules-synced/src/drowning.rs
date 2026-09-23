//! Drowning, ported from `unit_drown_eskimos.lua`.

use alloc::vec::Vec;

use spring::rules_params::los::PUBLIC;

use crate::state::Units;

pub fn update_drowning(units: &mut Units) -> Vec<i32> {
    let ids: Vec<i32> = units
        .eskimos
        .iter()
        .map(|unit| unit.id)
        .chain(units.pirates.iter().copied())
        .collect();
    if ids.is_empty() {
        return Vec::new();
    }
    let mut dead = Vec::new();
    for id in ids {
        if let Ok(position) = spring::get_unit_position(id, false, false)
            && spring::get_ground_height(position.x, position.z).unwrap_or(1.0) <= 0.0
        {
            // unit_drown_eskimos.lua calls StartDrowning on every water
            // frame. The Lua function itself is idempotent, and repeating the
            // call matters here: a map-authored unit can reach this gadget
            // before its deferred CUS instance has been attached.
            if let Some(eskimo) = units.eskimos.iter_mut().find(|eskimo| eskimo.id == id) {
                eskimo.drowning = true;
            }
            let _ = spring::set_unit_rules_param_f32(id, "drowning", 1.0, PUBLIC);
            // Lua invokes StartDrowning from this gadget callin. Calling
            // back through Spring.UnitScript here would re-enter this same
            // Core-Wasm store, so defer the equivalent named callback to the
            // CUS tick that follows the callin.
            crate::unit_scripts::request_drowning(id);
            let health = spring::get_unit_rules_param_f32(id, "health").unwrap_or(0.0) - 1.0;
            if health <= 0.0 {
                // Use ordinary destruction so Killed plays the authored death
                // sound/animation and Spring creates the configured corpse.
                dead.push(id);
            } else {
                let _ = spring::set_unit_rules_param_f32(id, "health", health, PUBLIC);
                if let Some(eskimo) = units.eskimos.iter_mut().find(|eskimo| eskimo.id == id) {
                    eskimo.health = health;
                }
            }
        }
    }
    dead
}
