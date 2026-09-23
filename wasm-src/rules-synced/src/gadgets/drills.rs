use spring_addons::rules::Gadget;

use crate::state::Global;
use crate::{drills, terrain};

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

    fn game_frame(&self, ctx: &spring_addons::AddonContext<'_, Global>, frame: i32) {
        if frame % DRILL_INTERVAL == 0 {
            let orders = drills::update_drills(
                &mut ctx.global().units.access_mut(),
                &mut ctx.global().terrain.access_mut(),
            );
            drills::give_orders(&orders);
        } else {
            terrain::process_terrain_jobs(&mut ctx.global().terrain.access_mut());
        }
    }
}
