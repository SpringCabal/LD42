use alloc::string::String;
use mycube_shared::Status;
use spring_addons::ui::Widget;

use crate::state::{Global, Meters};

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

    fn recv_from_synced(
        &self,
        ctx: &spring_addons::AddonContext<'_, Global, spring::Ui>,
        message: &[u8],
    ) {
        let spring = ctx.spring();
        let Some(status) = core::str::from_utf8(message).ok().and_then(Status::decode) else {
            return;
        };
        *ctx.global().meters.access_mut() = Meters {
            food: status.food,
            heat: status.heat,
            money: status.money,
        };
        let mut hud = ctx.global().hud.access_mut();
        hud.game_end = status.game_end;
        hud.survival_frames = status.survival_frames;
        if hud.intro != status.intro {
            hud.intro = String::from(status.intro);
            hud.intro_frame = spring.game().current_frame().unwrap_or(0);
        }
    }
}
