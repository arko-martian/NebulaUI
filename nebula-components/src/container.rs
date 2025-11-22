use nebula_core::{LayoutEngine, NodeId, Layout};
use tracing::info;

/// Alignment options for containers
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Alignment {
    /// Align to start (left/top)
    Start,
    /// Align to center
    Center,
    /// Align to end (right/bottom)
    End,
    /// Stretch to fill
    Stretch,
}

/// VStack - Vertical Stack Container 📚
/// Stacks children vertically (top to bottom)
/// Just like SwiftUI's VStack!
#[derive(Clone)]
pub struct VStack {
    /// Layout node ID
    pub node_id: Option<NodeId>,
    /// Children
    pub children: Vec<NodeId>,
    /// Spacing between children
    pub spacing: f32,
    /// Padding around container
    pub padding: f32,
    /// Alignment of children
    pub alignment: Alignment,
}

impl VStack {
    /// Create a new VStack
    pub fn new() -> Self {
        info!("📚 Creating VStack");
        Self {
            node_id: None,
            children: Vec::new(),
            spacing: 0.0,
            padding: 0.0,
            alignment: Alignment::Start,
        }
    }

    /// Set spacing between children
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }

    /// Set padding around container
    pub fn padding(mut self, padding: f32) -> Self {
        self.padding = padding;
        self
    }

    /// Set alignment of children
    pub fn alignment(mut self, alignment: Alignment) -> Self {
        self.alignment = alignment;
        self
    }

    /// Add a child
    pub fn add_child(&mut self, child: NodeId) {
        self.children.push(child);
    }

    /// Build the layout node
    pub fn build(&mut self, engine: &mut LayoutEngine) -> Result<NodeId, String> {
        let node = engine.create_vstack(&self.children)
            .map_err(|e| format!("Failed to create VStack: {:?}", e))?;
        
        self.node_id = Some(node);
        info!("✅ VStack built with {} children", self.children.len());
        Ok(node)
    }

    /// Get the layout
    pub fn get_layout(&self, engine: &LayoutEngine) -> Option<Layout> {
        self.node_id.and_then(|id| engine.get_layout(id).ok())
    }
}

impl Default for VStack {
    fn default() -> Self {
        Self::new()
    }
}

/// HStack - Horizontal Stack Container ↔️
/// Stacks children horizontally (left to right)
/// Just like SwiftUI's HStack!
#[derive(Clone)]
pub struct HStack {
    /// Layout node ID
    pub node_id: Option<NodeId>,
    /// Children
    pub children: Vec<NodeId>,
    /// Spacing between children
    pub spacing: f32,
    /// Padding around container
    pub padding: f32,
    /// Alignment of children
    pub alignment: Alignment,
}

impl HStack {
    /// Create a new HStack
    pub fn new() -> Self {
        info!("↔️ Creating HStack");
        Self {
            node_id: None,
            children: Vec::new(),
            spacing: 0.0,
            padding: 0.0,
            alignment: Alignment::Start,
        }
    }

    /// Set spacing between children
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }

    /// Set padding around container
    pub fn padding(mut self, padding: f32) -> Self {
        self.padding = padding;
        self
    }

    /// Set alignment of children
    pub fn alignment(mut self, alignment: Alignment) -> Self {
        self.alignment = alignment;
        self
    }

    /// Add a child
    pub fn add_child(&mut self, child: NodeId) {
        self.children.push(child);
    }

    /// Build the layout node
    pub fn build(&mut self, engine: &mut LayoutEngine) -> Result<NodeId, String> {
        let node = engine.create_hstack(&self.children)
            .map_err(|e| format!("Failed to create HStack: {:?}", e))?;
        
        self.node_id = Some(node);
        info!("✅ HStack built with {} children", self.children.len());
        Ok(node)
    }

    /// Get the layout
    pub fn get_layout(&self, engine: &LayoutEngine) -> Option<Layout> {
        self.node_id.and_then(|id| engine.get_layout(id).ok())
    }
}

impl Default for HStack {
    fn default() -> Self {
        Self::new()
    }
}

/// ZStack - Depth Stack Container 🎭
/// Stacks children on top of each other (z-axis)
/// Just like SwiftUI's ZStack!
#[derive(Clone)]
pub struct ZStack {
    /// Layout node ID
    pub node_id: Option<NodeId>,
    /// Children (rendered back to front)
    pub children: Vec<NodeId>,
    /// Alignment of children
    pub alignment: Alignment,
}

impl ZStack {
    /// Create a new ZStack
    pub fn new() -> Self {
        info!("🎭 Creating ZStack");
        Self {
            node_id: None,
            children: Vec::new(),
            alignment: Alignment::Center,
        }
    }

    /// Set alignment of children
    pub fn alignment(mut self, alignment: Alignment) -> Self {
        self.alignment = alignment;
        self
    }

    /// Add a child (will be rendered on top of previous children)
    pub fn add_child(&mut self, child: NodeId) {
        self.children.push(child);
    }

    /// Build the layout node
    /// Note: ZStack uses absolute positioning, so we create a container
    pub fn build(&mut self, engine: &mut LayoutEngine) -> Result<NodeId, String> {
        // For now, ZStack is implemented as a simple container
        // In a full implementation, we'd use absolute positioning
        let node = engine.create_vstack(&self.children)
            .map_err(|e| format!("Failed to create ZStack: {:?}", e))?;
        
        self.node_id = Some(node);
        info!("✅ ZStack built with {} children", self.children.len());
        Ok(node)
    }

    /// Get the layout
    pub fn get_layout(&self, engine: &LayoutEngine) -> Option<Layout> {
        self.node_id.and_then(|id| engine.get_layout(id).ok())
    }
}

impl Default for ZStack {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nebula_core::layout::styles;
    use taffy::prelude::*;

    #[test]
    fn vstack_creation() {
        let vstack = VStack::new();
        assert_eq!(vstack.children.len(), 0);
        assert_eq!(vstack.spacing, 0.0);
        assert_eq!(vstack.padding, 0.0);
        assert_eq!(vstack.alignment, Alignment::Start);
    }

    #[test]
    fn vstack_builder_pattern() {
        let vstack = VStack::new()
            .spacing(10.0)
            .padding(20.0)
            .alignment(Alignment::Center);

        assert_eq!(vstack.spacing, 10.0);
        assert_eq!(vstack.padding, 20.0);
        assert_eq!(vstack.alignment, Alignment::Center);
    }

    #[test]
    fn vstack_add_children() {
        let mut engine = LayoutEngine::new();
        let mut vstack = VStack::new();

        // Create some children
        let child1 = engine.new_leaf(styles::fixed_size(100.0, 50.0)).unwrap();
        let child2 = engine.new_leaf(styles::fixed_size(100.0, 50.0)).unwrap();

        vstack.add_child(child1);
        vstack.add_child(child2);

        assert_eq!(vstack.children.len(), 2);
    }

    #[test]
    fn vstack_build() {
        let mut engine = LayoutEngine::new();
        let mut vstack = VStack::new();

        // Create children
        let child1 = engine.new_leaf(styles::fixed_size(100.0, 50.0)).unwrap();
        let child2 = engine.new_leaf(styles::fixed_size(100.0, 50.0)).unwrap();

        vstack.add_child(child1);
        vstack.add_child(child2);

        // Build the VStack
        let node = vstack.build(&mut engine);
        assert!(node.is_ok());
        assert!(vstack.node_id.is_some());
    }

    #[test]
    fn vstack_layout() {
        let mut engine = LayoutEngine::new();
        let mut vstack = VStack::new();

        // Create two 100x50 children
        let child1 = engine.new_leaf(styles::fixed_size(100.0, 50.0)).unwrap();
        let child2 = engine.new_leaf(styles::fixed_size(100.0, 50.0)).unwrap();

        vstack.add_child(child1);
        vstack.add_child(child2);

        // Build and compute layout
        let node = vstack.build(&mut engine).unwrap();
        let available = Size {
            width: AvailableSpace::Definite(200.0),
            height: AvailableSpace::Definite(200.0),
        };
        engine.compute_layout(node, available).unwrap();

        // Get layout
        let layout = vstack.get_layout(&engine);
        assert!(layout.is_some());

        let layout = layout.unwrap();
        assert_eq!(layout.size.width, 100.0);
        assert_eq!(layout.size.height, 100.0); // 50 + 50
    }

    #[test]
    fn hstack_creation() {
        let hstack = HStack::new();
        assert_eq!(hstack.children.len(), 0);
        assert_eq!(hstack.spacing, 0.0);
        assert_eq!(hstack.padding, 0.0);
    }

    #[test]
    fn hstack_builder_pattern() {
        let hstack = HStack::new()
            .spacing(10.0)
            .padding(20.0)
            .alignment(Alignment::End);

        assert_eq!(hstack.spacing, 10.0);
        assert_eq!(hstack.padding, 20.0);
        assert_eq!(hstack.alignment, Alignment::End);
    }

    #[test]
    fn hstack_layout() {
        let mut engine = LayoutEngine::new();
        let mut hstack = HStack::new();

        // Create two 50x100 children
        let child1 = engine.new_leaf(styles::fixed_size(50.0, 100.0)).unwrap();
        let child2 = engine.new_leaf(styles::fixed_size(50.0, 100.0)).unwrap();

        hstack.add_child(child1);
        hstack.add_child(child2);

        // Build and compute layout
        let node = hstack.build(&mut engine).unwrap();
        let available = Size {
            width: AvailableSpace::Definite(200.0),
            height: AvailableSpace::Definite(200.0),
        };
        engine.compute_layout(node, available).unwrap();

        // Get layout
        let layout = hstack.get_layout(&engine);
        assert!(layout.is_some());

        let layout = layout.unwrap();
        assert_eq!(layout.size.width, 100.0); // 50 + 50
        assert_eq!(layout.size.height, 100.0);
    }

    #[test]
    fn zstack_creation() {
        let zstack = ZStack::new();
        assert_eq!(zstack.children.len(), 0);
        assert_eq!(zstack.alignment, Alignment::Center);
    }

    #[test]
    fn zstack_build() {
        let mut engine = LayoutEngine::new();
        let mut zstack = ZStack::new();

        // Create children
        let child1 = engine.new_leaf(styles::fixed_size(100.0, 100.0)).unwrap();
        let child2 = engine.new_leaf(styles::fixed_size(50.0, 50.0)).unwrap();

        zstack.add_child(child1);
        zstack.add_child(child2);

        // Build the ZStack
        let node = zstack.build(&mut engine);
        assert!(node.is_ok());
        assert!(zstack.node_id.is_some());
    }

    #[test]
    fn nested_containers() {
        let mut engine = LayoutEngine::new();

        // Create inner HStack with two children
        let mut inner_hstack = HStack::new();
        let child1 = engine.new_leaf(styles::fixed_size(50.0, 50.0)).unwrap();
        let child2 = engine.new_leaf(styles::fixed_size(50.0, 50.0)).unwrap();
        inner_hstack.add_child(child1);
        inner_hstack.add_child(child2);
        let inner_node = inner_hstack.build(&mut engine).unwrap();

        // Create outer VStack with inner HStack
        let mut outer_vstack = VStack::new();
        outer_vstack.add_child(inner_node);
        let child3 = engine.new_leaf(styles::fixed_size(100.0, 50.0)).unwrap();
        outer_vstack.add_child(child3);

        // Build outer VStack
        let outer_node = outer_vstack.build(&mut engine).unwrap();

        // Compute layout
        let available = Size {
            width: AvailableSpace::Definite(200.0),
            height: AvailableSpace::Definite(200.0),
        };
        engine.compute_layout(outer_node, available).unwrap();

        // Verify layout
        let layout = outer_vstack.get_layout(&engine);
        assert!(layout.is_some());
    }

    #[test]
    fn alignment_variants() {
        assert_eq!(Alignment::Start, Alignment::Start);
        assert_ne!(Alignment::Start, Alignment::Center);
        assert_ne!(Alignment::Center, Alignment::End);
        assert_ne!(Alignment::End, Alignment::Stretch);
    }
}
