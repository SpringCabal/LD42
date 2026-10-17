mod behavior;

use spring_addons::rules::{Gadget, UnitDestroyedEvent};

use self::behavior as eskimos;
use crate::defs::{self, UnitKind};
use crate::state::{Eskimo, Global};
use crate::units::{destroy_all, set_eskimo_needs};

const START_FOOD: f32 = 90.0;
const START_HEAT: f32 = 40.0;

#[derive(Default)]
pub struct EskimosGadget;

impl EskimosGadget {
    pub fn new() -> Self {
        Self
    }
}

impl Gadget<Global> for EskimosGadget {
    fn name(&self) -> &'static str {
        "Eskimos"
    }

    fn unit_created(
        &self,
        ctx: &spring_addons::AddonContext<'_, Global>,
        id: i32,
        def: i32,
        _team: i32,
        _builder: i32,
    ) {
        let spring = ctx.spring();
        let mut village = ctx.global().village.access_mut();
        match defs::kind(spring, def) {
            Some(UnitKind::Coalburner) => village.burners.push(id),
            Some(UnitKind::Eskimo) => {
                village.eskimos.push(Eskimo {
                    id,
                    food: START_FOOD,
                    heat: START_HEAT,
                    eating: false,
                    warm: false,
                });
                set_eskimo_needs(spring, id, START_FOOD, START_HEAT);
            }
            _ => {}
        }
    }

    fn unit_destroyed(
        &self,
        ctx: &spring_addons::AddonContext<'_, Global>,
        event: &UnitDestroyedEvent,
    ) {
        let id = event.unit_id;
        let mut village = ctx.global().village.access_mut();
        village.eskimos.retain(|eskimo| eskimo.id != id);
        village.burners.retain(|burner| *burner != id);
    }

    fn game_frame(&self, ctx: &spring_addons::AddonContext<'_, Global>, _frame: i32) {
        let spring = ctx.spring();
        let dead = eskimos::update_eskimos(
            spring,
            &mut ctx.global().village.access_mut(),
            &mut ctx.global().economy.access_mut(),
        );
        destroy_all(ctx, dead, false);
    }
}
