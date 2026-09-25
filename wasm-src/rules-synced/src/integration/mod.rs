//! The headless integration test: checks at fixed frames, driven by the
//! Integration gadget.
//!
//! The checks borrow the shared state only to read it. Anything that calls
//! the engine (damage, moving or destroying a unit) runs with no borrow held,
//! because the engine raises its callins (UnitPreDamaged, UnitDestroyed) at
//! once, and the gadgets handling them borrow the same state.

mod first_round;
mod second_round;

use alloc::vec::Vec;

use mycube_shared::test_sites::{trade_location, water_location};
use mycube_shared::unit;
use springcabal::check::Suite;

use crate::defs::{self, UnitKind};
use crate::state::{GameEnd, Global};

/// Checks that must pass; fewer means the run skipped some.
const EXPECTED_CHECKS: u32 = 32;

/// Engine damage for the combat check. The health gadget replaces it with
/// the attacker's gameplay damage and lets none of it through.
const ENGINE_HIT: f32 = 5.0;

/// The UI's test driver asks for a restart at frame 1005; give up on it
/// after this frame.
const RESTART_DEADLINE: i32 = 1100;

/// Frames after the restart at which the second round is checked.
const RESTART_CHECK: i32 = 15;
const RESTART_CONDEMN: i32 = 20;
const RESTART_LOSS_CHECK: i32 = 30;

/// How far the scenery check's move order goes; the engine drops a move to
/// where the unit already stands.
const MOVE_STEP: f32 = 100.0;

/// The drowning check puts an Eskimo in the sea on the first frame and
/// checks its health on the second.
const DROWN_START: i32 = 60;
const DROWN_CHECK: i32 = 70;

/// The warming check puts an Eskimo next to the coal burner on the first
/// frame and checks its heat on the second.
const WARM_START: i32 = 80;
const WARM_CHECK: i32 = 90;
/// How far from the burner's centre the Eskimo stands; within its 300-elmo
/// reach and clear of its footprint.
const WARM_OFFSET: f32 = 120.0;

/// The test's progress, owned by the Integration gadget.
pub struct Tracker {
    suite: Suite,
    /// The trade site and its height before the trade.
    trade_before: Option<(f32, f32, f32)>,
    /// Unit id, its gameplay and engine health before the hit.
    combat: Option<(i32, f32, f32)>,
    /// Unit id and its gameplay health when it was put in the water.
    drowning: Option<(i32, f32)>,
    /// Unit id, its heat and the village's heat when it was moved to the
    /// burner.
    warming: Option<(i32, f32, f32)>,
    /// The frame the second round started.
    restart: Option<i32>,
}

impl Default for Tracker {
    fn default() -> Self {
        Self {
            suite: Suite::new(
                "[MyCube-Rust-Test]",
                "MYCUBE INTEGRATION TESTS",
                EXPECTED_CHECKS,
            ),
            trade_before: None,
            combat: None,
            drowning: None,
            warming: None,
            restart: None,
        }
    }
}

impl Tracker {
    fn check(&mut self, condition: bool, message: &str) {
        self.suite.check(condition, message);
    }

    pub fn update(&mut self, global: &Global, frame: i32) {
        if self.suite.is_finished() {
            return;
        }
        if self.restart.is_some() || frame > 1000 {
            self.update_second_round(global, frame);
            return;
        }
        match frame {
            15 => self.check_opening(global),
            30 => {
                let economy = *global.economy.access();
                self.check(
                    economy.money == 30.0 && economy.heat >= 99.0,
                    "buy-coal Lua message updates authoritative resources",
                );
            }
            35 => {
                self.trade_before = trade_location(map_size(), ground).and_then(|(x, z)| {
                    spring::get_ground_height(x, z)
                        .ok()
                        .map(|height| (x, z, height))
                });
            }
            45 => self.check_first_needs_and_trades(global),
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
            500 => self.check_drill_orders(global),
            1000 => self.check(
                units_of(UnitKind::Parachute).is_empty(),
                "landed parachutes are reclaimed instead of left on the map",
            ),
            _ => {}
        }
        // Run every authored storyteller branch under its normal spawn path,
        // while spacing waves far enough apart to exercise bounded Core work.
        if (50..=470).contains(&frame) && (frame - 50) % 30 == 0 {
            global.story.access_mut().next_step = frame;
        }
    }
}

/// The opening stocks, less at most one frame of the Eskimos' needs.
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
