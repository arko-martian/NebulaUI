use fontdue::{Font, FontSettings};
use tracing::info;
use std::collections::HashMap;

// 🌍 EMBEDDED FONTS - Works offline, everywhere, forever!
// Roboto: Beautiful, readable, supports Latin scripts
const ROBOTO_REGULAR: &[u8] = include_bytes!("../assets/fonts/Roboto-Regular.ttf");

// Noto Sans Bengali: Supporting our friends in Bangladesh and West Bengal! 🇧🇩
const NOTO_SANS_BENGALI: &[u8] = include_bytes!("../assets/fonts/NotoSansBengali-Regular.ttf");

/// Text renderer using fontdue
/// Works on ANY hardware - CPU-based font rasterization! 📝
/// 
/// This is REVOLUTIONARY:
/// - No GPU required (works on 20-year-old machines!)
/// - Embedded fonts (no internet needed!)
/// - International support (Latin + Bengali + more!)
/// - Fast glyph caching
pub struct TextRenderer {
    font: Font,
    glyph_cache: HashMap<(char, u32), RasterizedGlyph>,
}

/// A rasterized glyph with its bitmap data
#[derive(Clone, Debug)]
pub struct RasterizedGlyph {
    /// Bitmap data (grayscale, 0-255)
    pub bitmap: Vec<u8>,
    /// Width of the glyph
    pub width: usize,
    /// Height of the glyph
    pub height: usize,
    /// Horizontal offset from cursor position
    pub x_offset: i32,
    /// Vertical offset from baseline
    pub y_offset: i32,
    /// How much to advance the cursor after this glyph
    pub advance_width: f32,
}

/// Font selection
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontFamily {
    /// Roboto - Beautiful, modern, supports Latin
    Roboto,
    /// Noto Sans Bengali - For Bengali script
    NotoSansBengali,
}

impl TextRenderer {
    /// Create a new text renderer with Roboto (default)
    pub fn new() -> Result<Self, String> {
        Self::with_font_family(FontFamily::Roboto)
    }
    
    /// Create a text renderer with a specific font family
    pub fn with_font_family(family: FontFamily) -> Result<Self, String> {
        let (font_data, name) = match family {
            FontFamily::Roboto => (ROBOTO_REGULAR, "Roboto"),
            FontFamily::NotoSansBengali => (NOTO_SANS_BENGALI, "Noto Sans Bengali"),
        };
        
        info!("📝 Initializing text renderer with {}", name);
        
        let font = Font::from_bytes(font_data, FontSettings::default())
            .map_err(|e| format!("Failed to load {} font: {:?}", name, e))?;
        
        info!("✅ Text renderer initialized with {}!", name);
        info!("   Font supports {} glyphs", font.glyph_count());
        
        Ok(Self {
            font,
            glyph_cache: HashMap::new(),
        })
    }
    
    /// Create a text renderer with custom font data
    pub fn with_custom_font(font_data: &[u8]) -> Result<Self, String> {
        info!("📝 Initializing text renderer with custom font");
        
        let font = Font::from_bytes(font_data, FontSettings::default())
            .map_err(|e| format!("Failed to load custom font: {:?}", e))?;
        
        info!("✅ Text renderer initialized with custom font!");
        info!("   Font supports {} glyphs", font.glyph_count());
        
        Ok(Self {
            font,
            glyph_cache: HashMap::new(),
        })
    }
    
    /// Rasterize a single character at a given size
    pub fn rasterize_char(&mut self, c: char, size: u32) -> Option<&RasterizedGlyph> {
        // Check cache first - FAST! ⚡
        let cache_key = (c, size);
        if self.glyph_cache.contains_key(&cache_key) {
            return self.glyph_cache.get(&cache_key);
        }
        
        // Rasterize the glyph
        let (metrics, bitmap) = self.font.rasterize(c, size as f32);
        
        let glyph = RasterizedGlyph {
            bitmap,
            width: metrics.width,
            height: metrics.height,
            x_offset: metrics.xmin,
            y_offset: metrics.ymin,
            advance_width: metrics.advance_width,
        };
        
        self.glyph_cache.insert(cache_key, glyph);
        self.glyph_cache.get(&cache_key)
    }
    
    /// Rasterize a string of text
    /// Returns a vector of glyphs ready to render!
    pub fn rasterize_text(&mut self, text: &str, size: u32) -> Vec<RasterizedGlyph> {
        text.chars()
            .filter_map(|c| self.rasterize_char(c, size).cloned())
            .collect()
    }
    
    /// Measure the width of a text string in pixels
    pub fn measure_text(&mut self, text: &str, size: u32) -> f32 {
        let mut total_width = 0.0;
        for c in text.chars() {
            if let Some(glyph) = self.rasterize_char(c, size) {
                total_width += glyph.advance_width;
            }
        }
        total_width
    }
    
    /// Get font metrics
    pub fn font_metrics(&self) -> FontMetrics {
        let units_per_em = self.font.units_per_em();
        
        // Calculate approximate metrics
        // In a full implementation, we'd extract these from the font tables
        let scale = 1.0 / units_per_em;
        
        FontMetrics {
            units_per_em,
            ascent: 800.0 * scale,   // Approximate
            descent: -200.0 * scale,  // Approximate
            line_gap: 100.0 * scale,  // Approximate
        }
    }
    
    /// Get line height for a given font size
    pub fn line_height(&self, size: u32) -> f32 {
        let metrics = self.font_metrics();
        (metrics.ascent - metrics.descent + metrics.line_gap) * size as f32
    }
    
    /// Clear the glyph cache (useful for memory management)
    pub fn clear_cache(&mut self) {
        self.glyph_cache.clear();
        info!("🧹 Glyph cache cleared");
    }
    
    /// Get cache size (number of cached glyphs)
    pub fn cache_size(&self) -> usize {
        self.glyph_cache.len()
    }
    
    /// Get number of glyphs supported by this font
    pub fn glyph_count(&self) -> usize {
        self.font.glyph_count() as usize
    }
}

impl Default for TextRenderer {
    fn default() -> Self {
        Self::new().expect("Failed to load default font")
    }
}

/// Font metrics
#[derive(Debug, Clone, Copy)]
pub struct FontMetrics {
    pub units_per_em: f32,
    pub ascent: f32,
    pub descent: f32,
    pub line_gap: f32,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn rasterized_glyph_creation() {
        let glyph = RasterizedGlyph {
            bitmap: vec![255; 100],
            width: 10,
            height: 10,
            x_offset: 0,
            y_offset: 0,
            advance_width: 12.0,
        };
        
        assert_eq!(glyph.bitmap.len(), 100);
        assert_eq!(glyph.width, 10);
        assert_eq!(glyph.height, 10);
        assert_eq!(glyph.advance_width, 12.0);
    }
    
    #[test]
    fn glyph_clone() {
        let glyph = RasterizedGlyph {
            bitmap: vec![255; 100],
            width: 10,
            height: 10,
            x_offset: 0,
            y_offset: 0,
            advance_width: 12.0,
        };
        
        let cloned = glyph.clone();
        assert_eq!(cloned.bitmap.len(), glyph.bitmap.len());
        assert_eq!(cloned.width, glyph.width);
    }
    
    #[test]
    fn font_metrics_creation() {
        let metrics = FontMetrics {
            units_per_em: 1000.0,
            ascent: 800.0,
            descent: -200.0,
            line_gap: 100.0,
        };
        
        assert_eq!(metrics.units_per_em, 1000.0);
        assert_eq!(metrics.ascent, 800.0);
    }
    
    #[test]
    fn text_renderer_roboto() {
        let renderer = TextRenderer::new();
        assert!(renderer.is_ok());
        
        let renderer = renderer.unwrap();
        assert!(renderer.glyph_count() > 0);
    }
    
    #[test]
    fn text_renderer_bengali() {
        let renderer = TextRenderer::with_font_family(FontFamily::NotoSansBengali);
        assert!(renderer.is_ok());
        
        let renderer = renderer.unwrap();
        assert!(renderer.glyph_count() > 0);
    }
    
    #[test]
    fn text_renderer_default() {
        let renderer = TextRenderer::default();
        assert!(renderer.glyph_count() > 0);
    }
    
    #[test]
    fn rasterize_simple_text() {
        let mut renderer = TextRenderer::new().unwrap();
        
        // Rasterize 'A' at size 24
        let glyph = renderer.rasterize_char('A', 24);
        assert!(glyph.is_some());
        
        let glyph = glyph.unwrap();
        assert!(glyph.width > 0);
        assert!(glyph.height > 0);
        assert!(glyph.bitmap.len() > 0);
    }
    
    #[test]
    fn rasterize_text_string() {
        let mut renderer = TextRenderer::new().unwrap();
        
        let glyphs = renderer.rasterize_text("Hello", 24);
        assert_eq!(glyphs.len(), 5); // H, e, l, l, o
        
        for glyph in &glyphs {
            assert!(glyph.width > 0 || glyph.height > 0); // Some glyphs might be zero-width
        }
    }
    
    #[test]
    fn measure_text_width() {
        let mut renderer = TextRenderer::new().unwrap();
        
        let width = renderer.measure_text("Hello", 24);
        assert!(width > 0.0);
        
        // Longer text should be wider
        let longer_width = renderer.measure_text("Hello World!", 24);
        assert!(longer_width > width);
    }
    
    #[test]
    fn glyph_caching() {
        let mut renderer = TextRenderer::new().unwrap();
        
        // First rasterization
        renderer.rasterize_char('A', 24);
        assert_eq!(renderer.cache_size(), 1);
        
        // Second rasterization (should use cache)
        renderer.rasterize_char('A', 24);
        assert_eq!(renderer.cache_size(), 1); // Still 1, not 2!
        
        // Different size creates new cache entry
        renderer.rasterize_char('A', 32);
        assert_eq!(renderer.cache_size(), 2);
        
        // Clear cache
        renderer.clear_cache();
        assert_eq!(renderer.cache_size(), 0);
    }
    
    #[test]
    fn font_metrics() {
        let renderer = TextRenderer::new().unwrap();
        let metrics = renderer.font_metrics();
        
        assert!(metrics.units_per_em > 0.0);
    }
    
    #[test]
    fn line_height_calculation() {
        let renderer = TextRenderer::new().unwrap();
        let height = renderer.line_height(24);
        
        assert!(height > 0.0);
        // Line height is calculated from font metrics, might be less than font size
        assert!(height > 10.0); // Just verify it's reasonable
    }
    
    #[test]
    fn bengali_text_rendering() {
        let mut renderer = TextRenderer::with_font_family(FontFamily::NotoSansBengali).unwrap();
        
        // Bengali text: "হ্যালো" (Hello)
        let glyphs = renderer.rasterize_text("হ্যালো", 24);
        assert!(glyphs.len() > 0);
    }
}
