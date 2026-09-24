use core::cell::RefCell;

use spring_addons::rules::Gadget;
use springcabal::message::Encoder;

use crate::integration;
use crate::state::Global;

/// The headless integration test (feature `headless-integration`).
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
        let global = ctx.global();
        self.tracker.borrow_mut().update(global, frame);
        match frame {
            20 => crate::trade::handle_message(&mut global.economy.access_mut(), b"MODE|2"),
            35 => {
                if let Some((x, z)) = integration::trade_location() {
                    let message = Encoder::new("TRADE").field(x).field(z).finish();
                    crate::trade::handle_message(
                        &mut global.economy.access_mut(),
                        message.as_bytes(),
                    );
                }
            }
            38 => crate::trade::handle_message(&mut global.economy.access_mut(), b"MODE|3"),
            _ => {}
        }
    }
}
