use mycube_shared::unit;
use spring::ResultExt;
use spring::rules_params::los::PUBLIC;

pub fn set_eskimo_needs(
    spring: &spring::Spring<'_, spring::RulesSynced>,
    id: i32,
    food: f32,
    heat: f32,
) {
    spring
        .rules_params()
        .set_unit_rules_param_f32(id, unit::FOOD, food, PUBLIC)
        .log_err_once(spring, "publishing an Eskimo's food failed");
    spring
        .rules_params()
        .set_unit_rules_param_f32(id, unit::HEAT, heat, PUBLIC)
        .log_err_once(spring, "publishing an Eskimo's heat failed");
}

pub fn is_alive(spring: &spring::Spring<'_, spring::RulesSynced>, id: i32) -> bool {
    spring.units_query().valid_unit_id(id).unwrap_or(false)
        && !spring.units_info().get_unit_is_dead(id).unwrap_or(true)
}

pub fn destroy(
    ctx: &spring_addons::AddonContext<'_, crate::state::Global>,
    id: i32,
    reclaimed: bool,
) {
    ctx.commands().run(move |spring| {
        if !is_alive(spring, id) {
            return;
        }
        if let Err(error) = spring.unit_control().destroy_unit(
            id,
            spring::unit_control::DestroyUnitOptions {
                selfd: false,
                reclaimed,
                attacker_id: -1,
                recycle_id: false,
            },
        ) {
            spring::log_warning!(spring, "mycube", "destroying unit {id} failed: {error:?}");
        }
    });
}

pub fn destroy_all(
    ctx: &spring_addons::AddonContext<'_, crate::state::Global>,
    ids: impl IntoIterator<Item = i32>,
    reclaimed: bool,
) {
    for id in ids {
        destroy(ctx, id, reclaimed);
    }
}
