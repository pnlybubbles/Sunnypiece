use super::integrator::Integrator;
use film::Film;
use std::ops::{AddAssign, DivAssign};

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
    Pixel: AddAssign<Pixel>,
    Pixel: DivAssign<f32>,
    F: Fn(&mut dyn FnMut(Pixel), f32, f32),
  {
    let film = &mut self.film;
    for y in 0..film.height {
      for x in 0..film.width {
        let u = x as f32 / film.width as f32;
        let v = y as f32 / film.height as f32;
        for _ in 0..self.spp {
          f(&mut |value| film.data[y][x] += value, u, v)
        }
        film.data[y][x] /= self.spp as f32;
      }
    }
  }
}
