use spring::gl;
use spring_addons::event::{EventResult, KeyEvent};
use spring_addons::ui::Widget;

use crate::state::{Global, TradePreview};

const TRADE_SIZE: f32 = 300.0;
/// `TraceScreenRay` hit type for the ground.
const HIT_GROUND: i32 = 3;

/// Player controls and their on-screen legend, ported from `keybinds.lua`.
const TRADE_BRUSH_FRAGMENT: &str = r#"
uniform sampler2D brushTex;

void main() {
    vec4 brushColor = texture2D(brushTex, gl_TexCoord[0].st);
    gl_FragColor = gl_Color * brushColor.a;
}
"#;

pub struct GuiKeybindsWidget {
    brush_shader: core::cell::Cell<Option<u32>>,
    brush_shader_failed: core::cell::Cell<bool>,
}

impl Default for GuiKeybindsWidget {
    fn default() -> Self {
        Self {
            brush_shader: core::cell::Cell::new(None),
            brush_shader_failed: core::cell::Cell::new(false),
        }
    }
}

impl GuiKeybindsWidget {
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

        let Ok(shader) = spring::gfx::create_shader(spring::gfx::ShaderCreateParams {
            fragment: TRADE_BRUSH_FRAGMENT,
            ..Default::default()
        }) else {
            self.brush_shader_failed.set(true);
            return None;
        };
        if shader.shader_id == 0 {
            self.brush_shader_failed.set(true);
            return None;
        }
        self.brush_shader.set(Some(shader.shader_id));
        Some(shader.shader_id)
    }

    fn update_preview(&self, ctx: &spring_addons::AddonContext<'_, Global>) {
        if !ctx.global().interaction.access().trading {
            ctx.global().interaction.access_mut().preview = None;
            return;
        }
        let Ok(mouse) = spring::input::get_mouse_state(0) else {
            return;
        };
        if mouse.offscreen {
            ctx.global().interaction.access_mut().preview = None;
            return;
        }
        let screen_x = mouse.x.round() as i32;
        let screen_y = mouse.y.round() as i32;
        if ctx
            .global()
            .interaction
            .access()
            .preview
            .is_some_and(|preview| preview.screen_x == screen_x && preview.screen_y == screen_y)
        {
            return;
        }

        let Ok(trace) = spring::camera::trace_screen_ray(
            screen_x as f32,
            screen_y as f32,
            spring::camera::TraceScreenRayOptions {
                only_coords: true,
                use_minimap: false,
                include_sky: false,
                ignore_water: false,
                height_offset: 0.0,
            },
        ) else {
            ctx.global().interaction.access_mut().preview = None;
            return;
        };
        if trace.hit_type != HIT_GROUND {
            ctx.global().interaction.access_mut().preview = None;
            return;
        }

        let value = crate::ice::ice_value(trace.hit_pos.x, trace.hit_pos.z) as i32;
        ctx.global().interaction.access_mut().preview = Some(TradePreview {
            screen_x,
            screen_y,
            world_x: trace.hit_pos.x,
            world_y: trace.hit_pos.y,
            world_z: trace.hit_pos.z,
            value,
        });
    }
}

impl Widget<Global> for GuiKeybindsWidget {
    fn name(&self) -> &'static str {
        "GuiKeybinds"
    }

    fn update(&self, ctx: &spring_addons::AddonContext<'_, Global>, _dt: f32) {
        self.update_preview(ctx);
    }

    fn draw_screen(&self, ctx: &spring_addons::AddonContext<'_, Global>, width: i32, height: i32) {
        // The strings from keybinds.lua, without its Chili colour escapes.
        let _ = spring::gfx::begin_text(false);
        let _ = spring::gfx::color(1.0, 1.0, 1.0, 1.0);
        for (line, text) in [
            "Ctrl+Q : Quit",
            "1 (+ left mouse): sell ice",
            "2 (need 50 money): buy heating resource",
            "3 (need 50 money): buy food",
        ]
        .into_iter()
        .enumerate()
        {
            let _ = spring::gfx::text(text, 10.0, 80.0 - line as f32 * 20.0, 14.0, "o");
        }
        let _ = spring::gfx::end_text(0);

        let interaction = ctx.global().interaction.access();
        let Some(preview) = interaction.preview else {
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

    fn draw_world(&self, ctx: &spring_addons::AddonContext<'_, Global>) {
        let interaction = ctx.global().interaction.access();
        let Some(preview) = interaction.preview else {
            return;
        };

        // game_trade.lua's 300x300 ground quad, masked by Bitmaps/circle.png.
        let Some(shader) = self.brush_shader() else {
            return;
        };
        let _ = spring::gfx::bind_texture("Bitmaps/circle.png", 0, true);
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
        let _ = spring::gfx::bind_texture("Bitmaps/circle.png", 0, false);
        let _ = spring::gfx::blend_func(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        let _ = spring::gfx::color(1.0, 1.0, 1.0, 1.0);
        let _ = spring::gfx::blending(false);
    }

    fn key_press(
        &self,
        ctx: &spring_addons::AddonContext<'_, Global>,
        event: &KeyEvent<'_>,
    ) -> EventResult {
        if event.ctrl && event.matches("r") {
            // Match ui_endgame.lua's restart action.  The cheat toggle is
            // needed because the reload command is protected in the engine;
            // turn it back off immediately after the rules reload.
            let _ = spring::send_commands("cheat", "");
            let _ = spring::send_commands("luarules reload", "");
            let _ = spring::send_commands("cheat", "");
            return EventResult::Handled;
        }
        if event.ctrl && event.matches("q") {
            // Same as the Lua keybind: reload into the menu in-process.
            let _ = spring::send_commands("reloadforce", "");
            return EventResult::Handled;
        }
        let message = if event.matches("1") {
            // ui_modes.lua toggles MODE 1; keep the local click-mode in
            // lock step so pressing 1 again cancels selling instead of
            // leaving a stale left-click action armed.
            let mut interaction = ctx.global().interaction.access_mut();
            interaction.trading = !interaction.trading;
            interaction.preview = None;
            "MODE|1"
        } else if event.matches("2") {
            let mut interaction = ctx.global().interaction.access_mut();
            interaction.trading = false;
            interaction.preview = None;
            "MODE|2"
        } else if event.matches("3") {
            let mut interaction = ctx.global().interaction.access_mut();
            interaction.trading = false;
            interaction.preview = None;
            "MODE|3"
        } else {
            return EventResult::Ignored;
        };
        let _ = spring::send_lua_rules_msg(message);
        EventResult::Handled
    }

    fn mouse_press(
        &self,
        ctx: &spring_addons::AddonContext<'_, Global>,
        x: i32,
        y: i32,
        button: i32,
    ) -> EventResult {
        if button != 1 {
            return EventResult::Ignored;
        }
        if !ctx.global().interaction.access().trading {
            // Leave normal clicks to Spring's selection handler.
            return EventResult::Ignored;
        }
        let Ok(trace) = spring::camera::trace_screen_ray(
            x as f32,
            y as f32,
            spring::camera::TraceScreenRayOptions {
                only_coords: true,
                use_minimap: false,
                include_sky: false,
                ignore_water: false,
                height_offset: 0.0,
            },
        ) else {
            return EventResult::Handled;
        };
        if trace.hit_type != HIT_GROUND {
            return EventResult::Handled;
        }
        let position = trace.hit_pos;
        let _ = spring::send_lua_rules_msg(&alloc::format!(
            "TRADE|{:.1}|{:.1}",
            position.x,
            position.z
        ));
        let mut interaction = ctx.global().interaction.access_mut();
        interaction.trading = false;
        interaction.preview = None;
        EventResult::Handled
    }
}
