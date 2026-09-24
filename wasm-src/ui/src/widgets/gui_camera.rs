use spring::Float3;
use spring_addons::ui::Widget;
use springcabal_control::camera::{CameraPlacement, View};

use crate::state::Global;

/// The authored overhead camera in `camera_control.lua`.
const VIEW: View = View::overhead(3300.0, 0.464).pitch(-0.901_499_33, -0.433_569_3);
const POSITION: Float3 = Float3::new(3150.0, 102.341_46, 3480.0);

/// Places the initial camera.
pub struct GuiCameraWidget {
    camera: CameraPlacement,
}

impl Default for GuiCameraWidget {
    fn default() -> Self {
        Self::new()
    }
}

impl GuiCameraWidget {
    pub fn new() -> Self {
        Self {
            camera: CameraPlacement::new(VIEW, POSITION),
        }
    }
}

impl Widget<Global> for GuiCameraWidget {
    fn name(&self) -> &'static str {
        "GuiCamera"
    }

    fn update(&self, ctx: &spring_addons::AddonContext<'_, Global>, dt: f32) {
        self.camera.update(ctx, dt);
    }
}
