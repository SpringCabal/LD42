use alloc::vec::Vec;
use core::cell::RefCell;

use spring_addons::rules::{Gadget, UnitDestroyedEvent};

use crate::defs::{self, UnitKind};
use crate::state::Global;
use crate::units::destroy_all;

const DROWNING_DAMAGE_PER_FRAME: f32 = 1.0;

#[derive(Default)]
pub struct DrowningGadget {
    pirates: RefCell<Vec<i32>>,
}

impl DrowningGadget {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Gadget<Global> for DrowningGadget {
    fn name(&self) -> &'static str {
        "Drowning"
    }

    fn unit_created(
        &self,
        _ctx: &spring_addons::AddonContext<'_, Global>,
        id: i32,
        def: i32,
        _team: i32,
        _builder: i32,
    ) {
        if defs::kind(def) == Some(UnitKind::Pirate) {
            self.pirates.borrow_mut().push(id);
        }
    }

    fn unit_destroyed(
        &self,
        _ctx: &spring_addons::AddonContext<'_, Global>,
        event: &UnitDestroyedEvent,
    ) {
        self.pirates.borrow_mut().retain(|id| *id != event.unit_id);
    }

    fn game_frame(&self, ctx: &spring_addons::AddonContext<'_, Global>, _frame: i32) {
        let units: Vec<i32> = {
            let village = ctx.global().village.access();
            let pirates = self.pirates.borrow();
            village
                .eskimos
                .iter()
                .map(|eskimo| eskimo.id)
                .chain(pirates.iter().copied())
                .collect()
        };
        destroy_all(update_drowning(units.into_iter()), false);
    }
}

fn update_drowning(units: impl Iterator<Item = i32>) -> Vec<i32> {
    let mut dead = Vec::new();
    for id in units {
        if let Ok(position) = spring::get_unit_position(id, false, false)
            && spring::get_ground_height(position.x, position.z).unwrap_or(1.0) <= 0.0
        {
            crate::with_cus_module(|host| {
                host.with_script(spring::UnitId(id), |ctx, script| script.start_drowning(ctx))
            });
            let Some(health) = crate::gadgets::health::behavior::pool(id)
                .map(|health| health - DROWNING_DAMAGE_PER_FRAME)
            else {
                continue;
            };
            if health <= 0.0 {
                dead.push(id);
            } else {
                crate::gadgets::health::behavior::set_pool(id, health);
            }
        }
    }
    dead
}
