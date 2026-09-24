//! Ice harvesting and the resource trade, ported from `game_trade.lua` and
//! the message half of `ui_modes.lua`.

use mycube_shared::Command;
use mycube_shared::ice::{self, TRADE_SIZE};
use mycube_shared::round::PURCHASE_PRICE;

use crate::state::{Economy, Supply};
use crate::terrain;

/// What a load of coal or food adds to the village's stock.
const PURCHASE_AMOUNT: f32 = 100.0;

pub fn handle_command(economy: &mut Economy, command: Command) {
    match command {
        Command::BuyHeat if economy.consume(Supply::Money, PURCHASE_PRICE as f32) => {
            economy.add(Supply::Heat, PURCHASE_AMOUNT);
        }
        Command::BuyFood if economy.consume(Supply::Money, PURCHASE_PRICE as f32) => {
            economy.add(Supply::Food, PURCHASE_AMOUNT);
        }
        Command::Trade { x, z } => economy.add(Supply::Money, harvest_ice(x, z)),
        // Selling is a UI mode; the sale arrives as a Trade.
        _ => {}
    }
}

/// Pay for the ice around `(x, z)` and lower it. The rules, not the UI,
/// decide the yield so a client cannot mint resources.
pub fn harvest_ice(x: f32, z: f32) -> f32 {
    let value = ice::ice_value(x, z, terrain::ground_height);
    terrain::decrease_terrain(x, z, TRADE_SIZE, 1.0);
    value
}
