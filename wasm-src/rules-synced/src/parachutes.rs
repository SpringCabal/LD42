//! Parachute cleanup, ported from `unit_parachute_remover.lua`.

use alloc::vec::Vec;

pub fn update_parachutes(parachutes: &[i32]) -> Vec<i32> {
    // The storyteller creates both the pirate and its parachute above the
    // map. ModRules enables the engine's normal ground-unit gravity, so leave
    // their descent to the engine and only retain the authored cleanup.
    let mut landed = Vec::new();
    for id in parachutes {
        let Ok(position) = spring::get_unit_position(*id, false, false) else {
            continue;
        };
        let Ok(ground) = spring::get_ground_height(position.x, position.z) else {
            continue;
        };
        if ground >= position.y {
            landed.push(*id);
        }
    }
    landed
}
