#![allow(unused_imports)]
#![allow(dead_code)]

mod animations;
mod color;
mod display;
mod display2;
mod pixel;
mod standard;
mod tests;
mod utility;

use animations::*;
use color::*;
use display::*;
use display2::*;
use pixel::*;
use standard::*;
use tests::*;
use utility::*;

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");

    let mut display = Display2::new(160, 40);

    rgb_load(&mut display);
    // dvd_logo(&mut display);
    // lsd(&mut display);
    // jumping_ball(&mut display, 1);

    // test_display();

    // let mut display = Display::new(120, 30);
    // display.fill_with_char('a');
    // display.fill_with_color(Color::blue());

    // println!("{}", display);

    // display.circle(display.get_center_point(), 5, 'a', Color::white());

    // println!("{}", display);
}
