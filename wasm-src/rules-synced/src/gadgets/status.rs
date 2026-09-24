use alloc::string::String;
use core::cell::RefCell;
use core::fmt::Write;

use spring_addons::rules::Gadget;

use crate::state::Global;

/// Sends the HUD state (resources, story text, game end) to the UI as
/// `MYCUBE_STATUS|food|heat|money|end|end_frame|intro`, whenever it changes.
#[derive(Default)]
pub struct StatusGadget {
    last_sent: RefCell<String>,
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
        let story = ctx.global().story.access();
        let mut status = String::new();
        let _ = write!(
            status,
            "MYCUBE_STATUS|{:.0}|{:.0}|{:.0}|{}|{}|{}",
            economy.food,
            economy.heat,
            economy.money,
            story.game_end.map_or("", |end| end.as_str()),
            story.end_frame.unwrap_or(-1),
            story.intro_event,
        );
        let mut last_sent = self.last_sent.borrow_mut();
        if *last_sent != status {
            if let Err(error) = spring::send_to_unsynced(&status) {
                spring::log_warning!("mycube", "sending the HUD status failed: {error:?}");
            }
            *last_sent = status;
        }
    }
}
