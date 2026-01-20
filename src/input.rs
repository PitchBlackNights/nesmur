use crate::app::AppConfig;
use eframe::egui;
use gilrs::Gilrs;
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    fmt::{Display, Formatter},
};
use uuid::Uuid;

const AXIS_DEADZONE: f32 = 0.1;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum Input {
    Key(egui::Key),
    ControllerButton(gilrs::ev::Button),
    ControllerAxis(gilrs::ev::Axis, bool),
    #[default]
    Unspecified,
}

impl Input {
    pub fn specified_and(self, f: impl FnOnce(Self) -> bool) -> bool {
        match self {
            Input::Unspecified => false,
            x => f(x),
        }
    }
}

impl Display for Input {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Key(k) => write!(f, "{:?}", *k),
            Self::ControllerButton(b) => write!(f, "{:?}", *b),
            Self::ControllerAxis(a, dir) => write!(f, "{:?} {}", *a, if *dir { "+" } else { "-" }),
            Self::Unspecified => write!(f, ""),
        }
    }
}

#[derive(PartialEq, Eq)]
pub enum InputType {
    Keyboard,
    Controller,
}

#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct InputMapping {
    pub up: Input,
    pub down: Input,
    pub left: Input,
    pub right: Input,
    pub b: Input,
    pub a: Input,
    pub start: Input,
    pub select: Input,
    pub pause: Input,
    pub rewind: Input,
    pub fast_forward: Input,
}

impl InputMapping {
    pub fn default_keyboard() -> Self {
        InputMapping {
            up: Input::Key(egui::Key::ArrowUp),
            down: Input::Key(egui::Key::ArrowDown),
            left: Input::Key(egui::Key::ArrowLeft),
            right: Input::Key(egui::Key::ArrowRight),
            b: Input::Key(egui::Key::S),
            a: Input::Key(egui::Key::A),
            start: Input::Key(egui::Key::Enter),
            select: Input::Key(egui::Key::Space),
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControllerConfig {
    pub name: String,
    pub input_mapping: InputMapping,
}

pub struct NesButtonState {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub b: bool,
    pub a: bool,
    pub start: bool,
    pub select: bool,
}

pub struct InputManager {
    pub gilrs: Gilrs,
    pub selected_controllers: (Option<Uuid>, Option<Uuid>),
    /// Controller to NES button mapping
    pub controller_input_mapping: HashMap<Uuid, ControllerConfig>,
    /// Keyboard to NES button mapping
    pub keyboard_input_mapping: (InputMapping, InputMapping),
    pub held_input: HashSet<Input>,
    pub pressed_input: HashSet<Input>,
}

impl InputManager {
    pub fn new(config: &AppConfig) -> Self {
        InputManager {
            gilrs: Gilrs::new().unwrap(),
            selected_controllers: config.selected_controllers,
            controller_input_mapping: config.controller_input_mapping.clone(),
            keyboard_input_mapping: config.keyboard_input_mapping,
            held_input: HashSet::with_capacity(32),
            pressed_input: HashSet::with_capacity(32),
        }
    }

    pub fn get_pressed_input(&mut self, ctx: &egui::Context) {
        // TODO: Only process selected controller with UUID
        self.pressed_input.clear();
        for event in std::iter::from_fn(|| self.gilrs.next_event()) {
            match event {
                gilrs::Event {
                    event: gilrs::EventType::ButtonPressed(button, _),
                    ..
                } => {
                    self.held_input.insert(Input::ControllerButton(button));
                    self.pressed_input.insert(Input::ControllerButton(button));
                }
                gilrs::Event {
                    event: gilrs::EventType::ButtonReleased(button, _),
                    ..
                } => {
                    self.held_input.remove(&Input::ControllerButton(button));
                }
                gilrs::Event {
                    event: gilrs::EventType::AxisChanged(axis, position, _),
                    ..
                } => {
                    if (-1.0..=-AXIS_DEADZONE).contains(&position) {
                        self.held_input.remove(&Input::ControllerAxis(axis, true));
                        self.held_input.insert(Input::ControllerAxis(axis, false));
                    } else if (AXIS_DEADZONE..=1.0).contains(&position) {
                        self.held_input.remove(&Input::ControllerAxis(axis, false));
                        self.held_input.insert(Input::ControllerAxis(axis, true));
                    } else if (-AXIS_DEADZONE..=AXIS_DEADZONE).contains(&position) {
                        self.held_input.remove(&Input::ControllerAxis(axis, false));
                        self.held_input.remove(&Input::ControllerAxis(axis, true));
                    }
                }
                _ => {}
            }
        }

        for (_id, gamepad) in self.gilrs.gamepads() {
            let _ = self.controller_input_mapping.insert(
                Uuid::from_slice(&gamepad.uuid()).unwrap(),
                ControllerConfig {
                    name: gamepad.name().to_owned(),
                    input_mapping: InputMapping::default(),
                },
            );
        }

        ctx.input(|input_state| {
            for event in input_state.events.iter() {
                match event {
                    egui::Event::Key {
                        key,
                        pressed: true,
                        repeat: false,
                        ..
                    } => {
                        self.held_input.insert(Input::Key(*key));
                    }
                    egui::Event::Key {
                        key,
                        pressed: false,
                        repeat: false,
                        ..
                    } => {
                        self.held_input.remove(&Input::Key(*key));
                    }
                    _ => {}
                }
            }
        });
    }
}
