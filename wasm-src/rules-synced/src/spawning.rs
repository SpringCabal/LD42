use spring::{Float3, ResultExt};

use crate::defs::{self, UnitKind};
use crate::state::Story;

#[derive(Clone, Copy)]
pub struct SpawnRequest {
    pub kind: UnitKind,
    pub position: Float3,
    pub team: i32,
}

pub fn spawn(
    ctx: &spring_addons::AddonContext<'_, crate::state::Global>,
    request: SpawnRequest,
    spawned: impl FnOnce(i32) + spring::RetainedCapture + 'static,
) {
    let spring = ctx.spring();
    let kind = request.kind;
    let Some(id) = defs::def_id(spring, kind) else {
        return;
    };
    let team = if kind.is_scenery() {
        spring.game().get_gaia_team_id().unwrap_or(request.team)
    } else {
        request.team
    };
    ctx.commands()
        .queue(move |spring| {
            spring.unit_control().create_unit(
                &spring::unit_control::DefRef {
                    name: kind.def_name().into(),
                    id,
                },
                request.position,
                0,
                team,
                spring::unit_control::CreateUnitOptions::default(),
            )
        })
        .then(move |spring, created| {
            let unit_id = match created {
                Ok(unit_id) => unit_id,
                Err(error) => {
                    spring::log_warning!(
                        spring,
                        "mycube",
                        "the story could not create a {} at {:?}: {error:?}",
                        kind.def_name(),
                        request.position
                    );
                    return;
                }
            };
            if kind.is_scenery() {
                spring
                    .unit_control()
                    .set_unit_neutral(unit_id, true)
                    .log_err(spring, "making the scenery neutral failed");
                spring
                    .unit_control()
                    .set_unit_blocking(
                        unit_id,
                        spring::unit_control::SetUnitBlockingOptions {
                            blocking: true,
                            solid_objects: true,
                            projectiles: false,
                            quad_map_rays: false,
                            crushable: false,
                            block_enemy_pushing: true,
                            block_height_changes: false,
                        },
                    )
                    .log_err(spring, "making the scenery blocking failed");
            }
            spawned(unit_id);
        });
}

pub fn count_story_spawn(story: &mut Story, kind: UnitKind) {
    match kind {
        UnitKind::Eskimo => story.story_eskimos += 1,
        UnitKind::Drillship => story.story_drills += 1,
        UnitKind::Parachute => story.story_parachutes += 1,
        UnitKind::Pirate => story.story_pirates += 1,
        _ => {}
    }
}
