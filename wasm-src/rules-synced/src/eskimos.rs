//! Per-Eskimo food/heat/health bookkeeping, ported from `unit_eskimos.lua`.

use alloc::vec::Vec;

use crate::state::{Economy, Eskimo, FRAMES_PER_SECOND, Village};
use crate::units::set_eskimo_needs;

const FPS: f32 = FRAMES_PER_SECOND as f32;
const MAX_NEED: f32 = 100.0;
const BURNER_RANGE: f32 = 300.0;
const WARMING_INCREASE: f32 = 4.0 / FPS;
const COLD_DECREASE: f32 = 1.0 / FPS;
const HEAT_RES_USAGE: f32 = 0.5 / FPS;
const FOOD_DECAY_RATE: f32 = 1.0 / FPS;
const FOOD_EATING_RATE: f32 = 10.0 / FPS;
const FOOD_RES_USAGE: f32 = 0.1 / FPS;
const FREEZING_HEALTH_DECAY: f32 = 1.0 / FPS;
const STARVING_HEALTH_DECAY: f32 = 0.2 / FPS;

/// Runs every frame, like `unit_eskimos.lua`'s GameFrame.
pub fn update_eskimos(village: &mut Village, economy: &mut Economy) -> Vec<i32> {
    let burner_positions: Vec<spring::Float3> = village
        .burners
        .iter()
        .filter_map(|id| spring::get_unit_position(*id, false, false).ok())
        .collect();
    let sync_lane = village.sync_lane;
    let mut dead = Vec::new();
    for (index, eskimo) in village.eskimos.iter_mut().enumerate() {
        let near_burner = spring::get_unit_position(eskimo.id, false, false)
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
        // The health pool lives in the "health" rules param, which combat
        // (health.rs) and drowning also lower, so read it back every frame.
        let mut health_loss = 0.0;
        if eskimo.heat <= 0.0 {
            health_loss += FREEZING_HEALTH_DECAY;
        }
        if eskimo.food <= 0.0 {
            health_loss += STARVING_HEALTH_DECAY;
        }
        if let Some(health) = crate::health::pool(eskimo.id) {
            if health_loss > 0.0 {
                crate::health::set_pool(eskimo.id, (health - health_loss).max(0.0));
            }
            if health - health_loss <= 0.0 {
                dead.push(eskimo.id);
            }
        }
        // Food and heat only drive the HUD. Rotating five lanes keeps every
        // value fresh within five frames without turning a 20-person wave
        // into forty host calls per frame.
        if index % 5 == sync_lane {
            set_eskimo_needs(eskimo.id, eskimo.food, eskimo.heat);
        }
    }
    village.sync_lane = (sync_lane + 1) % 5;
    dead
}

/// `DoHeat`: warm up next to a burner while the village has heat to spend,
/// otherwise cool down. Proximity is only re-checked while the meter is at
/// least two points below full, so a warm Eskimo stays warm until it tops
/// up. A failed purchase turns the Eskimo cold without cooling it this frame.
fn update_heat(eskimo: &mut Eskimo, economy: &mut Economy, near_burner: bool) {
    if eskimo.heat + 2.0 <= MAX_NEED {
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

/// `DoFood`: eating starts below 20 and continues until the meter passes 80.
/// An idle Eskimo gets hungrier; an eating one gains food instead while the
/// village has food, and stops eating (without getting hungrier this frame)
/// when it has none.
fn update_food(eskimo: &mut Eskimo, economy: &mut Economy) {
    if eskimo.food < 20.0 {
        eskimo.eating = true;
    } else if eskimo.food > 80.0 {
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
