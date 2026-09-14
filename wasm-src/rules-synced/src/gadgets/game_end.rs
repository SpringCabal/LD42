use spring_addons::rules::Gadget;

use crate::state::{self, Global};

/// Loss detection, ported from `game_end.lua`.
#[derive(Default)]
pub struct GameEndGadget;

impl GameEndGadget {
    pub fn new() -> Self {
        Self
    }
}

impl Gadget<Global> for GameEndGadget {
    fn name(&self) -> &'static str {
        "GameEnd"
    }

    fn game_frame(&self, ctx: &spring_addons::AddonContext<'_, Global>, frame: i32) {
        if ctx.global().units.access().eskimos.is_empty() && frame > 5 {
            state::story_game_end(&mut ctx.global().story.access_mut(), "loss");
        }
    }
}
