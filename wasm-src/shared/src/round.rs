pub const LUA_FRAMES_PER_SECOND: i32 = 33;

pub const PURCHASE_PRICE: i32 = 50;

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
