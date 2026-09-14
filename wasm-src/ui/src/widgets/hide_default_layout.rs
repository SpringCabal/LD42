use spring_addons::ui::Widget;

use crate::state::Global;

/// Hides Spring's stock HUD, ported from `hide_default_layout.lua`.
#[derive(Default)]
pub struct HideDefaultLayoutWidget {
    applied: core::cell::Cell<bool>,
}

impl HideDefaultLayoutWidget {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Widget<Global> for HideDefaultLayoutWidget {
    fn name(&self) -> &'static str {
        "HideDefaultLayout"
    }

    fn update(&self, _ctx: &spring_addons::AddonContext<'_, Global>, _dt: f32) {
        if !self.applied.get() {
            self.applied.set(true);
            for command in [
                "Console 0",
                "ResBar 0",
                "ToolTip 0",
                "Clock 0",
                "Info 0",
                "showhealthbars 0",
                "showrezbars 0",
                "disticon 99999",
            ] {
                let _ = spring::send_commands(command, "");
            }
            let _ = spring::slave_mini_map(true);
            let _ = spring::config_mini_map(-1, -1, -1, -1);
        }
    }
}
