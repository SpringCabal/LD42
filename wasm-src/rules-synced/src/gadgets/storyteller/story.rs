use alloc::vec::Vec;

use spring::Float3;

use crate::defs::UnitKind::{self, *};
use crate::spawning::SpawnRequest;
use crate::state::{GameEnd, Story};
use mycube_shared::round::LUA_FRAMES_PER_SECOND;

pub fn run_story(
    spring: &spring::Spring<'_, spring::RulesSynced>,
    state: &mut Story,
    frame: i32,
) -> Vec<SpawnRequest> {
    if frame < state.next_step {
        return Vec::new();
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
            return Vec::new();
        }
    };
    if !message.is_empty() {
        state.intro_event = message;
    }
    let mut requests = Vec::with_capacity(units.len());
    for &unit in units {
        let (x, y, z) = place(spring, unit, &state.supports);
        requests.push(SpawnRequest {
            kind: unit,
            position: Float3::new(x, y, z),
            team,
        });
        if is_support(unit) {
            state.supports.push((x, z));
        }
        if unit == Iglu {
            let lantern_x = x + random_int(spring, 130, 150) as f32 * random_sign(spring);
            let lantern_z = z + random_int(spring, 130, 150) as f32 * random_sign(spring);
            requests.push(SpawnRequest {
                kind: Lantern,
                position: Float3::new(
                    lantern_x,
                    spring
                        .terrain()
                        .get_ground_height(lantern_x, lantern_z)
                        .unwrap_or(y),
                    lantern_z,
                ),
                team,
            });
        } else if unit == Pirate {
            requests.push(SpawnRequest {
                kind: Parachute,
                position: Float3::new(x, y + 100.0, z),
                team,
            });
        }
    }
    state.step += 1;
    state.next_step = frame.saturating_add(wait.saturating_mul(LUA_FRAMES_PER_SECOND));
    requests
}

const IGLU_MIN_DISTANCE: f32 = 400.0;
const MAX_PLACEMENT_TRIES: u32 = 1000;

fn place(
    spring: &spring::Spring<'_, spring::RulesSynced>,
    unit: UnitKind,
    supports: &[(f32, f32)],
) -> (f32, f32, f32) {
    let clear = |(x, _, z): (f32, f32, f32)| {
        supports.iter().all(|(support_x, support_z)| {
            let dx = x - support_x;
            let dz = z - support_z;
            dx * dx + dz * dz > IGLU_MIN_DISTANCE * IGLU_MIN_DISTANCE
        })
    };
    let mut candidate = find_position(spring, unit);
    if !is_support(unit) {
        return candidate;
    }
    for _ in 1..MAX_PLACEMENT_TRIES {
        if clear(candidate) {
            return candidate;
        }
        candidate = find_position(spring, unit);
    }
    if !clear(candidate) {
        spring::log_warning!(
            spring,
            "mycube",
            "no room for a {} after {MAX_PLACEMENT_TRIES} tries; placing it at {candidate:?}",
            unit.def_name()
        );
    }
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
