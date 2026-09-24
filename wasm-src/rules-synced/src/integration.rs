//! The headless integration test: checks at fixed frames, driven by the
//! Integration gadget.

use alloc::format;

use crate::defs::{self, UnitKind};
use crate::drills::Drills;
use crate::state::{Economy, GameEnd, Global, Story, Village};

/// The shared state the checks read and, to drive scenarios, change.
struct Snapshot<'a> {
    economy: &'a mut Economy,
    village: &'a mut Village,
    drills: &'a Drills,
    story: &'a mut Story,
}

/// The test's progress, owned by the Integration gadget.
#[derive(Default)]
pub struct Tracker {
    passed: u32,
    failed: u32,
    done: bool,
    trade_before: Option<(f32, f32, f32)>,
    /// Unit id, health before the hit, and engine damage returned.
    combat: Option<(i32, f32, f32)>,
}

impl Tracker {
    fn check(&mut self, condition: bool, message: &str) {
        let line = if condition {
            self.passed += 1;
            format!(
                "[MyCube-Rust-Test] ASSERTION PASSED ({}): {message}",
                self.passed
            )
        } else {
            self.failed += 1;
            format!("[MyCube-Rust-Test] ASSERTION FAILED: {message}")
        };
        let _ = spring::echo(&line, "");
    }

    pub fn update(&mut self, global: &Global, frame: i32) {
        if self.done {
            return;
        }
        // Test-only bookkeeping. It holds the four resources it inspects for
        // the duration of the check, which is safe here because nothing below
        // issues a callout that re-enters a callin wanting them.
        let mut economy = global.economy.access_mut();
        let mut village = global.village.access_mut();
        let drills = global.drills.access();
        let mut story = global.story.access_mut();
        let state = &mut Snapshot {
            economy: &mut economy,
            village: &mut village,
            drills: &drills,
            story: &mut story,
        };
        match frame {
            15 => self.check_opening(state),
            30 => self.check(
                state.economy.money == 30.0 && state.economy.heat >= 99.0,
                "buy-coal Lua message updates authoritative resources",
            ),
            35 => {
                self.trade_before = trade_location().and_then(|(x, z)| {
                    spring::get_ground_height(x, z)
                        .ok()
                        .map(|height| (x, z, height))
                });
            }
            45 => self.check_first_needs_and_trades(state),
            47 => self.hit_first_eskimo(state),
            48 => make_first_eskimo_hungry(state),
            53 => self.check_combat(),
            55 => self.check(
                state.village.eskimos.first().is_some_and(|eskimo| {
                    eskimo.eating && eskimo.food > 20.0 && state.economy.food < 100.0
                }),
                "hungry Eskimo remains in the authored eating state across multiple frames",
            ),
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
            475 => self.check_story_end(state),
            500 => self.check(
                state.drills.ships.iter().any(|ship| ship.has_order),
                "drillships acquire an ice movement target",
            ),
            1000 => self.check(
                units_of(UnitKind::Parachute).is_empty(),
                "landed parachutes are reclaimed instead of left on the map",
            ),
            1020 => self.finish(),
            _ => {}
        }
        // Run every authored storyteller branch under its normal spawn path,
        // while spacing waves far enough apart to exercise bounded Core work.
        if (50..=470).contains(&frame) && (frame - 50) % 30 == 0 {
            state.story.next_step = frame;
        }
    }

    fn check_opening(&mut self, state: &Snapshot<'_>) {
        self.check(state.story.started, "resource state initialized");
        self.check(
            state.village.eskimos.len() == 10,
            "initial expedition has ten eskimos",
        );
        self.check(
            state
                .village
                .eskimos
                .iter()
                .all(|eskimo| spring::get_unit_position(eskimo.id, false, false).is_ok()),
            "initial eskimos have valid storyteller positions",
        );
        self.check(
            state.economy.food <= 80.0
                && state.economy.heat <= 80.0
                && state.economy.money == 80.0
                && state.economy.food > 79.0
                && state.economy.heat > 79.0,
            "initial resources begin balanced before normal simulation consumption",
        );
        self.check(
            state.story.step >= 1,
            "storyteller dispatched initial scenario",
        );
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

    fn check_first_needs_and_trades(&mut self, state: &Snapshot<'_>) {
        self.check(
            state.story.intro_event == "The last remaining good people, bundled on a block of ice.",
            "authored opening story event is forwarded to the UI",
        );
        self.check(
            state.village.eskimos.iter().all(|unit| {
                spring::get_unit_rules_param_f32(unit.id, "health").is_ok_and(|hp| hp > 0.0)
            }),
            "initial expedition survives early hunger/heat tick",
        );
        self.check(
            state.village.eskimos.iter().any(|unit| unit.food < 90.0),
            "hunger simulation consumes individual food",
        );
        self.check(
            state.economy.money > 30.0,
            "terrain trade Lua message pays authoritative money",
        );
        self.check(
            self.trade_before.is_some_and(|(x, z, before)| {
                spring::get_ground_height(x, z).is_ok_and(|after| after < before - 0.1)
            }),
            "terrain trade lowers the clicked ice",
        );
        self.check(
            state.economy.food >= 99.0,
            "buy-food Lua message consumes trade money and restores food",
        );
    }

    /// Hit the first Eskimo through the same UnitPreDamaged path the engine
    /// uses; frame 53 checks the loss survived the Eskimo needs updates in
    /// between.
    fn hit_first_eskimo(&mut self, state: &Snapshot<'_>) {
        self.combat = state.village.eskimos.first().and_then(|eskimo| {
            let def = defs::def_id(UnitKind::Eskimo)?;
            let health_before = spring::get_unit_rules_param_f32(eskimo.id, "health").ok()?;
            let result = crate::health::unit_pre_damaged(eskimo.id, def);
            Some((eskimo.id, health_before, result.new_damage))
        });
    }

    fn check_combat(&mut self) {
        let combat = self.combat;
        self.check(
            combat.is_some_and(|(id, health_before, engine_damage)| {
                engine_damage == 0.0
                    && spring::get_unit_rules_param_f32(id, "health")
                        .is_ok_and(|remaining| (remaining - (health_before - 10.0)).abs() < 0.01)
            }),
            "combat uses custom attacker damage and script health pool",
        );
    }

    fn check_story_end(&mut self, state: &Snapshot<'_>) {
        self.check(
            state.story.game_end == Some(GameEnd::Victory),
            "storyteller terminal branch publishes victory state",
        );
        self.check(
            state.story.story_eskimos == 20,
            "all initial and reinforcement Eskimo story spawns completed",
        );
        self.check(
            state.story.story_drills == 10
                && state.story.story_pirates == 5
                && state.story.story_parachutes == 5,
            "all authored pirate, drillship, and parachute waves completed",
        );
    }

    fn finish(&mut self) {
        let _ = spring::echo(
            if self.failed == 0 {
                "[MyCube-Rust-Test] ALL MYCUBE INTEGRATION TESTS PASSED"
            } else {
                "[MyCube-Rust-Test] MYCUBE INTEGRATION TESTS FAILED"
            },
            "",
        );
        self.done = true;
    }
}

/// Drive the first Eskimo into the EATING state across multiple frames.
fn make_first_eskimo_hungry(state: &mut Snapshot<'_>) {
    if let Some(eskimo) = state.village.eskimos.first_mut() {
        eskimo.food = 19.0;
        eskimo.eating = false;
        state.economy.food = 100.0;
    }
}

pub fn trade_location() -> Option<(f32, f32)> {
    let [width, height] = spring::get_height_map_size().ok()?;
    let mut best = None;
    for x in (256..width * crate::terrain::SQUARE_SIZE).step_by(512usize) {
        for z in (256..height * crate::terrain::SQUARE_SIZE).step_by(512usize) {
            let ground = spring::get_ground_height(x as f32, z as f32).ok()?;
            if best.is_none_or(|(_, _, highest)| ground > highest) {
                best = Some((x as f32, z as f32, ground));
            }
        }
    }
    best.filter(|(_, _, ground)| *ground > 0.0)
        .map(|(x, z, _)| (x, z))
}

fn units_of(kind: UnitKind) -> alloc::vec::Vec<i32> {
    spring::get_all_units()
        .unwrap_or_default()
        .into_iter()
        .filter(|id| spring::get_unit_def_id(*id).is_ok_and(|def| defs::kind(def.0) == Some(kind)))
        .collect()
}
