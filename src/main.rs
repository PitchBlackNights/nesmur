use glow::HasContext;
use glutin::{
    config::{Config, ConfigTemplateBuilder},
    context::{
        ContextAttributes, ContextAttributesBuilder, NotCurrentContext, NotCurrentGlContext,
        PossiblyCurrentContext,
    },
    display::{GetGlDisplay, GlDisplay},
    surface::{
        GlSurface, Surface, SurfaceAttributes, SurfaceAttributesBuilder, SwapInterval,
        WindowSurface,
    },
};
use glutin_winit::DisplayBuilder;
use imgui::{DrawData, Ui};
use imgui_glow_renderer::Renderer;
use imgui_winit_support::WinitPlatform;
use nesmur::{
    cli_parser::Args,
    gl_error,
    nes_manager::NESManager,
    prelude::*,
    setup,
    shared_ctx::{app::*, window::*},
    ui::NesmurUI,
    NESEvent, NESState, NesmurEvent, INITIAL_WINDOW_HEIGHT, INITIAL_WINDOW_WIDTH,
};
use raw_window_handle::HasWindowHandle;
use std::{num::NonZeroU32, time::Instant};
use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    event::{Event, KeyEvent, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop, EventLoopProxy},
    keyboard::PhysicalKey,
    window::{Window, WindowAttributes, WindowId},
};

fn main() {
    let _args: Args = setup::setup_logger_and_args();
    info!("Starting Emulator...");

    let event_loop: EventLoop<NesmurEvent> =
        EventLoop::<NesmurEvent>::with_user_event().build().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let proxy: EventLoopProxy<NesmurEvent> = event_loop.create_proxy();
    let _ = event_loop.run_app(&mut Nesmur::new(proxy));

    info!("Stopping Emulator...");
}

impl std::fmt::Debug for Nesmur {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Nesmur")
            .field("last_frame", &self.last_ui_time)
            .field("nes_state", &self.nes_state)
            .field("uninitialized", &self.uninitialized)
            .field("ui", &self.ui)
            .field("shared_window_ctx", &self.shared_window_ctx)
            .field("shared_app_ctx", &self.shared_app_ctx)
            .field("window", &self.window.as_ref().unwrap())
            .field("window_context", &self.window_context.as_ref().unwrap())
            .field("window_surface", &self.window_surface.as_ref().unwrap())
            .field("opengl", &self.opengl.as_ref().unwrap())
            .field("winit_platform", &self.winit_platform.as_ref().unwrap())
            .field("imgui_context", &self.imgui_context.as_ref().unwrap())
            .field("imgui_textures", &self.imgui_textures.as_ref().unwrap())
            .field("imgui_renderer", &"Renderer { .. }")
            .finish()
    }
}

struct Nesmur {
    uninitialized: bool,
    event_loop_proxy: EventLoopProxy<NesmurEvent>,
    nes_manager: NESManager,
    nes_state: NESState,
    ui: NesmurUI,
    last_ui_time: Instant,

    shared_window_ctx: SharedWindowCtx,
    /// DON'T USE IN THE MAIN APP
    shared_app_ctx: SharedAppCtx,

    // Meant only to keep the data from being dropped.
    // DO NOT ACCESS WITHOUT USING HELPER FUNCTIONS
    window: Option<Window>,
    window_context: Option<PossiblyCurrentContext>,
    window_surface: Option<Surface<WindowSurface>>,
    opengl: Option<glow::Context>,
    winit_platform: Option<WinitPlatform>,
    imgui_context: Option<imgui::Context>,
    imgui_textures: Option<imgui::Textures<glow::Texture>>,
    imgui_renderer: Option<Renderer>,
}

impl Nesmur {
    fn new(event_loop_proxy: EventLoopProxy<NesmurEvent>) -> Self {
        Nesmur {
            uninitialized: true,
            event_loop_proxy: event_loop_proxy.clone(),
            nes_manager: NESManager::new(&event_loop_proxy),
            nes_state: NESState::Stopped,
            ui: NesmurUI::new(),
            last_ui_time: Instant::now(),

            shared_window_ctx: SharedWindowCtx::default(),
            shared_app_ctx: SharedAppCtx::default(),

            window: None,
            window_context: None,
            window_surface: None,
            opengl: None,
            winit_platform: None,
            imgui_context: None,
            imgui_textures: None,
            imgui_renderer: None,
        }
    }

    fn init(&mut self, event_loop: &ActiveEventLoop) {
        trace!("Initializing window...");

        // =======================================
        //  Create the Window, Context, & Surface
        // =======================================
        let window_attributes: WindowAttributes = WindowAttributes::default()
            .with_title("NESMUR")
            .with_inner_size(LogicalSize::new(
                INITIAL_WINDOW_WIDTH,
                INITIAL_WINDOW_HEIGHT,
            ));
        let (window, window_config): (Option<Window>, Config) = DisplayBuilder::new()
            .with_window_attributes(Some(window_attributes))
            .build(event_loop, ConfigTemplateBuilder::new(), |mut configs| {
                configs.next().unwrap()
            })
            .expect("Failed to crate OpenGL window");
        let window: Window = window.unwrap();

        let window_context_attributes: ContextAttributes = ContextAttributesBuilder::new()
            .with_debug(true)
            .build(Some(window.window_handle().unwrap().as_raw()));
        let temp_window_context: NotCurrentContext = unsafe {
            window_config
                .display()
                .create_context(&window_config, &window_context_attributes)
                .expect("Failed to create OpenGL context")
        };

        let window_surface_attributes: SurfaceAttributes<WindowSurface> =
            SurfaceAttributesBuilder::<WindowSurface>::new()
                .with_srgb(Some(true))
                .build(
                    window.window_handle().unwrap().as_raw(),
                    NonZeroU32::new(INITIAL_WINDOW_WIDTH).unwrap(),
                    NonZeroU32::new(INITIAL_WINDOW_HEIGHT).unwrap(),
                );
        let window_surface: Surface<WindowSurface> = unsafe {
            window_config
                .display()
                .create_window_surface(&window_config, &window_surface_attributes)
                .expect("Failed to create OpenGL surface")
        };

        let window_context: PossiblyCurrentContext = temp_window_context
            .make_current(&window_surface)
            .expect("Failed to make OpenGL context the currect context");

        window_surface
            .set_swap_interval(
                &window_context,
                SwapInterval::Wait(NonZeroU32::new(1).unwrap()),
            )
            .expect("Failed to set surface swap interval");

        // ==================
        //  Initialize ImGUI
        // ==================
        let mut imgui_context: imgui::Context = imgui::Context::create();
        imgui_context.set_ini_filename(None);

        let mut winit_platform: WinitPlatform = WinitPlatform::new(&mut imgui_context);
        winit_platform.attach_window(
            imgui_context.io_mut(),
            &window,
            imgui_winit_support::HiDpiMode::Rounded,
        );

        nesmur::theme::apply_context(&mut imgui_context);
        imgui_context.io_mut().font_global_scale = (1.0 / winit_platform.hidpi_factor()) as f32;

        // ===============================
        //  Setup OpenGL for actual usage
        // ===============================
        let mut opengl: glow::Context = unsafe {
            glow::Context::from_loader_function_cstr(|cstr| {
                window_context.display().get_proc_address(cstr).cast()
            })
        };

        unsafe {
            opengl.debug_message_callback(
                |source: u32, msgtype: u32, id: u32, severity: u32, message: &str| {
                    let msg_source: &str = match source {
                        glow::DEBUG_SOURCE_API => "API",
                        glow::DEBUG_SOURCE_APPLICATION => "APPLICATION",
                        glow::DEBUG_SOURCE_OTHER => "OTHER",
                        glow::DEBUG_SOURCE_SHADER_COMPILER => "SHADER COMPILER",
                        glow::DEBUG_SOURCE_THIRD_PARTY => "THIRD PARTY",
                        glow::DEBUG_SOURCE_WINDOW_SYSTEM => "WINDOW SYSTEM",
                        _ => "UNKNONW",
                    };
                    let msg_type: &str = match msgtype {
                        glow::DEBUG_TYPE_DEPRECATED_BEHAVIOR => "DEPRECATED BEHAVIOR",
                        glow::DEBUG_TYPE_ERROR => "ERROR",
                        glow::DEBUG_TYPE_MARKER => "MARKER",
                        glow::DEBUG_TYPE_OTHER => "OTHER",
                        glow::DEBUG_TYPE_PERFORMANCE => "PERFORMACE",
                        glow::DEBUG_TYPE_POP_GROUP => "POP GROUP",
                        glow::DEBUG_TYPE_PORTABILITY => "PORTABILITY",
                        glow::DEBUG_TYPE_PUSH_GROUP => "PUSH GROUP",
                        glow::DEBUG_TYPE_UNDEFINED_BEHAVIOR => "UNDEFINED BEHAVIOR",
                        _ => "UNKNONW",
                    };
                    let msg_severity: &str = match severity {
                        glow::DEBUG_SEVERITY_HIGH => "HIGH",
                        glow::DEBUG_SEVERITY_LOW => "LOW",
                        glow::DEBUG_SEVERITY_MEDIUM => "MEDIUM",
                        glow::DEBUG_SEVERITY_NOTIFICATION => "NOTIFICATION",
                        _ => "UNKNONW",
                    };
                    if msg_severity == "NOTIFICATION" {
                        return;
                    }
                    debug!(
                        "[====OpenGL====]: [{}] [{}] [{}] [{}] {}",
                        msg_source, msg_type, id, msg_severity, message
                    );
                },
            );

            // Tells OpenGL to automatically convert the framebuffer in sRGB space after the fragment shader
            opengl.enable(glow::FRAMEBUFFER_SRGB);
            gl_error!(opengl);
        }

        let mut imgui_textures: imgui::Textures<glow::NativeTexture> =
            imgui::Textures::<glow::Texture>::default();
        let imgui_renderer: Renderer =
            Renderer::new(&opengl, &mut imgui_context, &mut imgui_textures, false)
                .expect("Failed to create ImGUI Renderer");

        // Move data to parent struct to prevent the actual data from being dropped
        self.window = Some(window);
        self.window_context = Some(window_context);
        self.window_surface = Some(window_surface);
        self.opengl = Some(opengl);
        self.winit_platform = Some(winit_platform);
        self.imgui_context = Some(imgui_context);
        self.imgui_textures = Some(imgui_textures);
        self.imgui_renderer = Some(imgui_renderer);

        // Bind raw pointers to allow for easy mutability anywhere
        self.shared_window_ctx.window = self.window.as_mut().unwrap();
        self.shared_window_ctx.window_context = self.window_context.as_mut().unwrap();
        self.shared_window_ctx.window_surface = self.window_surface.as_mut().unwrap();
        self.shared_window_ctx.opengl = self.opengl.as_mut().unwrap();
        self.shared_window_ctx.winit_platform = self.winit_platform.as_mut().unwrap();
        self.shared_window_ctx.imgui_context = self.imgui_context.as_mut().unwrap();
        self.shared_window_ctx.imgui_textures = self.imgui_textures.as_mut().unwrap();
        self.shared_window_ctx.imgui_renderer = self.imgui_renderer.as_mut().unwrap();
        self.shared_app_ctx.nes_state = SharedNESState(&mut self.nes_state);

        // Do what little UI initialization there is to do
        self.ui.init(
            &self.shared_window_ctx,
            &self.shared_app_ctx,
            &self.event_loop_proxy,
        );
        self.uninitialized = false;
        self.last_ui_time = Instant::now();

        trace!("Finished initializing window");
    }
}

impl ApplicationHandler<NesmurEvent> for Nesmur {
    fn user_event(&mut self, _event_loop: &ActiveEventLoop, event: NesmurEvent) {
        if self.uninitialized {
            return;
        }

        match event {
            NesmurEvent::NES(NESEvent::Start) => {
                debug!("NESEvent: Start");
                self.nes_manager.start_nes();
                self.nes_state = NESState::Running;
            }

            NesmurEvent::NES(NESEvent::Stop) => {
                debug!("NESEvent: Stop");
                self.nes_manager.stop_nes();
                self.nes_state = NESState::Stopped;
            }

            NesmurEvent::NES(NESEvent::Pause) => {
                debug!("NESEvent: Pause");
                self.nes_state = NESState::Paused;
            }

            NesmurEvent::NES(NESEvent::Resume) => {
                debug!("NESEvent: Resume");
                self.nes_state = NESState::Running;
            }

            NesmurEvent::NES(NESEvent::Step) => {
                debug!("NESEvent: Step");
                self.nes_state = NESState::Stepping;
            }

            NesmurEvent::NES(NESEvent::NewFrame(pixels)) => {
                // debug!("Got new frame data!");
                self.ui.update_nes_frame(&pixels);
            }

            #[allow(unreachable_patterns)]
            _ => warn!("Unhandled UserEvent: NesmurEvent::{:?}", event),
        };
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        if self.uninitialized {
            return;
        }

        match event {
            WindowEvent::RedrawRequested => {
                unsafe {
                    self.opengl().clear(glow::COLOR_BUFFER_BIT);
                    self.opengl().clear_color(0.5, 0.5, 0.5, 1.0);
                };

                let ui: &mut Ui = self
                    .ui
                    .redraw(self.nes_manager.framerate, self.nes_manager.frametime);

                self.winit_platform_mut().prepare_render(ui, self.window());
                let imgui_draw_data: &DrawData = self.imgui_context_mut().render();
                gl_error!(self.opengl());

                self.imgui_renderer_mut()
                    .render(self.opengl(), self.textures(), imgui_draw_data)
                    .expect("Error rendering ImGUI");
                gl_error!(self.opengl());

                self.surface()
                    .swap_buffers(self.context())
                    .expect("Failed to swap framebuffers");
                gl_error!(self.opengl());
            }

            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(key_code),
                        state,
                        ..
                    },
                ..
            } => {
                trace!(
                    "KeyboardInput -> key_code: {:?}, state: {:?}",
                    key_code,
                    state
                );
            }

            WindowEvent::Resized(new_size) => {
                if new_size.width > 0 && new_size.height > 0 {
                    self.surface().resize(
                        self.context(),
                        NonZeroU32::new(new_size.width).unwrap(),
                        NonZeroU32::new(new_size.height).unwrap(),
                    );
                    // trace!("Window resized: {}x{}", new_size.width, new_size.height);
                }

                let super_event: Event<()> = Event::WindowEvent { window_id, event };
                self.winit_platform_mut().handle_event(
                    self.imgui_context_mut().io_mut(),
                    self.window(),
                    &super_event,
                );
            }

            WindowEvent::CloseRequested => {
                trace!("Window close was requested");
                event_loop.exit();
            }

            event => {
                let super_event: Event<()> = Event::WindowEvent { window_id, event };
                self.winit_platform_mut().handle_event(
                    self.imgui_context_mut().io_mut(),
                    self.window(),
                    &super_event,
                );
            }
        }
    }

    fn resumed(&mut self, _event_loop: &ActiveEventLoop) {}

    fn new_events(&mut self, event_loop: &ActiveEventLoop, cause: winit::event::StartCause) {
        if self.uninitialized {
            if cause == winit::event::StartCause::Init {
                self.init(event_loop);
                self.event_loop_proxy
                    .send_event(NesmurEvent::NES(NESEvent::Start))
                    .unwrap();
            } else {
                return;
            }
        }

        self.nes_manager.handle_nes_messages();

        let now: Instant = Instant::now();
        self.imgui_context_mut()
            .io_mut()
            .update_delta_time(now.duration_since(self.last_ui_time));
        self.last_ui_time = now;
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if self.uninitialized {
            return;
        }

        self.winit_platform()
            .prepare_frame(self.imgui_context_mut().io_mut(), self.window())
            .unwrap();
        self.window().request_redraw();
    }

    fn exiting(&mut self, _event_loop: &ActiveEventLoop) {
        trace!("Exiting window loop...");
        if self.uninitialized {
            panic!("Program exited before it was even initialized!");
        }

        self.imgui_renderer_mut().destroy(self.opengl());
    }
}

// Shared Window Context getters
impl SharedWindowCtxAccess for Nesmur {
    fn window(&self) -> &Window {
        get_from_swc!(self.shared_window_ctx.window)
    }
    fn window_mut(&self) -> &mut Window {
        get_from_swc!(mut self.shared_window_ctx.window)
    }

    fn context(&self) -> &PossiblyCurrentContext {
        get_from_swc!(self.shared_window_ctx.window_context)
    }
    fn context_mut(&self) -> &mut PossiblyCurrentContext {
        get_from_swc!(mut self.shared_window_ctx.window_context)
    }

    fn surface(&self) -> &Surface<WindowSurface> {
        get_from_swc!(self.shared_window_ctx.window_surface)
    }
    fn surface_mut(&self) -> &mut Surface<WindowSurface> {
        get_from_swc!(mut self.shared_window_ctx.window_surface)
    }

    fn opengl(&self) -> &glow::Context {
        get_from_swc!(self.shared_window_ctx.opengl)
    }
    fn opengl_mut(&self) -> &mut glow::Context {
        get_from_swc!(mut self.shared_window_ctx.opengl)
    }

    fn winit_platform(&self) -> &WinitPlatform {
        get_from_swc!(self.shared_window_ctx.winit_platform)
    }
    fn winit_platform_mut(&self) -> &mut WinitPlatform {
        get_from_swc!(mut self.shared_window_ctx.winit_platform)
    }

    fn imgui_context(&self) -> &imgui::Context {
        get_from_swc!(self.shared_window_ctx.imgui_context)
    }
    fn imgui_context_mut(&self) -> &mut imgui::Context {
        get_from_swc!(mut self.shared_window_ctx.imgui_context)
    }

    fn textures(&self) -> &imgui::Textures<glow::Texture> {
        get_from_swc!(self.shared_window_ctx.imgui_textures)
    }
    fn textures_mut(&self) -> &mut imgui::Textures<glow::Texture> {
        get_from_swc!(mut self.shared_window_ctx.imgui_textures)
    }

    fn imgui_renderer(&self) -> &Renderer {
        get_from_swc!(self.shared_window_ctx.imgui_renderer)
    }
    fn imgui_renderer_mut(&self) -> &mut Renderer {
        get_from_swc!(mut self.shared_window_ctx.imgui_renderer)
    }
}
