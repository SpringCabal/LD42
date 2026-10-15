use mycube_shared::unit;
use spring_addons::ui::Widget;

use crate::defs::{self, UnitKind};
use crate::state::Global;

const BAR_WIDTH: f32 = 100.0;
const BAR_HEIGHT: f32 = 10.0;
const BAR_STEP: f32 = 11.0;
const UNIT_STEP: f32 = 40.0;
const FIRST_UNIT_Y: f32 = 50.0;
const UNIT_PANEL_HEIGHT: f32 = 32.0;

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

        for (index, unit_id) in selection.into_iter().enumerate() {
            let Ok(def) = spring::units_info::get_unit_def_id(unit_id) else {
                continue;
            };
            let y = height as f32 - FIRST_UNIT_Y - UNIT_PANEL_HEIGHT - UNIT_STEP * index as f32;
            if defs::kind(def) == Some(UnitKind::Eskimo) {
                self.draw_bar(
                    0.0,
                    y,
                    spring::get_unit_rules_param_f32(unit_id, unit::HEALTH).unwrap_or(0.0),
                    UnitKind::Eskimo.max_health(),
                    (1.0, 0.0, 0.0),
                );
                self.draw_bar(
                    0.0,
                    y + BAR_STEP,
                    spring::get_unit_rules_param_f32(unit_id, unit::FOOD).unwrap_or(0.0),
                    unit::MAX_NEED,
                    (0.0, 1.0, 0.0),
                );
                self.draw_bar(
                    0.0,
                    y + 2.0 * BAR_STEP,
                    spring::get_unit_rules_param_f32(unit_id, unit::HEAT).unwrap_or(0.0),
                    unit::MAX_NEED,
                    (0.5, 0.3, 0.1),
                );
            } else if let Ok(health) = spring::units_info::get_unit_health(unit_id) {
                self.draw_bar(0.0, y, health.health, health.max_health, (1.0, 0.0, 0.0));
            }
        }
        let _ = spring::gfx::color(1.0, 1.0, 1.0, 1.0);
    }
}
