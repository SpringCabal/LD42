use alloc::string::String;

use spring_addons::Resource;

pub use mycube_shared::GameEnd;

#[derive(Clone, Copy, Debug)]
pub struct Meters {
    pub food: i32,
    pub heat: i32,
    pub money: i32,
}

impl Default for Meters {
    fn default() -> Self {
        Self {
            food: 80,
            heat: 80,
            money: 80,
        }
    }
}

pub struct Hud {
    pub game_end: Option<GameEnd>,
    pub survival_frames: Option<i32>,
    pub intro: String,
    pub intro_frame: i32,
}

impl Default for Hud {
    fn default() -> Self {
        Self {
            game_end: None,
            survival_frames: None,
            intro: String::new(),
            intro_frame: -1_000,
        }
    }
}

#[derive(Default)]
pub struct Global {
    pub meters: Resource<Meters>,
    pub hud: Resource<Hud>,
}
