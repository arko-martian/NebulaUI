// FileUpload Component - File upload with drag and drop
// Essential for file handling and uploads

use nebula_core::layout::{LayoutEngine, NodeId};
use nebula_core::signal::Signal;

/// Uploaded file information
#[derive(Debug, Clone, PartialEq)]
pub struct UploadedFile {
    pub name: String,
    pub size: usize,
    pub mime_type: String,
    pub data: Vec<u8>,
}

impl UploadedFile {
    /// Create a new uploaded file
    pub fn new(name: impl Into<String>, size: usize, mime_type: impl Into<String>, data: Vec<u8>) -> Self {
        Self {
            name: name.into(),
            size,
            mime_type: mime_type.into(),
            data,
        }
    }

    /// Get file size in KB
    pub fn size_kb(&self) -> f64 {
        self.size as f64 / 1024.0
    }

    /// Get file size in MB
    pub fn size_mb(&self) -> f64 {
        self.size as f64 / (1024.0 * 1024.0)
    }

    /// Check if file is an image
    pub fn is_image(&self) -> bool {
        self.mime_type.starts_with("image/")
    }

    /// Check if file is a video
    pub fn is_video(&self) -> bool {
        self.mime_type.starts_with("video/")
    }

    /// Check if file is a document
    pub fn is_document(&self) -> bool {
        matches!(
            self.mime_type.as_str(),
            "application/pdf" | "application/msword" | "application/vnd.openxmlformats-officedocument.wordprocessingml.document"
        )
    }
}

/// FileUpload component - file upload with drag and drop
/// 
/// # Example
/// ```
/// let mut upload = FileUpload::new()
///     .accept("image/*,application/pdf")
///     .max_size(5 * 1024 * 1024) // 5MB
///     .multiple(true)
///     .on_upload(|files| println!("Uploaded {} files", files.len()));
/// ```
pub struct FileUpload {
    pub node_id: Option<NodeId>,
    pub files: Signal<Vec<UploadedFile>>,
    pub is_dragging: Signal<bool>,
    pub accept: Option<String>,
    pub max_size: Option<usize>,
    pub max_files: Option<usize>,
    pub multiple: bool,
    pub disabled: bool,
    pub width: f32,
    pub height: f32,
    pub background_color: (u8, u8, u8, u8),
    pub drag_color: (u8, u8, u8, u8),
    pub border_color: (u8, u8, u8, u8),
    pub drag_border_color: (u8, u8, u8, u8),
    pub text_color: (u8, u8, u8, u8),
    pub icon: String,
    pub label: String,
    pub hint: String,
    pub on_upload: Option<Box<dyn Fn(&[UploadedFile])>>,
    pub on_error: Option<Box<dyn Fn(&str)>>,
    pub on_remove: Option<Box<dyn Fn(&str)>>,
}

impl FileUpload {
    /// Create a new FileUpload component
    pub fn new() -> Self {
        Self {
            node_id: None,
            files: Signal::new(Vec::new()),
            is_dragging: Signal::new(false),
            accept: None,
            max_size: None,
            max_files: None,
            multiple: false,
            disabled: false,
            width: 400.0,
            height: 200.0,
            background_color: (250, 250, 250, 255),
            drag_color: (240, 248, 255, 255), // Light blue
            border_color: (220, 220, 220, 255),
            drag_border_color: (59, 130, 246, 255), // Blue
            text_color: (100, 100, 100, 255),
            icon: "📁".to_string(),
            label: "Drop files here or click to upload".to_string(),
            hint: "".to_string(),
            on_upload: None,
            on_error: None,
            on_remove: None,
        }
    }

    /// Set accepted file types (MIME types)
    pub fn accept(mut self, accept: impl Into<String>) -> Self {
        self.accept = Some(accept.into());
        self
    }

    /// Set maximum file size in bytes
    pub fn max_size(mut self, size: usize) -> Self {
        self.max_size = Some(size);
        self
    }

    /// Set maximum number of files
    pub fn max_files(mut self, count: usize) -> Self {
        self.max_files = Some(count);
        self
    }

    /// Allow multiple file uploads
    pub fn multiple(mut self, multiple: bool) -> Self {
        self.multiple = multiple;
        self
    }

    /// Set disabled state
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
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

    /// Set background color
    pub fn background_color(mut self, r: u8, g: u8, b: u8, a: u8) -> Self {
        self.background_color = (r, g, b, a);
        self
    }

    /// Set drag color
    pub fn drag_color(mut self, r: u8, g: u8, b: u8, a: u8) -> Self {
        self.drag_color = (r, g, b, a);
        self
    }

    /// Set icon
    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = icon.into();
        self
    }

    /// Set label
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = label.into();
        self
    }

    /// Set hint text
    pub fn hint(mut self, hint: impl Into<String>) -> Self {
        self.hint = hint.into();
        self
    }

    /// Set the upload callback
    pub fn on_upload<F>(mut self, callback: F) -> Self
    where
        F: Fn(&[UploadedFile]) + 'static,
    {
        self.on_upload = Some(Box::new(callback));
        self
    }

    /// Set the error callback
    pub fn on_error<F>(mut self, callback: F) -> Self
    where
        F: Fn(&str) + 'static,
    {
        self.on_error = Some(Box::new(callback));
        self
    }

    /// Set the remove callback
    pub fn on_remove<F>(mut self, callback: F) -> Self
    where
        F: Fn(&str) + 'static,
    {
        self.on_remove = Some(Box::new(callback));
        self
    }

    /// Add files
    pub fn add_files(&mut self, new_files: Vec<UploadedFile>) {
        if self.disabled {
            return;
        }

        let mut files = self.files.get();
        let mut valid_files = Vec::new();

        for file in new_files {
            // Check max files
            if let Some(max) = self.max_files {
                if files.len() + valid_files.len() >= max {
                    if let Some(ref callback) = self.on_error {
                        callback(&format!("Maximum {} files allowed", max));
                    }
                    break;
                }
            }

            // Check file size
            if let Some(max_size) = self.max_size {
                if file.size > max_size {
                    if let Some(ref callback) = self.on_error {
                        callback(&format!("File {} exceeds maximum size", file.name));
                    }
                    continue;
                }
            }

            // Check file type
            if let Some(ref accept) = self.accept {
                if !self.is_file_accepted(&file.mime_type, accept) {
                    if let Some(ref callback) = self.on_error {
                        callback(&format!("File type {} not accepted", file.mime_type));
                    }
                    continue;
                }
            }

            valid_files.push(file);
        }

        if !valid_files.is_empty() {
            if !self.multiple {
                files.clear();
            }
            files.extend(valid_files.clone());
            self.files.set(files);

            if let Some(ref callback) = self.on_upload {
                callback(&valid_files);
            }
        }
    }

    /// Check if file type is accepted
    fn is_file_accepted(&self, mime_type: &str, accept: &str) -> bool {
        for pattern in accept.split(',') {
            let pattern = pattern.trim();
            if pattern == "*/*" || pattern == mime_type {
                return true;
            }
            if pattern.ends_with("/*") {
                let prefix = &pattern[..pattern.len() - 2];
                if mime_type.starts_with(prefix) {
                    return true;
                }
            }
        }
        false
    }

    /// Remove a file by name
    pub fn remove_file(&mut self, name: &str) {
        let mut files = self.files.get();
        if let Some(pos) = files.iter().position(|f| f.name == name) {
            files.remove(pos);
            self.files.set(files);

            if let Some(ref callback) = self.on_remove {
                callback(name);
            }
        }
    }

    /// Clear all files
    pub fn clear(&mut self) {
        self.files.set(Vec::new());
    }

    /// Get uploaded files
    pub fn get_files(&self) -> Vec<UploadedFile> {
        self.files.get()
    }

    /// Get file count
    pub fn file_count(&self) -> usize {
        self.files.get().len()
    }

    /// Check if has files
    pub fn has_files(&self) -> bool {
        !self.files.get().is_empty()
    }

    /// Check if dragging
    pub fn is_file_dragging(&self) -> bool {
        self.is_dragging.get()
    }

    /// Set dragging state
    pub fn set_dragging(&mut self, dragging: bool) {
        self.is_dragging.set(dragging);
    }

    /// Get total size of all files
    pub fn total_size(&self) -> usize {
        self.files.get().iter().map(|f| f.size).sum()
    }

    /// Get total size in MB
    pub fn total_size_mb(&self) -> f64 {
        self.total_size() as f64 / (1024.0 * 1024.0)
    }

    /// Build the file upload layout
    pub fn build(&mut self, engine: &mut LayoutEngine) -> Result<NodeId, String> {
        let style = taffy::style::Style {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Length(self.width),
                height: taffy::style::Dimension::Length(self.height),
            },
            display: taffy::style::Display::Flex,
            flex_direction: taffy::style::FlexDirection::Column,
            justify_content: Some(taffy::style::JustifyContent::Center),
            align_items: Some(taffy::style::AlignItems::Center),
            ..Default::default()
        };

        let node = engine
            .new_leaf(style)
            .map_err(|e| format!("Failed to create file upload node: {:?}", e))?;
        self.node_id = Some(node);

        Ok(node)
    }
}

impl Default for FileUpload {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fileupload_starts_empty() {
        let upload = FileUpload::new();
        assert_eq!(upload.file_count(), 0);
        assert!(!upload.has_files());
    }

    #[test]
    fn fileupload_add_files() {
        let mut upload = FileUpload::new();
        let file = UploadedFile::new("test.txt", 1024, "text/plain", vec![]);
        
        upload.add_files(vec![file]);
        assert_eq!(upload.file_count(), 1);
        assert!(upload.has_files());
    }

    #[test]
    fn fileupload_multiple_files() {
        let mut upload = FileUpload::new().multiple(true);
        let file1 = UploadedFile::new("test1.txt", 1024, "text/plain", vec![]);
        let file2 = UploadedFile::new("test2.txt", 2048, "text/plain", vec![]);
        
        upload.add_files(vec![file1, file2]);
        assert_eq!(upload.file_count(), 2);
    }

    #[test]
    fn fileupload_single_file_replaces() {
        let mut upload = FileUpload::new().multiple(false);
        let file1 = UploadedFile::new("test1.txt", 1024, "text/plain", vec![]);
        let file2 = UploadedFile::new("test2.txt", 2048, "text/plain", vec![]);
        
        upload.add_files(vec![file1]);
        upload.add_files(vec![file2]);
        assert_eq!(upload.file_count(), 1);
        assert_eq!(upload.get_files()[0].name, "test2.txt");
    }

    #[test]
    fn fileupload_max_files() {
        let mut upload = FileUpload::new()
            .multiple(true)
            .max_files(2);
        
        let files = vec![
            UploadedFile::new("test1.txt", 1024, "text/plain", vec![]),
            UploadedFile::new("test2.txt", 1024, "text/plain", vec![]),
            UploadedFile::new("test3.txt", 1024, "text/plain", vec![]),
        ];
        
        upload.add_files(files);
        assert_eq!(upload.file_count(), 2); // Should only add 2
    }

    #[test]
    fn fileupload_max_size() {
        let mut upload = FileUpload::new()
            .max_size(1024);
        
        let file = UploadedFile::new("large.txt", 2048, "text/plain", vec![]);
        upload.add_files(vec![file]);
        
        assert_eq!(upload.file_count(), 0); // Should reject
    }

    #[test]
    fn fileupload_accept_filter() {
        let mut upload = FileUpload::new()
            .accept("image/*");
        
        let image = UploadedFile::new("photo.jpg", 1024, "image/jpeg", vec![]);
        let text = UploadedFile::new("doc.txt", 1024, "text/plain", vec![]);
        
        upload.add_files(vec![image, text]);
        assert_eq!(upload.file_count(), 1); // Only image accepted
    }

    #[test]
    fn fileupload_remove_file() {
        let mut upload = FileUpload::new();
        let file = UploadedFile::new("test.txt", 1024, "text/plain", vec![]);
        
        upload.add_files(vec![file]);
        assert_eq!(upload.file_count(), 1);
        
        upload.remove_file("test.txt");
        assert_eq!(upload.file_count(), 0);
    }

    #[test]
    fn fileupload_clear() {
        let mut upload = FileUpload::new().multiple(true);
        let files = vec![
            UploadedFile::new("test1.txt", 1024, "text/plain", vec![]),
            UploadedFile::new("test2.txt", 1024, "text/plain", vec![]),
        ];
        
        upload.add_files(files);
        assert_eq!(upload.file_count(), 2);
        
        upload.clear();
        assert_eq!(upload.file_count(), 0);
    }

    #[test]
    fn fileupload_disabled() {
        let mut upload = FileUpload::new().disabled(true);
        let file = UploadedFile::new("test.txt", 1024, "text/plain", vec![]);
        
        upload.add_files(vec![file]);
        assert_eq!(upload.file_count(), 0); // Should not add
    }

    #[test]
    fn fileupload_dragging_state() {
        let mut upload = FileUpload::new();
        assert!(!upload.is_file_dragging());
        
        upload.set_dragging(true);
        assert!(upload.is_file_dragging());
        
        upload.set_dragging(false);
        assert!(!upload.is_file_dragging());
    }

    #[test]
    fn fileupload_total_size() {
        let mut upload = FileUpload::new().multiple(true);
        let files = vec![
            UploadedFile::new("test1.txt", 1024, "text/plain", vec![]),
            UploadedFile::new("test2.txt", 2048, "text/plain", vec![]),
        ];
        
        upload.add_files(files);
        assert_eq!(upload.total_size(), 3072);
    }

    #[test]
    fn uploaded_file_size_conversions() {
        let file = UploadedFile::new("test.txt", 2048, "text/plain", vec![]);
        assert_eq!(file.size_kb(), 2.0);
        assert!((file.size_mb() - 0.001953125).abs() < 0.0001);
    }

    #[test]
    fn uploaded_file_type_checks() {
        let image = UploadedFile::new("photo.jpg", 1024, "image/jpeg", vec![]);
        let video = UploadedFile::new("video.mp4", 1024, "video/mp4", vec![]);
        let pdf = UploadedFile::new("doc.pdf", 1024, "application/pdf", vec![]);
        
        assert!(image.is_image());
        assert!(!image.is_video());
        
        assert!(video.is_video());
        assert!(!video.is_image());
        
        assert!(pdf.is_document());
        assert!(!pdf.is_image());
    }

    #[test]
    fn fileupload_callbacks() {
        use std::sync::{Arc, Mutex};

        let uploaded = Arc::new(Mutex::new(0));
        let uploaded_clone = uploaded.clone();

        let removed = Arc::new(Mutex::new(String::new()));
        let removed_clone = removed.clone();

        let mut upload = FileUpload::new()
            .on_upload(move |files| {
                *uploaded_clone.lock().unwrap() = files.len();
            })
            .on_remove(move |name| {
                *removed_clone.lock().unwrap() = name.to_string();
            });

        let file = UploadedFile::new("test.txt", 1024, "text/plain", vec![]);
        upload.add_files(vec![file]);
        assert_eq!(*uploaded.lock().unwrap(), 1);

        upload.remove_file("test.txt");
        assert_eq!(*removed.lock().unwrap(), "test.txt");
    }

    #[test]
    fn fileupload_builder_pattern() {
        let upload = FileUpload::new()
            .accept("image/*")
            .max_size(5 * 1024 * 1024)
            .max_files(10)
            .multiple(true)
            .disabled(false)
            .width(500.0)
            .height(300.0)
            .background_color(255, 255, 255, 255)
            .drag_color(200, 200, 255, 255)
            .icon("📤")
            .label("Upload files")
            .hint("Max 5MB");

        assert_eq!(upload.accept, Some("image/*".to_string()));
        assert_eq!(upload.max_size, Some(5 * 1024 * 1024));
        assert_eq!(upload.max_files, Some(10));
        assert!(upload.multiple);
        assert!(!upload.disabled);
        assert_eq!(upload.width, 500.0);
        assert_eq!(upload.height, 300.0);
        assert_eq!(upload.icon, "📤");
        assert_eq!(upload.label, "Upload files");
        assert_eq!(upload.hint, "Max 5MB");
    }

    #[test]
    fn fileupload_build_creates_node() {
        let mut engine = LayoutEngine::new();
        let mut upload = FileUpload::new();

        let result = upload.build(&mut engine);
        assert!(result.is_ok());
        assert!(upload.node_id.is_some());
    }
}
