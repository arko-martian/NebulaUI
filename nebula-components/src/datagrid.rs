// DataGrid Component - Advanced data grid with filtering and sorting
// Essential for complex data display and manipulation

use nebula_core::layout::{LayoutEngine, NodeId};
use nebula_core::signal::Signal;
use super::table::{TableColumn, TableRow, ColumnAlign, SortDirection};

/// Filter operator
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FilterOperator {
    Equals,
    NotEquals,
    Contains,
    StartsWith,
    EndsWith,
    GreaterThan,
    LessThan,
}

/// Column filter
#[derive(Debug, Clone, PartialEq)]
pub struct ColumnFilter {
    pub column_id: String,
    pub operator: FilterOperator,
    pub value: String,
}

impl ColumnFilter {
    /// Create a new filter
    pub fn new(column_id: impl Into<String>, operator: FilterOperator, value: impl Into<String>) -> Self {
        Self {
            column_id: column_id.into(),
            operator,
            value: value.into(),
        }
    }

    /// Check if a cell value matches this filter
    pub fn matches(&self, cell_value: &str) -> bool {
        match self.operator {
            FilterOperator::Equals => cell_value == self.value,
            FilterOperator::NotEquals => cell_value != self.value,
            FilterOperator::Contains => cell_value.contains(&self.value),
            FilterOperator::StartsWith => cell_value.starts_with(&self.value),
            FilterOperator::EndsWith => cell_value.ends_with(&self.value),
            FilterOperator::GreaterThan => cell_value > self.value.as_str(),
            FilterOperator::LessThan => cell_value < self.value.as_str(),
        }
    }
}

/// DataGrid component - advanced data grid with filtering and sorting
/// 
/// # Example
/// ```
/// let mut grid = DataGrid::new()
///     .add_column("name", "Name")
///     .add_column("age", "Age")
///     .add_row("row1", vec!["Alice".to_string(), "30".to_string()])
///     .add_row("row2", vec!["Bob".to_string(), "25".to_string()])
///     .filterable(true)
///     .on_filter(|filters| println!("Filters: {:?}", filters));
/// ```
pub struct DataGrid {
    pub node_id: Option<NodeId>,
    pub columns: Vec<TableColumn>,
    pub rows: Vec<TableRow>,
    pub filtered_rows: Signal<Vec<usize>>, // Indices of visible rows
    pub selected_rows: Signal<Vec<String>>,
    pub sort_column: Signal<Option<String>>,
    pub sort_direction: Signal<SortDirection>,
    pub filters: Signal<Vec<ColumnFilter>>,
    pub page: Signal<usize>,
    pub page_size: usize,
    pub row_height: f32,
    pub header_height: f32,
    pub padding: f32,
    pub filterable: bool,
    pub paginated: bool,
    pub background_color: (u8, u8, u8, u8),
    pub header_color: (u8, u8, u8, u8),
    pub row_color: (u8, u8, u8, u8),
    pub alt_row_color: (u8, u8, u8, u8),
    pub selected_color: (u8, u8, u8, u8),
    pub text_color: (u8, u8, u8, u8),
    pub on_row_click: Option<Box<dyn Fn(&str)>>,
    pub on_sort: Option<Box<dyn Fn(&str, SortDirection)>>,
    pub on_filter: Option<Box<dyn Fn(&[ColumnFilter])>>,
    pub on_page_change: Option<Box<dyn Fn(usize)>>,
}

impl DataGrid {
    /// Create a new DataGrid component
    pub fn new() -> Self {
        Self {
            node_id: None,
            columns: Vec::new(),
            rows: Vec::new(),
            filtered_rows: Signal::new(Vec::new()),
            selected_rows: Signal::new(Vec::new()),
            sort_column: Signal::new(None),
            sort_direction: Signal::new(SortDirection::Ascending),
            filters: Signal::new(Vec::new()),
            page: Signal::new(0),
            page_size: 10,
            row_height: 48.0,
            header_height: 56.0,
            padding: 16.0,
            filterable: true,
            paginated: true,
            background_color: (255, 255, 255, 255),
            header_color: (250, 250, 250, 255),
            row_color: (255, 255, 255, 255),
            alt_row_color: (249, 249, 249, 255),
            selected_color: (59, 130, 246, 20),
            text_color: (0, 0, 0, 255),
            on_row_click: None,
            on_sort: None,
            on_filter: None,
            on_page_change: None,
        }
    }

    /// Enable or disable filtering
    pub fn filterable(mut self, filterable: bool) -> Self {
        self.filterable = filterable;
        self
    }

    /// Enable or disable pagination
    pub fn paginated(mut self, paginated: bool) -> Self {
        self.paginated = paginated;
        self
    }

    /// Set page size
    pub fn page_size(mut self, size: usize) -> Self {
        self.page_size = size.max(1);
        self
    }

    /// Set row height
    pub fn row_height(mut self, height: f32) -> Self {
        self.row_height = height;
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

    /// Set all columns
    pub fn columns(mut self, columns: Vec<TableColumn>) -> Self {
        self.columns = columns;
        self
    }

    /// Add a row
    pub fn add_row(mut self, id: impl Into<String>, cells: Vec<String>) -> Self {
        self.rows.push(TableRow::new(id, cells));
        self
    }

    /// Add a row object
    pub fn add_row_object(mut self, row: TableRow) -> Self {
        self.rows.push(row);
        self
    }

    /// Set all rows
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

    /// Set the filter callback
    pub fn on_filter<F>(mut self, callback: F) -> Self
    where
        F: Fn(&[ColumnFilter]) + 'static,
    {
        self.on_filter = Some(Box::new(callback));
        self
    }

    /// Set the page change callback
    pub fn on_page_change<F>(mut self, callback: F) -> Self
    where
        F: Fn(usize) + 'static,
    {
        self.on_page_change = Some(Box::new(callback));
        self
    }

    /// Add a filter
    pub fn add_filter(&mut self, filter: ColumnFilter) {
        if !self.filterable {
            return;
        }

        let mut filters = self.filters.get();
        filters.push(filter);
        self.filters.set(filters.clone());
        self.apply_filters();

        if let Some(ref callback) = self.on_filter {
            callback(&filters);
        }
    }

    /// Remove a filter by column ID
    pub fn remove_filter(&mut self, column_id: &str) {
        let mut filters = self.filters.get();
        filters.retain(|f| f.column_id != column_id);
        self.filters.set(filters.clone());
        self.apply_filters();

        if let Some(ref callback) = self.on_filter {
            callback(&filters);
        }
    }

    /// Clear all filters
    pub fn clear_filters(&mut self) {
        self.filters.set(Vec::new());
        self.apply_filters();

        if let Some(ref callback) = self.on_filter {
            callback(&[]);
        }
    }

    /// Apply filters to rows
    fn apply_filters(&mut self) {
        let filters = self.filters.get();
        
        if filters.is_empty() {
            // No filters - show all rows
            let all_indices: Vec<usize> = (0..self.rows.len()).collect();
            self.filtered_rows.set(all_indices);
            return;
        }

        let mut visible: Vec<usize> = Vec::new();

        for (idx, row) in self.rows.iter().enumerate() {
            let mut matches_all = true;

            for filter in &filters {
                if let Some(col_idx) = self.columns.iter().position(|c| c.id == filter.column_id) {
                    if let Some(cell_value) = row.cells.get(col_idx) {
                        if !filter.matches(cell_value) {
                            matches_all = false;
                            break;
                        }
                    }
                }
            }

            if matches_all {
                visible.push(idx);
            }
        }

        self.filtered_rows.set(visible);
    }

    /// Get filtered row count
    pub fn filtered_row_count(&self) -> usize {
        self.filtered_rows.get().len()
    }

    /// Get total row count
    pub fn total_row_count(&self) -> usize {
        self.rows.len()
    }

    /// Get current page
    pub fn get_page(&self) -> usize {
        self.page.get()
    }

    /// Get total pages
    pub fn total_pages(&self) -> usize {
        if !self.paginated || self.page_size == 0 {
            return 1;
        }
        (self.filtered_row_count() + self.page_size - 1) / self.page_size
    }

    /// Go to page
    pub fn go_to_page(&mut self, page: usize) {
        if !self.paginated {
            return;
        }

        let max_page = self.total_pages().saturating_sub(1);
        let new_page = page.min(max_page);
        self.page.set(new_page);

        if let Some(ref callback) = self.on_page_change {
            callback(new_page);
        }
    }

    /// Go to next page
    pub fn next_page(&mut self) {
        let current = self.page.get();
        self.go_to_page(current + 1);
    }

    /// Go to previous page
    pub fn previous_page(&mut self) {
        let current = self.page.get();
        if current > 0 {
            self.go_to_page(current - 1);
        }
    }

    /// Sort by column
    pub fn sort_by_column(&mut self, column_id: &str) {
        if let Some(column) = self.columns.iter().find(|c| c.id == column_id) {
            if !column.sortable {
                return;
            }

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

    /// Get active filters
    pub fn get_filters(&self) -> Vec<ColumnFilter> {
        self.filters.get()
    }

    /// Check if has filters
    pub fn has_filters(&self) -> bool {
        !self.filters.get().is_empty()
    }

    /// Build the data grid layout
    pub fn build(&mut self, engine: &mut LayoutEngine) -> Result<NodeId, String> {
        // Initialize filtered rows if empty
        if self.filtered_rows.get().is_empty() && !self.rows.is_empty() {
            let all_indices: Vec<usize> = (0..self.rows.len()).collect();
            self.filtered_rows.set(all_indices);
        }

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
            .map_err(|e| format!("Failed to create data grid node: {:?}", e))?;
        self.node_id = Some(node);

        Ok(node)
    }
}

impl Default for DataGrid {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn datagrid_starts_empty() {
        let grid = DataGrid::new();
        assert_eq!(grid.total_row_count(), 0);
        assert_eq!(grid.filtered_row_count(), 0);
    }

    #[test]
    fn datagrid_add_rows() {
        let grid = DataGrid::new()
            .add_column("name", "Name")
            .add_row("row1", vec!["Alice".to_string()])
            .add_row("row2", vec!["Bob".to_string()]);

        assert_eq!(grid.total_row_count(), 2);
    }

    #[test]
    fn datagrid_filter_equals() {
        let mut grid = DataGrid::new()
            .add_column("name", "Name")
            .add_row("row1", vec!["Alice".to_string()])
            .add_row("row2", vec!["Bob".to_string()])
            .filterable(true);

        grid.build(&mut LayoutEngine::new()).unwrap();
        
        grid.add_filter(ColumnFilter::new("name", FilterOperator::Equals, "Alice"));
        assert_eq!(grid.filtered_row_count(), 1);
    }

    #[test]
    fn datagrid_filter_contains() {
        let mut grid = DataGrid::new()
            .add_column("name", "Name")
            .add_row("row1", vec!["Alice".to_string()])
            .add_row("row2", vec!["Bob".to_string()])
            .add_row("row3", vec!["Charlie".to_string()])
            .filterable(true);

        grid.build(&mut LayoutEngine::new()).unwrap();
        
        grid.add_filter(ColumnFilter::new("name", FilterOperator::Contains, "li"));
        assert_eq!(grid.filtered_row_count(), 2); // Alice and Charlie
    }

    #[test]
    fn datagrid_multiple_filters() {
        let mut grid = DataGrid::new()
            .add_column("name", "Name")
            .add_column("age", "Age")
            .add_row("row1", vec!["Alice".to_string(), "30".to_string()])
            .add_row("row2", vec!["Bob".to_string(), "25".to_string()])
            .add_row("row3", vec!["Alice".to_string(), "25".to_string()])
            .filterable(true);

        grid.build(&mut LayoutEngine::new()).unwrap();
        
        grid.add_filter(ColumnFilter::new("name", FilterOperator::Equals, "Alice"));
        grid.add_filter(ColumnFilter::new("age", FilterOperator::Equals, "25"));
        assert_eq!(grid.filtered_row_count(), 1); // Only row3
    }

    #[test]
    fn datagrid_clear_filters() {
        let mut grid = DataGrid::new()
            .add_column("name", "Name")
            .add_row("row1", vec!["Alice".to_string()])
            .add_row("row2", vec!["Bob".to_string()])
            .filterable(true);

        grid.build(&mut LayoutEngine::new()).unwrap();
        
        grid.add_filter(ColumnFilter::new("name", FilterOperator::Equals, "Alice"));
        assert_eq!(grid.filtered_row_count(), 1);

        grid.clear_filters();
        assert_eq!(grid.filtered_row_count(), 2);
    }

    #[test]
    fn datagrid_remove_filter() {
        let mut grid = DataGrid::new()
            .add_column("name", "Name")
            .add_row("row1", vec!["Alice".to_string()])
            .add_row("row2", vec!["Bob".to_string()])
            .filterable(true);

        grid.build(&mut LayoutEngine::new()).unwrap();
        
        grid.add_filter(ColumnFilter::new("name", FilterOperator::Equals, "Alice"));
        assert_eq!(grid.filtered_row_count(), 1);

        grid.remove_filter("name");
        assert_eq!(grid.filtered_row_count(), 2);
    }

    #[test]
    fn datagrid_pagination() {
        let mut grid = DataGrid::new()
            .add_column("name", "Name")
            .paginated(true)
            .page_size(2);

        for i in 0..5 {
            grid = grid.add_row(format!("row{}", i), vec![format!("Name {}", i)]);
        }

        grid.build(&mut LayoutEngine::new()).unwrap();

        assert_eq!(grid.total_pages(), 3); // 5 rows / 2 per page = 3 pages
        assert_eq!(grid.get_page(), 0);
    }

    #[test]
    fn datagrid_next_page() {
        let mut grid = DataGrid::new()
            .add_column("name", "Name")
            .paginated(true)
            .page_size(2);

        for i in 0..5 {
            grid = grid.add_row(format!("row{}", i), vec![format!("Name {}", i)]);
        }

        grid.build(&mut LayoutEngine::new()).unwrap();

        grid.next_page();
        assert_eq!(grid.get_page(), 1);

        grid.next_page();
        assert_eq!(grid.get_page(), 2);

        grid.next_page();
        assert_eq!(grid.get_page(), 2); // Should stay at last page
    }

    #[test]
    fn datagrid_previous_page() {
        let mut grid = DataGrid::new()
            .add_column("name", "Name")
            .paginated(true)
            .page_size(2);

        for i in 0..5 {
            grid = grid.add_row(format!("row{}", i), vec![format!("Name {}", i)]);
        }

        grid.build(&mut LayoutEngine::new()).unwrap();

        grid.go_to_page(2);
        assert_eq!(grid.get_page(), 2);

        grid.previous_page();
        assert_eq!(grid.get_page(), 1);

        grid.previous_page();
        assert_eq!(grid.get_page(), 0);

        grid.previous_page();
        assert_eq!(grid.get_page(), 0); // Should stay at first page
    }

    #[test]
    fn datagrid_sort_by_column() {
        let mut grid = DataGrid::new()
            .add_column("name", "Name")
            .add_row("row1", vec!["Alice".to_string()]);

        grid.sort_by_column("name");
        assert_eq!(grid.sort_column.get(), Some("name".to_string()));
        assert_eq!(grid.sort_direction.get(), SortDirection::Ascending);

        grid.sort_by_column("name");
        assert_eq!(grid.sort_direction.get(), SortDirection::Descending);
    }

    #[test]
    fn datagrid_has_filters() {
        let mut grid = DataGrid::new()
            .add_column("name", "Name")
            .add_row("row1", vec!["Alice".to_string()])
            .filterable(true);

        grid.build(&mut LayoutEngine::new()).unwrap();

        assert!(!grid.has_filters());

        grid.add_filter(ColumnFilter::new("name", FilterOperator::Equals, "Alice"));
        assert!(grid.has_filters());
    }

    #[test]
    fn datagrid_get_filters() {
        let mut grid = DataGrid::new()
            .add_column("name", "Name")
            .add_row("row1", vec!["Alice".to_string()])
            .filterable(true);

        grid.build(&mut LayoutEngine::new()).unwrap();

        grid.add_filter(ColumnFilter::new("name", FilterOperator::Equals, "Alice"));
        let filters = grid.get_filters();
        assert_eq!(filters.len(), 1);
        assert_eq!(filters[0].column_id, "name");
    }

    #[test]
    fn filter_operators() {
        let filter_eq = ColumnFilter::new("col", FilterOperator::Equals, "test");
        assert!(filter_eq.matches("test"));
        assert!(!filter_eq.matches("Test"));

        let filter_contains = ColumnFilter::new("col", FilterOperator::Contains, "es");
        assert!(filter_contains.matches("test"));
        assert!(!filter_contains.matches("fail"));

        let filter_starts = ColumnFilter::new("col", FilterOperator::StartsWith, "te");
        assert!(filter_starts.matches("test"));
        assert!(!filter_starts.matches("atest"));

        let filter_ends = ColumnFilter::new("col", FilterOperator::EndsWith, "st");
        assert!(filter_ends.matches("test"));
        assert!(!filter_ends.matches("testa"));
    }

    #[test]
    fn datagrid_builder_pattern() {
        let grid = DataGrid::new()
            .filterable(true)
            .paginated(true)
            .page_size(20)
            .row_height(60.0);

        assert!(grid.filterable);
        assert!(grid.paginated);
        assert_eq!(grid.page_size, 20);
        assert_eq!(grid.row_height, 60.0);
    }

    #[test]
    fn datagrid_build_creates_node() {
        let mut engine = LayoutEngine::new();
        let mut grid = DataGrid::new()
            .add_column("name", "Name")
            .add_row("row1", vec!["Data".to_string()]);

        let result = grid.build(&mut engine);
        assert!(result.is_ok());
        assert!(grid.node_id.is_some());
    }
}
