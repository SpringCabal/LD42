use alloc::vec::Vec;
use core::cell::RefCell;

use spring_addons::rules::{Gadget, UnitDestroyedEvent};

use crate::defs::{self, UnitKind};
use crate::drowning;
use crate::state::Global;
use crate::units::destroy_later;

/// Drowning, ported from `unit_drown_eskimos.lua`. Eskimos come from the
/// village; the pirates are tracked here.
#[derive(Default)]
pub struct DrowningGadget {
    pirates: RefCell<Vec<i32>>,
}

impl DrowningGadget {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Gadget<Global> for DrowningGadget {
    fn name(&self) -> &'static str {
        "Drowning"
    }

    fn unit_created(
        &self,
        _ctx: &spring_addons::AddonContext<'_, Global>,
        id: i32,
        def: i32,
        _team: i32,
        _builder: i32,
    ) {
        if defs::kind(def) == Some(UnitKind::Pirate) {
            self.pirates.borrow_mut().push(id);
        }
    }

    fn unit_destroyed(
        &self,
        _ctx: &spring_addons::AddonContext<'_, Global>,
        event: &UnitDestroyedEvent,
    ) {
        self.pirates.borrow_mut().retain(|id| *id != event.unit_id);
    }

    fn game_frame(&self, ctx: &spring_addons::AddonContext<'_, Global>, _frame: i32) {
        let units: Vec<i32> = {
            let village = ctx.global().village.access();
            let pirates = self.pirates.borrow();
            village
                .eskimos
                .iter()
                .map(|eskimo| eskimo.id)
                .chain(pirates.iter().copied())
                .collect()
        };
        destroy_later(ctx, drowning::update_drowning(units.into_iter()), false);
    }
}
