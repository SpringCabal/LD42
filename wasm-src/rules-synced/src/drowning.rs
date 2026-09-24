//! Drowning, ported from `unit_drown_eskimos.lua`.

use alloc::vec::Vec;

/// Every unit that can drown (Eskimos and pirates) loses a point of health
/// per frame in water. Returns the units that drowned.
pub fn update_drowning(units: impl Iterator<Item = i32>) -> Vec<i32> {
    let mut dead = Vec::new();
    for id in units {
        if let Ok(position) = spring::get_unit_position(id, false, false)
            && spring::get_ground_height(position.x, position.z).unwrap_or(1.0) <= 0.0
        {
            // unit_drown_eskimos.lua calls StartDrowning on every frame in
            // water; the script ignores repeats. The drowning gadget runs in
            // GameFrame, never inside a unit-script callin, so the module is
            // free here.
            crate::with_cus_module(|module| module.start_drowning(id));
            let Some(health) = crate::health::pool(id).map(|health| health - 1.0) else {
                continue;
            };
            if health <= 0.0 {
                // Use ordinary destruction so Killed plays the authored death
                // sound/animation and Spring creates the configured corpse.
                dead.push(id);
            } else {
                crate::health::set_pool(id, health);
            }
        }
    }
    dead
}
