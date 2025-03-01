use std::thread::sleep;
use std::time::Duration;

use crate::color::*;
use crate::display;
use crate::display::*;
use crate::utility::random_char;

pub fn lsd(display: &mut Display) {
    loop {
        let delay = Duration::from_millis(10);

        let mut x = 0;
        let mut y = 0;
        let center = display.get_center_point();
        let width = display.get_width() - 1;
        let height = display.get_height() - 1;

        let mut color = Color::new(0, 0, 0);
        let mut char = ' ';

        while x < width {
            display.draw_line((x, y), center, char, color.clone());
            x += 1;
            color = Color::random();
            char = random_char();
            print!("{display}");
            sleep(delay);
        }
        while y < height {
            display.draw_line((x, y), center, char, color.clone());
            y += 1;
            color = Color::random();
            char = random_char();
            print!("{display}");
            sleep(delay);
        }
        while x > 0 {
            display.draw_line((x, y), center, char, color.clone());
            x -= 1;
            color = Color::random();
            char = random_char();
            print!("{display}");
            sleep(delay);
        }
        while y > 0 {
            display.draw_line((x, y), center, char, color.clone());
            y -= 1;
            color = Color::random();
            char = random_char();
            print!("{display}");
            sleep(delay);
        }
    }
}
pub fn jumping_ball(display: &mut Display) {
    let delay = Duration::from_millis(50); // Adjust delay for speed

    let center_x = display.get_width() / 2;
    let mut center_y = display.get_height() / 2; // Start in the middle
    let radius = 5;
    let mut velocity_y = 1; // Initial downward velocity

    loop {
        // Clear the display
        display.fill_with_color(Color::black());

        // Draw the ball
        display.circle((center_x, center_y), radius as isize, 'O', Color::red());

        // Update position
        center_y = (center_y as isize + velocity_y) as usize;

        // Bounce on top and bottom
        if center_y <= radius || center_y >= display.get_height() - radius {
            velocity_y *= -1;
        }

        // Print and delay
        println!("{}", display);
        sleep(delay);
    }
}

pub fn dvd_logo(display: &mut Display) {
    let delay = Duration::from_millis(1);

    let logo = "DVD"; // Single string logo
    let logo_width = logo.len();
    let logo_height = 1; // Logo height is now 1
    let mut x = display.get_width() / 2 - logo_width / 2;
    let mut y = display.get_height() / 2 - logo_height / 2;
    let mut dx = 1;
    let mut dy = 1;

    let mut color = Color::red();

    loop {
        // Draw the single-line logo
        for (col_index, char) in logo.chars().enumerate() {
            let draw_x = x + col_index;
            if draw_x < display.get_width() && y < display.get_height() {
                display.set(draw_x, y, char, color.clone());
            }
        }

        x = (x as isize + dx) as usize;
        y = (y as isize + dy) as usize;

        if x == 0 || x >= display.get_width() - logo_width {
            dx *= -1;
            color = Color::random();
        }
        if y == 0 || y >= display.get_height() - logo_height {
            dy *= -1;
            color = Color::random();
        }

        println!("{}", display);
        sleep(delay);
    }
}
