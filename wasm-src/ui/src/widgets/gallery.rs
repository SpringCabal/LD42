//! Gallery mode (`springcabal::gallery`): only the widgets that draw what a
//! player sees in the world.

use spring_addons::ui::{HideDefaultInterface, WidgetHandler};
use springcabal::gallery::Gallery;

use super::*;
use crate::state::Global;

/// Only the engine interface is hidden: units never turn into icons.  The
/// game draws no world effects of its own; health bars and texts are HUD.
pub fn setup(handler: &mut WidgetHandler<Global>) {
    Gallery::new(handler)
        .widget(HideDefaultInterface::new(setup::HIDDEN_INTERFACE))
        .announce("mycube-ui");
}
