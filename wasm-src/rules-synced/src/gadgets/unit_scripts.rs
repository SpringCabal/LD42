use spring::{ResultExt, UnitId};
use spring_addons::rules::Gadget;

use crate::state::Global;

#[derive(Default)]
pub struct UnitScriptsGadget;

impl UnitScriptsGadget {
    pub fn new() -> Self {
        Self
    }
}

impl Gadget<Global> for UnitScriptsGadget {
    fn name(&self) -> &'static str {
        "UnitScripts"
    }

    fn init(&self, _ctx: &spring_addons::AddonContext<'_, Global>) {
        crate::with_cus_module(|host, spring| host.attach_existing(spring))
            .log_err_once(_ctx.spring(), "attaching existing unit scripts failed");
    }

    fn unit_created(
        &self,
        _ctx: &spring_addons::AddonContext<'_, Global>,
        id: i32,
        def: i32,
        _team: i32,
        _builder: i32,
    ) {
        crate::with_cus_module(move |host, spring| {
            host.unit_created(spring, UnitId(id), def)?;
            // Match the Lua script loader's synchronous Create boundary. The
            // engine may otherwise defer CUS Create until the import returns.
            host.with_script(spring, UnitId(id), |_, _| ())?;
            Ok(())
        })
        .log_err_once(_ctx.spring(), "attaching a unit script failed");
    }

    fn game_frame(&self, _ctx: &spring_addons::AddonContext<'_, Global>, frame: i32) {
        crate::with_cus_module(move |host, _| {
            host.game_frame(frame);
            Ok(())
        })
        .log_err_once(_ctx.spring(), "updating unit scripts failed");
    }
}
