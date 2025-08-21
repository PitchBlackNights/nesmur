#![allow(clippy::mut_from_ref)]

pub mod window {
    pub use crate::get_from_swc;

    #[derive(Debug, Clone)]
    pub struct SharedWindowCtx {
        pub window: *mut winit::window::Window,
        pub window_context: *mut glutin::context::PossiblyCurrentContext,
        pub window_surface: *mut glutin::surface::Surface<glutin::surface::WindowSurface>,
        pub opengl: *mut glow::Context,
        pub winit_platform: *mut imgui_winit_support::WinitPlatform,
        pub imgui_context: *mut imgui::Context,
        pub imgui_textures: *mut imgui::Textures<glow::Texture>,
        pub imgui_renderer: *mut imgui_glow_renderer::Renderer,
    }

    impl Default for SharedWindowCtx {
        fn default() -> Self {
            Self {
                window: std::ptr::null_mut(),
                window_context: std::ptr::null_mut(),
                window_surface: std::ptr::null_mut(),
                opengl: std::ptr::null_mut(),
                winit_platform: std::ptr::null_mut(),
                imgui_context: std::ptr::null_mut(),
                imgui_textures: std::ptr::null_mut(),
                imgui_renderer: std::ptr::null_mut(),
            }
        }
    }

    pub trait SharedWindowCtxAccess {
        fn window(&self) -> &winit::window::Window;
        fn window_mut(&self) -> &mut winit::window::Window;

        fn context(&self) -> &glutin::context::PossiblyCurrentContext;
        fn context_mut(&self) -> &mut glutin::context::PossiblyCurrentContext;

        fn surface(&self) -> &glutin::surface::Surface<glutin::surface::WindowSurface>;
        fn surface_mut(&self) -> &mut glutin::surface::Surface<glutin::surface::WindowSurface>;

        fn opengl(&self) -> &glow::Context;
        fn opengl_mut(&self) -> &mut glow::Context;

        fn winit_platform(&self) -> &imgui_winit_support::WinitPlatform;
        fn winit_platform_mut(&self) -> &mut imgui_winit_support::WinitPlatform;

        fn imgui_context(&self) -> &imgui::Context;
        fn imgui_context_mut(&self) -> &mut imgui::Context;

        fn textures(&self) -> &imgui::Textures<glow::Texture>;
        fn textures_mut(&self) -> &mut imgui::Textures<glow::Texture>;

        fn imgui_renderer(&self) -> &imgui_glow_renderer::Renderer;
        fn imgui_renderer_mut(&self) -> &mut imgui_glow_renderer::Renderer;
    }

    #[macro_export]
    macro_rules! get_from_swc {
        ($var:expr) => {
            if $var.is_null() {
                let mut var_str = stringify!($var);
                if let Some(index) = var_str.rfind('.') {
                    var_str = &var_str[index..]
                }
                panic!(
                    "{}",
                    format!(
                        "Tried to access `SharedWindowCtx.{}` before a valid pointer was assigned!",
                        var_str
                    )
                );
            } else {
                let var = $var;
                unsafe { &*var.clone() }
            }
        };
        (mut $var:expr) => {
            if $var.is_null() {
                let mut var_str = stringify!($var);
                if let Some(index) = var_str.rfind('.') {
                    var_str = &var_str[index + 1..]
                }
                panic!(
                    "{}",
                    format!(
                        "Tried to access `SharedWindowCtx.{}` before a valid pointer was assigned!",
                        var_str
                    )
                );
            } else {
                let var = $var;
                unsafe { &mut *var.clone() }
            }
        };
    }
}

pub mod app {
    #[derive(Debug, Clone)]
    pub struct SharedNESState(pub *mut crate::NESState);

    impl SharedNESState {
        pub fn get(&self) -> &mut crate::NESState {
            if self.0.is_null() {
                panic!(
                    "Tried to access `SharedAppCtx.nes_state` before a valid pointer was assigned!"
                );
            }
            unsafe { &mut *self.0 }
        }

        pub fn set(&mut self, value: crate::NESState) {
            unsafe { *self.0 = value };
        }
    }

    #[derive(Debug, Clone)]
    pub struct SharedAppCtx {
        pub nes_state: SharedNESState,
    }

    impl Default for SharedAppCtx {
        fn default() -> Self {
            Self {
                nes_state: SharedNESState(std::ptr::null_mut()),
            }
        }
    }

    impl SharedAppCtx {
        pub fn nes_state(&self) -> &mut crate::NESState {
            self.nes_state.get()
        }
    }
}
