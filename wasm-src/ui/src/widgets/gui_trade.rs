use core::cell::Cell;

use spring::gl;
use spring_addons::event::{EventResult, KeyEvent};
use spring_addons::ui::Widget;

use crate::state::Global;

/// Side of the square of ice a trade harvests.
const TRADE_SIZE: f32 = 300.0;
/// `TraceScreenRay` hit type for the ground.
const HIT_GROUND: i32 = 3;
const BRUSH_TEXTURE: &str = "Bitmaps/circle.png";

const TRADE_BRUSH_FRAGMENT: &str = r#"
uniform sampler2D brushTex;

void main() {
    vec4 brushColor = texture2D(brushTex, gl_TexCoord[0].st);
    gl_FragColor = gl_Color * brushColor.a;
}
"#;

/// The ice trade under the cursor.
#[derive(Clone, Copy, Debug)]
struct TradePreview {
    screen_x: i32,
    screen_y: i32,
    world_x: f32,
    world_z: f32,
    value: i32,
}

/// The trade controls: keys 1-3 (`ui_modes.lua`) and, while selling, the
/// harvest preview and click (`game_trade.lua`).
#[derive(Default)]
pub struct GuiTradeWidget {
    trading: Cell<bool>,
    preview: Cell<Option<TradePreview>>,
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

    fn update_preview(&self) {
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
        if self
            .preview
            .get()
            .is_some_and(|preview| preview.screen_x == screen_x && preview.screen_y == screen_y)
        {
            return;
        }
        self.preview
            .set(ground_under(screen_x, screen_y).map(|ground| TradePreview {
                screen_x,
                screen_y,
                world_x: ground.x,
                world_z: ground.z,
                value: crate::ice::ice_value(ground.x, ground.z) as i32,
            }));
    }
}

/// The ground point under a screen position, if the cursor is over ground.
fn ground_under(x: i32, y: i32) -> Option<spring::Float3> {
    let trace = spring::camera::trace_screen_ray(
        x as f32,
        y as f32,
        spring::camera::TraceScreenRayOptions {
            only_coords: true,
            use_minimap: false,
            include_sky: false,
            ignore_water: false,
            height_offset: 0.0,
        },
    )
    .ok()?;
    (trace.hit_type == HIT_GROUND).then_some(trace.hit_pos)
}

impl Widget<Global> for GuiTradeWidget {
    fn name(&self) -> &'static str {
        "GuiTrade"
    }

    fn update(&self, _ctx: &spring_addons::AddonContext<'_, Global>, _dt: f32) {
        self.update_preview();
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
        let _ = spring::gfx::text("Area: 300 x 300", left + 10.0, bottom + 13.0, 13.0, "o");
        let _ = spring::gfx::end_text(0);
    }

    fn draw_world(&self, _ctx: &spring_addons::AddonContext<'_, Global>) {
        let Some(preview) = self.preview.get() else {
            return;
        };
        // game_trade.lua's 300x300 ground quad, masked by Bitmaps/circle.png.
        let Some(shader) = self.brush_shader() else {
            return;
        };
        let _ = spring::gfx::bind_texture(BRUSH_TEXTURE, 0, true);
        let _ = spring::gfx::use_shader(shader);
        let _ = spring::gfx::blending(true);
        let _ = spring::gfx::blend_func(gl::SRC_ALPHA, gl::ONE);
        let half = TRADE_SIZE / 2.0;
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
        let _ = spring::gfx::blend_func(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        let _ = spring::gfx::color(1.0, 1.0, 1.0, 1.0);
        let _ = spring::gfx::blending(false);
    }

    fn key_press(
        &self,
        _ctx: &spring_addons::AddonContext<'_, Global>,
        event: &KeyEvent<'_>,
    ) -> EventResult {
        let message = if event.matches("1") {
            // ui_modes.lua toggles MODE 1; pressing 1 again cancels selling.
            let trading = !self.trading.get();
            self.stop_trading();
            self.trading.set(trading);
            "MODE|1"
        } else if event.matches("2") {
            self.stop_trading();
            "MODE|2"
        } else if event.matches("3") {
            self.stop_trading();
            "MODE|3"
        } else {
            return EventResult::Ignored;
        };
        let _ = spring::send_lua_rules_msg(message);
        EventResult::Handled
    }

    fn mouse_press(
        &self,
        _ctx: &spring_addons::AddonContext<'_, Global>,
        x: i32,
        y: i32,
        button: i32,
    ) -> EventResult {
        if button != 1 || !self.trading.get() {
            // Leave normal clicks to Spring's selection handler.
            return EventResult::Ignored;
        }
        let Some(ground) = ground_under(x, y) else {
            return EventResult::Handled;
        };
        let _ =
            spring::send_lua_rules_msg(&alloc::format!("TRADE|{:.1}|{:.1}", ground.x, ground.z));
        self.stop_trading();
        EventResult::Handled
    }
}
