extern crate alloc;

pub mod defs;
#[path = "../../common/ice.rs"]
pub mod ice;
#[path = "../../common/round.rs"]
pub mod round;
pub mod rules;
pub mod state;
#[cfg(feature = "headless-integration")]
#[path = "../../common/test_sites.rs"]
pub mod test_sites;
pub mod widgets;

spring_addons::export_ui_widgets! {
    state: state::Global,
    setup: widgets::setup,
    scratch: 4096,
}
