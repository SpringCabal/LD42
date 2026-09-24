//! My Cube's unit scripts, implemented with the portable CUS API.
//!
//! The gameplay gadgets remain the authority for health, drowning, drilling
//! and spawning; the scripts here animate the units and answer the
//! engine's unit-script callins. `MyCubeCusModule` attaches a script to each
//! unit and forwards the engine's calls to it.

mod common;
mod eskimo;
mod human;
mod scenery;
mod ship;

use alloc::collections::BTreeMap;
use alloc::rc::Rc;
use core::cell::RefCell;

use spring::UnitId;
use spring::cus::core_module::{CoreCusCallResult, CoreCusModule};
use spring::cus::wasm::WasmCus;
use spring::cus::{
    CusHandle, CusInstance, CusRegistry, InitCtx, Piece, ScriptCapabilities, UnitCtx, UnitScript,
    UnitScriptCall, WeaponId, WreckLevel,
};

use self::common::set_scenery_state;
use self::human::HumanScript;
use self::scenery::{CoalburnerScript, IgluScript};
use self::ship::{ShipScript, SinkingSoundLock};
use crate::defs::{self, UnitKind};

const RAW_START_DROWNING: i32 = 1;

fn capabilities(kind: UnitKind) -> ScriptCapabilities {
    let mut bits = ScriptCapabilities::KILLED | ScriptCapabilities::CREATE;
    match kind {
        UnitKind::Coalburner | UnitKind::Drillship => {
            bits |= ScriptCapabilities::ACTIVATE | ScriptCapabilities::DEACTIVATE;
        }
        UnitKind::Eskimo | UnitKind::Pirate => {
            bits |= ScriptCapabilities::RAW_CALL
                | ScriptCapabilities::START_MOVING
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

fn instance_id(unit: UnitId) -> u32 {
    0x4d43_0000u32.wrapping_add(unit.0 as u32)
}

enum MyCubeUnitScript {
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

impl UnitScript for MyCubeUnitScript {
    fn new(ctx: &mut InitCtx<'_>) -> Self {
        let unit = ctx.unit();
        let kind = spring::get_unit_def_id(unit.0)
            .ok()
            .and_then(|def| defs::kind(def.0));
        let map = spring::get_unit_piece_map(unit.0).unwrap_or_default();
        Self::for_kind(kind, unit, &map, &SinkingSoundLock::default())
    }

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
        // Create runs once the rules callin that created the unit has
        // returned. An activateWhenBuilt unit was activated before that,
        // while the module was busy, so the engine could not deliver its
        // Activate; deliver it now. Should the engine queue that Activate
        // instead (engine TODO §2), the scripts ignore the second one.
        if spring::units_info::get_unit_is_active(ctx.unit().0).unwrap_or(false) {
            self.activate(ctx);
        }
    }

    fn raw_call(&mut self, ctx: &UnitCtx, function: i32) {
        if let Self::Human(script) = self
            && function == RAW_START_DROWNING
        {
            script.start_drowning(ctx);
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
            Self::Coalburner(script) => script.set_lit(ctx, true),
            Self::Drillship(script) => script.set_active(ctx, true),
            _ => {}
        }
    }

    fn deactivate(&mut self, ctx: &UnitCtx) {
        match self {
            Self::Coalburner(script) => script.set_lit(ctx, false),
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

const LOG_SECTION: &str = "mycube-cus";

#[derive(Default)]
pub struct MyCubeCusModule {
    registry: CusRegistry<MyCubeUnitScript>,
    handles: BTreeMap<u32, CusHandle>,
    sinking_sound: SinkingSoundLock,
}

impl MyCubeCusModule {
    /// Attach the script for a new unit. Called from UnitCreated through
    /// `with_cus_module`; the engine runs the script's Create once that
    /// callin has returned.
    pub fn attach(&mut self, unit_id: i32, def: i32) {
        let Some(kind) = defs::kind(def).filter(|kind| kind.has_script()) else {
            return;
        };
        let unit = UnitId(unit_id);
        let instance = instance_id(unit);
        if self.handles.contains_key(&instance) {
            return;
        }
        let pieces = match spring::get_unit_piece_map(unit_id) {
            Ok(pieces) => pieces,
            Err(error) => {
                spring::log_warning!(
                    LOG_SECTION,
                    "no piece map for unit {unit_id} ({kind:?}), no script attached: {error:?}"
                );
                return;
            }
        };
        let script = MyCubeUnitScript::for_kind(Some(kind), unit, &pieces, &self.sinking_sound);
        let host = WasmCus::new(unit, instance);
        let engine = Rc::new(RefCell::new(host.engine()));
        let handle = self
            .registry
            .attach(CusInstance::attach(unit, script, engine));
        self.handles.insert(instance, handle);
        if let Err(error) = WasmCus::attach(unit, instance, capabilities(kind)) {
            spring::log_warning!(
                LOG_SECTION,
                "attaching the script of unit {unit_id} ({kind:?}) failed: {error:?}"
            );
            self.handles.remove(&instance);
            let _ = self.registry.detach(handle);
        }
    }

    /// `StartDrowning`, called by the drowning gadget on every frame the
    /// unit is in water. Units without a script are skipped.
    pub fn start_drowning(&mut self, unit_id: i32) {
        self.start_drowning_instance(instance_id(UnitId(unit_id)));
    }

    fn start_drowning_instance(&mut self, instance: u32) {
        let Some(handle) = self.handles.get(&instance).copied() else {
            return;
        };
        self.sync_instance_frame(handle);
        let mut result = spring::cus::UnitScriptCallResult::default();
        let _ = self.registry.with(handle, |instance| {
            instance.invoke(
                UnitScriptCall::RawCall,
                &[],
                &[RAW_START_DROWNING],
                &mut result,
            )
        });
    }

    fn sync_instance_frame(&mut self, handle: CusHandle) {
        let frame = spring::game::frame().map_or(0, u64::from);
        let _ = self.registry.with(handle, |instance| instance.tick(frame));
    }
}

impl CoreCusModule for MyCubeCusModule {
    fn cus_invoke(
        &mut self,
        instance_id: u32,
        call: u32,
        float_arguments: &[f32],
        integer_arguments: &[i32],
        result: &mut CoreCusCallResult<'_>,
    ) -> bool {
        let Some(handle) = self.handles.get(&instance_id).copied() else {
            return false;
        };
        let Some(call) = UnitScriptCall::from_u32(call) else {
            return false;
        };
        if matches!(
            call,
            UnitScriptCall::RawCall | UnitScriptCall::StartMoving | UnitScriptCall::StopMoving
        ) {
            self.sync_instance_frame(handle);
        }
        let mut call_result = spring::cus::UnitScriptCallResult::default();
        let handled = self
            .registry
            .with(handle, |instance| {
                instance.invoke(call, float_arguments, integer_arguments, &mut call_result)
            })
            .unwrap_or(false);
        if !handled {
            return false;
        }
        result.int_value = call_result.int_value;
        result.float_value = call_result.float_value;
        result.bool_value = call_result.bool_value;
        result.complete = call_result.complete;
        result.int_count = call_result.int_values.len().min(result.int_values.len());
        result.int_values[..result.int_count]
            .copy_from_slice(&call_result.int_values[..result.int_count]);
        true
    }

    fn cus_call_named(
        &mut self,
        instance_id: u32,
        function_name: &str,
        _arguments: &[f32],
        _return_values: &mut [f32],
        found: &mut bool,
    ) -> Option<usize> {
        if function_name != "StartDrowning" {
            return None;
        }
        *found = true;
        self.start_drowning_instance(instance_id);
        Some(0)
    }

    fn cus_tick(&mut self, frame: u32) {
        self.registry.tick(frame as u64);
    }

    fn cus_detach(&mut self, instance_id: u32) {
        if let Some(handle) = self.handles.remove(&instance_id) {
            let _ = self.registry.detach(handle);
        }
    }
}
