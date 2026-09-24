use spring_addons::rules::{Gadget, UnitDestroyedEvent};

use crate::defs::{self, UnitKind};
use crate::drills::{self, DrillShip};
use crate::state::Global;
use crate::terrain;

/// Frames between the checks for nearby ice (`CHECK_FRAME_INTERVAL`).
const CHECK_INTERVAL: i32 = 30;

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
            let frame = spring::game::current_frame();
            ctx.global()
                .drills
                .access_mut()
                .ships
                .push(DrillShip::new(id, frame));
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
        let mut state = ctx.global().drills.access_mut();
        drills::drill(&mut state, frame);
        if frame % CHECK_INTERVAL == 0 {
            let orders = drills::update_drills(&mut state);
            // Giving an order raises AllowCommand, so release the state first.
            drop(state);
            drills::give_orders(&orders);
        } else {
            // Height-map edits are metered host work; keep them off the
            // frames that search for ice.
            terrain::process_terrain_jobs(&mut state.jobs);
        }
    }
}
