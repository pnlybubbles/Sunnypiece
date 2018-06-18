use std::path::Path;
use std::io;

pub struct Image<T> {
  pub data: Vec<Vec<T>>,
  pub width: usize,
  pub height: usize,
}

impl<T: Copy> Image<T> {
  pub fn new(fill: T, width: usize, height: usize) -> Image<T> {
    Image {
      data: vec![vec![fill; width]; height],
      width: width,
      height: height,
    }
  }

  pub fn set(&mut self, x: usize, y: usize, v: T) {
    self.data[y][x] = v;
  }

  pub fn get(&self, x: usize, y: usize) -> T {
    self.data[y][x]
  }

  pub fn each_mut<F>(&mut self, f: F)
    where F: Fn(&mut T, usize, usize)
  {
    for y in 0..self.height {
      for x in 0..self.width {
        f(&mut self.data[y][x], x, y)
      }
    }
  }

  pub fn save<S>(&self, path: &Path, f: &Fn(T) -> S::Output) -> io::Result<()>
    where S: Save<T>
  {
    S::save(self, path, f)
  }
}

pub trait Save<T>: Format {
  type Output;

  fn save(&Image<T>, &Path, f: &Fn(T) -> Self::Output) -> io::Result<()>;
}

pub trait Format {
  fn ext() -> &'static str;
}
