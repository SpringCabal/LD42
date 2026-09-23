use core::cell::Cell;

use spring_addons::ui::Widget;

use crate::state::Global;

const MUSIC_FILE: &str = "sounds/remixed_background.ogg";
const MUSIC_VOLUME: f32 = 0.15;
const MUSIC_DURATION: f32 = 91.008;
const LOOP_BUFFER: f32 = 0.015;
const LOG_SECTION: &str = "mycube-ui";

/// The Lua archive's `snd_music.lua` equivalent.
#[derive(Default)]
pub struct SndMusicWidget {
    started: Cell<bool>,
    start_failure_logged: Cell<bool>,
    elapsed: Cell<f32>,
    duration: Cell<f32>,
}

impl SndMusicWidget {
    pub fn new() -> Self {
        Self::default()
    }

    fn start(&self) -> bool {
        match spring::sound::play_sound_stream(MUSIC_FILE, MUSIC_VOLUME, false) {
            Ok(true) => {
                let duration = spring::sound::get_sound_stream_time(0)
                    .ok()
                    .filter(|duration| *duration > 0.0)
                    .unwrap_or(MUSIC_DURATION);
                self.started.set(true);
                self.elapsed.set(0.0);
                self.duration.set(duration);
                spring::log_info!(LOG_SECTION, "MyCube music stream started");
                true
            }
            Ok(false) | Err(_) => {
                self.started.set(false);
                if !self.start_failure_logged.replace(true) {
                    spring::log_warning!(LOG_SECTION, "MyCube music stream could not be started");
                }
                false
            }
        }
    }
}

impl Widget<Global> for SndMusicWidget {
    fn name(&self) -> &'static str {
        "snd_music"
    }

    fn update(&self, _ctx: &spring_addons::AddonContext<'_, Global>, dt: f32) {
        if !self.started.get() {
            self.start();
            return;
        }

        let elapsed = self.elapsed.get() + dt.max(0.0);
        self.elapsed.set(elapsed);
        let duration = self.duration.get();
        if duration > LOOP_BUFFER && elapsed > duration - LOOP_BUFFER {
            self.start();
        }
    }

    fn shutdown(&self, _ctx: &spring_addons::AddonContext<'_, Global>) {
        let _ = spring::sound::stop_sound_stream(0);
        self.started.set(false);
    }
}
