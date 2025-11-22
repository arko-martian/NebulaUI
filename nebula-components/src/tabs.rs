// Tabs Component - Tab navigation for organizing content
// Essential for multi-view interfaces

use nebula_core::layout::{LayoutEngine, NodeId};
use nebula_core::signal::Signal;

/// Tab item
#[derive(Debug, Clone, PartialEq)]
pub struct Tab {
    pub label: String,
    pub id: String,
    pub disabled: bool,
    pub icon: Option<String>,
    pub badge: Option<String>,
    pub closable: bool,
}

impl Tab {
    /// Create a new tab
    pub fn new(label: impl Into<String>, id: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            id: id.into(),
            disabled: false,
            icon: None,
            badge: None,
            closable: false,
        }
    }

    /// Create a disabled tab
    pub fn disabled(label: impl Into<String>, id: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            id: id.into(),
            disabled: true,
            icon: None,
            badge: None,
            closable: false,
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

    /// Make the tab closable
    pub fn closable(mut self, closable: bool) -> Self {
        self.closable = closable;
        self
    }
}

/// Tabs component - tab navigation for organizing content
/// 
/// # Example
/// ```
/// let mut tabs = Tabs::new()
///     .add_tab("Home", "home")
///     .add_tab("Profile", "profile")
///     .add_tab("Settings", "settings")
///     .on_change(|tab_id| println!("Switched to: {}", tab_id))
///     .on_close(|tab_id| println!("Closed: {}", tab_id));
/// ```
pub struct Tabs {
    pub node_id: Option<NodeId>,
    pub tabs: Vec<Tab>,
    pub active_tab: Signal<Option<usize>>,
    pub height: f32,
    pub padding: f32,
    pub background_color: (u8, u8, u8, u8),
    pub active_color: (u8, u8, u8, u8),
    pub inactive_color: (u8, u8, u8, u8),
    pub hover_color: (u8, u8, u8, u8),
    pub text_color: (u8, u8, u8, u8),
    pub active_text_color: (u8, u8, u8, u8),
    pub border_color: (u8, u8, u8, u8),
    pub indicator_color: (u8, u8, u8, u8),
    pub indicator_height: f32,
    pub on_change: Option<Box<dyn Fn(&str)>>,
    pub on_close: Option<Box<dyn Fn(&str)>>,
}

impl Tabs {
    /// Create a new Tabs component
    pub fn new() -> Self {
        Self {
            node_id: None,
            tabs: Vec::new(),
            active_tab: Signal::new(None),
            height: 48.0,
            padding: 16.0,
            background_color: (255, 255, 255, 255),
            active_color: (255, 255, 255, 255),
            inactive_color: (245, 245, 245, 255),
            hover_color: (240, 240, 240, 255),
            text_color: (100, 100, 100, 255),
            active_text_color: (0, 0, 0, 255),
            border_color: (220, 220, 220, 255),
            indicator_color: (59, 130, 246, 255), // Blue
            indicator_height: 3.0,
            on_change: None,
            on_close: None,
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

    /// Set the active tab color
    pub fn active_color(mut self, r: u8, g: u8, b: u8, a: u8) -> Self {
        self.active_color = (r, g, b, a);
        self
    }

    /// Set the indicator color
    pub fn indicator_color(mut self, r: u8, g: u8, b: u8, a: u8) -> Self {
        self.indicator_color = (r, g, b, a);
        self
    }

    /// Set the indicator height
    pub fn indicator_height(mut self, height: f32) -> Self {
        self.indicator_height = height;
        self
    }

    /// Add a tab
    pub fn add_tab(mut self, label: impl Into<String>, id: impl Into<String>) -> Self {
        self.tabs.push(Tab::new(label, id));
        self
    }

    /// Add a disabled tab
    pub fn add_disabled_tab(mut self, label: impl Into<String>, id: impl Into<String>) -> Self {
        self.tabs.push(Tab::disabled(label, id));
        self
    }

    /// Add a tab object
    pub fn add_tab_object(mut self, tab: Tab) -> Self {
        self.tabs.push(tab);
        self
    }

    /// Set all tabs at once
    pub fn tabs(mut self, tabs: Vec<Tab>) -> Self {
        self.tabs = tabs;
        self
    }

    /// Set the change callback
    pub fn on_change<F>(mut self, callback: F) -> Self
    where
        F: Fn(&str) + 'static,
    {
        self.on_change = Some(Box::new(callback));
        self
    }

    /// Set the close callback
    pub fn on_close<F>(mut self, callback: F) -> Self
    where
        F: Fn(&str) + 'static,
    {
        self.on_close = Some(Box::new(callback));
        self
    }

    /// Select a tab by index
    pub fn select_tab(&mut self, index: usize) {
        if index < self.tabs.len() && !self.tabs[index].disabled {
            self.active_tab.set(Some(index));
            if let Some(ref callback) = self.on_change {
                callback(&self.tabs[index].id);
            }
        }
    }

    /// Select a tab by ID
    pub fn select_tab_by_id(&mut self, id: &str) {
        if let Some(index) = self.tabs.iter().position(|tab| tab.id == id) {
            self.select_tab(index);
        }
    }

    /// Close a tab by index
    pub fn close_tab(&mut self, index: usize) {
        if index < self.tabs.len() && self.tabs[index].closable {
            let tab_id = self.tabs[index].id.clone();
            
            // If closing the active tab, select another
            if self.get_active_tab() == Some(index) {
                if index > 0 {
                    self.select_tab(index - 1);
                } else if self.tabs.len() > 1 {
                    self.select_tab(0);
                } else {
                    self.active_tab.set(None);
                }
            }
            
            self.tabs.remove(index);
            
            if let Some(ref callback) = self.on_close {
                callback(&tab_id);
            }
        }
    }

    /// Close a tab by ID
    pub fn close_tab_by_id(&mut self, id: &str) {
        if let Some(index) = self.tabs.iter().position(|tab| tab.id == id) {
            self.close_tab(index);
        }
    }

    /// Get the active tab index
    pub fn get_active_tab(&self) -> Option<usize> {
        self.active_tab.get()
    }

    /// Get the active tab ID
    pub fn get_active_tab_id(&self) -> Option<String> {
        self.get_active_tab()
            .and_then(|idx| self.tabs.get(idx))
            .map(|tab| tab.id.clone())
    }

    /// Check if a tab is active
    pub fn is_tab_active(&self, index: usize) -> bool {
        self.active_tab.get() == Some(index)
    }

    /// Get tab count
    pub fn tab_count(&self) -> usize {
        self.tabs.len()
    }

    /// Check if has tabs
    pub fn has_tabs(&self) -> bool {
        !self.tabs.is_empty()
    }

    /// Find tab by ID
    pub fn find_tab(&self, id: &str) -> Option<usize> {
        self.tabs.iter().position(|tab| tab.id == id)
    }

    /// Get tab by index
    pub fn get_tab(&self, index: usize) -> Option<&Tab> {
        self.tabs.get(index)
    }

    /// Build the tabs layout
    pub fn build(&mut self, engine: &mut LayoutEngine) -> Result<NodeId, String> {
        let style = taffy::style::Style {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Percent(1.0),
                height: taffy::style::Dimension::Length(self.height),
            },
            display: taffy::style::Display::Flex,
            flex_direction: taffy::style::FlexDirection::Row,
            ..Default::default()
        };

        let node = engine
            .new_leaf(style)
            .map_err(|e| format!("Failed to create tabs node: {:?}", e))?;
        self.node_id = Some(node);

        Ok(node)
    }
}

impl Default for Tabs {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tabs_starts_empty() {
        let tabs = Tabs::new();
        assert_eq!(tabs.tab_count(), 0);
        assert!(!tabs.has_tabs());
        assert!(tabs.get_active_tab().is_none());
    }

    #[test]
    fn tabs_add_tabs() {
        let tabs = Tabs::new()
            .add_tab("Home", "home")
            .add_tab("Profile", "profile")
            .add_tab("Settings", "settings");

        assert_eq!(tabs.tab_count(), 3);
        assert!(tabs.has_tabs());
    }

    #[test]
    fn tabs_select_tab() {
        let mut tabs = Tabs::new()
            .add_tab("Home", "home")
            .add_tab("Profile", "profile");

        tabs.select_tab(1);
        assert_eq!(tabs.get_active_tab(), Some(1));
        assert!(tabs.is_tab_active(1));
        assert!(!tabs.is_tab_active(0));
    }

    #[test]
    fn tabs_select_tab_by_id() {
        let mut tabs = Tabs::new()
            .add_tab("Home", "home")
            .add_tab("Profile", "profile");

        tabs.select_tab_by_id("home");
        assert_eq!(tabs.get_active_tab_id(), Some("home".to_string()));
    }

    #[test]
    fn tabs_cannot_select_disabled() {
        let mut tabs = Tabs::new()
            .add_tab("Home", "home")
            .add_disabled_tab("Disabled", "disabled");

        tabs.select_tab(1);
        assert!(tabs.get_active_tab().is_none());
    }

    #[test]
    fn tabs_close_tab() {
        let mut tabs = Tabs::new()
            .add_tab_object(Tab::new("Home", "home").closable(true))
            .add_tab_object(Tab::new("Profile", "profile").closable(true));

        assert_eq!(tabs.tab_count(), 2);
        tabs.close_tab(0);
        assert_eq!(tabs.tab_count(), 1);
    }

    #[test]
    fn tabs_close_tab_by_id() {
        let mut tabs = Tabs::new()
            .add_tab_object(Tab::new("Home", "home").closable(true))
            .add_tab_object(Tab::new("Profile", "profile").closable(true));

        tabs.close_tab_by_id("home");
        assert_eq!(tabs.tab_count(), 1);
        assert_eq!(tabs.tabs[0].id, "profile");
    }

    #[test]
    fn tabs_cannot_close_non_closable() {
        let mut tabs = Tabs::new()
            .add_tab("Home", "home"); // Not closable

        tabs.close_tab(0);
        assert_eq!(tabs.tab_count(), 1); // Should still be there
    }

    #[test]
    fn tabs_closing_active_selects_previous() {
        let mut tabs = Tabs::new()
            .add_tab_object(Tab::new("Tab 1", "tab1").closable(true))
            .add_tab_object(Tab::new("Tab 2", "tab2").closable(true))
            .add_tab_object(Tab::new("Tab 3", "tab3").closable(true));

        tabs.select_tab(1);
        tabs.close_tab(1);
        
        assert_eq!(tabs.get_active_tab(), Some(0)); // Should select previous
    }

    #[test]
    fn tabs_find_tab() {
        let tabs = Tabs::new()
            .add_tab("Home", "home")
            .add_tab("Profile", "profile");

        assert_eq!(tabs.find_tab("profile"), Some(1));
        assert_eq!(tabs.find_tab("nonexistent"), None);
    }

    #[test]
    fn tabs_get_tab() {
        let tabs = Tabs::new()
            .add_tab("Home", "home")
            .add_tab("Profile", "profile");

        let tab = tabs.get_tab(0);
        assert!(tab.is_some());
        assert_eq!(tab.unwrap().label, "Home");
    }

    #[test]
    fn tab_with_icon_and_badge() {
        let tab = Tab::new("Messages", "messages")
            .with_icon("mail")
            .with_badge("5");

        assert_eq!(tab.icon, Some("mail".to_string()));
        assert_eq!(tab.badge, Some("5".to_string()));
    }

    #[test]
    fn tabs_callbacks() {
        use std::sync::{Arc, Mutex};

        let changed = Arc::new(Mutex::new(String::new()));
        let changed_clone = changed.clone();

        let closed = Arc::new(Mutex::new(String::new()));
        let closed_clone = closed.clone();

        let mut tabs = Tabs::new()
            .add_tab("Home", "home")
            .add_tab_object(Tab::new("Profile", "profile").closable(true))
            .on_change(move |id| {
                *changed_clone.lock().unwrap() = id.to_string();
            })
            .on_close(move |id| {
                *closed_clone.lock().unwrap() = id.to_string();
            });

        tabs.select_tab(0);
        assert_eq!(*changed.lock().unwrap(), "home");

        tabs.close_tab(1);
        assert_eq!(*closed.lock().unwrap(), "profile");
    }

    #[test]
    fn tabs_builder_pattern() {
        let tabs = Tabs::new()
            .height(60.0)
            .padding(20.0)
            .background_color(50, 50, 50, 255)
            .active_color(70, 70, 70, 255)
            .indicator_color(255, 0, 0, 255)
            .indicator_height(4.0);

        assert_eq!(tabs.height, 60.0);
        assert_eq!(tabs.padding, 20.0);
        assert_eq!(tabs.background_color, (50, 50, 50, 255));
        assert_eq!(tabs.active_color, (70, 70, 70, 255));
        assert_eq!(tabs.indicator_color, (255, 0, 0, 255));
        assert_eq!(tabs.indicator_height, 4.0);
    }

    #[test]
    fn tabs_build_creates_node() {
        let mut engine = LayoutEngine::new();
        let mut tabs = Tabs::new().add_tab("Home", "home");

        let result = tabs.build(&mut engine);
        assert!(result.is_ok());
        assert!(tabs.node_id.is_some());
    }
}
