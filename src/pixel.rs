use crate::color::Color;
use termion as TColor;

#[derive(PartialEq, Debug)]
pub struct Pixel {
    pub char: char,
    pub color: Color,
}
impl Pixel {
    pub fn new(char: char, color: Color) -> Self {
        Pixel { char, color }
    }
    pub fn new_empty() -> Self {
        Pixel {
            char: ' ',
            color: Color::default(),
        }
    }
}
impl Clone for Pixel {
    fn clone(&self) -> Pixel {
        Pixel {
            color: self.color.clone(),
            char: self.char,
        }
    }
}
impl Default for Pixel {
    fn default() -> Pixel {
        Pixel::new(' ', Color::black())
    }
}
impl std::fmt::Display for Pixel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}",
            self.color,
            self.char,
            TColor::color::Bg(TColor::color::Reset)
        )
    }
}
