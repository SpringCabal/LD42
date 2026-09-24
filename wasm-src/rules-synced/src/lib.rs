extern crate alloc;

pub mod defs;
pub mod drills;
pub mod drowning;
pub mod eskimos;
pub mod gadgets;
pub mod health;
#[path = "../../common/ice.rs"]
pub mod ice;
#[cfg(feature = "headless-integration")]
pub mod integration;
pub mod parachutes;
#[path = "../../common/round.rs"]
pub mod round;
pub mod spawning;
pub mod state;
pub mod story;
pub mod terrain;
pub mod trade;
pub mod unit_scripts;
pub mod units;
pub mod untargetable;

spring_addons::export_rules_gadgets! {
    state: state::Global,
    setup: gadgets::setup,
    scratch: 4096,
}

spring::export_core_cus!(unit_scripts::MyCubeCusModule);
