use imgui::{Context, FontSource, Style, StyleColor};

pub fn apply_context(imgui_context: &mut Context) {
    apply_style(imgui_context.style_mut());

    const FONT_SOURCE: [FontSource<'_>; 1] = [FontSource::TtfData {
        data: include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/third_party/CascadiaMono.ttf"
        )),
        size_pixels: 14.0,
        config: None,
    }];
    imgui_context.fonts().clear();
    imgui_context.fonts().add_font(FONT_SOURCE.as_slice());
}

pub fn apply_style(imgui_style: &mut Style) {
    let colors: &mut [[f32; 4]; StyleColor::COUNT] = &mut imgui_style.colors;

    colors[StyleColor::WindowBg as usize] = [0.0, 0.0, 0.0, 1.0];
    colors[StyleColor::TitleBgCollapsed as usize] = [0.0, 0.0, 0.0, 1.0];
    colors[StyleColor::WindowBg as usize] = [0.15, 0.15, 0.15, 1.0];
}
