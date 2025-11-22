// Stepper Component - Step indicator for multi-step processes
// Essential for wizards and onboarding flows

use nebula_core::layout::{LayoutEngine, NodeId};
use nebula_core::signal::Signal;

/// Step item
#[derive(Debug, Clone, PartialEq)]
pub struct Step {
    pub id: String,
    pub label: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub optional: bool,
    pub error: bool,
}

impl Step {
    /// Create a new step
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            description: None,
            icon: None,
            optional: false,
            error: false,
        }
    }

    /// Add a description
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Add an icon
    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Mark as optional
    pub fn optional(mut self, optional: bool) -> Self {
        self.optional = optional;
        self
    }

    /// Mark as error
    pub fn error(mut self, error: bool) -> Self {
        self.error = error;
        self
    }
}

/// Stepper orientation
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StepperOrientation {
    Horizontal,
    Vertical,
}

/// Stepper component - step indicator for multi-step processes
/// 
/// # Example
/// ```
/// let mut stepper = Stepper::new()
///     .add_step("step1", "Account")
///     .add_step("step2", "Profile")
///     .add_step("step3", "Confirm")
///     .current_step(1)
///     .on_step_click(|step_id| println!("Clicked: {}", step_id));
/// ```
pub struct Stepper {
    pub node_id: Option<NodeId>,
    pub steps: Vec<Step>,
    pub current_step: Signal<usize>,
    pub completed_steps: Signal<Vec<usize>>,
    pub orientation: StepperOrientation,
    pub step_size: f32,
    pub spacing: f32,
    pub connector_width: f32,
    pub show_numbers: bool,
    pub clickable: bool,
    pub active_color: (u8, u8, u8, u8),
    pub completed_color: (u8, u8, u8, u8),
    pub inactive_color: (u8, u8, u8, u8),
    pub error_color: (u8, u8, u8, u8),
    pub connector_color: (u8, u8, u8, u8),
    pub text_color: (u8, u8, u8, u8),
    pub active_text_color: (u8, u8, u8, u8),
    pub on_step_click: Option<Box<dyn Fn(&str)>>,
    pub on_complete: Option<Box<dyn Fn()>>,
}

impl Stepper {
    /// Create a new Stepper component
    pub fn new() -> Self {
        Self {
            node_id: None,
            steps: Vec::new(),
            current_step: Signal::new(0),
            completed_steps: Signal::new(Vec::new()),
            orientation: StepperOrientation::Horizontal,
            step_size: 40.0,
            spacing: 16.0,
            connector_width: 2.0,
            show_numbers: true,
            clickable: false,
            active_color: (59, 130, 246, 255), // Blue
            completed_color: (34, 197, 94, 255), // Green
            inactive_color: (200, 200, 200, 255), // Gray
            error_color: (239, 68, 68, 255), // Red
            connector_color: (220, 220, 220, 255),
            text_color: (100, 100, 100, 255),
            active_text_color: (0, 0, 0, 255),
            on_step_click: None,
            on_complete: None,
        }
    }

    /// Set the current step
    pub fn current_step(mut self, step: usize) -> Self {
        self.current_step.set(step);
        self
    }

    /// Set the orientation
    pub fn orientation(mut self, orientation: StepperOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// Set the step size
    pub fn step_size(mut self, size: f32) -> Self {
        self.step_size = size;
        self
    }

    /// Set the spacing
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }

    /// Set the connector width
    pub fn connector_width(mut self, width: f32) -> Self {
        self.connector_width = width;
        self
    }

    /// Show or hide step numbers
    pub fn show_numbers(mut self, show: bool) -> Self {
        self.show_numbers = show;
        self
    }

    /// Make steps clickable
    pub fn clickable(mut self, clickable: bool) -> Self {
        self.clickable = clickable;
        self
    }

    /// Set active color
    pub fn active_color(mut self, r: u8, g: u8, b: u8, a: u8) -> Self {
        self.active_color = (r, g, b, a);
        self
    }

    /// Set completed color
    pub fn completed_color(mut self, r: u8, g: u8, b: u8, a: u8) -> Self {
        self.completed_color = (r, g, b, a);
        self
    }

    /// Set error color
    pub fn error_color(mut self, r: u8, g: u8, b: u8, a: u8) -> Self {
        self.error_color = (r, g, b, a);
        self
    }

    /// Add a step
    pub fn add_step(mut self, id: impl Into<String>, label: impl Into<String>) -> Self {
        self.steps.push(Step::new(id, label));
        self
    }

    /// Add a step object
    pub fn add_step_object(mut self, step: Step) -> Self {
        self.steps.push(step);
        self
    }

    /// Set all steps at once
    pub fn steps(mut self, steps: Vec<Step>) -> Self {
        self.steps = steps;
        self
    }

    /// Set the step click callback
    pub fn on_step_click<F>(mut self, callback: F) -> Self
    where
        F: Fn(&str) + 'static,
    {
        self.on_step_click = Some(Box::new(callback));
        self
    }

    /// Set the complete callback
    pub fn on_complete<F>(mut self, callback: F) -> Self
    where
        F: Fn() + 'static,
    {
        self.on_complete = Some(Box::new(callback));
        self
    }

    /// Go to a specific step
    pub fn go_to_step(&mut self, index: usize) {
        if index < self.steps.len() {
            self.current_step.set(index);
        }
    }

    /// Go to step by ID
    pub fn go_to_step_by_id(&mut self, id: &str) {
        if let Some(index) = self.find_step(id) {
            self.go_to_step(index);
        }
    }

    /// Go to next step
    pub fn next(&mut self) {
        let current = self.current_step.get();
        if current < self.steps.len() - 1 {
            // Mark current as completed
            let mut completed = self.completed_steps.get();
            if !completed.contains(&current) {
                completed.push(current);
                self.completed_steps.set(completed);
            }

            self.current_step.set(current + 1);
        } else if current == self.steps.len() - 1 {
            // Last step - mark as completed and trigger callback
            let mut completed = self.completed_steps.get();
            if !completed.contains(&current) {
                completed.push(current);
                self.completed_steps.set(completed);
            }

            if let Some(ref callback) = self.on_complete {
                callback();
            }
        }
    }

    /// Go to previous step
    pub fn previous(&mut self) {
        let current = self.current_step.get();
        if current > 0 {
            self.current_step.set(current - 1);
        }
    }

    /// Get current step index
    pub fn get_current_step(&self) -> usize {
        self.current_step.get()
    }

    /// Get current step ID
    pub fn get_current_step_id(&self) -> Option<String> {
        self.steps.get(self.current_step.get()).map(|s| s.id.clone())
    }

    /// Check if step is current
    pub fn is_current(&self, index: usize) -> bool {
        self.current_step.get() == index
    }

    /// Check if step is completed
    pub fn is_completed(&self, index: usize) -> bool {
        self.completed_steps.get().contains(&index)
    }

    /// Mark step as completed
    pub fn mark_completed(&mut self, index: usize) {
        if index < self.steps.len() {
            let mut completed = self.completed_steps.get();
            if !completed.contains(&index) {
                completed.push(index);
                self.completed_steps.set(completed);
            }
        }
    }

    /// Mark step as incomplete
    pub fn mark_incomplete(&mut self, index: usize) {
        let mut completed = self.completed_steps.get();
        if let Some(pos) = completed.iter().position(|&i| i == index) {
            completed.remove(pos);
            self.completed_steps.set(completed);
        }
    }

    /// Reset all progress
    pub fn reset(&mut self) {
        self.current_step.set(0);
        self.completed_steps.set(Vec::new());
    }

    /// Check if on first step
    pub fn is_first_step(&self) -> bool {
        self.current_step.get() == 0
    }

    /// Check if on last step
    pub fn is_last_step(&self) -> bool {
        self.current_step.get() == self.steps.len().saturating_sub(1)
    }

    /// Check if all steps completed
    pub fn is_complete(&self) -> bool {
        self.completed_steps.get().len() == self.steps.len()
    }

    /// Get step count
    pub fn step_count(&self) -> usize {
        self.steps.len()
    }

    /// Check if has steps
    pub fn has_steps(&self) -> bool {
        !self.steps.is_empty()
    }

    /// Find step by ID
    pub fn find_step(&self, id: &str) -> Option<usize> {
        self.steps.iter().position(|step| step.id == id)
    }

    /// Get step by index
    pub fn get_step(&self, index: usize) -> Option<&Step> {
        self.steps.get(index)
    }

    /// Handle step click
    pub fn handle_step_click(&mut self, index: usize) {
        if !self.clickable || index >= self.steps.len() {
            return;
        }

        self.go_to_step(index);

        if let Some(ref callback) = self.on_step_click {
            callback(&self.steps[index].id);
        }
    }

    /// Build the stepper layout
    pub fn build(&mut self, engine: &mut LayoutEngine) -> Result<NodeId, String> {
        let flex_direction = match self.orientation {
            StepperOrientation::Horizontal => taffy::style::FlexDirection::Row,
            StepperOrientation::Vertical => taffy::style::FlexDirection::Column,
        };

        let style = taffy::style::Style {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Percent(1.0),
                height: taffy::style::Dimension::Auto,
            },
            display: taffy::style::Display::Flex,
            flex_direction,
            ..Default::default()
        };

        let node = engine
            .new_leaf(style)
            .map_err(|e| format!("Failed to create stepper node: {:?}", e))?;
        self.node_id = Some(node);

        Ok(node)
    }
}

impl Default for Stepper {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stepper_starts_empty() {
        let stepper = Stepper::new();
        assert_eq!(stepper.step_count(), 0);
        assert!(!stepper.has_steps());
        assert_eq!(stepper.get_current_step(), 0);
    }

    #[test]
    fn stepper_add_steps() {
        let stepper = Stepper::new()
            .add_step("step1", "Step 1")
            .add_step("step2", "Step 2")
            .add_step("step3", "Step 3");

        assert_eq!(stepper.step_count(), 3);
        assert!(stepper.has_steps());
    }

    #[test]
    fn stepper_go_to_step() {
        let mut stepper = Stepper::new()
            .add_step("step1", "Step 1")
            .add_step("step2", "Step 2");

        stepper.go_to_step(1);
        assert_eq!(stepper.get_current_step(), 1);
        assert!(stepper.is_current(1));
    }

    #[test]
    fn stepper_go_to_step_by_id() {
        let mut stepper = Stepper::new()
            .add_step("step1", "Step 1")
            .add_step("step2", "Step 2");

        stepper.go_to_step_by_id("step2");
        assert_eq!(stepper.get_current_step_id(), Some("step2".to_string()));
    }

    #[test]
    fn stepper_next() {
        let mut stepper = Stepper::new()
            .add_step("step1", "Step 1")
            .add_step("step2", "Step 2");

        stepper.next();
        assert_eq!(stepper.get_current_step(), 1);
        assert!(stepper.is_completed(0));
    }

    #[test]
    fn stepper_previous() {
        let mut stepper = Stepper::new()
            .add_step("step1", "Step 1")
            .add_step("step2", "Step 2")
            .current_step(1);

        stepper.previous();
        assert_eq!(stepper.get_current_step(), 0);
    }

    #[test]
    fn stepper_previous_at_first() {
        let mut stepper = Stepper::new()
            .add_step("step1", "Step 1")
            .current_step(0);

        stepper.previous();
        assert_eq!(stepper.get_current_step(), 0); // Should stay at 0
    }

    #[test]
    fn stepper_is_first_step() {
        let stepper = Stepper::new()
            .add_step("step1", "Step 1")
            .add_step("step2", "Step 2")
            .current_step(0);

        assert!(stepper.is_first_step());
    }

    #[test]
    fn stepper_is_last_step() {
        let stepper = Stepper::new()
            .add_step("step1", "Step 1")
            .add_step("step2", "Step 2")
            .current_step(1);

        assert!(stepper.is_last_step());
    }

    #[test]
    fn stepper_mark_completed() {
        let mut stepper = Stepper::new()
            .add_step("step1", "Step 1")
            .add_step("step2", "Step 2");

        stepper.mark_completed(0);
        assert!(stepper.is_completed(0));
    }

    #[test]
    fn stepper_mark_incomplete() {
        let mut stepper = Stepper::new()
            .add_step("step1", "Step 1");

        stepper.mark_completed(0);
        assert!(stepper.is_completed(0));

        stepper.mark_incomplete(0);
        assert!(!stepper.is_completed(0));
    }

    #[test]
    fn stepper_reset() {
        let mut stepper = Stepper::new()
            .add_step("step1", "Step 1")
            .add_step("step2", "Step 2")
            .current_step(1);

        stepper.mark_completed(0);
        stepper.reset();

        assert_eq!(stepper.get_current_step(), 0);
        assert!(!stepper.is_completed(0));
    }

    #[test]
    fn stepper_is_complete() {
        let mut stepper = Stepper::new()
            .add_step("step1", "Step 1")
            .add_step("step2", "Step 2");

        stepper.mark_completed(0);
        stepper.mark_completed(1);

        assert!(stepper.is_complete());
    }

    #[test]
    fn stepper_find_step() {
        let stepper = Stepper::new()
            .add_step("step1", "Step 1")
            .add_step("step2", "Step 2");

        assert_eq!(stepper.find_step("step2"), Some(1));
        assert_eq!(stepper.find_step("nonexistent"), None);
    }

    #[test]
    fn stepper_get_step() {
        let stepper = Stepper::new()
            .add_step("step1", "Step 1");

        let step = stepper.get_step(0);
        assert!(step.is_some());
        assert_eq!(step.unwrap().label, "Step 1");
    }

    #[test]
    fn step_with_description_and_icon() {
        let step = Step::new("step1", "Step 1")
            .with_description("Description")
            .with_icon("check")
            .optional(true)
            .error(false);

        assert_eq!(step.description, Some("Description".to_string()));
        assert_eq!(step.icon, Some("check".to_string()));
        assert!(step.optional);
        assert!(!step.error);
    }

    #[test]
    fn stepper_handle_step_click() {
        let mut stepper = Stepper::new()
            .add_step("step1", "Step 1")
            .add_step("step2", "Step 2")
            .clickable(true);

        stepper.handle_step_click(1);
        assert_eq!(stepper.get_current_step(), 1);
    }

    #[test]
    fn stepper_handle_step_click_not_clickable() {
        let mut stepper = Stepper::new()
            .add_step("step1", "Step 1")
            .add_step("step2", "Step 2")
            .clickable(false);

        stepper.handle_step_click(1);
        assert_eq!(stepper.get_current_step(), 0); // Should not change
    }

    #[test]
    fn stepper_callbacks() {
        use std::sync::{Arc, Mutex};

        let clicked = Arc::new(Mutex::new(String::new()));
        let clicked_clone = clicked.clone();

        let completed = Arc::new(Mutex::new(false));
        let completed_clone = completed.clone();

        let mut stepper = Stepper::new()
            .add_step("step1", "Step 1")
            .add_step("step2", "Step 2")
            .clickable(true)
            .on_step_click(move |id| {
                *clicked_clone.lock().unwrap() = id.to_string();
            })
            .on_complete(move || {
                *completed_clone.lock().unwrap() = true;
            });

        stepper.handle_step_click(1);
        assert_eq!(*clicked.lock().unwrap(), "step2");

        stepper.next(); // Complete last step
        assert!(*completed.lock().unwrap());
    }

    #[test]
    fn stepper_builder_pattern() {
        let stepper = Stepper::new()
            .orientation(StepperOrientation::Vertical)
            .step_size(50.0)
            .spacing(20.0)
            .connector_width(3.0)
            .show_numbers(false)
            .clickable(true)
            .active_color(255, 0, 0, 255)
            .completed_color(0, 255, 0, 255)
            .error_color(255, 255, 0, 255);

        assert_eq!(stepper.orientation, StepperOrientation::Vertical);
        assert_eq!(stepper.step_size, 50.0);
        assert_eq!(stepper.spacing, 20.0);
        assert_eq!(stepper.connector_width, 3.0);
        assert!(!stepper.show_numbers);
        assert!(stepper.clickable);
        assert_eq!(stepper.active_color, (255, 0, 0, 255));
        assert_eq!(stepper.completed_color, (0, 255, 0, 255));
        assert_eq!(stepper.error_color, (255, 255, 0, 255));
    }

    #[test]
    fn stepper_build_creates_node() {
        let mut engine = LayoutEngine::new();
        let mut stepper = Stepper::new().add_step("step1", "Step 1");

        let result = stepper.build(&mut engine);
        assert!(result.is_ok());
        assert!(stepper.node_id.is_some());
    }
}
