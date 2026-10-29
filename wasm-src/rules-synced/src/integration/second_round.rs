use super::*;

impl Tracker {
    pub(super) fn update_second_round(
        &mut self,
        spring: &spring::Spring<'_, spring::RulesSynced>,
        global: &Global,
        frame: i32,
        since: i32,
    ) {
        let Some(start) = self.restart.or_else(|| {
            let story = global.story.access();
            (story.started && story.start_frame > since).then_some(story.start_frame)
        }) else {
            if timed_out(spring, frame, since, "the UI's restart request") {
                self.check(spring, false, "the UI's restart request reaches the rules");
                self.suite.finish(spring);
            }
            return;
        };
        self.restart = Some(start);
        match frame - start {
            RESTART_CHECK => self.check_restart(spring, global, start),
            RESTART_CONDEMN => {
                for id in eskimo_ids(global) {
                    crate::with_addon_context(|ctx| crate::units::destroy(ctx, id, false));
                }
            }
            RESTART_LOSS_CHECK => {
                let (end, survived) = {
                    let story = global.story.access();
                    (story.game_end, story.survival_frames())
                };
                self.check(
                    spring,
                    end == Some(GameEnd::Loss),
                    "losing every Eskimo ends the round in a loss",
                );
                self.check(
                    spring,
                    survived == Some(RESTART_CONDEMN + 1),
                    "the survival time counts only the restarted round",
                );
                self.suite.finish(spring);
            }
            _ => {}
        }
    }

    pub(super) fn check_restart(
        &mut self,
        spring: &spring::Spring<'_, spring::RulesSynced>,
        global: &Global,
        start: i32,
    ) {
        let (end, step) = {
            let story = global.story.access();
            (story.game_end, story.step)
        };
        let economy = *global.economy.access();
        let tracked = eskimo_ids(global).len();
        self.check(
            spring,
            end.is_none() && step >= 1,
            "a restart clears the finished round and starts the story over",
        );
        // GetAllUnits retains killed units until their death sequence finishes.
        // The restart must replace living actors; a dying enemy from the first
        // round must not make this depend on when the UI requests the restart.
        let living = |kind| {
            units_of(spring, kind)
                .into_iter()
                .filter(|id| crate::units::is_alive(spring, *id))
                .count()
        };
        self.check(
            spring,
            tracked == 10
                && living(UnitKind::Eskimo) == 10
                && living(UnitKind::Drillship) == 0
                && living(UnitKind::Pirate) == 0,
            "a restart creates ten live Eskimos and removes live enemies",
        );
        self.check(
            spring,
            opening_stocks(&economy),
            "a restart refills the village's stocks",
        );
        self.check(
            spring,
            self.trade_before.is_some_and(|(x, z, before)| {
                spring
                    .terrain()
                    .get_ground_height(x, z)
                    .is_ok_and(|now| (now - before).abs() < 0.1)
            }),
            "a restart restores the ice the first round sold",
        );
        spring::log_info!(
            spring,
            "mycube-test",
            "the second round started at frame {start}"
        );
    }
}
