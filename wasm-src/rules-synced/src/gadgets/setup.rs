use spring_addons::register_gadgets;
use spring_addons::rules::GadgetHandler;

use super::*;
use crate::state::Global;

pub fn setup(
    handler: &mut GadgetHandler<Global>,
    _spring: &spring::Spring<'_, spring::RulesSynced>,
) {
    register_gadgets!(
        handler,
        [
            GlobalLosGadget::new(),
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
            RestartGadget::new(),
        ]
    );

    #[cfg(feature = "headless-integration")]
    register_gadgets!(handler, [IntegrationGadget::new()]);
}
