//! Drowning, ported from `unit_drown_eskimos.lua`.

use alloc::vec::Vec;

use crate::state::{PUBLIC, Units};

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
    let lane = units.drowning_cursor;
    units.drowning_cursor = (lane + 1) % 5;
    let mut dead = Vec::new();
    for (index, id) in ids.into_iter().enumerate() {
        if index % 5 != lane {
            continue;
        }
        if let Ok(position) = spring::get_unit_position(id, false, false)
            && spring::get_ground_height(position.x, position.z).unwrap_or(1.0) <= 0.0
        {
            // unit_drown_eskimos.lua invokes the retained Eskimo unit-script
            // hook as drowning begins. Core does not call that legacy gadget,
            // so preserve the authored animation explicitly and only once.
            if !units.drowning_animated.contains(&id) {
                let _ = spring::typed::call_unit_script(id, "StartDrowning", &[], 0);
                units.drowning_animated.push(id);
            }
            let health = spring::get_unit_rules_param_f32(id, "health").unwrap_or(1.0) - 5.0;
            let _ = spring::set_unit_rules_param_f32(id, "health", health, PUBLIC);
            if health <= 0.0 {
                dead.push(id);
            }
        }
    }
    dead
}
