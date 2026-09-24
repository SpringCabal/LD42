//! Terrain excavation shared by the drill controller and the ice trade.
//!
//! `SetHeightMapFunc` is required for height-map writes. Core meters host
//! calls, so the drill path queues jobs and spreads them over later frames
//! while the trade path, which is a one-off player action, still runs eagerly.

pub use crate::ice::{SQUARE_SIZE, round_to_square};
use alloc::collections::VecDeque;

/// A crater being dug over several frames.
pub struct TerrainJob {
    start_x: f32,
    start_z: f32,
    size: i32,
    amount: f32,
    next_cell: usize,
    /// Failed attempts at the current slice.
    failures: u32,
}

/// Attempts at one slice before its crater is given up.
const MAX_FAILURES: u32 = 5;

/// Terrain above this height still counts as ice for the drills.
pub const ICE_HEIGHT: f32 = -1.0;

pub fn queue_terrain_job(jobs: &mut VecDeque<TerrainJob>, x: f32, z: f32, size: i32, amount: f32) {
    jobs.push_back(TerrainJob {
        start_x: round_to_square(x - size as f32 / 2.0),
        start_z: round_to_square(z - size as f32 / 2.0),
        size,
        amount,
        next_cell: 0,
        failures: 0,
    });
}

pub fn process_terrain_jobs(jobs: &mut VecDeque<TerrainJob>) {
    // One SetHeightMapFunc callback is considerably cheaper than one host
    // call per sample, but keep each callback bounded so a late wave of ships
    // cannot monopolize a GameFrame. Four 256-sample slices complete an
    // authored drill crater in a few frames while preserving the original
    // queued behavior.
    const CELLS_PER_JOB: usize = 256;
    const JOBS_PER_FRAME: usize = 4;

    for _ in 0..JOBS_PER_FRAME {
        let Some((start_x, start_z, size, amount, first, last, cell_count)) =
            jobs.front().map(|job| {
                let cells_per_side = (job.size / SQUARE_SIZE + 1) as usize;
                let cell_count = cells_per_side * cells_per_side;
                (
                    job.start_x,
                    job.start_z,
                    job.size,
                    job.amount,
                    job.next_cell,
                    (job.next_cell + CELLS_PER_JOB).min(cell_count),
                    cell_count,
                )
            })
        else {
            break;
        };

        if first >= cell_count {
            jobs.pop_front();
            continue;
        }

        let result = apply_terrain_cells(start_x, start_z, size, amount, first, last);
        if result != Ok(true) {
            // Retry on a later frame, in case the host was only out of
            // budget; give up on a crater that keeps failing.
            let Some(job) = jobs.front_mut() else {
                break;
            };
            job.failures += 1;
            let failures = job.failures;
            let gave_up = failures >= MAX_FAILURES;
            spring::log_warning!(
                "mycube",
                "digging the crater at ({start_x}, {start_z}) failed ({result:?}, attempt \
                 {failures} of {MAX_FAILURES}){}",
                if gave_up { "; giving it up" } else { "" }
            );
            if gave_up {
                jobs.pop_front();
            }
            break;
        }
        if let Some(job) = jobs.front_mut() {
            job.next_cell = last;
            job.failures = 0;
        }
        if last >= cell_count {
            jobs.pop_front();
        }
    }
}

pub fn decrease_terrain(x: f32, z: f32, size: i32, amount: f32) {
    let cells_per_side = (size / SQUARE_SIZE + 1) as usize;
    let cell_count = cells_per_side * cells_per_side;
    let result = apply_terrain_cells(
        round_to_square(x - size as f32 / 2.0),
        round_to_square(z - size as f32 / 2.0),
        size,
        amount,
        0,
        cell_count,
    );
    if result != Ok(true) {
        spring::log_warning!(
            "mycube",
            "lowering the ice at ({x}, {z}) failed: {result:?}"
        );
    }
}

/// Apply one terrain slice inside Spring's height-map edit transaction.
///
/// The Lua implementation uses `SetHeightMap`, but its target is calculated
/// from the map corner height. The WASM host's ground-height query can be an
/// interpolated value, which made that target occasionally move a corner up.
/// Add the authored negative delta instead, clamped to the same -500 floor;
/// this preserves the crater profile while making an excavation incapable of
/// increasing terrain.
fn apply_terrain_cells(
    start_x: f32,
    start_z: f32,
    size: i32,
    amount: f32,
    first: usize,
    last: usize,
) -> spring::Result<bool> {
    let cells_per_side = (size / SQUARE_SIZE + 1) as usize;
    let half = size as f32 / 2.0;
    spring::set_height_map_func(move || {
        for cell in first..last {
            let local_x = (cell / cells_per_side) as i32 * SQUARE_SIZE;
            let local_z = (cell % cells_per_side) as i32 * SQUARE_SIZE;
            let dx = local_x as f32 - half;
            let dz = local_z as f32 - half;
            let falloff = (half - (dx * dx + dz * dz).sqrt()).max(0.0);
            let gx = start_x + local_x as f32;
            let gz = start_z + local_z as f32;
            if falloff <= 0.0 {
                continue;
            }
            let height = spring::get_ground_height(gx, gz).unwrap_or(-500.0);
            let delta = (amount * falloff).min((height + 500.0).max(0.0));
            if delta > 0.0 {
                let _ = spring::add_height_map(gx, gz, -delta);
            }
        }
    })
}
