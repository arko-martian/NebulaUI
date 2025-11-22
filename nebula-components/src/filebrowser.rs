// FileBrowser Component - File system browser
// Essential for file navigation and selection

use nebula_core::layout::{LayoutEngine, NodeId};
use nebula_core::signal::Signal;

/// File entry type
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FileType {
    File,
    Directory,
}

/// File entry
#[derive(Debug, Clone, PartialEq)]
pub struct FileEntry {
    pub name: String,
    pub path: String,
    pub file_type: FileType,
    pub size: Option<usize>,
    pub modified: Option<String>,
    pub icon: Option<String>,
}

impl FileEntry {
    /// Create a new file entry
    pub fn new(name: impl Into<String>, path: impl Into<String>, file_type: FileType) -> Self {
        Self {
            name: name.into(),
            path: path.into(),
            file_type,
            size: None,
            modified: None,
            icon: None,
        }
    }

    /// Create a file
    pub fn file(name: impl Into<String>, path: impl Into<String>) -> Self {
        Self::new(name, path, FileType::File)
    }

    /// Create a directory
    pub fn directory(name: impl Into<String>, path: impl Into<String>) -> Self {
        Self::new(name, path, FileType::Directory)
    }

    /// Set size
    pub fn with_size(mut self, size: usize) -> Self {
        self.size = Some(size);
        self
    }

    /// Set modified time
    pub fn with_modified(mut self, modified: impl Into<String>) -> Self {
        self.modified = Some(modified.into());
        self
    }

    /// Set icon
    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Check if is directory
    pub fn is_directory(&self) -> bool {
        self.file_type == FileType::Directory
    }

    /// Check if is file
    pub fn is_file(&self) -> bool {
        self.file_type == FileType::File
    }

    /// Get file extension
    pub fn extension(&self) -> Option<&str> {
        if self.is_file() {
            self.name.rsplit('.').next().filter(|ext| !ext.is_empty() && ext != &self.name)
        } else {
            None
        }
    }

    /// Get size in KB
    pub fn size_kb(&self) -> Option<f64> {
        self.size.map(|s| s as f64 / 1024.0)
    }

    /// Get size in MB
    pub fn size_mb(&self) -> Option<f64> {
        self.size.map(|s| s as f64 / (1024.0 * 1024.0))
    }
}

/// FileBrowser component - file system browser
/// 
/// # Example
/// ```
/// let mut browser = FileBrowser::new()
///     .current_path("/home/user")
///     .add_entry(FileEntry::directory("Documents", "/home/user/Documents"))
///     .add_entry(FileEntry::file("file.txt", "/home/user/file.txt"))
///     .on_select(|entry| println!("Selected: {}", entry.name));
/// ```
pub struct FileBrowser {
    pub node_id: Option<NodeId>,
    pub current_path: Signal<String>,
    pub entries: Signal<Vec<FileEntry>>,
    pub selected_entry: Signal<Option<String>>,
    pub show_hidden: bool,
    pub show_size: bool,
    pub show_modified: bool,
    pub item_height: f32,
    pub padding: f32,
    pub background_color: (u8, u8, u8, u8),
    pub selected_color: (u8, u8, u8, u8),
    pub hover_color: (u8, u8, u8, u8),
    pub directory_color: (u8, u8, u8, u8),
    pub file_color: (u8, u8, u8, u8),
    pub text_color: (u8, u8, u8, u8),
    pub on_select: Option<Box<dyn Fn(&FileEntry)>>,
    pub on_navigate: Option<Box<dyn Fn(&str)>>,
    pub on_double_click: Option<Box<dyn Fn(&FileEntry)>>,
}

impl FileBrowser {
    /// Create a new FileBrowser component
    pub fn new() -> Self {
        Self {
            node_id: None,
            current_path: Signal::new("/".to_string()),
            entries: Signal::new(Vec::new()),
            selected_entry: Signal::new(None),
            show_hidden: false,
            show_size: true,
            show_modified: true,
            item_height: 40.0,
            padding: 16.0,
            background_color: (255, 255, 255, 255),
            selected_color: (59, 130, 246, 20), // Light blue
            hover_color: (245, 245, 245, 255),
            directory_color: (59, 130, 246, 255), // Blue
            file_color: (100, 100, 100, 255), // Gray
            text_color: (0, 0, 0, 255),
            on_select: None,
            on_navigate: None,
            on_double_click: None,
        }
    }

    /// Set current path
    pub fn current_path(mut self, path: impl Into<String>) -> Self {
        self.current_path.set(path.into());
        self
    }

    /// Show or hide hidden files
    pub fn show_hidden(mut self, show: bool) -> Self {
        self.show_hidden = show;
        self
    }

    /// Show or hide file sizes
    pub fn show_size(mut self, show: bool) -> Self {
        self.show_size = show;
        self
    }

    /// Show or hide modified times
    pub fn show_modified(mut self, show: bool) -> Self {
        self.show_modified = show;
        self
    }

    /// Set item height
    pub fn item_height(mut self, height: f32) -> Self {
        self.item_height = height;
        self
    }

    /// Set padding
    pub fn padding(mut self, padding: f32) -> Self {
        self.padding = padding;
        self
    }

    /// Set selected color
    pub fn selected_color(mut self, r: u8, g: u8, b: u8, a: u8) -> Self {
        self.selected_color = (r, g, b, a);
        self
    }

    /// Set directory color
    pub fn directory_color(mut self, r: u8, g: u8, b: u8, a: u8) -> Self {
        self.directory_color = (r, g, b, a);
        self
    }

    /// Add an entry
    pub fn add_entry(mut self, entry: FileEntry) -> Self {
        let mut entries = self.entries.get();
        entries.push(entry);
        self.entries.set(entries);
        self
    }

    /// Set all entries
    pub fn entries(mut self, entries: Vec<FileEntry>) -> Self {
        self.entries.set(entries);
        self
    }

    /// Set the select callback
    pub fn on_select<F>(mut self, callback: F) -> Self
    where
        F: Fn(&FileEntry) + 'static,
    {
        self.on_select = Some(Box::new(callback));
        self
    }

    /// Set the navigate callback
    pub fn on_navigate<F>(mut self, callback: F) -> Self
    where
        F: Fn(&str) + 'static,
    {
        self.on_navigate = Some(Box::new(callback));
        self
    }

    /// Set the double click callback
    pub fn on_double_click<F>(mut self, callback: F) -> Self
    where
        F: Fn(&FileEntry) + 'static,
    {
        self.on_double_click = Some(Box::new(callback));
        self
    }

    /// Select an entry by path
    pub fn select_entry(&mut self, path: &str) {
        if let Some(entry) = self.find_entry(path) {
            self.selected_entry.set(Some(path.to_string()));
            
            if let Some(ref callback) = self.on_select {
                callback(&entry);
            }
        }
    }

    /// Deselect current entry
    pub fn deselect(&mut self) {
        self.selected_entry.set(None);
    }

    /// Get selected entry
    pub fn get_selected_entry(&self) -> Option<FileEntry> {
        self.selected_entry.get().and_then(|path| self.find_entry(&path))
    }

    /// Navigate to path
    pub fn navigate_to(&mut self, path: impl Into<String>) {
        let path = path.into();
        self.current_path.set(path.clone());
        self.deselect();

        if let Some(ref callback) = self.on_navigate {
            callback(&path);
        }
    }

    /// Navigate up one directory
    pub fn navigate_up(&mut self) {
        let current = self.current_path.get();
        if let Some(parent) = Self::parent_path(&current) {
            self.navigate_to(parent);
        }
    }

    /// Get parent path
    fn parent_path(path: &str) -> Option<String> {
        if path == "/" {
            return None;
        }
        
        let trimmed = path.trim_end_matches('/');
        if let Some(pos) = trimmed.rfind('/') {
            if pos == 0 {
                Some("/".to_string())
            } else {
                Some(trimmed[..pos].to_string())
            }
        } else {
            None
        }
    }

    /// Handle double click on entry
    pub fn handle_double_click(&mut self, path: &str) {
        if let Some(entry) = self.find_entry(path) {
            if entry.is_directory() {
                self.navigate_to(entry.path.clone());
            }

            if let Some(ref callback) = self.on_double_click {
                callback(&entry);
            }
        }
    }

    /// Find entry by path
    pub fn find_entry(&self, path: &str) -> Option<FileEntry> {
        self.entries.get().into_iter().find(|e| e.path == path)
    }

    /// Get visible entries (filtered by show_hidden)
    pub fn get_visible_entries(&self) -> Vec<FileEntry> {
        let entries = self.entries.get();
        if self.show_hidden {
            entries
        } else {
            entries.into_iter().filter(|e| !e.name.starts_with('.')).collect()
        }
    }

    /// Get entry count
    pub fn entry_count(&self) -> usize {
        self.get_visible_entries().len()
    }

    /// Check if has entries
    pub fn has_entries(&self) -> bool {
        !self.get_visible_entries().is_empty()
    }

    /// Get current path
    pub fn get_current_path(&self) -> String {
        self.current_path.get()
    }

    /// Check if entry is selected
    pub fn is_entry_selected(&self, path: &str) -> bool {
        self.selected_entry.get().as_deref() == Some(path)
    }

    /// Clear all entries
    pub fn clear(&mut self) {
        self.entries.set(Vec::new());
        self.deselect();
    }

    /// Build the file browser layout
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
            .map_err(|e| format!("Failed to create file browser node: {:?}", e))?;
        self.node_id = Some(node);

        Ok(node)
    }
}

impl Default for FileBrowser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filebrowser_starts_empty() {
        let browser = FileBrowser::new();
        assert_eq!(browser.entry_count(), 0);
        assert!(!browser.has_entries());
    }

    #[test]
    fn filebrowser_add_entries() {
        let browser = FileBrowser::new()
            .add_entry(FileEntry::directory("docs", "/home/docs"))
            .add_entry(FileEntry::file("file.txt", "/home/file.txt"));

        assert_eq!(browser.entry_count(), 2);
        assert!(browser.has_entries());
    }

    #[test]
    fn filebrowser_select_entry() {
        let mut browser = FileBrowser::new()
            .add_entry(FileEntry::file("file.txt", "/home/file.txt"));

        browser.select_entry("/home/file.txt");
        assert!(browser.is_entry_selected("/home/file.txt"));
    }

    #[test]
    fn filebrowser_deselect() {
        let mut browser = FileBrowser::new()
            .add_entry(FileEntry::file("file.txt", "/home/file.txt"));

        browser.select_entry("/home/file.txt");
        assert!(browser.is_entry_selected("/home/file.txt"));

        browser.deselect();
        assert!(!browser.is_entry_selected("/home/file.txt"));
    }

    #[test]
    fn filebrowser_navigate_to() {
        let mut browser = FileBrowser::new();
        
        browser.navigate_to("/home/user");
        assert_eq!(browser.get_current_path(), "/home/user");
    }

    #[test]
    fn filebrowser_navigate_up() {
        let mut browser = FileBrowser::new()
            .current_path("/home/user/documents");

        browser.navigate_up();
        assert_eq!(browser.get_current_path(), "/home/user");

        browser.navigate_up();
        assert_eq!(browser.get_current_path(), "/home");

        browser.navigate_up();
        assert_eq!(browser.get_current_path(), "/");
    }

    #[test]
    fn filebrowser_navigate_up_at_root() {
        let mut browser = FileBrowser::new()
            .current_path("/");

        browser.navigate_up();
        assert_eq!(browser.get_current_path(), "/"); // Should stay at root
    }

    #[test]
    fn filebrowser_handle_double_click_directory() {
        let mut browser = FileBrowser::new()
            .add_entry(FileEntry::directory("docs", "/home/docs"));

        browser.handle_double_click("/home/docs");
        assert_eq!(browser.get_current_path(), "/home/docs");
    }

    #[test]
    fn filebrowser_find_entry() {
        let browser = FileBrowser::new()
            .add_entry(FileEntry::file("file.txt", "/home/file.txt"));

        let entry = browser.find_entry("/home/file.txt");
        assert!(entry.is_some());
        assert_eq!(entry.unwrap().name, "file.txt");
    }

    #[test]
    fn filebrowser_hidden_files() {
        let browser = FileBrowser::new()
            .add_entry(FileEntry::file(".hidden", "/home/.hidden"))
            .add_entry(FileEntry::file("visible.txt", "/home/visible.txt"))
            .show_hidden(false);

        assert_eq!(browser.entry_count(), 1); // Only visible file
    }

    #[test]
    fn filebrowser_show_hidden_files() {
        let browser = FileBrowser::new()
            .add_entry(FileEntry::file(".hidden", "/home/.hidden"))
            .add_entry(FileEntry::file("visible.txt", "/home/visible.txt"))
            .show_hidden(true);

        assert_eq!(browser.entry_count(), 2); // Both files
    }

    #[test]
    fn filebrowser_clear() {
        let mut browser = FileBrowser::new()
            .add_entry(FileEntry::file("file.txt", "/home/file.txt"));

        assert_eq!(browser.entry_count(), 1);

        browser.clear();
        assert_eq!(browser.entry_count(), 0);
    }

    #[test]
    fn fileentry_is_directory() {
        let dir = FileEntry::directory("docs", "/home/docs");
        assert!(dir.is_directory());
        assert!(!dir.is_file());
    }

    #[test]
    fn fileentry_is_file() {
        let file = FileEntry::file("file.txt", "/home/file.txt");
        assert!(file.is_file());
        assert!(!file.is_directory());
    }

    #[test]
    fn fileentry_extension() {
        let file = FileEntry::file("document.pdf", "/home/document.pdf");
        assert_eq!(file.extension(), Some("pdf"));

        let no_ext = FileEntry::file("README", "/home/README");
        assert_eq!(no_ext.extension(), None);
    }

    #[test]
    fn fileentry_size_conversions() {
        let file = FileEntry::file("file.txt", "/home/file.txt")
            .with_size(2048);

        assert_eq!(file.size_kb(), Some(2.0));
        assert!((file.size_mb().unwrap() - 0.001953125).abs() < 0.0001);
    }

    #[test]
    fn fileentry_with_metadata() {
        let file = FileEntry::file("file.txt", "/home/file.txt")
            .with_size(1024)
            .with_modified("2025-11-22")
            .with_icon("📄");

        assert_eq!(file.size, Some(1024));
        assert_eq!(file.modified, Some("2025-11-22".to_string()));
        assert_eq!(file.icon, Some("📄".to_string()));
    }

    #[test]
    fn filebrowser_callbacks() {
        use std::sync::{Arc, Mutex};

        let selected = Arc::new(Mutex::new(String::new()));
        let selected_clone = selected.clone();

        let navigated = Arc::new(Mutex::new(String::new()));
        let navigated_clone = navigated.clone();

        let mut browser = FileBrowser::new()
            .add_entry(FileEntry::file("file.txt", "/home/file.txt"))
            .on_select(move |entry| {
                *selected_clone.lock().unwrap() = entry.name.clone();
            })
            .on_navigate(move |path| {
                *navigated_clone.lock().unwrap() = path.to_string();
            });

        browser.select_entry("/home/file.txt");
        assert_eq!(*selected.lock().unwrap(), "file.txt");

        browser.navigate_to("/home/user");
        assert_eq!(*navigated.lock().unwrap(), "/home/user");
    }

    #[test]
    fn filebrowser_builder_pattern() {
        let browser = FileBrowser::new()
            .current_path("/home/user")
            .show_hidden(true)
            .show_size(false)
            .show_modified(false)
            .item_height(50.0)
            .padding(20.0)
            .selected_color(255, 0, 0, 50)
            .directory_color(0, 0, 255, 255);

        assert_eq!(browser.get_current_path(), "/home/user");
        assert!(browser.show_hidden);
        assert!(!browser.show_size);
        assert!(!browser.show_modified);
        assert_eq!(browser.item_height, 50.0);
        assert_eq!(browser.padding, 20.0);
        assert_eq!(browser.selected_color, (255, 0, 0, 50));
        assert_eq!(browser.directory_color, (0, 0, 255, 255));
    }

    #[test]
    fn filebrowser_build_creates_node() {
        let mut engine = LayoutEngine::new();
        let mut browser = FileBrowser::new()
            .add_entry(FileEntry::file("file.txt", "/home/file.txt"));

        let result = browser.build(&mut engine);
        assert!(result.is_ok());
        assert!(browser.node_id.is_some());
    }
}
