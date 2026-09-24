use mycube_shared::Command;
use spring_addons::rules::Gadget;

use crate::state::Global;

/// Starts the round over when the player asks (Ctrl+R).
///
/// `ui_endgame.lua` restarts with `luarules reload`, which runs every
/// gadget's Initialize again. That command reloads only the Lua gadgets, not
/// this module, so the rules reset their round themselves.
#[derive(Default)]
pub struct RestartGadget;

impl RestartGadget {
    pub fn new() -> Self {
        Self
    }
}

impl Gadget<Global> for RestartGadget {
    fn name(&self) -> &'static str {
        "Restart"
    }

    fn handle_lua_msg(
        &self,
        ctx: &spring_addons::AddonContext<'_, Global>,
        _player_id: i32,
        _script: i32,
        _mode: i32,
        data: &[u8],
    ) {
        if core::str::from_utf8(data).ok().and_then(Command::decode) == Some(Command::Restart) {
            ctx.global().restart_round();
            spring::log_info!(
                "mycube",
                "the round restarts at frame {}",
                spring::game::current_frame()
            );
        }
    }
}
