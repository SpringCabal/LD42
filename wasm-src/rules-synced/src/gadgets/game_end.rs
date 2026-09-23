use spring_addons::rules::Gadget;

use crate::state::{GameEnd, Global};

/// Loss detection, ported from `game_end.lua`.
#[derive(Default)]
pub struct GameEndGadget;

impl GameEndGadget {
    pub fn new() -> Self {
        Self
    }
}

/// The opening Eskimos arrive on frame 0; don't call a loss before that.
const FIRST_CHECK_FRAME: i32 = 6;

impl Gadget<Global> for GameEndGadget {
    fn name(&self) -> &'static str {
        "GameEnd"
    }

    fn game_frame(&self, ctx: &spring_addons::AddonContext<'_, Global>, frame: i32) {
        if frame >= FIRST_CHECK_FRAME && ctx.global().village.access().eskimos.is_empty() {
            ctx.global()
                .story
                .access_mut()
                .end_game(GameEnd::Loss, frame);
        }
    }
}
