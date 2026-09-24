use core::cell::RefCell;

use spring_addons::rules::Gadget;

use crate::integration;
use crate::state::Global;

/// The headless integration test (feature `headless-integration`). The
/// UI's test driver sends the trades and the restart it checks.
#[derive(Default)]
pub struct IntegrationGadget {
    tracker: RefCell<integration::Tracker>,
}

impl IntegrationGadget {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Gadget<Global> for IntegrationGadget {
    fn name(&self) -> &'static str {
        "Integration"
    }

    fn game_frame(&self, ctx: &spring_addons::AddonContext<'_, Global>, frame: i32) {
        self.tracker.borrow_mut().update(ctx.global(), frame);
    }
}
