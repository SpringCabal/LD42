use alloc::rc::Rc;
use core::cell::RefCell;

use spring::cus::{Axis, Piece, TaskDefinition, UnitCtx};

use super::common::{SIG_AIM, SIG_IDLE, SIG_WALK, move_piece, turn};

#[derive(Clone, Copy, Debug)]
pub(super) enum HumanPart {
    Abs,
    ArmLeft,
    ArmRight,
    Gun,
    GunMuzzle,
    HandLeft,
    HandRight,
    Head,
    KneeLeft,
    KneeRight,
    Spear,
    ThighLeft,
    ThighRight,
    Torso,
}

impl HumanPart {
    pub(super) const COUNT: usize = 14;

    pub(super) const ALL: [Self; Self::COUNT] = [
        Self::Abs,
        Self::ArmLeft,
        Self::ArmRight,
        Self::Gun,
        Self::GunMuzzle,
        Self::HandLeft,
        Self::HandRight,
        Self::Head,
        Self::KneeLeft,
        Self::KneeRight,
        Self::Spear,
        Self::ThighLeft,
        Self::ThighRight,
        Self::Torso,
    ];

    pub(super) const fn index(self) -> usize {
        self as usize
    }

    pub(super) const fn name(self) -> &'static str {
        match self {
            Self::Abs => "Abs",
            Self::ArmLeft => "ArmLeft",
            Self::ArmRight => "ArmRight",
            Self::Gun => "Gun",
            Self::GunMuzzle => "GunMuzzle",
            Self::HandLeft => "HandLeft",
            Self::HandRight => "HandRight",
            Self::Head => "Head",
            Self::KneeLeft => "KneeLeft",
            Self::KneeRight => "KneeRight",
            Self::Spear => "Spear",
            Self::ThighLeft => "ThighLeft",
            Self::ThighRight => "ThighRight",
            Self::Torso => "Torso",
        }
    }
}

#[derive(Clone, Copy)]
pub(super) struct HumanPieces {
    pub(super) all: [Piece; HumanPart::COUNT],
}

impl HumanPieces {
    pub(super) fn from_map(map: &super::common::PieceMap) -> Self {
        Self {
            all: HumanPart::ALL.map(|part| super::common::piece_in(map, part.name())),
        }
    }

    pub(super) fn get(self, part: HumanPart) -> Piece {
        self.all[part.index()]
    }
}

#[derive(Clone, Copy)]
pub(super) struct HumanState {
    pub(super) pieces: HumanPieces,
    pub(super) offsets: [spring::Float3; HumanPart::COUNT],
    pub(super) moving: bool,
    pub(super) drowning: bool,
    pub(super) has_gun: bool,
    pub(super) throwing: bool,
    pub(super) death_pending: bool,
}

#[derive(Clone, Copy)]
pub(super) enum AnimOp {
    Move,
    Turn,
}

#[derive(Clone, Copy)]
pub(super) struct AnimationCommand {
    pub(super) op: AnimOp,
    pub(super) part: HumanPart,
    pub(super) axis: Axis,
    pub(super) target: f32,
    pub(super) speed: f32,
}

#[derive(Clone, Copy)]
pub(super) struct Keyframe {
    pub(super) time: u32,
    pub(super) commands: &'static [AnimationCommand],
}

include!("eskimo_animations.rs");

pub(super) fn apply_frame(
    ctx: &UnitCtx,
    pieces: HumanPieces,
    offsets: &[spring::Float3; HumanPart::COUNT],
    frame: Keyframe,
) {
    for command in frame.commands {
        let piece = pieces.get(command.part);
        match command.op {
            AnimOp::Turn => turn(ctx, piece, command.axis, command.target, command.speed),
            AnimOp::Move => {
                let offset = offsets[command.part.index()];
                let target = match command.axis {
                    Axis::X => command.target - offset.x,
                    Axis::Y => command.target - offset.y,
                    Axis::Z => command.target - offset.z,
                };
                move_piece(ctx, piece, command.axis, target, command.speed);
            }
        }
    }
}

pub(super) async fn run_animation(
    ctx: &UnitCtx,
    pieces: HumanPieces,
    offsets: &[spring::Float3; HumanPart::COUNT],
    animation: &'static [Keyframe],
    start: usize,
) {
    for index in start..animation.len() {
        let frame = animation[index];
        apply_frame(ctx, pieces, offsets, frame);
        if let Some(next) = animation.get(index + 1) {
            let delay = next.time.saturating_sub(frame.time) * 33;
            ctx.sleep(spring::cus::Duration::from_millis(delay as u64))
                .await;
        }
    }
}

fn snapshot(state: &Rc<RefCell<HumanState>>) -> (HumanPieces, [spring::Float3; HumanPart::COUNT]) {
    let state = state.borrow();
    (state.pieces, state.offsets)
}

pub(super) fn apply_idle_frame(ctx: &UnitCtx, state: &Rc<RefCell<HumanState>>) {
    let (pieces, offsets) = snapshot(state);
    apply_frame(ctx, pieces, &offsets, IDLE[0]);
}

pub(super) async fn play_idle(state: Rc<RefCell<HumanState>>, ctx: UnitCtx) {
    ctx.set_signal_mask(SIG_WALK);
    if state.borrow().drowning {
        return;
    }
    let (pieces, offsets) = snapshot(&state);
    apply_frame(&ctx, pieces, &offsets, IDLE[0]);
    run_animation(&ctx, pieces, &offsets, IDLE, 1).await;
}

pub(super) async fn play_walk(state: Rc<RefCell<HumanState>>, ctx: UnitCtx) {
    ctx.set_signal_mask(SIG_WALK);
    loop {
        let finished = {
            let state = state.borrow();
            !state.moving || state.drowning
        };
        if finished {
            return;
        }
        let (pieces, offsets) = snapshot(&state);
        run_animation(&ctx, pieces, &offsets, WALK, 0).await;
    }
}

pub(super) async fn play_aim(
    state: Rc<RefCell<HumanState>>,
    ctx: UnitCtx,
    animation: &'static [Keyframe],
    weapon: spring::cus::WeaponId,
) {
    ctx.set_signal_mask(SIG_AIM);
    let (pieces, offsets) = snapshot(&state);
    if weapon.0 == 0 {
        ctx.show(pieces.get(HumanPart::Spear));
    }
    run_animation(&ctx, pieces, &offsets, animation, 0).await;
    ctx.set_aim_ready(weapon, true);
}

pub(super) async fn play_throw(state: Rc<RefCell<HumanState>>, ctx: UnitCtx) {
    let (pieces, offsets) = snapshot(&state);
    ctx.hide(pieces.get(HumanPart::Spear));
    run_animation(&ctx, pieces, &offsets, THROW, 0).await;
    state.borrow_mut().throwing = false;
}

pub(super) async fn play_shoot(state: Rc<RefCell<HumanState>>, ctx: UnitCtx) {
    let pieces = state.borrow().pieces;
    move_piece(&ctx, pieces.get(HumanPart::Gun), Axis::X, 15.0, 0.0);
    move_piece(&ctx, pieces.get(HumanPart::Gun), Axis::X, 0.0, 15.0);
}

pub(super) async fn play_drown(state: Rc<RefCell<HumanState>>, ctx: UnitCtx) {
    ctx.set_signal_mask(SIG_IDLE);
    loop {
        let (pieces, offsets) = snapshot(&state);
        run_animation(&ctx, pieces, &offsets, FLOUNDER, 0).await;
    }
}

pub(super) async fn play_death(
    state: Rc<RefCell<HumanState>>,
    ctx: UnitCtx,
    animation: &'static [Keyframe],
    wreck_level: spring::cus::WreckLevel,
) {
    ctx.set_signal_mask(SIG_IDLE);
    let (pieces, offsets) = snapshot(&state);
    run_animation(&ctx, pieces, &offsets, animation, 0).await;
    state.borrow_mut().death_pending = false;
    ctx.set_killed_finished(wreck_level);
}

pub(super) fn idle_task(state: &Rc<RefCell<HumanState>>) -> TaskDefinition {
    TaskDefinition::with_state("mycube-eskimo-idle", Rc::clone(state), play_idle)
}
