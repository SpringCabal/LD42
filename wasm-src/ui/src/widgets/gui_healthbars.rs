use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use core::cell::{Cell, RefCell};

use mycube_shared::unit;
use spring_addons::ui::Widget;

use crate::defs::{self, UnitKind};

use crate::state::Global;

const BAR_WIDTH: f32 = 46.0;
const BAR_HEIGHT: f32 = 4.0;
const BAR_GAP: f32 = 2.0;
const UNIT_HEIGHT_SCALE: f32 = 1.8;
const INFO_DISTANCE_SQ: f32 = 800_000.0;

#[derive(Default)]
pub struct GuiHealthbarsWidget {
    tracked: RefCell<BTreeMap<i32, Option<UnitKind>>>,
    bars: RefCell<Vec<(i32, UnitKind)>>,
    listed_frame: Cell<Option<i32>>,
}

impl GuiHealthbarsWidget {
    pub fn new() -> Self {
        Self::default()
    }

    fn refresh_units(&self, spring: &spring::Spring<'_, spring::Ui>) {
        let frame = spring.game().current_frame().unwrap_or(0);
        if self.listed_frame.get() == Some(frame) {
            return;
        }
        self.listed_frame.set(Some(frame));
        let mut units = spring.units().get_all_units().unwrap_or_default();
        units.sort_unstable();
        let mut tracked = self.tracked.borrow_mut();
        tracked.retain(|id, _| units.binary_search(id).is_ok());
        let mut bars = self.bars.borrow_mut();
        bars.clear();
        for unit_id in units {
            let kind = *tracked.entry(unit_id).or_insert_with(|| {
                spring
                    .units_info()
                    .get_unit_def_id(unit_id)
                    .ok()
                    .and_then(|def| defs::kind(spring, def))
            });
            if let Some(kind) = kind {
                bars.push((unit_id, kind));
            }
        }
    }

    fn draw_bar(
        &self,
        spring: &spring::Spring<'_, spring::Ui>,
        x: f32,
        y: f32,
        value: f32,
        maximum: f32,
        color: (f32, f32, f32),
    ) {
        let fraction = (value / maximum.max(1.0)).clamp(0.0, 1.0);
        let left = x - BAR_WIDTH / 2.0;
        let _ = spring.gfx().color(0.06, 0.06, 0.06, 0.9);
        let _ = spring.gfx().rect(left, y, left + BAR_WIDTH, y + BAR_HEIGHT);
        let _ = spring.gfx().color(color.0, color.1, color.2, 0.95);
        let _ = spring
            .gfx()
            .rect(left, y, left + BAR_WIDTH * fraction, y + BAR_HEIGHT);
    }

    fn health_color(&self, fraction: f32) -> (f32, f32, f32) {
        let fraction = fraction.clamp(0.0, 1.0);
        if fraction < 0.5 {
            (1.0, fraction * 2.0, 0.0)
        } else {
            ((1.0 - fraction) * 2.0, 1.0, 0.0)
        }
    }
}

impl Widget<Global> for GuiHealthbarsWidget {
    fn name(&self) -> &'static str {
        "GuiHealthbars"
    }

    fn draw_screen(
        &self,
        _ctx: &spring_addons::AddonContext<'_, Global, spring::Ui>,
        width: i32,
        height: i32,
    ) {
        let spring = _ctx.spring();
        self.refresh_units(spring);
        let Ok(camera) = spring.camera().get_camera_position() else {
            return;
        };
        for &(unit_id, kind) in self.bars.borrow().iter() {
            let radius = spring.units_info().get_unit_radius(unit_id).unwrap_or(20.0);
            let Ok(position) = spring
                .unit_rendering()
                .get_unit_view_position(unit_id, false)
            else {
                continue;
            };
            let (dx, dy, dz) = (
                position.x - camera.x,
                position.y - camera.y,
                position.z - camera.z,
            );
            if dx * dx + dy * dy + dz * dz > INFO_DISTANCE_SQ {
                continue;
            }
            let Ok(projected) = spring.camera().world_to_screen_coords(spring::Float3::new(
                position.x,
                position.y + UNIT_HEIGHT_SCALE * radius,
                position.z,
            )) else {
                continue;
            };
            let screen = projected.screen_pos;
            if !projected.valid
                || screen.x < -BAR_WIDTH
                || screen.x > width as f32 + BAR_WIDTH
                || screen.y < -50.0
                || screen.y > height as f32 + 50.0
            {
                continue;
            }

            let health = spring
                .rules_params()
                .get_unit_rules_param_f32(unit_id, unit::HEALTH)
                .unwrap_or(0.0);
            let max_health = kind.max_health(spring);
            self.draw_bar(
                spring,
                screen.x,
                screen.y + 20.0,
                health,
                max_health,
                self.health_color(health / max_health),
            );
            if kind == UnitKind::Eskimo {
                let food = spring
                    .rules_params()
                    .get_unit_rules_param_f32(unit_id, unit::FOOD)
                    .unwrap_or(0.0);
                self.draw_bar(
                    spring,
                    screen.x,
                    screen.y + 20.0 - BAR_HEIGHT - BAR_GAP,
                    food,
                    unit::MAX_NEED,
                    (0.0, 1.0, 0.0),
                );
                let heat = spring
                    .rules_params()
                    .get_unit_rules_param_f32(unit_id, unit::HEAT)
                    .unwrap_or(0.0);
                self.draw_bar(
                    spring,
                    screen.x,
                    screen.y + 20.0 - 2.0 * (BAR_HEIGHT + BAR_GAP),
                    heat,
                    unit::MAX_NEED,
                    (0.5, 0.3, 0.1),
                );
            }
        }
        let _ = spring.gfx().color(1.0, 1.0, 1.0, 1.0);
    }
}
