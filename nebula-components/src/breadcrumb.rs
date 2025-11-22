// Breadcrumb Component - Breadcrumb navigation for hierarchical navigation
// Essential for showing current location in navigation hierarchy

use nebula_core::layout::{LayoutEngine, NodeId};

/// Breadcrumb item
#[derive(Debug, Clone, PartialEq)]
pub struct BreadcrumbItem {
    pub label: String,
    pub id: String,
    pub href: Option<String>,
    pub disabled: bool,
    pub icon: Option<String>,
}

impl BreadcrumbItem {
    /// Create a new breadcrumb item
    pub fn new(label: impl Into<String>, id: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            id: id.into(),
            href: None,
            disabled: false,
            icon: None,
        }
    }

    /// Create a disabled breadcrumb item
    pub fn disabled(label: impl Into<String>, id: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            id: id.into(),
            href: None,
            disabled: true,
            icon: None,
        }
    }

    /// Add a link href
    pub fn with_href(mut self, href: impl Into<String>) -> Self {
        self.href = Some(href.into());
        self
    }

    /// Add an icon
    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }
}

/// Breadcrumb component - breadcrumb navigation for hierarchical navigation
/// 
/// # Example
/// ```
/// let mut breadcrumb = Breadcrumb::new()
///     .add_item("Home", "home")
///     .add_item("Products", "products")
///     .add_item("Electronics", "electronics")
///     .separator(">")
///     .on_navigate(|id| println!("Navigate to: {}", id));
/// ```
pub struct Breadcrumb {
    pub node_id: Option<NodeId>,
    pub items: Vec<BreadcrumbItem>,
    pub separator: String,
    pub show_home_icon: bool,
    pub max_items: Option<usize>,
    pub height: f32,
    pub padding: f32,
    pub spacing: f32,
    pub text_color: (u8, u8, u8, u8),
    pub active_color: (u8, u8, u8, u8),
    pub separator_color: (u8, u8, u8, u8),
    pub hover_color: (u8, u8, u8, u8),
    pub background_color: (u8, u8, u8, u8),
    pub on_navigate: Option<Box<dyn Fn(&str)>>,
}

impl Breadcrumb {
    /// Create a new Breadcrumb component
    pub fn new() -> Self {
        Self {
            node_id: None,
            items: Vec::new(),
            separator: "/".to_string(),
            show_home_icon: false,
            max_items: None,
            height: 40.0,
            padding: 8.0,
            spacing: 8.0,
            text_color: (100, 100, 100, 255),
            active_color: (0, 0, 0, 255),
            separator_color: (150, 150, 150, 255),
            hover_color: (59, 130, 246, 255), // Blue
            background_color: (255, 255, 255, 0), // Transparent
            on_navigate: None,
        }
    }

    /// Set the separator
    pub fn separator(mut self, separator: impl Into<String>) -> Self {
        self.separator = separator.into();
        self
    }

    /// Show home icon for first item
    pub fn show_home_icon(mut self, show: bool) -> Self {
        self.show_home_icon = show;
        self
    }

    /// Set maximum number of items to show (will collapse middle items)
    pub fn max_items(mut self, max: usize) -> Self {
        self.max_items = Some(max);
        self
    }

    /// Set the height
    pub fn height(mut self, height: f32) -> Self {
        self.height = height;
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

    /// Set the text color
    pub fn text_color(mut self, r: u8, g: u8, b: u8, a: u8) -> Self {
        self.text_color = (r, g, b, a);
        self
    }

    /// Set the active (last) item color
    pub fn active_color(mut self, r: u8, g: u8, b: u8, a: u8) -> Self {
        self.active_color = (r, g, b, a);
        self
    }

    /// Set the separator color
    pub fn separator_color(mut self, r: u8, g: u8, b: u8, a: u8) -> Self {
        self.separator_color = (r, g, b, a);
        self
    }

    /// Set the hover color
    pub fn hover_color(mut self, r: u8, g: u8, b: u8, a: u8) -> Self {
        self.hover_color = (r, g, b, a);
        self
    }

    /// Set the background color
    pub fn background_color(mut self, r: u8, g: u8, b: u8, a: u8) -> Self {
        self.background_color = (r, g, b, a);
        self
    }

    /// Add a breadcrumb item
    pub fn add_item(mut self, label: impl Into<String>, id: impl Into<String>) -> Self {
        self.items.push(BreadcrumbItem::new(label, id));
        self
    }

    /// Add a disabled breadcrumb item
    pub fn add_disabled_item(mut self, label: impl Into<String>, id: impl Into<String>) -> Self {
        self.items.push(BreadcrumbItem::disabled(label, id));
        self
    }

    /// Add a breadcrumb item object
    pub fn add_item_object(mut self, item: BreadcrumbItem) -> Self {
        self.items.push(item);
        self
    }

    /// Set all items at once
    pub fn items(mut self, items: Vec<BreadcrumbItem>) -> Self {
        self.items = items;
        self
    }

    /// Set the navigate callback
    pub fn on_navigate<F>(mut self, callback: F) -> Self
    where
        F: Fn(&str) + 'static,
    {
        self.on_navigate = Some(Box::new(callback));
        self
    }

    /// Navigate to an item by index
    pub fn navigate_to(&mut self, index: usize) {
        if index < self.items.len() && !self.items[index].disabled {
            if let Some(ref callback) = self.on_navigate {
                callback(&self.items[index].id);
            }
        }
    }

    /// Navigate to an item by ID
    pub fn navigate_to_id(&mut self, id: &str) {
        if let Some(index) = self.items.iter().position(|item| item.id == id) {
            self.navigate_to(index);
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
    pub fn get_item(&self, index: usize) -> Option<&BreadcrumbItem> {
        self.items.get(index)
    }

    /// Get the last (current) item
    pub fn get_current_item(&self) -> Option<&BreadcrumbItem> {
        self.items.last()
    }

    /// Check if item is the last (current) item
    pub fn is_current_item(&self, index: usize) -> bool {
        !self.items.is_empty() && index == self.items.len() - 1
    }

    /// Get visible items (respecting max_items)
    pub fn get_visible_items(&self) -> Vec<&BreadcrumbItem> {
        if let Some(max) = self.max_items {
            if self.items.len() > max && max >= 2 {
                // Show first, ..., and last items
                let mut visible = vec![&self.items[0]];
                let remaining = max - 1; // Reserve space for first item
                let start_idx = self.items.len() - remaining;
                for i in start_idx..self.items.len() {
                    visible.push(&self.items[i]);
                }
                return visible;
            }
        }
        self.items.iter().collect()
    }

    /// Check if items are collapsed
    pub fn is_collapsed(&self) -> bool {
        if let Some(max) = self.max_items {
            self.items.len() > max
        } else {
            false
        }
    }

    /// Build the breadcrumb layout
    pub fn build(&mut self, engine: &mut LayoutEngine) -> Result<NodeId, String> {
        let style = taffy::style::Style {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Auto,
                height: taffy::style::Dimension::Length(self.height),
            },
            padding: taffy::geometry::Rect {
                left: taffy::style::LengthPercentage::Length(self.padding),
                right: taffy::style::LengthPercentage::Length(self.padding),
                top: taffy::style::LengthPercentage::Length(self.padding),
                bottom: taffy::style::LengthPercentage::Length(self.padding),
            },
            display: taffy::style::Display::Flex,
            flex_direction: taffy::style::FlexDirection::Row,
            align_items: Some(taffy::style::AlignItems::Center),
            gap: taffy::geometry::Size {
                width: taffy::style::LengthPercentage::Length(self.spacing),
                height: taffy::style::LengthPercentage::Length(0.0),
            },
            ..Default::default()
        };

        let node = engine
            .new_leaf(style)
            .map_err(|e| format!("Failed to create breadcrumb node: {:?}", e))?;
        self.node_id = Some(node);

        Ok(node)
    }
}

impl Default for Breadcrumb {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn breadcrumb_starts_empty() {
        let breadcrumb = Breadcrumb::new();
        assert_eq!(breadcrumb.item_count(), 0);
        assert!(!breadcrumb.has_items());
        assert_eq!(breadcrumb.separator, "/");
    }

    #[test]
    fn breadcrumb_add_items() {
        let breadcrumb = Breadcrumb::new()
            .add_item("Home", "home")
            .add_item("Products", "products")
            .add_item("Electronics", "electronics");

        assert_eq!(breadcrumb.item_count(), 3);
        assert!(breadcrumb.has_items());
    }

    #[test]
    fn breadcrumb_custom_separator() {
        let breadcrumb = Breadcrumb::new().separator(">");
        assert_eq!(breadcrumb.separator, ">");
    }

    #[test]
    fn breadcrumb_navigate_to() {
        use std::sync::{Arc, Mutex};

        let navigated = Arc::new(Mutex::new(String::new()));
        let navigated_clone = navigated.clone();

        let mut breadcrumb = Breadcrumb::new()
            .add_item("Home", "home")
            .add_item("Products", "products")
            .on_navigate(move |id| {
                *navigated_clone.lock().unwrap() = id.to_string();
            });

        breadcrumb.navigate_to(0);
        assert_eq!(*navigated.lock().unwrap(), "home");
    }

    #[test]
    fn breadcrumb_navigate_to_id() {
        use std::sync::{Arc, Mutex};

        let navigated = Arc::new(Mutex::new(String::new()));
        let navigated_clone = navigated.clone();

        let mut breadcrumb = Breadcrumb::new()
            .add_item("Home", "home")
            .add_item("Products", "products")
            .on_navigate(move |id| {
                *navigated_clone.lock().unwrap() = id.to_string();
            });

        breadcrumb.navigate_to_id("products");
        assert_eq!(*navigated.lock().unwrap(), "products");
    }

    #[test]
    fn breadcrumb_cannot_navigate_to_disabled() {
        use std::sync::{Arc, Mutex};

        let navigated = Arc::new(Mutex::new(false));
        let navigated_clone = navigated.clone();

        let mut breadcrumb = Breadcrumb::new()
            .add_disabled_item("Disabled", "disabled")
            .on_navigate(move |_| {
                *navigated_clone.lock().unwrap() = true;
            });

        breadcrumb.navigate_to(0);
        assert!(!*navigated.lock().unwrap());
    }

    #[test]
    fn breadcrumb_find_item() {
        let breadcrumb = Breadcrumb::new()
            .add_item("Home", "home")
            .add_item("Products", "products");

        assert_eq!(breadcrumb.find_item("products"), Some(1));
        assert_eq!(breadcrumb.find_item("nonexistent"), None);
    }

    #[test]
    fn breadcrumb_get_item() {
        let breadcrumb = Breadcrumb::new()
            .add_item("Home", "home")
            .add_item("Products", "products");

        let item = breadcrumb.get_item(0);
        assert!(item.is_some());
        assert_eq!(item.unwrap().label, "Home");
    }

    #[test]
    fn breadcrumb_get_current_item() {
        let breadcrumb = Breadcrumb::new()
            .add_item("Home", "home")
            .add_item("Products", "products")
            .add_item("Electronics", "electronics");

        let current = breadcrumb.get_current_item();
        assert!(current.is_some());
        assert_eq!(current.unwrap().label, "Electronics");
    }

    #[test]
    fn breadcrumb_is_current_item() {
        let breadcrumb = Breadcrumb::new()
            .add_item("Home", "home")
            .add_item("Products", "products")
            .add_item("Electronics", "electronics");

        assert!(!breadcrumb.is_current_item(0));
        assert!(!breadcrumb.is_current_item(1));
        assert!(breadcrumb.is_current_item(2));
    }

    #[test]
    fn breadcrumb_item_with_href() {
        let item = BreadcrumbItem::new("Home", "home").with_href("/home");
        assert_eq!(item.href, Some("/home".to_string()));
    }

    #[test]
    fn breadcrumb_item_with_icon() {
        let item = BreadcrumbItem::new("Home", "home").with_icon("home");
        assert_eq!(item.icon, Some("home".to_string()));
    }

    #[test]
    fn breadcrumb_max_items() {
        let breadcrumb = Breadcrumb::new()
            .add_item("Home", "home")
            .add_item("Products", "products")
            .add_item("Electronics", "electronics")
            .add_item("Phones", "phones")
            .add_item("iPhone", "iphone")
            .max_items(3);

        assert!(breadcrumb.is_collapsed());
        let visible = breadcrumb.get_visible_items();
        assert_eq!(visible.len(), 3);
        assert_eq!(visible[0].label, "Home");
        assert_eq!(visible[1].label, "Phones");
        assert_eq!(visible[2].label, "iPhone");
    }

    #[test]
    fn breadcrumb_not_collapsed() {
        let breadcrumb = Breadcrumb::new()
            .add_item("Home", "home")
            .add_item("Products", "products")
            .max_items(5);

        assert!(!breadcrumb.is_collapsed());
        let visible = breadcrumb.get_visible_items();
        assert_eq!(visible.len(), 2);
    }

    #[test]
    fn breadcrumb_builder_pattern() {
        let breadcrumb = Breadcrumb::new()
            .separator("→")
            .show_home_icon(true)
            .max_items(5)
            .height(50.0)
            .padding(12.0)
            .spacing(16.0)
            .text_color(100, 100, 100, 255)
            .active_color(0, 0, 0, 255)
            .separator_color(150, 150, 150, 255)
            .hover_color(59, 130, 246, 255)
            .background_color(255, 255, 255, 255);

        assert_eq!(breadcrumb.separator, "→");
        assert!(breadcrumb.show_home_icon);
        assert_eq!(breadcrumb.max_items, Some(5));
        assert_eq!(breadcrumb.height, 50.0);
        assert_eq!(breadcrumb.padding, 12.0);
        assert_eq!(breadcrumb.spacing, 16.0);
    }

    #[test]
    fn breadcrumb_build_creates_node() {
        let mut engine = LayoutEngine::new();
        let mut breadcrumb = Breadcrumb::new()
            .add_item("Home", "home")
            .add_item("Products", "products");

        let result = breadcrumb.build(&mut engine);
        assert!(result.is_ok());
        assert!(breadcrumb.node_id.is_some());
    }

    #[test]
    fn breadcrumb_item_disabled() {
        let item = BreadcrumbItem::disabled("Disabled", "disabled");
        assert!(item.disabled);
    }
}
