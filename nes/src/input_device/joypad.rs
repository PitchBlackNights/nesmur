use crate::input_device::{NESDevice, NESDeviceButton, NESDeviceType};
use crate::prelude::*;
use std::any::Any;

bitflags! {
    // https://wiki.nesdev.com/w/index.php/Controller_reading_code
    #[derive(Clone, Copy)]
    pub struct JoypadButton: u8 {
        const RIGHT    = 0b10000000;
        const LEFT     = 0b01000000;
        const DOWN     = 0b00100000;
        const UP       = 0b00010000;
        const START    = 0b00001000;
        const SELECT   = 0b00000100;
        const BUTTON_B = 0b00000010;
        const BUTTON_A = 0b00000001;
    }
}

impl NESDeviceButton for JoypadButton {
    fn get_device_type(&self) -> NESDeviceType {
        NESDeviceType::Joypad
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct Joypad {
    strobe: bool,
    button_index: u8,
    button_status: JoypadButton,
}

impl Joypad {
    pub fn new() -> Self {
        Joypad {
            strobe: false,
            button_index: 0,
            button_status: JoypadButton::from_bits_truncate(0),
        }
    }
}

impl NESDevice for Joypad {
    fn read(&mut self) -> u8 {
        if self.button_index > 7 {
            return 1;
        }
        let response: u8 =
            (self.button_status.bits() & (1 << self.button_index)) >> self.button_index;
        if !self.strobe && self.button_index <= 7 {
            self.button_index += 1;
        }
        response
    }

    fn write(&mut self, data: u8) {
        self.strobe = data & 1 == 1;
        if self.strobe {
            self.button_index = 0
        }
    }

    fn get_type(&self) -> NESDeviceType {
        NESDeviceType::Joypad
    }

    fn set_button_pressed_status(&mut self, button: Box<dyn NESDeviceButton>, pressed: bool) {
        if let Some(jp_button) = button.as_any().downcast_ref::<JoypadButton>() {
            self.button_status.set(*jp_button, pressed);
        }
    }
}
