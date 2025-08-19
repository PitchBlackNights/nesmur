use nes::cartridge::Rom;
use nesmur::cli_parser::Args;
use nesmur::prelude::*;
use nesmur::render::frame::Frame;
use nesmur::render::palette;
use nesmur::setup;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::{Sdl, VideoSubsystem};
use sdl2::video::{Window, WindowContext};
use sdl2::render::{Canvas, TextureCreator, Texture};
use sdl2::EventPump;
// use nes::NES;

fn show_tile(chr_rom: &Vec<u8>, bank: usize, tile_n: usize) -> Frame {
    assert!(bank <= 1);

    let mut frame: Frame = Frame::new();
    let bank: usize = (bank * 0x1000) as usize;

    let tile: &[u8] = &chr_rom[(bank + tile_n * 16)..=(bank + tile_n * 16 + 15)];

    for y in 0..=7 {
        let mut upper: u8 = tile[y];
        let mut lower: u8 = tile[y + 8];

        for x in (0..=7).rev() {
            let value: u8 = (1 & upper) << 1 | (1 & lower);
            upper = upper >> 1;
            lower = lower >> 1;
            let rgb: (u8, u8, u8) = match value {
                0 => palette::SYSTEM_PALLETE[0x01],
                1 => palette::SYSTEM_PALLETE[0x23],
                2 => palette::SYSTEM_PALLETE[0x27],
                3 => palette::SYSTEM_PALLETE[0x30],
                _ => panic!("This shouldn't happen!"),
            };
            frame.set_pixel(x, y, rgb)
        }
    }

    frame
}

fn show_tile_bank(chr_rom: &Vec<u8>, bank: usize) ->Frame {
    assert!(bank <= 1);

    let mut frame: Frame = Frame::new();
    let mut tile_y: usize = 0;
    let mut tile_x: usize = 0;
    let bank: usize = (bank * 0x1000) as usize;

    for tile_n in 0..255 {
        if tile_n != 0 && tile_n % 20 == 0 {
            tile_y += 10;
            tile_x = 0;
        }
        let tile: &[u8] = &chr_rom[(bank + tile_n * 16)..=(bank + tile_n * 16 + 15)];

        for y in 0..=7 {
            let mut upper: u8 = tile[y];
            let mut lower: u8 = tile[y + 8];

            for x in (0..=7).rev() {
                let value: u8 = (1 & upper) << 1 | (1 & lower);
                upper = upper >> 1;
                lower = lower >> 1;
                let rgb: (u8, u8, u8) = match value {
                    0 => palette::SYSTEM_PALLETE[0x01],
                    1 => palette::SYSTEM_PALLETE[0x23],
                    2 => palette::SYSTEM_PALLETE[0x27],
                    3 => palette::SYSTEM_PALLETE[0x30],
                    _ => panic!("This shouldn't happen!"),
                };
                frame.set_pixel(tile_x + x, tile_y + y, rgb)
            }
        }

        tile_x += 10;
    }
    frame
}

fn main() {
    let _args: Args = setup::setup_logger_and_args();
    info!("Starting Emulator...");

    let sdl_context: Sdl = sdl2::init().unwrap();
    let video_subsystem: VideoSubsystem = sdl_context.video().unwrap();
    let window: Window = video_subsystem
        .window("Tile viewer", (256.0 * 3.0) as u32, (240.0 * 3.0) as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas: Canvas<Window> = window.into_canvas().present_vsync().build().unwrap();
    let mut event_pump: EventPump = sdl_context.event_pump().unwrap();
    canvas.set_scale(3.0, 3.0).unwrap();

    let creator: TextureCreator<WindowContext> = canvas.texture_creator();
    let mut texture: Texture<'_> = creator
        .create_texture_target(PixelFormatEnum::RGB24, 256, 240)
        .unwrap();

    let bytes: Vec<u8> = std::fs::read("pacman.nes").unwrap();
    let rom: Rom = Rom::new(&bytes).unwrap();

    let right_bank: Frame = show_tile_bank(&rom.chr_rom, 0);

    texture.update(None, &right_bank.data, 256 * 3).unwrap();
    canvas.copy(&texture, None, None).unwrap();
    canvas.present();

    loop {
        for event in event_pump.poll_iter() {
            match event {
              Event::Quit { .. }
              | Event::KeyDown {
                  keycode: Some(Keycode::Escape),
                  ..
              } => std::process::exit(0),
              _ => { /* do nothing */ }
            }
         }
    }

    info!("Stopping Emulator...");
}
