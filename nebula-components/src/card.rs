// Card Component - Card container for content grouping
// Essential for organizing content in sections

use nebula_core::layout::{LayoutEngine, NodeId};
use nebula_core::signal::Signal;

/// Card variant
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CardVariant {
    Elevated,
    Outlined,
    Filled,
}

/// Card component - container for content grouping
/// 
/// # Example
/// ```
/// let card = Card::new()
///     .title("User Profile")
///     .subtitle("View and edit your profile")
///     .variant(CardVariant::Elevated)
///     .padding(16.0);
/// ```
pub struct Card {
    pub node_id: Option<NodeId>,
    pub title: Signal<Option<String>>,
    pub subtitle: Signal<Option<String>>,
    pub variant: CardVariant,
    pub width: f32,
    pub height: f32,
    pub padding: f32,
    pub background_color: (u8, u8, u8, u8),
    pub border_color: (u8, u8, u8, u8),
    pub border_width: f32,
    pub border_radius: f32,
    pub shadow_elevation: u8,
    pub hoverable: bool,
    pub clickable: bool,
    pub on_click: Option<Box<dyn Fn()>>,
}

impl Card {
    /// Create a new Card component
    pub fn new() -> Self {
        Self {
            node_id: None,
            title: Signal::new(None),
            subtitle: Signal::new(None),
            variant: CardVariant::Elevated,
            width: 300.0,
            height: 200.0,
            padding: 16.0,
            background_color: (255, 255, 255, 255),
            border_color: (229, 231, 235, 255),
            border_width: 1.0,
            border_radius: 8.0,
            shadow_elevation: 2,
            hoverable: false,
            clickable: false,
            on_click: None,
        }
    }

    /// Set the title
    pub fn title(self, title: impl Into<String>) -> Self {
        self.title.set(Some(title.into()));
        self
    }

    /// Set the subtitle
    pub fn subtitle(self, subtitle: impl Into<String>) -> Self {
        self.subtitle.set(Some(subtitle.into()));
        self
    }

    /// Set the variant
    pub fn variant(mut self, variant: CardVariant) -> Self {
        self.variant = variant;
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

    /// Set the padding
    pub fn padding(mut self, padding: f32) -> Self {
        self.padding = padding;
        self
    }

    /// Set the background color
    pub fn background_color(mut self, r: u8, g: u8, b: u8, a: u8) -> Self {
        self.background_color = (r, g, b, a);
        self
    }

    /// Set the border
    pub fn border(mut self, width: f32, r: u8, g: u8, b: u8, a: u8) -> Self {
        self.border_width = width;
        self.border_color = (r, g, b, a);
        self
    }

    /// Set the border radius
    pub fn border_radius(mut self, radius: f32) -> Self {
        self.border_radius = radius;
        self
    }

    /// Set the shadow elevation (0-5)
    pub fn shadow_elevation(mut self, elevation: u8) -> Self {
        self.shadow_elevation = elevation.min(5);
        self
    }

    /// Set hoverable state
    pub fn hoverable(mut self, hoverable: bool) -> Self {
        self.hoverable = hoverable;
        self
    }

    /// Set clickable state
    pub fn clickable(mut self, clickable: bool) -> Self {
        self.clickable = clickable;
        self
    }

    /// Set the click callback
    pub fn on_click<F>(mut self, callback: F) -> Self
    where
        F: Fn() + 'static,
    {
        self.on_click = Some(Box::new(callback));
        self.clickable = true;
        self
    }

    /// Get the title
    pub fn get_title(&self) -> Option<String> {
        self.title.get()
    }

    /// Set the title
    pub fn set_title(&mut self, title: Option<String>) {
        self.title.set(title);
    }

    /// Get the subtitle
    pub fn get_subtitle(&self) -> Option<String> {
        self.subtitle.get()
    }

    /// Set the subtitle
    pub fn set_subtitle(&mut self, subtitle: Option<String>) {
        self.subtitle.set(subtitle);
    }

    /// Check if has title
    pub fn has_title(&self) -> bool {
        self.title.get().is_some()
    }

    /// Check if has subtitle
    pub fn has_subtitle(&self) -> bool {
        self.subtitle.get().is_some()
    }

    /// Handle click
    pub fn click(&mut self) {
        if self.clickable {
            if let Some(ref callback) = self.on_click {
                callback();
            }
        }
    }

    /// Build the card layout
    pub fn build(&mut self, engine: &mut LayoutEngine) -> Result<NodeId, String> {
        let style = taffy::style::Style {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Length(self.width),
                height: taffy::style::Dimension::Length(self.height),
            },
            padding: taffy::geometry::Rect {
                left: taffy::style::LengthPercentage::Length(self.padding),
                right: taffy::style::LengthPercentage::Length(self.padding),
                top: taffy::style::LengthPercentage::Length(self.padding),
                bottom: taffy::style::LengthPercentage::Length(self.padding),
            },
            display: taffy::style::Display::Flex,
            flex_direction: taffy::style::FlexDirection::Column,
            ..Default::default()
        };

        let node = engine
            .new_leaf(style)
            .map_err(|e| format!("Failed to create card node: {:?}", e))?;
        self.node_id = Some(node);

        Ok(node)
    }
}

impl Default for Card {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn card_creation() {
        let card = Card::new();
        assert!(!card.has_title());
        assert!(!card.has_subtitle());
    }

    #[test]
    fn card_with_title() {
        let card = Card::new().title("User Profile");
        assert!(card.has_title());
        assert_eq!(card.get_title(), Some("User Profile".to_string()));
    }

    #[test]
    fn card_with_subtitle() {
        let card = Card::new().subtitle("View your profile");
        assert!(card.has_subtitle());
        assert_eq!(card.get_subtitle(), Some("View your profile".to_string()));
    }

    #[test]
    fn card_set_title() {
        let mut card = Card::new();
        card.set_title(Some("New Title".to_string()));
        assert!(card.has_title());
    }

    #[test]
    fn card_set_subtitle() {
        let mut card = Card::new();
        card.set_subtitle(Some("New Subtitle".to_string()));
        assert!(card.has_subtitle());
    }

    #[test]
    fn card_variants() {
        let card = Card::new().variant(CardVariant::Outlined);
        assert_eq!(card.variant, CardVariant::Outlined);
    }

    #[test]
    fn card_clickable() {
        let card = Card::new().clickable(true);
        assert!(card.clickable);
    }

    #[test]
    fn card_click() {
        use std::sync::{Arc, Mutex};

        let clicked = Arc::new(Mutex::new(false));
        let clicked_clone = clicked.clone();

        let mut card = Card::new().on_click(move || {
            *clicked_clone.lock().unwrap() = true;
        });

        card.click();
        assert!(*clicked.lock().unwrap());
    }

    #[test]
    fn card_shadow_elevation() {
        let card = Card::new().shadow_elevation(3);
        assert_eq!(card.shadow_elevation, 3);
        
        let card = Card::new().shadow_elevation(10);
        assert_eq!(card.shadow_elevation, 5); // Clamped to max
    }

    #[test]
    fn card_builder_pattern() {
        let card = Card::new()
            .title("User Profile")
            .subtitle("View your profile")
            .variant(CardVariant::Elevated)
            .width(400.0)
            .height(300.0)
            .padding(20.0)
            .background_color(255, 255, 255, 255)
            .border(2.0, 200, 200, 200, 255)
            .border_radius(12.0)
            .shadow_elevation(3)
            .hoverable(true)
            .clickable(true);

        assert!(card.has_title());
        assert!(card.has_subtitle());
        assert_eq!(card.variant, CardVariant::Elevated);
        assert_eq!(card.width, 400.0);
        assert_eq!(card.height, 300.0);
        assert_eq!(card.padding, 20.0);
        assert_eq!(card.background_color, (255, 255, 255, 255));
        assert_eq!(card.border_width, 2.0);
        assert_eq!(card.border_color, (200, 200, 200, 255));
        assert_eq!(card.border_radius, 12.0);
        assert_eq!(card.shadow_elevation, 3);
        assert!(card.hoverable);
        assert!(card.clickable);
    }

    #[test]
    fn card_build_creates_node() {
        let mut engine = LayoutEngine::new();
        let mut card = Card::new();

        let result = card.build(&mut engine);
        assert!(result.is_ok());
        assert!(card.node_id.is_some());
    }
}
