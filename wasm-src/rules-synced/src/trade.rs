//! Ice harvesting and the resource trade, ported from `game_trade.lua` and
//! the message half of `ui_modes.lua`.

use springcabal::message;

use crate::state::{Economy, Supply};
use crate::{ice, terrain};

/// Money for a load of coal or food (`ui_modes.lua`).
const PURCHASE_PRICE: f32 = 50.0;

pub fn handle_message(economy: &mut Economy, data: &[u8]) {
    let Ok(message) = core::str::from_utf8(data) else {
        return;
    };
    if let Some(mut fields) = message::fields(message, "MODE") {
        match fields.i32() {
            Some(2) if economy.consume(Supply::Money, PURCHASE_PRICE) => {
                economy.add(Supply::Heat, 100.0);
            }
            Some(3) if economy.consume(Supply::Money, PURCHASE_PRICE) => {
                economy.add(Supply::Food, 100.0);
            }
            // MODE|1 (sell ice) is a UI mode; the sale arrives as TRADE.
            _ => {}
        }
    } else if let Some((x, z)) = message::fields(message, "TRADE").and_then(|mut f| f.xz()) {
        economy.add(Supply::Money, harvest_ice(x, z));
    }
}

/// Pay for the ice around `(x, z)` and lower it. The rules, not the UI,
/// decide the yield so a client cannot mint resources.
pub fn harvest_ice(x: f32, z: f32) -> f32 {
    let value = ice::ice_value(x, z);
    terrain::decrease_terrain(x, z, 300, 1.0);
    value
}
