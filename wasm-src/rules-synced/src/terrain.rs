//! Terrain excavation shared by the drill controller and the ice trade.

use mycube_shared::ice;
pub use mycube_shared::ice::round_to_square;
use spring::ResultExt;

/// Terrain above this height still counts as ice for the drills.
pub const ICE_HEIGHT: f32 = -1.0;

/// The ground height at `(x, z)`; 0 (sea level) off the map.
pub fn ground_height(x: f32, z: f32) -> f32 {
    spring::get_ground_height(x, z).unwrap_or(0.0)
}

/// `DecreaseTerrain`: dig a round crater of `size` around `(x, z)`, deepest
/// (`amount` per elmo from the rim) in the middle.
///
/// The Lua implementation uses `SetHeightMap`, but its target is calculated
/// from the map corner height. The WASM host's ground-height query can be an
/// interpolated value, which made that target occasionally move a corner up.
/// Add the authored negative delta instead, clamped to the same -500 floor;
/// this preserves the crater profile while making an excavation incapable of
/// increasing terrain.
pub fn decrease_terrain(x: f32, z: f32, size: i32, amount: f32) {
    let (start_x, start_z) = ice::crater_start(x, z, size);
    let result = spring::set_height_map_func(move || {
        for (local_x, local_z, depth) in ice::crater(size) {
            let gx = start_x + local_x as f32;
            let gz = start_z + local_z as f32;
            let height = spring::get_ground_height(gx, gz).unwrap_or(-500.0);
            let delta = (amount * depth).min((height + 500.0).max(0.0));
            if delta > 0.0 {
                spring::add_height_map(gx, gz, -delta)
                    .log_err_once("lowering a heightmap point failed");
            }
        }
    });
    if result != Ok(true) {
        spring::log_warning!(
            "mycube",
            "lowering the ice at ({x}, {z}) failed: {result:?}"
        );
    }
}
