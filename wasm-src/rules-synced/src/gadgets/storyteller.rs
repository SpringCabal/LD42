use spring_addons::rules::Gadget;

use crate::spawning;
use crate::state::Global;
use crate::units::destroy_later;

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
        let requests = {
            let mut story = ctx.global().story.access_mut();
            if !story.started {
                story.started = true;
                // storyteller.lua clears the map's own units before its
                // scenario starts.
                destroy_later(ctx, spring::get_all_units().unwrap_or_default(), true);
            }
            crate::story::run_story(&mut story, frame)
        };
        if requests.is_empty() {
            return;
        }
        // Creating a unit raises UnitCreated immediately, and several
        // gadgets borrow state there, so spawn once this callin is over.
        ctx.delay(move |global| {
            for request in requests {
                if spawning::spawn(request).is_some() {
                    spawning::count_story_spawn(&mut global.story.access_mut(), request.kind);
                }
            }
        });
    }
}
