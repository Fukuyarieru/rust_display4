use rand::random_range as random;
use termion as TColor;

#[derive(PartialEq, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    // pub a: u8,
}
impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b }
    }
    pub fn set(&mut self, r: u8, g: u8, b: u8) {
        self.r = r;
        self.g = g;
        self.b = b;
    }
    pub fn reset() -> String {
        TColor::color::Bg(TColor::color::Reset).to_string()
    }
    pub fn white() -> Self {
        Color::new(200, 200, 200)
    }
    pub fn black() -> Self {
        Color::new(0, 0, 0)
    }
    pub fn red() -> Self {
        Color::new(200, 0, 0)
    }
    pub fn green() -> Self {
        Color::new(0, 200, 0)
    }
    pub fn blue() -> Self {
        Color::new(0, 0, 200)
    }
    pub fn pink() -> Self {
        Color::new(200, 0, 200)
    }
    pub fn random() -> Self {
        let color_range = 0..=255;
        Color::new(
            random(color_range.clone()),
            random(color_range.clone()),
            random(color_range.clone()),
        )
    }
}
impl Default for Color {
    fn default() -> Color {
        Color::new(0, 0, 100)
    }
}
impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            TColor::color::Bg(TColor::color::Rgb(self.r, self.g, self.b))
        )
    }
}
impl Clone for Color {
    fn clone(&self) -> Color {
        Color::new(self.r, self.g, self.b)
    }
}
