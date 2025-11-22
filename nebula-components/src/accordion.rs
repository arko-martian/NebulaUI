// Accordion Component - Expandable accordion for collapsible content
// Essential for FAQs and collapsible sections

use nebula_core::layout::{LayoutEngine, NodeId};
use nebula_core::signal::Signal;

/// Accordion item
#[derive(Debug, Clone)]
pub struct AccordionItem {
    pub id: String,
    pub title: String,
    pub content: String,
    pub expanded: Signal<bool>,
    pub disabled: bool,
}

impl AccordionItem {
    /// Create a new accordion item
    pub fn new(id: impl Into<String>, title: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            content: content.into(),
            expanded: Signal::new(false),
            disabled: false,
        }
    }

    /// Create a disabled item
    pub fn disabled(id: impl Into<String>, title: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            content: content.into(),
            expanded: Signal::new(false),
            disabled: true,
        }
    }

    /// Check if expanded
    pub fn is_expanded(&self) -> bool {
        self.expanded.get()
    }
}

/// Accordion component - expandable accordion for collapsible content
/// 
/// # Example
/// ```
/// let accordion = Accordion::new()
///     .add_item("1", "Question 1", "Answer 1")
///     .add_item("2", "Question 2", "Answer 2")
///     .allow_multiple(false)
///     .on_change(|id, expanded| println!("{}: {}", id, expanded));
/// ```
pub struct Accordion {
    pub node_id: Option<NodeId>,
    pub items: Vec<AccordionItem>,
    pub allow_multiple: bool,
    pub width: f32,
    pub item_height: f32,
    pub padding: f32,
    pub background_color: (u8, u8, u8, u8),
    pub header_color: (u8, u8, u8, u8),
    pub border_color: (u8, u8, u8, u8),
    pub border_radius: f32,
    pub on_change: Option<Box<dyn Fn(&str, bool)>>,
}

impl Accordion {
    /// Create a new Accordion component
    pub fn new() -> Self {
        Self {
            node_id: None,
            items: Vec::new(),
            allow_multiple: true,
            width: 400.0,
            item_height: 48.0,
            padding: 16.0,
            background_color: (255, 255, 255, 255),
            header_color: (249, 250, 251, 255),
            border_color: (229, 231, 235, 255),
            border_radius: 8.0,
            on_change: None,
        }
    }

    /// Add an item
    pub fn add_item(mut self, id: impl Into<String>, title: impl Into<String>, content: impl Into<String>) -> Self {
        self.items.push(AccordionItem::new(id, title, content));
        self
    }

    /// Add a disabled item
    pub fn add_disabled_item(mut self, id: impl Into<String>, title: impl Into<String>, content: impl Into<String>) -> Self {
        self.items.push(AccordionItem::disabled(id, title, content));
        self
    }

    /// Add an item object
    pub fn add_item_object(mut self, item: AccordionItem) -> Self {
        self.items.push(item);
        self
    }

    /// Set all items at once
    pub fn items(mut self, items: Vec<AccordionItem>) -> Self {
        self.items = items;
        self
    }

    /// Allow multiple items to be expanded
    pub fn allow_multiple(mut self, allow: bool) -> Self {
        self.allow_multiple = allow;
        self
    }

    /// Set the width
    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    /// Set the change callback
    pub fn on_change<F>(mut self, callback: F) -> Self
    where
        F: Fn(&str, bool) + 'static,
    {
        self.on_change = Some(Box::new(callback));
        self
    }

    /// Expand an item by index
    pub fn expand(&mut self, index: usize) {
        if index < self.items.len() && !self.items[index].disabled {
            // If not allowing multiple, collapse all others
            if !self.allow_multiple {
                for (i, item) in self.items.iter_mut().enumerate() {
                    if i != index {
                        item.expanded.set(false);
                    }
                }
            }
            
            self.items[index].expanded.set(true);
            
            if let Some(ref callback) = self.on_change {
                callback(&self.items[index].id, true);
            }
        }
    }

    /// Collapse an item by index
    pub fn collapse(&mut self, index: usize) {
        if index < self.items.len() && !self.items[index].disabled {
            self.items[index].expanded.set(false);
            
            if let Some(ref callback) = self.on_change {
                callback(&self.items[index].id, false);
            }
        }
    }

    /// Toggle an item by index
    pub fn toggle(&mut self, index: usize) {
        if index < self.items.len() {
            if self.items[index].is_expanded() {
                self.collapse(index);
            } else {
                self.expand(index);
            }
        }
    }

    /// Expand an item by ID
    pub fn expand_by_id(&mut self, id: &str) {
        if let Some(index) = self.find_item(id) {
            self.expand(index);
        }
    }

    /// Collapse an item by ID
    pub fn collapse_by_id(&mut self, id: &str) {
        if let Some(index) = self.find_item(id) {
            self.collapse(index);
        }
    }

    /// Toggle an item by ID
    pub fn toggle_by_id(&mut self, id: &str) {
        if let Some(index) = self.find_item(id) {
            self.toggle(index);
        }
    }

    /// Expand all items
    pub fn expand_all(&mut self) {
        if self.allow_multiple {
            for item in &mut self.items {
                if !item.disabled {
                    item.expanded.set(true);
                }
            }
        }
    }

    /// Collapse all items
    pub fn collapse_all(&mut self) {
        for item in &mut self.items {
            item.expanded.set(false);
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
    pub fn get_item(&self, index: usize) -> Option<&AccordionItem> {
        self.items.get(index)
    }

    /// Check if item is expanded
    pub fn is_item_expanded(&self, index: usize) -> bool {
        self.items.get(index).map(|item| item.is_expanded()).unwrap_or(false)
    }

    /// Build the accordion layout
    pub fn build(&mut self, engine: &mut LayoutEngine) -> Result<NodeId, String> {
        let style = taffy::style::Style {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Length(self.width),
                height: taffy::style::Dimension::Auto,
            },
            display: taffy::style::Display::Flex,
            flex_direction: taffy::style::FlexDirection::Column,
            ..Default::default()
        };

        let node = engine
            .new_leaf(style)
            .map_err(|e| format!("Failed to create accordion node: {:?}", e))?;
        self.node_id = Some(node);

        Ok(node)
    }
}

impl Default for Accordion {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accordion_starts_empty() {
        let accordion = Accordion::new();
        assert_eq!(accordion.item_count(), 0);
        assert!(!accordion.has_items());
    }

    #[test]
    fn accordion_add_items() {
        let accordion = Accordion::new()
            .add_item("1", "Question 1", "Answer 1")
            .add_item("2", "Question 2", "Answer 2");

        assert_eq!(accordion.item_count(), 2);
        assert!(accordion.has_items());
    }

    #[test]
    fn accordion_expand_collapse() {
        let mut accordion = Accordion::new()
            .add_item("1", "Question 1", "Answer 1");

        accordion.expand(0);
        assert!(accordion.is_item_expanded(0));

        accordion.collapse(0);
        assert!(!accordion.is_item_expanded(0));
    }

    #[test]
    fn accordion_toggle() {
        let mut accordion = Accordion::new()
            .add_item("1", "Question 1", "Answer 1");

        accordion.toggle(0);
        assert!(accordion.is_item_expanded(0));

        accordion.toggle(0);
        assert!(!accordion.is_item_expanded(0));
    }

    #[test]
    fn accordion_single_mode() {
        let mut accordion = Accordion::new()
            .add_item("1", "Question 1", "Answer 1")
            .add_item("2", "Question 2", "Answer 2")
            .allow_multiple(false);

        accordion.expand(0);
        assert!(accordion.is_item_expanded(0));

        accordion.expand(1);
        assert!(!accordion.is_item_expanded(0)); // First should collapse
        assert!(accordion.is_item_expanded(1));
    }

    #[test]
    fn accordion_multiple_mode() {
        let mut accordion = Accordion::new()
            .add_item("1", "Question 1", "Answer 1")
            .add_item("2", "Question 2", "Answer 2")
            .allow_multiple(true);

        accordion.expand(0);
        accordion.expand(1);
        assert!(accordion.is_item_expanded(0));
        assert!(accordion.is_item_expanded(1));
    }

    #[test]
    fn accordion_expand_by_id() {
        let mut accordion = Accordion::new()
            .add_item("q1", "Question 1", "Answer 1");

        accordion.expand_by_id("q1");
        assert!(accordion.is_item_expanded(0));
    }

    #[test]
    fn accordion_collapse_by_id() {
        let mut accordion = Accordion::new()
            .add_item("q1", "Question 1", "Answer 1");

        accordion.expand_by_id("q1");
        accordion.collapse_by_id("q1");
        assert!(!accordion.is_item_expanded(0));
    }

    #[test]
    fn accordion_expand_all() {
        let mut accordion = Accordion::new()
            .add_item("1", "Question 1", "Answer 1")
            .add_item("2", "Question 2", "Answer 2")
            .allow_multiple(true);

        accordion.expand_all();
        assert!(accordion.is_item_expanded(0));
        assert!(accordion.is_item_expanded(1));
    }

    #[test]
    fn accordion_collapse_all() {
        let mut accordion = Accordion::new()
            .add_item("1", "Question 1", "Answer 1")
            .add_item("2", "Question 2", "Answer 2")
            .allow_multiple(true);

        accordion.expand_all();
        accordion.collapse_all();
        assert!(!accordion.is_item_expanded(0));
        assert!(!accordion.is_item_expanded(1));
    }

    #[test]
    fn accordion_disabled_item() {
        let mut accordion = Accordion::new()
            .add_disabled_item("1", "Question 1", "Answer 1");

        accordion.expand(0);
        assert!(!accordion.is_item_expanded(0)); // Should not expand
    }

    #[test]
    fn accordion_find_item() {
        let accordion = Accordion::new()
            .add_item("q1", "Question 1", "Answer 1")
            .add_item("q2", "Question 2", "Answer 2");

        assert_eq!(accordion.find_item("q2"), Some(1));
        assert_eq!(accordion.find_item("q3"), None);
    }

    #[test]
    fn accordion_on_change_callback() {
        use std::sync::{Arc, Mutex};

        let changed = Arc::new(Mutex::new((String::new(), false)));
        let changed_clone = changed.clone();

        let mut accordion = Accordion::new()
            .add_item("q1", "Question 1", "Answer 1")
            .on_change(move |id, expanded| {
                *changed_clone.lock().unwrap() = (id.to_string(), expanded);
            });

        accordion.expand(0);
        assert_eq!(*changed.lock().unwrap(), ("q1".to_string(), true));
    }

    #[test]
    fn accordion_builder_pattern() {
        let accordion = Accordion::new()
            .add_item("1", "Q1", "A1")
            .add_item("2", "Q2", "A2")
            .allow_multiple(false)
            .width(500.0);

        assert_eq!(accordion.item_count(), 2);
        assert!(!accordion.allow_multiple);
        assert_eq!(accordion.width, 500.0);
    }

    #[test]
    fn accordion_build_creates_node() {
        let mut engine = LayoutEngine::new();
        let mut accordion = Accordion::new();

        let result = accordion.build(&mut engine);
        assert!(result.is_ok());
        assert!(accordion.node_id.is_some());
    }
}
