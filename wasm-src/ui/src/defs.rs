//! The unit defs the HUD draws bars for, resolved by name once.

use core::cell::OnceCell;

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
}

thread_local! {
    static DEFS: OnceCell<Defs> = const { OnceCell::new() };
}

fn defs() -> Defs {
    DEFS.with(|defs| {
        *defs.get_or_init(|| {
            let id = |name: &str| spring::get_unit_def_id_by_name(name).ok();
            Defs {
                eskimo: id("eskimo"),
                pirate: id("pirate"),
                drillship: id("drillship"),
            }
        })
    })
}

/// The kind of a def, if the HUD cares about it.
pub fn kind(def: i32) -> Option<UnitKind> {
    let defs = defs();
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

impl UnitKind {
    /// The full gameplay health pool (`customParams.health`).
    pub const fn max_health(self) -> f32 {
        match self {
            Self::Eskimo | Self::Pirate => 100.0,
            Self::Drillship => 500.0,
        }
    }
}
