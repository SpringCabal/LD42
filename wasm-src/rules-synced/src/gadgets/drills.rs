use spring_addons::rules::{Gadget, UnitDestroyedEvent};

use crate::defs::{self, UnitKind};
use crate::drills::{self, DrillShip};
use crate::state::Global;
use crate::terrain;

/// Frames between drill checks.
const DRILL_INTERVAL: i32 = 30;

/// The autonomous drill controller, ported from `unit_drill.lua`.
#[derive(Default)]
pub struct DrillsGadget;

impl DrillsGadget {
    pub fn new() -> Self {
        Self
    }
}

impl Gadget<Global> for DrillsGadget {
    fn name(&self) -> &'static str {
        "Drills"
    }

    fn unit_created(
        &self,
        ctx: &spring_addons::AddonContext<'_, Global>,
        id: i32,
        def: i32,
        _team: i32,
        _builder: i32,
    ) {
        if defs::kind(def) == Some(UnitKind::Drillship) {
            ctx.global().drills.access_mut().ships.push(DrillShip {
                id,
                ..DrillShip::default()
            });
        }
    }

    fn unit_destroyed(
        &self,
        ctx: &spring_addons::AddonContext<'_, Global>,
        event: &UnitDestroyedEvent,
    ) {
        let id = event.unit_id;
        ctx.global()
            .drills
            .access_mut()
            .ships
            .retain(|ship| ship.id != id);
    }

    fn game_frame(&self, ctx: &spring_addons::AddonContext<'_, Global>, frame: i32) {
        if frame % DRILL_INTERVAL == 0 {
            let orders = drills::update_drills(&mut ctx.global().drills.access_mut());
            // Giving an order raises AllowCommand, so the state is released first.
            drills::give_orders(&orders);
        } else {
            terrain::process_terrain_jobs(&mut ctx.global().drills.access_mut().jobs);
        }
    }
}
