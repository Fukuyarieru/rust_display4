use crate::{color::Color, display::Display};

pub fn test_display() {
    let display = Display::new(10, 10); // Create a small 10x10 display

    // Get the initial color of a few pixels
    let initial_pixel_0_0 = display.get(0, 0);
    let initial_pixel_1_1 = display.get(1, 1);
    let initial_pixel_5_5 = display.get(5, 5);

    println!("Display before change:\n{}", display);

    // Change only pixel at (1, 1)
    display.set_color(1, 1, Color::red());
    display.set_char(1, 1, 'X');

    println!("Display after changing pixel (1, 1):\n{}", display);

    // Check if only pixel (1, 1) changed significantly
    let after_pixel_0_0 = display.get(0, 0);
    let after_pixel_1_1 = display.get(1, 1);
    let after_pixel_5_5 = display.get(5, 5);

    println!("Initial pixel (0, 0): {}", initial_pixel_0_0);
    println!("Pixel (0, 0) after change: {}", after_pixel_0_0);
    println!("Initial pixel (1, 1): {}", initial_pixel_1_1);
    println!("Pixel (1, 1) after change: {}", after_pixel_1_1);
    println!("Initial pixel (5, 5): {}", initial_pixel_5_5);
    println!("Pixel (5, 5) after change: {}", after_pixel_5_5);

    assert_eq!(
        initial_pixel_0_0, after_pixel_0_0,
        "Pixel (0, 0) should not have changed"
    );
    assert_ne!(
        initial_pixel_1_1, after_pixel_1_1,
        "Pixel (1, 1) should have changed"
    );
    assert_eq!(
        after_pixel_1_1.color,
        Color::red(),
        "Pixel (1, 1) color should be Red"
    );
    assert_eq!(after_pixel_1_1.char, 'X', "Pixel (1, 1) char should be 'X'");
    assert_eq!(
        initial_pixel_5_5, after_pixel_5_5,
        "Pixel (5, 5) should not have changed"
    );

    println!("Test passed: Single pixel change works correctly!");
}
