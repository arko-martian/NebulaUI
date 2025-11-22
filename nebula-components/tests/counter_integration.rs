use nebula_components::{Button, Text};
use nebula_core::Signal;

#[test]
fn counter_app_integration() {
    // Create reactive counter
    let count = Signal::new(0);
    
    // Create text component
    let count_text = Text::new("Count: 0")
        .position(100.0, 50.0)
        .font_size(32);
    
    // Subscribe to count changes to update text
    let text_content = count_text.content.clone();
    count.subscribe(move |c| {
        text_content.set(format!("Count: {}", c));
    });
    
    // Create increment button
    let count_clone = count.clone();
    let inc_button = Button::new("+")
        .position(50.0, 100.0)
        .size(80.0, 50.0)
        .on_click(move || {
            count_clone.update(|c| c + 1);
        });
    
    // Create decrement button
    let count_clone = count.clone();
    let dec_button = Button::new("-")
        .position(150.0, 100.0)
        .size(80.0, 50.0)
        .on_click(move || {
            count_clone.update(|c| c - 1);
        });
    
    // Simulate clicks
    // Click increment button 3 times
    for _ in 0..3 {
        inc_button.handle_mouse_down(90.0, 125.0);
        inc_button.handle_mouse_up(90.0, 125.0);
    }
    
    // Click decrement button once
    dec_button.handle_mouse_down(190.0, 125.0);
    dec_button.handle_mouse_up(190.0, 125.0);
    
    // Verify results
    assert_eq!(count.get(), 2, "Counter should be 2 (3 increments - 1 decrement)");
    assert_eq!(count_text.get_content(), "Count: 2", "Text should display 'Count: 2'");
}
