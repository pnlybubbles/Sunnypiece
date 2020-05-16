use std::ops::{Add, Div};

pub trait Integrator<Pixel> {
  fn each<F>(&mut self, f: F)
  where
    Pixel: Clone,
    Pixel: Add<Pixel, Output = Pixel>,
    Pixel: Div<f32, Output = Pixel>,
    F: Fn(f32, f32) -> Pixel;
}
