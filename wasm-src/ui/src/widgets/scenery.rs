use spring_addons::ui::Widget;

use crate::defs;
use crate::state::Global;

#[derive(Default)]
pub struct SceneryWidget;

impl SceneryWidget {
    pub fn new() -> Self {
        Self
    }
}

fn make_unselectable(spring: &spring::Spring<'_, spring::Ui>, unit_id: i32, def: i32) {
    if defs::is_scenery(spring, def) {
        let _ = spring.unsynced_ctrl().set_unit_no_select(unit_id, true);
        let _ = spring.unsynced_ctrl().set_unit_no_minimap(unit_id, true);
    }
}

impl Widget<Global> for SceneryWidget {
    fn name(&self) -> &'static str {
        "Scenery"
    }

    fn unit_created(
        &self,
        _ctx: &spring_addons::AddonContext<'_, Global, spring::Ui>,
        unit_id: i32,
        def: i32,
        _team: i32,
        _builder: i32,
    ) {
        make_unselectable(_ctx.spring(), unit_id, def);
    }

    fn unit_entered_los(
        &self,
        _ctx: &spring_addons::AddonContext<'_, Global, spring::Ui>,
        unit_id: i32,
        def: i32,
        _team: i32,
        _ally_team: i32,
    ) {
        make_unselectable(_ctx.spring(), unit_id, def);
    }
}
