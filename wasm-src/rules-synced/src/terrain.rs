//! Terrain excavation shared by the drill controller and the ice trade.

pub use mycube_shared::ice::round_to_square;
use spring::consts::SQUARE_SIZE;

/// Terrain above this height still counts as ice for the drills.
pub const ICE_HEIGHT: f32 = -1.0;

/// The ground height at `(x, z)`; 0 (sea level) off the map.
pub fn ground_height(x: f32, z: f32) -> f32 {
    spring::get_ground_height(x, z).unwrap_or(0.0)
}

/// `DecreaseTerrain`: dig a round crater of `size` around `(x, z)`, deepest
/// (`amount` per elmo from the rim) in the middle.
pub fn decrease_terrain(x: f32, z: f32, size: i32, amount: f32) {
    let result = apply_terrain_cells(
        round_to_square(x - size as f32 / 2.0),
        round_to_square(z - size as f32 / 2.0),
        size,
        amount,
    );
    if result != Ok(true) {
        spring::log_warning!(
            "mycube",
            "lowering the ice at ({x}, {z}) failed: {result:?}"
        );
    }
}

/// Dig the crater inside Spring's height-map edit transaction.
///
/// The Lua implementation uses `SetHeightMap`, but its target is calculated
/// from the map corner height. The WASM host's ground-height query can be an
/// interpolated value, which made that target occasionally move a corner up.
/// Add the authored negative delta instead, clamped to the same -500 floor;
/// this preserves the crater profile while making an excavation incapable of
/// increasing terrain.
fn apply_terrain_cells(start_x: f32, start_z: f32, size: i32, amount: f32) -> spring::Result<bool> {
    let cells_per_side = (size / SQUARE_SIZE + 1) as usize;
    let half = size as f32 / 2.0;
    spring::set_height_map_func(move || {
        for cell in 0..cells_per_side * cells_per_side {
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
