use nes::cartridge::ROM;
use nes::input_device::joypad::JoypadButton;
use nes::input_device::NESDeviceType;
use nes::ppu::PPU;
use nes::NES;
use nes::{BoxNESDevice, RcRef};
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
use std::cell::{Ref, RefMut};
use std::collections::HashMap;
// use nes::cpu::CPU;
// use std::fs::File;
// use std::io::{BufWriter, Write};

fn main() {
    let _args: Args = setup::setup_logger_and_args();
    info!("Starting Emulator...");

    // Init SDL2
    let sdl_context: Sdl = sdl2::init().unwrap();
    let video_subsystem: VideoSubsystem = sdl_context.video().unwrap();
    let window: Window = video_subsystem
        .window("NESMUR", (256.0 * 2.0) as u32, (240.0 * 2.0) as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas: Canvas<Window> = window.into_canvas().present_vsync().build().unwrap();
    let mut event_pump: EventPump = sdl_context.event_pump().unwrap();
    canvas.set_scale(2.0, 2.0).unwrap();

    let creator: TextureCreator<WindowContext> = canvas.texture_creator();
    let mut texture: Texture<'_> = creator
        .create_texture_target(PixelFormatEnum::RGB24, 256, 240)
        .unwrap();
    let mut frame: Frame = Frame::new();

    let mut key_map: HashMap<Keycode, (u8, JoypadButton)> = HashMap::new();
    key_map.insert(Keycode::Down, (1, JoypadButton::DOWN));
    key_map.insert(Keycode::Up, (1, JoypadButton::UP));
    key_map.insert(Keycode::Right, (1, JoypadButton::RIGHT));
    key_map.insert(Keycode::Left, (1, JoypadButton::LEFT));
    key_map.insert(Keycode::Space, (1, JoypadButton::SELECT));
    key_map.insert(Keycode::Return, (1, JoypadButton::START));
    key_map.insert(Keycode::A, (1, JoypadButton::BUTTON_A));
    key_map.insert(Keycode::S, (1, JoypadButton::BUTTON_B));

    // Setup the NES
    // let bytes: Vec<u8> = std::fs::read("nes/tests/roms/instr_timing.nes").unwrap();
    let bytes: Vec<u8> = std::fs::read("smb.nes").unwrap();
    let rom: ROM = ROM::new(&bytes).unwrap();

    let mut nes: NES = NES::new(
        rom,
        move |ppu_ref: RcRef<PPU>,
              device1_ref: &mut Option<RcRef<BoxNESDevice>>,
              device2_ref: &mut Option<RcRef<BoxNESDevice>>| {
            let ppu: Ref<PPU> = ppu_ref.borrow();
            let mut device1: Option<RefMut<BoxNESDevice>> = match device1_ref {
                Some(device_ref) => Some(device_ref.borrow_mut()),
                None => None,
            };
            let mut device2: Option<RefMut<BoxNESDevice>> = match device2_ref {
                Some(device_ref) => Some(device_ref.borrow_mut()),
                None => None,
            };

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
                        if let Some((device_num, key)) =
                            key_map.get(&keycode.unwrap_or(Keycode::Ampersand))
                        {
                            match device_num {
                                1 => {
                                    if let Some(device) = device1.as_mut() {
                                        device.set_button_pressed_status(Box::new(*key), true);
                                    }
                                }
                                2 => {
                                    if let Some(device) = device2.as_mut() {
                                        device.set_button_pressed_status(Box::new(*key), true);
                                    }
                                }
                                _ => {}
                            }
                        }
                    }

                    Event::KeyUp { keycode, .. } => {
                        if let Some((device_num, key)) =
                            key_map.get(&keycode.unwrap_or(Keycode::Ampersand))
                        {
                            match device_num {
                                1 => {
                                    if let Some(device) = device1.as_mut() {
                                        device.set_button_pressed_status(Box::new(*key), false);
                                    }
                                }
                                2 => {
                                    if let Some(device) = device2.as_mut() {
                                        device.set_button_pressed_status(Box::new(*key), false);
                                    }
                                }
                                _ => {}
                            }
                        }
                    }

                    _ => { /* do nothing */ }
                }
            }
        },
    );

    nes::bus::set_quiet_log(true);
    nes.connect_input_device(1, NESDeviceType::Joypad);
    nes.cpu.run();

    // let mut debug_log: String = String::new();
    // nes.cpu.run_with_callback(|cpu: &mut CPU<'_>| {
    //     let trace: String = nes::tools::trace(cpu);
    //     debug_log += format!("{}\n", trace).as_str();
    //     // debug!("{}", trace);
    // });
    // let mut log_file: BufWriter<File> = BufWriter::new(File::create("test.log").unwrap());
    // write!(&mut log_file, "{}", &debug_log).unwrap();
    // log_file.flush().unwrap();

    info!("Stopping Emulator...");
}
