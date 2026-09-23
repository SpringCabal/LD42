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
            "2 (need 50 money): buy heating resource",
            "3 (need 50 money): buy food",
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
            // Match ui_endgame.lua's restart action.  The cheat toggle is
            // needed because the reload command is protected in the engine;
            // turn it back off immediately after the rules reload.
            let _ = spring::send_command("cheat");
            let _ = spring::send_command("luarules reload");
            let _ = spring::send_command("cheat");
            return EventResult::Handled;
        }
        if event.ctrl && event.matches("q") {
            // Same as the Lua keybind: reload into the menu in-process.
            let _ = spring::send_command("reloadforce");
            return EventResult::Handled;
        }
        EventResult::Ignored
    }
}
