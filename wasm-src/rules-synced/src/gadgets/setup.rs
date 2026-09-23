use spring_addons::register_gadgets;
use spring_addons::rules::{Gadget, GadgetHandler};

use super::*;
use crate::state::Global;

/// Global LOS, ported from `globals.lua`: MyCube is a single-player
/// scenario, so both ally teams see the whole map; otherwise the enemy
/// ships spawn at the map edge but stay invisible to the player.
#[derive(Default)]
pub struct SetupGadget;

impl SetupGadget {
    pub fn new() -> Self {
        Self
    }
}

impl Gadget<Global> for SetupGadget {
    fn name(&self) -> &'static str {
        "Setup"
    }

    fn init(&self, _ctx: &spring_addons::AddonContext<'_, Global>) {
        for ally_team in 0..2 {
            let _ = spring::team_control::set_global_los(ally_team, true);
        }
    }
}

pub fn setup(handler: &mut GadgetHandler<Global>) {
    register_gadgets!(
        handler,
        [
            SetupGadget::new(),
            StorytellerGadget::new(),
            UnitScriptsGadget::new(),
            HealthGadget::new(),
            UntargetableGadget::new(),
            EskimosGadget::new(),
            DrowningGadget::new(),
            DrillsGadget::new(),
            ParachutesGadget::new(),
            GameEndGadget::new(),
            StatusGadget::new(),
            TradeGadget::new(),
        ]
    );

    #[cfg(feature = "headless-integration")]
    register_gadgets!(handler, [IntegrationGadget::new()]);
}
