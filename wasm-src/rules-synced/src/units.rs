use mycube_shared::unit;
use spring::ResultExt;
use spring::rules_params::los::PUBLIC;

pub fn set_eskimo_needs(id: i32, food: f32, heat: f32) {
    spring::set_unit_rules_param_f32(id, unit::FOOD, food, PUBLIC)
        .log_err_once("publishing an Eskimo's food failed");
    spring::set_unit_rules_param_f32(id, unit::HEAT, heat, PUBLIC)
        .log_err_once("publishing an Eskimo's heat failed");
}

pub fn is_alive(id: i32) -> bool {
    spring::valid_unit_id(id).unwrap_or(false) && !spring::get_unit_is_dead(id).unwrap_or(true)
}

pub fn destroy(id: i32, reclaimed: bool) {
    spring::commands::run(move || {
        if !is_alive(id) {
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
    });
}

pub fn destroy_all(ids: impl IntoIterator<Item = i32>, reclaimed: bool) {
    for id in ids {
        destroy(id, reclaimed);
    }
}
