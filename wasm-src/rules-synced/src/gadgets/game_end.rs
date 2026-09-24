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

/// `game_end.lua` checks for a loss only once five frames of the round have
/// passed; the opening Eskimos arrive on its first frame.
const GRACE_FRAMES: i32 = 5;

impl Gadget<Global> for GameEndGadget {
    fn name(&self) -> &'static str {
        "GameEnd"
    }

    fn game_frame(&self, ctx: &spring_addons::AddonContext<'_, Global>, frame: i32) {
        let mut story = ctx.global().story.access_mut();
        if story.started
            && frame > story.start_frame + GRACE_FRAMES
            && ctx.global().village.access().eskimos.is_empty()
        {
            story.end_game(GameEnd::Loss, frame);
        }
    }
}
