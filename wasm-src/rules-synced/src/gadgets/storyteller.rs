mod story;

use core::cell::Cell;

use spring_addons::rules::Gadget;

use crate::state::Global;
use crate::units::destroy_all;

#[derive(Default)]
pub struct StorytellerGadget {
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
        let spring = _ctx.spring();
        let gallery = spring
            .game()
            .get_mod_option("gallery")
            .ok()
            .filter(|mode| mode.exists)
            .map(|mode| mode.value)
            .is_some_and(|mode| !mode.is_empty());
        self.off.set(gallery);
    }

    fn game_frame(&self, ctx: &spring_addons::AddonContext<'_, Global>, frame: i32) {
        let spring = ctx.spring();
        if self.off.get() {
            return;
        }
        let mut story = ctx.global().story.access_mut();
        if !story.started {
            story.started = true;
            story.start_frame = frame;
            destroy_all(
                ctx,
                spring.units().get_all_units().unwrap_or_default(),
                true,
            );
            story::restore_ice(spring);
        }
        let batch = story::run_story(&mut story, frame);
        drop(story);
        if let Some(batch) = batch {
            story::spawn_next(batch, 0);
        }
    }
}
