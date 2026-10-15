use alloc::rc::Rc;
use core::cell::RefCell;

use spring_addons::rules::Gadget;

use crate::integration;
use crate::state::Global;

pub struct IntegrationGadget {
    tracker: Rc<RefCell<integration::Tracker>>,
}

impl IntegrationGadget {
    pub fn new() -> Self {
        Self {
            tracker: integration::Tracker::new_shared(),
        }
    }
}

impl Default for IntegrationGadget {
    fn default() -> Self {
        Self::new()
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
