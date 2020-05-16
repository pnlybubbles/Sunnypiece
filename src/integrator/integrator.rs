use std::ops::{AddAssign, DivAssign};

pub trait Integrator<Pixel> {
  fn each<F>(&mut self, f: F)
  where
    Pixel: AddAssign<Pixel>,
    Pixel: DivAssign<f32>,
    F: Fn(&mut dyn FnMut(Pixel), f32, f32);
}
