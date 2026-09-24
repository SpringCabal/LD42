//! What the rules and UI modules must agree on: the round's constants, the
//! unit rules params, the ice-trade yield, and the two messages between
//! them, [`Status`] (rules to UI) and [`Command`] (UI to rules).

#![no_std]

extern crate alloc;

pub mod ice;
pub mod message;
pub mod round;
pub mod test_sites;
pub mod unit;

pub use message::{Command, Status};
pub use round::GameEnd;
