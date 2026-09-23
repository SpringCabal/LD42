use alloc::string::String;
use spring_addons::ui::Widget;

use crate::state::{GameEnd, Global, Meters, current_frame};

#[derive(Default)]
pub struct ApiSyncWidget;

impl ApiSyncWidget {
    pub fn new() -> Self {
        Self
    }
}

impl Widget<Global> for ApiSyncWidget {
    fn name(&self) -> &'static str {
        "ApiSync"
    }

    /// Parses the rules' `MYCUBE_STATUS|food|heat|money|end|end_frame|intro`.
    fn recv_from_synced(&self, ctx: &spring_addons::AddonContext<'_, Global>, message: &[u8]) {
        let Some(status) = core::str::from_utf8(message)
            .ok()
            .and_then(|message| message.strip_prefix("MYCUBE_STATUS|"))
        else {
            return;
        };
        let mut values = status.splitn(6, '|');
        let mut number = || values.next().and_then(|value| value.parse::<i32>().ok());
        let (Some(food), Some(heat), Some(money)) = (number(), number(), number()) else {
            return;
        };
        let (Some(game_end), Some(end_frame), Some(intro)) = (
            values.next().map(GameEnd::parse),
            values.next().and_then(|value| value.parse::<i32>().ok()),
            values.next(),
        ) else {
            return;
        };
        *ctx.global().meters.access_mut() = Meters { food, heat, money };
        let mut hud = ctx.global().hud.access_mut();
        hud.game_end = game_end;
        hud.end_frame = (end_frame >= 0).then_some(end_frame);
        if hud.intro != intro {
            hud.intro = String::from(intro);
            hud.intro_frame = current_frame();
        }
    }
}
