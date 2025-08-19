use crate::ppu::renderer::RGB;
use std::sync::LazyLock;

pub static NTSC: LazyLock<[RGB; 64]> = LazyLock::new(|| {
    let bytes = include_bytes!("./NTSC.pal");
    let colors: Vec<RGB> = bytes
        .chunks(3)
        .take(64)
        .map(|rgb| RGB(rgb[0], rgb[1], rgb[2]))
        .collect();
    colors.try_into().unwrap()
});

pub static PAL: LazyLock<[RGB; 64]> = LazyLock::new(|| {
    let bytes = include_bytes!("./PAL.pal");
    let colors: Vec<RGB> = bytes
        .chunks(3)
        .take(64)
        .map(|rgb| RGB(rgb[0], rgb[1], rgb[2]))
        .collect();
    colors.try_into().unwrap()
});
