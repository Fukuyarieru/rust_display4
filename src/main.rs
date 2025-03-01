#![allow(unused_imports)]
#![allow(dead_code)]

mod animations;
mod color;
mod display;
mod pixel;
mod standard;
mod tests;
mod utility;

use animations::*;
use color::*;
use display::*;
use pixel::*;
use standard::*;
use tests::*;
use utility::*;

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");

    let mut display = Display::new(240, 60);

    // dvd_logo(&mut display);
    lsd(&mut display);

    // test_display();

    // let mut display = Display::new(120, 30);
    // display.fill_with_char('a');
    // display.fill_with_color(Color::blue());

    // println!("{}", display);

    // display.circle(display.get_center_point(), 5, 'a', Color::white());

    // println!("{}", display);
}
