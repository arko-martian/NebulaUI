// Alert Component - Alert message box for notifications
// Essential for showing important messages

use nebula_core::layout::{LayoutEngine, NodeId};
use nebula_core::signal::Signal;

/// Alert severity
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlertSeverity {
    Info,
    Success,
    Warning,
    Error,
}

/// Alert component - alert message box
/// 
/// # Example
/// ```
/// let alert = Alert::new("Operation successful!")
///     .severity(AlertSeverity::Success)
///     .title("Success")
///     .closable(true)
///     .on_close(|| println!("Alert closed"));
/// ```
pub struct Alert {
    pub node_id: Option<NodeId>,
    pub message: Signal<String>,
    pub title: Signal<Option<String>>,
    pub severity: AlertSeverity,
    pub visible: Signal<bool>,
    pub closable: bool,
    pub icon: Option<String>,
    pub width: f32,
    pub padding: f32,
    pub background_color: (u8, u8, u8, u8),
    pub text_color: (u8, u8, u8, u8),
    pub border_color: (u8, u8, u8, u8),
    pub border_width: f32,
    pub border_radius: f32,
    pub on_close: Option<Box<dyn Fn()>>,
}

impl Alert {
    /// Create a new Alert component
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            node_id: None,
            message: Signal::new(message.into()),
            title: Signal::new(None),
            severity: AlertSeverity::Info,
            visible: Signal::new(true),
            closable: false,
            icon: None,
            width: 400.0,
            padding: 16.0,
            background_color: (239, 246, 255, 255), // Light blue
            text_color: (30, 64, 175, 255),
            border_color: (191, 219, 254, 255),
            border_width: 1.0,
            border_radius: 8.0,
            on_close: None,
        }
    }

    /// Set the message
    pub fn message(self, message: impl Into<String>) -> Self {
        self.message.set(message.into());
        self
    }

    /// Set the title
    pub fn title(self, title: impl Into<String>) -> Self {
        self.title.set(Some(title.into()));
        self
    }

    /// Set the severity
    pub fn severity(mut self, severity: AlertSeverity) -> Self {
        self.severity = severity;
        let (bg, text, border) = Self::severity_colors(severity);
        self.background_color = bg;
        self.text_color = text;
        self.border_color = border;
        self
    }

    /// Set closable
    pub fn closable(mut self, closable: bool) -> Self {
        self.closable = closable;
        self
    }

    /// Set icon
    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
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

    /// Set the close callback
    pub fn on_close<F>(mut self, callback: F) -> Self
    where
        F: Fn() + 'static,
    {
        self.on_close = Some(Box::new(callback));
        self
    }

    /// Get the message
    pub fn get_message(&self) -> String {
        self.message.get()
    }

    /// Set the message
    pub fn set_message(&mut self, message: impl Into<String>) {
        self.message.set(message.into());
    }

    /// Get the title
    pub fn get_title(&self) -> Option<String> {
        self.title.get()
    }

    /// Set the title
    pub fn set_title(&mut self, title: Option<String>) {
        self.title.set(title);
    }

    /// Show the alert
    pub fn show(&mut self) {
        self.visible.set(true);
    }

    /// Hide the alert
    pub fn hide(&mut self) {
        self.visible.set(false);
    }

    /// Close the alert
    pub fn close(&mut self) {
        if self.closable {
            self.hide();
            if let Some(ref callback) = self.on_close {
                callback();
            }
        }
    }

    /// Check if visible
    pub fn is_visible(&self) -> bool {
        self.visible.get()
    }

    /// Check if has title
    pub fn has_title(&self) -> bool {
        self.title.get().is_some()
    }

    /// Check if has icon
    pub fn has_icon(&self) -> bool {
        self.icon.is_some()
    }

    /// Get severity colors (background, text, border)
    fn severity_colors(severity: AlertSeverity) -> ((u8, u8, u8, u8), (u8, u8, u8, u8), (u8, u8, u8, u8)) {
        match severity {
            AlertSeverity::Info => (
                (239, 246, 255, 255), // bg
                (30, 64, 175, 255),   // text
                (191, 219, 254, 255), // border
            ),
            AlertSeverity::Success => (
                (240, 253, 244, 255), // bg
                (22, 101, 52, 255),   // text
                (187, 247, 208, 255), // border
            ),
            AlertSeverity::Warning => (
                (254, 252, 232, 255), // bg
                (133, 77, 14, 255),   // text
                (253, 230, 138, 255), // border
            ),
            AlertSeverity::Error => (
                (254, 242, 242, 255), // bg
                (153, 27, 27, 255),   // text
                (254, 202, 202, 255), // border
            ),
        }
    }

    /// Build the alert layout
    pub fn build(&mut self, engine: &mut LayoutEngine) -> Result<NodeId, String> {
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
            display: if self.is_visible() {
                taffy::style::Display::Flex
            } else {
                taffy::style::Display::None
            },
            flex_direction: taffy::style::FlexDirection::Row,
            align_items: Some(taffy::style::AlignItems::Start),
            gap: taffy::geometry::Size {
                width: taffy::style::LengthPercentage::Length(12.0),
                height: taffy::style::LengthPercentage::Length(0.0),
            },
            ..Default::default()
        };

        let node = engine
            .new_leaf(style)
            .map_err(|e| format!("Failed to create alert node: {:?}", e))?;
        self.node_id = Some(node);

        Ok(node)
    }
}

impl Default for Alert {
    fn default() -> Self {
        Self::new("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alert_creation() {
        let alert = Alert::new("Test message");
        assert_eq!(alert.get_message(), "Test message");
        assert!(alert.is_visible());
    }

    #[test]
    fn alert_set_message() {
        let mut alert = Alert::new("Test");
        alert.set_message("New message");
        assert_eq!(alert.get_message(), "New message");
    }

    #[test]
    fn alert_with_title() {
        let alert = Alert::new("Message").title("Title");
        assert!(alert.has_title());
        assert_eq!(alert.get_title(), Some("Title".to_string()));
    }

    #[test]
    fn alert_show_hide() {
        let mut alert = Alert::new("Test");
        alert.hide();
        assert!(!alert.is_visible());
        alert.show();
        assert!(alert.is_visible());
    }

    #[test]
    fn alert_close() {
        use std::sync::{Arc, Mutex};

        let closed = Arc::new(Mutex::new(false));
        let closed_clone = closed.clone();

        let mut alert = Alert::new("Test")
            .closable(true)
            .on_close(move || {
                *closed_clone.lock().unwrap() = true;
            });

        alert.close();
        assert!(!alert.is_visible());
        assert!(*closed.lock().unwrap());
    }

    #[test]
    fn alert_cannot_close_if_not_closable() {
        let mut alert = Alert::new("Test").closable(false);
        alert.close();
        assert!(alert.is_visible()); // Should still be visible
    }

    #[test]
    fn alert_severities() {
        let alert = Alert::new("Test").severity(AlertSeverity::Success);
        assert_eq!(alert.severity, AlertSeverity::Success);
        assert_eq!(alert.background_color, (240, 253, 244, 255));
    }

    #[test]
    fn alert_with_icon() {
        let alert = Alert::new("Test").icon("ℹ️");
        assert!(alert.has_icon());
        assert_eq!(alert.icon, Some("ℹ️".to_string()));
    }

    #[test]
    fn alert_severity_colors() {
        let (bg, text, border) = Alert::severity_colors(AlertSeverity::Info);
        assert_eq!(bg, (239, 246, 255, 255));
        assert_eq!(text, (30, 64, 175, 255));
        assert_eq!(border, (191, 219, 254, 255));
    }

    #[test]
    fn alert_builder_pattern() {
        let alert = Alert::new("Test message")
            .title("Test Title")
            .severity(AlertSeverity::Warning)
            .closable(true)
            .icon("⚠️")
            .width(500.0)
            .padding(20.0);

        assert_eq!(alert.get_message(), "Test message");
        assert!(alert.has_title());
        assert_eq!(alert.severity, AlertSeverity::Warning);
        assert!(alert.closable);
        assert!(alert.has_icon());
        assert_eq!(alert.width, 500.0);
        assert_eq!(alert.padding, 20.0);
    }

    #[test]
    fn alert_build_creates_node() {
        let mut engine = LayoutEngine::new();
        let mut alert = Alert::new("Test");

        let result = alert.build(&mut engine);
        assert!(result.is_ok());
        assert!(alert.node_id.is_some());
    }
}
