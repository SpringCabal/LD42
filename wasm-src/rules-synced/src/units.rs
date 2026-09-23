//! Small unit helpers shared by several gadgets.

use spring::rules_params::los::PUBLIC;

pub fn set_eskimo_needs(id: i32, food: f32, heat: f32) {
    let _ = spring::set_unit_rules_param_f32(id, "food", food, PUBLIC);
    let _ = spring::set_unit_rules_param_f32(id, "heat", heat, PUBLIC);
}

/// Destroy a unit unless it is already gone; several gadgets can condemn
/// the same unit in one frame (a drowning Eskimo also runs out of health).
pub fn destroy(id: i32, reclaimed: bool) {
    let alive =
        spring::valid_unit_id(id).unwrap_or(false) && !spring::get_unit_is_dead(id).unwrap_or(true);
    if !alive {
        return;
    }
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

/// Destroy these units once the current callin returns, when no gadget
/// state is borrowed (UnitDestroyed runs immediately).
pub fn destroy_later<G>(
    ctx: &spring_addons::AddonContext<'_, G>,
    ids: alloc::vec::Vec<i32>,
    reclaimed: bool,
) {
    if ids.is_empty() {
        return;
    }
    ctx.delay(move |_| {
        for id in ids {
            destroy(id, reclaimed);
        }
    });
}
