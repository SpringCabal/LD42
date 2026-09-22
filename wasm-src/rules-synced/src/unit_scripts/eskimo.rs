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
    pub(super) const ALL: [Self; 14] = [
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
        match self {
            Self::Abs => 0,
            Self::ArmLeft => 1,
            Self::ArmRight => 2,
            Self::Gun => 3,
            Self::GunMuzzle => 4,
            Self::HandLeft => 5,
            Self::HandRight => 6,
            Self::Head => 7,
            Self::KneeLeft => 8,
            Self::KneeRight => 9,
            Self::Spear => 10,
            Self::ThighLeft => 11,
            Self::ThighRight => 12,
            Self::Torso => 13,
        }
    }
}

#[derive(Clone, Copy)]
pub(super) struct HumanPieces {
    pub(super) all: [Piece; 14],
}

impl HumanPieces {
    pub(super) fn from_init(ctx: &spring::cus::InitCtx<'_>) -> Self {
        Self {
            all: [
                ctx.piece("Abs"),
                ctx.piece("ArmLeft"),
                ctx.piece("ArmRight"),
                ctx.piece("Gun"),
                ctx.piece("GunMuzzle"),
                ctx.piece("HandLeft"),
                ctx.piece("HandRight"),
                ctx.piece("Head"),
                ctx.piece("KneeLeft"),
                ctx.piece("KneeRight"),
                ctx.piece("Spear"),
                ctx.piece("ThighLeft"),
                ctx.piece("ThighRight"),
                ctx.piece("Torso"),
            ],
        }
    }

    pub(super) fn get(self, part: HumanPart) -> Piece {
        self.all[part.index()]
    }
}

#[derive(Clone, Copy)]
pub(super) struct HumanState {
    pub(super) pieces: HumanPieces,
    pub(super) offsets: [spring::Float3; 14],
    pub(super) moving: bool,
    pub(super) drowning: bool,
    pub(super) has_gun: bool,
    pub(super) aiming: bool,
    pub(super) aim_ready: bool,
    pub(super) throwing: bool,
    pub(super) death_pending: bool,
    pub(super) random: u32,
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
    offsets: &[spring::Float3; 14],
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
    offsets: &[spring::Float3; 14],
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

async fn run_aim_animation(
    ctx: &UnitCtx,
    pieces: HumanPieces,
    offsets: &[spring::Float3; 14],
    animation: &'static [Keyframe],
) {
    // The spear table ends with empty keyframes at t=9 and t=10. The pose is
    // already settled at t=9; waiting through the empty final interval would
    // make the CUS ready notification miss the engine's firing check by one
    // frame, unlike the blocking Lua callin.
    if animation.len() == 3 && animation[1].commands.is_empty() && animation[2].commands.is_empty()
    {
        apply_frame(ctx, pieces, offsets, animation[0]);
        ctx.sleep(spring::cus::Duration::from_millis(
            (animation[1].time - animation[0].time) as u64 * 33,
        ))
        .await;
        apply_frame(ctx, pieces, offsets, animation[1]);
        return;
    }
    run_animation(ctx, pieces, offsets, animation, 0).await;
}

fn snapshot(state: &Rc<RefCell<HumanState>>) -> (HumanPieces, [spring::Float3; 14]) {
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
    // Lua's blocking AimWeapon call is entered later by the legacy unit
    // script bridge than the asynchronous CUS callback. Preserve that
    // authored lead-in before changing any visible pieces or pose.
    ctx.sleep(spring::cus::Duration::from_millis(12 * 33)).await;
    if state.borrow().drowning {
        state.borrow_mut().aiming = false;
        ctx.set_aim_ready(weapon, false);
        return;
    }
    let (pieces, offsets) = snapshot(&state);
    if weapon.0 == 0 {
        ctx.show(pieces.get(HumanPart::Spear));
    }
    // run_animation applies the first keyframe immediately and then waits
    // through the authored time interval before each subsequent keyframe.
    // Starting at index 1 would skip the initial 0->next-keyframe interval,
    // making CUS report the weapon ready too early.
    run_aim_animation(&ctx, pieces, &offsets, animation).await;
    state.borrow_mut().aim_ready = true;
    state.borrow_mut().aiming = false;
    ctx.set_aim_ready(weapon, true);
}

pub(super) async fn play_throw(state: Rc<RefCell<HumanState>>, ctx: UnitCtx) {
    ctx.set_signal_mask(SIG_AIM);
    let (pieces, offsets) = snapshot(&state);
    ctx.hide(pieces.get(HumanPart::Spear));
    // PlayAnimation applies the first keyframe before waiting for its first
    // authored interval; do not skip THROW[0] -> THROW[1].
    run_animation(&ctx, pieces, &offsets, THROW, 0).await;
    state.borrow_mut().throwing = false;
}

pub(super) async fn play_shoot(state: Rc<RefCell<HumanState>>, ctx: UnitCtx) {
    ctx.set_signal_mask(SIG_AIM);
    let pieces = state.borrow().pieces;
    // This is the exact short recoil in eskimo.lua.  The original waits on
    // Gun's z axis after issuing the x-axis move; the CUS wait is intentionally
    // omitted because that axis is not animated by the recoil call.
    move_piece(&ctx, pieces.get(HumanPart::Gun), Axis::X, 15.0, 0.0);
    ctx.next_frame().await;
    move_piece(&ctx, pieces.get(HumanPart::Gun), Axis::X, 0.0, 15.0);
    state.borrow_mut().throwing = false;
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
    // PlayAnimation applies keyframe zero and waits through its first
    // authored interval before applying keyframe one.
    run_animation(&ctx, pieces, &offsets, animation, 0).await;
    state.borrow_mut().death_pending = false;
    ctx.set_killed_finished(wreck_level);
}

pub(super) fn idle_task(state: &Rc<RefCell<HumanState>>) -> TaskDefinition {
    TaskDefinition::with_state("mycube-eskimo-idle", Rc::clone(state), play_idle)
}
