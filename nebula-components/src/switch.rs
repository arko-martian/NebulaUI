// Switch Component - Switch component for boolean values (iOS-style)
// Similar to Toggle but with different visual style

use nebula_core::layout::{LayoutEngine, NodeId};
use nebula_core::signal::Signal;

/// Switch component - iOS-style switch for boolean values
/// 
/// # Example
/// ```
/// let mut switch = Switch::new()
///     .checked(false)
///     .label("Dark mode")
///     .on_change(|checked| println!("Checked: {}", checked));
/// ```
pub struct Switch {
    pub node_id: Option<NodeId>,
    pub checked: Signal<bool>,
    pub disabled: bool,
    pub label: Option<String>,
    pub width: f32,
    pub height: f32,
    pub padding: f32,
    pub thumb_size: f32,
    pub track_color_off: (u8, u8, u8, u8),
    pub track_color_on: (u8, u8, u8, u8),
    pub thumb_color: (u8, u8, u8, u8),
    pub thumb_shadow: bool,
    pub disabled_color: (u8, u8, u8, u8),
    pub animate: bool,
    pub on_change: Option<Box<dyn Fn(bool)>>,
}

impl Switch {
    /// Create a new Switch component
    pub fn new() -> Self {
        Self {
            node_id: None,
            checked: Signal::new(false),
            disabled: false,
            label: None,
            width: 51.0,
            height: 31.0,
            padding: 2.0,
            thumb_size: 27.0,
            track_color_off: (120, 120, 128, 255),
            track_color_on: (52, 199, 89, 255), // iOS green
            thumb_color: (255, 255, 255, 255),
            thumb_shadow: true,
            disabled_color: (200, 200, 200, 255),
            animate: true,
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

    /// Set the padding
    pub fn padding(mut self, padding: f32) -> Self {
        self.padding = padding;
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

    /// Enable or disable thumb shadow
    pub fn thumb_shadow(mut self, shadow: bool) -> Self {
        self.thumb_shadow = shadow;
        self
    }

    /// Enable or disable animation
    pub fn animate(mut self, animate: bool) -> Self {
        self.animate = animate;
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

    /// Get the current track color
    pub fn get_track_color(&self) -> (u8, u8, u8, u8) {
        if self.is_checked() {
            self.track_color_on
        } else {
            self.track_color_off
        }
    }

    /// Get the thumb position (0.0 = left, 1.0 = right)
    pub fn get_thumb_position(&self) -> f32 {
        if self.is_checked() {
            1.0
        } else {
            0.0
        }
    }

    /// Build the switch layout
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
            .map_err(|e| format!("Failed to create switch node: {:?}", e))?;
        self.node_id = Some(node);

        Ok(node)
    }
}

impl Default for Switch {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn switch_starts_unchecked() {
        let switch = Switch::new();
        assert!(!switch.is_checked());
    }

    #[test]
    fn switch_set_checked() {
        let mut switch = Switch::new();
        switch.set_checked(true);
        assert!(switch.is_checked());
    }

    #[test]
    fn switch_toggle() {
        let mut switch = Switch::new();
        switch.toggle();
        assert!(switch.is_checked());
        switch.toggle();
        assert!(!switch.is_checked());
    }

    #[test]
    fn switch_disabled_cannot_change() {
        let mut switch = Switch::new().disabled(true);
        switch.set_checked(true);
        assert!(!switch.is_checked());
    }

    #[test]
    fn switch_disabled_cannot_toggle() {
        let mut switch = Switch::new().disabled(true);
        switch.toggle();
        assert!(!switch.is_checked());
    }

    #[test]
    fn switch_with_label() {
        let switch = Switch::new().label("Enable feature");
        assert!(switch.has_label());
        assert_eq!(switch.get_label(), Some("Enable feature"));
    }

    #[test]
    fn switch_track_color_changes() {
        let switch = Switch::new().checked(false);
        assert_eq!(switch.get_track_color(), switch.track_color_off);

        let switch = Switch::new().checked(true);
        assert_eq!(switch.get_track_color(), switch.track_color_on);
    }

    #[test]
    fn switch_thumb_position() {
        let switch = Switch::new().checked(false);
        assert_eq!(switch.get_thumb_position(), 0.0);

        let switch = Switch::new().checked(true);
        assert_eq!(switch.get_thumb_position(), 1.0);
    }

    #[test]
    fn switch_on_change_callback() {
        use std::sync::{Arc, Mutex};

        let changed = Arc::new(Mutex::new(false));
        let changed_clone = changed.clone();

        let mut switch = Switch::new().on_change(move |checked| {
            *changed_clone.lock().unwrap() = checked;
        });

        switch.set_checked(true);
        assert!(*changed.lock().unwrap());
    }

    #[test]
    fn switch_builder_pattern() {
        let switch = Switch::new()
            .checked(true)
            .label("Test label")
            .disabled(true)
            .width(60.0)
            .height(35.0)
            .padding(3.0)
            .thumb_size(30.0)
            .track_color_off(100, 100, 100, 255)
            .track_color_on(0, 255, 0, 255)
            .thumb_color(200, 200, 200, 255)
            .thumb_shadow(false)
            .animate(false);

        assert!(switch.is_checked());
        assert_eq!(switch.get_label(), Some("Test label"));
        assert!(switch.disabled);
        assert_eq!(switch.width, 60.0);
        assert_eq!(switch.height, 35.0);
        assert_eq!(switch.padding, 3.0);
        assert_eq!(switch.thumb_size, 30.0);
        assert_eq!(switch.track_color_off, (100, 100, 100, 255));
        assert_eq!(switch.track_color_on, (0, 255, 0, 255));
        assert_eq!(switch.thumb_color, (200, 200, 200, 255));
        assert!(!switch.thumb_shadow);
        assert!(!switch.animate);
    }

    #[test]
    fn switch_build_creates_node() {
        let mut engine = LayoutEngine::new();
        let mut switch = Switch::new();

        let result = switch.build(&mut engine);
        assert!(result.is_ok());
        assert!(switch.node_id.is_some());
    }

    #[test]
    fn switch_initial_checked_state() {
        let switch = Switch::new().checked(true);
        assert!(switch.is_checked());
    }

    #[test]
    fn switch_no_label() {
        let switch = Switch::new();
        assert!(!switch.has_label());
        assert_eq!(switch.get_label(), None);
    }

    #[test]
    fn switch_ios_style_defaults() {
        let switch = Switch::new();
        assert_eq!(switch.width, 51.0);
        assert_eq!(switch.height, 31.0);
        assert_eq!(switch.track_color_on, (52, 199, 89, 255)); // iOS green
        assert!(switch.thumb_shadow);
        assert!(switch.animate);
    }
}
