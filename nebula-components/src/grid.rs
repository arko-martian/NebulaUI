use nebula_core::{LayoutEngine, NodeId, Layout};
use taffy::prelude::*;
use tracing::info;

/// Grid - Grid layout container 📊
/// 
/// Essential for dashboards, galleries, calendars, and more!
/// - Rows and columns
/// - Gap spacing
/// - Flexible sizing
/// - Responsive layouts
/// 
/// Just like CSS Grid, but simpler!
#[derive(Clone)]
pub struct Grid {
    /// Layout node ID
    pub node_id: Option<NodeId>,
    /// Children
    pub children: Vec<NodeId>,
    /// Number of columns
    pub columns: usize,
    /// Gap between items (horizontal and vertical)
    pub gap: f32,
    /// Padding around grid
    pub padding: f32,
}

impl Grid {
    /// Create a new grid with specified columns
    pub fn new(columns: usize) -> Self {
        info!("📊 Creating Grid with {} columns", columns);
        Self {
            node_id: None,
            children: Vec::new(),
            columns,
            gap: 0.0,
            padding: 0.0,
        }
    }

    /// Set gap between items
    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = gap;
        self
    }

    /// Set padding around grid
    pub fn padding(mut self, padding: f32) -> Self {
        self.padding = padding;
        self
    }

    /// Add a child to the grid
    pub fn add_child(&mut self, child: NodeId) {
        self.children.push(child);
    }

    /// Add multiple children
    pub fn add_children(&mut self, children: &[NodeId]) {
        self.children.extend_from_slice(children);
    }

    /// Get number of rows (calculated from children and columns)
    pub fn row_count(&self) -> usize {
        if self.columns == 0 {
            return 0;
        }
        (self.children.len() + self.columns - 1) / self.columns
    }

    /// Get number of children
    pub fn child_count(&self) -> usize {
        self.children.len()
    }

    /// Build the layout node
    /// Note: Taffy doesn't have native CSS Grid yet, so we'll use Flexbox
    /// to simulate a grid layout with rows
    pub fn build(&mut self, engine: &mut LayoutEngine) -> Result<NodeId, String> {
        if self.columns == 0 {
            return Err("Grid must have at least 1 column".to_string());
        }

        // Create rows using VStack
        let mut rows = Vec::new();
        
        // Group children into rows
        for row_children in self.children.chunks(self.columns) {
            // Create HStack for this row
            let row_style = Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                gap: Size {
                    width: LengthPercentage::Length(self.gap),
                    height: LengthPercentage::Length(self.gap),
                },
                ..Default::default()
            };

            let row_node = engine
                .new_with_children(row_style, row_children)
                .map_err(|e| format!("Failed to create grid row: {:?}", e))?;

            rows.push(row_node);
        }

        // Create VStack for all rows
        let grid_style = Style {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            gap: Size {
                width: LengthPercentage::Length(self.gap),
                height: LengthPercentage::Length(self.gap),
            },
            padding: Rect {
                left: LengthPercentage::Length(self.padding),
                right: LengthPercentage::Length(self.padding),
                top: LengthPercentage::Length(self.padding),
                bottom: LengthPercentage::Length(self.padding),
            },
            ..Default::default()
        };

        let node = engine
            .new_with_children(grid_style, &rows)
            .map_err(|e| format!("Failed to create grid: {:?}", e))?;

        self.node_id = Some(node);
        info!(
            "✅ Grid built ({} columns, {} rows, {} children)",
            self.columns,
            self.row_count(),
            self.child_count()
        );
        Ok(node)
    }

    /// Get the layout
    pub fn get_layout(&self, engine: &LayoutEngine) -> Option<Layout> {
        self.node_id.and_then(|id| engine.get_layout(id).ok())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nebula_core::layout::styles;

    #[test]
    fn grid_creation() {
        let grid = Grid::new(3);
        assert_eq!(grid.columns, 3);
        assert_eq!(grid.gap, 0.0);
        assert_eq!(grid.padding, 0.0);
        assert_eq!(grid.child_count(), 0);
    }

    #[test]
    fn grid_builder_pattern() {
        let grid = Grid::new(4).gap(10.0).padding(20.0);

        assert_eq!(grid.columns, 4);
        assert_eq!(grid.gap, 10.0);
        assert_eq!(grid.padding, 20.0);
    }

    #[test]
    fn grid_add_child() {
        let mut engine = LayoutEngine::new();
        let mut grid = Grid::new(2);

        let child = engine.new_leaf(styles::fixed_size(50.0, 50.0)).unwrap();
        grid.add_child(child);

        assert_eq!(grid.child_count(), 1);
    }

    #[test]
    fn grid_add_children() {
        let mut engine = LayoutEngine::new();
        let mut grid = Grid::new(3);

        let child1 = engine.new_leaf(styles::fixed_size(50.0, 50.0)).unwrap();
        let child2 = engine.new_leaf(styles::fixed_size(50.0, 50.0)).unwrap();
        let child3 = engine.new_leaf(styles::fixed_size(50.0, 50.0)).unwrap();

        grid.add_children(&[child1, child2, child3]);

        assert_eq!(grid.child_count(), 3);
    }

    #[test]
    fn grid_row_count() {
        let mut grid = Grid::new(3);

        // 0 children = 0 rows
        assert_eq!(grid.row_count(), 0);

        // Add mock children (we'll use dummy NodeIds for counting)
        // In real usage, these would be actual layout nodes
        let mut engine = LayoutEngine::new();
        
        // 1-3 children = 1 row
        for _ in 0..3 {
            let child = engine.new_leaf(styles::fixed_size(50.0, 50.0)).unwrap();
            grid.add_child(child);
        }
        assert_eq!(grid.row_count(), 1);

        // 4-6 children = 2 rows
        for _ in 0..3 {
            let child = engine.new_leaf(styles::fixed_size(50.0, 50.0)).unwrap();
            grid.add_child(child);
        }
        assert_eq!(grid.row_count(), 2);

        // 7 children = 3 rows (partial last row)
        let child = engine.new_leaf(styles::fixed_size(50.0, 50.0)).unwrap();
        grid.add_child(child);
        assert_eq!(grid.row_count(), 3);
    }

    #[test]
    fn grid_build_2x2() {
        let mut engine = LayoutEngine::new();
        let mut grid = Grid::new(2);

        // Create 4 children (2x2 grid)
        for _ in 0..4 {
            let child = engine.new_leaf(styles::fixed_size(50.0, 50.0)).unwrap();
            grid.add_child(child);
        }

        let node = grid.build(&mut engine);
        assert!(node.is_ok());
        assert!(grid.node_id.is_some());
    }

    #[test]
    fn grid_build_3x2() {
        let mut engine = LayoutEngine::new();
        let mut grid = Grid::new(3);

        // Create 6 children (3x2 grid)
        for _ in 0..6 {
            let child = engine.new_leaf(styles::fixed_size(50.0, 50.0)).unwrap();
            grid.add_child(child);
        }

        let node = grid.build(&mut engine);
        assert!(node.is_ok());
        assert_eq!(grid.row_count(), 2);
    }

    #[test]
    fn grid_build_partial_row() {
        let mut engine = LayoutEngine::new();
        let mut grid = Grid::new(3);

        // Create 5 children (3 columns = 2 rows, last row has 2 items)
        for _ in 0..5 {
            let child = engine.new_leaf(styles::fixed_size(50.0, 50.0)).unwrap();
            grid.add_child(child);
        }

        let node = grid.build(&mut engine);
        assert!(node.is_ok());
        assert_eq!(grid.row_count(), 2);
    }

    #[test]
    fn grid_with_gap() {
        let mut engine = LayoutEngine::new();
        let mut grid = Grid::new(2).gap(10.0);

        for _ in 0..4 {
            let child = engine.new_leaf(styles::fixed_size(50.0, 50.0)).unwrap();
            grid.add_child(child);
        }

        let node = grid.build(&mut engine);
        assert!(node.is_ok());
    }

    #[test]
    fn grid_with_padding() {
        let mut engine = LayoutEngine::new();
        let mut grid = Grid::new(2).padding(20.0);

        for _ in 0..4 {
            let child = engine.new_leaf(styles::fixed_size(50.0, 50.0)).unwrap();
            grid.add_child(child);
        }

        let node = grid.build(&mut engine);
        assert!(node.is_ok());
    }

    #[test]
    fn grid_layout() {
        let mut engine = LayoutEngine::new();
        let mut grid = Grid::new(2).gap(10.0);

        // Create 4 children (2x2 grid)
        for _ in 0..4 {
            let child = engine.new_leaf(styles::fixed_size(50.0, 50.0)).unwrap();
            grid.add_child(child);
        }

        let node = grid.build(&mut engine).unwrap();

        // Compute layout
        let available = Size {
            width: AvailableSpace::Definite(200.0),
            height: AvailableSpace::Definite(200.0),
        };
        engine.compute_layout(node, available).unwrap();

        let layout = grid.get_layout(&engine);
        assert!(layout.is_some());
    }

    #[test]
    fn grid_zero_columns_error() {
        let mut engine = LayoutEngine::new();
        let mut grid = Grid::new(0);

        let child = engine.new_leaf(styles::fixed_size(50.0, 50.0)).unwrap();
        grid.add_child(child);

        let result = grid.build(&mut engine);
        assert!(result.is_err());
    }

    #[test]
    fn grid_single_column() {
        let mut engine = LayoutEngine::new();
        let mut grid = Grid::new(1);

        for _ in 0..3 {
            let child = engine.new_leaf(styles::fixed_size(50.0, 50.0)).unwrap();
            grid.add_child(child);
        }

        let node = grid.build(&mut engine);
        assert!(node.is_ok());
        assert_eq!(grid.row_count(), 3);
    }

    #[test]
    fn grid_many_columns() {
        let mut engine = LayoutEngine::new();
        let mut grid = Grid::new(10);

        for _ in 0..5 {
            let child = engine.new_leaf(styles::fixed_size(50.0, 50.0)).unwrap();
            grid.add_child(child);
        }

        let node = grid.build(&mut engine);
        assert!(node.is_ok());
        assert_eq!(grid.row_count(), 1); // 5 items in 10 columns = 1 row
    }

    #[test]
    fn grid_clone() {
        let grid1 = Grid::new(3).gap(10.0);
        let grid2 = grid1.clone();

        assert_eq!(grid1.columns, grid2.columns);
        assert_eq!(grid1.gap, grid2.gap);
    }
}
