// Dialog Component - Modal dialog box with title, content, and actions
// Built on top of Modal for consistent overlay behavior

use crate::container::VStack;
use crate::modal::Modal;
use nebula_core::layout::{LayoutEngine, NodeId};

/// Dialog type determines the visual style and default buttons
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DialogType {
    /// Information dialog (OK button)
    Info,
    /// Warning dialog (OK button, yellow accent)
    Warning,
    /// Error dialog (OK button, red accent)
    Error,
    /// Confirmation dialog (OK and Cancel buttons)
    Confirm,
    /// Custom dialog (user-defined buttons)
    Custom,
}

/// Dialog component - displays a modal dialog with title, content, and action buttons
/// 
/// # Example
/// ```
/// let mut dialog = Dialog::new()
///     .title("Confirm Action")
///     .message("Are you sure you want to continue?")
///     .dialog_type(DialogType::Confirm)
///     .on_confirm(|| println!("Confirmed!"))
///     .on_cancel(|| println!("Cancelled!"));
/// ```
pub struct Dialog {
    pub node_id: Option<NodeId>,
    pub modal: Modal,
    pub title: String,
    pub message: String,
    pub dialog_type: DialogType,
    pub width: f32,
    pub height: Option<f32>, // None = auto height
    pub padding: f32,
    pub border_radius: f32,
    pub background_color: (u8, u8, u8, u8), // RGBA
    pub title_color: (u8, u8, u8, u8),
    pub message_color: (u8, u8, u8, u8),
    pub on_confirm: Option<Box<dyn Fn()>>,
    pub on_cancel: Option<Box<dyn Fn()>>,
    pub on_close: Option<Box<dyn Fn()>>,
    pub confirm_text: String,
    pub cancel_text: String,
    pub show_close_button: bool,
    pub closable_on_backdrop: bool,
}

impl Dialog {
    /// Create a new Dialog component
    pub fn new() -> Self {
        Self {
            node_id: None,
            modal: Modal::new(),
            title: String::new(),
            message: String::new(),
            dialog_type: DialogType::Info,
            width: 400.0,
            height: None,
            padding: 24.0,
            border_radius: 12.0,
            background_color: (255, 255, 255, 255), // White
            title_color: (0, 0, 0, 255), // Black
            message_color: (64, 64, 64, 255), // Dark gray
            on_confirm: None,
            on_cancel: None,
            on_close: None,
            confirm_text: "OK".to_string(),
            cancel_text: "Cancel".to_string(),
            show_close_button: true,
            closable_on_backdrop: true,
        }
    }

    /// Set the dialog title
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    /// Set the dialog message/content
    pub fn message(mut self, message: impl Into<String>) -> Self {
        self.message = message.into();
        self
    }

    /// Set the dialog type
    pub fn dialog_type(mut self, dialog_type: DialogType) -> Self {
        self.dialog_type = dialog_type;
        
        // Update default button text based on type
        match dialog_type {
            DialogType::Confirm => {
                self.confirm_text = "OK".to_string();
                self.cancel_text = "Cancel".to_string();
            }
            DialogType::Error | DialogType::Warning | DialogType::Info => {
                self.confirm_text = "OK".to_string();
            }
            DialogType::Custom => {
                // Keep user-defined text
            }
        }
        
        self
    }

    /// Set the dialog width
    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    /// Set the dialog height (None for auto)
    pub fn height(mut self, height: f32) -> Self {
        self.height = Some(height);
        self
    }

    /// Set the padding
    pub fn padding(mut self, padding: f32) -> Self {
        self.padding = padding;
        self
    }

    /// Set the border radius
    pub fn border_radius(mut self, radius: f32) -> Self {
        self.border_radius = radius;
        self
    }

    /// Set the background color
    pub fn background_color(mut self, r: u8, g: u8, b: u8, a: u8) -> Self {
        self.background_color = (r, g, b, a);
        self
    }

    /// Set the title color
    pub fn title_color(mut self, r: u8, g: u8, b: u8, a: u8) -> Self {
        self.title_color = (r, g, b, a);
        self
    }

    /// Set the message color
    pub fn message_color(mut self, r: u8, g: u8, b: u8, a: u8) -> Self {
        self.message_color = (r, g, b, a);
        self
    }

    /// Set the confirm button callback
    pub fn on_confirm<F>(mut self, callback: F) -> Self
    where
        F: Fn() + 'static,
    {
        self.on_confirm = Some(Box::new(callback));
        self
    }

    /// Set the cancel button callback
    pub fn on_cancel<F>(mut self, callback: F) -> Self
    where
        F: Fn() + 'static,
    {
        self.on_cancel = Some(Box::new(callback));
        self
    }

    /// Set the close button callback
    pub fn on_close<F>(mut self, callback: F) -> Self
    where
        F: Fn() + 'static,
    {
        self.on_close = Some(Box::new(callback));
        self
    }

    /// Set the confirm button text
    pub fn confirm_text(mut self, text: impl Into<String>) -> Self {
        self.confirm_text = text.into();
        self
    }

    /// Set the cancel button text
    pub fn cancel_text(mut self, text: impl Into<String>) -> Self {
        self.cancel_text = text.into();
        self
    }

    /// Set whether to show the close button
    pub fn show_close_button(mut self, show: bool) -> Self {
        self.show_close_button = show;
        self
    }

    /// Set whether clicking the backdrop closes the dialog
    pub fn closable_on_backdrop(mut self, closable: bool) -> Self {
        self.closable_on_backdrop = closable;
        self.modal = self.modal.close_on_backdrop_click(closable);
        self
    }

    /// Show the dialog
    pub fn show(&mut self) {
        self.modal.show();
    }

    /// Hide the dialog
    pub fn hide(&mut self) {
        self.modal.hide();
    }

    /// Toggle the dialog visibility
    pub fn toggle(&mut self) {
        self.modal.toggle();
    }

    /// Check if the dialog is currently visible
    pub fn is_visible(&self) -> bool {
        self.modal.is_visible()
    }

    /// Handle confirm button click
    pub fn handle_confirm(&mut self) {
        if let Some(ref callback) = self.on_confirm {
            callback();
        }
        self.hide();
    }

    /// Handle cancel button click
    pub fn handle_cancel(&mut self) {
        if let Some(ref callback) = self.on_cancel {
            callback();
        }
        self.hide();
    }

    /// Handle close button click
    pub fn handle_close(&mut self) {
        if let Some(ref callback) = self.on_close {
            callback();
        }
        self.hide();
    }

    /// Get the accent color based on dialog type
    pub fn get_accent_color(&self) -> (u8, u8, u8, u8) {
        match self.dialog_type {
            DialogType::Info => (0, 122, 255, 255),      // Blue
            DialogType::Warning => (255, 149, 0, 255),   // Orange
            DialogType::Error => (255, 59, 48, 255),     // Red
            DialogType::Confirm => (52, 199, 89, 255),   // Green
            DialogType::Custom => (0, 122, 255, 255),    // Blue (default)
        }
    }

    /// Check if the dialog should show a cancel button
    pub fn should_show_cancel(&self) -> bool {
        matches!(self.dialog_type, DialogType::Confirm | DialogType::Custom)
    }

    /// Build the dialog layout
    pub fn build(&mut self, engine: &mut LayoutEngine) -> Result<NodeId, String> {
        // Build the modal first
        let modal_node = self.modal.build(engine)?;
        
        if !self.is_visible() {
            self.node_id = Some(modal_node);
            return Ok(modal_node);
        }

        // Create the dialog content container (VStack)
        let mut content = VStack::new()
            .spacing(16.0)
            .padding(self.padding);
        
        let content_node = content.build(engine)?;
        
        // Set dialog box styling
        let style = taffy::style::Style {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Length(self.width),
                height: if let Some(h) = self.height {
                    taffy::style::Dimension::Length(h)
                } else {
                    taffy::style::Dimension::Auto
                },
            },
            ..Default::default()
        };
        engine.set_style(content_node, style)
            .map_err(|e| format!("Failed to set dialog style: {:?}", e))?;

        // Set the content as the modal's content
        self.modal.set_content(content_node);
        
        self.node_id = Some(modal_node);
        Ok(modal_node)
    }
}

impl Default for Dialog {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dialog_starts_hidden() {
        let dialog = Dialog::new();
        assert!(!dialog.is_visible());
    }

    #[test]
    fn dialog_can_be_shown() {
        let mut dialog = Dialog::new();
        dialog.show();
        assert!(dialog.is_visible());
    }

    #[test]
    fn dialog_can_be_hidden() {
        let mut dialog = Dialog::new();
        dialog.show();
        dialog.hide();
        assert!(!dialog.is_visible());
    }

    #[test]
    fn dialog_builder_pattern() {
        let dialog = Dialog::new()
            .title("Test Title")
            .message("Test Message")
            .dialog_type(DialogType::Confirm)
            .width(500.0)
            .height(300.0)
            .padding(32.0)
            .border_radius(16.0);

        assert_eq!(dialog.title, "Test Title");
        assert_eq!(dialog.message, "Test Message");
        assert_eq!(dialog.dialog_type, DialogType::Confirm);
        assert_eq!(dialog.width, 500.0);
        assert_eq!(dialog.height, Some(300.0));
        assert_eq!(dialog.padding, 32.0);
        assert_eq!(dialog.border_radius, 16.0);
    }

    #[test]
    fn dialog_type_sets_default_button_text() {
        let info_dialog = Dialog::new().dialog_type(DialogType::Info);
        assert_eq!(info_dialog.confirm_text, "OK");

        let confirm_dialog = Dialog::new().dialog_type(DialogType::Confirm);
        assert_eq!(confirm_dialog.confirm_text, "OK");
        assert_eq!(confirm_dialog.cancel_text, "Cancel");
    }

    #[test]
    fn dialog_custom_button_text() {
        let dialog = Dialog::new()
            .confirm_text("Yes")
            .cancel_text("No");

        assert_eq!(dialog.confirm_text, "Yes");
        assert_eq!(dialog.cancel_text, "No");
    }

    #[test]
    fn dialog_accent_colors() {
        let info = Dialog::new().dialog_type(DialogType::Info);
        assert_eq!(info.get_accent_color(), (0, 122, 255, 255)); // Blue

        let warning = Dialog::new().dialog_type(DialogType::Warning);
        assert_eq!(warning.get_accent_color(), (255, 149, 0, 255)); // Orange

        let error = Dialog::new().dialog_type(DialogType::Error);
        assert_eq!(error.get_accent_color(), (255, 59, 48, 255)); // Red

        let confirm = Dialog::new().dialog_type(DialogType::Confirm);
        assert_eq!(confirm.get_accent_color(), (52, 199, 89, 255)); // Green
    }

    #[test]
    fn dialog_should_show_cancel_button() {
        let info = Dialog::new().dialog_type(DialogType::Info);
        assert!(!info.should_show_cancel());

        let confirm = Dialog::new().dialog_type(DialogType::Confirm);
        assert!(confirm.should_show_cancel());

        let custom = Dialog::new().dialog_type(DialogType::Custom);
        assert!(custom.should_show_cancel());
    }

    #[test]
    fn dialog_confirm_callback_is_called() {
        use std::sync::{Arc, Mutex};
        
        let confirmed = Arc::new(Mutex::new(false));
        let confirmed_clone = confirmed.clone();
        
        let mut dialog = Dialog::new()
            .on_confirm(move || {
                *confirmed_clone.lock().unwrap() = true;
            });
        
        dialog.show();
        dialog.handle_confirm();
        
        assert!(*confirmed.lock().unwrap());
        assert!(!dialog.is_visible()); // Should hide after confirm
    }

    #[test]
    fn dialog_cancel_callback_is_called() {
        use std::sync::{Arc, Mutex};
        
        let cancelled = Arc::new(Mutex::new(false));
        let cancelled_clone = cancelled.clone();
        
        let mut dialog = Dialog::new()
            .on_cancel(move || {
                *cancelled_clone.lock().unwrap() = true;
            });
        
        dialog.show();
        dialog.handle_cancel();
        
        assert!(*cancelled.lock().unwrap());
        assert!(!dialog.is_visible()); // Should hide after cancel
    }

    #[test]
    fn dialog_close_callback_is_called() {
        use std::sync::{Arc, Mutex};
        
        let closed = Arc::new(Mutex::new(false));
        let closed_clone = closed.clone();
        
        let mut dialog = Dialog::new()
            .on_close(move || {
                *closed_clone.lock().unwrap() = true;
            });
        
        dialog.show();
        dialog.handle_close();
        
        assert!(*closed.lock().unwrap());
        assert!(!dialog.is_visible()); // Should hide after close
    }

    #[test]
    fn dialog_build_creates_node() {
        let mut engine = LayoutEngine::new();
        let mut dialog = Dialog::new()
            .title("Test")
            .message("Test message");
        
        dialog.show();
        let result = dialog.build(&mut engine);
        assert!(result.is_ok());
        assert!(dialog.node_id.is_some());
    }

    #[test]
    fn dialog_closable_on_backdrop() {
        let dialog = Dialog::new().closable_on_backdrop(false);
        assert!(!dialog.closable_on_backdrop);
        assert!(!dialog.modal.close_on_backdrop_click);

        let dialog = Dialog::new().closable_on_backdrop(true);
        assert!(dialog.closable_on_backdrop);
        assert!(dialog.modal.close_on_backdrop_click);
    }
}
