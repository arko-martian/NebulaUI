// Pagination Component - Page navigation for paginated content
// Essential for navigating through large datasets

use nebula_core::layout::{LayoutEngine, NodeId};
use nebula_core::signal::Signal;

/// Pagination component - page navigation for paginated content
/// 
/// # Example
/// ```
/// let mut pagination = Pagination::new()
///     .total_pages(10)
///     .current_page(1)
///     .max_visible_pages(5)
///     .show_first_last(true)
///     .show_prev_next(true)
///     .on_page_change(|page| println!("Page changed to: {}", page));
/// ```
pub struct Pagination {
    pub node_id: Option<NodeId>,
    pub current_page: Signal<usize>,
    pub total_pages: usize,
    pub max_visible_pages: usize,
    pub show_first_last: bool,
    pub show_prev_next: bool,
    pub show_page_size: bool,
    pub page_sizes: Vec<usize>,
    pub current_page_size: Signal<usize>,
    pub height: f32,
    pub padding: f32,
    pub spacing: f32,
    pub button_size: f32,
    pub background_color: (u8, u8, u8, u8),
    pub active_color: (u8, u8, u8, u8),
    pub inactive_color: (u8, u8, u8, u8),
    pub hover_color: (u8, u8, u8, u8),
    pub text_color: (u8, u8, u8, u8),
    pub active_text_color: (u8, u8, u8, u8),
    pub disabled_color: (u8, u8, u8, u8),
    pub on_page_change: Option<Box<dyn Fn(usize)>>,
    pub on_page_size_change: Option<Box<dyn Fn(usize)>>,
}

impl Pagination {
    /// Create a new Pagination component
    pub fn new() -> Self {
        Self {
            node_id: None,
            current_page: Signal::new(1),
            total_pages: 1,
            max_visible_pages: 7,
            show_first_last: true,
            show_prev_next: true,
            show_page_size: false,
            page_sizes: vec![10, 25, 50, 100],
            current_page_size: Signal::new(10),
            height: 40.0,
            padding: 8.0,
            spacing: 4.0,
            button_size: 36.0,
            background_color: (255, 255, 255, 255),
            active_color: (59, 130, 246, 255), // Blue
            inactive_color: (245, 245, 245, 255),
            hover_color: (220, 220, 220, 255),
            text_color: (100, 100, 100, 255),
            active_text_color: (255, 255, 255, 255),
            disabled_color: (200, 200, 200, 255),
            on_page_change: None,
            on_page_size_change: None,
        }
    }

    /// Set the total number of pages
    pub fn total_pages(mut self, total: usize) -> Self {
        self.total_pages = total.max(1);
        self
    }

    /// Set the current page (1-indexed)
    pub fn current_page(self, page: usize) -> Self {
        self.current_page.set(page.max(1));
        self
    }

    /// Set the maximum number of visible page buttons
    pub fn max_visible_pages(mut self, max: usize) -> Self {
        self.max_visible_pages = max.max(3);
        self
    }

    /// Show or hide first/last buttons
    pub fn show_first_last(mut self, show: bool) -> Self {
        self.show_first_last = show;
        self
    }

    /// Show or hide prev/next buttons
    pub fn show_prev_next(mut self, show: bool) -> Self {
        self.show_prev_next = show;
        self
    }

    /// Show or hide page size selector
    pub fn show_page_size(mut self, show: bool) -> Self {
        self.show_page_size = show;
        self
    }

    /// Set available page sizes
    pub fn page_sizes(mut self, sizes: Vec<usize>) -> Self {
        self.page_sizes = sizes;
        self
    }

    /// Set current page size
    pub fn page_size(self, size: usize) -> Self {
        self.current_page_size.set(size);
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

    /// Set the spacing between buttons
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }

    /// Set the button size
    pub fn button_size(mut self, size: f32) -> Self {
        self.button_size = size;
        self
    }

    /// Set the background color
    pub fn background_color(mut self, r: u8, g: u8, b: u8, a: u8) -> Self {
        self.background_color = (r, g, b, a);
        self
    }

    /// Set the active page color
    pub fn active_color(mut self, r: u8, g: u8, b: u8, a: u8) -> Self {
        self.active_color = (r, g, b, a);
        self
    }

    /// Set the inactive page color
    pub fn inactive_color(mut self, r: u8, g: u8, b: u8, a: u8) -> Self {
        self.inactive_color = (r, g, b, a);
        self
    }

    /// Set the hover color
    pub fn hover_color(mut self, r: u8, g: u8, b: u8, a: u8) -> Self {
        self.hover_color = (r, g, b, a);
        self
    }

    /// Set the text color
    pub fn text_color(mut self, r: u8, g: u8, b: u8, a: u8) -> Self {
        self.text_color = (r, g, b, a);
        self
    }

    /// Set the active text color
    pub fn active_text_color(mut self, r: u8, g: u8, b: u8, a: u8) -> Self {
        self.active_text_color = (r, g, b, a);
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

    /// Set the page size change callback
    pub fn on_page_size_change<F>(mut self, callback: F) -> Self
    where
        F: Fn(usize) + 'static,
    {
        self.on_page_size_change = Some(Box::new(callback));
        self
    }

    /// Go to a specific page
    pub fn go_to_page(&mut self, page: usize) {
        if page >= 1 && page <= self.total_pages {
            self.current_page.set(page);
            if let Some(ref callback) = self.on_page_change {
                callback(page);
            }
        }
    }

    /// Go to the next page
    pub fn next_page(&mut self) {
        let current = self.current_page.get();
        if current < self.total_pages {
            self.go_to_page(current + 1);
        }
    }

    /// Go to the previous page
    pub fn prev_page(&mut self) {
        let current = self.current_page.get();
        if current > 1 {
            self.go_to_page(current - 1);
        }
    }

    /// Go to the first page
    pub fn first_page(&mut self) {
        self.go_to_page(1);
    }

    /// Go to the last page
    pub fn last_page(&mut self) {
        self.go_to_page(self.total_pages);
    }

    /// Change the page size
    pub fn change_page_size(&mut self, size: usize) {
        if self.page_sizes.contains(&size) {
            self.current_page_size.set(size);
            if let Some(ref callback) = self.on_page_size_change {
                callback(size);
            }
        }
    }

    /// Get the current page
    pub fn get_current_page(&self) -> usize {
        self.current_page.get()
    }

    /// Get the current page size
    pub fn get_current_page_size(&self) -> usize {
        self.current_page_size.get()
    }

    /// Check if on first page
    pub fn is_first_page(&self) -> bool {
        self.current_page.get() == 1
    }

    /// Check if on last page
    pub fn is_last_page(&self) -> bool {
        self.current_page.get() == self.total_pages
    }

    /// Check if can go to previous page
    pub fn can_go_prev(&self) -> bool {
        !self.is_first_page()
    }

    /// Check if can go to next page
    pub fn can_go_next(&self) -> bool {
        !self.is_last_page()
    }

    /// Get visible page numbers
    pub fn get_visible_pages(&self) -> Vec<usize> {
        let current = self.current_page.get();
        let total = self.total_pages;
        let max_visible = self.max_visible_pages;

        if total <= max_visible {
            // Show all pages
            (1..=total).collect()
        } else {
            // Calculate range around current page
            let half = max_visible / 2;
            let mut start = current.saturating_sub(half);
            let mut end = current + half;

            // Adjust if at boundaries
            if start < 1 {
                start = 1;
                end = max_visible.min(total);
            }
            if end > total {
                end = total;
                start = (total - max_visible + 1).max(1);
            }

            (start..=end).collect()
        }
    }

    /// Check if page numbers are truncated at start
    pub fn is_truncated_start(&self) -> bool {
        let visible = self.get_visible_pages();
        !visible.is_empty() && visible[0] > 1
    }

    /// Check if page numbers are truncated at end
    pub fn is_truncated_end(&self) -> bool {
        let visible = self.get_visible_pages();
        !visible.is_empty() && visible[visible.len() - 1] < self.total_pages
    }

    /// Build the pagination layout
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
            .map_err(|e| format!("Failed to create pagination node: {:?}", e))?;
        self.node_id = Some(node);

        Ok(node)
    }
}

impl Default for Pagination {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pagination_starts_at_page_1() {
        let pagination = Pagination::new();
        assert_eq!(pagination.get_current_page(), 1);
        assert_eq!(pagination.total_pages, 1);
    }

    #[test]
    fn pagination_set_total_pages() {
        let pagination = Pagination::new().total_pages(10);
        assert_eq!(pagination.total_pages, 10);
    }

    #[test]
    fn pagination_set_current_page() {
        let pagination = Pagination::new().total_pages(10).current_page(5);
        assert_eq!(pagination.get_current_page(), 5);
    }

    #[test]
    fn pagination_go_to_page() {
        use std::sync::{Arc, Mutex};

        let changed = Arc::new(Mutex::new(0));
        let changed_clone = changed.clone();

        let mut pagination = Pagination::new()
            .total_pages(10)
            .on_page_change(move |page| {
                *changed_clone.lock().unwrap() = page;
            });

        pagination.go_to_page(5);
        assert_eq!(pagination.get_current_page(), 5);
        assert_eq!(*changed.lock().unwrap(), 5);
    }

    #[test]
    fn pagination_next_page() {
        let mut pagination = Pagination::new().total_pages(10).current_page(5);
        pagination.next_page();
        assert_eq!(pagination.get_current_page(), 6);
    }

    #[test]
    fn pagination_prev_page() {
        let mut pagination = Pagination::new().total_pages(10).current_page(5);
        pagination.prev_page();
        assert_eq!(pagination.get_current_page(), 4);
    }

    #[test]
    fn pagination_first_page() {
        let mut pagination = Pagination::new().total_pages(10).current_page(5);
        pagination.first_page();
        assert_eq!(pagination.get_current_page(), 1);
    }

    #[test]
    fn pagination_last_page() {
        let mut pagination = Pagination::new().total_pages(10).current_page(5);
        pagination.last_page();
        assert_eq!(pagination.get_current_page(), 10);
    }

    #[test]
    fn pagination_cannot_go_before_first() {
        let mut pagination = Pagination::new().total_pages(10).current_page(1);
        pagination.prev_page();
        assert_eq!(pagination.get_current_page(), 1);
    }

    #[test]
    fn pagination_cannot_go_after_last() {
        let mut pagination = Pagination::new().total_pages(10).current_page(10);
        pagination.next_page();
        assert_eq!(pagination.get_current_page(), 10);
    }

    #[test]
    fn pagination_is_first_page() {
        let pagination = Pagination::new().total_pages(10).current_page(1);
        assert!(pagination.is_first_page());
        assert!(!pagination.is_last_page());
    }

    #[test]
    fn pagination_is_last_page() {
        let pagination = Pagination::new().total_pages(10).current_page(10);
        assert!(pagination.is_last_page());
        assert!(!pagination.is_first_page());
    }

    #[test]
    fn pagination_can_go_prev() {
        let pagination = Pagination::new().total_pages(10).current_page(5);
        assert!(pagination.can_go_prev());
    }

    #[test]
    fn pagination_can_go_next() {
        let pagination = Pagination::new().total_pages(10).current_page(5);
        assert!(pagination.can_go_next());
    }

    #[test]
    fn pagination_visible_pages_all() {
        let pagination = Pagination::new().total_pages(5).max_visible_pages(7);
        let visible = pagination.get_visible_pages();
        assert_eq!(visible, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn pagination_visible_pages_truncated() {
        let pagination = Pagination::new()
            .total_pages(20)
            .current_page(10)
            .max_visible_pages(5);
        let visible = pagination.get_visible_pages();
        assert_eq!(visible.len(), 5);
        assert!(visible.contains(&10));
    }

    #[test]
    fn pagination_is_truncated_start() {
        let pagination = Pagination::new()
            .total_pages(20)
            .current_page(10)
            .max_visible_pages(5);
        assert!(pagination.is_truncated_start());
    }

    #[test]
    fn pagination_is_truncated_end() {
        let pagination = Pagination::new()
            .total_pages(20)
            .current_page(10)
            .max_visible_pages(5);
        assert!(pagination.is_truncated_end());
    }

    #[test]
    fn pagination_page_size() {
        let pagination = Pagination::new().page_size(25);
        assert_eq!(pagination.get_current_page_size(), 25);
    }

    #[test]
    fn pagination_change_page_size() {
        use std::sync::{Arc, Mutex};

        let changed = Arc::new(Mutex::new(0));
        let changed_clone = changed.clone();

        let mut pagination = Pagination::new()
            .page_sizes(vec![10, 25, 50])
            .on_page_size_change(move |size| {
                *changed_clone.lock().unwrap() = size;
            });

        pagination.change_page_size(25);
        assert_eq!(pagination.get_current_page_size(), 25);
        assert_eq!(*changed.lock().unwrap(), 25);
    }

    #[test]
    fn pagination_cannot_change_to_invalid_page_size() {
        let mut pagination = Pagination::new().page_sizes(vec![10, 25, 50]);
        pagination.change_page_size(100); // Not in list
        assert_eq!(pagination.get_current_page_size(), 10); // Should stay at default
    }

    #[test]
    fn pagination_builder_pattern() {
        let pagination = Pagination::new()
            .total_pages(20)
            .current_page(5)
            .max_visible_pages(5)
            .show_first_last(false)
            .show_prev_next(false)
            .show_page_size(true)
            .page_sizes(vec![10, 25, 50])
            .page_size(25)
            .height(50.0)
            .padding(12.0)
            .spacing(8.0)
            .button_size(40.0);

        assert_eq!(pagination.total_pages, 20);
        assert_eq!(pagination.get_current_page(), 5);
        assert_eq!(pagination.max_visible_pages, 5);
        assert!(!pagination.show_first_last);
        assert!(!pagination.show_prev_next);
        assert!(pagination.show_page_size);
        assert_eq!(pagination.get_current_page_size(), 25);
        assert_eq!(pagination.height, 50.0);
        assert_eq!(pagination.padding, 12.0);
        assert_eq!(pagination.spacing, 8.0);
        assert_eq!(pagination.button_size, 40.0);
    }

    #[test]
    fn pagination_build_creates_node() {
        let mut engine = LayoutEngine::new();
        let mut pagination = Pagination::new().total_pages(10);

        let result = pagination.build(&mut engine);
        assert!(result.is_ok());
        assert!(pagination.node_id.is_some());
    }

    #[test]
    fn pagination_callbacks() {
        use std::sync::{Arc, Mutex};

        let page_changed = Arc::new(Mutex::new(0));
        let page_changed_clone = page_changed.clone();

        let size_changed = Arc::new(Mutex::new(0));
        let size_changed_clone = size_changed.clone();

        let mut pagination = Pagination::new()
            .total_pages(10)
            .page_sizes(vec![10, 25, 50])
            .on_page_change(move |page| {
                *page_changed_clone.lock().unwrap() = page;
            })
            .on_page_size_change(move |size| {
                *size_changed_clone.lock().unwrap() = size;
            });

        pagination.go_to_page(5);
        assert_eq!(*page_changed.lock().unwrap(), 5);

        pagination.change_page_size(25);
        assert_eq!(*size_changed.lock().unwrap(), 25);
    }
}
