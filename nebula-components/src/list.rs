// List Component - Simple list with items and selection
// Essential for displaying collections of data

use nebula_core::layout::{LayoutEngine, NodeId};
use nebula_core::signal::Signal;

/// List item
#[derive(Debug, Clone, PartialEq)]
pub struct ListItem {
    pub id: String,
    pub label: String,
    pub disabled: bool,
    pub icon: Option<String>,
    pub badge: Option<String>,
    pub metadata: Option<String>,
}

impl ListItem {
    /// Create a new list item
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            disabled: false,
            icon: None,
            badge: None,
            metadata: None,
        }
    }

    /// Create a disabled list item
    pub fn disabled(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            disabled: true,
            icon: None,
            badge: None,
            metadata: None,
        }
    }

    /// Add an icon
    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Add a badge
    pub fn with_badge(mut self, badge: impl Into<String>) -> Self {
        self.badge = Some(badge.into());
        self
    }

    /// Add metadata
    pub fn with_metadata(mut self, metadata: impl Into<String>) -> Self {
        self.metadata = Some(metadata.into());
        self
    }
}

/// Selection mode for the list
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SelectionMode {
    None,
    Single,
    Multiple,
}

/// List component - display and manage collections of items
/// 
/// # Example
/// ```
/// let mut list = List::new()
///     .add_item("item1", "First Item")
///     .add_item("item2", "Second Item")
///     .add_item("item3", "Third Item")
///     .selection_mode(SelectionMode::Single)
///     .on_select(|id| println!("Selected: {}", id));
/// ```
pub struct List {
    pub node_id: Option<NodeId>,
    pub items: Vec<ListItem>,
    pub selected_items: Signal<Vec<String>>,
    pub selection_mode: SelectionMode,
    pub item_height: f32,
    pub padding: f32,
    pub spacing: f32,
    pub background_color: (u8, u8, u8, u8),
    pub item_color: (u8, u8, u8, u8),
    pub selected_color: (u8, u8, u8, u8),
    pub hover_color: (u8, u8, u8, u8),
    pub text_color: (u8, u8, u8, u8),
    pub selected_text_color: (u8, u8, u8, u8),
    pub border_color: (u8, u8, u8, u8),
    pub divider_color: (u8, u8, u8, u8),
    pub show_dividers: bool,
    pub on_select: Option<Box<dyn Fn(&str)>>,
    pub on_deselect: Option<Box<dyn Fn(&str)>>,
}

impl List {
    /// Create a new List component
    pub fn new() -> Self {
        Self {
            node_id: None,
            items: Vec::new(),
            selected_items: Signal::new(Vec::new()),
            selection_mode: SelectionMode::Single,
            item_height: 48.0,
            padding: 16.0,
            spacing: 0.0,
            background_color: (255, 255, 255, 255),
            item_color: (255, 255, 255, 255),
            selected_color: (59, 130, 246, 20), // Light blue
            hover_color: (240, 240, 240, 255),
            text_color: (0, 0, 0, 255),
            selected_text_color: (59, 130, 246, 255), // Blue
            border_color: (220, 220, 220, 255),
            divider_color: (240, 240, 240, 255),
            show_dividers: true,
            on_select: None,
            on_deselect: None,
        }
    }

    /// Set the selection mode
    pub fn selection_mode(mut self, mode: SelectionMode) -> Self {
        self.selection_mode = mode;
        self
    }

    /// Set the item height
    pub fn item_height(mut self, height: f32) -> Self {
        self.item_height = height;
        self
    }

    /// Set the padding
    pub fn padding(mut self, padding: f32) -> Self {
        self.padding = padding;
        self
    }

    /// Set the spacing between items
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }

    /// Set the background color
    pub fn background_color(mut self, r: u8, g: u8, b: u8, a: u8) -> Self {
        self.background_color = (r, g, b, a);
        self
    }

    /// Set the selected item color
    pub fn selected_color(mut self, r: u8, g: u8, b: u8, a: u8) -> Self {
        self.selected_color = (r, g, b, a);
        self
    }

    /// Show or hide dividers
    pub fn show_dividers(mut self, show: bool) -> Self {
        self.show_dividers = show;
        self
    }

    /// Add a list item
    pub fn add_item(mut self, id: impl Into<String>, label: impl Into<String>) -> Self {
        self.items.push(ListItem::new(id, label));
        self
    }

    /// Add a disabled item
    pub fn add_disabled_item(mut self, id: impl Into<String>, label: impl Into<String>) -> Self {
        self.items.push(ListItem::disabled(id, label));
        self
    }

    /// Add a list item object
    pub fn add_item_object(mut self, item: ListItem) -> Self {
        self.items.push(item);
        self
    }

    /// Set all items at once
    pub fn items(mut self, items: Vec<ListItem>) -> Self {
        self.items = items;
        self
    }

    /// Set the select callback
    pub fn on_select<F>(mut self, callback: F) -> Self
    where
        F: Fn(&str) + 'static,
    {
        self.on_select = Some(Box::new(callback));
        self
    }

    /// Set the deselect callback
    pub fn on_deselect<F>(mut self, callback: F) -> Self
    where
        F: Fn(&str) + 'static,
    {
        self.on_deselect = Some(Box::new(callback));
        self
    }

    /// Select an item by ID
    pub fn select_item(&mut self, id: &str) {
        if let Some(item) = self.items.iter().find(|i| i.id == id) {
            if item.disabled {
                return;
            }

            match self.selection_mode {
                SelectionMode::None => return,
                SelectionMode::Single => {
                    self.selected_items.set(vec![id.to_string()]);
                    if let Some(ref callback) = self.on_select {
                        callback(id);
                    }
                }
                SelectionMode::Multiple => {
                    let mut selected = self.selected_items.get();
                    if !selected.contains(&id.to_string()) {
                        selected.push(id.to_string());
                        self.selected_items.set(selected);
                        if let Some(ref callback) = self.on_select {
                            callback(id);
                        }
                    }
                }
            }
        }
    }

    /// Deselect an item by ID
    pub fn deselect_item(&mut self, id: &str) {
        let mut selected = self.selected_items.get();
        if let Some(pos) = selected.iter().position(|i| i == id) {
            selected.remove(pos);
            self.selected_items.set(selected);
            if let Some(ref callback) = self.on_deselect {
                callback(id);
            }
        }
    }

    /// Toggle item selection
    pub fn toggle_item(&mut self, id: &str) {
        if self.is_selected(id) {
            self.deselect_item(id);
        } else {
            self.select_item(id);
        }
    }

    /// Clear all selections
    pub fn clear_selection(&mut self) {
        self.selected_items.set(Vec::new());
    }

    /// Check if an item is selected
    pub fn is_selected(&self, id: &str) -> bool {
        self.selected_items.get().contains(&id.to_string())
    }

    /// Get selected items
    pub fn get_selected(&self) -> Vec<String> {
        self.selected_items.get()
    }

    /// Get selected count
    pub fn selected_count(&self) -> usize {
        self.selected_items.get().len()
    }

    /// Check if has selection
    pub fn has_selection(&self) -> bool {
        !self.selected_items.get().is_empty()
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
    pub fn get_item(&self, index: usize) -> Option<&ListItem> {
        self.items.get(index)
    }

    /// Remove item by ID
    pub fn remove_item(&mut self, id: &str) {
        if let Some(index) = self.find_item(id) {
            self.items.remove(index);
            self.deselect_item(id);
        }
    }

    /// Build the list layout
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
            .map_err(|e| format!("Failed to create list node: {:?}", e))?;
        self.node_id = Some(node);

        Ok(node)
    }
}

impl Default for List {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_starts_empty() {
        let list = List::new();
        assert_eq!(list.item_count(), 0);
        assert!(!list.has_items());
        assert!(!list.has_selection());
    }

    #[test]
    fn list_add_items() {
        let list = List::new()
            .add_item("item1", "First")
            .add_item("item2", "Second")
            .add_item("item3", "Third");

        assert_eq!(list.item_count(), 3);
        assert!(list.has_items());
    }

    #[test]
    fn list_single_selection() {
        let mut list = List::new()
            .selection_mode(SelectionMode::Single)
            .add_item("item1", "First")
            .add_item("item2", "Second");

        list.select_item("item1");
        assert!(list.is_selected("item1"));
        assert_eq!(list.selected_count(), 1);

        list.select_item("item2");
        assert!(!list.is_selected("item1"));
        assert!(list.is_selected("item2"));
        assert_eq!(list.selected_count(), 1);
    }

    #[test]
    fn list_multiple_selection() {
        let mut list = List::new()
            .selection_mode(SelectionMode::Multiple)
            .add_item("item1", "First")
            .add_item("item2", "Second")
            .add_item("item3", "Third");

        list.select_item("item1");
        list.select_item("item2");
        
        assert!(list.is_selected("item1"));
        assert!(list.is_selected("item2"));
        assert_eq!(list.selected_count(), 2);
    }

    #[test]
    fn list_no_selection_mode() {
        let mut list = List::new()
            .selection_mode(SelectionMode::None)
            .add_item("item1", "First");

        list.select_item("item1");
        assert!(!list.is_selected("item1"));
        assert_eq!(list.selected_count(), 0);
    }

    #[test]
    fn list_deselect_item() {
        let mut list = List::new()
            .selection_mode(SelectionMode::Multiple)
            .add_item("item1", "First")
            .add_item("item2", "Second");

        list.select_item("item1");
        list.select_item("item2");
        assert_eq!(list.selected_count(), 2);

        list.deselect_item("item1");
        assert!(!list.is_selected("item1"));
        assert!(list.is_selected("item2"));
        assert_eq!(list.selected_count(), 1);
    }

    #[test]
    fn list_toggle_item() {
        let mut list = List::new()
            .selection_mode(SelectionMode::Multiple)
            .add_item("item1", "First");

        list.toggle_item("item1");
        assert!(list.is_selected("item1"));

        list.toggle_item("item1");
        assert!(!list.is_selected("item1"));
    }

    #[test]
    fn list_clear_selection() {
        let mut list = List::new()
            .selection_mode(SelectionMode::Multiple)
            .add_item("item1", "First")
            .add_item("item2", "Second");

        list.select_item("item1");
        list.select_item("item2");
        assert_eq!(list.selected_count(), 2);

        list.clear_selection();
        assert_eq!(list.selected_count(), 0);
        assert!(!list.has_selection());
    }

    #[test]
    fn list_cannot_select_disabled() {
        let mut list = List::new()
            .selection_mode(SelectionMode::Single)
            .add_item("item1", "First")
            .add_disabled_item("item2", "Disabled");

        list.select_item("item2");
        assert!(!list.is_selected("item2"));
    }

    #[test]
    fn list_find_item() {
        let list = List::new()
            .add_item("item1", "First")
            .add_item("item2", "Second");

        assert_eq!(list.find_item("item2"), Some(1));
        assert_eq!(list.find_item("nonexistent"), None);
    }

    #[test]
    fn list_get_item() {
        let list = List::new()
            .add_item("item1", "First")
            .add_item("item2", "Second");

        let item = list.get_item(0);
        assert!(item.is_some());
        assert_eq!(item.unwrap().label, "First");
    }

    #[test]
    fn list_remove_item() {
        let mut list = List::new()
            .selection_mode(SelectionMode::Single)
            .add_item("item1", "First")
            .add_item("item2", "Second");

        list.select_item("item1");
        assert_eq!(list.item_count(), 2);

        list.remove_item("item1");
        assert_eq!(list.item_count(), 1);
        assert!(!list.is_selected("item1"));
    }

    #[test]
    fn list_item_with_icon_and_badge() {
        let item = ListItem::new("item1", "Messages")
            .with_icon("mail")
            .with_badge("5")
            .with_metadata("Unread");

        assert_eq!(item.icon, Some("mail".to_string()));
        assert_eq!(item.badge, Some("5".to_string()));
        assert_eq!(item.metadata, Some("Unread".to_string()));
    }

    #[test]
    fn list_callbacks() {
        use std::sync::{Arc, Mutex};

        let selected = Arc::new(Mutex::new(String::new()));
        let selected_clone = selected.clone();

        let deselected = Arc::new(Mutex::new(String::new()));
        let deselected_clone = deselected.clone();

        let mut list = List::new()
            .selection_mode(SelectionMode::Multiple)
            .add_item("item1", "First")
            .add_item("item2", "Second")
            .on_select(move |id| {
                *selected_clone.lock().unwrap() = id.to_string();
            })
            .on_deselect(move |id| {
                *deselected_clone.lock().unwrap() = id.to_string();
            });

        list.select_item("item1");
        assert_eq!(*selected.lock().unwrap(), "item1");

        list.deselect_item("item1");
        assert_eq!(*deselected.lock().unwrap(), "item1");
    }

    #[test]
    fn list_builder_pattern() {
        let list = List::new()
            .selection_mode(SelectionMode::Multiple)
            .item_height(60.0)
            .padding(20.0)
            .spacing(8.0)
            .background_color(50, 50, 50, 255)
            .selected_color(255, 0, 0, 50)
            .show_dividers(false);

        assert_eq!(list.selection_mode, SelectionMode::Multiple);
        assert_eq!(list.item_height, 60.0);
        assert_eq!(list.padding, 20.0);
        assert_eq!(list.spacing, 8.0);
        assert_eq!(list.background_color, (50, 50, 50, 255));
        assert_eq!(list.selected_color, (255, 0, 0, 50));
        assert!(!list.show_dividers);
    }

    #[test]
    fn list_build_creates_node() {
        let mut engine = LayoutEngine::new();
        let mut list = List::new().add_item("item1", "First");

        let result = list.build(&mut engine);
        assert!(result.is_ok());
        assert!(list.node_id.is_some());
    }
}
