//! My Cube's authored unit scripts, implemented with the portable CUS API.
//!
//! The animation tables in `eskimo_animations.rs` are generated from the
//! original Lua keyframes.  The gameplay gadgets remain the authority for
//! health, drowning, drilling and spawning; this module owns the visual and
//! engine-facing unit-script callbacks that used to be supplied by Lua.

mod common;
mod eskimo;

use alloc::collections::BTreeMap;
use alloc::rc::Rc;
use alloc::vec::Vec;
use core::cell::RefCell;

use spring::UnitId;
use spring::cus::core_module::{CoreCusCallResult, CoreCusModule};
use spring::cus::wasm::WasmCus;
use spring::cus::{
    Angle, AngularSpeed, Axis, CusHandle, CusInstance, CusRegistry, InitCtx, Piece,
    ScriptCapabilities, TaskDefinition, UnitCtx, UnitScript, UnitScriptCall, WeaponId, WreckLevel,
};

use self::common::{
    EnginePieces, SIG_AIM, SIG_IDLE, SIG_SHIP, SIG_WALK, all_named_pieces, explode_flags, hide_all,
    matching_pieces, piece_by_name, piece_position, play_unit_sound, random_unit, reset_all,
    set_scenery_state, show_all, turn,
};
use self::eskimo::{
    HumanPart, HumanPieces, HumanState, apply_idle_frame, idle_task, play_aim, play_death,
    play_drown, play_shoot, play_throw, play_walk,
};

const RAW_START_DROWNING: i32 = 1;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ScriptKind {
    Coalburner,
    Drillship,
    Eskimo,
    Pirate,
    Iglu,
    Lantern,
}

fn script_kind_for_def(def: i32) -> Option<ScriptKind> {
    let matches = |name: &str| spring::get_unit_def_id_by_name(name).ok() == Some(def);
    if matches("coalburner") {
        Some(ScriptKind::Coalburner)
    } else if matches("drillship") {
        Some(ScriptKind::Drillship)
    } else if matches("eskimo") {
        Some(ScriptKind::Eskimo)
    } else if matches("pirate") {
        Some(ScriptKind::Pirate)
    } else if matches("iglu") || matches("iglu2") {
        Some(ScriptKind::Iglu)
    } else if matches("lantern") {
        Some(ScriptKind::Lantern)
    } else {
        None
    }
}

fn capabilities(kind: ScriptKind) -> ScriptCapabilities {
    let mut bits = ScriptCapabilities::KILLED;
    match kind {
        ScriptKind::Coalburner => {
            bits |= ScriptCapabilities::CREATE
                | ScriptCapabilities::ACTIVATE
                | ScriptCapabilities::DEACTIVATE;
        }
        ScriptKind::Drillship => {
            bits |= ScriptCapabilities::CREATE
                | ScriptCapabilities::ACTIVATE
                | ScriptCapabilities::DEACTIVATE;
        }
        ScriptKind::Eskimo | ScriptKind::Pirate => {
            bits |= ScriptCapabilities::RAW_CALL
                | ScriptCapabilities::CREATE
                | ScriptCapabilities::START_MOVING
                | ScriptCapabilities::STOP_MOVING
                | ScriptCapabilities::QUERY_WEAPON
                | ScriptCapabilities::AIM_WEAPON
                | ScriptCapabilities::AIM_FROM_WEAPON
                | ScriptCapabilities::SHOT;
        }
        ScriptKind::Iglu => {
            bits |= ScriptCapabilities::CREATE
                | ScriptCapabilities::ACTIVATE
                | ScriptCapabilities::DEACTIVATE;
        }
        ScriptKind::Lantern => {
            bits |= ScriptCapabilities::CREATE;
        }
    }
    ScriptCapabilities::new(bits)
}

fn instance_id(unit: UnitId) -> u32 {
    0x4d43_0000u32.wrapping_add(unit.0 as u32)
}

#[derive(Clone)]
struct ShipState {
    active: bool,
    minetime: u32,
    random: u32,
    mine: Piece,
}

struct MyCubeUnitScript {
    kind: Option<ScriptKind>,
    all: Vec<Piece>,
    coal_heaters: Vec<Piece>,
    coal_firecycle: Vec<Piece>,
    coal_inards: Piece,
    iglu_build: Vec<Piece>,
    iglu_xy_deco: Vec<Piece>,
    iglu_y_deco: Vec<Piece>,
    ship_body: Vec<Piece>,
    ship_turret: Vec<Piece>,
    ship_drill: Vec<Piece>,
    ship_deco: Vec<Piece>,
    human: Rc<RefCell<HumanState>>,
    ship: Rc<RefCell<ShipState>>,
    random: u32,
}

impl UnitScript for MyCubeUnitScript {
    fn new(ctx: &mut InitCtx<'_>) -> Self {
        let unit = ctx.unit();
        let map = spring::get_unit_piece_map(unit.0).unwrap_or_default();
        let all = all_named_pieces(&map);
        let human_pieces = HumanPieces::from_init(ctx);
        let mut offsets = [spring::Float3::ZERO; 14];
        for part in HumanPart::ALL {
            let name = match part {
                HumanPart::Abs => "Abs",
                HumanPart::ArmLeft => "ArmLeft",
                HumanPart::ArmRight => "ArmRight",
                HumanPart::Gun => "Gun",
                HumanPart::GunMuzzle => "GunMuzzle",
                HumanPart::HandLeft => "HandLeft",
                HumanPart::HandRight => "HandRight",
                HumanPart::Head => "Head",
                HumanPart::KneeLeft => "KneeLeft",
                HumanPart::KneeRight => "KneeRight",
                HumanPart::Spear => "Spear",
                HumanPart::ThighLeft => "ThighLeft",
                HumanPart::ThighRight => "ThighRight",
                HumanPart::Torso => "Torso",
            };
            offsets[part.index()] = piece_position(unit, name);
        }
        let random = unit.0 as u32 ^ 0x4d59_4355;
        Self {
            kind: None,
            all,
            coal_heaters: matching_pieces(&map, "Heater"),
            coal_firecycle: matching_pieces(&map, "firecycle"),
            coal_inards: piece_by_name(unit, "Inards"),
            iglu_build: matching_pieces(&map, "Build"),
            iglu_xy_deco: matching_pieces(&map, "XY"),
            iglu_y_deco: matching_pieces(&map, "Y"),
            ship_body: matching_pieces(&map, "Boatbody"),
            ship_turret: matching_pieces(&map, "Turret"),
            ship_drill: matching_pieces(&map, "icedrill"),
            ship_deco: matching_pieces(&map, "Deco"),
            human: Rc::new(RefCell::new(HumanState {
                pieces: human_pieces,
                offsets,
                moving: false,
                drowning: false,
                has_gun: false,
                aiming: false,
                aim_ready: false,
                throwing: false,
                death_pending: false,
                random,
            })),
            ship: Rc::new(RefCell::new(ShipState {
                active: false,
                minetime: 0,
                random: random ^ 0x9e37_79b9,
                mine: piece_by_name(unit, "icemine"),
            })),
            random,
        }
    }

    fn create(&mut self, ctx: &UnitCtx) {
        match self.kind {
            Some(ScriptKind::Coalburner) => {
                set_scenery_state(ctx.unit());
                hide_all(ctx, &self.all);
                self.set_heat(ctx, 0.0);
            }
            Some(ScriptKind::Iglu) => {
                set_scenery_state(ctx.unit());
                self.build_iglu(ctx);
            }
            Some(ScriptKind::Lantern) => {
                set_scenery_state(ctx.unit());
            }
            Some(ScriptKind::Drillship) => self.create_ship(ctx),
            Some(ScriptKind::Eskimo | ScriptKind::Pirate) => self.create_human(ctx),
            None => {}
        }
    }

    fn raw_call(&mut self, ctx: &UnitCtx, function: i32) {
        if !matches!(self.kind, Some(ScriptKind::Eskimo | ScriptKind::Pirate))
            || function != RAW_START_DROWNING
        {
            return;
        }
        {
            let mut state = self.human.borrow_mut();
            if state.drowning {
                return;
            }
            state.drowning = true;
            state.moving = false;
            state.aiming = false;
            state.aim_ready = false;
        }
        ctx.signal(SIG_AIM);
        ctx.signal(SIG_IDLE);
        ctx.signal(SIG_WALK);
        ctx.spawn(TaskDefinition::with_state(
            "mycube-eskimo-drown",
            Rc::clone(&self.human),
            play_drown,
        ));
    }

    fn start_moving(&mut self, ctx: &UnitCtx, _reversing: bool) {
        if !matches!(self.kind, Some(ScriptKind::Eskimo | ScriptKind::Pirate)) {
            return;
        }
        if self.human.borrow().drowning {
            return;
        }
        self.human.borrow_mut().moving = true;
        ctx.signal(SIG_WALK);
        ctx.spawn(TaskDefinition::with_state(
            "mycube-eskimo-walk",
            Rc::clone(&self.human),
            play_walk,
        ));
    }

    fn stop_moving(&mut self, ctx: &UnitCtx) {
        if !matches!(self.kind, Some(ScriptKind::Eskimo | ScriptKind::Pirate)) {
            return;
        }
        if self.human.borrow().drowning {
            return;
        }
        self.human.borrow_mut().moving = false;
        ctx.signal(SIG_WALK);
        ctx.spawn(idle_task(&self.human));
    }

    fn activate(&mut self, ctx: &UnitCtx) {
        match self.kind {
            Some(ScriptKind::Coalburner) => self.set_heat(ctx, 1.0),
            Some(ScriptKind::Drillship) => self.set_drill_active(ctx, true),
            Some(ScriptKind::Iglu) | Some(ScriptKind::Lantern) => {}
            _ => {}
        }
    }

    fn deactivate(&mut self, ctx: &UnitCtx) {
        match self.kind {
            Some(ScriptKind::Coalburner) => self.set_heat(ctx, 0.0),
            Some(ScriptKind::Drillship) => self.set_drill_active(ctx, false),
            Some(ScriptKind::Iglu) | Some(ScriptKind::Lantern) => {}
            _ => {}
        }
    }

    fn query_weapon(&mut self, _ctx: &UnitCtx, _weapon: WeaponId) -> Piece {
        if matches!(self.kind, Some(ScriptKind::Eskimo | ScriptKind::Pirate)) {
            let state = self.human.borrow();
            if state.has_gun {
                state.pieces.get(HumanPart::GunMuzzle)
            } else {
                state.pieces.get(HumanPart::Head)
            }
        } else {
            Piece::INVALID
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
        if !matches!(self.kind, Some(ScriptKind::Eskimo | ScriptKind::Pirate)) {
            return Some(false);
        }
        let has_gun = self.human.borrow().has_gun;
        // CUS receives Spring's zero-based weapon index. The Lua adapter adds
        // one before calling AimWeapon, so the spear is 0 and the gun is 1.
        let valid = (has_gun && weapon.0 == 1) || (!has_gun && weapon.0 == 0);
        if !valid {
            // The CUS bridge can enter the first weapon's callback while the
            // Lua bridge enters both callbacks for a pirate.  Keep the spear
            // pending, but start the pirate's gun animation from that callback
            // so the gun's own ready state and pose remain identical to Lua.
            if has_gun && weapon.0 == 0 {
                let can_start = {
                    let state = self.human.borrow();
                    !state.drowning && !state.throwing && !state.aim_ready && !state.aiming
                };
                if can_start {
                    self.human.borrow_mut().aiming = true;
                    ctx.signal(SIG_AIM);
                    ctx.spawn(TaskDefinition::with_state(
                        "mycube-eskimo-aim-gun",
                        Rc::clone(&self.human),
                        run_gun,
                    ));
                }
            }
            // Lua's AimWeapon returns nil for the unit's inactive weapon;
            // leaving the CUS call incomplete preserves that behavior.
            return None;
        }
        if self.human.borrow().drowning || self.human.borrow().throwing {
            return Some(false);
        }
        if self.human.borrow().aim_ready {
            return Some(true);
        }
        if self.human.borrow().aiming {
            return None;
        }
        self.human.borrow_mut().aiming = true;
        ctx.signal(SIG_AIM);
        let animation = if has_gun {
            eskimo::AIM_GUN
        } else {
            eskimo::AIM_SPEAR
        };
        if core::ptr::eq(animation.as_ptr(), eskimo::AIM_GUN.as_ptr()) {
            ctx.spawn(TaskDefinition::with_state(
                "mycube-eskimo-aim-gun",
                Rc::clone(&self.human),
                run_gun,
            ));
        } else {
            ctx.spawn(TaskDefinition::with_state(
                "mycube-eskimo-aim-spear",
                Rc::clone(&self.human),
                run_spear,
            ));
        }
        // Keep the weapon pending while the animation task drives it to the
        // ready state. Lua's AimWeapon call is a blocking coroutine, so the
        // native weapon must wait for set_aim_ready(true).
        None
    }

    fn shot(&mut self, ctx: &UnitCtx, _weapon: WeaponId) {
        if !matches!(self.kind, Some(ScriptKind::Eskimo | ScriptKind::Pirate)) {
            return;
        }
        if self.human.borrow().drowning {
            return;
        }
        let has_gun = self.human.borrow().has_gun;
        {
            let mut state = self.human.borrow_mut();
            state.aim_ready = false;
            state.throwing = !has_gun;
        }
        if has_gun {
            ctx.spawn(TaskDefinition::with_state(
                "mycube-eskimo-shoot",
                Rc::clone(&self.human),
                play_shoot,
            ));
        } else {
            ctx.spawn(TaskDefinition::with_state(
                "mycube-eskimo-throw",
                Rc::clone(&self.human),
                play_throw,
            ));
        }
    }

    fn killed(&mut self, ctx: &UnitCtx, _recent_damage: f32, _max_health: f32) -> WreckLevel {
        match self.kind {
            Some(ScriptKind::Eskimo | ScriptKind::Pirate) => {
                let shot = {
                    let mut state = self.human.borrow_mut();
                    state.aiming = false;
                    state.aim_ready = false;
                    state.death_pending = true;
                    random_unit(&mut state.random) > 0.5
                };
                ctx.signal(SIG_AIM);
                ctx.signal(SIG_IDLE);
                ctx.signal(SIG_WALK);
                let sound = if self.kind == Some(ScriptKind::Pirate) {
                    // Match scripts/eskimo.lua: deathSound is the custom
                    // parameter's SoundItem name, not the raw file path.
                    "DeathPirate"
                } else {
                    "DeathEskimo"
                };
                play_unit_sound(ctx.unit(), sound, 9.0);
                let animation = if shot {
                    eskimo::DEATH_SHOT
                } else {
                    eskimo::DEATH_EXHAUSTED
                };
                if core::ptr::eq(animation.as_ptr(), eskimo::DEATH_SHOT.as_ptr()) {
                    ctx.spawn(TaskDefinition::with_state(
                        "mycube-eskimo-death-shot",
                        Rc::clone(&self.human),
                        run_death_shot,
                    ));
                } else {
                    ctx.spawn(TaskDefinition::with_state(
                        "mycube-eskimo-death-exhausted",
                        Rc::clone(&self.human),
                        run_death_exhausted,
                    ));
                }
                if shot {
                    WreckLevel::Two
                } else {
                    WreckLevel::One
                }
            }
            Some(ScriptKind::Iglu) => {
                self.explode_iglu(ctx);
                WreckLevel::One
            }
            Some(ScriptKind::Coalburner | ScriptKind::Drillship | ScriptKind::Lantern) => {
                WreckLevel::One
            }
            None => WreckLevel::None,
        }
    }

    fn killed_pending(&self) -> bool {
        matches!(self.kind, Some(ScriptKind::Eskimo | ScriptKind::Pirate))
            && self.human.borrow().death_pending
    }
}

fn run_gun(state: Rc<RefCell<HumanState>>, ctx: UnitCtx) -> impl core::future::Future<Output = ()> {
    play_aim(state, ctx, eskimo::AIM_GUN, WeaponId(1))
}

fn run_spear(
    state: Rc<RefCell<HumanState>>,
    ctx: UnitCtx,
) -> impl core::future::Future<Output = ()> {
    play_aim(state, ctx, eskimo::AIM_SPEAR, WeaponId(0))
}

fn run_death_shot(
    state: Rc<RefCell<HumanState>>,
    ctx: UnitCtx,
) -> impl core::future::Future<Output = ()> {
    play_death(state, ctx, eskimo::DEATH_SHOT, WreckLevel::Two)
}

fn run_death_exhausted(
    state: Rc<RefCell<HumanState>>,
    ctx: UnitCtx,
) -> impl core::future::Future<Output = ()> {
    play_death(state, ctx, eskimo::DEATH_EXHAUSTED, WreckLevel::One)
}

impl MyCubeUnitScript {
    fn create_human(&mut self, ctx: &UnitCtx) {
        let _ = spring::set_unit_mid_and_aim_pos(
            ctx.unit().0,
            spring::Float3::new(0.0, 30.0, 0.0),
            spring::Float3::new(0.0, 30.0, 0.0),
            true,
        );
        let _ = spring::set_unit_radius_and_height(ctx.unit().0, 20.0, 40.0);
        let has_gun = self.kind == Some(ScriptKind::Pirate);
        {
            let mut state = self.human.borrow_mut();
            state.has_gun = has_gun;
        }
        let pieces = self.human.borrow().pieces;
        ctx.hide(pieces.get(HumanPart::Spear));
        ctx.hide(pieces.get(HumanPart::Gun));
        if has_gun {
            ctx.show(pieces.get(HumanPart::Gun));
        }
        apply_idle_frame(ctx, &self.human);
        ctx.spawn(idle_task(&self.human));
    }

    fn set_heat(&mut self, ctx: &UnitCtx, factor: f32) {
        hide_all(ctx, &self.coal_heaters);
        hide_all(ctx, &self.coal_firecycle);
        ctx.hide(self.coal_inards);
        let factor = factor.clamp(0.0, 1.0);
        let index = ((factor * 4.0 + 1.0) as usize).clamp(1, 4);
        if index == 1 {
            ctx.show(self.coal_inards);
        }
        if let Some(piece) = self.coal_heaters.get(index - 1) {
            ctx.show(*piece);
        }
        if factor > 0.25 {
            show_all(ctx, &self.coal_firecycle);
            for (index, piece) in self.coal_firecycle.iter().enumerate() {
                let speed = if index == 0 { -150.0 } else { 150.0 };
                ctx.spin(
                    *piece,
                    Axis::X,
                    AngularSpeed::radians_per_second(speed),
                    AngularSpeed::radians_per_second(0.0),
                );
            }
        }
    }

    fn build_iglu(&mut self, ctx: &UnitCtx) {
        hide_all(ctx, &self.all);
        if let Some(piece) = self.iglu_build.first() {
            ctx.show(*piece);
        }
        for (index, piece) in self.iglu_build.iter().enumerate().skip(1) {
            if random_unit(&mut self.random) > 0.5 {
                ctx.show(*piece);
            }
            if index % 2 == 0 {
                ctx.turn_now(
                    *piece,
                    Axis::Y,
                    Angle::radians(random_unit(&mut self.random) * 6.2831855),
                );
            }
        }
        for piece in self.iglu_xy_deco.iter().chain(self.iglu_y_deco.iter()) {
            if random_unit(&mut self.random) > 0.5 {
                ctx.show(*piece);
                ctx.turn_now(
                    *piece,
                    Axis::Y,
                    Angle::radians(random_unit(&mut self.random) * 6.2831855),
                );
            }
        }
    }

    fn explode_iglu(&mut self, ctx: &UnitCtx) {
        for piece in self.all.iter().copied() {
            let flags = if random_unit(&mut self.random) > 0.5 {
                explode_flags(spring::cus::SfxFlags::FALL.0 | spring::cus::SfxFlags::NO_HEATCLOUD.0)
            } else {
                explode_flags(
                    spring::cus::SfxFlags::SHATTER.0 | spring::cus::SfxFlags::NO_HEATCLOUD.0,
                )
            };
            ctx.explode(piece, flags);
        }
    }

    fn create_ship(&mut self, ctx: &UnitCtx) {
        let _ = spring::set_unit_mid_and_aim_pos(
            ctx.unit().0,
            spring::Float3::new(0.0, 30.0, 0.0),
            spring::Float3::new(0.0, 30.0, 0.0),
            true,
        );
        let _ = spring::set_unit_radius_and_height(ctx.unit().0, 80.0, 60.0);
        reset_all(ctx, &self.all);
        hide_all(ctx, &self.all);
        show_all(ctx, &self.ship_body);
        if let Some(piece) = self.ship_turret.first() {
            ctx.show(*piece);
        }
        show_all(ctx, &self.ship_drill);
        self.stop_drill(ctx, 0.0);
        for piece in &self.ship_deco {
            if random_unit(&mut self.random) > 0.5 {
                ctx.show(*piece);
            }
        }
        // drillship.lua declares activateWhenBuilt=true.  The engine may send
        // Activate before the queued Rust CUS backend is attached, so the
        // normal replay hook is not sufficient on its own.  Start the same
        // authored drilling state from Create; a later Activate is idempotent
        // and will replace the signalled mining task.
        self.set_drill_active(ctx, true);
    }

    fn set_drill_active(&mut self, ctx: &UnitCtx, active: bool) {
        self.ship.borrow_mut().active = active;
        ctx.signal(SIG_SHIP);
        if active {
            self.start_drill(ctx);
            ctx.spawn(TaskDefinition::with_state(
                "mycube-drillship-mining",
                Rc::clone(&self.ship),
                mine_task,
            ));
        } else {
            self.stop_drill(ctx, 1.0);
        }
    }

    fn start_drill(&mut self, ctx: &UnitCtx) {
        for piece in &self.ship_drill {
            ctx.turn(
                *piece,
                Axis::X,
                Angle::radians(0.0),
                AngularSpeed::radians_per_second(1.0),
            );
            ctx.turn(
                *piece,
                Axis::Y,
                Angle::radians(0.0),
                AngularSpeed::radians_per_second(1.0),
            );
            ctx.turn(
                *piece,
                Axis::Z,
                Angle::radians(0.0),
                AngularSpeed::radians_per_second(1.0),
            );
        }
        if let Some(piece) = self.ship_drill.get(5) {
            ctx.turn(
                *piece,
                Axis::X,
                Angle::degrees(10.0 + random_unit(&mut self.random) * 20.0),
                AngularSpeed::radians_per_second(1.0),
            );
        }
        if let Some(piece) = self.ship_drill.get(6) {
            ctx.spin(
                *piece,
                Axis::Z,
                AngularSpeed::degrees_per_second(-142.0),
                AngularSpeed::radians_per_second(1.0),
            );
        }
    }

    fn stop_drill(&mut self, ctx: &UnitCtx, speed: f32) {
        if let Some(piece) = self.ship_drill.get(1) {
            turn(ctx, *piece, Axis::X, 70.0_f32.to_radians(), speed);
        }
        if let Some(piece) = self.ship_drill.get(3) {
            turn(ctx, *piece, Axis::X, (-160.0_f32).to_radians(), speed);
        }
        if let Some(piece) = self.ship_drill.get(5) {
            turn(ctx, *piece, Axis::X, (-80.0_f32).to_radians(), speed);
        }
        if let Some(piece) = self.ship_drill.get(6) {
            ctx.stop_spin(*piece, Axis::Z, AngularSpeed::radians_per_second(speed));
        }
    }
}

async fn mine_task(state: Rc<RefCell<ShipState>>, ctx: UnitCtx) {
    ctx.set_signal_mask(SIG_SHIP);
    loop {
        if !state.borrow().active {
            return;
        }
        ctx.sleep(spring::cus::Duration::from_millis(500)).await;
        if !state.borrow().active {
            return;
        }
        let mut state = state.borrow_mut();
        ctx.explode(
            state.mine,
            explode_flags(
                spring::cus::SfxFlags::SHATTER.0
                    | spring::cus::SfxFlags::FALL.0
                    | spring::cus::SfxFlags::NO_HEATCLOUD.0,
            ),
        );
        state.minetime = state.minetime.saturating_add(500);
        let _ = state.random;
        play_unit_sound(ctx.unit(), "sounds/icedrill.ogg", 1.0);
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

#[derive(Default)]
pub struct MyCubeCusModule {
    registry: CusRegistry<MyCubeUnitScript>,
    handles: BTreeMap<u32, CusHandle>,
}

impl MyCubeCusModule {
    fn attach_pending(&mut self) {
        let pending = PENDING_UNITS.with(|units| core::mem::take(&mut *units.borrow_mut()));
        for (unit_id, def) in pending {
            let Some(kind) = script_kind_for_def(def) else {
                continue;
            };
            let unit = UnitId(unit_id);
            let instance = instance_id(unit);
            if self.handles.contains_key(&instance) {
                continue;
            }
            let Ok(pieces) = spring::get_unit_piece_map(unit_id) else {
                queue_unit_created(unit_id, def);
                continue;
            };
            let root = spring::get_unit_root_piece(unit_id)
                .ok()
                .and_then(|piece| piece.checked_sub(1))
                .map(Piece)
                .unwrap_or(Piece::INVALID);
            let resolver = EnginePieces { map: pieces, root };
            let host = WasmCus::new(unit, instance);
            let engine = Rc::new(RefCell::new(host.engine()));
            let instance_state = CusInstance::construct(unit, &resolver, engine);
            let handle = self.registry.attach(instance_state);
            let _ = self.registry.with(handle, |instance| {
                instance.with_state(|script| script.kind = Some(kind));
            });
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
                queue_unit_created(unit_id, def);
            }
        }
    }

    fn dispatch_pending_drowning(&mut self) {
        let pending = PENDING_DROWNING.with(|units| core::mem::take(&mut *units.borrow_mut()));
        for unit_id in pending {
            let instance = instance_id(UnitId(unit_id));
            let Some(handle) = self.handles.get(&instance).copied() else {
                PENDING_DROWNING.with(|units| units.borrow_mut().push(unit_id));
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
        spring::game::get_game_frame(0)
            .map(|value| (value.low16 | (value.high16 << 16)) as u64)
            .unwrap_or(0)
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
