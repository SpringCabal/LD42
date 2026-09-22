use spring_addons::rules::Gadget;

use crate::eskimos;
use crate::state::{DestroyRequest, Global};

/// Eskimo food/heat/health bookkeeping, ported from `unit_eskimos.lua`.
#[derive(Default)]
pub struct EskimosGadget;

impl EskimosGadget {
    pub fn new() -> Self {
        Self
    }
}

impl Gadget<Global> for EskimosGadget {
    fn name(&self) -> &'static str {
        "Eskimos"
    }

    fn game_frame(&self, ctx: &spring_addons::AddonContext<'_, Global>, frame: i32) {
        if frame % 30 == 0 {
            return;
        }
        let dead = eskimos::update_eskimos(
            &mut ctx.global().units.access_mut(),
            &mut ctx.global().economy.access_mut(),
        );
        ctx.global()
            .pending
            .access_mut()
            .destroys
            .extend(dead.into_iter().map(|id| DestroyRequest {
                id,
                reclaimed: false,
            }));
    }
}
