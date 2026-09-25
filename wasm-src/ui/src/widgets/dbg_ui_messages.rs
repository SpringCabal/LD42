use core::cell::Cell;

use mycube_shared::Command;
use mycube_shared::test_sites::{STEP_COAL, STEP_PARAM, STEP_RESTART, STEP_SALE};
use spring_addons::ui::Widget;

use crate::state::Global;

/// A command the headless integration test sends to the rules, from the
/// first update at or after `frame` once the rules ask for `step`.
struct Step {
    step: i32,
    frame: i32,
    command: fn() -> Option<Command>,
}

/// In order. The rules ask for each step through [`STEP_PARAM`] and wait
/// for its result (`rules-synced/src/integration/`): this widget updates
/// only every so many frames, and the messages go over the network, so the
/// frame they arrive on depends on the game speed. The frames keep the
/// steps' timing at 1x.
const STEPS: [Step; 4] = [
    Step {
        step: STEP_COAL,
        frame: 20,
        command: || Some(Command::BuyHeat),
    },
    // Sell the ice the rules measured after the coal arrived.
    Step {
        step: STEP_SALE,
        frame: 36,
        command: || {
            let map = spring::game::map_size_elmos().ok()?;
            let (x, z) = mycube_shared::test_sites::trade_location(map, |x, z| {
                spring::get_ground_height(x, z).ok()
            })?;
            Some(Command::Trade { x, z })
        },
    },
    // Buy food with the trade's money.
    Step {
        step: STEP_SALE,
        frame: 38,
        command: || Some(Command::BuyFood),
    },
    // Restart the round the way Ctrl+R does.
    Step {
        step: STEP_RESTART,
        frame: 0,
        command: || Some(Command::Restart),
    },
];

/// The step the rules ask for, from the coal burner.
fn asked_step() -> i32 {
    let Ok(burner) = spring::get_unit_def_id_by_name("coalburner") else {
        return 0;
    };
    spring::get_all_units()
        .unwrap_or_default()
        .into_iter()
        .filter(|id| spring::get_unit_def_id(*id).is_ok_and(|def| def.0 == burner))
        .filter_map(|id| spring::get_unit_rules_param_f32(id, STEP_PARAM).ok())
        .fold(0, |asked, step| asked.max(step as i32))
}

/// Sends the headless integration test's trades and restart through the
/// same channel as the player's keys and clicks.
#[derive(Default)]
pub struct DbgUiMessagesWidget {
    sent: Cell<usize>,
}

impl DbgUiMessagesWidget {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Widget<Global> for DbgUiMessagesWidget {
    fn name(&self) -> &'static str {
        "DbgUiMessages"
    }

    fn update(&self, _ctx: &spring_addons::AddonContext<'_, Global>, _dt: f32) {
        let frame = spring::game::current_frame();
        let Some(next) = STEPS.get(self.sent.get()) else {
            return;
        };
        if frame < next.frame {
            return;
        }
        let asked = asked_step();
        while let Some(step) = STEPS.get(self.sent.get()) {
            if frame < step.frame || asked < step.step {
                return;
            }
            self.sent.set(self.sent.get() + 1);
            match (step.command)() {
                Some(command) => crate::rules::send(command),
                None => spring::log_warning!("mycube-ui", "no message for test step {}", step.step),
            }
        }
    }
}
