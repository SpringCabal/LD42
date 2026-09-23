use spring_addons::rules::Gadget;

use crate::state::Global;
use crate::unit_scripts;

/// Queues new units for the unit-script module, which attaches their
/// scripts on its next tick.
#[derive(Default)]
pub struct UnitScriptsGadget;

impl UnitScriptsGadget {
    pub fn new() -> Self {
        Self
    }
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
        unit_scripts::queue_unit_created(id, def);
    }
}
