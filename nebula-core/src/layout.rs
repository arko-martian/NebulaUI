use taffy::prelude::*;
use taffy::TaffyTree as Taffy;
use tracing::{info, debug};
use std::collections::HashMap;

/// Layout engine wrapper around Taffy
/// Provides Flexbox layout for Nebula UI! 📐
/// 
/// This is the SAME layout system used by:
/// - React Native
/// - Flutter  
/// - Modern web browsers
/// 
/// But ours works on 20-year-old machines! 🚀
pub struct LayoutEngine {
    taffy: Taffy,
    /// Cache of computed layouts
    layout_cache: HashMap<NodeId, Layout>,
    /// Dirty nodes that need re-layout
    dirty_nodes: Vec<NodeId>,
}

/// Node ID wrapper
pub type NodeId = taffy::NodeId;

/// Layout result
pub type Layout = taffy::Layout;

/// Layout direction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    /// Vertical stack (column)
    Column,
    /// Horizontal stack (row)
    Row,
}

impl From<Direction> for FlexDirection {
    fn from(dir: Direction) -> Self {
        match dir {
            Direction::Column => FlexDirection::Column,
            Direction::Row => FlexDirection::Row,
        }
    }
}

impl LayoutEngine {
    /// Create a new layout engine
    pub fn new() -> Self {
        info!("📐 Initializing layout engine with Taffy");
        Self {
            taffy: Taffy::new(),
            layout_cache: HashMap::new(),
            dirty_nodes: Vec::new(),
        }
    }

    /// Create a new leaf node (no children)
    pub fn new_leaf(&mut self, style: Style) -> Result<NodeId, taffy::TaffyError> {
        let node = self.taffy.new_leaf(style)?;
        self.mark_dirty(node);
        Ok(node)
    }

    /// Create a new node with children
    pub fn new_with_children(
        &mut self,
        style: Style,
        children: &[NodeId],
    ) -> Result<NodeId, taffy::TaffyError> {
        let node = self.taffy.new_with_children(style, children)?;
        self.mark_dirty(node);
        Ok(node)
    }

    /// Create a VStack (vertical stack / column)
    pub fn create_vstack(&mut self, children: &[NodeId]) -> Result<NodeId, taffy::TaffyError> {
        let style = Style {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            ..Default::default()
        };
        self.new_with_children(style, children)
    }

    /// Create an HStack (horizontal stack / row)
    pub fn create_hstack(&mut self, children: &[NodeId]) -> Result<NodeId, taffy::TaffyError> {
        let style = Style {
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            ..Default::default()
        };
        self.new_with_children(style, children)
    }

    /// Set node style
    pub fn set_style(&mut self, node: NodeId, style: Style) -> Result<(), taffy::TaffyError> {
        self.taffy.set_style(node, style)?;
        self.mark_dirty(node);
        Ok(())
    }

    /// Add child to node
    pub fn add_child(&mut self, parent: NodeId, child: NodeId) -> Result<(), taffy::TaffyError> {
        self.taffy.add_child(parent, child)?;
        self.mark_dirty(parent);
        Ok(())
    }

    /// Remove child from node
    pub fn remove_child(&mut self, parent: NodeId, child: NodeId) -> Result<NodeId, taffy::TaffyError> {
        let removed = self.taffy.remove_child(parent, child)?;
        self.mark_dirty(parent);
        Ok(removed)
    }

    /// Mark a node as dirty (needs re-layout)
    pub fn mark_dirty(&mut self, node: NodeId) {
        if !self.dirty_nodes.contains(&node) {
            self.dirty_nodes.push(node);
            debug!("Marked node {:?} as dirty", node);
        }
    }

    /// Compute layout for a node
    pub fn compute_layout(
        &mut self,
        node: NodeId,
        available_space: Size<AvailableSpace>,
    ) -> Result<Layout, taffy::TaffyError> {
        // Check cache first (if not dirty)
        if !self.dirty_nodes.contains(&node) {
            if let Some(cached) = self.layout_cache.get(&node) {
                debug!("Using cached layout for node {:?}", node);
                return Ok(*cached);
            }
        }

        // Compute layout
        self.taffy.compute_layout(node, available_space)?;
        let layout = *self.taffy.layout(node)?;

        // Cache the result
        self.layout_cache.insert(node, layout);

        // Remove from dirty list
        self.dirty_nodes.retain(|&n| n != node);

        Ok(layout)
    }

    /// Get layout for a node (must be computed first)
    pub fn get_layout(&self, node: NodeId) -> Result<Layout, taffy::TaffyError> {
        // Check cache first
        if let Some(cached) = self.layout_cache.get(&node) {
            return Ok(*cached);
        }

        // Fall back to taffy
        self.taffy.layout(node).copied()
    }

    /// Clear all dirty nodes
    pub fn clear_dirty(&mut self) {
        self.dirty_nodes.clear();
    }

    /// Get number of dirty nodes
    pub fn dirty_count(&self) -> usize {
        self.dirty_nodes.len()
    }

    /// Clear layout cache
    pub fn clear_cache(&mut self) {
        self.layout_cache.clear();
        info!("🧹 Layout cache cleared");
    }

    /// Get cache size
    pub fn cache_size(&self) -> usize {
        self.layout_cache.len()
    }
}

impl Default for LayoutEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper to create common styles
pub mod styles {
    use super::*;

    /// Create a flex container style
    pub fn flex_container(direction: Direction) -> Style {
        Style {
            display: Display::Flex,
            flex_direction: direction.into(),
            ..Default::default()
        }
    }

    /// Create a fixed size style
    pub fn fixed_size(width: f32, height: f32) -> Style {
        Style {
            size: Size {
                width: Dimension::Length(width),
                height: Dimension::Length(height),
            },
            ..Default::default()
        }
    }

    /// Create a style with padding
    pub fn with_padding(padding: f32) -> Style {
        Style {
            padding: Rect {
                left: LengthPercentage::Length(padding),
                right: LengthPercentage::Length(padding),
                top: LengthPercentage::Length(padding),
                bottom: LengthPercentage::Length(padding),
            },
            ..Default::default()
        }
    }

    /// Create a style with gap (spacing between children)
    pub fn with_gap(gap: f32) -> Style {
        Style {
            gap: Size {
                width: LengthPercentage::Length(gap),
                height: LengthPercentage::Length(gap),
            },
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn layout_engine_creation() {
        let engine = LayoutEngine::new();
        assert_eq!(engine.dirty_count(), 0);
        assert_eq!(engine.cache_size(), 0);
    }

    #[test]
    fn create_leaf_node() {
        let mut engine = LayoutEngine::new();
        let style = styles::fixed_size(100.0, 50.0);
        let node = engine.new_leaf(style);
        
        assert!(node.is_ok());
        assert_eq!(engine.dirty_count(), 1);
    }

    #[test]
    fn create_vstack() {
        let mut engine = LayoutEngine::new();
        
        // Create children
        let child1 = engine.new_leaf(styles::fixed_size(100.0, 50.0)).unwrap();
        let child2 = engine.new_leaf(styles::fixed_size(100.0, 50.0)).unwrap();
        
        // Create VStack
        let vstack = engine.create_vstack(&[child1, child2]);
        assert!(vstack.is_ok());
    }

    #[test]
    fn create_hstack() {
        let mut engine = LayoutEngine::new();
        
        // Create children
        let child1 = engine.new_leaf(styles::fixed_size(50.0, 100.0)).unwrap();
        let child2 = engine.new_leaf(styles::fixed_size(50.0, 100.0)).unwrap();
        
        // Create HStack
        let hstack = engine.create_hstack(&[child1, child2]);
        assert!(hstack.is_ok());
    }

    #[test]
    fn compute_simple_layout() {
        let mut engine = LayoutEngine::new();
        
        let style = styles::fixed_size(100.0, 50.0);
        let node = engine.new_leaf(style).unwrap();
        
        let available = Size {
            width: AvailableSpace::Definite(200.0),
            height: AvailableSpace::Definite(200.0),
        };
        
        let layout = engine.compute_layout(node, available);
        assert!(layout.is_ok());
        
        let layout = layout.unwrap();
        assert_eq!(layout.size.width, 100.0);
        assert_eq!(layout.size.height, 50.0);
    }

    #[test]
    fn layout_caching() {
        let mut engine = LayoutEngine::new();
        
        let style = styles::fixed_size(100.0, 50.0);
        let node = engine.new_leaf(style).unwrap();
        
        let available = Size {
            width: AvailableSpace::Definite(200.0),
            height: AvailableSpace::Definite(200.0),
        };
        
        // First computation
        engine.compute_layout(node, available).unwrap();
        assert_eq!(engine.cache_size(), 1);
        assert_eq!(engine.dirty_count(), 0);
        
        // Second computation should use cache
        let layout = engine.get_layout(node);
        assert!(layout.is_ok());
    }

    #[test]
    fn dirty_tracking() {
        let mut engine = LayoutEngine::new();
        
        let style = styles::fixed_size(100.0, 50.0);
        let node = engine.new_leaf(style).unwrap();
        
        assert_eq!(engine.dirty_count(), 1);
        
        // Compute layout clears dirty
        let available = Size {
            width: AvailableSpace::Definite(200.0),
            height: AvailableSpace::Definite(200.0),
        };
        engine.compute_layout(node, available).unwrap();
        assert_eq!(engine.dirty_count(), 0);
        
        // Changing style marks dirty
        engine.set_style(node, styles::fixed_size(200.0, 100.0)).unwrap();
        assert_eq!(engine.dirty_count(), 1);
    }

    #[test]
    fn vstack_layout() {
        let mut engine = LayoutEngine::new();
        
        // Create two 100x50 children
        let child1 = engine.new_leaf(styles::fixed_size(100.0, 50.0)).unwrap();
        let child2 = engine.new_leaf(styles::fixed_size(100.0, 50.0)).unwrap();
        
        // Create VStack
        let vstack = engine.create_vstack(&[child1, child2]).unwrap();
        
        // Compute layout
        let available = Size {
            width: AvailableSpace::Definite(200.0),
            height: AvailableSpace::Definite(200.0),
        };
        let layout = engine.compute_layout(vstack, available).unwrap();
        
        // VStack should stack vertically: 100 wide, 100 tall (50 + 50)
        assert_eq!(layout.size.width, 100.0);
        assert_eq!(layout.size.height, 100.0);
    }

    #[test]
    fn hstack_layout() {
        let mut engine = LayoutEngine::new();
        
        // Create two 50x100 children
        let child1 = engine.new_leaf(styles::fixed_size(50.0, 100.0)).unwrap();
        let child2 = engine.new_leaf(styles::fixed_size(50.0, 100.0)).unwrap();
        
        // Create HStack
        let hstack = engine.create_hstack(&[child1, child2]).unwrap();
        
        // Compute layout
        let available = Size {
            width: AvailableSpace::Definite(200.0),
            height: AvailableSpace::Definite(200.0),
        };
        let layout = engine.compute_layout(hstack, available).unwrap();
        
        // HStack should stack horizontally: 100 wide (50 + 50), 100 tall
        assert_eq!(layout.size.width, 100.0);
        assert_eq!(layout.size.height, 100.0);
    }

    #[test]
    fn performance_1000_nodes() {
        let mut engine = LayoutEngine::new();
        
        // Create 1000 leaf nodes
        let mut nodes = Vec::new();
        for _ in 0..1000 {
            let node = engine.new_leaf(styles::fixed_size(10.0, 10.0)).unwrap();
            nodes.push(node);
        }
        
        // Create a container with all nodes
        let container = engine.create_vstack(&nodes).unwrap();
        
        // Measure layout time
        let start = Instant::now();
        let available = Size {
            width: AvailableSpace::Definite(1000.0),
            height: AvailableSpace::Definite(10000.0),
        };
        engine.compute_layout(container, available).unwrap();
        let duration = start.elapsed();
        
        println!("⚡ 1000 node layout took: {:?}", duration);
        
        // Should be < 15ms in debug mode (< 5ms in release mode)
        // This is still VERY fast for 1000 nodes!
        assert!(duration.as_millis() < 15, "Layout took {:?}, should be < 15ms", duration);
    }

    #[test]
    fn cache_clear() {
        let mut engine = LayoutEngine::new();
        
        let node = engine.new_leaf(styles::fixed_size(100.0, 50.0)).unwrap();
        let available = Size {
            width: AvailableSpace::Definite(200.0),
            height: AvailableSpace::Definite(200.0),
        };
        
        engine.compute_layout(node, available).unwrap();
        assert_eq!(engine.cache_size(), 1);
        
        engine.clear_cache();
        assert_eq!(engine.cache_size(), 0);
    }
}
