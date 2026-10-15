use spring::UnitId;
use spring_addons::rules::Gadget;

use crate::state::Global;

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

    fn init(&self, _ctx: &spring_addons::AddonContext<'_, Global>) {
        crate::with_cus_module_or_defer(|host| host.attach_existing());
    }

    fn unit_created(
        &self,
        _ctx: &spring_addons::AddonContext<'_, Global>,
        id: i32,
        def: i32,
        _team: i32,
        _builder: i32,
    ) {
        crate::with_cus_module_or_defer(move |host| host.unit_created(UnitId(id), def));
    }

    fn game_frame(&self, _ctx: &spring_addons::AddonContext<'_, Global>, frame: i32) {
        crate::with_cus_module_or_defer(move |host| host.game_frame(frame));
    }
}
