//! The messages between the rules and the UI.

use alloc::string::String;

use springcabal::message::{Encoder, fields};

use crate::round::GameEnd;

const STATUS: &str = "MYCUBE_STATUS";
const MODE: &str = "MODE";
const TRADE: &str = "TRADE";
const RESTART: &str = "RESTART";

/// What the HUD shows, sent by the rules whenever it changes:
/// `MYCUBE_STATUS|food|heat|money|end|survived|intro`, where `survived` is
/// the round's length in frames once it has ended, else -1. The story text
/// comes last and may contain anything.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Status<'a> {
    pub food: i32,
    pub heat: i32,
    pub money: i32,
    pub game_end: Option<GameEnd>,
    pub survival_frames: Option<i32>,
    /// The last story text (`introEvent`).
    pub intro: &'a str,
}

impl<'a> Status<'a> {
    pub fn encode(&self) -> String {
        Encoder::new(STATUS)
            .field(self.food)
            .field(self.heat)
            .field(self.money)
            .field(self.game_end.map_or("", GameEnd::as_str))
            .field(self.survival_frames.unwrap_or(-1))
            .rest(self.intro)
            .finish()
    }

    pub fn decode(message: &'a str) -> Option<Self> {
        let mut fields = fields(message, STATUS)?;
        let food = fields.i32()?;
        let heat = fields.i32()?;
        let money = fields.i32()?;
        let game_end = GameEnd::parse(fields.next()?);
        let survived = fields.i32()?;
        // `rest` is `None` for an empty last field.
        let intro = fields.rest().unwrap_or("");
        Some(Self {
            food,
            heat,
            money,
            game_end,
            survival_frames: (survived >= 0).then_some(survived),
            intro,
        })
    }
}

/// What the player asks the rules for (`ui_modes.lua`, `game_trade.lua`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Command {
    /// Key 1: start or stop selling ice. The rules only hear of it.
    SellMode,
    /// Key 2: buy a load of coal.
    BuyHeat,
    /// Key 3: buy a load of food.
    BuyFood,
    /// A click while selling: sell the ice around this map point.
    Trade { x: f32, z: f32 },
    /// Ctrl+R: start the round over.
    Restart,
}

impl Command {
    pub fn encode(&self) -> String {
        match *self {
            Self::SellMode => Encoder::new(MODE).field(1).finish(),
            Self::BuyHeat => Encoder::new(MODE).field(2).finish(),
            Self::BuyFood => Encoder::new(MODE).field(3).finish(),
            Self::Trade { x, z } => Encoder::new(TRADE)
                .field(format_args!("{x:.1}"))
                .field(format_args!("{z:.1}"))
                .finish(),
            Self::Restart => String::from(RESTART),
        }
    }

    pub fn decode(message: &str) -> Option<Self> {
        if message == RESTART {
            return Some(Self::Restart);
        }
        if let Some(mut fields) = fields(message, MODE) {
            let command = match fields.i32()? {
                1 => Self::SellMode,
                2 => Self::BuyHeat,
                3 => Self::BuyFood,
                _ => return None,
            };
            return fields.is_done().then_some(command);
        }
        let mut fields = fields(message, TRADE)?;
        let (x, z) = fields.xz()?;
        fields.is_done().then_some(Self::Trade { x, z })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn status_round_trip() {
        let status = Status {
            food: 80,
            heat: 72,
            money: 30,
            game_end: Some(GameEnd::Victory),
            survival_frames: Some(1234),
            intro: "Your people are hungry and cold. Buy more: \nCoal: Press 2",
        };
        assert_eq!(Status::decode(&status.encode()), Some(status));

        let playing = Status {
            game_end: None,
            survival_frames: None,
            intro: "",
            ..status
        };
        assert_eq!(playing.encode(), "MYCUBE_STATUS|80|72|30||-1|");
        assert_eq!(Status::decode(&playing.encode()), Some(playing));
        assert_eq!(Status::decode("MYCUBE_STATUS|80|72"), None);
        assert_eq!(Status::decode("OTHER|80|72|30||-1|"), None);
    }

    #[test]
    fn story_text_may_contain_the_separator() {
        let status = Status {
            food: 1,
            heat: 2,
            money: 3,
            game_end: None,
            survival_frames: None,
            intro: "a|b",
        };
        assert_eq!(Status::decode(&status.encode()), Some(status));
    }

    #[test]
    fn command_round_trip() {
        for command in [
            Command::SellMode,
            Command::BuyHeat,
            Command::BuyFood,
            Command::Trade {
                x: 4352.5,
                z: 3840.0,
            },
            Command::Restart,
        ] {
            assert_eq!(Command::decode(&command.encode()), Some(command));
        }
        assert_eq!(Command::BuyHeat.encode(), "MODE|2");
        assert_eq!(Command::decode("MODE|4"), None);
        assert_eq!(Command::decode("MODE|2|1"), None);
        assert_eq!(Command::decode("TRADE|1|inf"), None);
        assert_eq!(Command::decode("RESTART|1"), None);
    }
}
