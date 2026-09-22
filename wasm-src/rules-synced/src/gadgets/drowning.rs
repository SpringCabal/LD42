use spring_addons::rules::Gadget;

use crate::drowning;
use crate::state::{DestroyRequest, Global};

/// Drowning, ported from `unit_drown_eskimos.lua`.
#[derive(Default)]
pub struct DrowningGadget;

impl DrowningGadget {
    pub fn new() -> Self {
        Self
    }
}

impl Gadget<Global> for DrowningGadget {
    fn name(&self) -> &'static str {
        "Drowning"
    }

    fn game_frame(&self, ctx: &spring_addons::AddonContext<'_, Global>, _frame: i32) {
        let dead = drowning::update_drowning(&mut ctx.global().units.access_mut());
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
