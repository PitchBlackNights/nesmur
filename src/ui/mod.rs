pub mod frame;

use crate::{
    gl_error,
    shared_ctx::{app::SharedAppCtx, window::*},
    thread_com::{ThreadCom, ThreadComError, ThreadMsg},
    NESState,
};
use glow::HasContext;
use glutin::{
    context::PossiblyCurrentContext,
    surface::{Surface, WindowSurface},
};
use imgui::{ColorStackToken, Condition, StyleColor, StyleStackToken, StyleVar, Ui};
use imgui_glow_renderer::Renderer;
use imgui_winit_support::WinitPlatform;
use nes::{SCREEN_HEIGHT, SCREEN_WIDTH};
use winit::window::Window;

#[derive(Debug)]
pub struct NesmurUI {
    pub win_ctx: SharedWindowCtx,
    pub thread_com: ThreadCom,
    pub nes_game_window: NESGameWindow,
    pub app: SharedAppCtx,
}

impl NesmurUI {
    pub fn new() -> Self {
        NesmurUI {
            win_ctx: SharedWindowCtx::default(),
            thread_com: ThreadCom::new(),
            nes_game_window: NESGameWindow::new(),
            app: SharedAppCtx::default(),
        }
    }

    pub fn init(&mut self) {
        self.nes_game_window.generate(
            get_from_swc!(self.win_ctx.opengl),
            get_from_swc!(mut self.win_ctx.imgui_textures),
        );
    }

    pub fn redraw(&self) -> &mut Ui {
        let framerate: f32 = self.imgui_context().io().framerate;

        let ui: &mut Ui = self.imgui_context_mut().frame();
        let style: ColorStackToken<'_> =
            ui.push_style_color(StyleColor::WindowBg, [0.0, 0.0, 0.0, 1.0]);
        let style2: ColorStackToken<'_> =
            ui.push_style_color(StyleColor::TitleBgCollapsed, [0.0, 0.0, 0.0, 1.0]);
        let style3: ColorStackToken<'_> =
            ui.push_style_color(StyleColor::WindowBg, [0.15, 0.15, 0.15, 1.0]);

        self.nes_game_window.show(ui);
        self.show_control_panel(ui, framerate);
        // OTHER WINDOWS

        style.pop();
        style2.pop();
        style3.pop();

        ui
    }

    fn show_control_panel(&self, ui: &Ui, framerate: f32) {
        let mut app: SharedAppCtx = self.app.clone();

        ui.window("Control Panel")
            .size([150.0, 300.0], Condition::Once)
            .position([512.0, 0.0], Condition::Once)
            .resizable(false)
            .build(|| {
                ui.text(format!(" UI FPS: {:.2}", framerate));
                ui.text(format!("NES FPS: {:.2}", framerate));
                ui.separator();

                ui.text(format!("State: {:?}", app.nes_state()));

                match app.nes_state() {
                    NESState::Stopped => {
                        if ui.button("Start") {
                            app.nes_state.set(NESState::Running);
                        }
                        ui.disabled(true, || {
                            ui.button("Pause");
                            ui.button("Step");
                        });
                    }

                    NESState::Running => {
                        if ui.button("Stop") {
                            app.nes_state.set(NESState::Stopped);
                        }
                        if ui.button("Pause") {
                            app.nes_state.set(NESState::Paused);
                        }
                        ui.disabled(true, || {
                            ui.button("Step");
                        });
                    }

                    NESState::Paused | NESState::Stepping => {
                        if ui.button("Stop") {
                            app.nes_state.set(NESState::Stopped);
                        }
                        if ui.button("Resume") {
                            app.nes_state.set(NESState::Running);
                        }
                        if ui.button("Step") {
                            app.nes_state.set(NESState::Stepping);
                        }
                    }
                };
            });
    }
}

#[derive(Debug)]
pub struct NESGameWindow {
    pub generated_texture: Option<imgui::TextureId>,
    pub image_data: Vec<u8>,
    width: usize,
    height: usize,
}

impl NESGameWindow {
    pub fn new() -> Self {
        Self {
            generated_texture: None,
            image_data: Vec::with_capacity(SCREEN_WIDTH * SCREEN_HEIGHT * 3),
            width: SCREEN_WIDTH,
            height: SCREEN_HEIGHT,
        }
    }

    /// Generate dummy texture
    fn generate(&mut self, opengl: &glow::Context, textures: &mut imgui::Textures<glow::Texture>) {
        // Allocate enough space for RGB (3 bytes per pixel)
        for _ in 0..(self.width * self.height) {
            self.image_data.push(0xFF); // R
            self.image_data.push(0xAA); // G
            self.image_data.push(0x55); // B
        }

        let opengl_texture: glow::NativeTexture = unsafe {
            let tmp: Result<glow::NativeTexture, String> = opengl.create_texture();
            gl_error!(opengl);
            tmp
        }
        .expect("Unable to create OpenGL texture");

        unsafe {
            opengl.bind_texture(glow::TEXTURE_2D, Some(opengl_texture));
            gl_error!(opengl);
            // Set unpack alignment to 1 for tightly packed RGB data
            opengl.pixel_store_i32(glow::UNPACK_ALIGNMENT, 1);
            gl_error!(opengl);
            opengl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_MIN_FILTER,
                glow::LINEAR as _,
            );
            gl_error!(opengl);
            opengl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_MAG_FILTER,
                glow::LINEAR as _,
            );
            gl_error!(opengl);
            opengl.tex_image_2d(
                glow::TEXTURE_2D,
                0,
                glow::RGB as _,
                self.width as _,
                self.height as _,
                0,
                glow::RGB,
                glow::UNSIGNED_BYTE,
                Some(&self.image_data),
            );
            gl_error!(opengl);
        }

        self.generated_texture = Some(textures.insert(opengl_texture));
    }

    fn show(&self, ui: &imgui::Ui) {
        let style: StyleStackToken<'_> = ui.push_style_var(StyleVar::WindowPadding([0.0, 0.0]));
        ui.window("Emulator Output")
            .size([256.0 * 2.0, 240.0 * 2.0], Condition::Once)
            .position([0.0, 0.0], Condition::Once)
            .resizable(false)
            .scroll_bar(false)
            .scrollable(false)
            .nav_inputs(false)
            .build(|| {
                if let Some(generated_texture) = self.generated_texture {
                    imgui::Image::new(generated_texture, [256.0 * 2.0, 240.0 * 2.0]).build(ui);
                }
            });
        style.pop();
    }
}

// Shared Window Context getters
impl SharedWindowCtxAccess for NesmurUI {
    fn window(&self) -> &Window {
        get_from_swc!(self.win_ctx.window)
    }
    fn window_mut(&self) -> &mut Window {
        get_from_swc!(mut self.win_ctx.window)
    }

    fn context(&self) -> &PossiblyCurrentContext {
        get_from_swc!(self.win_ctx.window_context)
    }
    fn context_mut(&self) -> &mut PossiblyCurrentContext {
        get_from_swc!(mut self.win_ctx.window_context)
    }

    fn surface(&self) -> &Surface<WindowSurface> {
        get_from_swc!(self.win_ctx.window_surface)
    }
    fn surface_mut(&self) -> &mut Surface<WindowSurface> {
        get_from_swc!(mut self.win_ctx.window_surface)
    }

    fn opengl(&self) -> &glow::Context {
        get_from_swc!(self.win_ctx.opengl)
    }
    fn opengl_mut(&self) -> &mut glow::Context {
        get_from_swc!(mut self.win_ctx.opengl)
    }

    fn winit_platform(&self) -> &WinitPlatform {
        get_from_swc!(self.win_ctx.winit_platform)
    }
    fn winit_platform_mut(&self) -> &mut WinitPlatform {
        get_from_swc!(mut self.win_ctx.winit_platform)
    }

    fn imgui_context(&self) -> &imgui::Context {
        get_from_swc!(self.win_ctx.imgui_context)
    }
    fn imgui_context_mut(&self) -> &mut imgui::Context {
        get_from_swc!(mut self.win_ctx.imgui_context)
    }

    fn textures(&self) -> &imgui::Textures<glow::Texture> {
        get_from_swc!(self.win_ctx.imgui_textures)
    }
    fn textures_mut(&self) -> &mut imgui::Textures<glow::Texture> {
        get_from_swc!(mut self.win_ctx.imgui_textures)
    }

    fn imgui_renderer(&self) -> &Renderer {
        get_from_swc!(self.win_ctx.imgui_renderer)
    }
    fn imgui_renderer_mut(&self) -> &mut Renderer {
        get_from_swc!(mut self.win_ctx.imgui_renderer)
    }
}
