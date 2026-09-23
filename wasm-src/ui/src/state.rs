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

/// Screen state: the scripted intro text and the end-game banner.
pub struct Hud {
    pub game_end: String,
    pub end_frame: Option<i32>,
    pub intro: String,
    pub intro_frame: i32,
}

impl Default for Hud {
    fn default() -> Self {
        Self {
            game_end: String::new(),
            end_frame: None,
            intro: String::new(),
            intro_frame: -1_000,
        }
    }
}

/// The unsynced preview of the ice trade under the cursor.
#[derive(Clone, Copy, Debug)]
pub struct TradePreview {
    pub screen_x: i32,
    pub screen_y: i32,
    pub world_x: f32,
    pub world_y: f32,
    pub world_z: f32,
    pub value: i32,
}

/// Whether the player is placing an ice-trade order.
#[derive(Clone, Copy, Debug, Default)]
pub struct Interaction {
    pub trading: bool,
    pub preview: Option<TradePreview>,
}

pub fn current_frame() -> i32 {
    spring::game::frame().map_or(0, |frame| frame as i32)
}

/// The shared communication namespace for the UI module.
#[derive(Default)]
pub struct Global {
    pub meters: Resource<Meters>,
    pub hud: Resource<Hud>,
    pub interaction: Resource<Interaction>,
}
