// Avatar Component - User avatar display
// Essential for user profiles and identity

use nebula_core::layout::{LayoutEngine, NodeId};
use nebula_core::signal::Signal;

/// Avatar size preset
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AvatarSize {
    Small,   // 32px
    Medium,  // 40px
    Large,   // 48px
    XLarge,  // 64px
}

impl AvatarSize {
    pub fn to_pixels(&self) -> f32 {
        match self {
            AvatarSize::Small => 32.0,
            AvatarSize::Medium => 40.0,
            AvatarSize::Large => 48.0,
            AvatarSize::XLarge => 64.0,
        }
    }
}

/// Avatar component - user avatar display
/// 
/// # Example
/// ```
/// let avatar = Avatar::new()
///     .image("user.png")
///     .size(AvatarSize::Large)
///     .fallback_text("JD");
/// ```
pub struct Avatar {
    pub node_id: Option<NodeId>,
    pub image: Signal<Option<String>>,
    pub fallback_text: Signal<Option<String>>,
    pub size_preset: AvatarSize,
    pub custom_size: Option<f32>,
    pub background_color: (u8, u8, u8, u8),
    pub text_color: (u8, u8, u8, u8),
    pub border_width: f32,
    pub border_color: (u8, u8, u8, u8),
    pub show_status: bool,
    pub status_color: (u8, u8, u8, u8),
    pub on_click: Option<Box<dyn Fn()>>,
}

impl Avatar {
    /// Create a new Avatar component
    pub fn new() -> Self {
        Self {
            node_id: None,
            image: Signal::new(None),
            fallback_text: Signal::new(None),
            size_preset: AvatarSize::Medium,
            custom_size: None,
            background_color: (156, 163, 175, 255), // Gray
            text_color: (255, 255, 255, 255),
            border_width: 0.0,
            border_color: (255, 255, 255, 255),
            show_status: false,
            status_color: (34, 197, 94, 255), // Green (online)
            on_click: None,
        }
    }

    /// Set the image URL
    pub fn image(self, url: impl Into<String>) -> Self {
        self.image.set(Some(url.into()));
        self
    }

    /// Set the fallback text (initials)
    pub fn fallback_text(self, text: impl Into<String>) -> Self {
        self.fallback_text.set(Some(text.into()));
        self
    }

    /// Set the size preset
    pub fn size(mut self, size: AvatarSize) -> Self {
        self.size_preset = size;
        self
    }

    /// Set a custom size
    pub fn custom_size(mut self, size: f32) -> Self {
        self.custom_size = Some(size);
        self
    }

    /// Set the background color
    pub fn background_color(mut self, r: u8, g: u8, b: u8, a: u8) -> Self {
        self.background_color = (r, g, b, a);
        self
    }

    /// Set the text color
    pub fn text_color(mut self, r: u8, g: u8, b: u8, a: u8) -> Self {
        self.text_color = (r, g, b, a);
        self
    }

    /// Set the border
    pub fn border(mut self, width: f32, r: u8, g: u8, b: u8, a: u8) -> Self {
        self.border_width = width;
        self.border_color = (r, g, b, a);
        self
    }

    /// Show status indicator
    pub fn show_status(mut self, show: bool) -> Self {
        self.show_status = show;
        self
    }

    /// Set the status color
    pub fn status_color(mut self, r: u8, g: u8, b: u8, a: u8) -> Self {
        self.status_color = (r, g, b, a);
        self
    }

    /// Set the click callback
    pub fn on_click<F>(mut self, callback: F) -> Self
    where
        F: Fn() + 'static,
    {
        self.on_click = Some(Box::new(callback));
        self
    }

    /// Get the image URL
    pub fn get_image(&self) -> Option<String> {
        self.image.get()
    }

    /// Set the image URL
    pub fn set_image(&mut self, url: Option<String>) {
        self.image.set(url);
    }

    /// Get the fallback text
    pub fn get_fallback_text(&self) -> Option<String> {
        self.fallback_text.get()
    }

    /// Set the fallback text
    pub fn set_fallback_text(&mut self, text: Option<String>) {
        self.fallback_text.set(text);
    }

    /// Get the effective size
    pub fn get_size(&self) -> f32 {
        self.custom_size.unwrap_or_else(|| self.size_preset.to_pixels())
    }

    /// Check if has image
    pub fn has_image(&self) -> bool {
        self.image.get().is_some()
    }

    /// Check if has fallback text
    pub fn has_fallback_text(&self) -> bool {
        self.fallback_text.get().is_some()
    }

    /// Handle click
    pub fn click(&mut self) {
        if let Some(ref callback) = self.on_click {
            callback();
        }
    }

    /// Build the avatar layout
    pub fn build(&mut self, engine: &mut LayoutEngine) -> Result<NodeId, String> {
        let size = self.get_size();

        let style = taffy::style::Style {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Length(size),
                height: taffy::style::Dimension::Length(size),
            },
            display: taffy::style::Display::Flex,
            justify_content: Some(taffy::style::JustifyContent::Center),
            align_items: Some(taffy::style::AlignItems::Center),
            ..Default::default()
        };

        let node = engine
            .new_leaf(style)
            .map_err(|e| format!("Failed to create avatar node: {:?}", e))?;
        self.node_id = Some(node);

        Ok(node)
    }
}

impl Default for Avatar {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn avatar_creation() {
        let avatar = Avatar::new();
        assert!(!avatar.has_image());
        assert!(!avatar.has_fallback_text());
    }

    #[test]
    fn avatar_with_image() {
        let avatar = Avatar::new().image("user.png");
        assert!(avatar.has_image());
        assert_eq!(avatar.get_image(), Some("user.png".to_string()));
    }

    #[test]
    fn avatar_with_fallback() {
        let avatar = Avatar::new().fallback_text("JD");
        assert!(avatar.has_fallback_text());
        assert_eq!(avatar.get_fallback_text(), Some("JD".to_string()));
    }

    #[test]
    fn avatar_set_image() {
        let mut avatar = Avatar::new();
        avatar.set_image(Some("user.png".to_string()));
        assert!(avatar.has_image());
    }

    #[test]
    fn avatar_set_fallback() {
        let mut avatar = Avatar::new();
        avatar.set_fallback_text(Some("JD".to_string()));
        assert!(avatar.has_fallback_text());
    }

    #[test]
    fn avatar_size_presets() {
        assert_eq!(AvatarSize::Small.to_pixels(), 32.0);
        assert_eq!(AvatarSize::Medium.to_pixels(), 40.0);
        assert_eq!(AvatarSize::Large.to_pixels(), 48.0);
        assert_eq!(AvatarSize::XLarge.to_pixels(), 64.0);
    }

    #[test]
    fn avatar_get_size() {
        let avatar = Avatar::new().size(AvatarSize::Large);
        assert_eq!(avatar.get_size(), 48.0);

        let avatar = Avatar::new().custom_size(100.0);
        assert_eq!(avatar.get_size(), 100.0);
    }

    #[test]
    fn avatar_click() {
        use std::sync::{Arc, Mutex};

        let clicked = Arc::new(Mutex::new(false));
        let clicked_clone = clicked.clone();

        let mut avatar = Avatar::new().on_click(move || {
            *clicked_clone.lock().unwrap() = true;
        });

        avatar.click();
        assert!(*clicked.lock().unwrap());
    }

    #[test]
    fn avatar_builder_pattern() {
        let avatar = Avatar::new()
            .image("user.png")
            .fallback_text("JD")
            .size(AvatarSize::Large)
            .custom_size(60.0)
            .background_color(59, 130, 246, 255)
            .text_color(255, 255, 255, 255)
            .border(2.0, 255, 255, 255, 255)
            .show_status(true)
            .status_color(34, 197, 94, 255);

        assert!(avatar.has_image());
        assert!(avatar.has_fallback_text());
        assert_eq!(avatar.size_preset, AvatarSize::Large);
        assert_eq!(avatar.get_size(), 60.0);
        assert_eq!(avatar.background_color, (59, 130, 246, 255));
        assert_eq!(avatar.text_color, (255, 255, 255, 255));
        assert_eq!(avatar.border_width, 2.0);
        assert_eq!(avatar.border_color, (255, 255, 255, 255));
        assert!(avatar.show_status);
        assert_eq!(avatar.status_color, (34, 197, 94, 255));
    }

    #[test]
    fn avatar_build_creates_node() {
        let mut engine = LayoutEngine::new();
        let mut avatar = Avatar::new();

        let result = avatar.build(&mut engine);
        assert!(result.is_ok());
        assert!(avatar.node_id.is_some());
    }
}
