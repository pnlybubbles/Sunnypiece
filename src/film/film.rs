use std::io;
use std::path::Path;

pub struct Film<T> {
  pub data: Vec<Vec<T>>,
  pub width: usize,
  pub height: usize,
}

impl<T: Copy> Film<T> {
  pub fn new(fill: T, width: usize, height: usize) -> Film<T> {
    Film {
      data: vec![vec![fill; width]; height],
      width: width,
      height: height,
    }
  }

  pub fn set(&mut self, x: usize, y: usize, v: T) {
    self.data[y][x] = v;
  }

  pub fn get(&self, x: usize, y: usize) -> T {
    // flipping
    self.data[self.height - y - 1][self.width - x - 1]
  }

  pub fn each_mut<F>(&mut self, f: F)
  where
    F: Fn(&mut T, usize, usize),
  {
    for y in 0..self.height {
      for x in 0..self.width {
        f(&mut self.data[y][x], x, y)
      }
    }
  }

  pub fn aspect(&self) -> f32 {
    return self.width as f32 / self.height as f32;
  }
}

pub trait Save<T>: Format {
  type Output;

  fn save(&Film<T>, &Path, impl Fn(T) -> Self::Output) -> io::Result<()>;
}

pub trait Format {
  fn ext() -> &'static str;
}
