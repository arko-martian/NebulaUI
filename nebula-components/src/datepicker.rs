// DatePicker Component - Date selection component
// Essential for date input in forms

use nebula_core::layout::{LayoutEngine, NodeId};
use nebula_core::signal::Signal;

/// Simple date representation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Date {
    pub year: i32,
    pub month: u8,  // 1-12
    pub day: u8,    // 1-31
}

impl Date {
    /// Create a new date
    pub fn new(year: i32, month: u8, day: u8) -> Self {
        Self { year, month, day }
    }

    /// Get today's date (placeholder - would use system time in real impl)
    pub fn today() -> Self {
        Self::new(2025, 11, 22)
    }

    /// Format as YYYY-MM-DD
    pub fn format(&self) -> String {
        format!("{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }

    /// Check if date is valid
    pub fn is_valid(&self) -> bool {
        if self.month < 1 || self.month > 12 {
            return false;
        }
        if self.day < 1 {
            return false;
        }
        let max_day = match self.month {
            2 => if self.is_leap_year() { 29 } else { 28 },
            4 | 6 | 9 | 11 => 30,
            _ => 31,
        };
        self.day <= max_day
    }

    /// Check if year is a leap year
    pub fn is_leap_year(&self) -> bool {
        (self.year % 4 == 0 && self.year % 100 != 0) || (self.year % 400 == 0)
    }
}

/// DatePicker component - date selection component
/// 
/// # Example
/// ```
/// let mut datepicker = DatePicker::new()
///     .selected_date(Date::today())
///     .min_date(Date::new(2020, 1, 1))
///     .max_date(Date::new(2030, 12, 31))
///     .on_change(|date| println!("Selected: {}", date.format()));
/// ```
pub struct DatePicker {
    pub node_id: Option<NodeId>,
    pub selected_date: Signal<Option<Date>>,
    pub min_date: Option<Date>,
    pub max_date: Option<Date>,
    pub disabled: bool,
    pub show_calendar: Signal<bool>,
    pub width: f32,
    pub height: f32,
    pub calendar_width: f32,
    pub calendar_height: f32,
    pub background_color: (u8, u8, u8, u8),
    pub selected_color: (u8, u8, u8, u8),
    pub today_color: (u8, u8, u8, u8),
    pub disabled_color: (u8, u8, u8, u8),
    pub on_change: Option<Box<dyn Fn(Date)>>,
}

impl DatePicker {
    /// Create a new DatePicker component
    pub fn new() -> Self {
        Self {
            node_id: None,
            selected_date: Signal::new(None),
            min_date: None,
            max_date: None,
            disabled: false,
            show_calendar: Signal::new(false),
            width: 200.0,
            height: 40.0,
            calendar_width: 280.0,
            calendar_height: 320.0,
            background_color: (255, 255, 255, 255),
            selected_color: (59, 130, 246, 255), // Blue
            today_color: (220, 220, 220, 255),
            disabled_color: (200, 200, 200, 255),
            on_change: None,
        }
    }

    /// Set the selected date
    pub fn selected_date(self, date: Date) -> Self {
        self.selected_date.set(Some(date));
        self
    }

    /// Set the minimum selectable date
    pub fn min_date(mut self, date: Date) -> Self {
        self.min_date = Some(date);
        self
    }

    /// Set the maximum selectable date
    pub fn max_date(mut self, date: Date) -> Self {
        self.max_date = Some(date);
        self
    }

    /// Set disabled state
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Set the width
    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    /// Set the height
    pub fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    /// Set the change callback
    pub fn on_change<F>(mut self, callback: F) -> Self
    where
        F: Fn(Date) + 'static,
    {
        self.on_change = Some(Box::new(callback));
        self
    }

    /// Select a date
    pub fn select_date(&mut self, date: Date) {
        if !self.disabled && date.is_valid() && self.is_date_selectable(&date) {
            self.selected_date.set(Some(date));
            if let Some(ref callback) = self.on_change {
                callback(date);
            }
        }
    }

    /// Get the selected date
    pub fn get_selected_date(&self) -> Option<Date> {
        self.selected_date.get()
    }

    /// Clear the selected date
    pub fn clear(&mut self) {
        self.selected_date.set(None);
    }

    /// Check if a date is selectable
    pub fn is_date_selectable(&self, date: &Date) -> bool {
        if let Some(min) = self.min_date {
            if date.year < min.year
                || (date.year == min.year && date.month < min.month)
                || (date.year == min.year && date.month == min.month && date.day < min.day)
            {
                return false;
            }
        }
        if let Some(max) = self.max_date {
            if date.year > max.year
                || (date.year == max.year && date.month > max.month)
                || (date.year == max.year && date.month == max.month && date.day > max.day)
            {
                return false;
            }
        }
        true
    }

    /// Show the calendar
    pub fn show(&mut self) {
        if !self.disabled {
            self.show_calendar.set(true);
        }
    }

    /// Hide the calendar
    pub fn hide(&mut self) {
        self.show_calendar.set(false);
    }

    /// Toggle the calendar
    pub fn toggle(&mut self) {
        if !self.disabled {
            self.show_calendar.set(!self.is_calendar_visible());
        }
    }

    /// Check if calendar is visible
    pub fn is_calendar_visible(&self) -> bool {
        self.show_calendar.get()
    }

    /// Check if has selected date
    pub fn has_selected_date(&self) -> bool {
        self.selected_date.get().is_some()
    }

    /// Build the datepicker layout
    pub fn build(&mut self, engine: &mut LayoutEngine) -> Result<NodeId, String> {
        let style = taffy::style::Style {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Length(self.width),
                height: taffy::style::Dimension::Length(self.height),
            },
            display: taffy::style::Display::Flex,
            align_items: Some(taffy::style::AlignItems::Center),
            ..Default::default()
        };

        let node = engine
            .new_leaf(style)
            .map_err(|e| format!("Failed to create datepicker node: {:?}", e))?;
        self.node_id = Some(node);

        Ok(node)
    }
}

impl Default for DatePicker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn datepicker_starts_empty() {
        let datepicker = DatePicker::new();
        assert!(!datepicker.has_selected_date());
    }

    #[test]
    fn datepicker_select_date() {
        let mut datepicker = DatePicker::new();
        let date = Date::new(2025, 11, 22);
        datepicker.select_date(date);
        assert_eq!(datepicker.get_selected_date(), Some(date));
    }

    #[test]
    fn datepicker_clear() {
        let mut datepicker = DatePicker::new().selected_date(Date::new(2025, 11, 22));
        datepicker.clear();
        assert!(!datepicker.has_selected_date());
    }

    #[test]
    fn datepicker_min_date_restriction() {
        let mut datepicker = DatePicker::new().min_date(Date::new(2025, 1, 1));
        
        datepicker.select_date(Date::new(2024, 12, 31));
        assert!(!datepicker.has_selected_date()); // Should not select
        
        datepicker.select_date(Date::new(2025, 1, 1));
        assert!(datepicker.has_selected_date()); // Should select
    }

    #[test]
    fn datepicker_max_date_restriction() {
        let mut datepicker = DatePicker::new().max_date(Date::new(2025, 12, 31));
        
        datepicker.select_date(Date::new(2026, 1, 1));
        assert!(!datepicker.has_selected_date()); // Should not select
        
        datepicker.select_date(Date::new(2025, 12, 31));
        assert!(datepicker.has_selected_date()); // Should select
    }

    #[test]
    fn datepicker_disabled_cannot_select() {
        let mut datepicker = DatePicker::new().disabled(true);
        datepicker.select_date(Date::new(2025, 11, 22));
        assert!(!datepicker.has_selected_date());
    }

    #[test]
    fn datepicker_show_hide_calendar() {
        let mut datepicker = DatePicker::new();
        assert!(!datepicker.is_calendar_visible());
        
        datepicker.show();
        assert!(datepicker.is_calendar_visible());
        
        datepicker.hide();
        assert!(!datepicker.is_calendar_visible());
    }

    #[test]
    fn datepicker_toggle_calendar() {
        let mut datepicker = DatePicker::new();
        datepicker.toggle();
        assert!(datepicker.is_calendar_visible());
        datepicker.toggle();
        assert!(!datepicker.is_calendar_visible());
    }

    #[test]
    fn datepicker_disabled_cannot_show_calendar() {
        let mut datepicker = DatePicker::new().disabled(true);
        datepicker.show();
        assert!(!datepicker.is_calendar_visible());
    }

    #[test]
    fn date_format() {
        let date = Date::new(2025, 11, 22);
        assert_eq!(date.format(), "2025-11-22");
    }

    #[test]
    fn date_validation() {
        assert!(Date::new(2025, 11, 22).is_valid());
        assert!(!Date::new(2025, 13, 1).is_valid()); // Invalid month
        assert!(!Date::new(2025, 2, 30).is_valid()); // Invalid day for February
        assert!(Date::new(2024, 2, 29).is_valid()); // Leap year
        assert!(!Date::new(2025, 2, 29).is_valid()); // Not a leap year
    }

    #[test]
    fn date_leap_year() {
        assert!(Date::new(2024, 1, 1).is_leap_year());
        assert!(!Date::new(2025, 1, 1).is_leap_year());
        assert!(Date::new(2000, 1, 1).is_leap_year());
        assert!(!Date::new(1900, 1, 1).is_leap_year());
    }

    #[test]
    fn datepicker_on_change_callback() {
        use std::sync::{Arc, Mutex};

        let changed = Arc::new(Mutex::new(None));
        let changed_clone = changed.clone();

        let mut datepicker = DatePicker::new().on_change(move |date| {
            *changed_clone.lock().unwrap() = Some(date);
        });

        let date = Date::new(2025, 11, 22);
        datepicker.select_date(date);
        assert_eq!(*changed.lock().unwrap(), Some(date));
    }

    #[test]
    fn datepicker_builder_pattern() {
        let datepicker = DatePicker::new()
            .selected_date(Date::new(2025, 11, 22))
            .min_date(Date::new(2020, 1, 1))
            .max_date(Date::new(2030, 12, 31))
            .disabled(true)
            .width(250.0)
            .height(45.0);

        assert_eq!(datepicker.get_selected_date(), Some(Date::new(2025, 11, 22)));
        assert_eq!(datepicker.min_date, Some(Date::new(2020, 1, 1)));
        assert_eq!(datepicker.max_date, Some(Date::new(2030, 12, 31)));
        assert!(datepicker.disabled);
        assert_eq!(datepicker.width, 250.0);
        assert_eq!(datepicker.height, 45.0);
    }

    #[test]
    fn datepicker_build_creates_node() {
        let mut engine = LayoutEngine::new();
        let mut datepicker = DatePicker::new();

        let result = datepicker.build(&mut engine);
        assert!(result.is_ok());
        assert!(datepicker.node_id.is_some());
    }
}
