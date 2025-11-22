// Tooltip Component - Shows helpful text on hover
// Lightweight overlay that appears near the target element

use nebula_core::layout::{LayoutEngine, NodeId};
use nebula_core::signal::Signal;

/// Tooltip position relative to target
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TooltipPosition {
    Top,
    Bottom,
    Left,
    Right,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

/// Tooltip component - displays helpful text on hover
/// 
/// # Example
/// ```
/// let mut tooltip = Tooltip::new("This is a helpful tip!")
///     .position(TooltipPosition::Top)
///     .delay(500)
///     .show_arrow(true);
/// ```
pub struct Tooltip {
    pub node_id: Option<NodeId>,
    pub content: String,
    pub is_visible: Signal<bool>,
    pub position: TooltipPosition,
    pub offset: f32,
    pub delay: u32, // milliseconds before showing
    pub max_width: f32,
    pub padding: f32,
    pub background_color: (u8, u8, u8, u8), // RGBA
    pub text_color: (u8, u8, u8, u8),
    pub border_radius: f32,
    pub show_arrow: bool,
    pub arrow_size: f32,
    pub target_node: Option<NodeId>,
    pub on_show: Option<Box<dyn Fn()>>,
    pub on_hide: Option<Box<dyn Fn()>>,
}

impl Tooltip {
    /// Create a new Tooltip component
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            node_id: None,
            content: content.into(),
            is_visible: Signal::new(false),
            position: TooltipPosition::Top,
            offset: 8.0,
            delay: 500,
            max_width: 200.0,
            padding: 8.0,
            background_color: (45, 45, 45, 255), // Dark gray
            text_color: (255, 255, 255, 255),    // White
            border_radius: 4.0,
            show_arrow: true,
            arrow_size: 6.0,
            target_node: None,
            on_show: None,
            on_hide: None,
        }
    }

    /// Set the tooltip content
    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.content = content.into();
        self
    }

    /// Set the tooltip position
    pub fn position(mut self, position: TooltipPosition) -> Self {
        self.position = position;
        self
    }

    /// Set the offset from target
    pub fn offset(mut self, offset: f32) -> Self {
        self.offset = offset;
        self
    }

    /// Set the delay before showing (milliseconds)
    pub fn delay(mut self, delay: u32) -> Self {
        self.delay = delay;
        self
    }

    /// Set the max width
    pub fn max_width(mut self, width: f32) -> Self {
        self.max_width = width;
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

    /// Set whether to show arrow
    pub fn show_arrow(mut self, show: bool) -> Self {
        self.show_arrow = show;
        self
    }

    /// Set the arrow size
    pub fn arrow_size(mut self, size: f32) -> Self {
        self.arrow_size = size;
        self
    }

    /// Set the target node
    pub fn target(mut self, node: NodeId) -> Self {
        self.target_node = Some(node);
        self
    }

    /// Set the show callback
    pub fn on_show<F>(mut self, callback: F) -> Self
    where
        F: Fn() + 'static,
    {
        self.on_show = Some(Box::new(callback));
        self
    }

    /// Set the hide callback
    pub fn on_hide<F>(mut self, callback: F) -> Self
    where
        F: Fn() + 'static,
    {
        self.on_hide = Some(Box::new(callback));
        self
    }

    /// Show the tooltip
    pub fn show(&mut self) {
        self.is_visible.set(true);
        if let Some(ref callback) = self.on_show {
            callback();
        }
    }

    /// Hide the tooltip
    pub fn hide(&mut self) {
        self.is_visible.set(false);
        if let Some(ref callback) = self.on_hide {
            callback();
        }
    }

    /// Toggle the tooltip
    pub fn toggle(&mut self) {
        if self.is_visible.get() {
            self.hide();
        } else {
            self.show();
        }
    }

    /// Check if the tooltip is visible
    pub fn is_visible(&self) -> bool {
        self.is_visible.get()
    }

    /// Get the position offset based on position type
    pub fn get_position_offset(&self) -> (f32, f32) {
        let offset = self.offset + if self.show_arrow { self.arrow_size } else { 0.0 };
        
        match self.position {
            TooltipPosition::Top => (0.0, -offset),
            TooltipPosition::Bottom => (0.0, offset),
            TooltipPosition::Left => (-offset, 0.0),
            TooltipPosition::Right => (offset, 0.0),
            TooltipPosition::TopLeft => (-offset, -offset),
            TooltipPosition::TopRight => (offset, -offset),
            TooltipPosition::BottomLeft => (-offset, offset),
            TooltipPosition::BottomRight => (offset, offset),
        }
    }

    /// Check if position is on top
    pub fn is_top_position(&self) -> bool {
        matches!(
            self.position,
            TooltipPosition::Top | TooltipPosition::TopLeft | TooltipPosition::TopRight
        )
    }

    /// Check if position is on bottom
    pub fn is_bottom_position(&self) -> bool {
        matches!(
            self.position,
            TooltipPosition::Bottom | TooltipPosition::BottomLeft | TooltipPosition::BottomRight
        )
    }

    /// Check if position is on left
    pub fn is_left_position(&self) -> bool {
        matches!(
            self.position,
            TooltipPosition::Left | TooltipPosition::TopLeft | TooltipPosition::BottomLeft
        )
    }

    /// Check if position is on right
    pub fn is_right_position(&self) -> bool {
        matches!(
            self.position,
            TooltipPosition::Right | TooltipPosition::TopRight | TooltipPosition::BottomRight
        )
    }

    /// Build the tooltip layout
    pub fn build(&mut self, engine: &mut LayoutEngine) -> Result<NodeId, String> {
        if !self.is_visible() {
            // If not visible, return a zero-sized node
            let style = taffy::style::Style {
                display: taffy::style::Display::None,
                ..Default::default()
            };
            let node = engine
                .new_leaf(style)
                .map_err(|e| format!("Failed to create hidden tooltip node: {:?}", e))?;
            self.node_id = Some(node);
            return Ok(node);
        }

        // Create tooltip node
        let style = taffy::style::Style {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Auto,
                height: taffy::style::Dimension::Auto,
            },
            max_size: taffy::geometry::Size {
                width: taffy::style::Dimension::Length(self.max_width),
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
            .map_err(|e| format!("Failed to create tooltip node: {:?}", e))?;
        self.node_id = Some(node);

        Ok(node)
    }
}

impl Default for Tooltip {
    fn default() -> Self {
        Self::new("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tooltip_starts_hidden() {
        let tooltip = Tooltip::new("Test");
        assert!(!tooltip.is_visible());
    }

    #[test]
    fn tooltip_can_be_shown() {
        let mut tooltip = Tooltip::new("Test");
        tooltip.show();
        assert!(tooltip.is_visible());
    }

    #[test]
    fn tooltip_can_be_hidden() {
        let mut tooltip = Tooltip::new("Test");
        tooltip.show();
        tooltip.hide();
        assert!(!tooltip.is_visible());
    }

    #[test]
    fn tooltip_can_be_toggled() {
        let mut tooltip = Tooltip::new("Test");
        assert!(!tooltip.is_visible());

        tooltip.toggle();
        assert!(tooltip.is_visible());

        tooltip.toggle();
        assert!(!tooltip.is_visible());
    }

    #[test]
    fn tooltip_builder_pattern() {
        let tooltip = Tooltip::new("Test content")
            .position(TooltipPosition::Bottom)
            .offset(12.0)
            .delay(1000)
            .max_width(300.0)
            .padding(16.0)
            .border_radius(8.0)
            .show_arrow(false);

        assert_eq!(tooltip.content, "Test content");
        assert_eq!(tooltip.position, TooltipPosition::Bottom);
        assert_eq!(tooltip.offset, 12.0);
        assert_eq!(tooltip.delay, 1000);
        assert_eq!(tooltip.max_width, 300.0);
        assert_eq!(tooltip.padding, 16.0);
        assert_eq!(tooltip.border_radius, 8.0);
        assert!(!tooltip.show_arrow);
    }

    #[test]
    fn tooltip_position_offsets() {
        let tooltip = Tooltip::new("Test").offset(10.0).show_arrow(false);

        let mut top = tooltip.clone();
        top.position = TooltipPosition::Top;
        assert_eq!(top.get_position_offset(), (0.0, -10.0));

        let mut bottom = tooltip.clone();
        bottom.position = TooltipPosition::Bottom;
        assert_eq!(bottom.get_position_offset(), (0.0, 10.0));

        let mut left = tooltip.clone();
        left.position = TooltipPosition::Left;
        assert_eq!(left.get_position_offset(), (-10.0, 0.0));

        let mut right = tooltip.clone();
        right.position = TooltipPosition::Right;
        assert_eq!(right.get_position_offset(), (10.0, 0.0));
    }

    #[test]
    fn tooltip_position_checks() {
        let mut tooltip = Tooltip::new("Test");

        tooltip.position = TooltipPosition::Top;
        assert!(tooltip.is_top_position());
        assert!(!tooltip.is_bottom_position());

        tooltip.position = TooltipPosition::Bottom;
        assert!(tooltip.is_bottom_position());
        assert!(!tooltip.is_top_position());

        tooltip.position = TooltipPosition::Left;
        assert!(tooltip.is_left_position());
        assert!(!tooltip.is_right_position());

        tooltip.position = TooltipPosition::Right;
        assert!(tooltip.is_right_position());
        assert!(!tooltip.is_left_position());
    }

    #[test]
    fn tooltip_arrow_affects_offset() {
        let tooltip = Tooltip::new("Test")
            .offset(10.0)
            .show_arrow(true)
            .arrow_size(6.0);

        let (_, y_offset) = tooltip.get_position_offset();
        assert_eq!(y_offset, -16.0); // -(10 + 6)
    }

    #[test]
    fn tooltip_colors() {
        let tooltip = Tooltip::new("Test")
            .background_color(255, 0, 0, 200)
            .text_color(0, 255, 0, 255);

        assert_eq!(tooltip.background_color, (255, 0, 0, 200));
        assert_eq!(tooltip.text_color, (0, 255, 0, 255));
    }

    #[test]
    fn tooltip_callbacks() {
        use std::sync::{Arc, Mutex};

        let shown = Arc::new(Mutex::new(false));
        let shown_clone = shown.clone();

        let hidden = Arc::new(Mutex::new(false));
        let hidden_clone = hidden.clone();

        let mut tooltip = Tooltip::new("Test")
            .on_show(move || {
                *shown_clone.lock().unwrap() = true;
            })
            .on_hide(move || {
                *hidden_clone.lock().unwrap() = true;
            });

        tooltip.show();
        assert!(*shown.lock().unwrap());

        tooltip.hide();
        assert!(*hidden.lock().unwrap());
    }

    #[test]
    fn tooltip_build_creates_node() {
        let mut engine = LayoutEngine::new();
        let mut tooltip = Tooltip::new("Test");

        tooltip.show();
        let result = tooltip.build(&mut engine);
        assert!(result.is_ok());
        assert!(tooltip.node_id.is_some());
    }

    #[test]
    fn tooltip_hidden_creates_hidden_node() {
        let mut engine = LayoutEngine::new();
        let mut tooltip = Tooltip::new("Test");

        let result = tooltip.build(&mut engine);
        assert!(result.is_ok());
        assert!(tooltip.node_id.is_some());
    }

    #[test]
    fn tooltip_content_can_be_updated() {
        let mut tooltip = Tooltip::new("Original");
        assert_eq!(tooltip.content, "Original");

        tooltip = tooltip.content("Updated");
        assert_eq!(tooltip.content, "Updated");
    }
}

// Implement Clone for Tooltip (needed for tests)
impl Clone for Tooltip {
    fn clone(&self) -> Self {
        Self {
            node_id: self.node_id,
            content: self.content.clone(),
            is_visible: Signal::new(self.is_visible.get()),
            position: self.position,
            offset: self.offset,
            delay: self.delay,
            max_width: self.max_width,
            padding: self.padding,
            background_color: self.background_color,
            text_color: self.text_color,
            border_radius: self.border_radius,
            show_arrow: self.show_arrow,
            arrow_size: self.arrow_size,
            target_node: self.target_node,
            on_show: None, // Can't clone closures
            on_hide: None, // Can't clone closures
        }
    }
}
