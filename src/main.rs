use nes::cartridge::Rom;
use nes::ppu::PPU;
use nes::NES;
use nesmur::cli_parser::Args;
use nesmur::render::frame::Frame;
use nesmur::setup;
use nesmur::{prelude::*, render};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};
use sdl2::EventPump;
use sdl2::{Sdl, VideoSubsystem};
use std::cell::{Ref, RefCell};
use std::rc::Rc;
use std::collections::HashMap;
use nes::joypad::JoypadButton;

fn main() {
    let _args: Args = setup::setup_logger_and_args();
    info!("Starting Emulator...");

    // let rom_bytes: Vec<u8> = std::fs::read("nes/tests/roms/nestest.nes").unwrap();
    // let rom: Rom = Rom::new(&rom_bytes).unwrap();
    // let mut nes: NES = NES::new(rom);

    // nes::bus::set_quiet_log(true);
    // nes.cpu.reset();
    // let mut instruction_cycle: u16 = 0;
    // nes.cpu.run_with_callback(|cpu| {
    //     instruction_cycle += 1;
    //     if instruction_cycle == 8992 {
    //         cpu.running = false;
    //     }
    // });

    // Init SDL2
    let sdl_context: Sdl = sdl2::init().unwrap();
    let video_subsystem: VideoSubsystem = sdl_context.video().unwrap();
    let window: Window = video_subsystem
        .window("NESMUR", (256.0 * 3.0) as u32, (240.0 * 3.0) as u32)
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
    let mut frame: Frame = Frame::new();

    let mut key_map: HashMap<Keycode, JoypadButton> = HashMap::new();
    key_map.insert(Keycode::Down, JoypadButton::DOWN);
    key_map.insert(Keycode::Up, JoypadButton::UP);
    key_map.insert(Keycode::Right, JoypadButton::RIGHT);
    key_map.insert(Keycode::Left, JoypadButton::LEFT);
    key_map.insert(Keycode::Space, JoypadButton::SELECT);
    key_map.insert(Keycode::Return, JoypadButton::START);
    key_map.insert(Keycode::A, JoypadButton::BUTTON_A);
    key_map.insert(Keycode::S, JoypadButton::BUTTON_B);

    // Setup the NES
    let bytes: Vec<u8> = std::fs::read("pacman.nes").unwrap();
    let rom: Rom = Rom::new(&bytes).unwrap();
    let mut nes: NES = NES::new(rom, move |ppu_ref: Rc<RefCell<PPU>>, joypad1| {
        let ppu: Ref<'_, PPU> = ppu_ref.borrow();

        render::render(&ppu, &mut frame);
        texture.update(None, &frame.data, 256 * 3).unwrap();
        canvas.copy(&texture, None, None).unwrap();
        canvas.present();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => std::process::exit(0),

                Event::KeyDown { keycode, .. } => {
                    if let Some(key) = key_map.get(&keycode.unwrap_or(Keycode::Ampersand)) {
                        joypad1.set_button_pressed_status(*key, true);
                    }
                }
                Event::KeyUp { keycode, .. } => {
                    if let Some(key) = key_map.get(&keycode.unwrap_or(Keycode::Ampersand)) {
                        joypad1.set_button_pressed_status(*key, false);
                    }
                }

                _ => { /* do nothing */ }
            }
        }
    });

    nes::bus::set_quiet_log(true);
    nes.cpu.reset();
    nes.cpu.run();

    info!("Stopping Emulator...");
}
