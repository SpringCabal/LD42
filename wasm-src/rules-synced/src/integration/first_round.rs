//! The first round's checks, frames 15 to 1000.

use spring::cmd::options::ALT_KEY;
use spring::cmd::{ATTACK, INSERT, MOVE};

use super::*;

impl Tracker {
    pub(super) fn check_opening(&mut self, global: &Global) {
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
        self.check_scenery_orders(&eskimos);
    }

    /// Orders through the engine, whose AllowCommand reaches the
    /// Untargetable gadget: attacks on the coal burner are dropped, while an
    /// attack on an Eskimo and a move are queued. Each order is cleared
    /// again at once, so no shot is fired and nobody walks off.
    pub(super) fn check_scenery_orders(&mut self, eskimos: &[i32]) {
        let burner = units_of(UnitKind::Coalburner).first().copied();
        let (Some(&unit), Some(&other), Some(burner)) = (eskimos.get(3), eskimos.get(4), burner)
        else {
            self.check(
                false,
                "an attack order on the coal burner is dropped, one on an Eskimo is kept",
            );
            self.check(
                false,
                "an inserted attack order on the coal burner is dropped",
            );
            self.check(false, "a move order next to the scenery is kept");
            return;
        };
        let position = spring::get_unit_position(unit, false, false).unwrap_or_default();
        let step = [position.x + MOVE_STEP, position.y, position.z];
        // INSERT with ALT_KEY takes a queue position: -1 is the end.
        let insert_attack = |target: i32| [-1.0, ATTACK as f32, 0.0, target as f32];
        let attack_other = queued_orders(unit, &[(ATTACK, &[other as f32])]);
        let attack_burner = queued_orders(unit, &[(ATTACK, &[burner as f32])]);
        let insert_other = queued_orders(unit, &[(MOVE, &step), (INSERT, &insert_attack(other))]);
        let insert_burner = queued_orders(unit, &[(MOVE, &step), (INSERT, &insert_attack(burner))]);
        let move_step = queued_orders(unit, &[(MOVE, &step)]);
        self.check(
            spring::get_unit_rules_param_f32(burner, unit::UNTARGETABLE) == Ok(1.0)
                && attack_other == [ATTACK]
                && attack_burner.is_empty(),
            "an attack order on the coal burner is dropped, one on an Eskimo is kept",
        );
        self.check(
            insert_other == [MOVE, ATTACK] && insert_burner == [MOVE],
            "an inserted attack order on the coal burner is dropped",
        );
        self.check(
            move_step == [MOVE],
            "a move order next to the scenery is kept",
        );
    }

    pub(super) fn check_first_needs_and_trades(&mut self, global: &Global) {
        let intro_forwarded = global.story.access().intro_event
            == "The last remaining good people, bundled on a block of ice.";
        let (alive, hungry) = {
            let village = global.village.access();
            (
                village.eskimos.iter().all(|unit| {
                    spring::get_unit_rules_param_f32(unit.id, unit::HEALTH).is_ok_and(|hp| hp > 0.0)
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
    pub(super) fn hit_first_eskimo(&mut self, global: &Global) {
        let eskimos = eskimo_ids(global);
        let (Some(&target), Some(&attacker)) = (eskimos.first(), eskimos.get(1)) else {
            return;
        };
        let (Ok(pool), Ok(engine)) = (
            spring::get_unit_rules_param_f32(target, unit::HEALTH),
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

    pub(super) fn check_combat(&mut self) {
        let eskimo_damage = defs::def_id(UnitKind::Eskimo).map_or(0.0, defs::damage);
        let combat = self.combat;
        self.check(
            eskimo_damage > 0.0
                && combat.is_some_and(|(id, pool_before, engine_before)| {
                    spring::get_unit_rules_param_f32(id, unit::HEALTH)
                        .is_ok_and(|pool| (pool - (pool_before - eskimo_damage)).abs() < 0.01)
                        && spring::get_unit_health(id)
                            .is_ok_and(|engine| engine.health == engine_before)
                }),
            "an engine hit takes the attacker's damage off the health pool and none off the unit",
        );
    }

    /// Put the last Eskimo in the sea; `unit_drown_eskimos.lua` takes a
    /// point of health on every frame there.
    pub(super) fn drown_last_eskimo(&mut self, global: &Global) {
        let Some(&id) = eskimo_ids(global).last() else {
            return;
        };
        let (Some((x, z)), Ok(pool)) = (
            water_location(map_size(), ground),
            spring::get_unit_rules_param_f32(id, unit::HEALTH),
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

    pub(super) fn check_drowning(&mut self) {
        let frames = (DROWN_CHECK - DROWN_START) as f32;
        let drowning = self.drowning;
        self.check(
            drowning.is_some_and(|(id, before)| {
                spring::get_unit_rules_param_f32(id, unit::HEALTH)
                    .is_ok_and(|pool| before - pool >= frames - 1.0)
            }),
            "an Eskimo in the sea loses a point of health per frame",
        );
        let floundering = drowning.and_then(|(id, _)| {
            crate::with_cus_module(|host| {
                host.with_state(spring::UnitId(id), |script| script.is_drowning())
            })
            .flatten()
        });
        self.check(
            floundering == Some(true),
            "an Eskimo in the sea runs its script's StartDrowning",
        );
    }

    /// Put the third Eskimo next to the coal burner; `unit_eskimos.lua`
    /// warms it there with the village's heat.
    pub(super) fn warm_third_eskimo(&mut self, global: &Global) {
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

    pub(super) fn check_warming(&mut self, global: &Global) {
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

    pub(super) fn check_story_end(&mut self, global: &Global) {
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
    pub(super) fn check_drill_orders(&mut self, global: &Global) {
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
                        command.cmd_id == MOVE
                            && command.params.len() >= 3
                            && (command.params[0] - x).abs() < 1.0
                            && (command.params[2] - z).abs() < 1.0
                    })
                })
            }),
            "drillships are ordered to move to the ice they found",
        );
    }
}

/// Drive the first Eskimo into the EATING state across multiple frames.
pub(super) fn make_first_eskimo_hungry(global: &Global) {
    if let Some(eskimo) = global.village.access_mut().eskimos.first_mut() {
        eskimo.food = 19.0;
        eskimo.eating = false;
    }
    global.economy.access_mut().food = 100.0;
}

/// Give `unit` these orders through the engine, then the commands in its
/// queue, and clear the queue.
fn queued_orders(unit: i32, orders: &[(i32, &[f32])]) -> Vec<i32> {
    for &(command, params) in orders {
        let options = if command == INSERT { ALT_KEY } else { 0 };
        if let Err(error) = spring::give_order_to_unit(unit, command, params, options, 0) {
            spring::log_warning!("mycube-test", "ordering unit {unit} failed: {error:?}");
        }
    }
    let queue = spring::units_commands::get_unit_commands(unit, 4)
        .map(|queue| queue.iter().map(|command| command.cmd_id).collect())
        .unwrap_or_default();
    if let Err(error) = spring::give_order_to_unit(unit, spring::cmd::STOP, &[], 0, 0) {
        spring::log_warning!("mycube-test", "stopping unit {unit} failed: {error:?}");
    }
    queue
}
