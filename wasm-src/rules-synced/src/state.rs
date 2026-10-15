use alloc::vec::Vec;

use spring_addons::Resource;

use crate::gadgets::drills::behavior::Drills;
pub use mycube_shared::GameEnd;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Supply {
    Food,
    Heat,
    Money,
}

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

#[derive(Default)]
pub struct Story {
    pub started: bool,
    pub start_frame: i32,
    pub step: usize,
    pub next_step: i32,
    pub intro_event: &'static str,
    pub story_eskimos: usize,
    pub story_drills: usize,
    pub story_parachutes: usize,
    pub story_pirates: usize,
    pub supports: Vec<(f32, f32)>,
    pub game_end: Option<GameEnd>,
    pub end_frame: Option<i32>,
}

impl Story {
    pub fn end_game(&mut self, end: GameEnd, frame: i32) {
        if self.game_end == Some(GameEnd::Loss) || self.game_end == Some(end) {
            return;
        }
        self.game_end = Some(end);
        self.end_frame = Some(frame);
    }

    pub fn survival_frames(&self) -> Option<i32> {
        self.end_frame.map(|end| end - self.start_frame)
    }
}

#[derive(Clone, Copy)]
pub struct Eskimo {
    pub id: i32,
    pub food: f32,
    pub heat: f32,
    pub eating: bool,
    pub warm: bool,
}

#[derive(Default)]
pub struct Village {
    pub eskimos: Vec<Eskimo>,
    pub burners: Vec<i32>,
}

#[derive(Default)]
pub struct Global {
    pub economy: Resource<Economy>,
    pub story: Resource<Story>,
    pub village: Resource<Village>,
    pub drills: Resource<Drills>,
}

impl Global {
    pub fn restart_round(&self) {
        *self.economy.access_mut() = Economy::default();
        *self.story.access_mut() = Story::default();
        *self.village.access_mut() = Village::default();
        *self.drills.access_mut() = Drills::default();
    }
}
