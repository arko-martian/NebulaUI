// Rating Component - Star rating for user feedback
// Essential for reviews and ratings

use nebula_core::layout::{LayoutEngine, NodeId};
use nebula_core::signal::Signal;

/// Rating component - star rating for user feedback
/// 
/// # Example
/// ```
/// let mut rating = Rating::new()
///     .max_rating(5)
///     .value(3.5)
///     .allow_half_stars(true)
///     .on_change(|value| println!("Rating: {}", value));
/// ```
pub struct Rating {
    pub node_id: Option<NodeId>,
    pub value: Signal<f32>,
    pub max_rating: u8,
    pub size: f32,
    pub spacing: f32,
    pub allow_half_stars: bool,
    pub readonly: bool,
    pub show_value: bool,
    pub filled_color: (u8, u8, u8, u8),
    pub empty_color: (u8, u8, u8, u8),
    pub hover_color: (u8, u8, u8, u8),
    pub filled_icon: String,
    pub empty_icon: String,
    pub half_icon: String,
    pub on_change: Option<Box<dyn Fn(f32)>>,
    pub on_hover: Option<Box<dyn Fn(Option<f32>)>>,
}

impl Rating {
    /// Create a new Rating component
    pub fn new() -> Self {
        Self {
            node_id: None,
            value: Signal::new(0.0),
            max_rating: 5,
            size: 24.0,
            spacing: 4.0,
            allow_half_stars: false,
            readonly: false,
            show_value: false,
            filled_color: (255, 193, 7, 255), // Amber/Gold
            empty_color: (200, 200, 200, 255), // Gray
            hover_color: (255, 213, 79, 255), // Light amber
            filled_icon: "★".to_string(),
            empty_icon: "☆".to_string(),
            half_icon: "⯨".to_string(),
            on_change: None,
            on_hover: None,
        }
    }

    /// Set the current value
    pub fn value(mut self, value: f32) -> Self {
        self.value.set(value.clamp(0.0, self.max_rating as f32));
        self
    }

    /// Set the maximum rating
    pub fn max_rating(mut self, max: u8) -> Self {
        self.max_rating = max.max(1);
        self
    }

    /// Set the star size
    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    /// Set the spacing between stars
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }

    /// Allow half star ratings
    pub fn allow_half_stars(mut self, allow: bool) -> Self {
        self.allow_half_stars = allow;
        self
    }

    /// Set readonly mode
    pub fn readonly(mut self, readonly: bool) -> Self {
        self.readonly = readonly;
        self
    }

    /// Show numeric value
    pub fn show_value(mut self, show: bool) -> Self {
        self.show_value = show;
        self
    }

    /// Set filled star color
    pub fn filled_color(mut self, r: u8, g: u8, b: u8, a: u8) -> Self {
        self.filled_color = (r, g, b, a);
        self
    }

    /// Set empty star color
    pub fn empty_color(mut self, r: u8, g: u8, b: u8, a: u8) -> Self {
        self.empty_color = (r, g, b, a);
        self
    }

    /// Set hover color
    pub fn hover_color(mut self, r: u8, g: u8, b: u8, a: u8) -> Self {
        self.hover_color = (r, g, b, a);
        self
    }

    /// Set filled icon
    pub fn filled_icon(mut self, icon: impl Into<String>) -> Self {
        self.filled_icon = icon.into();
        self
    }

    /// Set empty icon
    pub fn empty_icon(mut self, icon: impl Into<String>) -> Self {
        self.empty_icon = icon.into();
        self
    }

    /// Set half icon
    pub fn half_icon(mut self, icon: impl Into<String>) -> Self {
        self.half_icon = icon.into();
        self
    }

    /// Set the change callback
    pub fn on_change<F>(mut self, callback: F) -> Self
    where
        F: Fn(f32) + 'static,
    {
        self.on_change = Some(Box::new(callback));
        self
    }

    /// Set the hover callback
    pub fn on_hover<F>(mut self, callback: F) -> Self
    where
        F: Fn(Option<f32>) + 'static,
    {
        self.on_hover = Some(Box::new(callback));
        self
    }

    /// Set the rating value
    pub fn set_value(&mut self, value: f32) {
        if self.readonly {
            return;
        }

        let clamped = value.clamp(0.0, self.max_rating as f32);
        let final_value = if self.allow_half_stars {
            (clamped * 2.0).round() / 2.0 // Round to nearest 0.5
        } else {
            clamped.round()
        };

        self.value.set(final_value);

        if let Some(ref callback) = self.on_change {
            callback(final_value);
        }
    }

    /// Get the current value
    pub fn get_value(&self) -> f32 {
        self.value.get()
    }

    /// Clear the rating
    pub fn clear(&mut self) {
        self.set_value(0.0);
    }

    /// Check if rating is empty
    pub fn is_empty(&self) -> bool {
        self.value.get() == 0.0
    }

    /// Check if rating is full
    pub fn is_full(&self) -> bool {
        self.value.get() == self.max_rating as f32
    }

    /// Get percentage (0.0 to 1.0)
    pub fn get_percentage(&self) -> f32 {
        self.value.get() / self.max_rating as f32
    }

    /// Set rating from percentage (0.0 to 1.0)
    pub fn set_from_percentage(&mut self, percentage: f32) {
        let value = percentage.clamp(0.0, 1.0) * self.max_rating as f32;
        self.set_value(value);
    }

    /// Increment rating by one step
    pub fn increment(&mut self) {
        let step = if self.allow_half_stars { 0.5 } else { 1.0 };
        let new_value = (self.value.get() + step).min(self.max_rating as f32);
        self.set_value(new_value);
    }

    /// Decrement rating by one step
    pub fn decrement(&mut self) {
        let step = if self.allow_half_stars { 0.5 } else { 1.0 };
        let new_value = (self.value.get() - step).max(0.0);
        self.set_value(new_value);
    }

    /// Handle hover at position (0 to max_rating)
    pub fn handle_hover(&mut self, position: Option<f32>) {
        if self.readonly {
            return;
        }

        if let Some(ref callback) = self.on_hover {
            callback(position);
        }
    }

    /// Build the rating layout
    pub fn build(&mut self, engine: &mut LayoutEngine) -> Result<NodeId, String> {
        let style = taffy::style::Style {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Auto,
                height: taffy::style::Dimension::Length(self.size),
            },
            display: taffy::style::Display::Flex,
            flex_direction: taffy::style::FlexDirection::Row,
            gap: taffy::geometry::Size {
                width: taffy::style::LengthPercentage::Length(self.spacing),
                height: taffy::style::LengthPercentage::Length(0.0),
            },
            ..Default::default()
        };

        let node = engine
            .new_leaf(style)
            .map_err(|e| format!("Failed to create rating node: {:?}", e))?;
        self.node_id = Some(node);

        Ok(node)
    }
}

impl Default for Rating {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rating_starts_at_zero() {
        let rating = Rating::new();
        assert_eq!(rating.get_value(), 0.0);
        assert!(rating.is_empty());
        assert!(!rating.is_full());
    }

    #[test]
    fn rating_set_value() {
        let mut rating = Rating::new().max_rating(5);
        rating.set_value(3.0);
        assert_eq!(rating.get_value(), 3.0);
    }

    #[test]
    fn rating_clamps_value() {
        let mut rating = Rating::new().max_rating(5);
        rating.set_value(10.0);
        assert_eq!(rating.get_value(), 5.0);

        rating.set_value(-1.0);
        assert_eq!(rating.get_value(), 0.0);
    }

    #[test]
    fn rating_half_stars() {
        let mut rating = Rating::new()
            .max_rating(5)
            .allow_half_stars(true);

        rating.set_value(3.7);
        assert_eq!(rating.get_value(), 3.5); // Rounds to nearest 0.5
    }

    #[test]
    fn rating_no_half_stars() {
        let mut rating = Rating::new()
            .max_rating(5)
            .allow_half_stars(false);

        rating.set_value(3.7);
        assert_eq!(rating.get_value(), 4.0); // Rounds to nearest integer
    }

    #[test]
    fn rating_readonly() {
        let mut rating = Rating::new()
            .max_rating(5)
            .value(3.0)
            .readonly(true);

        rating.set_value(5.0);
        assert_eq!(rating.get_value(), 3.0); // Should not change
    }

    #[test]
    fn rating_clear() {
        let mut rating = Rating::new().value(4.0);
        assert_eq!(rating.get_value(), 4.0);

        rating.clear();
        assert_eq!(rating.get_value(), 0.0);
        assert!(rating.is_empty());
    }

    #[test]
    fn rating_is_full() {
        let mut rating = Rating::new().max_rating(5);
        rating.set_value(5.0);
        assert!(rating.is_full());
    }

    #[test]
    fn rating_percentage() {
        let rating = Rating::new()
            .max_rating(5)
            .value(2.5);

        assert_eq!(rating.get_percentage(), 0.5);
    }

    #[test]
    fn rating_set_from_percentage() {
        let mut rating = Rating::new().max_rating(5);
        rating.set_from_percentage(0.6);
        assert_eq!(rating.get_value(), 3.0);
    }

    #[test]
    fn rating_increment() {
        let mut rating = Rating::new()
            .max_rating(5)
            .value(2.0);

        rating.increment();
        assert_eq!(rating.get_value(), 3.0);
    }

    #[test]
    fn rating_increment_half_stars() {
        let mut rating = Rating::new()
            .max_rating(5)
            .allow_half_stars(true)
            .value(2.0);

        rating.increment();
        assert_eq!(rating.get_value(), 2.5);
    }

    #[test]
    fn rating_increment_at_max() {
        let mut rating = Rating::new()
            .max_rating(5)
            .value(5.0);

        rating.increment();
        assert_eq!(rating.get_value(), 5.0); // Should stay at max
    }

    #[test]
    fn rating_decrement() {
        let mut rating = Rating::new()
            .max_rating(5)
            .value(3.0);

        rating.decrement();
        assert_eq!(rating.get_value(), 2.0);
    }

    #[test]
    fn rating_decrement_at_zero() {
        let mut rating = Rating::new()
            .max_rating(5)
            .value(0.0);

        rating.decrement();
        assert_eq!(rating.get_value(), 0.0); // Should stay at zero
    }

    #[test]
    fn rating_callback() {
        use std::sync::{Arc, Mutex};

        let changed = Arc::new(Mutex::new(0.0));
        let changed_clone = changed.clone();

        let mut rating = Rating::new()
            .max_rating(5)
            .on_change(move |value| {
                *changed_clone.lock().unwrap() = value;
            });

        rating.set_value(4.0);
        assert_eq!(*changed.lock().unwrap(), 4.0);
    }

    #[test]
    fn rating_hover_callback() {
        use std::sync::{Arc, Mutex};

        let hovered = Arc::new(Mutex::new(None));
        let hovered_clone = hovered.clone();

        let mut rating = Rating::new()
            .on_hover(move |pos| {
                *hovered_clone.lock().unwrap() = pos;
            });

        rating.handle_hover(Some(3.5));
        assert_eq!(*hovered.lock().unwrap(), Some(3.5));
    }

    #[test]
    fn rating_builder_pattern() {
        let rating = Rating::new()
            .max_rating(10)
            .value(7.5)
            .size(32.0)
            .spacing(8.0)
            .allow_half_stars(true)
            .readonly(true)
            .show_value(true)
            .filled_color(255, 0, 0, 255)
            .empty_color(100, 100, 100, 255)
            .hover_color(255, 100, 100, 255)
            .filled_icon("★")
            .empty_icon("☆")
            .half_icon("⯨");

        assert_eq!(rating.max_rating, 10);
        assert_eq!(rating.get_value(), 7.5);
        assert_eq!(rating.size, 32.0);
        assert_eq!(rating.spacing, 8.0);
        assert!(rating.allow_half_stars);
        assert!(rating.readonly);
        assert!(rating.show_value);
        assert_eq!(rating.filled_color, (255, 0, 0, 255));
        assert_eq!(rating.empty_color, (100, 100, 100, 255));
        assert_eq!(rating.hover_color, (255, 100, 100, 255));
        assert_eq!(rating.filled_icon, "★");
        assert_eq!(rating.empty_icon, "☆");
        assert_eq!(rating.half_icon, "⯨");
    }

    #[test]
    fn rating_build_creates_node() {
        let mut engine = LayoutEngine::new();
        let mut rating = Rating::new();

        let result = rating.build(&mut engine);
        assert!(result.is_ok());
        assert!(rating.node_id.is_some());
    }
}
