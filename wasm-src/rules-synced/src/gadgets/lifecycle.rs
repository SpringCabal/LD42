use spring_addons::rules::{Gadget, UnitDestroyedEvent};

use crate::defs::{self, UnitKind};
use spring::rules_params::los::PUBLIC;

use crate::state::{DrillShip, Eskimo, Global};
use crate::unit_scripts;
use crate::units::set_eskimo_needs;

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
        unit_scripts::queue_unit_created(id, def);
        let _ = spring::set_unit_rules_param_f32(id, "health", defs::health(def), PUBLIC);
        let Some(kind) = defs::kind(def) else {
            return;
        };
        if kind.is_scenery() {
            let _ = spring::set_unit_rules_param_f32(id, "untargetable", 1.0, PUBLIC);
        }

        let mut units = ctx.global().units.access_mut();
        match kind {
            UnitKind::Coalburner if !units.burners.contains(&id) => units.burners.push(id),
            UnitKind::Eskimo if !units.eskimos.iter().any(|unit| unit.id == id) => {
                units.eskimos.push(Eskimo {
                    id,
                    food: 90.0,
                    heat: 40.0,
                    eating: false,
                    warm: false,
                });
                set_eskimo_needs(id, 90.0, 40.0);
            }
            UnitKind::Drillship if !units.drills.iter().any(|ship| ship.id == id) => {
                units.drills.push(DrillShip::new(id));
            }
            UnitKind::Parachute if !units.parachutes.contains(&id) => units.parachutes.push(id),
            UnitKind::Pirate if !units.pirates.contains(&id) => units.pirates.push(id),
            _ => {}
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
