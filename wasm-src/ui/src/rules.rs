//! Messages to the rules, which own the game state.

use mycube_shared::Command;

/// Send `command` to the rules; a failure is logged.
pub fn send(command: Command) {
    let message = command.encode();
    if let Err(error) = spring::send_lua_rules_msg(&message) {
        spring::log_warning!(
            "mycube-ui",
            "sending {message} to the rules failed: {error:?}"
        );
    }
}
