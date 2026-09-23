use alloc::vec::Vec;
use core::cell::RefCell;

use spring_addons::rules::{Gadget, UnitDestroyedEvent};

use crate::defs::{self, UnitKind};
use crate::parachutes;
use crate::state::Global;
use crate::units::destroy_later;

/// Parachute cleanup, ported from `unit_parachute_remover.lua`.
#[derive(Default)]
pub struct ParachutesGadget {
    parachutes: RefCell<Vec<i32>>,
}

impl ParachutesGadget {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Gadget<Global> for ParachutesGadget {
    fn name(&self) -> &'static str {
        "Parachutes"
    }

    fn unit_created(
        &self,
        _ctx: &spring_addons::AddonContext<'_, Global>,
        id: i32,
        def: i32,
        _team: i32,
        _builder: i32,
    ) {
        if defs::kind(def) == Some(UnitKind::Parachute) {
            self.parachutes.borrow_mut().push(id);
        }
    }

    fn unit_destroyed(
        &self,
        _ctx: &spring_addons::AddonContext<'_, Global>,
        event: &UnitDestroyedEvent,
    ) {
        self.parachutes
            .borrow_mut()
            .retain(|id| *id != event.unit_id);
    }

    fn game_frame(&self, ctx: &spring_addons::AddonContext<'_, Global>, _frame: i32) {
        let landed = parachutes::update_parachutes(&self.parachutes.borrow());
        destroy_later(ctx, landed, true);
    }
}
