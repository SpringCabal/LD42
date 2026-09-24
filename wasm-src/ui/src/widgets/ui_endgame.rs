use spring_addons::ui::Widget;

use crate::state::{GameEnd, Global};

/// The victory banner, ported from `ui_endgame.lua`.
#[derive(Default)]
pub struct UiEndgameWidget;

impl UiEndgameWidget {
    pub fn new() -> Self {
        Self
    }
}

impl Widget<Global> for UiEndgameWidget {
    fn name(&self) -> &'static str {
        "UiEndgame"
    }

    fn draw_screen(&self, ctx: &spring_addons::AddonContext<'_, Global>, width: i32, height: i32) {
        let hud = ctx.global().hud.access();
        let Some(game_end) = hud.game_end else {
            return;
        };

        // A centred card, clear of the resource meters at the bottom.
        let card_width = 460.0;
        let card_height = 220.0;
        let card_x = (width as f32 - card_width) / 2.0;
        let card_y = (height as f32 - card_height) / 2.0;
        let _ = spring::gfx::color(0.03, 0.06, 0.08, 0.94);
        let _ = spring::gfx::rect(card_x, card_y, card_x + card_width, card_y + card_height);
        let _ = spring::gfx::color(0.3, 0.75, 0.85, 1.0);
        let _ = spring::gfx::rect(
            card_x,
            card_y + card_height - 5.0,
            card_x + card_width,
            card_y + card_height,
        );

        let title = match game_end {
            GameEnd::Loss => "Game Over",
            GameEnd::Victory => "Victory!",
        };
        let survival_seconds =
            hud.survival_frames.unwrap_or(0).max(0) / mycube_shared::round::LUA_FRAMES_PER_SECOND;
        let _ = spring::gfx::begin_text(false);
        let _ = spring::gfx::color(0.85, 0.95, 1.0, 1.0);
        let _ = spring::gfx::text(title, width as f32 / 2.0, card_y + 135.0, 32.0, "cson");
        let _ = spring::gfx::text(
            &alloc::format!("Survival time: {survival_seconds}s"),
            width as f32 / 2.0,
            card_y + 90.0,
            18.0,
            "cson",
        );
        let _ = spring::gfx::text(
            "Ctrl+R: restart    Ctrl+Q: return to menu",
            width as f32 / 2.0,
            card_y + 42.0,
            16.0,
            "cson",
        );
        let _ = spring::gfx::end_text(0);
    }
}
