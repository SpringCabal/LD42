use spring::Float3;

use crate::defs::UnitKind::{self, *};
use crate::spawning::{self, SpawnRequest};
use crate::state::{GameEnd, Story};
use mycube_shared::round::LUA_FRAMES_PER_SECOND;

#[derive(Clone, Copy)]
pub struct SpawnBatch {
    units: &'static [UnitKind],
    team: i32,
}

pub fn run_story(state: &mut Story, frame: i32) -> Option<SpawnBatch> {
    if frame < state.next_step {
        return None;
    }
    let (message, units, team, wait): (&str, &[UnitKind], i32, i32) = match state.step {
        0 => ("", &INIT[..], 0, 1),
        1 => (
            "The last remaining good people, bundled on a block of ice.",
            &[][..],
            0,
            7,
        ),
        2 => ("Survive.", &[][..], 0, 5),
        3 => ("", &PIRATE[..], 1, 10),
        4 => (
            "Your people are cold. Send them to the stove to warm up.",
            &[][..],
            0,
            20,
        ),
        5 => ("They come for Ice.", &[][..], 0, 1),
        6 => ("", &DRILLS_3[..], 1, 60),
        7 => (
            "Your people are hungry and cold. Buy more: \nCoal: Press 2\n Food: Press 3",
            &[][..],
            0,
            10,
        ),
        8 => ("They are relentless.", &[][..], 0, 3),
        9 => ("", &PIRATE_DRILL[..], 1, 30),
        10 => (
            "Sell the Ice to feed and heat your people.\n\n(Press 1 and Left Mouse Button)",
            &[][..],
            0,
            10,
        ),
        11 => ("", &RAID[..], 1, 15),
        12 => ("Reinforcements. Just in time!", &[][..], 0, 1),
        13 => ("", &ESKIMOS_10[..], 0, 50),
        14 => ("The ice...", &[][..], 0, 3),
        15 => ("", &DRILLS_5[..], 1, 160),
        _ => {
            state.end_game(GameEnd::Victory, frame);
            return None;
        }
    };
    if !message.is_empty() {
        state.intro_event = message;
    }
    state.step += 1;
    state.next_step = frame.saturating_add(wait.saturating_mul(LUA_FRAMES_PER_SECOND));
    (!units.is_empty()).then_some(SpawnBatch { units, team })
}

const IGLU_MIN_DISTANCE: f32 = 400.0;
const MAX_PLACEMENT_TRIES: u32 = 1000;

// Unit creation and Create scripts consume the same synced RNG as placement.
// Queue the next random placement only after this unit and its companion exist,
// preserving Lua's SpawnUnits -> s11n:Add -> companion -> next-unit sequence.
pub fn spawn_next(batch: SpawnBatch, index: usize) {
    let Some(&unit) = batch.units.get(index) else {
        return;
    };
    crate::with_addon_context(|ctx| {
        spawning::spawn(
            ctx,
            move |spring| {
                let (x, y, z) = place(spring, unit);
                SpawnRequest {
                    kind: unit,
                    position: Float3::new(x, y, z),
                    team: batch.team,
                }
            },
            move |request, id| {
                if id.is_none() {
                    spawn_next(batch, index + 1);
                    return;
                }
                crate::with_global(|global| {
                    spawning::count_story_spawn(&mut global.story.access_mut(), request.kind)
                });
                let companion = match request.kind {
                    Iglu => Lantern,
                    Pirate => Parachute,
                    _ => {
                        spawn_next(batch, index + 1);
                        return;
                    }
                };
                crate::with_addon_context(|ctx| {
                    spawning::spawn(
                        ctx,
                        move |spring| {
                            let mut position = request.position;
                            if companion == Lantern {
                                position.x +=
                                    random_int(spring, 130, 150) as f32 * random_sign(spring);
                                position.z +=
                                    random_int(spring, 130, 150) as f32 * random_sign(spring);
                                position.y = spring
                                    .terrain()
                                    .get_ground_height(position.x, position.z)
                                    .unwrap_or(position.y);
                            } else {
                                position.y += 100.0;
                            }
                            SpawnRequest {
                                kind: companion,
                                position,
                                team: batch.team,
                            }
                        },
                        move |request, id| {
                            if id.is_some() {
                                crate::with_global(|global| {
                                    spawning::count_story_spawn(
                                        &mut global.story.access_mut(),
                                        request.kind,
                                    )
                                });
                            }
                            spawn_next(batch, index + 1);
                        },
                    );
                });
            },
        );
    });
}

fn place(spring: &spring::Spring<'_, spring::RulesSynced>, unit: UnitKind) -> (f32, f32, f32) {
    let mut candidate = find_position(spring, unit);
    if !is_support(unit) {
        return candidate;
    }
    for attempt in 0..MAX_PLACEMENT_TRIES {
        let found = spring
            .units_query()
            .get_units_in_cylinder(candidate.0, candidate.2, IGLU_MIN_DISTANCE, -1)
            .unwrap_or_default()
            .into_iter()
            .any(|id| {
                !spring.units_info().get_unit_is_dead(id).unwrap_or(true)
                    && spring
                        .units_info()
                        .get_unit_def_id(id)
                        .ok()
                        .and_then(|def| crate::defs::kind(spring, def))
                        .is_some_and(is_support)
            });
        if !found {
            return candidate;
        }
        if attempt + 1 < MAX_PLACEMENT_TRIES {
            candidate = find_position(spring, unit);
        }
    }
    spring::log_warning!(
        spring,
        "mycube",
        "no room for a {} after {MAX_PLACEMENT_TRIES} tries",
        unit.def_name()
    );
    candidate
}

pub fn restore_ice(spring: &spring::Spring<'_, spring::RulesSynced>) {
    let (map_x, map_z) = map_size(spring);
    if let Err(error) = spring
        .terrain_control()
        .revert_height_map(0.0, 0.0, map_x, map_z, 1.0)
    {
        spring::log_warning!(spring, "mycube", "restoring the ice failed: {error:?}");
    }
}

fn map_size(spring: &spring::Spring<'_, spring::RulesSynced>) -> (f32, f32) {
    spring
        .game()
        .map_size_elmos()
        .map(|(x, z)| (x as f32, z as f32))
        .unwrap_or((8192.0, 8192.0))
}

fn random_int(spring: &spring::Spring<'_, spring::RulesSynced>, min: i32, max: i32) -> i32 {
    spring
        .synced_random()
        .next_int(min, max)
        .expect("synced random int")
}

fn random_sign(spring: &spring::Spring<'_, spring::RulesSynced>) -> f32 {
    if spring
        .synced_random()
        .next_float()
        .expect("synced random float")
        < 0.5
    {
        -1.0
    } else {
        1.0
    }
}

fn find_position(
    spring: &spring::Spring<'_, spring::RulesSynced>,
    unit: UnitKind,
) -> (f32, f32, f32) {
    let (map_x, map_z) = map_size(spring);
    let (x, z) = if unit == Drillship {
        (
            map_x * 0.5
                + random_int(spring, (map_x / 4.0) as i32, (map_x / 2.0) as i32) as f32
                    * random_sign(spring),
            map_z * 0.5
                + random_int(spring, (map_z / 4.0) as i32, (map_z / 2.0) as i32) as f32
                    * random_sign(spring),
        )
    } else {
        (
            map_x * 0.5 + random_int(spring, -(map_x / 8.0) as i32, (map_x / 8.0) as i32) as f32,
            map_z * 0.5 + random_int(spring, -(map_z / 8.0) as i32, (map_z / 8.0) as i32) as f32,
        )
    };
    let y = spring.terrain().get_ground_height(x, z).unwrap_or(0.0);
    (x, y + if unit == Pirate { 5000.0 } else { 0.0 }, z)
}

fn is_support(kind: UnitKind) -> bool {
    matches!(kind, Iglu | Coalburner)
}

const INIT: [UnitKind; 16] = [
    Coalburner, Iglu, Iglu, Iglu, Iglu, Iglu, Eskimo, Eskimo, Eskimo, Eskimo, Eskimo, Eskimo,
    Eskimo, Eskimo, Eskimo, Eskimo,
];
const PIRATE: [UnitKind; 1] = [Pirate];
const DRILLS_3: [UnitKind; 3] = [Drillship, Drillship, Drillship];
const PIRATE_DRILL: [UnitKind; 2] = [Pirate, Drillship];
const RAID: [UnitKind; 4] = [Pirate, Drillship, Pirate, Pirate];
const ESKIMOS_10: [UnitKind; 10] = [
    Eskimo, Eskimo, Eskimo, Eskimo, Eskimo, Eskimo, Eskimo, Eskimo, Eskimo, Eskimo,
];
const DRILLS_5: [UnitKind; 5] = [Drillship, Drillship, Drillship, Drillship, Drillship];
