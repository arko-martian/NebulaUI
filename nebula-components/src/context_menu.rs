// ContextMenu Component - Right-click menu with actions
// Shows a menu of options at the cursor position

use nebula_core::layout::{LayoutEngine, NodeId};
use nebula_core::signal::Signal;

/// Context menu item
#[derive(Debug, Clone, PartialEq)]
pub struct ContextMenuItem {
    pub label: String,
    pub action: String,
    pub disabled: bool,
    pub is_separator: bool,
    pub shortcut: Option<String>,
    pub icon: Option<String>,
}

impl ContextMenuItem {
    /// Create a new menu item
    pub fn new(label: impl Into<String>, action: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            action: action.into(),
            disabled: false,
            is_separator: false,
            shortcut: None,
            icon: None,
        }
    }

    /// Create a separator
    pub fn separator() -> Self {
        Self {
            label: String::new(),
            action: String::new(),
            disabled: false,
            is_separator: true,
            shortcut: None,
            icon: None,
        }
    }

    /// Create a disabled item
    pub fn disabled(label: impl Into<String>, action: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            action: action.into(),
            disabled: true,
            is_separator: false,
            shortcut: None,
            icon: None,
        }
    }

    /// Add a keyboard shortcut
    pub fn with_shortcut(mut self, shortcut: impl Into<String>) -> Self {
        self.shortcut = Some(shortcut.into());
        self
    }

    /// Add an icon
    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }
}

/// ContextMenu component - displays a menu on right-click
/// 
/// # Example
/// ```
/// let mut menu = ContextMenu::new()
///     .add_item("Copy", "copy")
///     .add_item("Paste", "paste")
///     .add_separator()
///     .add_item("Delete", "delete")
///     .on_select(|action| println!("Action: {}", action));
/// ```
pub struct ContextMenu {
    pub node_id: Option<NodeId>,
    pub items: Vec<ContextMenuItem>,
    pub is_visible: Signal<bool>,
    pub position_x: f32,
    pub position_y: f32,
    pub width: f32,
    pub max_height: f32,
    pub padding: f32,
    pub background_color: (u8, u8, u8, u8),
    pub text_color: (u8, u8, u8, u8),
    pub hover_color: (u8, u8, u8, u8),
    pub disabled_color: (u8, u8, u8, u8),
    pub border_radius: f32,
    pub on_select: Option<Box<dyn Fn(&str)>>,
    pub on_open: Option<Box<dyn Fn()>>,
    pub on_close: Option<Box<dyn Fn()>>,
}

impl ContextMenu {
    /// Create a new ContextMenu component
    pub fn new() -> Self {
        Self {
            node_id: None,
            items: Vec::new(),
            is_visible: Signal::new(false),
            position_x: 0.0,
            position_y: 0.0,
            width: 200.0,
            max_height: 400.0,
            padding: 4.0,
            background_color: (255, 255, 255, 255),
            text_color: (0, 0, 0, 255),
            hover_color: (240, 240, 240, 255),
            disabled_color: (150, 150, 150, 255),
            border_radius: 8.0,
            on_select: None,
            on_open: None,
            on_close: None,
        }
    }

    /// Set the width
    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
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

    /// Set the hover color
    pub fn hover_color(mut self, r: u8, g: u8, b: u8, a: u8) -> Self {
        self.hover_color = (r, g, b, a);
        self
    }

    /// Set the border radius
    pub fn border_radius(mut self, radius: f32) -> Self {
        self.border_radius = radius;
        self
    }

    /// Add a menu item
    pub fn add_item(mut self, label: impl Into<String>, action: impl Into<String>) -> Self {
        self.items.push(ContextMenuItem::new(label, action));
        self
    }

    /// Add a disabled item
    pub fn add_disabled_item(mut self, label: impl Into<String>, action: impl Into<String>) -> Self {
        self.items.push(ContextMenuItem::disabled(label, action));
        self
    }

    /// Add a separator
    pub fn add_separator(mut self) -> Self {
        self.items.push(ContextMenuItem::separator());
        self
    }

    /// Add an item with shortcut
    pub fn add_item_with_shortcut(
        mut self,
        label: impl Into<String>,
        action: impl Into<String>,
        shortcut: impl Into<String>,
    ) -> Self {
        self.items.push(
            ContextMenuItem::new(label, action).with_shortcut(shortcut)
        );
        self
    }

    /// Set all items at once
    pub fn items(mut self, items: Vec<ContextMenuItem>) -> Self {
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

    /// Show the menu at position
    pub fn show_at(&mut self, x: f32, y: f32) {
        self.position_x = x;
        self.position_y = y;
        self.is_visible.set(true);
        if let Some(ref callback) = self.on_open {
            callback();
        }
    }

    /// Hide the menu
    pub fn hide(&mut self) {
        self.is_visible.set(false);
        if let Some(ref callback) = self.on_close {
            callback();
        }
    }

    /// Check if the menu is visible
    pub fn is_visible(&self) -> bool {
        self.is_visible.get()
    }

    /// Select an item by index
    pub fn select(&mut self, index: usize) {
        if index < self.items.len() {
            let item = &self.items[index];
            if !item.disabled && !item.is_separator {
                if let Some(ref callback) = self.on_select {
                    callback(&item.action);
                }
                self.hide();
            }
        }
    }

    /// Select an item by action
    pub fn select_by_action(&mut self, action: &str) {
        if let Some(index) = self.items.iter().position(|item| item.action == action) {
            self.select(index);
        }
    }

    /// Get item count
    pub fn item_count(&self) -> usize {
        self.items.len()
    }

    /// Get non-separator item count
    pub fn action_item_count(&self) -> usize {
        self.items.iter().filter(|item| !item.is_separator).count()
    }

    /// Check if has items
    pub fn has_items(&self) -> bool {
        !self.items.is_empty()
    }

    /// Get position
    pub fn get_position(&self) -> (f32, f32) {
        (self.position_x, self.position_y)
    }

    /// Build the context menu layout
    pub fn build(&mut self, engine: &mut LayoutEngine) -> Result<NodeId, String> {
        if !self.is_visible() {
            let style = taffy::style::Style {
                display: taffy::style::Display::None,
                ..Default::default()
            };
            let node = engine
                .new_leaf(style)
                .map_err(|e| format!("Failed to create hidden context menu node: {:?}", e))?;
            self.node_id = Some(node);
            return Ok(node);
        }

        let style = taffy::style::Style {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Length(self.width),
                height: taffy::style::Dimension::Auto,
            },
            max_size: taffy::geometry::Size {
                width: taffy::style::Dimension::Length(self.width),
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
            .map_err(|e| format!("Failed to create context menu node: {:?}", e))?;
        self.node_id = Some(node);

        Ok(node)
    }
}

impl Default for ContextMenu {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn context_menu_starts_hidden() {
        let menu = ContextMenu::new();
        assert!(!menu.is_visible());
    }

    #[test]
    fn context_menu_can_be_shown() {
        let mut menu = ContextMenu::new();
        menu.show_at(100.0, 200.0);
        assert!(menu.is_visible());
        assert_eq!(menu.get_position(), (100.0, 200.0));
    }

    #[test]
    fn context_menu_can_be_hidden() {
        let mut menu = ContextMenu::new();
        menu.show_at(0.0, 0.0);
        menu.hide();
        assert!(!menu.is_visible());
    }

    #[test]
    fn context_menu_add_items() {
        let menu = ContextMenu::new()
            .add_item("Copy", "copy")
            .add_item("Paste", "paste")
            .add_separator()
            .add_item("Delete", "delete");

        assert_eq!(menu.item_count(), 4);
        assert_eq!(menu.action_item_count(), 3);
    }

    #[test]
    fn context_menu_separator() {
        let separator = ContextMenuItem::separator();
        assert!(separator.is_separator);
        assert!(separator.label.is_empty());
    }

    #[test]
    fn context_menu_disabled_item() {
        let item = ContextMenuItem::disabled("Disabled", "disabled");
        assert!(item.disabled);
    }

    #[test]
    fn context_menu_item_with_shortcut() {
        let item = ContextMenuItem::new("Copy", "copy").with_shortcut("Ctrl+C");
        assert_eq!(item.shortcut, Some("Ctrl+C".to_string()));
    }

    #[test]
    fn context_menu_select_by_index() {
        use std::sync::{Arc, Mutex};

        let selected = Arc::new(Mutex::new(String::new()));
        let selected_clone = selected.clone();

        let mut menu = ContextMenu::new()
            .add_item("Copy", "copy")
            .add_item("Paste", "paste")
            .on_select(move |action| {
                *selected_clone.lock().unwrap() = action.to_string();
            });

        menu.show_at(0.0, 0.0);
        menu.select(1);

        assert_eq!(*selected.lock().unwrap(), "paste");
        assert!(!menu.is_visible()); // Should hide after selection
    }

    #[test]
    fn context_menu_select_by_action() {
        use std::sync::{Arc, Mutex};

        let selected = Arc::new(Mutex::new(String::new()));
        let selected_clone = selected.clone();

        let mut menu = ContextMenu::new()
            .add_item("Copy", "copy")
            .add_item("Paste", "paste")
            .on_select(move |action| {
                *selected_clone.lock().unwrap() = action.to_string();
            });

        menu.show_at(0.0, 0.0);
        menu.select_by_action("copy");

        assert_eq!(*selected.lock().unwrap(), "copy");
    }

    #[test]
    fn context_menu_cannot_select_disabled() {
        use std::sync::{Arc, Mutex};

        let selected = Arc::new(Mutex::new(false));
        let selected_clone = selected.clone();

        let mut menu = ContextMenu::new()
            .add_disabled_item("Disabled", "disabled")
            .on_select(move |_| {
                *selected_clone.lock().unwrap() = true;
            });

        menu.show_at(0.0, 0.0);
        menu.select(0);

        assert!(!*selected.lock().unwrap());
    }

    #[test]
    fn context_menu_cannot_select_separator() {
        use std::sync::{Arc, Mutex};

        let selected = Arc::new(Mutex::new(false));
        let selected_clone = selected.clone();

        let mut menu = ContextMenu::new()
            .add_separator()
            .on_select(move |_| {
                *selected_clone.lock().unwrap() = true;
            });

        menu.show_at(0.0, 0.0);
        menu.select(0);

        assert!(!*selected.lock().unwrap());
    }

    #[test]
    fn context_menu_builder_pattern() {
        let menu = ContextMenu::new()
            .width(250.0)
            .max_height(500.0)
            .padding(8.0)
            .background_color(50, 50, 50, 255)
            .text_color(255, 255, 255, 255)
            .border_radius(12.0);

        assert_eq!(menu.width, 250.0);
        assert_eq!(menu.max_height, 500.0);
        assert_eq!(menu.padding, 8.0);
        assert_eq!(menu.background_color, (50, 50, 50, 255));
        assert_eq!(menu.text_color, (255, 255, 255, 255));
        assert_eq!(menu.border_radius, 12.0);
    }

    #[test]
    fn context_menu_callbacks() {
        use std::sync::{Arc, Mutex};

        let opened = Arc::new(Mutex::new(false));
        let opened_clone = opened.clone();

        let closed = Arc::new(Mutex::new(false));
        let closed_clone = closed.clone();

        let mut menu = ContextMenu::new()
            .on_open(move || {
                *opened_clone.lock().unwrap() = true;
            })
            .on_close(move || {
                *closed_clone.lock().unwrap() = true;
            });

        menu.show_at(0.0, 0.0);
        assert!(*opened.lock().unwrap());

        menu.hide();
        assert!(*closed.lock().unwrap());
    }

    #[test]
    fn context_menu_build_creates_node() {
        let mut engine = LayoutEngine::new();
        let mut menu = ContextMenu::new().add_item("Test", "test");

        menu.show_at(0.0, 0.0);
        let result = menu.build(&mut engine);
        assert!(result.is_ok());
        assert!(menu.node_id.is_some());
    }
}
