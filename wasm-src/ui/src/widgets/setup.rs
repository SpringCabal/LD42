use spring::default_interface::{
    CLOCK, CONSOLE, MINIMAP, PLAYER_INFO, RESOURCE_BAR, TOOLTIP, UNIT_ICONS,
};
use spring_addons::register_widgets;
use spring_addons::ui::{HideDefaultInterface, WidgetHandler};
use springcabal::music::MusicLoop;

use super::*;
use crate::state::Global;

/// `snd_music.lua`'s track, looped.
const MUSIC_FILE: &str = "sounds/remixed_background.ogg";
const MUSIC_VOLUME: f32 = 0.15;

/// hide_default_layout.lua: the console, resource bar, tooltip, clock, player
/// list and minimap; units never turn into icons.
pub const HIDDEN_INTERFACE: u32 =
    CONSOLE | RESOURCE_BAR | TOOLTIP | CLOCK | PLAYER_INFO | MINIMAP | UNIT_ICONS;

pub fn setup(handler: &mut WidgetHandler<Global>) {
    if springcabal::gallery::active() {
        gallery::setup(handler);
        return;
    }
    register_widgets!(
        handler,
        [
            ApiSyncWidget::new(),
            HideDefaultInterface::new(HIDDEN_INTERFACE),
            SceneryWidget::new(),
            MusicLoop::new(MUSIC_FILE, MUSIC_VOLUME),
            GuiHealthbarsWidget::new(),
            GuiSelectionStatsWidget::new(),
            GuiResourcesWidget::new(),
            UiFloatingTextWidget::new(),
            GuiKeybindsWidget::new(),
            GuiTradeWidget::new(),
            UiEndgameWidget::new(),
        ]
    );
    #[cfg(feature = "e2e")]
    register_widgets!(handler, [dbg_e2e()]);
    #[cfg(feature = "headless-integration")]
    register_widgets!(handler, [DbgUiMessagesWidget::new()]);
}
