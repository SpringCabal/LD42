use springcabal::shots::{Clock, Screenshots};

/// Game frames of the visual suite's screenshots.
const SHOT_FRAMES: [u32; 5] = [25, 90, 210, 390, 530];
/// How long the engine keeps running after the last screenshot, so it is
/// written before the quit.
const QUIT_DELAY_FRAMES: u32 = 35;

/// The visual suite's screenshots; quits the engine once they are taken.
pub fn dbg_e2e() -> Screenshots {
    Screenshots::new(Clock::GameFrame, &SHOT_FRAMES)
        .done_marker("MYCUBE VISUAL E2E PASSED")
        .quit_after(QUIT_DELAY_FRAMES)
}
