// ProgressBar Component - Linear progress indicator
// Shows progress of operations with smooth animations

use nebula_core::layout::{LayoutEngine, NodeId};
use nebula_core::signal::Signal;

/// ProgressBar component - displays linear progress
/// 
/// # Example
/// ```
/// let mut progress = ProgressBar::new()
///     .value(0.65) // 65%
///     .show_label(true)
///     .color(0, 200, 0, 255)
///     .animated(true);
/// ```
pub struct ProgressBar {
    pub node_id: Option<NodeId>,
    pub value: Signal<f32>, // 0.0 to 1.0
    pub width: f32,
    pub height: f32,
    pub background_color: (u8, u8, u8, u8),
    pub fill_color: (u8, u8, u8, u8),
    pub border_radius: f32,
    pub show_label: bool,
    pub label_format: String, // e.g., "{percent}%" or "{value}/{max}"
    pub animated: bool,
    pub animation_duration: f32, // seconds
    pub indeterminate: bool, // For unknown progress
    pub on_complete: Option<Box<dyn Fn()>>,
}

impl ProgressBar {
    /// Create a new ProgressBar component
    pub fn new() -> Self {
        Self {
            node_id: None,
            value: Signal::new(0.0),
            width: 200.0,
            height: 8.0,
            background_color: (230, 230, 230, 255),
            fill_color: (59, 130, 246, 255), // Blue
            border_radius: 4.0,
            show_label: false,
            label_format: "{percent}%".to_string(),
            animated: true,
            animation_duration: 0.3,
            indeterminate: false,
            on_complete: None,
        }
    }

    /// Set the progress value (0.0 to 1.0)
    pub fn value(self, value: f32) -> Self {
        self.value.set(value.clamp(0.0, 1.0));
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

    /// Set the background color
    pub fn background_color(mut self, r: u8, g: u8, b: u8, a: u8) -> Self {
        self.background_color = (r, g, b, a);
        self
    }

    /// Set the fill color
    pub fn color(mut self, r: u8, g: u8, b: u8, a: u8) -> Self {
        self.fill_color = (r, g, b, a);
        self
    }

    /// Set the border radius
    pub fn border_radius(mut self, radius: f32) -> Self {
        self.border_radius = radius;
        self
    }

    /// Set whether to show label
    pub fn show_label(mut self, show: bool) -> Self {
        self.show_label = show;
        self
    }

    /// Set the label format
    pub fn label_format(mut self, format: impl Into<String>) -> Self {
        self.label_format = format.into();
        self
    }

    /// Set whether to animate
    pub fn animated(mut self, animated: bool) -> Self {
        self.animated = animated;
        self
    }

    /// Set animation duration
    pub fn animation_duration(mut self, duration: f32) -> Self {
        self.animation_duration = duration;
        self
    }

    /// Set indeterminate mode
    pub fn indeterminate(mut self, indeterminate: bool) -> Self {
        self.indeterminate = indeterminate;
        self
    }

    /// Set the complete callback
    pub fn on_complete<F>(mut self, callback: F) -> Self
    where
        F: Fn() + 'static,
    {
        self.on_complete = Some(Box::new(callback));
        self
    }

    /// Update the progress value
    pub fn set_value(&mut self, value: f32) {
        let clamped = value.clamp(0.0, 1.0);
        let was_complete = self.is_complete();
        
        self.value.set(clamped);
        
        if !was_complete && self.is_complete() {
            if let Some(ref callback) = self.on_complete {
                callback();
            }
        }
    }

    /// Get the current value
    pub fn get_value(&self) -> f32 {
        self.value.get()
    }

    /// Get the percentage (0-100)
    pub fn get_percent(&self) -> f32 {
        self.get_value() * 100.0
    }

    /// Check if complete
    pub fn is_complete(&self) -> bool {
        self.get_value() >= 1.0
    }

    /// Reset to zero
    pub fn reset(&mut self) {
        self.value.set(0.0);
    }

    /// Increment by amount
    pub fn increment(&mut self, amount: f32) {
        let new_value = self.get_value() + amount;
        self.set_value(new_value);
    }

    /// Get the formatted label
    pub fn get_label(&self) -> String {
        self.label_format
            .replace("{percent}", &format!("{:.0}", self.get_percent()))
            .replace("{value}", &format!("{:.2}", self.get_value()))
    }

    /// Build the progress bar layout
    pub fn build(&mut self, engine: &mut LayoutEngine) -> Result<NodeId, String> {
        let style = taffy::style::Style {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Length(self.width),
                height: taffy::style::Dimension::Length(self.height),
            },
            ..Default::default()
        };

        let node = engine
            .new_leaf(style)
            .map_err(|e| format!("Failed to create progress bar node: {:?}", e))?;
        self.node_id = Some(node);

        Ok(node)
    }
}

impl Default for ProgressBar {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn progress_bar_starts_at_zero() {
        let progress = ProgressBar::new();
        assert_eq!(progress.get_value(), 0.0);
        assert_eq!(progress.get_percent(), 0.0);
        assert!(!progress.is_complete());
    }

    #[test]
    fn progress_bar_set_value() {
        let mut progress = ProgressBar::new();
        progress.set_value(0.5);
        assert_eq!(progress.get_value(), 0.5);
        assert_eq!(progress.get_percent(), 50.0);
    }

    #[test]
    fn progress_bar_clamps_value() {
        let mut progress = ProgressBar::new();
        
        progress.set_value(1.5);
        assert_eq!(progress.get_value(), 1.0);
        
        progress.set_value(-0.5);
        assert_eq!(progress.get_value(), 0.0);
    }

    #[test]
    fn progress_bar_is_complete() {
        let mut progress = ProgressBar::new();
        assert!(!progress.is_complete());
        
        progress.set_value(1.0);
        assert!(progress.is_complete());
    }

    #[test]
    fn progress_bar_reset() {
        let mut progress = ProgressBar::new();
        progress.set_value(0.75);
        progress.reset();
        assert_eq!(progress.get_value(), 0.0);
    }

    #[test]
    fn progress_bar_increment() {
        let mut progress = ProgressBar::new();
        progress.increment(0.25);
        assert_eq!(progress.get_value(), 0.25);
        
        progress.increment(0.25);
        assert_eq!(progress.get_value(), 0.5);
    }

    #[test]
    fn progress_bar_increment_clamps() {
        let mut progress = ProgressBar::new();
        progress.set_value(0.9);
        progress.increment(0.5);
        assert_eq!(progress.get_value(), 1.0);
    }

    #[test]
    fn progress_bar_label_format() {
        let progress = ProgressBar::new().value(0.65);
        
        let label = progress.get_label();
        assert_eq!(label, "65%");
    }

    #[test]
    fn progress_bar_custom_label_format() {
        let progress = ProgressBar::new()
            .value(0.65)
            .label_format("Progress: {percent}%");
        
        let label = progress.get_label();
        assert_eq!(label, "Progress: 65%");
    }

    #[test]
    fn progress_bar_on_complete_callback() {
        use std::sync::{Arc, Mutex};
        
        let completed = Arc::new(Mutex::new(false));
        let completed_clone = completed.clone();
        
        let mut progress = ProgressBar::new()
            .on_complete(move || {
                *completed_clone.lock().unwrap() = true;
            });
        
        progress.set_value(0.5);
        assert!(!*completed.lock().unwrap());
        
        progress.set_value(1.0);
        assert!(*completed.lock().unwrap());
    }

    #[test]
    fn progress_bar_on_complete_only_once() {
        use std::sync::{Arc, Mutex};
        
        let count = Arc::new(Mutex::new(0));
        let count_clone = count.clone();
        
        let mut progress = ProgressBar::new()
            .on_complete(move || {
                *count_clone.lock().unwrap() += 1;
            });
        
        progress.set_value(1.0);
        progress.set_value(1.0); // Should not trigger again
        
        assert_eq!(*count.lock().unwrap(), 1);
    }

    #[test]
    fn progress_bar_builder_pattern() {
        let progress = ProgressBar::new()
            .value(0.5)
            .width(300.0)
            .height(12.0)
            .background_color(200, 200, 200, 255)
            .color(0, 255, 0, 255)
            .border_radius(6.0)
            .show_label(true)
            .animated(false)
            .indeterminate(true);

        assert_eq!(progress.get_value(), 0.5);
        assert_eq!(progress.width, 300.0);
        assert_eq!(progress.height, 12.0);
        assert_eq!(progress.background_color, (200, 200, 200, 255));
        assert_eq!(progress.fill_color, (0, 255, 0, 255));
        assert_eq!(progress.border_radius, 6.0);
        assert!(progress.show_label);
        assert!(!progress.animated);
        assert!(progress.indeterminate);
    }

    #[test]
    fn progress_bar_build_creates_node() {
        let mut engine = LayoutEngine::new();
        let mut progress = ProgressBar::new();

        let result = progress.build(&mut engine);
        assert!(result.is_ok());
        assert!(progress.node_id.is_some());
    }
}
