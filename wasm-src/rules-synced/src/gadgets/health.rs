use spring_addons::rules::{Gadget, UnitPreDamagedEvent};

use crate::health;
use crate::state::Global;

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

    fn unit_pre_damaged(
        &self,
        _ctx: &spring_addons::AddonContext<'_, Global>,
        event: &UnitPreDamagedEvent,
    ) -> Option<spring::DamageResult> {
        Some(health::unit_pre_damaged(
            event.unit_id,
            event.unit_def_id,
            event.unit_team,
            event.damage,
            event.paralyzer,
            event.weapon_def_id,
            event.projectile_id,
            event.attacker_id,
            event.attacker_def_id,
            event.attacker_team,
        ))
    }
}
