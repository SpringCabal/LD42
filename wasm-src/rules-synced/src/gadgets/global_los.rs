use spring_addons::rules::Gadget;

use crate::state::Global;

#[derive(Default)]
pub struct GlobalLosGadget;

impl GlobalLosGadget {
    pub fn new() -> Self {
        Self
    }
}

impl Gadget<Global> for GlobalLosGadget {
    fn name(&self) -> &'static str {
        "GlobalLos"
    }

    fn init(&self, _ctx: &spring_addons::AddonContext<'_, Global>) {
        let spring = _ctx.spring();
        for ally_team in 0..2 {
            if let Err(error) = spring.team_control().set_global_los(ally_team, true) {
                spring::log_warning!(
                    spring,
                    "mycube",
                    "global LOS for ally team {ally_team} failed: {error:?}"
                );
            }
        }
    }
}
