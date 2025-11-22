// Badge Component - Small badge indicator for notifications and counts
// Essential for showing counts, status, and notifications

use nebula_core::layout::{LayoutEngine, NodeId};
use nebula_core::signal::Signal;

/// Badge variant
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BadgeVariant {
    Default,
    Primary,
    Success,
    Warning,
    Error,
    Info,
}

/// Badge component - small badge indicator
/// 
/// # Example
/// ```
/// let badge = Badge::new("5")
///     .variant(BadgeVariant::Error)
///     .position(BadgePosition::TopRight);
/// ```
pub struct Badge {
    pub node_id: Option<NodeId>,
    pub content: Signal<String>,
    pub variant: BadgeVariant,
    pub visible: Signal<bool>,
    pub size: f32,
    pub padding: f32,
    pub background_color: (u8, u8, u8, u8),
    pub text_color: (u8, u8, u8, u8),
    pub border_radius: f32,
    pub show_dot: bool,
}

impl Badge {
    /// Create a new Badge component
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            node_id: None,
            content: Signal::new(content.into()),
            variant: BadgeVariant::Default,
            visible: Signal::new(true),
            size: 20.0,
            padding: 4.0,
            background_color: (220, 38, 38, 255), // Red
            text_color: (255, 255, 255, 255),
            border_radius: 10.0,
            show_dot: false,
        }
    }

    /// Create a dot badge (no content)
    pub fn dot() -> Self {
        let mut badge = Self::new("");
        badge.show_dot = true;
        badge.size = 8.0;
        badge
    }

    /// Set the content
    pub fn content(self, content: impl Into<String>) -> Self {
        self.content.set(content.into());
        self
    }

    /// Set the variant
    pub fn variant(mut self, variant: BadgeVariant) -> Self {
        self.variant = variant;
        self.background_color = Self::variant_color(variant);
        self
    }

    /// Set the size
    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    /// Set the padding
    pub fn padding(mut self, padding: f32) -> Self {
        self.padding = padding;
        self
    }

    /// Set the background color
    pub fn background_color(mut self, r: u8, g: u8, b: u8, a: u8) -> Self {
        self.background_color = (r, g, b, a);
        self
    }

    /// Set the text color
    pub fn text_color(mut self, r: u8, g: u8, b: u8, a: u8) -> Self {
        self.text_color = (r, g, b, a);
        self
    }

    /// Set the border radius
    pub fn border_radius(mut self, radius: f32) -> Self {
        self.border_radius = radius;
        self
    }

    /// Show or hide the badge
    pub fn visible(self, visible: bool) -> Self {
        self.visible.set(visible);
        self
    }

    /// Get the content
    pub fn get_content(&self) -> String {
        self.content.get()
    }

    /// Set the content
    pub fn set_content(&mut self, content: impl Into<String>) {
        self.content.set(content.into());
    }

    /// Show the badge
    pub fn show(&mut self) {
        self.visible.set(true);
    }

    /// Hide the badge
    pub fn hide(&mut self) {
        self.visible.set(false);
    }

    /// Check if visible
    pub fn is_visible(&self) -> bool {
        self.visible.get()
    }

    /// Check if is dot badge
    pub fn is_dot(&self) -> bool {
        self.show_dot
    }

    /// Get variant color
    fn variant_color(variant: BadgeVariant) -> (u8, u8, u8, u8) {
        match variant {
            BadgeVariant::Default => (107, 114, 128, 255),  // Gray
            BadgeVariant::Primary => (59, 130, 246, 255),   // Blue
            BadgeVariant::Success => (34, 197, 94, 255),    // Green
            BadgeVariant::Warning => (251, 191, 36, 255),   // Yellow
            BadgeVariant::Error => (220, 38, 38, 255),      // Red
            BadgeVariant::Info => (14, 165, 233, 255),      // Cyan
        }
    }

    /// Build the badge layout
    pub fn build(&mut self, engine: &mut LayoutEngine) -> Result<NodeId, String> {
        let size = if self.show_dot {
            self.size
        } else {
            self.size + self.padding * 2.0
        };

        let style = taffy::style::Style {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Length(size),
                height: taffy::style::Dimension::Length(size),
            },
            display: if self.is_visible() {
                taffy::style::Display::Flex
            } else {
                taffy::style::Display::None
            },
            justify_content: Some(taffy::style::JustifyContent::Center),
            align_items: Some(taffy::style::AlignItems::Center),
            ..Default::default()
        };

        let node = engine
            .new_leaf(style)
            .map_err(|e| format!("Failed to create badge node: {:?}", e))?;
        self.node_id = Some(node);

        Ok(node)
    }
}

impl Default for Badge {
    fn default() -> Self {
        Self::new("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn badge_creation() {
        let badge = Badge::new("5");
        assert_eq!(badge.get_content(), "5");
        assert!(badge.is_visible());
    }

    #[test]
    fn badge_dot() {
        let badge = Badge::dot();
        assert!(badge.is_dot());
        assert_eq!(badge.size, 8.0);
    }

    #[test]
    fn badge_set_content() {
        let mut badge = Badge::new("5");
        badge.set_content("10");
        assert_eq!(badge.get_content(), "10");
    }

    #[test]
    fn badge_show_hide() {
        let mut badge = Badge::new("5");
        badge.hide();
        assert!(!badge.is_visible());
        badge.show();
        assert!(badge.is_visible());
    }

    #[test]
    fn badge_variants() {
        let badge = Badge::new("5").variant(BadgeVariant::Success);
        assert_eq!(badge.variant, BadgeVariant::Success);
        assert_eq!(badge.background_color, (34, 197, 94, 255));
    }

    #[test]
    fn badge_builder_pattern() {
        let badge = Badge::new("99+")
            .variant(BadgeVariant::Error)
            .size(24.0)
            .padding(6.0)
            .background_color(255, 0, 0, 255)
            .text_color(255, 255, 255, 255)
            .border_radius(12.0)
            .visible(false);

        assert_eq!(badge.get_content(), "99+");
        assert_eq!(badge.variant, BadgeVariant::Error);
        assert_eq!(badge.size, 24.0);
        assert_eq!(badge.padding, 6.0);
        assert_eq!(badge.background_color, (255, 0, 0, 255));
        assert_eq!(badge.text_color, (255, 255, 255, 255));
        assert_eq!(badge.border_radius, 12.0);
        assert!(!badge.is_visible());
    }

    #[test]
    fn badge_build_creates_node() {
        let mut engine = LayoutEngine::new();
        let mut badge = Badge::new("5");

        let result = badge.build(&mut engine);
        assert!(result.is_ok());
        assert!(badge.node_id.is_some());
    }

    #[test]
    fn badge_variant_colors() {
        assert_eq!(Badge::variant_color(BadgeVariant::Default), (107, 114, 128, 255));
        assert_eq!(Badge::variant_color(BadgeVariant::Primary), (59, 130, 246, 255));
        assert_eq!(Badge::variant_color(BadgeVariant::Success), (34, 197, 94, 255));
        assert_eq!(Badge::variant_color(BadgeVariant::Warning), (251, 191, 36, 255));
        assert_eq!(Badge::variant_color(BadgeVariant::Error), (220, 38, 38, 255));
        assert_eq!(Badge::variant_color(BadgeVariant::Info), (14, 165, 233, 255));
    }
}
