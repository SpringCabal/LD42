use spring_addons::rules::Gadget;

use crate::eskimos;
use crate::state::Global;

/// Eskimo food/heat/health bookkeeping, ported from `unit_eskimos.lua`.
#[derive(Default)]
pub struct EskimosGadget;

impl EskimosGadget {
    pub fn new() -> Self {
        Self
    }
}

impl Gadget<Global> for EskimosGadget {
    fn name(&self) -> &'static str {
        "Eskimos"
    }

    fn game_frame(&self, ctx: &spring_addons::AddonContext<'_, Global>, _frame: i32) {
        let dead = eskimos::update_eskimos(
            &mut ctx.global().units.access_mut(),
            &mut ctx.global().economy.access_mut(),
        );
        let mut pending = ctx.global().pending.access_mut();
        for id in dead {
            pending.destroy(id, false);
        }
    }
}
