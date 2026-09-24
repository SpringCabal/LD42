//! My Cube's unit scripts, implemented with the portable CUS API.
//!
//! The gameplay gadgets remain the authority for health, drowning, drilling
//! and spawning; the scripts here animate the units and answer the
//! engine's unit-script callins. The engine's `CusHost` attaches a script to
//! each unit and forwards the engine's calls to it.

mod common;
mod eskimo;
mod human;
mod scenery;
mod ship;

use alloc::rc::Rc;

use spring::UnitId;
use spring::cus::host::{CusHost, UnitDefScript};
use spring::cus::{InitCtx, Piece, ScriptCapabilities, UnitCtx, UnitScript, WeaponId, WreckLevel};

use self::common::set_scenery_state;
use self::human::HumanScript;
use self::scenery::{CoalburnerScript, IgluScript};
use self::ship::{ShipScript, SinkingSoundLock};
use crate::defs::{self, UnitKind};

/// The module's unit scripts. Rules code reaches them through
/// `crate::with_cus_module`. The drill ships share one [`SinkingSoundLock`].
pub type MyCubeCusModule = CusHost<MyCubeUnitScript, SinkingSoundLock>;

fn capabilities(kind: UnitKind) -> ScriptCapabilities {
    let mut bits = ScriptCapabilities::KILLED | ScriptCapabilities::CREATE;
    match kind {
        UnitKind::Coalburner | UnitKind::Drillship => {
            bits |= ScriptCapabilities::ACTIVATE | ScriptCapabilities::DEACTIVATE;
        }
        UnitKind::Eskimo | UnitKind::Pirate => {
            bits |= ScriptCapabilities::START_MOVING
                | ScriptCapabilities::STOP_MOVING
                | ScriptCapabilities::QUERY_WEAPON
                | ScriptCapabilities::AIM_WEAPON
                | ScriptCapabilities::AIM_FROM_WEAPON
                | ScriptCapabilities::SHOT;
        }
        UnitKind::Iglu | UnitKind::Iglu2 | UnitKind::Lantern | UnitKind::Parachute => {}
    }
    ScriptCapabilities::new(bits)
}

pub enum MyCubeUnitScript {
    Coalburner(CoalburnerScript),
    Drillship(ShipScript),
    Human(HumanScript),
    Iglu(IgluScript),
    Lantern,
    /// A unit without a script.
    None,
}

impl MyCubeUnitScript {
    fn for_kind(
        kind: Option<UnitKind>,
        unit: UnitId,
        map: &common::PieceMap,
        sinking_sound: &SinkingSoundLock,
    ) -> Self {
        match kind {
            Some(UnitKind::Coalburner) => Self::Coalburner(CoalburnerScript::new(map)),
            Some(UnitKind::Drillship) => {
                Self::Drillship(ShipScript::new(map, Rc::clone(sinking_sound)))
            }
            Some(UnitKind::Eskimo) => Self::Human(HumanScript::new(unit, map, false)),
            Some(UnitKind::Pirate) => Self::Human(HumanScript::new(unit, map, true)),
            Some(UnitKind::Iglu) => Self::Iglu(IgluScript::new(map)),
            Some(UnitKind::Lantern) => Self::Lantern,
            Some(UnitKind::Iglu2 | UnitKind::Parachute) | None => Self::None,
        }
    }
}

impl MyCubeUnitScript {
    /// `env.StartDrowning()`, which the drowning rules call on Eskimos and
    /// pirates in the water. Other scripts have no such function.
    pub fn start_drowning(&mut self, ctx: &UnitCtx) {
        if let Self::Human(script) = self {
            script.start_drowning(ctx);
        }
    }

    /// Whether this is an Eskimo's or pirate's script that is drowning.
    #[cfg(feature = "headless-integration")]
    pub fn is_drowning(&self) -> bool {
        matches!(self, Self::Human(script) if script.is_drowning())
    }
}

impl UnitDefScript<SinkingSoundLock> for MyCubeUnitScript {
    fn capabilities(def: i32) -> Option<ScriptCapabilities> {
        defs::kind(def)
            .filter(|kind| kind.has_script())
            .map(capabilities)
    }

    fn for_def(def: i32, ctx: &mut InitCtx<'_>, sinking_sound: &mut SinkingSoundLock) -> Self {
        Self::for_kind(defs::kind(def), ctx.unit(), &ctx.pieces(), sinking_sound)
    }
}

impl UnitScript for MyCubeUnitScript {
    fn create(&mut self, ctx: &UnitCtx) {
        match self {
            Self::Coalburner(script) => {
                set_scenery_state(ctx.unit());
                script.create(ctx);
            }
            Self::Iglu(script) => {
                set_scenery_state(ctx.unit());
                script.create(ctx);
            }
            Self::Lantern => set_scenery_state(ctx.unit()),
            Self::Drillship(script) => script.create(ctx),
            Self::Human(script) => script.create(ctx),
            Self::None => {}
        }
    }

    fn start_moving(&mut self, ctx: &UnitCtx, _reversing: bool) {
        if let Self::Human(script) = self {
            script.start_moving(ctx);
        }
    }

    fn stop_moving(&mut self, ctx: &UnitCtx) {
        if let Self::Human(script) = self {
            script.stop_moving(ctx);
        }
    }

    fn activate(&mut self, ctx: &UnitCtx) {
        match self {
            Self::Coalburner(script) => script.set_heat(ctx, 1.0),
            Self::Drillship(script) => script.set_active(ctx, true),
            _ => {}
        }
    }

    fn deactivate(&mut self, ctx: &UnitCtx) {
        match self {
            Self::Coalburner(script) => script.set_heat(ctx, 0.0),
            Self::Drillship(script) => script.set_active(ctx, false),
            _ => {}
        }
    }

    fn query_weapon(&mut self, _ctx: &UnitCtx, _weapon: WeaponId) -> Piece {
        match self {
            Self::Human(script) => script.query_weapon(),
            _ => Piece::INVALID,
        }
    }

    fn aim_from_weapon(&mut self, ctx: &UnitCtx, weapon: WeaponId) -> Piece {
        self.query_weapon(ctx, weapon)
    }

    fn aim_weapon(
        &mut self,
        ctx: &UnitCtx,
        weapon: WeaponId,
        _heading: f32,
        _pitch: f32,
    ) -> Option<bool> {
        match self {
            Self::Human(script) => script.aim_weapon(ctx, weapon),
            _ => Some(false),
        }
    }

    fn shot(&mut self, ctx: &UnitCtx, _weapon: WeaponId) {
        if let Self::Human(script) = self {
            script.shot(ctx);
        }
    }

    fn killed(&mut self, ctx: &UnitCtx, _recent_damage: f32, _max_health: f32) -> WreckLevel {
        match self {
            Self::Human(script) => script.killed(ctx),
            Self::Iglu(script) => {
                script.killed(ctx);
                WreckLevel::One
            }
            Self::Drillship(script) => {
                script.killed(ctx);
                WreckLevel::One
            }
            Self::Coalburner(_) | Self::Lantern => WreckLevel::One,
            Self::None => WreckLevel::None,
        }
    }

    fn killed_pending(&self) -> bool {
        match self {
            Self::Human(script) => script.killed_pending(),
            _ => false,
        }
    }
}
