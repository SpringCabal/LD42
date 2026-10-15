pub const HEALTH: &str = "health";
pub const FOOD: &str = "food";
pub const HEAT: &str = "heat";
pub const UNTARGETABLE: &str = "untargetable";

pub const MAX_NEED: f32 = 100.0;

pub const DEFAULT_HEALTH: f32 = 100_000.0;

pub fn custom_param_number(value: Option<&str>) -> Option<f32> {
    value?.trim().parse().ok()
}

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
