//! Creating the story's units, ported from `storyteller.lua`.

use spring::Float3;

use crate::defs::{self, UnitKind};
use crate::state::Story;

#[derive(Clone, Copy)]
pub struct SpawnRequest {
    pub kind: UnitKind,
    pub position: Float3,
    pub team: i32,
}

/// Create a story unit. Scenery goes to Gaia, neutral and blocking.
pub fn spawn(request: SpawnRequest) -> Option<i32> {
    let kind = request.kind;
    let id = defs::def_id(kind)?;
    let team = if kind.is_scenery() {
        spring::game::get_gaia_team_id(0).unwrap_or(request.team)
    } else {
        request.team
    };
    let unit_id = spring::create_unit(
        spring::UnitDefRef {
            name: kind.def_name(),
            id,
        },
        request.position,
        team,
        spring::CreateUnitOptions {
            build: false,
            flatten_ground: false,
            unit_id: -1,
            builder_id: -1,
        },
    )
    .ok()?;
    if kind.is_scenery() {
        let _ = spring::set_unit_neutral(unit_id, true);
        let _ = spring::set_unit_blocking(
            unit_id,
            spring::types::SetUnitBlockingOptions {
                blocking: true,
                solid_objects: true,
                projectiles: false,
                quad_map_rays: false,
                crushable: false,
                block_enemy_pushing: true,
                block_height_changes: false,
            },
        );
    }
    Some(unit_id)
}

/// Count a unit the story spawned.
pub fn count_story_spawn(story: &mut Story, kind: UnitKind) {
    match kind {
        UnitKind::Eskimo => story.story_eskimos += 1,
        UnitKind::Drillship => story.story_drills += 1,
        UnitKind::Parachute => story.story_parachutes += 1,
        UnitKind::Pirate => story.story_pirates += 1,
        _ => {}
    }
}
