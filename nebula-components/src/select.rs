// Select Component - Enhanced dropdown with multi-select support
// Similar to Dropdown but with additional features like multi-selection

use crate::container::VStack;
use nebula_core::layout::{LayoutEngine, NodeId};
use nebula_core::signal::Signal;

/// Select option
#[derive(Debug, Clone, PartialEq)]
pub struct SelectOption {
    pub label: String,
    pub value: String,
    pub disabled: bool,
    pub group: Option<String>,
}

impl SelectOption {
    /// Create a new select option
    pub fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
            disabled: false,
            group: None,
        }
    }

    /// Create a disabled option
    pub fn disabled(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
            disabled: true,
            group: None,
        }
    }

    /// Set the option group
    pub fn with_group(mut self, group: impl Into<String>) -> Self {
        self.group = Some(group.into());
        self
    }
}

/// Select component - enhanced dropdown with multi-select support
/// 
/// # Example
/// ```
/// let mut select = Select::new()
///     .placeholder("Select items")
///     .multi_select(true)
///     .add_option("Option 1", "opt1")
///     .add_option("Option 2", "opt2")
///     .on_change(|values| println!("Selected: {:?}", values));
/// ```
pub struct Select {
    pub node_id: Option<NodeId>,
    pub options: Vec<SelectOption>,
    pub selected_indices: Signal<Vec<usize>>,
    pub is_open: Signal<bool>,
    pub placeholder: String,
    pub width: f32,
    pub max_height: f32,
    pub multi_select: bool,
    pub max_selections: Option<usize>,
    pub on_change: Option<Box<dyn Fn(&[String])>>,
    pub on_open: Option<Box<dyn Fn()>>,
    pub on_close: Option<Box<dyn Fn()>>,
    pub searchable: bool,
    pub search_query: String,
    pub disabled: bool,
    pub clearable: bool,
}

impl Select {
    /// Create a new Select component
    pub fn new() -> Self {
        Self {
            node_id: None,
            options: Vec::new(),
            selected_indices: Signal::new(Vec::new()),
            is_open: Signal::new(false),
            placeholder: "Select...".to_string(),
            width: 200.0,
            max_height: 300.0,
            multi_select: false,
            max_selections: None,
            on_change: None,
            on_open: None,
            on_close: None,
            searchable: false,
            search_query: String::new(),
            disabled: false,
            clearable: true,
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

    /// Set the max height
    pub fn max_height(mut self, height: f32) -> Self {
        self.max_height = height;
        self
    }

    /// Enable multi-select mode
    pub fn multi_select(mut self, enabled: bool) -> Self {
        self.multi_select = enabled;
        self
    }

    /// Set maximum number of selections (for multi-select)
    pub fn max_selections(mut self, max: usize) -> Self {
        self.max_selections = Some(max);
        self
    }

    /// Add an option
    pub fn add_option(mut self, label: impl Into<String>, value: impl Into<String>) -> Self {
        self.options.push(SelectOption::new(label, value));
        self
    }

    /// Add a disabled option
    pub fn add_disabled_option(mut self, label: impl Into<String>, value: impl Into<String>) -> Self {
        self.options.push(SelectOption::disabled(label, value));
        self
    }

    /// Add an option with a group
    pub fn add_grouped_option(
        mut self,
        label: impl Into<String>,
        value: impl Into<String>,
        group: impl Into<String>,
    ) -> Self {
        self.options.push(SelectOption::new(label, value).with_group(group));
        self
    }

    /// Set all options at once
    pub fn options(mut self, options: Vec<SelectOption>) -> Self {
        self.options = options;
        self
    }

    /// Set the change callback
    pub fn on_change<F>(mut self, callback: F) -> Self
    where
        F: Fn(&[String]) + 'static,
    {
        self.on_change = Some(Box::new(callback));
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

    /// Enable/disable clearable
    pub fn clearable(mut self, clearable: bool) -> Self {
        self.clearable = clearable;
        self
    }

    /// Set disabled state
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Open the select
    pub fn open(&mut self) {
        if !self.disabled {
            self.is_open.set(true);
            if let Some(ref callback) = self.on_open {
                callback();
            }
        }
    }

    /// Close the select
    pub fn close(&mut self) {
        self.is_open.set(false);
        self.search_query.clear();
        if let Some(ref callback) = self.on_close {
            callback();
        }
    }

    /// Toggle the select
    pub fn toggle(&mut self) {
        if self.is_open.get() {
            self.close();
        } else {
            self.open();
        }
    }

    /// Check if the select is open
    pub fn is_open(&self) -> bool {
        self.is_open.get()
    }

    /// Select an option by index
    pub fn select(&mut self, index: usize) {
        if index >= self.options.len() || self.options[index].disabled {
            return;
        }

        let mut indices = self.selected_indices.get();

        if self.multi_select {
            // Multi-select mode
            if let Some(pos) = indices.iter().position(|&i| i == index) {
                // Already selected, remove it
                indices.remove(pos);
            } else {
                // Not selected, add it (if under max limit)
                if let Some(max) = self.max_selections {
                    if indices.len() >= max {
                        return; // Max selections reached
                    }
                }
                indices.push(index);
            }
        } else {
            // Single-select mode
            indices = vec![index];
            self.close();
        }

        self.selected_indices.set(indices.clone());
        self.trigger_change();
    }

    /// Select multiple options by indices
    pub fn select_multiple(&mut self, indices: Vec<usize>) {
        if !self.multi_select {
            return;
        }

        let valid_indices: Vec<usize> = indices
            .into_iter()
            .filter(|&i| i < self.options.len() && !self.options[i].disabled)
            .collect();

        let limited_indices = if let Some(max) = self.max_selections {
            valid_indices.into_iter().take(max).collect()
        } else {
            valid_indices
        };

        self.selected_indices.set(limited_indices);
        self.trigger_change();
    }

    /// Select by value
    pub fn select_by_value(&mut self, value: &str) {
        if let Some(index) = self.options.iter().position(|opt| opt.value == value) {
            self.select(index);
        }
    }

    /// Get selected options
    pub fn get_selected(&self) -> Vec<&SelectOption> {
        self.selected_indices
            .get()
            .iter()
            .filter_map(|&idx| self.options.get(idx))
            .collect()
    }

    /// Get selected values
    pub fn get_selected_values(&self) -> Vec<String> {
        self.get_selected()
            .iter()
            .map(|opt| opt.value.clone())
            .collect()
    }

    /// Get selected labels
    pub fn get_selected_labels(&self) -> Vec<String> {
        self.get_selected()
            .iter()
            .map(|opt| opt.label.clone())
            .collect()
    }

    /// Check if an option is selected
    pub fn is_selected(&self, index: usize) -> bool {
        self.selected_indices.get().contains(&index)
    }

    /// Clear all selections
    pub fn clear(&mut self) {
        self.selected_indices.set(Vec::new());
        self.trigger_change();
    }

    /// Trigger the change callback
    fn trigger_change(&self) {
        if let Some(ref callback) = self.on_change {
            let values = self.get_selected_values();
            callback(&values);
        }
    }

    /// Update search query
    pub fn set_search_query(&mut self, query: impl Into<String>) {
        self.search_query = query.into();
    }

    /// Get filtered options based on search query
    pub fn get_filtered_options(&self) -> Vec<(usize, &SelectOption)> {
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

    /// Get option count
    pub fn option_count(&self) -> usize {
        self.options.len()
    }

    /// Get selection count
    pub fn selection_count(&self) -> usize {
        self.selected_indices.get().len()
    }

    /// Check if has selections
    pub fn has_selections(&self) -> bool {
        !self.selected_indices.get().is_empty()
    }

    /// Build the select layout
    pub fn build(&mut self, engine: &mut LayoutEngine) -> Result<NodeId, String> {
        let mut vstack = VStack::new().spacing(4.0);
        let node = vstack.build(engine)?;
        self.node_id = Some(node);

        let style = taffy::style::Style {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Length(self.width),
                height: taffy::style::Dimension::Auto,
            },
            ..Default::default()
        };
        engine
            .set_style(node, style)
            .map_err(|e| format!("Failed to set select style: {:?}", e))?;

        Ok(node)
    }
}

impl Default for Select {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn select_starts_closed() {
        let select = Select::new();
        assert!(!select.is_open());
    }

    #[test]
    fn select_can_be_opened() {
        let mut select = Select::new();
        select.open();
        assert!(select.is_open());
    }

    #[test]
    fn select_can_be_closed() {
        let mut select = Select::new();
        select.open();
        select.close();
        assert!(!select.is_open());
    }

    #[test]
    fn select_single_mode() {
        let mut select = Select::new()
            .add_option("Option 1", "opt1")
            .add_option("Option 2", "opt2");

        select.select(0);
        assert_eq!(select.selection_count(), 1);
        assert_eq!(select.get_selected_values(), vec!["opt1"]);

        select.select(1);
        assert_eq!(select.selection_count(), 1);
        assert_eq!(select.get_selected_values(), vec!["opt2"]);
    }

    #[test]
    fn select_multi_mode() {
        let mut select = Select::new()
            .multi_select(true)
            .add_option("Option 1", "opt1")
            .add_option("Option 2", "opt2")
            .add_option("Option 3", "opt3");

        select.select(0);
        assert_eq!(select.selection_count(), 1);

        select.select(1);
        assert_eq!(select.selection_count(), 2);

        select.select(2);
        assert_eq!(select.selection_count(), 3);

        let values = select.get_selected_values();
        assert!(values.contains(&"opt1".to_string()));
        assert!(values.contains(&"opt2".to_string()));
        assert!(values.contains(&"opt3".to_string()));
    }

    #[test]
    fn select_multi_mode_toggle() {
        let mut select = Select::new()
            .multi_select(true)
            .add_option("Option 1", "opt1");

        select.select(0);
        assert_eq!(select.selection_count(), 1);

        select.select(0); // Toggle off
        assert_eq!(select.selection_count(), 0);
    }

    #[test]
    fn select_max_selections() {
        let mut select = Select::new()
            .multi_select(true)
            .max_selections(2)
            .add_option("Option 1", "opt1")
            .add_option("Option 2", "opt2")
            .add_option("Option 3", "opt3");

        select.select(0);
        select.select(1);
        select.select(2); // Should be ignored

        assert_eq!(select.selection_count(), 2);
    }

    #[test]
    fn select_disabled_option() {
        let mut select = Select::new()
            .add_option("Option 1", "opt1")
            .add_disabled_option("Option 2", "opt2");

        select.select(1);
        assert_eq!(select.selection_count(), 0);
    }

    #[test]
    fn select_by_value() {
        let mut select = Select::new()
            .add_option("Option 1", "opt1")
            .add_option("Option 2", "opt2");

        select.select_by_value("opt2");
        assert_eq!(select.get_selected_values(), vec!["opt2"]);
    }

    #[test]
    fn select_clear() {
        let mut select = Select::new()
            .multi_select(true)
            .add_option("Option 1", "opt1")
            .add_option("Option 2", "opt2");

        select.select(0);
        select.select(1);
        assert_eq!(select.selection_count(), 2);

        select.clear();
        assert_eq!(select.selection_count(), 0);
    }

    #[test]
    fn select_is_selected() {
        let mut select = Select::new()
            .add_option("Option 1", "opt1")
            .add_option("Option 2", "opt2");

        select.select(0);
        assert!(select.is_selected(0));
        assert!(!select.is_selected(1));
    }

    #[test]
    fn select_search_filters() {
        let mut select = Select::new()
            .add_option("Apple", "apple")
            .add_option("Banana", "banana")
            .add_option("Cherry", "cherry")
            .searchable(true);

        select.set_search_query("an");
        let filtered = select.get_filtered_options();
        assert_eq!(filtered.len(), 1); // Only Banana
    }

    #[test]
    fn select_grouped_options() {
        let select = Select::new()
            .add_grouped_option("Apple", "apple", "Fruits")
            .add_grouped_option("Carrot", "carrot", "Vegetables");

        assert_eq!(select.options[0].group, Some("Fruits".to_string()));
        assert_eq!(select.options[1].group, Some("Vegetables".to_string()));
    }

    #[test]
    fn select_callbacks() {
        use std::sync::{Arc, Mutex};

        let changed = Arc::new(Mutex::new(Vec::new()));
        let changed_clone = changed.clone();

        let mut select = Select::new()
            .add_option("Option 1", "opt1")
            .on_change(move |values| {
                *changed_clone.lock().unwrap() = values.to_vec();
            });

        select.select(0);
        assert_eq!(*changed.lock().unwrap(), vec!["opt1"]);
    }

    #[test]
    fn select_build_creates_node() {
        let mut engine = LayoutEngine::new();
        let mut select = Select::new().add_option("Test", "test");

        let result = select.build(&mut engine);
        assert!(result.is_ok());
        assert!(select.node_id.is_some());
    }
}
