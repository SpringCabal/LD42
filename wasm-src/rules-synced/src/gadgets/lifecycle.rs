use spring_addons::rules::{Gadget, UnitDestroyedEvent};

use crate::health;
use crate::state::{DrillShip, Eskimo, Global, PUBLIC};
use crate::units::set_eskimo;

#[derive(Default)]
pub struct LifecycleGadget;

impl LifecycleGadget {
    pub fn new() -> Self {
        Self
    }
}

impl Gadget<Global> for LifecycleGadget {
    fn name(&self) -> &'static str {
        "Lifecycle"
    }

    fn unit_created(
        &self,
        ctx: &spring_addons::AddonContext<'_, Global>,
        id: i32,
        def: i32,
        _team: i32,
        _builder: i32,
    ) {
        // `game_health.lua` owns a separate, gameplay-facing health pool.
        // Spring's engine health is deliberately not the authority here.
        let unit_health = health::gameplay_health(def);
        let _ = spring::set_unit_rules_param_f32(id, "health", unit_health, PUBLIC);
        if ["coalburner", "iglu", "lantern"]
            .iter()
            .any(|name| spring::get_unit_def_id_by_name(name).ok() == Some(def))
        {
            let _ = spring::set_unit_rules_param_f32(id, "untargetable", 1.0, PUBLIC);
        }

        let named = |name: &str| spring::get_unit_def_id_by_name(name).ok() == Some(def);
        let mut units = ctx.global().units.access_mut();
        if named("eskimo") {
            if !units.eskimos.iter().any(|unit| unit.id == id) {
                units.eskimos.push(Eskimo {
                    id,
                    health: unit_health,
                    food: 90.0,
                    heat: 40.0,
                    eating: false,
                    warm: false,
                });
                set_eskimo(id, unit_health, 90.0, 40.0);
            }
        } else if named("drillship") {
            if !units.drills.iter().any(|ship| ship.id == id) {
                units.drills.push(DrillShip::new(id));
            }
        } else if named("parachut") {
            if !units.parachutes.contains(&id) {
                units.parachutes.push(id);
            }
        } else if named("pirate") && !units.pirates.contains(&id) {
            units.pirates.push(id);
        }
    }

    fn unit_destroyed(
        &self,
        ctx: &spring_addons::AddonContext<'_, Global>,
        event: &UnitDestroyedEvent,
    ) {
        let id = event.unit_id;
        let mut units = ctx.global().units.access_mut();
        units.eskimos.retain(|unit| unit.id != id);
        units.burners.retain(|burner| *burner != id);
        units.drills.retain(|ship| ship.id != id);
        units.parachutes.retain(|unit| *unit != id);
        units.pirates.retain(|unit| *unit != id);
        units.drowning_animated.retain(|unit| *unit != id);
    }
}
