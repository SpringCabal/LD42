use core::cell::Cell;

use spring_addons::rules::Gadget;

use crate::spawning;
use crate::state::Global;
use crate::units::destroy_all;

/// The authored scenario script, ported from `storyteller.lua`.
#[derive(Default)]
pub struct StorytellerGadget {
    /// Off in the verification gallery (mod option `gallery`), which
    /// photographs one unit at a time: the story's village and ships would
    /// stand in front of it.
    off: Cell<bool>,
}

impl StorytellerGadget {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Gadget<Global> for StorytellerGadget {
    fn name(&self) -> &'static str {
        "Storyteller"
    }

    fn init(&self, _ctx: &spring_addons::AddonContext<'_, Global>) {
        let gallery = spring::game_options::get_mod_option("gallery")
            .ok()
            .flatten()
            .is_some_and(|mode| !mode.is_empty());
        self.off.set(gallery);
    }

    fn game_frame(&self, ctx: &spring_addons::AddonContext<'_, Global>, frame: i32) {
        if self.off.get() {
            return;
        }
        let mut story = ctx.global().story.access_mut();
        if !story.started {
            story.started = true;
            story.start_frame = frame;
            // storyteller.lua's Initialize clears the map and restores
            // the ice an earlier round dug away.
            destroy_all(spring::get_all_units().unwrap_or_default(), true);
            crate::story::restore_ice();
        }
        for request in crate::story::run_story(&mut story, frame) {
            spawning::spawn(request, move |_| {
                crate::with_global(|global| {
                    spawning::count_story_spawn(&mut global.story.access_mut(), request.kind);
                });
            });
        }
    }
}
