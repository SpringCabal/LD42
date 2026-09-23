//! Small unit helpers shared by several gadgets.

use spring::rules_params::los::PUBLIC;

pub fn set_eskimo_needs(id: i32, food: f32, heat: f32) {
    let _ = spring::set_unit_rules_param_f32(id, "food", food, PUBLIC);
    let _ = spring::set_unit_rules_param_f32(id, "heat", heat, PUBLIC);
}

pub fn destroy(id: i32, reclaimed: bool) {
    let _ = spring::destroy_unit(
        id,
        spring::rules_synced::unit_control::DestroyUnitOptions {
            selfd: false,
            reclaimed,
            attacker_id: -1,
            recycle_id: false,
        },
    );
}
