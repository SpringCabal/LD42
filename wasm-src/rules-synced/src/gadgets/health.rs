use spring::rules_params::los::PUBLIC;
use spring_addons::rules::{Gadget, UnitPreDamagedEvent};

use crate::state::Global;
use crate::{defs, health};

/// The gameplay health pool, ported from `game_health.lua`.
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
        let _ = spring::set_unit_rules_param_f32(id, "health", defs::health(def), PUBLIC);
    }

    fn unit_pre_damaged(
        &self,
        _ctx: &spring_addons::AddonContext<'_, Global>,
        event: &UnitPreDamagedEvent,
    ) -> Option<spring::DamageResult> {
        Some(health::unit_pre_damaged(
            event.unit_id,
            event.attacker_def_id,
        ))
    }
}
