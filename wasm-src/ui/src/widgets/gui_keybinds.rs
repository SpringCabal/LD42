use mycube_shared::Command;
use mycube_shared::round::PURCHASE_PRICE;
use spring_addons::event::{EventResult, KeyEvent};
use spring_addons::ui::Widget;

use crate::state::Global;

/// The key legend and the restart/quit keys, ported from `keybinds.lua`.
/// The trade keys (1-3) belong to [`super::GuiTradeWidget`].
#[derive(Default)]
pub struct GuiKeybindsWidget;

impl GuiKeybindsWidget {
    pub fn new() -> Self {
        Self
    }
}

impl Widget<Global> for GuiKeybindsWidget {
    fn name(&self) -> &'static str {
        "GuiKeybinds"
    }

    fn draw_screen(
        &self,
        _ctx: &spring_addons::AddonContext<'_, Global>,
        _width: i32,
        _height: i32,
    ) {
        // The strings from keybinds.lua, without its Chili colour escapes.
        let _ = spring::gfx::begin_text(false);
        let _ = spring::gfx::color(1.0, 1.0, 1.0, 1.0);
        for (line, text) in [
            "Ctrl+Q : Quit",
            "1 (+ left mouse): sell ice",
            &alloc::format!("2 (need {PURCHASE_PRICE} money): buy heating resource"),
            &alloc::format!("3 (need {PURCHASE_PRICE} money): buy food"),
        ]
        .into_iter()
        .enumerate()
        {
            let _ = spring::gfx::text(text, 10.0, 80.0 - line as f32 * 20.0, 14.0, "o");
        }
        let _ = spring::gfx::end_text(0);
    }

    fn key_press(
        &self,
        _ctx: &spring_addons::AddonContext<'_, Global>,
        event: &KeyEvent<'_>,
    ) -> EventResult {
        if event.ctrl && event.matches("r") {
            // ui_endgame.lua's Restart button.
            crate::rules::send(Command::Restart);
            return EventResult::Handled;
        }
        if springcabal::menu::is_quit_to_menu(event) {
            springcabal::menu::quit_to_menu();
            return EventResult::Handled;
        }
        EventResult::Ignored
    }
}
