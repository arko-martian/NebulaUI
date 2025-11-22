use nebula_components::{Button, Text};
use nebula_core::Signal;

fn main() {
    println!("🎉 Nebula UI Counter Example!");
    println!("============================\n");
    
    // Create reactive counter
    let count = Signal::new(0);
    println!("✅ Created reactive counter signal");
    
    // Create text component
    let count_text = Text::new("Count: 0")
        .position(100.0, 50.0)
        .font_size(32);
    
    println!("✅ Created text component at position (100, 50)");
    
    // Subscribe to count changes to update text
    let text_content = count_text.content.clone();
    count.subscribe(move |c| {
        text_content.set(format!("Count: {}", c));
    });
    
    println!("✅ Connected text to counter signal");
    
    // Create increment button
    let count_clone = count.clone();
    let inc_button = Button::new("+")
        .position(50.0, 100.0)
        .size(80.0, 50.0)
        .on_click(move || {
            count_clone.update(|c| c + 1);
        });
    
    println!("✅ Created increment button (+) at position (50, 100)");
    
    // Create decrement button
    let count_clone = count.clone();
    let dec_button = Button::new("-")
        .position(150.0, 100.0)
        .size(80.0, 50.0)
        .on_click(move || {
            count_clone.update(|c| c - 1);
        });
    
    println!("✅ Created decrement button (-) at position (150, 100)");
    
    // Simulate some clicks
    println!("\n🖱️  Simulating button clicks...\n");
    
    // Click increment button 3 times
    for i in 1..=3 {
        inc_button.handle_mouse_down(90.0, 125.0);
        inc_button.handle_mouse_up(90.0, 125.0);
        println!("  ➕ Click {}: Count = {}, Text = '{}'", i, count.get(), count_text.get_content());
    }
    
    // Click decrement button once
    dec_button.handle_mouse_down(190.0, 125.0);
    dec_button.handle_mouse_up(190.0, 125.0);
    println!("  ➖ Click 4: Count = {}, Text = '{}'", count.get(), count_text.get_content());
    
    println!("\n📊 Final Results:");
    println!("  • Counter value: {}", count.get());
    println!("  • Text displays: '{}'", count_text.get_content());
    println!("  • Buttons created: 2");
    println!("  • Total clicks: 4");
    
    // Verify correctness
    assert_eq!(count.get(), 2, "Counter should be 2 (3 increments - 1 decrement)");
    assert_eq!(count_text.get_content(), "Count: 2", "Text should display 'Count: 2'");
    
    println!("\n✅ All assertions passed!");
    println!("🎉 Counter app works perfectly!");
    println!("🚀 Phase 0.5 - Proof of Concept COMPLETE!");
}
