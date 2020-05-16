use super::integrator::Integrator;
use film::Film;
use std::ops::{Add, Div};

pub struct Debug<'a, Pixel> {
  pub film: &'a mut Film<Pixel>,
  pub spp: usize,
}

impl<'a, Pixel> Debug<'a, Pixel> {
  pub fn new<'b>(film: &'b mut Film<Pixel>, spp: usize) -> Debug<'b, Pixel> {
    Debug {
      film: film,
      spp: spp,
    }
  }
}

impl<'a, Pixel> Integrator<Pixel> for Debug<'a, Pixel> {
  fn each<F>(&mut self, f: F)
  where
    Pixel: Clone + Send + Sync + Add<Pixel, Output = Pixel> + Div<f32, Output = Pixel>,
    F: Fn(f32, f32) -> Pixel,
  {
    let spp = self.spp;
    let uv = self.film.uv();
    self
      .film
      .data
      .iter_mut()
      .enumerate()
      .for_each(|(index, pixel)| {
        *pixel = (0..spp).fold(pixel.clone(), |sum, _| {
          let (u, v) = uv(index);
          sum + f(u, v)
        }) / spp as f32
      })
  }
}
