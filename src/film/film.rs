use std::io;
use std::path::Path;

pub struct Film<T> {
  /**
   * Row-major order
   *
   * [(0 0), (1 0), ..., (w, 0)
   *  (0 1), (1 1), ..., (w, 1)
   * ...
   *  (0 h), (1 h), ..., (w, h)]
   */
  pub data: Vec<T>,
  pub width: usize,
  pub height: usize,
}

impl<T: Clone> Film<T> {
  pub fn new(fill: T, width: usize, height: usize) -> Film<T> {
    Film {
      data: vec![fill; height * width],
      width: width,
      height: height,
    }
  }

  pub fn uv(&self) -> impl Fn(usize) -> (f32, f32) {
    let w = self.width;
    let h = self.height;
    move |index| {
      let x = index % w;
      let y = index / w;
      (x as f32 / w as f32, y as f32 / h as f32)
    }
  }

  pub fn get(&self, x: usize, y: usize) -> &T {
    // flipping
    let row = self.height - y - 1;
    let col = self.width - x - 1;
    &self.data[row * self.width + col]
  }

  pub fn aspect(&self) -> f32 {
    return self.width as f32 / self.height as f32;
  }
}

pub trait Save<T>: Format {
  type Output;

  fn save(&Film<T>, &Path, impl Fn(&T) -> Self::Output) -> io::Result<()>;
}

pub trait Format {
  fn ext() -> &'static str;
}
