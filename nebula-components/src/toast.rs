// Toast Component - Temporary notification messages
// Auto-dismissing notifications that appear at screen edges

use nebula_core::layout::{LayoutEngine, NodeId};
use nebula_core::signal::Signal;

/// Toast type determines the visual style and icon
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToastType {
    Info,
    Success,
    Warning,
    Error,
}

/// Toast position on screen
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToastPosition {
    TopLeft,
    TopCenter,
    TopRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
}

/// Toast component - displays temporary notification messages
/// 
/// # Example
/// ```
/// let mut toast = Toast::new("Operation successful!")
///     .toast_type(ToastType::Success)
///     .position(ToastPosition::TopRight)
///     .duration(3000)
///     .closable(true);
/// ```
pub struct Toast {
    pub node_id: Option<NodeId>,
    pub message: String,
    pub toast_type: ToastType,
    pub position: ToastPosition,
    pub is_visible: Signal<bool>,
    pub duration: u32, // milliseconds (0 = no auto-dismiss)
    pub width: f32,
    pub padding: f32,
    pub margin: f32,
    pub background_color: Option<(u8, u8, u8, u8)>, // None = use type default
    pub text_color: Option<(u8, u8, u8, u8)>,
    pub border_radius: f32,
    pub closable: bool,
    pub show_icon: bool,
    pub on_close: Option<Box<dyn Fn()>>,
    pub on_click: Option<Box<dyn Fn()>>,
}

impl Toast {
    /// Create a new Toast component
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            node_id: None,
            message: message.into(),
            toast_type: ToastType::Info,
            position: ToastPosition::TopRight,
            is_visible: Signal::new(false),
            duration: 3000,
            width: 300.0,
            padding: 16.0,
            margin: 16.0,
            background_color: None,
            text_color: None,
            border_radius: 8.0,
            closable: true,
            show_icon: true,
            on_close: None,
            on_click: None,
        }
    }

    /// Set the message
    pub fn message(mut self, message: impl Into<String>) -> Self {
        self.message = message.into();
        self
    }

    /// Set the toast type
    pub fn toast_type(mut self, toast_type: ToastType) -> Self {
        self.toast_type = toast_type;
        self
    }

    /// Set the position
    pub fn position(mut self, position: ToastPosition) -> Self {
        self.position = position;
        self
    }

    /// Set the duration (0 = no auto-dismiss)
    pub fn duration(mut self, duration: u32) -> Self {
        self.duration = duration;
        self
    }

    /// Set the width
    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    /// Set the padding
    pub fn padding(mut self, padding: f32) -> Self {
        self.padding = padding;
        self
    }

    /// Set the margin from screen edge
    pub fn margin(mut self, margin: f32) -> Self {
        self.margin = margin;
        self
    }

    /// Set custom background color
    pub fn background_color(mut self, r: u8, g: u8, b: u8, a: u8) -> Self {
        self.background_color = Some((r, g, b, a));
        self
    }

    /// Set custom text color
    pub fn text_color(mut self, r: u8, g: u8, b: u8, a: u8) -> Self {
        self.text_color = Some((r, g, b, a));
        self
    }

    /// Set the border radius
    pub fn border_radius(mut self, radius: f32) -> Self {
        self.border_radius = radius;
        self
    }

    /// Set whether the toast is closable
    pub fn closable(mut self, closable: bool) -> Self {
        self.closable = closable;
        self
    }

    /// Set whether to show icon
    pub fn show_icon(mut self, show: bool) -> Self {
        self.show_icon = show;
        self
    }

    /// Set the close callback
    pub fn on_close<F>(mut self, callback: F) -> Self
    where
        F: Fn() + 'static,
    {
        self.on_close = Some(Box::new(callback));
        self
    }

    /// Set the click callback
    pub fn on_click<F>(mut self, callback: F) -> Self
    where
        F: Fn() + 'static,
    {
        self.on_click = Some(Box::new(callback));
        self
    }

    /// Show the toast
    pub fn show(&mut self) {
        self.is_visible.set(true);
    }

    /// Hide the toast
    pub fn hide(&mut self) {
        self.is_visible.set(false);
        if let Some(ref callback) = self.on_close {
            callback();
        }
    }

    /// Check if the toast is visible
    pub fn is_visible(&self) -> bool {
        self.is_visible.get()
    }

    /// Handle click event
    pub fn handle_click(&self) {
        if let Some(ref callback) = self.on_click {
            callback();
        }
    }

    /// Handle close button click
    pub fn handle_close(&mut self) {
        self.hide();
    }

    /// Get the default background color for the toast type
    pub fn get_default_background_color(&self) -> (u8, u8, u8, u8) {
        match self.toast_type {
            ToastType::Info => (59, 130, 246, 255),     // Blue
            ToastType::Success => (34, 197, 94, 255),   // Green
            ToastType::Warning => (251, 146, 60, 255),  // Orange
            ToastType::Error => (239, 68, 68, 255),     // Red
        }
    }

    /// Get the background color (custom or default)
    pub fn get_background_color(&self) -> (u8, u8, u8, u8) {
        self.background_color
            .unwrap_or_else(|| self.get_default_background_color())
    }

    /// Get the text color (custom or default white)
    pub fn get_text_color(&self) -> (u8, u8, u8, u8) {
        self.text_color.unwrap_or((255, 255, 255, 255))
    }

    /// Check if toast should auto-dismiss
    pub fn should_auto_dismiss(&self) -> bool {
        self.duration > 0
    }

    /// Get position alignment
    pub fn get_alignment(&self) -> (bool, bool, bool, bool) {
        // (is_top, is_bottom, is_left, is_right)
        match self.position {
            ToastPosition::TopLeft => (true, false, true, false),
            ToastPosition::TopCenter => (true, false, false, false),
            ToastPosition::TopRight => (true, false, false, true),
            ToastPosition::BottomLeft => (false, true, true, false),
            ToastPosition::BottomCenter => (false, true, false, false),
            ToastPosition::BottomRight => (false, true, false, true),
        }
    }

    /// Build the toast layout
    pub fn build(&mut self, engine: &mut LayoutEngine) -> Result<NodeId, String> {
        if !self.is_visible() {
            let style = taffy::style::Style {
                display: taffy::style::Display::None,
                ..Default::default()
            };
            let node = engine
                .new_leaf(style)
                .map_err(|e| format!("Failed to create hidden toast node: {:?}", e))?;
            self.node_id = Some(node);
            return Ok(node);
        }

        let style = taffy::style::Style {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Length(self.width),
                height: taffy::style::Dimension::Auto,
            },
            padding: taffy::geometry::Rect {
                left: taffy::style::LengthPercentage::Length(self.padding),
                right: taffy::style::LengthPercentage::Length(self.padding),
                top: taffy::style::LengthPercentage::Length(self.padding),
                bottom: taffy::style::LengthPercentage::Length(self.padding),
            },
            position: taffy::style::Position::Absolute,
            ..Default::default()
        };

        let node = engine
            .new_leaf(style)
            .map_err(|e| format!("Failed to create toast node: {:?}", e))?;
        self.node_id = Some(node);

        Ok(node)
    }
}

impl Default for Toast {
    fn default() -> Self {
        Self::new("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn toast_starts_hidden() {
        let toast = Toast::new("Test");
        assert!(!toast.is_visible());
    }

    #[test]
    fn toast_can_be_shown() {
        let mut toast = Toast::new("Test");
        toast.show();
        assert!(toast.is_visible());
    }

    #[test]
    fn toast_can_be_hidden() {
        let mut toast = Toast::new("Test");
        toast.show();
        toast.hide();
        assert!(!toast.is_visible());
    }

    #[test]
    fn toast_builder_pattern() {
        let toast = Toast::new("Test message")
            .toast_type(ToastType::Success)
            .position(ToastPosition::BottomLeft)
            .duration(5000)
            .width(400.0)
            .padding(20.0)
            .margin(24.0)
            .border_radius(12.0)
            .closable(false)
            .show_icon(false);

        assert_eq!(toast.message, "Test message");
        assert_eq!(toast.toast_type, ToastType::Success);
        assert_eq!(toast.position, ToastPosition::BottomLeft);
        assert_eq!(toast.duration, 5000);
        assert_eq!(toast.width, 400.0);
        assert_eq!(toast.padding, 20.0);
        assert_eq!(toast.margin, 24.0);
        assert_eq!(toast.border_radius, 12.0);
        assert!(!toast.closable);
        assert!(!toast.show_icon);
    }

    #[test]
    fn toast_type_colors() {
        let info = Toast::new("Info").toast_type(ToastType::Info);
        assert_eq!(info.get_default_background_color(), (59, 130, 246, 255));

        let success = Toast::new("Success").toast_type(ToastType::Success);
        assert_eq!(success.get_default_background_color(), (34, 197, 94, 255));

        let warning = Toast::new("Warning").toast_type(ToastType::Warning);
        assert_eq!(warning.get_default_background_color(), (251, 146, 60, 255));

        let error = Toast::new("Error").toast_type(ToastType::Error);
        assert_eq!(error.get_default_background_color(), (239, 68, 68, 255));
    }

    #[test]
    fn toast_custom_colors() {
        let toast = Toast::new("Test")
            .background_color(255, 0, 0, 200)
            .text_color(0, 255, 0, 255);

        assert_eq!(toast.get_background_color(), (255, 0, 0, 200));
        assert_eq!(toast.get_text_color(), (0, 255, 0, 255));
    }

    #[test]
    fn toast_auto_dismiss() {
        let auto = Toast::new("Auto").duration(3000);
        assert!(auto.should_auto_dismiss());

        let manual = Toast::new("Manual").duration(0);
        assert!(!manual.should_auto_dismiss());
    }

    #[test]
    fn toast_position_alignment() {
        let top_left = Toast::new("Test").position(ToastPosition::TopLeft);
        assert_eq!(top_left.get_alignment(), (true, false, true, false));

        let bottom_right = Toast::new("Test").position(ToastPosition::BottomRight);
        assert_eq!(bottom_right.get_alignment(), (false, true, false, true));

        let top_center = Toast::new("Test").position(ToastPosition::TopCenter);
        assert_eq!(top_center.get_alignment(), (true, false, false, false));
    }

    #[test]
    fn toast_callbacks() {
        use std::sync::{Arc, Mutex};

        let closed = Arc::new(Mutex::new(false));
        let closed_clone = closed.clone();

        let clicked = Arc::new(Mutex::new(false));
        let clicked_clone = clicked.clone();

        let mut toast = Toast::new("Test")
            .on_close(move || {
                *closed_clone.lock().unwrap() = true;
            })
            .on_click(move || {
                *clicked_clone.lock().unwrap() = true;
            });

        toast.show();
        toast.handle_click();
        assert!(*clicked.lock().unwrap());

        toast.hide();
        assert!(*closed.lock().unwrap());
    }

    #[test]
    fn toast_handle_close() {
        let mut toast = Toast::new("Test");
        toast.show();
        assert!(toast.is_visible());

        toast.handle_close();
        assert!(!toast.is_visible());
    }

    #[test]
    fn toast_build_creates_node() {
        let mut engine = LayoutEngine::new();
        let mut toast = Toast::new("Test");

        toast.show();
        let result = toast.build(&mut engine);
        assert!(result.is_ok());
        assert!(toast.node_id.is_some());
    }

    #[test]
    fn toast_hidden_creates_hidden_node() {
        let mut engine = LayoutEngine::new();
        let mut toast = Toast::new("Test");

        let result = toast.build(&mut engine);
        assert!(result.is_ok());
        assert!(toast.node_id.is_some());
    }
}
