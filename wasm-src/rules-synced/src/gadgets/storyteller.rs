use spring_addons::rules::Gadget;

use crate::state::Global;

/// The authored scenario script, ported from `storyteller.lua`.
#[derive(Default)]
pub struct StorytellerGadget;

impl StorytellerGadget {
    pub fn new() -> Self {
        Self
    }
}

impl Gadget<Global> for StorytellerGadget {
    fn name(&self) -> &'static str {
        "Storyteller"
    }

    fn game_frame(&self, ctx: &spring_addons::AddonContext<'_, Global>, frame: i32) {
        let requests = crate::story::run_story(&mut ctx.global().story.access_mut(), frame);
        ctx.global().pending.access_mut().spawns.extend(requests);
    }
}
