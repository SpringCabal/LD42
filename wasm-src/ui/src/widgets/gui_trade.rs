use core::cell::Cell;

use mycube_shared::Command;
use mycube_shared::ice::{TRADE_SIZE, ice_value, round_to_square};
use spring::gl;
use spring_addons::event::{EventResult, KeyEvent};
use spring_addons::ui::Widget;

use crate::state::Global;

const BRUSH_TEXTURE: &str = "Bitmaps/circle.png";
const VALUE_INTERVAL: f32 = 0.1;
const VALUE_REFRESH: f32 = 1.0;

const TRADE_BRUSH_FRAGMENT: &str = r#"
uniform sampler2D brushTex;

void main() {
    vec4 brushColor = texture2D(brushTex, gl_TexCoord[0].st);
    gl_FragColor = gl_Color * brushColor.a;
}
"#;

#[derive(Clone, Copy, Debug)]
struct TradePreview {
    screen_x: i32,
    screen_y: i32,
    world_x: f32,
    world_z: f32,
    value: i32,
    valued_square: Option<(f32, f32)>,
}

#[derive(Default)]
pub struct GuiTradeWidget {
    trading: Cell<bool>,
    preview: Cell<Option<TradePreview>>,
    value_age: Cell<f32>,
    brush_shader: Cell<Option<u32>>,
    brush_shader_failed: Cell<bool>,
}

impl GuiTradeWidget {
    pub fn new() -> Self {
        Self::default()
    }

    fn brush_shader(&self) -> Option<u32> {
        if let Some(shader) = self.brush_shader.get() {
            return Some(shader);
        }
        if self.brush_shader_failed.get() {
            return None;
        }
        let shader = spring::gfx::create_shader(spring::gfx::ShaderCreateParams {
            fragment: TRADE_BRUSH_FRAGMENT,
            ..Default::default()
        })
        .ok()
        .map(|shader| shader.shader_id)
        .filter(|id| *id != 0);
        self.brush_shader.set(shader);
        self.brush_shader_failed.set(shader.is_none());
        shader
    }

    fn stop_trading(&self) {
        self.trading.set(false);
        self.preview.set(None);
    }

    fn update_preview(&self, dt: f32) {
        if !self.trading.get() {
            self.preview.set(None);
            return;
        }
        let Ok(mouse) = spring::input::get_mouse_state(0) else {
            return;
        };
        if mouse.offscreen {
            self.preview.set(None);
            return;
        }
        let screen_x = mouse.x.round() as i32;
        let screen_y = mouse.y.round() as i32;
        let Some(ground) = ground_under(screen_x, screen_y) else {
            self.preview.set(None);
            return;
        };
        let last = self.preview.get();
        let mut preview = TradePreview {
            screen_x,
            screen_y,
            world_x: ground.x,
            world_z: ground.z,
            value: last.map_or(0, |last| last.value),
            valued_square: last.and_then(|last| last.valued_square),
        };
        let age = self.value_age.get() + dt;
        let square = (
            round_to_square(preview.world_x),
            round_to_square(preview.world_z),
        );
        let moved = preview.valued_square != Some(square);
        if preview.valued_square.is_none()
            || (moved && age >= VALUE_INTERVAL)
            || age >= VALUE_REFRESH
        {
            preview.value = ice_value(preview.world_x, preview.world_z, |x, z| {
                spring::get_ground_height(x, z).unwrap_or(0.0)
            }) as i32;
            preview.valued_square = Some(square);
            self.value_age.set(0.0);
        } else {
            self.value_age.set(age);
        }
        self.preview.set(Some(preview));
    }
}

fn ground_under(x: i32, y: i32) -> Option<spring::Float3> {
    spring::pick::ground_at(x as f32, y as f32)
}

impl Widget<Global> for GuiTradeWidget {
    fn name(&self) -> &'static str {
        "GuiTrade"
    }

    fn shutdown(&self, _ctx: &spring_addons::AddonContext<'_, Global>) {
        self.stop_trading();
        if let Some(shader) = self.brush_shader.take() {
            let _ = spring::gfx::delete_shader(shader);
        }
        self.brush_shader_failed.set(false);
    }

    fn update(&self, _ctx: &spring_addons::AddonContext<'_, Global>, dt: f32) {
        self.update_preview(dt);
    }

    fn draw_screen(&self, _ctx: &spring_addons::AddonContext<'_, Global>, width: i32, height: i32) {
        let Some(preview) = self.preview.get() else {
            return;
        };
        let left = (preview.screen_x as f32 + 18.0)
            .min(width as f32 - 228.0)
            .max(8.0);
        let bottom = (preview.screen_y as f32 + 18.0)
            .min(height as f32 - 64.0)
            .max(8.0);
        let _ = spring::gfx::color(0.02, 0.08, 0.14, 0.92);
        let _ = spring::gfx::rect(left, bottom, left + 220.0, bottom + 54.0);
        let _ = spring::gfx::begin_text(false);
        let _ = spring::gfx::color(0.75, 0.95, 1.0, 1.0);
        let _ = spring::gfx::text(
            &alloc::format!("Harvest: +{} money", preview.value),
            left + 10.0,
            bottom + 33.0,
            15.0,
            "o",
        );
        let _ = spring::gfx::text(
            &alloc::format!("Area: {TRADE_SIZE} x {TRADE_SIZE}"),
            left + 10.0,
            bottom + 13.0,
            13.0,
            "o",
        );
        let _ = spring::gfx::end_text(0);
    }

    fn draw_world(&self, _ctx: &spring_addons::AddonContext<'_, Global>) {
        let Some(preview) = self.preview.get() else {
            return;
        };
        let Some(shader) = self.brush_shader() else {
            return;
        };
        if spring::gfx::push_attrib(gl::ALL_ATTRIB_BITS).is_err() {
            return;
        }
        let _ = spring::gfx::bind_texture(BRUSH_TEXTURE, 0, true);
        let _ = spring::gfx::use_shader(shader);
        let _ = spring::gfx::blending(true);
        let _ = spring::gfx::blend_func(gl::SRC_ALPHA, gl::ONE);
        let half = TRADE_SIZE as f32 / 2.0;
        let _ = spring::gfx::color(0.05, 0.45, 0.95, 0.32);
        let _ = spring::gfx::draw_ground_quad(
            preview.world_x - half,
            preview.world_z - half,
            preview.world_x + half,
            preview.world_z + half,
            true,
            0.0,
            0.0,
            1.0,
            1.0,
        );
        let _ = spring::gfx::use_shader(0);
        let _ = spring::gfx::bind_texture(BRUSH_TEXTURE, 0, false);
        let _ = spring::gfx::pop_attrib(0);
    }

    fn key_press(
        &self,
        _ctx: &spring_addons::AddonContext<'_, Global>,
        event: &KeyEvent<'_>,
    ) -> EventResult {
        if event.ctrl || event.alt || event.meta || event.shift {
            return EventResult::Ignored;
        }
        let command = if event.matches("1") {
            let trading = !self.trading.get();
            self.stop_trading();
            self.trading.set(trading);
            Command::SellMode
        } else if event.matches("2") {
            self.stop_trading();
            Command::BuyHeat
        } else if event.matches("3") {
            self.stop_trading();
            Command::BuyFood
        } else {
            return EventResult::Ignored;
        };
        crate::rules::send(command);
        EventResult::Handled
    }

    fn mouse_press(
        &self,
        _ctx: &spring_addons::AddonContext<'_, Global>,
        x: i32,
        y: i32,
        button: i32,
    ) -> EventResult {
        if button != spring::mouse::LEFT || !self.trading.get() {
            return EventResult::Ignored;
        }
        let Some(ground) = ground_under(x, y) else {
            return EventResult::Handled;
        };
        crate::rules::send(Command::Trade {
            x: ground.x,
            z: ground.z,
        });
        self.stop_trading();
        EventResult::Handled
    }
}
