use spring_addons::rules::Gadget;

use crate::integration;
use crate::state::Global;

#[derive(Default)]
pub struct IntegrationGadget;

impl IntegrationGadget {
    pub fn new() -> Self {
        Self
    }
}

impl Gadget<Global> for IntegrationGadget {
    fn name(&self) -> &'static str {
        "Integration"
    }

    fn game_frame(&self, ctx: &spring_addons::AddonContext<'_, Global>, frame: i32) {
        let global = ctx.global();
        integration::update(global, frame);
        match frame {
            20 => crate::trade::handle_message(&mut global.economy.access_mut(), b"MODE|2"),
            35 => {
                if let Some((x, z)) = integration::trade_location() {
                    let message = alloc::format!("TRADE|{x}|{z}");
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
