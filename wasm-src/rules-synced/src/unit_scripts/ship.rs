//! The drill ship script (`scripts/shipscript.lua`).

use alloc::rc::Rc;
use alloc::vec::Vec;
use core::cell::RefCell;

use spring::cus::{Angle, AngularSpeed, Axis, Piece, SfxFlags, TaskDefinition, UnitCtx};

use super::common::{
    PieceMap, SIG_SHIP, all_named_pieces, coin_flip, hide_all, matching_pieces, piece_in,
    play_unit_sound, reset_all, show_all, turn,
};

struct MiningState {
    active: bool,
    mine: Piece,
}

pub(super) struct ShipScript {
    all: Vec<Piece>,
    body: Vec<Piece>,
    turret: Vec<Piece>,
    drill: Vec<Piece>,
    deco: Vec<Piece>,
    mining: Rc<RefCell<MiningState>>,
}

impl ShipScript {
    pub(super) fn new(map: &PieceMap) -> Self {
        Self {
            all: all_named_pieces(map),
            body: matching_pieces(map, "Boatbody"),
            turret: matching_pieces(map, "Turret"),
            drill: matching_pieces(map, "icedrill"),
            deco: matching_pieces(map, "Deco"),
            mining: Rc::new(RefCell::new(MiningState {
                active: false,
                mine: piece_in(map, "icemine"),
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
        let _ = spring::set_unit_radius_and_height(ctx.unit().0, 80.0, 60.0);
        reset_all(ctx, &self.all);
        hide_all(ctx, &self.all);
        show_all(ctx, &self.body);
        if let Some(piece) = self.turret.first() {
            ctx.show(*piece);
        }
        show_all(ctx, &self.drill);
        self.stop_drill(ctx, 0.0);
        for piece in &self.deco {
            if coin_flip() {
                ctx.show(*piece);
            }
        }
        // drillship.lua declares activateWhenBuilt=true.  The engine may send
        // Activate before the queued Rust CUS backend is attached, so the
        // normal replay hook is not sufficient on its own.  Start the same
        // authored drilling state from Create; a later Activate is idempotent
        // and will replace the signalled mining task.
        self.set_active(ctx, true);
    }

    pub(super) fn set_active(&mut self, ctx: &UnitCtx, active: bool) {
        self.mining.borrow_mut().active = active;
        ctx.signal(SIG_SHIP);
        if active {
            self.start_drill(ctx);
            ctx.spawn(TaskDefinition::with_state(
                "mycube-drillship-mining",
                Rc::clone(&self.mining),
                mine_task,
            ));
        } else {
            self.stop_drill(ctx, 1.0);
        }
    }

    fn start_drill(&mut self, ctx: &UnitCtx) {
        let speed = AngularSpeed::radians_per_second(1.0);
        for piece in &self.drill {
            for axis in [Axis::X, Axis::Y, Axis::Z] {
                ctx.turn(*piece, axis, Angle::radians(0.0), speed);
            }
        }
        if let Some(piece) = self.drill.get(5) {
            let angle = spring::random::synced::int(10, 30) as f32;
            ctx.turn(*piece, Axis::X, Angle::degrees(angle), speed);
        }
        if let Some(piece) = self.drill.get(6) {
            ctx.spin(
                *piece,
                Axis::Z,
                AngularSpeed::degrees_per_second(-142.0),
                speed,
            );
        }
    }

    fn stop_drill(&mut self, ctx: &UnitCtx, speed: f32) {
        if let Some(piece) = self.drill.get(1) {
            turn(ctx, *piece, Axis::X, 70.0_f32.to_radians(), speed);
        }
        if let Some(piece) = self.drill.get(3) {
            turn(ctx, *piece, Axis::X, (-160.0_f32).to_radians(), speed);
        }
        if let Some(piece) = self.drill.get(5) {
            turn(ctx, *piece, Axis::X, (-80.0_f32).to_radians(), speed);
        }
        if let Some(piece) = self.drill.get(6) {
            ctx.stop_spin(*piece, Axis::Z, AngularSpeed::radians_per_second(speed));
        }
    }
}

/// `sfxIceMining`: ice chips and the drill sound every half second.
async fn mine_task(state: Rc<RefCell<MiningState>>, ctx: UnitCtx) {
    ctx.set_signal_mask(SIG_SHIP);
    loop {
        if !state.borrow().active {
            return;
        }
        ctx.sleep(spring::cus::Duration::from_millis(500)).await;
        let (active, mine) = {
            let state = state.borrow();
            (state.active, state.mine)
        };
        if !active {
            return;
        }
        // shipscript.lua chips ice on half of the ticks.
        if spring::random::synced::int(1, 50) <= 25 {
            continue;
        }
        ctx.explode(
            mine,
            SfxFlags(SfxFlags::SHATTER.0 | SfxFlags::FALL.0 | SfxFlags::NO_HEATCLOUD.0),
        );
        play_unit_sound(ctx.unit(), "sounds/icedrill.ogg", 1.0);
    }
}
