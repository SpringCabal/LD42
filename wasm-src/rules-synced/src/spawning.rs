//! Storyteller spawn bookkeeping, ported from `storyteller.lua`.

use crate::state::Story;

/// Count a unit the story spawned. The lifecycle gadget registers it.
pub fn count_story_spawn(story: &mut Story, name: &str) {
    match name {
        "eskimo" => story.story_eskimos += 1,
        "drillship" => story.story_drills += 1,
        "parachut" => story.story_parachutes += 1,
        "pirate" => story.story_pirates += 1,
        _ => {}
    }
}
