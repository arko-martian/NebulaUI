// Skeleton Component - Loading skeleton for content placeholders
// Essential for loading states and perceived performance

use nebula_core::layout::{LayoutEngine, NodeId};
use nebula_core::signal::Signal;

/// Skeleton variant
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SkeletonVariant {
    Text,
    Circular,
    Rectangular,
    Rounded,
}

/// Skeleton component - loading skeleton for placeholders
/// 
/// # Example
/// ```
/// let skeleton = Skeleton::new()
///     .variant(SkeletonVariant::Text)
///     .width(200.0)
///     .height(20.0)
///     .animate(true);
/// ```
pub struct Skeleton {
    pub node_id: Option<NodeId>,
    pub variant: SkeletonVariant,
    pub width: f32,
    pub height: f32,
    pub visible: Signal<bool>,
    pub animate: bool,
    pub animation_duration: f32,
    pub base_color: (u8, u8, u8, u8),
    pub highlight_color: (u8, u8, u8, u8),
    pub border_radius: f32,
}

impl Skeleton {
    /// Create a new Skeleton component
    pub fn new() -> Self {
        Self {
            node_id: None,
            variant: SkeletonVariant::Rectangular,
            width: 100.0,
            height: 100.0,
            visible: Signal::new(true),
            animate: true,
            animation_duration: 1.5,
            base_color: (229, 231, 235, 255),      // Gray-200
            highlight_color: (243, 244, 246, 255), // Gray-100
            border_radius: 4.0,
        }
    }

    /// Create a text skeleton
    pub fn text() -> Self {
        Self::new()
            .variant(SkeletonVariant::Text)
            .width(200.0)
            .height(16.0)
    }

    /// Create a circular skeleton
    pub fn circular(size: f32) -> Self {
        Self::new()
            .variant(SkeletonVariant::Circular)
            .width(size)
            .height(size)
    }

    /// Create a rectangular skeleton
    pub fn rectangular(width: f32, height: f32) -> Self {
        Self::new()
            .variant(SkeletonVariant::Rectangular)
            .width(width)
            .height(height)
    }

    /// Set the variant
    pub fn variant(mut self, variant: SkeletonVariant) -> Self {
        self.variant = variant;
        // Set border radius based on variant
        self.border_radius = match variant {
            SkeletonVariant::Text => 4.0,
            SkeletonVariant::Circular => 9999.0, // Full circle
            SkeletonVariant::Rectangular => 0.0,
            SkeletonVariant::Rounded => 8.0,
        };
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

    /// Set animate
    pub fn animate(mut self, animate: bool) -> Self {
        self.animate = animate;
        self
    }

    /// Set animation duration
    pub fn animation_duration(mut self, duration: f32) -> Self {
        self.animation_duration = duration;
        self
    }

    /// Set the base color
    pub fn base_color(mut self, r: u8, g: u8, b: u8, a: u8) -> Self {
        self.base_color = (r, g, b, a);
        self
    }

    /// Set the highlight color
    pub fn highlight_color(mut self, r: u8, g: u8, b: u8, a: u8) -> Self {
        self.highlight_color = (r, g, b, a);
        self
    }

    /// Set the border radius
    pub fn border_radius(mut self, radius: f32) -> Self {
        self.border_radius = radius;
        self
    }

    /// Show the skeleton
    pub fn show(&mut self) {
        self.visible.set(true);
    }

    /// Hide the skeleton
    pub fn hide(&mut self) {
        self.visible.set(false);
    }

    /// Check if visible
    pub fn is_visible(&self) -> bool {
        self.visible.get()
    }

    /// Check if is circular
    pub fn is_circular(&self) -> bool {
        self.variant == SkeletonVariant::Circular
    }

    /// Check if is text
    pub fn is_text(&self) -> bool {
        self.variant == SkeletonVariant::Text
    }

    /// Build the skeleton layout
    pub fn build(&mut self, engine: &mut LayoutEngine) -> Result<NodeId, String> {
        let style = taffy::style::Style {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Length(self.width),
                height: taffy::style::Dimension::Length(self.height),
            },
            display: if self.is_visible() {
                taffy::style::Display::Flex
            } else {
                taffy::style::Display::None
            },
            ..Default::default()
        };

        let node = engine
            .new_leaf(style)
            .map_err(|e| format!("Failed to create skeleton node: {:?}", e))?;
        self.node_id = Some(node);

        Ok(node)
    }
}

impl Default for Skeleton {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn skeleton_creation() {
        let skeleton = Skeleton::new();
        assert_eq!(skeleton.width, 100.0);
        assert_eq!(skeleton.height, 100.0);
        assert!(skeleton.is_visible());
    }

    #[test]
    fn skeleton_text() {
        let skeleton = Skeleton::text();
        assert_eq!(skeleton.variant, SkeletonVariant::Text);
        assert_eq!(skeleton.width, 200.0);
        assert_eq!(skeleton.height, 16.0);
    }

    #[test]
    fn skeleton_circular() {
        let skeleton = Skeleton::circular(48.0);
        assert_eq!(skeleton.variant, SkeletonVariant::Circular);
        assert_eq!(skeleton.width, 48.0);
        assert_eq!(skeleton.height, 48.0);
        assert!(skeleton.is_circular());
    }

    #[test]
    fn skeleton_rectangular() {
        let skeleton = Skeleton::rectangular(200.0, 100.0);
        assert_eq!(skeleton.variant, SkeletonVariant::Rectangular);
        assert_eq!(skeleton.width, 200.0);
        assert_eq!(skeleton.height, 100.0);
    }

    #[test]
    fn skeleton_show_hide() {
        let mut skeleton = Skeleton::new();
        skeleton.hide();
        assert!(!skeleton.is_visible());
        skeleton.show();
        assert!(skeleton.is_visible());
    }

    #[test]
    fn skeleton_variants() {
        let skeleton = Skeleton::new().variant(SkeletonVariant::Rounded);
        assert_eq!(skeleton.variant, SkeletonVariant::Rounded);
        assert_eq!(skeleton.border_radius, 8.0);
    }

    #[test]
    fn skeleton_circular_border_radius() {
        let skeleton = Skeleton::new().variant(SkeletonVariant::Circular);
        assert_eq!(skeleton.border_radius, 9999.0);
    }

    #[test]
    fn skeleton_animate() {
        let skeleton = Skeleton::new().animate(false);
        assert!(!skeleton.animate);
    }

    #[test]
    fn skeleton_animation_duration() {
        let skeleton = Skeleton::new().animation_duration(2.0);
        assert_eq!(skeleton.animation_duration, 2.0);
    }

    #[test]
    fn skeleton_is_text() {
        let skeleton = Skeleton::text();
        assert!(skeleton.is_text());
    }

    #[test]
    fn skeleton_builder_pattern() {
        let skeleton = Skeleton::new()
            .variant(SkeletonVariant::Rounded)
            .width(300.0)
            .height(200.0)
            .animate(false)
            .animation_duration(2.5)
            .base_color(200, 200, 200, 255)
            .highlight_color(220, 220, 220, 255)
            .border_radius(12.0);

        assert_eq!(skeleton.variant, SkeletonVariant::Rounded);
        assert_eq!(skeleton.width, 300.0);
        assert_eq!(skeleton.height, 200.0);
        assert!(!skeleton.animate);
        assert_eq!(skeleton.animation_duration, 2.5);
        assert_eq!(skeleton.base_color, (200, 200, 200, 255));
        assert_eq!(skeleton.highlight_color, (220, 220, 220, 255));
        assert_eq!(skeleton.border_radius, 12.0);
    }

    #[test]
    fn skeleton_build_creates_node() {
        let mut engine = LayoutEngine::new();
        let mut skeleton = Skeleton::new();

        let result = skeleton.build(&mut engine);
        assert!(result.is_ok());
        assert!(skeleton.node_id.is_some());
    }
}
