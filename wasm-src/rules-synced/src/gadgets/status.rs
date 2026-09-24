use spring_addons::rules::Gadget;
use springcabal::message::{Encoder, SendOnChange};

use crate::state::{GameEnd, Global};

/// What the HUD shows.
#[derive(Clone, Copy, PartialEq)]
struct Status {
    food: i32,
    heat: i32,
    money: i32,
    game_end: Option<GameEnd>,
    survival_frames: Option<i32>,
    intro: &'static str,
}

impl Status {
    fn encode(&self) -> alloc::string::String {
        Encoder::new("MYCUBE_STATUS")
            .field(self.food)
            .field(self.heat)
            .field(self.money)
            .field(self.game_end.map_or("", GameEnd::as_str))
            .field(self.survival_frames.unwrap_or(-1))
            .field(self.intro)
            .finish()
    }
}

/// Sends the HUD state (resources, story text, game end) to the UI as
/// `MYCUBE_STATUS|food|heat|money|end|survived|intro`, whenever it changes.
/// `survived` is the round's length in frames once it has ended, else -1.
#[derive(Default)]
pub struct StatusGadget {
    status: SendOnChange<Status>,
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
            spring::send_to_unsynced(&status.encode())
                .inspect_err(|error| {
                    spring::log_warning!("mycube", "sending the HUD status failed: {error:?}");
                })
                .is_ok()
        });
    }
}
