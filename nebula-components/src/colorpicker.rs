// ColorPicker Component - Color selection component
// Essential for color input in design tools and forms

use nebula_core::layout::{LayoutEngine, NodeId};
use nebula_core::signal::Signal;

/// Color representation (RGBA)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    /// Create a new color
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    /// Create an RGB color (opaque)
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self::new(r, g, b, 255)
    }

    /// Create from hex string (#RRGGBB or #RRGGBBAA)
    pub fn from_hex(hex: &str) -> Option<Self> {
        let hex = hex.trim_start_matches('#');
        if hex.len() == 6 {
            let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
            let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
            let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
            Some(Self::rgb(r, g, b))
        } else if hex.len() == 8 {
            let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
            let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
            let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
            let a = u8::from_str_radix(&hex[6..8], 16).ok()?;
            Some(Self::new(r, g, b, a))
        } else {
            None
        }
    }

    /// Convert to hex string (#RRGGBB)
    pub fn to_hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }

    /// Convert to hex string with alpha (#RRGGBBAA)
    pub fn to_hex_alpha(&self) -> String {
        format!("#{:02X}{:02X}{:02X}{:02X}", self.r, self.g, self.b, self.a)
    }

    /// Convert to HSV (Hue, Saturation, Value)
    pub fn to_hsv(&self) -> (f32, f32, f32) {
        let r = self.r as f32 / 255.0;
        let g = self.g as f32 / 255.0;
        let b = self.b as f32 / 255.0;

        let max = r.max(g).max(b);
        let min = r.min(g).min(b);
        let delta = max - min;

        let h = if delta == 0.0 {
            0.0
        } else if max == r {
            60.0 * (((g - b) / delta) % 6.0)
        } else if max == g {
            60.0 * (((b - r) / delta) + 2.0)
        } else {
            60.0 * (((r - g) / delta) + 4.0)
        };

        let s = if max == 0.0 { 0.0 } else { delta / max };
        let v = max;

        (h, s, v)
    }

    /// Create from HSV (Hue, Saturation, Value)
    pub fn from_hsv(h: f32, s: f32, v: f32) -> Self {
        let c = v * s;
        let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
        let m = v - c;

        let (r, g, b) = if h < 60.0 {
            (c, x, 0.0)
        } else if h < 120.0 {
            (x, c, 0.0)
        } else if h < 180.0 {
            (0.0, c, x)
        } else if h < 240.0 {
            (0.0, x, c)
        } else if h < 300.0 {
            (x, 0.0, c)
        } else {
            (c, 0.0, x)
        };

        Self::rgb(
            ((r + m) * 255.0) as u8,
            ((g + m) * 255.0) as u8,
            ((b + m) * 255.0) as u8,
        )
    }
}

/// ColorPicker component - color selection component
/// 
/// # Example
/// ```
/// let mut colorpicker = ColorPicker::new()
///     .selected_color(Color::rgb(59, 130, 246))
///     .show_alpha(true)
///     .on_change(|color| println!("Selected: {}", color.to_hex()));
/// ```
pub struct ColorPicker {
    pub node_id: Option<NodeId>,
    pub selected_color: Signal<Color>,
    pub show_alpha: bool,
    pub show_hex_input: bool,
    pub disabled: bool,
    pub show_picker: Signal<bool>,
    pub width: f32,
    pub height: f32,
    pub picker_width: f32,
    pub picker_height: f32,
    pub on_change: Option<Box<dyn Fn(Color)>>,
}

impl ColorPicker {
    /// Create a new ColorPicker component
    pub fn new() -> Self {
        Self {
            node_id: None,
            selected_color: Signal::new(Color::rgb(255, 255, 255)),
            show_alpha: false,
            show_hex_input: true,
            disabled: false,
            show_picker: Signal::new(false),
            width: 200.0,
            height: 40.0,
            picker_width: 280.0,
            picker_height: 320.0,
            on_change: None,
        }
    }

    /// Set the selected color
    pub fn selected_color(self, color: Color) -> Self {
        self.selected_color.set(color);
        self
    }

    /// Show or hide alpha channel control
    pub fn show_alpha(mut self, show: bool) -> Self {
        self.show_alpha = show;
        self
    }

    /// Show or hide hex input field
    pub fn show_hex_input(mut self, show: bool) -> Self {
        self.show_hex_input = show;
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
        F: Fn(Color) + 'static,
    {
        self.on_change = Some(Box::new(callback));
        self
    }

    /// Select a color
    pub fn select_color(&mut self, color: Color) {
        if !self.disabled {
            self.selected_color.set(color);
            if let Some(ref callback) = self.on_change {
                callback(color);
            }
        }
    }

    /// Get the selected color
    pub fn get_selected_color(&self) -> Color {
        self.selected_color.get()
    }

    /// Set color from hex string
    pub fn set_from_hex(&mut self, hex: &str) -> Result<(), String> {
        if let Some(color) = Color::from_hex(hex) {
            self.select_color(color);
            Ok(())
        } else {
            Err("Invalid hex color".to_string())
        }
    }

    /// Get color as hex string
    pub fn get_hex(&self) -> String {
        self.get_selected_color().to_hex()
    }

    /// Show the color picker
    pub fn show(&mut self) {
        if !self.disabled {
            self.show_picker.set(true);
        }
    }

    /// Hide the color picker
    pub fn hide(&mut self) {
        self.show_picker.set(false);
    }

    /// Toggle the color picker
    pub fn toggle(&mut self) {
        if !self.disabled {
            self.show_picker.set(!self.is_picker_visible());
        }
    }

    /// Check if picker is visible
    pub fn is_picker_visible(&self) -> bool {
        self.show_picker.get()
    }

    /// Build the colorpicker layout
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
            .map_err(|e| format!("Failed to create colorpicker node: {:?}", e))?;
        self.node_id = Some(node);

        Ok(node)
    }
}

impl Default for ColorPicker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn colorpicker_starts_white() {
        let colorpicker = ColorPicker::new();
        assert_eq!(colorpicker.get_selected_color(), Color::rgb(255, 255, 255));
    }

    #[test]
    fn colorpicker_select_color() {
        let mut colorpicker = ColorPicker::new();
        let color = Color::rgb(59, 130, 246);
        colorpicker.select_color(color);
        assert_eq!(colorpicker.get_selected_color(), color);
    }

    #[test]
    fn colorpicker_disabled_cannot_select() {
        let mut colorpicker = ColorPicker::new().disabled(true);
        colorpicker.select_color(Color::rgb(255, 0, 0));
        assert_eq!(colorpicker.get_selected_color(), Color::rgb(255, 255, 255));
    }

    #[test]
    fn colorpicker_show_hide_picker() {
        let mut colorpicker = ColorPicker::new();
        assert!(!colorpicker.is_picker_visible());
        
        colorpicker.show();
        assert!(colorpicker.is_picker_visible());
        
        colorpicker.hide();
        assert!(!colorpicker.is_picker_visible());
    }

    #[test]
    fn colorpicker_toggle_picker() {
        let mut colorpicker = ColorPicker::new();
        colorpicker.toggle();
        assert!(colorpicker.is_picker_visible());
        colorpicker.toggle();
        assert!(!colorpicker.is_picker_visible());
    }

    #[test]
    fn colorpicker_hex_conversion() {
        let mut colorpicker = ColorPicker::new();
        colorpicker.set_from_hex("#3B82F6").unwrap();
        assert_eq!(colorpicker.get_hex(), "#3B82F6");
    }

    #[test]
    fn color_from_hex() {
        let color = Color::from_hex("#FF0000").unwrap();
        assert_eq!(color, Color::rgb(255, 0, 0));
        
        let color = Color::from_hex("#00FF00FF").unwrap();
        assert_eq!(color, Color::new(0, 255, 0, 255));
    }

    #[test]
    fn color_to_hex() {
        let color = Color::rgb(255, 0, 0);
        assert_eq!(color.to_hex(), "#FF0000");
        
        let color = Color::new(0, 255, 0, 128);
        assert_eq!(color.to_hex_alpha(), "#00FF0080");
    }

    #[test]
    fn color_hsv_conversion() {
        let color = Color::rgb(255, 0, 0);
        let (h, s, v) = color.to_hsv();
        assert!((h - 0.0).abs() < 0.1);
        assert!((s - 1.0).abs() < 0.1);
        assert!((v - 1.0).abs() < 0.1);
        
        let converted = Color::from_hsv(h, s, v);
        assert_eq!(converted.r, 255);
        assert_eq!(converted.g, 0);
        assert_eq!(converted.b, 0);
    }

    #[test]
    fn colorpicker_on_change_callback() {
        use std::sync::{Arc, Mutex};

        let changed = Arc::new(Mutex::new(Color::rgb(0, 0, 0)));
        let changed_clone = changed.clone();

        let mut colorpicker = ColorPicker::new().on_change(move |color| {
            *changed_clone.lock().unwrap() = color;
        });

        let color = Color::rgb(255, 0, 0);
        colorpicker.select_color(color);
        assert_eq!(*changed.lock().unwrap(), color);
    }

    #[test]
    fn colorpicker_builder_pattern() {
        let colorpicker = ColorPicker::new()
            .selected_color(Color::rgb(59, 130, 246))
            .show_alpha(true)
            .show_hex_input(false)
            .disabled(true)
            .width(250.0)
            .height(45.0);

        assert_eq!(colorpicker.get_selected_color(), Color::rgb(59, 130, 246));
        assert!(colorpicker.show_alpha);
        assert!(!colorpicker.show_hex_input);
        assert!(colorpicker.disabled);
        assert_eq!(colorpicker.width, 250.0);
        assert_eq!(colorpicker.height, 45.0);
    }

    #[test]
    fn colorpicker_build_creates_node() {
        let mut engine = LayoutEngine::new();
        let mut colorpicker = ColorPicker::new();

        let result = colorpicker.build(&mut engine);
        assert!(result.is_ok());
        assert!(colorpicker.node_id.is_some());
    }

    #[test]
    fn color_invalid_hex() {
        assert!(Color::from_hex("#GGGGGG").is_none());
        assert!(Color::from_hex("#12345").is_none());
    }
}
