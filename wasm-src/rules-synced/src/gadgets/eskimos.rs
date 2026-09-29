use spring_addons::rules::{Gadget, UnitDestroyedEvent};

use crate::defs::{self, UnitKind};
use crate::eskimos;
use crate::state::{Eskimo, Global};
use crate::units::{destroy_all, set_eskimo_needs};

const START_FOOD: f32 = 90.0;
const START_HEAT: f32 = 40.0;

/// Eskimo food/heat/health bookkeeping, ported from `unit_eskimos.lua`.
/// Tracks the Eskimos and the coal burners that warm them.
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
        let mut village = ctx.global().village.access_mut();
        match defs::kind(def) {
            Some(UnitKind::Coalburner) => village.burners.push(id),
            Some(UnitKind::Eskimo) => {
                village.eskimos.push(Eskimo {
                    id,
                    food: START_FOOD,
                    heat: START_HEAT,
                    eating: false,
                    warm: false,
                });
                set_eskimo_needs(id, START_FOOD, START_HEAT);
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
        let dead = eskimos::update_eskimos(
            &mut ctx.global().village.access_mut(),
            &mut ctx.global().economy.access_mut(),
        );
        destroy_all(dead, false);
    }
}
