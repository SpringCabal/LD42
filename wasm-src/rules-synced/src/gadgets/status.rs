use spring_addons::rules::Gadget;

use crate::state::Global;

#[derive(Default)]
pub struct StatusGadget;

impl StatusGadget {
    pub fn new() -> Self {
        Self
    }
}

impl Gadget<Global> for StatusGadget {
    fn name(&self) -> &'static str {
        "Status"
    }

    fn game_frame(&self, ctx: &spring_addons::AddonContext<'_, Global>, frame: i32) {
        if frame % 15 != 0 {
            return;
        }
        let economy = *ctx.global().economy.access();
        let story = ctx.global().story.access();
        let _ = spring::send_to_unsynced(&alloc::format!(
            "MYCUBE_STATUS|{:.0}|{:.0}|{:.0}|{}|{}|{}",
            economy.food,
            economy.heat,
            economy.money,
            ctx.global().units.access().eskimos.len(),
            story.game_end,
            story.intro_event,
        ));
    }
}
