use mycube_shared::unit;
use spring::ResultExt;
use spring::rules_params::los::PUBLIC;

const LETHAL_DAMAGE: f32 = 1.0e16;

pub fn pool(spring: &spring::Spring<'_, spring::RulesSynced>, id: i32) -> Option<f32> {
    spring
        .rules_params()
        .get_unit_rules_param_f32(id, unit::HEALTH)
        .map_err(|error| (id, error))
        .log_err_once(spring, "no health pool for (unit, error)")
}

pub fn set_pool(spring: &spring::Spring<'_, spring::RulesSynced>, id: i32, health: f32) {
    if let Err(error) =
        spring
            .rules_params()
            .set_unit_rules_param_f32(id, unit::HEALTH, health, PUBLIC)
    {
        spring::log_warning!(
            spring,
            "mycube",
            "setting the health of unit {id} failed: {error:?}"
        );
    }
}

pub fn unit_pre_damaged(
    spring: &spring::Spring<'_, spring::RulesSynced>,
    id: i32,
    attacker_def: i32,
) -> spring::DamageResult {
    let attacker_damage = crate::defs::damage(spring, attacker_def);
    let health = if attacker_damage == 0.0 {
        None
    } else {
        pool(spring, id)
    };
    let Some(health) = health else {
        return spring::DamageResult {
            new_damage: 0.0,
            impulse_mult: 1.0,
        };
    };
    let remaining = health - attacker_damage;
    set_pool(spring, id, remaining);
    let engine_damage = if remaining < 0.0 { LETHAL_DAMAGE } else { 0.0 };
    spring::DamageResult {
        new_damage: engine_damage,
        impulse_mult: 1.0,
    }
}
