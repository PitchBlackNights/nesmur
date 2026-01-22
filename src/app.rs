use crate::{
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
    pub should_exit: bool,
    pub show_reset_app_data: bool,
    pub do_reset_app_data: Option<bool>,
    #[cfg(debug_assertions)]
    pub do_debug_visuals: bool,
    pub request_nes_event: Vec<crate::NESEvent>,

    // Data
    pub input_manager: InputManager,
    // pub screen_texture: egui::TextureHandle,
    pub nes_manager: crate::nes_manager::NESManager,
    pub nes_state: crate::NESState,

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

        cc.egui_ctx.set_theme(egui::ThemePreference::Dark);

        #[cfg(debug_assertions)]
        cc.egui_ctx.style_mut(|style: &mut egui::Style| {
            style.debug.show_resize = false;
            style.debug.show_expand_height = false;
            style.debug.show_expand_width = false;
            style.debug.debug_on_hover_with_all_modifiers = false;
        });

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
            should_exit: false,
            show_reset_app_data: false,
            do_reset_app_data: None,
            #[cfg(debug_assertions)]
            do_debug_visuals: false,
            request_nes_event: vec![],

            // Data
            input_manager,
            // screen_texture,
            nes_manager,
            nes_state: crate::NESState::Stopped,

            // Misc
            last_frametime: Instant::now(),
            avg_framerate: 0.0,
            avg_frametime: 0.0,
            frametimes: Vec::with_capacity(120),
            frametimes_index: 0,
            volume: config.volume,
        }
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

    fn refresh_config(&mut self, storage: Option<&'_ dyn Storage>) {
        let config: AppConfig = Self::read_config(storage);
        self.input_manager = InputManager::new(&config);
        self.volume = config.volume;
    }

    fn delete_config() {
        if let Err(e) = std::fs::remove_file(crate::PERSISTENT_DATA_PATH) {
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
}

impl eframe::App for App {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        self.update_frametimes();
        ctx.request_repaint();

        if ctx.input(|i| i.viewport().close_requested()) {
            if self.nes_state != crate::NESState::Stopped {
                self.nes_manager.stop_nes();
            }
        }

        if self.should_exit {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            ctx.request_discard("exit");
            return;
        }

        if self.do_reset_app_data == Some(true) {
            Self::delete_config();
            self.refresh_config(frame.storage());
            self.save_config(frame.storage_mut());
            self.do_reset_app_data = Some(false);
        }

        if ctx.input(|ui| ui.focused) {
            self.input_manager.get_pressed_input(ctx);
            self.update_nes_buttons();
        }

        if self.input_manager.pause_pressed() {
            self.is_paused = !self.is_paused;
            match self.is_paused {
                true => self.request_nes_event.push(crate::NESEvent::Pause),
                false => self.request_nes_event.push(crate::NESEvent::Resume),
            };
        }

        if self.request_nes_event.len() > 0 {
            for event in self.request_nes_event.drain(..) {
                debug!("New NES Event: {:?}", event);
                match event {
                    crate::NESEvent::Start(path) => {
                        if self.nes_state != crate::NESState::Stopped {
                            error!(
                                "Tried to start nes when in a non-stopped state: {:?}",
                                self.nes_state
                            );
                            return;
                        }
                        self.nes_manager.start_nes(path);
                        self.nes_manager
                            .connect_device(1, nes::input_device::NESDeviceType::Joypad);
                        self.nes_state = crate::NESState::Running;
                    }
                    crate::NESEvent::Stop => {
                        if self.nes_state == crate::NESState::Stopped {
                            error!("Tried to stop nes when it is already stopped");
                            return;
                        }
                        self.nes_manager.stop_nes();
                        self.nes_state = crate::NESState::Stopped;
                        self.is_paused = false;
                    }
                    crate::NESEvent::Pause => {
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
                    crate::NESEvent::Resume => {
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
                    e => warn!("ignored requested nes event: {:?}", e),
                }
            }
        }

        self.nes_manager.handle_nes_messages();

        self.draw_ui(ctx);
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        self.save_config(Some(storage));
    }
}
