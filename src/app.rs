use crate::{
    INITIAL_SIZE_HEIGHT, INITIAL_SIZE_WIDTH, PERSISTENT_DATA_PATH,
    events::{AppEvent, AppEventQueue, ResetTarget},
    input::{ControllerConfig, InputManager, InputMapping},
    prelude::*,
};
use eframe::{CreationContext, Storage};
use egui::{Color32, ColorImage, TextureOptions};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    time::{Duration, Instant},
};
use uuid::Uuid;

const APP_CONFIG_KEY: &str = "app_config";

#[derive(Serialize, Deserialize)]
pub struct AppConfig {
    pub volume: f64,
    pub keyboard_input_mapping: (InputMapping, InputMapping),
    pub controller_input_mapping: HashMap<Uuid, ControllerConfig>,
    pub selected_controllers: (Option<Uuid>, Option<Uuid>),
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            volume: 1.0,
            keyboard_input_mapping: (InputMapping::default_keyboard(), InputMapping::default()),
            controller_input_mapping: HashMap::new(),
            selected_controllers: (None, None),
        }
    }
}

/// Main application struct holding all state/context
pub struct App {
    // States
    pub show_controller_config: bool,
    pub is_paused: bool,
    pub show_reset_app_data: bool,
    pub do_reset_app_data: Option<bool>,
    #[cfg(debug_assertions)]
    pub debug: crate::debug::DebugOptions,

    // Data
    pub input_manager: InputManager,
    pub nes_manager: crate::nes_manager::NESManager,
    pub nes_state: crate::NESState,
    events: AppEventQueue,

    // Misc
    last_frametime: Instant,
    pub avg_framerate: f64,
    pub avg_frametime: f64,
    frametimes: Vec<f64>,
    frametimes_index: usize,
    pub volume: f64,
}

impl App {
    pub fn new(cc: &CreationContext) -> Self {
        debug!("Initializing app...");

        egui_extras::install_image_loaders(&cc.egui_ctx);
        // cc.egui_ctx.set_theme(egui::ThemePreference::Dark);

        let screen_texture: egui::TextureHandle = cc.egui_ctx.load_texture(
            "nes",
            ColorImage::new([256, 240], vec![Color32::BLACK; 256 * 240]),
            TextureOptions::NEAREST,
        );
        let nes_manager: crate::nes_manager::NESManager =
            crate::nes_manager::NESManager::new(screen_texture);

        let config: AppConfig = Self::read_config(cc.storage);
        let input_manager: InputManager = InputManager::new(&config);

        debug!("Finished initializing app");
        App {
            // States
            show_controller_config: false,
            is_paused: false,
            show_reset_app_data: false,
            do_reset_app_data: None,
            #[cfg(debug_assertions)]
            debug: crate::debug::DebugOptions::new(),

            // Data
            input_manager,
            // screen_texture,
            nes_manager,
            nes_state: crate::NESState::Stopped,
            events: AppEventQueue::new(),

            // Misc
            last_frametime: Instant::now(),
            avg_framerate: 0.0,
            avg_frametime: 0.0,
            frametimes: Vec::with_capacity(120),
            frametimes_index: 0,
            volume: config.volume,
        }
    }

    fn reset_app(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        debug!("Resetting app data...");

        if self.nes_state != crate::NESState::Stopped {
            self.nes_manager.stop_nes();
        }
        Self::delete_config();
        ctx.memory_mut(|mem: &mut egui::Memory| *mem = Default::default());
        ctx.forget_all_images();
        ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(egui::vec2(
            INITIAL_SIZE_WIDTH,
            INITIAL_SIZE_HEIGHT,
        )));

        egui_extras::install_image_loaders(ctx);
        // ctx.set_theme(egui::ThemePreference::Dark);

        let screen_texture: egui::TextureHandle = ctx.load_texture(
            "nes",
            ColorImage::new([256, 240], vec![Color32::BLACK; 256 * 240]),
            TextureOptions::NEAREST,
        );
        let nes_manager: crate::nes_manager::NESManager =
            crate::nes_manager::NESManager::new(screen_texture);

        let config: AppConfig = Self::read_config(frame.storage());
        let input_manager: InputManager = InputManager::new(&config);

        self.show_controller_config = false;
        self.is_paused = false;
        #[cfg(debug_assertions)]
        {
            self.debug = crate::debug::DebugOptions::new();
        }
        self.input_manager = input_manager;
        self.nes_manager = nes_manager;
        self.nes_state = crate::NESState::Stopped;
        self.events = AppEventQueue::new();
        self.volume = config.volume;

        self.save_config(frame.storage_mut());
        frame.storage_mut().unwrap().flush();

        debug!("Finished resetting app data");
    }

    fn read_config(storage: Option<&'_ dyn Storage>) -> AppConfig {
        match storage {
            Some(storage) => match storage.get_string(APP_CONFIG_KEY) {
                Some(string) => {
                    info!("Using previously saved app config");
                    serde_json::from_str(&string).unwrap_or_else(|_| -> AppConfig {
                        error!("Failed to decode app config, using default values");
                        AppConfig::default()
                    })
                }
                None => {
                    info!("App config has never been saved before, using default values");
                    AppConfig::default()
                }
            },
            None => {
                error!(
                    "Failed to get eframe storage when trying to read app config, using default values"
                );
                AppConfig::default()
            }
        }
    }

    fn delete_config() {
        if let Err(e) = std::fs::remove_file(PERSISTENT_DATA_PATH) {
            error!("Failed to delete app config: {}", e);
        }
    }

    fn save_config<'s>(&self, storage: Option<&mut (dyn Storage + 's)>) {
        match storage {
            Some(storage) => {
                let state: AppConfig = AppConfig {
                    keyboard_input_mapping: self.input_manager.keyboard_input_mapping,
                    controller_input_mapping: self.input_manager.controller_input_mapping.clone(),
                    selected_controllers: self.input_manager.selected_controllers,
                    ..Default::default()
                };
                match serde_json::to_string(&state) {
                    Ok(config) => {
                        // info!("Saving app config...");
                        storage.set_string(APP_CONFIG_KEY, config);
                        storage.flush();
                        info!("Saved app config");
                    }
                    Err(e) => error!("Failed to serialize app config: {}", e),
                };
            }
            None => error!("Failed to get eframe storage when trying to save app config"),
        }
    }

    fn update_frametimes(&mut self) {
        let frametime: Duration = self.last_frametime.elapsed();
        self.last_frametime = Instant::now();

        // Even if a frame took a full year, it will still have
        // an accuracy of at least 0.001 miliseconds (1 microsecond)
        let frametime: f64 = (frametime.as_micros() as f64) / 1000.0;
        if self.frametimes.len() != self.frametimes.capacity() {
            self.frametimes.push(frametime);
        } else {
            self.frametimes[self.frametimes_index] = frametime;
            self.frametimes_index = (self.frametimes_index + 1) % self.frametimes.len();
        }
        // Summing all of the frametimes will never overflow, unless the
        // cumulative frametime is longer than something like 999 trillion years
        self.avg_frametime = self.frametimes.iter().sum::<f64>() / self.frametimes.len() as f64;
        self.avg_framerate = 1000.0 / self.avg_frametime;
    }

    pub fn new_event(&mut self, event: AppEvent) {
        self.events.push(event);
    }

    fn handle_events(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        if self.events.is_empty() {
            return;
        }
        trace!("Handling {} app events", self.events.len());

        while let Some(event) = self.events.pull() {
            use crate::events::AppEvent::*;

            trace!("Handling event: {:?}", event);

            match event {
                ResetData(ResetTarget::Everything) => self.reset_app(ctx, frame),
                ResetData(ResetTarget::Egui) => {
                    Self::delete_config();
                    ctx.memory_mut(|mem: &mut egui::Memory| *mem = Default::default());
                    ctx.forget_all_images();
                    ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(egui::vec2(
                        INITIAL_SIZE_WIDTH,
                        INITIAL_SIZE_HEIGHT,
                    )));
                    egui_extras::install_image_loaders(ctx);

                    let screen_texture: egui::TextureHandle = ctx.load_texture(
                        "nes",
                        ColorImage::new([256, 240], vec![Color32::BLACK; 256 * 240]),
                        TextureOptions::NEAREST,
                    );
                    self.nes_manager.screen_texture = screen_texture;

                    self.save_config(frame.storage_mut());
                    frame.storage_mut().unwrap().flush();
                }

                Exit => {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    ctx.request_discard("exit");
                }

                NES(crate::NESEvent::Start(rom_path)) => {
                    if self.nes_state != crate::NESState::Stopped {
                        error!(
                            "Tried to start nes when in a non-stopped state: {:?}",
                            self.nes_state
                        );
                        return;
                    }
                    self.nes_manager.start_nes(rom_path);
                    self.nes_manager
                        .connect_device(1, nes::input_device::NESDeviceType::Joypad);
                    self.nes_state = crate::NESState::Running;
                }
                NES(crate::NESEvent::Stop) => {
                    if self.nes_state == crate::NESState::Stopped {
                        error!("Tried to stop nes when it is already stopped");
                        return;
                    }
                    self.nes_manager.stop_nes();
                    self.nes_state = crate::NESState::Stopped;
                    self.is_paused = false;
                }
                NES(crate::NESEvent::Pause) => {
                    if self.nes_state != crate::NESState::Running {
                        error!(
                            "Tried to pause nes when it a non-running state: {:?}",
                            self.nes_state
                        );
                        return;
                    }
                    self.nes_manager.pause();
                    self.nes_state = crate::NESState::Paused;
                }
                NES(crate::NESEvent::Resume) => {
                    if self.nes_state != crate::NESState::Paused {
                        error!(
                            "Tried to resume nes when it a non-paused state: {:?}",
                            self.nes_state
                        );
                        return;
                    }
                    self.nes_manager.resume();
                    self.nes_state = crate::NESState::Running;
                }

                e => warn!("Unhandled app event: {:?}", e),
            }
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.update_frametimes();
        ctx.request_repaint();

        self.handle_events(ctx, frame);
        self.nes_manager.handle_nes_messages();

        #[cfg(debug_assertions)]
        self.debug.update(ctx);

        if ctx.input(|ui: &egui::InputState| ui.focused) {
            self.input_manager.get_pressed_input(ctx);
            self.update_nes_buttons();
        }

        if self.input_manager.pause_pressed() {
            self.is_paused = !self.is_paused;
            match self.is_paused {
                true => self.new_event(AppEvent::NES(crate::NESEvent::Pause)),
                false => self.new_event(AppEvent::NES(crate::NESEvent::Resume)),
            };
        }

        self.draw_ui(ctx);
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        self.save_config(Some(storage));
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        if self.nes_state != crate::NESState::Stopped {
            self.nes_manager.stop_nes();
        }
    }
}
