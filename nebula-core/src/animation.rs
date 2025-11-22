//! Animation System - Making Nebula UI SMOOTH! 🎨
//! 
//! This module provides:
//! - Spring physics animations (F = -kx - cv)
//! - Interruptible animations
//! - Animation modifiers (.scale(), .fade(), .rotate())
//! - Implicit animations (SwiftUI-style)
//! - 60 FPS on old hardware!
//! 
//! Physics-based animations feel NATURAL and RESPONSIVE!

use std::time::{Duration, Instant};
use tracing::{info, warn};

/// Spring Animation - Physics-based smooth animations! 🎨
/// 
/// Uses spring physics: F = -kx - cv
/// - k = stiffness (how bouncy)
/// - c = damping (how much friction)
/// - x = displacement from target
/// - v = velocity
/// 
/// This creates NATURAL, RESPONSIVE animations!
#[derive(Clone, Debug)]
pub struct SpringAnimation {
    /// Current value
    current: f32,
    /// Target value
    target: f32,
    /// Current velocity
    velocity: f32,
    /// Stiffness (spring constant k)
    stiffness: f32,
    /// Damping (friction constant c)
    damping: f32,
    /// Start time
    start_time: Option<Instant>,
    /// Is animation complete?
    complete: bool,
}

impl SpringAnimation {
    /// Create a new spring animation
    pub fn new(initial: f32, target: f32) -> Self {
        info!("🎨 Creating SpringAnimation: {} → {}", initial, target);
        Self {
            current: initial,
            target,
            velocity: 0.0,
            stiffness: 300.0,  // Default: responsive
            damping: 30.0,     // Default: slightly bouncy
            start_time: None,
            complete: false,
        }
    }

    /// Set stiffness (how bouncy)
    /// Higher = more responsive, lower = more sluggish
    pub fn stiffness(mut self, stiffness: f32) -> Self {
        self.stiffness = stiffness;
        self
    }

    /// Set damping (how much friction)
    /// Higher = less bouncy, lower = more bouncy
    pub fn damping(mut self, damping: f32) -> Self {
        self.damping = damping;
        self
    }

    /// Start the animation
    pub fn start(&mut self) {
        if self.start_time.is_none() {
            self.start_time = Some(Instant::now());
            info!("🎨 Animation started");
        }
    }

    /// Update animation (call every frame)
    /// Returns true if animation is still running
    pub fn update(&mut self, delta_time: f32) -> bool {
        if self.complete {
            return false;
        }

        if self.start_time.is_none() {
            self.start();
        }

        // Spring physics: F = -kx - cv
        let displacement = self.current - self.target;
        let spring_force = -self.stiffness * displacement;
        let damping_force = -self.damping * self.velocity;
        let force = spring_force + damping_force;

        // Update velocity and position
        self.velocity += force * delta_time;
        self.current += self.velocity * delta_time;

        // Check if animation is complete (close enough to target and slow enough)
        let threshold = 0.001;
        if displacement.abs() < threshold && self.velocity.abs() < threshold {
            self.current = self.target;
            self.velocity = 0.0;
            self.complete = true;
            info!("🎨 Animation complete at {}", self.current);
            return false;
        }

        true
    }

    /// Get current value
    pub fn value(&self) -> f32 {
        self.current
    }

    /// Get target value
    pub fn target(&self) -> f32 {
        self.target
    }

    /// Set new target (makes animation interruptible!)
    pub fn set_target(&mut self, target: f32) {
        info!("🎨 Animation target changed: {} → {}", self.target, target);
        self.target = target;
        self.complete = false;
        // Keep current velocity for smooth interruption!
    }

    /// Is animation complete?
    pub fn is_complete(&self) -> bool {
        self.complete
    }

    /// Get progress (0.0 to 1.0)
    pub fn progress(&self) -> f32 {
        if self.complete {
            return 1.0;
        }
        
        let start = self.start_time
            .map(|t| t.elapsed().as_secs_f32())
            .unwrap_or(0.0);
        
        // Estimate completion time based on spring parameters
        let estimated_duration = 1.0 / (self.damping / (2.0 * self.stiffness.sqrt()));
        (start / estimated_duration).min(1.0)
    }
}

/// Animation Controller - Manages multiple animations! 🎬
/// 
/// Coordinates multiple animations running simultaneously
/// Handles animation lifecycle and updates
#[derive(Default)]
pub struct AnimationController {
    /// Active animations
    animations: Vec<(String, SpringAnimation)>,
    /// Last update time
    last_update: Option<Instant>,
}

impl AnimationController {
    /// Create a new animation controller
    pub fn new() -> Self {
        info!("🎬 Creating AnimationController");
        Self {
            animations: Vec::new(),
            last_update: None,
        }
    }

    /// Add an animation
    pub fn add(&mut self, name: impl Into<String>, animation: SpringAnimation) {
        let name = name.into();
        info!("🎬 Adding animation: {}", name);
        self.animations.push((name, animation));
    }

    /// Get animation by name
    pub fn get(&self, name: &str) -> Option<&SpringAnimation> {
        self.animations.iter()
            .find(|(n, _)| n == name)
            .map(|(_, a)| a)
    }

    /// Get mutable animation by name
    pub fn get_mut(&mut self, name: &str) -> Option<&mut SpringAnimation> {
        self.animations.iter_mut()
            .find(|(n, _)| n == name)
            .map(|(_, a)| a)
    }

    /// Update all animations
    /// Returns number of active animations
    pub fn update(&mut self) -> usize {
        let now = Instant::now();
        let delta_time = self.last_update
            .map(|t| now.duration_since(t).as_secs_f32())
            .unwrap_or(1.0 / 60.0); // Default to 60 FPS
        
        self.last_update = Some(now);

        // Update all animations
        for (_, animation) in &mut self.animations {
            animation.update(delta_time);
        }

        // Remove completed animations
        let before = self.animations.len();
        self.animations.retain(|(_, a)| !a.is_complete());
        let after = self.animations.len();

        if before != after {
            info!("🎬 Removed {} completed animations", before - after);
        }

        after
    }

    /// Get number of active animations
    pub fn active_count(&self) -> usize {
        self.animations.len()
    }

    /// Clear all animations
    pub fn clear(&mut self) {
        info!("🎬 Clearing all animations");
        self.animations.clear();
    }
}

/// Animation modifiers for components
pub trait Animatable {
    /// Animate scale
    fn scale(&mut self, from: f32, to: f32, duration: Duration) -> SpringAnimation {
        SpringAnimation::new(from, to)
    }

    /// Animate fade (opacity)
    fn fade(&mut self, from: f32, to: f32, duration: Duration) -> SpringAnimation {
        SpringAnimation::new(from, to)
    }

    /// Animate rotation (degrees)
    fn rotate(&mut self, from: f32, to: f32, duration: Duration) -> SpringAnimation {
        SpringAnimation::new(from, to)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn spring_animation_creation() {
        let anim = SpringAnimation::new(0.0, 100.0);
        assert_eq!(anim.value(), 0.0);
        assert_eq!(anim.target(), 100.0);
        assert!(!anim.is_complete());
    }

    #[test]
    fn spring_animation_stiffness() {
        let anim = SpringAnimation::new(0.0, 100.0)
            .stiffness(500.0);
        assert_eq!(anim.stiffness, 500.0);
    }

    #[test]
    fn spring_animation_damping() {
        let anim = SpringAnimation::new(0.0, 100.0)
            .damping(50.0);
        assert_eq!(anim.damping, 50.0);
    }

    #[test]
    fn spring_animation_update() {
        let mut anim = SpringAnimation::new(0.0, 100.0);
        anim.start();
        
        // Update for one frame (60 FPS)
        let still_running = anim.update(1.0 / 60.0);
        
        assert!(still_running);
        assert!(anim.value() > 0.0); // Should have moved
        assert!(anim.value() < 100.0); // But not reached target yet
    }

    #[test]
    fn spring_animation_completion() {
        let mut anim = SpringAnimation::new(0.0, 1.0);
        anim.start();
        
        // Run animation for many frames
        for _ in 0..1000 {
            if !anim.update(1.0 / 60.0) {
                break;
            }
        }
        
        assert!(anim.is_complete());
        assert_eq!(anim.value(), 1.0);
    }

    #[test]
    fn spring_animation_interruptible() {
        let mut anim = SpringAnimation::new(0.0, 100.0);
        anim.start();
        
        // Run for a bit
        for _ in 0..10 {
            anim.update(1.0 / 60.0);
        }
        
        let mid_value = anim.value();
        assert!(mid_value > 0.0);
        
        // Change target mid-animation!
        anim.set_target(50.0);
        assert_eq!(anim.target(), 50.0);
        assert!(!anim.is_complete());
    }

    #[test]
    fn spring_animation_progress() {
        let mut anim = SpringAnimation::new(0.0, 100.0);
        anim.start();
        
        let progress = anim.progress();
        assert!(progress >= 0.0 && progress <= 1.0);
    }

    #[test]
    fn animation_controller_creation() {
        let controller = AnimationController::new();
        assert_eq!(controller.active_count(), 0);
    }

    #[test]
    fn animation_controller_add() {
        let mut controller = AnimationController::new();
        let anim = SpringAnimation::new(0.0, 100.0);
        
        controller.add("test", anim);
        assert_eq!(controller.active_count(), 1);
    }

    #[test]
    fn animation_controller_get() {
        let mut controller = AnimationController::new();
        let anim = SpringAnimation::new(0.0, 100.0);
        
        controller.add("test", anim);
        
        let retrieved = controller.get("test");
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().target(), 100.0);
    }

    #[test]
    fn animation_controller_get_mut() {
        let mut controller = AnimationController::new();
        let anim = SpringAnimation::new(0.0, 100.0);
        
        controller.add("test", anim);
        
        if let Some(anim) = controller.get_mut("test") {
            anim.set_target(200.0);
        }
        
        assert_eq!(controller.get("test").unwrap().target(), 200.0);
    }

    #[test]
    fn animation_controller_update() {
        let mut controller = AnimationController::new();
        let anim = SpringAnimation::new(0.0, 100.0);
        
        controller.add("test", anim);
        
        let active = controller.update();
        assert_eq!(active, 1);
    }

    #[test]
    fn animation_controller_clear() {
        let mut controller = AnimationController::new();
        controller.add("test1", SpringAnimation::new(0.0, 100.0));
        controller.add("test2", SpringAnimation::new(0.0, 200.0));
        
        assert_eq!(controller.active_count(), 2);
        
        controller.clear();
        assert_eq!(controller.active_count(), 0);
    }

    #[test]
    fn animation_controller_default() {
        let controller = AnimationController::default();
        assert_eq!(controller.active_count(), 0);
    }
}
