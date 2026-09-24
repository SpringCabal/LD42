use core::cell::Cell;

use spring_addons::ui::Widget;

use crate::state::Global;

/// The frame the headless integration test restarts the round, after the
/// rules' checks of the first round (`rules-synced/src/integration.rs`).
const RESTART_FRAME: i32 = 1005;

/// Restarts the round once in the headless integration test, with the
/// message Ctrl+R sends.
#[derive(Default)]
pub struct DbgRestartWidget {
    sent: Cell<bool>,
}

impl DbgRestartWidget {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Widget<Global> for DbgRestartWidget {
    fn name(&self) -> &'static str {
        "DbgRestart"
    }

    fn update(&self, _ctx: &spring_addons::AddonContext<'_, Global>, _dt: f32) {
        if !self.sent.get() && spring::game::current_frame() >= RESTART_FRAME {
            self.sent.set(true);
            crate::rules::restart_round();
        }
    }
}
