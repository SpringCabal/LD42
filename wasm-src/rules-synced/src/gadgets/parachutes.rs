use alloc::vec::Vec;
use core::cell::RefCell;

use spring_addons::rules::{Gadget, UnitDestroyedEvent};

use crate::defs::{self, UnitKind};
use crate::state::Global;
use crate::units::destroy_all;

#[derive(Default)]
pub struct ParachutesGadget {
    parachutes: RefCell<Vec<i32>>,
}

impl ParachutesGadget {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Gadget<Global> for ParachutesGadget {
    fn name(&self) -> &'static str {
        "Parachutes"
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
        if defs::kind(spring, def) == Some(UnitKind::Parachute) {
            self.parachutes.borrow_mut().push(id);
        }
    }

    fn unit_destroyed(
        &self,
        _ctx: &spring_addons::AddonContext<'_, Global>,
        event: &UnitDestroyedEvent,
    ) {
        self.parachutes
            .borrow_mut()
            .retain(|id| *id != event.unit_id);
    }

    fn game_frame(&self, _ctx: &spring_addons::AddonContext<'_, Global>, _frame: i32) {
        let spring = _ctx.spring();
        destroy_all(
            _ctx,
            update_parachutes(spring, &self.parachutes.borrow()),
            true,
        );
    }
}

fn update_parachutes(
    spring: &spring::Spring<'_, spring::RulesSynced>,
    parachutes: &[i32],
) -> Vec<i32> {
    let mut landed = Vec::new();
    for id in parachutes {
        let Ok(position) = spring
            .units_info()
            .get_unit_position(*id, Default::default())
        else {
            continue;
        };
        let Ok(ground) = spring.terrain().get_ground_height(position.x, position.z) else {
            continue;
        };
        if ground >= position.y {
            landed.push(*id);
        }
    }
    landed
}
