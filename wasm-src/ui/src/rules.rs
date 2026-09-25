//! Messages to the rules, which own the game state.

use mycube_shared::Command;

/// Send `command` to the rules; a failure is logged.
pub fn send(command: Command) {
    springcabal::message::send_to_rules(&command.encode());
}
