use alloc::format;
use core::cell::RefCell;

use crate::state::{Economy, Global, Story, Units};

struct Snapshot<'a> {
    economy: &'a mut Economy,
    units: &'a mut Units,
    story: &'a mut Story,
}

#[derive(Default)]
struct Tracker {
    passed: u32,
    failed: u32,
    done: bool,
    trade_before: Option<(f32, f32, f32)>,
}
thread_local! { static TRACKER: RefCell<Tracker> = RefCell::new(Tracker::default()); }

fn check(t: &mut Tracker, condition: bool, message: &str) {
    if condition {
        t.passed += 1;
        let _ = spring::echo(
            &format!(
                "[MyCube-Rust-Test] ASSERTION PASSED ({}): {message}",
                t.passed
            ),
            "",
        );
    } else {
        t.failed += 1;
        let _ = spring::echo(
            &format!("[MyCube-Rust-Test] ASSERTION FAILED: {message}"),
            "",
        );
    }
}

pub fn update(global: &Global, frame: i32) {
    // Test-only bookkeeping. It holds the three resources it inspects for the
    // duration of the check, which is safe here because nothing below issues a
    // callout that re-enters a callin wanting them.
    let mut economy = global.economy.access_mut();
    let mut units = global.units.access_mut();
    let mut story = global.story.access_mut();
    let state = &mut Snapshot {
        economy: &mut economy,
        units: &mut units,
        story: &mut story,
    };
    TRACKER.with_borrow_mut(|t| {
        if t.done {
            return;
        }
        if frame == 15 {
            check(t, state.story.initialized, "resource state initialized");
            check(
                t,
                state.units.eskimos.len() == 10,
                "initial expedition has ten eskimos",
            );
            check(
                t,
                state
                    .units
                    .eskimos
                    .iter()
                    .all(|eskimo| spring::get_unit_position(eskimo.id, false, false).is_ok()),
                "initial eskimos have valid storyteller positions",
            );
            check(
                t,
                state.economy.food <= 80.0
                    && state.economy.heat <= 80.0
                    && state.economy.money == 80.0
                    && state.economy.food > 79.0
                    && state.economy.heat > 79.0,
                "initial resources begin balanced before normal simulation consumption",
            );
            check(
                t,
                state.story.step >= 1,
                "storyteller dispatched initial scenario",
            );
            let scenery = spring::get_unit_def_id_by_name("coalburner")
                .ok()
                .and_then(|def| {
                    spring::get_all_units().ok()?.into_iter().find(|id| {
                        spring::get_unit_def_id(*id)
                            .ok()
                            .is_some_and(|candidate| candidate.0 == def)
                    })
                });
            check(
                t,
                scenery.is_some_and(|id| {
                    spring::get_unit_rules_param_f32(id, "untargetable") == Ok(1.0)
                        && !crate::untargetable::allow_command(crate::cmd::ATTACK, &[id as f32])
                        && crate::untargetable::allow_command(
                            crate::cmd::MOVE,
                            &[100.0, 0.0, 100.0],
                        )
                }),
                "support scenery rejects targeted commands without blocking movement",
            );
        }
        if frame == 45 {
            check(
                t,
                state.story.intro_event
                    == "The last remaining good people, bundled on a block of ice.",
                "authored opening story event is forwarded to the UI",
            );
            check(
                t,
                state.units.eskimos.iter().all(|unit| unit.health > 0.0),
                "initial expedition survives early hunger/heat tick",
            );
            check(
                t,
                state.units.eskimos.iter().any(|unit| unit.food < 90.0),
                "hunger simulation consumes individual food",
            );
            check(
                t,
                state.economy.money > 30.0,
                "terrain trade Lua message pays authoritative money",
            );
            check(
                t,
                t.trade_before.is_some_and(|(x, z, before)| {
                    spring::get_ground_height(x, z).is_ok_and(|after| after < before - 0.1)
                }),
                "terrain trade lowers the clicked ice",
            );
            check(
                t,
                state.economy.food >= 99.0,
                "buy-food Lua message consumes trade money and restores food",
            );
        }
        if frame == 47 {
            let result = state.units.eskimos.first().and_then(|eskimo| {
                let def = spring::get_unit_def_id_by_name("eskimo").ok()?;
                let health_before = spring::get_unit_rules_param_f32(eskimo.id, "health").ok()?;
                // Call through the same UnitPreDamaged path the engine uses.
                // Keep the state mirror in sync with the script health pool.
                let result = crate::health::unit_pre_damaged(eskimo.id, def);
                Some((result, eskimo.id, health_before))
            });
            if let Some((result, id, health_before)) = result {
                if let Some(eskimo) = state.units.eskimos.iter_mut().find(|unit| unit.id == id) {
                    eskimo.health -= 10.0;
                }
                let health = spring::get_unit_rules_param_f32(id, "health").ok();
                check(
                    t,
                    result.new_damage == 0.0
                        && health.is_some_and(|remaining| {
                            (remaining - (health_before - 10.0)).abs() < 0.01
                        }),
                    "combat uses custom attacker damage and script health pool",
                );
            } else {
                check(t, false, "combat test can resolve the initial eskimo");
            }
        }
        if frame == 48
            && let Some(eskimo) = state.units.eskimos.first_mut()
        {
            // Drive the EATING state across multiple frames.
            eskimo.food = 19.0;
            eskimo.eating = false;
            state.economy.food = 100.0;
        }
        if frame == 55 {
            check(
                t,
                state.units.eskimos.first().is_some_and(|eskimo| {
                    eskimo.eating && eskimo.food > 20.0 && state.economy.food < 100.0
                }),
                "hungry Eskimo remains in the authored eating state across multiple frames",
            );
        }
        if frame == 140 {
            check(
                t,
                state.units.parachutes.iter().any(|id| {
                    let Ok(position) = spring::get_unit_position(*id, false, false) else {
                        return false;
                    };
                    spring::get_ground_height(position.x, position.z)
                        .is_ok_and(|ground| ground < position.y)
                }),
                "enemy parachute remains in flight after spawning",
            );
        }
        if frame == 500 {
            check(
                t,
                state.units.drills.iter().any(|ship| ship.has_order),
                "drillships acquire an ice movement target",
            );
        }
        if frame == 30 {
            check(
                t,
                state.economy.money == 30.0 && state.economy.heat >= 99.0,
                "buy-coal Lua message updates authoritative resources",
            );
        }
        if frame == 35 {
            t.trade_before = trade_location().and_then(|(x, z)| {
                spring::get_ground_height(x, z)
                    .ok()
                    .map(|height| (x, z, height))
            });
        }
        // Run every authored storyteller branch under its normal spawn path,
        // while spacing waves far enough apart to exercise bounded Core work.
        if (50..=470).contains(&frame) && (frame - 50) % 30 == 0 {
            state.story.next_step = frame;
        }
        if frame == 475 {
            check(
                t,
                state.story.victory,
                "storyteller terminal branch publishes victory state",
            );
            check(
                t,
                state.story.story_eskimos == 20,
                "all initial and reinforcement Eskimo story spawns completed",
            );
            check(
                t,
                state.story.story_drills == 10
                    && state.story.story_pirates == 5
                    && state.story.story_parachutes == 5,
                "all authored pirate, drillship, and parachute waves completed",
            );
        }
        if frame == 1000 {
            check(
                t,
                state.units.parachutes.is_empty(),
                "landed parachutes are reclaimed instead of left on the map",
            );
        }
        if frame == 1020 {
            let _ = spring::echo(
                if t.failed == 0 {
                    "[MyCube-Rust-Test] ALL MYCUBE INTEGRATION TESTS PASSED"
                } else {
                    "[MyCube-Rust-Test] MYCUBE INTEGRATION TESTS FAILED"
                },
                "",
            );
            t.done = true;
        }
    });
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
