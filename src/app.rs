//! Main entry point and window/context management for NESMUR emulator
//!
//! Handles initialization, event loop, OpenGL, ImGUI, and NES state

use crate::{
    input::{ControllerConfig, InputManager, InputMapping},
    prelude::*,
};
use eframe::{
    egui::{self, Color32, ColorImage, TextureOptions},
    CreationContext, Storage,
};
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
            keyboard_input_mapping: (InputMapping::default(), InputMapping::default()),
            controller_input_mapping: HashMap::new(),
            selected_controllers: (None, None),
        }
    }
}

/// Main application struct holding all state/context
pub struct App {
    // UI states
    pub show_controller_config: bool,
    pub is_paused: bool,

    // App data
    /// Managment struct for controller and keyboard inputs
    pub input_manager: InputManager,
    pub screen_texture: egui::TextureHandle,

    // Misc values
    /// Last UI update frametime
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

        let screen_texture: egui::TextureHandle = cc.egui_ctx.load_texture(
            "nes",
            ColorImage::new([256, 240], vec![Color32::BLACK; 256 * 240]),
            TextureOptions::NEAREST,
        );
        let state: AppConfig = Self::read_config(cc.storage);

        debug!("Finished initializing app");
        App {
            // UI states
            show_controller_config: false,
            is_paused: false,

            // App data
            input_manager: InputManager::new(state),
            screen_texture,

            // Misc values
            last_frametime: Instant::now(),
            avg_framerate: 0.0,
            avg_frametime: 0.0,
            frametimes: Vec::with_capacity(120),
            frametimes_index: 0,
            volume: 0.0,
        }
    }

    fn read_config(storage: Option<&'_ dyn Storage>) -> AppConfig {
        match storage {
            Some(storage) => match storage.get_string(APP_CONFIG_KEY) {
                Some(string) => {
                    info!("Using previously saved app config");
                    serde_json::from_str(&string).unwrap_or_else(|_| {
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
                error!("Failed to get eframe storage when trying to read app config, using default values");
                AppConfig::default()
            }
        }
    }

    pub fn save_config(&self, storage: Option<&mut dyn Storage>) {
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
                        info!("Saving app config...");
                        storage.set_string(APP_CONFIG_KEY, config);
                        info!("Saved app config");
                    }
                    Err(e) => error!("Failed to serialize app config: {}", e),
                };
            }
            None => error!("Failed to get eframe storage when trying to save app config"),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
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

        if ctx.input(|ui| ui.focused) {
            self.input_manager.get_pressed_input(ctx);
        }

        self.ui_draw_top_panel(ctx);
        self.ui_draw_bottom_panel(ctx);
        self.ui_draw_center_panel(ctx);

        if self.show_controller_config {
            self.ui_draw_controller_config(ctx);
        }

        ctx.request_repaint();
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        self.save_config(Some(storage));
    }
}

// /// Main event loop and window event handling
// impl ApplicationHandler<NesmurEvent> for Nesmur {
//     /// Handle window events (redraw, keyboard, resize, close, etc)
//     fn window_event(
//         &mut self,
//         event_loop: &ActiveEventLoop,
//         window_id: WindowId,
//         event: WindowEvent,
//     ) {
//         match event {
//             WindowEvent::KeyboardInput {
//                 event:
//                     KeyEvent {
//                         physical_key: PhysicalKey::Code(key_code),
//                         state,
//                         ..
//                     },
//                 ..
//             } => {
//                 // Handle NES joypad button presses from keyboard
//                 if let Some((port, button)) = self.nes_keymap.get(&key_code) {
//                     let pressed: bool = state == ElementState::Pressed;
//                     // self.nes_manager
//                     //     .update_device_button(*port, Box::new(*button), pressed);
//                 }
//             }
//         }
//     }
// }
