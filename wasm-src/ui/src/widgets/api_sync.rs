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

    /// Parses the rules' `MYCUBE_STATUS|food|heat|money|end|survived|intro`.
    fn recv_from_synced(&self, ctx: &spring_addons::AddonContext<'_, Global>, message: &[u8]) {
        let Some(mut fields) = core::str::from_utf8(message)
            .ok()
            .and_then(|message| springcabal::message::fields(message, "MYCUBE_STATUS"))
        else {
            return;
        };
        let (Some(food), Some(heat), Some(money)) = (fields.i32(), fields.i32(), fields.i32())
        else {
            return;
        };
        let (Some(game_end), Some(survived), Some(intro)) = (
            fields.next().map(GameEnd::parse),
            fields.i32(),
            fields.next(),
        ) else {
            return;
        };
        *ctx.global().meters.access_mut() = Meters { food, heat, money };
        let mut hud = ctx.global().hud.access_mut();
        hud.game_end = game_end;
        hud.survival_frames = (survived >= 0).then_some(survived);
        if hud.intro != intro {
            hud.intro = String::from(intro);
            hud.intro_frame = current_frame();
        }
    }
}
