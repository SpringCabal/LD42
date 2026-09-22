use spring_addons::ui::Widget;

use crate::state::Global;

const BAR_WIDTH: f32 = 46.0;
const BAR_HEIGHT: f32 = 4.0;
const BAR_GAP: f32 = 2.0;
const UNIT_HEIGHT_SCALE: f32 = 1.8;

/// The original healthbar widget is a billboarded world draw call. Core's
/// portable drawing API does not expose the widget's old display-list path,
/// so project the same unit anchor to screen space and draw the bars there.
/// This keeps the bars attached to the units and, unlike engine bars, uses the
/// game's public health/food/heat rules parameters.
#[derive(Default)]
pub struct GuiHealthbarsWidget;

impl GuiHealthbarsWidget {
    pub fn new() -> Self {
        Self
    }

    fn draw_bar(&self, x: f32, y: f32, value: f32, maximum: f32, color: (f32, f32, f32)) {
        let fraction = (value / maximum.max(1.0)).clamp(0.0, 1.0);
        let left = x - BAR_WIDTH / 2.0;
        let _ = spring::gfx::color(0.06, 0.06, 0.06, 0.9);
        let _ = spring::gfx::rect(left, y, left + BAR_WIDTH, y + BAR_HEIGHT);
        let _ = spring::gfx::color(color.0, color.1, color.2, 0.95);
        let _ = spring::gfx::rect(left, y, left + BAR_WIDTH * fraction, y + BAR_HEIGHT);
    }

    fn health_color(&self, fraction: f32) -> (f32, f32, f32) {
        // Same red -> yellow -> green ramp used by gui_healthbars.lua.
        let fraction = fraction.clamp(0.0, 1.0);
        if fraction < 0.5 {
            (1.0, fraction * 2.0, 0.0)
        } else {
            ((1.0 - fraction) * 2.0, 1.0, 0.0)
        }
    }
}

impl Widget<Global> for GuiHealthbarsWidget {
    fn name(&self) -> &'static str {
        "GuiHealthbars"
    }

    fn draw_screen(&self, _ctx: &spring_addons::AddonContext<'_, Global>, width: i32, height: i32) {
        let Ok(eskimo_def) = spring::get_unit_def_id_by_name("eskimo") else {
            return;
        };
        let pirate_def = spring::get_unit_def_id_by_name("pirate").ok();
        let drillship_def = spring::get_unit_def_id_by_name("drillship").ok();

        for unit_id in spring::get_all_units().unwrap_or_default() {
            let Ok(def) = spring::units_info::get_unit_def_id(unit_id) else {
                continue;
            };
            let is_eskimo = def == eskimo_def;
            let is_pirate = pirate_def == Some(def);
            let is_drillship = drillship_def == Some(def);
            if !is_eskimo && !is_pirate && !is_drillship {
                continue;
            }
            let Ok(position) = spring::unit_rendering::get_unit_view_position(unit_id, false)
            else {
                continue;
            };
            let radius = spring::units_info::get_unit_radius(unit_id).unwrap_or(20.0);
            let Ok(projected) = spring::camera::world_to_screen_coords(spring::Float3::new(
                position.x,
                position.y + UNIT_HEIGHT_SCALE * radius,
                position.z,
            )) else {
                continue;
            };
            let screen = projected.screen_pos;
            if !projected.valid
                || screen.x < -BAR_WIDTH
                || screen.x > width as f32 + BAR_WIDTH
                || screen.y < -50.0
                || screen.y > height as f32 + 50.0
            {
                continue;
            }

            let health = spring::get_unit_rules_param_f32(unit_id, "health").unwrap_or(0.0);
            if is_eskimo {
                self.draw_bar(
                    screen.x,
                    screen.y + 20.0,
                    health,
                    100.0,
                    self.health_color(health / 100.0),
                );
                let food = spring::get_unit_rules_param_f32(unit_id, "food").unwrap_or(0.0);
                self.draw_bar(
                    screen.x,
                    screen.y + 20.0 - BAR_HEIGHT - BAR_GAP,
                    food,
                    100.0,
                    (0.0, 1.0, 0.0),
                );
                let heat = spring::get_unit_rules_param_f32(unit_id, "heat").unwrap_or(0.0);
                self.draw_bar(
                    screen.x,
                    screen.y + 20.0 - 2.0 * (BAR_HEIGHT + BAR_GAP),
                    heat,
                    100.0,
                    (0.5, 0.3, 0.1),
                );
            } else if is_pirate {
                self.draw_bar(
                    screen.x,
                    screen.y + 20.0,
                    health,
                    100.0,
                    self.health_color(health / 100.0),
                );
            } else {
                self.draw_bar(
                    screen.x,
                    screen.y + 20.0,
                    health,
                    500.0,
                    self.health_color(health / 500.0),
                );
            }
        }
        let _ = spring::gfx::color(1.0, 1.0, 1.0, 1.0);
    }
}
