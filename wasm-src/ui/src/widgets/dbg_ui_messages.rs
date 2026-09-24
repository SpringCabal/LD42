use core::cell::Cell;

use mycube_shared::Command;
use spring_addons::ui::Widget;

use crate::state::Global;

/// A command the headless integration test sends to the rules, from the
/// first update at or after `frame`.
struct Step {
    frame: i32,
    command: fn() -> Option<Command>,
}

/// In order; the rules check each result a few frames later
/// (`rules-synced/src/integration.rs`), since the messages go over the
/// network and may arrive a frame after they are sent.
const STEPS: [Step; 4] = [
    // Buy coal (checked at frame 30).
    Step {
        frame: 20,
        command: || Some(Command::BuyHeat),
    },
    // Sell the ice the rules measured at frame 35 (checked at 45).
    Step {
        frame: 36,
        command: || {
            let map = spring::game::map_size_elmos().ok()?;
            let (x, z) = mycube_shared::test_sites::trade_location(map, |x, z| {
                spring::get_ground_height(x, z).ok()
            })?;
            Some(Command::Trade { x, z })
        },
    },
    // Buy food with the trade's money (checked at 45).
    Step {
        frame: 38,
        command: || Some(Command::BuyFood),
    },
    // Restart the round the way Ctrl+R does, after the first round's checks.
    Step {
        frame: 1005,
        command: || Some(Command::Restart),
    },
];

/// Sends the headless integration test's trades and restart through the
/// same channel as the player's keys and clicks.
#[derive(Default)]
pub struct DbgUiMessagesWidget {
    sent: Cell<usize>,
}

impl DbgUiMessagesWidget {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Widget<Global> for DbgUiMessagesWidget {
    fn name(&self) -> &'static str {
        "DbgUiMessages"
    }

    fn update(&self, _ctx: &spring_addons::AddonContext<'_, Global>, _dt: f32) {
        let frame = spring::game::current_frame();
        while let Some(step) = STEPS.get(self.sent.get()) {
            if frame < step.frame {
                return;
            }
            self.sent.set(self.sent.get() + 1);
            match (step.command)() {
                Some(command) => crate::rules::send(command),
                None => spring::log_warning!(
                    "mycube-ui",
                    "no message for the test step at frame {}",
                    step.frame
                ),
            }
        }
    }
}
