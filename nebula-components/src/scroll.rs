use nebula_core::{LayoutEngine, NodeId, Layout};
use taffy::prelude::*;
use tracing::{info, warn};

/// ScrollView - Scrollable Container 📜
/// 
/// Essential for content that doesn't fit on screen!
/// - Vertical and horizontal scrolling
/// - Smooth scrolling with momentum
/// - Scroll position tracking
/// - Scroll indicators
/// - Nested scrolling support
/// - Works on old hardware!
/// 
/// Just like SwiftUI's ScrollView!
#[derive(Clone)]
pub struct ScrollView {
    /// Layout node ID
    pub node_id: Option<NodeId>,
    /// Child content node
    pub content: Option<NodeId>,
    /// Scroll direction
    pub direction: ScrollDirection,
    /// Current scroll offset (x, y)
    pub scroll_offset: (f32, f32),
    /// Content size (width, height)
    pub content_size: (f32, f32),
    /// Viewport size (width, height)
    pub viewport_size: (f32, f32),
    /// Show scroll indicators
    pub show_indicators: bool,
    /// Enable bouncing at edges
    pub bounces: bool,
    /// Scroll velocity for momentum (x, y)
    pub velocity: (f32, f32),
    /// Width (None = fill parent)
    pub width: Option<f32>,
    /// Height (None = fill parent)
    pub height: Option<f32>,
}

/// Scroll direction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrollDirection {
    /// Vertical scrolling only
    Vertical,
    /// Horizontal scrolling only
    Horizontal,
    /// Both directions
    Both,
}

impl ScrollView {
    /// Create a new ScrollView (vertical by default)
    pub fn new() -> Self {
        info!("📜 Creating ScrollView");
        Self {
            node_id: None,
            content: None,
            direction: ScrollDirection::Vertical,
            scroll_offset: (0.0, 0.0),
            content_size: (0.0, 0.0),
            viewport_size: (0.0, 0.0),
            show_indicators: true,
            bounces: true,
            velocity: (0.0, 0.0),
            width: None,
            height: None,
        }
    }

    /// Set scroll direction
    pub fn direction(mut self, direction: ScrollDirection) -> Self {
        self.direction = direction;
        self
    }

    /// Set width
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    /// Set height
    pub fn height(mut self, height: f32) -> Self {
        self.height = Some(height);
        self
    }

    /// Set size (width and height)
    pub fn size(mut self, width: f32, height: f32) -> Self {
        self.width = Some(width);
        self.height = Some(height);
        self
    }

    /// Show or hide scroll indicators
    pub fn show_indicators(mut self, show: bool) -> Self {
        self.show_indicators = show;
        self
    }

    /// Enable or disable bouncing at edges
    pub fn bounces(mut self, bounces: bool) -> Self {
        self.bounces = bounces;
        self
    }

    /// Set content node
    pub fn content(mut self, content: NodeId) -> Self {
        self.content = Some(content);
        self
    }

    /// Scroll to a specific offset
    pub fn scroll_to(&mut self, x: f32, y: f32) {
        let (max_x, max_y) = self.max_scroll_offset();
        
        self.scroll_offset = (
            x.max(0.0).min(max_x),
            y.max(0.0).min(max_y),
        );
        
        info!("📜 Scrolled to ({}, {})", self.scroll_offset.0, self.scroll_offset.1);
    }

    /// Scroll by a delta amount
    pub fn scroll_by(&mut self, dx: f32, dy: f32) {
        let (x, y) = self.scroll_offset;
        self.scroll_to(x + dx, y + dy);
    }

    /// Scroll to top
    pub fn scroll_to_top(&mut self) {
        self.scroll_to(self.scroll_offset.0, 0.0);
    }

    /// Scroll to bottom
    pub fn scroll_to_bottom(&mut self) {
        let (_, max_y) = self.max_scroll_offset();
        self.scroll_to(self.scroll_offset.0, max_y);
    }

    /// Scroll to left
    pub fn scroll_to_left(&mut self) {
        self.scroll_to(0.0, self.scroll_offset.1);
    }

    /// Scroll to right
    pub fn scroll_to_right(&mut self) {
        let (max_x, _) = self.max_scroll_offset();
        self.scroll_to(max_x, self.scroll_offset.1);
    }

    /// Get current scroll offset
    pub fn get_scroll_offset(&self) -> (f32, f32) {
        self.scroll_offset
    }

    /// Get maximum scroll offset
    pub fn max_scroll_offset(&self) -> (f32, f32) {
        let max_x = (self.content_size.0 - self.viewport_size.0).max(0.0);
        let max_y = (self.content_size.1 - self.viewport_size.1).max(0.0);
        (max_x, max_y)
    }

    /// Check if can scroll vertically
    pub fn can_scroll_vertical(&self) -> bool {
        matches!(self.direction, ScrollDirection::Vertical | ScrollDirection::Both)
            && self.content_size.1 > self.viewport_size.1
    }

    /// Check if can scroll horizontally
    pub fn can_scroll_horizontal(&self) -> bool {
        matches!(self.direction, ScrollDirection::Horizontal | ScrollDirection::Both)
            && self.content_size.0 > self.viewport_size.0
    }

    /// Check if at top
    pub fn is_at_top(&self) -> bool {
        self.scroll_offset.1 <= 0.0
    }

    /// Check if at bottom
    pub fn is_at_bottom(&self) -> bool {
        let (_, max_y) = self.max_scroll_offset();
        self.scroll_offset.1 >= max_y
    }

    /// Check if at left
    pub fn is_at_left(&self) -> bool {
        self.scroll_offset.0 <= 0.0
    }

    /// Check if at right
    pub fn is_at_right(&self) -> bool {
        let (max_x, _) = self.max_scroll_offset();
        self.scroll_offset.0 >= max_x
    }

    /// Get scroll progress (0.0 to 1.0)
    pub fn scroll_progress(&self) -> (f32, f32) {
        let (max_x, max_y) = self.max_scroll_offset();
        let progress_x = if max_x > 0.0 {
            self.scroll_offset.0 / max_x
        } else {
            0.0
        };
        let progress_y = if max_y > 0.0 {
            self.scroll_offset.1 / max_y
        } else {
            0.0
        };
        (progress_x, progress_y)
    }

    /// Update viewport size (called by layout engine)
    pub fn update_viewport_size(&mut self, width: f32, height: f32) {
        self.viewport_size = (width, height);
        info!("📜 Viewport size updated: {}x{}", width, height);
    }

    /// Update content size (called by layout engine)
    pub fn update_content_size(&mut self, width: f32, height: f32) {
        self.content_size = (width, height);
        info!("📜 Content size updated: {}x{}", width, height);
    }

    /// Apply momentum scrolling (called each frame)
    pub fn apply_momentum(&mut self, delta_time: f32) {
        if self.velocity.0.abs() < 0.1 && self.velocity.1.abs() < 0.1 {
            self.velocity = (0.0, 0.0);
            return;
        }

        // Apply velocity
        self.scroll_by(
            self.velocity.0 * delta_time,
            self.velocity.1 * delta_time,
        );

        // Apply friction (deceleration)
        let friction = 0.95;
        self.velocity.0 *= friction;
        self.velocity.1 *= friction;
    }

    /// Start momentum scrolling
    pub fn set_velocity(&mut self, vx: f32, vy: f32) {
        self.velocity = (vx, vy);
    }

    /// Stop momentum scrolling
    pub fn stop_momentum(&mut self) {
        self.velocity = (0.0, 0.0);
    }

    /// Build the layout node
    pub fn build(&mut self, engine: &mut LayoutEngine) -> Result<NodeId, String> {
        if self.content.is_none() {
            warn!("⚠️ ScrollView has no content");
            return Err("ScrollView requires content".to_string());
        }

        // Create a container that clips content
        let style = Style {
            size: Size {
                width: self.width.map(Dimension::Length).unwrap_or(Dimension::Auto),
                height: self.height.map(Dimension::Length).unwrap_or(Dimension::Auto),
            },
            // Scrolling is handled manually via scroll_offset
            // The renderer will clip content outside the viewport
            ..Default::default()
        };

        let children = if let Some(content) = self.content {
            vec![content]
        } else {
            vec![]
        };

        let node = engine
            .new_with_children(style, &children)
            .map_err(|e| format!("Failed to create ScrollView: {:?}", e))?;

        self.node_id = Some(node);
        info!(
            "✅ ScrollView built ({:?}, {}x{})",
            self.direction,
            self.width.map(|w| w.to_string()).unwrap_or_else(|| "auto".to_string()),
            self.height.map(|h| h.to_string()).unwrap_or_else(|| "auto".to_string())
        );
        Ok(node)
    }

    /// Get the layout
    pub fn get_layout(&self, engine: &LayoutEngine) -> Option<Layout> {
        self.node_id.and_then(|id| engine.get_layout(id).ok())
    }
}

impl Default for ScrollView {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scroll_view_creation() {
        let scroll = ScrollView::new();
        assert_eq!(scroll.direction, ScrollDirection::Vertical);
        assert_eq!(scroll.scroll_offset, (0.0, 0.0));
        assert!(scroll.show_indicators);
        assert!(scroll.bounces);
    }

    #[test]
    fn scroll_view_direction() {
        let scroll = ScrollView::new().direction(ScrollDirection::Horizontal);
        assert_eq!(scroll.direction, ScrollDirection::Horizontal);
    }

    #[test]
    fn scroll_view_size() {
        let scroll = ScrollView::new().size(400.0, 300.0);
        assert_eq!(scroll.width, Some(400.0));
        assert_eq!(scroll.height, Some(300.0));
    }

    #[test]
    fn scroll_view_indicators() {
        let scroll = ScrollView::new().show_indicators(false);
        assert!(!scroll.show_indicators);
    }

    #[test]
    fn scroll_view_bounces() {
        let scroll = ScrollView::new().bounces(false);
        assert!(!scroll.bounces);
    }

    #[test]
    fn scroll_to() {
        let mut scroll = ScrollView::new();
        scroll.content_size = (1000.0, 2000.0);
        scroll.viewport_size = (400.0, 600.0);

        scroll.scroll_to(100.0, 200.0);
        assert_eq!(scroll.scroll_offset, (100.0, 200.0));
    }

    #[test]
    fn scroll_to_clamped() {
        let mut scroll = ScrollView::new();
        scroll.content_size = (1000.0, 2000.0);
        scroll.viewport_size = (400.0, 600.0);

        // Try to scroll beyond max
        scroll.scroll_to(1000.0, 2000.0);
        
        let (max_x, max_y) = scroll.max_scroll_offset();
        assert_eq!(scroll.scroll_offset, (max_x, max_y));
    }

    #[test]
    fn scroll_to_negative_clamped() {
        let mut scroll = ScrollView::new();
        scroll.content_size = (1000.0, 2000.0);
        scroll.viewport_size = (400.0, 600.0);

        // Try to scroll to negative
        scroll.scroll_to(-100.0, -200.0);
        assert_eq!(scroll.scroll_offset, (0.0, 0.0));
    }

    #[test]
    fn scroll_by() {
        let mut scroll = ScrollView::new();
        scroll.content_size = (1000.0, 2000.0);
        scroll.viewport_size = (400.0, 600.0);

        scroll.scroll_to(100.0, 200.0);
        scroll.scroll_by(50.0, 100.0);
        assert_eq!(scroll.scroll_offset, (150.0, 300.0));
    }

    #[test]
    fn scroll_to_top() {
        let mut scroll = ScrollView::new();
        scroll.content_size = (1000.0, 2000.0);
        scroll.viewport_size = (400.0, 600.0);
        scroll.scroll_to(100.0, 500.0);

        scroll.scroll_to_top();
        assert_eq!(scroll.scroll_offset.1, 0.0);
    }

    #[test]
    fn scroll_to_bottom() {
        let mut scroll = ScrollView::new();
        scroll.content_size = (1000.0, 2000.0);
        scroll.viewport_size = (400.0, 600.0);

        scroll.scroll_to_bottom();
        let (_, max_y) = scroll.max_scroll_offset();
        assert_eq!(scroll.scroll_offset.1, max_y);
    }

    #[test]
    fn scroll_to_left() {
        let mut scroll = ScrollView::new();
        scroll.content_size = (1000.0, 2000.0);
        scroll.viewport_size = (400.0, 600.0);
        scroll.scroll_to(500.0, 100.0);

        scroll.scroll_to_left();
        assert_eq!(scroll.scroll_offset.0, 0.0);
    }

    #[test]
    fn scroll_to_right() {
        let mut scroll = ScrollView::new();
        scroll.content_size = (1000.0, 2000.0);
        scroll.viewport_size = (400.0, 600.0);

        scroll.scroll_to_right();
        let (max_x, _) = scroll.max_scroll_offset();
        assert_eq!(scroll.scroll_offset.0, max_x);
    }

    #[test]
    fn max_scroll_offset() {
        let mut scroll = ScrollView::new();
        scroll.content_size = (1000.0, 2000.0);
        scroll.viewport_size = (400.0, 600.0);

        let (max_x, max_y) = scroll.max_scroll_offset();
        assert_eq!(max_x, 600.0); // 1000 - 400
        assert_eq!(max_y, 1400.0); // 2000 - 600
    }

    #[test]
    fn max_scroll_offset_no_overflow() {
        let mut scroll = ScrollView::new();
        scroll.content_size = (300.0, 400.0);
        scroll.viewport_size = (400.0, 600.0);

        let (max_x, max_y) = scroll.max_scroll_offset();
        assert_eq!(max_x, 0.0);
        assert_eq!(max_y, 0.0);
    }

    #[test]
    fn can_scroll_vertical() {
        let mut scroll = ScrollView::new().direction(ScrollDirection::Vertical);
        scroll.content_size = (400.0, 1000.0);
        scroll.viewport_size = (400.0, 600.0);

        assert!(scroll.can_scroll_vertical());
        assert!(!scroll.can_scroll_horizontal());
    }

    #[test]
    fn can_scroll_horizontal() {
        let mut scroll = ScrollView::new().direction(ScrollDirection::Horizontal);
        scroll.content_size = (1000.0, 600.0);
        scroll.viewport_size = (400.0, 600.0);

        assert!(!scroll.can_scroll_vertical());
        assert!(scroll.can_scroll_horizontal());
    }

    #[test]
    fn can_scroll_both() {
        let mut scroll = ScrollView::new().direction(ScrollDirection::Both);
        scroll.content_size = (1000.0, 1000.0);
        scroll.viewport_size = (400.0, 600.0);

        assert!(scroll.can_scroll_vertical());
        assert!(scroll.can_scroll_horizontal());
    }

    #[test]
    fn is_at_top() {
        let mut scroll = ScrollView::new();
        scroll.content_size = (400.0, 1000.0);
        scroll.viewport_size = (400.0, 600.0);

        assert!(scroll.is_at_top());
        scroll.scroll_to(0.0, 100.0);
        assert!(!scroll.is_at_top());
    }

    #[test]
    fn is_at_bottom() {
        let mut scroll = ScrollView::new();
        scroll.content_size = (400.0, 1000.0);
        scroll.viewport_size = (400.0, 600.0);

        assert!(!scroll.is_at_bottom());
        scroll.scroll_to_bottom();
        assert!(scroll.is_at_bottom());
    }

    #[test]
    fn is_at_left() {
        let mut scroll = ScrollView::new();
        scroll.content_size = (1000.0, 600.0);
        scroll.viewport_size = (400.0, 600.0);

        assert!(scroll.is_at_left());
        scroll.scroll_to(100.0, 0.0);
        assert!(!scroll.is_at_left());
    }

    #[test]
    fn is_at_right() {
        let mut scroll = ScrollView::new();
        scroll.content_size = (1000.0, 600.0);
        scroll.viewport_size = (400.0, 600.0);

        assert!(!scroll.is_at_right());
        scroll.scroll_to_right();
        assert!(scroll.is_at_right());
    }

    #[test]
    fn scroll_progress() {
        let mut scroll = ScrollView::new();
        scroll.content_size = (1000.0, 2000.0);
        scroll.viewport_size = (400.0, 600.0);

        // At start
        let (px, py) = scroll.scroll_progress();
        assert_eq!(px, 0.0);
        assert_eq!(py, 0.0);

        // Halfway
        scroll.scroll_to(300.0, 700.0);
        let (px, py) = scroll.scroll_progress();
        assert!((px - 0.5).abs() < 0.01);
        assert!((py - 0.5).abs() < 0.01);

        // At end
        scroll.scroll_to_right();
        scroll.scroll_to_bottom();
        let (px, py) = scroll.scroll_progress();
        assert_eq!(px, 1.0);
        assert_eq!(py, 1.0);
    }

    #[test]
    fn momentum_scrolling() {
        let mut scroll = ScrollView::new();
        scroll.content_size = (1000.0, 2000.0);
        scroll.viewport_size = (400.0, 600.0);

        scroll.set_velocity(100.0, 200.0);
        assert_eq!(scroll.velocity, (100.0, 200.0));

        scroll.apply_momentum(0.016); // ~60 FPS
        assert!(scroll.scroll_offset.0 > 0.0);
        assert!(scroll.scroll_offset.1 > 0.0);

        scroll.stop_momentum();
        assert_eq!(scroll.velocity, (0.0, 0.0));
    }

    #[test]
    fn update_viewport_size() {
        let mut scroll = ScrollView::new();
        scroll.update_viewport_size(800.0, 600.0);
        assert_eq!(scroll.viewport_size, (800.0, 600.0));
    }

    #[test]
    fn update_content_size() {
        let mut scroll = ScrollView::new();
        scroll.update_content_size(1200.0, 1600.0);
        assert_eq!(scroll.content_size, (1200.0, 1600.0));
    }

    #[test]
    fn scroll_view_default() {
        let scroll = ScrollView::default();
        assert_eq!(scroll.direction, ScrollDirection::Vertical);
    }

    #[test]
    fn scroll_view_clone() {
        let scroll1 = ScrollView::new().size(400.0, 300.0);
        let scroll2 = scroll1.clone();
        assert_eq!(scroll1.width, scroll2.width);
        assert_eq!(scroll1.height, scroll2.height);
    }

    #[test]
    fn scroll_direction_equality() {
        assert_eq!(ScrollDirection::Vertical, ScrollDirection::Vertical);
        assert_ne!(ScrollDirection::Vertical, ScrollDirection::Horizontal);
        assert_ne!(ScrollDirection::Horizontal, ScrollDirection::Both);
    }
}
