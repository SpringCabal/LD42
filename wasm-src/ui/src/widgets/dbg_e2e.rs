use core::cell::Cell;

use spring_addons::ui::Widget;
use springcabal::shots::{Clock, Screenshots};

use crate::state::Global;

/// Game frames of the visual suite's screenshots.
const SHOT_FRAMES: [u32; 5] = [25, 90, 210, 390, 530];
/// How long the engine keeps running after the last screenshot, so it is
/// written before the quit.
const QUIT_DELAY_FRAMES: i32 = 35;

/// The visual suite's screenshots; quits the engine once they are taken.
pub struct DbgE2eWidget {
    shots: Screenshots,
    quit_frame: Cell<Option<i32>>,
}

impl Default for DbgE2eWidget {
    fn default() -> Self {
        Self {
            shots: Screenshots::new(Clock::GameFrame, &SHOT_FRAMES)
                .done_marker("MYCUBE VISUAL E2E PASSED"),
            quit_frame: Cell::new(None),
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

    fn update(&self, ctx: &spring_addons::AddonContext<'_, Global>, dt: f32) {
        let frame = spring::game::current_frame();
        match self.quit_frame.get() {
            Some(quit_frame) if frame >= quit_frame => {
                let _ = spring::system_control::quit(0);
            }
            Some(_) => {}
            // Before the game starts `spring::game::frame` reports -1 as
            // `u32::MAX`, which the schedule would take as past every shot.
            None if frame < 0 => {}
            None => {
                self.shots.update(ctx, dt);
                if self.shots.is_done() {
                    self.quit_frame.set(Some(frame + QUIT_DELAY_FRAMES));
                }
            }
        }
    }
}
