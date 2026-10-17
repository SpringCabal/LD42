use mycube_shared::ice;
pub use mycube_shared::ice::round_to_square;
use spring::ResultExt;

pub const ICE_HEIGHT: f32 = -1.0;
const MIN_TERRAIN_HEIGHT: f32 = -500.0;

pub fn ground_height(spring: &spring::Spring<'_, spring::RulesSynced>, x: f32, z: f32) -> f32 {
    spring.terrain().get_ground_height(x, z).unwrap_or(0.0)
}

pub fn decrease_terrain(
    spring: &spring::Spring<'_, spring::RulesSynced>,
    x: f32,
    z: f32,
    size: i32,
    amount: f32,
) {
    let (start_x, start_z) = ice::crater_start(x, z, size);
    let result = spring.terrain_control().set_height_map_func(move |spring| {
        for (local_x, local_z, depth) in ice::crater(size) {
            let gx = start_x + local_x as f32;
            let gz = start_z + local_z as f32;
            let height = spring
                .terrain()
                .get_ground_height(gx, gz)
                .unwrap_or(MIN_TERRAIN_HEIGHT);
            let delta = (amount * depth).min((height - MIN_TERRAIN_HEIGHT).max(0.0));
            if delta > 0.0 {
                spring
                    .terrain_control()
                    .add_height_map(gx, gz, -delta)
                    .log_err_once(spring, "lowering a heightmap point failed");
            }
        }
    });
    if result != Ok(true) {
        spring::log_warning!(
            spring,
            "mycube",
            "lowering the ice at ({x}, {z}) failed: {result:?}"
        );
    }
}
