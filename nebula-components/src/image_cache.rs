use image::DynamicImage;
use std::collections::HashMap;
use std::path::PathBuf;
use tracing::info;

/// Image cache - Prevents reloading the same image! 🚀
/// 
/// This is ESSENTIAL for performance:
/// - Load once, use forever!
/// - Saves memory
/// - Saves CPU time
/// - Saves disk I/O
/// 
/// Works great on old hardware!
pub struct ImageCache {
    /// Cached images by path
    file_cache: HashMap<PathBuf, CachedImage>,
    /// Cached images by URL
    url_cache: HashMap<String, CachedImage>,
    /// Total cache size in bytes
    total_size: usize,
    /// Maximum cache size (None = unlimited)
    max_size: Option<usize>,
}

/// A cached image with metadata
#[derive(Clone)]
pub struct CachedImage {
    /// The decoded image
    pub image: DynamicImage,
    /// Image dimensions
    pub dimensions: (u32, u32),
    /// Approximate size in bytes
    pub size_bytes: usize,
}

impl ImageCache {
    /// Create a new image cache
    pub fn new() -> Self {
        info!("🗄️ Creating ImageCache");
        Self {
            file_cache: HashMap::new(),
            url_cache: HashMap::new(),
            total_size: 0,
            max_size: None,
        }
    }

    /// Create a cache with maximum size (in bytes)
    pub fn with_max_size(max_size: usize) -> Self {
        info!("🗄️ Creating ImageCache (max: {} bytes)", max_size);
        Self {
            file_cache: HashMap::new(),
            url_cache: HashMap::new(),
            total_size: 0,
            max_size: Some(max_size),
        }
    }

    /// Get image from file cache
    pub fn get_file(&self, path: &PathBuf) -> Option<&CachedImage> {
        self.file_cache.get(path)
    }

    /// Get image from URL cache
    pub fn get_url(&self, url: &str) -> Option<&CachedImage> {
        self.url_cache.get(url)
    }

    /// Cache an image from file
    pub fn cache_file(&mut self, path: PathBuf, image: DynamicImage) {
        let dimensions = (image.width(), image.height());
        let size_bytes = (dimensions.0 * dimensions.1 * 4) as usize; // RGBA

        // Check if we need to evict
        if let Some(max_size) = self.max_size {
            if self.total_size + size_bytes > max_size {
                self.evict_oldest();
            }
        }

        let cached = CachedImage {
            image,
            dimensions,
            size_bytes,
        };

        self.file_cache.insert(path.clone(), cached);
        self.total_size += size_bytes;

        info!(
            "🗄️ Cached image: {:?} ({}x{}, {} bytes)",
            path, dimensions.0, dimensions.1, size_bytes
        );
    }

    /// Cache an image from URL
    pub fn cache_url(&mut self, url: String, image: DynamicImage) {
        let dimensions = (image.width(), image.height());
        let size_bytes = (dimensions.0 * dimensions.1 * 4) as usize; // RGBA

        // Check if we need to evict
        if let Some(max_size) = self.max_size {
            if self.total_size + size_bytes > max_size {
                self.evict_oldest();
            }
        }

        let cached = CachedImage {
            image,
            dimensions,
            size_bytes,
        };

        self.url_cache.insert(url.clone(), cached);
        self.total_size += size_bytes;

        info!(
            "🗄️ Cached image: {} ({}x{}, {} bytes)",
            url, dimensions.0, dimensions.1, size_bytes
        );
    }

    /// Evict oldest entry (simple FIFO for now)
    fn evict_oldest(&mut self) {
        // For simplicity, just clear the first entry
        // In a real implementation, we'd use LRU
        if let Some((path, cached)) = self.file_cache.iter().next() {
            let path = path.clone();
            let size = cached.size_bytes;
            self.file_cache.remove(&path);
            self.total_size -= size;
            info!("🗑️ Evicted image from cache: {:?}", path);
        }
    }

    /// Clear all cached images
    pub fn clear(&mut self) {
        self.file_cache.clear();
        self.url_cache.clear();
        self.total_size = 0;
        info!("🧹 Image cache cleared");
    }

    /// Get number of cached images
    pub fn count(&self) -> usize {
        self.file_cache.len() + self.url_cache.len()
    }

    /// Get total cache size in bytes
    pub fn total_size(&self) -> usize {
        self.total_size
    }

    /// Check if cache contains a file
    pub fn contains_file(&self, path: &PathBuf) -> bool {
        self.file_cache.contains_key(path)
    }

    /// Check if cache contains a URL
    pub fn contains_url(&self, url: &str) -> bool {
        self.url_cache.contains_key(url)
    }
}

impl Default for ImageCache {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{RgbaImage, Rgba};

    fn create_test_image(width: u32, height: u32) -> DynamicImage {
        let img = RgbaImage::from_pixel(width, height, Rgba([255, 0, 0, 255]));
        DynamicImage::ImageRgba8(img)
    }

    #[test]
    fn cache_creation() {
        let cache = ImageCache::new();
        assert_eq!(cache.count(), 0);
        assert_eq!(cache.total_size(), 0);
    }

    #[test]
    fn cache_with_max_size() {
        let cache = ImageCache::with_max_size(1024 * 1024); // 1 MB
        assert_eq!(cache.max_size, Some(1024 * 1024));
    }

    #[test]
    fn cache_file() {
        let mut cache = ImageCache::new();
        let img = create_test_image(10, 10);
        let path = PathBuf::from("test.png");

        cache.cache_file(path.clone(), img);

        assert_eq!(cache.count(), 1);
        assert!(cache.contains_file(&path));
        assert!(cache.get_file(&path).is_some());
    }

    #[test]
    fn cache_url() {
        let mut cache = ImageCache::new();
        let img = create_test_image(10, 10);
        let url = "https://example.com/test.png".to_string();

        cache.cache_url(url.clone(), img);

        assert_eq!(cache.count(), 1);
        assert!(cache.contains_url(&url));
        assert!(cache.get_url(&url).is_some());
    }

    #[test]
    fn cache_size_tracking() {
        let mut cache = ImageCache::new();
        let img = create_test_image(10, 10);
        let path = PathBuf::from("test.png");

        cache.cache_file(path, img);

        // 10x10 RGBA = 400 bytes
        assert_eq!(cache.total_size(), 400);
    }

    #[test]
    fn cache_clear() {
        let mut cache = ImageCache::new();
        let img = create_test_image(10, 10);

        cache.cache_file(PathBuf::from("test1.png"), img.clone());
        cache.cache_file(PathBuf::from("test2.png"), img);

        assert_eq!(cache.count(), 2);

        cache.clear();

        assert_eq!(cache.count(), 0);
        assert_eq!(cache.total_size(), 0);
    }

    #[test]
    fn cache_get_nonexistent() {
        let cache = ImageCache::new();
        let path = PathBuf::from("nonexistent.png");

        assert!(!cache.contains_file(&path));
        assert!(cache.get_file(&path).is_none());
    }

    #[test]
    fn cached_image_data() {
        let img = create_test_image(5, 5);
        let cached = CachedImage {
            image: img,
            dimensions: (5, 5),
            size_bytes: 100,
        };

        assert_eq!(cached.dimensions, (5, 5));
        assert_eq!(cached.size_bytes, 100);
    }

    #[test]
    fn cache_default() {
        let cache = ImageCache::default();
        assert_eq!(cache.count(), 0);
    }
}
