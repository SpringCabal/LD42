//! Messages to the rules, which own the game state.

/// Send `message` to the rules; a failure is logged.
pub fn send(message: &str) {
    if let Err(error) = spring::send_lua_rules_msg(message) {
        spring::log_warning!(
            "mycube-ui",
            "sending {message} to the rules failed: {error:?}"
        );
    }
}

/// Start the round over (Ctrl+R).
pub fn restart_round() {
    send(crate::round::RESTART_MESSAGE);
}
