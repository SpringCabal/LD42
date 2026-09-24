use alloc::vec::Vec;
use core::cell::RefCell;

use spring_addons::rules::Gadget;

use crate::state::Global;

/// Attaches each new unit's script to the unit-script module.
#[derive(Default)]
pub struct UnitScriptsGadget {
    /// Units created while the unit-script module was busy: inside one of its
    /// own callins, or before the engine initialized it (the map's units).
    /// They are attached on the next frame.
    pending: RefCell<Vec<(i32, i32)>>,
}

impl UnitScriptsGadget {
    pub fn new() -> Self {
        Self::default()
    }
}

fn attach(unit: i32, def: i32) -> bool {
    crate::with_cus_module(|module| module.attach(unit, def)).is_some()
}

impl Gadget<Global> for UnitScriptsGadget {
    fn name(&self) -> &'static str {
        "UnitScripts"
    }

    fn unit_created(
        &self,
        _ctx: &spring_addons::AddonContext<'_, Global>,
        id: i32,
        def: i32,
        _team: i32,
        _builder: i32,
    ) {
        if !attach(id, def) {
            self.pending.borrow_mut().push((id, def));
        }
    }

    fn game_frame(&self, _ctx: &spring_addons::AddonContext<'_, Global>, _frame: i32) {
        let pending = core::mem::take(&mut *self.pending.borrow_mut());
        for (id, def) in pending {
            let alive = spring::valid_unit_id(id).unwrap_or(false)
                && !spring::get_unit_is_dead(id).unwrap_or(true);
            if alive && !attach(id, def) {
                self.pending.borrow_mut().push((id, def));
            }
        }
    }
}
