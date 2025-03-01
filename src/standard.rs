use std::{
    fmt::Display,
    sync::{Arc, Mutex},
};

pub struct Vec2<T> {
    vec2: Vec<Vec<T>>,
}
impl<T> Vec2<T> {
    pub fn new_default(width: usize, height: usize) -> Self
    where
        T: Clone + Default,
    {
        Vec2 {
            vec2: { vec![vec![T::default().clone(); height]; width] },
        }
    }
    pub fn new(width: usize, height: usize, value: T) -> Self
    where
        T: Clone,
    {
        Vec2 {
            vec2: { vec![vec![value.clone(); height]; width] },
        }
    }
    pub fn new_arc(width: usize, height: usize, value: T) -> Vec2<Arc<Mutex<T>>>
    where
        T: Clone,
    {
        let mut vec2 = Vec::<Vec<Arc<Mutex<T>>>>::with_capacity(width);

        for _ in 0..width {
            let mut inner_vec = Vec::<Arc<Mutex<T>>>::new();
            for _ in 0..height {
                inner_vec.push(Arc::new(Mutex::new(value.clone())));
            }
            vec2.push(inner_vec);
        }

        Vec2 { vec2 }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<T>
    where
        T: Clone,
    {
        if x >= self.get_width() || y >= self.get_height() {
            None
        } else {
            Some(self.vec2[x][y].clone())
        }
    }
    pub fn get_width(&self) -> usize {
        self.vec2.len()
    }
    pub fn get_height(&self) -> usize {
        self.vec2[0].len()
    }
    pub fn set(&mut self, x: usize, y: usize, value: T) -> Result<(), (usize, usize)> {
        if x >= self.get_width() || y >= self.get_height() {
            return Err((x, y));
        }

        self.vec2[x][y] = value;
        Ok(())
    }
    pub fn fill(&mut self, value: T)
    where
        T: Clone,
    {
        for inner_vec in &mut self.vec2 {
            for cell in inner_vec {
                *cell = value.clone();
            }
        }
    }
}
impl<T: Display + Clone> Display for Vec2<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.get_height() {
            for x in 0..self.get_width() {
                write!(f, "{}", self.get(x, y).unwrap().clone())?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<T: Clone> Clone for Vec2<T> {
    fn clone(&self) -> Self {
        Self {
            vec2: self.vec2.clone(),
        }
    }
}
