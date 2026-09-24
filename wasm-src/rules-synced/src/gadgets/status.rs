use alloc::string::String;

use spring_addons::rules::Gadget;
use springcabal::message::{Encoder, SendOnChange};

use crate::state::Global;

/// Sends the HUD state (resources, story text, game end) to the UI as
/// `MYCUBE_STATUS|food|heat|money|end|survived|intro`, whenever it changes.
/// `survived` is the round's length in frames once it has ended, else -1.
#[derive(Default)]
pub struct StatusGadget {
    status: SendOnChange<String>,
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
            Encoder::new("MYCUBE_STATUS")
                .field(format_args!("{:.0}", economy.food))
                .field(format_args!("{:.0}", economy.heat))
                .field(format_args!("{:.0}", economy.money))
                .field(story.game_end.map_or("", |end| end.as_str()))
                .field(story.survival_frames().unwrap_or(-1))
                .field(&story.intro_event)
                .finish()
        };
        self.status.send_if_changed(status, |status| {
            spring::send_to_unsynced(status)
                .inspect_err(|error| {
                    spring::log_warning!("mycube", "sending the HUD status failed: {error:?}");
                })
                .is_ok()
        });
    }
}
