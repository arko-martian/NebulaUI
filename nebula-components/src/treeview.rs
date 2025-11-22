// TreeView Component - Hierarchical tree structure view
// Essential for displaying nested data like file systems

use nebula_core::layout::{LayoutEngine, NodeId};
use nebula_core::signal::Signal;

/// Tree node
#[derive(Debug, Clone, PartialEq)]
pub struct TreeNode {
    pub id: String,
    pub label: String,
    pub children: Vec<TreeNode>,
    pub expanded: bool,
    pub disabled: bool,
    pub icon: Option<String>,
    pub badge: Option<String>,
    pub metadata: Option<String>,
}

impl TreeNode {
    /// Create a new tree node
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            children: Vec::new(),
            expanded: false,
            disabled: false,
            icon: None,
            badge: None,
            metadata: None,
        }
    }

    /// Create a disabled tree node
    pub fn disabled(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            children: Vec::new(),
            expanded: false,
            disabled: true,
            icon: None,
            badge: None,
            metadata: None,
        }
    }

    /// Add a child node
    pub fn with_child(mut self, child: TreeNode) -> Self {
        self.children.push(child);
        self
    }

    /// Add multiple children
    pub fn with_children(mut self, children: Vec<TreeNode>) -> Self {
        self.children.extend(children);
        self
    }

    /// Set expanded state
    pub fn expanded(mut self, expanded: bool) -> Self {
        self.expanded = expanded;
        self
    }

    /// Add an icon
    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Add a badge
    pub fn with_badge(mut self, badge: impl Into<String>) -> Self {
        self.badge = Some(badge.into());
        self
    }

    /// Add metadata
    pub fn with_metadata(mut self, metadata: impl Into<String>) -> Self {
        self.metadata = Some(metadata.into());
        self
    }

    /// Check if node has children
    pub fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    /// Get child count
    pub fn child_count(&self) -> usize {
        self.children.len()
    }

    /// Find child by ID (recursive)
    pub fn find_child(&self, id: &str) -> Option<&TreeNode> {
        if self.id == id {
            return Some(self);
        }
        for child in &self.children {
            if let Some(found) = child.find_child(id) {
                return Some(found);
            }
        }
        None
    }

    /// Find child by ID (mutable, recursive)
    pub fn find_child_mut(&mut self, id: &str) -> Option<&mut TreeNode> {
        if self.id == id {
            return Some(self);
        }
        for child in &mut self.children {
            if let Some(found) = child.find_child_mut(id) {
                return Some(found);
            }
        }
        None
    }
}

/// TreeView component - hierarchical tree structure for nested data
/// 
/// # Example
/// ```
/// let mut tree = TreeView::new()
///     .add_node(
///         TreeNode::new("root", "Root")
///             .with_child(TreeNode::new("child1", "Child 1"))
///             .with_child(TreeNode::new("child2", "Child 2"))
///             .expanded(true)
///     )
///     .on_select(|node_id| println!("Selected: {}", node_id))
///     .on_expand(|node_id| println!("Expanded: {}", node_id));
/// ```
pub struct TreeView {
    pub node_id: Option<NodeId>,
    pub nodes: Vec<TreeNode>,
    pub selected_node: Signal<Option<String>>,
    pub indent_size: f32,
    pub node_height: f32,
    pub padding: f32,
    pub background_color: (u8, u8, u8, u8),
    pub node_color: (u8, u8, u8, u8),
    pub selected_color: (u8, u8, u8, u8),
    pub hover_color: (u8, u8, u8, u8),
    pub text_color: (u8, u8, u8, u8),
    pub selected_text_color: (u8, u8, u8, u8),
    pub expand_icon: String,
    pub collapse_icon: String,
    pub leaf_icon: String,
    pub show_lines: bool,
    pub on_select: Option<Box<dyn Fn(&str)>>,
    pub on_expand: Option<Box<dyn Fn(&str)>>,
    pub on_collapse: Option<Box<dyn Fn(&str)>>,
}

impl TreeView {
    /// Create a new TreeView component
    pub fn new() -> Self {
        Self {
            node_id: None,
            nodes: Vec::new(),
            selected_node: Signal::new(None),
            indent_size: 24.0,
            node_height: 32.0,
            padding: 8.0,
            background_color: (255, 255, 255, 255),
            node_color: (255, 255, 255, 255),
            selected_color: (59, 130, 246, 20), // Light blue
            hover_color: (245, 245, 245, 255),
            text_color: (0, 0, 0, 255),
            selected_text_color: (59, 130, 246, 255), // Blue
            expand_icon: "▶".to_string(),
            collapse_icon: "▼".to_string(),
            leaf_icon: "•".to_string(),
            show_lines: true,
            on_select: None,
            on_expand: None,
            on_collapse: None,
        }
    }

    /// Set indent size
    pub fn indent_size(mut self, size: f32) -> Self {
        self.indent_size = size;
        self
    }

    /// Set node height
    pub fn node_height(mut self, height: f32) -> Self {
        self.node_height = height;
        self
    }

    /// Set padding
    pub fn padding(mut self, padding: f32) -> Self {
        self.padding = padding;
        self
    }

    /// Set background color
    pub fn background_color(mut self, r: u8, g: u8, b: u8, a: u8) -> Self {
        self.background_color = (r, g, b, a);
        self
    }

    /// Set selected node color
    pub fn selected_color(mut self, r: u8, g: u8, b: u8, a: u8) -> Self {
        self.selected_color = (r, g, b, a);
        self
    }

    /// Set expand icon
    pub fn expand_icon(mut self, icon: impl Into<String>) -> Self {
        self.expand_icon = icon.into();
        self
    }

    /// Set collapse icon
    pub fn collapse_icon(mut self, icon: impl Into<String>) -> Self {
        self.collapse_icon = icon.into();
        self
    }

    /// Set leaf icon
    pub fn leaf_icon(mut self, icon: impl Into<String>) -> Self {
        self.leaf_icon = icon.into();
        self
    }

    /// Show or hide connecting lines
    pub fn show_lines(mut self, show: bool) -> Self {
        self.show_lines = show;
        self
    }

    /// Add a root node
    pub fn add_node(mut self, node: TreeNode) -> Self {
        self.nodes.push(node);
        self
    }

    /// Set all nodes at once
    pub fn nodes(mut self, nodes: Vec<TreeNode>) -> Self {
        self.nodes = nodes;
        self
    }

    /// Set the select callback
    pub fn on_select<F>(mut self, callback: F) -> Self
    where
        F: Fn(&str) + 'static,
    {
        self.on_select = Some(Box::new(callback));
        self
    }

    /// Set the expand callback
    pub fn on_expand<F>(mut self, callback: F) -> Self
    where
        F: Fn(&str) + 'static,
    {
        self.on_expand = Some(Box::new(callback));
        self
    }

    /// Set the collapse callback
    pub fn on_collapse<F>(mut self, callback: F) -> Self
    where
        F: Fn(&str) + 'static,
    {
        self.on_collapse = Some(Box::new(callback));
        self
    }

    /// Select a node by ID
    pub fn select_node(&mut self, id: &str) {
        if let Some(node) = self.find_node(id) {
            if node.disabled {
                return;
            }

            self.selected_node.set(Some(id.to_string()));
            if let Some(ref callback) = self.on_select {
                callback(id);
            }
        }
    }

    /// Deselect the current node
    pub fn deselect(&mut self) {
        self.selected_node.set(None);
    }

    /// Get selected node ID
    pub fn get_selected(&self) -> Option<String> {
        self.selected_node.get()
    }

    /// Check if a node is selected
    pub fn is_selected(&self, id: &str) -> bool {
        self.selected_node.get().as_deref() == Some(id)
    }

    /// Expand a node by ID
    pub fn expand_node(&mut self, id: &str) {
        if let Some(node) = self.find_node_mut(id) {
            if node.has_children() && !node.expanded {
                node.expanded = true;
                if let Some(ref callback) = self.on_expand {
                    callback(id);
                }
            }
        }
    }

    /// Collapse a node by ID
    pub fn collapse_node(&mut self, id: &str) {
        if let Some(node) = self.find_node_mut(id) {
            if node.has_children() && node.expanded {
                node.expanded = false;
                if let Some(ref callback) = self.on_collapse {
                    callback(id);
                }
            }
        }
    }

    /// Toggle node expansion
    pub fn toggle_node(&mut self, id: &str) {
        if let Some(node) = self.find_node(id) {
            if node.expanded {
                self.collapse_node(id);
            } else {
                self.expand_node(id);
            }
        }
    }

    /// Expand all nodes
    pub fn expand_all(&mut self) {
        for node in &mut self.nodes {
            Self::expand_recursive(node);
        }
    }

    /// Collapse all nodes
    pub fn collapse_all(&mut self) {
        for node in &mut self.nodes {
            Self::collapse_recursive(node);
        }
    }

    /// Expand node and all descendants recursively
    fn expand_recursive(node: &mut TreeNode) {
        node.expanded = true;
        for child in &mut node.children {
            Self::expand_recursive(child);
        }
    }

    /// Collapse node and all descendants recursively
    fn collapse_recursive(node: &mut TreeNode) {
        node.expanded = false;
        for child in &mut node.children {
            Self::collapse_recursive(child);
        }
    }

    /// Find a node by ID (immutable)
    pub fn find_node(&self, id: &str) -> Option<&TreeNode> {
        for node in &self.nodes {
            if let Some(found) = node.find_child(id) {
                return Some(found);
            }
        }
        None
    }

    /// Find a node by ID (mutable)
    pub fn find_node_mut(&mut self, id: &str) -> Option<&mut TreeNode> {
        for node in &mut self.nodes {
            if let Some(found) = node.find_child_mut(id) {
                return Some(found);
            }
        }
        None
    }

    /// Get root node count
    pub fn root_count(&self) -> usize {
        self.nodes.len()
    }

    /// Check if has nodes
    pub fn has_nodes(&self) -> bool {
        !self.nodes.is_empty()
    }

    /// Get total node count (including all descendants)
    pub fn total_node_count(&self) -> usize {
        self.nodes.iter().map(|n| Self::count_nodes(n)).sum()
    }

    /// Count nodes recursively
    fn count_nodes(node: &TreeNode) -> usize {
        1 + node.children.iter().map(|n| Self::count_nodes(n)).sum::<usize>()
    }

    /// Build the tree layout
    pub fn build(&mut self, engine: &mut LayoutEngine) -> Result<NodeId, String> {
        let style = taffy::style::Style {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Percent(1.0),
                height: taffy::style::Dimension::Auto,
            },
            display: taffy::style::Display::Flex,
            flex_direction: taffy::style::FlexDirection::Column,
            ..Default::default()
        };

        let node = engine
            .new_leaf(style)
            .map_err(|e| format!("Failed to create tree node: {:?}", e))?;
        self.node_id = Some(node);

        Ok(node)
    }
}

impl Default for TreeView {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn treeview_starts_empty() {
        let tree = TreeView::new();
        assert_eq!(tree.root_count(), 0);
        assert!(!tree.has_nodes());
        assert!(tree.get_selected().is_none());
    }

    #[test]
    fn treeview_add_nodes() {
        let tree = TreeView::new()
            .add_node(TreeNode::new("node1", "Node 1"))
            .add_node(TreeNode::new("node2", "Node 2"));

        assert_eq!(tree.root_count(), 2);
        assert!(tree.has_nodes());
    }

    #[test]
    fn treenode_with_children() {
        let node = TreeNode::new("parent", "Parent")
            .with_child(TreeNode::new("child1", "Child 1"))
            .with_child(TreeNode::new("child2", "Child 2"));

        assert!(node.has_children());
        assert_eq!(node.child_count(), 2);
    }

    #[test]
    fn treeview_select_node() {
        let mut tree = TreeView::new()
            .add_node(TreeNode::new("node1", "Node 1"));

        tree.select_node("node1");
        assert!(tree.is_selected("node1"));
        assert_eq!(tree.get_selected(), Some("node1".to_string()));
    }

    #[test]
    fn treeview_deselect_node() {
        let mut tree = TreeView::new()
            .add_node(TreeNode::new("node1", "Node 1"));

        tree.select_node("node1");
        assert!(tree.is_selected("node1"));

        tree.deselect();
        assert!(!tree.is_selected("node1"));
        assert!(tree.get_selected().is_none());
    }

    #[test]
    fn treeview_cannot_select_disabled() {
        let mut tree = TreeView::new()
            .add_node(TreeNode::disabled("node1", "Disabled"));

        tree.select_node("node1");
        assert!(!tree.is_selected("node1"));
    }

    #[test]
    fn treeview_expand_node() {
        let mut tree = TreeView::new()
            .add_node(
                TreeNode::new("parent", "Parent")
                    .with_child(TreeNode::new("child", "Child"))
            );

        tree.expand_node("parent");
        let node = tree.find_node("parent").unwrap();
        assert!(node.expanded);
    }

    #[test]
    fn treeview_collapse_node() {
        let mut tree = TreeView::new()
            .add_node(
                TreeNode::new("parent", "Parent")
                    .with_child(TreeNode::new("child", "Child"))
                    .expanded(true)
            );

        tree.collapse_node("parent");
        let node = tree.find_node("parent").unwrap();
        assert!(!node.expanded);
    }

    #[test]
    fn treeview_toggle_node() {
        let mut tree = TreeView::new()
            .add_node(
                TreeNode::new("parent", "Parent")
                    .with_child(TreeNode::new("child", "Child"))
            );

        tree.toggle_node("parent");
        assert!(tree.find_node("parent").unwrap().expanded);

        tree.toggle_node("parent");
        assert!(!tree.find_node("parent").unwrap().expanded);
    }

    #[test]
    fn treeview_expand_all() {
        let mut tree = TreeView::new()
            .add_node(
                TreeNode::new("root", "Root")
                    .with_child(
                        TreeNode::new("parent", "Parent")
                            .with_child(TreeNode::new("child", "Child"))
                    )
            );

        tree.expand_all();
        assert!(tree.find_node("root").unwrap().expanded);
        assert!(tree.find_node("parent").unwrap().expanded);
    }

    #[test]
    fn treeview_collapse_all() {
        let mut tree = TreeView::new()
            .add_node(
                TreeNode::new("root", "Root")
                    .expanded(true)
                    .with_child(
                        TreeNode::new("parent", "Parent")
                            .expanded(true)
                            .with_child(TreeNode::new("child", "Child"))
                    )
            );

        tree.collapse_all();
        assert!(!tree.find_node("root").unwrap().expanded);
        assert!(!tree.find_node("parent").unwrap().expanded);
    }

    #[test]
    fn treeview_find_node() {
        let tree = TreeView::new()
            .add_node(
                TreeNode::new("root", "Root")
                    .with_child(TreeNode::new("child", "Child"))
            );

        assert!(tree.find_node("root").is_some());
        assert!(tree.find_node("child").is_some());
        assert!(tree.find_node("nonexistent").is_none());
    }

    #[test]
    fn treenode_find_child() {
        let node = TreeNode::new("root", "Root")
            .with_child(
                TreeNode::new("parent", "Parent")
                    .with_child(TreeNode::new("child", "Child"))
            );

        assert!(node.find_child("root").is_some());
        assert!(node.find_child("parent").is_some());
        assert!(node.find_child("child").is_some());
        assert!(node.find_child("nonexistent").is_none());
    }

    #[test]
    fn treeview_total_node_count() {
        let tree = TreeView::new()
            .add_node(
                TreeNode::new("root1", "Root 1")
                    .with_child(TreeNode::new("child1", "Child 1"))
                    .with_child(TreeNode::new("child2", "Child 2"))
            )
            .add_node(TreeNode::new("root2", "Root 2"));

        assert_eq!(tree.total_node_count(), 4); // 2 roots + 2 children
    }

    #[test]
    fn treenode_with_icon_and_badge() {
        let node = TreeNode::new("node1", "Node")
            .with_icon("folder")
            .with_badge("5")
            .with_metadata("Important");

        assert_eq!(node.icon, Some("folder".to_string()));
        assert_eq!(node.badge, Some("5".to_string()));
        assert_eq!(node.metadata, Some("Important".to_string()));
    }

    #[test]
    fn treeview_callbacks() {
        use std::sync::{Arc, Mutex};

        let selected = Arc::new(Mutex::new(String::new()));
        let selected_clone = selected.clone();

        let expanded = Arc::new(Mutex::new(String::new()));
        let expanded_clone = expanded.clone();

        let collapsed = Arc::new(Mutex::new(String::new()));
        let collapsed_clone = collapsed.clone();

        let mut tree = TreeView::new()
            .add_node(
                TreeNode::new("parent", "Parent")
                    .with_child(TreeNode::new("child", "Child"))
            )
            .on_select(move |id| {
                *selected_clone.lock().unwrap() = id.to_string();
            })
            .on_expand(move |id| {
                *expanded_clone.lock().unwrap() = id.to_string();
            })
            .on_collapse(move |id| {
                *collapsed_clone.lock().unwrap() = id.to_string();
            });

        tree.select_node("parent");
        assert_eq!(*selected.lock().unwrap(), "parent");

        tree.expand_node("parent");
        assert_eq!(*expanded.lock().unwrap(), "parent");

        tree.collapse_node("parent");
        assert_eq!(*collapsed.lock().unwrap(), "parent");
    }

    #[test]
    fn treeview_builder_pattern() {
        let tree = TreeView::new()
            .indent_size(32.0)
            .node_height(40.0)
            .padding(12.0)
            .background_color(50, 50, 50, 255)
            .selected_color(255, 0, 0, 50)
            .expand_icon("+")
            .collapse_icon("-")
            .leaf_icon("*")
            .show_lines(false);

        assert_eq!(tree.indent_size, 32.0);
        assert_eq!(tree.node_height, 40.0);
        assert_eq!(tree.padding, 12.0);
        assert_eq!(tree.background_color, (50, 50, 50, 255));
        assert_eq!(tree.selected_color, (255, 0, 0, 50));
        assert_eq!(tree.expand_icon, "+");
        assert_eq!(tree.collapse_icon, "-");
        assert_eq!(tree.leaf_icon, "*");
        assert!(!tree.show_lines);
    }

    #[test]
    fn treeview_build_creates_node() {
        let mut engine = LayoutEngine::new();
        let mut tree = TreeView::new()
            .add_node(TreeNode::new("node1", "Node 1"));

        let result = tree.build(&mut engine);
        assert!(result.is_ok());
        assert!(tree.node_id.is_some());
    }
}
