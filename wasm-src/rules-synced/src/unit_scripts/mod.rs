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
use alloc::vec::Vec;
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
use self::ship::ShipScript;
use crate::defs::{self, UnitKind};

const RAW_START_DROWNING: i32 = 1;

/// Seeds each script's animation randomness from its unit id.
const RANDOM_SEED: u32 = 0x4d59_4355;

fn capabilities(kind: UnitKind) -> ScriptCapabilities {
    let mut bits = ScriptCapabilities::KILLED | ScriptCapabilities::CREATE;
    match kind {
        UnitKind::Coalburner | UnitKind::Drillship | UnitKind::Iglu => {
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
        UnitKind::Lantern | UnitKind::Parachute => {}
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
    /// A unit without a script (parachutes, unknown defs).
    None,
}

impl MyCubeUnitScript {
    fn for_kind(kind: Option<UnitKind>, unit: UnitId, map: &common::PieceMap) -> Self {
        let random = unit.0 as u32 ^ RANDOM_SEED;
        match kind {
            Some(UnitKind::Coalburner) => Self::Coalburner(CoalburnerScript::new(map)),
            Some(UnitKind::Drillship) => Self::Drillship(ShipScript::new(map, random)),
            Some(UnitKind::Eskimo) => Self::Human(HumanScript::new(unit, map, false, random)),
            Some(UnitKind::Pirate) => Self::Human(HumanScript::new(unit, map, true, random)),
            Some(UnitKind::Iglu) => Self::Iglu(IgluScript::new(map, random)),
            Some(UnitKind::Lantern) => Self::Lantern,
            Some(UnitKind::Parachute) | None => Self::None,
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
        Self::for_kind(kind, unit, &map)
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
            Self::Coalburner(_) | Self::Drillship(_) | Self::Lantern => WreckLevel::One,
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

thread_local! {
    static PENDING_UNITS: RefCell<Vec<(i32, i32)>> = const { RefCell::new(Vec::new()) };
    static PENDING_DROWNING: RefCell<Vec<i32>> = const { RefCell::new(Vec::new()) };
}

pub fn queue_unit_created(unit: i32, def: i32) {
    if unit >= 0 {
        PENDING_UNITS.with(|pending| pending.borrow_mut().push((unit, def)));
    }
}

pub fn request_drowning(unit: i32) {
    if unit < 0 {
        return;
    }
    PENDING_DROWNING.with(|pending| {
        let mut pending = pending.borrow_mut();
        if !pending.contains(&unit) {
            pending.push(unit);
        }
    });
}

/// Keep a unit queued for attach only while it is still alive; units that
/// died before their script attached would otherwise be retried forever.
fn retry_if_alive(unit: i32, def: i32) {
    let alive = spring::valid_unit_id(unit).unwrap_or(false)
        && !spring::get_unit_is_dead(unit).unwrap_or(true);
    if alive {
        queue_unit_created(unit, def);
    }
}

#[derive(Default)]
pub struct MyCubeCusModule {
    registry: CusRegistry<MyCubeUnitScript>,
    handles: BTreeMap<u32, CusHandle>,
}

impl MyCubeCusModule {
    fn attach_pending(&mut self) {
        let pending = PENDING_UNITS.with(|units| core::mem::take(&mut *units.borrow_mut()));
        for (unit_id, def) in pending {
            let Some(kind) = defs::kind(def).filter(|kind| *kind != UnitKind::Parachute) else {
                continue;
            };
            let unit = UnitId(unit_id);
            let instance = instance_id(unit);
            if self.handles.contains_key(&instance) {
                continue;
            }
            let Ok(pieces) = spring::get_unit_piece_map(unit_id) else {
                retry_if_alive(unit_id, def);
                continue;
            };
            let script = MyCubeUnitScript::for_kind(Some(kind), unit, &pieces);
            let host = WasmCus::new(unit, instance);
            let engine = Rc::new(RefCell::new(host.engine()));
            let handle = self
                .registry
                .attach(CusInstance::attach(unit, script, engine));
            // The engine invokes Create synchronously from WasmCus::attach.
            // Register the instance first so that callin can find its handle.
            self.handles.insert(instance, handle);
            if WasmCus::attach(unit, instance, capabilities(kind)).is_ok() {
                // A unit with activateWhenBuilt can finish before the queued
                // native backend is attached. Lua receives that callin
                // synchronously; replay it for Rust when the unit is already
                // active by the time the backend is ready.
                self.replay_active_callin(unit, handle);
            } else {
                self.handles.remove(&instance);
                let _ = self.registry.detach(handle);
                retry_if_alive(unit_id, def);
            }
        }
    }

    fn dispatch_pending_drowning(&mut self) {
        let pending = PENDING_DROWNING.with(|units| core::mem::take(&mut *units.borrow_mut()));
        for unit_id in pending {
            let instance = instance_id(UnitId(unit_id));
            // Drowning is requested again on every frame the unit is in
            // water, so a unit without a script yet (or any more) can be
            // dropped.
            let Some(handle) = self.handles.get(&instance).copied() else {
                continue;
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
    }

    fn current_game_frame() -> u64 {
        spring::game::frame().map_or(0, u64::from)
    }

    fn sync_instance_frame(&mut self, handle: CusHandle) {
        let frame = Self::current_game_frame();
        let _ = self.registry.with(handle, |instance| instance.tick(frame));
    }

    fn replay_active_callin(&mut self, unit: UnitId, handle: CusHandle) {
        if !spring::units_info::get_unit_is_active(unit.0).unwrap_or(false) {
            return;
        }
        let mut result = spring::cus::UnitScriptCallResult::default();
        let _ = self.registry.with(handle, |instance| {
            instance.invoke(UnitScriptCall::Activate, &[], &[], &mut result)
        });
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
        let Some(handle) = self.handles.get(&instance_id).copied() else {
            return Some(0);
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
        Some(0)
    }

    fn cus_tick(&mut self, frame: u32) {
        self.attach_pending();
        self.dispatch_pending_drowning();
        self.registry.tick(frame as u64);
    }

    fn cus_detach(&mut self, instance_id: u32) {
        if let Some(handle) = self.handles.remove(&instance_id) {
            let _ = self.registry.detach(handle);
        }
    }
}
