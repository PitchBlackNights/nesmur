pub mod colored_text;

use self::colored_text::ImGUIColoredText;
use crate::{
    gl_error,
    prelude::*,
    shared_ctx::{app::SharedAppCtx, window::*},
    NESEvent, NESState, NesmurEvent,
};
use glow::{HasContext, PixelUnpackData};
use glutin::{
    context::PossiblyCurrentContext,
    surface::{Surface, WindowSurface},
};
use imgui::{Condition, StyleStackToken, StyleVar, Ui};
use imgui_glow_renderer::Renderer;
use imgui_winit_support::WinitPlatform;
use nes::{ppu::renderer::RGB, SCREEN_HEIGHT, SCREEN_WIDTH};
use winit::{event_loop::EventLoopProxy, window::Window};

#[derive(Debug)]
pub struct NesmurUI {
    pub nes_game_window: NESGameWindow,
    event_loop_proxy: Option<EventLoopProxy<NesmurEvent>>,
    win_ctx: SharedWindowCtx,
    app: SharedAppCtx,
}

impl NesmurUI {
    pub fn new() -> Self {
        NesmurUI {
            nes_game_window: NESGameWindow::new(),
            event_loop_proxy: None,
            win_ctx: SharedWindowCtx::default(),
            app: SharedAppCtx::default(),
        }
    }

    pub fn init(
        &mut self,
        shared_window_ctx: &SharedWindowCtx,
        shared_app_ctx: &SharedAppCtx,
        event_loop_proxy: &EventLoopProxy<NesmurEvent>,
    ) {
        self.win_ctx = shared_window_ctx.clone();
        self.app = shared_app_ctx.clone();
        self.event_loop_proxy = Some(event_loop_proxy.clone());

        self.nes_game_window.generate(
            get_from_swc!(self.win_ctx.opengl),
            get_from_swc!(mut self.win_ctx.imgui_textures),
        );
    }

    fn send_app_event(&self, evnet: NesmurEvent) {
        if self.event_loop_proxy.is_none() {
            panic!("UI tried to access its EventLoopProxy before it was created!");
        }

        let proxy: &EventLoopProxy<NesmurEvent> = self.event_loop_proxy.as_ref().unwrap();
        proxy
            .send_event(evnet)
            .expect("The app's EventLoop somehow does not exist!");
    }

    pub fn redraw(&self, nes_framerate: f32, nes_frametime: f32) -> &mut Ui {
        let framerate: f32 = self.imgui_context().io().framerate;
        let ui: &mut Ui = self.imgui_context_mut().frame();

        self.nes_game_window.show(ui);
        self.show_control_panel(ui, framerate, nes_framerate, nes_frametime);
        // OTHER WINDOWS
        // ui.window("Test").build(|| {
        //     ui.text_with_colors(format!("Test {{FF0000}}COLORED{{}} end string!"));
        // });

        ui
    }

    fn show_control_panel(&self, ui: &Ui, framerate: f32, nes_framerate: f32, nes_frametime: f32) {
        let app: SharedAppCtx = self.app.clone();

        ui.window("Control Panel")
            .size([150.0, 300.0], Condition::Once)
            .position([512.0, 0.0], Condition::Once)
            .resizable(false)
            .build(|| {
                ui.text(format!(" UI FPS: {:.0}", framerate));
                ui.separator();

                ui.text(format!("NES FPS: {:.0}", nes_framerate));
                ui.text(format!("NES  FT: {:.3} ms", nes_frametime));
                ui.separator();

                ui.text(format!("State: {:?}", app.nes_state()));

                match app.nes_state() {
                    NESState::Stopped => {
                        if ui.button("Start") {
                            self.send_app_event(NesmurEvent::NES(NESEvent::Start));
                        }
                        ui.disabled(true, || {
                            ui.button("Pause");
                            ui.button("Step");
                        });
                    }

                    NESState::Running => {
                        if ui.button("Stop") {
                            self.send_app_event(NesmurEvent::NES(NESEvent::Stop));
                        }
                        if ui.button("Pause") {
                            self.send_app_event(NesmurEvent::NES(NESEvent::Pause));
                        }
                        ui.disabled(true, || {
                            ui.button("Step");
                        });
                    }

                    NESState::Paused | NESState::Stepping => {
                        if ui.button("Stop") {
                            self.send_app_event(NesmurEvent::NES(NESEvent::Stop));
                        }
                        if ui.button("Resume") {
                            self.send_app_event(NesmurEvent::NES(NESEvent::Resume));
                        }
                        if ui.button("Step") {
                            self.send_app_event(NesmurEvent::NES(NESEvent::Step));
                        }
                    }
                };
            });
    }

    pub fn update_nes_frame(&mut self, pixels: &[RGB]) {
        let opengl_ptr: *const glow::Context = self.opengl() as *const glow::Context;
        let opengl_ref: &glow::Context = unsafe { &*opengl_ptr };
        self.nes_game_window.update(pixels, opengl_ref);
    }
}

#[derive(Debug)]
pub struct NESGameWindow {
    pub generated_texture: Option<imgui::TextureId>,
    pub image_data: Vec<u8>,
    opengl_texture: Option<glow::NativeTexture>,
    width: usize,
    height: usize,
}

impl NESGameWindow {
    pub fn new() -> Self {
        Self {
            generated_texture: None,
            image_data: Vec::with_capacity(SCREEN_WIDTH * SCREEN_HEIGHT * 3),
            opengl_texture: None,
            width: SCREEN_WIDTH,
            height: SCREEN_HEIGHT,
        }
    }

    pub fn update(&mut self, pixels: &[RGB], opengl: &glow::Context) {
        if pixels.len() != self.width * self.height {
            error!("NESGameWindow was not initialized with the same dimensions as the NES Pixel Buffer!");
            return;
        }

        for (index, color) in pixels.iter().enumerate() {
            let idx: usize = index * 3;
            self.image_data[idx] = color.0;
            self.image_data[idx + 1] = color.1;
            self.image_data[idx + 2] = color.2;
        }

        if let Some(texture) = self.opengl_texture {
            unsafe {
                opengl.bind_texture(glow::TEXTURE_2D, Some(texture));
                gl_error!(opengl);
                opengl.tex_sub_image_2d(
                    glow::TEXTURE_2D,
                    0,
                    0,
                    0,
                    self.width as i32,
                    self.height as i32,
                    glow::RGB,
                    glow::UNSIGNED_BYTE,
                    PixelUnpackData::Slice(Some(&self.image_data)),
                );
                gl_error!(opengl);
            }
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
                glow::NEAREST as _,
            );
            gl_error!(opengl);
            opengl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_MAG_FILTER,
                glow::NEAREST as _,
            );
            gl_error!(opengl);
            opengl.tex_image_2d(
                glow::TEXTURE_2D,
                0,
                glow::SRGB8 as i32,
                self.width as i32,
                self.height as i32,
                0,
                glow::RGB,
                glow::UNSIGNED_BYTE,
                PixelUnpackData::Slice(Some(&self.image_data)),
            );
            gl_error!(opengl);
        }

        self.generated_texture = Some(textures.insert(opengl_texture));
        self.opengl_texture = Some(opengl_texture);
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
