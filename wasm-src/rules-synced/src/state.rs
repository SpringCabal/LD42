use alloc::collections::VecDeque;
use alloc::string::String;
use alloc::vec::Vec;

use spring::Float3;
use spring::rules_params::los::PUBLIC;
use spring_addons::Resource;

pub const FRAMES_PER_SECOND: i32 = 33;

#[derive(Clone, Copy)]
pub struct Eskimo {
    pub id: i32,
    pub health: f32,
    pub food: f32,
    pub heat: f32,
    // unit_eskimos.lua carries these states between frames. They are not
    // cosmetic: eating continues until food is safely replenished and warmth
    // is retained at the resource-cap threshold.
    pub eating: bool,
    pub warm: bool,
    // Water starts the authored flounder/death sequence.
    pub drowning: bool,
}

/// Runtime state matching the Lua drill gadget's per-ship table. The search
/// cursor is persisted because Core-Wasm height queries are metered, while
/// the original Lua gadget can scan the complete map in one call.
#[derive(Clone, Copy, Default)]
pub struct DrillShip {
    pub id: i32,
    pub has_order: bool,
    pub order_x: f32,
    pub order_z: f32,
    pub checked_frame: i32,
    pub search_initialized: bool,
    pub search_center_x: i32,
    pub search_center_z: i32,
    pub search_radius: i32,
    pub search_side: u8,
    pub search_offset: i32,
    pub search_complete: bool,
}

impl DrillShip {
    pub fn new(id: i32) -> Self {
        Self {
            id,
            checked_frame: -1,
            ..Self::default()
        }
    }
}

#[derive(Clone, Copy)]
pub struct SpawnRequest {
    pub name: &'static str,
    pub position: Float3,
    pub team: i32,
}

pub struct TerrainJob {
    pub start_x: f32,
    pub start_z: f32,
    pub size: i32,
    pub amount: f32,
    pub next_cell: usize,
}

/// Village resources.
#[derive(Clone, Copy, Debug)]
pub struct Economy {
    pub food: f32,
    pub heat: f32,
    pub money: f32,
}

impl Default for Economy {
    fn default() -> Self {
        Self {
            food: 0.0,
            heat: 0.0,
            money: 0.0,
        }
    }
}

/// Everything the gadget tracks by unit id, plus the round-robin cursors that
/// walk those lists across frames.
#[derive(Default)]
pub struct Units {
    pub eskimos: Vec<Eskimo>,
    pub burners: Vec<i32>,
    pub drills: Vec<DrillShip>,
    pub parachutes: Vec<i32>,
    pub pirates: Vec<i32>,
    pub eskimo_sync_cursor: usize,
}

/// Scripted-story progress.
#[derive(Default)]
pub struct Story {
    pub initialized: bool,
    pub step: usize,
    pub next_step: i32,
    pub intro_event: String,
    pub story_eskimos: usize,
    pub story_drills: usize,
    pub story_parachutes: usize,
    pub story_pirates: usize,
    pub spawn_rng: u32,
    pub victory: bool,
    pub game_end: String,
    pub end_frame: i32,
}

/// In-flight terrain deformation jobs.
#[derive(Default)]
pub struct Terrain {
    pub jobs: VecDeque<TerrainJob>,
}

/// Work deferred out of a nested callin and drained by the reaper gadget.
#[derive(Default)]
pub struct Pending {
    pub spawns: Vec<SpawnRequest>,
    pub destroys: Vec<DestroyRequest>,
}

#[derive(Clone, Copy)]
pub struct DestroyRequest {
    pub id: i32,
    pub reclaimed: bool,
}

pub fn f32_param(name: &str, value: f32) {
    let _ = spring::set_game_rules_param(
        name,
        &spring::owned::rules_params::RulesParamValue {
            type_: spring::owned::rules_params::RulesParamType::RulesparamTypeFloat,
            bool_value: false,
            float_value: value,
            string_value: String::new(),
        },
        PUBLIC,
    );
}
pub fn text_param(name: &str, value: &str) {
    let _ = spring::set_game_rules_param(
        name,
        &spring::owned::rules_params::RulesParamValue {
            type_: spring::owned::rules_params::RulesParamType::RulesparamTypeString,
            bool_value: false,
            float_value: 0.0,
            string_value: String::from(value),
        },
        PUBLIC,
    );
}
pub fn initialize(global: &Global) {
    {
        let mut story = global.story.access_mut();
        if story.initialized {
            return;
        }
        story.initialized = true;
        story.intro_event.clear();
        story.end_frame = -1;
        // storyteller.lua's first GameFrame is frame 0 in the normal game
        // mode (sb_gameMode is unset), so the opening units are created on
        // frame 0 rather than one simulation frame later.
        story.next_step = 0;
        story.spawn_rng = 0x4d59_4355;
    }
    // storyteller.lua clears map-authored units before starting its scenario.
    // Queue the same reclaimed destruction so stale buildings or units cannot
    // retain their map teams and appear alongside the authored expedition.
    if let Ok(units) = spring::get_all_units() {
        global
            .pending
            .access_mut()
            .destroys
            .extend(units.into_iter().map(|id| DestroyRequest {
                id,
                reclaimed: true,
            }));
    }
    let economy = {
        let mut economy = global.economy.access_mut();
        economy.food = 80.0;
        economy.heat = 80.0;
        economy.money = 80.0;
        *economy
    };
    f32_param("maxFood", 100.0);
    f32_param("maxHeat", 100.0);
    f32_param("maxMoney", 100.0);
    publish_resources(&economy);
    f32_param("uiMode", -1.0);
    text_param("gameEnd", "");
    text_param("introEvent", "");
}

pub fn story_game_end(story: &mut Story, value: &str, frame: i32) {
    // A loss takes precedence if the final player unit disappears on the
    // same frame that the scripted story reaches its outro.
    if story.game_end == "loss" || story.game_end == value {
        return;
    }
    story.game_end = String::from(value);
    if value == "loss" {
        story.victory = false;
    }
    story.end_frame = frame;
    text_param("gameEnd", value);
}
pub fn publish_resources(economy: &Economy) {
    f32_param("food", economy.food);
    f32_param("heat", economy.heat);
    f32_param("money", economy.money);
}
pub fn consume(economy: &mut Economy, resource: &str, amount: f32) -> bool {
    let value = match resource {
        "food" => &mut economy.food,
        "heat" => &mut economy.heat,
        "money" => &mut economy.money,
        _ => return false,
    };
    if *value < amount {
        return false;
    }
    *value -= amount;
    publish_resources(economy);
    true
}
pub fn add(economy: &mut Economy, resource: &str, amount: f32) {
    let value = match resource {
        "food" => &mut economy.food,
        "heat" => &mut economy.heat,
        "money" => &mut economy.money,
        _ => return,
    };
    *value = (*value + amount).min(100.0);
    publish_resources(economy);
}
pub fn spawn(name: &str, position: Float3, team: i32) -> Option<i32> {
    let id = spring::get_unit_def_id_by_name(name).ok()?;
    let is_scenery = matches!(name, "coalburner" | "iglu" | "lantern");
    let create_team = if is_scenery {
        spring::game::get_gaia_team_id(0).unwrap_or(team)
    } else {
        team
    };
    let unit_id = spring::create_unit(
        spring::UnitDefRef { name, id },
        position,
        create_team,
        spring::CreateUnitOptions {
            build: false,
            flatten_ground: false,
            unit_id: -1,
            builder_id: -1,
        },
    )
    .ok()?;
    if is_scenery {
        let _ = spring::set_unit_neutral(unit_id, true);
        let _ = spring::set_unit_blocking(
            unit_id,
            spring::types::SetUnitBlockingOptions {
                blocking: true,
                solid_objects: true,
                projectiles: false,
                quad_map_rays: false,
                crushable: false,
                block_enemy_pushing: true,
                block_height_changes: false,
            },
        );
    }
    Some(unit_id)
}

/// The shared communication namespace.
///
/// Each subsystem is an independently borrowable [`Resource`], so a nested
/// callin only conflicts when it touches the same subsystem.
#[derive(Default)]
pub struct Global {
    pub economy: Resource<Economy>,
    pub units: Resource<Units>,
    pub story: Resource<Story>,
    pub terrain: Resource<Terrain>,
    pub pending: Resource<Pending>,
}
