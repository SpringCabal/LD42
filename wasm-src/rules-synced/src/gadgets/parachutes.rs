use spring_addons::rules::Gadget;

use crate::parachutes;
use crate::state::Global;

/// Parachute cleanup, ported from `unit_parachute_remover.lua`.
#[derive(Default)]
pub struct ParachutesGadget;

impl ParachutesGadget {
    pub fn new() -> Self {
        Self
    }
}

impl Gadget<Global> for ParachutesGadget {
    fn name(&self) -> &'static str {
        "Parachutes"
    }

    fn game_frame(&self, ctx: &spring_addons::AddonContext<'_, Global>, _frame: i32) {
        let landed = parachutes::update_parachutes(&ctx.global().units.access().parachutes);
        let mut pending = ctx.global().pending.access_mut();
        for id in landed {
            pending.destroy(id, true);
        }
    }
}
