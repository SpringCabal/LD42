extern crate alloc;

pub mod defs;
#[path = "../../common/ice.rs"]
pub mod ice;
pub mod state;
pub mod widgets;

spring_addons::export_ui_widgets! {
    state: state::Global,
    setup: widgets::setup,
    scratch: 4096,
}
