use spring_addons::rules::Gadget;

use crate::spawning;
use crate::state::{self, Global};
use crate::units;

#[derive(Default)]
pub struct ReaperGadget;

impl ReaperGadget {
    pub fn new() -> Self {
        Self
    }
}

impl Gadget<Global> for ReaperGadget {
    fn name(&self) -> &'static str {
        "Reaper"
    }

    fn game_frame(&self, ctx: &spring_addons::AddonContext<'_, Global>, _frame: i32) {
        let global = ctx.global();
        let (spawns, destroys) = {
            let mut pending = global.pending.access_mut();
            (
                core::mem::take(&mut pending.spawns),
                core::mem::take(&mut pending.destroys),
            )
        };
        for request in spawns {
            // Creating a unit raises UnitCreated immediately, and the
            // lifecycle gadget borrows `units` there, so nothing may be
            // borrowed across `spawn`.
            if state::spawn(request.name, request.position, request.team).is_some() {
                spawning::count_story_spawn(&mut global.story.access_mut(), request.name);
            }
        }
        for request in destroys {
            units::destroy(request.id, request.reclaimed);
        }
    }
}
