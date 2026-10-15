use spring_addons::ui::Widget;

use crate::state::Global;

#[derive(Default)]
pub struct GuiResourcesWidget;

impl GuiResourcesWidget {
    pub fn new() -> Self {
        Self
    }

    fn draw_bar(&self, x: f32, panel_y: f32, name: &str, value: i32, color: (f32, f32, f32)) {
        let value = value.clamp(0, 100) as f32;
        let bar_y = panel_y + 8.0;
        let _ = spring::gfx::color(0.08, 0.08, 0.08, 0.95);
        let _ = spring::gfx::rect(x, bar_y, x + 100.0, bar_y + 30.0);
        let _ = spring::gfx::color(color.0, color.1, color.2, 1.0);
        let _ = spring::gfx::rect(x, bar_y, x + value, bar_y + 30.0);
        let _ = spring::gfx::begin_text(false);
        let _ = spring::gfx::color(0.9, 0.9, 0.9, 1.0);
        let _ = spring::gfx::text(name, x + 50.0, panel_y + 44.0, 14.0, "cson");
        let _ = spring::gfx::text(
            &alloc::format!("{}", value as i32),
            x + 50.0,
            bar_y + 9.0,
            14.0,
            "cson",
        );
        let _ = spring::gfx::end_text(0);
    }
}

impl Widget<Global> for GuiResourcesWidget {
    fn name(&self) -> &'static str {
        "GuiResources"
    }

    fn draw_screen(&self, ctx: &spring_addons::AddonContext<'_, Global>, width: i32, _height: i32) {
        let meters = *ctx.global().meters.access();
        let panel_x = width as f32 * 0.4;
        let panel_y = 10.0;
        let _ = spring::gfx::color(0.02, 0.02, 0.02, 0.78);
        let _ = spring::gfx::rect(panel_x, panel_y, panel_x + 500.0, panel_y + 60.0);

        self.draw_bar(panel_x, panel_y, "Money", meters.money, (0.2, 0.8, 0.1));
        self.draw_bar(
            panel_x + 110.0,
            panel_y,
            "Heat",
            meters.heat,
            (0.5, 0.3, 0.1),
        );
        self.draw_bar(
            panel_x + 220.0,
            panel_y,
            "Food",
            meters.food,
            (0.7, 0.1, 0.0),
        );
    }
}
