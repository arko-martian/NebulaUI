use nebula_core::{Signal, LayoutEngine, NodeId, Layout};
use taffy::prelude::*;
use tracing::info;
use std::rc::Rc;

/// Checkbox - Interactive boolean input ✅
/// 
/// Essential for forms, settings, todo lists!
/// - Reactive state (powered by Signals!)
/// - Click to toggle
/// - Optional label
/// - Keyboard accessible
/// 
/// Just like HTML's checkbox, but better!
#[derive(Clone)]
pub struct Checkbox {
    /// Layout node ID
    pub node_id: Option<NodeId>,
    /// Checked state (reactive!)
    pub is_checked: Signal<bool>,
    /// Label text (optional)
    pub label: Option<String>,
    /// Size of the checkbox box
    pub size: f32,
    /// Position
    pub position: (f32, f32),
    /// Change handler
    on_change: Option<Rc<dyn Fn(bool)>>,
}

impl Checkbox {
    /// Create a new checkbox (unchecked by default)
    pub fn new() -> Self {
        info!("✅ Creating Checkbox");
        Self {
            node_id: None,
            is_checked: Signal::new(false),
            label: None,
            size: 20.0,
            position: (0.0, 0.0),
            on_change: None,
        }
    }

    /// Create a checkbox with initial checked state
    pub fn with_state(checked: bool) -> Self {
        info!("✅ Creating Checkbox (checked: {})", checked);
        Self {
            node_id: None,
            is_checked: Signal::new(checked),
            label: None,
            size: 20.0,
            position: (0.0, 0.0),
            on_change: None,
        }
    }

    /// Create a checkbox from a Signal
    pub fn from_signal(is_checked: Signal<bool>) -> Self {
        info!("✅ Creating Checkbox from Signal");
        Self {
            node_id: None,
            is_checked,
            label: None,
            size: 20.0,
            position: (0.0, 0.0),
            on_change: None,
        }
    }

    /// Set label
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Set size
    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    /// Set position
    pub fn position(mut self, x: f32, y: f32) -> Self {
        self.position = (x, y);
        self
    }

    /// Set change handler
    pub fn on_change<F>(mut self, handler: F) -> Self
    where
        F: Fn(bool) + 'static,
    {
        self.on_change = Some(Rc::new(handler));
        self
    }

    /// Toggle the checkbox
    pub fn toggle(&self) {
        let new_state = !self.is_checked.get();
        self.is_checked.set(new_state);
        
        info!("✅ Checkbox toggled to: {}", new_state);
        
        // Call change handler
        if let Some(handler) = &self.on_change {
            handler(new_state);
        }
    }

    /// Set checked state
    pub fn set_checked(&self, checked: bool) {
        if self.is_checked.get() != checked {
            self.is_checked.set(checked);
            
            info!("✅ Checkbox set to: {}", checked);
            
            // Call change handler
            if let Some(handler) = &self.on_change {
                handler(checked);
            }
        }
    }

    /// Get checked state
    pub fn is_checked(&self) -> bool {
        self.is_checked.get()
    }

    /// Handle mouse click
    pub fn handle_click(&self, mouse_x: f32, mouse_y: f32) -> bool {
        if self.is_point_inside(mouse_x, mouse_y) {
            self.toggle();
            true
        } else {
            false
        }
    }

    /// Check if a point is inside the checkbox
    pub fn is_point_inside(&self, x: f32, y: f32) -> bool {
        let (cx, cy) = self.position;
        let size = self.size;
        
        x >= cx && x <= cx + size && y >= cy && y <= cy + size
    }

    /// Build the layout node
    pub fn build(&mut self, engine: &mut LayoutEngine) -> Result<NodeId, String> {
        let style = Style {
            size: Size {
                width: Dimension::Length(self.size),
                height: Dimension::Length(self.size),
            },
            ..Default::default()
        };

        let node = engine
            .new_leaf(style)
            .map_err(|e| format!("Failed to create Checkbox: {:?}", e))?;

        self.node_id = Some(node);
        info!("✅ Checkbox built ({}x{})", self.size, self.size);
        Ok(node)
    }

    /// Get the layout
    pub fn get_layout(&self, engine: &LayoutEngine) -> Option<Layout> {
        self.node_id.and_then(|id| engine.get_layout(id).ok())
    }

    /// Get bounds (x, y, width, height)
    pub fn bounds(&self) -> (f32, f32, f32, f32) {
        (self.position.0, self.position.1, self.size, self.size)
    }
}

impl Default for Checkbox {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    #[test]
    fn checkbox_creation() {
        let checkbox = Checkbox::new();
        assert_eq!(checkbox.is_checked(), false);
        assert_eq!(checkbox.size, 20.0);
        assert_eq!(checkbox.position, (0.0, 0.0));
        assert!(checkbox.label.is_none());
    }

    #[test]
    fn checkbox_with_state() {
        let checkbox = Checkbox::with_state(true);
        assert_eq!(checkbox.is_checked(), true);
    }

    #[test]
    fn checkbox_from_signal() {
        let signal = Signal::new(true);
        let checkbox = Checkbox::from_signal(signal.clone());
        
        assert_eq!(checkbox.is_checked(), true);
        
        // Changing signal changes checkbox
        signal.set(false);
        assert_eq!(checkbox.is_checked(), false);
    }

    #[test]
    fn checkbox_builder_pattern() {
        let checkbox = Checkbox::new()
            .label("Accept terms")
            .size(24.0)
            .position(10.0, 20.0);

        assert_eq!(checkbox.label, Some("Accept terms".to_string()));
        assert_eq!(checkbox.size, 24.0);
        assert_eq!(checkbox.position, (10.0, 20.0));
    }

    #[test]
    fn checkbox_toggle() {
        let checkbox = Checkbox::new();
        assert_eq!(checkbox.is_checked(), false);

        checkbox.toggle();
        assert_eq!(checkbox.is_checked(), true);

        checkbox.toggle();
        assert_eq!(checkbox.is_checked(), false);
    }

    #[test]
    fn checkbox_set_checked() {
        let checkbox = Checkbox::new();
        
        checkbox.set_checked(true);
        assert_eq!(checkbox.is_checked(), true);

        checkbox.set_checked(false);
        assert_eq!(checkbox.is_checked(), false);
    }

    #[test]
    fn checkbox_on_change_handler() {
        let changed = Rc::new(RefCell::new(false));
        let changed_clone = changed.clone();

        let checkbox = Checkbox::new().on_change(move |checked| {
            *changed_clone.borrow_mut() = checked;
        });

        // Toggle should trigger handler
        checkbox.toggle();
        assert_eq!(*changed.borrow(), true);

        checkbox.toggle();
        assert_eq!(*changed.borrow(), false);
    }

    #[test]
    fn checkbox_handle_click_inside() {
        let checkbox = Checkbox::new()
            .position(10.0, 10.0)
            .size(20.0);

        assert_eq!(checkbox.is_checked(), false);

        // Click inside
        let clicked = checkbox.handle_click(15.0, 15.0);
        assert!(clicked);
        assert_eq!(checkbox.is_checked(), true);
    }

    #[test]
    fn checkbox_handle_click_outside() {
        let checkbox = Checkbox::new()
            .position(10.0, 10.0)
            .size(20.0);

        assert_eq!(checkbox.is_checked(), false);

        // Click outside
        let clicked = checkbox.handle_click(50.0, 50.0);
        assert!(!clicked);
        assert_eq!(checkbox.is_checked(), false);
    }

    #[test]
    fn checkbox_is_point_inside() {
        let checkbox = Checkbox::new()
            .position(10.0, 10.0)
            .size(20.0);

        // Inside
        assert!(checkbox.is_point_inside(15.0, 15.0));
        assert!(checkbox.is_point_inside(10.0, 10.0)); // Top-left corner
        assert!(checkbox.is_point_inside(30.0, 30.0)); // Bottom-right corner

        // Outside
        assert!(!checkbox.is_point_inside(5.0, 15.0)); // Left
        assert!(!checkbox.is_point_inside(35.0, 15.0)); // Right
        assert!(!checkbox.is_point_inside(15.0, 5.0)); // Above
        assert!(!checkbox.is_point_inside(15.0, 35.0)); // Below
    }

    #[test]
    fn checkbox_bounds() {
        let checkbox = Checkbox::new()
            .position(10.0, 20.0)
            .size(24.0);

        let (x, y, w, h) = checkbox.bounds();
        assert_eq!(x, 10.0);
        assert_eq!(y, 20.0);
        assert_eq!(w, 24.0);
        assert_eq!(h, 24.0);
    }

    #[test]
    fn checkbox_build() {
        let mut engine = LayoutEngine::new();
        let mut checkbox = Checkbox::new().size(20.0);

        let node = checkbox.build(&mut engine);
        assert!(node.is_ok());
        assert!(checkbox.node_id.is_some());
    }

    #[test]
    fn checkbox_layout() {
        let mut engine = LayoutEngine::new();
        let mut checkbox = Checkbox::new().size(24.0);

        let node = checkbox.build(&mut engine).unwrap();

        // Compute layout
        let available = Size {
            width: AvailableSpace::Definite(100.0),
            height: AvailableSpace::Definite(100.0),
        };
        engine.compute_layout(node, available).unwrap();

        let layout = checkbox.get_layout(&engine);
        assert!(layout.is_some());

        let layout = layout.unwrap();
        assert_eq!(layout.size.width, 24.0);
        assert_eq!(layout.size.height, 24.0);
    }

    #[test]
    fn checkbox_reactive_state() {
        let checkbox = Checkbox::new();
        let state = checkbox.is_checked.clone();

        // Subscribe to changes
        let changed = Rc::new(RefCell::new(0));
        let changed_clone = changed.clone();
        state.subscribe(move |_| {
            *changed_clone.borrow_mut() += 1;
        });

        // Toggle should notify subscribers
        checkbox.toggle();
        assert_eq!(*changed.borrow(), 1);

        checkbox.toggle();
        assert_eq!(*changed.borrow(), 2);
    }

    #[test]
    fn checkbox_default() {
        let checkbox = Checkbox::default();
        assert_eq!(checkbox.is_checked(), false);
    }

    #[test]
    fn checkbox_clone() {
        let checkbox1 = Checkbox::new();
        let checkbox2 = checkbox1.clone();

        // Both share the same signal
        checkbox1.toggle();
        assert_eq!(checkbox2.is_checked(), true);
    }

    #[test]
    fn checkbox_multiple_clicks() {
        let click_count = Rc::new(RefCell::new(0));
        let click_count_clone = click_count.clone();

        let checkbox = Checkbox::new()
            .position(0.0, 0.0)
            .size(20.0)
            .on_change(move |_| {
                *click_count_clone.borrow_mut() += 1;
            });

        // Multiple clicks
        checkbox.handle_click(10.0, 10.0);
        checkbox.handle_click(10.0, 10.0);
        checkbox.handle_click(10.0, 10.0);

        assert_eq!(*click_count.borrow(), 3);
        assert_eq!(checkbox.is_checked(), true); // Odd number of clicks
    }
}
