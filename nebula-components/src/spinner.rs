// Spinner Component - Loading spinner animation
// Indicates loading state with smooth rotation

use nebula_core::layout::{LayoutEngine, NodeId};
use nebula_core::signal::Signal;

/// Spinner size presets
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpinnerSize {
    Small,   // 16px
    Medium,  // 24px
    Large,   // 32px
    XLarge,  // 48px
    Custom(u32),
}

impl SpinnerSize {
    /// Get the size in pixels
    pub fn to_pixels(&self) -> f32 {
        match self {
            SpinnerSize::Small => 16.0,
            SpinnerSize::Medium => 24.0,
            SpinnerSize::Large => 32.0,
            SpinnerSize::XLarge => 48.0,
            SpinnerSize::Custom(size) => *size as f32,
        }
    }
}

/// Spinner component - displays a loading spinner
/// 
/// # Example
/// ```
/// let mut spinner = Spinner::new()
///     .size(SpinnerSize::Large)
///     .color(59, 130, 246, 255)
///     .speed(1.0)
///     .thickness(3.0);
/// ```
pub struct Spinner {
    pub node_id: Option<NodeId>,
    pub is_spinning: Signal<bool>,
    pub size: SpinnerSize,
    pub color: (u8, u8, u8, u8),
    pub thickness: f32,
    pub speed: f32, // Rotation speed multiplier
    pub label: Option<String>,
    pub label_position: LabelPosition,
}

/// Label position relative to spinner
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LabelPosition {
    Top,
    Bottom,
    Left,
    Right,
}

impl Spinner {
    /// Create a new Spinner component
    pub fn new() -> Self {
        Self {
            node_id: None,
            is_spinning: Signal::new(true),
            size: SpinnerSize::Medium,
            color: (59, 130, 246, 255), // Blue
            thickness: 2.0,
            speed: 1.0,
            label: None,
            label_position: LabelPosition::Bottom,
        }
    }

    /// Set the size
    pub fn size(mut self, size: SpinnerSize) -> Self {
        self.size = size;
        self
    }

    /// Set the color
    pub fn color(mut self, r: u8, g: u8, b: u8, a: u8) -> Self {
        self.color = (r, g, b, a);
        self
    }

    /// Set the thickness
    pub fn thickness(mut self, thickness: f32) -> Self {
        self.thickness = thickness;
        self
    }

    /// Set the speed
    pub fn speed(mut self, speed: f32) -> Self {
        self.speed = speed;
        self
    }

    /// Set the label
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Set the label position
    pub fn label_position(mut self, position: LabelPosition) -> Self {
        self.label_position = position;
        self
    }

    /// Start spinning
    pub fn start(&mut self) {
        self.is_spinning.set(true);
    }

    /// Stop spinning
    pub fn stop(&mut self) {
        self.is_spinning.set(false);
    }

    /// Toggle spinning
    pub fn toggle(&mut self) {
        let current = self.is_spinning.get();
        self.is_spinning.set(!current);
    }

    /// Check if spinning
    pub fn is_spinning(&self) -> bool {
        self.is_spinning.get()
    }

    /// Get the size in pixels
    pub fn get_size_pixels(&self) -> f32 {
        self.size.to_pixels()
    }

    /// Check if has label
    pub fn has_label(&self) -> bool {
        self.label.is_some()
    }

    /// Build the spinner layout
    pub fn build(&mut self, engine: &mut LayoutEngine) -> Result<NodeId, String> {
        let size = self.get_size_pixels();
        
        let style = taffy::style::Style {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Length(size),
                height: taffy::style::Dimension::Length(size),
            },
            ..Default::default()
        };

        let node = engine
            .new_leaf(style)
            .map_err(|e| format!("Failed to create spinner node: {:?}", e))?;
        self.node_id = Some(node);

        Ok(node)
    }
}

impl Default for Spinner {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spinner_starts_spinning() {
        let spinner = Spinner::new();
        assert!(spinner.is_spinning());
    }

    #[test]
    fn spinner_can_be_stopped() {
        let mut spinner = Spinner::new();
        spinner.stop();
        assert!(!spinner.is_spinning());
    }

    #[test]
    fn spinner_can_be_started() {
        let mut spinner = Spinner::new();
        spinner.stop();
        spinner.start();
        assert!(spinner.is_spinning());
    }

    #[test]
    fn spinner_can_be_toggled() {
        let mut spinner = Spinner::new();
        assert!(spinner.is_spinning());
        
        spinner.toggle();
        assert!(!spinner.is_spinning());
        
        spinner.toggle();
        assert!(spinner.is_spinning());
    }

    #[test]
    fn spinner_size_presets() {
        assert_eq!(SpinnerSize::Small.to_pixels(), 16.0);
        assert_eq!(SpinnerSize::Medium.to_pixels(), 24.0);
        assert_eq!(SpinnerSize::Large.to_pixels(), 32.0);
        assert_eq!(SpinnerSize::XLarge.to_pixels(), 48.0);
        assert_eq!(SpinnerSize::Custom(64).to_pixels(), 64.0);
    }

    #[test]
    fn spinner_get_size_pixels() {
        let small = Spinner::new().size(SpinnerSize::Small);
        assert_eq!(small.get_size_pixels(), 16.0);

        let large = Spinner::new().size(SpinnerSize::Large);
        assert_eq!(large.get_size_pixels(), 32.0);

        let custom = Spinner::new().size(SpinnerSize::Custom(100));
        assert_eq!(custom.get_size_pixels(), 100.0);
    }

    #[test]
    fn spinner_has_label() {
        let without_label = Spinner::new();
        assert!(!without_label.has_label());

        let with_label = Spinner::new().label("Loading...");
        assert!(with_label.has_label());
    }

    #[test]
    fn spinner_builder_pattern() {
        let spinner = Spinner::new()
            .size(SpinnerSize::Large)
            .color(255, 0, 0, 255)
            .thickness(4.0)
            .speed(2.0)
            .label("Please wait...")
            .label_position(LabelPosition::Right);

        assert_eq!(spinner.size, SpinnerSize::Large);
        assert_eq!(spinner.color, (255, 0, 0, 255));
        assert_eq!(spinner.thickness, 4.0);
        assert_eq!(spinner.speed, 2.0);
        assert_eq!(spinner.label, Some("Please wait...".to_string()));
        assert_eq!(spinner.label_position, LabelPosition::Right);
    }

    #[test]
    fn spinner_build_creates_node() {
        let mut engine = LayoutEngine::new();
        let mut spinner = Spinner::new();

        let result = spinner.build(&mut engine);
        assert!(result.is_ok());
        assert!(spinner.node_id.is_some());
    }

    #[test]
    fn spinner_label_positions() {
        let top = Spinner::new().label_position(LabelPosition::Top);
        assert_eq!(top.label_position, LabelPosition::Top);

        let bottom = Spinner::new().label_position(LabelPosition::Bottom);
        assert_eq!(bottom.label_position, LabelPosition::Bottom);

        let left = Spinner::new().label_position(LabelPosition::Left);
        assert_eq!(left.label_position, LabelPosition::Left);

        let right = Spinner::new().label_position(LabelPosition::Right);
        assert_eq!(right.label_position, LabelPosition::Right);
    }
}
