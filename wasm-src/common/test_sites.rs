//! Map sites the headless integration test uses, found the same way by the
//! rules' checks and the UI's test driver. Both crates include this file
//! with `#[path]` under the `headless-integration` feature.

/// The highest point on a 512-elmo grid: the ice, for the trade check.
pub fn trade_location() -> Option<(f32, f32)> {
    grid_extreme(|ground, best| ground > best)
        .filter(|(_, _, ground)| *ground > 0.0)
        .map(|(x, z, _)| (x, z))
}

/// The lowest point on the same grid: the sea, for the drowning check.
pub fn water_location() -> Option<(f32, f32)> {
    grid_extreme(|ground, best| ground < best)
        .filter(|(_, _, ground)| *ground < 0.0)
        .map(|(x, z, _)| (x, z))
}

fn grid_extreme(better: impl Fn(f32, f32) -> bool) -> Option<(f32, f32, f32)> {
    let (width, height) = spring::game::map_size_elmos().ok()?;
    let mut best: Option<(f32, f32, f32)> = None;
    for x in (256..width).step_by(512usize) {
        for z in (256..height).step_by(512usize) {
            let ground = spring::get_ground_height(x as f32, z as f32).ok()?;
            if best.is_none_or(|(_, _, current)| better(ground, current)) {
                best = Some((x as f32, z as f32, ground));
            }
        }
    }
    best
}
