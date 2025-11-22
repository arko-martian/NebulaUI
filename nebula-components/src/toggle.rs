// Toggle Component - Toggle switch for boolean values
// Essential for on/off settings

use nebula_core::layout::{LayoutEngine, NodeId};
use nebula_core::signal::Signal;

/// Toggle component - toggle switch for boolean values
/// 
/// # Example
/// ```
/// let mut toggle = Toggle::new()
///     .checked(false)
///     .label("Enable notifications")
///     .on_change(|checked| println!("Checked: {}", checked));
/// ```
pub struct Toggle {
    pub node_id: Option<NodeId>,
    pub checked: Signal<bool>,
    pub disabled: bool,
    pub label: Option<String>,
    pub label_position: LabelPosition,
    pub width: f32,
    pub height: f32,
    pub thumb_size: f32,
    pub track_color_off: (u8, u8, u8, u8),
    pub track_color_on: (u8, u8, u8, u8),
    pub thumb_color: (u8, u8, u8, u8),
    pub disabled_color: (u8, u8, u8, u8),
    pub on_change: Option<Box<dyn Fn(bool)>>,
}

/// Label position for toggle
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LabelPosition {
    Left,
    Right,
    Top,
    Bottom,
}

impl Toggle {
    /// Create a new Toggle component
    pub fn new() -> Self {
        Self {
            node_id: None,
            checked: Signal::new(false),
            disabled: false,
            label: None,
            label_position: LabelPosition::Right,
            width: 48.0,
            height: 28.0,
            thumb_size: 24.0,
            track_color_off: (200, 200, 200, 255),
            track_color_on: (59, 130, 246, 255), // Blue
            thumb_color: (255, 255, 255, 255),
            disabled_color: (220, 220, 220, 255),
            on_change: None,
        }
    }

    /// Set the checked state
    pub fn checked(self, checked: bool) -> Self {
        self.checked.set(checked);
        self
    }

    /// Set the label
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Set the label position
    pub fn label_position(mut self, position: LabelPosition) -> Self {
        self.label_position = position;
        self
    }

    /// Set disabled state
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Set the width
    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    /// Set the height
    pub fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    /// Set the thumb size
    pub fn thumb_size(mut self, size: f32) -> Self {
        self.thumb_size = size;
        self
    }

    /// Set the track color when off
    pub fn track_color_off(mut self, r: u8, g: u8, b: u8, a: u8) -> Self {
        self.track_color_off = (r, g, b, a);
        self
    }

    /// Set the track color when on
    pub fn track_color_on(mut self, r: u8, g: u8, b: u8, a: u8) -> Self {
        self.track_color_on = (r, g, b, a);
        self
    }

    /// Set the thumb color
    pub fn thumb_color(mut self, r: u8, g: u8, b: u8, a: u8) -> Self {
        self.thumb_color = (r, g, b, a);
        self
    }

    /// Set the change callback
    pub fn on_change<F>(mut self, callback: F) -> Self
    where
        F: Fn(bool) + 'static,
    {
        self.on_change = Some(Box::new(callback));
        self
    }

    /// Toggle the checked state
    pub fn toggle(&mut self) {
        if !self.disabled {
            let new_state = !self.is_checked();
            self.set_checked(new_state);
        }
    }

    /// Set the checked state
    pub fn set_checked(&mut self, checked: bool) {
        if !self.disabled {
            self.checked.set(checked);
            if let Some(ref callback) = self.on_change {
                callback(checked);
            }
        }
    }

    /// Get the checked state
    pub fn is_checked(&self) -> bool {
        self.checked.get()
    }

    /// Check if has label
    pub fn has_label(&self) -> bool {
        self.label.is_some()
    }

    /// Get the label
    pub fn get_label(&self) -> Option<&str> {
        self.label.as_deref()
    }

    /// Build the toggle layout
    pub fn build(&mut self, engine: &mut LayoutEngine) -> Result<NodeId, String> {
        let style = taffy::style::Style {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Length(self.width),
                height: taffy::style::Dimension::Length(self.height),
            },
            display: taffy::style::Display::Flex,
            align_items: Some(taffy::style::AlignItems::Center),
            ..Default::default()
        };

        let node = engine
            .new_leaf(style)
            .map_err(|e| format!("Failed to create toggle node: {:?}", e))?;
        self.node_id = Some(node);

        Ok(node)
    }
}

impl Default for Toggle {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn toggle_starts_unchecked() {
        let toggle = Toggle::new();
        assert!(!toggle.is_checked());
    }

    #[test]
    fn toggle_set_checked() {
        let mut toggle = Toggle::new();
        toggle.set_checked(true);
        assert!(toggle.is_checked());
    }

    #[test]
    fn toggle_toggle() {
        let mut toggle = Toggle::new();
        toggle.toggle();
        assert!(toggle.is_checked());
        toggle.toggle();
        assert!(!toggle.is_checked());
    }

    #[test]
    fn toggle_disabled_cannot_change() {
        let mut toggle = Toggle::new().disabled(true);
        toggle.set_checked(true);
        assert!(!toggle.is_checked());
    }

    #[test]
    fn toggle_disabled_cannot_toggle() {
        let mut toggle = Toggle::new().disabled(true);
        toggle.toggle();
        assert!(!toggle.is_checked());
    }

    #[test]
    fn toggle_with_label() {
        let toggle = Toggle::new().label("Enable feature");
        assert!(toggle.has_label());
        assert_eq!(toggle.get_label(), Some("Enable feature"));
    }

    #[test]
    fn toggle_label_position() {
        let toggle = Toggle::new()
            .label("Test")
            .label_position(LabelPosition::Left);
        assert_eq!(toggle.label_position, LabelPosition::Left);
    }

    #[test]
    fn toggle_on_change_callback() {
        use std::sync::{Arc, Mutex};

        let changed = Arc::new(Mutex::new(false));
        let changed_clone = changed.clone();

        let mut toggle = Toggle::new().on_change(move |checked| {
            *changed_clone.lock().unwrap() = checked;
        });

        toggle.set_checked(true);
        assert!(*changed.lock().unwrap());
    }

    #[test]
    fn toggle_builder_pattern() {
        let toggle = Toggle::new()
            .checked(true)
            .label("Test label")
            .label_position(LabelPosition::Left)
            .disabled(true)
            .width(60.0)
            .height(32.0)
            .thumb_size(28.0)
            .track_color_off(100, 100, 100, 255)
            .track_color_on(0, 255, 0, 255)
            .thumb_color(200, 200, 200, 255);

        assert!(toggle.is_checked());
        assert_eq!(toggle.get_label(), Some("Test label"));
        assert_eq!(toggle.label_position, LabelPosition::Left);
        assert!(toggle.disabled);
        assert_eq!(toggle.width, 60.0);
        assert_eq!(toggle.height, 32.0);
        assert_eq!(toggle.thumb_size, 28.0);
        assert_eq!(toggle.track_color_off, (100, 100, 100, 255));
        assert_eq!(toggle.track_color_on, (0, 255, 0, 255));
        assert_eq!(toggle.thumb_color, (200, 200, 200, 255));
    }

    #[test]
    fn toggle_build_creates_node() {
        let mut engine = LayoutEngine::new();
        let mut toggle = Toggle::new();

        let result = toggle.build(&mut engine);
        assert!(result.is_ok());
        assert!(toggle.node_id.is_some());
    }

    #[test]
    fn toggle_initial_checked_state() {
        let toggle = Toggle::new().checked(true);
        assert!(toggle.is_checked());
    }

    #[test]
    fn toggle_no_label() {
        let toggle = Toggle::new();
        assert!(!toggle.has_label());
        assert_eq!(toggle.get_label(), None);
    }
}
