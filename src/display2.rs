use crate::color::*;
use crate::pixel::*;
use crate::standard::*;

use std::sync::Arc;
use std::sync::Mutex;

const X_STRETCH_FACTOR: f64 = 2.0;

pub struct Display2 {
    width: usize,
    height: usize,
    vec2: Vec2<Pixel>,
}

impl Display2 {
    pub fn new(width: usize, height: usize) -> Self {
        Display2 {
            width,
            height,
            vec2: Vec2::new(width, height, Pixel::default()),
        }
    }
    pub fn new_from_vec2(vec2: Vec2<Pixel>) -> Self {
        Display2 {
            width: vec2.get_width(),
            height: vec2.get_height(),
            vec2,
        }
    }
    pub fn get(&self, x: usize, y: usize) -> Pixel {
        self.vec2.get(x, y).expect("Out of bounds").clone()
    }
    pub fn get_ref(&self, x: usize, y: usize) -> Pixel {
        self.vec2.get(x, y).expect("Out of bounds").clone()
    }
    pub fn set_color(&mut self, x: usize, y: usize, new_color: Color) {
        self.vec2.get_mut_ref(x, y).expect("Out of bounds").color = new_color;
    }
    pub fn set_char(&mut self, x: usize, y: usize, new_char: char) {
        self.vec2.get_mut_ref(x, y).expect("Out of bounds").char = new_char;
    }
    pub fn set(&mut self, x: usize, y: usize, new_char: char, new_color: Color) {
        self.set_char(x, y, new_char);
        self.set_color(x, y, new_color);
    }
    pub fn get_region_as_vec2(
        &self,
        left: usize,
        right: usize,
        top: usize,
        bottom: usize,
    ) -> Vec2<Pixel> {
        if right > self.vec2.get_width() {
            panic!(
                "Out of bounds, right: {}, width: {}",
                right,
                self.vec2.get_width()
            );
        }
        if top > self.vec2.get_height() {
            panic!(
                "Out of bounds, top: {}, height: {}",
                top,
                self.vec2.get_height()
            );
        }
        if left > right {
            panic!("left > right");
        }
        if bottom > top {
            panic!("bottom > top");
        }
        let width = right - left;
        let height = top - bottom;
        let mut region = Vec2::new(width, height, Pixel::default());
        for y in bottom..top {
            for x in left..right {
                region
                    .set(x - left, y - bottom, self.get_ref(x, y).clone())
                    .unwrap();
            }
        }
        region
    }
    pub fn get_region_as_display2(
        &self,
        left: usize,
        right: usize,
        top: usize,
        bottom: usize,
    ) -> Display2 {
        if right > self.get_width() {
            panic!(
                "right > vec2 bounds, right: {}, width: {}",
                right,
                self.get_width()
            );
        }
        if top > self.get_height() {
            panic!(
                "top > vec2 bounds, top: {}, height: {}",
                top,
                self.get_height()
            );
        }
        if left > right {
            panic!("left > right, left: {}, right: {}", left, right);
        }
        if bottom > top {
            panic!("bottom > top, bottom: {}, top: {}", bottom, top);
        }
        Display2::new_from_vec2(self.get_region_as_vec2(left, right, top, bottom))
    }

    pub fn fill_with_color(&mut self, color: Color) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.set_color(x, y, color.clone());
            }
        }
    }
    pub fn fill_with_char(&mut self, char: char) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.set_char(x, y, char);
            }
        }
    }
    pub fn draw_line(
        // this function works fine even if the coordinates do not satisfy the boundaries, it will just not work on non-existent regions
        &mut self,
        (x1, y1): (usize, usize),
        (x2, y2): (usize, usize),
        char: char,
        color: Color,
    ) {
        let mut x1 = x1;
        let mut x2 = x2;
        let mut y1 = y1;
        let mut y2 = y2;

        if x1 > self.get_width() {
            x1 = self.get_width();
        }
        if x2 > self.get_width() {
            x2 = self.get_width();
        }
        if y1 > self.get_height() {
            y1 = self.get_height();
        }
        if y2 > self.get_height() {
            y2 = self.get_height()
        }

        let mut x = x1 as isize;
        let mut y = y1 as isize;
        let dx = (x2 as isize - x1 as isize).abs();
        let dy = (y2 as isize - y1 as isize).abs();
        let sx = if x1 < x2 { 1 } else { -1 };
        let sy = if y1 < y2 { 1 } else { -1 };
        let mut err = dx - dy;

        loop {
            self.set(x as usize, y as usize, char, color.clone()); // Set pixel

            if x == x2 as isize && y == y2 as isize {
                break;
            }

            let e2 = 2 * err;
            if e2 > -dy {
                err -= dy;
                x += sx;
            }
            if e2 < dx {
                err += dx;
                y += sy;
            }
        }
    }
    pub fn get_height(&self) -> usize {
        self.vec2.get_height()
    }
    pub fn get_width(&self) -> usize {
        self.vec2.get_width()
    }
    pub fn get_center_point(&self) -> (usize, usize) {
        (self.width / 2, self.height / 2)
    }
    pub fn circle(&mut self, point: (usize, usize), radius: isize, char: char, color: Color) {
        if radius <= 0 {
            return; // No circle to draw for non-positive radius
        }
        if X_STRETCH_FACTOR <= 0.0 {
            return; // Avoid non-positive stretch factor
        }

        let center_x = point.0 as isize;
        let center_y = point.1 as isize;
        let mut x = radius;
        let mut y = 0;
        let mut decision_parameter = 1 - radius;

        while x >= y {
            // Draw pixels in 8 octants, applying X_STRETCH_FACTOR
            let points = [
                (
                    center_x + (x as f64 * X_STRETCH_FACTOR) as isize,
                    center_y + y,
                ),
                (
                    center_x - (x as f64 * X_STRETCH_FACTOR) as isize,
                    center_y + y,
                ),
                (
                    center_x + (x as f64 * X_STRETCH_FACTOR) as isize,
                    center_y - y,
                ),
                (
                    center_x - (x as f64 * X_STRETCH_FACTOR) as isize,
                    center_y - y,
                ),
                (
                    center_x + (y as f64 * X_STRETCH_FACTOR) as isize,
                    center_y + x,
                ),
                (
                    center_x - (y as f64 * X_STRETCH_FACTOR) as isize,
                    center_y + x,
                ),
                (
                    center_x + (y as f64 * X_STRETCH_FACTOR) as isize,
                    center_y - x,
                ),
                (
                    center_x - (y as f64 * X_STRETCH_FACTOR) as isize,
                    center_y - x,
                ),
            ];

            for (draw_x, draw_y) in points.iter() {
                if *draw_x >= 0
                    && *draw_x < self.get_width() as isize
                    && *draw_y >= 0
                    && *draw_y < self.get_height() as isize
                {
                    self.set(*draw_x as usize, *draw_y as usize, char, color.clone());
                }
            }

            if decision_parameter < 0 {
                decision_parameter += 2 * y + 3;
                y += 1;
            } else {
                decision_parameter += 2 * (y - x) + 5;
                x -= 1;
                y += 1;
            }
        }
    }
    pub fn triangle(
        &mut self,
        point1: (usize, usize),
        point2: (usize, usize),
        point3: (usize, usize),
        char: char,
        color: Color,
    ) {
        self.draw_line(point1, point2, char, color.clone());
        self.draw_line(point2, point3, char, color.clone());
        self.draw_line(point3, point1, char, color.clone());
    }
}
impl std::fmt::Display for Display2 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for y in 0..self.get_height() {
            for x in 0..self.get_width() {
                write!(f, "{}", self.get(x, y))?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Clone for Display2 {
    fn clone(&self) -> Self {
        Self {
            width: self.width,
            height: self.height,
            vec2: self.vec2.clone(),
        }
    }
}
