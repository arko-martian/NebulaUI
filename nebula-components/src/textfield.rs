use nebula_core::{Signal, LayoutEngine, NodeId, Layout};
use taffy::prelude::*;
use tracing::info;
use std::rc::Rc;

/// TextField - Text input component ✏️
/// 
/// Essential for forms, search, chat, and more!
/// - Reactive text content (powered by Signals!)
/// - Cursor position tracking
/// - Keyboard input handling
/// - Focus state
/// - Placeholder text
/// - Input validation
/// 
/// Just like HTML's input, but better!
#[derive(Clone)]
pub struct TextField {
    /// Layout node ID
    pub node_id: Option<NodeId>,
    /// Text content (reactive!)
    pub text: Signal<String>,
    /// Cursor position (index in string)
    pub cursor_position: Signal<usize>,
    /// Is focused?
    pub is_focused: Signal<bool>,
    /// Placeholder text (shown when empty)
    pub placeholder: Option<String>,
    /// Maximum length (None = unlimited)
    pub max_length: Option<usize>,
    /// Width
    pub width: f32,
    /// Height
    pub height: f32,
    /// Position
    pub position: (f32, f32),
    /// Change handler
    on_change: Option<Rc<dyn Fn(String)>>,
    /// Submit handler (Enter key)
    on_submit: Option<Rc<dyn Fn(String)>>,
}

impl TextField {
    /// Create a new text field
    pub fn new() -> Self {
        info!("✏️ Creating TextField");
        Self {
            node_id: None,
            text: Signal::new(String::new()),
            cursor_position: Signal::new(0),
            is_focused: Signal::new(false),
            placeholder: None,
            max_length: None,
            width: 200.0,
            height: 40.0,
            position: (0.0, 0.0),
            on_change: None,
            on_submit: None,
        }
    }

    /// Create a text field with initial text
    pub fn with_text(text: impl Into<String>) -> Self {
        let text_str = text.into();
        let cursor_pos = text_str.len();
        info!("✏️ Creating TextField with text: '{}'", text_str);
        Self {
            node_id: None,
            text: Signal::new(text_str),
            cursor_position: Signal::new(cursor_pos),
            is_focused: Signal::new(false),
            placeholder: None,
            max_length: None,
            width: 200.0,
            height: 40.0,
            position: (0.0, 0.0),
            on_change: None,
            on_submit: None,
        }
    }

    /// Set placeholder text
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    /// Set maximum length
    pub fn max_length(mut self, max_length: usize) -> Self {
        self.max_length = Some(max_length);
        self
    }

    /// Set width
    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    /// Set height
    pub fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    /// Set position
    pub fn position(mut self, x: f32, y: f32) -> Self {
        self.position = (x, y);
        self
    }

    /// Set change handler
    pub fn on_change<F>(mut self, handler: F) -> Self
    where
        F: Fn(String) + 'static,
    {
        self.on_change = Some(Rc::new(handler));
        self
    }

    /// Set submit handler (Enter key)
    pub fn on_submit<F>(mut self, handler: F) -> Self
    where
        F: Fn(String) + 'static,
    {
        self.on_submit = Some(Rc::new(handler));
        self
    }

    /// Get text content
    pub fn get_text(&self) -> String {
        self.text.get()
    }

    /// Set text content
    pub fn set_text(&self, text: impl Into<String>) {
        let text = text.into();
        
        // Apply max length
        let text = if let Some(max_len) = self.max_length {
            text.chars().take(max_len).collect()
        } else {
            text
        };
        
        self.text.set(text.clone());
        
        // Move cursor to end
        self.cursor_position.set(text.len());
        
        // Call change handler
        if let Some(handler) = &self.on_change {
            handler(text);
        }
    }

    /// Insert character at cursor
    pub fn insert_char(&self, c: char) {
        let mut text = self.get_text();
        let cursor = self.cursor_position.get();
        
        // Check max length
        if let Some(max_len) = self.max_length {
            if text.len() >= max_len {
                return;
            }
        }
        
        // Insert character
        text.insert(cursor, c);
        self.text.set(text.clone());
        
        // Move cursor forward
        self.cursor_position.set(cursor + 1);
        
        info!("✏️ Inserted '{}' at position {}", c, cursor);
        
        // Call change handler
        if let Some(handler) = &self.on_change {
            handler(text);
        }
    }

    /// Delete character before cursor (Backspace)
    pub fn delete_before_cursor(&self) {
        let mut text = self.get_text();
        let cursor = self.cursor_position.get();
        
        if cursor > 0 {
            text.remove(cursor - 1);
            self.text.set(text.clone());
            self.cursor_position.set(cursor - 1);
            
            info!("✏️ Deleted character at position {}", cursor - 1);
            
            // Call change handler
            if let Some(handler) = &self.on_change {
                handler(text);
            }
        }
    }

    /// Delete character at cursor (Delete)
    pub fn delete_at_cursor(&self) {
        let mut text = self.get_text();
        let cursor = self.cursor_position.get();
        
        if cursor < text.len() {
            text.remove(cursor);
            self.text.set(text.clone());
            
            info!("✏️ Deleted character at position {}", cursor);
            
            // Call change handler
            if let Some(handler) = &self.on_change {
                handler(text);
            }
        }
    }

    /// Move cursor left
    pub fn move_cursor_left(&self) {
        let cursor = self.cursor_position.get();
        if cursor > 0 {
            self.cursor_position.set(cursor - 1);
        }
    }

    /// Move cursor right
    pub fn move_cursor_right(&self) {
        let cursor = self.cursor_position.get();
        let text_len = self.get_text().len();
        if cursor < text_len {
            self.cursor_position.set(cursor + 1);
        }
    }

    /// Move cursor to start
    pub fn move_cursor_to_start(&self) {
        self.cursor_position.set(0);
    }

    /// Move cursor to end
    pub fn move_cursor_to_end(&self) {
        let text_len = self.get_text().len();
        self.cursor_position.set(text_len);
    }

    /// Submit (Enter key)
    pub fn submit(&self) {
        let text = self.get_text();
        info!("✏️ TextField submitted: '{}'", text);
        
        if let Some(handler) = &self.on_submit {
            handler(text);
        }
    }

    /// Focus the text field
    pub fn focus(&self) {
        self.is_focused.set(true);
        info!("✏️ TextField focused");
    }

    /// Blur (unfocus) the text field
    pub fn blur(&self) {
        self.is_focused.set(false);
        info!("✏️ TextField blurred");
    }

    /// Check if focused
    pub fn is_focused(&self) -> bool {
        self.is_focused.get()
    }

    /// Clear the text field
    pub fn clear(&self) {
        self.text.set(String::new());
        self.cursor_position.set(0);
        
        if let Some(handler) = &self.on_change {
            handler(String::new());
        }
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.get_text().is_empty()
    }

    /// Get cursor position
    pub fn get_cursor_position(&self) -> usize {
        self.cursor_position.get()
    }

    /// Handle mouse click (focus and position cursor)
    pub fn handle_click(&self, mouse_x: f32, mouse_y: f32) -> bool {
        if self.is_point_inside(mouse_x, mouse_y) {
            self.focus();
            // TODO: Calculate cursor position from mouse x
            // For now, just move to end
            self.move_cursor_to_end();
            true
        } else {
            false
        }
    }

    /// Check if a point is inside the text field
    pub fn is_point_inside(&self, x: f32, y: f32) -> bool {
        let (tx, ty) = self.position;
        let (tw, th) = (self.width, self.height);
        
        x >= tx && x <= tx + tw && y >= ty && y <= ty + th
    }

    /// Build the layout node
    pub fn build(&mut self, engine: &mut LayoutEngine) -> Result<NodeId, String> {
        let style = Style {
            size: Size {
                width: Dimension::Length(self.width),
                height: Dimension::Length(self.height),
            },
            ..Default::default()
        };

        let node = engine
            .new_leaf(style)
            .map_err(|e| format!("Failed to create TextField: {:?}", e))?;

        self.node_id = Some(node);
        info!("✅ TextField built ({}x{})", self.width, self.height);
        Ok(node)
    }

    /// Get the layout
    pub fn get_layout(&self, engine: &LayoutEngine) -> Option<Layout> {
        self.node_id.and_then(|id| engine.get_layout(id).ok())
    }

    /// Get bounds (x, y, width, height)
    pub fn bounds(&self) -> (f32, f32, f32, f32) {
        (self.position.0, self.position.1, self.width, self.height)
    }
}

impl Default for TextField {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    #[test]
    fn textfield_creation() {
        let field = TextField::new();
        assert_eq!(field.get_text(), "");
        assert_eq!(field.get_cursor_position(), 0);
        assert_eq!(field.is_focused(), false);
        assert_eq!(field.width, 200.0);
        assert_eq!(field.height, 40.0);
    }

    #[test]
    fn textfield_with_text() {
        let field = TextField::with_text("Hello");
        assert_eq!(field.get_text(), "Hello");
        assert_eq!(field.get_cursor_position(), 5);
    }

    #[test]
    fn textfield_builder_pattern() {
        let field = TextField::new()
            .placeholder("Enter name")
            .max_length(50)
            .width(300.0)
            .height(50.0)
            .position(10.0, 20.0);

        assert_eq!(field.placeholder, Some("Enter name".to_string()));
        assert_eq!(field.max_length, Some(50));
        assert_eq!(field.width, 300.0);
        assert_eq!(field.height, 50.0);
        assert_eq!(field.position, (10.0, 20.0));
    }

    #[test]
    fn textfield_set_text() {
        let field = TextField::new();
        field.set_text("World");
        assert_eq!(field.get_text(), "World");
        assert_eq!(field.get_cursor_position(), 5);
    }

    #[test]
    fn textfield_insert_char() {
        let field = TextField::new();
        field.insert_char('H');
        field.insert_char('i');
        assert_eq!(field.get_text(), "Hi");
        assert_eq!(field.get_cursor_position(), 2);
    }

    #[test]
    fn textfield_delete_before_cursor() {
        let field = TextField::with_text("Hello");
        field.delete_before_cursor();
        assert_eq!(field.get_text(), "Hell");
        assert_eq!(field.get_cursor_position(), 4);
    }

    #[test]
    fn textfield_delete_at_cursor() {
        let field = TextField::with_text("Hello");
        field.move_cursor_to_start();
        field.delete_at_cursor();
        assert_eq!(field.get_text(), "ello");
        assert_eq!(field.get_cursor_position(), 0);
    }

    #[test]
    fn textfield_cursor_movement() {
        let field = TextField::with_text("Hello");
        
        field.move_cursor_to_start();
        assert_eq!(field.get_cursor_position(), 0);
        
        field.move_cursor_right();
        assert_eq!(field.get_cursor_position(), 1);
        
        field.move_cursor_left();
        assert_eq!(field.get_cursor_position(), 0);
        
        field.move_cursor_to_end();
        assert_eq!(field.get_cursor_position(), 5);
    }

    #[test]
    fn textfield_max_length() {
        let field = TextField::new().max_length(3);
        field.insert_char('A');
        field.insert_char('B');
        field.insert_char('C');
        field.insert_char('D'); // Should be ignored
        
        assert_eq!(field.get_text(), "ABC");
    }

    #[test]
    fn textfield_focus_blur() {
        let field = TextField::new();
        assert_eq!(field.is_focused(), false);
        
        field.focus();
        assert_eq!(field.is_focused(), true);
        
        field.blur();
        assert_eq!(field.is_focused(), false);
    }

    #[test]
    fn textfield_clear() {
        let field = TextField::with_text("Hello");
        field.clear();
        assert_eq!(field.get_text(), "");
        assert_eq!(field.get_cursor_position(), 0);
    }

    #[test]
    fn textfield_is_empty() {
        let field = TextField::new();
        assert!(field.is_empty());
        
        field.insert_char('A');
        assert!(!field.is_empty());
    }

    #[test]
    fn textfield_on_change_handler() {
        let changed_text = Rc::new(RefCell::new(String::new()));
        let changed_text_clone = changed_text.clone();

        let field = TextField::new().on_change(move |text| {
            *changed_text_clone.borrow_mut() = text;
        });

        field.insert_char('H');
        assert_eq!(*changed_text.borrow(), "H");
        
        field.insert_char('i');
        assert_eq!(*changed_text.borrow(), "Hi");
    }

    #[test]
    fn textfield_on_submit_handler() {
        let submitted_text = Rc::new(RefCell::new(String::new()));
        let submitted_text_clone = submitted_text.clone();

        let field = TextField::with_text("Hello").on_submit(move |text| {
            *submitted_text_clone.borrow_mut() = text;
        });

        field.submit();
        assert_eq!(*submitted_text.borrow(), "Hello");
    }

    #[test]
    fn textfield_handle_click() {
        let field = TextField::new()
            .position(10.0, 10.0)
            .width(200.0)
            .height(40.0);

        assert_eq!(field.is_focused(), false);
        
        let clicked = field.handle_click(100.0, 25.0);
        assert!(clicked);
        assert_eq!(field.is_focused(), true);
    }

    #[test]
    fn textfield_is_point_inside() {
        let field = TextField::new()
            .position(10.0, 10.0)
            .width(200.0)
            .height(40.0);

        assert!(field.is_point_inside(100.0, 25.0));
        assert!(field.is_point_inside(10.0, 10.0));
        assert!(field.is_point_inside(210.0, 50.0));
        
        assert!(!field.is_point_inside(5.0, 25.0));
        assert!(!field.is_point_inside(215.0, 25.0));
    }

    #[test]
    fn textfield_bounds() {
        let field = TextField::new()
            .position(10.0, 20.0)
            .width(300.0)
            .height(50.0);

        let (x, y, w, h) = field.bounds();
        assert_eq!(x, 10.0);
        assert_eq!(y, 20.0);
        assert_eq!(w, 300.0);
        assert_eq!(h, 50.0);
    }

    #[test]
    fn textfield_build() {
        let mut engine = LayoutEngine::new();
        let mut field = TextField::new();

        let node = field.build(&mut engine);
        assert!(node.is_ok());
        assert!(field.node_id.is_some());
    }

    #[test]
    fn textfield_layout() {
        let mut engine = LayoutEngine::new();
        let mut field = TextField::new().width(300.0).height(50.0);

        let node = field.build(&mut engine).unwrap();

        let available = Size {
            width: AvailableSpace::Definite(400.0),
            height: AvailableSpace::Definite(100.0),
        };
        engine.compute_layout(node, available).unwrap();

        let layout = field.get_layout(&engine);
        assert!(layout.is_some());

        let layout = layout.unwrap();
        assert_eq!(layout.size.width, 300.0);
        assert_eq!(layout.size.height, 50.0);
    }
}
