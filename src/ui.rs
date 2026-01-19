use crate::{
    app::App,
    input::Input,
    prelude::*,
};
use eframe::egui::{self, include_image, load::SizedTexture, Image, ViewportBuilder, ViewportId};

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
    pub fn ui_draw_top_panel(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal_centered(|ui| {
                if ui.button("Load ROM").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_file() {
                        debug!("Loading ROM from path: {:?}", path);
                    }
                }

                if ui.button("Controller Config").clicked() {
                    self.show_controller_config = !self.show_controller_config
                }

                ui.separator();

                ui.add_sized(
                    [40.0, ui.available_height()],
                    egui::Label::new(format!("UI FPS: {:.0}", self.avg_framerate)),
                );

                ui.add_sized(
                    [40.0, ui.available_height()],
                    egui::Label::new(format!("UI FPS: {:.0}", self.avg_framerate)),
                );
            });
        });
    }

    pub fn ui_draw_center_panel(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add(
                Image::from_texture(SizedTexture::from_handle(&self.screen_texture))
                    .shrink_to_fit(),
            );
        });
    }

    pub fn ui_draw_bottom_panel(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.horizontal_centered(|ui| {
                if ui
                    .add(egui::Button::image(Image::new(match self.is_paused {
                        true => include_image!("assets/play.svg"),
                        false => include_image!("assets/pause.svg"),
                    })))
                    .clicked()
                {
                    self.is_paused = !self.is_paused;
                }

                ui.add(egui::Slider::new(&mut self.volume, 0.0..=1.0));
            });
        });
    }

    pub fn ui_draw_controller_config(&mut self, ctx: &egui::Context) {
        ctx.show_viewport_immediate(
            ViewportId::from_hash_of("controller_config"),
            ViewportBuilder::default().with_inner_size([500.0, 500.0]),
            |ctx, _class| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    egui::Grid::new("controller_config_grid").show(ui, |ui| {
                        if ctx.input(|ui| ui.focused) {
                            self.input_manager.get_pressed_input(ctx);
                        }

                        let maybe_input: Option<Input> =
                            self.input_manager.held_input.iter().next().copied();

                        ui.label("");
                        ui.image(include_image!("assets/keyboard.svg"));
                        ui.image(include_image!("assets/gamepad.svg"));
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
                                |con| {
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

                if ctx.input(|i| i.viewport().close_requested()) {
                    self.show_controller_config = false;
                }
            },
        );
    }
}
