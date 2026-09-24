use core::cell::Cell;

use spring_addons::ui::Widget;
use springcabal::message::Encoder;

use crate::state::Global;

/// A message the headless integration test sends to the rules, from the
/// first update at or after `frame`.
struct Step {
    frame: i32,
    message: fn() -> Option<String>,
}

/// In order; the rules check each result a few frames later
/// (`rules-synced/src/integration.rs`), since the messages go over the
/// network and may arrive a frame after they are sent.
const STEPS: [Step; 4] = [
    // Buy coal (checked at frame 30).
    Step {
        frame: 20,
        message: || Some(String::from("MODE|2")),
    },
    // Sell the ice the rules measured at frame 35 (checked at 45).
    Step {
        frame: 36,
        message: || {
            let (x, z) = crate::test_sites::trade_location()?;
            Some(Encoder::new("TRADE").field(x).field(z).finish())
        },
    },
    // Buy food with the trade's money (checked at 45).
    Step {
        frame: 38,
        message: || Some(String::from("MODE|3")),
    },
    // Restart the round the way Ctrl+R does, after the first round's checks.
    Step {
        frame: 1005,
        message: || Some(String::from(crate::round::RESTART_MESSAGE)),
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
            match (step.message)() {
                Some(message) => crate::rules::send(&message),
                None => spring::log_warning!(
                    "mycube-ui",
                    "no message for the test step at frame {}",
                    step.frame
                ),
            }
        }
    }
}
