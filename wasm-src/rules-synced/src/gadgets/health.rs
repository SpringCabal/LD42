pub mod behavior;
use spring_addons::rules::{Gadget, UnitPreDamagedEvent};

use crate::defs;
use crate::state::Global;

#[derive(Default)]
pub struct HealthGadget;

impl HealthGadget {
    pub fn new() -> Self {
        Self
    }
}

impl Gadget<Global> for HealthGadget {
    fn name(&self) -> &'static str {
        "Health"
    }

    fn unit_created(
        &self,
        _ctx: &spring_addons::AddonContext<'_, Global>,
        id: i32,
        def: i32,
        _team: i32,
        _builder: i32,
    ) {
        let spring = _ctx.spring();
        behavior::set_pool(spring, id, defs::health(spring, def));
    }

    fn unit_pre_damaged(
        &self,
        _ctx: &spring_addons::AddonContext<'_, Global>,
        event: &UnitPreDamagedEvent,
    ) -> Option<spring::DamageResult> {
        let spring = _ctx.spring();
        Some(behavior::unit_pre_damaged(
            spring,
            event.unit_id,
            event.attacker_def_id,
        ))
    }
}
