use spring_addons::rules::Gadget;

use crate::state::{GameEnd, Global};

#[derive(Default)]
pub struct GameEndGadget;

impl GameEndGadget {
    pub fn new() -> Self {
        Self
    }
}

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
