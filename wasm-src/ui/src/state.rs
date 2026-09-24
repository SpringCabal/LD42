use alloc::string::String;

use spring_addons::Resource;

pub use crate::round::GameEnd;

/// The three resource meters, refreshed from the rules module's status
/// broadcast and read by the resource panel.
#[derive(Clone, Copy, Debug)]
pub struct Meters {
    pub food: i32,
    pub heat: i32,
    pub money: i32,
}

impl Default for Meters {
    /// The meters start full; the rules module overwrites them with its first
    /// status broadcast.
    fn default() -> Self {
        Self {
            food: 80,
            heat: 80,
            money: 80,
        }
    }
}

/// Screen state: the scripted intro text and the end-game banner.
pub struct Hud {
    pub game_end: Option<GameEnd>,
    /// How long the round lasted, in frames, once it has ended.
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

/// The shared communication namespace for the UI module.
#[derive(Default)]
pub struct Global {
    pub meters: Resource<Meters>,
    pub hud: Resource<Hud>,
}
