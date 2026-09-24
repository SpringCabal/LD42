//! The ice-trade yield, shared by the rules (which pays it) and the UI (which
//! previews it). Both crates include this file with `#[path]`.

use spring::consts::SQUARE_SIZE;

/// Side of the square sampled around the clicked point.
const SAMPLE_SIZE: i32 = 200;

/// Snap a world coordinate to the heightmap grid, like `game_trade.lua`.
pub fn round_to_square(value: f32) -> f32 {
    let rounded = value.round() as i32;
    (rounded - rounded.rem_euclid(SQUARE_SIZE)) as f32
}

/// Money paid for the ice above sea level around `(x, z)`.
pub fn ice_value(x: f32, z: f32) -> f32 {
    let start_x = round_to_square(x - SAMPLE_SIZE as f32 / 2.0);
    let start_z = round_to_square(z - SAMPLE_SIZE as f32 / 2.0);
    let mut ice_amount = 0.0;
    for dx in (0..=SAMPLE_SIZE).step_by(SQUARE_SIZE as usize) {
        for dz in (0..=SAMPLE_SIZE).step_by(SQUARE_SIZE as usize) {
            let height =
                spring::get_ground_height(start_x + dx as f32, start_z + dz as f32).unwrap_or(0.0);
            if height > 0.0 {
                ice_amount += height;
            }
        }
    }
    (ice_amount.sqrt().floor() / 2.0).min(100.0)
}
