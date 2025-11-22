// Modal Component - Full-screen overlay with backdrop
// Provides a container for dialogs, alerts, and other overlay content

use crate::container::ZStack;
use nebula_core::layout::{LayoutEngine, NodeId};
use nebula_core::signal::Signal;

/// Modal component - displays content in a full-screen overlay with backdrop
/// 
/// # Example
/// ```
/// let mut modal = Modal::new()
///     .backdrop_color(Color::rgba(0, 0, 0, 0.5))
///     .on_backdrop_click(|| println!("Backdrop clicked!"))
///     .visible(true);
/// ```
pub struct Modal {
    pub node_id: Option<NodeId>,
    pub visible: Signal<bool>,
    pub backdrop_color: (u8, u8, u8, u8), // RGBA
    pub backdrop_blur: f32,
    pub on_backdrop_click: Option<Box<dyn Fn()>>,
    pub close_on_backdrop_click: bool,
    pub animation_duration: f32, // seconds
    pub z_index: i32,
    pub content_node: Option<NodeId>,
}

impl Modal {
    /// Create a new Modal component
    pub fn new() -> Self {
        Self {
            node_id: None,
            visible: Signal::new(false),
            backdrop_color: (0, 0, 0, 128), // Semi-transparent black
            backdrop_blur: 0.0,
            on_backdrop_click: None,
            close_on_backdrop_click: true,
            animation_duration: 0.3,
            z_index: 1000,
            content_node: None,
        }
    }

    /// Set the backdrop color (RGBA)
    pub fn backdrop_color(mut self, r: u8, g: u8, b: u8, a: u8) -> Self {
        self.backdrop_color = (r, g, b, a);
        self
    }

    /// Set the backdrop blur amount (in pixels)
    pub fn backdrop_blur(mut self, blur: f32) -> Self {
        self.backdrop_blur = blur;
        self
    }

    /// Set the callback for when the backdrop is clicked
    pub fn on_backdrop_click<F>(mut self, callback: F) -> Self
    where
        F: Fn() + 'static,
    {
        self.on_backdrop_click = Some(Box::new(callback));
        self
    }

    /// Set whether clicking the backdrop should close the modal
    pub fn close_on_backdrop_click(mut self, close: bool) -> Self {
        self.close_on_backdrop_click = close;
        self
    }

    /// Set the animation duration for show/hide transitions
    pub fn animation_duration(mut self, duration: f32) -> Self {
        self.animation_duration = duration;
        self
    }

    /// Set the z-index for stacking order
    pub fn z_index(mut self, z: i32) -> Self {
        self.z_index = z;
        self
    }

    /// Set the visibility of the modal
    pub fn visible(self, visible: bool) -> Self {
        self.visible.set(visible);
        self
    }

    /// Show the modal
    pub fn show(&mut self) {
        self.visible.set(true);
    }

    /// Hide the modal
    pub fn hide(&mut self) {
        self.visible.set(false);
    }

    /// Toggle the modal visibility
    pub fn toggle(&mut self) {
        let current = self.visible.get();
        self.visible.set(!current);
    }

    /// Check if the modal is currently visible
    pub fn is_visible(&self) -> bool {
        self.visible.get()
    }

    /// Handle backdrop click event
    pub fn handle_backdrop_click(&mut self) {
        if let Some(ref callback) = self.on_backdrop_click {
            callback();
        }
        
        if self.close_on_backdrop_click {
            self.hide();
        }
    }

    /// Build the modal layout
    pub fn build(&mut self, engine: &mut LayoutEngine) -> Result<NodeId, String> {
        if !self.is_visible() {
            // If not visible, return a zero-sized node
            let style = taffy::style::Style {
                display: taffy::style::Display::None,
                ..Default::default()
            };
            let node = engine.new_leaf(style)
                .map_err(|e| format!("Failed to create hidden modal node: {:?}", e))?;
            self.node_id = Some(node);
            return Ok(node);
        }

        // Create a ZStack for the modal (backdrop + content)
        let mut zstack = ZStack::new();
        
        // Build the ZStack
        let node = zstack.build(engine)?;
        self.node_id = Some(node);

        // Set full-screen size
        let style = taffy::style::Style {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Percent(1.0),
                height: taffy::style::Dimension::Percent(1.0),
            },
            position: taffy::style::Position::Absolute,
            ..Default::default()
        };
        engine.set_style(node, style)
            .map_err(|e| format!("Failed to set modal style: {:?}", e))?;

        Ok(node)
    }

    /// Set the content node for the modal
    pub fn set_content(&mut self, content: NodeId) {
        self.content_node = Some(content);
    }
}

impl Default for Modal {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn modal_starts_hidden() {
        let modal = Modal::new();
        assert!(!modal.is_visible());
    }

    #[test]
    fn modal_can_be_shown() {
        let mut modal = Modal::new();
        modal.show();
        assert!(modal.is_visible());
    }

    #[test]
    fn modal_can_be_hidden() {
        let mut modal = Modal::new().visible(true);
        modal.hide();
        assert!(!modal.is_visible());
    }

    #[test]
    fn modal_can_be_toggled() {
        let mut modal = Modal::new();
        assert!(!modal.is_visible());
        
        modal.toggle();
        assert!(modal.is_visible());
        
        modal.toggle();
        assert!(!modal.is_visible());
    }

    #[test]
    fn modal_builder_pattern() {
        let modal = Modal::new()
            .backdrop_color(255, 0, 0, 200)
            .backdrop_blur(10.0)
            .close_on_backdrop_click(false)
            .animation_duration(0.5)
            .z_index(2000)
            .visible(true);

        assert_eq!(modal.backdrop_color, (255, 0, 0, 200));
        assert_eq!(modal.backdrop_blur, 10.0);
        assert!(!modal.close_on_backdrop_click);
        assert_eq!(modal.animation_duration, 0.5);
        assert_eq!(modal.z_index, 2000);
        assert!(modal.is_visible());
    }

    #[test]
    fn modal_backdrop_click_closes_when_enabled() {
        let mut modal = Modal::new()
            .visible(true)
            .close_on_backdrop_click(true);
        
        assert!(modal.is_visible());
        modal.handle_backdrop_click();
        assert!(!modal.is_visible());
    }

    #[test]
    fn modal_backdrop_click_does_not_close_when_disabled() {
        let mut modal = Modal::new()
            .visible(true)
            .close_on_backdrop_click(false);
        
        assert!(modal.is_visible());
        modal.handle_backdrop_click();
        assert!(modal.is_visible());
    }

    #[test]
    fn modal_backdrop_click_callback_is_called() {
        use std::sync::{Arc, Mutex};
        
        let clicked = Arc::new(Mutex::new(false));
        let clicked_clone = clicked.clone();
        
        let mut modal = Modal::new()
            .visible(true)
            .on_backdrop_click(move || {
                *clicked_clone.lock().unwrap() = true;
            });
        
        modal.handle_backdrop_click();
        assert!(*clicked.lock().unwrap());
    }

    #[test]
    fn modal_build_creates_node() {
        let mut engine = LayoutEngine::new();
        let mut modal = Modal::new().visible(true);
        
        let result = modal.build(&mut engine);
        assert!(result.is_ok());
        assert!(modal.node_id.is_some());
    }

    #[test]
    fn modal_hidden_creates_hidden_node() {
        let mut engine = LayoutEngine::new();
        let mut modal = Modal::new().visible(false);
        
        let result = modal.build(&mut engine);
        assert!(result.is_ok());
        assert!(modal.node_id.is_some());
    }
}
