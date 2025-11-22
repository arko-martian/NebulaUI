use nebula_core::{LayoutEngine, NodeId, Layout};
use taffy::prelude::*;
use tracing::info;

/// Divider - Visual separator for layouts ➖
/// 
/// Creates a thin line to separate content sections.
/// Essential for clean, organized UIs!
/// 
/// - Horizontal divider: Full width, thin height
/// - Vertical divider: Full height, thin width
/// - Customizable thickness and color
/// 
/// Just like Material Design's Divider!
#[derive(Clone)]
pub struct Divider {
    /// Layout node ID
    pub node_id: Option<NodeId>,
    /// Divider orientation
    pub orientation: DividerOrientation,
    /// Thickness in pixels
    pub thickness: f32,
    /// Color (for future rendering)
    pub color: DividerColor,
}

/// Divider orientation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DividerOrientation {
    /// Horizontal divider (spans width)
    Horizontal,
    /// Vertical divider (spans height)
    Vertical,
}

/// Divider color (simplified for now)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DividerColor {
    /// Light gray (default)
    Light,
    /// Medium gray
    Medium,
    /// Dark gray
    Dark,
    /// Custom RGB (for future)
    Custom { r: u8, g: u8, b: u8 },
}

impl Divider {
    /// Create a horizontal divider (default)
    /// Spans full width, 1px tall
    pub fn new() -> Self {
        info!("➖ Creating horizontal Divider");
        Self {
            node_id: None,
            orientation: DividerOrientation::Horizontal,
            thickness: 1.0,
            color: DividerColor::Light,
        }
    }

    /// Create a horizontal divider
    pub fn horizontal() -> Self {
        Self::new()
    }

    /// Create a vertical divider
    /// Spans full height, 1px wide
    pub fn vertical() -> Self {
        info!("➖ Creating vertical Divider");
        Self {
            node_id: None,
            orientation: DividerOrientation::Vertical,
            thickness: 1.0,
            color: DividerColor::Light,
        }
    }

    /// Set thickness
    pub fn thickness(mut self, thickness: f32) -> Self {
        self.thickness = thickness;
        self
    }

    /// Set color
    pub fn color(mut self, color: DividerColor) -> Self {
        self.color = color;
        self
    }

    /// Build the layout node
    pub fn build(&mut self, engine: &mut LayoutEngine) -> Result<NodeId, String> {
        let style = match self.orientation {
            DividerOrientation::Horizontal => Style {
                size: Size {
                    width: Dimension::Percent(1.0), // 100% width
                    height: Dimension::Length(self.thickness),
                },
                ..Default::default()
            },
            DividerOrientation::Vertical => Style {
                size: Size {
                    width: Dimension::Length(self.thickness),
                    height: Dimension::Percent(1.0), // 100% height
                },
                ..Default::default()
            },
        };

        let node = engine
            .new_leaf(style)
            .map_err(|e| format!("Failed to create Divider: {:?}", e))?;

        self.node_id = Some(node);
        info!("✅ Divider built ({:?}, {}px)", self.orientation, self.thickness);
        Ok(node)
    }

    /// Get the layout
    pub fn get_layout(&self, engine: &LayoutEngine) -> Option<Layout> {
        self.node_id.and_then(|id| engine.get_layout(id).ok())
    }

    /// Get orientation
    pub fn orientation(&self) -> DividerOrientation {
        self.orientation
    }

    /// Get thickness
    pub fn get_thickness(&self) -> f32 {
        self.thickness
    }

    /// Get color
    pub fn get_color(&self) -> DividerColor {
        self.color
    }
}

impl Default for Divider {
    fn default() -> Self {
        Self::new()
    }
}

impl DividerColor {
    /// Convert to RGB values (0-255)
    pub fn to_rgb(&self) -> (u8, u8, u8) {
        match self {
            DividerColor::Light => (220, 220, 220),    // #DCDCDC
            DividerColor::Medium => (160, 160, 160),   // #A0A0A0
            DividerColor::Dark => (80, 80, 80),        // #505050
            DividerColor::Custom { r, g, b } => (*r, *g, *b),
        }
    }

    /// Convert to hex string
    pub fn to_hex(&self) -> String {
        let (r, g, b) = self.to_rgb();
        format!("#{:02X}{:02X}{:02X}", r, g, b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn divider_horizontal_creation() {
        let divider = Divider::new();
        assert_eq!(divider.orientation, DividerOrientation::Horizontal);
        assert_eq!(divider.thickness, 1.0);
        assert_eq!(divider.color, DividerColor::Light);
    }

    #[test]
    fn divider_vertical_creation() {
        let divider = Divider::vertical();
        assert_eq!(divider.orientation, DividerOrientation::Vertical);
        assert_eq!(divider.thickness, 1.0);
    }

    #[test]
    fn divider_builder_pattern() {
        let divider = Divider::horizontal()
            .thickness(2.0)
            .color(DividerColor::Dark);

        assert_eq!(divider.thickness, 2.0);
        assert_eq!(divider.color, DividerColor::Dark);
    }

    #[test]
    fn divider_default() {
        let divider = Divider::default();
        assert_eq!(divider.orientation, DividerOrientation::Horizontal);
    }

    #[test]
    fn divider_build_horizontal() {
        let mut engine = LayoutEngine::new();
        let mut divider = Divider::horizontal();

        let node = divider.build(&mut engine);
        assert!(node.is_ok());
        assert!(divider.node_id.is_some());
    }

    #[test]
    fn divider_build_vertical() {
        let mut engine = LayoutEngine::new();
        let mut divider = Divider::vertical();

        let node = divider.build(&mut engine);
        assert!(node.is_ok());
        assert!(divider.node_id.is_some());
    }

    #[test]
    fn divider_horizontal_layout() {
        let mut engine = LayoutEngine::new();
        let mut divider = Divider::horizontal().thickness(2.0);

        let node = divider.build(&mut engine).unwrap();

        // Compute layout in a 200x100 container
        let available = Size {
            width: AvailableSpace::Definite(200.0),
            height: AvailableSpace::Definite(100.0),
        };
        engine.compute_layout(node, available).unwrap();

        let layout = divider.get_layout(&engine);
        assert!(layout.is_some());

        let layout = layout.unwrap();
        // Horizontal divider should span full width
        assert_eq!(layout.size.width, 200.0);
        assert_eq!(layout.size.height, 2.0);
    }

    #[test]
    fn divider_vertical_layout() {
        let mut engine = LayoutEngine::new();
        let mut divider = Divider::vertical().thickness(3.0);

        let node = divider.build(&mut engine).unwrap();

        // Compute layout in a 200x100 container
        let available = Size {
            width: AvailableSpace::Definite(200.0),
            height: AvailableSpace::Definite(100.0),
        };
        engine.compute_layout(node, available).unwrap();

        let layout = divider.get_layout(&engine);
        assert!(layout.is_some());

        let layout = layout.unwrap();
        // Vertical divider should span full height
        assert_eq!(layout.size.width, 3.0);
        assert_eq!(layout.size.height, 100.0);
    }

    #[test]
    fn divider_in_vstack() {
        let mut engine = LayoutEngine::new();
        use nebula_core::layout::styles;

        // Create VStack with: [Box] [Divider] [Box]
        let box1 = engine.new_leaf(styles::fixed_size(100.0, 50.0)).unwrap();

        let mut divider = Divider::horizontal();
        let divider_node = divider.build(&mut engine).unwrap();

        let box2 = engine.new_leaf(styles::fixed_size(100.0, 50.0)).unwrap();

        // Create VStack
        let vstack = engine
            .create_vstack(&[box1, divider_node, box2])
            .unwrap();

        // Compute layout
        let available = Size {
            width: AvailableSpace::Definite(200.0),
            height: AvailableSpace::Definite(200.0),
        };
        engine.compute_layout(vstack, available).unwrap();

        // Divider should span the width
        let layout = divider.get_layout(&engine);
        assert!(layout.is_some());
    }

    #[test]
    fn divider_color_light() {
        let color = DividerColor::Light;
        assert_eq!(color.to_rgb(), (220, 220, 220));
        assert_eq!(color.to_hex(), "#DCDCDC");
    }

    #[test]
    fn divider_color_medium() {
        let color = DividerColor::Medium;
        assert_eq!(color.to_rgb(), (160, 160, 160));
        assert_eq!(color.to_hex(), "#A0A0A0");
    }

    #[test]
    fn divider_color_dark() {
        let color = DividerColor::Dark;
        assert_eq!(color.to_rgb(), (80, 80, 80));
        assert_eq!(color.to_hex(), "#505050");
    }

    #[test]
    fn divider_color_custom() {
        let color = DividerColor::Custom {
            r: 255,
            g: 128,
            b: 64,
        };
        assert_eq!(color.to_rgb(), (255, 128, 64));
        assert_eq!(color.to_hex(), "#FF8040");
    }

    #[test]
    fn divider_getters() {
        let divider = Divider::vertical().thickness(5.0).color(DividerColor::Dark);

        assert_eq!(divider.orientation(), DividerOrientation::Vertical);
        assert_eq!(divider.get_thickness(), 5.0);
        assert_eq!(divider.get_color(), DividerColor::Dark);
    }

    #[test]
    fn divider_orientation_equality() {
        assert_eq!(
            DividerOrientation::Horizontal,
            DividerOrientation::Horizontal
        );
        assert_ne!(
            DividerOrientation::Horizontal,
            DividerOrientation::Vertical
        );
    }
}
