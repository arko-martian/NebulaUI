// Calendar Component - Full calendar for date selection
// Essential for date pickers and scheduling

use nebula_core::layout::{LayoutEngine, NodeId};
use nebula_core::signal::Signal;

/// Simple date representation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CalendarDate {
    pub year: i32,
    pub month: u8,  // 1-12
    pub day: u8,    // 1-31
}

impl CalendarDate {
    /// Create a new date
    pub fn new(year: i32, month: u8, day: u8) -> Self {
        Self { year, month, day }
    }

    /// Get today's date (simplified - would use system time in real impl)
    pub fn today() -> Self {
        Self::new(2025, 11, 22)
    }

    /// Check if this is today
    pub fn is_today(&self) -> bool {
        *self == Self::today()
    }

    /// Get the first day of the month
    pub fn first_of_month(&self) -> Self {
        Self::new(self.year, self.month, 1)
    }

    /// Get days in month
    pub fn days_in_month(&self) -> u8 {
        match self.month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => {
                if self.is_leap_year() {
                    29
                } else {
                    28
                }
            }
            _ => 30,
        }
    }

    /// Check if leap year
    pub fn is_leap_year(&self) -> bool {
        (self.year % 4 == 0 && self.year % 100 != 0) || (self.year % 400 == 0)
    }

    /// Add months
    pub fn add_months(&self, months: i32) -> Self {
        let mut year = self.year;
        let mut month = self.month as i32 + months;

        while month > 12 {
            month -= 12;
            year += 1;
        }
        while month < 1 {
            month += 12;
            year -= 1;
        }

        let max_day = Self::new(year, month as u8, 1).days_in_month();
        let day = self.day.min(max_day);

        Self::new(year, month as u8, day)
    }

    /// Format as string
    pub fn format(&self) -> String {
        format!("{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
}

/// Calendar view mode
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CalendarView {
    Month,
    Year,
    Decade,
}

/// Calendar component - full calendar for date selection
/// 
/// # Example
/// ```
/// let mut calendar = Calendar::new()
///     .selected_date(CalendarDate::today())
///     .on_select(|date| println!("Selected: {}", date.format()));
/// ```
pub struct Calendar {
    pub node_id: Option<NodeId>,
    pub current_date: Signal<CalendarDate>,
    pub selected_date: Signal<Option<CalendarDate>>,
    pub view: Signal<CalendarView>,
    pub min_date: Option<CalendarDate>,
    pub max_date: Option<CalendarDate>,
    pub disabled_dates: Vec<CalendarDate>,
    pub show_week_numbers: bool,
    pub first_day_of_week: u8, // 0 = Sunday, 1 = Monday
    pub cell_size: f32,
    pub background_color: (u8, u8, u8, u8),
    pub header_color: (u8, u8, u8, u8),
    pub today_color: (u8, u8, u8, u8),
    pub selected_color: (u8, u8, u8, u8),
    pub disabled_color: (u8, u8, u8, u8),
    pub text_color: (u8, u8, u8, u8),
    pub on_select: Option<Box<dyn Fn(CalendarDate)>>,
    pub on_month_change: Option<Box<dyn Fn(i32, u8)>>,
}

impl Calendar {
    /// Create a new Calendar component
    pub fn new() -> Self {
        let today = CalendarDate::today();
        Self {
            node_id: None,
            current_date: Signal::new(today),
            selected_date: Signal::new(None),
            view: Signal::new(CalendarView::Month),
            min_date: None,
            max_date: None,
            disabled_dates: Vec::new(),
            show_week_numbers: false,
            first_day_of_week: 0, // Sunday
            cell_size: 40.0,
            background_color: (255, 255, 255, 255),
            header_color: (250, 250, 250, 255),
            today_color: (59, 130, 246, 50), // Light blue
            selected_color: (59, 130, 246, 255), // Blue
            disabled_color: (200, 200, 200, 255),
            text_color: (0, 0, 0, 255),
            on_select: None,
            on_month_change: None,
        }
    }

    /// Set the selected date
    pub fn selected_date(mut self, date: CalendarDate) -> Self {
        self.selected_date.set(Some(date));
        self.current_date.set(date);
        self
    }

    /// Set minimum date
    pub fn min_date(mut self, date: CalendarDate) -> Self {
        self.min_date = Some(date);
        self
    }

    /// Set maximum date
    pub fn max_date(mut self, date: CalendarDate) -> Self {
        self.max_date = Some(date);
        self
    }

    /// Add disabled date
    pub fn add_disabled_date(mut self, date: CalendarDate) -> Self {
        self.disabled_dates.push(date);
        self
    }

    /// Show week numbers
    pub fn show_week_numbers(mut self, show: bool) -> Self {
        self.show_week_numbers = show;
        self
    }

    /// Set first day of week (0 = Sunday, 1 = Monday)
    pub fn first_day_of_week(mut self, day: u8) -> Self {
        self.first_day_of_week = day.min(6);
        self
    }

    /// Set cell size
    pub fn cell_size(mut self, size: f32) -> Self {
        self.cell_size = size;
        self
    }

    /// Set selected color
    pub fn selected_color(mut self, r: u8, g: u8, b: u8, a: u8) -> Self {
        self.selected_color = (r, g, b, a);
        self
    }

    /// Set today color
    pub fn today_color(mut self, r: u8, g: u8, b: u8, a: u8) -> Self {
        self.today_color = (r, g, b, a);
        self
    }

    /// Set the select callback
    pub fn on_select<F>(mut self, callback: F) -> Self
    where
        F: Fn(CalendarDate) + 'static,
    {
        self.on_select = Some(Box::new(callback));
        self
    }

    /// Set the month change callback
    pub fn on_month_change<F>(mut self, callback: F) -> Self
    where
        F: Fn(i32, u8) + 'static,
    {
        self.on_month_change = Some(Box::new(callback));
        self
    }

    /// Select a date
    pub fn select_date(&mut self, date: CalendarDate) {
        if self.is_date_disabled(&date) {
            return;
        }

        self.selected_date.set(Some(date));
        self.current_date.set(date);

        if let Some(ref callback) = self.on_select {
            callback(date);
        }
    }

    /// Clear selection
    pub fn clear_selection(&mut self) {
        self.selected_date.set(None);
    }

    /// Get selected date
    pub fn get_selected_date(&self) -> Option<CalendarDate> {
        self.selected_date.get()
    }

    /// Get current viewing date
    pub fn get_current_date(&self) -> CalendarDate {
        self.current_date.get()
    }

    /// Go to next month
    pub fn next_month(&mut self) {
        let current = self.current_date.get();
        let next = current.add_months(1);
        self.current_date.set(next);

        if let Some(ref callback) = self.on_month_change {
            callback(next.year, next.month);
        }
    }

    /// Go to previous month
    pub fn previous_month(&mut self) {
        let current = self.current_date.get();
        let prev = current.add_months(-1);
        self.current_date.set(prev);

        if let Some(ref callback) = self.on_month_change {
            callback(prev.year, prev.month);
        }
    }

    /// Go to today
    pub fn go_to_today(&mut self) {
        let today = CalendarDate::today();
        self.current_date.set(today);
    }

    /// Check if date is disabled
    pub fn is_date_disabled(&self, date: &CalendarDate) -> bool {
        if let Some(min) = self.min_date {
            if date.year < min.year || (date.year == min.year && date.month < min.month) || 
               (date.year == min.year && date.month == min.month && date.day < min.day) {
                return true;
            }
        }

        if let Some(max) = self.max_date {
            if date.year > max.year || (date.year == max.year && date.month > max.month) ||
               (date.year == max.year && date.month == max.month && date.day > max.day) {
                return true;
            }
        }

        self.disabled_dates.contains(date)
    }

    /// Check if date is selected
    pub fn is_date_selected(&self, date: &CalendarDate) -> bool {
        self.selected_date.get().as_ref() == Some(date)
    }

    /// Get current view
    pub fn get_view(&self) -> CalendarView {
        self.view.get()
    }

    /// Set view
    pub fn set_view(&mut self, view: CalendarView) {
        self.view.set(view);
    }

    /// Build the calendar layout
    pub fn build(&mut self, engine: &mut LayoutEngine) -> Result<NodeId, String> {
        let width = self.cell_size * 7.0; // 7 days
        let height = self.cell_size * 8.0; // Header + 6 weeks max

        let style = taffy::style::Style {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Length(width),
                height: taffy::style::Dimension::Length(height),
            },
            display: taffy::style::Display::Flex,
            flex_direction: taffy::style::FlexDirection::Column,
            ..Default::default()
        };

        let node = engine
            .new_leaf(style)
            .map_err(|e| format!("Failed to create calendar node: {:?}", e))?;
        self.node_id = Some(node);

        Ok(node)
    }
}

impl Default for Calendar {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calendar_starts_with_today() {
        let calendar = Calendar::new();
        let current = calendar.get_current_date();
        assert_eq!(current, CalendarDate::today());
    }

    #[test]
    fn calendar_select_date() {
        let mut calendar = Calendar::new();
        let date = CalendarDate::new(2025, 12, 25);
        
        calendar.select_date(date);
        assert_eq!(calendar.get_selected_date(), Some(date));
    }

    #[test]
    fn calendar_clear_selection() {
        let mut calendar = Calendar::new()
            .selected_date(CalendarDate::new(2025, 12, 25));
        
        calendar.clear_selection();
        assert_eq!(calendar.get_selected_date(), None);
    }

    #[test]
    fn calendar_next_month() {
        let mut calendar = Calendar::new();
        calendar.current_date.set(CalendarDate::new(2025, 11, 15));
        
        calendar.next_month();
        let current = calendar.get_current_date();
        assert_eq!(current.year, 2025);
        assert_eq!(current.month, 12);
    }

    #[test]
    fn calendar_previous_month() {
        let mut calendar = Calendar::new();
        calendar.current_date.set(CalendarDate::new(2025, 11, 15));
        
        calendar.previous_month();
        let current = calendar.get_current_date();
        assert_eq!(current.year, 2025);
        assert_eq!(current.month, 10);
    }

    #[test]
    fn calendar_go_to_today() {
        let mut calendar = Calendar::new();
        calendar.current_date.set(CalendarDate::new(2020, 1, 1));
        
        calendar.go_to_today();
        assert_eq!(calendar.get_current_date(), CalendarDate::today());
    }

    #[test]
    fn calendar_min_date() {
        let mut calendar = Calendar::new()
            .min_date(CalendarDate::new(2025, 11, 20));
        
        let before = CalendarDate::new(2025, 11, 15);
        let after = CalendarDate::new(2025, 11, 25);
        
        assert!(calendar.is_date_disabled(&before));
        assert!(!calendar.is_date_disabled(&after));
    }

    #[test]
    fn calendar_max_date() {
        let mut calendar = Calendar::new()
            .max_date(CalendarDate::new(2025, 11, 30));
        
        let before = CalendarDate::new(2025, 11, 25);
        let after = CalendarDate::new(2025, 12, 5);
        
        assert!(!calendar.is_date_disabled(&before));
        assert!(calendar.is_date_disabled(&after));
    }

    #[test]
    fn calendar_disabled_dates() {
        let mut calendar = Calendar::new()
            .add_disabled_date(CalendarDate::new(2025, 11, 25));
        
        let disabled = CalendarDate::new(2025, 11, 25);
        let enabled = CalendarDate::new(2025, 11, 26);
        
        assert!(calendar.is_date_disabled(&disabled));
        assert!(!calendar.is_date_disabled(&enabled));
    }

    #[test]
    fn calendar_cannot_select_disabled() {
        let mut calendar = Calendar::new()
            .add_disabled_date(CalendarDate::new(2025, 11, 25));
        
        calendar.select_date(CalendarDate::new(2025, 11, 25));
        assert_eq!(calendar.get_selected_date(), None);
    }

    #[test]
    fn calendar_is_date_selected() {
        let mut calendar = Calendar::new();
        let date = CalendarDate::new(2025, 12, 25);
        
        calendar.select_date(date);
        assert!(calendar.is_date_selected(&date));
        assert!(!calendar.is_date_selected(&CalendarDate::new(2025, 12, 26)));
    }

    #[test]
    fn calendar_view_modes() {
        let mut calendar = Calendar::new();
        assert_eq!(calendar.get_view(), CalendarView::Month);
        
        calendar.set_view(CalendarView::Year);
        assert_eq!(calendar.get_view(), CalendarView::Year);
    }

    #[test]
    fn calendar_date_days_in_month() {
        assert_eq!(CalendarDate::new(2025, 1, 1).days_in_month(), 31);
        assert_eq!(CalendarDate::new(2025, 2, 1).days_in_month(), 28);
        assert_eq!(CalendarDate::new(2024, 2, 1).days_in_month(), 29); // Leap year
        assert_eq!(CalendarDate::new(2025, 4, 1).days_in_month(), 30);
    }

    #[test]
    fn calendar_date_is_leap_year() {
        assert!(CalendarDate::new(2024, 1, 1).is_leap_year());
        assert!(!CalendarDate::new(2025, 1, 1).is_leap_year());
        assert!(CalendarDate::new(2000, 1, 1).is_leap_year());
        assert!(!CalendarDate::new(1900, 1, 1).is_leap_year());
    }

    #[test]
    fn calendar_date_add_months() {
        let date = CalendarDate::new(2025, 11, 15);
        
        let next = date.add_months(1);
        assert_eq!(next, CalendarDate::new(2025, 12, 15));
        
        let prev = date.add_months(-1);
        assert_eq!(prev, CalendarDate::new(2025, 10, 15));
        
        let next_year = date.add_months(2);
        assert_eq!(next_year, CalendarDate::new(2026, 1, 15));
    }

    #[test]
    fn calendar_date_format() {
        let date = CalendarDate::new(2025, 11, 22);
        assert_eq!(date.format(), "2025-11-22");
    }

    #[test]
    fn calendar_callbacks() {
        use std::sync::{Arc, Mutex};

        let selected = Arc::new(Mutex::new(None));
        let selected_clone = selected.clone();

        let month_changed = Arc::new(Mutex::new((0, 0)));
        let month_changed_clone = month_changed.clone();

        let mut calendar = Calendar::new()
            .on_select(move |date| {
                *selected_clone.lock().unwrap() = Some(date);
            })
            .on_month_change(move |year, month| {
                *month_changed_clone.lock().unwrap() = (year, month);
            });

        let date = CalendarDate::new(2025, 12, 25);
        calendar.select_date(date);
        assert_eq!(*selected.lock().unwrap(), Some(date));

        calendar.next_month();
        assert_eq!(*month_changed.lock().unwrap(), (2026, 1)); // Next month after Dec 2025 is Jan 2026
    }

    #[test]
    fn calendar_builder_pattern() {
        let calendar = Calendar::new()
            .selected_date(CalendarDate::new(2025, 12, 25))
            .min_date(CalendarDate::new(2025, 1, 1))
            .max_date(CalendarDate::new(2025, 12, 31))
            .show_week_numbers(true)
            .first_day_of_week(1)
            .cell_size(50.0)
            .selected_color(255, 0, 0, 255)
            .today_color(0, 255, 0, 100);

        assert_eq!(calendar.get_selected_date(), Some(CalendarDate::new(2025, 12, 25)));
        assert_eq!(calendar.min_date, Some(CalendarDate::new(2025, 1, 1)));
        assert_eq!(calendar.max_date, Some(CalendarDate::new(2025, 12, 31)));
        assert!(calendar.show_week_numbers);
        assert_eq!(calendar.first_day_of_week, 1);
        assert_eq!(calendar.cell_size, 50.0);
        assert_eq!(calendar.selected_color, (255, 0, 0, 255));
        assert_eq!(calendar.today_color, (0, 255, 0, 100));
    }

    #[test]
    fn calendar_build_creates_node() {
        let mut engine = LayoutEngine::new();
        let mut calendar = Calendar::new();

        let result = calendar.build(&mut engine);
        assert!(result.is_ok());
        assert!(calendar.node_id.is_some());
    }
}
