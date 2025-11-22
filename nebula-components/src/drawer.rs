// Drawer Component - Side panel for navigation or content
// Essential for mobile-style navigation and side panels

use nebula_core::layout::{LayoutEngine, NodeId};
use nebula_core::signal::Signal;

/// Drawer position
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DrawerPosition {
    Left,
    Right,
    Top,
    Bottom,
}

/// Drawer variant
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DrawerVariant {
    Temporary,  // Overlays content, closes on backdrop click
    Persistent, // Pushes content, stays open
    Permanent,  // Always visible, cannot be closed
}

/// Drawer component - side panel for navigation or content
/// 
/// # Example
/// ```
/// let mut drawer = Drawer::new()
///     .position(DrawerPosition::Left)
///     .variant(DrawerVariant::Temporary)
///     .width(280.0)
///     .on_open(|| println!("Drawer opened"))
///     .on_close(|| println!("Drawer closed"));
/// ```
pub struct Drawer {
    pub node_id: Option<NodeId>,
    pub is_open: Signal<bool>,
    pub position: DrawerPosition,
    pub variant: DrawerVariant,
    pub width: f32,
    pub height: f32,
    pub backdrop_opacity: f32,
    pub show_backdrop: bool,
    pub close_on_backdrop_click: bool,
    pub close_on_escape: bool,
    pub background_color: (u8, u8, u8, u8),
    pub backdrop_color: (u8, u8, u8, u8),
    pub shadow_color: (u8, u8, u8, u8),
    pub shadow_blur: f32,
    pub animation_duration: f32,
    pub on_open: Option<Box<dyn Fn()>>,
    pub on_close: Option<Box<dyn Fn()>>,
    pub on_backdrop_click: Option<Box<dyn Fn()>>,
}

impl Drawer {
    /// Create a new Drawer component
    pub fn new() -> Self {
        Self {
            node_id: None,
            is_open: Signal::new(false),
            position: DrawerPosition::Left,
            variant: DrawerVariant::Temporary,
            width: 280.0,
            height: 400.0,
            backdrop_opacity: 0.5,
            show_backdrop: true,
            close_on_backdrop_click: true,
            close_on_escape: true,
            background_color: (255, 255, 255, 255),
            backdrop_color: (0, 0, 0, 128),
            shadow_color: (0, 0, 0, 50),
            shadow_blur: 10.0,
            animation_duration: 0.3,
            on_open: None,
            on_close: None,
            on_backdrop_click: None,
        }
    }

    /// Set the position
    pub fn position(mut self, position: DrawerPosition) -> Self {
        self.position = position;
        self
    }

    /// Set the variant
    pub fn variant(mut self, variant: DrawerVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set the width (for left/right drawers)
    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    /// Set the height (for top/bottom drawers)
    pub fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    /// Set backdrop opacity
    pub fn backdrop_opacity(mut self, opacity: f32) -> Self {
        self.backdrop_opacity = opacity.clamp(0.0, 1.0);
        self
    }

    /// Show or hide backdrop
    pub fn show_backdrop(mut self, show: bool) -> Self {
        self.show_backdrop = show;
        self
    }

    /// Close on backdrop click
    pub fn close_on_backdrop_click(mut self, close: bool) -> Self {
        self.close_on_backdrop_click = close;
        self
    }

    /// Close on escape key
    pub fn close_on_escape(mut self, close: bool) -> Self {
        self.close_on_escape = close;
        self
    }

    /// Set background color
    pub fn background_color(mut self, r: u8, g: u8, b: u8, a: u8) -> Self {
        self.background_color = (r, g, b, a);
        self
    }

    /// Set backdrop color
    pub fn backdrop_color(mut self, r: u8, g: u8, b: u8, a: u8) -> Self {
        self.backdrop_color = (r, g, b, a);
        self
    }

    /// Set shadow blur
    pub fn shadow_blur(mut self, blur: f32) -> Self {
        self.shadow_blur = blur;
        self
    }

    /// Set animation duration
    pub fn animation_duration(mut self, duration: f32) -> Self {
        self.animation_duration = duration;
        self
    }

    /// Set the open callback
    pub fn on_open<F>(mut self, callback: F) -> Self
    where
        F: Fn() + 'static,
    {
        self.on_open = Some(Box::new(callback));
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

    /// Set the backdrop click callback
    pub fn on_backdrop_click<F>(mut self, callback: F) -> Self
    where
        F: Fn() + 'static,
    {
        self.on_backdrop_click = Some(Box::new(callback));
        self
    }

    /// Open the drawer
    pub fn open(&mut self) {
        if self.variant == DrawerVariant::Permanent {
            return; // Permanent drawers cannot be opened/closed
        }

        if !self.is_open.get() {
            self.is_open.set(true);
            if let Some(ref callback) = self.on_open {
                callback();
            }
        }
    }

    /// Close the drawer
    pub fn close(&mut self) {
        if self.variant == DrawerVariant::Permanent {
            return; // Permanent drawers cannot be opened/closed
        }

        if self.is_open.get() {
            self.is_open.set(false);
            if let Some(ref callback) = self.on_close {
                callback();
            }
        }
    }

    /// Toggle the drawer
    pub fn toggle(&mut self) {
        if self.is_open.get() {
            self.close();
        } else {
            self.open();
        }
    }

    /// Check if drawer is open
    pub fn is_drawer_open(&self) -> bool {
        self.variant == DrawerVariant::Permanent || self.is_open.get()
    }

    /// Check if drawer is closed
    pub fn is_drawer_closed(&self) -> bool {
        !self.is_drawer_open()
    }

    /// Handle backdrop click
    pub fn handle_backdrop_click(&mut self) {
        if let Some(ref callback) = self.on_backdrop_click {
            callback();
        }

        if self.close_on_backdrop_click && self.variant == DrawerVariant::Temporary {
            self.close();
        }
    }

    /// Handle escape key
    pub fn handle_escape(&mut self) {
        if self.close_on_escape && self.variant == DrawerVariant::Temporary {
            self.close();
        }
    }

    /// Check if drawer is temporary
    pub fn is_temporary(&self) -> bool {
        self.variant == DrawerVariant::Temporary
    }

    /// Check if drawer is persistent
    pub fn is_persistent(&self) -> bool {
        self.variant == DrawerVariant::Persistent
    }

    /// Check if drawer is permanent
    pub fn is_permanent(&self) -> bool {
        self.variant == DrawerVariant::Permanent
    }

    /// Check if drawer is horizontal (left/right)
    pub fn is_horizontal(&self) -> bool {
        matches!(self.position, DrawerPosition::Left | DrawerPosition::Right)
    }

    /// Check if drawer is vertical (top/bottom)
    pub fn is_vertical(&self) -> bool {
        matches!(self.position, DrawerPosition::Top | DrawerPosition::Bottom)
    }

    /// Build the drawer layout
    pub fn build(&mut self, engine: &mut LayoutEngine) -> Result<NodeId, String> {
        let (width, height) = match self.position {
            DrawerPosition::Left | DrawerPosition::Right => {
                (taffy::style::Dimension::Length(self.width), taffy::style::Dimension::Percent(1.0))
            }
            DrawerPosition::Top | DrawerPosition::Bottom => {
                (taffy::style::Dimension::Percent(1.0), taffy::style::Dimension::Length(self.height))
            }
        };

        let style = taffy::style::Style {
            size: taffy::geometry::Size { width, height },
            display: taffy::style::Display::Flex,
            flex_direction: taffy::style::FlexDirection::Column,
            position: taffy::style::Position::Absolute,
            ..Default::default()
        };

        let node = engine
            .new_leaf(style)
            .map_err(|e| format!("Failed to create drawer node: {:?}", e))?;
        self.node_id = Some(node);

        Ok(node)
    }
}

impl Default for Drawer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn drawer_starts_closed() {
        let drawer = Drawer::new();
        assert!(!drawer.is_drawer_open());
        assert!(drawer.is_drawer_closed());
    }

    #[test]
    fn drawer_open() {
        let mut drawer = Drawer::new();
        drawer.open();
        assert!(drawer.is_drawer_open());
        assert!(!drawer.is_drawer_closed());
    }

    #[test]
    fn drawer_close() {
        let mut drawer = Drawer::new();
        drawer.open();
        drawer.close();
        assert!(drawer.is_drawer_closed());
    }

    #[test]
    fn drawer_toggle() {
        let mut drawer = Drawer::new();
        
        drawer.toggle();
        assert!(drawer.is_drawer_open());

        drawer.toggle();
        assert!(drawer.is_drawer_closed());
    }

    #[test]
    fn drawer_permanent_cannot_close() {
        let mut drawer = Drawer::new()
            .variant(DrawerVariant::Permanent);

        drawer.open();
        assert!(drawer.is_drawer_open()); // Permanent is always open

        drawer.close();
        assert!(drawer.is_drawer_open()); // Should still be open
    }

    #[test]
    fn drawer_handle_backdrop_click() {
        let mut drawer = Drawer::new()
            .variant(DrawerVariant::Temporary)
            .close_on_backdrop_click(true);

        drawer.open();
        drawer.handle_backdrop_click();
        assert!(drawer.is_drawer_closed());
    }

    #[test]
    fn drawer_handle_backdrop_click_no_close() {
        let mut drawer = Drawer::new()
            .variant(DrawerVariant::Temporary)
            .close_on_backdrop_click(false);

        drawer.open();
        drawer.handle_backdrop_click();
        assert!(drawer.is_drawer_open()); // Should stay open
    }

    #[test]
    fn drawer_handle_escape() {
        let mut drawer = Drawer::new()
            .variant(DrawerVariant::Temporary)
            .close_on_escape(true);

        drawer.open();
        drawer.handle_escape();
        assert!(drawer.is_drawer_closed());
    }

    #[test]
    fn drawer_handle_escape_no_close() {
        let mut drawer = Drawer::new()
            .variant(DrawerVariant::Temporary)
            .close_on_escape(false);

        drawer.open();
        drawer.handle_escape();
        assert!(drawer.is_drawer_open()); // Should stay open
    }

    #[test]
    fn drawer_is_temporary() {
        let drawer = Drawer::new().variant(DrawerVariant::Temporary);
        assert!(drawer.is_temporary());
        assert!(!drawer.is_persistent());
        assert!(!drawer.is_permanent());
    }

    #[test]
    fn drawer_is_persistent() {
        let drawer = Drawer::new().variant(DrawerVariant::Persistent);
        assert!(!drawer.is_temporary());
        assert!(drawer.is_persistent());
        assert!(!drawer.is_permanent());
    }

    #[test]
    fn drawer_is_permanent() {
        let drawer = Drawer::new().variant(DrawerVariant::Permanent);
        assert!(!drawer.is_temporary());
        assert!(!drawer.is_persistent());
        assert!(drawer.is_permanent());
    }

    #[test]
    fn drawer_is_horizontal() {
        let left = Drawer::new().position(DrawerPosition::Left);
        let right = Drawer::new().position(DrawerPosition::Right);
        
        assert!(left.is_horizontal());
        assert!(right.is_horizontal());
        assert!(!left.is_vertical());
    }

    #[test]
    fn drawer_is_vertical() {
        let top = Drawer::new().position(DrawerPosition::Top);
        let bottom = Drawer::new().position(DrawerPosition::Bottom);
        
        assert!(top.is_vertical());
        assert!(bottom.is_vertical());
        assert!(!top.is_horizontal());
    }

    #[test]
    fn drawer_callbacks() {
        use std::sync::{Arc, Mutex};

        let opened = Arc::new(Mutex::new(false));
        let opened_clone = opened.clone();

        let closed = Arc::new(Mutex::new(false));
        let closed_clone = closed.clone();

        let backdrop_clicked = Arc::new(Mutex::new(false));
        let backdrop_clicked_clone = backdrop_clicked.clone();

        let mut drawer = Drawer::new()
            .on_open(move || {
                *opened_clone.lock().unwrap() = true;
            })
            .on_close(move || {
                *closed_clone.lock().unwrap() = true;
            })
            .on_backdrop_click(move || {
                *backdrop_clicked_clone.lock().unwrap() = true;
            });

        drawer.open();
        assert!(*opened.lock().unwrap());

        drawer.handle_backdrop_click();
        assert!(*backdrop_clicked.lock().unwrap());
        assert!(*closed.lock().unwrap());
    }

    #[test]
    fn drawer_builder_pattern() {
        let drawer = Drawer::new()
            .position(DrawerPosition::Right)
            .variant(DrawerVariant::Persistent)
            .width(320.0)
            .height(500.0)
            .backdrop_opacity(0.7)
            .show_backdrop(false)
            .close_on_backdrop_click(false)
            .close_on_escape(false)
            .background_color(50, 50, 50, 255)
            .backdrop_color(0, 0, 0, 200)
            .shadow_blur(15.0)
            .animation_duration(0.5);

        assert_eq!(drawer.position, DrawerPosition::Right);
        assert_eq!(drawer.variant, DrawerVariant::Persistent);
        assert_eq!(drawer.width, 320.0);
        assert_eq!(drawer.height, 500.0);
        assert_eq!(drawer.backdrop_opacity, 0.7);
        assert!(!drawer.show_backdrop);
        assert!(!drawer.close_on_backdrop_click);
        assert!(!drawer.close_on_escape);
        assert_eq!(drawer.background_color, (50, 50, 50, 255));
        assert_eq!(drawer.backdrop_color, (0, 0, 0, 200));
        assert_eq!(drawer.shadow_blur, 15.0);
        assert_eq!(drawer.animation_duration, 0.5);
    }

    #[test]
    fn drawer_build_creates_node() {
        let mut engine = LayoutEngine::new();
        let mut drawer = Drawer::new();

        let result = drawer.build(&mut engine);
        assert!(result.is_ok());
        assert!(drawer.node_id.is_some());
    }
}
