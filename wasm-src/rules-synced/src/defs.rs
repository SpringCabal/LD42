use alloc::vec::Vec;
use springcabal::defs::DefCache;

use mycube_shared::unit;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UnitKind {
    Coalburner,
    Drillship,
    Eskimo,
    Iglu,
    Iglu2,
    Lantern,
    Parachute,
    Pirate,
}

impl UnitKind {
    pub const fn is_scenery(self) -> bool {
        matches!(self, Self::Coalburner | Self::Iglu | Self::Lantern)
    }

    pub const fn has_script(self) -> bool {
        !matches!(self, Self::Parachute | Self::Iglu2)
    }

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
    health: f32,
    damage: f32,
}

struct Defs {
    entries: Vec<DefInfo>,
}

impl Defs {
    fn resolve(spring: &spring::Spring<'_, spring::RulesSynced>) -> Self {
        let entries = DEF_NAMES
            .iter()
            .filter_map(|&(name, kind)| {
                let id = spring.unit_defs().get_unit_def_id_by_name(name).ok()?;
                Some(DefInfo {
                    id,
                    kind,
                    health: unit::max_health(custom_param(spring, id, "health").as_deref()),
                    damage: unit::custom_param_number(
                        custom_param(spring, id, "damage").as_deref(),
                    )
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

fn custom_param(
    spring: &spring::Spring<'_, spring::RulesSynced>,
    def: i32,
    key: &str,
) -> Option<alloc::string::String> {
    spring.unit_defs().get_unit_def_custom_param(def, key).ok()
}

static DEFS: DefCache<Defs> = DefCache::new();

fn with_defs<R>(spring: &spring::Spring<'_, spring::RulesSynced>, f: impl FnOnce(&Defs) -> R) -> R {
    f(DEFS.get_or_init(|| Defs::resolve(spring)))
}

pub fn kind(spring: &spring::Spring<'_, spring::RulesSynced>, def: i32) -> Option<UnitKind> {
    with_defs(spring, |defs| defs.info(def).map(|info| info.kind))
}

pub fn def_id(spring: &spring::Spring<'_, spring::RulesSynced>, kind: UnitKind) -> Option<i32> {
    with_defs(spring, |defs| {
        defs.entries
            .iter()
            .find(|entry| entry.kind == kind)
            .map(|entry| entry.id)
    })
}

pub fn health(spring: &spring::Spring<'_, spring::RulesSynced>, def: i32) -> f32 {
    with_defs(spring, |defs| {
        defs.info(def)
            .map_or(unit::DEFAULT_HEALTH, |info| info.health)
    })
}

pub fn damage(spring: &spring::Spring<'_, spring::RulesSynced>, def: i32) -> f32 {
    with_defs(spring, |defs| {
        defs.info(def).map_or(0.0, |info| info.damage)
    })
}
