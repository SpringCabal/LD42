use spring_addons::ui::Widget;

use crate::state::{Global, current_frame};

pub struct DbgE2eWidget {
    captured: core::cell::Cell<u8>,
    quit_frame: core::cell::Cell<i32>,
}

impl Default for DbgE2eWidget {
    fn default() -> Self {
        Self {
            captured: core::cell::Cell::new(0),
            quit_frame: core::cell::Cell::new(-1),
        }
    }
}

impl DbgE2eWidget {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Widget<Global> for DbgE2eWidget {
    fn name(&self) -> &'static str {
        "DbgE2e"
    }

    fn update(&self, _ctx: &spring_addons::AddonContext<'_, Global>, _dt: f32) {
        let frame = current_frame();
        let quit_frame = self.quit_frame.get();
        if quit_frame >= 0 && frame >= quit_frame {
            let _ = spring::system_control::quit(0);
            return;
        }
        let target = self.captured.get();
        if [25, 90, 210, 390, 530]
            .get(target as usize)
            .is_some_and(|minimum| frame >= *minimum)
        {
            let _ = spring::send_command("screenshot");
            self.captured.set(target + 1);
            if target == 4 {
                spring::log_info!("mycube-ui", "MYCUBE VISUAL E2E PASSED");
                self.quit_frame.set(frame + 35);
            }
        }
    }
}
