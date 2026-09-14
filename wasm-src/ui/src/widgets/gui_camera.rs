use spring_addons::ui::Widget;

use crate::state::Global;

/// Places the initial camera from the authored state in `camera_control.lua`.
#[derive(Default)]
pub struct GuiCameraWidget {
    applied: core::cell::Cell<bool>,
}

impl GuiCameraWidget {
    pub fn new() -> Self {
        Self::default()
    }

    fn apply(&self) {
        if self.applied.get() {
            return;
        }
        let Ok(true) = spring::set_camera_state(
            spring::CameraState {
                name: "spring",
                position: spring::Float3::new(3150.0, 102.34146, 3480.0),
                direction: spring::Float3::new(0.0, -0.90149933, -0.4335693),
                up: spring::Float3::new(0.0, 0.4335693, -0.90149933),
                right: spring::Float3::new(1.0, 0.0, 0.0),
                fov: 45.0,
                rx: 0.0,
                ry: 0.0,
                rz: 0.0,
                dist: 0.0,
                height: 3300.0,
                angle: 0.464,
                mode: 1,
            },
            0.0,
            1.0,
            1.0,
        ) else {
            return;
        };
        self.applied.set(true);
    }
}

impl Widget<Global> for GuiCameraWidget {
    fn name(&self) -> &'static str {
        "GuiCamera"
    }

    fn update(&self, _ctx: &spring_addons::AddonContext<'_, Global>, _dt: f32) {
        self.apply();
    }
}
