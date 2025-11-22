// Chip Component - Chip/tag component for labels and selections
// Essential for tags, filters, and selections

use nebula_core::layout::{LayoutEngine, NodeId};
use nebula_core::signal::Signal;

/// Chip variant
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChipVariant {
    Filled,
    Outlined,
    Light,
}

/// Chip component - chip/tag for labels and selections
/// 
/// # Example
/// ```
/// let chip = Chip::new("React")
///     .variant(ChipVariant::Filled)
///     .closable(true)
///     .on_close(|| println!("Chip closed"));
/// ```
pub struct Chip {
    pub node_id: Option<NodeId>,
    pub label: Signal<String>,
    pub variant: ChipVariant,
    pub selected: Signal<bool>,
    pub disabled: bool,
    pub closable: bool,
    pub icon: Option<String>,
    pub avatar: Option<String>,
    pub height: f32,
    pub padding_x: f32,
    pub padding_y: f32,
    pub background_color: (u8, u8, u8, u8),
    pub text_color: (u8, u8, u8, u8),
    pub border_color: (u8, u8, u8, u8),
    pub selected_color: (u8, u8, u8, u8),
    pub border_radius: f32,
    pub on_click: Option<Box<dyn Fn()>>,
    pub on_close: Option<Box<dyn Fn()>>,
}

impl Chip {
    /// Create a new Chip component
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            node_id: None,
            label: Signal::new(label.into()),
            variant: ChipVariant::Filled,
            selected: Signal::new(false),
            disabled: false,
            closable: false,
            icon: None,
            avatar: None,
            height: 32.0,
            padding_x: 12.0,
            padding_y: 6.0,
            background_color: (229, 231, 235, 255), // Gray
            text_color: (31, 41, 55, 255),
            border_color: (209, 213, 219, 255),
            selected_color: (59, 130, 246, 255), // Blue
            border_radius: 16.0,
            on_click: None,
            on_close: None,
        }
    }

    /// Set the label
    pub fn label(self, label: impl Into<String>) -> Self {
        self.label.set(label.into());
        self
    }

    /// Set the variant
    pub fn variant(mut self, variant: ChipVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set selected state
    pub fn selected(self, selected: bool) -> Self {
        self.selected.set(selected);
        self
    }

    /// Set disabled state
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Set closable
    pub fn closable(mut self, closable: bool) -> Self {
        self.closable = closable;
        self
    }

    /// Set icon
    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Set avatar
    pub fn avatar(mut self, avatar: impl Into<String>) -> Self {
        self.avatar = Some(avatar.into());
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

    /// Set the text color
    pub fn text_color(mut self, r: u8, g: u8, b: u8, a: u8) -> Self {
        self.text_color = (r, g, b, a);
        self
    }

    /// Set the selected color
    pub fn selected_color(mut self, r: u8, g: u8, b: u8, a: u8) -> Self {
        self.selected_color = (r, g, b, a);
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

    /// Set the close callback
    pub fn on_close<F>(mut self, callback: F) -> Self
    where
        F: Fn() + 'static,
    {
        self.on_close = Some(Box::new(callback));
        self
    }

    /// Get the label
    pub fn get_label(&self) -> String {
        self.label.get()
    }

    /// Set the label
    pub fn set_label(&mut self, label: impl Into<String>) {
        self.label.set(label.into());
    }

    /// Toggle selected state
    pub fn toggle(&mut self) {
        if !self.disabled {
            self.selected.set(!self.is_selected());
        }
    }

    /// Check if selected
    pub fn is_selected(&self) -> bool {
        self.selected.get()
    }

    /// Handle click
    pub fn click(&mut self) {
        if !self.disabled {
            if let Some(ref callback) = self.on_click {
                callback();
            }
        }
    }

    /// Handle close
    pub fn close(&mut self) {
        if self.closable && !self.disabled {
            if let Some(ref callback) = self.on_close {
                callback();
            }
        }
    }

    /// Check if has icon
    pub fn has_icon(&self) -> bool {
        self.icon.is_some()
    }

    /// Check if has avatar
    pub fn has_avatar(&self) -> bool {
        self.avatar.is_some()
    }

    /// Build the chip layout
    pub fn build(&mut self, engine: &mut LayoutEngine) -> Result<NodeId, String> {
        let style = taffy::style::Style {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Auto,
                height: taffy::style::Dimension::Length(self.height),
            },
            padding: taffy::geometry::Rect {
                left: taffy::style::LengthPercentage::Length(self.padding_x),
                right: taffy::style::LengthPercentage::Length(self.padding_x),
                top: taffy::style::LengthPercentage::Length(self.padding_y),
                bottom: taffy::style::LengthPercentage::Length(self.padding_y),
            },
            display: taffy::style::Display::Flex,
            flex_direction: taffy::style::FlexDirection::Row,
            align_items: Some(taffy::style::AlignItems::Center),
            gap: taffy::geometry::Size {
                width: taffy::style::LengthPercentage::Length(8.0),
                height: taffy::style::LengthPercentage::Length(0.0),
            },
            ..Default::default()
        };

        let node = engine
            .new_leaf(style)
            .map_err(|e| format!("Failed to create chip node: {:?}", e))?;
        self.node_id = Some(node);

        Ok(node)
    }
}

impl Default for Chip {
    fn default() -> Self {
        Self::new("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chip_creation() {
        let chip = Chip::new("React");
        assert_eq!(chip.get_label(), "React");
        assert!(!chip.is_selected());
    }

    #[test]
    fn chip_set_label() {
        let mut chip = Chip::new("React");
        chip.set_label("Vue");
        assert_eq!(chip.get_label(), "Vue");
    }

    #[test]
    fn chip_toggle() {
        let mut chip = Chip::new("React");
        chip.toggle();
        assert!(chip.is_selected());
        chip.toggle();
        assert!(!chip.is_selected());
    }

    #[test]
    fn chip_disabled_cannot_toggle() {
        let mut chip = Chip::new("React").disabled(true);
        chip.toggle();
        assert!(!chip.is_selected());
    }

    #[test]
    fn chip_click() {
        use std::sync::{Arc, Mutex};

        let clicked = Arc::new(Mutex::new(false));
        let clicked_clone = clicked.clone();

        let mut chip = Chip::new("React").on_click(move || {
            *clicked_clone.lock().unwrap() = true;
        });

        chip.click();
        assert!(*clicked.lock().unwrap());
    }

    #[test]
    fn chip_close() {
        use std::sync::{Arc, Mutex};

        let closed = Arc::new(Mutex::new(false));
        let closed_clone = closed.clone();

        let mut chip = Chip::new("React")
            .closable(true)
            .on_close(move || {
                *closed_clone.lock().unwrap() = true;
            });

        chip.close();
        assert!(*closed.lock().unwrap());
    }

    #[test]
    fn chip_cannot_close_if_not_closable() {
        use std::sync::{Arc, Mutex};

        let closed = Arc::new(Mutex::new(false));
        let closed_clone = closed.clone();

        let mut chip = Chip::new("React")
            .closable(false)
            .on_close(move || {
                *closed_clone.lock().unwrap() = true;
            });

        chip.close();
        assert!(!*closed.lock().unwrap());
    }

    #[test]
    fn chip_with_icon() {
        let chip = Chip::new("React").icon("⚛️");
        assert!(chip.has_icon());
        assert_eq!(chip.icon, Some("⚛️".to_string()));
    }

    #[test]
    fn chip_with_avatar() {
        let chip = Chip::new("John").avatar("avatar.png");
        assert!(chip.has_avatar());
        assert_eq!(chip.avatar, Some("avatar.png".to_string()));
    }

    #[test]
    fn chip_variants() {
        let chip = Chip::new("React").variant(ChipVariant::Outlined);
        assert_eq!(chip.variant, ChipVariant::Outlined);
    }

    #[test]
    fn chip_builder_pattern() {
        let chip = Chip::new("React")
            .variant(ChipVariant::Filled)
            .selected(true)
            .disabled(true)
            .closable(true)
            .icon("⚛️")
            .avatar("avatar.png")
            .height(40.0)
            .background_color(59, 130, 246, 255)
            .text_color(255, 255, 255, 255)
            .selected_color(37, 99, 235, 255);

        assert_eq!(chip.get_label(), "React");
        assert_eq!(chip.variant, ChipVariant::Filled);
        assert!(chip.is_selected());
        assert!(chip.disabled);
        assert!(chip.closable);
        assert!(chip.has_icon());
        assert!(chip.has_avatar());
        assert_eq!(chip.height, 40.0);
        assert_eq!(chip.background_color, (59, 130, 246, 255));
        assert_eq!(chip.text_color, (255, 255, 255, 255));
        assert_eq!(chip.selected_color, (37, 99, 235, 255));
    }

    #[test]
    fn chip_build_creates_node() {
        let mut engine = LayoutEngine::new();
        let mut chip = Chip::new("React");

        let result = chip.build(&mut engine);
        assert!(result.is_ok());
        assert!(chip.node_id.is_some());
    }
}
