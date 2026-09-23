use spring::rules_params::los::PUBLIC;
use spring_addons::rules::{CommandEvent, Gadget};

use crate::state::Global;
use crate::{defs, untargetable};

/// Command filtering, ported from `unit_untargetable.lua`: scenery can't be
/// targeted.
#[derive(Default)]
pub struct UntargetableGadget;

impl UntargetableGadget {
    pub fn new() -> Self {
        Self
    }
}

impl Gadget<Global> for UntargetableGadget {
    fn name(&self) -> &'static str {
        "Untargetable"
    }

    fn unit_created(
        &self,
        _ctx: &spring_addons::AddonContext<'_, Global>,
        id: i32,
        def: i32,
        _team: i32,
        _builder: i32,
    ) {
        if defs::kind(def).is_some_and(|kind| kind.is_scenery()) {
            let _ = spring::set_unit_rules_param_f32(id, "untargetable", 1.0, PUBLIC);
        }
    }

    fn allow_command(
        &self,
        _ctx: &spring_addons::AddonContext<'_, Global>,
        event: &CommandEvent<'_>,
    ) -> bool {
        untargetable::allow_command(event.command_id, event.command_params)
    }
}
