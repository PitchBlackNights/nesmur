use egui::Ui;

pub struct DebugVisuals {
    pub show_resize: bool,
    pub show_expanded_height: bool,
    pub show_expanded_width: bool,
    pub debug_hover: bool,
}

pub struct DebugOptions {
    pub visuals: DebugVisuals,
    pub show_settings: bool,
    pub show_inspection: bool,
    pub show_textures: bool,
    pub show_loaders: bool,
    pub show_memory: bool,
}

impl DebugOptions {
    pub fn new() -> Self {
        DebugOptions {
            visuals: DebugVisuals {
                show_resize: false,
                show_expanded_height: false,
                show_expanded_width: false,
                debug_hover: false,
            },
            show_settings: false,
            show_inspection: false,
            show_textures: false,
            show_loaders: false,
            show_memory: false,
        }
    }

    pub fn ui(&mut self, ui: &mut Ui) {
        ui.checkbox(&mut self.visuals.show_resize, "Show resize");
        ui.checkbox(
            &mut self.visuals.show_expanded_height,
            "Show expanded height",
        );
        ui.checkbox(&mut self.visuals.show_expanded_width, "Show expanded width");
        ui.checkbox(&mut self.visuals.debug_hover, "Debug hover");
        ui.separator();

        if ui.button("Settings").clicked() {
            self.show_settings = true;
        }
        if ui.button("Inspection").clicked() {
            self.show_inspection = true;
        }
        if ui.button("Textures").clicked() {
            self.show_textures = true;
        }
        if ui.button("Loaders").clicked() {
            self.show_loaders = true;
        }
        if ui.button("Memory").clicked() {
            self.show_memory = true;
        }
    }

    pub fn update(&mut self, ctx: &egui::Context) {
        ctx.style_mut(|s: &mut egui::Style| {
            s.debug.show_resize = self.visuals.show_resize;
            s.debug.show_expand_height = self.visuals.show_expanded_height;
            s.debug.show_expand_width = self.visuals.show_expanded_width;
            s.debug.debug_on_hover_with_all_modifiers = self.visuals.debug_hover;
        });

        if self.show_settings {
            egui::Window::new("Settings")
                .open(&mut self.show_settings)
                .vscroll(true)
                .show(ctx, |ui: &mut Ui| {
                    ctx.settings_ui(ui);
                });
        }

        if self.show_inspection {
            egui::Window::new("Inspection")
                .open(&mut self.show_inspection)
                .vscroll(true)
                .show(ctx, |ui: &mut Ui| {
                    ctx.inspection_ui(ui);
                });
        }

        if self.show_textures {
            egui::Window::new("Textures")
                .open(&mut self.show_textures)
                .vscroll(true)
                .show(ctx, |ui: &mut Ui| {
                    ctx.texture_ui(ui);
                });
        }

        if self.show_loaders {
            egui::Window::new("Loaders")
                .open(&mut self.show_loaders)
                .vscroll(true)
                .show(ctx, |ui: &mut Ui| {
                    ctx.loaders_ui(ui);
                });
        }

        if self.show_memory {
            egui::Window::new("Memory")
                .open(&mut self.show_memory)
                .vscroll(true)
                .show(ctx, |ui: &mut Ui| {
                    ctx.memory_ui(ui);
                });
        }
    }
}
