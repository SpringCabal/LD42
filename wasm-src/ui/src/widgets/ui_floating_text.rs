use spring_addons::ui::Widget;

use crate::state::{Global, current_frame};

/// The story overlay, ported from `ui_floating_text.lua`.
#[derive(Default)]
pub struct UiFloatingTextWidget;

impl UiFloatingTextWidget {
    pub fn new() -> Self {
        Self
    }
}

impl Widget<Global> for UiFloatingTextWidget {
    fn name(&self) -> &'static str {
        "UiFloatingText"
    }

    fn draw_screen(&self, ctx: &spring_addons::AddonContext<'_, Global>, width: i32, height: i32) {
        let frame = current_frame();
        let hud = ctx.global().hud.access();
        let start = hud.intro_frame;
        let age = frame - start;
        // ui_floating_text.lua displays a story event for five seconds and moves
        // it upward by 50 screen units over that time.
        if !(0..165).contains(&age) {
            return;
        }
        let _ = spring::gfx::begin_text(false);
        let _ = spring::gfx::color(0.85, 0.95, 1.0, 1.0);
        let y = height as f32 / 4.0 - age as f32 / 165.0 * 50.0;
        for (line, text) in hud.intro.lines().enumerate() {
            let _ = spring::gfx::text(
                text,
                width as f32 / 2.0,
                y - line as f32 * 34.0,
                30.0,
                "cson",
            );
        }
        let _ = spring::gfx::end_text(0);
    }
}
