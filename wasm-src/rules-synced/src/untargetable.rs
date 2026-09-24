//! The Core counterpart of `unit_untargetable.lua`.

/// Targeted Spring commands carry exactly one parameter, the unit id; an
/// INSERT carries the inserted command's parameters after its own three.
pub fn allow_command(command_id: i32, command_params: &[f32]) -> bool {
    let target = match command_params {
        [_, _, _, target] if command_id == spring::cmd::INSERT => *target,
        [target] if command_id != spring::cmd::INSERT => *target,
        _ => return true,
    };
    spring::get_unit_rules_param_f32(target as i32, "untargetable") != Ok(1.0)
}
