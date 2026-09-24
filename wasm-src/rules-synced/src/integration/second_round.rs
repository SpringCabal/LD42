//! The second round, after the UI's restart request.

use super::*;

impl Tracker {
    /// The UI's test driver restarts the round the way Ctrl+R does. Check
    /// the new round, then lose it.
    pub(super) fn update_second_round(&mut self, global: &Global, frame: i32) {
        let Some(start) = self.restart.or_else(|| {
            let story = global.story.access();
            (story.started && story.start_frame > 1000).then_some(story.start_frame)
        }) else {
            if frame > RESTART_DEADLINE {
                self.check(false, "the UI's restart request reaches the rules");
                self.suite.finish();
            }
            return;
        };
        self.restart = Some(start);
        match frame - start {
            RESTART_CHECK => self.check_restart(global, start),
            RESTART_CONDEMN => {
                for id in eskimo_ids(global) {
                    crate::units::destroy(id, false);
                }
            }
            RESTART_LOSS_CHECK => {
                let (end, survived) = {
                    let story = global.story.access();
                    (story.game_end, story.survival_frames())
                };
                self.check(
                    end == Some(GameEnd::Loss),
                    "losing every Eskimo ends the round in a loss",
                );
                // The Eskimos die on RESTART_CONDEMN; the loss is called on
                // the next frame.
                self.check(
                    survived == Some(RESTART_CONDEMN + 1),
                    "the survival time counts only the restarted round",
                );
                self.suite.finish();
            }
            _ => {}
        }
    }

    pub(super) fn check_restart(&mut self, global: &Global, start: i32) {
        let (end, step) = {
            let story = global.story.access();
            (story.game_end, story.step)
        };
        let economy = *global.economy.access();
        let tracked = eskimo_ids(global).len();
        self.check(
            end.is_none() && step >= 1,
            "a restart clears the finished round and starts the story over",
        );
        self.check(
            tracked == 10
                && units_of(UnitKind::Eskimo).len() == 10
                && units_of(UnitKind::Drillship).is_empty()
                && units_of(UnitKind::Pirate).is_empty(),
            "a restart replaces every unit with a fresh expedition",
        );
        self.check(
            opening_stocks(&economy),
            "a restart refills the village's stocks",
        );
        self.check(
            self.trade_before.is_some_and(|(x, z, before)| {
                spring::get_ground_height(x, z).is_ok_and(|now| (now - before).abs() < 0.1)
            }),
            "a restart restores the ice the first round sold",
        );
        spring::log_info!("mycube-test", "the second round started at frame {start}");
    }
}
