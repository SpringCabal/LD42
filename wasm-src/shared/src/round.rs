//! What the rules and the UI agree on about a round.

/// Frames in one of the Lua game's seconds (`GAME_FRAME_PER_SEC`). The Lua
/// code counts 33, though the engine simulates 30 frames per second; the
/// story's waits, the Eskimos' needs and the survival time keep its count.
pub const LUA_FRAMES_PER_SECOND: i32 = 33;

/// Money a load of coal or food costs (`ui_modes.lua`). The rules charge it;
/// the key legend shows it.
pub const PURCHASE_PRICE: i32 = 50;

/// How a round ended.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GameEnd {
    Victory,
    Loss,
}

impl GameEnd {
    /// The name in the status message (`game_end.lua`'s `gameEnd`).
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Victory => "victory",
            Self::Loss => "loss",
        }
    }

    /// The game end a status names; anything else means still playing.
    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "victory" => Some(Self::Victory),
            "loss" => Some(Self::Loss),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn game_end_round_trip() {
        for end in [GameEnd::Victory, GameEnd::Loss] {
            assert_eq!(GameEnd::parse(end.as_str()), Some(end));
        }
        assert_eq!(GameEnd::parse(""), None);
    }
}
