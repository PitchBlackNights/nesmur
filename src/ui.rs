use crate::{app::App, input::Input, prelude::*};
use egui::{
    Image, ViewportBuilder, ViewportId, containers::menu, include_image, load::SizedTexture,
};

impl App {
    pub fn draw_ui(&mut self, ctx: &egui::Context) {
        self.menu_bar(ctx);
        self.bottom_panel(ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add(
                Image::from_texture(SizedTexture::from_handle(&self.nes_manager.screen_texture))
                    .shrink_to_fit(),
            );
        });

        if self.show_controller_config {
            self.controller_config(ctx);
        }

        if self.show_reset_app_data {
            self.reset_app_data(ctx);
        }
    }

    fn menu_bar(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            menu::MenuBar::new().ui(ui, |ui| {
                self.menu_bar_file(ui);
                #[cfg(debug_assertions)]
                self.menu_bar_debug(ui);

                ui.separator();


                ui.add_sized(
                    [72.0, ui.available_height()],
                    egui::Label::new(format!("UI FPS: {:.0}", self.avg_framerate)),
                );
                ui.add_sized(
                    [95.0, ui.available_height()],
                    egui::Label::new(format!("UI FT: {:.2} ms", self.avg_frametime)),
                );
                ui.separator();

                ui.add_sized(
                    [83.0, ui.available_height()],
                    egui::Label::new(format!("NES FPS: {:.0}", self.nes_manager.framerate)),
                );
                ui.add_sized(
                    [106.0, ui.available_height()],
                    egui::Label::new(format!("NES FT: {:.2} ms", self.nes_manager.frametime)),
                );
            });
        });
    }

    #[cfg(debug_assertions)]
    fn menu_bar_debug(&mut self, ui: &mut egui::Ui) {
        ui.menu_button("Debug", |ui| {
            ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Extend);

            // ui.checkbox(&mut self.debug.visuals.show_resize, "Show resize");
            // ui.checkbox(&mut self.debug.visuals.show_expanded_height, "Show expanded height");
            // ui.checkbox(&mut self.debug.visuals.show_expanded_width, "Show expanded width");
            // ui.checkbox(&mut self.debug.visuals.debug_hover, "Debug hover");
            // ui.separator();

            if ui.button("Settings").clicked() {
                self.debug.show_settings = true;
            }
            if ui.button("Inspection").clicked() {
                self.debug.show_inspection = true;
            }
            if ui.button("Textures").clicked() {
                self.debug.show_textures = true;
            }
            if ui.button("Loaders").clicked() {
                self.debug.show_loaders = true;
            }
            if ui.button("Memory").clicked() {
                self.debug.show_memory = true;
            }
        });
    }

    fn menu_bar_file(&mut self, ui: &mut egui::Ui) {
        ui.menu_button("File", |ui| {
            ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Extend);

            if ui.button("Load ROM").clicked()
                && let Some(path) = rfd::FileDialog::new().pick_file()
            {
                debug!("Loading ROM from path: {:?}", path);
                self.request_nes_event.push(crate::NESEvent::Start(path));
            }
            ui.separator();

            ui.menu_button("Preferences", |ui| {
                if ui.button("Controllers").clicked() {
                    self.show_controller_config = !self.show_controller_config
                }

                ui.button("Stuff").clicked();
            });

            if ui.button("Reset app data").clicked() {
                self.show_reset_app_data = true;
            }
            ui.separator();

            if ui.button("Exit").clicked() {
                self.should_exit = true;
            }
        });
    }

    fn reset_app_data(&mut self, ctx: &egui::Context) {
        egui::Window::new("Are you sure?")
            .open(&mut self.show_reset_app_data)
            .collapsible(false)
            .resizable(false)
            .fixed_size([210.0, 72.0])
            .show(ctx, |ui| {
                ui.label("This will reset ALL app data & settings, including any unsaved NES data.");
                ui.separator();

                ui.with_layout(
                    egui::Layout::left_to_right(egui::Align::Center),
                    |ui| {
                        if ui.button("No").clicked() {
                            ui.close_kind(egui::UiKind::Window);
                        }

                        if ui.button("Yes").clicked() {
                            self.do_reset_app_data = Some(true);
                        }
                    },
                );

                match self.do_reset_app_data {
                    Some(true) => {
                        ui.separator();
                        ui.label("Resetting app data...");
                        ui.label(format!("{:?}", self.do_reset_app_data));
                    }
                    Some(false) => {
                        self.do_reset_app_data = None;
                        ui.close_kind(egui::UiKind::Window);
                    }
                    None => {}
                }
            });
    }

    fn bottom_panel(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.horizontal_centered(|ui| {
                if ui
                    .add_enabled(
                        self.nes_state != crate::NESState::Stopped,
                        egui::Button::image(Image::new(include_image!("assets/stop.svg")))
                        .image_tint_follows_text_color(true)
                    )
                    .clicked()
                {
                    self.request_nes_event.push(crate::NESEvent::Stop);
                }

                if ui
                    .add_enabled(
                        self.nes_state != crate::NESState::Stopped,
                        egui::Button::image(Image::new(match self.is_paused {
                            true => include_image!("assets/play.svg"),
                            false => include_image!("assets/pause.svg"),
                        }))
                        .image_tint_follows_text_color(true),
                    )
                    .clicked()
                {
                    self.is_paused = !self.is_paused;
                    match self.is_paused {
                        true => self.request_nes_event.push(crate::NESEvent::Pause),
                        false => self.request_nes_event.push(crate::NESEvent::Resume),
                    };
                }

                ui.add(egui::Slider::new(&mut self.volume, 0.0..=1.0).text("Volume"));
            });
        });
    }
}

macro_rules! define_key_mapping {
    (@internal $self:ident, $ui:ident, $input:ident, $id_prefix:literal, $($key:tt)+) => {
        $ui.label(::std::format!("{}:", ::std::stringify!($($key)+).to_uppercase()));
        $ui.add($crate::widgets::input_select::InputSelect::new(
            $input,
            ::std::option::Option::Some(&mut $self.input_manager.keyboard_input_mapping.0.$($key)+),
            ::std::concat!($id_prefix, ::std::stringify!($($key)+), "-key"),
            $crate::input::InputType::Keyboard,
        ));
        $ui.add_enabled(
            $self.input_manager.selected_controllers.0.is_some(),
            $crate::widgets::input_select::InputSelect::new(
                $input,
                $self.input_manager.selected_controllers.0.map(|id| {
                    &mut $self
                        .input_manager
                        .controller_input_mapping
                        .get_mut(&id)
                        .unwrap()
                        .input_mapping
                        .$($key)+
                }),
                ::std::concat!($id_prefix, ::std::stringify!($($key)+), "-gamepad"),
                $crate::input::InputType::Controller,
            )
        );
        $ui.end_row();
    };

    ($self:ident, $ui:ident, input: $input:ident, key: $($key:tt)+) => {
        define_key_mapping!(@internal $self, $ui, $input, "con1-", $($key)+)
    };

    (sys $self:ident, $ui:ident, input: $input:ident, key: $($key:tt)+) => {
        define_key_mapping!(@internal $self, $ui, $input, "", $($key)+)
    }
}

impl App {
    fn controller_config(&mut self, ctx: &egui::Context) {
        ctx.show_viewport_immediate(
            ViewportId::from_hash_of("controller_config"),
            ViewportBuilder::default()
                .with_inner_size([400.0, 400.0])
                .with_title("Configure Controllers"),
            |ctx: &egui::Context, _class: egui::ViewportClass| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    egui::Grid::new("controller_config_grid").show(ui, |ui| {
                        if ctx.input(|ui: &egui::InputState| ui.focused) {
                            self.input_manager.get_pressed_input(ctx);
                        }

                        let maybe_input: Option<Input> =
                            self.input_manager.held_input.iter().next().copied();

                        ui.label("");
                        ui.scope(|ui| {
                            ui.style_mut().visuals.widgets.noninteractive.bg_stroke =
                                egui::Stroke::NONE;
                            ui.group(|ui| {
                                ui.add_sized(
                                    [40.0, 40.0],
                                    egui::Image::new(include_image!("assets/keyboard.svg")),
                                );
                                ui.allocate_space(egui::vec2(10.0, 1.0));
                            });
                        });
                        ui.add_sized(
                            [40.0, 40.0],
                            egui::Image::new(include_image!("assets/gamepad.svg")),
                        );
                        ui.end_row();

                        define_key_mapping!(self, ui, input: maybe_input, key: up);
                        define_key_mapping!(self, ui, input: maybe_input, key: down);
                        define_key_mapping!(self, ui, input: maybe_input, key: left);
                        define_key_mapping!(self, ui, input: maybe_input, key: right);
                        define_key_mapping!(self, ui, input: maybe_input, key: a);
                        define_key_mapping!(self, ui, input: maybe_input, key: b);
                        define_key_mapping!(self, ui, input: maybe_input, key: select);
                        define_key_mapping!(self, ui, input: maybe_input, key: start);

                        ui.separator();
                        ui.separator();
                        ui.separator();
                        ui.end_row();

                        define_key_mapping!(sys self, ui, input: maybe_input, key: pause);
                        define_key_mapping!(sys self, ui, input: maybe_input, key: rewind);
                        // TODO: Do some token tree magic to replace spaces with _ and -
                        define_key_mapping!(sys self, ui, input: maybe_input, key: fast_forward);

                        ui.label("");
                        ui.label("");

                        egui::ComboBox::from_id_salt("controller_select")
                            .selected_text(self.input_manager.selected_controllers.0.map_or(
                                "None",
                                |con: uuid::Uuid| -> &str {
                                    self.input_manager
                                        .controller_input_mapping
                                        .get(&con)
                                        .unwrap()
                                        .name
                                        .as_str()
                                },
                            ))
                            .show_ui(ui, |ui| {
                                ui.selectable_value(
                                    &mut self.input_manager.selected_controllers.0,
                                    None,
                                    "None",
                                );
                                for (uuid, controller_config) in
                                    self.input_manager.controller_input_mapping.iter()
                                {
                                    ui.horizontal(|ui| {
                                        ui.selectable_value(
                                            &mut self.input_manager.selected_controllers.0,
                                            Some(*uuid),
                                            &controller_config.name,
                                        );
                                    });
                                }
                            });

                        ui.end_row();
                    });
                });

                if ctx.input(|i: &egui::InputState| i.viewport().close_requested()) {
                    self.show_controller_config = false;
                }
            },
        );
    }
}
