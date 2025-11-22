// Slider Component - Value slider for numeric input
// Essential for adjusting values with visual feedback

use nebula_core::layout::{LayoutEngine, NodeId};
use nebula_core::signal::Signal;

/// Slider component - value slider for numeric input
/// 
/// # Example
/// ```
/// let mut slider = Slider::new()
///     .min(0.0)
///     .max(100.0)
///     .value(50.0)
///     .step(1.0)
///     .on_change(|value| println!("Value: {}", value));
/// ```
pub struct Slider {
    pub node_id: Option<NodeId>,
    pub value: Signal<f32>,
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
    pub show_value: bool,
    pub show_ticks: bool,
    pub tick_count: usize,
    pub on_change: Option<Box<dyn Fn(f32)>>,
    pub on_change_end: Option<Box<dyn Fn(f32)>>,
}

impl Slider {
    /// Create a new Slider component
    pub fn new() -> Self {
        Self {
            node_id: None,
            value: Signal::new(0.0),
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
            show_value: false,
            show_ticks: false,
            tick_count: 0,
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

    /// Set the current value
    pub fn value(self, value: f32) -> Self {
        self.value.set(value.clamp(self.min, self.max));
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

    /// Set the track height
    pub fn track_height(mut self, height: f32) -> Self {
        self.track_height = height;
        self
    }

    /// Set the thumb size
    pub fn thumb_size(mut self, size: f32) -> Self {
        self.thumb_size = size;
        self
    }

    /// Set the track color
    pub fn track_color(mut self, r: u8, g: u8, b: u8, a: u8) -> Self {
        self.track_color = (r, g, b, a);
        self
    }

    /// Set the track fill color
    pub fn track_fill_color(mut self, r: u8, g: u8, b: u8, a: u8) -> Self {
        self.track_fill_color = (r, g, b, a);
        self
    }

    /// Set the thumb color
    pub fn thumb_color(mut self, r: u8, g: u8, b: u8, a: u8) -> Self {
        self.thumb_color = (r, g, b, a);
        self
    }

    /// Show or hide the value label
    pub fn show_value(mut self, show: bool) -> Self {
        self.show_value = show;
        self
    }

    /// Show or hide tick marks
    pub fn show_ticks(mut self, show: bool) -> Self {
        self.show_ticks = show;
        self
    }

    /// Set the number of tick marks
    pub fn tick_count(mut self, count: usize) -> Self {
        self.tick_count = count;
        self.show_ticks = count > 0;
        self
    }

    /// Set the change callback (called during drag)
    pub fn on_change<F>(mut self, callback: F) -> Self
    where
        F: Fn(f32) + 'static,
    {
        self.on_change = Some(Box::new(callback));
        self
    }

    /// Set the change end callback (called when drag ends)
    pub fn on_change_end<F>(mut self, callback: F) -> Self
    where
        F: Fn(f32) + 'static,
    {
        self.on_change_end = Some(Box::new(callback));
        self
    }

    /// Set the value
    pub fn set_value(&mut self, value: f32) {
        let clamped = value.clamp(self.min, self.max);
        let snapped = if let Some(step) = self.step {
            (clamped / step).round() * step
        } else {
            clamped
        };
        
        self.value.set(snapped);
        
        if let Some(ref callback) = self.on_change {
            callback(snapped);
        }
    }

    /// Get the current value
    pub fn get_value(&self) -> f32 {
        self.value.get()
    }

    /// Get the value as a percentage (0.0 to 1.0)
    pub fn get_percentage(&self) -> f32 {
        if self.max == self.min {
            0.0
        } else {
            (self.get_value() - self.min) / (self.max - self.min)
        }
    }

    /// Set value from percentage (0.0 to 1.0)
    pub fn set_from_percentage(&mut self, percentage: f32) {
        let value = self.min + (self.max - self.min) * percentage.clamp(0.0, 1.0);
        self.set_value(value);
    }

    /// Increment the value by step
    pub fn increment(&mut self) {
        let step = self.step.unwrap_or(1.0);
        self.set_value(self.get_value() + step);
    }

    /// Decrement the value by step
    pub fn decrement(&mut self) {
        let step = self.step.unwrap_or(1.0);
        self.set_value(self.get_value() - step);
    }

    /// Notify that dragging has ended
    pub fn end_change(&mut self) {
        if let Some(ref callback) = self.on_change_end {
            callback(self.get_value());
        }
    }

    /// Check if value is at minimum
    pub fn is_at_min(&self) -> bool {
        (self.get_value() - self.min).abs() < f32::EPSILON
    }

    /// Check if value is at maximum
    pub fn is_at_max(&self) -> bool {
        (self.get_value() - self.max).abs() < f32::EPSILON
    }

    /// Build the slider layout
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
            .map_err(|e| format!("Failed to create slider node: {:?}", e))?;
        self.node_id = Some(node);

        Ok(node)
    }
}

impl Default for Slider {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slider_starts_at_zero() {
        let slider = Slider::new();
        assert_eq!(slider.get_value(), 0.0);
    }

    #[test]
    fn slider_set_value() {
        let mut slider = Slider::new().min(0.0).max(100.0);
        slider.set_value(50.0);
        assert_eq!(slider.get_value(), 50.0);
    }

    #[test]
    fn slider_clamps_value() {
        let mut slider = Slider::new().min(0.0).max(100.0);
        slider.set_value(150.0);
        assert_eq!(slider.get_value(), 100.0);
        
        slider.set_value(-50.0);
        assert_eq!(slider.get_value(), 0.0);
    }

    #[test]
    fn slider_step_snapping() {
        let mut slider = Slider::new().min(0.0).max(100.0).step(10.0);
        slider.set_value(47.0);
        assert_eq!(slider.get_value(), 50.0); // Snaps to nearest step
    }

    #[test]
    fn slider_percentage() {
        let slider = Slider::new().min(0.0).max(100.0).value(50.0);
        assert_eq!(slider.get_percentage(), 0.5);
    }

    #[test]
    fn slider_set_from_percentage() {
        let mut slider = Slider::new().min(0.0).max(100.0);
        slider.set_from_percentage(0.75);
        assert_eq!(slider.get_value(), 75.0);
    }

    #[test]
    fn slider_increment() {
        let mut slider = Slider::new().min(0.0).max(100.0).value(50.0).step(5.0);
        slider.increment();
        assert_eq!(slider.get_value(), 55.0);
    }

    #[test]
    fn slider_decrement() {
        let mut slider = Slider::new().min(0.0).max(100.0).value(50.0).step(5.0);
        slider.decrement();
        assert_eq!(slider.get_value(), 45.0);
    }

    #[test]
    fn slider_is_at_min() {
        let slider = Slider::new().min(0.0).max(100.0).value(0.0);
        assert!(slider.is_at_min());
    }

    #[test]
    fn slider_is_at_max() {
        let slider = Slider::new().min(0.0).max(100.0).value(100.0);
        assert!(slider.is_at_max());
    }

    #[test]
    fn slider_on_change_callback() {
        use std::sync::{Arc, Mutex};

        let changed = Arc::new(Mutex::new(0.0));
        let changed_clone = changed.clone();

        let mut slider = Slider::new()
            .min(0.0)
            .max(100.0)
            .on_change(move |value| {
                *changed_clone.lock().unwrap() = value;
            });

        slider.set_value(75.0);
        assert_eq!(*changed.lock().unwrap(), 75.0);
    }

    #[test]
    fn slider_on_change_end_callback() {
        use std::sync::{Arc, Mutex};

        let ended = Arc::new(Mutex::new(0.0));
        let ended_clone = ended.clone();

        let mut slider = Slider::new()
            .min(0.0)
            .max(100.0)
            .value(50.0)
            .on_change_end(move |value| {
                *ended_clone.lock().unwrap() = value;
            });

        slider.end_change();
        assert_eq!(*ended.lock().unwrap(), 50.0);
    }

    #[test]
    fn slider_builder_pattern() {
        let slider = Slider::new()
            .min(0.0)
            .max(200.0)
            .value(100.0)
            .step(10.0)
            .disabled(true)
            .width(300.0)
            .height(50.0)
            .track_height(6.0)
            .thumb_size(24.0)
            .show_value(true)
            .show_ticks(true)
            .tick_count(10);

        assert_eq!(slider.min, 0.0);
        assert_eq!(slider.max, 200.0);
        assert_eq!(slider.get_value(), 100.0);
        assert_eq!(slider.step, Some(10.0));
        assert!(slider.disabled);
        assert_eq!(slider.width, 300.0);
        assert_eq!(slider.height, 50.0);
        assert_eq!(slider.track_height, 6.0);
        assert_eq!(slider.thumb_size, 24.0);
        assert!(slider.show_value);
        assert!(slider.show_ticks);
        assert_eq!(slider.tick_count, 10);
    }

    #[test]
    fn slider_build_creates_node() {
        let mut engine = LayoutEngine::new();
        let mut slider = Slider::new();

        let result = slider.build(&mut engine);
        assert!(result.is_ok());
        assert!(slider.node_id.is_some());
    }

    #[test]
    fn slider_disabled() {
        let slider = Slider::new().disabled(true);
        assert!(slider.disabled);
    }
}
