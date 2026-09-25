//! The Eskimo and pirate script (`scripts/eskimo.lua`).

use alloc::rc::Rc;
use core::cell::RefCell;

use spring::cus::{Piece, TaskDefinition, UnitCtx, WeaponId, WreckLevel};
use spring::{ResultExt, UnitId};

use super::common::{PieceMap, SIG_AIM, SIG_IDLE, SIG_WALK, piece_positions, play_unit_sound};
use super::eskimo::{
    self, HumanPart, HumanPieces, HumanState, apply_idle_frame, idle_task, play_aim, play_death,
    play_drown, play_shoot, play_throw, play_walk,
};

pub struct HumanScript {
    pirate: bool,
    state: Rc<RefCell<HumanState>>,
}

impl HumanScript {
    pub(super) fn new(unit: UnitId, map: &PieceMap, pirate: bool) -> Self {
        let pieces = HumanPieces::from_map(map);
        let offsets = piece_positions(unit, map, HumanPart::ALL.map(HumanPart::name));
        Self {
            pirate,
            state: Rc::new(RefCell::new(HumanState {
                pieces,
                offsets,
                moving: false,
                drowning: false,
                has_gun: false,
                throwing: false,
                death_pending: false,
            })),
        }
    }

    pub(super) fn create(&mut self, ctx: &UnitCtx) {
        let _ = spring::set_unit_mid_and_aim_pos(
            ctx.unit().0,
            spring::Float3::new(0.0, 30.0, 0.0),
            spring::Float3::new(0.0, 30.0, 0.0),
            true,
        );
        let _ = spring::set_unit_radius_and_height(ctx.unit().0, 20.0, 40.0);
        let has_gun = self.pirate;
        self.state.borrow_mut().has_gun = has_gun;
        let pieces = self.state.borrow().pieces;
        ctx.hide(pieces.get(HumanPart::Spear));
        ctx.hide(pieces.get(HumanPart::Gun));
        if has_gun {
            ctx.show(pieces.get(HumanPart::Gun));
        }
        apply_idle_frame(ctx, &self.state);
        ctx.spawn(idle_task(&self.state));
    }

    /// Whether `StartDrowning` has run.
    #[cfg(feature = "headless-integration")]
    pub(super) fn is_drowning(&self) -> bool {
        self.state.borrow().drowning
    }

    /// `StartDrowning`: stop everything and flounder until dead.
    pub(super) fn start_drowning(&mut self, ctx: &UnitCtx) {
        {
            let mut state = self.state.borrow_mut();
            if state.drowning {
                return;
            }
            state.drowning = true;
            state.moving = false;
        }
        ctx.signal(SIG_AIM);
        ctx.signal(SIG_IDLE);
        ctx.signal(SIG_WALK);
        self.spawn("mycube-eskimo-drown", ctx, play_drown);
    }

    pub(super) fn start_moving(&mut self, ctx: &UnitCtx) {
        if self.state.borrow().drowning {
            return;
        }
        self.state.borrow_mut().moving = true;
        ctx.signal(SIG_WALK);
        self.spawn("mycube-eskimo-walk", ctx, play_walk);
    }

    pub(super) fn stop_moving(&mut self, ctx: &UnitCtx) {
        if self.state.borrow().drowning {
            return;
        }
        self.state.borrow_mut().moving = false;
        ctx.signal(SIG_WALK);
        ctx.spawn(idle_task(&self.state));
    }

    pub(super) fn query_weapon(&self) -> Piece {
        let state = self.state.borrow();
        if state.has_gun {
            state.pieces.get(HumanPart::GunMuzzle)
        } else {
            state.pieces.get(HumanPart::Head)
        }
    }

    /// eskimo.lua's AimWeapon: the Eskimo aims its spear (weapon 1, 0
    /// here), the pirate its gun (weapon 2). Each call that finds the
    /// weapon loaded, with no throw running, restarts the aim animation,
    /// which readies the weapon when it ends; every other call leaves the
    /// weapon unready, as Lua's `nil` does.
    pub(super) fn aim_weapon(&mut self, ctx: &UnitCtx, weapon: WeaponId) -> Option<bool> {
        let (drowning, has_gun, throwing) = {
            let state = self.state.borrow();
            (state.drowning, state.has_gun, state.throwing)
        };
        let own_weapon = if has_gun { 1 } else { 0 };
        if drowning || weapon.0 != own_weapon || throwing || !loaded(ctx.unit(), weapon) {
            return Some(false);
        }
        ctx.signal(SIG_AIM);
        if has_gun {
            self.spawn("mycube-eskimo-aim-gun", ctx, run_gun);
        } else {
            self.spawn("mycube-eskimo-aim-spear", ctx, run_spear);
        }
        None
    }

    pub(super) fn shot(&mut self, ctx: &UnitCtx) {
        let has_gun = {
            let mut state = self.state.borrow_mut();
            if state.drowning {
                return;
            }
            state.throwing = !state.has_gun;
            state.has_gun
        };
        if has_gun {
            self.spawn("mycube-eskimo-shoot", ctx, play_shoot);
        } else {
            self.spawn("mycube-eskimo-throw", ctx, play_throw);
        }
    }

    pub(super) fn killed(&mut self, ctx: &UnitCtx) -> WreckLevel {
        let shot = {
            let mut state = self.state.borrow_mut();
            state.death_pending = true;
            spring::random::synced::float() > 0.5
        };
        ctx.signal(SIG_AIM);
        ctx.signal(SIG_IDLE);
        ctx.signal(SIG_WALK);
        // deathSound is the SoundItem name from customParams, not a file.
        let sound = if self.pirate {
            "DeathPirate"
        } else {
            "DeathEskimo"
        };
        play_unit_sound(ctx.unit(), sound, 9.0);
        if shot {
            self.spawn("mycube-eskimo-death-shot", ctx, run_death_shot);
            WreckLevel::Two
        } else {
            self.spawn("mycube-eskimo-death-exhausted", ctx, run_death_exhausted);
            WreckLevel::One
        }
    }

    pub(super) fn killed_pending(&self) -> bool {
        self.state.borrow().death_pending
    }

    fn spawn<F>(
        &self,
        name: &'static str,
        ctx: &UnitCtx,
        task: fn(Rc<RefCell<HumanState>>, UnitCtx) -> F,
    ) where
        F: core::future::Future<Output = ()> + 'static,
    {
        ctx.spawn(TaskDefinition::with_state(
            name,
            Rc::clone(&self.state),
            task,
        ));
    }
}

/// Lua's `isLoaded`: the weapon's reload has finished.
fn loaded(unit: UnitId, weapon: WeaponId) -> bool {
    spring::units_weapons::get_unit_weapon_state(unit.0, weapon.0 + 1, "reloadFrame")
        .log_err_once("My Cube eskimo script: get unit weapon state")
        .is_some_and(|state| state.reload_frame <= spring::game::current_frame() as f32)
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
