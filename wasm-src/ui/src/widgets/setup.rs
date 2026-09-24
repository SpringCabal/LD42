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
/// Seconds, for when the engine can't report the stream length.
const MUSIC_LENGTH: f32 = 91.008;

pub fn setup(handler: &mut WidgetHandler<Global>) {
    register_widgets!(
        handler,
        [
            ApiSyncWidget::new(),
            // hide_default_layout.lua: the console, resource bar, tooltip,
            // clock, player list and minimap; units never turn into icons.
            HideDefaultInterface::new(
                CONSOLE | RESOURCE_BAR | TOOLTIP | CLOCK | PLAYER_INFO | MINIMAP | UNIT_ICONS
            ),
            SceneryWidget::new(),
            GuiCameraWidget::new(),
            MusicLoop::new(MUSIC_FILE, MUSIC_VOLUME).length_hint(MUSIC_LENGTH),
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
    register_widgets!(handler, [DbgE2eWidget::new()]);
}
