// Table Component - Data table with columns, rows, and sorting
// Essential for displaying tabular data

use nebula_core::layout::{LayoutEngine, NodeId};
use nebula_core::signal::Signal;

/// Table column definition
#[derive(Debug, Clone, PartialEq)]
pub struct TableColumn {
    pub id: String,
    pub label: String,
    pub width: Option<f32>,
    pub sortable: bool,
    pub resizable: bool,
    pub align: ColumnAlign,
}

/// Column alignment
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ColumnAlign {
    Left,
    Center,
    Right,
}

impl TableColumn {
    /// Create a new column
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            width: None,
            sortable: true,
            resizable: true,
            align: ColumnAlign::Left,
        }
    }

    /// Set the width
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    /// Set sortable
    pub fn sortable(mut self, sortable: bool) -> Self {
        self.sortable = sortable;
        self
    }

    /// Set resizable
    pub fn resizable(mut self, resizable: bool) -> Self {
        self.resizable = resizable;
        self
    }

    /// Set alignment
    pub fn align(mut self, align: ColumnAlign) -> Self {
        self.align = align;
        self
    }
}

/// Table row
#[derive(Debug, Clone, PartialEq)]
pub struct TableRow {
    pub id: String,
    pub cells: Vec<String>,
    pub disabled: bool,
    pub metadata: Option<String>,
}

impl TableRow {
    /// Create a new row
    pub fn new(id: impl Into<String>, cells: Vec<String>) -> Self {
        Self {
            id: id.into(),
            cells,
            disabled: false,
            metadata: None,
        }
    }

    /// Create a disabled row
    pub fn disabled(id: impl Into<String>, cells: Vec<String>) -> Self {
        Self {
            id: id.into(),
            cells,
            disabled: true,
            metadata: None,
        }
    }

    /// Add metadata
    pub fn with_metadata(mut self, metadata: impl Into<String>) -> Self {
        self.metadata = Some(metadata.into());
        self
    }
}

/// Sort direction
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SortDirection {
    Ascending,
    Descending,
}

/// Table component - display tabular data with sorting and selection
/// 
/// # Example
/// ```
/// let mut table = Table::new()
///     .add_column("name", "Name")
///     .add_column("age", "Age")
///     .add_column("email", "Email")
///     .add_row("row1", vec!["Alice".to_string(), "30".to_string(), "alice@example.com".to_string()])
///     .add_row("row2", vec!["Bob".to_string(), "25".to_string(), "bob@example.com".to_string()])
///     .on_row_click(|row_id| println!("Clicked: {}", row_id));
/// ```
pub struct Table {
    pub node_id: Option<NodeId>,
    pub columns: Vec<TableColumn>,
    pub rows: Vec<TableRow>,
    pub selected_rows: Signal<Vec<String>>,
    pub sort_column: Signal<Option<String>>,
    pub sort_direction: Signal<SortDirection>,
    pub row_height: f32,
    pub header_height: f32,
    pub padding: f32,
    pub background_color: (u8, u8, u8, u8),
    pub header_color: (u8, u8, u8, u8),
    pub row_color: (u8, u8, u8, u8),
    pub alt_row_color: (u8, u8, u8, u8),
    pub selected_color: (u8, u8, u8, u8),
    pub hover_color: (u8, u8, u8, u8),
    pub text_color: (u8, u8, u8, u8),
    pub header_text_color: (u8, u8, u8, u8),
    pub border_color: (u8, u8, u8, u8),
    pub show_header: bool,
    pub striped: bool,
    pub hoverable: bool,
    pub selectable: bool,
    pub on_row_click: Option<Box<dyn Fn(&str)>>,
    pub on_sort: Option<Box<dyn Fn(&str, SortDirection)>>,
}

impl Table {
    /// Create a new Table component
    pub fn new() -> Self {
        Self {
            node_id: None,
            columns: Vec::new(),
            rows: Vec::new(),
            selected_rows: Signal::new(Vec::new()),
            sort_column: Signal::new(None),
            sort_direction: Signal::new(SortDirection::Ascending),
            row_height: 48.0,
            header_height: 56.0,
            padding: 16.0,
            background_color: (255, 255, 255, 255),
            header_color: (250, 250, 250, 255),
            row_color: (255, 255, 255, 255),
            alt_row_color: (249, 249, 249, 255),
            selected_color: (59, 130, 246, 20), // Light blue
            hover_color: (245, 245, 245, 255),
            text_color: (0, 0, 0, 255),
            header_text_color: (100, 100, 100, 255),
            border_color: (220, 220, 220, 255),
            show_header: true,
            striped: true,
            hoverable: true,
            selectable: true,
            on_row_click: None,
            on_sort: None,
        }
    }

    /// Set row height
    pub fn row_height(mut self, height: f32) -> Self {
        self.row_height = height;
        self
    }

    /// Set header height
    pub fn header_height(mut self, height: f32) -> Self {
        self.header_height = height;
        self
    }

    /// Set padding
    pub fn padding(mut self, padding: f32) -> Self {
        self.padding = padding;
        self
    }

    /// Show or hide header
    pub fn show_header(mut self, show: bool) -> Self {
        self.show_header = show;
        self
    }

    /// Enable or disable striped rows
    pub fn striped(mut self, striped: bool) -> Self {
        self.striped = striped;
        self
    }

    /// Enable or disable hover effect
    pub fn hoverable(mut self, hoverable: bool) -> Self {
        self.hoverable = hoverable;
        self
    }

    /// Enable or disable row selection
    pub fn selectable(mut self, selectable: bool) -> Self {
        self.selectable = selectable;
        self
    }

    /// Set background color
    pub fn background_color(mut self, r: u8, g: u8, b: u8, a: u8) -> Self {
        self.background_color = (r, g, b, a);
        self
    }

    /// Set selected row color
    pub fn selected_color(mut self, r: u8, g: u8, b: u8, a: u8) -> Self {
        self.selected_color = (r, g, b, a);
        self
    }

    /// Add a column
    pub fn add_column(mut self, id: impl Into<String>, label: impl Into<String>) -> Self {
        self.columns.push(TableColumn::new(id, label));
        self
    }

    /// Add a column object
    pub fn add_column_object(mut self, column: TableColumn) -> Self {
        self.columns.push(column);
        self
    }

    /// Set all columns at once
    pub fn columns(mut self, columns: Vec<TableColumn>) -> Self {
        self.columns = columns;
        self
    }

    /// Add a row
    pub fn add_row(mut self, id: impl Into<String>, cells: Vec<String>) -> Self {
        self.rows.push(TableRow::new(id, cells));
        self
    }

    /// Add a disabled row
    pub fn add_disabled_row(mut self, id: impl Into<String>, cells: Vec<String>) -> Self {
        self.rows.push(TableRow::disabled(id, cells));
        self
    }

    /// Add a row object
    pub fn add_row_object(mut self, row: TableRow) -> Self {
        self.rows.push(row);
        self
    }

    /// Set all rows at once
    pub fn rows(mut self, rows: Vec<TableRow>) -> Self {
        self.rows = rows;
        self
    }

    /// Set the row click callback
    pub fn on_row_click<F>(mut self, callback: F) -> Self
    where
        F: Fn(&str) + 'static,
    {
        self.on_row_click = Some(Box::new(callback));
        self
    }

    /// Set the sort callback
    pub fn on_sort<F>(mut self, callback: F) -> Self
    where
        F: Fn(&str, SortDirection) + 'static,
    {
        self.on_sort = Some(Box::new(callback));
        self
    }

    /// Select a row by ID
    pub fn select_row(&mut self, id: &str) {
        if !self.selectable {
            return;
        }

        if let Some(row) = self.rows.iter().find(|r| r.id == id) {
            if row.disabled {
                return;
            }

            let mut selected = self.selected_rows.get();
            if !selected.contains(&id.to_string()) {
                selected.push(id.to_string());
                self.selected_rows.set(selected);
            }
        }
    }

    /// Deselect a row by ID
    pub fn deselect_row(&mut self, id: &str) {
        let mut selected = self.selected_rows.get();
        if let Some(pos) = selected.iter().position(|i| i == id) {
            selected.remove(pos);
            self.selected_rows.set(selected);
        }
    }

    /// Toggle row selection
    pub fn toggle_row(&mut self, id: &str) {
        if self.is_row_selected(id) {
            self.deselect_row(id);
        } else {
            self.select_row(id);
        }
    }

    /// Clear all selections
    pub fn clear_selection(&mut self) {
        self.selected_rows.set(Vec::new());
    }

    /// Check if a row is selected
    pub fn is_row_selected(&self, id: &str) -> bool {
        self.selected_rows.get().contains(&id.to_string())
    }

    /// Get selected rows
    pub fn get_selected_rows(&self) -> Vec<String> {
        self.selected_rows.get()
    }

    /// Sort by column
    pub fn sort_by_column(&mut self, column_id: &str) {
        if let Some(column) = self.columns.iter().find(|c| c.id == column_id) {
            if !column.sortable {
                return;
            }

            // Toggle direction if same column
            let direction = if self.sort_column.get().as_deref() == Some(column_id) {
                match self.sort_direction.get() {
                    SortDirection::Ascending => SortDirection::Descending,
                    SortDirection::Descending => SortDirection::Ascending,
                }
            } else {
                SortDirection::Ascending
            };

            self.sort_column.set(Some(column_id.to_string()));
            self.sort_direction.set(direction);

            if let Some(ref callback) = self.on_sort {
                callback(column_id, direction);
            }
        }
    }

    /// Get sort column
    pub fn get_sort_column(&self) -> Option<String> {
        self.sort_column.get()
    }

    /// Get sort direction
    pub fn get_sort_direction(&self) -> SortDirection {
        self.sort_direction.get()
    }

    /// Get row count
    pub fn row_count(&self) -> usize {
        self.rows.len()
    }

    /// Get column count
    pub fn column_count(&self) -> usize {
        self.columns.len()
    }

    /// Check if has rows
    pub fn has_rows(&self) -> bool {
        !self.rows.is_empty()
    }

    /// Check if has columns
    pub fn has_columns(&self) -> bool {
        !self.columns.is_empty()
    }

    /// Find row by ID
    pub fn find_row(&self, id: &str) -> Option<usize> {
        self.rows.iter().position(|row| row.id == id)
    }

    /// Get row by index
    pub fn get_row(&self, index: usize) -> Option<&TableRow> {
        self.rows.get(index)
    }

    /// Find column by ID
    pub fn find_column(&self, id: &str) -> Option<usize> {
        self.columns.iter().position(|col| col.id == id)
    }

    /// Get column by index
    pub fn get_column(&self, index: usize) -> Option<&TableColumn> {
        self.columns.get(index)
    }

    /// Remove row by ID
    pub fn remove_row(&mut self, id: &str) {
        if let Some(index) = self.find_row(id) {
            self.rows.remove(index);
            self.deselect_row(id);
        }
    }

    /// Build the table layout
    pub fn build(&mut self, engine: &mut LayoutEngine) -> Result<NodeId, String> {
        let style = taffy::style::Style {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Percent(1.0),
                height: taffy::style::Dimension::Auto,
            },
            display: taffy::style::Display::Flex,
            flex_direction: taffy::style::FlexDirection::Column,
            ..Default::default()
        };

        let node = engine
            .new_leaf(style)
            .map_err(|e| format!("Failed to create table node: {:?}", e))?;
        self.node_id = Some(node);

        Ok(node)
    }
}

impl Default for Table {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn table_starts_empty() {
        let table = Table::new();
        assert_eq!(table.row_count(), 0);
        assert_eq!(table.column_count(), 0);
        assert!(!table.has_rows());
        assert!(!table.has_columns());
    }

    #[test]
    fn table_add_columns() {
        let table = Table::new()
            .add_column("name", "Name")
            .add_column("age", "Age")
            .add_column("email", "Email");

        assert_eq!(table.column_count(), 3);
        assert!(table.has_columns());
    }

    #[test]
    fn table_add_rows() {
        let table = Table::new()
            .add_column("name", "Name")
            .add_row("row1", vec!["Alice".to_string()])
            .add_row("row2", vec!["Bob".to_string()]);

        assert_eq!(table.row_count(), 2);
        assert!(table.has_rows());
    }

    #[test]
    fn table_select_row() {
        let mut table = Table::new()
            .selectable(true)
            .add_row("row1", vec!["Data".to_string()]);

        table.select_row("row1");
        assert!(table.is_row_selected("row1"));
    }

    #[test]
    fn table_deselect_row() {
        let mut table = Table::new()
            .selectable(true)
            .add_row("row1", vec!["Data".to_string()]);

        table.select_row("row1");
        assert!(table.is_row_selected("row1"));

        table.deselect_row("row1");
        assert!(!table.is_row_selected("row1"));
    }

    #[test]
    fn table_toggle_row() {
        let mut table = Table::new()
            .selectable(true)
            .add_row("row1", vec!["Data".to_string()]);

        table.toggle_row("row1");
        assert!(table.is_row_selected("row1"));

        table.toggle_row("row1");
        assert!(!table.is_row_selected("row1"));
    }

    #[test]
    fn table_clear_selection() {
        let mut table = Table::new()
            .selectable(true)
            .add_row("row1", vec!["Data1".to_string()])
            .add_row("row2", vec!["Data2".to_string()]);

        table.select_row("row1");
        table.select_row("row2");
        assert_eq!(table.get_selected_rows().len(), 2);

        table.clear_selection();
        assert_eq!(table.get_selected_rows().len(), 0);
    }

    #[test]
    fn table_cannot_select_disabled_row() {
        let mut table = Table::new()
            .selectable(true)
            .add_disabled_row("row1", vec!["Data".to_string()]);

        table.select_row("row1");
        assert!(!table.is_row_selected("row1"));
    }

    #[test]
    fn table_cannot_select_when_not_selectable() {
        let mut table = Table::new()
            .selectable(false)
            .add_row("row1", vec!["Data".to_string()]);

        table.select_row("row1");
        assert!(!table.is_row_selected("row1"));
    }

    #[test]
    fn table_sort_by_column() {
        let mut table = Table::new()
            .add_column("name", "Name")
            .add_column("age", "Age");

        table.sort_by_column("name");
        assert_eq!(table.get_sort_column(), Some("name".to_string()));
        assert_eq!(table.get_sort_direction(), SortDirection::Ascending);

        table.sort_by_column("name");
        assert_eq!(table.get_sort_direction(), SortDirection::Descending);
    }

    #[test]
    fn table_cannot_sort_non_sortable_column() {
        let mut table = Table::new()
            .add_column_object(TableColumn::new("name", "Name").sortable(false));

        table.sort_by_column("name");
        assert_eq!(table.get_sort_column(), None);
    }

    #[test]
    fn table_find_row() {
        let table = Table::new()
            .add_row("row1", vec!["Data1".to_string()])
            .add_row("row2", vec!["Data2".to_string()]);

        assert_eq!(table.find_row("row2"), Some(1));
        assert_eq!(table.find_row("nonexistent"), None);
    }

    #[test]
    fn table_get_row() {
        let table = Table::new()
            .add_row("row1", vec!["Data1".to_string()]);

        let row = table.get_row(0);
        assert!(row.is_some());
        assert_eq!(row.unwrap().id, "row1");
    }

    #[test]
    fn table_find_column() {
        let table = Table::new()
            .add_column("name", "Name")
            .add_column("age", "Age");

        assert_eq!(table.find_column("age"), Some(1));
        assert_eq!(table.find_column("nonexistent"), None);
    }

    #[test]
    fn table_get_column() {
        let table = Table::new()
            .add_column("name", "Name");

        let column = table.get_column(0);
        assert!(column.is_some());
        assert_eq!(column.unwrap().id, "name");
    }

    #[test]
    fn table_remove_row() {
        let mut table = Table::new()
            .selectable(true)
            .add_row("row1", vec!["Data1".to_string()])
            .add_row("row2", vec!["Data2".to_string()]);

        table.select_row("row1");
        assert_eq!(table.row_count(), 2);

        table.remove_row("row1");
        assert_eq!(table.row_count(), 1);
        assert!(!table.is_row_selected("row1"));
    }

    #[test]
    fn table_column_builder() {
        let column = TableColumn::new("name", "Name")
            .width(200.0)
            .sortable(false)
            .resizable(false)
            .align(ColumnAlign::Center);

        assert_eq!(column.width, Some(200.0));
        assert!(!column.sortable);
        assert!(!column.resizable);
        assert_eq!(column.align, ColumnAlign::Center);
    }

    #[test]
    fn table_row_with_metadata() {
        let row = TableRow::new("row1", vec!["Data".to_string()])
            .with_metadata("Important");

        assert_eq!(row.metadata, Some("Important".to_string()));
    }

    #[test]
    fn table_callbacks() {
        use std::sync::{Arc, Mutex};

        let clicked = Arc::new(Mutex::new(String::new()));
        let clicked_clone = clicked.clone();

        let sorted = Arc::new(Mutex::new((String::new(), SortDirection::Ascending)));
        let sorted_clone = sorted.clone();

        let mut table = Table::new()
            .add_column("name", "Name")
            .add_row("row1", vec!["Data".to_string()])
            .on_row_click(move |id| {
                *clicked_clone.lock().unwrap() = id.to_string();
            })
            .on_sort(move |col, dir| {
                *sorted_clone.lock().unwrap() = (col.to_string(), dir);
            });

        if let Some(ref callback) = table.on_row_click {
            callback("row1");
        }
        assert_eq!(*clicked.lock().unwrap(), "row1");

        table.sort_by_column("name");
        assert_eq!(sorted.lock().unwrap().0, "name");
    }

    #[test]
    fn table_builder_pattern() {
        let table = Table::new()
            .row_height(60.0)
            .header_height(70.0)
            .padding(20.0)
            .show_header(false)
            .striped(false)
            .hoverable(false)
            .selectable(false)
            .background_color(50, 50, 50, 255)
            .selected_color(255, 0, 0, 50);

        assert_eq!(table.row_height, 60.0);
        assert_eq!(table.header_height, 70.0);
        assert_eq!(table.padding, 20.0);
        assert!(!table.show_header);
        assert!(!table.striped);
        assert!(!table.hoverable);
        assert!(!table.selectable);
        assert_eq!(table.background_color, (50, 50, 50, 255));
        assert_eq!(table.selected_color, (255, 0, 0, 50));
    }

    #[test]
    fn table_build_creates_node() {
        let mut engine = LayoutEngine::new();
        let mut table = Table::new()
            .add_column("name", "Name")
            .add_row("row1", vec!["Data".to_string()]);

        let result = table.build(&mut engine);
        assert!(result.is_ok());
        assert!(table.node_id.is_some());
    }
}
