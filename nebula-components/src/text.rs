use nebula_core::{Signal, TextRenderer, FontFamily};

/// Text component - Display reactive text! 📝
/// 
/// This wraps the TextRenderer with a reactive Signal!
/// - Reactive content (powered by Signals!)
/// - CPU rendering (works everywhere!)
/// - International support (Latin, Bengali, etc!)
#[derive(Clone)]
pub struct Text {
    /// Text content (reactive!)
    pub content: Signal<String>,
    /// Text position (x, y)
    pub position: (f32, f32),
    /// Font size
    pub font_size: u32,
    /// Font family
    pub font_family: FontFamily,
}

impl Text {
    /// Create a new text component
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: Signal::new(content.into()),
            position: (0.0, 0.0),
            font_size: 24,
            font_family: FontFamily::Roboto,
        }
    }

    /// Create text from a Signal
    pub fn from_signal(content: Signal<String>) -> Self {
        Self {
            content,
            position: (0.0, 0.0),
            font_size: 24,
            font_family: FontFamily::Roboto,
        }
    }

    /// Set text position
    pub fn position(mut self, x: f32, y: f32) -> Self {
        self.position = (x, y);
        self
    }

    /// Set font size
    pub fn font_size(mut self, size: u32) -> Self {
        self.font_size = size;
        self
    }

    /// Set font family
    pub fn font_family(mut self, family: FontFamily) -> Self {
        self.font_family = family;
        self
    }

    /// Get the current text content
    pub fn get_content(&self) -> String {
        self.content.get()
    }

    /// Set the text content
    pub fn set_content(&self, content: impl Into<String>) {
        self.content.set(content.into());
    }

    /// Measure the text width using a TextRenderer
    pub fn measure_width(&self, renderer: &mut TextRenderer) -> f32 {
        renderer.measure_text(&self.get_content(), self.font_size)
    }

    /// Get text bounds (x, y, width, height)
    /// Note: Height is approximate based on font size
    pub fn bounds(&self, renderer: &mut TextRenderer) -> (f32, f32, f32, f32) {
        let width = self.measure_width(renderer);
        let height = self.font_size as f32 * 1.2; // Approximate line height
        (self.position.0, self.position.1, width, height)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_creation() {
        let text = Text::new("Hello World");
        assert_eq!(text.get_content(), "Hello World");
        assert_eq!(text.position, (0.0, 0.0));
        assert_eq!(text.font_size, 24);
    }

    #[test]
    fn text_from_signal() {
        let signal = Signal::new("Test".to_string());
        let text = Text::from_signal(signal.clone());

        assert_eq!(text.get_content(), "Test");

        // Update signal
        signal.set("Updated".to_string());
        assert_eq!(text.get_content(), "Updated");
    }

    #[test]
    fn text_builder_pattern() {
        let text = Text::new("Test")
            .position(10.0, 20.0)
            .font_size(32)
            .font_family(FontFamily::NotoSansBengali);

        assert_eq!(text.position, (10.0, 20.0));
        assert_eq!(text.font_size, 32);
    }

    #[test]
    fn text_set_content() {
        let text = Text::new("Initial");
        assert_eq!(text.get_content(), "Initial");

        text.set_content("Updated");
        assert_eq!(text.get_content(), "Updated");
    }

    #[test]
    fn text_reactive_updates() {
        let text = Text::new("Count: 0");

        // Simulate counter updates
        text.set_content("Count: 1");
        assert_eq!(text.get_content(), "Count: 1");

        text.set_content("Count: 2");
        assert_eq!(text.get_content(), "Count: 2");
    }

    #[test]
    fn text_clone() {
        let text1 = Text::new("Test");
        let text2 = text1.clone();

        // Both share the same signal
        text1.set_content("Updated");
        assert_eq!(text2.get_content(), "Updated");
    }

    #[test]
    fn text_measure_width() {
        let text = Text::new("Hello");
        let mut renderer = TextRenderer::new().unwrap();

        let width = text.measure_width(&mut renderer);
        assert!(width > 0.0);

        // Longer text should be wider
        text.set_content("Hello World!");
        let longer_width = text.measure_width(&mut renderer);
        assert!(longer_width > width);
    }

    #[test]
    fn text_bounds() {
        let text = Text::new("Test")
            .position(10.0, 20.0)
            .font_size(24);

        let mut renderer = TextRenderer::new().unwrap();
        let (x, y, w, h) = text.bounds(&mut renderer);

        assert_eq!(x, 10.0);
        assert_eq!(y, 20.0);
        assert!(w > 0.0);
        assert!(h > 0.0);
    }
}
