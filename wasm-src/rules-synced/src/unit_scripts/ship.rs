//! The drill ship script (`scripts/shipscript.lua`). The drilling itself
//! (`GG.Drill.DrillUnit`) is the drills gadget's job.

use alloc::rc::Rc;
use alloc::vec::Vec;
use core::cell::{Cell, RefCell};

use spring::cus::{Angle, AngularSpeed, Axis, Piece, SfxFlags, TaskDefinition, UnitCtx};

use super::common::{
    PieceMap, all_named_pieces, coin_flip, hide_all, matching_pieces, piece_in, play_unit_sound,
    reset_all, show_all, turn,
};

/// Chips of ice after which the hold is full (`setIceDrillLoad`).
const FULL_LOAD_CHIPS: u32 = 100;
const SINKING_SOUND: &str = "sounds/sinkingship.ogg";
/// The engine's simulation frames per second (`Game.gameSpeed`).
const ENGINE_FRAMES_PER_SECOND: u32 = 30;

/// The frame until which a sinking sound is playing, shared by every drill
/// ship: `PlaySoundByUnitDefID` plays one at a time per unit def.
pub(super) type SinkingSoundLock = Rc<Cell<u32>>;

struct MiningState {
    active: bool,
    /// Ice chips mined so far (`minetime`).
    chips: u32,
    mine: Piece,
    icecubes: Vec<Piece>,
}

pub(super) struct ShipScript {
    all: Vec<Piece>,
    body: Vec<Piece>,
    turret: Vec<Piece>,
    drill: Vec<Piece>,
    deco: Vec<Piece>,
    mining: Rc<RefCell<MiningState>>,
    sinking_sound: SinkingSoundLock,
}

impl ShipScript {
    pub(super) fn new(map: &PieceMap, sinking_sound: SinkingSoundLock) -> Self {
        Self {
            all: all_named_pieces(map),
            body: matching_pieces(map, "Boatbody"),
            turret: matching_pieces(map, "Turret"),
            drill: matching_pieces(map, "icedrill"),
            deco: matching_pieces(map, "Deco"),
            mining: Rc::new(RefCell::new(MiningState {
                active: false,
                chips: 0,
                mine: piece_in(map, "icemine"),
                icecubes: matching_pieces(map, "icecube"),
            })),
            sinking_sound,
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
        ctx.spawn(TaskDefinition::with_state(
            "mycube-drillship-mining",
            Rc::clone(&self.mining),
            mine_task,
        ));
    }

    /// `startIcedrill(1.0, math.random(10, 30))` / `stopIcedrill(1.0)`.
    /// A repeated call does nothing.
    pub(super) fn set_active(&mut self, ctx: &UnitCtx, active: bool) {
        if self.mining.borrow().active == active {
            return;
        }
        if active {
            self.start_drill(ctx);
        } else {
            self.stop_drill(ctx, 1.0);
        }
        self.mining.borrow_mut().active = active;
    }

    /// Killed: the sinking sound, unless another drill ship's is playing.
    pub(super) fn killed(&mut self, ctx: &UnitCtx) {
        let length_ms = spring::random::synced::int(1000, 1500);
        let frame = spring::game::frame().unwrap_or(0);
        if frame < self.sinking_sound.get() {
            return;
        }
        self.sinking_sound
            .set(frame + (length_ms as u32) * ENGINE_FRAMES_PER_SECOND / 1000);
        play_unit_sound(ctx.unit(), SINKING_SOUND, 0.75);
    }

    fn start_drill(&mut self, ctx: &UnitCtx) {
        let angle = spring::random::synced::int(10, 30) as f32;
        let speed = AngularSpeed::radians_per_second(1.0);
        for piece in &self.drill {
            for axis in [Axis::X, Axis::Y, Axis::Z] {
                ctx.turn(*piece, axis, Angle::radians(0.0), speed);
            }
        }
        if let Some(piece) = self.drill.get(5) {
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

/// `sfxIceMining`: every half second while drilling, on a coin flip, ice
/// chips fly off the drill and the hold fills up.
async fn mine_task(state: Rc<RefCell<MiningState>>, ctx: UnitCtx) {
    loop {
        ctx.sleep(spring::cus::Duration::from_millis(500)).await;
        // shipscript.lua: `bIceMiningActive == true and math.random(1, 50) > 25`.
        if !state.borrow().active || spring::random::synced::int(1, 50) <= 25 {
            continue;
        }
        let mine = state.borrow().mine;
        ctx.explode(
            mine,
            SfxFlags(SfxFlags::SHATTER.0 | SfxFlags::FALL.0 | SfxFlags::NO_HEATCLOUD.0),
        );
        play_unit_sound(ctx.unit(), "sounds/icedrill.ogg", 1.0);
        let mut state = state.borrow_mut();
        state.chips += 1;
        let load = state.chips.min(FULL_LOAD_CHIPS) as f32 / FULL_LOAD_CHIPS as f32;
        show_ice_load(&ctx, &state.icecubes, load);
    }
}

/// `setIceDrillLoad`: show the first `load` share of the ice cubes in the
/// hold, each turned and spun at random again.
fn show_ice_load(ctx: &UnitCtx, icecubes: &[Piece], load: f32) {
    let total = icecubes.len() as f32;
    for (index, piece) in icecubes.iter().copied().enumerate() {
        if (index + 1) as f32 / total >= load {
            break;
        }
        ctx.show(piece);
        // turnPieceRandDir(id, math.random(0.5, 5)); Lua truncates 0.5 to 0.
        let speed = spring::random::synced::int(0, 5) as f32;
        for axis in [Axis::X, Axis::Y, Axis::Z] {
            let angle = spring::random::synced::int(-360, 360) as f32;
            turn(ctx, piece, axis, angle.to_radians(), speed);
        }
        // spinRand(id, -3, 3, 2)
        for axis in [Axis::X, Axis::Y, Axis::Z] {
            let speed = spring::random::synced::int(-3, 3) as f32;
            ctx.spin(
                piece,
                axis,
                AngularSpeed::degrees_per_second(speed),
                AngularSpeed::radians_per_second(2.0),
            );
        }
    }
}
