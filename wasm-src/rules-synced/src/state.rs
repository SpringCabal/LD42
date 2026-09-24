//! State shared between gadgets. Everything else lives in the gadget that
//! owns it.

use alloc::string::String;
use alloc::vec::Vec;

use spring_addons::Resource;

use crate::drills::Drills;

pub const FRAMES_PER_SECOND: i32 = 33;

/// The village's three stocks.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Supply {
    Food,
    Heat,
    Money,
}

/// The village's stocks, ported from `game_resources.lua`.
#[derive(Clone, Copy, Debug)]
pub struct Economy {
    pub food: f32,
    pub heat: f32,
    pub money: f32,
}

impl Economy {
    const MAX: f32 = 100.0;

    fn stock(&mut self, supply: Supply) -> &mut f32 {
        match supply {
            Supply::Food => &mut self.food,
            Supply::Heat => &mut self.heat,
            Supply::Money => &mut self.money,
        }
    }

    /// Take `amount` if the village has it.
    pub fn consume(&mut self, supply: Supply, amount: f32) -> bool {
        let stock = self.stock(supply);
        if *stock < amount {
            return false;
        }
        *stock -= amount;
        true
    }

    pub fn add(&mut self, supply: Supply, amount: f32) {
        let stock = self.stock(supply);
        *stock = (*stock + amount).min(Self::MAX);
    }
}

impl Default for Economy {
    fn default() -> Self {
        Self {
            food: 80.0,
            heat: 80.0,
            money: 80.0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GameEnd {
    Victory,
    Loss,
}

impl GameEnd {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Victory => "victory",
            Self::Loss => "loss",
        }
    }
}

/// Scripted-story progress.
#[derive(Default)]
pub struct Story {
    /// The map's own units have been cleared and the story has begun.
    pub started: bool,
    pub step: usize,
    pub next_step: i32,
    pub intro_event: String,
    pub story_eskimos: usize,
    pub story_drills: usize,
    pub story_parachutes: usize,
    pub story_pirates: usize,
    /// Where the story placed iglus and the coal burner, which keep their
    /// distance from each other.
    pub supports: Vec<(f32, f32)>,
    pub game_end: Option<GameEnd>,
    pub end_frame: Option<i32>,
}

impl Story {
    pub fn end_game(&mut self, end: GameEnd, frame: i32) {
        // A loss takes precedence if the final player unit disappears on the
        // same frame that the scripted story reaches its outro.
        if self.game_end == Some(GameEnd::Loss) || self.game_end == Some(end) {
            return;
        }
        self.game_end = Some(end);
        self.end_frame = Some(frame);
    }
}

#[derive(Clone, Copy)]
pub struct Eskimo {
    pub id: i32,
    pub food: f32,
    pub heat: f32,
    // unit_eskimos.lua carries these states between frames. They are not
    // cosmetic: eating continues until food is safely replenished and warmth
    // is retained at the resource-cap threshold.
    pub eating: bool,
    pub warm: bool,
}

/// The player's people and the coal burners that warm them. Owned by the
/// Eskimos gadget; drowning and the game-end check read it.
#[derive(Default)]
pub struct Village {
    pub eskimos: Vec<Eskimo>,
    pub burners: Vec<i32>,
    /// Which fifth of the Eskimos publishes its food and heat this frame.
    pub sync_lane: usize,
}

/// The shared communication namespace.
///
/// Each subsystem is an independently borrowable [`Resource`], so a nested
/// callin only conflicts when it touches the same subsystem.
#[derive(Default)]
pub struct Global {
    pub economy: Resource<Economy>,
    pub story: Resource<Story>,
    pub village: Resource<Village>,
    /// Owned by the Drills gadget; shared only with the integration test.
    pub drills: Resource<Drills>,
}
