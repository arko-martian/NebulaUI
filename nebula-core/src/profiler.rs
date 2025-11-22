//! Performance Profiler - Making Nebula UI FAST! ⚡
//! 
//! This module provides:
//! - Frame time tracking (60 FPS = 16ms target)
//! - Memory usage monitoring
//! - Render pass visualization
//! - Signal dependency graph
//! - Performance warnings
//! 
//! Built with Puffin - the lightweight profiler!

use std::time::{Duration, Instant};
use std::collections::VecDeque;
use tracing::{info, warn};

/// Performance Profiler - Monitor and optimize! ⚡
/// 
/// Tracks performance metrics in real-time:
/// - Frame times (target: 16ms for 60 FPS)
/// - Memory usage
/// - Render passes
/// - Signal updates
/// 
/// Helps you keep Nebula UI BLAZINGLY FAST!
pub struct Profiler {
    /// Is profiler enabled?
    enabled: bool,
    /// Frame time history (last 120 frames = 2 seconds at 60 FPS)
    frame_times: VecDeque<Duration>,
    /// Current frame start time
    frame_start: Option<Instant>,
    /// Memory usage samples
    memory_samples: VecDeque<usize>,
    /// Render pass count
    render_passes: usize,
    /// Signal update count
    signal_updates: usize,
    /// Layout computation count
    layout_computations: usize,
    /// Warnings
    warnings: Vec<String>,
}

impl Profiler {
    /// Create a new profiler
    pub fn new() -> Self {
        info!("⚡ Creating Performance Profiler");
        Self {
            enabled: false,
            frame_times: VecDeque::with_capacity(120),
            frame_start: None,
            memory_samples: VecDeque::with_capacity(120),
            render_passes: 0,
            signal_updates: 0,
            layout_computations: 0,
            warnings: Vec::new(),
        }
    }

    /// Enable profiler
    pub fn enable(&mut self) {
        self.enabled = true;
        puffin::set_scopes_on(true);
        info!("⚡ Profiler ENABLED");
    }

    /// Disable profiler
    pub fn disable(&mut self) {
        self.enabled = false;
        puffin::set_scopes_on(false);
        info!("⚡ Profiler DISABLED");
    }

    /// Is profiler enabled?
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Start frame timing
    pub fn begin_frame(&mut self) {
        if !self.enabled {
            return;
        }

        puffin::profile_scope!("frame");
        self.frame_start = Some(Instant::now());
    }

    /// End frame timing
    pub fn end_frame(&mut self) {
        if !self.enabled {
            return;
        }

        if let Some(start) = self.frame_start.take() {
            let frame_time = start.elapsed();
            
            // Add to history
            self.frame_times.push_back(frame_time);
            if self.frame_times.len() > 120 {
                self.frame_times.pop_front();
            }

            // Check if we exceeded 16ms (60 FPS target)
            if frame_time.as_millis() > 16 {
                let warning = format!(
                    "⚠️ Frame time exceeded target: {}ms (target: 16ms)",
                    frame_time.as_millis()
                );
                warn!("{}", warning);
                self.warnings.push(warning);
            }
        }
    }

    /// Record memory usage
    pub fn record_memory(&mut self, bytes: usize) {
        if !self.enabled {
            return;
        }

        self.memory_samples.push_back(bytes);
        if self.memory_samples.len() > 120 {
            self.memory_samples.pop_front();
        }

        // Warn if memory usage is high (> 100 MB)
        if bytes > 100 * 1024 * 1024 {
            let warning = format!(
                "⚠️ High memory usage: {} MB",
                bytes / (1024 * 1024)
            );
            warn!("{}", warning);
            self.warnings.push(warning);
        }
    }

    /// Record render pass
    pub fn record_render_pass(&mut self) {
        if !self.enabled {
            return;
        }

        puffin::profile_scope!("render_pass");
        self.render_passes += 1;
    }

    /// Record signal update
    pub fn record_signal_update(&mut self) {
        if !self.enabled {
            return;
        }

        puffin::profile_scope!("signal_update");
        self.signal_updates += 1;
    }

    /// Record layout computation
    pub fn record_layout(&mut self) {
        if !self.enabled {
            return;
        }

        puffin::profile_scope!("layout");
        self.layout_computations += 1;
    }

    /// Get average frame time
    pub fn avg_frame_time(&self) -> Option<Duration> {
        if self.frame_times.is_empty() {
            return None;
        }

        let total: Duration = self.frame_times.iter().sum();
        Some(total / self.frame_times.len() as u32)
    }

    /// Get current FPS
    pub fn fps(&self) -> Option<f32> {
        self.avg_frame_time().map(|avg| {
            1.0 / avg.as_secs_f32()
        })
    }

    /// Get min frame time
    pub fn min_frame_time(&self) -> Option<Duration> {
        self.frame_times.iter().min().copied()
    }

    /// Get max frame time
    pub fn max_frame_time(&self) -> Option<Duration> {
        self.frame_times.iter().max().copied()
    }

    /// Get average memory usage
    pub fn avg_memory(&self) -> Option<usize> {
        if self.memory_samples.is_empty() {
            return None;
        }

        let total: usize = self.memory_samples.iter().sum();
        Some(total / self.memory_samples.len())
    }

    /// Get render pass count
    pub fn render_passes(&self) -> usize {
        self.render_passes
    }

    /// Get signal update count
    pub fn signal_updates(&self) -> usize {
        self.signal_updates
    }

    /// Get layout computation count
    pub fn layout_computations(&self) -> usize {
        self.layout_computations
    }

    /// Get warnings
    pub fn warnings(&self) -> &[String] {
        &self.warnings
    }

    /// Clear warnings
    pub fn clear_warnings(&mut self) {
        self.warnings.clear();
    }

    /// Reset all counters
    pub fn reset(&mut self) {
        info!("⚡ Resetting profiler");
        self.frame_times.clear();
        self.memory_samples.clear();
        self.render_passes = 0;
        self.signal_updates = 0;
        self.layout_computations = 0;
        self.warnings.clear();
    }

    /// Print performance summary
    pub fn print_summary(&self) {
        if !self.enabled {
            return;
        }

        info!("⚡ PERFORMANCE SUMMARY");
        info!("====================");
        
        if let Some(avg) = self.avg_frame_time() {
            info!("  Frame Time: {:.2}ms (avg)", avg.as_secs_f32() * 1000.0);
        }
        
        if let Some(fps) = self.fps() {
            info!("  FPS: {:.1}", fps);
        }
        
        if let Some(min) = self.min_frame_time() {
            info!("  Min Frame: {:.2}ms", min.as_secs_f32() * 1000.0);
        }
        
        if let Some(max) = self.max_frame_time() {
            info!("  Max Frame: {:.2}ms", max.as_secs_f32() * 1000.0);
        }
        
        if let Some(mem) = self.avg_memory() {
            info!("  Memory: {} MB (avg)", mem / (1024 * 1024));
        }
        
        info!("  Render Passes: {}", self.render_passes);
        info!("  Signal Updates: {}", self.signal_updates);
        info!("  Layout Computations: {}", self.layout_computations);
        
        if !self.warnings.is_empty() {
            info!("  Warnings: {}", self.warnings.len());
        }
    }
}

impl Default for Profiler {
    fn default() -> Self {
        Self::new()
    }
}

/// Performance audit macro helper
/// 
/// Use this to enforce performance constraints:
/// ```ignore
/// #[performance_audit(max_frame_time = 16, max_memory = 100)]
/// fn my_function() {
///     // Your code here
/// }
/// ```
pub struct PerformanceAudit {
    /// Maximum frame time in milliseconds
    pub max_frame_time: u64,
    /// Maximum memory in megabytes
    pub max_memory: usize,
}

impl PerformanceAudit {
    /// Create a new performance audit
    pub fn new(max_frame_time: u64, max_memory: usize) -> Self {
        Self {
            max_frame_time,
            max_memory,
        }
    }

    /// Check if frame time is within limits
    pub fn check_frame_time(&self, frame_time: Duration) -> bool {
        frame_time.as_millis() <= self.max_frame_time as u128
    }

    /// Check if memory is within limits
    pub fn check_memory(&self, memory_bytes: usize) -> bool {
        memory_bytes <= self.max_memory * 1024 * 1024
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn profiler_creation() {
        let profiler = Profiler::new();
        assert!(!profiler.is_enabled());
    }

    #[test]
    fn profiler_enable_disable() {
        let mut profiler = Profiler::new();
        
        profiler.enable();
        assert!(profiler.is_enabled());
        
        profiler.disable();
        assert!(!profiler.is_enabled());
    }

    #[test]
    fn profiler_frame_timing() {
        let mut profiler = Profiler::new();
        profiler.enable();
        
        profiler.begin_frame();
        thread::sleep(Duration::from_millis(1));
        profiler.end_frame();
        
        assert!(profiler.avg_frame_time().is_some());
    }

    #[test]
    fn profiler_fps() {
        let mut profiler = Profiler::new();
        profiler.enable();
        
        // Simulate a few frames
        for _ in 0..10 {
            profiler.begin_frame();
            thread::sleep(Duration::from_millis(16)); // ~60 FPS
            profiler.end_frame();
        }
        
        let fps = profiler.fps();
        assert!(fps.is_some());
        assert!(fps.unwrap() > 0.0);
    }

    #[test]
    fn profiler_memory_tracking() {
        let mut profiler = Profiler::new();
        profiler.enable();
        
        profiler.record_memory(1024 * 1024); // 1 MB
        profiler.record_memory(2 * 1024 * 1024); // 2 MB
        
        let avg = profiler.avg_memory();
        assert!(avg.is_some());
        assert_eq!(avg.unwrap(), 1536 * 1024); // 1.5 MB
    }

    #[test]
    fn profiler_render_passes() {
        let mut profiler = Profiler::new();
        profiler.enable();
        
        profiler.record_render_pass();
        profiler.record_render_pass();
        profiler.record_render_pass();
        
        assert_eq!(profiler.render_passes(), 3);
    }

    #[test]
    fn profiler_signal_updates() {
        let mut profiler = Profiler::new();
        profiler.enable();
        
        profiler.record_signal_update();
        profiler.record_signal_update();
        
        assert_eq!(profiler.signal_updates(), 2);
    }

    #[test]
    fn profiler_layout_computations() {
        let mut profiler = Profiler::new();
        profiler.enable();
        
        profiler.record_layout();
        
        assert_eq!(profiler.layout_computations(), 1);
    }

    #[test]
    fn profiler_warnings() {
        let mut profiler = Profiler::new();
        profiler.enable();
        
        // Simulate slow frame
        profiler.begin_frame();
        thread::sleep(Duration::from_millis(20)); // > 16ms
        profiler.end_frame();
        
        assert!(!profiler.warnings().is_empty());
    }

    #[test]
    fn profiler_clear_warnings() {
        let mut profiler = Profiler::new();
        profiler.enable();
        
        profiler.begin_frame();
        thread::sleep(Duration::from_millis(20));
        profiler.end_frame();
        
        assert!(!profiler.warnings().is_empty());
        
        profiler.clear_warnings();
        assert!(profiler.warnings().is_empty());
    }

    #[test]
    fn profiler_reset() {
        let mut profiler = Profiler::new();
        profiler.enable();
        
        profiler.record_render_pass();
        profiler.record_signal_update();
        
        profiler.reset();
        
        assert_eq!(profiler.render_passes(), 0);
        assert_eq!(profiler.signal_updates(), 0);
    }

    #[test]
    fn profiler_default() {
        let profiler = Profiler::default();
        assert!(!profiler.is_enabled());
    }

    #[test]
    fn performance_audit_creation() {
        let audit = PerformanceAudit::new(16, 100);
        assert_eq!(audit.max_frame_time, 16);
        assert_eq!(audit.max_memory, 100);
    }

    #[test]
    fn performance_audit_frame_time() {
        let audit = PerformanceAudit::new(16, 100);
        
        assert!(audit.check_frame_time(Duration::from_millis(10)));
        assert!(audit.check_frame_time(Duration::from_millis(16)));
        assert!(!audit.check_frame_time(Duration::from_millis(20)));
    }

    #[test]
    fn performance_audit_memory() {
        let audit = PerformanceAudit::new(16, 100);
        
        assert!(audit.check_memory(50 * 1024 * 1024)); // 50 MB
        assert!(audit.check_memory(100 * 1024 * 1024)); // 100 MB
        assert!(!audit.check_memory(150 * 1024 * 1024)); // 150 MB
    }
}
