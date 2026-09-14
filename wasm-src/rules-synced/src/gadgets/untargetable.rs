use spring_addons::rules::{CommandEvent, Gadget};

use crate::state::Global;
use crate::untargetable;

/// Command filtering, ported from `unit_untargetable.lua`.
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

    fn allow_command(
        &self,
        _ctx: &spring_addons::AddonContext<'_, Global>,
        event: &CommandEvent<'_>,
    ) -> bool {
        untargetable::allow_command(event.command_id, event.command_params)
    }
}
