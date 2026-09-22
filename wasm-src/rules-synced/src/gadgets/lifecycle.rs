use core::cell::Cell;

use spring_addons::rules::{Gadget, UnitDestroyedEvent};

use crate::health;
use crate::state::{DrillShip, Eskimo, Global, PUBLIC};
use crate::unit_scripts;
use crate::units::set_eskimo;

#[derive(Default)]
pub struct LifecycleGadget {
    swept: Cell<bool>,
}

impl LifecycleGadget {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Gadget<Global> for LifecycleGadget {
    fn name(&self) -> &'static str {
        "Lifecycle"
    }

    fn game_frame(&self, _ctx: &spring_addons::AddonContext<'_, Global>, _frame: i32) {
        // Map-authored units exist before the Core rules module receives its
        // first UnitCreated callin. Queue them once so their CUS instances
        // are attached just like units spawned by the Rust story.
        if self.swept.replace(true) {
            return;
        }
        if let Ok(units) = spring::get_all_units() {
            for id in units {
                if let Ok(def) = spring::get_unit_def_id(id) {
                    unit_scripts::queue_unit_created(id, def.0);
                }
            }
        }
    }

    fn unit_created(
        &self,
        ctx: &spring_addons::AddonContext<'_, Global>,
        id: i32,
        def: i32,
        _team: i32,
        _builder: i32,
    ) {
        unit_scripts::queue_unit_created(id, def);
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
                    drowning: false,
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
    }
}
