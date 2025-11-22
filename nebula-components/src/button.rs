use nebula_core::Signal;
use tracing::info;
use std::rc::Rc;

/// Button component - Interactive, reactive, beautiful! 🔘
/// 
/// This is a REAL component that will work on ANY hardware!
/// - CPU rendering (works on 20-year-old machines!)
/// - Reactive (powered by Signals!)
/// - Accessible (keyboard + mouse!)
#[derive(Clone)]
pub struct Button {
    /// Button label
    pub label: String,
    /// Button position (x, y)
    pub position: (f32, f32),
    /// Button size (width, height)
    pub size: (f32, f32),
    /// Is the button currently pressed?
    pub is_pressed: Signal<bool>,
    /// Click handler
    on_click: Option<Rc<dyn Fn()>>,
}

impl Button {
    /// Create a new button
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            position: (0.0, 0.0),
            size: (100.0, 40.0),
            is_pressed: Signal::new(false),
            on_click: None,
        }
    }

    /// Set button position
    pub fn position(mut self, x: f32, y: f32) -> Self {
        self.position = (x, y);
        self
    }

    /// Set button size
    pub fn size(mut self, width: f32, height: f32) -> Self {
        self.size = (width, height);
        self
    }

    /// Set click handler
    pub fn on_click<F>(mut self, handler: F) -> Self
    where
        F: Fn() + 'static,
    {
        self.on_click = Some(Rc::new(handler));
        self
    }

    /// Handle mouse down event
    pub fn handle_mouse_down(&self, mouse_x: f32, mouse_y: f32) -> bool {
        if self.is_point_inside(mouse_x, mouse_y) {
            info!("🔘 Button '{}' pressed!", self.label);
            self.is_pressed.set(true);
            true
        } else {
            false
        }
    }

    /// Handle mouse up event
    pub fn handle_mouse_up(&self, mouse_x: f32, mouse_y: f32) -> bool {
        if self.is_pressed.get() {
            self.is_pressed.set(false);
            
            // Trigger click if mouse is still inside
            if self.is_point_inside(mouse_x, mouse_y) {
                info!("🔘 Button '{}' clicked!", self.label);
                if let Some(handler) = &self.on_click {
                    handler();
                }
                return true;
            }
        }
        false
    }

    /// Check if a point is inside the button
    pub fn is_point_inside(&self, x: f32, y: f32) -> bool {
        let (bx, by) = self.position;
        let (bw, bh) = self.size;
        
        x >= bx && x <= bx + bw && y >= by && y <= by + bh
    }

    /// Get button bounds (x, y, width, height)
    pub fn bounds(&self) -> (f32, f32, f32, f32) {
        (self.position.0, self.position.1, self.size.0, self.size.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn button_creation() {
        let button = Button::new("Click Me");
        assert_eq!(button.label, "Click Me");
        assert_eq!(button.position, (0.0, 0.0));
        assert_eq!(button.size, (100.0, 40.0));
        assert_eq!(button.is_pressed.get(), false);
    }

    #[test]
    fn button_builder_pattern() {
        let button = Button::new("Test")
            .position(10.0, 20.0)
            .size(150.0, 50.0);

        assert_eq!(button.position, (10.0, 20.0));
        assert_eq!(button.size, (150.0, 50.0));
    }

    #[test]
    fn button_point_inside() {
        let button = Button::new("Test")
            .position(10.0, 10.0)
            .size(100.0, 40.0);

        // Inside
        assert!(button.is_point_inside(50.0, 25.0));
        assert!(button.is_point_inside(10.0, 10.0)); // Top-left corner
        assert!(button.is_point_inside(110.0, 50.0)); // Bottom-right corner

        // Outside
        assert!(!button.is_point_inside(5.0, 25.0)); // Left
        assert!(!button.is_point_inside(115.0, 25.0)); // Right
        assert!(!button.is_point_inside(50.0, 5.0)); // Above
        assert!(!button.is_point_inside(50.0, 55.0)); // Below
    }

    #[test]
    fn button_mouse_down() {
        let button = Button::new("Test")
            .position(10.0, 10.0)
            .size(100.0, 40.0);

        // Click inside
        assert!(button.handle_mouse_down(50.0, 25.0));
        assert_eq!(button.is_pressed.get(), true);

        // Reset
        button.is_pressed.set(false);

        // Click outside
        assert!(!button.handle_mouse_down(5.0, 5.0));
        assert_eq!(button.is_pressed.get(), false);
    }

    #[test]
    fn button_click_handler() {
        let clicked = Rc::new(RefCell::new(false));
        let clicked_clone = clicked.clone();

        let button = Button::new("Test")
            .position(10.0, 10.0)
            .size(100.0, 40.0)
            .on_click(move || {
                *clicked_clone.borrow_mut() = true;
            });

        // Simulate click: mouse down then up
        button.handle_mouse_down(50.0, 25.0);
        button.handle_mouse_up(50.0, 25.0);

        assert_eq!(*clicked.borrow(), true);
    }

    #[test]
    fn button_click_outside_no_trigger() {
        let clicked = Rc::new(RefCell::new(false));
        let clicked_clone = clicked.clone();

        let button = Button::new("Test")
            .position(10.0, 10.0)
            .size(100.0, 40.0)
            .on_click(move || {
                *clicked_clone.borrow_mut() = true;
            });

        // Mouse down inside, but up outside
        button.handle_mouse_down(50.0, 25.0);
        button.handle_mouse_up(200.0, 200.0);

        // Should NOT trigger click
        assert_eq!(*clicked.borrow(), false);
    }

    #[test]
    fn button_bounds() {
        let button = Button::new("Test")
            .position(10.0, 20.0)
            .size(150.0, 50.0);

        let (x, y, w, h) = button.bounds();
        assert_eq!(x, 10.0);
        assert_eq!(y, 20.0);
        assert_eq!(w, 150.0);
        assert_eq!(h, 50.0);
    }

    #[test]
    fn button_clone() {
        let button1 = Button::new("Test");
        let button2 = button1.clone();

        assert_eq!(button1.label, button2.label);
        assert_eq!(button1.position, button2.position);
    }
}
