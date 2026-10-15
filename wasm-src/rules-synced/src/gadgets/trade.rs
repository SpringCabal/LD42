use mycube_shared::Command;
use mycube_shared::ice::{self, TRADE_SIZE};
use mycube_shared::round::PURCHASE_PRICE;
use spring_addons::rules::Gadget;

use crate::state::{Global, Supply};
use crate::terrain;

const PURCHASE_AMOUNT: f32 = 100.0;

#[derive(Default)]
pub struct TradeGadget;

impl TradeGadget {
    pub fn new() -> Self {
        Self
    }
}

impl Gadget<Global> for TradeGadget {
    fn name(&self) -> &'static str {
        "Trade"
    }

    fn handle_lua_msg(
        &self,
        ctx: &spring_addons::AddonContext<'_, Global>,
        _player_id: i32,
        _script: i32,
        _mode: i32,
        data: &[u8],
    ) {
        if let Some(command) = core::str::from_utf8(data).ok().and_then(Command::decode) {
            let mut economy = ctx.global().economy.access_mut();
            match command {
                Command::BuyHeat if economy.consume(Supply::Money, PURCHASE_PRICE as f32) => {
                    economy.add(Supply::Heat, PURCHASE_AMOUNT);
                }
                Command::BuyFood if economy.consume(Supply::Money, PURCHASE_PRICE as f32) => {
                    economy.add(Supply::Food, PURCHASE_AMOUNT);
                }
                Command::Trade { x, z } => economy.add(Supply::Money, harvest_ice(x, z)),
                _ => {}
            }
        }
    }
}

fn harvest_ice(x: f32, z: f32) -> f32 {
    let value = ice::ice_value(x, z, terrain::ground_height);
    terrain::decrease_terrain(x, z, TRADE_SIZE, 1.0);
    value
}
