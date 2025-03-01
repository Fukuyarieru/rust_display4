use rand::random_bool;
use rand::random_range as random;

use crate::color::Color;

pub fn random_char() -> char {
    if random_bool(0.5) {
        random('a'..='z')
    } else {
        random('A'..='Z')
    }
}
pub fn random_color() -> Color {
    Color::random()
}
