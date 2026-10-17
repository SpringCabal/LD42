use alloc::vec::Vec;

use mycube_shared::round::LUA_FRAMES_PER_SECOND;
use mycube_shared::unit::MAX_NEED;

use crate::state::{Economy, Eskimo, Village};
use crate::units::set_eskimo_needs;

const WARMING_RESTART_MARGIN: f32 = 2.0;
const START_EATING_BELOW: f32 = 20.0;
const STOP_EATING_ABOVE: f32 = 80.0;

const FPS: f32 = LUA_FRAMES_PER_SECOND as f32;
const BURNER_RANGE: f32 = 300.0;
const WARMING_INCREASE: f32 = 4.0 / FPS;
const COLD_DECREASE: f32 = 1.0 / FPS;
const HEAT_RES_USAGE: f32 = 0.5 / FPS;
const FOOD_DECAY_RATE: f32 = 1.0 / FPS;
const FOOD_EATING_RATE: f32 = 10.0 / FPS;
const FOOD_RES_USAGE: f32 = 0.1 / FPS;
const FREEZING_HEALTH_DECAY: f32 = 1.0 / FPS;
const STARVING_HEALTH_DECAY: f32 = 0.2 / FPS;

pub fn update_eskimos(
    spring: &spring::Spring<'_, spring::RulesSynced>,
    village: &mut Village,
    economy: &mut Economy,
) -> Vec<i32> {
    let burner_positions: Vec<spring::Float3> = village
        .burners
        .iter()
        .filter_map(|id| {
            spring
                .units_info()
                .get_unit_position(*id, Default::default())
                .ok()
        })
        .collect();
    let mut dead = Vec::new();
    for eskimo in &mut village.eskimos {
        let near_burner = spring
            .units_info()
            .get_unit_position(eskimo.id, Default::default())
            .ok()
            .is_some_and(|position| {
                burner_positions.iter().any(|burner| {
                    let dx = position.x - burner.x;
                    let dz = position.z - burner.z;
                    dx * dx + dz * dz <= BURNER_RANGE * BURNER_RANGE
                })
            });
        update_heat(eskimo, economy, near_burner);
        update_food(eskimo, economy);
        let mut health_loss = 0.0;
        if eskimo.heat <= 0.0 {
            health_loss += FREEZING_HEALTH_DECAY;
        }
        if eskimo.food <= 0.0 {
            health_loss += STARVING_HEALTH_DECAY;
        }
        if let Some(health) = crate::gadgets::health::behavior::pool(spring, eskimo.id) {
            if health_loss > 0.0 {
                crate::gadgets::health::behavior::set_pool(
                    spring,
                    eskimo.id,
                    (health - health_loss).max(0.0),
                );
            }
            if health - health_loss <= 0.0 {
                dead.push(eskimo.id);
            }
        }
        set_eskimo_needs(spring, eskimo.id, eskimo.food, eskimo.heat);
    }
    dead
}

fn update_heat(eskimo: &mut Eskimo, economy: &mut Economy, near_burner: bool) {
    if eskimo.heat + WARMING_RESTART_MARGIN <= MAX_NEED {
        eskimo.warm = near_burner;
    }
    if eskimo.warm {
        if economy.heat >= HEAT_RES_USAGE {
            economy.heat -= HEAT_RES_USAGE;
            eskimo.heat += WARMING_INCREASE;
        } else {
            eskimo.warm = false;
        }
    } else {
        eskimo.heat -= COLD_DECREASE;
    }
    if eskimo.heat >= MAX_NEED {
        eskimo.warm = false;
    }
    eskimo.heat = eskimo.heat.clamp(0.0, MAX_NEED);
}

fn update_food(eskimo: &mut Eskimo, economy: &mut Economy) {
    if eskimo.food < START_EATING_BELOW {
        eskimo.eating = true;
    } else if eskimo.food > STOP_EATING_ABOVE {
        eskimo.eating = false;
    }
    if !eskimo.eating {
        eskimo.food -= FOOD_DECAY_RATE;
    } else if economy.food >= FOOD_RES_USAGE {
        economy.food -= FOOD_RES_USAGE;
        eskimo.food += FOOD_EATING_RATE;
    } else {
        eskimo.eating = false;
    }
    if eskimo.food >= MAX_NEED {
        eskimo.eating = false;
    }
    eskimo.food = eskimo.food.clamp(0.0, MAX_NEED);
}
