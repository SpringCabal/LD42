use springcabal::shots::{Clock, Screenshots};

/// Game frames of the visual suite's screenshots, spread over the story:
/// the landing, "Survive.", the first pirate on the ice, the drill ships'
/// arrival and the raid after it.
const SHOT_FRAMES: [u32; 5] = [25, 390, 800, 1500, 2000];
/// How long the engine keeps running after the last screenshot, so it is
/// written before the quit.
const QUIT_DELAY_FRAMES: u32 = 35;

/// The visual suite's screenshots; quits the engine once they are taken.
pub fn dbg_e2e() -> Screenshots {
    Screenshots::new(Clock::GameFrame, &SHOT_FRAMES)
        .done_marker("MYCUBE VISUAL E2E PASSED")
        .quit_after(QUIT_DELAY_FRAMES)
}
