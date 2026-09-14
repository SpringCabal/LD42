use spring_addons::rules::Gadget;

use crate::state::Global;
use crate::trade;

/// Resource trading and mode switching, ported from `game_trade.lua` and the
/// message half of `ui_modes.lua`.
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
        trade::handle_message(&mut ctx.global().economy.access_mut(), data);
    }
}
