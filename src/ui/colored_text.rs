const COLOR_MARKER_START: char = '{';
const COLOR_MARKER_END: char = '}';

pub trait ImGUIColoredText {
    fn text_with_colors(&self, text: String);
}

impl ImGUIColoredText for imgui::Ui {
    fn text_with_colors(&self, text: String) {
        let mut temp_new_color: String = String::new();
        let mut curr_text: String = String::new();
        let mut iter_color: bool = false;
        let mut text_color: Option<[f32; 4]> = None;

        for character in text.chars() {
            if character == COLOR_MARKER_START {
                if !curr_text.is_empty() {
                    if let Some(color) = text_color.take() {
                        self.text_colored(color, &curr_text);
                    } else {
                        self.text(&curr_text);
                    }
                    self.same_line_with_spacing(0.0, 0.0);
                    curr_text = String::new();
                }
                iter_color = true;
                temp_new_color = String::new();
            } else if character == COLOR_MARKER_END && iter_color {
                let color_result: Result<u32, std::num::ParseIntError> =
                    u32::from_str_radix(temp_new_color.as_str(), 16);
                if color_result.is_ok() && temp_new_color.len() == 6 {
                    let color: u32 = color_result.unwrap();
                    text_color = Some([
                        ((color & 0x00FF0000) >> 16) as f32 / 255.0,
                        ((color & 0x0000FF00) >> 8) as f32 / 255.0,
                        (color & 0x000000FF) as f32 / 255.0,
                        1.0,
                    ]);
                } else if temp_new_color.is_empty() {
                    text_color = None;
                } else {
                    curr_text += format!("{{{}}}", temp_new_color).as_str();
                }
                iter_color = false;
            } else if iter_color {
                temp_new_color.push(character);
            } else {
                curr_text.push(character);
            }
        }

        if !curr_text.is_empty() {
            if let Some(color) = text_color.take() {
                self.text_colored(color, &curr_text);
            } else {
                self.text(&curr_text);
            }
        } else {
            self.new_line();
        }
    }
}
