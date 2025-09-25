use super::renderer::RGB;
use std::sync::LazyLock;

pub static NTSC: LazyLock<[RGB; 64]> = LazyLock::new(|| -> [RGB; 64] {
    let bytes: &'static [u8; 1536] = include_bytes!("./NTSC.pal");
    let colors: Vec<RGB> = bytes
        .chunks(3)
        .take(64)
        .map(|rgb: &[u8]| RGB(rgb[0], rgb[1], rgb[2]))
        .collect();
    colors.try_into().unwrap()
});

pub static PAL: LazyLock<[RGB; 64]> = LazyLock::new(|| -> [RGB; 64] {
    let bytes: &'static [u8; 1536] = include_bytes!("./PAL.pal");
    let colors: Vec<RGB> = bytes
        .chunks(3)
        .take(64)
        .map(|rgb: &[u8]| RGB(rgb[0], rgb[1], rgb[2]))
        .collect();
    colors.try_into().unwrap()
});
