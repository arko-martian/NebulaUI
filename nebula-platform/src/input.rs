use winit::event::{ElementState, KeyEvent, MouseButton};
use winit::keyboard::{KeyCode, PhysicalKey};

/// Mouse button press event
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseButtonEvent {
    Left,
    Right,
    Middle,
    Other(u16),
}

impl From<MouseButton> for MouseButtonEvent {
    fn from(button: MouseButton) -> Self {
        match button {
            MouseButton::Left => MouseButtonEvent::Left,
            MouseButton::Right => MouseButtonEvent::Right,
            MouseButton::Middle => MouseButtonEvent::Middle,
            MouseButton::Back => MouseButtonEvent::Other(3),
            MouseButton::Forward => MouseButtonEvent::Other(4),
            MouseButton::Other(id) => MouseButtonEvent::Other(id),
        }
    }
}

/// Mouse position
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MousePosition {
    pub x: f64,
    pub y: f64,
}

impl MousePosition {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

/// Keyboard key codes (common keys)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Key {
    // Letters
    A, B, C, D, E, F, G, H, I, J, K, L, M,
    N, O, P, Q, R, S, T, U, V, W, X, Y, Z,
    
    // Numbers
    Num0, Num1, Num2, Num3, Num4, Num5, Num6, Num7, Num8, Num9,
    
    // Function keys
    F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12,
    
    // Special keys
    Escape,
    Enter,
    Space,
    Backspace,
    Tab,
    Delete,
    
    // Arrow keys
    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    
    // Modifiers
    Shift,
    Control,
    Alt,
    Meta, // Command on Mac, Windows key on Windows
    
    // Other
    Unknown,
}

impl From<KeyCode> for Key {
    fn from(code: KeyCode) -> Self {
        match code {
            // Letters
            KeyCode::KeyA => Key::A,
            KeyCode::KeyB => Key::B,
            KeyCode::KeyC => Key::C,
            KeyCode::KeyD => Key::D,
            KeyCode::KeyE => Key::E,
            KeyCode::KeyF => Key::F,
            KeyCode::KeyG => Key::G,
            KeyCode::KeyH => Key::H,
            KeyCode::KeyI => Key::I,
            KeyCode::KeyJ => Key::J,
            KeyCode::KeyK => Key::K,
            KeyCode::KeyL => Key::L,
            KeyCode::KeyM => Key::M,
            KeyCode::KeyN => Key::N,
            KeyCode::KeyO => Key::O,
            KeyCode::KeyP => Key::P,
            KeyCode::KeyQ => Key::Q,
            KeyCode::KeyR => Key::R,
            KeyCode::KeyS => Key::S,
            KeyCode::KeyT => Key::T,
            KeyCode::KeyU => Key::U,
            KeyCode::KeyV => Key::V,
            KeyCode::KeyW => Key::W,
            KeyCode::KeyX => Key::X,
            KeyCode::KeyY => Key::Y,
            KeyCode::KeyZ => Key::Z,
            
            // Numbers
            KeyCode::Digit0 => Key::Num0,
            KeyCode::Digit1 => Key::Num1,
            KeyCode::Digit2 => Key::Num2,
            KeyCode::Digit3 => Key::Num3,
            KeyCode::Digit4 => Key::Num4,
            KeyCode::Digit5 => Key::Num5,
            KeyCode::Digit6 => Key::Num6,
            KeyCode::Digit7 => Key::Num7,
            KeyCode::Digit8 => Key::Num8,
            KeyCode::Digit9 => Key::Num9,
            
            // Function keys
            KeyCode::F1 => Key::F1,
            KeyCode::F2 => Key::F2,
            KeyCode::F3 => Key::F3,
            KeyCode::F4 => Key::F4,
            KeyCode::F5 => Key::F5,
            KeyCode::F6 => Key::F6,
            KeyCode::F7 => Key::F7,
            KeyCode::F8 => Key::F8,
            KeyCode::F9 => Key::F9,
            KeyCode::F10 => Key::F10,
            KeyCode::F11 => Key::F11,
            KeyCode::F12 => Key::F12,
            
            // Special keys
            KeyCode::Escape => Key::Escape,
            KeyCode::Enter => Key::Enter,
            KeyCode::Space => Key::Space,
            KeyCode::Backspace => Key::Backspace,
            KeyCode::Tab => Key::Tab,
            KeyCode::Delete => Key::Delete,
            
            // Arrow keys
            KeyCode::ArrowUp => Key::ArrowUp,
            KeyCode::ArrowDown => Key::ArrowDown,
            KeyCode::ArrowLeft => Key::ArrowLeft,
            KeyCode::ArrowRight => Key::ArrowRight,
            
            // Modifiers
            KeyCode::ShiftLeft | KeyCode::ShiftRight => Key::Shift,
            KeyCode::ControlLeft | KeyCode::ControlRight => Key::Control,
            KeyCode::AltLeft | KeyCode::AltRight => Key::Alt,
            KeyCode::SuperLeft | KeyCode::SuperRight => Key::Meta,
            
            _ => Key::Unknown,
        }
    }
}

/// Input event handler trait
pub trait InputHandler {
    /// Called when a mouse button is pressed
    fn on_mouse_down(&mut self, button: MouseButtonEvent, position: MousePosition) {
        let _ = (button, position); // Default: do nothing
    }
    
    /// Called when a mouse button is released
    fn on_mouse_up(&mut self, button: MouseButtonEvent, position: MousePosition) {
        let _ = (button, position); // Default: do nothing
    }
    
    /// Called when the mouse moves
    fn on_mouse_move(&mut self, position: MousePosition) {
        let _ = position; // Default: do nothing
    }
    
    /// Called when a key is pressed
    fn on_key_down(&mut self, key: Key) {
        let _ = key; // Default: do nothing
    }
    
    /// Called when a key is released
    fn on_key_up(&mut self, key: Key) {
        let _ = key; // Default: do nothing
    }
}

/// Helper to extract key from KeyEvent
pub fn key_from_event(event: &KeyEvent) -> Option<Key> {
    if let PhysicalKey::Code(code) = event.physical_key {
        Some(Key::from(code))
    } else {
        None
    }
}

/// Helper to check if key event is pressed
pub fn is_key_pressed(event: &KeyEvent) -> bool {
    event.state == ElementState::Pressed
}

/// Helper to check if key event is released
pub fn is_key_released(event: &KeyEvent) -> bool {
    event.state == ElementState::Released
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mouse_position_creation() {
        let pos = MousePosition::new(100.0, 200.0);
        assert_eq!(pos.x, 100.0);
        assert_eq!(pos.y, 200.0);
    }

    #[test]
    fn mouse_button_conversion() {
        assert_eq!(MouseButtonEvent::from(MouseButton::Left), MouseButtonEvent::Left);
        assert_eq!(MouseButtonEvent::from(MouseButton::Right), MouseButtonEvent::Right);
        assert_eq!(MouseButtonEvent::from(MouseButton::Middle), MouseButtonEvent::Middle);
    }

    #[test]
    fn key_conversion() {
        assert_eq!(Key::from(KeyCode::KeyA), Key::A);
        assert_eq!(Key::from(KeyCode::Escape), Key::Escape);
        assert_eq!(Key::from(KeyCode::Enter), Key::Enter);
        assert_eq!(Key::from(KeyCode::Space), Key::Space);
        assert_eq!(Key::from(KeyCode::ArrowUp), Key::ArrowUp);
    }
}
