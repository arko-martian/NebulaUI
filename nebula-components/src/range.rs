// Range Component - Dual-handle range slider for selecting a range
// Essential for price ranges, date ranges, etc.

use nebula_core::layout::{LayoutEngine, NodeId};
use nebula_core::signal::Signal;

/// Range component - dual-handle range slider for selecting a range
/// 
/// # Example
/// ```
/// let mut range = Range::new()
///     .min(0.0)
///     .max(100.0)
///     .start_value(25.0)
///     .end_value(75.0)
///     .on_change(|start, end| println!("Range: {} - {}", start, end));
/// ```
pub struct Range {
    pub node_id: Option<NodeId>,
    pub start_value: Signal<f32>,
    pub end_value: Signal<f32>,
    pub min: f32,
    pub max: f32,
    pub step: Option<f32>,
    pub disabled: bool,
    pub width: f32,
    pub height: f32,
    pub track_height: f32,
    pub thumb_size: f32,
    pub track_color: (u8, u8, u8, u8),
    pub track_fill_color: (u8, u8, u8, u8),
    pub thumb_color: (u8, u8, u8, u8),
    pub thumb_hover_color: (u8, u8, u8, u8),
    pub disabled_color: (u8, u8, u8, u8),
    pub show_values: bool,
    pub on_change: Option<Box<dyn Fn(f32, f32)>>,
    pub on_change_end: Option<Box<dyn Fn(f32, f32)>>,
}

impl Range {
    /// Create a new Range component
    pub fn new() -> Self {
        Self {
            node_id: None,
            start_value: Signal::new(0.0),
            end_value: Signal::new(100.0),
            min: 0.0,
            max: 100.0,
            step: None,
            disabled: false,
            width: 200.0,
            height: 40.0,
            track_height: 4.0,
            thumb_size: 20.0,
            track_color: (220, 220, 220, 255),
            track_fill_color: (59, 130, 246, 255), // Blue
            thumb_color: (255, 255, 255, 255),
            thumb_hover_color: (245, 245, 245, 255),
            disabled_color: (200, 200, 200, 255),
            show_values: false,
            on_change: None,
            on_change_end: None,
        }
    }

    /// Set the minimum value
    pub fn min(mut self, min: f32) -> Self {
        self.min = min;
        self
    }

    /// Set the maximum value
    pub fn max(mut self, max: f32) -> Self {
        self.max = max;
        self
    }

    /// Set the start value
    pub fn start_value(self, value: f32) -> Self {
        self.start_value.set(value.clamp(self.min, self.max));
        self
    }

    /// Set the end value
    pub fn end_value(self, value: f32) -> Self {
        self.end_value.set(value.clamp(self.min, self.max));
        self
    }

    /// Set the step increment
    pub fn step(mut self, step: f32) -> Self {
        self.step = Some(step);
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

    /// Show or hide the value labels
    pub fn show_values(mut self, show: bool) -> Self {
        self.show_values = show;
        self
    }

    /// Set the change callback (called during drag)
    pub fn on_change<F>(mut self, callback: F) -> Self
    where
        F: Fn(f32, f32) + 'static,
    {
        self.on_change = Some(Box::new(callback));
        self
    }

    /// Set the change end callback (called when drag ends)
    pub fn on_change_end<F>(mut self, callback: F) -> Self
    where
        F: Fn(f32, f32) + 'static,
    {
        self.on_change_end = Some(Box::new(callback));
        self
    }

    /// Set the start value
    pub fn set_start_value(&mut self, value: f32) {
        let clamped = value.clamp(self.min, self.get_end_value());
        let snapped = if let Some(step) = self.step {
            (clamped / step).round() * step
        } else {
            clamped
        };
        
        self.start_value.set(snapped);
        
        if let Some(ref callback) = self.on_change {
            callback(snapped, self.get_end_value());
        }
    }

    /// Set the end value
    pub fn set_end_value(&mut self, value: f32) {
        let clamped = value.clamp(self.get_start_value(), self.max);
        let snapped = if let Some(step) = self.step {
            (clamped / step).round() * step
        } else {
            clamped
        };
        
        self.end_value.set(snapped);
        
        if let Some(ref callback) = self.on_change {
            callback(self.get_start_value(), snapped);
        }
    }

    /// Get the start value
    pub fn get_start_value(&self) -> f32 {
        self.start_value.get()
    }

    /// Get the end value
    pub fn get_end_value(&self) -> f32 {
        self.end_value.get()
    }

    /// Get the range span
    pub fn get_span(&self) -> f32 {
        self.get_end_value() - self.get_start_value()
    }

    /// Get the start value as a percentage (0.0 to 1.0)
    pub fn get_start_percentage(&self) -> f32 {
        if self.max == self.min {
            0.0
        } else {
            (self.get_start_value() - self.min) / (self.max - self.min)
        }
    }

    /// Get the end value as a percentage (0.0 to 1.0)
    pub fn get_end_percentage(&self) -> f32 {
        if self.max == self.min {
            1.0
        } else {
            (self.get_end_value() - self.min) / (self.max - self.min)
        }
    }

    /// Set start value from percentage (0.0 to 1.0)
    pub fn set_start_from_percentage(&mut self, percentage: f32) {
        let value = self.min + (self.max - self.min) * percentage.clamp(0.0, 1.0);
        self.set_start_value(value);
    }

    /// Set end value from percentage (0.0 to 1.0)
    pub fn set_end_from_percentage(&mut self, percentage: f32) {
        let value = self.min + (self.max - self.min) * percentage.clamp(0.0, 1.0);
        self.set_end_value(value);
    }

    /// Notify that dragging has ended
    pub fn end_change(&mut self) {
        if let Some(ref callback) = self.on_change_end {
            callback(self.get_start_value(), self.get_end_value());
        }
    }

    /// Check if start value is at minimum
    pub fn is_start_at_min(&self) -> bool {
        (self.get_start_value() - self.min).abs() < f32::EPSILON
    }

    /// Check if end value is at maximum
    pub fn is_end_at_max(&self) -> bool {
        (self.get_end_value() - self.max).abs() < f32::EPSILON
    }

    /// Check if range covers full span
    pub fn is_full_range(&self) -> bool {
        self.is_start_at_min() && self.is_end_at_max()
    }

    /// Build the range layout
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
            .map_err(|e| format!("Failed to create range node: {:?}", e))?;
        self.node_id = Some(node);

        Ok(node)
    }
}

impl Default for Range {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn range_starts_at_full_range() {
        let range = Range::new();
        assert_eq!(range.get_start_value(), 0.0);
        assert_eq!(range.get_end_value(), 100.0);
    }

    #[test]
    fn range_set_values() {
        let mut range = Range::new().min(0.0).max(100.0);
        range.set_start_value(25.0);
        range.set_end_value(75.0);
        assert_eq!(range.get_start_value(), 25.0);
        assert_eq!(range.get_end_value(), 75.0);
    }

    #[test]
    fn range_start_cannot_exceed_end() {
        let mut range = Range::new().min(0.0).max(100.0).end_value(50.0);
        range.set_start_value(75.0);
        assert_eq!(range.get_start_value(), 50.0); // Clamped to end
    }

    #[test]
    fn range_end_cannot_go_below_start() {
        let mut range = Range::new().min(0.0).max(100.0).start_value(50.0);
        range.set_end_value(25.0);
        assert_eq!(range.get_end_value(), 50.0); // Clamped to start
    }

    #[test]
    fn range_step_snapping() {
        let mut range = Range::new().min(0.0).max(100.0).step(10.0);
        range.set_start_value(23.0);
        range.set_end_value(77.0);
        assert_eq!(range.get_start_value(), 20.0);
        assert_eq!(range.get_end_value(), 80.0);
    }

    #[test]
    fn range_get_span() {
        let range = Range::new()
            .min(0.0)
            .max(100.0)
            .start_value(25.0)
            .end_value(75.0);
        assert_eq!(range.get_span(), 50.0);
    }

    #[test]
    fn range_percentages() {
        let range = Range::new()
            .min(0.0)
            .max(100.0)
            .start_value(25.0)
            .end_value(75.0);
        assert_eq!(range.get_start_percentage(), 0.25);
        assert_eq!(range.get_end_percentage(), 0.75);
    }

    #[test]
    fn range_set_from_percentages() {
        let mut range = Range::new().min(0.0).max(100.0);
        range.set_start_from_percentage(0.25);
        range.set_end_from_percentage(0.75);
        assert_eq!(range.get_start_value(), 25.0);
        assert_eq!(range.get_end_value(), 75.0);
    }

    #[test]
    fn range_is_start_at_min() {
        let range = Range::new().min(0.0).max(100.0).start_value(0.0);
        assert!(range.is_start_at_min());
    }

    #[test]
    fn range_is_end_at_max() {
        let range = Range::new().min(0.0).max(100.0).end_value(100.0);
        assert!(range.is_end_at_max());
    }

    #[test]
    fn range_is_full_range() {
        let range = Range::new().min(0.0).max(100.0).start_value(0.0).end_value(100.0);
        assert!(range.is_full_range());
    }

    #[test]
    fn range_on_change_callback() {
        use std::sync::{Arc, Mutex};

        let changed = Arc::new(Mutex::new((0.0, 0.0)));
        let changed_clone = changed.clone();

        let mut range = Range::new()
            .min(0.0)
            .max(100.0)
            .on_change(move |start, end| {
                *changed_clone.lock().unwrap() = (start, end);
            });

        range.set_start_value(25.0);
        assert_eq!(*changed.lock().unwrap(), (25.0, 100.0));

        range.set_end_value(75.0);
        assert_eq!(*changed.lock().unwrap(), (25.0, 75.0));
    }

    #[test]
    fn range_on_change_end_callback() {
        use std::sync::{Arc, Mutex};

        let ended = Arc::new(Mutex::new((0.0, 0.0)));
        let ended_clone = ended.clone();

        let mut range = Range::new()
            .min(0.0)
            .max(100.0)
            .start_value(25.0)
            .end_value(75.0)
            .on_change_end(move |start, end| {
                *ended_clone.lock().unwrap() = (start, end);
            });

        range.end_change();
        assert_eq!(*ended.lock().unwrap(), (25.0, 75.0));
    }

    #[test]
    fn range_builder_pattern() {
        let range = Range::new()
            .min(0.0)
            .max(200.0)
            .start_value(50.0)
            .end_value(150.0)
            .step(10.0)
            .disabled(true)
            .width(300.0)
            .height(50.0)
            .show_values(true);

        assert_eq!(range.min, 0.0);
        assert_eq!(range.max, 200.0);
        assert_eq!(range.get_start_value(), 50.0);
        assert_eq!(range.get_end_value(), 150.0);
        assert_eq!(range.step, Some(10.0));
        assert!(range.disabled);
        assert_eq!(range.width, 300.0);
        assert_eq!(range.height, 50.0);
        assert!(range.show_values);
    }

    #[test]
    fn range_build_creates_node() {
        let mut engine = LayoutEngine::new();
        let mut range = Range::new();

        let result = range.build(&mut engine);
        assert!(result.is_ok());
        assert!(range.node_id.is_some());
    }
}
