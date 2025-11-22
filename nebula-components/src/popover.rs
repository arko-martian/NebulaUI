// Popover Component - Floating content container
// Shows rich content in a positioned overlay

use nebula_core::layout::{LayoutEngine, NodeId};
use nebula_core::signal::Signal;

/// Popover position relative to trigger
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PopoverPosition {
    Top,
    Bottom,
    Left,
    Right,
    TopStart,
    TopEnd,
    BottomStart,
    BottomEnd,
    LeftStart,
    LeftEnd,
    RightStart,
    RightEnd,
}

/// Popover trigger type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PopoverTrigger {
    Click,
    Hover,
    Focus,
    Manual,
}

/// Popover component - displays rich content in a floating overlay
/// 
/// # Example
/// ```
/// let mut popover = Popover::new()
///     .title("More Information")
///     .content("This is detailed content...")
///     .position(PopoverPosition::Bottom)
///     .trigger(PopoverTrigger::Click)
///     .show_arrow(true);
/// ```
pub struct Popover {
    pub node_id: Option<NodeId>,
    pub title: Option<String>,
    pub content: String,
    pub is_visible: Signal<bool>,
    pub position: PopoverPosition,
    pub trigger: PopoverTrigger,
    pub offset: f32,
    pub width: f32,
    pub max_width: f32,
    pub max_height: f32,
    pub padding: f32,
    pub background_color: (u8, u8, u8, u8),
    pub text_color: (u8, u8, u8, u8),
    pub border_color: (u8, u8, u8, u8),
    pub border_width: f32,
    pub border_radius: f32,
    pub show_arrow: bool,
    pub arrow_size: f32,
    pub closable: bool,
    pub close_on_outside_click: bool,
    pub target_node: Option<NodeId>,
    pub on_show: Option<Box<dyn Fn()>>,
    pub on_hide: Option<Box<dyn Fn()>>,
}

impl Popover {
    /// Create a new Popover component
    pub fn new() -> Self {
        Self {
            node_id: None,
            title: None,
            content: String::new(),
            is_visible: Signal::new(false),
            position: PopoverPosition::Bottom,
            trigger: PopoverTrigger::Click,
            offset: 12.0,
            width: 300.0,
            max_width: 400.0,
            max_height: 600.0,
            padding: 16.0,
            background_color: (255, 255, 255, 255),
            text_color: (0, 0, 0, 255),
            border_color: (200, 200, 200, 255),
            border_width: 1.0,
            border_radius: 8.0,
            show_arrow: true,
            arrow_size: 8.0,
            closable: true,
            close_on_outside_click: true,
            target_node: None,
            on_show: None,
            on_hide: None,
        }
    }

    /// Set the title
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set the content
    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.content = content.into();
        self
    }

    /// Set the position
    pub fn position(mut self, position: PopoverPosition) -> Self {
        self.position = position;
        self
    }

    /// Set the trigger type
    pub fn trigger(mut self, trigger: PopoverTrigger) -> Self {
        self.trigger = trigger;
        self
    }

    /// Set the offset from target
    pub fn offset(mut self, offset: f32) -> Self {
        self.offset = offset;
        self
    }

    /// Set the width
    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    /// Set the max width
    pub fn max_width(mut self, width: f32) -> Self {
        self.max_width = width;
        self
    }

    /// Set the max height
    pub fn max_height(mut self, height: f32) -> Self {
        self.max_height = height;
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

    /// Set the border color
    pub fn border_color(mut self, r: u8, g: u8, b: u8, a: u8) -> Self {
        self.border_color = (r, g, b, a);
        self
    }

    /// Set the border width
    pub fn border_width(mut self, width: f32) -> Self {
        self.border_width = width;
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

    /// Set whether the popover is closable
    pub fn closable(mut self, closable: bool) -> Self {
        self.closable = closable;
        self
    }

    /// Set whether to close on outside click
    pub fn close_on_outside_click(mut self, close: bool) -> Self {
        self.close_on_outside_click = close;
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

    /// Show the popover
    pub fn show(&mut self) {
        self.is_visible.set(true);
        if let Some(ref callback) = self.on_show {
            callback();
        }
    }

    /// Hide the popover
    pub fn hide(&mut self) {
        self.is_visible.set(false);
        if let Some(ref callback) = self.on_hide {
            callback();
        }
    }

    /// Toggle the popover
    pub fn toggle(&mut self) {
        if self.is_visible.get() {
            self.hide();
        } else {
            self.show();
        }
    }

    /// Check if the popover is visible
    pub fn is_visible(&self) -> bool {
        self.is_visible.get()
    }

    /// Check if has title
    pub fn has_title(&self) -> bool {
        self.title.is_some()
    }

    /// Get the position offset based on position type
    pub fn get_position_offset(&self) -> (f32, f32) {
        let offset = self.offset + if self.show_arrow { self.arrow_size } else { 0.0 };

        match self.position {
            PopoverPosition::Top | PopoverPosition::TopStart | PopoverPosition::TopEnd => {
                (0.0, -offset)
            }
            PopoverPosition::Bottom | PopoverPosition::BottomStart | PopoverPosition::BottomEnd => {
                (0.0, offset)
            }
            PopoverPosition::Left | PopoverPosition::LeftStart | PopoverPosition::LeftEnd => {
                (-offset, 0.0)
            }
            PopoverPosition::Right | PopoverPosition::RightStart | PopoverPosition::RightEnd => {
                (offset, 0.0)
            }
        }
    }

    /// Check if position is on top
    pub fn is_top_position(&self) -> bool {
        matches!(
            self.position,
            PopoverPosition::Top | PopoverPosition::TopStart | PopoverPosition::TopEnd
        )
    }

    /// Check if position is on bottom
    pub fn is_bottom_position(&self) -> bool {
        matches!(
            self.position,
            PopoverPosition::Bottom | PopoverPosition::BottomStart | PopoverPosition::BottomEnd
        )
    }

    /// Check if position is on left
    pub fn is_left_position(&self) -> bool {
        matches!(
            self.position,
            PopoverPosition::Left | PopoverPosition::LeftStart | PopoverPosition::LeftEnd
        )
    }

    /// Check if position is on right
    pub fn is_right_position(&self) -> bool {
        matches!(
            self.position,
            PopoverPosition::Right | PopoverPosition::RightStart | PopoverPosition::RightEnd
        )
    }

    /// Build the popover layout
    pub fn build(&mut self, engine: &mut LayoutEngine) -> Result<NodeId, String> {
        if !self.is_visible() {
            let style = taffy::style::Style {
                display: taffy::style::Display::None,
                ..Default::default()
            };
            let node = engine
                .new_leaf(style)
                .map_err(|e| format!("Failed to create hidden popover node: {:?}", e))?;
            self.node_id = Some(node);
            return Ok(node);
        }

        let style = taffy::style::Style {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Length(self.width),
                height: taffy::style::Dimension::Auto,
            },
            max_size: taffy::geometry::Size {
                width: taffy::style::Dimension::Length(self.max_width),
                height: taffy::style::Dimension::Length(self.max_height),
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
            .map_err(|e| format!("Failed to create popover node: {:?}", e))?;
        self.node_id = Some(node);

        Ok(node)
    }
}

impl Default for Popover {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn popover_starts_hidden() {
        let popover = Popover::new();
        assert!(!popover.is_visible());
    }

    #[test]
    fn popover_can_be_shown() {
        let mut popover = Popover::new();
        popover.show();
        assert!(popover.is_visible());
    }

    #[test]
    fn popover_can_be_hidden() {
        let mut popover = Popover::new();
        popover.show();
        popover.hide();
        assert!(!popover.is_visible());
    }

    #[test]
    fn popover_can_be_toggled() {
        let mut popover = Popover::new();
        assert!(!popover.is_visible());

        popover.toggle();
        assert!(popover.is_visible());

        popover.toggle();
        assert!(!popover.is_visible());
    }

    #[test]
    fn popover_builder_pattern() {
        let popover = Popover::new()
            .title("Test Title")
            .content("Test content")
            .position(PopoverPosition::Right)
            .trigger(PopoverTrigger::Hover)
            .offset(16.0)
            .width(350.0)
            .max_width(500.0)
            .max_height(700.0)
            .padding(20.0)
            .border_radius(12.0)
            .show_arrow(false)
            .closable(false)
            .close_on_outside_click(false);

        assert_eq!(popover.title, Some("Test Title".to_string()));
        assert_eq!(popover.content, "Test content");
        assert_eq!(popover.position, PopoverPosition::Right);
        assert_eq!(popover.trigger, PopoverTrigger::Hover);
        assert_eq!(popover.offset, 16.0);
        assert_eq!(popover.width, 350.0);
        assert_eq!(popover.max_width, 500.0);
        assert_eq!(popover.max_height, 700.0);
        assert_eq!(popover.padding, 20.0);
        assert_eq!(popover.border_radius, 12.0);
        assert!(!popover.show_arrow);
        assert!(!popover.closable);
        assert!(!popover.close_on_outside_click);
    }

    #[test]
    fn popover_has_title() {
        let without_title = Popover::new();
        assert!(!without_title.has_title());

        let with_title = Popover::new().title("Title");
        assert!(with_title.has_title());
    }

    #[test]
    fn popover_position_offsets() {
        let popover = Popover::new().offset(10.0).show_arrow(false);

        let mut top = popover.clone();
        top.position = PopoverPosition::Top;
        assert_eq!(top.get_position_offset(), (0.0, -10.0));

        let mut bottom = popover.clone();
        bottom.position = PopoverPosition::Bottom;
        assert_eq!(bottom.get_position_offset(), (0.0, 10.0));

        let mut left = popover.clone();
        left.position = PopoverPosition::Left;
        assert_eq!(left.get_position_offset(), (-10.0, 0.0));

        let mut right = popover.clone();
        right.position = PopoverPosition::Right;
        assert_eq!(right.get_position_offset(), (10.0, 0.0));
    }

    #[test]
    fn popover_position_checks() {
        let mut popover = Popover::new();

        popover.position = PopoverPosition::Top;
        assert!(popover.is_top_position());

        popover.position = PopoverPosition::Bottom;
        assert!(popover.is_bottom_position());

        popover.position = PopoverPosition::Left;
        assert!(popover.is_left_position());

        popover.position = PopoverPosition::Right;
        assert!(popover.is_right_position());
    }

    #[test]
    fn popover_arrow_affects_offset() {
        let popover = Popover::new()
            .offset(10.0)
            .show_arrow(true)
            .arrow_size(8.0);

        let (_, y_offset) = popover.get_position_offset();
        assert_eq!(y_offset, 18.0); // 10 + 8
    }

    #[test]
    fn popover_colors() {
        let popover = Popover::new()
            .background_color(255, 0, 0, 255)
            .text_color(0, 255, 0, 255)
            .border_color(0, 0, 255, 255);

        assert_eq!(popover.background_color, (255, 0, 0, 255));
        assert_eq!(popover.text_color, (0, 255, 0, 255));
        assert_eq!(popover.border_color, (0, 0, 255, 255));
    }

    #[test]
    fn popover_callbacks() {
        use std::sync::{Arc, Mutex};

        let shown = Arc::new(Mutex::new(false));
        let shown_clone = shown.clone();

        let hidden = Arc::new(Mutex::new(false));
        let hidden_clone = hidden.clone();

        let mut popover = Popover::new()
            .on_show(move || {
                *shown_clone.lock().unwrap() = true;
            })
            .on_hide(move || {
                *hidden_clone.lock().unwrap() = true;
            });

        popover.show();
        assert!(*shown.lock().unwrap());

        popover.hide();
        assert!(*hidden.lock().unwrap());
    }

    #[test]
    fn popover_build_creates_node() {
        let mut engine = LayoutEngine::new();
        let mut popover = Popover::new().content("Test");

        popover.show();
        let result = popover.build(&mut engine);
        assert!(result.is_ok());
        assert!(popover.node_id.is_some());
    }

    #[test]
    fn popover_hidden_creates_hidden_node() {
        let mut engine = LayoutEngine::new();
        let mut popover = Popover::new();

        let result = popover.build(&mut engine);
        assert!(result.is_ok());
        assert!(popover.node_id.is_some());
    }
}

// Implement Clone for Popover (needed for tests)
impl Clone for Popover {
    fn clone(&self) -> Self {
        Self {
            node_id: self.node_id,
            title: self.title.clone(),
            content: self.content.clone(),
            is_visible: Signal::new(self.is_visible.get()),
            position: self.position,
            trigger: self.trigger,
            offset: self.offset,
            width: self.width,
            max_width: self.max_width,
            max_height: self.max_height,
            padding: self.padding,
            background_color: self.background_color,
            text_color: self.text_color,
            border_color: self.border_color,
            border_width: self.border_width,
            border_radius: self.border_radius,
            show_arrow: self.show_arrow,
            arrow_size: self.arrow_size,
            closable: self.closable,
            close_on_outside_click: self.close_on_outside_click,
            target_node: self.target_node,
            on_show: None, // Can't clone closures
            on_hide: None, // Can't clone closures
        }
    }
}
