//! The autonomous drill controller, ported from `unit_drill.lua`.

use crate::state::{DrillShip, Terrain, Units};
use crate::terrain::{self, ICE_HEIGHT, SQUARE_SIZE};

const SEARCH_STEP: i32 = 64;

pub fn update_drills(units: &mut Units, terrain: &mut Terrain, frame: i32) {
    if frame % 30 != 0 {
        return;
    }
    if units.drills.is_empty() {
        return;
    }

    // The Lua gadget drills every active ship. The first WASM pass only
    // selected one ship every 30 frames, which made most ships look idle and
    // made the ice disappear far too slowly. SetHeightMapFunc slices the
    // actual work safely, so keep all ships on the authored cadence.
    for drill_ship in &mut units.drills {
        let id = drill_ship.id;
        let Some(hull) = spring::get_unit_position(id, false, false).ok() else {
            continue;
        };
        // The ship's origin is the stable gameplay anchor. The original Lua
        // uses the animated `icemine` piece, but querying the complete piece
        // map from Core on every metered callin costs more than this scenario
        // can afford; the hull anchor keeps the drill action reliable.
        terrain::queue_terrain_job(terrain, hull.x, hull.z, 250, 2.0);

        // The original drill gadget is autonomous: once it has exhausted the
        // ice immediately around its hull it searches outward and issues a
        // normal movement order toward the closest remaining ice.
        if nearby_ice(hull.x, hull.z) {
            continue;
        }
        if drill_ship.has_order
            && distance(drill_ship.order_x, drill_ship.order_z, hull.x, hull.z) > NEARBY_SIZE
        {
            let y =
                spring::get_ground_height(drill_ship.order_x, drill_ship.order_z).unwrap_or(0.0);
            let _ = spring::give_order_to_unit(
                id,
                10,
                &[drill_ship.order_x, y, drill_ship.order_z],
                0,
                0,
            );
            continue;
        }

        // The previous target has been reached (or this is the first search),
        // so restart the Lua-style outward search from the ship's current
        // position. A search slice that finds nothing yet simply leaves its
        // cursor in place for the next 30-frame check.
        reset_search(drill_ship, hull.x, hull.z);
        if let Some((x, z)) = find_closest_ice(drill_ship) {
            let y = spring::get_ground_height(x, z).unwrap_or(0.0);
            let _ = spring::give_order_to_unit(id, 10, &[x, y, z], 0, 0);
            drill_ship.has_order = true;
            drill_ship.order_x = x;
            drill_ship.order_z = z;
        }
    }
}

const NEARBY_SIZE: f32 = 100.0;

fn distance(x1: f32, z1: f32, x2: f32, z2: f32) -> f32 {
    let dx = x1 - x2;
    let dz = z1 - z2;
    spring::Float3::new(dx, 0.0, dz).length()
}

fn reset_search(drill: &mut DrillShip, x: f32, z: f32) {
    if drill.has_order || !drill.search_initialized {
        drill.has_order = false;
        drill.search_initialized = true;
        drill.search_center_x = terrain::round_i_to_square(x as i32);
        drill.search_center_z = terrain::round_i_to_square(z as i32);
        drill.search_radius = SEARCH_STEP;
        drill.search_side = 0;
        drill.search_offset = 0;
        drill.search_complete = false;
    }
}

fn nearby_ice(x: f32, z: f32) -> bool {
    let center_x = terrain::round_i_to_square(x as i32);
    let center_z = terrain::round_i_to_square(z as i32);
    // Core counts every terrain query as host work.  The old Lua loop could
    // freely inspect the whole 13x13 neighbourhood, but doing that for every
    // drill exhausts the per-callin budget once the expedition has expanded.
    // These five samples still cover the drill hull and its immediate edge;
    // the bounded outward search below supplies a new move goal otherwise.
    [(0, 0), (-48, 0), (48, 0), (0, -48), (0, 48)]
        .into_iter()
        .any(|(dx, dz)| {
            spring::get_ground_height((center_x + dx) as f32, (center_z + dz) as f32)
                .is_ok_and(|height| height > ICE_HEIGHT)
        })
}

fn find_closest_ice(drill: &mut DrillShip) -> Option<(f32, f32)> {
    let [points_x, points_z] = spring::get_height_map_size().ok()?;
    let max_x = (points_x - 1) * SQUARE_SIZE;
    let max_z = (points_z - 1) * SQUARE_SIZE;
    let center_x = drill.search_center_x;
    let center_z = drill.search_center_z;
    let max_radius = max_x.max(max_z);
    // Match the Lua perimeter search, but retain the cursor between callins.
    // The old WASM implementation restarted after 32 probes, so ships
    // spawning in the outer quarters never reached the central ice. 256
    // coarse probes per check cover the map in a few seconds without
    // exhausting Core's per-callin host-work budget.
    const MAX_ICE_PROBES: usize = 256;
    let mut probes = 0;
    while probes < MAX_ICE_PROBES {
        if drill.search_complete {
            return None;
        }
        if drill.search_radius > max_radius {
            drill.search_complete = true;
            return None;
        }

        let radius = drill.search_radius;
        let side = drill.search_side as i32;
        let side_steps = (radius * 2 / SEARCH_STEP) + 1;
        if drill.search_offset >= side_steps {
            drill.search_offset = 0;
            drill.search_side = drill.search_side.saturating_add(1);
            if drill.search_side >= 4 {
                drill.search_side = 0;
                drill.search_radius = drill.search_radius.saturating_add(SEARCH_STEP);
            }
            continue;
        }

        let offset = drill.search_offset;
        drill.search_offset += 1;
        let (start, fixed, horizontal) = match side {
            0 => (center_x - radius, center_z - radius, true),
            1 => (center_x - radius, center_z + radius, true),
            2 => (center_z - radius, center_x - radius, false),
            _ => (center_z - radius, center_x + radius, false),
        };
        let varying = start + offset * SEARCH_STEP;
        let (candidate_x, candidate_z) = if horizontal {
            (varying, fixed)
        } else {
            (fixed, varying)
        };
        if !(0..=max_x).contains(&candidate_x) || !(0..=max_z).contains(&candidate_z) {
            continue;
        }
        probes += 1;
        if spring::get_ground_height(candidate_x as f32, candidate_z as f32)
            .is_ok_and(|height| height > ICE_HEIGHT)
        {
            return Some((candidate_x as f32, candidate_z as f32));
        }
    }
    None
}
