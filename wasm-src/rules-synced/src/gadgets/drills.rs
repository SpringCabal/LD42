pub(crate) mod behavior;

use spring_addons::rules::{Gadget, UnitDestroyedEvent};

use self::behavior::{self as drills, DrillShip};
use crate::defs::{self, UnitKind};
use crate::state::Global;

#[derive(Default)]
pub struct DrillsGadget;

impl DrillsGadget {
    pub fn new() -> Self {
        Self
    }
}

impl Gadget<Global> for DrillsGadget {
    fn name(&self) -> &'static str {
        "Drills"
    }

    fn unit_created(
        &self,
        ctx: &spring_addons::AddonContext<'_, Global>,
        id: i32,
        def: i32,
        _team: i32,
        _builder: i32,
    ) {
        if defs::kind(def) == Some(UnitKind::Drillship) {
            let frame = spring::game::current_frame();
            ctx.global()
                .drills
                .access_mut()
                .ships
                .push(DrillShip::new(id, frame));
        }
    }

    fn unit_destroyed(
        &self,
        ctx: &spring_addons::AddonContext<'_, Global>,
        event: &UnitDestroyedEvent,
    ) {
        let id = event.unit_id;
        ctx.global()
            .drills
            .access_mut()
            .ships
            .retain(|ship| ship.id != id);
    }

    fn game_frame(&self, ctx: &spring_addons::AddonContext<'_, Global>, frame: i32) {
        let mut state = ctx.global().drills.access_mut();
        drills::drill(&state, frame);
        drills::update_drills(&mut state, frame);
    }
}
