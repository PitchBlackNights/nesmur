use crate::input::{Input, InputType};
use egui::{Color32, FontId, Response, Ui, Vec2, Widget};
use std::sync::Arc;

const SPACING: f32 = 8.0;

pub struct InputSelect<'a> {
    pub pressed_input: Option<Input>,
    pub stored_input: Option<&'a mut Input>,
    pub unique_id: &'static str,
    pub input_type: InputType,
}

impl<'a> InputSelect<'a> {
    pub fn new(
        pressed_input: Option<Input>,
        stored_input: Option<&'a mut Input>,
        unique_id: &'static str,
        input_type: InputType,
    ) -> Self {
        InputSelect {
            pressed_input,
            stored_input,
            unique_id,
            input_type,
        }
    }
}

impl<'a> Widget for InputSelect<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let text: String = self
            .stored_input
            .as_ref()
            .map_or("".to_owned(), |input: &&mut Input| input.to_string());
        let text_font: FontId = if let Some(font) = &ui.style().override_font_id {
            font.clone()
        } else if let Some(style) = &ui.style().override_text_style {
            style.resolve(ui.style())
        } else {
            egui::TextStyle::Button.resolve(ui.style())
        };

        let text_layout: Arc<egui::Galley> =
            ui.painter()
                .layout_no_wrap(text.clone(), text_font.clone(), Color32::WHITE);

        let (rect, mut response) = ui.allocate_exact_size(
            Vec2 {
                y: text_layout.size().y + SPACING,
                x: (text_layout.size().x + SPACING).max(25.0),
            },
            egui::Sense::click(),
        );

        let state_id: egui::Id = ui.id().with(self.unique_id);
        let mut listening: bool = ui
            .ctx()
            .data(|x: &egui::util::IdTypeMap| x.get_temp::<bool>(state_id).unwrap_or(false));

        if !listening && response.clicked() {
            listening = true;
            response.mark_changed();
        } else if listening
            && (response.clicked_elsewhere()
                || self
                    .pressed_input
                    .is_some_and(|b: Input| b == Input::Key(egui::Key::Escape)))
        {
            listening = false;
            response.mark_changed();
        } else if listening
            && self.pressed_input.is_some_and(|i: Input| -> bool {
                (self.input_type == InputType::Keyboard && matches!(i, Input::Key(_)))
                    || (self.input_type == InputType::Controller
                        && matches!(i, Input::ControllerAxis(_, _) | Input::ControllerButton(_)))
            })
        {
            listening = false;
            if let Some(si) = self.stored_input
                && let Some(pressed_input) = self.pressed_input
            {
                *si = pressed_input;
            }
            response.mark_changed();
        }

        ui.data_mut(|data: &mut egui::util::IdTypeMap| data.insert_temp(state_id, listening));

        if ui.is_rect_visible(rect) {
            let visuals: egui::style::WidgetVisuals =
                ui.style().interact_selectable(&response, listening);
            ui.painter().rect(
                rect,
                3.0,
                visuals.bg_fill,
                visuals.bg_stroke,
                egui::StrokeKind::Middle,
            );

            let offset_pos: Vec2 = rect.center() - text_layout.rect.center();
            let text: Arc<egui::Galley> =
                ui.painter()
                    .layout_no_wrap(text, text_font, visuals.text_color());
            ui.painter()
                .galley(offset_pos.to_pos2(), text, visuals.text_color());
        }
        response
    }
}
