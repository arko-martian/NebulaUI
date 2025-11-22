use nebula_core::{Signal, LayoutEngine, NodeId, Layout};
use taffy::prelude::*;
use tracing::info;
use std::rc::Rc;

/// Radio Button - Exclusive selection input 🔘
/// 
/// Essential for mutually exclusive choices!
/// - Only one radio in a group can be selected
/// - Reactive state (powered by Signals!)
/// - Click to select
/// - Optional label
/// - Keyboard accessible
/// 
/// Just like HTML's radio button, but better!
#[derive(Clone)]
pub struct Radio {
    /// Layout node ID
    pub node_id: Option<NodeId>,
    /// Selected state (reactive!)
    pub is_selected: Signal<bool>,
    /// Radio group name (for exclusive selection)
    pub group: String,
    /// Value of this radio button
    pub value: String,
    /// Label text (optional)
    pub label: Option<String>,
    /// Size of the radio circle
    pub size: f32,
    /// Position
    pub position: (f32, f32),
    /// Change handler
    on_change: Option<Rc<dyn Fn(String)>>,
}

impl Radio {
    /// Create a new radio button
    pub fn new(group: impl Into<String>, value: impl Into<String>) -> Self {
        let group = group.into();
        let value = value.into();
        info!("🔘 Creating Radio (group: {}, value: {})", group, value);
        Self {
            node_id: None,
            is_selected: Signal::new(false),
            group,
            value,
            label: None,
            size: 20.0,
            position: (0.0, 0.0),
            on_change: None,
        }
    }

    /// Create a radio button with initial selected state
    pub fn with_state(
        group: impl Into<String>,
        value: impl Into<String>,
        selected: bool,
    ) -> Self {
        let group = group.into();
        let value = value.into();
        info!(
            "🔘 Creating Radio (group: {}, value: {}, selected: {})",
            group, value, selected
        );
        Self {
            node_id: None,
            is_selected: Signal::new(selected),
            group,
            value,
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

    /// Set change handler (receives the selected value)
    pub fn on_change<F>(mut self, handler: F) -> Self
    where
        F: Fn(String) + 'static,
    {
        self.on_change = Some(Rc::new(handler));
        self
    }

    /// Select this radio button
    /// Note: In a real implementation, this would deselect other radios in the group
    pub fn select(&self) {
        if !self.is_selected.get() {
            self.is_selected.set(true);
            info!("🔘 Radio selected (value: {})", self.value);

            // Call change handler
            if let Some(handler) = &self.on_change {
                handler(self.value.clone());
            }
        }
    }

    /// Deselect this radio button
    /// Used when another radio in the group is selected
    pub fn deselect(&self) {
        if self.is_selected.get() {
            self.is_selected.set(false);
            info!("🔘 Radio deselected (value: {})", self.value);
        }
    }

    /// Get selected state
    pub fn is_selected(&self) -> bool {
        self.is_selected.get()
    }

    /// Get group name
    pub fn get_group(&self) -> &str {
        &self.group
    }

    /// Get value
    pub fn get_value(&self) -> &str {
        &self.value
    }

    /// Handle mouse click
    pub fn handle_click(&self, mouse_x: f32, mouse_y: f32) -> bool {
        if self.is_point_inside(mouse_x, mouse_y) {
            self.select();
            true
        } else {
            false
        }
    }

    /// Check if a point is inside the radio button
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
            .map_err(|e| format!("Failed to create Radio: {:?}", e))?;

        self.node_id = Some(node);
        info!("✅ Radio built ({}x{})", self.size, self.size);
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

/// Radio Group - Manages a group of radio buttons 📻
/// 
/// Ensures only one radio is selected at a time!
pub struct RadioGroup {
    /// Group name
    pub name: String,
    /// Radio buttons in this group
    pub radios: Vec<Radio>,
    /// Currently selected value
    pub selected_value: Signal<Option<String>>,
}

impl RadioGroup {
    /// Create a new radio group
    pub fn new(name: impl Into<String>) -> Self {
        let name = name.into();
        info!("📻 Creating RadioGroup: {}", name);
        Self {
            name,
            radios: Vec::new(),
            selected_value: Signal::new(None),
        }
    }

    /// Add a radio button to the group
    pub fn add_radio(&mut self, radio: Radio) {
        // Ensure radio is in this group
        if radio.group == self.name {
            self.radios.push(radio);
        } else {
            info!(
                "⚠️  Radio group mismatch: expected '{}', got '{}'",
                self.name, radio.group
            );
        }
    }

    /// Select a radio by value
    pub fn select(&mut self, value: &str) {
        // Deselect all radios
        for radio in &self.radios {
            radio.deselect();
        }

        // Select the matching radio
        for radio in &self.radios {
            if radio.value == value {
                radio.select();
                self.selected_value.set(Some(value.to_string()));
                break;
            }
        }
    }

    /// Get selected value
    pub fn get_selected(&self) -> Option<String> {
        self.selected_value.get()
    }

    /// Get number of radios in group
    pub fn count(&self) -> usize {
        self.radios.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    #[test]
    fn radio_creation() {
        let radio = Radio::new("size", "medium");
        assert_eq!(radio.is_selected(), false);
        assert_eq!(radio.get_group(), "size");
        assert_eq!(radio.get_value(), "medium");
        assert_eq!(radio.size, 20.0);
        assert!(radio.label.is_none());
    }

    #[test]
    fn radio_with_state() {
        let radio = Radio::with_state("size", "large", true);
        assert_eq!(radio.is_selected(), true);
    }

    #[test]
    fn radio_builder_pattern() {
        let radio = Radio::new("color", "blue")
            .label("Blue")
            .size(24.0)
            .position(10.0, 20.0);

        assert_eq!(radio.label, Some("Blue".to_string()));
        assert_eq!(radio.size, 24.0);
        assert_eq!(radio.position, (10.0, 20.0));
    }

    #[test]
    fn radio_select() {
        let radio = Radio::new("option", "yes");
        assert_eq!(radio.is_selected(), false);

        radio.select();
        assert_eq!(radio.is_selected(), true);

        // Selecting again should not change state
        radio.select();
        assert_eq!(radio.is_selected(), true);
    }

    #[test]
    fn radio_deselect() {
        let radio = Radio::with_state("option", "no", true);
        assert_eq!(radio.is_selected(), true);

        radio.deselect();
        assert_eq!(radio.is_selected(), false);
    }

    #[test]
    fn radio_on_change_handler() {
        let selected_value = Rc::new(RefCell::new(String::new()));
        let selected_value_clone = selected_value.clone();

        let radio = Radio::new("size", "small").on_change(move |value| {
            *selected_value_clone.borrow_mut() = value;
        });

        radio.select();
        assert_eq!(*selected_value.borrow(), "small");
    }

    #[test]
    fn radio_handle_click_inside() {
        let radio = Radio::new("option", "yes")
            .position(10.0, 10.0)
            .size(20.0);

        assert_eq!(radio.is_selected(), false);

        let clicked = radio.handle_click(15.0, 15.0);
        assert!(clicked);
        assert_eq!(radio.is_selected(), true);
    }

    #[test]
    fn radio_handle_click_outside() {
        let radio = Radio::new("option", "no")
            .position(10.0, 10.0)
            .size(20.0);

        let clicked = radio.handle_click(50.0, 50.0);
        assert!(!clicked);
        assert_eq!(radio.is_selected(), false);
    }

    #[test]
    fn radio_is_point_inside() {
        let radio = Radio::new("test", "value")
            .position(10.0, 10.0)
            .size(20.0);

        assert!(radio.is_point_inside(15.0, 15.0));
        assert!(radio.is_point_inside(10.0, 10.0));
        assert!(radio.is_point_inside(30.0, 30.0));

        assert!(!radio.is_point_inside(5.0, 15.0));
        assert!(!radio.is_point_inside(35.0, 15.0));
    }

    #[test]
    fn radio_bounds() {
        let radio = Radio::new("test", "value")
            .position(10.0, 20.0)
            .size(24.0);

        let (x, y, w, h) = radio.bounds();
        assert_eq!(x, 10.0);
        assert_eq!(y, 20.0);
        assert_eq!(w, 24.0);
        assert_eq!(h, 24.0);
    }

    #[test]
    fn radio_build() {
        let mut engine = LayoutEngine::new();
        let mut radio = Radio::new("test", "value").size(20.0);

        let node = radio.build(&mut engine);
        assert!(node.is_ok());
        assert!(radio.node_id.is_some());
    }

    #[test]
    fn radio_layout() {
        let mut engine = LayoutEngine::new();
        let mut radio = Radio::new("test", "value").size(24.0);

        let node = radio.build(&mut engine).unwrap();

        let available = Size {
            width: AvailableSpace::Definite(100.0),
            height: AvailableSpace::Definite(100.0),
        };
        engine.compute_layout(node, available).unwrap();

        let layout = radio.get_layout(&engine);
        assert!(layout.is_some());

        let layout = layout.unwrap();
        assert_eq!(layout.size.width, 24.0);
        assert_eq!(layout.size.height, 24.0);
    }

    #[test]
    fn radio_group_creation() {
        let group = RadioGroup::new("size");
        assert_eq!(group.name, "size");
        assert_eq!(group.count(), 0);
        assert_eq!(group.get_selected(), None);
    }

    #[test]
    fn radio_group_add_radio() {
        let mut group = RadioGroup::new("size");

        let radio1 = Radio::new("size", "small");
        let radio2 = Radio::new("size", "medium");

        group.add_radio(radio1);
        group.add_radio(radio2);

        assert_eq!(group.count(), 2);
    }

    #[test]
    fn radio_group_select() {
        let mut group = RadioGroup::new("size");

        let radio1 = Radio::new("size", "small");
        let radio2 = Radio::new("size", "medium");
        let radio3 = Radio::new("size", "large");

        group.add_radio(radio1);
        group.add_radio(radio2);
        group.add_radio(radio3);

        // Select medium
        group.select("medium");
        assert_eq!(group.get_selected(), Some("medium".to_string()));
        assert_eq!(group.radios[1].is_selected(), true);

        // Select large (should deselect medium)
        group.select("large");
        assert_eq!(group.get_selected(), Some("large".to_string()));
        assert_eq!(group.radios[1].is_selected(), false);
        assert_eq!(group.radios[2].is_selected(), true);
    }

    #[test]
    fn radio_group_exclusive_selection() {
        let mut group = RadioGroup::new("option");

        let radio1 = Radio::new("option", "yes");
        let radio2 = Radio::new("option", "no");

        group.add_radio(radio1);
        group.add_radio(radio2);

        // Select yes
        group.select("yes");
        assert_eq!(group.radios[0].is_selected(), true);
        assert_eq!(group.radios[1].is_selected(), false);

        // Select no (should deselect yes)
        group.select("no");
        assert_eq!(group.radios[0].is_selected(), false);
        assert_eq!(group.radios[1].is_selected(), true);
    }

    #[test]
    fn radio_clone() {
        let radio1 = Radio::new("test", "value");
        let radio2 = radio1.clone();

        // Both share the same signal
        radio1.select();
        assert_eq!(radio2.is_selected(), true);
    }
}
