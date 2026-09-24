//! State shared between gadgets. Everything else lives in the gadget that
//! owns it.

use alloc::vec::Vec;

use spring_addons::Resource;

use crate::drills::Drills;
pub use mycube_shared::GameEnd;

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

/// Scripted-story progress.
#[derive(Default)]
pub struct Story {
    /// The map's units have been cleared and the story has begun.
    pub started: bool,
    /// The frame the round started (`game_end.lua`'s `startTime`).
    pub start_frame: i32,
    pub step: usize,
    pub next_step: i32,
    /// The last story text (`introEvent`).
    pub intro_event: &'static str,
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

    /// Frames from the start of the round to its end, once it has ended.
    pub fn survival_frames(&self) -> Option<i32> {
        self.end_frame.map(|end| end - self.start_frame)
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

impl Global {
    /// Start the round over. The storyteller clears the map and restores
    /// the ice on the next frame, as `storyteller.lua`'s Initialize does.
    pub fn restart_round(&self) {
        *self.economy.access_mut() = Economy::default();
        *self.story.access_mut() = Story::default();
        *self.village.access_mut() = Village::default();
        *self.drills.access_mut() = Drills::default();
    }
}
