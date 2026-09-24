//! The headless integration test: checks at fixed frames, driven by the
//! Integration gadget.
//!
//! The checks borrow the shared state only to read it. Anything that calls
//! the engine (damage, moving or destroying a unit) runs with no borrow held,
//! because the engine raises its callins (UnitPreDamaged, UnitDestroyed) at
//! once, and the gadgets handling them borrow the same state.

use alloc::vec::Vec;

use springcabal::check::Suite;

use crate::defs::{self, UnitKind};
use crate::state::{GameEnd, Global};
use crate::test_sites::{trade_location, water_location};

/// Checks that must pass; fewer means the run skipped some.
const EXPECTED_CHECKS: u32 = 29;

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
            suite: Suite::new("[MyCube-Rust-Test]", "MYCUBE INTEGRATION TESTS")
                .require_at_least(EXPECTED_CHECKS),
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
                self.trade_before = trade_location().and_then(|(x, z)| {
                    spring::get_ground_height(x, z)
                        .ok()
                        .map(|height| (x, z, height))
                });
            }
            45 => self.check_first_needs_and_trades(global),
            47 => self.hit_first_eskimo(global),
            48 => make_first_eskimo_hungry(global),
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

    fn check_opening(&mut self, global: &Global) {
        let (started, step) = {
            let story = global.story.access();
            (story.started, story.step)
        };
        let eskimos = eskimo_ids(global);
        let economy = *global.economy.access();
        self.check(started, "resource state initialized");
        self.check(eskimos.len() == 10, "initial expedition has ten eskimos");
        self.check(
            eskimos
                .iter()
                .all(|id| spring::get_unit_position(*id, false, false).is_ok()),
            "initial eskimos have valid storyteller positions",
        );
        self.check(
            opening_stocks(&economy),
            "initial resources begin balanced before normal simulation consumption",
        );
        self.check(step >= 1, "storyteller dispatched initial scenario");
        let scenery = units_of(UnitKind::Coalburner).first().copied();
        self.check(
            scenery.is_some_and(|id| {
                spring::get_unit_rules_param_f32(id, "untargetable") == Ok(1.0)
                    && !crate::untargetable::allow_command(spring::cmd::ATTACK, &[id as f32])
                    && crate::untargetable::allow_command(spring::cmd::MOVE, &[100.0, 0.0, 100.0])
            }),
            "support scenery rejects targeted commands without blocking movement",
        );
    }

    fn check_first_needs_and_trades(&mut self, global: &Global) {
        let intro_forwarded = global.story.access().intro_event
            == "The last remaining good people, bundled on a block of ice.";
        let (alive, hungry) = {
            let village = global.village.access();
            (
                village.eskimos.iter().all(|unit| {
                    spring::get_unit_rules_param_f32(unit.id, "health").is_ok_and(|hp| hp > 0.0)
                }),
                village.eskimos.iter().any(|unit| unit.food < 90.0),
            )
        };
        let economy = *global.economy.access();
        self.check(
            intro_forwarded,
            "authored opening story event is forwarded to the UI",
        );
        self.check(alive, "initial expedition survives early hunger/heat tick");
        self.check(hungry, "hunger simulation consumes individual food");
        self.check(
            economy.money > 30.0,
            "terrain trade Lua message pays authoritative money",
        );
        self.check(
            self.trade_before.is_some_and(|(x, z, before)| {
                spring::get_ground_height(x, z).is_ok_and(|after| after < before - 0.1)
            }),
            "terrain trade lowers the clicked ice",
        );
        self.check(
            economy.food >= 99.0,
            "buy-food Lua message consumes trade money and restores food",
        );
    }

    /// One Eskimo hits another through the engine: `AddUnitDamage` raises
    /// UnitPreDamaged, where the health gadget takes the attacker's
    /// gameplay damage off the pool. Frame 53 checks that the loss survived
    /// the Eskimo needs updates in between.
    fn hit_first_eskimo(&mut self, global: &Global) {
        let eskimos = eskimo_ids(global);
        let (Some(&target), Some(&attacker)) = (eskimos.first(), eskimos.get(1)) else {
            return;
        };
        let (Ok(pool), Ok(engine)) = (
            spring::get_unit_rules_param_f32(target, "health"),
            spring::get_unit_health(target),
        ) else {
            return;
        };
        let hit = spring::unit_control::add_unit_damage(
            target,
            ENGINE_HIT,
            0.0,
            -1,
            attacker,
            spring::Float3::new(0.0, 0.0, 0.0),
        );
        if let Err(error) = hit {
            spring::log_warning!("mycube-test", "damaging unit {target} failed: {error:?}");
            return;
        }
        self.combat = Some((target, pool, engine.health));
    }

    fn check_combat(&mut self) {
        let eskimo_damage = defs::def_id(UnitKind::Eskimo).map_or(0.0, defs::damage);
        let combat = self.combat;
        self.check(
            eskimo_damage > 0.0
                && combat.is_some_and(|(id, pool_before, engine_before)| {
                    spring::get_unit_rules_param_f32(id, "health")
                        .is_ok_and(|pool| (pool - (pool_before - eskimo_damage)).abs() < 0.01)
                        && spring::get_unit_health(id)
                            .is_ok_and(|engine| engine.health == engine_before)
                }),
            "an engine hit takes the attacker's damage off the health pool and none off the unit",
        );
    }

    /// Put the last Eskimo in the sea; `unit_drown_eskimos.lua` takes a
    /// point of health on every frame there.
    fn drown_last_eskimo(&mut self, global: &Global) {
        let Some(&id) = eskimo_ids(global).last() else {
            return;
        };
        let (Some((x, z)), Ok(pool)) = (
            water_location(),
            spring::get_unit_rules_param_f32(id, "health"),
        ) else {
            return;
        };
        let ground = spring::get_ground_height(x, z).unwrap_or(0.0);
        match spring::unit_control::set_unit_position(id, spring::Float3::new(x, ground, z)) {
            Ok(_) => self.drowning = Some((id, pool)),
            Err(error) => {
                spring::log_warning!("mycube-test", "moving unit {id} failed: {error:?}");
            }
        }
    }

    fn check_drowning(&mut self) {
        let frames = (DROWN_CHECK - DROWN_START) as f32;
        let drowning = self.drowning;
        self.check(
            drowning.is_some_and(|(id, before)| {
                spring::get_unit_rules_param_f32(id, "health")
                    .is_ok_and(|pool| before - pool >= frames - 1.0)
            }),
            "an Eskimo in the sea loses a point of health per frame",
        );
    }

    /// Put the third Eskimo next to the coal burner; `unit_eskimos.lua`
    /// warms it there with the village's heat.
    fn warm_third_eskimo(&mut self, global: &Global) {
        let Some(id) = global
            .village
            .access()
            .eskimos
            .get(2)
            .map(|eskimo| eskimo.id)
        else {
            return;
        };
        let Some(burner) = units_of(UnitKind::Coalburner)
            .first()
            .and_then(|burner| spring::get_unit_position(*burner, false, false).ok())
        else {
            return;
        };
        let (x, z) = (burner.x + WARM_OFFSET, burner.z);
        let ground = spring::get_ground_height(x, z).unwrap_or(burner.y);
        if let Err(error) =
            spring::unit_control::set_unit_position(id, spring::Float3::new(x, ground, z))
        {
            spring::log_warning!("mycube-test", "moving unit {id} failed: {error:?}");
            return;
        }
        let heat = global
            .village
            .access()
            .eskimos
            .iter()
            .find(|eskimo| eskimo.id == id)
            .map(|eskimo| eskimo.heat);
        let stock = global.economy.access().heat;
        self.warming = heat.map(|heat| (id, heat, stock));
    }

    fn check_warming(&mut self, global: &Global) {
        let warmed = self.warming.is_some_and(|(id, heat_before, stock_before)| {
            let heat = global
                .village
                .access()
                .eskimos
                .iter()
                .find(|eskimo| eskimo.id == id)
                .map(|eskimo| eskimo.heat);
            heat.is_some_and(|heat| heat > heat_before)
                && global.economy.access().heat < stock_before
        });
        self.check(
            warmed,
            "an Eskimo by the coal burner warms up on the village's heat",
        );
    }

    fn check_story_end(&mut self, global: &Global) {
        let story = global.story.access();
        let victory = story.game_end == Some(GameEnd::Victory);
        let eskimos = story.story_eskimos;
        let waves = (
            story.story_drills,
            story.story_pirates,
            story.story_parachutes,
        );
        drop(story);
        self.check(
            victory,
            "storyteller terminal branch publishes victory state",
        );
        self.check(
            eskimos == 20,
            "all initial and reinforcement Eskimo story spawns completed",
        );
        self.check(
            waves == (10, 5, 5),
            "all authored pirate, drillship, and parachute waves completed",
        );
    }

    /// The drill gadget orders ships toward ice; the order must be in the
    /// engine's command queue, not only in the gadget's bookkeeping.
    fn check_drill_orders(&mut self, global: &Global) {
        let targets: Vec<(i32, f32, f32)> = global
            .drills
            .access()
            .ships
            .iter()
            .filter(|ship| ship.has_order)
            .map(|ship| (ship.id, ship.order_x, ship.order_z))
            .collect();
        self.check(
            targets.iter().any(|&(id, x, z)| {
                spring::units_commands::get_unit_commands(id, 1).is_ok_and(|commands| {
                    commands.first().is_some_and(|command| {
                        command.cmd_id == spring::cmd::MOVE
                            && command.params.len() >= 3
                            && (command.params[0] - x).abs() < 1.0
                            && (command.params[2] - z).abs() < 1.0
                    })
                })
            }),
            "drillships are ordered to move to the ice they found",
        );
    }

    /// The UI's test driver restarts the round the way Ctrl+R does. Check
    /// the new round, then lose it.
    fn update_second_round(&mut self, global: &Global, frame: i32) {
        let Some(start) = self.restart.or_else(|| {
            let story = global.story.access();
            (story.started && story.start_frame > 1000).then_some(story.start_frame)
        }) else {
            if frame > RESTART_DEADLINE {
                self.check(false, "the UI's restart request reaches the rules");
                self.suite.finish();
            }
            return;
        };
        self.restart = Some(start);
        match frame - start {
            RESTART_CHECK => self.check_restart(global, start),
            RESTART_CONDEMN => {
                for id in eskimo_ids(global) {
                    crate::units::destroy(id, false);
                }
            }
            RESTART_LOSS_CHECK => {
                let (end, survived) = {
                    let story = global.story.access();
                    (story.game_end, story.survival_frames())
                };
                self.check(
                    end == Some(GameEnd::Loss),
                    "losing every Eskimo ends the round in a loss",
                );
                // The Eskimos die on RESTART_CONDEMN; the loss is called on
                // the next frame.
                self.check(
                    survived == Some(RESTART_CONDEMN + 1),
                    "the survival time counts only the restarted round",
                );
                self.suite.finish();
            }
            _ => {}
        }
    }

    fn check_restart(&mut self, global: &Global, start: i32) {
        let (end, step) = {
            let story = global.story.access();
            (story.game_end, story.step)
        };
        let economy = *global.economy.access();
        let tracked = eskimo_ids(global).len();
        self.check(
            end.is_none() && step >= 1,
            "a restart clears the finished round and starts the story over",
        );
        self.check(
            tracked == 10
                && units_of(UnitKind::Eskimo).len() == 10
                && units_of(UnitKind::Drillship).is_empty()
                && units_of(UnitKind::Pirate).is_empty(),
            "a restart replaces every unit with a fresh expedition",
        );
        self.check(
            opening_stocks(&economy),
            "a restart refills the village's stocks",
        );
        self.check(
            self.trade_before.is_some_and(|(x, z, before)| {
                spring::get_ground_height(x, z).is_ok_and(|now| (now - before).abs() < 0.1)
            }),
            "a restart restores the ice the first round sold",
        );
        spring::log_info!("mycube-test", "the second round started at frame {start}");
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

/// Drive the first Eskimo into the EATING state across multiple frames.
fn make_first_eskimo_hungry(global: &Global) {
    if let Some(eskimo) = global.village.access_mut().eskimos.first_mut() {
        eskimo.food = 19.0;
        eskimo.eating = false;
    }
    global.economy.access_mut().food = 100.0;
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
