//! Ice harvesting and the resource trade, ported from `game_trade.lua` and
//! the message half of `ui_modes.lua`.

use crate::state::{self, Economy};
use crate::terrain;

pub fn handle_message(economy: &mut Economy, data: &[u8]) {
    let Ok(message) = core::str::from_utf8(data) else {
        return;
    };
    {
        let mut fields = message.split('|');
        let Some(command) = fields.next() else {
            return;
        };
        match command {
            "MODE" => match fields
                .next()
                .and_then(|value| value.parse::<i32>().ok())
                .unwrap_or(-1)
            {
                1 => state::f32_param("uiMode", 1.0),
                2 if state::consume(economy, "money", 50.0) => {
                    state::add(economy, "heat", 100.0);
                    state::f32_param("uiMode", -1.0);
                }
                3 if state::consume(economy, "money", 50.0) => {
                    state::add(economy, "food", 100.0);
                    state::f32_param("uiMode", -1.0);
                }
                _ => state::f32_param("uiMode", -1.0),
            },
            "TRADE" => {
                let x = fields.next().and_then(|value| value.parse::<f32>().ok());
                let z = fields.next().and_then(|value| value.parse::<f32>().ok());
                if let (Some(x), Some(z)) = (x, z) {
                    let value = harvest_ice(x, z);
                    state::add(economy, "money", value);
                }
                state::f32_param("uiMode", -1.0);
            }
            _ => {}
        }
    }
}

pub fn harvest_ice(x: f32, z: f32) -> f32 {
    // Mirrors the old trade gadget: terrain above sea level becomes money and
    // is lowered in the same operation.  The server, rather than the UI,
    // decides the yield so a client cannot mint resources.
    let mut ice_amount = 0.0;
    for dx in (0..=200).step_by(8) {
        for dz in (0..=200).step_by(8) {
            let gx = terrain::round_to_square(x - 100.0) + dx as f32;
            let gz = terrain::round_to_square(z - 100.0) + dz as f32;
            let height = spring::get_ground_height(gx, gz).unwrap_or(0.0);
            if height > 0.0 {
                ice_amount += height;
            }
        }
    }
    terrain::decrease_terrain(x, z, 300, 1.0);
    (ice_amount.sqrt().floor() / 2.0).min(100.0)
}
