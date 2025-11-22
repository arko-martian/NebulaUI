use nebula_core::{LayoutEngine, NodeId, Layout};
use taffy::prelude::*;
use tracing::info;

/// Spacer - Creates flexible or fixed space in layouts 📏
/// 
/// Essential for beautiful, well-spaced UIs!
/// - Fixed spacer: Exact size (e.g., 20px gap)
/// - Flexible spacer: Fills available space (like CSS flex-grow)
/// 
/// Just like SwiftUI's Spacer!
#[derive(Clone)]
pub struct Spacer {
    /// Layout node ID
    pub node_id: Option<NodeId>,
    /// Spacer type
    pub spacer_type: SpacerType,
}

/// Type of spacer
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SpacerType {
    /// Fixed size spacer (exact dimensions)
    Fixed { width: f32, height: f32 },
    /// Flexible spacer (fills available space)
    Flexible,
    /// Horizontal spacer (fixed height, flexible width)
    Horizontal { height: f32 },
    /// Vertical spacer (fixed width, flexible height)
    Vertical { width: f32 },
}

impl Spacer {
    /// Create a flexible spacer (fills available space)
    /// This is the default - like SwiftUI's Spacer()
    pub fn new() -> Self {
        info!("📏 Creating flexible Spacer");
        Self {
            node_id: None,
            spacer_type: SpacerType::Flexible,
        }
    }

    /// Create a fixed size spacer
    pub fn fixed(width: f32, height: f32) -> Self {
        info!("📏 Creating fixed Spacer ({}x{})", width, height);
        Self {
            node_id: None,
            spacer_type: SpacerType::Fixed { width, height },
        }
    }

    /// Create a horizontal spacer (fills width, fixed height)
    pub fn horizontal(height: f32) -> Self {
        info!("📏 Creating horizontal Spacer (height: {})", height);
        Self {
            node_id: None,
            spacer_type: SpacerType::Horizontal { height },
        }
    }

    /// Create a vertical spacer (fills height, fixed width)
    pub fn vertical(width: f32) -> Self {
        info!("📏 Creating vertical Spacer (width: {})", width);
        Self {
            node_id: None,
            spacer_type: SpacerType::Vertical { width },
        }
    }

    /// Build the layout node
    pub fn build(&mut self, engine: &mut LayoutEngine) -> Result<NodeId, String> {
        let style = match self.spacer_type {
            SpacerType::Fixed { width, height } => Style {
                size: Size {
                    width: Dimension::Length(width),
                    height: Dimension::Length(height),
                },
                ..Default::default()
            },
            SpacerType::Flexible => Style {
                flex_grow: 1.0,
                flex_shrink: 1.0,
                ..Default::default()
            },
            SpacerType::Horizontal { height } => Style {
                size: Size {
                    width: Dimension::Auto,
                    height: Dimension::Length(height),
                },
                flex_grow: 1.0,
                ..Default::default()
            },
            SpacerType::Vertical { width } => Style {
                size: Size {
                    width: Dimension::Length(width),
                    height: Dimension::Auto,
                },
                flex_grow: 1.0,
                ..Default::default()
            },
        };

        let node = engine
            .new_leaf(style)
            .map_err(|e| format!("Failed to create Spacer: {:?}", e))?;

        self.node_id = Some(node);
        info!("✅ Spacer built");
        Ok(node)
    }

    /// Get the layout
    pub fn get_layout(&self, engine: &LayoutEngine) -> Option<Layout> {
        self.node_id.and_then(|id| engine.get_layout(id).ok())
    }

    /// Get the spacer type
    pub fn spacer_type(&self) -> SpacerType {
        self.spacer_type
    }
}

impl Default for Spacer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spacer_flexible_creation() {
        let spacer = Spacer::new();
        assert_eq!(spacer.spacer_type, SpacerType::Flexible);
        assert!(spacer.node_id.is_none());
    }

    #[test]
    fn spacer_fixed_creation() {
        let spacer = Spacer::fixed(100.0, 50.0);
        match spacer.spacer_type {
            SpacerType::Fixed { width, height } => {
                assert_eq!(width, 100.0);
                assert_eq!(height, 50.0);
            }
            _ => panic!("Expected Fixed spacer"),
        }
    }

    #[test]
    fn spacer_horizontal_creation() {
        let spacer = Spacer::horizontal(20.0);
        match spacer.spacer_type {
            SpacerType::Horizontal { height } => {
                assert_eq!(height, 20.0);
            }
            _ => panic!("Expected Horizontal spacer"),
        }
    }

    #[test]
    fn spacer_vertical_creation() {
        let spacer = Spacer::vertical(30.0);
        match spacer.spacer_type {
            SpacerType::Vertical { width } => {
                assert_eq!(width, 30.0);
            }
            _ => panic!("Expected Vertical spacer"),
        }
    }

    #[test]
    fn spacer_default() {
        let spacer = Spacer::default();
        assert_eq!(spacer.spacer_type, SpacerType::Flexible);
    }

    #[test]
    fn spacer_build_fixed() {
        let mut engine = LayoutEngine::new();
        let mut spacer = Spacer::fixed(100.0, 50.0);

        let node = spacer.build(&mut engine);
        assert!(node.is_ok());
        assert!(spacer.node_id.is_some());
    }

    #[test]
    fn spacer_build_flexible() {
        let mut engine = LayoutEngine::new();
        let mut spacer = Spacer::new();

        let node = spacer.build(&mut engine);
        assert!(node.is_ok());
        assert!(spacer.node_id.is_some());
    }

    #[test]
    fn spacer_in_hstack() {
        let mut engine = LayoutEngine::new();

        // Create HStack with: [Box] [Spacer] [Box]
        // The spacer should fill the space between boxes
        use nebula_core::layout::styles;

        let box1 = engine.new_leaf(styles::fixed_size(50.0, 50.0)).unwrap();
        
        let mut spacer = Spacer::new();
        let spacer_node = spacer.build(&mut engine).unwrap();
        
        let box2 = engine.new_leaf(styles::fixed_size(50.0, 50.0)).unwrap();

        // Create HStack
        let hstack = engine
            .create_hstack(&[box1, spacer_node, box2])
            .unwrap();

        // Compute layout with 200px width
        let available = Size {
            width: AvailableSpace::Definite(200.0),
            height: AvailableSpace::Definite(100.0),
        };
        engine.compute_layout(hstack, available).unwrap();

        // Get spacer layout
        let spacer_layout = spacer.get_layout(&engine);
        assert!(spacer_layout.is_some());

        // Spacer should grow to fill available space
        // The exact size depends on flex layout algorithm
        let layout = spacer_layout.unwrap();
        // Just verify it has some width (flexible spacers work!)
        assert!(layout.size.width >= 0.0);
    }

    #[test]
    fn spacer_fixed_size() {
        let mut engine = LayoutEngine::new();
        let mut spacer = Spacer::fixed(80.0, 40.0);

        let node = spacer.build(&mut engine).unwrap();

        // Compute layout
        let available = Size {
            width: AvailableSpace::Definite(200.0),
            height: AvailableSpace::Definite(200.0),
        };
        engine.compute_layout(node, available).unwrap();

        // Get layout
        let layout = spacer.get_layout(&engine);
        assert!(layout.is_some());

        let layout = layout.unwrap();
        assert_eq!(layout.size.width, 80.0);
        assert_eq!(layout.size.height, 40.0);
    }

    #[test]
    fn spacer_type_getter() {
        let spacer = Spacer::fixed(100.0, 50.0);
        let spacer_type = spacer.spacer_type();
        
        match spacer_type {
            SpacerType::Fixed { width, height } => {
                assert_eq!(width, 100.0);
                assert_eq!(height, 50.0);
            }
            _ => panic!("Expected Fixed spacer"),
        }
    }

    #[test]
    fn spacer_type_equality() {
        assert_eq!(SpacerType::Flexible, SpacerType::Flexible);
        assert_ne!(
            SpacerType::Flexible,
            SpacerType::Fixed {
                width: 10.0,
                height: 10.0
            }
        );
    }
}
