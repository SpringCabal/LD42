//! Map sites the headless integration test uses, found the same way by the
//! rules' checks and the UI's test driver. `map` is the map's size in elmos
//! and `ground(x, z)` its height.

/// The highest point on a 512-elmo grid: the ice, for the trade check.
pub fn trade_location(
    map: (i32, i32),
    ground: impl Fn(f32, f32) -> Option<f32>,
) -> Option<(f32, f32)> {
    grid_extreme(map, ground, |ground, best| ground > best)
        .filter(|(_, _, ground)| *ground > 0.0)
        .map(|(x, z, _)| (x, z))
}

/// The lowest point on the same grid: the sea, for the drowning check.
pub fn water_location(
    map: (i32, i32),
    ground: impl Fn(f32, f32) -> Option<f32>,
) -> Option<(f32, f32)> {
    grid_extreme(map, ground, |ground, best| ground < best)
        .filter(|(_, _, ground)| *ground < 0.0)
        .map(|(x, z, _)| (x, z))
}

fn grid_extreme(
    (width, height): (i32, i32),
    ground: impl Fn(f32, f32) -> Option<f32>,
    better: impl Fn(f32, f32) -> bool,
) -> Option<(f32, f32, f32)> {
    let mut best: Option<(f32, f32, f32)> = None;
    for x in (256..width).step_by(512usize) {
        for z in (256..height).step_by(512usize) {
            let ground = ground(x as f32, z as f32)?;
            if best.is_none_or(|(_, _, current)| better(ground, current)) {
                best = Some((x as f32, z as f32, ground));
            }
        }
    }
    best
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_the_highest_ice_and_the_deepest_sea() {
        let ground = |x: f32, z: f32| {
            Some(if x == 768.0 && z == 256.0 {
                30.0
            } else {
                -x / 100.0
            })
        };
        assert_eq!(trade_location((1024, 1024), ground), Some((768.0, 256.0)));
        assert_eq!(water_location((1024, 1024), ground), Some((768.0, 768.0)));
        assert_eq!(trade_location((1024, 1024), |_, _| Some(-1.0)), None);
        assert_eq!(water_location((1024, 1024), |_, _| None), None);
    }
}
