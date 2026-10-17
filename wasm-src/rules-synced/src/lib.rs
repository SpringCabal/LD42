extern crate alloc;

pub mod defs;
pub mod gadgets;
#[cfg(feature = "headless-integration")]
pub mod integration;
pub mod spawning;
pub mod state;
pub mod terrain;
pub mod unit_scripts;
pub mod units;

spring_addons::export_rules_gadgets! {
    state: state::Global,
    setup: gadgets::setup,
    scratch: 4096,
    cus: unit_scripts::MyCubeCusModule,
}
