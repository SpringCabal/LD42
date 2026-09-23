//! The Core counterpart of `unit_untargetable.lua`.

/// Targeted Spring commands carry exactly one unit ID; positional commands
/// carry x/y/z.
pub fn allow_command(command_id: i32, command_params: &[f32]) -> bool {
    let target_id = if command_id == crate::cmd::INSERT && command_params.len() >= 4 {
        // INSERT carries the inserted command's first parameter at index 3.
        command_params[3] as i32
    } else if command_params.len() == 1 {
        command_params[0] as i32
    } else {
        return true;
    };
    spring::get_unit_rules_param_f32(target_id, "untargetable") != Ok(1.0)
}
