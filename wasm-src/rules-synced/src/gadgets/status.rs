use mycube_shared::Status;
use spring_addons::rules::Gadget;
use springcabal::message::SendOnChange;

use crate::state::Global;

#[derive(Default)]
pub struct StatusGadget {
    status: SendOnChange<Status<'static>>,
}

impl StatusGadget {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Gadget<Global> for StatusGadget {
    fn name(&self) -> &'static str {
        "Status"
    }

    fn game_frame(&self, ctx: &spring_addons::AddonContext<'_, Global>, _frame: i32) {
        let economy = *ctx.global().economy.access();
        let status = {
            let story = ctx.global().story.access();
            Status {
                food: economy.food.round() as i32,
                heat: economy.heat.round() as i32,
                money: economy.money.round() as i32,
                game_end: story.game_end,
                survival_frames: story.survival_frames(),
                intro: story.intro_event,
            }
        };
        self.status.send_if_changed(status, |status| {
            springcabal::message::send_to_ui(ctx.spring(), &status.encode())
        });
    }
}
