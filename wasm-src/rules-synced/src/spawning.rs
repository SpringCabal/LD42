//! Storyteller spawn bookkeeping, ported from `storyteller.lua`.

use crate::defs::UnitKind;
use crate::state::Story;

/// Count a unit the story spawned. The lifecycle gadget registers it.
pub fn count_story_spawn(story: &mut Story, kind: UnitKind) {
    match kind {
        UnitKind::Eskimo => story.story_eskimos += 1,
        UnitKind::Drillship => story.story_drills += 1,
        UnitKind::Parachute => story.story_parachutes += 1,
        UnitKind::Pirate => story.story_pirates += 1,
        _ => {}
    }
}
