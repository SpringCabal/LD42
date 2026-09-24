//! The ice trade: the rules pay the yield, the UI previews it.

use spring::consts::SQUARE_SIZE;

/// Side of the square of ice a trade harvests (`game_trade.lua`'s `size`).
pub const TRADE_SIZE: i32 = 300;

/// Side of the square sampled around the clicked point.
const SAMPLE_SIZE: i32 = 200;

/// The most one trade pays.
const MAX_VALUE: f32 = 100.0;

/// `Math.RoundInt(value, Game.squareSize)`: round, then snap down to the
/// heightmap grid.
pub fn round_to_square(value: f32) -> f32 {
    let rounded = value.round() as i32;
    (rounded - rounded.rem_euclid(SQUARE_SIZE)) as f32
}

/// `DecreaseTerrain`'s crater of `size` whose corner is at `start`: each
/// heightmap point it lowers, with how deep, in multiples of `amount`
/// (`d` in `unit_drill.lua`). Like Lua, the corner comes from the unrounded
/// size, while the crater's side and radius come from the size rounded to
/// the grid.
pub fn crater(size: i32) -> impl Iterator<Item = (i32, i32, f32)> {
    let side = round_to_square(size as f32) as i32;
    let radius = side as f32 / 2.0;
    let steps = (0..=side).step_by(SQUARE_SIZE as usize);
    steps.clone().flat_map(move |x| {
        steps.clone().filter_map(move |z| {
            let dx = x as f32 - radius;
            let dz = z as f32 - radius;
            let depth = radius - (dx * dx + dz * dz).sqrt();
            (depth > 0.0).then_some((x, z, depth))
        })
    })
}

/// The corner of the crater of `size` around `(x, z)`.
pub fn crater_start(x: f32, z: f32, size: i32) -> (f32, f32) {
    let half = size as f32 / 2.0;
    (round_to_square(x - half), round_to_square(z - half))
}

/// Money paid for the ice above sea level around `(x, z)`, on the ground
/// `height(x, z)`.
pub fn ice_value(x: f32, z: f32, height: impl Fn(f32, f32) -> f32) -> f32 {
    let start_x = round_to_square(x - SAMPLE_SIZE as f32 / 2.0);
    let start_z = round_to_square(z - SAMPLE_SIZE as f32 / 2.0);
    let mut ice_amount = 0.0;
    for dx in (0..=SAMPLE_SIZE).step_by(SQUARE_SIZE as usize) {
        for dz in (0..=SAMPLE_SIZE).step_by(SQUARE_SIZE as usize) {
            let height = height(start_x + dx as f32, start_z + dz as f32);
            if height > 0.0 {
                ice_amount += height;
            }
        }
    }
    (ice_amount.sqrt().floor() / 2.0).min(MAX_VALUE)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rounds_then_snaps_down() {
        assert_eq!(round_to_square(0.0), 0.0);
        assert_eq!(round_to_square(7.4), 0.0);
        assert_eq!(round_to_square(7.5), 8.0);
        assert_eq!(round_to_square(4103.9), 4104.0);
        assert_eq!(round_to_square(-1.0), -8.0);
    }

    #[test]
    fn crater_radius_comes_from_the_rounded_size() {
        // The drill's 250 rounds to 248: radius 124, not 125.
        let cells: alloc::vec::Vec<_> = crater(250).collect();
        assert!(cells.iter().all(|&(x, z, _)| x <= 248 && z <= 248));
        let depth = |x, z| {
            cells
                .iter()
                .find(|cell| (cell.0, cell.1) == (x, z))
                .map(|cell| cell.2)
        };
        // 124 - sqrt(4^2 + 4^2) next to the centre.
        assert!((depth(120, 120).unwrap() - 118.343).abs() < 0.001);
        // 124.06 from the centre: outside a radius of 124.
        assert_eq!(depth(0, 120), None);
        assert_eq!(crater_start(1000.0, 1000.0, 250), (872.0, 872.0));
        // The trade's 300 rounds to 296.
        assert!(crater(300).all(|(x, z, depth)| x <= 296 && z <= 296 && depth <= 148.0));
    }

    #[test]
    fn values_only_the_ice_above_the_sea() {
        // 26 x 26 samples of height 1: sqrt(676) = 26, halved.
        assert_eq!(ice_value(1000.0, 1000.0, |_, _| 1.0), 13.0);
        assert_eq!(ice_value(1000.0, 1000.0, |_, _| -5.0), 0.0);
        assert_eq!(ice_value(1000.0, 1000.0, |_, _| 1000.0), MAX_VALUE);
    }
}
