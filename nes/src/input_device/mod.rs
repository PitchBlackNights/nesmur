pub mod joypad;
use std::any::Any;

pub enum NESDeviceType {
    Joypad,
}

pub trait NESDevice {
    fn read(&mut self) -> u8;
    fn write(&mut self, data: u8);
    fn get_type(&self) -> NESDeviceType;
    fn set_button_pressed_status(&mut self, button: Box<dyn NESDeviceButton>, pressed: bool);
}

pub trait NESDeviceButton {
    fn get_device_type(&self) -> NESDeviceType;
    fn as_any(&self) -> &dyn Any;
}

pub fn new_device(device_type: NESDeviceType) -> Box<dyn NESDevice> {
    match device_type {
        NESDeviceType::Joypad => Box::new(joypad::Joypad::new()),
    }
}
