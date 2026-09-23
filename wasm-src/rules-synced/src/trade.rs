//! Ice harvesting and the resource trade, ported from `game_trade.lua` and
//! the message half of `ui_modes.lua`.

use crate::state::{self, Economy};
use crate::{ice, terrain};

pub fn handle_message(economy: &mut Economy, data: &[u8]) {
    let Ok(message) = core::str::from_utf8(data) else {
        return;
    };
    let mut fields = message.split('|');
    match fields.next() {
        Some("MODE") => match fields
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
        Some("TRADE") => {
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

/// Pay for the ice around `(x, z)` and lower it. The rules, not the UI,
/// decide the yield so a client cannot mint resources.
pub fn harvest_ice(x: f32, z: f32) -> f32 {
    let value = ice::ice_value(x, z);
    terrain::decrease_terrain(x, z, 300, 1.0);
    value
}
