use spring_addons::ui::{HideDefaultInterface, WidgetHandler};
use springcabal::verification::WorldWidgets;

use super::*;
use crate::state::Global;

pub fn setup(handler: &mut WidgetHandler<Global>, spring: &spring::Spring<'_, spring::Ui>) {
    WorldWidgets::new(handler)
        .widget(HideDefaultInterface::new(setup::HIDDEN_INTERFACE))
        .announce(spring, "mycube-ui");
}
