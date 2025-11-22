// MenuBar Component - Native application menu bar
// Essential for desktop applications (File, Edit, View, Help, etc.)

use nebula_core::layout::{LayoutEngine, NodeId};
use nebula_core::signal::Signal;

/// Menu item in a menu
#[derive(Debug, Clone, PartialEq)]
pub struct MenuItem {
    pub label: String,
    pub action: String,
    pub disabled: bool,
    pub is_separator: bool,
    pub shortcut: Option<String>,
    pub icon: Option<String>,
    pub submenu: Option<Vec<MenuItem>>,
}

impl MenuItem {
    /// Create a new menu item
    pub fn new(label: impl Into<String>, action: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            action: action.into(),
            disabled: false,
            is_separator: false,
            shortcut: None,
            icon: None,
            submenu: None,
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
            submenu: None,
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
            submenu: None,
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

    /// Add a submenu
    pub fn with_submenu(mut self, submenu: Vec<MenuItem>) -> Self {
        self.submenu = Some(submenu);
        self
    }

    /// Check if has submenu
    pub fn has_submenu(&self) -> bool {
        self.submenu.is_some()
    }
}

/// Menu in the menu bar
#[derive(Debug, Clone, PartialEq)]
pub struct Menu {
    pub label: String,
    pub items: Vec<MenuItem>,
}

impl Menu {
    /// Create a new menu
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            items: Vec::new(),
        }
    }

    /// Add an item
    pub fn add_item(mut self, label: impl Into<String>, action: impl Into<String>) -> Self {
        self.items.push(MenuItem::new(label, action));
        self
    }

    /// Add a disabled item
    pub fn add_disabled_item(mut self, label: impl Into<String>, action: impl Into<String>) -> Self {
        self.items.push(MenuItem::disabled(label, action));
        self
    }

    /// Add a separator
    pub fn add_separator(mut self) -> Self {
        self.items.push(MenuItem::separator());
        self
    }

    /// Add an item with shortcut
    pub fn add_item_with_shortcut(
        mut self,
        label: impl Into<String>,
        action: impl Into<String>,
        shortcut: impl Into<String>,
    ) -> Self {
        self.items.push(MenuItem::new(label, action).with_shortcut(shortcut));
        self
    }

    /// Add a menu item directly
    pub fn add_menu_item(mut self, item: MenuItem) -> Self {
        self.items.push(item);
        self
    }

    /// Set all items at once
    pub fn items(mut self, items: Vec<MenuItem>) -> Self {
        self.items = items;
        self
    }

    /// Get item count
    pub fn item_count(&self) -> usize {
        self.items.len()
    }
}

/// MenuBar component - native application menu bar
/// 
/// # Example
/// ```
/// let mut menubar = MenuBar::new()
///     .add_menu(
///         Menu::new("File")
///             .add_item_with_shortcut("New", "file.new", "Ctrl+N")
///             .add_item_with_shortcut("Open", "file.open", "Ctrl+O")
///             .add_separator()
///             .add_item_with_shortcut("Exit", "file.exit", "Ctrl+Q")
///     )
///     .add_menu(
///         Menu::new("Edit")
///             .add_item_with_shortcut("Undo", "edit.undo", "Ctrl+Z")
///             .add_item_with_shortcut("Redo", "edit.redo", "Ctrl+Y")
///     )
///     .on_action(|action| println!("Action: {}", action));
/// ```
pub struct MenuBar {
    pub node_id: Option<NodeId>,
    pub menus: Vec<Menu>,
    pub active_menu: Signal<Option<usize>>,
    pub height: f32,
    pub padding: f32,
    pub background_color: (u8, u8, u8, u8),
    pub text_color: (u8, u8, u8, u8),
    pub hover_color: (u8, u8, u8, u8),
    pub active_color: (u8, u8, u8, u8),
    pub disabled_color: (u8, u8, u8, u8),
    pub on_action: Option<Box<dyn Fn(&str)>>,
    pub on_menu_open: Option<Box<dyn Fn(&str)>>,
    pub on_menu_close: Option<Box<dyn Fn()>>,
}

impl MenuBar {
    /// Create a new MenuBar component
    pub fn new() -> Self {
        Self {
            node_id: None,
            menus: Vec::new(),
            active_menu: Signal::new(None),
            height: 32.0,
            padding: 8.0,
            background_color: (240, 240, 240, 255),
            text_color: (0, 0, 0, 255),
            hover_color: (220, 220, 220, 255),
            active_color: (200, 200, 200, 255),
            disabled_color: (150, 150, 150, 255),
            on_action: None,
            on_menu_open: None,
            on_menu_close: None,
        }
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

    /// Set the active color
    pub fn active_color(mut self, r: u8, g: u8, b: u8, a: u8) -> Self {
        self.active_color = (r, g, b, a);
        self
    }

    /// Add a menu
    pub fn add_menu(mut self, menu: Menu) -> Self {
        self.menus.push(menu);
        self
    }

    /// Set all menus at once
    pub fn menus(mut self, menus: Vec<Menu>) -> Self {
        self.menus = menus;
        self
    }

    /// Set the action callback
    pub fn on_action<F>(mut self, callback: F) -> Self
    where
        F: Fn(&str) + 'static,
    {
        self.on_action = Some(Box::new(callback));
        self
    }

    /// Set the menu open callback
    pub fn on_menu_open<F>(mut self, callback: F) -> Self
    where
        F: Fn(&str) + 'static,
    {
        self.on_menu_open = Some(Box::new(callback));
        self
    }

    /// Set the menu close callback
    pub fn on_menu_close<F>(mut self, callback: F) -> Self
    where
        F: Fn() + 'static,
    {
        self.on_menu_close = Some(Box::new(callback));
        self
    }

    /// Open a menu by index
    pub fn open_menu(&mut self, index: usize) {
        if index < self.menus.len() {
            self.active_menu.set(Some(index));
            if let Some(ref callback) = self.on_menu_open {
                callback(&self.menus[index].label);
            }
        }
    }

    /// Close the active menu
    pub fn close_menu(&mut self) {
        self.active_menu.set(None);
        if let Some(ref callback) = self.on_menu_close {
            callback();
        }
    }

    /// Get the active menu index
    pub fn get_active_menu(&self) -> Option<usize> {
        self.active_menu.get()
    }

    /// Check if a menu is active
    pub fn is_menu_active(&self, index: usize) -> bool {
        self.active_menu.get() == Some(index)
    }

    /// Execute an action
    pub fn execute_action(&mut self, action: &str) {
        if let Some(ref callback) = self.on_action {
            callback(action);
        }
        self.close_menu();
    }

    /// Select an item from the active menu
    pub fn select_item(&mut self, item_index: usize) {
        if let Some(menu_index) = self.get_active_menu() {
            if menu_index < self.menus.len() {
                let menu = &self.menus[menu_index];
                if item_index < menu.items.len() {
                    let item = &menu.items[item_index];
                    if !item.disabled && !item.is_separator {
                        let action = item.action.clone();
                        self.execute_action(&action);
                    }
                }
            }
        }
    }

    /// Get menu count
    pub fn menu_count(&self) -> usize {
        self.menus.len()
    }

    /// Check if has menus
    pub fn has_menus(&self) -> bool {
        !self.menus.is_empty()
    }

    /// Find menu by label
    pub fn find_menu(&self, label: &str) -> Option<usize> {
        self.menus.iter().position(|menu| menu.label == label)
    }

    /// Get menu by index
    pub fn get_menu(&self, index: usize) -> Option<&Menu> {
        self.menus.get(index)
    }

    /// Build the menubar layout
    pub fn build(&mut self, engine: &mut LayoutEngine) -> Result<NodeId, String> {
        let style = taffy::style::Style {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Percent(1.0),
                height: taffy::style::Dimension::Length(self.height),
            },
            padding: taffy::geometry::Rect {
                left: taffy::style::LengthPercentage::Length(self.padding),
                right: taffy::style::LengthPercentage::Length(self.padding),
                top: taffy::style::LengthPercentage::Length(0.0),
                bottom: taffy::style::LengthPercentage::Length(0.0),
            },
            display: taffy::style::Display::Flex,
            flex_direction: taffy::style::FlexDirection::Row,
            ..Default::default()
        };

        let node = engine
            .new_leaf(style)
            .map_err(|e| format!("Failed to create menubar node: {:?}", e))?;
        self.node_id = Some(node);

        Ok(node)
    }
}

impl Default for MenuBar {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn menubar_starts_empty() {
        let menubar = MenuBar::new();
        assert_eq!(menubar.menu_count(), 0);
        assert!(!menubar.has_menus());
    }

    #[test]
    fn menubar_add_menus() {
        let menubar = MenuBar::new()
            .add_menu(Menu::new("File"))
            .add_menu(Menu::new("Edit"))
            .add_menu(Menu::new("View"));

        assert_eq!(menubar.menu_count(), 3);
        assert!(menubar.has_menus());
    }

    #[test]
    fn menu_add_items() {
        let menu = Menu::new("File")
            .add_item("New", "file.new")
            .add_item("Open", "file.open")
            .add_separator()
            .add_item("Exit", "file.exit");

        assert_eq!(menu.item_count(), 4);
    }

    #[test]
    fn menu_item_with_shortcut() {
        let item = MenuItem::new("Save", "file.save").with_shortcut("Ctrl+S");
        assert_eq!(item.shortcut, Some("Ctrl+S".to_string()));
    }

    #[test]
    fn menu_item_with_submenu() {
        let submenu = vec![
            MenuItem::new("Recent 1", "file.recent.1"),
            MenuItem::new("Recent 2", "file.recent.2"),
        ];
        let item = MenuItem::new("Recent", "file.recent").with_submenu(submenu);
        assert!(item.has_submenu());
    }

    #[test]
    fn menubar_open_menu() {
        let mut menubar = MenuBar::new()
            .add_menu(Menu::new("File"))
            .add_menu(Menu::new("Edit"));

        menubar.open_menu(0);
        assert_eq!(menubar.get_active_menu(), Some(0));
        assert!(menubar.is_menu_active(0));
        assert!(!menubar.is_menu_active(1));
    }

    #[test]
    fn menubar_close_menu() {
        let mut menubar = MenuBar::new().add_menu(Menu::new("File"));

        menubar.open_menu(0);
        assert!(menubar.get_active_menu().is_some());

        menubar.close_menu();
        assert!(menubar.get_active_menu().is_none());
    }

    #[test]
    fn menubar_execute_action() {
        use std::sync::{Arc, Mutex};

        let executed = Arc::new(Mutex::new(String::new()));
        let executed_clone = executed.clone();

        let mut menubar = MenuBar::new()
            .add_menu(Menu::new("File").add_item("New", "file.new"))
            .on_action(move |action| {
                *executed_clone.lock().unwrap() = action.to_string();
            });

        menubar.execute_action("file.new");
        assert_eq!(*executed.lock().unwrap(), "file.new");
    }

    #[test]
    fn menubar_select_item() {
        use std::sync::{Arc, Mutex};

        let executed = Arc::new(Mutex::new(String::new()));
        let executed_clone = executed.clone();

        let mut menubar = MenuBar::new()
            .add_menu(
                Menu::new("File")
                    .add_item("New", "file.new")
                    .add_item("Open", "file.open"),
            )
            .on_action(move |action| {
                *executed_clone.lock().unwrap() = action.to_string();
            });

        menubar.open_menu(0);
        menubar.select_item(1);

        assert_eq!(*executed.lock().unwrap(), "file.open");
        assert!(menubar.get_active_menu().is_none()); // Should close after selection
    }

    #[test]
    fn menubar_cannot_select_disabled_item() {
        use std::sync::{Arc, Mutex};

        let executed = Arc::new(Mutex::new(false));
        let executed_clone = executed.clone();

        let mut menubar = MenuBar::new()
            .add_menu(Menu::new("File").add_disabled_item("Disabled", "file.disabled"))
            .on_action(move |_| {
                *executed_clone.lock().unwrap() = true;
            });

        menubar.open_menu(0);
        menubar.select_item(0);

        assert!(!*executed.lock().unwrap());
    }

    #[test]
    fn menubar_cannot_select_separator() {
        use std::sync::{Arc, Mutex};

        let executed = Arc::new(Mutex::new(false));
        let executed_clone = executed.clone();

        let mut menubar = MenuBar::new()
            .add_menu(Menu::new("File").add_separator())
            .on_action(move |_| {
                *executed_clone.lock().unwrap() = true;
            });

        menubar.open_menu(0);
        menubar.select_item(0);

        assert!(!*executed.lock().unwrap());
    }

    #[test]
    fn menubar_find_menu() {
        let menubar = MenuBar::new()
            .add_menu(Menu::new("File"))
            .add_menu(Menu::new("Edit"))
            .add_menu(Menu::new("View"));

        assert_eq!(menubar.find_menu("Edit"), Some(1));
        assert_eq!(menubar.find_menu("Help"), None);
    }

    #[test]
    fn menubar_get_menu() {
        let menubar = MenuBar::new()
            .add_menu(Menu::new("File"))
            .add_menu(Menu::new("Edit"));

        let menu = menubar.get_menu(0);
        assert!(menu.is_some());
        assert_eq!(menu.unwrap().label, "File");

        let invalid = menubar.get_menu(10);
        assert!(invalid.is_none());
    }

    #[test]
    fn menubar_callbacks() {
        use std::sync::{Arc, Mutex};

        let opened = Arc::new(Mutex::new(String::new()));
        let opened_clone = opened.clone();

        let closed = Arc::new(Mutex::new(false));
        let closed_clone = closed.clone();

        let mut menubar = MenuBar::new()
            .add_menu(Menu::new("File"))
            .on_menu_open(move |label| {
                *opened_clone.lock().unwrap() = label.to_string();
            })
            .on_menu_close(move || {
                *closed_clone.lock().unwrap() = true;
            });

        menubar.open_menu(0);
        assert_eq!(*opened.lock().unwrap(), "File");

        menubar.close_menu();
        assert!(*closed.lock().unwrap());
    }

    #[test]
    fn menubar_builder_pattern() {
        let menubar = MenuBar::new()
            .height(40.0)
            .padding(12.0)
            .background_color(50, 50, 50, 255)
            .text_color(255, 255, 255, 255)
            .hover_color(70, 70, 70, 255)
            .active_color(90, 90, 90, 255);

        assert_eq!(menubar.height, 40.0);
        assert_eq!(menubar.padding, 12.0);
        assert_eq!(menubar.background_color, (50, 50, 50, 255));
        assert_eq!(menubar.text_color, (255, 255, 255, 255));
        assert_eq!(menubar.hover_color, (70, 70, 70, 255));
        assert_eq!(menubar.active_color, (90, 90, 90, 255));
    }

    #[test]
    fn menubar_build_creates_node() {
        let mut engine = LayoutEngine::new();
        let mut menubar = MenuBar::new().add_menu(Menu::new("File"));

        let result = menubar.build(&mut engine);
        assert!(result.is_ok());
        assert!(menubar.node_id.is_some());
    }

    #[test]
    fn menu_item_separator() {
        let separator = MenuItem::separator();
        assert!(separator.is_separator);
        assert!(separator.label.is_empty());
    }

    #[test]
    fn menu_item_disabled() {
        let item = MenuItem::disabled("Disabled", "action");
        assert!(item.disabled);
    }
}
