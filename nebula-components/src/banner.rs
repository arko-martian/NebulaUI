// Banner Component - Banner notification for important announcements
// Essential for site-wide notifications and announcements

use nebula_core::layout::{LayoutEngine, NodeId};
use nebula_core::signal::Signal;

/// Banner position
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BannerPosition {
    Top,
    Bottom,
}

/// Banner variant
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BannerVariant {
    Info,
    Success,
    Warning,
    Error,
}

/// Banner component - banner notification for announcements
/// 
/// # Example
/// ```
/// let banner = Banner::new("New features available!")
///     .variant(BannerVariant::Info)
///     .position(BannerPosition::Top)
///     .closable(true)
///     .action_text("Learn More")
///     .on_action(|| println!("Action clicked"))
///     .on_close(|| println!("Banner closed"));
/// ```
pub struct Banner {
    pub node_id: Option<NodeId>,
    pub message: Signal<String>,
    pub variant: BannerVariant,
    pub position: BannerPosition,
    pub visible: Signal<bool>,
    pub closable: bool,
    pub action_text: Option<String>,
    pub icon: Option<String>,
    pub width: f32,
    pub height: f32,
    pub padding: f32,
    pub background_color: (u8, u8, u8, u8),
    pub text_color: (u8, u8, u8, u8),
    pub on_action: Option<Box<dyn Fn()>>,
    pub on_close: Option<Box<dyn Fn()>>,
}

impl Banner {
    /// Create a new Banner component
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            node_id: None,
            message: Signal::new(message.into()),
            variant: BannerVariant::Info,
            position: BannerPosition::Top,
            visible: Signal::new(true),
            closable: false,
            action_text: None,
            icon: None,
            width: 0.0, // Full width
            height: 48.0,
            padding: 16.0,
            background_color: (59, 130, 246, 255), // Blue
            text_color: (255, 255, 255, 255),
            on_action: None,
            on_close: None,
        }
    }

    /// Set the message
    pub fn message(self, message: impl Into<String>) -> Self {
        self.message.set(message.into());
        self
    }

    /// Set the variant
    pub fn variant(mut self, variant: BannerVariant) -> Self {
        self.variant = variant;
        self.background_color = Self::variant_color(variant);
        self
    }

    /// Set the position
    pub fn position(mut self, position: BannerPosition) -> Self {
        self.position = position;
        self
    }

    /// Set closable
    pub fn closable(mut self, closable: bool) -> Self {
        self.closable = closable;
        self
    }

    /// Set action text
    pub fn action_text(mut self, text: impl Into<String>) -> Self {
        self.action_text = Some(text.into());
        self
    }

    /// Set icon
    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Set the height
    pub fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    /// Set the action callback
    pub fn on_action<F>(mut self, callback: F) -> Self
    where
        F: Fn() + 'static,
    {
        self.on_action = Some(Box::new(callback));
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

    /// Show the banner
    pub fn show(&mut self) {
        self.visible.set(true);
    }

    /// Hide the banner
    pub fn hide(&mut self) {
        self.visible.set(false);
    }

    /// Close the banner
    pub fn close(&mut self) {
        if self.closable {
            self.hide();
            if let Some(ref callback) = self.on_close {
                callback();
            }
        }
    }

    /// Handle action click
    pub fn action(&mut self) {
        if self.has_action() {
            if let Some(ref callback) = self.on_action {
                callback();
            }
        }
    }

    /// Check if visible
    pub fn is_visible(&self) -> bool {
        self.visible.get()
    }

    /// Check if has action
    pub fn has_action(&self) -> bool {
        self.action_text.is_some()
    }

    /// Check if has icon
    pub fn has_icon(&self) -> bool {
        self.icon.is_some()
    }

    /// Get variant color
    fn variant_color(variant: BannerVariant) -> (u8, u8, u8, u8) {
        match variant {
            BannerVariant::Info => (59, 130, 246, 255),    // Blue
            BannerVariant::Success => (34, 197, 94, 255),  // Green
            BannerVariant::Warning => (251, 191, 36, 255), // Yellow
            BannerVariant::Error => (220, 38, 38, 255),    // Red
        }
    }

    /// Build the banner layout
    pub fn build(&mut self, engine: &mut LayoutEngine) -> Result<NodeId, String> {
        let style = taffy::style::Style {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Percent(1.0),
                height: taffy::style::Dimension::Length(self.height),
            },
            padding: taffy::geometry::Rect {
                left: taffy::style::LengthPercentage::Length(self.padding),
                right: taffy::style::LengthPercentage::Length(self.padding),
                top: taffy::style::LengthPercentage::Length(0.0),
                bottom: taffy::style::LengthPercentage::Length(0.0),
            },
            display: if self.is_visible() {
                taffy::style::Display::Flex
            } else {
                taffy::style::Display::None
            },
            flex_direction: taffy::style::FlexDirection::Row,
            justify_content: Some(taffy::style::JustifyContent::Center),
            align_items: Some(taffy::style::AlignItems::Center),
            gap: taffy::geometry::Size {
                width: taffy::style::LengthPercentage::Length(16.0),
                height: taffy::style::LengthPercentage::Length(0.0),
            },
            ..Default::default()
        };

        let node = engine
            .new_leaf(style)
            .map_err(|e| format!("Failed to create banner node: {:?}", e))?;
        self.node_id = Some(node);

        Ok(node)
    }
}

impl Default for Banner {
    fn default() -> Self {
        Self::new("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn banner_creation() {
        let banner = Banner::new("Test message");
        assert_eq!(banner.get_message(), "Test message");
        assert!(banner.is_visible());
    }

    #[test]
    fn banner_set_message() {
        let mut banner = Banner::new("Test");
        banner.set_message("New message");
        assert_eq!(banner.get_message(), "New message");
    }

    #[test]
    fn banner_show_hide() {
        let mut banner = Banner::new("Test");
        banner.hide();
        assert!(!banner.is_visible());
        banner.show();
        assert!(banner.is_visible());
    }

    #[test]
    fn banner_close() {
        use std::sync::{Arc, Mutex};

        let closed = Arc::new(Mutex::new(false));
        let closed_clone = closed.clone();

        let mut banner = Banner::new("Test")
            .closable(true)
            .on_close(move || {
                *closed_clone.lock().unwrap() = true;
            });

        banner.close();
        assert!(!banner.is_visible());
        assert!(*closed.lock().unwrap());
    }

    #[test]
    fn banner_cannot_close_if_not_closable() {
        let mut banner = Banner::new("Test").closable(false);
        banner.close();
        assert!(banner.is_visible());
    }

    #[test]
    fn banner_action() {
        use std::sync::{Arc, Mutex};

        let clicked = Arc::new(Mutex::new(false));
        let clicked_clone = clicked.clone();

        let mut banner = Banner::new("Test")
            .action_text("Click me")
            .on_action(move || {
                *clicked_clone.lock().unwrap() = true;
            });

        banner.action();
        assert!(*clicked.lock().unwrap());
    }

    #[test]
    fn banner_variants() {
        let banner = Banner::new("Test").variant(BannerVariant::Success);
        assert_eq!(banner.variant, BannerVariant::Success);
        assert_eq!(banner.background_color, (34, 197, 94, 255));
    }

    #[test]
    fn banner_positions() {
        let banner = Banner::new("Test").position(BannerPosition::Bottom);
        assert_eq!(banner.position, BannerPosition::Bottom);
    }

    #[test]
    fn banner_with_action() {
        let banner = Banner::new("Test").action_text("Learn More");
        assert!(banner.has_action());
        assert_eq!(banner.action_text, Some("Learn More".to_string()));
    }

    #[test]
    fn banner_with_icon() {
        let banner = Banner::new("Test").icon("ℹ️");
        assert!(banner.has_icon());
        assert_eq!(banner.icon, Some("ℹ️".to_string()));
    }

    #[test]
    fn banner_variant_colors() {
        assert_eq!(Banner::variant_color(BannerVariant::Info), (59, 130, 246, 255));
        assert_eq!(Banner::variant_color(BannerVariant::Success), (34, 197, 94, 255));
        assert_eq!(Banner::variant_color(BannerVariant::Warning), (251, 191, 36, 255));
        assert_eq!(Banner::variant_color(BannerVariant::Error), (220, 38, 38, 255));
    }

    #[test]
    fn banner_builder_pattern() {
        let banner = Banner::new("Test message")
            .variant(BannerVariant::Warning)
            .position(BannerPosition::Bottom)
            .closable(true)
            .action_text("Action")
            .icon("⚠️")
            .height(60.0);

        assert_eq!(banner.get_message(), "Test message");
        assert_eq!(banner.variant, BannerVariant::Warning);
        assert_eq!(banner.position, BannerPosition::Bottom);
        assert!(banner.closable);
        assert!(banner.has_action());
        assert!(banner.has_icon());
        assert_eq!(banner.height, 60.0);
    }

    #[test]
    fn banner_build_creates_node() {
        let mut engine = LayoutEngine::new();
        let mut banner = Banner::new("Test");

        let result = banner.build(&mut engine);
        assert!(result.is_ok());
        assert!(banner.node_id.is_some());
    }
}
