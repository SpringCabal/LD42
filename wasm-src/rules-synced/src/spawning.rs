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
    request: impl FnOnce(&spring::Spring<'_, spring::RulesSynced>) -> SpawnRequest
    + spring::RetainedCapture
    + 'static,
    spawned: impl FnOnce(SpawnRequest, Option<i32>) + spring::RetainedCapture + 'static,
) {
    ctx.commands()
        .queue(move |spring| {
            let request = request(spring);
            let kind = request.kind;
            let Some(id) = defs::def_id(spring, kind) else {
                return (request, None);
            };
            let team = if kind.is_scenery() {
                spring.game().get_gaia_team_id().unwrap_or(request.team)
            } else {
                request.team
            };
            let created = spring.unit_control().create_unit(
                &spring::unit_control::DefRef {
                    name: kind.def_name().into(),
                    id,
                },
                request.position,
                0,
                team,
                spring::unit_control::CreateUnitOptions {
                    flatten_ground: true,
                    ..Default::default()
                },
            );
            (request, Some(created))
        })
        .then(move |spring, (request, created)| {
            let kind = request.kind;
            let Some(created) = created else {
                spawned(request, None);
                return;
            };
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
                    spawned(request, None);
                    return;
                }
            };
            // s11n reapplies position and preserves rotation after CreateUnit.
            // ForcedMove updates native movement/blocking state; restoring the
            // Euler rotation also matches the original building orientation.
            if let Ok(rotation) = spring.units_info().get_unit_rotation(unit_id) {
                spring
                    .unit_control()
                    .set_unit_position(unit_id, request.position)
                    .log_err(spring, "placing the new story unit failed");
                spring
                    .unit_control()
                    .set_unit_rotation(
                        unit_id,
                        Float3::new(rotation.pitch, rotation.yaw, rotation.roll),
                    )
                    .log_err(spring, "restoring the new story unit rotation failed");
            }
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
            spawned(request, Some(unit_id));
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
