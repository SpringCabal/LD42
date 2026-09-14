extern crate alloc;

pub mod drills;
pub mod drowning;
pub mod eskimos;
pub mod gadgets;
pub mod health;
#[cfg(feature = "headless-integration")]
pub mod integration;
pub mod parachutes;
pub mod spawning;
pub mod state;
pub mod story;
pub mod terrain;
pub mod trade;
pub mod units;
pub mod untargetable;

spring_addons::export_rules_gadgets! {
    state: state::Global,
    setup: gadgets::setup,
    scratch: 4096,
}
