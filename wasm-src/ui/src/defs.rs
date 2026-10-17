use springcabal::defs::DefCache;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UnitKind {
    Eskimo,
    Pirate,
    Drillship,
}

#[derive(Clone, Copy)]
struct Defs {
    eskimo: Option<i32>,
    pirate: Option<i32>,
    drillship: Option<i32>,
    health: [f32; 3],
    scenery: [Option<i32>; 3],
}

static DEFS: DefCache<Defs> = DefCache::new();

fn defs(spring: &spring::Spring<'_, spring::Ui>) -> Defs {
    *DEFS.get_or_init(|| {
        let id = |name: &str| spring.unit_defs().get_unit_def_id_by_name(name).ok();
        let (eskimo, pirate, drillship) = (id("eskimo"), id("pirate"), id("drillship"));
        Defs {
            eskimo,
            pirate,
            drillship,
            health: [eskimo, pirate, drillship].map(|def| health(spring, def)),
            scenery: [id("coalburner"), id("iglu"), id("lantern")],
        }
    })
}

pub fn kind(spring: &spring::Spring<'_, spring::Ui>, def: i32) -> Option<UnitKind> {
    let defs = defs(spring);
    if defs.eskimo == Some(def) {
        Some(UnitKind::Eskimo)
    } else if defs.pirate == Some(def) {
        Some(UnitKind::Pirate)
    } else if defs.drillship == Some(def) {
        Some(UnitKind::Drillship)
    } else {
        None
    }
}

pub fn is_scenery(spring: &spring::Spring<'_, spring::Ui>, def: i32) -> bool {
    defs(spring).scenery.contains(&Some(def))
}

fn health(spring: &spring::Spring<'_, spring::Ui>, def: Option<i32>) -> f32 {
    let param = def.and_then(|def| {
        spring
            .unit_defs()
            .get_unit_def_custom_param(def, "health")
            .ok()
    });
    mycube_shared::unit::max_health(param.as_deref())
}

impl UnitKind {
    pub fn max_health(self, spring: &spring::Spring<'_, spring::Ui>) -> f32 {
        let index = match self {
            Self::Eskimo => 0,
            Self::Pirate => 1,
            Self::Drillship => 2,
        };
        defs(spring).health[index]
    }
}
