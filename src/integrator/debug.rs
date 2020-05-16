use super::integrator::Integrator;
use film::Film;
use std::ops::{Add, Div};

pub struct DebugIntegrator<'a, Pixel> {
  pub film: &'a mut Film<Pixel>,
  pub spp: usize,
}

impl<'a, Pixel> DebugIntegrator<'a, Pixel> {
  pub fn new<'b>(film: &'b mut Film<Pixel>, spp: usize) -> DebugIntegrator<'b, Pixel> {
    DebugIntegrator {
      film: film,
      spp: spp,
    }
  }
}

impl<'a, Pixel> Integrator<Pixel> for DebugIntegrator<'a, Pixel> {
  fn each<F>(&mut self, f: F)
  where
    Pixel: Clone,
    Pixel: Add<Pixel, Output = Pixel>,
    Pixel: Div<f32, Output = Pixel>,
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
