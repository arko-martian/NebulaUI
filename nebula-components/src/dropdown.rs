// Dropdown Component - Shows a list of selectable options
// Opens on click, closes on selection or outside click

use crate::container::VStack;
use nebula_core::layout::{LayoutEngine, NodeId};
use nebula_core::signal::Signal;

/// Dropdown option
#[derive(Debug, Clone, PartialEq)]
pub struct DropdownOption {
    pub label: String,
    pub value: String,
    pub disabled: bool,
}

impl DropdownOption {
    /// Create a new dropdown option
    pub fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
            disabled: false,
        }
    }

    /// Create a disabled option
    pub fn disabled(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
            disabled: true,
        }
    }
}

/// Dropdown component - displays a button that opens a list of options
/// 
/// # Example
/// ```
/// let mut dropdown = Dropdown::new()
///     .placeholder("Select an option")
///     .add_option("Option 1", "opt1")
///     .add_option("Option 2", "opt2")
///     .on_select(|value| println!("Selected: {}", value));
/// ```
pub struct Dropdown {
    pub node_id: Option<NodeId>,
    pub options: Vec<DropdownOption>,
    pub selected_index: Signal<Option<usize>>,
    pub is_open: Signal<bool>,
    pub placeholder: String,
    pub width: f32,
    pub max_height: f32, // Max height for dropdown list
    pub on_select: Option<Box<dyn Fn(&str)>>,
    pub on_open: Option<Box<dyn Fn()>>,
    pub on_close: Option<Box<dyn Fn()>>,
    pub searchable: bool,
    pub search_query: String,
    pub disabled: bool,
}

impl Dropdown {
    /// Create a new Dropdown component
    pub fn new() -> Self {
        Self {
            node_id: None,
            options: Vec::new(),
            selected_index: Signal::new(None),
            is_open: Signal::new(false),
            placeholder: "Select...".to_string(),
            width: 200.0,
            max_height: 300.0,
            on_select: None,
            on_open: None,
            on_close: None,
            searchable: false,
            search_query: String::new(),
            disabled: false,
        }
    }

    /// Set the placeholder text
    pub fn placeholder(mut self, text: impl Into<String>) -> Self {
        self.placeholder = text.into();
        self
    }

    /// Set the width
    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    /// Set the max height for the dropdown list
    pub fn max_height(mut self, height: f32) -> Self {
        self.max_height = height;
        self
    }

    /// Add an option to the dropdown
    pub fn add_option(mut self, label: impl Into<String>, value: impl Into<String>) -> Self {
        self.options.push(DropdownOption::new(label, value));
        self
    }

    /// Add a disabled option
    pub fn add_disabled_option(mut self, label: impl Into<String>, value: impl Into<String>) -> Self {
        self.options.push(DropdownOption::disabled(label, value));
        self
    }

    /// Set all options at once
    pub fn options(mut self, options: Vec<DropdownOption>) -> Self {
        self.options = options;
        self
    }

    /// Set the selection callback
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

    /// Enable search functionality
    pub fn searchable(mut self, searchable: bool) -> Self {
        self.searchable = searchable;
        self
    }

    /// Set disabled state
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Open the dropdown
    pub fn open(&mut self) {
        if !self.disabled {
            self.is_open.set(true);
            if let Some(ref callback) = self.on_open {
                callback();
            }
        }
    }

    /// Close the dropdown
    pub fn close(&mut self) {
        self.is_open.set(false);
        self.search_query.clear();
        if let Some(ref callback) = self.on_close {
            callback();
        }
    }

    /// Toggle the dropdown
    pub fn toggle(&mut self) {
        if self.is_open.get() {
            self.close();
        } else {
            self.open();
        }
    }

    /// Check if the dropdown is open
    pub fn is_open(&self) -> bool {
        self.is_open.get()
    }

    /// Select an option by index
    pub fn select(&mut self, index: usize) {
        if index < self.options.len() && !self.options[index].disabled {
            self.selected_index.set(Some(index));
            
            if let Some(ref callback) = self.on_select {
                callback(&self.options[index].value);
            }
            
            self.close();
        }
    }

    /// Select an option by value
    pub fn select_by_value(&mut self, value: &str) {
        if let Some(index) = self.options.iter().position(|opt| opt.value == value) {
            self.select(index);
        }
    }

    /// Get the currently selected option
    pub fn get_selected(&self) -> Option<&DropdownOption> {
        self.selected_index.get().and_then(|idx| self.options.get(idx))
    }

    /// Get the selected value
    pub fn get_selected_value(&self) -> Option<&str> {
        self.get_selected().map(|opt| opt.value.as_str())
    }

    /// Get the selected label
    pub fn get_selected_label(&self) -> Option<&str> {
        self.get_selected().map(|opt| opt.label.as_str())
    }

    /// Clear the selection
    pub fn clear(&mut self) {
        self.selected_index.set(None);
    }

    /// Update search query
    pub fn set_search_query(&mut self, query: impl Into<String>) {
        self.search_query = query.into();
    }

    /// Get filtered options based on search query
    pub fn get_filtered_options(&self) -> Vec<(usize, &DropdownOption)> {
        if self.search_query.is_empty() {
            self.options.iter().enumerate().collect()
        } else {
            let query = self.search_query.to_lowercase();
            self.options
                .iter()
                .enumerate()
                .filter(|(_, opt)| opt.label.to_lowercase().contains(&query))
                .collect()
        }
    }

    /// Get the number of options
    pub fn option_count(&self) -> usize {
        self.options.len()
    }

    /// Check if dropdown has options
    pub fn has_options(&self) -> bool {
        !self.options.is_empty()
    }

    /// Build the dropdown layout
    pub fn build(&mut self, engine: &mut LayoutEngine) -> Result<NodeId, String> {
        // Create a VStack for the dropdown (button + list)
        let mut vstack = VStack::new()
            .spacing(4.0);
        
        let node = vstack.build(engine)?;
        self.node_id = Some(node);

        // Set dropdown width
        let style = taffy::style::Style {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Length(self.width),
                height: taffy::style::Dimension::Auto,
            },
            ..Default::default()
        };
        engine.set_style(node, style)
            .map_err(|e| format!("Failed to set dropdown style: {:?}", e))?;

        Ok(node)
    }
}

impl Default for Dropdown {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dropdown_starts_closed() {
        let dropdown = Dropdown::new();
        assert!(!dropdown.is_open());
    }

    #[test]
    fn dropdown_can_be_opened() {
        let mut dropdown = Dropdown::new();
        dropdown.open();
        assert!(dropdown.is_open());
    }

    #[test]
    fn dropdown_can_be_closed() {
        let mut dropdown = Dropdown::new();
        dropdown.open();
        dropdown.close();
        assert!(!dropdown.is_open());
    }

    #[test]
    fn dropdown_can_be_toggled() {
        let mut dropdown = Dropdown::new();
        assert!(!dropdown.is_open());
        
        dropdown.toggle();
        assert!(dropdown.is_open());
        
        dropdown.toggle();
        assert!(!dropdown.is_open());
    }

    #[test]
    fn dropdown_disabled_cannot_open() {
        let mut dropdown = Dropdown::new().disabled(true);
        dropdown.open();
        assert!(!dropdown.is_open());
    }

    #[test]
    fn dropdown_add_options() {
        let dropdown = Dropdown::new()
            .add_option("Option 1", "opt1")
            .add_option("Option 2", "opt2")
            .add_option("Option 3", "opt3");

        assert_eq!(dropdown.option_count(), 3);
        assert!(dropdown.has_options());
    }

    #[test]
    fn dropdown_select_by_index() {
        let mut dropdown = Dropdown::new()
            .add_option("Option 1", "opt1")
            .add_option("Option 2", "opt2");

        dropdown.select(1);
        assert_eq!(dropdown.get_selected_value(), Some("opt2"));
        assert_eq!(dropdown.get_selected_label(), Some("Option 2"));
    }

    #[test]
    fn dropdown_select_by_value() {
        let mut dropdown = Dropdown::new()
            .add_option("Option 1", "opt1")
            .add_option("Option 2", "opt2");

        dropdown.select_by_value("opt1");
        assert_eq!(dropdown.get_selected_value(), Some("opt1"));
        assert_eq!(dropdown.get_selected_label(), Some("Option 1"));
    }

    #[test]
    fn dropdown_select_closes_dropdown() {
        let mut dropdown = Dropdown::new()
            .add_option("Option 1", "opt1");

        dropdown.open();
        assert!(dropdown.is_open());
        
        dropdown.select(0);
        assert!(!dropdown.is_open());
    }

    #[test]
    fn dropdown_cannot_select_disabled_option() {
        let mut dropdown = Dropdown::new()
            .add_option("Option 1", "opt1")
            .add_disabled_option("Option 2", "opt2");

        dropdown.select(1);
        assert_eq!(dropdown.get_selected_value(), None);
    }

    #[test]
    fn dropdown_clear_selection() {
        let mut dropdown = Dropdown::new()
            .add_option("Option 1", "opt1");

        dropdown.select(0);
        assert!(dropdown.get_selected().is_some());
        
        dropdown.clear();
        assert!(dropdown.get_selected().is_none());
    }

    #[test]
    fn dropdown_search_filters_options() {
        let mut dropdown = Dropdown::new()
            .add_option("Apple", "apple")
            .add_option("Banana", "banana")
            .add_option("Cherry", "cherry")
            .searchable(true);

        dropdown.set_search_query("an");
        let filtered = dropdown.get_filtered_options();
        
        assert_eq!(filtered.len(), 1); // Only Banana contains "an"
    }

    #[test]
    fn dropdown_search_case_insensitive() {
        let mut dropdown = Dropdown::new()
            .add_option("Apple", "apple")
            .add_option("Banana", "banana")
            .searchable(true);

        dropdown.set_search_query("APPLE");
        let filtered = dropdown.get_filtered_options();
        
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].1.value, "apple");
    }

    #[test]
    fn dropdown_builder_pattern() {
        let dropdown = Dropdown::new()
            .placeholder("Choose one")
            .width(300.0)
            .max_height(400.0)
            .searchable(true)
            .disabled(false);

        assert_eq!(dropdown.placeholder, "Choose one");
        assert_eq!(dropdown.width, 300.0);
        assert_eq!(dropdown.max_height, 400.0);
        assert!(dropdown.searchable);
        assert!(!dropdown.disabled);
    }

    #[test]
    fn dropdown_callbacks_are_called() {
        use std::sync::{Arc, Mutex};
        
        let selected = Arc::new(Mutex::new(String::new()));
        let selected_clone = selected.clone();
        
        let opened = Arc::new(Mutex::new(false));
        let opened_clone = opened.clone();
        
        let closed = Arc::new(Mutex::new(false));
        let closed_clone = closed.clone();
        
        let mut dropdown = Dropdown::new()
            .add_option("Test", "test_value")
            .on_select(move |value| {
                *selected_clone.lock().unwrap() = value.to_string();
            })
            .on_open(move || {
                *opened_clone.lock().unwrap() = true;
            })
            .on_close(move || {
                *closed_clone.lock().unwrap() = true;
            });

        dropdown.open();
        assert!(*opened.lock().unwrap());
        
        dropdown.select(0);
        assert_eq!(*selected.lock().unwrap(), "test_value");
        assert!(*closed.lock().unwrap());
    }

    #[test]
    fn dropdown_build_creates_node() {
        let mut engine = LayoutEngine::new();
        let mut dropdown = Dropdown::new()
            .add_option("Test", "test");
        
        let result = dropdown.build(&mut engine);
        assert!(result.is_ok());
        assert!(dropdown.node_id.is_some());
    }
}
