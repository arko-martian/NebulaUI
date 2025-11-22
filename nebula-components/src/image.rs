use nebula_core::{LayoutEngine, NodeId, Layout};
use taffy::prelude::*;
use tracing::{info, warn, error};
use std::path::PathBuf;
use image::DynamicImage;
use crate::image_cache::ImageCache;
use std::cell::RefCell;

thread_local! {
    /// Thread-local image cache for blazing fast performance! 🚀
    static IMAGE_CACHE: RefCell<ImageCache> = RefCell::new(ImageCache::new());
}

/// Image - Display images 🖼️
/// 
/// Essential for photos, icons, logos, and more!
/// - Load from file paths (PNG, JPEG, GIF, BMP, etc.)
/// - REAL image decoding with `image` crate!
/// - Lazy loading support
/// - Error handling
/// - Fit modes (fill, contain, cover)
/// - Actual dimensions after loading
/// 
/// Works on old hardware with CPU rendering!
#[derive(Clone)]
pub struct Image {
    /// Layout node ID
    pub node_id: Option<NodeId>,
    /// Image source
    pub source: ImageSource,
    /// Image state
    pub state: ImageState,
    /// Decoded image data (after loading)
    pub decoded_image: Option<DynamicImage>,
    /// Actual image dimensions (after loading)
    pub actual_dimensions: Option<(u32, u32)>,
    /// Fit mode (how image scales)
    pub fit: ImageFit,
    /// Width (None = auto)
    pub width: Option<f32>,
    /// Height (None = auto)
    pub height: Option<f32>,
    /// Position
    pub position: (f32, f32),
}

/// Image source
#[derive(Debug, Clone, PartialEq)]
pub enum ImageSource {
    /// Load from file path
    File(PathBuf),
    /// Load from URL (future)
    Url(String),
    /// Load from memory (bytes)
    Memory(Vec<u8>),
    /// Placeholder (no image)
    None,
}

/// Image loading state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageState {
    /// Not loaded yet
    NotLoaded,
    /// Currently loading
    Loading,
    /// Successfully loaded
    Loaded,
    /// Failed to load
    Error,
}

/// How the image should fit in its container
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageFit {
    /// Fill the container (may distort)
    Fill,
    /// Contain within container (maintain aspect ratio)
    Contain,
    /// Cover the container (maintain aspect ratio, may crop)
    Cover,
    /// Original size (no scaling)
    None,
}

impl Image {
    /// Create a new image (placeholder)
    pub fn new() -> Self {
        info!("🖼️ Creating Image (placeholder)");
        Self {
            node_id: None,
            source: ImageSource::None,
            state: ImageState::NotLoaded,
            decoded_image: None,
            actual_dimensions: None,
            fit: ImageFit::Contain,
            width: None,
            height: None,
            position: (0.0, 0.0),
        }
    }

    /// Create an image from a file path
    pub fn from_file(path: impl Into<PathBuf>) -> Self {
        let path = path.into();
        info!("🖼️ Creating Image from file: {:?}", path);
        Self {
            node_id: None,
            source: ImageSource::File(path),
            state: ImageState::NotLoaded,
            decoded_image: None,
            actual_dimensions: None,
            fit: ImageFit::Contain,
            width: None,
            height: None,
            position: (0.0, 0.0),
        }
    }

    /// Create an image from a URL (for future implementation)
    pub fn from_url(url: impl Into<String>) -> Self {
        let url = url.into();
        info!("🖼️ Creating Image from URL: {}", url);
        Self {
            node_id: None,
            source: ImageSource::Url(url),
            state: ImageState::NotLoaded,
            decoded_image: None,
            actual_dimensions: None,
            fit: ImageFit::Contain,
            width: None,
            height: None,
            position: (0.0, 0.0),
        }
    }

    /// Create an image from memory (bytes)
    pub fn from_memory(bytes: Vec<u8>) -> Self {
        info!("🖼️ Creating Image from memory ({} bytes)", bytes.len());
        Self {
            node_id: None,
            source: ImageSource::Memory(bytes),
            state: ImageState::NotLoaded,
            decoded_image: None,
            actual_dimensions: None,
            fit: ImageFit::Contain,
            width: None,
            height: None,
            position: (0.0, 0.0),
        }
    }

    /// Set fit mode
    pub fn fit(mut self, fit: ImageFit) -> Self {
        self.fit = fit;
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

    /// Set position
    pub fn position(mut self, x: f32, y: f32) -> Self {
        self.position = (x, y);
        self
    }

    /// Load the image - NOW WITH CACHING! 🚀🎨
    /// Supports PNG, JPEG, GIF, BMP, ICO, TIFF, WebP, and more!
    /// Uses a thread-local cache to avoid reloading the same image!
    pub fn load(&mut self) -> Result<(), String> {
        match &self.source {
            ImageSource::None => {
                warn!("Cannot load image: no source");
                self.state = ImageState::Error;
                Err("No image source".to_string())
            }
            ImageSource::File(path) => {
                info!("🖼️ Loading image from file: {:?}", path);
                self.state = ImageState::Loading;
                
                // CHECK CACHE FIRST! 🚀
                let cached = IMAGE_CACHE.with(|cache| {
                    cache.borrow().get_file(path).map(|cached| {
                        info!("🎯 Cache HIT! Using cached image: {:?}", path);
                        cached.image.clone()
                    })
                });
                
                if let Some(img) = cached {
                    // CACHE HIT! 🎉
                    let (width, height) = (img.width(), img.height());
                    self.decoded_image = Some(img);
                    self.actual_dimensions = Some((width, height));
                    self.state = ImageState::Loaded;
                    return Ok(());
                }
                
                // CACHE MISS - Load and decode! 🎨
                info!("💾 Cache MISS - Loading from disk: {:?}", path);
                match image::open(path) {
                    Ok(img) => {
                        let (width, height) = (img.width(), img.height());
                        info!("✅ Image loaded successfully! {}x{} pixels", width, height);
                        
                        // CACHE IT! 🗄️
                        IMAGE_CACHE.with(|cache| {
                            cache.borrow_mut().cache_file(path.clone(), img.clone());
                        });
                        
                        self.decoded_image = Some(img);
                        self.actual_dimensions = Some((width, height));
                        self.state = ImageState::Loaded;
                        Ok(())
                    }
                    Err(e) => {
                        error!("❌ Failed to load image: {}", e);
                        self.state = ImageState::Error;
                        Err(format!("Failed to load image: {}", e))
                    }
                }
            }
            ImageSource::Url(url) => {
                info!("🖼️ Loading image from URL: {}", url);
                self.state = ImageState::Loading;
                
                // CHECK CACHE FIRST! 🚀
                let cached = IMAGE_CACHE.with(|cache| {
                    cache.borrow().get_url(url).map(|cached| {
                        info!("🎯 Cache HIT! Using cached image: {}", url);
                        cached.image.clone()
                    })
                });
                
                if let Some(img) = cached {
                    // CACHE HIT! 🎉
                    let (width, height) = (img.width(), img.height());
                    self.decoded_image = Some(img);
                    self.actual_dimensions = Some((width, height));
                    self.state = ImageState::Loaded;
                    return Ok(());
                }
                
                // URL loading not implemented yet
                self.state = ImageState::Error;
                Err("URL loading not implemented yet".to_string())
            }
            ImageSource::Memory(bytes) => {
                info!("🖼️ Loading image from memory ({} bytes)", bytes.len());
                self.state = ImageState::Loading;
                
                // Memory images aren't cached (they're already in memory!)
                // REAL IMAGE DECODING FROM MEMORY! 🎨
                match image::load_from_memory(bytes) {
                    Ok(img) => {
                        let (width, height) = (img.width(), img.height());
                        info!("✅ Image loaded from memory! {}x{} pixels", width, height);
                        
                        self.decoded_image = Some(img);
                        self.actual_dimensions = Some((width, height));
                        self.state = ImageState::Loaded;
                        Ok(())
                    }
                    Err(e) => {
                        error!("❌ Failed to decode image from memory: {}", e);
                        self.state = ImageState::Error;
                        Err(format!("Failed to decode image: {}", e))
                    }
                }
            }
        }
    }

    /// Get image state
    pub fn get_state(&self) -> ImageState {
        self.state
    }

    /// Check if loaded
    pub fn is_loaded(&self) -> bool {
        self.state == ImageState::Loaded
    }

    /// Check if loading
    pub fn is_loading(&self) -> bool {
        self.state == ImageState::Loading
    }

    /// Check if error
    pub fn is_error(&self) -> bool {
        self.state == ImageState::Error
    }

    /// Get source
    pub fn get_source(&self) -> &ImageSource {
        &self.source
    }

    /// Get actual image dimensions (after loading)
    pub fn get_dimensions(&self) -> Option<(u32, u32)> {
        self.actual_dimensions
    }

    /// Get decoded image data
    pub fn get_decoded_image(&self) -> Option<&DynamicImage> {
        self.decoded_image.as_ref()
    }

    /// Get pixel data as RGBA bytes (after loading)
    pub fn get_rgba_bytes(&self) -> Option<Vec<u8>> {
        self.decoded_image.as_ref().map(|img| {
            img.to_rgba8().into_raw()
        })
    }

    /// Clear the global image cache
    /// Useful for freeing memory when needed
    pub fn clear_cache() {
        IMAGE_CACHE.with(|cache| {
            cache.borrow_mut().clear();
        });
        info!("🧹 Image cache cleared");
    }

    /// Get cache statistics
    pub fn cache_stats() -> (usize, usize) {
        IMAGE_CACHE.with(|cache| {
            let cache = cache.borrow();
            (cache.count(), cache.total_size())
        })
    }

    /// Check if an image is cached
    pub fn is_cached(source: &ImageSource) -> bool {
        match source {
            ImageSource::File(path) => {
                IMAGE_CACHE.with(|cache| cache.borrow().contains_file(path))
            }
            ImageSource::Url(url) => {
                IMAGE_CACHE.with(|cache| cache.borrow().contains_url(url))
            }
            _ => false,
        }
    }

    /// Build the layout node
    pub fn build(&mut self, engine: &mut LayoutEngine) -> Result<NodeId, String> {
        let style = Style {
            size: Size {
                width: self.width.map(Dimension::Length).unwrap_or(Dimension::Auto),
                height: self
                    .height
                    .map(Dimension::Length)
                    .unwrap_or(Dimension::Auto),
            },
            ..Default::default()
        };

        let node = engine
            .new_leaf(style)
            .map_err(|e| format!("Failed to create Image: {:?}", e))?;

        self.node_id = Some(node);
        info!(
            "✅ Image built ({}x{})",
            self.width.map(|w| w.to_string()).unwrap_or_else(|| "auto".to_string()),
            self.height.map(|h| h.to_string()).unwrap_or_else(|| "auto".to_string())
        );
        Ok(node)
    }

    /// Get the layout
    pub fn get_layout(&self, engine: &LayoutEngine) -> Option<Layout> {
        self.node_id.and_then(|id| engine.get_layout(id).ok())
    }

    /// Get bounds (x, y, width, height)
    pub fn bounds(&self) -> (f32, f32, Option<f32>, Option<f32>) {
        (self.position.0, self.position.1, self.width, self.height)
    }
}

impl Default for Image {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn image_creation() {
        let image = Image::new();
        assert_eq!(image.source, ImageSource::None);
        assert_eq!(image.state, ImageState::NotLoaded);
        assert_eq!(image.fit, ImageFit::Contain);
        assert_eq!(image.width, None);
        assert_eq!(image.height, None);
    }

    #[test]
    fn image_from_file() {
        let image = Image::from_file("test.png");
        match image.source {
            ImageSource::File(path) => {
                assert_eq!(path.to_str().unwrap(), "test.png");
            }
            _ => panic!("Expected File source"),
        }
    }

    #[test]
    fn image_from_url() {
        let image = Image::from_url("https://example.com/image.png");
        match image.source {
            ImageSource::Url(url) => {
                assert_eq!(url, "https://example.com/image.png");
            }
            _ => panic!("Expected Url source"),
        }
    }

    #[test]
    fn image_from_memory() {
        let bytes = vec![1, 2, 3, 4, 5];
        let image = Image::from_memory(bytes.clone());
        match image.source {
            ImageSource::Memory(b) => {
                assert_eq!(b, bytes);
            }
            _ => panic!("Expected Memory source"),
        }
    }

    #[test]
    fn image_builder_pattern() {
        let image = Image::new()
            .fit(ImageFit::Cover)
            .width(300.0)
            .height(200.0)
            .position(10.0, 20.0);

        assert_eq!(image.fit, ImageFit::Cover);
        assert_eq!(image.width, Some(300.0));
        assert_eq!(image.height, Some(200.0));
        assert_eq!(image.position, (10.0, 20.0));
    }

    #[test]
    fn image_size() {
        let image = Image::new().size(400.0, 300.0);
        assert_eq!(image.width, Some(400.0));
        assert_eq!(image.height, Some(300.0));
    }

    #[test]
    fn image_state_checks() {
        let image = Image::new();
        assert!(!image.is_loaded());
        assert!(!image.is_loading());
        assert!(!image.is_error());
    }

    #[test]
    fn image_load_no_source() {
        let mut image = Image::new();
        let result = image.load();
        assert!(result.is_err());
        assert!(image.is_error());
    }

    #[test]
    fn image_load_memory() {
        // Create a simple 1x1 PNG image in memory
        // This is a minimal valid PNG file
        let png_bytes = vec![
            0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, // PNG signature
            // For testing, we'll just test that invalid data fails gracefully
        ];
        let mut image = Image::from_memory(png_bytes);
        let result = image.load();
        // Invalid PNG data should fail
        assert!(result.is_err());
        assert!(image.is_error());
    }

    #[test]
    fn image_load_url() {
        let mut image = Image::from_url("https://example.com/test.png");
        let result = image.load();
        // URL loading not implemented yet, should fail gracefully
        assert!(result.is_err());
        assert!(image.is_error());
    }

    #[test]
    fn image_fit_modes() {
        assert_eq!(ImageFit::Fill, ImageFit::Fill);
        assert_ne!(ImageFit::Fill, ImageFit::Contain);
        assert_ne!(ImageFit::Contain, ImageFit::Cover);
        assert_ne!(ImageFit::Cover, ImageFit::None);
    }

    #[test]
    fn image_state_transitions() {
        assert_eq!(ImageState::NotLoaded, ImageState::NotLoaded);
        assert_ne!(ImageState::NotLoaded, ImageState::Loading);
        assert_ne!(ImageState::Loading, ImageState::Loaded);
        assert_ne!(ImageState::Loaded, ImageState::Error);
    }

    #[test]
    fn image_bounds() {
        let image = Image::new()
            .position(10.0, 20.0)
            .size(300.0, 200.0);

        let (x, y, w, h) = image.bounds();
        assert_eq!(x, 10.0);
        assert_eq!(y, 20.0);
        assert_eq!(w, Some(300.0));
        assert_eq!(h, Some(200.0));
    }

    #[test]
    fn image_build() {
        let mut engine = LayoutEngine::new();
        let mut image = Image::new().size(200.0, 150.0);

        let node = image.build(&mut engine);
        assert!(node.is_ok());
        assert!(image.node_id.is_some());
    }

    #[test]
    fn image_layout() {
        let mut engine = LayoutEngine::new();
        let mut image = Image::new().size(300.0, 200.0);

        let node = image.build(&mut engine).unwrap();

        let available = Size {
            width: AvailableSpace::Definite(400.0),
            height: AvailableSpace::Definite(400.0),
        };
        engine.compute_layout(node, available).unwrap();

        let layout = image.get_layout(&engine);
        assert!(layout.is_some());

        let layout = layout.unwrap();
        assert_eq!(layout.size.width, 300.0);
        assert_eq!(layout.size.height, 200.0);
    }

    #[test]
    fn image_default() {
        let image = Image::default();
        assert_eq!(image.source, ImageSource::None);
    }

    #[test]
    fn image_clone() {
        let image1 = Image::from_file("test.png").size(100.0, 100.0);
        let image2 = image1.clone();

        assert_eq!(image1.width, image2.width);
        assert_eq!(image1.height, image2.height);
    }
}

    #[test]
    fn image_dimensions_after_load() {
        let image = Image::new();
        assert_eq!(image.get_dimensions(), None);
        
        // After loading, dimensions would be available
        // (tested with real image files in integration tests)
    }

    #[test]
    fn image_decoded_image_initially_none() {
        let image = Image::new();
        assert!(image.get_decoded_image().is_none());
    }

    #[test]
    fn image_rgba_bytes_initially_none() {
        let image = Image::new();
        assert!(image.get_rgba_bytes().is_none());
    }

    #[test]
    fn cache_clear() {
        Image::clear_cache();
        let (count, size) = Image::cache_stats();
        assert_eq!(count, 0);
        assert_eq!(size, 0);
    }

    #[test]
    fn cache_stats() {
        Image::clear_cache();
        let (count, _size) = Image::cache_stats();
        assert_eq!(count, 0);
    }

    #[test]
    fn is_cached_none() {
        let source = ImageSource::None;
        assert!(!Image::is_cached(&source));
    }

    #[test]
    fn is_cached_file() {
        Image::clear_cache();
        let path = PathBuf::from("nonexistent.png");
        let source = ImageSource::File(path);
        assert!(!Image::is_cached(&source));
    }

    #[test]
    fn is_cached_url() {
        Image::clear_cache();
        let source = ImageSource::Url("https://example.com/test.png".to_string());
        assert!(!Image::is_cached(&source));
    }

    #[test]
    fn is_cached_memory() {
        let source = ImageSource::Memory(vec![1, 2, 3]);
        // Memory images are never cached
        assert!(!Image::is_cached(&source));
    }
