use mycube_shared::ice;
pub use mycube_shared::ice::round_to_square;
use spring::ResultExt;

pub const ICE_HEIGHT: f32 = -1.0;
const MIN_TERRAIN_HEIGHT: f32 = -500.0;

pub fn ground_height(x: f32, z: f32) -> f32 {
    spring::get_ground_height(x, z).unwrap_or(0.0)
}

pub fn decrease_terrain(x: f32, z: f32, size: i32, amount: f32) {
    let (start_x, start_z) = ice::crater_start(x, z, size);
    let result = spring::set_height_map_func(move || {
        for (local_x, local_z, depth) in ice::crater(size) {
            let gx = start_x + local_x as f32;
            let gz = start_z + local_z as f32;
            let height = spring::get_ground_height(gx, gz).unwrap_or(MIN_TERRAIN_HEIGHT);
            let delta = (amount * depth).min((height - MIN_TERRAIN_HEIGHT).max(0.0));
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
