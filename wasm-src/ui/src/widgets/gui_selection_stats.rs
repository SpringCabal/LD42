use spring_addons::ui::Widget;

use crate::state::Global;

const BAR_WIDTH: f32 = 100.0;
const BAR_HEIGHT: f32 = 10.0;
const BAR_STEP: f32 = 11.0;
const UNIT_STEP: f32 = 40.0;
const FIRST_UNIT_Y: f32 = 50.0;
const UNIT_PANEL_HEIGHT: f32 = 32.0;

/// The selected-unit panel from `gui_eskimo_bars.lua`.
///
/// The original uses Chili controls at the lower-left of the screen. Keeping
/// this as a screen widget is important: the world healthbar widget above the
/// units is a separate Lua widget and does not replace the selected-unit
/// readout.
#[derive(Default)]
pub struct GuiSelectionStatsWidget;

impl GuiSelectionStatsWidget {
    pub fn new() -> Self {
        Self
    }

    fn draw_bar(&self, x: f32, y: f32, value: f32, maximum: f32, color: (f32, f32, f32)) {
        let fraction = (value / maximum.max(1.0)).clamp(0.0, 1.0);
        let _ = spring::gfx::color(0.08, 0.08, 0.08, 0.9);
        let _ = spring::gfx::rect(x, y, x + BAR_WIDTH, y + BAR_HEIGHT);
        let _ = spring::gfx::color(color.0, color.1, color.2, 1.0);
        let _ = spring::gfx::rect(x, y, x + BAR_WIDTH * fraction, y + BAR_HEIGHT);
    }
}

impl Widget<Global> for GuiSelectionStatsWidget {
    fn name(&self) -> &'static str {
        "GuiSelectionStats"
    }

    fn draw_screen(
        &self,
        _ctx: &spring_addons::AddonContext<'_, Global>,
        _width: i32,
        height: i32,
    ) {
        let Ok(selection) = spring::selection::get_selected_units(0) else {
            return;
        };
        let eskimo_def = spring::get_unit_def_id_by_name("eskimo").ok();

        for (index, unit_id) in selection.into_iter().enumerate() {
            let Ok(def) = spring::units_info::get_unit_def_id(unit_id) else {
                continue;
            };
            // Chili's y coordinate is measured from the top. Core screen
            // primitives use a bottom origin, so mirror the selected-unit
            // panel into the top-left instead of drawing it at the bottom.
            let y = height as f32 - FIRST_UNIT_Y - UNIT_PANEL_HEIGHT - UNIT_STEP * index as f32;
            if eskimo_def == Some(def) {
                self.draw_bar(
                    0.0,
                    y,
                    spring::get_unit_rules_param_f32(unit_id, "health").unwrap_or(0.0),
                    100.0,
                    (1.0, 0.0, 0.0),
                );
                self.draw_bar(
                    0.0,
                    y + BAR_STEP,
                    spring::get_unit_rules_param_f32(unit_id, "food").unwrap_or(0.0),
                    100.0,
                    (0.0, 1.0, 0.0),
                );
                self.draw_bar(
                    0.0,
                    y + 2.0 * BAR_STEP,
                    spring::get_unit_rules_param_f32(unit_id, "heat").unwrap_or(0.0),
                    100.0,
                    (0.5, 0.3, 0.1),
                );
            } else if let Ok(health) = spring::units_info::get_unit_health(unit_id) {
                self.draw_bar(0.0, y, health.health, health.max_health, (1.0, 0.0, 0.0));
            }
        }
        let _ = spring::gfx::color(1.0, 1.0, 1.0, 1.0);
    }
}
