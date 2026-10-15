use mycube_shared::unit;
use spring_addons::rules::{CommandEvent, Gadget};

use crate::state::Global;

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
        let target = match event.command_params {
            [_, _, _, target] if event.command_id == spring::cmd::INSERT => *target,
            [target] if event.command_id != spring::cmd::INSERT => *target,
            _ => return true,
        };
        spring::get_unit_rules_param_f32(target as i32, unit::UNTARGETABLE) != Ok(1.0)
    }
}
