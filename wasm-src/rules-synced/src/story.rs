//! The authored scenario script, ported from `storyteller.lua`.

use alloc::{string::String, vec::Vec};

use spring::Float3;

use crate::defs::UnitKind::{self, *};
use crate::spawning::SpawnRequest;
use crate::state::{FRAMES_PER_SECOND, GameEnd, Story};

pub fn run_story(state: &mut Story, frame: i32) -> Vec<SpawnRequest> {
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
        state.intro_event = String::from(message);
    }
    let mut requests = Vec::with_capacity(units.len());
    for &unit in units {
        let (x, y, z) = loop {
            let candidate = find_position(unit);
            if !is_support(unit)
                || state.supports.iter().all(|(support_x, support_z)| {
                    let dx = candidate.0 - support_x;
                    let dz = candidate.2 - support_z;
                    dx * dx + dz * dz > IGLU_MIN_DISTANCE * IGLU_MIN_DISTANCE
                })
            {
                break candidate;
            }
        };
        requests.push(SpawnRequest {
            kind: unit,
            position: Float3::new(x, y, z),
            team,
        });
        if is_support(unit) {
            state.supports.push((x, z));
        }
        if unit == Iglu {
            let lantern_x = x + random_int(130, 150) as f32 * random_sign();
            let lantern_z = z + random_int(130, 150) as f32 * random_sign();
            requests.push(SpawnRequest {
                kind: Lantern,
                position: Float3::new(
                    lantern_x,
                    spring::get_ground_height(lantern_x, lantern_z).unwrap_or(y),
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
    state.next_step = frame.saturating_add(wait.saturating_mul(FRAMES_PER_SECOND));
    requests
}

const IGLU_MIN_DISTANCE: f32 = 400.0;

fn map_size() -> (f32, f32) {
    spring::get_height_map_size()
        .map(|[x, z]| {
            (
                (x.saturating_sub(1) * 8) as f32,
                (z.saturating_sub(1) * 8) as f32,
            )
        })
        .unwrap_or((8192.0, 8192.0))
}

/// `math.random(min, max)` from the synced stream Lua's storyteller uses.
fn random_int(min: i32, max: i32) -> i32 {
    spring::random::synced::int(min, max)
}

/// `math.sgn(math.random() - 0.5)`.
fn random_sign() -> f32 {
    if spring::random::synced::float() < 0.5 {
        -1.0
    } else {
        1.0
    }
}

fn find_position(unit: UnitKind) -> (f32, f32, f32) {
    let (map_x, map_z) = map_size();
    let (x, z) = if unit == Drillship {
        (
            map_x * 0.5
                + random_int((map_x / 4.0) as i32, (map_x / 2.0) as i32) as f32 * random_sign(),
            map_z * 0.5
                + random_int((map_z / 4.0) as i32, (map_z / 2.0) as i32) as f32 * random_sign(),
        )
    } else {
        (
            map_x * 0.5 + random_int(-(map_x / 8.0) as i32, (map_x / 8.0) as i32) as f32,
            map_z * 0.5 + random_int(-(map_z / 8.0) as i32, (map_z / 8.0) as i32) as f32,
        )
    };
    let y = spring::get_ground_height(x, z).unwrap_or(0.0);
    (x, y + if unit == Pirate { 5000.0 } else { 0.0 }, z)
}

/// Iglus and the coal burner keep IGLU_MIN_DISTANCE from each other.
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
