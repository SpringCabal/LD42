//! The per-unit state the rules publish as public unit rules params, and the
//! unit-def custom params both modules read.

/// The gameplay health pool (`game_health.lua`); engine health is not used.
pub const HEALTH: &str = "health";
/// An Eskimo's food, 0 to [`MAX_NEED`] (`unit_eskimos.lua`).
pub const FOOD: &str = "food";
/// An Eskimo's warmth, 0 to [`MAX_NEED`].
pub const HEAT: &str = "heat";
/// 1 on scenery, which commands can't target (`unit_untargetable.lua`).
pub const UNTARGETABLE: &str = "untargetable";

/// The most food or heat an Eskimo can have.
pub const MAX_NEED: f32 = 100.0;

/// `game_health.lua`'s pool for defs without `customParams.health`.
pub const DEFAULT_HEALTH: f32 = 100_000.0;

/// A numeric unit-def custom param, as Lua's `tonumber` reads it; `None`
/// when it is missing or not a number.
pub fn custom_param_number(value: Option<&str>) -> Option<f32> {
    value?.trim().parse().ok()
}

/// A def's full gameplay health pool, from its `customParams.health`.
pub fn max_health(custom_param: Option<&str>) -> f32 {
    custom_param_number(custom_param).unwrap_or(DEFAULT_HEALTH)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn health_comes_from_the_custom_param() {
        assert_eq!(max_health(Some("100")), 100.0);
        assert_eq!(max_health(Some(" 12.5 ")), 12.5);
        assert_eq!(max_health(Some("lots")), DEFAULT_HEALTH);
        assert_eq!(max_health(None), DEFAULT_HEALTH);
        assert_eq!(custom_param_number(Some("3")), Some(3.0));
        assert_eq!(custom_param_number(None), None);
    }
}
