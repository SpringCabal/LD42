use alloc::string::String;

use spring_addons::Resource;

pub const FRAMES_PER_SECOND: i32 = 33;

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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GameEnd {
    Victory,
    Loss,
}

impl GameEnd {
    /// The rules' name for a game end; anything else means still playing.
    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "victory" => Some(Self::Victory),
            "loss" => Some(Self::Loss),
            _ => None,
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

pub fn current_frame() -> i32 {
    spring::game::frame().map_or(0, |frame| frame as i32)
}

/// The shared communication namespace for the UI module.
#[derive(Default)]
pub struct Global {
    pub meters: Resource<Meters>,
    pub hud: Resource<Hud>,
}
