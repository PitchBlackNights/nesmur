use crate::{
    BoxNESDevice,
    input_device::{NESDeviceType, joypad::JoypadButton, new_device},
};

#[test]
fn test_strobe_mode() {
    let mut joypad: BoxNESDevice = new_device(NESDeviceType::Joypad);

    joypad.write(1);
    joypad.set_button_pressed_status(Box::new(JoypadButton::BUTTON_A), true);
    for _x in 0..10 {
        assert_eq!(joypad.read(), 1);
    }
}

#[test]
fn test_strobe_mode_on_off() {
    let mut joypad: BoxNESDevice = new_device(NESDeviceType::Joypad);

    joypad.write(0);
    joypad.set_button_pressed_status(Box::new(JoypadButton::RIGHT), true);
    joypad.set_button_pressed_status(Box::new(JoypadButton::LEFT), true);
    joypad.set_button_pressed_status(Box::new(JoypadButton::SELECT), true);
    joypad.set_button_pressed_status(Box::new(JoypadButton::BUTTON_B), true);

    for _ in 0..=1 {
        assert_eq!(joypad.read(), 0);
        assert_eq!(joypad.read(), 1);
        assert_eq!(joypad.read(), 1);
        assert_eq!(joypad.read(), 0);
        assert_eq!(joypad.read(), 0);
        assert_eq!(joypad.read(), 0);
        assert_eq!(joypad.read(), 1);
        assert_eq!(joypad.read(), 1);

        for _x in 0..10 {
            assert_eq!(joypad.read(), 1);
        }
        joypad.write(1);
        joypad.write(0);
    }
}
