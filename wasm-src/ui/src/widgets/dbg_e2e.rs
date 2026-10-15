use springcabal::shots::{Clock, Screenshots};

const SHOT_FRAMES: [u32; 5] = [25, 390, 800, 1500, 2000];
const QUIT_DELAY_FRAMES: u32 = 35;

pub fn dbg_e2e() -> Screenshots {
    Screenshots::new(Clock::GameFrame, &SHOT_FRAMES)
        .done_marker("MYCUBE VISUAL E2E PASSED")
        .quit_after(QUIT_DELAY_FRAMES)
}
