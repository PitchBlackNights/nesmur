pub mod joypad;
use crate::BoxNESDevice;
use std::any::Any;

#[derive(Debug, Clone, Copy)]
pub enum NESDeviceType {
    Joypad,
}

// #[derive(Clone)]
pub trait NESDevice {
    fn read(&mut self) -> u8;
    fn write(&mut self, data: u8);
    fn get_type(&self) -> NESDeviceType;
    fn set_button_pressed_status(&mut self, button: Box<dyn NESDeviceButton>, pressed: bool);
}

// Add this to the NESDeviceButton trait (in nes/input_device.rs):
// trait NESDeviceButton: Any + Send + Sync {
//     fn box_clone(&self) -> Box<dyn NESDeviceButton>;
//     ...
// }

pub trait NESDeviceButton: Any + Send + Sync {
    fn box_clone(&self) -> Box<dyn NESDeviceButton>;
    fn get_device_type(&self) -> NESDeviceType;
    fn get_button_type_string(&self) -> &str;
    fn as_any(&self) -> &dyn Any;
}

pub fn new_device(device_type: NESDeviceType) -> BoxNESDevice {
    match device_type {
        NESDeviceType::Joypad => Box::new(joypad::Joypad::new()),
    }
}
