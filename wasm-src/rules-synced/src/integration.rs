mod first_round;
mod second_round;

use alloc::rc::{Rc, Weak};
use alloc::vec::Vec;
use core::cell::RefCell;

use mycube_shared::test_sites::{
    STEP_COAL, STEP_PARAM, STEP_RESTART, STEP_SALE, trade_location, water_location,
};
use mycube_shared::unit;
use springcabal::check::Suite;

use crate::defs::{self, UnitKind};
use crate::state::{GameEnd, Global};

const EXPECTED_CHECKS: u32 = 32;

const ENGINE_HIT: f32 = 5.0;

const OPENING_CHECK: i32 = 15;

const SALE_CHECK: i32 = 45;

const FIRST_ROUND_END: i32 = 1000;

const UI_TIMEOUT: i32 = 200;

const OPENING_MONEY: f32 = 80.0;
const MONEY_AFTER_COAL: f32 = 30.0;

const RESTART_CHECK: i32 = 15;
const RESTART_CONDEMN: i32 = 20;
const RESTART_LOSS_CHECK: i32 = 30;

const MOVE_STEP: f32 = 100.0;

const DROWN_START: i32 = 60;
const DROWN_CHECK: i32 = 70;

const WARM_START: i32 = 80;
const WARM_CHECK: i32 = 90;
const WARM_OFFSET: f32 = 120.0;

#[derive(Clone, Copy)]
enum Stage {
    Opening,
    Coal { since: i32 },
    Sale { since: i32 },
    Schedule { delay: i32 },
    SecondRound { since: i32 },
}

pub struct Tracker {
    this: Weak<RefCell<Tracker>>,
    suite: Suite,
    stage: Stage,
    trade_before: Option<(f32, f32, f32)>,
    combat: Option<(i32, f32, f32)>,
    drowning: Option<(i32, f32)>,
    warming: Option<(i32, f32, f32)>,
    restart: Option<i32>,
}

impl Tracker {
    pub fn new_shared() -> Rc<RefCell<Self>> {
        Rc::new_cyclic(|this| {
            RefCell::new(Self {
                this: this.clone(),
                suite: Suite::new(
                    "[MyCube-Rust-Test]",
                    "MYCUBE INTEGRATION TESTS",
                    EXPECTED_CHECKS,
                ),
                stage: Stage::Opening,
                trade_before: None,
                combat: None,
                drowning: None,
                warming: None,
                restart: None,
            })
        })
    }

    fn after_commands(&self, f: impl FnOnce(&mut Tracker) + 'static) {
        let this = self.this.clone();
        spring::commands::run(move || {
            if let Some(tracker) = this.upgrade() {
                f(&mut tracker.borrow_mut());
            }
        });
    }

    fn then<T: 'static>(
        &self,
        command: spring::commands::Pending<T>,
        f: impl FnOnce(&mut Tracker, T) + 'static,
    ) {
        let this = self.this.clone();
        command.then(move |result| {
            if let Some(tracker) = this.upgrade() {
                f(&mut tracker.borrow_mut(), result);
            }
        });
    }

    fn check(&mut self, condition: bool, message: &str) {
        self.suite.check(condition, message);
    }

    pub fn update(&mut self, global: &Global, frame: i32) {
        if self.suite.is_finished() {
            return;
        }
        match self.stage {
            Stage::Opening if frame >= OPENING_CHECK => {
                self.check_opening(global);
                ask_ui(STEP_COAL);
                self.stage = Stage::Coal { since: frame };
            }
            Stage::Opening => {}
            Stage::Coal { since } => self.wait_for_coal(global, frame, since),
            Stage::Sale { since } => self.wait_for_sale(global, frame, since),
            Stage::Schedule { delay } => self.update_first_round(global, frame, delay),
            Stage::SecondRound { since } => self.update_second_round(global, frame, since),
        }
    }

    fn wait_for_coal(&mut self, global: &Global, frame: i32, since: i32) {
        let economy = *global.economy.access();
        let arrived = economy.money != OPENING_MONEY;
        if !arrived && !timed_out(frame, since, "the UI's coal purchase") {
            return;
        }
        self.check(
            economy.money == MONEY_AFTER_COAL && economy.heat >= 99.0,
            "buy-coal Lua message updates authoritative resources",
        );
        self.trade_before = trade_location(map_size(), ground).and_then(|(x, z)| {
            spring::get_ground_height(x, z)
                .ok()
                .map(|height| (x, z, height))
        });
        if arrived {
            ask_ui(STEP_SALE);
            self.stage = Stage::Sale { since: frame };
        } else {
            self.suite.finish();
        }
    }

    fn wait_for_sale(&mut self, global: &Global, frame: i32, since: i32) {
        let lowered = self.trade_before.is_some_and(|(x, z, before)| {
            spring::get_ground_height(x, z).is_ok_and(|after| after < before - 0.1)
        });
        let arrived = lowered && global.economy.access().food >= 99.0;
        if !arrived && !timed_out(frame, since, "the UI's ice sale and food purchase") {
            return;
        }
        if frame < SALE_CHECK {
            return;
        }
        self.check_first_needs_and_trades(global);
        if arrived {
            self.stage = Stage::Schedule {
                delay: frame - SALE_CHECK,
            };
        } else {
            self.suite.finish();
        }
    }

    fn update_first_round(&mut self, global: &Global, frame: i32, delay: i32) {
        let nominal = frame - delay;
        match nominal {
            47 => self.hit_first_eskimo(global),
            48 => first_round::make_first_eskimo_hungry(global),
            53 => self.check_combat(),
            55 => {
                let economy_food = global.economy.access().food;
                let eating = global
                    .village
                    .access()
                    .eskimos
                    .first()
                    .is_some_and(|eskimo| {
                        eskimo.eating && eskimo.food > 20.0 && economy_food < 100.0
                    });
                self.check(
                    eating,
                    "hungry Eskimo remains in the authored eating state across multiple frames",
                );
            }
            DROWN_START => self.drown_last_eskimo(global),
            DROWN_CHECK => self.check_drowning(),
            WARM_START => self.warm_third_eskimo(global),
            WARM_CHECK => self.check_warming(global),
            140 => self.check(
                units_of(UnitKind::Parachute).iter().any(|id| {
                    let Ok(position) = spring::get_unit_position(*id, false, false) else {
                        return false;
                    };
                    spring::get_ground_height(position.x, position.z)
                        .is_ok_and(|ground| ground < position.y)
                }),
                "enemy parachute remains in flight after spawning",
            ),
            475 => self.check_story_end(global),
            500 => self.check_drill_orders(),
            FIRST_ROUND_END => {
                self.check(
                    units_of(UnitKind::Parachute).is_empty(),
                    "landed parachutes are reclaimed instead of left on the map",
                );
                ask_ui(STEP_RESTART);
                self.stage = Stage::SecondRound { since: frame };
            }
            _ => {}
        }
        if (50..=470).contains(&nominal) && (nominal - 50) % 30 == 0 {
            global.story.access_mut().next_step = frame;
        }
    }
}

fn timed_out(frame: i32, since: i32, what: &str) -> bool {
    if frame - since <= UI_TIMEOUT {
        return false;
    }
    spring::log_warning!(
        "mycube-test",
        "{what} did not arrive within {UI_TIMEOUT} frames"
    );
    true
}

fn ask_ui(step: i32) {
    let Some(&burner) = units_of(UnitKind::Coalburner).first() else {
        spring::log_warning!(
            "mycube-test",
            "no coal burner to ask the UI for step {step}"
        );
        return;
    };
    let asked = spring::set_unit_rules_param_f32(
        burner,
        STEP_PARAM,
        step as f32,
        spring::rules_params::los::PUBLIC,
    );
    if let Err(error) = asked {
        spring::log_warning!(
            "mycube-test",
            "asking the UI for step {step} failed: {error:?}"
        );
    }
}

fn opening_stocks(economy: &crate::state::Economy) -> bool {
    economy.food <= 80.0
        && economy.heat <= 80.0
        && economy.money == 80.0
        && economy.food > 79.0
        && economy.heat > 79.0
}

fn eskimo_ids(global: &Global) -> Vec<i32> {
    global
        .village
        .access()
        .eskimos
        .iter()
        .map(|eskimo| eskimo.id)
        .collect()
}

fn units_of(kind: UnitKind) -> Vec<i32> {
    spring::get_all_units()
        .unwrap_or_default()
        .into_iter()
        .filter(|id| spring::get_unit_def_id(*id).is_ok_and(|def| defs::kind(def.0) == Some(kind)))
        .collect()
}

fn map_size() -> (i32, i32) {
    spring::game::map_size_elmos().unwrap_or_default()
}

fn ground(x: f32, z: f32) -> Option<f32> {
    spring::get_ground_height(x, z).ok()
}
