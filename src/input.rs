use crate::math::Vec2;
use std::collections::HashSet;
pub use winit::event::VirtualKeyCode as Key;
use winit::event::{DeviceEvent, ElementState, Event, MouseButton, WindowEvent};

pub struct FrameInput {
    pub pressed_keys: HashSet<Key>,
    pub pressed_mouse_buttons: HashSet<MouseButton>,

    pub cursor_change: Vec2<f64>,
    pub scroll_change: Vec2<f64>,
}
impl FrameInput {
    pub fn new() -> Self {
        Self {
            pressed_keys: HashSet::new(),
            pressed_mouse_buttons: HashSet::new(),

            cursor_change: Vec2::all(0.0),
            scroll_change: Vec2::all(0.0),
        }
    }

    pub fn clear(&mut self) {
        self.cursor_change = Vec2::all(0.0);
        self.scroll_change = Vec2::all(0.0);
    }

    #[inline(always)]
    pub fn cursor_change(&self) -> Vec2<f64> {
        self.cursor_change
    }
    #[inline(always)]
    pub fn scroll_change(&self) -> Vec2<f64> {
        self.scroll_change
    }

    #[inline(always)]
    pub fn key_pressed(&self, key: Key) -> bool {
        self.pressed_keys.contains(&key)
    }
    #[inline(always)]
    pub fn mouse_button_pressed(&self, button: MouseButton) -> bool {
        self.pressed_mouse_buttons.contains(&button)
    }

    pub fn capture_event(&mut self, event: &Event<()>) -> bool {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::KeyboardInput { input, .. } => {
                    if let Some(key) = input.virtual_keycode {
                        if input.state == ElementState::Pressed {
                            self.pressed_keys.insert(key);
                        } else {
                            self.pressed_keys.remove(&key);
                        }
                    }
                    true
                }
                _ => false,
            },
            Event::DeviceEvent { event, .. } => match event {
                DeviceEvent::MouseMotion { delta, .. } => {
                    self.cursor_change.x += delta.0;
                    self.cursor_change.y += delta.1;
                    true
                }
                _ => false,
            },
            _ => false,
        }
    }
}
