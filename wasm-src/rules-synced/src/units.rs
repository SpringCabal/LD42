//! Small unit helpers shared by several gadgets.

use mycube_shared::unit;
use spring::ResultExt;
use spring::rules_params::los::PUBLIC;

pub fn set_eskimo_needs(id: i32, food: f32, heat: f32) {
    spring::set_unit_rules_param_f32(id, unit::FOOD, food, PUBLIC)
        .log_err_once("publishing an Eskimo's food failed");
    spring::set_unit_rules_param_f32(id, unit::HEAT, heat, PUBLIC)
        .log_err_once("publishing an Eskimo's heat failed");
}

/// Destroy a unit unless it is already gone; several gadgets can condemn
/// the same unit in one frame (a drowning Eskimo also runs out of health).
pub fn destroy(id: i32, reclaimed: bool) {
    let alive =
        spring::valid_unit_id(id).unwrap_or(false) && !spring::get_unit_is_dead(id).unwrap_or(true);
    if !alive {
        return;
    }
    if let Err(error) = spring::destroy_unit(
        id,
        spring::rules_synced::unit_control::DestroyUnitOptions {
            selfd: false,
            reclaimed,
            attacker_id: -1,
            recycle_id: false,
        },
    ) {
        spring::log_warning!("mycube", "destroying unit {id} failed: {error:?}");
    }
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
