//! Per-Eskimo food/heat/health bookkeeping, ported from `unit_eskimos.lua`.

use alloc::vec::Vec;

use crate::state::{self, Economy, Units};
use crate::units::set_eskimo;

pub fn update_eskimos(units: &mut Units, economy: &mut Economy) -> Vec<i32> {
    let burner_positions: Vec<spring::Float3> = units
        .burners
        .iter()
        .filter_map(|id| spring::get_unit_position(*id, false, false).ok())
        .collect();
    let sync_lane = units.eskimo_sync_cursor;
    for (index, eskimo) in units.eskimos.iter_mut().enumerate() {
        eskimo.food = (eskimo.food - 1.0 / 33.0).max(0.0);
        let near_burner = spring::get_unit_position(eskimo.id, false, false)
            .ok()
            .is_some_and(|position| {
                burner_positions.iter().any(|burner| {
                    let dx = position.x - burner.x;
                    let dz = position.z - burner.z;
                    dx * dx + dz * dz <= 300.0 * 300.0
                })
            });
        // unit_eskimos.lua only reevaluates proximity while the personal heat
        // meter is at least two points below full. Preserve that sticky warm
        // state at the cap rather than immediately cooling someone who steps
        // away from the burner on the next frame.
        if eskimo.heat + 2.0 <= 100.0 {
            eskimo.warm = near_burner;
        }
        if eskimo.warm {
            if economy.heat >= 0.5 / 33.0 {
                economy.heat -= 0.5 / 33.0;
                eskimo.heat = (eskimo.heat + 4.0 / 33.0).min(100.0);
            } else {
                // GG.Resources.Consume failure changes WARM to COLD before
                // applying the normal cold decay in the original gadget.
                eskimo.warm = false;
                eskimo.heat = (eskimo.heat - 1.0 / 33.0).max(0.0);
            }
        } else {
            eskimo.heat = (eskimo.heat - 1.0 / 33.0).max(0.0);
        }
        if eskimo.heat >= 100.0 {
            eskimo.warm = false;
        }
        // The authored EATING state begins below 20 and remains active until
        // the meter passes 80. A single-frame top-up left the port's Eskimos
        // permanently near starvation.
        if eskimo.food < 20.0 {
            eskimo.eating = true;
        } else if eskimo.food > 80.0 {
            eskimo.eating = false;
        }
        if eskimo.eating && economy.food >= 0.1 / 33.0 {
            economy.food -= 0.1 / 33.0;
            eskimo.food = (eskimo.food + 10.0 / 33.0).min(100.0);
        }
        if eskimo.heat <= 0.0 {
            eskimo.health -= 1.0 / 33.0;
        }
        if eskimo.food <= 0.0 {
            eskimo.health -= 0.2 / 33.0;
        }
        // These public values drive per-unit HUD effects only. Rotating five
        // lanes keeps every value fresh within five frames without turning a
        // 20-person reinforcement wave into sixty synchronous host calls.
        if index % 5 == sync_lane {
            set_eskimo(eskimo.id, eskimo.health.max(0.0), eskimo.food, eskimo.heat);
        }
    }
    units.eskimo_sync_cursor = (sync_lane + 1) % 5;
    let dead: Vec<i32> = units
        .eskimos
        .iter()
        .filter(|unit| unit.health <= 0.0)
        .map(|unit| unit.id)
        .collect();
    state::publish_resources(economy);
    dead
}
