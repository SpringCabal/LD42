//! Unit defs, resolved by name once and then looked up by def id.

use alloc::vec::Vec;
use springcabal::defs::DefCache;

use mycube_shared::unit;

/// The unit types the rules and unit scripts care about.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UnitKind {
    Coalburner,
    Drillship,
    Eskimo,
    Iglu,
    /// The second iglu model. Its Lua script, `igluscript2.lua`, is empty.
    Iglu2,
    Lantern,
    Parachute,
    Pirate,
}

impl UnitKind {
    /// Scenery is created for Gaia, neutral and untargetable.
    pub const fn is_scenery(self) -> bool {
        matches!(self, Self::Coalburner | Self::Iglu | Self::Lantern)
    }

    /// Parachutes and the second iglu have no unit script (`parachut.lua`
    /// does not exist, `igluscript2.lua` is empty).
    pub const fn has_script(self) -> bool {
        !matches!(self, Self::Parachute | Self::Iglu2)
    }

    /// The def the story spawns for this kind.
    pub const fn def_name(self) -> &'static str {
        match self {
            Self::Coalburner => "coalburner",
            Self::Drillship => "drillship",
            Self::Eskimo => "eskimo",
            Self::Iglu => "iglu",
            Self::Iglu2 => "iglu2",
            Self::Lantern => "lantern",
            Self::Parachute => "parachut",
            Self::Pirate => "pirate",
        }
    }
}

/// Every def name with its kind.
const DEF_NAMES: [(&str, UnitKind); 8] = [
    ("coalburner", UnitKind::Coalburner),
    ("drillship", UnitKind::Drillship),
    ("eskimo", UnitKind::Eskimo),
    ("iglu", UnitKind::Iglu),
    ("iglu2", UnitKind::Iglu2),
    ("lantern", UnitKind::Lantern),
    ("parachut", UnitKind::Parachute),
    ("pirate", UnitKind::Pirate),
];

#[derive(Clone, Copy)]
struct DefInfo {
    id: i32,
    kind: UnitKind,
    /// `customParams.health`: the gameplay health pool.
    health: f32,
    /// `customParams.damage`: what this unit's attacks take off that pool.
    damage: f32,
}

struct Defs {
    entries: Vec<DefInfo>,
}

impl Defs {
    fn resolve() -> Self {
        let entries = DEF_NAMES
            .iter()
            .filter_map(|&(name, kind)| {
                let id = spring::get_unit_def_id_by_name(name).ok()?;
                Some(DefInfo {
                    id,
                    kind,
                    health: unit::max_health(custom_param(id, "health").as_deref()),
                    damage: unit::custom_param_number(custom_param(id, "damage").as_deref())
                        .unwrap_or(0.0),
                })
            })
            .collect();
        Self { entries }
    }

    fn info(&self, def: i32) -> Option<&DefInfo> {
        self.entries.iter().find(|entry| entry.id == def)
    }
}

fn custom_param(def: i32, key: &str) -> Option<alloc::string::String> {
    spring::unit_defs::get_unit_def_custom_param(def, key).ok()
}

static DEFS: DefCache<Defs> = DefCache::new(Defs::resolve);

fn with_defs<R>(f: impl FnOnce(&Defs) -> R) -> R {
    f(DEFS.get())
}

/// The kind of a def, if it is one of ours.
pub fn kind(def: i32) -> Option<UnitKind> {
    with_defs(|defs| defs.info(def).map(|info| info.kind))
}

/// The def id the story spawns for `kind`.
pub fn def_id(kind: UnitKind) -> Option<i32> {
    with_defs(|defs| {
        defs.entries
            .iter()
            .find(|entry| entry.kind == kind)
            .map(|entry| entry.id)
    })
}

/// The gameplay health a unit of this def starts with (`game_health.lua`).
pub fn health(def: i32) -> f32 {
    with_defs(|defs| {
        defs.info(def)
            .map_or(unit::DEFAULT_HEALTH, |info| info.health)
    })
}

/// The gameplay damage a unit of this def deals per hit.
pub fn damage(def: i32) -> f32 {
    with_defs(|defs| defs.info(def).map_or(0.0, |info| info.damage))
}
