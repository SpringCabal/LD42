use alloc::string::String;
use spring_addons::ui::Widget;

use crate::state::{Global, current_frame};

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

    fn recv_from_synced(&self, ctx: &spring_addons::AddonContext<'_, Global>, message: &[u8]) {
        let Ok(message) = core::str::from_utf8(message) else {
            return;
        };
        let mut values = message
            .strip_prefix("MYCUBE_STATUS|")
            .unwrap_or_default()
            .splitn(6, '|');
        let parsed = (
            values.next().and_then(|value| value.parse::<i32>().ok()),
            values.next().and_then(|value| value.parse::<i32>().ok()),
            values.next().and_then(|value| value.parse::<i32>().ok()),
            values.next().and_then(|value| value.parse::<i32>().ok()),
            values.next(),
            values.next(),
        );
        if let (Some(food), Some(heat), Some(money), Some(_eskimos), Some(game_end), Some(intro)) =
            parsed
        {
            *ctx.global().meters.access_mut() = crate::state::Meters { food, heat, money };
            let mut hud = ctx.global().hud.access_mut();
            hud.game_end = String::from(game_end);
            if hud.intro != intro {
                hud.intro = String::from(intro);
                hud.intro_frame = current_frame();
            }
        }
    }
}
