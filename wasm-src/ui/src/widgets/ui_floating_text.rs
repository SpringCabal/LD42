use spring_addons::ui::Widget;

use crate::state::Global;
use mycube_shared::round::LUA_FRAMES_PER_SECOND;

const SHOW_FRAMES: i32 = 5 * LUA_FRAMES_PER_SECOND;
const RISE: f32 = 50.0;

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

    fn draw_screen(
        &self,
        ctx: &spring_addons::AddonContext<'_, Global, spring::Ui>,
        width: i32,
        height: i32,
    ) {
        let spring = ctx.spring();
        let frame = spring.game().current_frame().unwrap_or(0);
        let hud = ctx.global().hud.access();
        let start = hud.intro_frame;
        let age = frame - start;
        if !(0..SHOW_FRAMES).contains(&age) {
            return;
        }
        let _ = spring.gfx().begin_text(false);
        let _ = spring.gfx().color(0.85, 0.95, 1.0, 1.0);
        let y = height as f32 / 4.0 - age as f32 / SHOW_FRAMES as f32 * RISE;
        for (line, text) in hud.intro.lines().enumerate() {
            let _ = spring.gfx().text(
                text,
                width as f32 / 2.0,
                y - line as f32 * 34.0,
                30.0,
                "cson",
            );
        }
        let _ = spring.gfx().end_text();
    }
}
