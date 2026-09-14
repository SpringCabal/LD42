use spring_addons::register_gadgets;
use spring_addons::rules::{Gadget, GadgetHandler};

use super::*;
use crate::state::{self, Global};

/// Resource initialization, ported from `globals.lua`.
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

    fn game_frame(&self, ctx: &spring_addons::AddonContext<'_, Global>, frame: i32) {
        // Match LuaRules/Gadgets/globals.lua.  MyCube is a single-player
        // scenario, so both the player's and the enemy's ally-team LOS must
        // remain globally revealed; otherwise the authored ships spawn at
        // the map edge but stay invisible to the player.
        for ally_team in 0..2 {
            let _ = spring::team_control::set_global_los(ally_team, true);
        }
        if frame == 0 {
            let _ = spring::echo("[MyCube] rules: bounded-drills/map-dimension-spawns", "");
        }
        state::initialize(ctx.global());
    }
}

pub fn setup(handler: &mut GadgetHandler<Global>) {
    register_gadgets!(
        handler,
        [
            SetupGadget::new(),
            StorytellerGadget::new(),
            EskimosGadget::new(),
            DrowningGadget::new(),
            DrillsGadget::new(),
            ParachutesGadget::new(),
            GameEndGadget::new(),
            StatusGadget::new(),
        ]
    );

    #[cfg(feature = "headless-integration")]
    register_gadgets!(handler, [IntegrationGadget::new()]);

    register_gadgets!(
        handler,
        [
            LifecycleGadget::new(),
            HealthGadget::new(),
            TradeGadget::new(),
            UntargetableGadget::new(),
        ]
    );

    register_gadgets!(handler, [ReaperGadget::new()]);
}
