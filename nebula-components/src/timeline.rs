// Timeline Component - Timeline view for events
// Essential for activity feeds and history

use nebula_core::layout::{LayoutEngine, NodeId};
use nebula_core::signal::Signal;

/// Timeline item
#[derive(Debug, Clone, PartialEq)]
pub struct TimelineItem {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub timestamp: String,
    pub icon: Option<String>,
    pub color: Option<(u8, u8, u8, u8)>,
    pub metadata: Option<String>,
}

impl TimelineItem {
    /// Create a new timeline item
    pub fn new(id: impl Into<String>, title: impl Into<String>, timestamp: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            description: None,
            timestamp: timestamp.into(),
            icon: None,
            color: None,
            metadata: None,
        }
    }

    /// Add a description
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Add an icon
    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Set color
    pub fn with_color(mut self, r: u8, g: u8, b: u8, a: u8) -> Self {
        self.color = Some((r, g, b, a));
        self
    }

    /// Add metadata
    pub fn with_metadata(mut self, metadata: impl Into<String>) -> Self {
        self.metadata = Some(metadata.into());
        self
    }
}

/// Timeline mode
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TimelineMode {
    Left,      // Items on left, line on right
    Right,     // Items on right, line on left
    Alternate, // Items alternate left/right
}

/// Timeline component - timeline view for events
/// 
/// # Example
/// ```
/// let mut timeline = Timeline::new()
///     .add_item("item1", "Event 1", "2025-11-22 10:00")
///     .add_item("item2", "Event 2", "2025-11-22 11:00")
///     .mode(TimelineMode::Alternate)
///     .on_item_click(|id| println!("Clicked: {}", id));
/// ```
pub struct Timeline {
    pub node_id: Option<NodeId>,
    pub items: Vec<TimelineItem>,
    pub mode: TimelineMode,
    pub line_width: f32,
    pub dot_size: f32,
    pub spacing: f32,
    pub line_color: (u8, u8, u8, u8),
    pub dot_color: (u8, u8, u8, u8),
    pub background_color: (u8, u8, u8, u8),
    pub text_color: (u8, u8, u8, u8),
    pub timestamp_color: (u8, u8, u8, u8),
    pub show_icons: bool,
    pub clickable: bool,
    pub on_item_click: Option<Box<dyn Fn(&str)>>,
}

impl Timeline {
    /// Create a new Timeline component
    pub fn new() -> Self {
        Self {
            node_id: None,
            items: Vec::new(),
            mode: TimelineMode::Left,
            line_width: 2.0,
            dot_size: 12.0,
            spacing: 32.0,
            line_color: (220, 220, 220, 255),
            dot_color: (59, 130, 246, 255), // Blue
            background_color: (255, 255, 255, 255),
            text_color: (0, 0, 0, 255),
            timestamp_color: (100, 100, 100, 255),
            show_icons: true,
            clickable: false,
            on_item_click: None,
        }
    }

    /// Set the mode
    pub fn mode(mut self, mode: TimelineMode) -> Self {
        self.mode = mode;
        self
    }

    /// Set line width
    pub fn line_width(mut self, width: f32) -> Self {
        self.line_width = width;
        self
    }

    /// Set dot size
    pub fn dot_size(mut self, size: f32) -> Self {
        self.dot_size = size;
        self
    }

    /// Set spacing between items
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }

    /// Set line color
    pub fn line_color(mut self, r: u8, g: u8, b: u8, a: u8) -> Self {
        self.line_color = (r, g, b, a);
        self
    }

    /// Set dot color
    pub fn dot_color(mut self, r: u8, g: u8, b: u8, a: u8) -> Self {
        self.dot_color = (r, g, b, a);
        self
    }

    /// Show or hide icons
    pub fn show_icons(mut self, show: bool) -> Self {
        self.show_icons = show;
        self
    }

    /// Make items clickable
    pub fn clickable(mut self, clickable: bool) -> Self {
        self.clickable = clickable;
        self
    }

    /// Add a timeline item
    pub fn add_item(mut self, id: impl Into<String>, title: impl Into<String>, timestamp: impl Into<String>) -> Self {
        self.items.push(TimelineItem::new(id, title, timestamp));
        self
    }

    /// Add a timeline item object
    pub fn add_item_object(mut self, item: TimelineItem) -> Self {
        self.items.push(item);
        self
    }

    /// Set all items at once
    pub fn items(mut self, items: Vec<TimelineItem>) -> Self {
        self.items = items;
        self
    }

    /// Set the item click callback
    pub fn on_item_click<F>(mut self, callback: F) -> Self
    where
        F: Fn(&str) + 'static,
    {
        self.on_item_click = Some(Box::new(callback));
        self
    }

    /// Handle item click
    pub fn handle_item_click(&mut self, id: &str) {
        if !self.clickable {
            return;
        }

        if let Some(ref callback) = self.on_item_click {
            callback(id);
        }
    }

    /// Get item count
    pub fn item_count(&self) -> usize {
        self.items.len()
    }

    /// Check if has items
    pub fn has_items(&self) -> bool {
        !self.items.is_empty()
    }

    /// Find item by ID
    pub fn find_item(&self, id: &str) -> Option<usize> {
        self.items.iter().position(|item| item.id == id)
    }

    /// Get item by index
    pub fn get_item(&self, index: usize) -> Option<&TimelineItem> {
        self.items.get(index)
    }

    /// Remove item by ID
    pub fn remove_item(&mut self, id: &str) {
        if let Some(index) = self.find_item(id) {
            self.items.remove(index);
        }
    }

    /// Clear all items
    pub fn clear(&mut self) {
        self.items.clear();
    }

    /// Check if item should be on left (for alternate mode)
    pub fn is_item_on_left(&self, index: usize) -> bool {
        match self.mode {
            TimelineMode::Left => true,
            TimelineMode::Right => false,
            TimelineMode::Alternate => index % 2 == 0,
        }
    }

    /// Build the timeline layout
    pub fn build(&mut self, engine: &mut LayoutEngine) -> Result<NodeId, String> {
        let style = taffy::style::Style {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Percent(1.0),
                height: taffy::style::Dimension::Auto,
            },
            display: taffy::style::Display::Flex,
            flex_direction: taffy::style::FlexDirection::Column,
            gap: taffy::geometry::Size {
                width: taffy::style::LengthPercentage::Length(0.0),
                height: taffy::style::LengthPercentage::Length(self.spacing),
            },
            ..Default::default()
        };

        let node = engine
            .new_leaf(style)
            .map_err(|e| format!("Failed to create timeline node: {:?}", e))?;
        self.node_id = Some(node);

        Ok(node)
    }
}

impl Default for Timeline {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn timeline_starts_empty() {
        let timeline = Timeline::new();
        assert_eq!(timeline.item_count(), 0);
        assert!(!timeline.has_items());
    }

    #[test]
    fn timeline_add_items() {
        let timeline = Timeline::new()
            .add_item("item1", "Event 1", "10:00")
            .add_item("item2", "Event 2", "11:00")
            .add_item("item3", "Event 3", "12:00");

        assert_eq!(timeline.item_count(), 3);
        assert!(timeline.has_items());
    }

    #[test]
    fn timeline_find_item() {
        let timeline = Timeline::new()
            .add_item("item1", "Event 1", "10:00")
            .add_item("item2", "Event 2", "11:00");

        assert_eq!(timeline.find_item("item2"), Some(1));
        assert_eq!(timeline.find_item("nonexistent"), None);
    }

    #[test]
    fn timeline_get_item() {
        let timeline = Timeline::new()
            .add_item("item1", "Event 1", "10:00");

        let item = timeline.get_item(0);
        assert!(item.is_some());
        assert_eq!(item.unwrap().title, "Event 1");
    }

    #[test]
    fn timeline_remove_item() {
        let mut timeline = Timeline::new()
            .add_item("item1", "Event 1", "10:00")
            .add_item("item2", "Event 2", "11:00");

        assert_eq!(timeline.item_count(), 2);
        
        timeline.remove_item("item1");
        assert_eq!(timeline.item_count(), 1);
        assert_eq!(timeline.items[0].id, "item2");
    }

    #[test]
    fn timeline_clear() {
        let mut timeline = Timeline::new()
            .add_item("item1", "Event 1", "10:00")
            .add_item("item2", "Event 2", "11:00");

        timeline.clear();
        assert_eq!(timeline.item_count(), 0);
    }

    #[test]
    fn timeline_mode_left() {
        let timeline = Timeline::new().mode(TimelineMode::Left);
        
        assert!(timeline.is_item_on_left(0));
        assert!(timeline.is_item_on_left(1));
        assert!(timeline.is_item_on_left(2));
    }

    #[test]
    fn timeline_mode_right() {
        let timeline = Timeline::new().mode(TimelineMode::Right);
        
        assert!(!timeline.is_item_on_left(0));
        assert!(!timeline.is_item_on_left(1));
        assert!(!timeline.is_item_on_left(2));
    }

    #[test]
    fn timeline_mode_alternate() {
        let timeline = Timeline::new().mode(TimelineMode::Alternate);
        
        assert!(timeline.is_item_on_left(0));
        assert!(!timeline.is_item_on_left(1));
        assert!(timeline.is_item_on_left(2));
        assert!(!timeline.is_item_on_left(3));
    }

    #[test]
    fn timeline_handle_item_click() {
        let mut timeline = Timeline::new()
            .add_item("item1", "Event 1", "10:00")
            .clickable(true);

        // Should not panic
        timeline.handle_item_click("item1");
    }

    #[test]
    fn timeline_handle_item_click_not_clickable() {
        let mut timeline = Timeline::new()
            .add_item("item1", "Event 1", "10:00")
            .clickable(false);

        // Should not trigger callback
        timeline.handle_item_click("item1");
    }

    #[test]
    fn timeline_item_with_description_and_icon() {
        let item = TimelineItem::new("item1", "Event", "10:00")
            .with_description("Description")
            .with_icon("check")
            .with_color(255, 0, 0, 255)
            .with_metadata("Important");

        assert_eq!(item.description, Some("Description".to_string()));
        assert_eq!(item.icon, Some("check".to_string()));
        assert_eq!(item.color, Some((255, 0, 0, 255)));
        assert_eq!(item.metadata, Some("Important".to_string()));
    }

    #[test]
    fn timeline_callback() {
        use std::sync::{Arc, Mutex};

        let clicked = Arc::new(Mutex::new(String::new()));
        let clicked_clone = clicked.clone();

        let mut timeline = Timeline::new()
            .add_item("item1", "Event 1", "10:00")
            .clickable(true)
            .on_item_click(move |id| {
                *clicked_clone.lock().unwrap() = id.to_string();
            });

        timeline.handle_item_click("item1");
        assert_eq!(*clicked.lock().unwrap(), "item1");
    }

    #[test]
    fn timeline_builder_pattern() {
        let timeline = Timeline::new()
            .mode(TimelineMode::Alternate)
            .line_width(3.0)
            .dot_size(16.0)
            .spacing(40.0)
            .line_color(200, 200, 200, 255)
            .dot_color(255, 0, 0, 255)
            .show_icons(false)
            .clickable(true);

        assert_eq!(timeline.mode, TimelineMode::Alternate);
        assert_eq!(timeline.line_width, 3.0);
        assert_eq!(timeline.dot_size, 16.0);
        assert_eq!(timeline.spacing, 40.0);
        assert_eq!(timeline.line_color, (200, 200, 200, 255));
        assert_eq!(timeline.dot_color, (255, 0, 0, 255));
        assert!(!timeline.show_icons);
        assert!(timeline.clickable);
    }

    #[test]
    fn timeline_build_creates_node() {
        let mut engine = LayoutEngine::new();
        let mut timeline = Timeline::new()
            .add_item("item1", "Event 1", "10:00");

        let result = timeline.build(&mut engine);
        assert!(result.is_ok());
        assert!(timeline.node_id.is_some());
    }
}
