//! The Eskimo and pirate script (`scripts/eskimo.lua`).

use alloc::rc::Rc;
use core::cell::RefCell;

use spring::UnitId;
use spring::cus::{Piece, TaskDefinition, UnitCtx, WeaponId, WreckLevel};

use super::common::{PieceMap, SIG_AIM, SIG_IDLE, SIG_WALK, piece_positions, play_unit_sound};
use super::eskimo::{
    self, HumanPart, HumanPieces, HumanState, apply_idle_frame, idle_task, play_aim, play_death,
    play_drown, play_shoot, play_throw, play_walk,
};

pub(super) struct HumanScript {
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
                aiming: false,
                aim_ready: false,
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

    /// `StartDrowning`: stop everything and flounder until dead.
    pub(super) fn start_drowning(&mut self, ctx: &UnitCtx) {
        {
            let mut state = self.state.borrow_mut();
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

    pub(super) fn aim_weapon(&mut self, ctx: &UnitCtx, weapon: WeaponId) -> Option<bool> {
        let has_gun = self.state.borrow().has_gun;
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
                    let state = self.state.borrow();
                    !state.drowning && !state.throwing && !state.aim_ready && !state.aiming
                };
                if can_start {
                    self.state.borrow_mut().aiming = true;
                    ctx.signal(SIG_AIM);
                    self.spawn("mycube-eskimo-aim-gun", ctx, run_gun);
                }
            }
            // Lua's AimWeapon returns nil for the unit's inactive weapon;
            // leaving the CUS call incomplete preserves that behavior.
            return None;
        }
        {
            let state = self.state.borrow();
            if state.drowning || state.throwing {
                return Some(false);
            }
            if state.aim_ready {
                return Some(true);
            }
            if state.aiming {
                return None;
            }
        }
        self.state.borrow_mut().aiming = true;
        ctx.signal(SIG_AIM);
        if has_gun {
            self.spawn("mycube-eskimo-aim-gun", ctx, run_gun);
        } else {
            self.spawn("mycube-eskimo-aim-spear", ctx, run_spear);
        }
        // Keep the weapon pending while the animation task drives it to the
        // ready state. Lua's AimWeapon call is a blocking coroutine, so the
        // native weapon must wait for set_aim_ready(true).
        None
    }

    pub(super) fn shot(&mut self, ctx: &UnitCtx) {
        let has_gun = {
            let mut state = self.state.borrow_mut();
            if state.drowning {
                return;
            }
            state.aim_ready = false;
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
            state.aiming = false;
            state.aim_ready = false;
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

fn run_gun(state: Rc<RefCell<HumanState>>, ctx: UnitCtx) -> impl core::future::Future<Output = ()> {
    play_aim(state, ctx, eskimo::AIM_GUN, WeaponId(1))
}

fn run_spear(
    state: Rc<RefCell<HumanState>>,
    ctx: UnitCtx,
) -> impl core::future::Future<Output = ()> {
    // The spear table ends with an empty keyframe at t=10 after the pose has
    // settled at t=9. Lua's blocking AimWeapon is ready at t=9; waiting out
    // the empty interval would miss the engine's firing check by a frame.
    let settled = &eskimo::AIM_SPEAR[..eskimo::AIM_SPEAR.len() - 1];
    play_aim(state, ctx, settled, WeaponId(0))
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
