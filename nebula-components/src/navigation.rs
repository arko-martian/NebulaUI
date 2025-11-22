// Navigation Component - Navigation bar for app/website navigation
// Essential for top-level navigation with logo, links, and actions

use nebula_core::layout::{LayoutEngine, NodeId};
use nebula_core::signal::Signal;

/// Navigation item (link or button)
#[derive(Debug, Clone, PartialEq)]
pub struct NavItem {
    pub label: String,
    pub id: String,
    pub disabled: bool,
    pub icon: Option<String>,
    pub badge: Option<String>,
    pub href: Option<String>,
}

impl NavItem {
    /// Create a new navigation item
    pub fn new(label: impl Into<String>, id: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            id: id.into(),
            disabled: false,
            icon: None,
            badge: None,
            href: None,
        }
    }

    /// Create a disabled navigation item
    pub fn disabled(label: impl Into<String>, id: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            id: id.into(),
            disabled: true,
            icon: None,
            badge: None,
            href: None,
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

    /// Add a link href
    pub fn with_href(mut self, href: impl Into<String>) -> Self {
        self.href = Some(href.into());
        self
    }
}

/// Navigation component - navigation bar for app/website navigation
/// 
/// # Example
/// ```
/// let mut nav = Navigation::new()
///     .logo("MyApp")
///     .add_item("Home", "home")
///     .add_item("About", "about")
///     .add_item("Contact", "contact")
///     .add_action("Login", "login")
///     .on_navigate(|id| println!("Navigate to: {}", id))
///     .on_action(|id| println!("Action: {}", id));
/// ```
pub struct Navigation {
    pub node_id: Option<NodeId>,
    pub logo: Option<String>,
    pub items: Vec<NavItem>,
    pub actions: Vec<NavItem>,
    pub active_item: Signal<Option<usize>>,
    pub height: f32,
    pub padding: f32,
    pub background_color: (u8, u8, u8, u8),
    pub text_color: (u8, u8, u8, u8),
    pub active_color: (u8, u8, u8, u8),
    pub hover_color: (u8, u8, u8, u8),
    pub logo_color: (u8, u8, u8, u8),
    pub border_color: (u8, u8, u8, u8),
    pub show_border: bool,
    pub on_navigate: Option<Box<dyn Fn(&str)>>,
    pub on_action: Option<Box<dyn Fn(&str)>>,
    pub on_logo_click: Option<Box<dyn Fn()>>,
}

impl Navigation {
    /// Create a new Navigation component
    pub fn new() -> Self {
        Self {
            node_id: None,
            logo: None,
            items: Vec::new(),
            actions: Vec::new(),
            active_item: Signal::new(None),
            height: 64.0,
            padding: 16.0,
            background_color: (255, 255, 255, 255),
            text_color: (100, 100, 100, 255),
            active_color: (59, 130, 246, 255), // Blue
            hover_color: (240, 240, 240, 255),
            logo_color: (0, 0, 0, 255),
            border_color: (220, 220, 220, 255),
            show_border: true,
            on_navigate: None,
            on_action: None,
            on_logo_click: None,
        }
    }

    /// Set the logo text
    pub fn logo(mut self, logo: impl Into<String>) -> Self {
        self.logo = Some(logo.into());
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

    /// Set the active item color
    pub fn active_color(mut self, r: u8, g: u8, b: u8, a: u8) -> Self {
        self.active_color = (r, g, b, a);
        self
    }

    /// Set the hover color
    pub fn hover_color(mut self, r: u8, g: u8, b: u8, a: u8) -> Self {
        self.hover_color = (r, g, b, a);
        self
    }

    /// Set the logo color
    pub fn logo_color(mut self, r: u8, g: u8, b: u8, a: u8) -> Self {
        self.logo_color = (r, g, b, a);
        self
    }

    /// Show or hide the bottom border
    pub fn show_border(mut self, show: bool) -> Self {
        self.show_border = show;
        self
    }

    /// Add a navigation item
    pub fn add_item(mut self, label: impl Into<String>, id: impl Into<String>) -> Self {
        self.items.push(NavItem::new(label, id));
        self
    }

    /// Add a disabled navigation item
    pub fn add_disabled_item(mut self, label: impl Into<String>, id: impl Into<String>) -> Self {
        self.items.push(NavItem::disabled(label, id));
        self
    }

    /// Add a navigation item object
    pub fn add_item_object(mut self, item: NavItem) -> Self {
        self.items.push(item);
        self
    }

    /// Set all items at once
    pub fn items(mut self, items: Vec<NavItem>) -> Self {
        self.items = items;
        self
    }

    /// Add an action button (right side)
    pub fn add_action(mut self, label: impl Into<String>, id: impl Into<String>) -> Self {
        self.actions.push(NavItem::new(label, id));
        self
    }

    /// Add an action object
    pub fn add_action_object(mut self, action: NavItem) -> Self {
        self.actions.push(action);
        self
    }

    /// Set all actions at once
    pub fn actions(mut self, actions: Vec<NavItem>) -> Self {
        self.actions = actions;
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

    /// Set the action callback
    pub fn on_action<F>(mut self, callback: F) -> Self
    where
        F: Fn(&str) + 'static,
    {
        self.on_action = Some(Box::new(callback));
        self
    }

    /// Set the logo click callback
    pub fn on_logo_click<F>(mut self, callback: F) -> Self
    where
        F: Fn() + 'static,
    {
        self.on_logo_click = Some(Box::new(callback));
        self
    }

    /// Navigate to an item by index
    pub fn navigate_to(&mut self, index: usize) {
        if index < self.items.len() && !self.items[index].disabled {
            self.active_item.set(Some(index));
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

    /// Execute an action by index
    pub fn execute_action(&mut self, index: usize) {
        if index < self.actions.len() && !self.actions[index].disabled {
            if let Some(ref callback) = self.on_action {
                callback(&self.actions[index].id);
            }
        }
    }

    /// Execute an action by ID
    pub fn execute_action_by_id(&mut self, id: &str) {
        if let Some(index) = self.actions.iter().position(|action| action.id == id) {
            self.execute_action(index);
        }
    }

    /// Click the logo
    pub fn click_logo(&mut self) {
        if let Some(ref callback) = self.on_logo_click {
            callback();
        }
    }

    /// Get the active item index
    pub fn get_active_item(&self) -> Option<usize> {
        self.active_item.get()
    }

    /// Get the active item ID
    pub fn get_active_item_id(&self) -> Option<String> {
        self.get_active_item()
            .and_then(|idx| self.items.get(idx))
            .map(|item| item.id.clone())
    }

    /// Check if an item is active
    pub fn is_item_active(&self, index: usize) -> bool {
        self.active_item.get() == Some(index)
    }

    /// Get item count
    pub fn item_count(&self) -> usize {
        self.items.len()
    }

    /// Get action count
    pub fn action_count(&self) -> usize {
        self.actions.len()
    }

    /// Check if has items
    pub fn has_items(&self) -> bool {
        !self.items.is_empty()
    }

    /// Check if has actions
    pub fn has_actions(&self) -> bool {
        !self.actions.is_empty()
    }

    /// Check if has logo
    pub fn has_logo(&self) -> bool {
        self.logo.is_some()
    }

    /// Find item by ID
    pub fn find_item(&self, id: &str) -> Option<usize> {
        self.items.iter().position(|item| item.id == id)
    }

    /// Find action by ID
    pub fn find_action(&self, id: &str) -> Option<usize> {
        self.actions.iter().position(|action| action.id == id)
    }

    /// Get item by index
    pub fn get_item(&self, index: usize) -> Option<&NavItem> {
        self.items.get(index)
    }

    /// Get action by index
    pub fn get_action(&self, index: usize) -> Option<&NavItem> {
        self.actions.get(index)
    }

    /// Build the navigation layout
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
            justify_content: Some(taffy::style::JustifyContent::SpaceBetween),
            align_items: Some(taffy::style::AlignItems::Center),
            ..Default::default()
        };

        let node = engine
            .new_leaf(style)
            .map_err(|e| format!("Failed to create navigation node: {:?}", e))?;
        self.node_id = Some(node);

        Ok(node)
    }
}

impl Default for Navigation {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn navigation_starts_empty() {
        let nav = Navigation::new();
        assert_eq!(nav.item_count(), 0);
        assert_eq!(nav.action_count(), 0);
        assert!(!nav.has_items());
        assert!(!nav.has_actions());
        assert!(!nav.has_logo());
    }

    #[test]
    fn navigation_add_logo() {
        let nav = Navigation::new().logo("MyApp");
        assert!(nav.has_logo());
        assert_eq!(nav.logo, Some("MyApp".to_string()));
    }

    #[test]
    fn navigation_add_items() {
        let nav = Navigation::new()
            .add_item("Home", "home")
            .add_item("About", "about")
            .add_item("Contact", "contact");

        assert_eq!(nav.item_count(), 3);
        assert!(nav.has_items());
    }

    #[test]
    fn navigation_add_actions() {
        let nav = Navigation::new()
            .add_action("Login", "login")
            .add_action("Sign Up", "signup");

        assert_eq!(nav.action_count(), 2);
        assert!(nav.has_actions());
    }

    #[test]
    fn navigation_navigate_to() {
        let mut nav = Navigation::new()
            .add_item("Home", "home")
            .add_item("About", "about");

        nav.navigate_to(1);
        assert_eq!(nav.get_active_item(), Some(1));
        assert!(nav.is_item_active(1));
        assert!(!nav.is_item_active(0));
    }

    #[test]
    fn navigation_navigate_to_id() {
        let mut nav = Navigation::new()
            .add_item("Home", "home")
            .add_item("About", "about");

        nav.navigate_to_id("home");
        assert_eq!(nav.get_active_item_id(), Some("home".to_string()));
    }

    #[test]
    fn navigation_cannot_navigate_to_disabled() {
        let mut nav = Navigation::new()
            .add_item("Home", "home")
            .add_disabled_item("Disabled", "disabled");

        nav.navigate_to(1);
        assert!(nav.get_active_item().is_none());
    }

    #[test]
    fn navigation_execute_action() {
        use std::sync::{Arc, Mutex};

        let executed = Arc::new(Mutex::new(String::new()));
        let executed_clone = executed.clone();

        let mut nav = Navigation::new()
            .add_action("Login", "login")
            .on_action(move |id| {
                *executed_clone.lock().unwrap() = id.to_string();
            });

        nav.execute_action(0);
        assert_eq!(*executed.lock().unwrap(), "login");
    }

    #[test]
    fn navigation_execute_action_by_id() {
        use std::sync::{Arc, Mutex};

        let executed = Arc::new(Mutex::new(String::new()));
        let executed_clone = executed.clone();

        let mut nav = Navigation::new()
            .add_action("Login", "login")
            .add_action("Sign Up", "signup")
            .on_action(move |id| {
                *executed_clone.lock().unwrap() = id.to_string();
            });

        nav.execute_action_by_id("signup");
        assert_eq!(*executed.lock().unwrap(), "signup");
    }

    #[test]
    fn navigation_logo_click() {
        use std::sync::{Arc, Mutex};

        let clicked = Arc::new(Mutex::new(false));
        let clicked_clone = clicked.clone();

        let mut nav = Navigation::new()
            .logo("MyApp")
            .on_logo_click(move || {
                *clicked_clone.lock().unwrap() = true;
            });

        nav.click_logo();
        assert!(*clicked.lock().unwrap());
    }

    #[test]
    fn navigation_callbacks() {
        use std::sync::{Arc, Mutex};

        let navigated = Arc::new(Mutex::new(String::new()));
        let navigated_clone = navigated.clone();

        let mut nav = Navigation::new()
            .add_item("Home", "home")
            .add_item("About", "about")
            .on_navigate(move |id| {
                *navigated_clone.lock().unwrap() = id.to_string();
            });

        nav.navigate_to(1);
        assert_eq!(*navigated.lock().unwrap(), "about");
    }

    #[test]
    fn navigation_find_item() {
        let nav = Navigation::new()
            .add_item("Home", "home")
            .add_item("About", "about");

        assert_eq!(nav.find_item("about"), Some(1));
        assert_eq!(nav.find_item("nonexistent"), None);
    }

    #[test]
    fn navigation_find_action() {
        let nav = Navigation::new()
            .add_action("Login", "login")
            .add_action("Sign Up", "signup");

        assert_eq!(nav.find_action("signup"), Some(1));
        assert_eq!(nav.find_action("nonexistent"), None);
    }

    #[test]
    fn navigation_get_item() {
        let nav = Navigation::new()
            .add_item("Home", "home")
            .add_item("About", "about");

        let item = nav.get_item(0);
        assert!(item.is_some());
        assert_eq!(item.unwrap().label, "Home");
    }

    #[test]
    fn navigation_get_action() {
        let nav = Navigation::new()
            .add_action("Login", "login");

        let action = nav.get_action(0);
        assert!(action.is_some());
        assert_eq!(action.unwrap().label, "Login");
    }

    #[test]
    fn nav_item_with_icon_and_badge() {
        let item = NavItem::new("Messages", "messages")
            .with_icon("mail")
            .with_badge("5");

        assert_eq!(item.icon, Some("mail".to_string()));
        assert_eq!(item.badge, Some("5".to_string()));
    }

    #[test]
    fn nav_item_with_href() {
        let item = NavItem::new("Home", "home")
            .with_href("/home");

        assert_eq!(item.href, Some("/home".to_string()));
    }

    #[test]
    fn navigation_builder_pattern() {
        let nav = Navigation::new()
            .height(80.0)
            .padding(24.0)
            .background_color(50, 50, 50, 255)
            .text_color(255, 255, 255, 255)
            .active_color(255, 0, 0, 255)
            .hover_color(70, 70, 70, 255)
            .logo_color(255, 255, 0, 255)
            .show_border(false);

        assert_eq!(nav.height, 80.0);
        assert_eq!(nav.padding, 24.0);
        assert_eq!(nav.background_color, (50, 50, 50, 255));
        assert_eq!(nav.text_color, (255, 255, 255, 255));
        assert_eq!(nav.active_color, (255, 0, 0, 255));
        assert_eq!(nav.hover_color, (70, 70, 70, 255));
        assert_eq!(nav.logo_color, (255, 255, 0, 255));
        assert!(!nav.show_border);
    }

    #[test]
    fn navigation_build_creates_node() {
        let mut engine = LayoutEngine::new();
        let mut nav = Navigation::new()
            .logo("MyApp")
            .add_item("Home", "home");

        let result = nav.build(&mut engine);
        assert!(result.is_ok());
        assert!(nav.node_id.is_some());
    }

    #[test]
    fn nav_item_disabled() {
        let item = NavItem::disabled("Disabled", "disabled");
        assert!(item.disabled);
    }
}
